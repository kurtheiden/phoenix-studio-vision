//! Pure transactional assembly of one conductor plus ordered musical tracks.
//!
//! This layer accepts decoded MIDI-export events and explicit policy. It does
//! not parse Studio Vision data, infer channels, read files, or own any
//! authenticated-project knowledge.

use crate::{
    midi_export::{
        adapt_conductor, adapt_text, adapt_track, ChannelAssignment, DecodedExportEvent,
        ExportCounts, ExportWarning, MeterPolicy, MidiExportError, PatchPolicy, TimingPolicy,
        UntranslatedMetadata,
    },
    smf::{
        serialize_conductor_track, serialize_format1, serialize_named_musical_track,
        SmfSerializeError,
    },
};
use std::fmt;

/// One caller-ordered musical track containing only decoded export values.
#[derive(Clone, Debug)]
pub struct MusicalTrackInput<'a> {
    /// Opaque diagnostic identity; it is not used for ordering or uniqueness.
    pub context: &'a str,
    pub name: &'a [u8],
    pub channel_assignment: ChannelAssignment,
    pub events: &'a [DecodedExportEvent],
    pub patch_policy: PatchPolicy,
}

/// The complete MIDI-domain policy input for one Format 1 sequence.
#[derive(Clone, Debug)]
pub struct MultitrackSequenceInput<'a> {
    pub sequence_name: &'a [u8],
    pub tempo_mpqn: u32,
    pub meter_values: (u8, u8, u8, u8),
    pub timing_policy: TimingPolicy,
    pub meter_policy: MeterPolicy,
    pub tracks: &'a [MusicalTrackInput<'a>],
}

/// Report for one successfully adapted and serialized musical track.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MusicalTrackExportReport {
    pub context: String,
    pub name: Vec<u8>,
    pub channel_assignment: ChannelAssignment,
    pub counts: ExportCounts,
    pub warnings: Vec<ExportWarning>,
    pub untranslated_metadata: Vec<UntranslatedMetadata>,
}

/// Aggregate report derived only from successful conductor/track results.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MultitrackExportReport {
    pub sequence_name: Vec<u8>,
    pub musical_track_count: usize,
    pub total_smf_track_count: usize,
    pub tracks: Vec<MusicalTrackExportReport>,
    pub totals: ExportCounts,
    pub warnings: Vec<ExportWarning>,
    pub untranslated_metadata: Vec<UntranslatedMetadata>,
}

/// Complete in-memory Format 1 result. There is no partial-success form.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MultitrackExportResult {
    pub smf_bytes: Vec<u8>,
    pub report: MultitrackExportReport,
}

/// Context-preserving failures from pure sequence assembly.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MultitrackExportError {
    ConductorAdaptation {
        source: MidiExportError,
    },
    MusicalTrackAdaptation {
        track_index: usize,
        context: String,
        source: MidiExportError,
    },
    ConductorSerialization {
        source: SmfSerializeError,
    },
    MusicalTrackSerialization {
        track_index: usize,
        context: String,
        source: SmfSerializeError,
    },
    Format1Assembly {
        source: SmfSerializeError,
    },
}

impl fmt::Display for MultitrackExportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConductorAdaptation { source } => {
                write!(formatter, "cannot adapt conductor: {source}")
            }
            Self::MusicalTrackAdaptation {
                track_index,
                context,
                source,
            } => write!(
                formatter,
                "cannot adapt musical track {track_index} ({context}): {source}"
            ),
            Self::ConductorSerialization { source } => {
                write!(formatter, "cannot serialize conductor: {source}")
            }
            Self::MusicalTrackSerialization {
                track_index,
                context,
                source,
            } => write!(
                formatter,
                "cannot serialize musical track {track_index} ({context}): {source}"
            ),
            Self::Format1Assembly { source } => {
                write!(formatter, "cannot assemble Format 1 file: {source}")
            }
        }
    }
}

impl std::error::Error for MultitrackExportError {}

/// Adapts and serializes one complete sequence transactionally.
///
/// Track zero is the conductor. Musical tracks retain the caller's exact
/// order, including empty tracks and duplicate names/channels.
pub fn assemble_multitrack_sequence(
    input: &MultitrackSequenceInput<'_>,
) -> Result<MultitrackExportResult, MultitrackExportError> {
    let conductor = adapt_conductor(
        input.sequence_name,
        input.tempo_mpqn,
        input.meter_values,
        input.timing_policy,
        input.meter_policy,
    )
    .map_err(|source| MultitrackExportError::ConductorAdaptation { source })?;

    let conductor_track = serialize_conductor_track(
        &conductor.sequence_name,
        conductor.tempo_mpqn,
        conductor.time_signature,
    )
    .map_err(|source| MultitrackExportError::ConductorSerialization { source })?;

    let mut serialized_tracks = Vec::with_capacity(input.tracks.len() + 1);
    serialized_tracks.push(conductor_track);
    let mut track_reports = Vec::with_capacity(input.tracks.len());
    let mut totals = conductor.counts.clone();
    let mut warnings = conductor.warnings.clone();
    let mut untranslated_metadata = Vec::new();

    for (track_index, track) in input.tracks.iter().enumerate() {
        let name = adapt_text(track.name).map_err(|source| {
            MultitrackExportError::MusicalTrackAdaptation {
                track_index,
                context: track.context.to_owned(),
                source,
            }
        })?;
        let adapted = adapt_track(
            track.events,
            Some(track.channel_assignment),
            input.timing_policy,
            track.patch_policy,
        )
        .map_err(|source| MultitrackExportError::MusicalTrackAdaptation {
            track_index,
            context: track.context.to_owned(),
            source,
        })?;
        let serialized =
            serialize_named_musical_track(&name, &adapted.scheduled_events).map_err(|source| {
                MultitrackExportError::MusicalTrackSerialization {
                    track_index,
                    context: track.context.to_owned(),
                    source,
                }
            })?;

        totals.add_assign(&adapted.counts);
        warnings.extend(adapted.warnings.iter().cloned());
        untranslated_metadata.extend(adapted.untranslated_metadata.iter().cloned());
        track_reports.push(MusicalTrackExportReport {
            context: track.context.to_owned(),
            name,
            channel_assignment: adapted.channel_assignment,
            counts: adapted.counts,
            warnings: adapted.warnings,
            untranslated_metadata: adapted.untranslated_metadata,
        });
        serialized_tracks.push(serialized);
    }

    let smf_bytes = serialize_format1(conductor.ppqn, &serialized_tracks)
        .map_err(|source| MultitrackExportError::Format1Assembly { source })?;
    let musical_track_count = track_reports.len();
    Ok(MultitrackExportResult {
        smf_bytes,
        report: MultitrackExportReport {
            sequence_name: conductor.sequence_name,
            musical_track_count,
            total_smf_track_count: musical_track_count + 1,
            tracks: track_reports,
            totals,
            warnings,
            untranslated_metadata,
        },
    })
}
