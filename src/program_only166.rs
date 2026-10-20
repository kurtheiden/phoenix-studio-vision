//! Bounded I/00 Program-only Patch-to-explicit-Note coverage. No export authority.
use crate::{
    bounded_routing::{decode_bounded_routing, BoundedRoutingResolution},
    controller166::{qualify_track as qualify_controllers, RoutingValidatedController},
    midi_export::{DecodedExportEvent, DecodedExportEventKind},
    mixed_event::{walk_bounded_mixed_events, MixedEventBounds, MixedEventItem, MixedEventKind},
    sequence_container::parse_project_166,
};
use std::ops::Range;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramOnlyProvenance {
    pub sequence_ordinal: usize,
    pub descriptor_ordinal: usize,
    pub pair_ordinal: usize,
    pub primary_range: Range<usize>,
    pub secondary_range: Range<usize>,
    pub composition_range: Range<usize>,
    pub composition_bytes: Vec<u8>,
    pub patch_range: Range<usize>,
    pub note_range: Range<usize>,
    pub pre_name_range: Range<usize>,
    pub pre_name_bytes: [u8; 5],
    pub post_name_range: Range<usize>,
    pub post_name_bytes: [u8; 8],
    pub patch_timing_range: Range<usize>,
    pub patch_interval: u32,
    pub note_timing_range: Range<usize>,
    pub note_interval: u32,
    pub previous_tick: u32,
    pub patch_tick: u32,
    pub note_tick: u32,
    pub logical_source_ordinal: u64,
    pub raw_source_object_ordinal: u64,
    pub declared_source_objects: u32,
}
/// Owned immutable source bytes, routing and validated Controller prefix survive handoff.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgramOnlyPatch {
    provenance: ProgramOnlyProvenance,
    routing: BoundedRoutingResolution,
    prefix: Vec<RoutingValidatedController>,
    events: [DecodedExportEvent; 2],
}
impl ProgramOnlyPatch {
    pub fn provenance(&self) -> &ProgramOnlyProvenance {
        &self.provenance
    }
    pub fn routing(&self) -> &BoundedRoutingResolution {
        &self.routing
    }
    pub fn controller_prefix(&self) -> &[RoutingValidatedController] {
        &self.prefix
    }
    pub fn decoded_export_events(&self) -> [DecodedExportEvent; 2] {
        self.events.clone()
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProgramOnlyRefusal {
    Structure,
    Routing,
    Accounting,
    Prefix,
    Composition,
    Wrapper,
    MidiData,
}

/// Only the first Patch after zero or more ordinary FF41 Controllers qualifies.
/// All facts are derived again from this immutable source, not caller assertions.
pub fn qualify_track(
    bytes: &[u8],
    sequence_ordinal: usize,
    pair_ordinal: usize,
) -> Result<ProgramOnlyPatch, ProgramOnlyRefusal> {
    let project = parse_project_166(bytes).map_err(|_| ProgramOnlyRefusal::Structure)?;
    let s = project
        .sequences
        .get(sequence_ordinal)
        .ok_or(ProgramOnlyRefusal::Structure)?;
    let pair = s
        .track_pairs
        .get(pair_ordinal)
        .ok_or(ProgramOnlyRefusal::Structure)?;
    // This private proof constructor also validates exhaustive ownership, the
    // complete walk and logical events + retained contexts == declared objects.
    let controllers = qualify_controllers(bytes, sequence_ordinal, pair_ordinal)
        .map_err(|_| ProgramOnlyRefusal::Accounting)?;
    let route = decode_bounded_routing(bytes)
        .map_err(|_| ProgramOnlyRefusal::Routing)?
        .into_iter()
        .find(|r| {
            r.structural_ordinal == sequence_ordinal && r.descriptor_ordinal == pair_ordinal + 2
        })
        .ok_or(ProgramOnlyRefusal::Routing)?
        .result
        .map_err(|_| ProgramOnlyRefusal::Routing)?;
    let walk = walk_bounded_mixed_events(
        bytes,
        MixedEventBounds {
            event_range: route.event_range.clone(),
        },
        Default::default(),
    )
    .map_err(|_| ProgramOnlyRefusal::Structure)?;
    let mut prefix = Vec::new();
    let mut previous = 0;
    for (index, item) in walk.items.iter().enumerate() {
        if let MixedEventItem::Event(e) = item {
            if let MixedEventKind::Controller(c) = &e.event {
                let proof = controllers
                    .iter()
                    .find(|p| {
                        p.provenance().logical_source_ordinal == index as u64
                            && p.provenance().representation_range == c.record_range
                    })
                    .ok_or(ProgramOnlyRefusal::Prefix)?;
                prefix.push(proof.clone());
                previous = e.position;
                continue;
            }
        }
        let MixedEventItem::PatchToNote(t) = item else {
            return Err(ProgramOnlyRefusal::Composition);
        };
        let p = &t.patch;
        if t.initial_context.is_some()
            || t.context.is_some()
            || t.final_timing.is_some()
            || !p.pre_note_context.bytes.is_empty()
            || p.note_status.value != 0x90
            || t.first_note.status.map(|s| s.value) != Some(0x90)
        {
            return Err(ProgramOnlyRefusal::Composition);
        }
        let program = p.program_change.value;
        if program > 127
            || t.first_note.pitch.value > 127
            || t.first_note.attack_velocity.value > 127
            || t.first_note.release_velocity.value > 127
        {
            return Err(ProgramOnlyRefusal::MidiData);
        }
        let pre = [0, 0, program, 0, program];
        let post = p.post_name_context.bytes;
        if p.pre_name_context.bytes != pre
            || post.len() != 8
            || post[0] != 3
            || post[1] != b'I'
            || !post[2..4].iter().all(u8::is_ascii_digit)
            || post[4..] != [4, 255, 255, 255]
            || p.payload_range.end != p.program_change.offset + 1
            || p.payload_range.len() != usize::from(p.payload_length.value)
            || p.name.bytes.len() != usize::from(p.name_length.value)
            || p.payload_range.len() != 5 + 1 + p.name.bytes.len() + 8 + 1
        {
            return Err(ProgramOnlyRefusal::Wrapper);
        }
        if previous.checked_add(p.position.value) != Some(t.patch_position)
            || t.patch_position
                .checked_add(p.post_pc_timing_component.value)
                != Some(t.first_note_position)
        {
            return Err(ProgramOnlyRefusal::Composition);
        }
        let ordinal = index as u64;
        let events = [
            DecodedExportEvent {
                absolute_position: t.patch_position,
                source_ordinal: ordinal,
                source_range: Some(p.representation_range.clone()),
                kind: DecodedExportEventKind::TargetedProgram {
                    channel: route.midi_channel,
                    program,
                },
            },
            DecodedExportEvent::from_note_body(t.first_note_position, ordinal + 1, &t.first_note),
        ];
        return Ok(ProgramOnlyPatch {
            provenance: ProgramOnlyProvenance {
                sequence_ordinal,
                descriptor_ordinal: pair_ordinal + 2,
                pair_ordinal,
                primary_range: pair.primary.record_range.clone(),
                secondary_range: pair.secondary.record_range.clone(),
                composition_range: t.representation_range.clone(),
                composition_bytes: bytes[t.representation_range.clone()].to_vec(),
                patch_range: p.representation_range.clone(),
                note_range: t.first_note.representation_range.clone(),
                pre_name_range: p.pre_name_context.range.clone(),
                pre_name_bytes: pre,
                post_name_range: p.post_name_context.range.clone(),
                post_name_bytes: post.try_into().map_err(|_| ProgramOnlyRefusal::Wrapper)?,
                patch_timing_range: p.position.range.clone(),
                patch_interval: p.position.value,
                note_timing_range: p.post_pc_timing_component.range.clone(),
                note_interval: p.post_pc_timing_component.value,
                previous_tick: previous,
                patch_tick: t.patch_position,
                note_tick: t.first_note_position,
                logical_source_ordinal: ordinal,
                raw_source_object_ordinal: ordinal,
                // qualify_controllers already checked this count against the
                // complete walk, including when there are no Controllers.
                declared_source_objects: u32::from_be_bytes(
                    pair.primary.payload.bytes[2..6]
                        .try_into()
                        .map_err(|_| ProgramOnlyRefusal::Accounting)?,
                ),
            },
            routing: route,
            prefix,
            events,
        });
    }
    Err(ProgramOnlyRefusal::Composition)
}
