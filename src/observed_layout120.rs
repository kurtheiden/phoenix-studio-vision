//! Bounded structural observations for the evidenced 75 + 120 * count layout.
//! These are not semantic sequences, track ownership, or export permission.

use std::ops::Range;

use crate::patch::{LocatedByte, LocatedBytes};
use crate::sequence_container::{
    parse_root_record_stream, parse_sequence_candidate, FramedRecord, RootHeader, RootRecordError,
};

/// Exact observed bytes, with no decoded role, version or flag meaning.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Observed120MarkerForm {
    FeFf,
    FfFf,
}

impl Observed120MarkerForm {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        match bytes {
            [0xfe, 0xff] => Some(Self::FeFf),
            [0xff, 0xff] => Some(Self::FfFf),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Observed120Candidate<'a> {
    pub record_index: usize,
    pub candidate_range: Range<usize>,
    pub count: LocatedByte,
    pub marker_form: Observed120MarkerForm,
    pub observed_marker: LocatedBytes<'a>,
    pub observed_name: LocatedBytes<'a>,
    pub terminator_offset: usize,
    // Opaque bytes, explicitly NOT required to be zero or interpreted as padding.
    pub after_terminator: LocatedBytes<'a>,
    pub following_record_range: Range<usize>,
}

impl Observed120Candidate<'_> {
    pub(crate) fn name_as_utf8(&self) -> Option<&str> {
        std::str::from_utf8(self.observed_name.bytes).ok()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Observed120Project<'a> {
    pub root_header: RootHeader<'a>,
    pub consumed_range: Range<usize>,
    pub candidates: Vec<Observed120Candidate<'a>>,
}

impl Observed120Project<'_> {
    pub(crate) fn diagnostic_summary(&self) -> String {
        let empty = self
            .candidates
            .iter()
            .filter(|c| c.observed_name.bytes.is_empty())
            .count();
        let undecoded = self
            .candidates
            .iter()
            .filter(|c| c.name_as_utf8().is_none())
            .count();
        let fe_count = self
            .candidates
            .iter()
            .filter(|c| c.marker_form == Observed120MarkerForm::FeFf)
            .count();
        let ff_count = self
            .candidates
            .iter()
            .filter(|c| c.marker_form == Observed120MarkerForm::FfFf)
            .count();
        format!("Bounded structural observation: {} 120-layout structural candidates, including {empty} empty observed name spans and {undecoded} non-UTF-8 observed name spans. Semantic sequence ownership, readiness and export capability are not established. Observed marker forms: fe ff {fe_count}, ff ff {ff_count}.", self.candidates.len())
    }
}

// Internal errors only; these do not introduce public machine-readable categories.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ObservationError {
    Root(RootRecordError),
    NoMatch,
    MalformedCandidate { record_index: usize },
    AmbiguousLayout,
    MixedLayouts,
}

impl ObservationError {
    pub(crate) fn is_layout_conflict(&self) -> bool {
        matches!(self, Self::AmbiguousLayout | Self::MixedLayouts)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum CandidateLayout {
    Observed120,
    Established166,
}

fn discriminate(
    observed120: bool,
    established166: bool,
) -> Result<CandidateLayout, ObservationError> {
    match (observed120, established166) {
        (true, false) => Ok(CandidateLayout::Observed120),
        (false, true) => Ok(CandidateLayout::Established166),
        (true, true) => Err(ObservationError::AmbiguousLayout),
        (false, false) => Err(ObservationError::NoMatch),
    }
}

/// Independently tests positive local 120 guards and the unchanged full 166
/// candidate validator. A 166 refusal is never itself a 120 selection predicate.
pub(crate) fn observe_project_120(
    bytes: &[u8],
) -> Result<Observed120Project<'_>, ObservationError> {
    let root = parse_root_record_stream(bytes).map_err(ObservationError::Root)?;
    let mut candidates = Vec::new();
    let mut established166 = false;
    for (index, record) in root.records.iter().enumerate() {
        if record.record_type.value != 0x01 {
            continue;
        }
        let observed = observe_candidate(bytes, record, root.records.get(index + 1), index);
        let semantic166 = parse_sequence_candidate(bytes, &root.records, index).is_ok();
        match discriminate(observed.is_some(), semantic166) {
            Ok(CandidateLayout::Observed120) => {
                // The discriminator established presence, but never unwrap malformed input.
                if let Some(candidate) = observed {
                    candidates.push(candidate);
                }
            }
            Ok(CandidateLayout::Established166) => established166 = true,
            Err(ObservationError::NoMatch) => {
                return Err(ObservationError::MalformedCandidate {
                    record_index: index,
                });
            }
            Err(error) => return Err(error),
        }
    }
    if candidates.is_empty() {
        return Err(ObservationError::NoMatch);
    }
    if established166 {
        return Err(ObservationError::MixedLayouts);
    }
    Ok(Observed120Project {
        root_header: root.root_header,
        consumed_range: root.consumed_range,
        candidates,
    })
}

fn local_range(candidate: &Range<usize>, relative: Range<usize>) -> Option<Range<usize>> {
    let start = candidate.start.checked_add(relative.start)?;
    let end = candidate.start.checked_add(relative.end)?;
    (start <= end && end <= candidate.end).then_some(start..end)
}

fn observe_candidate<'a>(
    bytes: &'a [u8],
    record: &FramedRecord<'_>,
    following: Option<&FramedRecord<'_>>,
    record_index: usize,
) -> Option<Observed120Candidate<'a>> {
    let range = &record.record_range;
    let raw = bytes.get(range.clone())?;
    if record.record_type.value != 0x01 || raw.first().copied()? != 0x01 {
        return None;
    }
    // Verify header/range consistency even though the root framer supplies them.
    let declared = u32::from_be_bytes(raw.get(1..5)?.try_into().ok()?);
    if usize::try_from(declared).ok()?.checked_add(5)? != raw.len() {
        return None;
    }
    let count = *raw.get(5)?;
    // PROPOSED DEFENSIVE GUARD, not decoded track/count semantics. No subtraction
    // or ownership claims; observed counts are NOT whitelisted.
    if count < 2 || 120usize.checked_mul(usize::from(count))?.checked_add(75)? != raw.len() {
        return None;
    }
    // Established local corroborating bytes; their semantics remain unknown.
    if raw.get(15..22)? != [0, 0, 0, 0, 0, 0, 1] || !matches!(raw.get(22)?, 0x80 | 0x88) {
        return None;
    }
    let marker_range = local_range(range, 41..43)?;
    let marker_bytes = bytes.get(marker_range.clone())?;
    let marker_form = Observed120MarkerForm::from_bytes(marker_bytes)?;
    let following = following?;
    if following.record_type.value != 0x07 || following.record_range.start != range.end {
        return None;
    }
    let window = local_range(range, 23..41)?; // +41 is excluded, not a capacity claim.
    let name_window = bytes.get(window.clone())?;
    let length = name_window.iter().position(|byte| *byte == 0)?;
    let terminator_offset = window.start.checked_add(length)?;
    let name_range = window.start..terminator_offset;
    let after_range = terminator_offset.checked_add(1)?..window.end;
    Some(Observed120Candidate {
        record_index,
        candidate_range: range.clone(),
        count: LocatedByte {
            value: count,
            offset: range.start.checked_add(5)?,
        },
        marker_form,
        observed_marker: LocatedBytes {
            bytes: marker_bytes,
            range: marker_range,
        },
        observed_name: LocatedBytes {
            bytes: bytes.get(name_range.clone())?,
            range: name_range,
        },
        terminator_offset,
        after_terminator: LocatedBytes {
            bytes: bytes.get(after_range.clone())?,
            range: after_range,
        },
        following_record_range: following.record_range.clone(),
    })
}

#[cfg(test)]
#[path = "observed_layout120_tests.rs"]
pub(crate) mod tests;
