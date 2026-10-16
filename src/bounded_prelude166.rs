//! Exact source Note-inventory coverage, not universal type09 semantics.
use crate::bounded_routing::{decode_bounded_routing, BoundedRoutingResolution};
use crate::mixed_event::{
    walk_bounded_mixed_events, MixedEventBounds, MixedEventItem, MixedEventKind,
};
use crate::sequence_container::{parse_project_166, TrackAssociations};
use std::{collections::BTreeSet, ops::Range};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PreludeRefusal {
    Structure,
    Ownership,
    Framing,
    Header,
    Count,
    Entry,
    PitchOrder,
    TrackCoverage,
    Routing,
    UnmatchedOrAmbiguous,
    TrackOrder,
}

/// Private construction binds uninterpreted bytes to independently recovered
/// source Notes and routing. Nothing in this proof becomes a MIDI event.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreludeCoverage {
    record_range: Range<usize>,
    record: Vec<u8>,
    payload_range: Range<usize>,
    payload: Vec<u8>,
    routing: BoundedRoutingResolution,
    pitches: BTreeSet<u8>,
}
impl PreludeCoverage {
    pub fn record_range(&self) -> Range<usize> {
        self.record_range.clone()
    }
    pub fn record_bytes(&self) -> &[u8] {
        &self.record
    }
    pub fn payload_range(&self) -> Range<usize> {
        self.payload_range.clone()
    }
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
    pub fn routing(&self) -> &BoundedRoutingResolution {
        &self.routing
    }
    pub fn pitches(&self) -> &BTreeSet<u8> {
        &self.pitches
    }
}

/// CCA-077 exact-match subset. Reparse one source for ownership, framing,
/// complete event walks and Instrument lookup; never accept caller bindings.
/// Surplus/stale inventories (including Experiment021) deliberately refuse.
pub fn classify_prelude_coverage(
    bytes: &[u8],
    ordinal: usize,
) -> Result<Vec<PreludeCoverage>, PreludeRefusal> {
    let project = parse_project_166(bytes).map_err(|_| PreludeRefusal::Structure)?;
    let s = project
        .sequences
        .get(ordinal)
        .ok_or(PreludeRefusal::Structure)?;
    let TrackAssociations::Ordinal(bindings) = &s.track_associations else {
        return Err(PreludeRefusal::Ownership);
    };
    if bindings.len() != s.track_pairs.len()
        || bindings.len() != s.track_descriptors().len()
        || bindings
            .iter()
            .enumerate()
            .any(|(i, b)| b.pair_ordinal != i || b.descriptor_ordinal != i + 2)
    {
        return Err(PreludeRefusal::Ownership);
    }
    if s.prelude_records.is_empty() {
        return Ok(vec![]);
    }
    let mut end = s.name_record.record_range.end;
    for r in &s.prelude_records {
        if r.record_type.value != 9 || r.record_range.start != end {
            return Err(PreludeRefusal::Framing);
        }
        end = r.record_range.end;
    }
    if end != s.meter_primary.record_range.start {
        return Err(PreludeRefusal::Framing);
    }
    let routes = decode_bounded_routing(bytes).map_err(|_| PreludeRefusal::Structure)?;
    let mut candidates = Vec::new();
    for binding in bindings {
        let pair = &s.track_pairs[binding.pair_ordinal];
        let bounds = s
            .validated_track_event_bounds(binding.pair_ordinal)
            .map_err(|_| PreludeRefusal::TrackCoverage)?
            .event_range;
        let declared = pair
            .primary
            .payload
            .bytes
            .get(2..6)
            .ok_or(PreludeRefusal::TrackCoverage)?;
        let declared = u32::from_be_bytes(
            declared
                .try_into()
                .map_err(|_| PreludeRefusal::TrackCoverage)?,
        ) as usize;
        if bounds.is_empty() && declared == 0 {
            continue;
        }
        let route = routes
            .iter()
            .find(|r| {
                r.structural_ordinal == ordinal
                    && r.descriptor_ordinal == binding.descriptor_ordinal
            })
            .ok_or(PreludeRefusal::Routing)?
            .result
            .as_ref()
            .map_err(|_| PreludeRefusal::Routing)?;
        let w = walk_bounded_mixed_events(
            bytes,
            MixedEventBounds {
                event_range: bounds.clone(),
            },
            Default::default(),
        )
        .map_err(|_| PreludeRefusal::TrackCoverage)?;
        let mut notes = BTreeSet::new();
        let mut contexts = 0;
        for item in &w.items {
            match item {
                MixedEventItem::PatchToNote(p) => {
                    notes.insert(p.first_note.pitch.value);
                    contexts +=
                        usize::from(p.initial_context.is_some()) + usize::from(p.context.is_some());
                }
                MixedEventItem::Event(e) => match &e.event {
                    MixedEventKind::Note(n) => {
                        notes.insert(n.pitch.value);
                    }
                    MixedEventKind::ContextMediatedNote(n) => {
                        notes.insert(n.note.pitch.value);
                        contexts += 1;
                    }
                    MixedEventKind::DoubleContextMediatedNote(n) => {
                        notes.insert(n.note.pitch.value);
                        contexts += 2;
                    }
                    MixedEventKind::ContextMediatedController(_) => contexts += 1,
                    MixedEventKind::MidiController(c) => {
                        contexts += usize::from(c.context.is_some())
                    }
                    _ => {}
                },
                MixedEventItem::Patch(_) => {}
            }
        }
        if w.consumed_range != bounds
            || w.logical_event_count() + contexts != declared
            || notes.iter().any(|p| *p > 127)
        {
            return Err(PreludeRefusal::TrackCoverage);
        }
        candidates.push((route, notes));
    }
    let mut result = Vec::new();
    let mut previous_pair = None;
    for r in &s.prelude_records {
        let p = r.payload.bytes;
        if p.len() < 4 || !matches!(&p[..2], [0, 1] | [0, 2]) {
            return Err(PreludeRefusal::Header);
        }
        let n = usize::from(u16::from_be_bytes([p[2], p[3]]));
        if n == 0 || p.len() != 4 + 4 * n {
            return Err(PreludeRefusal::Count);
        }
        let key = u16::from_be_bytes([p[4], p[5]]);
        let mut pitches = BTreeSet::new();
        let mut previous = None;
        for e in p[4..].chunks_exact(4) {
            if u16::from_be_bytes([e[0], e[1]]) != key || e[2] > 127 || e[3] != 15 {
                return Err(PreludeRefusal::Entry);
            }
            if previous.is_some_and(|v| v <= e[2]) {
                return Err(PreludeRefusal::PitchOrder);
            }
            previous = Some(e[2]);
            pitches.insert(e[2]);
        }
        let matches: Vec<_> = candidates
            .iter()
            .filter(|(route, notes)| {
                u16::from(route.instrument_candidate) == key && *notes == pitches
            })
            .collect();
        let [matched] = matches.as_slice() else {
            return Err(PreludeRefusal::UnmatchedOrAmbiguous);
        };
        let (route, _) = matched;
        if previous_pair.is_some_and(|v| v >= route.pair_ordinal) {
            return Err(PreludeRefusal::TrackOrder);
        }
        previous_pair = Some(route.pair_ordinal);
        result.push(PreludeCoverage {
            record_range: r.record_range.clone(),
            record: bytes[r.record_range.clone()].to_vec(),
            payload_range: r.payload.range.clone(),
            payload: p.to_vec(),
            routing: (*route).clone(),
            pitches,
        });
    }
    Ok(result)
}
