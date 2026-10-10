//! Incomplete source-layout semantic associations, never export capabilities.
//! Ordinal ownership is a bounded engineering inference. Event-bound probes
//! below measure the existing 166 machinery; they do not authorize source events.

use std::ops::Range;

use crate::observed_layout120::{observe_project_120, ObservationError, Observed120Candidate};
use crate::patch::LocatedBytes;
use crate::sequence_container::{
    parse_root_record_stream, FramedRecord, TrackEventBoundsError, TrackRecordPair,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum AssociationRefusal {
    UnsupportedTrailer,
    Bounds,
    MissingLabel { ordinal: usize },
    SpecialPositions,
    IncompleteNeighborhood,
    Cardinality { slots: usize, pairs: usize },
    SparseMetadata,
    Overlap,
}

#[derive(Debug)]
pub(crate) struct Source120Slot<'a> {
    pub ordinal: usize,
    /// Correspondence view, not exclusive ownership of all bytes in the view.
    pub view_range: Range<usize>,
    pub label: LocatedBytes<'a>,
}

#[derive(Debug)]
pub(crate) struct Source120Pair<'a> {
    pub primary: FramedRecord<'a>,
    pub secondary: FramedRecord<'a>,
}

impl Source120Pair<'_> {
    /// Probe only: do not return a Descriptor166 TrackEventBounds authority token.
    fn probe_existing_bounds(&self) -> Result<(), TrackEventBoundsError> {
        let start = self.primary.payload.range.start.checked_add(14).ok_or(
            TrackEventBoundsError::ArithmeticOverflow {
                detail: "event start",
            },
        )?;
        TrackRecordPair {
            pair_ordinal: 0,
            primary: self.primary.clone(),
            secondary: self.secondary.clone(),
            candidate_event_start: start,
            event_containing_range: start..self.primary.payload.range.end,
        }
        .validated_event_bounds()
        .map(|_| ())
    }
}

/// Semantic evidence only: no secondary, event-bound token or export authority.
#[derive(Debug)]
pub(crate) struct PrimaryOnlyBinding<'a> {
    pub slot: Source120Slot<'a>,
    pub local_primary_ordinal: usize,
    pub primary: FramedRecord<'a>,
    pub zero_prefix_field: LocatedBytes<'a>,
    pub declared_count: u32,
    pub declared_count_range: Range<usize>,
    pub event_range: Range<usize>,
    pub tail: LocatedBytes<'a>,
}

#[derive(Debug)]
enum Source120Unit<'a> {
    Paired(Source120Pair<'a>),
    PrimaryOnly(FramedRecord<'a>),
}

#[derive(Debug)]
pub(crate) struct LeadingSpecialBinding<'a> {
    pub slot: Source120Slot<'a>,
    pub pair: Source120Pair<'a>,
}

#[derive(Debug)]
pub(crate) struct OrdinaryBinding<'a> {
    pub slot: Source120Slot<'a>,
    pub pair: Source120Pair<'a>,
    pub local_pair_ordinal: usize,
    pub existing_bounds_probe: Result<(), TrackEventBoundsError>,
}

/// Legitimate ordinary metadata slot with structurally established absent event data.
/// This does not mean that all event-empty tracks have this state.
#[derive(Debug)]
pub(crate) struct NoEventDataTrack<'a> {
    pub slot: Source120Slot<'a>,
    pub state_context: LocatedBytes<'a>,
}

/// Opaque, independently framed metadata; its payload never selects event pairs.
#[derive(Debug)]
pub(crate) struct SparseMetadata120<'a> {
    pub track_ordinal: usize,
    pub presence: LocatedBytes<'a>,
    pub record: FramedRecord<'a>,
}

#[derive(Debug)]
pub(crate) struct Source120SequenceAssociation<'a> {
    pub candidate_record_index: usize,
    pub sequence_range: Range<usize>,
    pub source_name: LocatedBytes<'a>,
    /// Exact framed prelude, retained without interpreting its opaque field.
    pub prelude: FramedRecord<'a>,
    pub metadata: Vec<SparseMetadata120<'a>>,
    pub leading_special: [LeadingSpecialBinding<'a>; 2],
    pub ordinary: Vec<OrdinaryBinding<'a>>,
    pub primary_only: Vec<PrimaryOnlyBinding<'a>>,
    /// Retained zero-event tracks; merge by slot.ordinal to recover source order.
    pub no_event_data: Vec<NoEventDataTrack<'a>>,
}

#[derive(Debug)]
pub(crate) struct CandidateAssociation<'a> {
    pub observation: Observed120Candidate<'a>,
    pub association: Result<Source120SequenceAssociation<'a>, AssociationRefusal>,
}

#[derive(Debug)]
pub(crate) struct Source120Bridge<'a> {
    pub candidates: Vec<CandidateAssociation<'a>>,
}

impl Source120Bridge<'_> {
    pub(crate) fn diagnostic_details(&self) -> Vec<String> {
        let mut details = Vec::new();
        for candidate in &self.candidates {
            match &candidate.association {
                Err(error) => details.push(format!(
                    "120-layout record {} association refused: {error:?}",
                    candidate.observation.record_index
                )),
                Ok(sequence) => {
                    details.push(format!(
                        "Opaque type07 prelude record {:?}, payload {:?}: {:?}; no decoded semantic meaning",
                        sequence.prelude.record_range, sequence.prelude.payload.range,
                        sequence.prelude.payload.bytes
                    ));
                    details.push(format!(
                        "120-layout record {}: incomplete semantic sequence {:?}, source name span {:?}; association basis: bounded ordinal inference",
                        sequence.candidate_record_index, sequence.sequence_range, sequence.source_name.range
                    ));
                    for metadata in &sequence.metadata {
                        details.push(format!(
                            "Bounded sparse metadata Track {}: presence {:?} {:?}, record {:?}, payload {:?}; payload opaque, not event ownership",
                            metadata.track_ordinal, metadata.presence.range, metadata.presence.bytes,
                            metadata.record.record_range, metadata.record.payload.range
                        ));
                    }
                    for special in &sequence.leading_special {
                        details.push(format!(
                            "Leading special position {}: view {:?}, label {:?}, pair {:?}/{:?}; source conductor event authority unestablished",
                            special.slot.ordinal, special.slot.view_range, special.slot.label.range,
                            special.pair.primary.record_range, special.pair.secondary.record_range
                        ));
                    }
                    for track in &sequence.no_event_data {
                        details.push(format!(
                            "Ordinary unit {}: view {:?}, label {:?}, ZERO-STATE context {:?} {:?}; retained zero-event track without a pair (no export authority)",
                            track.slot.ordinal, track.slot.view_range, track.slot.label.range,
                            track.state_context.range, track.state_context.bytes
                        ));
                    }
                    for item in &sequence.primary_only {
                        details.push(format!(
                            "Ordinary unit {}: view {:?}, label {:?}, local primary {} {:?}; primary-only, no secondary; zero prefix {:?} {:?}, declared {} at {:?}, body {:?}, tail {:?} {:?}; no event/export authority",
                            item.slot.ordinal, item.slot.view_range, item.slot.label.range,
                            item.local_primary_ordinal, item.primary.record_range,
                            item.zero_prefix_field.range, item.zero_prefix_field.bytes,
                            item.declared_count, item.declared_count_range, item.event_range,
                            item.tail.range, item.tail.bytes
                        ));
                    }
                    for item in &sequence.ordinary {
                        details.push(format!(
                            "Ordinary unit {}: view {:?}, label {:?}, local pair {} {:?}/{:?}; existing-bounds probe {:?} (no source event authority)",
                            item.slot.ordinal, item.slot.view_range, item.slot.label.range,
                            item.local_pair_ordinal, item.pair.primary.record_range,
                            item.pair.secondary.record_range, item.existing_bounds_probe
                        ));
                    }
                }
            }
        }
        details
    }

    pub(crate) fn diagnostic_summary(&self) -> String {
        let mut sequences = 0;
        let mut tracks = 0;
        let mut matched = 0;
        let mut unsupported = 0;
        let mut no_event_data = 0;
        let mut primary_only = 0;
        for candidate in &self.candidates {
            if let Ok(sequence) = &candidate.association {
                sequences += 1;
                tracks += sequence.ordinary.len();
                primary_only += sequence.primary_only.len();
                no_event_data += sequence.no_event_data.len();
                for item in &sequence.ordinary {
                    if item.existing_bounds_probe.is_ok() {
                        matched += 1;
                    } else {
                        unsupported += 1;
                    }
                }
            }
        }
        let additional = if primary_only == 0 {
            String::new()
        } else {
            format!(
                "; {primary_only} primary-only ordinary bindings without event/export authority"
            )
        };
        format!("Incomplete source-layout semantic bridge: {sequences} sequence associations, {tracks} ordinary pair bindings, {no_event_data} retained zero-event tracks without pairs; existing event-bound probes matched {matched}, refused {unsupported}; {} candidate associations refused. Source event/conductor authority, readiness and export capability remain unestablished.{additional}", self.candidates.len() - sequences)
    }
}

pub(crate) fn associate_project_120(bytes: &[u8]) -> Result<Source120Bridge<'_>, ObservationError> {
    associate_project_120_mode(bytes, false)
}

/// Broader diagnostic ownership only. Export callers retain the paired-only entry point.
pub(crate) fn associate_semantic_project_120(
    bytes: &[u8],
) -> Result<Source120Bridge<'_>, ObservationError> {
    associate_project_120_mode(bytes, true)
}

fn associate_project_120_mode(
    bytes: &[u8],
    allow_primary_only: bool,
) -> Result<Source120Bridge<'_>, ObservationError> {
    let observations = observe_project_120(bytes)?;
    let root = parse_root_record_stream(bytes).map_err(ObservationError::Root)?;
    let mut candidates: Vec<_> = observations
        .candidates
        .into_iter()
        .map(|observation| {
            let association = if allow_primary_only {
                associate_candidate_mode(bytes, &root.records, &observation, true)
            } else {
                associate_candidate(bytes, &root.records, &observation)
            };
            CandidateAssociation {
                observation,
                association,
            }
        })
        .collect();
    // Refused siblings still delimit ownership: an accepted run may not swallow
    // any observed candidate, even one outside this narrower semantic subprofile.
    let starts: Vec<_> = candidates
        .iter()
        .map(|c| c.observation.candidate_range.start)
        .collect();
    let accepted_ranges: Vec<_> = candidates
        .iter()
        .filter_map(|c| {
            c.association
                .as_ref()
                .ok()
                .map(|s| s.sequence_range.clone())
        })
        .collect();
    for candidate in &mut candidates {
        if let Ok(sequence) = &candidate.association {
            let range = &sequence.sequence_range;
            if starts.iter().any(|s| *s > range.start && *s < range.end)
                || accepted_ranges.iter().any(|other| {
                    other.start != range.start && range.start < other.end && other.start < range.end
                })
            {
                candidate.association = Err(AssociationRefusal::Overlap);
            }
        }
    }
    Ok(Source120Bridge { candidates })
}

/// Preserve the original opaque ten-byte payload contract. Longer forms are
/// limited to the evidenced counted printable field and eight-byte zero suffix.
fn supported_prelude(record: &FramedRecord<'_>) -> bool {
    if record.record_type.value != 7 {
        return false;
    }
    if record.record_range.len() == 15 {
        return true;
    }
    let payload = record.payload.bytes;
    if !matches!(record.record_range.len(), 27 | 35)
        || payload.len() < 2
        || payload[0] != 0
        || !matches!(payload[1], 12 | 20)
    {
        return false;
    }
    let field_end = 2 + usize::from(payload[1]);
    payload.len() == field_end + 8
        && payload[2..field_end]
            .iter()
            .all(|byte| (0x20..=0x7e).contains(byte))
        && payload[field_end..].iter().all(|byte| *byte == 0)
}

/// Additional records form a sparse list, distinct from the initial prelude.
/// Applicable only after the existing positive 120-profile and prelude guards.
fn associate_sparse_metadata<'a>(
    bytes: &'a [u8],
    records: &[FramedRecord<'a>],
    candidate: &Observed120Candidate<'a>,
    cursor: &mut usize,
) -> Result<Vec<SparseMetadata120<'a>>, AssociationRefusal> {
    let mut metadata = Vec::new();
    // Preserve the existing no-additional-record grammar, including opaque windows.
    if records.get(*cursor).map(|r| r.record_type.value) != Some(7) {
        return Ok(metadata);
    }
    let mut next = *cursor;
    for ordinal in 0..usize::from(candidate.count.value) {
        let start = ordinal
            .checked_mul(120)
            .and_then(|n| n.checked_add(106 + 85))
            .and_then(|n| candidate.candidate_range.start.checked_add(n))
            .ok_or(AssociationRefusal::Bounds)?;
        let end = start.checked_add(4).ok_or(AssociationRefusal::Bounds)?;
        if end > candidate.candidate_range.end {
            return Err(AssociationRefusal::Bounds);
        }
        // The independent label must terminate before the presence window;
        // overlapping label/selector bytes would be a conflicting grammar.
        let label_start = start.checked_sub(70).ok_or(AssociationRefusal::Bounds)?;
        if !bytes
            .get(label_start..start)
            .ok_or(AssociationRefusal::Bounds)?
            .contains(&0)
        {
            return Err(AssociationRefusal::SparseMetadata);
        }
        let field = bytes.get(start..end).ok_or(AssociationRefusal::Bounds)?;
        if field.iter().all(|v| *v == 0) {
            continue;
        }
        if ordinal < 2 {
            return Err(AssociationRefusal::SparseMetadata);
        }
        let record = records
            .get(next)
            .ok_or(AssociationRefusal::SparseMetadata)?;
        let previous = records
            .get(next.checked_sub(1).ok_or(AssociationRefusal::Bounds)?)
            .ok_or(AssociationRefusal::Bounds)?;
        if record.record_type.value != 7
            || !matches!(record.payload.bytes.len(), 2 | 3 | 6 | 8 | 10 | 12 | 15)
            || record.record_range.start != previous.record_range.end
            || record.record_range.start.checked_add(5) != Some(record.payload.range.start)
            || record.payload.range.end != record.record_range.end
        {
            return Err(AssociationRefusal::SparseMetadata);
        }
        metadata.push(SparseMetadata120 {
            track_ordinal: ordinal,
            presence: LocatedBytes {
                bytes: field,
                range: start..end,
            },
            record: record.clone(),
        });
        next = next.checked_add(1).ok_or(AssociationRefusal::Bounds)?;
    }
    if records.get(next).map(|r| r.record_type.value) == Some(7) {
        return Err(AssociationRefusal::SparseMetadata);
    }
    *cursor = next;
    Ok(metadata)
}

fn associate_candidate<'a>(
    bytes: &'a [u8],
    records: &[FramedRecord<'a>],
    candidate: &Observed120Candidate<'a>,
) -> Result<Source120SequenceAssociation<'a>, AssociationRefusal> {
    associate_candidate_mode(bytes, records, candidate, false)
}

fn associate_candidate_mode<'a>(
    bytes: &'a [u8],
    records: &[FramedRecord<'a>],
    candidate: &Observed120Candidate<'a>,
    allow_primary_only: bool,
) -> Result<Source120SequenceAssociation<'a>, AssociationRefusal> {
    let bounds = || AssociationRefusal::Bounds;
    let following = records
        .get(candidate.record_index.checked_add(1).ok_or_else(bounds)?)
        .ok_or_else(bounds)?;
    // Defensive subprofile boundary, not decoded universal type07 semantics.
    if !supported_prelude(following) {
        return Err(AssociationRefusal::UnsupportedTrailer);
    }
    let mut cursor = candidate.record_index.checked_add(2).ok_or_else(bounds)?;
    let metadata = associate_sparse_metadata(bytes, records, candidate, &mut cursor)?;
    let mut units = Vec::new();
    let terminal = loop {
        let primary = records
            .get(cursor)
            .ok_or(AssociationRefusal::IncompleteNeighborhood)?;
        if primary.record_type.value == 0 {
            break primary;
        }
        let next = records
            .get(cursor.checked_add(1).ok_or_else(bounds)?)
            .ok_or(AssociationRefusal::IncompleteNeighborhood)?;
        if primary.record_type.value != 2 || primary.record_range.end != next.record_range.start {
            return Err(AssociationRefusal::IncompleteNeighborhood);
        }
        if next.record_type.value == 0x29 {
            // A present real pair remains paired, including existing zero-prefix fixtures.
            units.push(Source120Unit::Paired(Source120Pair {
                primary: primary.clone(),
                secondary: next.clone(),
            }));
            cursor = cursor.checked_add(2).ok_or_else(bounds)?;
        } else if allow_primary_only
            && units.len() >= 2
            && matches!(next.record_type.value, 2 | 0)
            && supported_primary_only(primary)
        {
            units.push(Source120Unit::PrimaryOnly(primary.clone()));
            cursor = cursor.checked_add(1).ok_or_else(bounds)?;
        } else {
            return Err(AssociationRefusal::IncompleteNeighborhood);
        }
    };
    let count = usize::from(candidate.count.value);
    let start = candidate.candidate_range.start;
    let mut slots = Vec::new();
    for ordinal in 0..count {
        let view_start = start
            .checked_add(106)
            .and_then(|s| ordinal.checked_mul(120).and_then(|n| s.checked_add(n)))
            .ok_or_else(bounds)?;
        let view_end = view_start.checked_add(120).ok_or_else(bounds)?;
        if view_end > terminal.record_range.end || view_end > bytes.len() {
            return Err(AssociationRefusal::Bounds);
        }
        let label_start = view_start.checked_add(15).ok_or_else(bounds)?;
        // A label may not borrow following framing, despite the +31 view overlap.
        let label_limit = view_end.min(candidate.candidate_range.end);
        let window = bytes.get(label_start..label_limit).ok_or_else(bounds)?;
        let length = window
            .iter()
            .position(|b| *b == 0)
            .ok_or(AssociationRefusal::MissingLabel { ordinal })?;
        let end = label_start.checked_add(length).ok_or_else(bounds)?;
        slots.push(Source120Slot {
            ordinal,
            view_range: view_start..view_end,
            label: LocatedBytes {
                bytes: &bytes[label_start..end],
                range: label_start..end,
            },
        });
    }
    if slots[0].label.bytes != b"Meter Track" || slots[1].label.bytes != b"Tempo Track" {
        return Err(AssociationRefusal::SpecialPositions);
    }
    // EXP039: exact observed ordinary ZERO-STATE composition only. Names and
    // Instrument assignments are not presence predicates; specials never skip.
    let mut states = Vec::new();
    for slot in slots.iter().skip(2) {
        let state_start = slot.label.range.start.checked_sub(39).ok_or_else(bounds)?;
        let state_end = state_start.checked_add(8).ok_or_else(bounds)?;
        if state_start < start || state_end > candidate.candidate_range.end {
            return Err(AssociationRefusal::Bounds);
        }
        let context = bytes.get(state_start..state_end).ok_or_else(bounds)?;
        states.push(
            (context == [0, 0, 4, 0, 0, 4, 0, 0]).then_some(LocatedBytes {
                bytes: context,
                range: state_start..state_end,
            }),
        );
    }
    let consuming_count = count - states.iter().filter(|state| state.is_some()).count();
    if units.len() != consuming_count {
        return Err(AssociationRefusal::Cardinality {
            slots: consuming_count,
            pairs: units.len(),
        });
    }
    // One independently witnessed interruption is the bounded scope, not a format maximum.
    if units
        .iter()
        .filter(|u| matches!(u, Source120Unit::PrimaryOnly(_)))
        .count()
        > 1
    {
        return Err(AssociationRefusal::IncompleteNeighborhood);
    }
    let mut slots = slots.into_iter();
    let mut units = units.into_iter().enumerate();
    let slot0 = slots.next().ok_or_else(bounds)?;
    let slot1 = slots.next().ok_or_else(bounds)?;
    let (_, Source120Unit::Paired(pair0)) = units.next().ok_or_else(bounds)? else {
        return Err(AssociationRefusal::IncompleteNeighborhood);
    };
    let (_, Source120Unit::Paired(pair1)) = units.next().ok_or_else(bounds)? else {
        return Err(AssociationRefusal::IncompleteNeighborhood);
    };
    let mut ordinary = Vec::new();
    let mut primary_only = Vec::new();
    let mut no_event_data = Vec::new();
    for (slot, state) in slots.zip(states) {
        if let Some(state_context) = state {
            no_event_data.push(NoEventDataTrack {
                slot,
                state_context,
            });
        } else {
            let (local_primary_ordinal, unit) = units.next().ok_or_else(bounds)?;
            match unit {
                Source120Unit::Paired(pair) => {
                    let existing_bounds_probe = pair.probe_existing_bounds();
                    ordinary.push(OrdinaryBinding {
                        local_pair_ordinal: local_primary_ordinal,
                        slot,
                        pair,
                        existing_bounds_probe,
                    });
                }
                Source120Unit::PrimaryOnly(primary) => {
                    // supported_primary_only already proves these bounded slice geometries.
                    let payload = primary.payload.bytes;
                    let base = primary.payload.range.start;
                    let tail_start = primary.payload.range.end - 7;
                    primary_only.push(PrimaryOnlyBinding {
                        slot,
                        local_primary_ordinal,
                        zero_prefix_field: LocatedBytes {
                            bytes: &payload[10..14],
                            range: base + 10..base + 14,
                        },
                        declared_count: u32::from_be_bytes(
                            payload[2..6].try_into().map_err(|_| bounds())?,
                        ),
                        declared_count_range: base + 2..base + 6,
                        event_range: base + 14..tail_start,
                        tail: LocatedBytes {
                            bytes: &payload[payload.len() - 7..],
                            range: tail_start..primary.payload.range.end,
                        },
                        primary,
                    });
                }
            }
        }
    }
    Ok(Source120SequenceAssociation {
        candidate_record_index: candidate.record_index,
        sequence_range: start..terminal.record_range.end,
        source_name: candidate.observed_name.clone(),
        prelude: following.clone(),
        metadata,
        leading_special: [
            LeadingSpecialBinding {
                slot: slot0,
                pair: pair0,
            },
            LeadingSpecialBinding {
                slot: slot1,
                pair: pair1,
            },
        ],
        ordinary,
        primary_only,
        no_event_data,
    })
}

/// CCA-038 envelope only; no event decoder or terminal semantics are introduced.
fn supported_primary_only(primary: &FramedRecord<'_>) -> bool {
    let payload = primary.payload.bytes;
    payload.len() >= 21
        && payload[..2] == [0, 1]
        && payload[10..14] == [0; 4]
        && payload[payload.len() - 7] == 0xff
        && payload[payload.len() - 3..] == [0xff, 0x2f, 0]
}

#[cfg(test)]
#[path = "semantic_layout120_tests.rs"]
mod tests;
