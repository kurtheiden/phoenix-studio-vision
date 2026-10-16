//! Terminal standalone I/00 Program-only Patch proof. No inclusion/export authority.
use crate::{
    bounded_routing::{decode_bounded_routing, BoundedRoutingResolution},
    controller166::qualify_track as qualify_controllers,
    midi_export::{DecodedExportEvent, DecodedExportEventKind},
    mixed_event::{walk_bounded_mixed_events, MixedEventBounds, MixedEventItem, MixedEventKind},
    sequence_container::parse_project_166,
};
use std::ops::Range;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TerminalProgramProvenance {
    pub sequence_ordinal: usize,
    pub descriptor_ordinal: usize,
    pub pair_ordinal: usize,
    pub primary_range: Range<usize>,
    pub secondary_range: Range<usize>,
    pub event_bounds: Range<usize>,
    pub track_suffix_range: Range<usize>,
    pub declared_source_objects: u32,
    pub representation_range: Range<usize>,
    pub representation_bytes: Vec<u8>,
    pub pre_name_range: Range<usize>,
    pub pre_name_bytes: [u8; 5],
    pub post_name_range: Range<usize>,
    pub post_name_bytes: [u8; 8],
    pub timing_range: Range<usize>,
    pub previous_tick: u32,
    pub interval: u32,
    pub absolute_tick: u32,
    pub logical_source_ordinal: u64,
    pub raw_source_object_ordinal: u64,
    pub program: u8,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TerminalProgram {
    provenance: TerminalProgramProvenance,
    routing: BoundedRoutingResolution,
}
impl TerminalProgram {
    pub fn provenance(&self) -> &TerminalProgramProvenance {
        &self.provenance
    }
    pub fn routing(&self) -> &BoundedRoutingResolution {
        &self.routing
    }
    pub fn decoded_export_event(&self) -> DecodedExportEvent {
        let p = &self.provenance;
        DecodedExportEvent {
            absolute_position: p.absolute_tick,
            source_ordinal: p.logical_source_ordinal,
            source_range: Some(p.representation_range.clone()),
            kind: DecodedExportEventKind::TargetedProgram {
                channel: self.routing.midi_channel,
                program: p.program,
            },
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TerminalProgramRefusal {
    Structure,
    Accounting,
    Routing,
    Walk,
    Placement,
    Wrapper,
    Program,
    Timing,
}
/// Reparse and prove the complete owning Track; only its final standalone core
/// can qualify. No preceding event gains translation permission from this proof.
pub fn qualify_track(
    bytes: &[u8],
    sequence_ordinal: usize,
    pair_ordinal: usize,
) -> Result<TerminalProgram, TerminalProgramRefusal> {
    let project = parse_project_166(bytes).map_err(|_| TerminalProgramRefusal::Structure)?;
    let s = project
        .sequences
        .get(sequence_ordinal)
        .ok_or(TerminalProgramRefusal::Structure)?;
    let pair = s
        .track_pairs
        .get(pair_ordinal)
        .ok_or(TerminalProgramRefusal::Structure)?;
    // Reuse the unchanged ownership, full-walk, routing, context and declared
    // object-accounting gate. This does not require a Controller to be present.
    qualify_controllers(bytes, sequence_ordinal, pair_ordinal)
        .map_err(|_| TerminalProgramRefusal::Accounting)?;
    let route = decode_bounded_routing(bytes)
        .map_err(|_| TerminalProgramRefusal::Routing)?
        .into_iter()
        .find(|r| {
            r.structural_ordinal == sequence_ordinal && r.descriptor_ordinal == pair_ordinal + 2
        })
        .ok_or(TerminalProgramRefusal::Routing)?
        .result
        .map_err(|_| TerminalProgramRefusal::Routing)?;
    let bounds = s
        .validated_track_event_bounds(pair_ordinal)
        .map_err(|_| TerminalProgramRefusal::Walk)?
        .event_range;
    let w = walk_bounded_mixed_events(
        bytes,
        MixedEventBounds {
            event_range: bounds.clone(),
        },
        Default::default(),
    )
    .map_err(|_| TerminalProgramRefusal::Walk)?;
    if w.consumed_range != bounds || route.consumed_range != bounds || route.event_range != bounds {
        return Err(TerminalProgramRefusal::Walk);
    }
    let Some(MixedEventItem::Patch(event)) = w.items.last() else {
        return Err(TerminalProgramRefusal::Placement);
    };
    let p = &event.patch;
    if p.representation_range.end != bounds.end {
        return Err(TerminalProgramRefusal::Placement);
    }
    let program = p.program_change.value;
    if program > 127 {
        return Err(TerminalProgramRefusal::Program);
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
        return Err(TerminalProgramRefusal::Wrapper);
    }
    let mut previous = 0;
    let mut logical = 0u64;
    let mut raw = 0u64;
    for item in &w.items[..w.items.len() - 1] {
        previous = match item {
            MixedEventItem::Event(e) => e.position,
            MixedEventItem::Patch(p) => p.position,
            MixedEventItem::PatchToNote(p) => p.first_note_position,
        };
        let contexts = match item {
            MixedEventItem::PatchToNote(p) => {
                usize::from(p.initial_context.is_some()) + usize::from(p.context.is_some())
            }
            MixedEventItem::Event(e) => match &e.event {
                MixedEventKind::ContextMediatedNote(_)
                | MixedEventKind::ContextMediatedController(_) => 1,
                MixedEventKind::DoubleContextMediatedNote(_) => 2,
                MixedEventKind::MidiController(c) => usize::from(c.context.is_some()),
                _ => 0,
            },
            _ => 0,
        };
        logical += item.logical_event_count() as u64;
        raw += (item.logical_event_count() + contexts) as u64;
    }
    if previous.checked_add(p.position.value) != Some(event.position) {
        return Err(TerminalProgramRefusal::Timing);
    }
    Ok(TerminalProgram {
        provenance: TerminalProgramProvenance {
            sequence_ordinal,
            descriptor_ordinal: pair_ordinal + 2,
            pair_ordinal,
            primary_range: pair.primary.record_range.clone(),
            secondary_range: pair.secondary.record_range.clone(),
            event_bounds: bounds.clone(),
            track_suffix_range: bounds.end..pair.primary.payload.range.end,
            declared_source_objects: u32::from_be_bytes(
                pair.primary.payload.bytes[2..6]
                    .try_into()
                    .map_err(|_| TerminalProgramRefusal::Accounting)?,
            ),
            representation_range: p.representation_range.clone(),
            representation_bytes: bytes[p.representation_range.clone()].to_vec(),
            pre_name_range: p.pre_name_context.range.clone(),
            pre_name_bytes: pre,
            post_name_range: p.post_name_context.range.clone(),
            post_name_bytes: post
                .try_into()
                .map_err(|_| TerminalProgramRefusal::Wrapper)?,
            timing_range: p.position.range.clone(),
            previous_tick: previous,
            interval: p.position.value,
            absolute_tick: event.position,
            logical_source_ordinal: logical,
            raw_source_object_ordinal: raw,
            program,
        },
        routing: route,
    })
}
