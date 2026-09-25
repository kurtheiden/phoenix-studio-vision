//! Bounded saved track-mute interpretation for the evidenced Descriptor166 subset.
//!
//! This is inferred saved state, not an observed UI state or MIDI inclusion policy.
//! No export, profile, readiness, or event-decoding decision consumes this module.
//! See docs/MUTE_INCLUSION_STRUCTURAL_SCOPE.md and Experiments 035/036.

use std::ops::Range;

use crate::mixed_event::{walk_bounded_mixed_events, MixedEventBounds, MixedEventTimingBasis};
use crate::sequence_container::{parse_project_166, Project166Error, TrackAssociations};

const GUARD: [u8; 7] = [0x00, 0x04, 0x00, 0x00, 0x04, 0x01, 0x00];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SavedMuteUnknown {
    UnresolvedBinding,
    BlankOrUnboundedLabel,
    CandidateOutOfBounds,
    InvalidEventBounds,
    EmptyEvents,
    GuardMismatch,
    UnsupportedValue,
    IncompleteEventWalk,
}

/// ON/OFF apply only within the guarded subset; Unknown never means OFF.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SavedMuteState {
    On,
    Off,
    Unknown(SavedMuteUnknown),
}

/// Owned, source-located evidence. Ordinals are zero-based; ranges are half-open.
/// The candidate occupies the preceding descriptor slice, but belongs to the
/// following track under the experimentally supported label-minus-39 rule.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SavedMuteEvidence {
    pub sequence_ordinal: usize,
    pub sequence_name: Vec<u8>,
    pub descriptor_ordinal: usize,
    pub descriptor_range: Range<usize>,
    pub label_bytes: Option<Vec<u8>>,
    pub pair_ordinal: Option<usize>,
    pub candidate_offset: Option<usize>,
    pub candidate_value: Option<u8>,
    pub neighboring_guard: Option<[u8; 7]>,
    pub event_range: Option<Range<usize>>,
    pub state: SavedMuteState,
}

/// Parse immutable project bytes and collect one observation per musical
/// descriptor, including Unknown rows. Malformed framing returns the existing
/// parser error with no partial result. Unknown track evidence is not an error.
///
/// Parsed structures cannot be supplied by callers: all bounds and bindings
/// used below are derived by the existing parser from these same bytes.
pub fn collect_saved_mute_evidence(
    bytes: &[u8],
) -> Result<Vec<SavedMuteEvidence>, Project166Error> {
    let project = parse_project_166(bytes)?;
    let mut evidence = Vec::new();
    for (sequence_ordinal, sequence) in project.sequences.iter().enumerate() {
        for descriptor in sequence.track_descriptors() {
            let mut row = SavedMuteEvidence {
                sequence_ordinal,
                sequence_name: sequence.sequence_name.bytes.bytes.to_vec(),
                descriptor_ordinal: descriptor.ordinal,
                descriptor_range: descriptor.range.clone(),
                label_bytes: descriptor.label.as_ref().map(|label| label.bytes.to_vec()),
                pair_ordinal: None,
                candidate_offset: None,
                candidate_value: None,
                neighboring_guard: None,
                event_range: None,
                state: SavedMuteState::Unknown(SavedMuteUnknown::UnresolvedBinding),
            };
            // Keep all failures local, retaining available provenance without
            // allowing a failed gate to fall through to ON/OFF.
            let result = (|| {
                let TrackAssociations::Ordinal(bindings) = &sequence.track_associations else {
                    return Err(SavedMuteUnknown::UnresolvedBinding);
                };
                let binding = bindings
                    .iter()
                    .find(|binding| binding.descriptor_ordinal == descriptor.ordinal)
                    .ok_or(SavedMuteUnknown::UnresolvedBinding)?;
                row.pair_ordinal = Some(binding.pair_ordinal);
                let label = descriptor
                    .label
                    .as_ref()
                    .filter(|label| {
                        !label.bytes.is_empty() && !label.bytes.iter().all(u8::is_ascii_whitespace)
                    })
                    .ok_or(SavedMuteUnknown::BlankOrUnboundedLabel)?;
                let candidate = descriptor
                    .range
                    .start
                    .checked_sub(24)
                    .ok_or(SavedMuteUnknown::CandidateOutOfBounds)?;
                let end = candidate
                    .checked_add(8)
                    .ok_or(SavedMuteUnknown::CandidateOutOfBounds)?;
                let area_start = sequence.descriptors[0].range.start;
                if label.range.start.checked_sub(39) != Some(candidate)
                    || candidate < area_start
                    || end > descriptor.range.start
                {
                    return Err(SavedMuteUnknown::CandidateOutOfBounds);
                }
                let raw = bytes
                    .get(candidate..end)
                    .ok_or(SavedMuteUnknown::CandidateOutOfBounds)?;
                row.candidate_offset = Some(candidate);
                row.candidate_value = Some(raw[0]);
                let guard: [u8; 7] = raw[1..].try_into().expect("checked eight-byte window");
                row.neighboring_guard = Some(guard);
                let bounds = sequence
                    .validated_track_event_bounds(binding.pair_ordinal)
                    .map_err(|_| SavedMuteUnknown::InvalidEventBounds)?;
                row.event_range = Some(bounds.event_range.clone());
                if bounds.event_range.is_empty() {
                    return Err(SavedMuteUnknown::EmptyEvents);
                }
                if guard != GUARD {
                    return Err(SavedMuteUnknown::GuardMismatch);
                }
                let state = match raw[0] {
                    0x80 => SavedMuteState::Off,
                    0x88 => SavedMuteState::On,
                    _ => return Err(SavedMuteUnknown::UnsupportedValue),
                };
                let walk = walk_bounded_mixed_events(
                    bytes,
                    MixedEventBounds {
                        event_range: bounds.event_range.clone(),
                    },
                    MixedEventTimingBasis::default(),
                )
                .map_err(|_| SavedMuteUnknown::IncompleteEventWalk)?;
                if walk.consumed_range != bounds.event_range || walk.logical_event_count() == 0 {
                    return Err(SavedMuteUnknown::IncompleteEventWalk);
                }
                Ok(state)
            })();
            row.state = result.unwrap_or_else(SavedMuteState::Unknown);
            evidence.push(row);
        }
    }
    Ok(evidence)
}
