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

#[derive(Debug)]
pub(crate) struct Source120SequenceAssociation<'a> {
    pub candidate_record_index: usize,
    pub sequence_range: Range<usize>,
    pub source_name: LocatedBytes<'a>,
    pub leading_special: [LeadingSpecialBinding<'a>; 2],
    pub ordinary: Vec<OrdinaryBinding<'a>>,
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
                        "120-layout record {}: incomplete semantic sequence {:?}, source name span {:?}; association basis: bounded ordinal inference",
                        sequence.candidate_record_index, sequence.sequence_range, sequence.source_name.range
                    ));
                    for special in &sequence.leading_special {
                        details.push(format!(
                            "Leading special position {}: view {:?}, label {:?}, pair {:?}/{:?}; source conductor event authority unestablished",
                            special.slot.ordinal, special.slot.view_range, special.slot.label.range,
                            special.pair.primary.record_range, special.pair.secondary.record_range
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
        for candidate in &self.candidates {
            if let Ok(sequence) = &candidate.association {
                sequences += 1;
                tracks += sequence.ordinary.len();
                for item in &sequence.ordinary {
                    if item.existing_bounds_probe.is_ok() {
                        matched += 1;
                    } else {
                        unsupported += 1;
                    }
                }
            }
        }
        format!("Incomplete source-layout semantic bridge: {sequences} sequence associations, {tracks} ordinary pair bindings; existing event-bound probes matched {matched}, refused {unsupported}; {} candidate associations refused. Source event/conductor authority, readiness and export capability remain unestablished.", self.candidates.len() - sequences)
    }
}

pub(crate) fn associate_project_120(bytes: &[u8]) -> Result<Source120Bridge<'_>, ObservationError> {
    let observations = observe_project_120(bytes)?;
    let root = parse_root_record_stream(bytes).map_err(ObservationError::Root)?;
    let mut candidates: Vec<_> = observations
        .candidates
        .into_iter()
        .map(|observation| {
            let association = associate_candidate(bytes, &root.records, &observation);
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

fn associate_candidate<'a>(
    bytes: &'a [u8],
    records: &[FramedRecord<'a>],
    candidate: &Observed120Candidate<'a>,
) -> Result<Source120SequenceAssociation<'a>, AssociationRefusal> {
    let bounds = || AssociationRefusal::Bounds;
    let following = records
        .get(candidate.record_index.checked_add(1).ok_or_else(bounds)?)
        .ok_or_else(bounds)?;
    // Defensive subprofile boundary, not decoded universal type07 semantics.
    if following.record_range.len() != 15 {
        return Err(AssociationRefusal::UnsupportedTrailer);
    }
    let mut cursor = candidate.record_index.checked_add(2).ok_or_else(bounds)?;
    let mut pairs = Vec::new();
    let terminal = loop {
        let primary = records
            .get(cursor)
            .ok_or(AssociationRefusal::IncompleteNeighborhood)?;
        if primary.record_type.value == 0 {
            break primary;
        }
        let secondary = records
            .get(cursor.checked_add(1).ok_or_else(bounds)?)
            .ok_or(AssociationRefusal::IncompleteNeighborhood)?;
        if primary.record_type.value != 2
            || secondary.record_type.value != 0x29
            || primary.record_range.end != secondary.record_range.start
        {
            return Err(AssociationRefusal::IncompleteNeighborhood);
        }
        pairs.push(Source120Pair {
            primary: primary.clone(),
            secondary: secondary.clone(),
        });
        cursor = cursor.checked_add(2).ok_or_else(bounds)?;
    };
    let count = usize::from(candidate.count.value);
    if pairs.len() != count {
        return Err(AssociationRefusal::Cardinality {
            slots: count,
            pairs: pairs.len(),
        });
    }
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
    let mut bound = slots.into_iter().zip(pairs);
    let (slot0, pair0) = bound.next().ok_or_else(bounds)?;
    let (slot1, pair1) = bound.next().ok_or_else(bounds)?;
    let ordinary = bound
        .map(|(slot, pair)| {
            let existing_bounds_probe = pair.probe_existing_bounds();
            OrdinaryBinding {
                local_pair_ordinal: slot.ordinal,
                slot,
                pair,
                existing_bounds_probe,
            }
        })
        .collect();
    Ok(Source120SequenceAssociation {
        candidate_record_index: candidate.record_index,
        sequence_range: start..terminal.record_range.end,
        source_name: candidate.observed_name.clone(),
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
    })
}

#[cfg(test)]
#[path = "semantic_layout120_tests.rs"]
mod tests;
