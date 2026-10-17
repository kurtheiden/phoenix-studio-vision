//! CCA-091's two exact Program-only tuples. No numeric-label semantics or export authority.
use crate::{
    bounded_routing::{decode_bounded_routing, BoundedRoutingResolution},
    controller166::qualify_track as qualify_accounting,
    midi_export::{DecodedExportEvent, DecodedExportEventKind},
    mixed_event::{
        walk_bounded_mixed_events, BoundedPatchToNoteTransition, MixedEventBounds, MixedEventItem,
        MixedEventKind,
    },
    sequence_container::parse_project_166,
};
use std::ops::Range;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NumericProgramProvenance {
    pub sequence_ordinal: usize,
    pub descriptor_ordinal: usize,
    pub pair_ordinal: usize,
    pub primary_range: Range<usize>,
    pub secondary_range: Range<usize>,
    pub event_bounds: Range<usize>,
    pub declared_source_objects: u32,
    pub composition_range: Range<usize>,
    pub composition_bytes: Vec<u8>,
    /// Includes the Note status byte, as in the source parser; not another object.
    pub patch_range: Range<usize>,
    pub note_range: Range<usize>,
    pub payload_range: Range<usize>,
    pub payload_length_offset: usize,
    pub pre_name_range: Range<usize>,
    pub pre_name_bytes: [u8; 5],
    pub name_length_offset: usize,
    pub name_range: Range<usize>,
    pub post_name_range: Range<usize>,
    pub post_name_bytes: Vec<u8>,
    pub program_offset: usize,
    pub program: u8,
    pub patch_timing_range: Range<usize>,
    pub note_timing_range: Range<usize>,
    pub previous_tick: u32,
    pub patch_interval: u32,
    pub note_interval: u32,
    pub patch_tick: u32,
    pub note_tick: u32,
    pub logical_patch_ordinal: u64,
    pub logical_note_ordinal: u64,
    pub raw_patch_ordinal: u64,
    pub raw_note_ordinal: u64,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NumericProgramPatch {
    provenance: NumericProgramProvenance,
    routing: BoundedRoutingResolution,
    events: [DecodedExportEvent; 2],
}
impl NumericProgramPatch {
    pub fn provenance(&self) -> &NumericProgramProvenance {
        &self.provenance
    }
    pub fn routing(&self) -> &BoundedRoutingResolution {
        &self.routing
    }
    pub fn decoded_export_events(&self) -> [DecodedExportEvent; 2] {
        self.events.clone()
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NumericProgramRefusal {
    Ownership,
    Accounting,
    Routing,
    Walk,
    Placement,
    Wrapper,
    Successor,
    MidiData,
    Timing,
    Binding,
}

fn validate_transition(
    t: &BoundedPatchToNoteTransition<'_>,
    opening: bool,
    previous: u32,
) -> Result<(), NumericProgramRefusal> {
    use NumericProgramRefusal::*;
    let p = &t.patch;
    let (pre, post, program): (&[u8], &[u8], u8) = if opening {
        (&[0, 0, 1, 0, 1], &[1, b'2', 4, 255, 255, 255], 1)
    } else {
        (&[0, 0, 63, 8, 31], &[2, b'3', b'2', 4, 255, 255, 255], 63)
    };
    if p.pre_name_context.bytes != pre
        || p.post_name_context.bytes != post
        || p.program_change.value != program
        || p.payload_range.end != p.program_change.offset + 1
        || p.payload_range.len() != usize::from(p.payload_length.value)
        || p.name.bytes.len() != usize::from(p.name_length.value)
        || p.payload_range.len() != 5 + 1 + p.name.bytes.len() + post.len() + 1
    {
        return Err(Wrapper);
    }
    if t.initial_context.is_some()
        || t.context.is_some()
        || t.final_timing.is_some()
        || !p.pre_note_context.bytes.is_empty()
        || p.note_status.value != 0x90
        || t.first_note.status.map(|s| s.value) != Some(0x90)
        || p.representation_range.end != t.first_note.representation_range.start + 1
        || t.representation_range.start != p.representation_range.start
        || t.representation_range.end != t.first_note.representation_range.end
    {
        return Err(Successor);
    }
    if t.first_note.pitch.value > 127
        || t.first_note.attack_velocity.value > 127
        || t.first_note.release_velocity.value > 127
    {
        return Err(MidiData);
    }
    if previous.checked_add(p.position.value) != Some(t.patch_position)
        || t.patch_position
            .checked_add(p.post_pc_timing_component.value)
            != Some(t.first_note_position)
        || t.first_note_position
            .checked_add(t.first_note.duration.value)
            .is_none()
    {
        return Err(Timing);
    }
    Ok(())
}

/// Source-derived evidence entry point. A must open the Track. B must follow
/// ordinary Notes, optionally preceded by opening A. Contexts and other prefix
/// families receive no new placement authority. Names/offsets identify no class.
pub fn qualify_at(
    bytes: &[u8],
    sequence_ordinal: usize,
    pair_ordinal: usize,
    logical_ordinal: u64,
) -> Result<NumericProgramPatch, NumericProgramRefusal> {
    use NumericProgramRefusal::*;
    let project = parse_project_166(bytes).map_err(|_| Ownership)?;
    let s = project.sequences.get(sequence_ordinal).ok_or(Ownership)?;
    let pair = s.track_pairs.get(pair_ordinal).ok_or(Ownership)?;
    // Includes exhaustive ordinal ownership, full routing/walk and raw object
    // accounting (including retained contexts), even when no Controllers exist.
    qualify_accounting(bytes, sequence_ordinal, pair_ordinal).map_err(|_| Accounting)?;
    let routing = decode_bounded_routing(bytes)
        .map_err(|_| Routing)?
        .into_iter()
        .find(|r| {
            r.structural_ordinal == sequence_ordinal && r.descriptor_ordinal == pair_ordinal + 2
        })
        .ok_or(Routing)?
        .result
        .map_err(|_| Routing)?;
    let bounds = s
        .validated_track_event_bounds(pair_ordinal)
        .map_err(|_| Walk)?
        .event_range;
    let walk = walk_bounded_mixed_events(
        bytes,
        MixedEventBounds {
            event_range: bounds.clone(),
        },
        Default::default(),
    )
    .map_err(|_| Walk)?;
    if walk.consumed_range != bounds
        || routing.event_range != bounds
        || routing.consumed_range != bounds
    {
        return Err(Walk);
    }
    let mut ordinal = 0;
    let mut previous = 0;
    let mut previous_is_note = false;
    for item in &walk.items {
        if ordinal == logical_ordinal {
            let MixedEventItem::PatchToNote(t) = item else {
                return Err(Placement);
            };
            let opening = ordinal == 0;
            if !opening && !previous_is_note {
                return Err(Placement);
            }
            validate_transition(t, opening, previous)?;
            let p = &t.patch;
            let events = [
                DecodedExportEvent {
                    absolute_position: t.patch_position,
                    source_ordinal: ordinal,
                    source_range: Some(p.representation_range.clone()),
                    kind: DecodedExportEventKind::TargetedProgram {
                        channel: routing.midi_channel,
                        program: p.program_change.value,
                    },
                },
                DecodedExportEvent::from_note_body(
                    t.first_note_position,
                    ordinal + 1,
                    &t.first_note,
                ),
            ];
            return Ok(NumericProgramPatch {
                provenance: NumericProgramProvenance {
                    sequence_ordinal,
                    descriptor_ordinal: pair_ordinal + 2,
                    pair_ordinal,
                    primary_range: pair.primary.record_range.clone(),
                    secondary_range: pair.secondary.record_range.clone(),
                    event_bounds: bounds,
                    declared_source_objects: u32::from_be_bytes(
                        pair.primary.payload.bytes[2..6]
                            .try_into()
                            .map_err(|_| Accounting)?,
                    ),
                    composition_range: t.representation_range.clone(),
                    composition_bytes: bytes[t.representation_range.clone()].to_vec(),
                    patch_range: p.representation_range.clone(),
                    note_range: t.first_note.representation_range.clone(),
                    payload_range: p.payload_range.clone(),
                    payload_length_offset: p.payload_length.offset,
                    pre_name_range: p.pre_name_context.range.clone(),
                    pre_name_bytes: p.pre_name_context.bytes.try_into().map_err(|_| Wrapper)?,
                    name_length_offset: p.name_length.offset,
                    name_range: p.name.range.clone(),
                    post_name_range: p.post_name_context.range.clone(),
                    post_name_bytes: p.post_name_context.bytes.to_vec(),
                    program_offset: p.program_change.offset,
                    program: p.program_change.value,
                    patch_timing_range: p.position.range.clone(),
                    note_timing_range: p.post_pc_timing_component.range.clone(),
                    previous_tick: previous,
                    patch_interval: p.position.value,
                    note_interval: p.post_pc_timing_component.value,
                    patch_tick: t.patch_position,
                    note_tick: t.first_note_position,
                    logical_patch_ordinal: ordinal,
                    logical_note_ordinal: ordinal + 1,
                    // The admitted prefix contains no separate context objects.
                    raw_patch_ordinal: ordinal,
                    raw_note_ordinal: ordinal + 1,
                },
                routing,
                events,
            });
        }
        match item {
            MixedEventItem::Event(e) if matches!(e.event, MixedEventKind::Note(_)) => {
                previous = e.position;
                previous_is_note = true;
            }
            MixedEventItem::PatchToNote(t) if ordinal == 0 => {
                validate_transition(t, true, 0)?;
                previous = t.first_note_position;
                previous_is_note = true;
            }
            _ => return Err(Placement),
        }
        ordinal += item.logical_event_count() as u64;
    }
    Err(Placement)
}

/// Re-derive all facts and bind to the exact currently owned builder item. A
/// supplied route is checked against source authority, never used as authority.
pub(crate) fn qualify_current(
    bytes: &[u8],
    sequence_ordinal: usize,
    pair_ordinal: usize,
    current: &MixedEventItem<'_>,
    routing: &BoundedRoutingResolution,
    ordinal: u64,
) -> Result<NumericProgramPatch, NumericProgramRefusal> {
    let proof = qualify_at(bytes, sequence_ordinal, pair_ordinal, ordinal)?;
    if proof.routing() != routing {
        return Err(NumericProgramRefusal::Binding);
    }
    let walk = walk_bounded_mixed_events(
        bytes,
        MixedEventBounds {
            event_range: proof.routing.event_range.clone(),
        },
        Default::default(),
    )
    .map_err(|_| NumericProgramRefusal::Walk)?;
    let mut source_ordinal = 0;
    for item in &walk.items {
        if source_ordinal == ordinal {
            return if item == current {
                Ok(proof)
            } else {
                Err(NumericProgramRefusal::Binding)
            };
        }
        source_ordinal += item.logical_event_count() as u64;
    }
    Err(NumericProgramRefusal::Binding)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> Vec<u8> {
        let mut events = vec![40, 255, 124, 18, 0, 0, 1, 0, 1, 5];
        events.extend(b"Fresh");
        events.extend([1, b'2', 4, 255, 255, 255, 1, 100, 144, 60, 100, 64, 33]);
        let mut b = crate::bounded_sequence::tests::synthetic(&events);
        let project = parse_project_166(&b).unwrap();
        let at = project.sequences[0].track_pairs[0]
            .primary
            .payload
            .range
            .start;
        b[at + 2..at + 6].copy_from_slice(&2u32.to_be_bytes());
        b
    }
    #[test]
    fn current_binding_refuses_forged_timing_ranges_routing_ordinals_and_ownership() {
        let b = fixture();
        let q = qualify_at(&b, 0, 0, 0).unwrap();
        let w = walk_bounded_mixed_events(
            &b,
            MixedEventBounds {
                event_range: q.routing.event_range.clone(),
            },
            Default::default(),
        )
        .unwrap();
        let current = &w.items[0];
        assert_eq!(
            qualify_current(&b, 0, 0, current, &q.routing, 0).unwrap(),
            q
        );
        for field in 0..8 {
            let mut changed = current.clone();
            let MixedEventItem::PatchToNote(t) = &mut changed else {
                panic!()
            };
            match field {
                0 => t.patch_position += 1,
                1 => t.first_note_position += 1,
                2 => t.representation_range.start += 1,
                3 => t.patch.representation_range.end -= 1,
                4 => t.first_note.representation_range.start += 1,
                5 => t.patch.position.value += 1,
                6 => t.patch.post_pc_timing_component.value += 1,
                _ => t.first_note.duration.value += 1,
            }
            assert_eq!(
                qualify_current(&b, 0, 0, &changed, &q.routing, 0).unwrap_err(),
                NumericProgramRefusal::Binding
            );
        }
        for field in 0..6 {
            let mut route = q.routing.clone();
            match field {
                0 => route.midi_channel = 5,
                1 => route.instrument_candidate = 2,
                2 => route.pair_ordinal = 1,
                3 => route.descriptor_range.start += 1,
                4 => route.event_range.end -= 1,
                _ => route.association_range.start += 1,
            }
            assert_eq!(
                qualify_current(&b, 0, 0, current, &route, 0).unwrap_err(),
                NumericProgramRefusal::Binding
            );
        }
        for ordinal in [1, 2, u64::MAX] {
            assert!(qualify_current(&b, 0, 0, current, &q.routing, ordinal).is_err());
        }
        assert!(qualify_current(&b, 1, 0, current, &q.routing, 0).is_err());
        assert!(qualify_current(&b, 0, 1, current, &q.routing, 0).is_err());
        let mut other = b.clone();
        other[q.routing.type10_payload_range.start + 27] = 5;
        assert!(qualify_current(&other, 0, 0, current, &q.routing, 0).is_err());
        // Range and ordinal are both mandatory: another Track's borrowed item
        // cannot stand in for the source-derived composition.
        let mut changed = current.clone();
        let MixedEventItem::PatchToNote(t) = &mut changed else {
            panic!()
        };
        t.patch.program_change.value = 63;
        assert!(qualify_current(&b, 0, 0, &changed, &q.routing, 0).is_err());
    }
    #[test]
    fn checked_patch_note_and_release_arithmetic_refuses_overflow() {
        let b = fixture();
        let q = qualify_at(&b, 0, 0, 0).unwrap();
        let w = walk_bounded_mixed_events(
            &b,
            MixedEventBounds {
                event_range: q.routing.event_range.clone(),
            },
            Default::default(),
        )
        .unwrap();
        let MixedEventItem::PatchToNote(t) = &w.items[0] else {
            panic!()
        };
        assert_eq!(
            validate_transition(t, true, u32::MAX),
            Err(NumericProgramRefusal::Timing)
        );
        let mut bad = t.clone();
        bad.patch_position = u32::MAX;
        bad.patch.position.value = u32::MAX;
        assert_eq!(
            validate_transition(&bad, true, 0),
            Err(NumericProgramRefusal::Timing)
        );
        bad.patch.post_pc_timing_component.value = 0;
        bad.first_note_position = u32::MAX;
        assert_eq!(
            validate_transition(&bad, true, 0),
            Err(NumericProgramRefusal::Timing)
        );
    }
}
