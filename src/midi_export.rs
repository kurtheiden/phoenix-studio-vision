//! Pure adaptation from already decoded musical values to SMF-domain values.
//!
//! This module does not locate, bound, or parse Studio Vision structures. It
//! accepts decoded fields plus explicit policy and channel assignment, then
//! produces values consumed directly by [`crate::smf`].

use crate::{
    channel_pressure::ChannelPressureEntry,
    controller::BoundedControllerRecord,
    meter::InitialMeterEvent,
    patch::BoundedPatchRepresentation,
    pitch_bend::PitchBendEntry,
    smf::{MidiChannel, MidiDataByte, ScheduledEvent, SmfSerializeError, TimeSignature},
    tempo::InitialTempoEvent,
    track7::{BoundedNoteBody, BoundedNoteEvent},
};
use std::{collections::BTreeSet, fmt, ops::Range};

pub const IDENTITY_480_PPQN: u16 = 480;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimingPolicy {
    Identity480,
    Unsupported,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChannelAssignmentProvenance {
    ParsedRouting,
    AuthenticatedOverride,
    Synthetic,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChannelAssignment {
    pub channel: MidiChannel,
    pub provenance: ChannelAssignmentProvenance,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MeterPolicy {
    HistoricalWhenKnownOtherwiseStandard,
    KnownHistoricalOnly,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PatchPolicy {
    StrictKnownOnly,
}

/// An upstream, evidence-based classification of translatable Patch state.
///
/// The adapter never derives this classification from opaque Patch bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PatchTranslation {
    ProgramOnlyConfirmed,
    ConfirmedBankSelect { msb: u8, lsb: u8 },
    UnsupportedOpaque,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecodedExportEvent {
    pub absolute_position: u32,
    pub source_ordinal: u64,
    pub source_range: Option<Range<usize>>,
    pub kind: DecodedExportEventKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecodedExportEventKind {
    Note {
        pitch: u8,
        attack_velocity: u8,
        release_velocity: u8,
        duration: u32,
    },
    Controller {
        number: u8,
        value: u8,
        has_opaque_context: bool,
    },
    ChannelPressure {
        value: u8,
    },
    PitchBend {
        lsb: u8,
        msb: u8,
    },
    Patch {
        program: u8,
        translation: PatchTranslation,
    },
    Unsupported {
        family: &'static str,
    },
}

impl DecodedExportEvent {
    pub fn from_note(
        absolute_position: u32,
        source_ordinal: u64,
        note: &BoundedNoteEvent<'_>,
    ) -> Self {
        Self::from_note_fields(
            absolute_position,
            source_ordinal,
            Some(note.representation_range.clone()),
            note.pitch.value,
            note.attack_velocity.value,
            note.release_velocity.value,
            note.duration.value,
        )
    }

    pub fn from_note_body(
        absolute_position: u32,
        source_ordinal: u64,
        note: &BoundedNoteBody<'_>,
    ) -> Self {
        Self::from_note_fields(
            absolute_position,
            source_ordinal,
            Some(note.representation_range.clone()),
            note.pitch.value,
            note.attack_velocity.value,
            note.release_velocity.value,
            note.duration.value,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_note_fields(
        absolute_position: u32,
        source_ordinal: u64,
        source_range: Option<Range<usize>>,
        pitch: u8,
        attack_velocity: u8,
        release_velocity: u8,
        duration: u32,
    ) -> Self {
        Self {
            absolute_position,
            source_ordinal,
            source_range,
            kind: DecodedExportEventKind::Note {
                pitch,
                attack_velocity,
                release_velocity,
                duration,
            },
        }
    }

    pub fn from_controller(
        absolute_position: u32,
        source_ordinal: u64,
        controller: &BoundedControllerRecord<'_>,
    ) -> Self {
        Self {
            absolute_position,
            source_ordinal,
            source_range: Some(controller.record_range.clone()),
            kind: DecodedExportEventKind::Controller {
                number: controller.controller_number.value,
                value: controller.controller_value.value,
                has_opaque_context: true,
            },
        }
    }

    pub fn from_channel_pressure(
        absolute_position: u32,
        source_ordinal: u64,
        pressure: &ChannelPressureEntry<'_>,
    ) -> Self {
        Self {
            absolute_position,
            source_ordinal,
            source_range: Some(pressure.entry_range.clone()),
            kind: DecodedExportEventKind::ChannelPressure {
                value: pressure.pressure_value.value,
            },
        }
    }

    pub fn from_pitch_bend(
        absolute_position: u32,
        source_ordinal: u64,
        bend: &PitchBendEntry<'_>,
    ) -> Self {
        Self {
            absolute_position,
            source_ordinal,
            source_range: Some(bend.entry_range.clone()),
            kind: DecodedExportEventKind::PitchBend {
                lsb: bend.pitch_lsb.value,
                msb: bend.pitch_msb.value,
            },
        }
    }

    pub fn from_patch(
        source_ordinal: u64,
        patch: &BoundedPatchRepresentation<'_>,
        translation: PatchTranslation,
    ) -> Self {
        Self {
            absolute_position: patch.position.value,
            source_ordinal,
            source_range: Some(patch.representation_range.clone()),
            kind: DecodedExportEventKind::Patch {
                program: patch.program_change.value,
                translation,
            },
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ExportCounts {
    pub notes: u64,
    pub generated_note_offs: u64,
    pub controllers: u64,
    pub bank_select_msb: u64,
    pub bank_select_lsb: u64,
    pub program_changes: u64,
    pub channel_pressure: u64,
    pub pitch_bend: u64,
    pub tempo: u64,
    pub meter: u64,
}

impl ExportCounts {
    pub fn add_assign(&mut self, other: &Self) {
        self.notes += other.notes;
        self.generated_note_offs += other.generated_note_offs;
        self.controllers += other.controllers;
        self.bank_select_msb += other.bank_select_msb;
        self.bank_select_lsb += other.bank_select_lsb;
        self.program_changes += other.program_changes;
        self.channel_pressure += other.channel_pressure;
        self.pitch_bend += other.pitch_bend;
        self.tempo += other.tempo;
        self.meter += other.meter;
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExportWarning {
    MeterClocksFallback { source_third_payload: u8, used: u8 },
    MeterThirtySecondsFallback { source_fourth_payload: u8, used: u8 },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UntranslatedMetadata {
    ControllerContext { source_ordinal: u64 },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExportTrackResult {
    pub channel_assignment: ChannelAssignment,
    pub scheduled_events: Vec<ScheduledEvent>,
    pub counts: ExportCounts,
    pub warnings: Vec<ExportWarning>,
    pub untranslated_metadata: Vec<UntranslatedMetadata>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConductorResult {
    pub sequence_name: Vec<u8>,
    pub tempo_mpqn: u32,
    pub time_signature: TimeSignature,
    pub ppqn: u16,
    pub counts: ExportCounts,
    pub warnings: Vec<ExportWarning>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ExportReport {
    pub counts: ExportCounts,
    pub warnings: Vec<ExportWarning>,
    pub channel_assignments: Vec<ChannelAssignment>,
    pub untranslated_metadata: Vec<UntranslatedMetadata>,
}

impl ExportReport {
    pub fn include_conductor(&mut self, conductor: &ConductorResult) {
        self.counts.add_assign(&conductor.counts);
        self.warnings.extend(conductor.warnings.iter().cloned());
    }

    pub fn include_track(&mut self, track: &ExportTrackResult) {
        self.counts.add_assign(&track.counts);
        self.warnings.extend(track.warnings.iter().cloned());
        self.channel_assignments.push(track.channel_assignment);
        self.untranslated_metadata
            .extend(track.untranslated_metadata.iter().cloned());
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MidiExportError {
    UnsupportedTimingConversion,
    PositionOverflow {
        source_ordinal: u64,
        position: u32,
        duration: u32,
    },
    OrdinalOverflow {
        source_ordinal: u64,
    },
    DuplicateSourceOrdinal {
        source_ordinal: u64,
    },
    UnknownChannel,
    InvalidMidiValue {
        source_ordinal: Option<u64>,
        source_range: Option<Range<usize>>,
        source: SmfSerializeError,
    },
    UnsupportedPatchTranslation {
        source_ordinal: u64,
        source_range: Option<Range<usize>>,
    },
    UnsupportedMeterMapping {
        third_payload: u8,
    },
    TextConversion {
        reason: TextConversionReason,
    },
    UnsupportedEvent {
        source_ordinal: u64,
        family: &'static str,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextConversionReason {
    InteriorNul,
    MacRomanDeferred,
}

impl fmt::Display for MidiExportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "cannot adapt decoded MIDI export event: {self:?}"
        )
    }
}

impl std::error::Error for MidiExportError {}

/// Converts all events transactionally. No result is returned unless every
/// event validates and translates.
pub fn adapt_track(
    events: &[DecodedExportEvent],
    channel_assignment: Option<ChannelAssignment>,
    timing_policy: TimingPolicy,
    patch_policy: PatchPolicy,
) -> Result<ExportTrackResult, MidiExportError> {
    require_identity_timing(timing_policy)?;
    let channel_assignment = channel_assignment.ok_or(MidiExportError::UnknownChannel)?;
    let mut seen_ordinals = BTreeSet::new();
    let mut scheduled_events = Vec::new();
    let mut counts = ExportCounts::default();
    let warnings = Vec::new();
    let mut untranslated_metadata = Vec::new();

    for event in events {
        if !seen_ordinals.insert(event.source_ordinal) {
            return Err(MidiExportError::DuplicateSourceOrdinal {
                source_ordinal: event.source_ordinal,
            });
        }
        let source_stable_ordinal = source_ordinal(event.source_ordinal)?;
        let channel = channel_assignment.channel;
        match event.kind {
            DecodedExportEventKind::Note {
                pitch,
                attack_velocity,
                release_velocity,
                duration,
            } => {
                let pitch = midi_data(event, pitch)?;
                let attack_velocity = midi_data(event, attack_velocity)?;
                let release_velocity = midi_data(event, release_velocity)?;
                let end = event.absolute_position.checked_add(duration).ok_or(
                    MidiExportError::PositionOverflow {
                        source_ordinal: event.source_ordinal,
                        position: event.absolute_position,
                        duration,
                    },
                )?;
                scheduled_events.push(ScheduledEvent {
                    absolute_tick: event.absolute_position,
                    stable_ordinal: source_stable_ordinal,
                    message: crate::smf::ChannelMessage::NoteOn {
                        channel,
                        key: pitch,
                        attack_velocity,
                    },
                });
                scheduled_events.push(ScheduledEvent {
                    absolute_tick: end,
                    stable_ordinal: generated_ordinal(event.source_ordinal)?,
                    message: crate::smf::ChannelMessage::NoteOff {
                        channel,
                        key: pitch,
                        release_velocity,
                    },
                });
                counts.notes += 1;
                counts.generated_note_offs += 1;
            }
            DecodedExportEventKind::Controller {
                number,
                value,
                has_opaque_context,
            } => {
                scheduled_events.push(ScheduledEvent {
                    absolute_tick: event.absolute_position,
                    stable_ordinal: source_stable_ordinal,
                    message: crate::smf::ChannelMessage::ControlChange {
                        channel,
                        controller: midi_data(event, number)?,
                        value: midi_data(event, value)?,
                    },
                });
                counts.controllers += 1;
                if has_opaque_context {
                    untranslated_metadata.push(UntranslatedMetadata::ControllerContext {
                        source_ordinal: event.source_ordinal,
                    });
                }
            }
            DecodedExportEventKind::ChannelPressure { value } => {
                scheduled_events.push(ScheduledEvent {
                    absolute_tick: event.absolute_position,
                    stable_ordinal: source_stable_ordinal,
                    message: crate::smf::ChannelMessage::ChannelPressure {
                        channel,
                        pressure: midi_data(event, value)?,
                    },
                });
                counts.channel_pressure += 1;
            }
            DecodedExportEventKind::PitchBend { lsb, msb } => {
                scheduled_events.push(ScheduledEvent {
                    absolute_tick: event.absolute_position,
                    stable_ordinal: source_stable_ordinal,
                    message: crate::smf::ChannelMessage::PitchBend {
                        channel,
                        lsb: midi_data(event, lsb)?,
                        msb: midi_data(event, msb)?,
                    },
                });
                counts.pitch_bend += 1;
            }
            DecodedExportEventKind::Patch {
                program,
                translation,
            } => {
                if patch_policy != PatchPolicy::StrictKnownOnly {
                    return Err(MidiExportError::UnsupportedPatchTranslation {
                        source_ordinal: event.source_ordinal,
                        source_range: event.source_range.clone(),
                    });
                }
                if let PatchTranslation::ConfirmedBankSelect { msb, lsb } = translation {
                    scheduled_events.push(ScheduledEvent {
                        absolute_tick: event.absolute_position,
                        stable_ordinal: source_stable_ordinal,
                        message: crate::smf::ChannelMessage::ControlChange {
                            channel,
                            controller: midi_data(event, 0)?,
                            value: midi_data(event, msb)?,
                        },
                    });
                    scheduled_events.push(ScheduledEvent {
                        absolute_tick: event.absolute_position,
                        stable_ordinal: source_stable_ordinal,
                        message: crate::smf::ChannelMessage::ControlChange {
                            channel,
                            controller: midi_data(event, 32)?,
                            value: midi_data(event, lsb)?,
                        },
                    });
                    counts.bank_select_msb += 1;
                    counts.bank_select_lsb += 1;
                } else if translation == PatchTranslation::UnsupportedOpaque {
                    return Err(MidiExportError::UnsupportedPatchTranslation {
                        source_ordinal: event.source_ordinal,
                        source_range: event.source_range.clone(),
                    });
                }
                scheduled_events.push(ScheduledEvent {
                    absolute_tick: event.absolute_position,
                    stable_ordinal: source_stable_ordinal,
                    message: crate::smf::ChannelMessage::ProgramChange {
                        channel,
                        program: midi_data(event, program)?,
                    },
                });
                counts.program_changes += 1;
            }
            DecodedExportEventKind::Unsupported { family } => {
                return Err(MidiExportError::UnsupportedEvent {
                    source_ordinal: event.source_ordinal,
                    family,
                });
            }
        }
    }

    Ok(ExportTrackResult {
        channel_assignment,
        scheduled_events,
        counts,
        warnings,
        untranslated_metadata,
    })
}

pub fn adapt_meter(
    meter: &InitialMeterEvent,
    policy: MeterPolicy,
) -> Result<(TimeSignature, Vec<ExportWarning>), MidiExportError> {
    adapt_meter_values(
        meter.numerator.value,
        meter.denominator_exponent.value,
        meter.third_payload.value,
        meter.fourth_payload.value,
        policy,
    )
}

pub fn adapt_meter_values(
    numerator: u8,
    denominator_exponent: u8,
    third_payload: u8,
    fourth_payload: u8,
    policy: MeterPolicy,
) -> Result<(TimeSignature, Vec<ExportWarning>), MidiExportError> {
    let mut warnings = Vec::new();
    let clocks = match third_payload {
        8 => 24,
        6 => 12,
        unknown if policy == MeterPolicy::KnownHistoricalOnly => {
            return Err(MidiExportError::UnsupportedMeterMapping {
                third_payload: unknown,
            });
        }
        unknown => {
            warnings.push(ExportWarning::MeterClocksFallback {
                source_third_payload: unknown,
                used: 24,
            });
            24
        }
    };
    let thirty_seconds = if fourth_payload <= 0x7f {
        fourth_payload
    } else {
        warnings.push(ExportWarning::MeterThirtySecondsFallback {
            source_fourth_payload: fourth_payload,
            used: 8,
        });
        8
    };
    let signature = TimeSignature::new(
        midi_data_without_event(numerator)?,
        midi_data_without_event(denominator_exponent)?,
        midi_data_without_event(clocks)?,
        midi_data_without_event(thirty_seconds)?,
    )
    .map_err(|source| MidiExportError::InvalidMidiValue {
        source_ordinal: None,
        source_range: None,
        source,
    })?;
    Ok((signature, warnings))
}

pub fn adapt_tempo(tempo: &InitialTempoEvent) -> Result<u32, MidiExportError> {
    adapt_tempo_mpqn(tempo.mpqn())
}

pub fn adapt_tempo_mpqn(mpqn: u32) -> Result<u32, MidiExportError> {
    if mpqn == 0 || mpqn > 0x00ff_ffff {
        return Err(MidiExportError::InvalidMidiValue {
            source_ordinal: None,
            source_range: None,
            source: SmfSerializeError::InvalidTempo { mpqn },
        });
    }
    Ok(mpqn)
}

pub fn adapt_text(raw: &[u8]) -> Result<Vec<u8>, MidiExportError> {
    if raw.contains(&0) {
        return Err(MidiExportError::TextConversion {
            reason: TextConversionReason::InteriorNul,
        });
    }
    std::str::from_utf8(raw)
        .map(|_| raw.to_vec())
        .map_err(|_| MidiExportError::TextConversion {
            reason: TextConversionReason::MacRomanDeferred,
        })
}

pub fn adapt_conductor(
    raw_sequence_name: &[u8],
    tempo_mpqn: u32,
    meter_values: (u8, u8, u8, u8),
    timing_policy: TimingPolicy,
    meter_policy: MeterPolicy,
) -> Result<ConductorResult, MidiExportError> {
    let ppqn = require_identity_timing(timing_policy)?;
    let sequence_name = adapt_text(raw_sequence_name)?;
    let tempo_mpqn = adapt_tempo_mpqn(tempo_mpqn)?;
    let (time_signature, warnings) = adapt_meter_values(
        meter_values.0,
        meter_values.1,
        meter_values.2,
        meter_values.3,
        meter_policy,
    )?;
    Ok(ConductorResult {
        sequence_name,
        tempo_mpqn,
        time_signature,
        ppqn,
        counts: ExportCounts {
            tempo: 1,
            meter: 1,
            ..ExportCounts::default()
        },
        warnings,
    })
}

fn require_identity_timing(policy: TimingPolicy) -> Result<u16, MidiExportError> {
    match policy {
        TimingPolicy::Identity480 => Ok(IDENTITY_480_PPQN),
        TimingPolicy::Unsupported => Err(MidiExportError::UnsupportedTimingConversion),
    }
}

fn source_ordinal(source_ordinal: u64) -> Result<u64, MidiExportError> {
    source_ordinal
        .checked_mul(2)
        .ok_or(MidiExportError::OrdinalOverflow { source_ordinal })
}

fn generated_ordinal(source_ordinal: u64) -> Result<u64, MidiExportError> {
    source_ordinal
        .checked_mul(2)
        .and_then(|value| value.checked_add(1))
        .ok_or(MidiExportError::OrdinalOverflow { source_ordinal })
}

fn midi_data(event: &DecodedExportEvent, value: u8) -> Result<MidiDataByte, MidiExportError> {
    MidiDataByte::new(value).map_err(|source| MidiExportError::InvalidMidiValue {
        source_ordinal: Some(event.source_ordinal),
        source_range: event.source_range.clone(),
        source,
    })
}

fn midi_data_without_event(value: u8) -> Result<MidiDataByte, MidiExportError> {
    MidiDataByte::new(value).map_err(|source| MidiExportError::InvalidMidiValue {
        source_ordinal: None,
        source_range: None,
        source,
    })
}
