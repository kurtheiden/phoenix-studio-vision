//! Owned Descriptor166 F0/07 source framing, independent of generic MIDI data.
//! Private construction reparses ownership and exact bounds from the same source.
use std::ops::Range;

use crate::mixed_event::{
    walk_bounded_mixed_events, MixedEventBounds, MixedEventItem, MixedEventKind, MixedEventWalk,
};
use crate::sequence_container::{parse_project_166, TrackAssociations};
use crate::track7::decode_7bit_be_vlq;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SysEx07Provenance {
    pub sequence_ordinal: usize,
    pub descriptor_ordinal: usize,
    pub pair_ordinal: usize,
    pub primary_range: Range<usize>,
    pub secondary_range: Range<usize>,
    pub event_bounds: Range<usize>,
    pub declared_source_objects: u32,
    pub representation_range: Range<usize>,
    pub representation_bytes: Vec<u8>,
    pub status_offset: usize,
    pub stored_length_offset: usize,
    pub stored_length: u8,
    pub counted_payload_range: Range<usize>,
    pub counted_payload: [u8; 7],
    pub framing_offset: usize,
    pub framing_value: u8,
    pub midi_payload_range: Range<usize>,
    pub timing_range: Range<usize>,
    pub timing_bytes: Vec<u8>,
    pub interval: u32,
    pub absolute_tick: u32,
    pub logical_source_ordinal: u64,
    pub raw_source_object_ordinal: u64,
    pub successor_timing_range: Range<usize>,
    pub successor_interval: u32,
    pub successor_status_offset: usize,
    pub successor_status: u8,
}

/// A source-derived six-byte payload with immutable ownership/framing proof.
/// No public constructor accepts arbitrary MIDI bytes or caller-provided facts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceFramedSysEx07 {
    provenance: SysEx07Provenance,
    midi_payload: [u8; 6],
}
impl SourceFramedSysEx07 {
    pub fn provenance(&self) -> &SysEx07Provenance {
        &self.provenance
    }
    pub fn midi_payload(&self) -> &[u8; 6] {
        &self.midi_payload
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum SysEx166Refusal {
    Ownership,
    Bounds,
    Walk,
    Framing,
    Successor,
    Count,
    Overflow,
}

/// No partial qualification is returned if any SysEx in the owned body refuses.
pub(crate) fn qualify_track(
    source: &[u8],
    sequence_ordinal: usize,
    pair_ordinal: usize,
    walk: &MixedEventWalk<'_>,
) -> Result<Vec<SourceFramedSysEx07>, SysEx166Refusal> {
    let project = parse_project_166(source).map_err(|_| SysEx166Refusal::Ownership)?;
    let sequence = project
        .sequences
        .get(sequence_ordinal)
        .ok_or(SysEx166Refusal::Ownership)?;
    let TrackAssociations::Ordinal(bindings) = &sequence.track_associations else {
        return Err(SysEx166Refusal::Ownership);
    };
    let binding = bindings
        .iter()
        .find(|b| b.pair_ordinal == pair_ordinal)
        .ok_or(SysEx166Refusal::Ownership)?;
    if bindings.len() != sequence.track_pairs.len()
        || bindings.len() != sequence.track_descriptors().len()
        || bindings
            .iter()
            .enumerate()
            .any(|(i, b)| b.pair_ordinal != i || b.descriptor_ordinal != i + 2)
    {
        return Err(SysEx166Refusal::Ownership);
    }
    let pair = sequence
        .track_pairs
        .get(pair_ordinal)
        .ok_or(SysEx166Refusal::Ownership)?;
    let bounds = pair
        .validated_event_bounds()
        .map_err(|_| SysEx166Refusal::Bounds)?
        .event_range;
    if walk.event_range != bounds || walk.consumed_range != bounds {
        return Err(SysEx166Refusal::Bounds);
    }
    let actual = walk_bounded_mixed_events(
        source,
        MixedEventBounds {
            event_range: bounds.clone(),
        },
        Default::default(),
    )
    .map_err(|_| SysEx166Refusal::Walk)?;
    if &actual != walk {
        return Err(SysEx166Refusal::Walk);
    }
    let declared = u32::from_be_bytes(
        pair.primary.payload.bytes[2..6]
            .try_into()
            .map_err(|_| SysEx166Refusal::Count)?,
    );
    let mut logical = 0u64;
    let mut raw = 0u64;
    let mut out = vec![];
    for item in &walk.items {
        if let MixedEventItem::Event(e) = item {
            if let MixedEventKind::SysEx(s) = &e.event {
                let payload: [u8; 7] = s
                    .payload
                    .bytes
                    .try_into()
                    .map_err(|_| SysEx166Refusal::Framing)?;
                if s.status.value != 0xf0
                    || s.payload_length.value != 7
                    || s.framing.is_some()
                    || payload[0] != 0
                    || payload[6] != 0xf7
                    || payload[1..6].iter().any(|b| *b > 127)
                {
                    return Err(SysEx166Refusal::Framing);
                }
                let next = s.representation_range.end;
                if next >= bounds.end {
                    return Err(SysEx166Refusal::Successor);
                }
                let timing = decode_7bit_be_vlq(source, next, bounds.end)
                    .map_err(|_| SysEx166Refusal::Successor)?;
                let status = next
                    .checked_add(timing.bytes_consumed)
                    .ok_or(SysEx166Refusal::Overflow)?;
                let value = *source
                    .get(status)
                    .filter(|_| status < bounds.end)
                    .ok_or(SysEx166Refusal::Successor)?;
                if !matches!(value, 0xf0 | 0x90) {
                    return Err(SysEx166Refusal::Successor);
                }
                out.push(SourceFramedSysEx07 {
                    midi_payload: payload[1..]
                        .try_into()
                        .map_err(|_| SysEx166Refusal::Framing)?,
                    provenance: SysEx07Provenance {
                        sequence_ordinal,
                        descriptor_ordinal: binding.descriptor_ordinal,
                        pair_ordinal,
                        primary_range: pair.primary.record_range.clone(),
                        secondary_range: pair.secondary.record_range.clone(),
                        event_bounds: bounds.clone(),
                        declared_source_objects: declared,
                        representation_range: s.representation_range.clone(),
                        representation_bytes: source[s.representation_range.clone()].to_vec(),
                        status_offset: s.status.offset,
                        stored_length_offset: s.payload_length.offset,
                        stored_length: s.payload_length.value,
                        counted_payload_range: s.payload.range.clone(),
                        counted_payload: payload,
                        framing_offset: s.payload.range.start,
                        framing_value: payload[0],
                        midi_payload_range: s.payload.range.start + 1..s.payload.range.end,
                        timing_range: s.timing_delta.range.clone(),
                        timing_bytes: s.timing_delta.bytes.to_vec(),
                        interval: s.timing_delta.value,
                        absolute_tick: e.position,
                        logical_source_ordinal: logical,
                        raw_source_object_ordinal: raw,
                        successor_timing_range: next..status,
                        successor_interval: timing.value,
                        successor_status_offset: status,
                        successor_status: value,
                    },
                });
            }
        }
        let contexts = match item {
            MixedEventItem::PatchToNote(p) => {
                usize::from(p.initial_context.is_some()) + usize::from(p.context.is_some())
            }
            MixedEventItem::Event(e) => match &e.event {
                MixedEventKind::ContextMediatedNote(_) => 1,
                MixedEventKind::DoubleContextMediatedNote(_) => 2,
                MixedEventKind::MidiController(c) => usize::from(c.context.is_some()),
                _ => 0,
            },
            _ => 0,
        };
        logical = logical
            .checked_add(item.logical_event_count() as u64)
            .ok_or(SysEx166Refusal::Overflow)?;
        raw = raw
            .checked_add(item.logical_event_count() as u64)
            .and_then(|n| n.checked_add(contexts as u64))
            .ok_or(SysEx166Refusal::Overflow)?;
    }
    Ok(out)
}

#[cfg(test)]
#[path = "sysex166_tests.rs"]
mod tests;
