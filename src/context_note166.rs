//! Owned Descriptor166 single-context Note coverage, not universal FF60 semantics.
use crate::{
    bounded_routing::{decode_bounded_routing, BoundedRoutingResolution, ContextKind},
    controller166::qualify_track as qualify_controllers,
    midi_export::DecodedExportEvent,
    mixed_event::{walk_bounded_mixed_events, MixedEventBounds, MixedEventItem, MixedEventKind},
    sequence_container::parse_project_166,
};
use std::ops::Range;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContextNoteProvenance {
    pub sequence_ordinal: usize,
    pub descriptor_ordinal: usize,
    pub pair_ordinal: usize,
    pub primary_range: Range<usize>,
    pub secondary_range: Range<usize>,
    pub event_bounds: Range<usize>,
    pub declared_source_objects: u32,
    pub logical_track_objects: usize,
    pub retained_track_contexts: usize,
    pub composition_range: Range<usize>,
    pub composition_bytes: Vec<u8>,
    pub context_range: Range<usize>,
    pub context_bytes: Vec<u8>,
    pub payload_range: Range<usize>,
    pub payload: Vec<u8>,
    pub leading_timing_range: Range<usize>,
    pub leading_timing_bytes: Vec<u8>,
    pub leading_interval: u32,
    pub final_timing_range: Range<usize>,
    pub final_timing_bytes: Vec<u8>,
    pub final_interval: u32,
    pub note_range: Range<usize>,
    pub note_bytes: Vec<u8>,
    pub absolute_tick: u32,
    pub ending_tick: u32,
    pub logical_source_ordinal: u64,
    pub raw_context_ordinal: u64,
    pub raw_note_ordinal: u64,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OwnedContextNote {
    provenance: ContextNoteProvenance,
    routing: BoundedRoutingResolution,
    event: DecodedExportEvent,
}
impl OwnedContextNote {
    pub fn provenance(&self) -> &ContextNoteProvenance {
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
pub enum ContextNoteRefusal {
    Ownership,
    Accounting,
    Routing,
    Walk,
    Form,
    MidiData,
    Ending,
}
/// Every independent single-context Note in the completely owned walk must
/// qualify. Other decoded families are counted, not granted handoff authority.
pub fn qualify_track(
    bytes: &[u8],
    sequence_ordinal: usize,
    pair_ordinal: usize,
) -> Result<Vec<OwnedContextNote>, ContextNoteRefusal> {
    let project = parse_project_166(bytes).map_err(|_| ContextNoteRefusal::Ownership)?;
    let s = project
        .sequences
        .get(sequence_ordinal)
        .ok_or(ContextNoteRefusal::Ownership)?;
    let pair = s
        .track_pairs
        .get(pair_ordinal)
        .ok_or(ContextNoteRefusal::Ownership)?;
    // Reuse the existing exhaustive ownership, same-source routing, full-walk
    // and declared raw-object accounting gate, even on tracks without FF41.
    qualify_controllers(bytes, sequence_ordinal, pair_ordinal)
        .map_err(|_| ContextNoteRefusal::Accounting)?;
    let route = decode_bounded_routing(bytes)
        .map_err(|_| ContextNoteRefusal::Routing)?
        .into_iter()
        .find(|r| {
            r.structural_ordinal == sequence_ordinal && r.descriptor_ordinal == pair_ordinal + 2
        })
        .ok_or(ContextNoteRefusal::Routing)?
        .result
        .map_err(|_| ContextNoteRefusal::Routing)?;
    let bounds = s
        .validated_track_event_bounds(pair_ordinal)
        .map_err(|_| ContextNoteRefusal::Walk)?
        .event_range;
    let walk = walk_bounded_mixed_events(
        bytes,
        MixedEventBounds {
            event_range: bounds.clone(),
        },
        Default::default(),
    )
    .map_err(|_| ContextNoteRefusal::Walk)?;
    if walk.consumed_range != bounds
        || route.event_range != bounds
        || route.consumed_range != bounds
    {
        return Err(ContextNoteRefusal::Walk);
    }
    let declared = u32::from_be_bytes(
        pair.primary.payload.bytes[2..6]
            .try_into()
            .map_err(|_| ContextNoteRefusal::Accounting)?,
    );
    let logical_track = walk.logical_event_count();
    let retained = (declared as usize)
        .checked_sub(logical_track)
        .ok_or(ContextNoteRefusal::Accounting)?;
    let mut out = Vec::new();
    let mut logical = 0u64;
    let mut raw = 0u64;
    for item in &walk.items {
        if let MixedEventItem::Event(e) = item {
            if let MixedEventKind::ContextMediatedNote(n) = &e.event {
                let c = &n.context;
                let p = c.payload.bytes;
                let len = usize::from(c.payload_length.value);
                let form = (p.starts_with(&[0x57, 0x7f, 0]) && matches!(len, 6..=8))
                    || (p.starts_with(&[0x23, 0x60, 0]) && matches!(len, 6 | 7));
                if !form
                    || p.len() != len
                    || n.note.status.map(|s| s.value) != Some(0x90)
                    || !route.validated_contexts.iter().any(|v| {
                        v.kind == ContextKind::Ff60 && v.range == c.payload.range && v.bytes == p
                    })
                {
                    return Err(ContextNoteRefusal::Form);
                }
                if n.note.pitch.value > 127
                    || n.note.attack_velocity.value > 127
                    || n.note.release_velocity.value > 127
                {
                    return Err(ContextNoteRefusal::MidiData);
                }
                let ending = e
                    .position
                    .checked_add(n.note.duration.value)
                    .ok_or(ContextNoteRefusal::Ending)?;
                out.push(OwnedContextNote {
                    provenance: ContextNoteProvenance {
                        sequence_ordinal,
                        descriptor_ordinal: pair_ordinal + 2,
                        pair_ordinal,
                        primary_range: pair.primary.record_range.clone(),
                        secondary_range: pair.secondary.record_range.clone(),
                        event_bounds: bounds.clone(),
                        declared_source_objects: declared,
                        logical_track_objects: logical_track,
                        retained_track_contexts: retained,
                        composition_range: n.representation_range.clone(),
                        composition_bytes: bytes[n.representation_range.clone()].to_vec(),
                        context_range: c.range.clone(),
                        context_bytes: bytes[c.range.clone()].to_vec(),
                        payload_range: c.payload.range.clone(),
                        payload: p.to_vec(),
                        leading_timing_range: n.leading_timing.range.clone(),
                        leading_timing_bytes: n.leading_timing.bytes.to_vec(),
                        leading_interval: n.leading_timing.value,
                        final_timing_range: n.final_timing.range.clone(),
                        final_timing_bytes: n.final_timing.bytes.to_vec(),
                        final_interval: n.final_timing.value,
                        note_range: n.note.representation_range.clone(),
                        note_bytes: bytes[n.note.representation_range.clone()].to_vec(),
                        absolute_tick: e.position,
                        ending_tick: ending,
                        logical_source_ordinal: logical,
                        raw_context_ordinal: raw,
                        raw_note_ordinal: raw + 1,
                    },
                    routing: route.clone(),
                    event: DecodedExportEvent::from_note_body(e.position, logical, &n.note),
                });
            }
        }
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
    if raw != u64::from(declared) {
        return Err(ContextNoteRefusal::Accounting);
    }
    Ok(out)
}
