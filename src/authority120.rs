//! Necessary, non-exporting proofs for the established bounded 120 layout.
//! Construction reparses one immutable source; callers cannot supply ownership,
//! walks, routing tables or accounting facts. No application capability is minted.

use crate::bounded_multi_patch::{decode_bounded_multi_patch_track, BoundedMultiPatchTrack};
use crate::bounded_routing::{validate_bounded_routing_contexts, ValidatedContext};
use crate::mixed_event::{
    walk_bounded_120_single_events, walk_bounded_mixed_events, MixedEventBounds, MixedEventItem,
    MixedEventKind, MixedEventWalk,
};
use crate::semantic_layout120::{
    associate_project_120, OrdinaryBinding, Source120SequenceAssociation,
};
use crate::sequence_container::{parse_root_record_stream, FramedRecord, TrackRecordPair};
use std::ops::Range;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Authority120Refusal {
    Observation(String),
    Association(String),
    Bounds,
    EventBounds(String),
    Decode(String),
    Accounting {
        declared: u32,
        logical: usize,
        contexts: usize,
    },
    UnsupportedAccounting,
    AssociationShape,
    RoutingTable,
    Device,
    Channel,
    Context(String),
    MultiPatch(String),
}

/// An owned routing observation, bound by its enclosing source/Track proof.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SingleRoute120 {
    pub association_range: Range<usize>,
    pub instrument_row: u8,
    pub routing_record_range: Range<usize>,
    pub device_record_range: Range<usize>,
    pub channel: u8,
    pub contexts: Vec<ValidatedContext>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrackRoute120 {
    /// Empty paired and legitimate absent-pair Tracks do not invent a channel.
    NoEvents,
    Single(SingleRoute120),
    Multi(BoundedMultiPatchTrack),
}

#[derive(Debug)]
pub struct TrackProof120<'a> {
    ordinal: usize,
    name: &'a [u8],
    pair_ranges: Option<[Range<usize>; 2]>,
    absent_pair_state_range: Option<Range<usize>>,
    declared: Option<u32>,
    contexts: usize,
    walk: Option<MixedEventWalk<'a>>,
    route: TrackRoute120,
}
impl<'a> TrackProof120<'a> {
    pub fn ordinal(&self) -> usize {
        self.ordinal
    }
    pub fn name(&self) -> &'a [u8] {
        self.name
    }
    pub fn pair_ranges(&self) -> Option<&[Range<usize>; 2]> {
        self.pair_ranges.as_ref()
    }
    pub fn absent_pair_state_range(&self) -> Option<Range<usize>> {
        self.absent_pair_state_range.clone()
    }
    pub fn declared(&self) -> Option<u32> {
        self.declared
    }
    pub fn context_records(&self) -> usize {
        self.contexts
    }
    pub fn walk(&self) -> Option<&MixedEventWalk<'a>> {
        self.walk.as_ref()
    }
    pub fn route(&self) -> &TrackRoute120 {
        &self.route
    }
}

/// Same-source ownership/accounting/routing proof, NOT sufficient for export.
/// Conductor, Patch translation, source-order serialization and terminal gates
/// must still be checked by a later builder against this exact immutable source.
#[derive(Debug)]
pub struct SequenceProof120<'a> {
    source: &'a [u8],
    range: Range<usize>,
    name: &'a [u8],
    tracks: Vec<TrackProof120<'a>>,
}
impl<'a> SequenceProof120<'a> {
    pub fn source(&self) -> &'a [u8] {
        self.source
    }
    pub fn range(&self) -> Range<usize> {
        self.range.clone()
    }
    pub fn name(&self) -> &'a [u8] {
        self.name
    }
    pub fn tracks(&self) -> &[TrackProof120<'a>] {
        &self.tracks
    }
}

#[derive(Debug)]
pub struct CandidateProof120<'a> {
    pub ordinal: usize,
    pub name: &'a [u8],
    pub result: Result<SequenceProof120<'a>, Authority120Refusal>,
}

/// Every observed candidate remains visible, including association refusals.
/// Success proves only necessary ownership, accounting and routing premises.
pub fn prove_project120(source: &[u8]) -> Result<Vec<CandidateProof120<'_>>, Authority120Refusal> {
    let bridge = associate_project_120(source)
        .map_err(|e| Authority120Refusal::Observation(format!("{e:?}")))?;
    let root = parse_root_record_stream(source)
        .map_err(|e| Authority120Refusal::Observation(format!("{e:?}")))?;
    Ok(bridge
        .candidates
        .into_iter()
        .enumerate()
        .map(|(ordinal, candidate)| {
            let name = candidate.observation.observed_name.bytes;
            let result = candidate
                .association
                .map_err(|e| Authority120Refusal::Association(format!("{e:?}")))
                .and_then(|s| prove_sequence(source, &root.records, s));
            CandidateProof120 {
                ordinal,
                name,
                result,
            }
        })
        .collect())
}

fn prove_sequence<'a>(
    source: &'a [u8],
    records: &[FramedRecord<'a>],
    s: Source120SequenceAssociation<'a>,
) -> Result<SequenceProof120<'a>, Authority120Refusal> {
    let mut tracks = Vec::new();
    for t in &s.ordinary {
        tracks.push(prove_track(source, records, &s, t)?);
    }
    for t in &s.no_event_data {
        // Association construction alone recognizes the exact established
        // ZERO-STATE; no caller-supplied omitted Track or invented count exists.
        tracks.push(TrackProof120 {
            ordinal: t.slot.ordinal,
            name: t.slot.label.bytes,
            pair_ranges: None,
            absent_pair_state_range: Some(t.state_context.range.clone()),
            declared: None,
            contexts: 0,
            walk: None,
            route: TrackRoute120::NoEvents,
        });
    }
    tracks.sort_by_key(|t| t.ordinal);
    Ok(SequenceProof120 {
        source,
        range: s.sequence_range,
        name: s.source_name.bytes,
        tracks,
    })
}

fn prove_track<'a>(
    source: &'a [u8],
    records: &[FramedRecord<'a>],
    s: &Source120SequenceAssociation<'a>,
    t: &OrdinaryBinding<'a>,
) -> Result<TrackProof120<'a>, Authority120Refusal> {
    let start = t
        .pair
        .primary
        .payload
        .range
        .start
        .checked_add(14)
        .ok_or(Authority120Refusal::Bounds)?;
    // This type is only a bounds-validation helper. The ownership authority
    // comes from the complete 120 association above, never a 166 container.
    let bounds = TrackRecordPair {
        pair_ordinal: t.local_pair_ordinal,
        primary: t.pair.primary.clone(),
        secondary: t.pair.secondary.clone(),
        candidate_event_start: start,
        event_containing_range: start..t.pair.primary.payload.range.end,
    }
    .validated_event_bounds()
    .map_err(|e| Authority120Refusal::EventBounds(format!("{e:?}")))?;
    let raw = t
        .pair
        .primary
        .payload
        .bytes
        .get(2..6)
        .ok_or(Authority120Refusal::Bounds)?;
    let declared = u32::from_be_bytes(raw.try_into().map_err(|_| Authority120Refusal::Bounds)?);
    // Only independently associated ordinary 120 slots with a single-Instrument
    // selector may decode the canonical Program form. Full routing proof below
    // still must succeed before any Track proof is returned; Multi stays strict.
    let label_start = t.slot.label.range.start;
    let association_start = label_start
        .checked_sub(33)
        .ok_or(Authority120Refusal::Bounds)?;
    if association_start < s.sequence_range.start || label_start > t.pair.primary.record_range.start
    {
        return Err(Authority120Refusal::Bounds);
    }
    let single = source.get(association_start) == Some(&1);
    let walker = if single {
        walk_bounded_120_single_events
    } else {
        walk_bounded_mixed_events
    };
    let walk = walker(
        source,
        MixedEventBounds {
            event_range: bounds.event_range.clone(),
        },
        Default::default(),
    )
    .map_err(|e| Authority120Refusal::Decode(format!("{e:?}")))?;
    let contexts = reconcile(&walk, &bounds.event_range, declared)?;
    let route = if walk.items.is_empty() {
        TrackRoute120::NoEvents
    } else {
        let label = &t.slot.label;
        let a_start = label
            .range
            .start
            .checked_sub(33)
            .ok_or(Authority120Refusal::Bounds)?;
        if a_start < s.sequence_range.start || label.range.end > t.pair.primary.record_range.start {
            return Err(Authority120Refusal::Bounds);
        }
        let association_range = a_start..label.range.start;
        let a = source
            .get(association_range.clone())
            .ok_or(Authority120Refusal::Bounds)?;
        if a[0] > 1 {
            TrackRoute120::Multi(
                decode_bounded_multi_patch_track(source, association_range, bounds.event_range)
                    .map_err(|e| Authority120Refusal::MultiPatch(format!("{e:?}")))?,
            )
        } else {
            TrackRoute120::Single(single_route(records, association_range, a, &walk)?)
        }
    };
    Ok(TrackProof120 {
        ordinal: t.slot.ordinal,
        name: t.slot.label.bytes,
        pair_ranges: Some([
            t.pair.primary.record_range.clone(),
            t.pair.secondary.record_range.clone(),
        ]),
        absent_pair_state_range: None,
        declared: Some(declared),
        contexts,
        walk: Some(walk),
        route,
    })
}

fn reconcile(
    walk: &MixedEventWalk<'_>,
    expected: &Range<usize>,
    declared: u32,
) -> Result<usize, Authority120Refusal> {
    if &walk.event_range != expected || &walk.consumed_range != expected {
        return Err(Authority120Refusal::Bounds);
    }
    let mut logical = 0usize;
    let mut contexts = 0usize;
    for item in &walk.items {
        let (l, c) = match item {
            MixedEventItem::Event(e) => (
                1,
                match &e.event {
                    MixedEventKind::ContextMediatedNote(_) => 1,
                    MixedEventKind::DoubleContextMediatedNote(_) => 2,
                    MixedEventKind::SysEx(_) => {
                        return Err(Authority120Refusal::UnsupportedAccounting)
                    }
                    MixedEventKind::Note(_)
                    | MixedEventKind::Program(_)
                    | MixedEventKind::Controller(_)
                    | MixedEventKind::MidiController(_)
                    | MixedEventKind::PitchBend { .. }
                    | MixedEventKind::ChannelPressure { .. } => 0,
                },
            ),
            MixedEventItem::Patch(_) => (1, 0),
            MixedEventItem::PatchToNote(p) => {
                // No general raw-count evidence for an initial Patch context
                // is certified by this subprofile. Preserve fail-closed scope.
                if p.initial_context.is_some() {
                    return Err(Authority120Refusal::UnsupportedAccounting);
                }
                (2, usize::from(p.context.is_some()))
            }
        };
        logical = logical.checked_add(l).ok_or(Authority120Refusal::Bounds)?;
        contexts = contexts.checked_add(c).ok_or(Authority120Refusal::Bounds)?;
    }
    if logical
        .checked_add(contexts)
        .and_then(|n| u32::try_from(n).ok())
        != Some(declared)
    {
        return Err(Authority120Refusal::Accounting {
            declared,
            logical,
            contexts,
        });
    }
    Ok(contexts)
}

fn single_route(
    records: &[FramedRecord<'_>],
    range: Range<usize>,
    a: &[u8],
    walk: &MixedEventWalk<'_>,
) -> Result<SingleRoute120, Authority120Refusal> {
    if a.len() != 33
        || a[0] != 1
        || a[1] != 0
        || !(a[3..].iter().all(|b| *b == 0xff) || a[3..].iter().all(|b| *b == 0))
    {
        return Err(Authority120Refusal::AssociationShape);
    }
    let i = a[2];
    let table: Vec<_> = records
        .iter()
        .filter(|r| r.record_type.value == 0x10)
        .collect();
    if i == 0
        || usize::from(i) >= table.len()
        || table.len() > 99
        || table
            .iter()
            .enumerate()
            .any(|(n, r)| r.payload.bytes.len() != 36 || usize::from(r.payload.bytes[25]) != n)
    {
        return Err(Authority120Refusal::RoutingTable);
    }
    let selected = table[usize::from(i)];
    let p = selected.payload.bytes;
    if p[27] > 15 {
        return Err(Authority120Refusal::Channel);
    }
    let devices: Vec<_> = records
        .iter()
        .filter(|r| r.record_type.value == 0x2a)
        .collect();
    if devices.iter().any(|r| {
        let b = r.payload.bytes;
        b.len() != 40
            || !(1..=32).contains(&b[0])
            || b[1..1 + usize::from(b[0])]
                .iter()
                .all(u8::is_ascii_whitespace)
    }) {
        return Err(Authority120Refusal::Device);
    }
    let matches: Vec<_> = devices
        .into_iter()
        .filter(|r| r.payload.bytes[33] == p[26])
        .collect();
    let [device] = matches.as_slice() else {
        return Err(Authority120Refusal::Device);
    };
    let contexts = validate_bounded_routing_contexts(walk, i)
        .map_err(|e| Authority120Refusal::Context(format!("{e:?}")))?;
    Ok(SingleRoute120 {
        association_range: range,
        instrument_row: i,
        routing_record_range: selected.record_range.clone(),
        device_record_range: device.record_range.clone(),
        channel: p[27] + 1,
        contexts,
    })
}

#[cfg(test)]
#[path = "authority120_tests.rs"]
mod tests;
