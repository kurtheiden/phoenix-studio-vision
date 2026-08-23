//! Owned, transport-independent application contract DTOs.
//!
//! This module deliberately contains no parser, Studio Vision, or MIDI
//! implementation types. It is the domain model a future application service
//! can map into a Swift/FFI transport.

use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

pub const CONTRACT_VERSION: u32 = 1;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ApiInfo {
    pub contract_version: u32,
    pub core_version: String,
}

impl ApiInfo {
    pub fn new(core_version: impl Into<String>) -> Self {
        Self {
            contract_version: CONTRACT_VERSION,
            core_version: core_version.into(),
        }
    }
}

macro_rules! opaque_id {
    ($name:ident) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

opaque_id!(SessionId);
opaque_id!(SequenceId);
opaque_id!(OperationId);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Readiness {
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "partially_supported")]
    PartiallySupported,
    #[serde(rename = "unsupported")]
    Unsupported,
    #[serde(rename = "unknown")]
    Unknown,
}

impl Readiness {
    pub const fn stable_code(self) -> u32 {
        match self {
            Self::Ready => 1,
            Self::PartiallySupported => 2,
            Self::Unsupported => 3,
            Self::Unknown => 4,
        }
    }

    pub const fn stable_name(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::PartiallySupported => "partially_supported",
            Self::Unsupported => "unsupported",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum ReasonSeverity {
    #[serde(rename = "informational")]
    Informational,
    #[serde(rename = "caution")]
    Caution,
    #[serde(rename = "data_loss_risk")]
    DataLossRisk,
}

impl ReasonSeverity {
    pub const fn stable_code(self) -> u32 {
        match self {
            Self::Informational => 1,
            Self::Caution => 2,
            Self::DataLossRisk => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum ReadinessReasonCode {
    #[serde(rename = "validated_compatibility_profile")]
    ValidatedCompatibilityProfile,
    #[serde(rename = "missing_channel_routing")]
    MissingChannelRouting,
    #[serde(rename = "unsupported_event_family")]
    UnsupportedEventFamily,
    #[serde(rename = "unsupported_patch_translation")]
    UnsupportedPatchTranslation,
    #[serde(rename = "unsupported_project_profile")]
    UnsupportedProjectProfile,
    #[serde(rename = "incomplete_sequence_structure")]
    IncompleteSequenceStructure,
    #[serde(rename = "unknown_structure")]
    UnknownStructure,
}

impl ReadinessReasonCode {
    pub const fn stable_code(self) -> u32 {
        match self {
            Self::ValidatedCompatibilityProfile => 1,
            Self::MissingChannelRouting => 2,
            Self::UnsupportedEventFamily => 3,
            Self::UnsupportedPatchTranslation => 4,
            Self::UnsupportedProjectProfile => 5,
            Self::IncompleteSequenceStructure => 6,
            Self::UnknownStructure => 7,
        }
    }

    pub const fn stable_name(self) -> &'static str {
        match self {
            Self::ValidatedCompatibilityProfile => "validated_compatibility_profile",
            Self::MissingChannelRouting => "missing_channel_routing",
            Self::UnsupportedEventFamily => "unsupported_event_family",
            Self::UnsupportedPatchTranslation => "unsupported_patch_translation",
            Self::UnsupportedProjectProfile => "unsupported_project_profile",
            Self::IncompleteSequenceStructure => "incomplete_sequence_structure",
            Self::UnknownStructure => "unknown_structure",
        }
    }

    pub const fn default_severity(self) -> ReasonSeverity {
        match self {
            Self::ValidatedCompatibilityProfile => ReasonSeverity::Informational,
            Self::MissingChannelRouting
            | Self::UnsupportedEventFamily
            | Self::UnsupportedPatchTranslation => ReasonSeverity::DataLossRisk,
            Self::UnsupportedProjectProfile | Self::IncompleteSequenceStructure => {
                ReasonSeverity::Caution
            }
            Self::UnknownStructure => ReasonSeverity::Caution,
        }
    }

    pub const fn export_enabled(self) -> bool {
        matches!(self, Self::ValidatedCompatibilityProfile)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ReadinessReason {
    pub code: ReadinessReasonCode,
    pub severity: ReasonSeverity,
    pub export_enabled: bool,
    pub display_detail: String,
    pub diagnostic_ref: Option<String>,
}

impl ReadinessReason {
    pub fn new(code: ReadinessReasonCode, display_detail: impl Into<String>) -> Self {
        Self {
            code,
            severity: code.default_severity(),
            export_enabled: code.export_enabled(),
            display_detail: display_detail.into(),
            diagnostic_ref: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum WarningSeverity {
    #[serde(rename = "informational")]
    Informational,
    #[serde(rename = "caution")]
    Caution,
    #[serde(rename = "data_loss_risk")]
    DataLossRisk,
}

impl WarningSeverity {
    pub const fn stable_code(self) -> u32 {
        match self {
            Self::Informational => 1,
            Self::Caution => 2,
            Self::DataLossRisk => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum WarningScope {
    #[serde(rename = "project")]
    Project,
    #[serde(rename = "sequence")]
    Sequence,
    #[serde(rename = "generic_track")]
    GenericTrack,
}

impl WarningScope {
    pub const fn stable_code(self) -> u32 {
        match self {
            Self::Project => 1,
            Self::Sequence => 2,
            Self::GenericTrack => 3,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Warning {
    pub code: String,
    pub message: String,
    pub technical_detail: Option<String>,
    pub scope: WarningScope,
    pub severity: WarningSeverity,
    pub diagnostic_ref: Option<String>,
    pub source_order: u32,
}

impl Warning {
    pub fn sort_key(&self) -> (u32, &str, u32) {
        (self.scope.stable_code(), &self.code, self.source_order)
    }
}

pub fn compare_warnings(left: &Warning, right: &Warning) -> Ordering {
    left.sort_key().cmp(&right.sort_key())
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct IdentificationSummary {
    pub recognized: bool,
    pub label: String,
    pub confidence: String,
    pub profile_label: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ProjectSummary {
    pub display_name: String,
    pub byte_size: u64,
    pub identification: IdentificationSummary,
    pub recognized_studio_vision: bool,
    pub profile_label: Option<String>,
    pub sequence_count: u32,
    pub overall_readiness: Readiness,
    pub warning_count: u32,
    pub diagnostics_available: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum EventFamily {
    #[serde(rename = "note")]
    Note,
    #[serde(rename = "controller")]
    Controller,
    #[serde(rename = "program_change")]
    ProgramChange,
    #[serde(rename = "channel_pressure")]
    ChannelPressure,
    #[serde(rename = "pitch_bend")]
    PitchBend,
    #[serde(rename = "tempo")]
    Tempo,
    #[serde(rename = "meter")]
    Meter,
}

impl EventFamily {
    pub const fn stable_code(self) -> u32 {
        match self {
            Self::Note => 1,
            Self::Controller => 2,
            Self::ProgramChange => 3,
            Self::ChannelPressure => 4,
            Self::PitchBend => 5,
            Self::Tempo => 6,
            Self::Meter => 7,
        }
    }

    pub const fn stable_name(self) -> &'static str {
        match self {
            Self::Note => "note",
            Self::Controller => "controller",
            Self::ProgramChange => "program_change",
            Self::ChannelPressure => "channel_pressure",
            Self::PitchBend => "pitch_bend",
            Self::Tempo => "tempo",
            Self::Meter => "meter",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct EventFamilySummary {
    pub family: EventFamily,
    pub count: u64,
    pub supported: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ProfileCapability {
    pub profile_id: String,
    pub profile_version: u32,
    pub display_label: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct SequenceSummary {
    pub sequence_id: SequenceId,
    pub display_name: String,
    pub readiness: Readiness,
    pub readiness_reason: ReadinessReason,
    pub musical_track_count: Option<u32>,
    pub supported_event_families: Vec<EventFamilySummary>,
    pub warning_count: u32,
    pub export_capability: Option<ProfileCapability>,
    pub diagnostics_available: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum DiagnosticsLevel {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "summary")]
    Summary,
    #[serde(rename = "full")]
    Full,
}

impl DiagnosticsLevel {
    pub const fn stable_code(self) -> u32 {
        match self {
            Self::None => 1,
            Self::Summary => 2,
            Self::Full => 3,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InspectProjectRequest {
    pub contract_version: u32,
    pub source_path: String,
    pub diagnostics_level: DiagnosticsLevel,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct InspectProjectResponse {
    pub contract_version: u32,
    pub session_id: SessionId,
    pub project: ProjectSummary,
    pub sequences: Vec<SequenceSummary>,
    pub warnings: Vec<Warning>,
    pub diagnostics_available: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Diagnostics {
    pub core_version: String,
    pub contract_version: u32,
    pub source_sha256: Option<String>,
    pub identification_evidence: Vec<String>,
    pub recognized_profile: Option<String>,
    pub structural_status: Option<String>,
    pub unsupported_families: Vec<String>,
    pub compatibility_profile: Option<ProfileCapability>,
    pub technical_errors: Vec<String>,
    pub export_report: Option<ExportSequenceResponse>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
pub enum CollisionPolicy {
    #[serde(rename = "fail_if_exists")]
    FailIfExists,
    #[serde(rename = "generate_unique_name")]
    GenerateUniqueName,
}

impl CollisionPolicy {
    pub const fn stable_code(self) -> u32 {
        match self {
            Self::FailIfExists => 1,
            Self::GenerateUniqueName => 2,
        }
    }

    pub const fn stable_name(self) -> &'static str {
        match self {
            Self::FailIfExists => "fail_if_exists",
            Self::GenerateUniqueName => "generate_unique_name",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExportSequenceRequest {
    pub contract_version: u32,
    pub session_id: SessionId,
    pub sequence_id: SequenceId,
    pub destination_folder: String,
    pub filename_stem: String,
    pub collision_policy: CollisionPolicy,
    pub operation_id: Option<OperationId>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct ExportCounts {
    pub notes: u64,
    pub generated_note_offs: u64,
    pub controllers: u64,
    pub bank_select_msb: u64,
    pub bank_select_lsb: u64,
    pub programs: u64,
    pub pressure: u64,
    pub pitch_bend: u64,
    pub tempo: u64,
    pub meter: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CountOverflow;

impl ExportCounts {
    pub fn checked_add(&self, other: &Self) -> Result<Self, CountOverflow> {
        Ok(Self {
            notes: self.notes.checked_add(other.notes).ok_or(CountOverflow)?,
            generated_note_offs: self
                .generated_note_offs
                .checked_add(other.generated_note_offs)
                .ok_or(CountOverflow)?,
            controllers: self
                .controllers
                .checked_add(other.controllers)
                .ok_or(CountOverflow)?,
            bank_select_msb: self
                .bank_select_msb
                .checked_add(other.bank_select_msb)
                .ok_or(CountOverflow)?,
            bank_select_lsb: self
                .bank_select_lsb
                .checked_add(other.bank_select_lsb)
                .ok_or(CountOverflow)?,
            programs: self
                .programs
                .checked_add(other.programs)
                .ok_or(CountOverflow)?,
            pressure: self
                .pressure
                .checked_add(other.pressure)
                .ok_or(CountOverflow)?,
            pitch_bend: self
                .pitch_bend
                .checked_add(other.pitch_bend)
                .ok_or(CountOverflow)?,
            tempo: self.tempo.checked_add(other.tempo).ok_or(CountOverflow)?,
            meter: self.meter.checked_add(other.meter).ok_or(CountOverflow)?,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum ValidationStatus {
    #[serde(rename = "validated")]
    Validated,
}

impl ValidationStatus {
    pub const fn stable_code(self) -> u32 {
        1
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ExportSequenceResponse {
    pub session_id: SessionId,
    pub sequence_id: SequenceId,
    pub sequence_display_name: String,
    pub output_path: String,
    pub compatibility_profile: Option<ProfileCapability>,
    pub musical_track_count: u32,
    pub total_smf_track_count: u32,
    pub counts: ExportCounts,
    pub warnings: Vec<Warning>,
    pub untranslated_metadata_count: u64,
    pub validation_status: ValidationStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioReferenceStatus {
    Unknown,
    Missing,
    Present,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProvenanceConfidence {
    Unknown,
    Provisional,
    Established,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AudioReferenceSummary {
    pub display_name: String,
    pub path_hint: Option<String>,
    pub status: AudioReferenceStatus,
    pub provenance_confidence: ProvenanceConfidence,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum AppErrorCategory {
    #[serde(rename = "file_unreadable")]
    FileUnreadable,
    #[serde(rename = "not_recognized")]
    NotRecognized,
    #[serde(rename = "unsupported_profile")]
    UnsupportedProfile,
    #[serde(rename = "sequence_unsupported")]
    SequenceUnsupported,
    #[serde(rename = "missing_routing")]
    MissingRouting,
    #[serde(rename = "unsupported_event_family")]
    UnsupportedEventFamily,
    #[serde(rename = "export_validation_failed")]
    ExportValidationFailed,
    #[serde(rename = "destination_exists")]
    DestinationExists,
    #[serde(rename = "output_io_failed")]
    OutputIoFailed,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "internal_error")]
    InternalError,
}

impl AppErrorCategory {
    pub const fn stable_code(self) -> u32 {
        match self {
            Self::FileUnreadable => 1,
            Self::NotRecognized => 2,
            Self::UnsupportedProfile => 3,
            Self::SequenceUnsupported => 4,
            Self::MissingRouting => 5,
            Self::UnsupportedEventFamily => 6,
            Self::ExportValidationFailed => 7,
            Self::DestinationExists => 8,
            Self::OutputIoFailed => 9,
            Self::Cancelled => 10,
            Self::InternalError => 11,
        }
    }

    pub const fn stable_name(self) -> &'static str {
        match self {
            Self::FileUnreadable => "file_unreadable",
            Self::NotRecognized => "not_recognized",
            Self::UnsupportedProfile => "unsupported_profile",
            Self::SequenceUnsupported => "sequence_unsupported",
            Self::MissingRouting => "missing_routing",
            Self::UnsupportedEventFamily => "unsupported_event_family",
            Self::ExportValidationFailed => "export_validation_failed",
            Self::DestinationExists => "destination_exists",
            Self::OutputIoFailed => "output_io_failed",
            Self::Cancelled => "cancelled",
            Self::InternalError => "internal_error",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum AppOperation {
    #[serde(rename = "get_api_info")]
    GetApiInfo,
    #[serde(rename = "inspect_project")]
    InspectProject,
    #[serde(rename = "get_diagnostics")]
    GetDiagnostics,
    #[serde(rename = "export_sequence")]
    ExportSequence,
    #[serde(rename = "cancel_operation")]
    CancelOperation,
}

impl AppOperation {
    pub const fn stable_code(self) -> u32 {
        match self {
            Self::GetApiInfo => 1,
            Self::InspectProject => 2,
            Self::GetDiagnostics => 3,
            Self::ExportSequence => 4,
            Self::CancelOperation => 5,
        }
    }

    pub const fn stable_name(self) -> &'static str {
        match self {
            Self::GetApiInfo => "get_api_info",
            Self::InspectProject => "inspect_project",
            Self::GetDiagnostics => "get_diagnostics",
            Self::ExportSequence => "export_sequence",
            Self::CancelOperation => "cancel_operation",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AppError {
    pub contract_version: u32,
    pub category: AppErrorCategory,
    pub display_message: String,
    pub technical_message: String,
    pub operation: AppOperation,
    pub session_id: Option<SessionId>,
    pub sequence_id: Option<SequenceId>,
    pub diagnostic_code: String,
    pub diagnostic_ref: Option<String>,
}
