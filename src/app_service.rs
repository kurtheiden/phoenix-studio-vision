//! Owned application-service operations for project inspection.
//!
//! This layer deliberately stops at inspection.  It owns file/session state
//! and translates the established parser's results into application DTOs, but
//! it does not expose parser structures or choose an export profile.

use crate::app_contract::{
    ApiInfo, AppError, AppErrorCategory, AppOperation, Diagnostics, DiagnosticsLevel,
    EventFamilySummary, InspectProjectRequest, InspectProjectResponse, ProjectSummary, Readiness,
    ReadinessReason, ReadinessReasonCode, SequenceId, SequenceSummary, SessionId, Warning,
    WarningScope, WarningSeverity, CONTRACT_VERSION,
};
use crate::identification::{identify, read_finder_metadata};
use crate::inspection::inspect;
use crate::sequence_container::{parse_project_166, TrackAssociations};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Clone)]
struct Session {
    source_path: String,
    source_bytes: Vec<u8>,
    source_sha256: String,
    response: InspectProjectResponse,
    diagnostics: Diagnostics,
}

/// Synchronous, owned service state for one or more inspection sessions.
pub struct AppService {
    sessions: HashMap<SessionId, Session>,
    next_session: u64,
}

impl Default for AppService {
    fn default() -> Self {
        Self::new()
    }
}

impl AppService {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            next_session: 1,
        }
    }

    pub fn api_info(&self) -> ApiInfo {
        ApiInfo::new(env!("CARGO_PKG_VERSION"))
    }

    pub fn session_count(&self) -> usize {
        self.sessions.len()
    }

    #[allow(clippy::result_large_err)]
    pub fn inspect_project(
        &mut self,
        request: InspectProjectRequest,
    ) -> Result<InspectProjectResponse, AppError> {
        if request.contract_version != CONTRACT_VERSION {
            return Err(self.error(
                AppErrorCategory::InternalError,
                "This request uses an unsupported Phoenix contract version.",
                format!(
                    "requested contract version {}; supported version {}",
                    request.contract_version, CONTRACT_VERSION
                ),
                AppOperation::InspectProject,
                "contract_version_mismatch",
                None,
                None,
            ));
        }

        let path = PathBuf::from(&request.source_path);
        let inspection = inspect(&path).map_err(|error| {
            self.error(
                AppErrorCategory::FileUnreadable,
                "Phoenix could not read the selected file.",
                error.to_string(),
                AppOperation::InspectProject,
                "file_read_failed",
                None,
                None,
            )
        })?;
        let bytes = fs::read(&path).map_err(|error| {
            self.error(
                AppErrorCategory::FileUnreadable,
                "Phoenix could not read the selected file.",
                error.to_string(),
                AppOperation::InspectProject,
                "file_read_failed",
                None,
                None,
            )
        })?;

        let finder = identify(read_finder_metadata(&inspection.full_path));
        let finder_recognized = !finder.confidence.to_string().eq("Unknown");
        let session_id = self.allocate_session_id();
        let (project, sequences, warnings, diagnostics) = match parse_project_166(&bytes) {
            Ok(parsed) => self.build_parsed_result(
                &inspection.filename,
                inspection.size,
                &finder,
                parsed.sequences,
                &session_id,
                request.diagnostics_level,
            ),
            Err(error) if finder_recognized => self.build_profile_failure(
                &inspection.filename,
                inspection.size,
                &finder,
                format!("166-byte profile rejected: {error}"),
                &session_id,
            ),
            Err(error) => self.build_unrecognized_result(
                &inspection.filename,
                inspection.size,
                &finder,
                format!("no established Studio Vision profile accepted the input: {error}"),
                &session_id,
            ),
        };
        let response = InspectProjectResponse {
            contract_version: CONTRACT_VERSION,
            session_id: session_id.clone(),
            project,
            sequences,
            warnings,
            diagnostics_available: true,
        };
        let mut diagnostics = diagnostics;
        diagnostics.source_sha256 = Some(inspection.sha256.clone());
        self.sessions.insert(
            session_id.clone(),
            Session {
                source_path: request.source_path,
                source_bytes: bytes,
                source_sha256: inspection.sha256,
                response: response.clone(),
                diagnostics,
            },
        );
        Ok(response)
    }

    #[allow(clippy::result_large_err)]
    pub fn get_inspection(
        &self,
        session_id: &SessionId,
    ) -> Result<InspectProjectResponse, AppError> {
        self.sessions
            .get(session_id)
            .map(|session| session.response.clone())
            .ok_or_else(|| self.unknown_session(AppOperation::GetDiagnostics))
    }

    #[allow(clippy::result_large_err)]
    pub fn get_diagnostics(
        &self,
        session_id: &SessionId,
        level: DiagnosticsLevel,
    ) -> Result<Diagnostics, AppError> {
        let session = self
            .sessions
            .get(session_id)
            .ok_or_else(|| self.unknown_session(AppOperation::GetDiagnostics))?;
        let mut diagnostics = session.diagnostics.clone();
        if matches!(level, DiagnosticsLevel::None) {
            diagnostics.source_sha256 = None;
            diagnostics.identification_evidence.clear();
            diagnostics.technical_errors.clear();
        } else if matches!(level, DiagnosticsLevel::Summary) {
            diagnostics.source_sha256 = None;
            diagnostics.technical_errors.clear();
        }
        Ok(diagnostics)
    }

    /// Source identity retained for the future export revalidation seam.
    #[allow(clippy::result_large_err)]
    pub fn source_identity(
        &self,
        session_id: &SessionId,
    ) -> Result<(String, u64, String), AppError> {
        let session = self
            .sessions
            .get(session_id)
            .ok_or_else(|| self.unknown_session(AppOperation::GetDiagnostics))?;
        Ok((
            session.source_path.clone(),
            session.source_bytes.len() as u64,
            session.source_sha256.clone(),
        ))
    }

    fn allocate_session_id(&mut self) -> SessionId {
        let value = format!("session-{:08}", self.next_session);
        self.next_session = self.next_session.saturating_add(1);
        SessionId::new(value)
    }

    fn build_parsed_result<'a>(
        &self,
        filename: &str,
        size: u64,
        finder: &crate::identification::Identification,
        sequences: Vec<crate::sequence_container::SequenceContainer<'a>>,
        session_id: &SessionId,
        _level: DiagnosticsLevel,
    ) -> (
        ProjectSummary,
        Vec<SequenceSummary>,
        Vec<Warning>,
        Diagnostics,
    ) {
        let mut summaries = Vec::with_capacity(sequences.len());
        let mut warnings = Vec::new();
        let mut technical_errors = Vec::new();
        for (index, sequence) in sequences.iter().enumerate() {
            let name = sequence
                .sequence_name
                .as_utf8()
                .map(str::to_owned)
                .unwrap_or_else(|| "Unnamed Sequence".to_owned());
            let (readiness, reason) = match sequence.track_associations {
                TrackAssociations::Ordinal(_) => (
                    Readiness::PartiallySupported,
                    ReadinessReason::new(
                        ReadinessReasonCode::MissingChannelRouting,
                        "Phoenix can inspect this sequence, but general MIDI routing is not established.",
                    ),
                ),
                TrackAssociations::Unresolved { .. } => (
                    Readiness::PartiallySupported,
                    ReadinessReason::new(
                        ReadinessReasonCode::IncompleteSequenceStructure,
                        "Phoenix found the sequence but could not safely bind every track structure.",
                    ),
                ),
            };
            warnings.push(Warning {
                code: "missing_channel_routing".into(),
                message: "This sequence is inspectable but is not generally export-ready.".into(),
                technical_detail: Some(reason.display_detail.clone()),
                scope: WarningScope::Sequence,
                severity: WarningSeverity::DataLossRisk,
                diagnostic_ref: Some(format!("sequence-{index}")),
                source_order: index as u32,
            });
            summaries.push(SequenceSummary {
                sequence_id: SequenceId::new(format!(
                    "{}-sequence-{:04}",
                    session_id.as_str(),
                    index + 1
                )),
                display_name: name,
                readiness,
                readiness_reason: reason,
                musical_track_count: Some(sequence.track_pairs.len() as u32),
                supported_event_families: Vec::<EventFamilySummary>::new(),
                warning_count: 1,
                export_capability: None,
                diagnostics_available: true,
            });
        }
        warnings.sort_by(crate::app_contract::compare_warnings);
        let overall = overall_readiness(&summaries);
        if summaries.is_empty() {
            technical_errors.push("recognized profile contained no sequences".into());
        }
        let project = ProjectSummary {
            display_name: filename.to_owned(),
            byte_size: size,
            identification: identification_summary(finder, true),
            recognized_studio_vision: true,
            profile_label: Some("Descriptor166".into()),
            sequence_count: summaries.len() as u32,
            overall_readiness: overall,
            warning_count: warnings.len() as u32,
            diagnostics_available: true,
        };
        let diagnostics = Diagnostics {
            core_version: env!("CARGO_PKG_VERSION").into(),
            contract_version: CONTRACT_VERSION,
            source_sha256: None,
            identification_evidence: finder.evidence.clone(),
            recognized_profile: Some("Descriptor166".into()),
            structural_status: Some("parsed successfully".into()),
            unsupported_families: Vec::new(),
            compatibility_profile: None,
            technical_errors,
            export_report: None,
        };
        (project, summaries, warnings, diagnostics)
    }

    fn build_profile_failure(
        &self,
        filename: &str,
        size: u64,
        finder: &crate::identification::Identification,
        detail: String,
        _session_id: &SessionId,
    ) -> (
        ProjectSummary,
        Vec<SequenceSummary>,
        Vec<Warning>,
        Diagnostics,
    ) {
        self.build_failure_result(
            filename,
            size,
            finder,
            true,
            detail,
            ReadinessReasonCode::UnsupportedProjectProfile,
        )
    }

    fn build_unrecognized_result(
        &self,
        filename: &str,
        size: u64,
        finder: &crate::identification::Identification,
        detail: String,
        _session_id: &SessionId,
    ) -> (
        ProjectSummary,
        Vec<SequenceSummary>,
        Vec<Warning>,
        Diagnostics,
    ) {
        self.build_failure_result(
            filename,
            size,
            finder,
            false,
            detail,
            ReadinessReasonCode::UnknownStructure,
        )
    }

    fn build_failure_result(
        &self,
        filename: &str,
        size: u64,
        finder: &crate::identification::Identification,
        recognized: bool,
        detail: String,
        reason_code: ReadinessReasonCode,
    ) -> (
        ProjectSummary,
        Vec<SequenceSummary>,
        Vec<Warning>,
        Diagnostics,
    ) {
        let message = if recognized {
            "Phoenix recognized the file metadata, but its established project profile could not parse it."
        } else {
            "Phoenix could read the file, but it did not recognize an established Studio Vision profile."
        };
        let warning = Warning {
            code: "unsupported_project_profile".into(),
            message: message.into(),
            technical_detail: Some(detail.clone()),
            scope: WarningScope::Project,
            severity: WarningSeverity::Caution,
            diagnostic_ref: Some("inspection-parse".into()),
            source_order: 0,
        };
        let project = ProjectSummary {
            display_name: filename.to_owned(),
            byte_size: size,
            identification: identification_summary(finder, recognized),
            recognized_studio_vision: recognized,
            profile_label: None,
            sequence_count: 0,
            overall_readiness: if recognized {
                Readiness::Unsupported
            } else {
                Readiness::Unknown
            },
            warning_count: 1,
            diagnostics_available: true,
        };
        let diagnostics = Diagnostics {
            core_version: env!("CARGO_PKG_VERSION").into(),
            contract_version: CONTRACT_VERSION,
            source_sha256: None,
            identification_evidence: finder.evidence.clone(),
            recognized_profile: None,
            structural_status: Some(detail.clone()),
            unsupported_families: Vec::new(),
            compatibility_profile: None,
            technical_errors: vec![detail],
            export_report: None,
        };
        let mut reason = ReadinessReason::new(reason_code, message);
        reason.diagnostic_ref = Some("inspection-parse".into());
        (project, Vec::new(), vec![warning], diagnostics)
    }

    #[allow(clippy::too_many_arguments)]
    fn error(
        &self,
        category: AppErrorCategory,
        display: &str,
        technical: String,
        operation: AppOperation,
        code: &str,
        session_id: Option<SessionId>,
        sequence_id: Option<SequenceId>,
    ) -> AppError {
        AppError {
            contract_version: CONTRACT_VERSION,
            category,
            display_message: display.into(),
            technical_message: technical,
            operation,
            session_id,
            sequence_id,
            diagnostic_code: code.into(),
            diagnostic_ref: None,
        }
    }

    fn unknown_session(&self, operation: AppOperation) -> AppError {
        self.error(
            AppErrorCategory::InternalError,
            "The Phoenix inspection session is no longer available.",
            "no session exists for the supplied session identifier".into(),
            operation,
            "unknown_session",
            None,
            None,
        )
    }
}

fn identification_summary(
    identification: &crate::identification::Identification,
    recognized: bool,
) -> crate::app_contract::IdentificationSummary {
    crate::app_contract::IdentificationSummary {
        recognized,
        label: if recognized {
            "Studio Vision".into()
        } else {
            "Unrecognized".into()
        },
        confidence: identification.confidence.to_string(),
        profile_label: None,
    }
}

fn overall_readiness(sequences: &[SequenceSummary]) -> Readiness {
    if sequences.iter().any(|s| s.readiness == Readiness::Ready) {
        Readiness::Ready
    } else if sequences
        .iter()
        .any(|s| s.readiness == Readiness::PartiallySupported)
    {
        Readiness::PartiallySupported
    } else if sequences
        .iter()
        .any(|s| s.readiness == Readiness::Unsupported)
    {
        Readiness::Unsupported
    } else {
        Readiness::Unknown
    }
}
