//! Owned conversion-ready handoff for a freshly revalidated sequence.
//!
//! This module adapts fresh Descriptor166 bytes plus an authenticated resolved
//! policy into the MIDI-domain values consumed by `multitrack_export`. It does
//! not read paths, infer routing, or serialize output.

// UI0D2 will call this internal boundary from service orchestration. Keep it
// compiled and validated without exposing it through the app contract yet.
#![allow(dead_code)]

use crate::app_service::FreshValidatedSequence;
use crate::compatibility::PatchTranslationPolicy;
use crate::meter::{decode_bounded_initial_meter, InitialMeterBounds};
use crate::midi_export::{
    ChannelAssignment, ChannelAssignmentProvenance, DecodedExportEvent, DecodedExportEventKind,
    MeterPolicy, PatchPolicy, PatchTranslation, TimingPolicy,
};
use crate::mixed_event::{
    walk_bounded_mixed_events, MixedEventBounds, MixedEventItem, MixedEventKind,
    MixedEventTimingBasis,
};
use crate::multitrack_export::{MultitrackSequenceInput, MusicalTrackInput};
use crate::sequence_container::{parse_project_166, TrackAssociations};
use crate::smf::MidiChannel;
use crate::tempo::{decode_bounded_initial_tempo, InitialTempoBounds};
use std::{fmt, ops::Range};

/// Fully owned MIDI-domain values ready to be borrowed by the existing pure
/// multitrack assembler.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ConversionReadySequence {
    pub(crate) sequence_name: Vec<u8>,
    pub(crate) tempo_mpqn: u32,
    pub(crate) meter_values: (u8, u8, u8, u8),
    pub(crate) timing_policy: TimingPolicy,
    pub(crate) meter_policy: MeterPolicy,
    pub(crate) tracks: Vec<ConversionReadyTrack>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ConversionReadyTrack {
    pub(crate) context: String,
    pub(crate) name: Vec<u8>,
    pub(crate) channel_assignment: ChannelAssignment,
    pub(crate) events: Vec<DecodedExportEvent>,
    pub(crate) patch_policy: PatchPolicy,
}

impl ConversionReadySequence {
    /// Borrow this owned handoff as the existing assembler's input for one
    /// immediate in-memory conversion. The returned view cannot outlive the
    /// owned handoff and performs no source or policy lookup.
    pub(crate) fn with_multitrack_input<R>(
        &self,
        callback: impl FnOnce(MultitrackSequenceInput<'_>) -> R,
    ) -> R {
        let tracks = self
            .tracks
            .iter()
            .map(|track| MusicalTrackInput {
                context: &track.context,
                name: &track.name,
                channel_assignment: track.channel_assignment,
                events: &track.events,
                patch_policy: track.patch_policy,
            })
            .collect::<Vec<_>>();
        callback(MultitrackSequenceInput {
            sequence_name: &self.sequence_name,
            tempo_mpqn: self.tempo_mpqn,
            meter_values: self.meter_values,
            timing_policy: self.timing_policy,
            meter_policy: self.meter_policy,
            tracks: &tracks,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ConversionReadyError {
    Parse(String),
    SequenceIdentityMismatch,
    TrackCoverage(String),
    PolicyMismatch(String),
    MetadataDecode(String),
    Bounds(String),
    Walk { track: usize, detail: String },
}

impl fmt::Display for ConversionReadyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "conversion-ready handoff failed: {self:?}")
    }
}

impl std::error::Error for ConversionReadyError {}

/// Build owned conversion input from exactly one successful UI0C4B handoff.
pub(crate) fn build_conversion_ready_sequence(
    fresh: &FreshValidatedSequence,
) -> Result<ConversionReadySequence, ConversionReadyError> {
    let parsed = parse_project_166(&fresh.source_bytes)
        .map_err(|error| ConversionReadyError::Parse(format!("{error:?}")))?;
    let structural_ordinal = usize::try_from(fresh.structural_ordinal)
        .map_err(|_| ConversionReadyError::SequenceIdentityMismatch)?;
    let sequence = parsed
        .sequences
        .get(structural_ordinal)
        .ok_or(ConversionReadyError::SequenceIdentityMismatch)?;
    let evidence_sequence = fresh
        .evidence
        .sequences
        .iter()
        .find(|item| item.structural_ordinal == fresh.structural_ordinal)
        .ok_or(ConversionReadyError::SequenceIdentityMismatch)?;
    let pair_count = u32::try_from(sequence.track_pairs.len())
        .map_err(|_| ConversionReadyError::SequenceIdentityMismatch)?;
    if byte_range(&sequence.sequence_range)? != evidence_sequence.sequence_range
        || byte_range(&sequence.sequence_name.bytes.range)? != evidence_sequence.name_range
        || sequence.sequence_name.bytes.bytes != evidence_sequence.name_bytes
        || u32::from(sequence.descriptor_count.value) != evidence_sequence.descriptor_count
        || pair_count != evidence_sequence.pair_count
        || fresh.resolved_policy.sequence.structural_ordinal != fresh.structural_ordinal
        || fresh.resolved_policy.sequence.sequence_range != evidence_sequence.sequence_range
    {
        return Err(ConversionReadyError::SequenceIdentityMismatch);
    }

    let bindings = match &sequence.track_associations {
        TrackAssociations::Ordinal(bindings) => bindings,
        TrackAssociations::Unresolved { .. } => {
            return Err(ConversionReadyError::TrackCoverage(
                "track associations are unresolved".into(),
            ))
        }
    };
    if bindings.len() != sequence.track_pairs.len()
        || fresh.resolved_policy.tracks.len() != bindings.len()
        || evidence_sequence.tracks.len() != bindings.len()
    {
        return Err(ConversionReadyError::TrackCoverage(
            "policy, evidence, and parser track counts differ".into(),
        ));
    }

    let tempo = decode_bounded_initial_tempo(
        &fresh.source_bytes,
        InitialTempoBounds {
            event_range: sequence.initial_tempo_range.clone(),
        },
    )
    .map_err(|error| ConversionReadyError::MetadataDecode(format!("tempo: {error:?}")))?;
    let meter = decode_bounded_initial_meter(
        &fresh.source_bytes,
        InitialMeterBounds {
            event_range: sequence.initial_meter_range.clone(),
        },
    )
    .map_err(|error| ConversionReadyError::MetadataDecode(format!("meter: {error:?}")))?;
    let meter_values = (
        meter.numerator.value,
        meter.denominator_exponent.value,
        meter.third_payload.value,
        meter.fourth_payload.value,
    );

    let mut tracks = Vec::with_capacity(fresh.resolved_policy.tracks.len());
    let mut seen_keys = std::collections::BTreeSet::new();
    for policy_track in &fresh.resolved_policy.tracks {
        if !seen_keys.insert(policy_track.key.clone()) {
            return Err(ConversionReadyError::PolicyMismatch(
                "duplicate resolved track policy key".into(),
            ));
        }
    }
    for (track_index, binding) in bindings.iter().enumerate() {
        let policy_track = fresh.resolved_policy.tracks.iter().find(|policy_track| {
            u32::try_from(binding.descriptor_ordinal).ok()
                == Some(policy_track.key.descriptor_ordinal)
                && u32::try_from(binding.pair_ordinal).ok() == Some(policy_track.key.pair_ordinal)
        });
        let Some(policy_track) = policy_track else {
            return Err(ConversionReadyError::TrackCoverage(format!(
                "missing policy for descriptor {} / pair {}",
                binding.descriptor_ordinal, binding.pair_ordinal
            )));
        };
        let descriptor = sequence
            .descriptors
            .iter()
            .find(|descriptor| descriptor.ordinal == binding.descriptor_ordinal)
            .ok_or_else(|| ConversionReadyError::TrackCoverage("descriptor is absent".into()))?;
        let pair = sequence
            .track_pairs
            .iter()
            .find(|pair| pair.pair_ordinal == binding.pair_ordinal)
            .ok_or_else(|| ConversionReadyError::TrackCoverage("pair is absent".into()))?;
        let evidence_track = evidence_sequence
            .tracks
            .iter()
            .find(|track| {
                track.descriptor_ordinal == policy_track.key.descriptor_ordinal
                    && track.pair_ordinal == policy_track.key.pair_ordinal
            })
            .ok_or_else(|| ConversionReadyError::TrackCoverage("evidence row is absent".into()))?;
        let bounds = pair
            .validated_event_bounds()
            .map_err(|error| ConversionReadyError::Bounds(format!("{error:?}")))?;
        if byte_range(&descriptor.range)? != evidence_track.descriptor_range
            || byte_range(&pair.primary.record_range)? != evidence_track.primary_range
            || Some(byte_range(&bounds.event_range)?) != evidence_track.exact_event_range
            || descriptor.label.as_ref().map(|label| label.bytes)
                != Some(evidence_track.label_bytes.as_slice())
        {
            return Err(ConversionReadyError::PolicyMismatch(format!(
                "fresh structural evidence differs for track {track_index}"
            )));
        }
        let channel = MidiChannel::new(policy_track.midi_channel).map_err(|error| {
            ConversionReadyError::PolicyMismatch(format!("invalid channel: {error:?}"))
        })?;
        let walk = walk_bounded_mixed_events(
            &fresh.source_bytes,
            MixedEventBounds {
                event_range: bounds.event_range,
            },
            MixedEventTimingBasis::default(),
        )
        .map_err(|error| ConversionReadyError::Walk {
            track: track_index,
            detail: format!("{error:?}"),
        })?;
        if walk.consumed_range != evidence_track.exact_event_range_to_range()? {
            return Err(ConversionReadyError::Bounds(format!(
                "walk did not consume exact range for track {track_index}"
            )));
        }
        let mut events = Vec::with_capacity(walk.logical_event_count());
        let mut source_ordinal = 0_u64;
        let mut used_patch_indices = std::collections::BTreeSet::new();
        for item in walk.items {
            match item {
                MixedEventItem::Patch(patch) => {
                    let patch_start =
                        u64::try_from(patch.representation_range.start).map_err(|_| {
                            ConversionReadyError::PolicyMismatch(format!(
                                "Patch range overflows fixed-width evidence for track {track_index}"
                            ))
                        })?;
                    let patch_end =
                        u64::try_from(patch.representation_range.end).map_err(|_| {
                            ConversionReadyError::PolicyMismatch(format!(
                            "Patch range overflows fixed-width evidence for track {track_index}"
                        ))
                        })?;
                    let patch_index = evidence_track
                        .patch_evidence
                        .iter()
                        .enumerate()
                        .find(|(index, evidence)| {
                            !used_patch_indices.contains(index)
                                && evidence.source_range.start() == patch_start
                                && evidence.source_range.end_exclusive() == patch_end
                        })
                        .map(|(index, _)| index)
                        .ok_or_else(|| {
                            ConversionReadyError::PolicyMismatch(format!(
                                "Patch evidence missing for track {track_index}"
                            ))
                        })?;
                    used_patch_indices.insert(patch_index);
                    let translation = policy_track.patches.get(patch_index).ok_or_else(|| {
                        ConversionReadyError::PolicyMismatch(format!(
                            "Patch policy missing for track {track_index}"
                        ))
                    })?;
                    events.push(DecodedExportEvent {
                        absolute_position: patch.position.value,
                        source_ordinal,
                        source_range: Some(patch.representation_range),
                        kind: DecodedExportEventKind::Patch {
                            program: patch.program_change.value,
                            translation: patch_translation(translation),
                        },
                    });
                    source_ordinal += 1;
                }
                MixedEventItem::PatchToNote(transition) => {
                    let patch_start = u64::try_from(transition.patch.representation_range.start)
                        .map_err(|_| {
                            ConversionReadyError::PolicyMismatch(format!(
                                "Patch range overflows fixed-width evidence for track {track_index}"
                            ))
                        })?;
                    let patch_end = u64::try_from(transition.patch.representation_range.end)
                        .map_err(|_| {
                            ConversionReadyError::PolicyMismatch(format!(
                                "Patch range overflows fixed-width evidence for track {track_index}"
                            ))
                        })?;
                    let patch_index = evidence_track
                        .patch_evidence
                        .iter()
                        .enumerate()
                        .find(|(index, patch)| {
                            !used_patch_indices.contains(index)
                                && patch.source_range.start() == patch_start
                                && patch.source_range.end_exclusive() == patch_end
                        })
                        .map(|(index, _)| index)
                        .ok_or_else(|| {
                            ConversionReadyError::PolicyMismatch(format!(
                                "Patch evidence missing for track {track_index}"
                            ))
                        })?;
                    used_patch_indices.insert(patch_index);
                    let translation = policy_track.patches.get(patch_index).ok_or_else(|| {
                        ConversionReadyError::PolicyMismatch(format!(
                            "Patch policy missing for track {track_index}"
                        ))
                    })?;
                    events.push(DecodedExportEvent::from_patch(
                        source_ordinal,
                        &transition.patch,
                        patch_translation(translation),
                    ));
                    source_ordinal += 1;
                    events.push(DecodedExportEvent::from_note_body(
                        transition.first_note_position,
                        source_ordinal,
                        &transition.first_note,
                    ));
                    source_ordinal += 1;
                }
                MixedEventItem::Event(positioned) => {
                    let event = match positioned.event {
                        MixedEventKind::Note(note) => DecodedExportEvent::from_note(
                            positioned.position,
                            source_ordinal,
                            &note,
                        ),
                        MixedEventKind::ContextMediatedNote(note) => {
                            DecodedExportEvent::from_note_body(
                                positioned.position,
                                source_ordinal,
                                &note.note,
                            )
                        }
                        MixedEventKind::Controller(controller) => {
                            DecodedExportEvent::from_controller(
                                positioned.position,
                                source_ordinal,
                                &controller,
                            )
                        }
                        MixedEventKind::ChannelPressure { entry, .. } => {
                            DecodedExportEvent::from_channel_pressure(
                                positioned.position,
                                source_ordinal,
                                &entry,
                            )
                        }
                        MixedEventKind::PitchBend { entry, .. } => {
                            DecodedExportEvent::from_pitch_bend(
                                positioned.position,
                                source_ordinal,
                                &entry,
                            )
                        }
                    };
                    events.push(event);
                    source_ordinal += 1;
                }
            }
        }
        let event_count = u64::try_from(events.len()).map_err(|_| {
            ConversionReadyError::PolicyMismatch(format!(
                "decoded event count overflows fixed-width evidence for track {track_index}"
            ))
        })?;
        if used_patch_indices.len() != policy_track.patches.len()
            || event_count != evidence_track.decoded_event_count
        {
            return Err(ConversionReadyError::PolicyMismatch(format!(
                "decoded event or Patch coverage differs for track {track_index}"
            )));
        }
        tracks.push(ConversionReadyTrack {
            context: format!(
                "descriptor {} / pair {}",
                policy_track.key.descriptor_ordinal, policy_track.key.pair_ordinal
            ),
            name: evidence_track.label_bytes.clone(),
            channel_assignment: ChannelAssignment {
                channel,
                provenance: ChannelAssignmentProvenance::AuthenticatedOverride,
            },
            events,
            patch_policy: PatchPolicy::StrictKnownOnly,
        });
    }
    Ok(ConversionReadySequence {
        sequence_name: evidence_sequence.name_bytes.clone(),
        tempo_mpqn: tempo.mpqn(),
        meter_values,
        timing_policy: TimingPolicy::Identity480,
        meter_policy: MeterPolicy::HistoricalWhenKnownOtherwiseStandard,
        tracks,
    })
}

fn patch_translation(policy: &PatchTranslationPolicy) -> PatchTranslation {
    match policy {
        PatchTranslationPolicy::ProgramOnly { .. } => PatchTranslation::ProgramOnlyConfirmed,
        PatchTranslationPolicy::BankSelectAndProgram { msb, lsb, .. } => {
            PatchTranslation::ConfirmedBankSelect {
                msb: *msb,
                lsb: *lsb,
            }
        }
    }
}

fn byte_range(
    range: &Range<usize>,
) -> Result<crate::compatibility::ByteRange, ConversionReadyError> {
    let start = u64::try_from(range.start)
        .map_err(|_| ConversionReadyError::Bounds("range start overflows u64".into()))?;
    let end = u64::try_from(range.end)
        .map_err(|_| ConversionReadyError::Bounds("range end overflows u64".into()))?;
    crate::compatibility::ByteRange::new(start, end)
        .map_err(|_| ConversionReadyError::Bounds("range is not ordered".into()))
}

trait EvidenceRange {
    fn exact_event_range_to_range(&self) -> Result<Range<usize>, ConversionReadyError>;
}

impl EvidenceRange for crate::compatibility::TrackEvidence {
    fn exact_event_range_to_range(&self) -> Result<Range<usize>, ConversionReadyError> {
        let range = self
            .exact_event_range
            .ok_or_else(|| ConversionReadyError::Bounds("exact event range is absent".into()))?;
        let start = usize::try_from(range.start()).map_err(|_| {
            ConversionReadyError::Bounds("event range start overflows usize".into())
        })?;
        let end = usize::try_from(range.end_exclusive())
            .map_err(|_| ConversionReadyError::Bounds("event range end overflows usize".into()))?;
        if start > end {
            return Err(ConversionReadyError::Bounds(
                "exact event range is not ordered".into(),
            ));
        }
        Ok(start..end)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::app_contract::{DiagnosticsLevel, InspectProjectRequest, CONTRACT_VERSION};
    use crate::app_service::AppService;
    use crate::compatibility::{
        ByteRange, ParserProfileId, PatchEvidence, ProfileEvidence, ProfileId, ProfileVersion,
        ResolvedProfilePolicy, ResolvedSequenceIdentity, ResolvedTrackPolicy, SequenceEvidence,
        TrackEvidence, TrackKey,
    };
    use crate::midi_export::DecodedExportEventKind;
    use std::{fs, path::Path};

    const AUTHENTIC_SOURCE: &str =
        "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline";

    fn push_record(bytes: &mut Vec<u8>, record_type: u8, payload: &[u8]) {
        bytes.push(record_type);
        bytes.extend_from_slice(&(payload.len() as u32).to_be_bytes());
        bytes.extend_from_slice(payload);
    }

    fn patch_to_note(program: u8) -> Vec<u8> {
        let name = b"Portable";
        let payload_length = 5 + 1 + name.len() + 1;
        let mut bytes = vec![0, 0xff, 0x7c, payload_length as u8];
        bytes.extend([1, 2, 3, 4, 5, name.len() as u8]);
        bytes.extend(name);
        bytes.push(program);
        bytes.extend([0, 0x90, 60, 64, 32, 1]);
        bytes
    }

    pub(crate) fn portable_project() -> Vec<u8> {
        const ROOT_HEADER_LENGTH: usize = 8;
        const SEQUENCE_PREAMBLE_LENGTH: usize = 208;
        const DESCRIPTOR_STRIDE: usize = 166;
        let labels = [
            b"internal-a".as_slice(),
            b"internal-b",
            b"Track A",
            b"Track B",
        ];
        let count = labels.len();
        let mut bytes = vec![0xa5; ROOT_HEADER_LENGTH];
        let mut sequence_payload = vec![0; count * DESCRIPTOR_STRIDE + 172];
        sequence_payload[0] = count as u8;
        push_record(&mut bytes, 0x01, &sequence_payload);

        let descriptor_start = ROOT_HEADER_LENGTH + SEQUENCE_PREAMBLE_LENGTH;
        for (ordinal, label) in labels.iter().enumerate() {
            let start = descriptor_start + ordinal * DESCRIPTOR_STRIDE + 15;
            bytes[start..start + label.len()].copy_from_slice(label);
            bytes[start + label.len()] = 0;
        }

        let name = b"Portable Sequence";
        let mut name_payload = vec![0; 11];
        name_payload.push(name.len() as u8);
        name_payload.extend(name);
        push_record(&mut bytes, 0x07, &name_payload);

        let mut meter = vec![0; 22];
        meter[14..].copy_from_slice(&[0, 0xff, 0x58, 4, 7, 3, 6, 8]);
        push_record(&mut bytes, 0x02, &meter);
        push_record(&mut bytes, 0x29, &[]);
        let mut tempo = vec![0; 21];
        tempo[14..].copy_from_slice(&[0, 0xff, 0x51, 3, 0x07, 0xa1, 0x20]);
        push_record(&mut bytes, 0x02, &tempo);
        push_record(&mut bytes, 0x29, &[]);

        let track_events = [patch_to_note(42), vec![0, 0x90, 65, 70, 16, 2]];
        for events in track_events {
            let mut payload = vec![0x11; 14];
            payload.extend(events);
            payload.extend([0xff, 1, 2, 3, 0xff, 0x2f, 0]);
            push_record(&mut bytes, 0x02, &payload);
            push_record(&mut bytes, 0x29, &[]);
        }
        push_record(&mut bytes, 0, &[]);
        bytes
    }

    fn fixed_range(range: &Range<usize>) -> ByteRange {
        ByteRange::new(range.start as u64, range.end as u64).unwrap()
    }

    fn portable_fresh() -> FreshValidatedSequence {
        let source_bytes = portable_project();
        let parsed = parse_project_166(&source_bytes).unwrap();
        let sequence = &parsed.sequences[0];
        let TrackAssociations::Ordinal(bindings) = &sequence.track_associations else {
            panic!("portable project must have ordinal associations")
        };
        let mut tracks = Vec::new();
        let mut policies = Vec::new();
        for (index, binding) in bindings.iter().enumerate() {
            let descriptor = &sequence.descriptors[binding.descriptor_ordinal];
            let pair = &sequence.track_pairs[binding.pair_ordinal];
            let bounds = pair.validated_event_bounds().unwrap();
            let walk = walk_bounded_mixed_events(
                &source_bytes,
                MixedEventBounds {
                    event_range: bounds.event_range.clone(),
                },
                MixedEventTimingBasis::default(),
            )
            .unwrap();
            let patch_evidence = walk
                .items
                .iter()
                .enumerate()
                .filter_map(|(ordinal, item)| match item {
                    MixedEventItem::Patch(patch) => Some(PatchEvidence {
                        source_ordinal: ordinal as u32,
                        source_range: fixed_range(&patch.representation_range),
                        decoded_program: patch.program_change.value,
                        decoded_bank_msb: None,
                        decoded_bank_lsb: None,
                    }),
                    MixedEventItem::PatchToNote(transition) => Some(PatchEvidence {
                        source_ordinal: ordinal as u32,
                        source_range: fixed_range(&transition.patch.representation_range),
                        decoded_program: transition.patch.program_change.value,
                        decoded_bank_msb: None,
                        decoded_bank_lsb: None,
                    }),
                    MixedEventItem::Event(_) => None,
                })
                .collect::<Vec<_>>();
            let key = TrackKey::new(
                binding.descriptor_ordinal as u32,
                binding.pair_ordinal as u32,
            );
            tracks.push(TrackEvidence {
                descriptor_ordinal: key.descriptor_ordinal,
                descriptor_range: fixed_range(&descriptor.range),
                pair_ordinal: key.pair_ordinal,
                primary_range: fixed_range(&pair.primary.record_range),
                exact_event_range: Some(fixed_range(&bounds.event_range)),
                label_bytes: descriptor.label.as_ref().unwrap().bytes.to_vec(),
                decoded_event_families: Vec::new(),
                decoded_event_count: walk.logical_event_count() as u64,
                patch_evidence,
                observed_channel: None,
                evidence_complete: false,
            });
            policies.push(ResolvedTrackPolicy {
                key,
                midi_channel: [3, 11][index],
                patches: if index == 0 {
                    vec![PatchTranslationPolicy::BankSelectAndProgram {
                        msb: 81,
                        lsb: 2,
                        program: 42,
                    }]
                } else {
                    Vec::new()
                },
            });
        }
        let sequence_range = fixed_range(&sequence.sequence_range);
        let source_byte_size = parsed.consumed_range.end as u64;
        let name_bytes = sequence.sequence_name.bytes.bytes.to_vec();
        let name_range = fixed_range(&sequence.sequence_name.bytes.range);
        let descriptor_count = u32::from(sequence.descriptor_count.value);
        let pair_count = sequence.track_pairs.len() as u32;
        FreshValidatedSequence {
            source_bytes,
            source_sha256: "portable-owned-source".into(),
            structural_ordinal: 0,
            evidence: ProfileEvidence {
                source_sha256: "portable-owned-source".into(),
                source_byte_size,
                parser_profile: ParserProfileId::new("descriptor166"),
                sequences: vec![SequenceEvidence {
                    structural_ordinal: 0,
                    sequence_range,
                    name_bytes,
                    name_range,
                    descriptor_count,
                    pair_count,
                    tracks,
                }],
            },
            resolved_policy: ResolvedProfilePolicy {
                profile_id: ProfileId::new("portable"),
                profile_version: ProfileVersion::new(1),
                sequence: ResolvedSequenceIdentity {
                    structural_ordinal: 0,
                    sequence_range,
                },
                tracks: policies,
            },
        }
    }

    #[test]
    fn patch_policy_translation_is_explicit_and_non_defaulting() {
        assert_eq!(
            patch_translation(&PatchTranslationPolicy::ProgramOnly { program: 7 }),
            PatchTranslation::ProgramOnlyConfirmed
        );
        assert_eq!(
            patch_translation(&PatchTranslationPolicy::BankSelectAndProgram {
                msb: 1,
                lsb: 2,
                program: 7,
            }),
            PatchTranslation::ConfirmedBankSelect { msb: 1, lsb: 2 }
        );
    }

    #[test]
    fn portable_handoff_preserves_structure_policy_metadata_and_ownership() {
        let mut fresh = portable_fresh();
        fresh.resolved_policy.tracks.reverse();
        let ready = build_conversion_ready_sequence(&fresh).unwrap();
        drop(fresh);

        assert_eq!(ready.sequence_name, b"Portable Sequence");
        assert_eq!(ready.tempo_mpqn, 500_000);
        assert_eq!(ready.meter_values, (7, 3, 6, 8));
        assert_eq!(
            ready
                .tracks
                .iter()
                .map(|track| track.name.as_slice())
                .collect::<Vec<_>>(),
            vec![b"Track A".as_slice(), b"Track B".as_slice()]
        );
        assert_eq!(
            ready
                .tracks
                .iter()
                .map(|track| track.channel_assignment.channel.get())
                .collect::<Vec<_>>(),
            vec![3, 11]
        );
        assert!(matches!(
            ready.tracks[0].events[0].kind,
            DecodedExportEventKind::Patch {
                program: 42,
                translation: PatchTranslation::ConfirmedBankSelect { msb: 81, lsb: 2 }
            }
        ));
        ready.with_multitrack_input(|input| {
            assert_eq!(input.sequence_name, b"Portable Sequence");
            assert_eq!(input.tracks.len(), 2);
            assert_eq!(input.tracks[0].name, b"Track A");
            assert_eq!(input.tracks[1].name, b"Track B");
        });
    }

    #[test]
    fn portable_handoff_rejects_identity_and_track_policy_drift() {
        let fresh = portable_fresh();

        let mut wrong_ordinal = fresh.clone();
        wrong_ordinal.structural_ordinal = 1;
        assert!(matches!(
            build_conversion_ready_sequence(&wrong_ordinal),
            Err(ConversionReadyError::SequenceIdentityMismatch)
        ));

        for mutate in [
            |value: &mut FreshValidatedSequence| {
                value.evidence.sequences[0].sequence_range = ByteRange::new(0, 1).unwrap()
            },
            |value: &mut FreshValidatedSequence| {
                value.evidence.sequences[0].name_range = ByteRange::new(0, 1).unwrap()
            },
            |value: &mut FreshValidatedSequence| value.evidence.sequences[0].descriptor_count += 1,
            |value: &mut FreshValidatedSequence| value.evidence.sequences[0].pair_count += 1,
        ] {
            let mut changed = fresh.clone();
            mutate(&mut changed);
            assert!(matches!(
                build_conversion_ready_sequence(&changed),
                Err(ConversionReadyError::SequenceIdentityMismatch)
            ));
        }

        let mut missing = fresh.clone();
        missing.resolved_policy.tracks.pop();
        assert!(matches!(
            build_conversion_ready_sequence(&missing),
            Err(ConversionReadyError::TrackCoverage(_))
        ));

        let mut extra = fresh.clone();
        extra
            .resolved_policy
            .tracks
            .push(extra.resolved_policy.tracks[0].clone());
        assert!(matches!(
            build_conversion_ready_sequence(&extra),
            Err(ConversionReadyError::TrackCoverage(_))
        ));

        let mut duplicate = fresh.clone();
        duplicate.resolved_policy.tracks[1].key = duplicate.resolved_policy.tracks[0].key.clone();
        assert!(matches!(
            build_conversion_ready_sequence(&duplicate),
            Err(ConversionReadyError::PolicyMismatch(_))
        ));

        let mut inconsistent = fresh;
        inconsistent.resolved_policy.tracks[0].key = TrackKey::new(99, 99);
        assert!(matches!(
            build_conversion_ready_sequence(&inconsistent),
            Err(ConversionReadyError::TrackCoverage(_))
        ));

        let mut inconsistent_range = portable_fresh();
        inconsistent_range.evidence.sequences[0].tracks[0].descriptor_range =
            ByteRange::new(0, 1).unwrap();
        assert!(matches!(
            build_conversion_ready_sequence(&inconsistent_range),
            Err(ConversionReadyError::PolicyMismatch(_))
        ));
    }

    #[test]
    fn portable_handoff_rejects_patch_policy_coverage_drift() {
        let fresh = portable_fresh();

        let mut missing = fresh.clone();
        missing.resolved_policy.tracks[0].patches.clear();
        assert!(matches!(
            build_conversion_ready_sequence(&missing),
            Err(ConversionReadyError::PolicyMismatch(_))
        ));

        let mut extra = fresh.clone();
        extra.resolved_policy.tracks[0]
            .patches
            .push(PatchTranslationPolicy::ProgramOnly { program: 42 });
        assert!(matches!(
            build_conversion_ready_sequence(&extra),
            Err(ConversionReadyError::PolicyMismatch(_))
        ));

        let mut program_only = fresh;
        program_only.resolved_policy.tracks[0].patches[0] =
            PatchTranslationPolicy::ProgramOnly { program: 42 };
        let ready = build_conversion_ready_sequence(&program_only).unwrap();
        assert!(matches!(
            ready.tracks[0].events[0].kind,
            DecodedExportEventKind::Patch {
                program: 42,
                translation: PatchTranslation::ProgramOnlyConfirmed
            }
        ));
    }

    #[test]
    fn portable_handoff_does_not_reread_the_original_source() {
        let path = std::env::temp_dir().join(format!(
            "phoenix-ui0d1-{}-{}",
            std::process::id(),
            std::thread::current().name().unwrap_or("test")
        ));
        let mut fresh = portable_fresh();
        fs::write(&path, &fresh.source_bytes).unwrap();
        fresh.source_bytes = fs::read(&path).unwrap();
        fs::remove_file(&path).unwrap();

        let ready = build_conversion_ready_sequence(&fresh).unwrap();
        assert_eq!(ready.sequence_name, b"Portable Sequence");
        assert!(!path.exists());
    }

    #[test]
    fn authentic_revalidated_sequence_becomes_owned_assembler_input_when_fixture_exists() {
        let source = Path::new(AUTHENTIC_SOURCE);
        if !source.is_file() {
            return;
        }
        let mut service = AppService::new();
        let response = service
            .inspect_project(InspectProjectRequest {
                contract_version: CONTRACT_VERSION,
                source_path: source.to_string_lossy().into_owned(),
                diagnostics_level: DiagnosticsLevel::Full,
            })
            .expect("authentic source should inspect");
        let mut candidates = response
            .sequences
            .iter()
            .filter_map(|sequence| {
                let fresh = service
                    .revalidated_policy_for_sequence(&response.session_id, &sequence.sequence_id)
                    .ok()?;
                let ready = build_conversion_ready_sequence(&fresh).ok()?;
                Some((fresh, ready))
            })
            .collect::<Vec<_>>();
        assert_eq!(candidates.len(), 1);
        let (fresh, ready) = candidates.pop().expect("one matched sequence");
        assert_eq!(ready.tracks.len(), 9);
        assert_eq!(
            ready
                .tracks
                .iter()
                .map(|track| track.channel_assignment.channel.get())
                .collect::<Vec<_>>(),
            vec![1, 2, 10, 10, 10, 1, 10, 15, 10]
        );
        assert_eq!(
            ready
                .tracks
                .iter()
                .map(|track| track.events.len())
                .sum::<usize>(),
            fresh
                .evidence
                .sequences
                .iter()
                .find(|sequence| sequence.structural_ordinal == fresh.structural_ordinal)
                .expect("fresh target evidence")
                .tracks
                .iter()
                .map(|track| track.decoded_event_count as usize)
                .sum::<usize>()
        );
        assert_eq!(
            ready
                .tracks
                .iter()
                .map(|track| track
                    .events
                    .iter()
                    .filter(|event| {
                        matches!(
                            event.kind,
                            crate::midi_export::DecodedExportEventKind::Patch { .. }
                        )
                    })
                    .count())
                .sum::<usize>(),
            4
        );
        ready.with_multitrack_input(|input| {
            assert_eq!(input.tracks.len(), 9);
        });

        let mut missing_fresh_bytes = fresh.clone();
        missing_fresh_bytes.source_bytes.clear();
        assert!(matches!(
            build_conversion_ready_sequence(&missing_fresh_bytes),
            Err(ConversionReadyError::Parse(_))
        ));

        let mut wrong_sequence = fresh.clone();
        wrong_sequence.structural_ordinal = wrong_sequence.structural_ordinal.saturating_add(1);
        assert!(matches!(
            build_conversion_ready_sequence(&wrong_sequence),
            Err(ConversionReadyError::SequenceIdentityMismatch)
        ));

        let mut incomplete = fresh.clone();
        incomplete.resolved_policy.tracks.pop();
        assert!(matches!(
            build_conversion_ready_sequence(&incomplete),
            Err(ConversionReadyError::TrackCoverage(_))
        ));

        let mut extra = fresh.clone();
        let extra_row = extra.resolved_policy.tracks[0].clone();
        extra.resolved_policy.tracks.push(extra_row);
        assert!(matches!(
            build_conversion_ready_sequence(&extra),
            Err(ConversionReadyError::TrackCoverage(_))
        ));

        let mut duplicate = fresh;
        let duplicate_key = duplicate.resolved_policy.tracks[0].key.clone();
        duplicate.resolved_policy.tracks[1].key = duplicate_key;
        assert!(matches!(
            build_conversion_ready_sequence(&duplicate),
            Err(ConversionReadyError::PolicyMismatch(_))
        ));
    }
}
