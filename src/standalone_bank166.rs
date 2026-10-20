//! Owned single-event handoff for CCA-071 standalone banks. No new bank or inclusion rules.
use crate::{
    bounded_patch_translation::{
        qualify_descriptor166_bank_patch, qualify_descriptor166_bank_patch_at, retained_contexts,
        BoundedBankPatch,
    },
    bounded_routing::{decode_bounded_routing, BoundedRoutingResolution},
    controller166::qualify_track as qualify_accounting,
    midi_export::{DecodedExportEvent, DecodedExportEventKind},
    mixed_event::{walk_bounded_mixed_events, MixedEventBounds, MixedEventItem},
    sequence_container::parse_project_166,
};
use std::ops::Range;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StandaloneBankProvenance {
    pub sequence_ordinal: usize,
    pub descriptor_ordinal: usize,
    pub pair_ordinal: usize,
    pub primary_range: Range<usize>,
    pub secondary_range: Range<usize>,
    pub event_bounds: Range<usize>,
    pub declared_source_objects: u32,
    pub logical_track_objects: usize,
    pub retained_track_contexts: usize,
    pub patch_range: Range<usize>,
    pub patch_bytes: Vec<u8>,
    pub marker_range: Range<usize>,
    pub payload_length_offset: usize,
    pub payload_range: Range<usize>,
    pub pre_name_range: Range<usize>,
    pub pre_name_bytes: [u8; 5],
    pub name_length_offset: usize,
    pub name_range: Range<usize>,
    pub post_name_range: Range<usize>,
    pub post_name_bytes: Vec<u8>,
    pub program_offset: usize,
    pub timing_range: Range<usize>,
    pub timing_bytes: Vec<u8>,
    pub previous_tick: u32,
    pub interval: u32,
    pub absolute_tick: u32,
    pub logical_source_ordinal: u64,
    pub raw_source_object_ordinal: u64,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OwnedStandaloneBankPatch {
    provenance: StandaloneBankProvenance,
    routing: BoundedRoutingResolution,
    event: DecodedExportEvent,
}
impl OwnedStandaloneBankPatch {
    pub fn provenance(&self) -> &StandaloneBankProvenance {
        &self.provenance
    }
    pub fn routing(&self) -> &BoundedRoutingResolution {
        &self.routing
    }
    pub fn decoded_export_event(&self) -> DecodedExportEvent {
        self.event.clone()
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StandaloneBankRefusal {
    Qualification,
    Ownership,
    Accounting,
    Walk,
    Placement,
    Binding,
    Timing,
}
fn check_timing(previous: u32, interval: u32, tick: u32) -> Result<(), StandaloneBankRefusal> {
    if previous.checked_add(interval) != Some(tick) {
        return Err(StandaloneBankRefusal::Timing);
    }
    Ok(())
}
fn bind(
    bytes: &[u8],
    sequence_ordinal: usize,
    pair_ordinal: usize,
    current: &MixedEventItem<'_>,
    routing: &BoundedRoutingResolution,
    ordinal: u64,
    q: &BoundedBankPatch<'_>,
) -> Result<OwnedStandaloneBankPatch, StandaloneBankRefusal> {
    use StandaloneBankRefusal::*;
    let MixedEventItem::Patch(p) = current else {
        return Err(Placement);
    };
    let event = q.decoded_export_event();
    if q.source() != current
        || q.routing() != routing
        || event.source_ordinal != ordinal
        || event.source_range.as_ref() != Some(&p.patch.representation_range)
        || event.absolute_position != p.position
        || !matches!(event.kind, DecodedExportEventKind::TargetedPatch { channel, .. } if channel == routing.midi_channel)
    {
        return Err(Binding);
    }
    // Revalidate exhaustive pair/descriptor ownership and the complete raw
    // object count, including all contexts outside this standalone core.
    qualify_accounting(bytes, sequence_ordinal, pair_ordinal).map_err(|_| Accounting)?;
    let source_route = decode_bounded_routing(bytes)
        .map_err(|_| Ownership)?
        .into_iter()
        .find(|r| {
            r.structural_ordinal == sequence_ordinal && r.descriptor_ordinal == pair_ordinal + 2
        })
        .ok_or(Ownership)?
        .result
        .map_err(|_| Ownership)?;
    if &source_route != routing {
        return Err(Binding);
    }
    let project = parse_project_166(bytes).map_err(|_| Ownership)?;
    let s = project.sequences.get(sequence_ordinal).ok_or(Ownership)?;
    let pair = s.track_pairs.get(pair_ordinal).ok_or(Ownership)?;
    let bounds = s
        .validated_track_event_bounds(pair_ordinal)
        .map_err(|_| Walk)?
        .event_range;
    if routing.pair_ordinal != pair_ordinal
        || s.descriptors.get(pair_ordinal + 2).map(|d| &d.range) != Some(&routing.descriptor_range)
        || routing.event_range != bounds
        || routing.consumed_range != bounds
    {
        return Err(Ownership);
    }
    let walk = walk_bounded_mixed_events(
        bytes,
        MixedEventBounds {
            event_range: bounds.clone(),
        },
        Default::default(),
    )
    .map_err(|_| Walk)?;
    if walk.consumed_range != bounds {
        return Err(Walk);
    }
    let mut previous = 0;
    let mut logical = 0;
    let mut raw = 0;
    let mut found = false;
    for item in &walk.items {
        if logical == ordinal {
            if item != current {
                return Err(Binding);
            }
            found = true;
            break;
        }
        previous = match item {
            MixedEventItem::Event(e) => e.position,
            MixedEventItem::Patch(p) => p.position,
            MixedEventItem::PatchToNote(t) => t.first_note_position,
        };
        logical += item.logical_event_count() as u64;
        raw += (item.logical_event_count() + retained_contexts(item)) as u64;
    }
    if !found || raw != q.raw_patch_ordinal() {
        return Err(Binding);
    }
    let p = &p.patch;
    check_timing(previous, p.position.value, event.absolute_position)?;
    let declared = u32::from_be_bytes(
        pair.primary.payload.bytes[2..6]
            .try_into()
            .map_err(|_| Accounting)?,
    );
    let logical_track_objects = walk.logical_event_count();
    let retained_track_contexts = (declared as usize)
        .checked_sub(logical_track_objects)
        .ok_or(Accounting)?;
    Ok(OwnedStandaloneBankPatch {
        provenance: StandaloneBankProvenance {
            sequence_ordinal,
            descriptor_ordinal: pair_ordinal + 2,
            pair_ordinal,
            primary_range: pair.primary.record_range.clone(),
            secondary_range: pair.secondary.record_range.clone(),
            event_bounds: bounds,
            declared_source_objects: declared,
            logical_track_objects,
            retained_track_contexts,
            patch_range: p.representation_range.clone(),
            patch_bytes: bytes[p.representation_range.clone()].to_vec(),
            marker_range: p.marker_range.clone(),
            payload_length_offset: p.payload_length.offset,
            payload_range: p.payload_range.clone(),
            pre_name_range: p.pre_name_context.range.clone(),
            pre_name_bytes: p.pre_name_context.bytes.try_into().map_err(|_| Binding)?,
            name_length_offset: p.name_length.offset,
            name_range: p.name.range.clone(),
            post_name_range: p.post_name_context.range.clone(),
            post_name_bytes: p.post_name_context.bytes.to_vec(),
            program_offset: p.program_change.offset,
            timing_range: p.position.range.clone(),
            timing_bytes: p.position.bytes.to_vec(),
            previous_tick: previous,
            interval: p.position.value,
            absolute_tick: event.absolute_position,
            logical_source_ordinal: logical,
            raw_source_object_ordinal: raw,
        },
        routing: routing.clone(),
        event,
    })
}

/// Builder seam: a supplied route/item must equal immutable source-derived
/// evidence. Neither caller assertions nor neighboring events grant authority.
pub(crate) fn qualify_current(
    bytes: &[u8],
    sequence_ordinal: usize,
    pair_ordinal: usize,
    current: &MixedEventItem<'_>,
    routing: &BoundedRoutingResolution,
    ordinal: u64,
) -> Result<OwnedStandaloneBankPatch, StandaloneBankRefusal> {
    let q = qualify_descriptor166_bank_patch_at(bytes, sequence_ordinal, pair_ordinal, ordinal)
        .map_err(|_| StandaloneBankRefusal::Qualification)?;
    bind(
        bytes,
        sequence_ordinal,
        pair_ordinal,
        current,
        routing,
        ordinal,
        &q,
    )
}
/// Read-only qualification is independent of saved-mute inclusion and export.
pub fn qualify_track(
    bytes: &[u8],
    sequence_ordinal: usize,
    pair_ordinal: usize,
) -> Result<OwnedStandaloneBankPatch, StandaloneBankRefusal> {
    let q = qualify_descriptor166_bank_patch(bytes, sequence_ordinal, pair_ordinal)
        .map_err(|_| StandaloneBankRefusal::Qualification)?;
    bind(
        bytes,
        sequence_ordinal,
        pair_ordinal,
        q.source(),
        q.routing(),
        q.decoded_export_event().source_ordinal,
        &q,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> Vec<u8> {
        let mut e = vec![
            9, 255, 65, 5, 0, 1, 0, 7, 90, 37, 255, 124, 19, 0, 0, 42, 8, 42, 5,
        ];
        e.extend(b"Fresh");
        e.extend([
            2, b'9', b'9', 4, 255, 80, 0, 42, 0, 255, 65, 5, 0, 1, 0, 7, 90, 0, 144, 60, 100, 64,
            12,
        ]);
        let mut b = crate::bounded_sequence::tests::synthetic(&e);
        let at = parse_project_166(&b).unwrap().sequences[0].track_pairs[0]
            .primary
            .payload
            .range
            .start;
        b[at + 2..at + 6].copy_from_slice(&4u32.to_be_bytes());
        b
    }
    #[test]
    fn exact_current_binding_refuses_forged_source_fields_route_channel_and_ordinals() {
        let b = fixture();
        let q = qualify_descriptor166_bank_patch(&b, 0, 0).unwrap();
        let current = q.source();
        let route = q.routing();
        let owned = qualify_current(&b, 0, 0, current, route, 1).unwrap();
        assert_eq!(
            (
                owned.provenance.previous_tick,
                owned.provenance.absolute_tick,
                owned.provenance.raw_source_object_ordinal
            ),
            (9, 46, 1)
        );
        for field in 0..9 {
            let mut changed = current.clone();
            let MixedEventItem::Patch(p) = &mut changed else {
                panic!()
            };
            match field {
                0 => p.position += 1,
                1 => p.patch.position.value += 1,
                2 => p.patch.representation_range.start += 1,
                3 => p.patch.representation_range.end -= 1,
                4 => p.patch.payload_range.end -= 1,
                5 => p.patch.program_change.value += 1,
                6 => p.patch.name.range.start += 1,
                7 => p.patch.post_name_context.range.end -= 1,
                _ => p.patch.position.range.start += 1,
            }
            assert_eq!(
                qualify_current(&b, 0, 0, &changed, route, 1).unwrap_err(),
                StandaloneBankRefusal::Binding
            );
        }
        let mut bytes = b.clone();
        bytes[owned.provenance.name_range.start] = b'Z';
        assert!(bind(&bytes, 0, 0, current, route, 1, &q).is_err());
        for field in 0..7 {
            let mut changed = route.clone();
            match field {
                0 => changed.midi_channel = 5,
                1 => changed.instrument_candidate = 2,
                2 => changed.pair_ordinal = 1,
                3 => changed.descriptor_range.start += 1,
                4 => changed.event_range.end -= 1,
                5 => changed.type10_record_range.start += 1,
                _ => changed.association_range.start += 1,
            }
            assert_eq!(
                qualify_current(&b, 0, 0, current, &changed, 1).unwrap_err(),
                StandaloneBankRefusal::Binding
            );
        }
        for ordinal in [0, 2, u64::MAX] {
            assert!(qualify_current(&b, 0, 0, current, route, ordinal).is_err());
        }
        assert!(qualify_current(&b, 1, 0, current, route, 1).is_err());
        assert!(qualify_current(&b, 0, 1, current, route, 1).is_err());
        let mut other = b.clone();
        other[route.type10_payload_range.start + 27] = 5;
        let other_q = qualify_descriptor166_bank_patch(&other, 0, 0).unwrap();
        assert!(bind(&b, 0, 0, other_q.source(), other_q.routing(), 1, &other_q).is_err());
        let walk = walk_bounded_mixed_events(
            &b,
            MixedEventBounds {
                event_range: route.event_range.clone(),
            },
            Default::default(),
        )
        .unwrap();
        assert!(qualify_current(&b, 0, 0, &walk.items[2], route, 1).is_err());
    }
    #[test]
    fn checked_timing_refuses_overflow_and_forged_absolute_positions() {
        assert_eq!(
            check_timing(u32::MAX, 1, 0),
            Err(StandaloneBankRefusal::Timing)
        );
        assert_eq!(check_timing(9, 37, 47), Err(StandaloneBankRefusal::Timing));
        assert_eq!(check_timing(9, 37, 46), Ok(()));
    }
}
