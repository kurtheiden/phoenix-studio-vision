//! Owned application-service operations for project inspection and export.
//!
//! This layer owns file/session state, translates parser results into
//! application DTOs, prepares authorized exports in memory, and commits the
//! public single-sequence export destination transaction. It does not expose
//! parser structures or own broader UI/FFI behavior.

use crate::app_contract::{
    ApiInfo, AppError, AppErrorCategory, AppOperation, CollisionPolicy, Diagnostics,
    DiagnosticsLevel, EventFamilySummary, ExportCounts as AppExportCounts, ExportSequenceRequest,
    ExportSequenceResponse, InspectProjectRequest, InspectProjectResponse, OperationId,
    ProfileCapability, ProjectSummary, Readiness, ReadinessReason, ReadinessReasonCode, SequenceId,
    SequenceSummary, SessionId, ValidationStatus, Warning, WarningScope, WarningSeverity,
    CONTRACT_VERSION,
};
use crate::compatibility::{
    ByteRange, CompatibilityRegistry, EvidenceEventFamily, ParserProfileId, PatchEvidence,
    ProfileEvidence, ProfileMatch, ResolvedProfilePolicy, SequenceEvidence, TrackEvidence,
};
use crate::export_handoff::build_conversion_ready_sequence;
use crate::identification::{identify, read_finder_metadata};
use crate::inspection::inspect;
use crate::midi_export::ExportWarning;
use crate::mixed_event::{
    walk_bounded_mixed_events, MixedEventBounds, MixedEventItem, MixedEventKind,
    MixedEventTimingBasis,
};
use crate::multitrack_export::{assemble_multitrack_sequence, MultitrackExportResult};
use crate::sequence_container::{parse_project_166, TrackAssociations};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, PartialEq, Eq)]
enum SequenceMatchState {
    NoMatch,
    Matched,
    Rejected,
    RegistryError,
}

#[derive(Clone)]
struct SequenceAssessment {
    structural_ordinal: u32,
    #[allow(dead_code)]
    generic_readiness: Readiness,
    match_state: SequenceMatchState,
    capability: Option<crate::app_contract::ProfileCapability>,
    resolved_policy: Option<ResolvedProfilePolicy>,
    diagnostic_code: Option<String>,
    #[allow(dead_code)]
    technical_detail: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SequenceAssessmentKind {
    NoMatch,
    Matched,
    Rejected,
    RegistryError,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SequenceAssessmentStatus {
    pub structural_ordinal: u32,
    pub match_kind: SequenceAssessmentKind,
    pub capability: Option<crate::app_contract::ProfileCapability>,
    pub has_resolved_policy: bool,
    pub diagnostic_code: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SequenceRevalidationKind {
    Validated,
    NoStoredPolicy,
    SourceUnreadable,
    SourceIdentityChanged,
    SequenceIdentityChanged,
    ProfileNoLongerMatches,
    ProfileRejected,
    RegistryError,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SequenceRevalidationStatus {
    pub kind: SequenceRevalidationKind,
    pub source_sha256: Option<String>,
    pub capability: Option<crate::app_contract::ProfileCapability>,
    pub diagnostic_code: String,
}

/// Fresh, owned state that may be handed to a later Core export operation.
/// It is only constructed after source identity, structure, and policy have
/// all been revalidated in one call.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct FreshValidatedSequence {
    pub(crate) source_bytes: Vec<u8>,
    pub(crate) source_sha256: String,
    pub(crate) structural_ordinal: u32,
    pub(crate) evidence: ProfileEvidence,
    pub(crate) resolved_policy: ResolvedProfilePolicy,
}

/// Complete owned in-memory export preparation. Destination resolution and
/// public response construction remain UI0D3 responsibilities.
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) struct PreparedExportSequence {
    pub(crate) session_id: SessionId,
    pub(crate) sequence_id: SequenceId,
    pub(crate) sequence_display_name: String,
    pub(crate) compatibility_profile: ProfileCapability,
    pub(crate) result: MultitrackExportResult,
}

const EXPORT_CANDIDATE_LIMIT: usize = 10_000;
const EXPORT_TEMP_ATTEMPT_LIMIT: usize = 128;
const EXPORT_TEMP_PREFIX: &str = ".phoenix-export-";
static NEXT_EXPORT_TEMP_ID: AtomicU64 = AtomicU64::new(0);
static NEXT_SERVICE_NAMESPACE: AtomicU64 = AtomicU64::new(1);

fn allocate_service_namespace() -> u64 {
    NEXT_SERVICE_NAMESPACE
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |value| {
            value.checked_add(1)
        })
        .expect("AppService namespace space exhausted")
}

struct ExportResponsePreflight {
    session_id: SessionId,
    sequence_id: SequenceId,
    sequence_display_name: String,
    compatibility_profile: ProfileCapability,
    musical_track_count: u32,
    total_smf_track_count: u32,
    counts: AppExportCounts,
    warnings: Vec<Warning>,
    cleanup_warning_source_order: u32,
    untranslated_metadata_count: u64,
}

#[derive(Clone, Copy)]
struct ExportLimits {
    candidate_count: usize,
    temp_attempt_count: usize,
}

trait ExportFileSystem {
    type TempFile: Write;

    fn destination_is_dir(&self, path: &Path) -> io::Result<bool>;
    fn create_temp(&self, path: &Path) -> io::Result<Self::TempFile>;
    fn sync_temp(&self, file: &Self::TempFile) -> io::Result<()>;
    fn hard_link(&self, temp: &Path, candidate: &Path) -> io::Result<()>;
    fn remove_temp(&self, path: &Path) -> io::Result<()>;
}

struct RealExportFileSystem;

impl ExportFileSystem for RealExportFileSystem {
    type TempFile = File;

    fn destination_is_dir(&self, path: &Path) -> io::Result<bool> {
        fs::metadata(path).map(|metadata| metadata.is_dir())
    }

    fn create_temp(&self, path: &Path) -> io::Result<Self::TempFile> {
        OpenOptions::new().write(true).create_new(true).open(path)
    }

    fn sync_temp(&self, file: &Self::TempFile) -> io::Result<()> {
        file.sync_all()
    }

    fn hard_link(&self, temp: &Path, candidate: &Path) -> io::Result<()> {
        fs::hard_link(temp, candidate)
    }

    fn remove_temp(&self, path: &Path) -> io::Result<()> {
        fs::remove_file(path)
    }
}

#[derive(Clone)]
struct Session {
    source_path: String,
    source_bytes: Vec<u8>,
    source_sha256: String,
    response: InspectProjectResponse,
    diagnostics: Diagnostics,
    structure: Option<InspectedProjectStructure>,
    #[allow(dead_code)]
    sequence_ordinals: HashMap<SequenceId, u32>,
    assessments: HashMap<SequenceId, SequenceAssessment>,
}

#[derive(Clone)]
struct InspectedProjectStructure {
    parser_profile: ParserProfileId,
    sequences: Vec<InspectedSequenceStructure>,
}

#[derive(Clone)]
struct InspectedSequenceStructure {
    structural_ordinal: u32,
    sequence_range: ByteRange,
    name_bytes: Vec<u8>,
    name_range: ByteRange,
    descriptor_count: u32,
    pair_count: u32,
    tracks: Vec<InspectedTrackStructure>,
}

#[derive(Clone)]
struct InspectedTrackStructure {
    descriptor_ordinal: u32,
    descriptor_range: ByteRange,
    pair_ordinal: u32,
    primary_range: ByteRange,
    exact_event_range: Option<ByteRange>,
    label_bytes: Vec<u8>,
    decoded_event_families: Vec<EvidenceEventFamily>,
    decoded_event_count: u64,
    patch_evidence: Vec<PatchEvidence>,
}

/// Synchronous, owned service state for one or more inspection sessions.
pub struct AppService {
    sessions: HashMap<SessionId, Session>,
    session_namespace: u64,
    next_session: u64,
    registry: Option<CompatibilityRegistry>,
    registry_error: Option<String>,
}

impl Default for AppService {
    fn default() -> Self {
        Self::new()
    }
}

impl AppService {
    pub fn new() -> Self {
        match crate::compatibility_profiles::built_in_compatibility_registry() {
            Ok(registry) => Self::with_registry(registry),
            Err(error) => Self {
                sessions: HashMap::new(),
                session_namespace: allocate_service_namespace(),
                next_session: 1,
                registry: None,
                registry_error: Some(format!("{error:?}")),
            },
        }
    }

    pub fn with_registry(registry: CompatibilityRegistry) -> Self {
        Self {
            sessions: HashMap::new(),
            session_namespace: allocate_service_namespace(),
            next_session: 1,
            registry: Some(registry),
            registry_error: None,
        }
    }

    pub fn api_info(&self) -> ApiInfo {
        ApiInfo::new(env!("CARGO_PKG_VERSION"))
    }

    pub fn session_count(&self) -> usize {
        self.sessions.len()
    }

    /// Report that synchronous v0 Core operations do not support cancellation.
    #[allow(clippy::result_large_err)]
    pub fn cancel_operation(&self, _operation_id: &OperationId) -> Result<(), AppError> {
        Err(self.error(
            AppErrorCategory::InternalError,
            "Phoenix cannot cancel this operation.",
            "synchronous v0 Core operations do not have a cancellation registry".into(),
            AppOperation::CancelOperation,
            "cancellation_not_supported",
            None,
            None,
        ))
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
        let (project, sequences, warnings, diagnostics, structure) = match parse_project_166(&bytes)
        {
            Ok(parsed) => self.build_parsed_result(
                &inspection.filename,
                inspection.size,
                &finder,
                &bytes,
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
        let sequence_ordinals = response
            .sequences
            .iter()
            .enumerate()
            .map(|(index, summary)| (summary.sequence_id.clone(), index as u32))
            .collect();
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
                structure,
                sequence_ordinals,
                assessments: HashMap::new(),
            },
        );
        self.assess_session(&session_id);
        self.project_readiness(&session_id);
        self.sessions
            .get(&session_id)
            .map(|session| session.response.clone())
            .ok_or_else(|| self.unknown_session(AppOperation::InspectProject))
    }

    fn project_readiness(&mut self, session_id: &SessionId) {
        let Some(session) = self.sessions.get_mut(session_id) else {
            return;
        };
        for summary in &mut session.response.sequences {
            let Some(assessment) = session.assessments.get(&summary.sequence_id) else {
                continue;
            };
            project_sequence_readiness(summary, assessment);
        }
        session.response.warnings.retain(|warning| {
            !(warning.scope == WarningScope::Sequence
                && warning.code == "missing_channel_routing"
                && session
                    .response
                    .sequences
                    .get(warning.source_order as usize)
                    .is_some_and(|summary| summary.readiness == Readiness::Ready))
        });
        for (index, summary) in session.response.sequences.iter_mut().enumerate() {
            summary.warning_count = session
                .response
                .warnings
                .iter()
                .filter(|warning| {
                    warning.scope == WarningScope::Sequence && warning.source_order == index as u32
                })
                .count() as u32;
        }
        session.response.project.overall_readiness = overall_readiness(&session.response.sequences);
    }

    fn assess_session(&mut self, session_id: &SessionId) {
        let Ok(evidence) = self.profile_evidence(session_id) else {
            return;
        };
        let Some(registry) = self.registry.clone() else {
            let diagnostic = self
                .registry_error
                .clone()
                .unwrap_or_else(|| "compatibility registry unavailable".into());
            if let Some(session) = self.sessions.get_mut(session_id) {
                for summary in &session.response.sequences {
                    let ordinal = *session
                        .sequence_ordinals
                        .get(&summary.sequence_id)
                        .unwrap_or(&0);
                    let generic_readiness = summary.readiness;
                    session.assessments.insert(
                        summary.sequence_id.clone(),
                        SequenceAssessment {
                            structural_ordinal: ordinal,
                            generic_readiness,
                            match_state: SequenceMatchState::RegistryError,
                            capability: None,
                            resolved_policy: None,
                            diagnostic_code: Some("profile_registry_configuration".into()),
                            technical_detail: Some(diagnostic.clone()),
                        },
                    );
                }
            }
            return;
        };
        if let Some(session) = self.sessions.get_mut(session_id) {
            for summary in &session.response.sequences {
                let Some(&ordinal) = session.sequence_ordinals.get(&summary.sequence_id) else {
                    continue;
                };
                let (match_state, capability, resolved_policy, diagnostic_code, technical_detail) =
                    match registry.assess(&evidence, ordinal) {
                        Ok(ProfileMatch::NoMatch) => {
                            (SequenceMatchState::NoMatch, None, None, None, None)
                        }
                        Ok(ProfileMatch::Matched {
                            capability,
                            resolved_policy,
                        }) => (
                            SequenceMatchState::Matched,
                            Some(capability),
                            Some(resolved_policy),
                            None,
                            None,
                        ),
                        Ok(ProfileMatch::Rejected { reason, .. }) => (
                            SequenceMatchState::Rejected,
                            None,
                            None,
                            Some(reason.diagnostic_code().into()),
                            Some(
                                "authenticated profile candidate rejected by exact evidence".into(),
                            ),
                        ),
                        Err(error) => (
                            SequenceMatchState::RegistryError,
                            None,
                            None,
                            Some("profile_ambiguous_match".into()),
                            Some(format!("{error:?}")),
                        ),
                    };
                session.assessments.insert(
                    summary.sequence_id.clone(),
                    SequenceAssessment {
                        structural_ordinal: ordinal,
                        generic_readiness: summary.readiness,
                        match_state,
                        capability,
                        resolved_policy,
                        diagnostic_code,
                        technical_detail,
                    },
                );
            }
        }
    }

    #[allow(clippy::result_large_err)]
    pub fn assessment_for_sequence(
        &self,
        session_id: &SessionId,
        sequence_id: &SequenceId,
    ) -> Result<SequenceAssessmentStatus, AppError> {
        let session = self
            .sessions
            .get(session_id)
            .ok_or_else(|| self.unknown_session(AppOperation::GetDiagnostics))?;
        let assessment = session.assessments.get(sequence_id).ok_or_else(|| {
            self.unknown_sequence(session_id, sequence_id, AppOperation::GetDiagnostics)
        })?;
        Ok(SequenceAssessmentStatus {
            structural_ordinal: assessment.structural_ordinal,
            match_kind: match assessment.match_state {
                SequenceMatchState::NoMatch => SequenceAssessmentKind::NoMatch,
                SequenceMatchState::Matched => SequenceAssessmentKind::Matched,
                SequenceMatchState::Rejected => SequenceAssessmentKind::Rejected,
                SequenceMatchState::RegistryError => SequenceAssessmentKind::RegistryError,
            },
            capability: assessment.capability.clone(),
            has_resolved_policy: assessment.resolved_policy.is_some(),
            diagnostic_code: assessment.diagnostic_code.clone(),
        })
    }

    #[allow(clippy::result_large_err)]
    #[allow(dead_code)]
    /// Inspection-time policy only. Future authorization must use
    /// `revalidated_policy_for_sequence` instead.
    pub(crate) fn inspected_policy_for_sequence(
        &self,
        session_id: &SessionId,
        sequence_id: &SequenceId,
    ) -> Result<Option<ResolvedProfilePolicy>, AppError> {
        let session = self
            .sessions
            .get(session_id)
            .ok_or_else(|| self.unknown_session(AppOperation::GetDiagnostics))?;
        session
            .assessments
            .get(sequence_id)
            .map(|assessment| assessment.resolved_policy.clone())
            .ok_or_else(|| {
                self.unknown_sequence(session_id, sequence_id, AppOperation::GetDiagnostics)
            })
    }

    /// Re-read and revalidate one inspected sequence against the current
    /// source on disk. This operation never changes readiness or performs
    /// export; it is the only Core path that may produce fresh policy state.
    #[allow(clippy::result_large_err)]
    pub fn revalidate_sequence_policy(
        &self,
        session_id: &SessionId,
        sequence_id: &SequenceId,
    ) -> Result<SequenceRevalidationStatus, AppError> {
        let fresh = self.revalidated_policy_for_sequence(session_id, sequence_id)?;
        let capability = self
            .sessions
            .get(session_id)
            .and_then(|session| session.assessments.get(sequence_id))
            .and_then(|assessment| assessment.capability.clone());
        Ok(SequenceRevalidationStatus {
            kind: SequenceRevalidationKind::Validated,
            source_sha256: Some(fresh.source_sha256),
            capability,
            diagnostic_code: "source_revalidated".into(),
        })
    }

    /// Validate one export request through fresh source authorization and
    /// complete in-memory assembly. Destination fields remain untouched for
    /// UI0D3.
    #[allow(clippy::result_large_err)]
    #[allow(dead_code)]
    pub(crate) fn prepare_export_sequence(
        &self,
        request: &ExportSequenceRequest,
    ) -> Result<PreparedExportSequence, AppError> {
        if request.contract_version != CONTRACT_VERSION {
            return Err(self.error(
                AppErrorCategory::InternalError,
                "This request uses an unsupported Phoenix contract version.",
                format!(
                    "requested contract version {}; supported version {}",
                    request.contract_version, CONTRACT_VERSION
                ),
                AppOperation::ExportSequence,
                "contract_version_mismatch",
                Some(request.session_id.clone()),
                Some(request.sequence_id.clone()),
            ));
        }

        let (sequence_display_name, compatibility_profile) = {
            let session = self
                .sessions
                .get(&request.session_id)
                .ok_or_else(|| self.unknown_session(AppOperation::ExportSequence))?;
            let sequence = session
                .response
                .sequences
                .iter()
                .find(|sequence| sequence.sequence_id == request.sequence_id)
                .ok_or_else(|| {
                    self.unknown_sequence(
                        &request.session_id,
                        &request.sequence_id,
                        AppOperation::ExportSequence,
                    )
                })?;
            let capability = session
                .assessments
                .get(&request.sequence_id)
                .and_then(|assessment| {
                    (sequence.readiness == Readiness::Ready
                        && assessment.match_state == SequenceMatchState::Matched
                        && assessment.resolved_policy.is_some())
                    .then_some(assessment)
                })
                .and_then(|assessment| assessment.capability.as_ref())
                .filter(|capability| sequence.export_capability.as_ref() == Some(*capability))
                .cloned();
            let Some(capability) = capability else {
                return Err(self.error(
                    AppErrorCategory::ExportValidationFailed,
                    "This sequence is not eligible for export.",
                    "inspection-time readiness, capability, and matched policy are not complete"
                        .into(),
                    AppOperation::ExportSequence,
                    "sequence_not_export_capable",
                    Some(request.session_id.clone()),
                    Some(request.sequence_id.clone()),
                ));
            };
            (sequence.display_name.clone(), capability)
        };

        let fresh = self.revalidated_policy_for_sequence_with_operation(
            &request.session_id,
            &request.sequence_id,
            AppOperation::ExportSequence,
        )?;
        let ready = build_conversion_ready_sequence(&fresh).map_err(|error| {
            self.error(
                AppErrorCategory::ExportValidationFailed,
                "Phoenix could not prepare this sequence for export.",
                error.to_string(),
                AppOperation::ExportSequence,
                "conversion_failed",
                Some(request.session_id.clone()),
                Some(request.sequence_id.clone()),
            )
        })?;
        let result = ready
            .with_multitrack_input(|input| assemble_multitrack_sequence(&input))
            .map_err(|error| {
                self.error(
                    AppErrorCategory::ExportValidationFailed,
                    "Phoenix could not prepare this sequence for export.",
                    error.to_string(),
                    AppOperation::ExportSequence,
                    "conversion_failed",
                    Some(request.session_id.clone()),
                    Some(request.sequence_id.clone()),
                )
            })?;

        Ok(PreparedExportSequence {
            session_id: request.session_id.clone(),
            sequence_id: request.sequence_id.clone(),
            sequence_display_name,
            compatibility_profile,
            result,
        })
    }

    /// Prepare, commit, and report one single-sequence MIDI export.
    #[allow(clippy::result_large_err)]
    pub fn export_sequence(
        &self,
        request: ExportSequenceRequest,
    ) -> Result<ExportSequenceResponse, AppError> {
        self.export_sequence_with_file_system(
            &request,
            &RealExportFileSystem,
            ExportLimits {
                candidate_count: EXPORT_CANDIDATE_LIMIT,
                temp_attempt_count: EXPORT_TEMP_ATTEMPT_LIMIT,
            },
        )
    }

    #[allow(clippy::result_large_err)]
    fn export_sequence_with_file_system<F: ExportFileSystem>(
        &self,
        request: &ExportSequenceRequest,
        file_system: &F,
        limits: ExportLimits,
    ) -> Result<ExportSequenceResponse, AppError> {
        let prepared = self.prepare_export_sequence(request)?;
        self.commit_prepared_export(request, prepared, file_system, limits)
    }

    #[allow(clippy::result_large_err)]
    fn commit_prepared_export<F: ExportFileSystem>(
        &self,
        request: &ExportSequenceRequest,
        prepared: PreparedExportSequence,
        file_system: &F,
        limits: ExportLimits,
    ) -> Result<ExportSequenceResponse, AppError> {
        let mut response = self.preflight_export_response(&prepared)?;
        let normalized_stem =
            normalize_export_filename_stem(&request.filename_stem).ok_or_else(|| {
                self.export_error(
                    request,
                    AppErrorCategory::ExportValidationFailed,
                    "Phoenix could not use this MIDI filename.",
                    "filename_stem is empty or contains a forbidden filename component value"
                        .into(),
                    "invalid_filename_stem",
                )
            })?;
        let destination = PathBuf::from(&request.destination_folder);
        match file_system.destination_is_dir(&destination) {
            Ok(true) => {}
            Ok(false) => {
                return Err(self.export_error(
                    request,
                    AppErrorCategory::OutputIoFailed,
                    "Phoenix could not use the selected destination folder.",
                    "the supplied destination exists but is not a directory".into(),
                    "invalid_destination_folder",
                ))
            }
            Err(error) => {
                return Err(self.export_error(
                    request,
                    AppErrorCategory::OutputIoFailed,
                    "Phoenix could not use the selected destination folder.",
                    bounded_io_detail(&error),
                    "invalid_destination_folder",
                ))
            }
        }

        let (temp_path, mut temp_file) = self.allocate_export_temp(
            request,
            file_system,
            &destination,
            limits.temp_attempt_count,
        )?;
        if let Err(error) = temp_file.write_all(&prepared.result.smf_bytes) {
            drop(temp_file);
            return Err(self.export_error_with_temp_cleanup(
                request,
                file_system,
                &temp_path,
                AppErrorCategory::OutputIoFailed,
                "Phoenix could not write the MIDI export.",
                bounded_io_detail(&error),
                "output_write_failed",
            ));
        }
        if let Err(error) = temp_file.flush() {
            drop(temp_file);
            return Err(self.export_error_with_temp_cleanup(
                request,
                file_system,
                &temp_path,
                AppErrorCategory::OutputIoFailed,
                "Phoenix could not synchronize the MIDI export.",
                bounded_io_detail(&error),
                "output_sync_failed",
            ));
        }
        if let Err(error) = file_system.sync_temp(&temp_file) {
            drop(temp_file);
            return Err(self.export_error_with_temp_cleanup(
                request,
                file_system,
                &temp_path,
                AppErrorCategory::OutputIoFailed,
                "Phoenix could not synchronize the MIDI export.",
                bounded_io_detail(&error),
                "output_sync_failed",
            ));
        }
        drop(temp_file);

        let candidate_count = match request.collision_policy {
            CollisionPolicy::FailIfExists => 1,
            CollisionPolicy::GenerateUniqueName => limits.candidate_count,
        };
        let mut committed = None;
        for candidate_index in 0..candidate_count {
            let filename = export_candidate_filename(&normalized_stem, candidate_index);
            let candidate = destination.join(filename);
            let Some(output_path) = candidate.to_str().map(str::to_owned) else {
                return Err(self.export_error_with_temp_cleanup(
                    request,
                    file_system,
                    &temp_path,
                    AppErrorCategory::OutputIoFailed,
                    "Phoenix could not use the selected destination folder.",
                    "the output path is not representable as UTF-8".into(),
                    "invalid_destination_folder",
                ));
            };
            match file_system.hard_link(&temp_path, &candidate) {
                Ok(()) => {
                    committed = Some((candidate, output_path));
                    break;
                }
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                    if request.collision_policy == CollisionPolicy::FailIfExists {
                        return Err(self.export_error_with_temp_cleanup(
                            request,
                            file_system,
                            &temp_path,
                            AppErrorCategory::DestinationExists,
                            "A file already exists at the selected destination.",
                            "the final candidate was occupied at no-overwrite publication".into(),
                            "destination_exists",
                        ));
                    }
                }
                Err(error) => {
                    return Err(self.export_error_with_temp_cleanup(
                        request,
                        file_system,
                        &temp_path,
                        AppErrorCategory::OutputIoFailed,
                        "Phoenix could not commit the MIDI export.",
                        bounded_io_detail(&error),
                        "output_commit_failed",
                    ));
                }
            }
        }
        let Some((committed_path, output_path)) = committed else {
            return Err(self.export_error_with_temp_cleanup(
                request,
                file_system,
                &temp_path,
                AppErrorCategory::DestinationExists,
                "Phoenix could not find an unused MIDI filename.",
                format!("all {candidate_count} deterministic candidates were occupied"),
                "destination_name_exhausted",
            ));
        };

        if let Err(error) = file_system.remove_temp(&temp_path) {
            response.warnings.push(Warning {
                code: "temporary_cleanup_failed".into(),
                message: "The MIDI export succeeded, but Phoenix could not remove a private temporary filesystem entry.".into(),
                technical_detail: Some(format!(
                    "temporary filename {}; {}",
                    temp_path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("<unrepresentable>"),
                    bounded_io_detail(&error)
                )),
                scope: WarningScope::Sequence,
                severity: WarningSeverity::Caution,
                diagnostic_ref: None,
                source_order: response.cleanup_warning_source_order,
            });
        }

        debug_assert_eq!(committed_path.to_str(), Some(output_path.as_str()));
        Ok(ExportSequenceResponse {
            session_id: response.session_id,
            sequence_id: response.sequence_id,
            sequence_display_name: response.sequence_display_name,
            output_path,
            compatibility_profile: Some(response.compatibility_profile),
            musical_track_count: response.musical_track_count,
            total_smf_track_count: response.total_smf_track_count,
            counts: response.counts,
            warnings: response.warnings,
            untranslated_metadata_count: response.untranslated_metadata_count,
            validation_status: ValidationStatus::Validated,
        })
    }

    #[allow(clippy::result_large_err)]
    fn preflight_export_response(
        &self,
        prepared: &PreparedExportSequence,
    ) -> Result<ExportResponsePreflight, AppError> {
        let overflow = || {
            self.error(
                AppErrorCategory::InternalError,
                "Phoenix could not represent the completed export report.",
                "an in-memory export report count exceeds the public contract width".into(),
                AppOperation::ExportSequence,
                "export_response_overflow",
                Some(prepared.session_id.clone()),
                Some(prepared.sequence_id.clone()),
            )
        };
        let report = &prepared.result.report;
        let musical_track_count =
            u32::try_from(report.musical_track_count).map_err(|_| overflow())?;
        let total_smf_track_count =
            u32::try_from(report.total_smf_track_count).map_err(|_| overflow())?;
        let untranslated_metadata_count =
            u64::try_from(report.untranslated_metadata.len()).map_err(|_| overflow())?;
        let cleanup_warning_source_order =
            u32::try_from(report.warnings.len()).map_err(|_| overflow())?;
        let mut warnings = Vec::with_capacity(report.warnings.len() + 1);
        for (index, warning) in report.warnings.iter().enumerate() {
            let source_order = u32::try_from(index).map_err(|_| overflow())?;
            warnings.push(map_export_warning(warning, source_order));
        }
        let totals = &report.totals;
        Ok(ExportResponsePreflight {
            session_id: prepared.session_id.clone(),
            sequence_id: prepared.sequence_id.clone(),
            sequence_display_name: prepared.sequence_display_name.clone(),
            compatibility_profile: prepared.compatibility_profile.clone(),
            musical_track_count,
            total_smf_track_count,
            counts: AppExportCounts {
                notes: totals.notes,
                generated_note_offs: totals.generated_note_offs,
                controllers: totals.controllers,
                bank_select_msb: totals.bank_select_msb,
                bank_select_lsb: totals.bank_select_lsb,
                programs: totals.program_changes,
                pressure: totals.channel_pressure,
                pitch_bend: totals.pitch_bend,
                tempo: totals.tempo,
                meter: totals.meter,
            },
            warnings,
            cleanup_warning_source_order,
            untranslated_metadata_count,
        })
    }

    #[allow(clippy::result_large_err)]
    fn allocate_export_temp<F: ExportFileSystem>(
        &self,
        request: &ExportSequenceRequest,
        file_system: &F,
        destination: &Path,
        attempt_count: usize,
    ) -> Result<(PathBuf, F::TempFile), AppError> {
        for _ in 0..attempt_count {
            let path = destination.join(next_export_temp_filename());
            match file_system.create_temp(&path) {
                Ok(file) => return Ok((path, file)),
                Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
                Err(error) => {
                    return Err(self.export_error(
                        request,
                        AppErrorCategory::OutputIoFailed,
                        "Phoenix could not allocate a temporary export file.",
                        bounded_io_detail(&error),
                        "temporary_file_allocation_failed",
                    ))
                }
            }
        }
        Err(self.export_error(
            request,
            AppErrorCategory::OutputIoFailed,
            "Phoenix could not allocate a temporary export file.",
            format!("all {attempt_count} private temporary names were occupied"),
            "temporary_file_allocation_failed",
        ))
    }

    #[allow(clippy::too_many_arguments)]
    fn export_error_with_temp_cleanup<F: ExportFileSystem>(
        &self,
        request: &ExportSequenceRequest,
        file_system: &F,
        temp_path: &Path,
        category: AppErrorCategory,
        display_message: &str,
        mut technical_message: String,
        diagnostic_code: &str,
    ) -> AppError {
        if let Err(cleanup) = file_system.remove_temp(temp_path) {
            technical_message.push_str("; temporary cleanup also failed: ");
            technical_message.push_str(&bounded_io_detail(&cleanup));
        }
        self.export_error(
            request,
            category,
            display_message,
            technical_message,
            diagnostic_code,
        )
    }

    fn export_error(
        &self,
        request: &ExportSequenceRequest,
        category: AppErrorCategory,
        display_message: &str,
        technical_message: String,
        diagnostic_code: &str,
    ) -> AppError {
        self.error(
            category,
            display_message,
            technical_message,
            AppOperation::ExportSequence,
            diagnostic_code,
            Some(request.session_id.clone()),
            Some(request.sequence_id.clone()),
        )
    }

    /// Internal handoff for a future exporter. The returned bytes and policy
    /// belong to the same successful revalidation call, avoiding a stale
    /// policy or a second source read at the export boundary.
    #[allow(clippy::result_large_err)]
    pub(crate) fn revalidated_policy_for_sequence(
        &self,
        session_id: &SessionId,
        sequence_id: &SequenceId,
    ) -> Result<FreshValidatedSequence, AppError> {
        self.revalidated_policy_for_sequence_with_operation(
            session_id,
            sequence_id,
            AppOperation::GetDiagnostics,
        )
    }

    #[allow(clippy::result_large_err)]
    fn revalidated_policy_for_sequence_with_operation(
        &self,
        session_id: &SessionId,
        sequence_id: &SequenceId,
        operation: AppOperation,
    ) -> Result<FreshValidatedSequence, AppError> {
        let (
            source_path,
            inspected_sha256,
            inspected_size,
            ordinal,
            stored_capability,
            stored_policy,
        ) = {
            let session = self
                .sessions
                .get(session_id)
                .ok_or_else(|| self.unknown_session(operation))?;
            let assessment = session
                .assessments
                .get(sequence_id)
                .ok_or_else(|| self.unknown_sequence(session_id, sequence_id, operation))?;
            let Some(policy) = assessment.resolved_policy.clone() else {
                return Err(self.error(
                    AppErrorCategory::ExportValidationFailed,
                    "This sequence has no freshly validated compatibility policy.",
                    "the inspection assessment did not retain a matched profile policy".into(),
                    operation,
                    "no_validated_profile_policy",
                    Some(session_id.clone()),
                    Some(sequence_id.clone()),
                ));
            };
            (
                session.source_path.clone(),
                session.source_sha256.clone(),
                session.response.project.byte_size,
                assessment.structural_ordinal,
                assessment.capability.clone(),
                policy,
            )
        };

        let fresh_bytes = fs::read(&source_path).map_err(|error| {
            self.error(
                AppErrorCategory::FileUnreadable,
                "Phoenix could not re-read the inspected source.",
                error.to_string(),
                operation,
                "source_revalidation_failed",
                Some(session_id.clone()),
                Some(sequence_id.clone()),
            )
        })?;
        let fresh_size = u64::try_from(fresh_bytes.len()).map_err(|_| {
            self.error(
                AppErrorCategory::ExportValidationFailed,
                "Phoenix could not validate the current source identity.",
                "fresh source byte length exceeds compatibility evidence range".into(),
                operation,
                "source_identity_changed",
                Some(session_id.clone()),
                Some(sequence_id.clone()),
            )
        })?;
        let fresh_sha256 = sha256_hex(&fresh_bytes);
        if fresh_size != inspected_size || fresh_sha256 != inspected_sha256 {
            return Err(self.error(
                AppErrorCategory::ExportValidationFailed,
                "The inspected source changed and cannot use its old policy.",
                "fresh source size or SHA-256 differs from the inspection identity".into(),
                operation,
                "source_identity_changed",
                Some(session_id.clone()),
                Some(sequence_id.clone()),
            ));
        }

        let parsed = parse_project_166(&fresh_bytes).map_err(|error| {
            self.error(
                AppErrorCategory::ExportValidationFailed,
                "The current source no longer has the inspected structure.",
                error.to_string(),
                operation,
                "source_revalidation_failed",
                Some(session_id.clone()),
                Some(sequence_id.clone()),
            )
        })?;
        let structure =
            build_structure_snapshot(&parsed.sequences, &fresh_bytes).ok_or_else(|| {
                self.error(
                    AppErrorCategory::ExportValidationFailed,
                    "The current source no longer has the inspected structure.",
                    "fresh structural evidence conversion was incomplete".into(),
                    operation,
                    "source_sequence_identity_changed",
                    Some(session_id.clone()),
                    Some(sequence_id.clone()),
                )
            })?;
        if !structure
            .sequences
            .iter()
            .any(|sequence| sequence.structural_ordinal == ordinal)
        {
            return Err(self.error(
                AppErrorCategory::ExportValidationFailed,
                "The selected sequence no longer exists in the source.",
                "fresh evidence does not contain the inspected structural ordinal".into(),
                operation,
                "source_sequence_identity_changed",
                Some(session_id.clone()),
                Some(sequence_id.clone()),
            ));
        }
        let evidence = profile_evidence_from_structure(&fresh_sha256, fresh_size, &structure);
        let Some(registry) = self.registry.clone() else {
            return Err(self.error(
                AppErrorCategory::ExportValidationFailed,
                "Phoenix could not validate the compatibility policy.",
                self.registry_error
                    .clone()
                    .unwrap_or_else(|| "compatibility registry unavailable".into()),
                operation,
                "profile_registry_configuration",
                Some(session_id.clone()),
                Some(sequence_id.clone()),
            ));
        };
        let fresh_match = registry.assess(&evidence, ordinal).map_err(|error| {
            self.error(
                AppErrorCategory::ExportValidationFailed,
                "Phoenix could not validate the compatibility policy.",
                format!("{error:?}"),
                operation,
                "profile_ambiguous_match",
                Some(session_id.clone()),
                Some(sequence_id.clone()),
            )
        })?;
        let (fresh_capability, fresh_policy) = match fresh_match {
            ProfileMatch::Matched {
                capability,
                resolved_policy,
            } => (capability, resolved_policy),
            ProfileMatch::NoMatch => {
                return Err(self.error(
                    AppErrorCategory::ExportValidationFailed,
                    "The current source no longer matches its inspected profile.",
                    "fresh compatibility assessment returned no match".into(),
                    operation,
                    "profile_no_longer_matches",
                    Some(session_id.clone()),
                    Some(sequence_id.clone()),
                ))
            }
            ProfileMatch::Rejected { reason, .. } => {
                return Err(self.error(
                    AppErrorCategory::ExportValidationFailed,
                    "The current source no longer matches its inspected profile.",
                    "fresh compatibility assessment rejected the profile candidate".into(),
                    operation,
                    reason.diagnostic_code(),
                    Some(session_id.clone()),
                    Some(sequence_id.clone()),
                ))
            }
        };
        if stored_capability.as_ref() != Some(&fresh_capability)
            || fresh_policy.profile_id != stored_policy.profile_id
            || fresh_policy.profile_version != stored_policy.profile_version
            || fresh_policy != stored_policy
        {
            return Err(self.error(
                AppErrorCategory::ExportValidationFailed,
                "The current compatibility policy differs from the inspected policy.",
                "fresh profile identity or resolved policy is not equivalent".into(),
                operation,
                "profile_policy_changed",
                Some(session_id.clone()),
                Some(sequence_id.clone()),
            ));
        }
        Ok(FreshValidatedSequence {
            source_bytes: fresh_bytes,
            source_sha256: fresh_sha256,
            structural_ordinal: ordinal,
            evidence,
            resolved_policy: fresh_policy,
        })
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

    /// Builds Core-only compatibility evidence from the retained structural
    /// snapshot. This never performs profile matching or changes readiness.
    #[allow(clippy::result_large_err)]
    pub fn profile_evidence(&self, session_id: &SessionId) -> Result<ProfileEvidence, AppError> {
        let session = self
            .sessions
            .get(session_id)
            .ok_or_else(|| self.unknown_session(AppOperation::GetDiagnostics))?;
        let Some(structure) = &session.structure else {
            return Err(self.error(
                AppErrorCategory::InternalError,
                "Structural compatibility evidence is unavailable for this session.",
                "the inspected input did not produce an established project structure".into(),
                AppOperation::GetDiagnostics,
                "profile_evidence_unavailable",
                Some(session_id.clone()),
                None,
            ));
        };
        let source_byte_size = u64::try_from(session.source_bytes.len()).map_err(|_| {
            self.error(
                AppErrorCategory::InternalError,
                "Phoenix could not represent the inspected source size.",
                "source byte length exceeds compatibility evidence range".into(),
                AppOperation::GetDiagnostics,
                "profile_evidence_size_overflow",
                Some(session_id.clone()),
                None,
            )
        })?;
        Ok(profile_evidence_from_structure(
            &session.source_sha256,
            source_byte_size,
            structure,
        ))
    }

    /// Core-only mapping used later when a registry assesses one sequence.
    #[allow(clippy::result_large_err)]
    #[allow(dead_code)]
    pub fn sequence_ordinal_for_id(
        &self,
        session_id: &SessionId,
        sequence_id: &SequenceId,
    ) -> Result<u32, AppError> {
        let session = self
            .sessions
            .get(session_id)
            .ok_or_else(|| self.unknown_session(AppOperation::GetDiagnostics))?;
        session
            .sequence_ordinals
            .get(sequence_id)
            .copied()
            .ok_or_else(|| {
                self.unknown_sequence(session_id, sequence_id, AppOperation::GetDiagnostics)
            })
    }

    fn allocate_session_id(&mut self) -> SessionId {
        let value = format!(
            "session-{:016x}-{:08}",
            self.session_namespace, self.next_session
        );
        self.next_session = self.next_session.saturating_add(1);
        SessionId::new(value)
    }

    #[allow(clippy::too_many_arguments)]
    fn build_parsed_result<'a>(
        &self,
        filename: &str,
        size: u64,
        finder: &crate::identification::Identification,
        bytes: &'a [u8],
        sequences: Vec<crate::sequence_container::SequenceContainer<'a>>,
        session_id: &SessionId,
        _level: DiagnosticsLevel,
    ) -> (
        ProjectSummary,
        Vec<SequenceSummary>,
        Vec<Warning>,
        Diagnostics,
        Option<InspectedProjectStructure>,
    ) {
        let mut summaries = Vec::with_capacity(sequences.len());
        let mut warnings = Vec::new();
        let mut technical_errors = Vec::new();
        let structure = build_structure_snapshot(&sequences, bytes);
        if structure.is_none() {
            technical_errors.push("structural evidence conversion was incomplete".into());
        }
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
        (project, summaries, warnings, diagnostics, structure)
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
        Option<InspectedProjectStructure>,
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
        Option<InspectedProjectStructure>,
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
        Option<InspectedProjectStructure>,
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
        (project, Vec::new(), vec![warning], diagnostics, None)
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

    #[allow(dead_code)]
    fn unknown_sequence(
        &self,
        session_id: &SessionId,
        sequence_id: &SequenceId,
        operation: AppOperation,
    ) -> AppError {
        self.error(
            AppErrorCategory::InternalError,
            "The selected sequence is not available in this inspection session.",
            "no sequence exists for the supplied session and sequence identifiers".into(),
            operation,
            "unknown_sequence",
            Some(session_id.clone()),
            Some(sequence_id.clone()),
        )
    }
}

fn build_structure_snapshot<'a>(
    sequences: &[crate::sequence_container::SequenceContainer<'a>],
    bytes: &[u8],
) -> Option<InspectedProjectStructure> {
    let mut snapshots = Vec::with_capacity(sequences.len());
    for (index, sequence) in sequences.iter().enumerate() {
        let sequence_range = owned_range(&sequence.sequence_range)?;
        let name_range = owned_range(&sequence.sequence_name.bytes.range)?;
        let mut tracks = Vec::new();
        if let TrackAssociations::Ordinal(bindings) = &sequence.track_associations {
            for binding in bindings {
                let descriptor = sequence
                    .descriptors
                    .iter()
                    .find(|descriptor| descriptor.ordinal == binding.descriptor_ordinal)?;
                let pair = sequence
                    .track_pairs
                    .iter()
                    .find(|pair| pair.pair_ordinal == binding.pair_ordinal)?;
                let (
                    exact_event_range,
                    decoded_event_families,
                    decoded_event_count,
                    patch_evidence,
                ) = inventory_track(bytes, pair);
                tracks.push(InspectedTrackStructure {
                    descriptor_ordinal: u32::try_from(binding.descriptor_ordinal).ok()?,
                    descriptor_range: owned_range(&descriptor.range)?,
                    pair_ordinal: u32::try_from(binding.pair_ordinal).ok()?,
                    primary_range: owned_range(&pair.primary.record_range)?,
                    exact_event_range,
                    label_bytes: descriptor
                        .label
                        .as_ref()
                        .map(|label| label.bytes.to_vec())
                        .unwrap_or_default(),
                    decoded_event_families,
                    decoded_event_count,
                    patch_evidence,
                });
            }
        }
        snapshots.push(InspectedSequenceStructure {
            structural_ordinal: u32::try_from(index).ok()?,
            sequence_range,
            name_bytes: sequence.sequence_name.bytes.bytes.to_vec(),
            name_range,
            descriptor_count: u32::from(sequence.descriptor_count.value),
            pair_count: u32::try_from(sequence.track_pairs.len()).ok()?,
            tracks,
        });
    }
    Some(InspectedProjectStructure {
        parser_profile: ParserProfileId::new("descriptor166"),
        sequences: snapshots,
    })
}

fn inventory_track(
    bytes: &[u8],
    pair: &crate::sequence_container::TrackRecordPair<'_>,
) -> (
    Option<ByteRange>,
    Vec<EvidenceEventFamily>,
    u64,
    Vec<PatchEvidence>,
) {
    let Ok(bounds) = pair.validated_event_bounds() else {
        return (None, Vec::new(), 0, Vec::new());
    };
    let exact_event_range = owned_range(&bounds.event_range);
    let Ok(walk) = walk_bounded_mixed_events(
        bytes,
        MixedEventBounds {
            event_range: bounds.event_range.clone(),
        },
        MixedEventTimingBasis::default(),
    ) else {
        return (exact_event_range, Vec::new(), 0, Vec::new());
    };
    if walk.consumed_range != bounds.event_range {
        return (exact_event_range, Vec::new(), 0, Vec::new());
    }

    let (families, count, patch_evidence) = inventory_families(&walk);
    (exact_event_range, families, count, patch_evidence)
}

fn inventory_families(
    walk: &crate::mixed_event::MixedEventWalk<'_>,
) -> (Vec<EvidenceEventFamily>, u64, Vec<PatchEvidence>) {
    let mut present = [false; 5];
    let mut patch_evidence = Vec::new();
    for (item_index, item) in walk.items.iter().enumerate() {
        match item {
            MixedEventItem::Patch(patch) => {
                present[0] = true;
                if let (Ok(source_ordinal), Some(source_range)) = (
                    u32::try_from(item_index),
                    owned_range(&patch.patch.representation_range),
                ) {
                    patch_evidence.push(PatchEvidence {
                        source_ordinal,
                        source_range,
                        decoded_program: patch.patch.program_change.value,
                        decoded_bank_msb: None,
                        decoded_bank_lsb: None,
                    });
                }
            }
            MixedEventItem::PatchToNote(transition) => {
                present[0] = true;
                present[1] = true;
                if let (Ok(source_ordinal), Some(source_range)) = (
                    u32::try_from(item_index),
                    owned_range(&transition.patch.representation_range),
                ) {
                    patch_evidence.push(PatchEvidence {
                        source_ordinal,
                        source_range,
                        decoded_program: transition.patch.program_change.value,
                        decoded_bank_msb: None,
                        decoded_bank_lsb: None,
                    });
                }
            }
            MixedEventItem::Event(positioned) => match &positioned.event {
                MixedEventKind::Note(_)
                | MixedEventKind::ContextMediatedNote(_)
                | MixedEventKind::DoubleContextMediatedNote(_) => {
                    present[1] = true;
                }
                MixedEventKind::Controller(_) => present[2] = true,
                MixedEventKind::ChannelPressure { .. } => present[3] = true,
                MixedEventKind::PitchBend { .. } => present[4] = true,
            },
        }
    }
    let canonical = [
        EvidenceEventFamily::Patch,
        EvidenceEventFamily::Note,
        EvidenceEventFamily::Controller,
        EvidenceEventFamily::ChannelPressure,
        EvidenceEventFamily::PitchBend,
    ];
    let families = canonical
        .into_iter()
        .enumerate()
        .filter_map(|(index, family)| {
            present
                .get(index)
                .copied()
                .unwrap_or(false)
                .then_some(family)
        })
        .collect();
    let count = u64::try_from(walk.logical_event_count()).unwrap_or(0);
    (families, count, patch_evidence)
}

fn owned_range(range: &std::ops::Range<usize>) -> Option<ByteRange> {
    ByteRange::new(
        u64::try_from(range.start).ok()?,
        u64::try_from(range.end).ok()?,
    )
    .ok()
}

fn project_sequence_readiness(summary: &mut SequenceSummary, assessment: &SequenceAssessment) {
    if assessment.match_state == SequenceMatchState::Matched && assessment.resolved_policy.is_some()
    {
        if let Some(capability) = assessment.capability.clone() {
            summary.readiness = Readiness::Ready;
            summary.readiness_reason = ReadinessReason::new(
                ReadinessReasonCode::ValidatedCompatibilityProfile,
                "This sequence matches a validated compatibility profile.",
            );
            summary.export_capability = Some(capability);
        }
    }
}

fn profile_evidence_from_structure(
    source_sha256: &str,
    source_byte_size: u64,
    structure: &InspectedProjectStructure,
) -> ProfileEvidence {
    ProfileEvidence {
        source_sha256: source_sha256.to_owned(),
        source_byte_size,
        parser_profile: structure.parser_profile.clone(),
        sequences: structure
            .sequences
            .iter()
            .map(|sequence| SequenceEvidence {
                structural_ordinal: sequence.structural_ordinal,
                sequence_range: sequence.sequence_range,
                name_bytes: sequence.name_bytes.clone(),
                name_range: sequence.name_range,
                descriptor_count: sequence.descriptor_count,
                pair_count: sequence.pair_count,
                tracks: sequence
                    .tracks
                    .iter()
                    .map(|track| TrackEvidence {
                        descriptor_ordinal: track.descriptor_ordinal,
                        descriptor_range: track.descriptor_range,
                        pair_ordinal: track.pair_ordinal,
                        primary_range: track.primary_range,
                        exact_event_range: track.exact_event_range,
                        label_bytes: track.label_bytes.clone(),
                        decoded_event_families: track.decoded_event_families.clone(),
                        decoded_event_count: track.decoded_event_count,
                        patch_evidence: track.patch_evidence.clone(),
                        observed_channel: None,
                        evidence_complete: false,
                    })
                    .collect(),
            })
            .collect(),
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
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

fn normalize_export_filename_stem(filename_stem: &str) -> Option<String> {
    let stem = if filename_stem
        .get(filename_stem.len().saturating_sub(4)..)
        .is_some_and(|suffix| suffix.eq_ignore_ascii_case(".mid"))
    {
        &filename_stem[..filename_stem.len() - 4]
    } else {
        filename_stem
    };
    (!stem.is_empty()
        && stem != "."
        && stem != ".."
        && !stem.contains('/')
        && !stem.contains('\\')
        && !stem.contains('\0'))
    .then(|| stem.to_owned())
}

fn export_candidate_filename(normalized_stem: &str, candidate_index: usize) -> String {
    if candidate_index == 0 {
        format!("{normalized_stem}.mid")
    } else {
        format!("{normalized_stem} {}.mid", candidate_index + 1)
    }
}

fn next_export_temp_filename() -> String {
    let counter = NEXT_EXPORT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos());
    format!(
        "{EXPORT_TEMP_PREFIX}{}-{timestamp}-{counter}.tmp",
        std::process::id()
    )
}

fn bounded_io_detail(error: &io::Error) -> String {
    const MAX_CHARS: usize = 512;
    let detail = error.to_string();
    if detail.chars().count() <= MAX_CHARS {
        detail
    } else {
        detail.chars().take(MAX_CHARS).collect()
    }
}

fn map_export_warning(warning: &ExportWarning, source_order: u32) -> Warning {
    let (code, message, technical_detail) = match warning {
        ExportWarning::MeterClocksFallback {
            source_third_payload,
            used,
        } => (
            "meter_clocks_fallback",
            "Phoenix used the standard MIDI clocks-per-click value for this sequence.",
            format!("source clocks-per-click {source_third_payload}; used {used}"),
        ),
        ExportWarning::MeterThirtySecondsFallback {
            source_fourth_payload,
            used,
        } => (
            "meter_thirty_seconds_fallback",
            "Phoenix used the standard MIDI notated-32nd-notes value for this sequence.",
            format!("source notated 32nd notes {source_fourth_payload}; used {used}"),
        ),
    };
    Warning {
        code: code.into(),
        message: message.into(),
        technical_detail: Some(technical_detail),
        scope: WarningScope::Sequence,
        severity: WarningSeverity::Caution,
        diagnostic_ref: None,
        source_order,
    }
}

fn overall_readiness(sequences: &[SequenceSummary]) -> Readiness {
    if sequences.is_empty() {
        return Readiness::Unknown;
    }
    if sequences.iter().all(|s| s.readiness == Readiness::Ready) {
        Readiness::Ready
    } else if sequences.iter().any(|s| {
        matches!(
            s.readiness,
            Readiness::Ready | Readiness::PartiallySupported
        )
    }) {
        Readiness::PartiallySupported
    } else if sequences
        .iter()
        .all(|s| s.readiness == Readiness::Unsupported)
    {
        Readiness::Unsupported
    } else {
        Readiness::Unknown
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::app_contract::{CollisionPolicy, ExportSequenceRequest, OperationId};
    use crate::compatibility::{
        CompatibilityProfile, IncludedTrackOutputExpectation, PatchExpectation,
        PatchTranslationPolicy, ProfileId, ProfileVersion, ProjectExpectation,
        ResolvedTrackOutputDisposition, SequenceExpectation, TrackChannelPolicy, TrackExpectation,
        TrackKey, TrackOutputDispositionExpectation,
    };
    use crate::midi_export::{ExportCounts as MidiExportCounts, UntranslatedMetadata};
    use crate::multitrack_export::{MultitrackExportReport, MultitrackExportResult};
    use std::cell::{Cell, RefCell};
    use std::collections::VecDeque;
    use std::rc::Rc;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    pub(crate) fn portable_path(bytes: &[u8]) -> PathBuf {
        static NEXT_PATH_ID: AtomicU64 = AtomicU64::new(0);
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path_id = NEXT_PATH_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "phoenix-ui0d2-{}-{nonce}-{path_id}",
            std::process::id()
        ));
        fs::write(&path, bytes).unwrap();
        path
    }

    pub(crate) fn portable_directory() -> PathBuf {
        static NEXT_DIRECTORY_ID: AtomicU64 = AtomicU64::new(0);
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let id = NEXT_DIRECTORY_ID.fetch_add(1, Ordering::Relaxed);
        let path =
            std::env::temp_dir().join(format!("phoenix-ui0d3-{}-{nonce}-{id}", std::process::id()));
        fs::create_dir(&path).unwrap();
        path
    }

    fn temp_entries(directory: &Path) -> Vec<PathBuf> {
        fs::read_dir(directory)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| {
                path.file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.starts_with(EXPORT_TEMP_PREFIX))
            })
            .collect()
    }

    #[derive(Default)]
    struct FaultExportFileSystem {
        create_errors: RefCell<VecDeque<io::ErrorKind>>,
        hard_link_errors: RefCell<VecDeque<io::ErrorKind>>,
        remove_errors: RefCell<VecDeque<io::ErrorKind>>,
        sync_errors: RefCell<VecDeque<io::ErrorKind>>,
        fail_write: Cell<bool>,
        fail_flush: Cell<bool>,
        destination_calls: Cell<usize>,
        create_calls: Cell<usize>,
        write_calls: Rc<Cell<usize>>,
        sync_calls: Cell<usize>,
        hard_link_calls: Cell<usize>,
        remove_calls: Cell<usize>,
    }

    #[derive(Debug)]
    struct FaultTempFile {
        file: File,
        fail_write: bool,
        fail_flush: bool,
        write_calls: Rc<Cell<usize>>,
    }

    impl Write for FaultTempFile {
        fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
            self.write_calls.set(self.write_calls.get() + 1);
            if self.fail_write {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "injected write failure",
                ))
            } else {
                self.file.write(buffer)
            }
        }

        fn flush(&mut self) -> io::Result<()> {
            if self.fail_flush {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "injected flush failure",
                ))
            } else {
                self.file.flush()
            }
        }
    }

    impl ExportFileSystem for FaultExportFileSystem {
        type TempFile = FaultTempFile;

        fn destination_is_dir(&self, path: &Path) -> io::Result<bool> {
            self.destination_calls.set(self.destination_calls.get() + 1);
            fs::metadata(path).map(|metadata| metadata.is_dir())
        }

        fn create_temp(&self, path: &Path) -> io::Result<Self::TempFile> {
            self.create_calls.set(self.create_calls.get() + 1);
            if let Some(kind) = self.create_errors.borrow_mut().pop_front() {
                return Err(io::Error::from(kind));
            }
            let file = OpenOptions::new().write(true).create_new(true).open(path)?;
            Ok(FaultTempFile {
                file,
                fail_write: self.fail_write.get(),
                fail_flush: self.fail_flush.get(),
                write_calls: Rc::clone(&self.write_calls),
            })
        }

        fn sync_temp(&self, file: &Self::TempFile) -> io::Result<()> {
            self.sync_calls.set(self.sync_calls.get() + 1);
            if let Some(kind) = self.sync_errors.borrow_mut().pop_front() {
                Err(io::Error::from(kind))
            } else {
                file.file.sync_all()
            }
        }

        fn hard_link(&self, temp: &Path, candidate: &Path) -> io::Result<()> {
            self.hard_link_calls.set(self.hard_link_calls.get() + 1);
            if let Some(kind) = self.hard_link_errors.borrow_mut().pop_front() {
                Err(io::Error::from(kind))
            } else {
                fs::hard_link(temp, candidate)
            }
        }

        fn remove_temp(&self, path: &Path) -> io::Result<()> {
            self.remove_calls.set(self.remove_calls.get() + 1);
            if let Some(kind) = self.remove_errors.borrow_mut().pop_front() {
                Err(io::Error::from(kind))
            } else {
                fs::remove_file(path)
            }
        }
    }

    pub(crate) fn portable_registry(bytes: &[u8]) -> CompatibilityRegistry {
        let parsed = parse_project_166(bytes).unwrap();
        let structure = build_structure_snapshot(&parsed.sequences, bytes).unwrap();
        let hash = sha256_hex(bytes);
        let evidence = profile_evidence_from_structure(&hash, bytes.len() as u64, &structure);
        let sequences = evidence
            .sequences
            .iter()
            .map(|sequence| SequenceExpectation {
                structural_ordinal: sequence.structural_ordinal,
                sequence_range: sequence.sequence_range,
                expected_name_bytes: sequence.name_bytes.clone(),
                name_range: sequence.name_range,
                descriptor_count: sequence.descriptor_count,
                pair_count: sequence.pair_count,
                track_expectations: sequence
                    .tracks
                    .iter()
                    .enumerate()
                    .map(|(index, track)| {
                        let key = TrackKey::new(track.descriptor_ordinal, track.pair_ordinal);
                        TrackExpectation {
                            key: key.clone(),
                            descriptor_range: track.descriptor_range,
                            primary_range: track.primary_range,
                            exact_event_range: track.exact_event_range,
                            expected_label_bytes: Some(track.label_bytes.clone()),
                            output: TrackOutputDispositionExpectation::Included(
                                IncludedTrackOutputExpectation {
                                    channel_policy: TrackChannelPolicy::new(key, [3, 11, 7][index])
                                        .unwrap(),
                                    patch_expectations: track
                                        .patch_evidence
                                        .iter()
                                        .map(|patch| PatchExpectation {
                                            source_ordinal: patch.source_ordinal,
                                            source_range: patch.source_range,
                                            decoded_program: patch.decoded_program,
                                            decoded_bank_msb: patch.decoded_bank_msb,
                                            decoded_bank_lsb: patch.decoded_bank_lsb,
                                            translation:
                                                PatchTranslationPolicy::BankSelectAndProgram {
                                                    msb: 81,
                                                    lsb: 2,
                                                    program: patch.decoded_program,
                                                },
                                        })
                                        .collect(),
                                },
                            ),
                        }
                    })
                    .collect(),
            })
            .collect();
        CompatibilityRegistry::new(vec![CompatibilityProfile {
            id: ProfileId::new("portable-ui0d2"),
            version: ProfileVersion::new(1),
            display_label: "Portable UI0D2".into(),
            project: ProjectExpectation::new(
                hash,
                bytes.len() as u64,
                ParserProfileId::new("descriptor166"),
                evidence.sequences.len() as u32,
            )
            .unwrap(),
            sequences,
        }])
        .unwrap()
    }

    fn export_request(session_id: SessionId, sequence_id: SequenceId) -> ExportSequenceRequest {
        ExportSequenceRequest {
            contract_version: CONTRACT_VERSION,
            session_id,
            sequence_id,
            destination_folder: "/definitely/not/accessed/ui0d2".into(),
            filename_stem: "not-used".into(),
            collision_policy: CollisionPolicy::FailIfExists,
            operation_id: None,
        }
    }

    fn portable_service() -> (AppService, PathBuf, InspectProjectResponse) {
        let bytes = crate::export_handoff::tests::portable_project();
        portable_service_for(bytes)
    }

    fn portable_service_for(bytes: Vec<u8>) -> (AppService, PathBuf, InspectProjectResponse) {
        let path = portable_path(&bytes);
        let mut service = AppService::with_registry(portable_registry(&bytes));
        let response = service
            .inspect_project(InspectProjectRequest {
                contract_version: CONTRACT_VERSION,
                source_path: path.to_string_lossy().into_owned(),
                diagnostics_level: DiagnosticsLevel::Full,
            })
            .unwrap();
        (service, path, response)
    }

    #[test]
    fn ui0d2_prepares_owned_export_and_ignores_destination_fields() {
        let (service, path, response) = portable_service();
        let sequence = &response.sequences[0];
        assert_eq!(sequence.readiness, Readiness::Ready);
        let first_request =
            export_request(response.session_id.clone(), sequence.sequence_id.clone());
        let first = service.prepare_export_sequence(&first_request).unwrap();

        let mut second_request = first_request;
        second_request.destination_folder = path
            .with_extension("missing-dir")
            .to_string_lossy()
            .into_owned();
        let unused_destination = PathBuf::from(&second_request.destination_folder);
        second_request.filename_stem = "different-and-unused".into();
        second_request.collision_policy = CollisionPolicy::GenerateUniqueName;
        second_request.operation_id = Some(OperationId::new("different-operation"));
        let second = service.prepare_export_sequence(&second_request).unwrap();

        assert_eq!(first, second);
        assert_eq!(first.session_id, response.session_id);
        assert_eq!(first.sequence_id, sequence.sequence_id);
        assert_eq!(first.sequence_display_name, "Portable Sequence");
        assert_eq!(first.compatibility_profile.profile_id, "portable-ui0d2");
        assert!(!first.result.smf_bytes.is_empty());
        assert_eq!(first.result.report.musical_track_count, 3);
        assert_eq!(first.result.report.total_smf_track_count, 4);
        assert_eq!(first.result.report.totals.notes, 2);
        assert_eq!(first.result.report.totals.program_changes, 1);
        assert_eq!(first.result.report.totals.bank_select_msb, 1);
        assert_eq!(first.result.report.totals.bank_select_lsb, 1);
        assert!(first.result.report.warnings.is_empty());
        assert!(!unused_destination.exists());
        fs::remove_file(path).ok();
    }

    #[test]
    fn ui0d2_assembler_failure_is_bounded_export_validation() {
        let mut bytes = crate::export_handoff::tests::portable_project();
        let label_start = {
            let parsed = parse_project_166(&bytes).unwrap();
            parsed.sequences[0].descriptors[2].label_start
        };
        bytes[label_start] = 0xff;
        let (service, path, response) = portable_service_for(bytes);
        let request = export_request(
            response.session_id.clone(),
            response.sequences[0].sequence_id.clone(),
        );
        let error = service.prepare_export_sequence(&request).unwrap_err();
        assert_eq!(error.category, AppErrorCategory::ExportValidationFailed);
        assert_eq!(error.diagnostic_code, "conversion_failed");
        assert_eq!(error.operation, AppOperation::ExportSequence);
        assert!(!error.technical_message.is_empty());
        fs::remove_file(path).ok();
    }

    #[test]
    fn ui0d2_contract_and_session_identity_errors_are_export_operations() {
        let (mut service, path, response) = portable_service();
        let sequence_id = response.sequences[0].sequence_id.clone();

        let mut wrong_version = export_request(response.session_id.clone(), sequence_id.clone());
        wrong_version.contract_version += 1;
        let error = service.prepare_export_sequence(&wrong_version).unwrap_err();
        assert_eq!(error.diagnostic_code, "contract_version_mismatch");
        assert_eq!(error.operation, AppOperation::ExportSequence);

        let unknown = export_request(SessionId::new("missing"), sequence_id.clone());
        let error = service.prepare_export_sequence(&unknown).unwrap_err();
        assert_eq!(error.diagnostic_code, "unknown_session");
        assert_eq!(error.operation, AppOperation::ExportSequence);

        let unknown_sequence = export_request(
            response.session_id.clone(),
            SequenceId::new("missing-sequence"),
        );
        let error = service
            .prepare_export_sequence(&unknown_sequence)
            .unwrap_err();
        assert_eq!(error.diagnostic_code, "unknown_sequence");
        assert_eq!(error.operation, AppOperation::ExportSequence);

        let second = service
            .inspect_project(InspectProjectRequest {
                contract_version: CONTRACT_VERSION,
                source_path: path.to_string_lossy().into_owned(),
                diagnostics_level: DiagnosticsLevel::None,
            })
            .unwrap();
        let cross_session = export_request(second.session_id, sequence_id);
        let error = service.prepare_export_sequence(&cross_session).unwrap_err();
        assert_eq!(error.diagnostic_code, "unknown_sequence");
        assert_eq!(error.operation, AppOperation::ExportSequence);
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn ui0d2_missing_assessment_is_not_export_capable() {
        let (mut service, path, response) = portable_service();
        let sequence_id = response.sequences[0].sequence_id.clone();
        let request = export_request(response.session_id.clone(), sequence_id.clone());
        let session = service.sessions.get_mut(&response.session_id).unwrap();
        assert!(session
            .response
            .sequences
            .iter()
            .any(|sequence| sequence.sequence_id == sequence_id));
        assert!(session.assessments.remove(&sequence_id).is_some());

        let error = service.prepare_export_sequence(&request).unwrap_err();
        assert_eq!(error.category, AppErrorCategory::ExportValidationFailed);
        assert_eq!(error.diagnostic_code, "sequence_not_export_capable");
        assert_ne!(error.diagnostic_code, "unknown_sequence");
        assert_eq!(error.operation, AppOperation::ExportSequence);
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn ui0d2_ready_sequence_and_nonready_sibling_remain_isolated() {
        let (mut service, path, response) = portable_service();
        let ready_id = response.sequences[0].sequence_id.clone();
        let sibling_id = SequenceId::new(format!(
            "{}-synthetic-sibling",
            response.session_id.as_str()
        ));
        let session = service.sessions.get_mut(&response.session_id).unwrap();
        let mut sibling = session.response.sequences[0].clone();
        sibling.sequence_id = sibling_id.clone();
        sibling.display_name = "Non-ready sibling".into();
        sibling.readiness = Readiness::PartiallySupported;
        sibling.export_capability = None;
        session.response.sequences.push(sibling);
        session.response.project.overall_readiness = Readiness::PartiallySupported;
        session.assessments.insert(
            sibling_id.clone(),
            SequenceAssessment {
                structural_ordinal: 1,
                generic_readiness: Readiness::PartiallySupported,
                match_state: SequenceMatchState::NoMatch,
                capability: None,
                resolved_policy: None,
                diagnostic_code: None,
                technical_detail: None,
            },
        );

        let prepared = service
            .prepare_export_sequence(&export_request(response.session_id.clone(), ready_id))
            .unwrap();
        assert_eq!(prepared.sequence_display_name, "Portable Sequence");
        let error = service
            .prepare_export_sequence(&export_request(response.session_id.clone(), sibling_id))
            .unwrap_err();
        assert_eq!(error.category, AppErrorCategory::ExportValidationFailed);
        assert_eq!(error.diagnostic_code, "sequence_not_export_capable");
        assert_eq!(error.operation, AppOperation::ExportSequence);
        let session = service.sessions.get(&response.session_id).unwrap();
        assert_eq!(
            session.response.project.overall_readiness,
            Readiness::PartiallySupported
        );
        assert_eq!(session.response.sequences[0].readiness, Readiness::Ready);
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn ui0d2_revalidation_failures_preserve_export_operation() {
        let (mut service, path, response) = portable_service();
        let request = export_request(
            response.session_id.clone(),
            response.sequences[0].sequence_id.clone(),
        );
        let mut changed = fs::read(&path).unwrap();
        changed[0] ^= 1;
        fs::write(&path, &changed).unwrap();
        let diagnostics_error = service
            .revalidate_sequence_policy(&request.session_id, &request.sequence_id)
            .unwrap_err();
        assert_eq!(diagnostics_error.operation, AppOperation::GetDiagnostics);
        let error = service.prepare_export_sequence(&request).unwrap_err();
        assert_eq!(error.diagnostic_code, "source_identity_changed");
        assert_eq!(error.operation, AppOperation::ExportSequence);

        fs::write(&path, crate::export_handoff::tests::portable_project()).unwrap();
        service.registry = Some(CompatibilityRegistry::empty());
        let error = service.prepare_export_sequence(&request).unwrap_err();
        assert_eq!(error.diagnostic_code, "profile_no_longer_matches");
        assert_eq!(error.operation, AppOperation::ExportSequence);

        service.registry = Some(portable_registry(&fs::read(&path).unwrap()));
        let stored_policy = service
            .sessions
            .get_mut(&response.session_id)
            .unwrap()
            .assessments
            .get_mut(&request.sequence_id)
            .unwrap()
            .resolved_policy
            .as_mut()
            .unwrap();
        let original_output = stored_policy.track_manifest[0].output.clone();
        stored_policy.track_manifest[0].output =
            ResolvedTrackOutputDisposition::OmittedStructuralEmpty;
        let error = service.prepare_export_sequence(&request).unwrap_err();
        assert_eq!(error.diagnostic_code, "profile_policy_changed");
        assert_eq!(error.operation, AppOperation::ExportSequence);

        service
            .sessions
            .get_mut(&response.session_id)
            .unwrap()
            .assessments
            .get_mut(&request.sequence_id)
            .unwrap()
            .resolved_policy
            .as_mut()
            .unwrap()
            .track_manifest[0]
            .output = original_output;
        service
            .sessions
            .get_mut(&response.session_id)
            .unwrap()
            .assessments
            .get_mut(&request.sequence_id)
            .unwrap()
            .resolved_policy
            .as_mut()
            .unwrap()
            .track_manifest[0]
            .output = ResolvedTrackOutputDisposition::Included {
            midi_channel: 16,
            patches: vec![PatchTranslationPolicy::BankSelectAndProgram {
                msb: 81,
                lsb: 2,
                program: 42,
            }],
        };
        let error = service.prepare_export_sequence(&request).unwrap_err();
        assert_eq!(error.diagnostic_code, "profile_policy_changed");
        assert_eq!(error.operation, AppOperation::ExportSequence);

        fs::remove_file(&path).unwrap();
        let error = service.prepare_export_sequence(&request).unwrap_err();
        assert_eq!(error.diagnostic_code, "source_revalidation_failed");
        assert_eq!(error.operation, AppOperation::ExportSequence);
    }

    fn ui0d3_request(
        response: &InspectProjectResponse,
        destination: &Path,
        filename_stem: &str,
        collision_policy: CollisionPolicy,
    ) -> ExportSequenceRequest {
        let mut request = export_request(
            response.session_id.clone(),
            response.sequences[0].sequence_id.clone(),
        );
        request.destination_folder = destination.to_string_lossy().into_owned();
        request.filename_stem = filename_stem.into();
        request.collision_policy = collision_policy;
        request
    }

    fn prepared_report(warnings: Vec<ExportWarning>) -> PreparedExportSequence {
        PreparedExportSequence {
            session_id: SessionId::new("prepared-session"),
            sequence_id: SequenceId::new("prepared-sequence"),
            sequence_display_name: "Prepared Sequence".into(),
            compatibility_profile: ProfileCapability {
                profile_id: "prepared-profile".into(),
                profile_version: 7,
                display_label: "Prepared Profile".into(),
            },
            result: MultitrackExportResult {
                smf_bytes: b"prepared-smf".to_vec(),
                report: MultitrackExportReport {
                    sequence_name: b"Prepared Sequence".to_vec(),
                    musical_track_count: 9,
                    total_smf_track_count: 10,
                    tracks: Vec::new(),
                    totals: MidiExportCounts {
                        notes: 1,
                        generated_note_offs: 2,
                        controllers: 3,
                        bank_select_msb: 4,
                        bank_select_lsb: 5,
                        program_changes: 6,
                        channel_pressure: 7,
                        pitch_bend: 8,
                        tempo: 9,
                        meter: 10,
                    },
                    warnings,
                    untranslated_metadata: vec![UntranslatedMetadata::ControllerContext {
                        source_ordinal: 11,
                    }],
                },
            },
        }
    }

    #[test]
    fn ui0d3_public_export_writes_exact_bytes_and_maps_response() {
        let (service, source, inspection) = portable_service();
        let destination = portable_directory();
        let request = ui0d3_request(
            &inspection,
            &destination,
            "Song.MID",
            CollisionPolicy::FailIfExists,
        );
        let expected = service.prepare_export_sequence(&request).unwrap();

        let response = service.export_sequence(request).unwrap();
        let output = destination.join("Song.mid");
        assert_eq!(response.output_path, output.to_string_lossy());
        assert_eq!(fs::read(&output).unwrap(), expected.result.smf_bytes);
        assert_eq!(response.session_id, expected.session_id);
        assert_eq!(response.sequence_id, expected.sequence_id);
        assert_eq!(
            response.sequence_display_name,
            expected.sequence_display_name
        );
        assert_eq!(
            response.compatibility_profile,
            Some(expected.compatibility_profile)
        );
        assert_eq!(response.musical_track_count, 3);
        assert_eq!(response.total_smf_track_count, 4);
        assert_eq!(response.counts.notes, 2);
        assert_eq!(response.counts.programs, 1);
        assert_eq!(response.counts.bank_select_msb, 1);
        assert_eq!(response.counts.bank_select_lsb, 1);
        assert_eq!(response.validation_status, ValidationStatus::Validated);
        assert!(response.warnings.is_empty());
        assert!(temp_entries(&destination).is_empty());

        fs::remove_dir_all(destination).unwrap();
        fs::remove_file(source).unwrap();
    }

    #[test]
    fn ui0d3_preparation_precedes_destination_and_filename_rules_are_exact() {
        assert_eq!(
            normalize_export_filename_stem("Song").as_deref(),
            Some("Song")
        );
        assert_eq!(
            normalize_export_filename_stem("Song.mid").as_deref(),
            Some("Song")
        );
        assert_eq!(
            normalize_export_filename_stem("Song.MID").as_deref(),
            Some("Song")
        );
        assert_eq!(
            normalize_export_filename_stem("Song.mid.mid").as_deref(),
            Some("Song.mid")
        );
        assert_eq!(export_candidate_filename("Song.mid", 0), "Song.mid.mid");
        assert_eq!(
            normalize_export_filename_stem("embedded.mid.name").as_deref(),
            Some("embedded.mid.name")
        );
        assert_eq!(
            normalize_export_filename_stem(" Song ").as_deref(),
            Some(" Song ")
        );
        for invalid in ["", ".mid", ".MID", ".", "..", "a/b", "a\\b", "a\0b"] {
            assert_eq!(normalize_export_filename_stem(invalid), None, "{invalid:?}");
        }

        let (service, source, inspection) = portable_service();
        let missing = source.with_extension("destination-must-not-exist");
        let mut request =
            ui0d3_request(&inspection, &missing, "Song", CollisionPolicy::FailIfExists);
        request.contract_version += 1;
        let error = service.export_sequence(request).unwrap_err();
        assert_eq!(error.diagnostic_code, "contract_version_mismatch");
        assert_eq!(error.operation, AppOperation::ExportSequence);
        assert!(!missing.exists());

        let invalid_stem =
            ui0d3_request(&inspection, &missing, ".mid", CollisionPolicy::FailIfExists);
        let error = service.export_sequence(invalid_stem).unwrap_err();
        assert_eq!(error.category, AppErrorCategory::ExportValidationFailed);
        assert_eq!(error.diagnostic_code, "invalid_filename_stem");
        assert_eq!(error.operation, AppOperation::ExportSequence);
        assert!(!missing.exists());
        fs::remove_file(source).unwrap();
    }

    #[test]
    fn ui0d3_invalid_destinations_and_directory_symlink_are_bounded() {
        let (service, source, inspection) = portable_service();
        let missing = source.with_extension("missing-directory");
        let request = ui0d3_request(&inspection, &missing, "Song", CollisionPolicy::FailIfExists);
        let error = service.export_sequence(request).unwrap_err();
        assert_eq!(error.category, AppErrorCategory::OutputIoFailed);
        assert_eq!(error.diagnostic_code, "invalid_destination_folder");
        assert_eq!(error.operation, AppOperation::ExportSequence);

        let request = ui0d3_request(&inspection, &source, "Song", CollisionPolicy::FailIfExists);
        let error = service.export_sequence(request).unwrap_err();
        assert_eq!(error.diagnostic_code, "invalid_destination_folder");

        #[cfg(unix)]
        {
            let destination = portable_directory();
            let symlink = destination.with_extension("symlink");
            std::os::unix::fs::symlink(&destination, &symlink).unwrap();
            let request = ui0d3_request(
                &inspection,
                &symlink,
                "Symlink Song",
                CollisionPolicy::FailIfExists,
            );
            let response = service.export_sequence(request).unwrap();
            assert_eq!(fs::read(response.output_path).unwrap()[..4], *b"MThd");
            fs::remove_file(symlink).unwrap();
            fs::remove_dir_all(destination).unwrap();
        }
        fs::remove_file(source).unwrap();
    }

    #[test]
    fn ui0d3_collision_policies_preserve_existing_entries_and_choose_lowest_gap() {
        let (service, source, inspection) = portable_service();
        let fail_directory = portable_directory();
        let occupied = fail_directory.join("Song.mid");
        fs::write(&occupied, b"preserve-me").unwrap();
        let request = ui0d3_request(
            &inspection,
            &fail_directory,
            "Song",
            CollisionPolicy::FailIfExists,
        );
        let error = service.export_sequence(request).unwrap_err();
        assert_eq!(error.category, AppErrorCategory::DestinationExists);
        assert_eq!(error.diagnostic_code, "destination_exists");
        assert_eq!(fs::read(&occupied).unwrap(), b"preserve-me");
        assert!(temp_entries(&fail_directory).is_empty());

        let unique_directory = portable_directory();
        fs::write(unique_directory.join("Song.mid"), b"base").unwrap();
        fs::write(unique_directory.join("Song 3.mid"), b"three").unwrap();
        let request = ui0d3_request(
            &inspection,
            &unique_directory,
            "Song.mid",
            CollisionPolicy::GenerateUniqueName,
        );
        let response = service.export_sequence(request).unwrap();
        assert_eq!(
            response.output_path,
            unique_directory.join("Song 2.mid").to_string_lossy()
        );
        assert_eq!(
            fs::read(unique_directory.join("Song.mid")).unwrap(),
            b"base"
        );
        assert_eq!(
            fs::read(unique_directory.join("Song 3.mid")).unwrap(),
            b"three"
        );

        let base_directory = portable_directory();
        let response = service
            .export_sequence(ui0d3_request(
                &inspection,
                &base_directory,
                "Song",
                CollisionPolicy::GenerateUniqueName,
            ))
            .unwrap();
        assert_eq!(
            response.output_path,
            base_directory.join("Song.mid").to_string_lossy()
        );

        let third_directory = portable_directory();
        fs::write(third_directory.join("Song.mid"), b"base").unwrap();
        fs::write(third_directory.join("Song 2.mid"), b"two").unwrap();
        let response = service
            .export_sequence(ui0d3_request(
                &inspection,
                &third_directory,
                "Song",
                CollisionPolicy::GenerateUniqueName,
            ))
            .unwrap();
        assert_eq!(
            response.output_path,
            third_directory.join("Song 3.mid").to_string_lossy()
        );

        fs::remove_dir_all(fail_directory).unwrap();
        fs::remove_dir_all(unique_directory).unwrap();
        fs::remove_dir_all(base_directory).unwrap();
        fs::remove_dir_all(third_directory).unwrap();
        fs::remove_file(source).unwrap();
    }

    #[test]
    fn ui0d3_public_operation_id_is_inert() {
        let (service, source, inspection) = portable_service();
        let first_directory = portable_directory();
        let second_directory = portable_directory();
        let first = ui0d3_request(
            &inspection,
            &first_directory,
            "Song",
            CollisionPolicy::FailIfExists,
        );
        let mut second = ui0d3_request(
            &inspection,
            &second_directory,
            "Song",
            CollisionPolicy::FailIfExists,
        );
        second.operation_id = Some(OperationId::new("ignored-token"));
        let first_response = service.export_sequence(first).unwrap();
        let second_response = service.export_sequence(second).unwrap();
        assert_eq!(
            fs::read(first_response.output_path).unwrap(),
            fs::read(second_response.output_path).unwrap()
        );
        assert_eq!(first_response.counts, second_response.counts);
        assert_eq!(first_response.warnings, second_response.warnings);
        assert_eq!(
            first_response.compatibility_profile,
            second_response.compatibility_profile
        );
        fs::remove_dir_all(first_directory).unwrap();
        fs::remove_dir_all(second_directory).unwrap();
        fs::remove_file(source).unwrap();
    }

    #[test]
    fn ui0d3_response_preflight_maps_all_counts_and_warnings() {
        let prepared = prepared_report(vec![
            ExportWarning::MeterClocksFallback {
                source_third_payload: 12,
                used: 24,
            },
            ExportWarning::MeterThirtySecondsFallback {
                source_fourth_payload: 6,
                used: 8,
            },
        ]);
        let preflight = AppService::new()
            .preflight_export_response(&prepared)
            .unwrap();
        assert_eq!(preflight.musical_track_count, 9);
        assert_eq!(preflight.total_smf_track_count, 10);
        assert_eq!(preflight.untranslated_metadata_count, 1);
        assert_eq!(preflight.cleanup_warning_source_order, 2);
        assert_eq!(
            preflight.counts,
            AppExportCounts {
                notes: 1,
                generated_note_offs: 2,
                controllers: 3,
                bank_select_msb: 4,
                bank_select_lsb: 5,
                programs: 6,
                pressure: 7,
                pitch_bend: 8,
                tempo: 9,
                meter: 10,
            }
        );
        assert_eq!(preflight.warnings[0].code, "meter_clocks_fallback");
        assert_eq!(preflight.warnings[0].severity, WarningSeverity::Caution);
        assert_eq!(preflight.warnings[0].scope, WarningScope::Sequence);
        assert_eq!(preflight.warnings[0].source_order, 0);
        assert_eq!(
            preflight.warnings[0].message,
            "Phoenix used the standard MIDI clocks-per-click value for this sequence."
        );
        assert_eq!(
            preflight.warnings[0].technical_detail.as_deref(),
            Some("source clocks-per-click 12; used 24")
        );
        assert_eq!(preflight.warnings[1].code, "meter_thirty_seconds_fallback");
        assert_eq!(preflight.warnings[1].source_order, 1);
        assert_eq!(
            preflight.warnings[1].message,
            "Phoenix used the standard MIDI notated-32nd-notes value for this sequence."
        );
        assert_eq!(
            preflight.warnings[1].technical_detail.as_deref(),
            Some("source notated 32nd notes 6; used 8")
        );
        assert!(preflight
            .warnings
            .iter()
            .all(|warning| warning.diagnostic_ref.is_none()));
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn ui0d3_response_overflow_precedes_every_filesystem_operation() {
        let service = AppService::new();
        let mut prepared = prepared_report(Vec::new());
        prepared.result.report.musical_track_count = u32::MAX as usize + 1;
        let expected_session_id = prepared.session_id.clone();
        let expected_sequence_id = prepared.sequence_id.clone();
        let request = ExportSequenceRequest {
            contract_version: CONTRACT_VERSION,
            session_id: expected_session_id.clone(),
            sequence_id: expected_sequence_id.clone(),
            destination_folder: "/must/not/be/accessed/ui0d3-overflow".into(),
            filename_stem: "Overflow".into(),
            collision_policy: CollisionPolicy::FailIfExists,
            operation_id: None,
        };
        let file_system = FaultExportFileSystem::default();

        let error = service
            .commit_prepared_export(
                &request,
                prepared,
                &file_system,
                ExportLimits {
                    candidate_count: 10_000,
                    temp_attempt_count: 128,
                },
            )
            .unwrap_err();

        assert_eq!(error.category, AppErrorCategory::InternalError);
        assert_eq!(error.diagnostic_code, "export_response_overflow");
        assert_eq!(error.operation, AppOperation::ExportSequence);
        assert_eq!(error.session_id, Some(expected_session_id));
        assert_eq!(error.sequence_id, Some(expected_sequence_id));
        assert_eq!(file_system.destination_calls.get(), 0);
        assert_eq!(file_system.create_calls.get(), 0);
        assert_eq!(file_system.write_calls.get(), 0);
        assert_eq!(file_system.sync_calls.get(), 0);
        assert_eq!(file_system.hard_link_calls.get(), 0);
        assert_eq!(file_system.remove_calls.get(), 0);
    }

    #[test]
    fn ui0d3_temp_allocation_retries_only_collisions_and_is_bounded() {
        let service = AppService::new();
        let destination = portable_directory();
        let request = export_request(SessionId::new("s"), SequenceId::new("q"));
        let retry = FaultExportFileSystem::default();
        retry
            .create_errors
            .borrow_mut()
            .extend([io::ErrorKind::AlreadyExists, io::ErrorKind::AlreadyExists]);
        let (path, file) = service
            .allocate_export_temp(&request, &retry, &destination, 3)
            .unwrap();
        drop(file);
        assert_eq!(retry.create_calls.get(), 3);
        fs::remove_file(path).unwrap();

        let exhausted = FaultExportFileSystem::default();
        exhausted
            .create_errors
            .borrow_mut()
            .extend(std::iter::repeat(io::ErrorKind::AlreadyExists).take(128));
        let error = service
            .allocate_export_temp(&request, &exhausted, &destination, 128)
            .unwrap_err();
        assert_eq!(error.diagnostic_code, "temporary_file_allocation_failed");
        assert_eq!(exhausted.create_calls.get(), 128);

        let immediate = FaultExportFileSystem::default();
        immediate.create_errors.borrow_mut().extend([
            io::ErrorKind::PermissionDenied,
            io::ErrorKind::AlreadyExists,
        ]);
        let error = service
            .allocate_export_temp(&request, &immediate, &destination, 128)
            .unwrap_err();
        assert_eq!(error.diagnostic_code, "temporary_file_allocation_failed");
        assert_eq!(immediate.create_calls.get(), 1);
        fs::remove_dir_all(destination).unwrap();
    }

    #[test]
    fn ui0d3_write_sync_and_publication_failures_keep_primary_errors() {
        let service = AppService::new();
        for (stage, diagnostic) in [
            ("write", "output_write_failed"),
            ("flush", "output_sync_failed"),
            ("sync", "output_sync_failed"),
            ("publish", "output_commit_failed"),
        ] {
            let destination = portable_directory();
            let prepared = prepared_report(Vec::new());
            let request = ExportSequenceRequest {
                contract_version: CONTRACT_VERSION,
                session_id: prepared.session_id.clone(),
                sequence_id: prepared.sequence_id.clone(),
                destination_folder: destination.to_string_lossy().into_owned(),
                filename_stem: "Failure".into(),
                collision_policy: CollisionPolicy::FailIfExists,
                operation_id: None,
            };
            let file_system = FaultExportFileSystem::default();
            match stage {
                "write" => file_system.fail_write.set(true),
                "flush" => file_system.fail_flush.set(true),
                "sync" => file_system
                    .sync_errors
                    .borrow_mut()
                    .push_back(io::ErrorKind::Other),
                "publish" => file_system
                    .hard_link_errors
                    .borrow_mut()
                    .push_back(io::ErrorKind::PermissionDenied),
                _ => unreachable!(),
            }
            file_system
                .remove_errors
                .borrow_mut()
                .push_back(io::ErrorKind::PermissionDenied);
            let error = service
                .commit_prepared_export(
                    &request,
                    prepared,
                    &file_system,
                    ExportLimits {
                        candidate_count: 10_000,
                        temp_attempt_count: 128,
                    },
                )
                .unwrap_err();
            assert_eq!(error.category, AppErrorCategory::OutputIoFailed);
            assert_eq!(error.diagnostic_code, diagnostic);
            assert_eq!(error.operation, AppOperation::ExportSequence);
            assert!(error.technical_message.contains("cleanup also failed"));
            assert!(!destination.join("Failure.mid").exists());
            for temp in temp_entries(&destination) {
                fs::remove_file(temp).unwrap();
            }
            fs::remove_dir(destination).unwrap();
        }
    }

    #[test]
    fn ui0d3_publication_collisions_retry_and_exhaust_without_rewriting() {
        let service = AppService::new();
        let destination = portable_directory();
        assert_eq!(EXPORT_CANDIDATE_LIMIT, 10_000);
        assert_eq!(export_candidate_filename("Race", 9_999), "Race 10000.mid");
        let prepared = prepared_report(Vec::new());
        let request = ExportSequenceRequest {
            contract_version: CONTRACT_VERSION,
            session_id: prepared.session_id.clone(),
            sequence_id: prepared.sequence_id.clone(),
            destination_folder: destination.to_string_lossy().into_owned(),
            filename_stem: "Race".into(),
            collision_policy: CollisionPolicy::GenerateUniqueName,
            operation_id: None,
        };
        let file_system = FaultExportFileSystem::default();
        file_system
            .hard_link_errors
            .borrow_mut()
            .push_back(io::ErrorKind::AlreadyExists);
        let response = service
            .commit_prepared_export(
                &request,
                prepared,
                &file_system,
                ExportLimits {
                    candidate_count: 3,
                    temp_attempt_count: 3,
                },
            )
            .unwrap();
        assert_eq!(
            response.output_path,
            destination.join("Race 2.mid").to_string_lossy()
        );
        assert_eq!(file_system.hard_link_calls.get(), 2);
        assert_eq!(file_system.write_calls.get(), 1);

        let fail_prepared = prepared_report(Vec::new());
        let fail_request = ExportSequenceRequest {
            collision_policy: CollisionPolicy::FailIfExists,
            ..request.clone()
        };
        let fail_file_system = FaultExportFileSystem::default();
        fail_file_system
            .hard_link_errors
            .borrow_mut()
            .push_back(io::ErrorKind::AlreadyExists);
        let error = service
            .commit_prepared_export(
                &fail_request,
                fail_prepared,
                &fail_file_system,
                ExportLimits {
                    candidate_count: 3,
                    temp_attempt_count: 3,
                },
            )
            .unwrap_err();
        assert_eq!(error.category, AppErrorCategory::DestinationExists);
        assert_eq!(error.diagnostic_code, "destination_exists");
        assert!(!destination.join("Race.mid").exists());

        let exhausted_prepared = prepared_report(Vec::new());
        let exhausted = FaultExportFileSystem::default();
        exhausted
            .hard_link_errors
            .borrow_mut()
            .extend(std::iter::repeat(io::ErrorKind::AlreadyExists).take(3));
        let error = service
            .commit_prepared_export(
                &request,
                exhausted_prepared,
                &exhausted,
                ExportLimits {
                    candidate_count: 3,
                    temp_attempt_count: 3,
                },
            )
            .unwrap_err();
        assert_eq!(error.category, AppErrorCategory::DestinationExists);
        assert_eq!(error.diagnostic_code, "destination_name_exhausted");
        assert_eq!(exhausted.hard_link_calls.get(), 3);
        assert_eq!(exhausted.write_calls.get(), 1);
        fs::remove_dir_all(destination).unwrap();
    }

    #[test]
    fn ui0d3_post_publication_cleanup_failure_preserves_output_and_warns() {
        let service = AppService::new();
        let destination = portable_directory();
        let prepared = prepared_report(vec![ExportWarning::MeterClocksFallback {
            source_third_payload: 12,
            used: 24,
        }]);
        let expected_bytes = prepared.result.smf_bytes.clone();
        let request = ExportSequenceRequest {
            contract_version: CONTRACT_VERSION,
            session_id: prepared.session_id.clone(),
            sequence_id: prepared.sequence_id.clone(),
            destination_folder: destination.to_string_lossy().into_owned(),
            filename_stem: "Committed".into(),
            collision_policy: CollisionPolicy::FailIfExists,
            operation_id: None,
        };
        let file_system = FaultExportFileSystem::default();
        file_system
            .remove_errors
            .borrow_mut()
            .push_back(io::ErrorKind::PermissionDenied);
        let response = service
            .commit_prepared_export(
                &request,
                prepared,
                &file_system,
                ExportLimits {
                    candidate_count: 10_000,
                    temp_attempt_count: 128,
                },
            )
            .unwrap();
        let output = destination.join("Committed.mid");
        assert!(output.exists());
        assert_eq!(fs::read(&output).unwrap(), expected_bytes);
        assert_eq!(response.output_path, output.to_string_lossy());
        assert_eq!(response.validation_status, ValidationStatus::Validated);
        assert_eq!(response.warnings.len(), 2);
        let warning = &response.warnings[1];
        assert_eq!(warning.code, "temporary_cleanup_failed");
        assert_eq!(warning.severity, WarningSeverity::Caution);
        assert_eq!(warning.scope, WarningScope::Sequence);
        assert_eq!(warning.source_order, 1);
        assert_eq!(warning.message, "The MIDI export succeeded, but Phoenix could not remove a private temporary filesystem entry.");
        assert!(warning.diagnostic_ref.is_none());
        assert_eq!(temp_entries(&destination).len(), 1);

        fs::remove_dir_all(destination).unwrap();
    }

    #[test]
    fn ui0e_public_error_identity_and_context_are_stable() {
        let mut service = AppService::new();
        let contract_error = service
            .inspect_project(InspectProjectRequest {
                contract_version: CONTRACT_VERSION + 1,
                source_path: "/not/accessed/ui0e-contract".into(),
                diagnostics_level: DiagnosticsLevel::None,
            })
            .unwrap_err();
        assert_eq!(contract_error.category, AppErrorCategory::InternalError);
        assert_eq!(contract_error.diagnostic_code, "contract_version_mismatch");
        assert_eq!(contract_error.operation, AppOperation::InspectProject);

        let missing_source = portable_directory().join("missing-source");
        let file_error = service
            .inspect_project(InspectProjectRequest {
                contract_version: CONTRACT_VERSION,
                source_path: missing_source.to_string_lossy().into_owned(),
                diagnostics_level: DiagnosticsLevel::None,
            })
            .unwrap_err();
        assert_eq!(file_error.category, AppErrorCategory::FileUnreadable);
        assert_eq!(file_error.diagnostic_code, "file_read_failed");
        assert_eq!(file_error.operation, AppOperation::InspectProject);
        fs::remove_dir(missing_source.parent().unwrap()).unwrap();

        let unknown_session_id = SessionId::new("unknown-session");
        let session_error = service
            .get_diagnostics(&unknown_session_id, DiagnosticsLevel::Summary)
            .unwrap_err();
        assert_eq!(session_error.category, AppErrorCategory::InternalError);
        assert_eq!(session_error.diagnostic_code, "unknown_session");
        assert_eq!(session_error.operation, AppOperation::GetDiagnostics);
        assert_eq!(session_error.session_id, None);
        assert_eq!(session_error.sequence_id, None);

        let (mut portable, source, inspection) = portable_service();
        let unknown_sequence_id = SequenceId::new("unknown-sequence");
        let sequence_error = portable
            .assessment_for_sequence(&inspection.session_id, &unknown_sequence_id)
            .unwrap_err();
        assert_eq!(sequence_error.category, AppErrorCategory::InternalError);
        assert_eq!(sequence_error.diagnostic_code, "unknown_sequence");
        assert_eq!(sequence_error.operation, AppOperation::GetDiagnostics);
        assert_eq!(
            sequence_error.session_id,
            Some(inspection.session_id.clone())
        );
        assert_eq!(sequence_error.sequence_id, Some(unknown_sequence_id));

        let second_inspection = portable
            .inspect_project(InspectProjectRequest {
                contract_version: CONTRACT_VERSION,
                source_path: source.to_string_lossy().into_owned(),
                diagnostics_level: DiagnosticsLevel::None,
            })
            .unwrap();
        let cross_session_sequence_id = inspection.sequences[0].sequence_id.clone();
        let cross_session_error = portable
            .assessment_for_sequence(&second_inspection.session_id, &cross_session_sequence_id)
            .unwrap_err();
        assert_eq!(
            cross_session_error.category,
            AppErrorCategory::InternalError
        );
        assert_eq!(cross_session_error.diagnostic_code, "unknown_sequence");
        assert_eq!(cross_session_error.operation, AppOperation::GetDiagnostics);
        assert_eq!(
            cross_session_error.session_id,
            Some(second_inspection.session_id)
        );
        assert_eq!(
            cross_session_error.sequence_id,
            Some(cross_session_sequence_id)
        );

        let destination = portable_directory();
        let invalid_stem = ui0d3_request(
            &inspection,
            &destination,
            ".mid",
            CollisionPolicy::FailIfExists,
        );
        let validation_error = portable.export_sequence(invalid_stem).unwrap_err();
        assert_eq!(
            validation_error.category,
            AppErrorCategory::ExportValidationFailed
        );
        assert_eq!(validation_error.diagnostic_code, "invalid_filename_stem");
        assert_eq!(validation_error.operation, AppOperation::ExportSequence);

        let missing_destination = destination.join("missing");
        let destination_request = ui0d3_request(
            &inspection,
            &missing_destination,
            "Song",
            CollisionPolicy::FailIfExists,
        );
        let destination_error = portable.export_sequence(destination_request).unwrap_err();
        assert_eq!(destination_error.category, AppErrorCategory::OutputIoFailed);
        assert_eq!(
            destination_error.diagnostic_code,
            "invalid_destination_folder"
        );
        assert_eq!(destination_error.operation, AppOperation::ExportSequence);

        fs::remove_dir(destination).unwrap();
        fs::remove_file(source).unwrap();
    }

    #[test]
    fn ui0e_source_revalidation_code_tuple_distinguishes_failure_categories() {
        let (unreadable_service, unreadable_path, unreadable_inspection) = portable_service();
        let unreadable_request = export_request(
            unreadable_inspection.session_id,
            unreadable_inspection.sequences[0].sequence_id.clone(),
        );
        fs::remove_file(unreadable_path).unwrap();
        let unreadable = unreadable_service
            .prepare_export_sequence(&unreadable_request)
            .unwrap_err();
        assert_eq!(unreadable.category, AppErrorCategory::FileUnreadable);
        assert_eq!(unreadable.diagnostic_code, "source_revalidation_failed");
        assert_eq!(unreadable.operation, AppOperation::ExportSequence);

        let (mut invalid_service, invalid_path, invalid_inspection) = portable_service();
        let invalid_request = export_request(
            invalid_inspection.session_id.clone(),
            invalid_inspection.sequences[0].sequence_id.clone(),
        );
        let invalid_bytes = vec![0; fs::read(&invalid_path).unwrap().len()];
        fs::write(&invalid_path, &invalid_bytes).unwrap();
        invalid_service
            .sessions
            .get_mut(&invalid_inspection.session_id)
            .unwrap()
            .source_sha256 = sha256_hex(&invalid_bytes);
        let invalid = invalid_service
            .prepare_export_sequence(&invalid_request)
            .unwrap_err();
        assert_eq!(invalid.category, AppErrorCategory::ExportValidationFailed);
        assert_eq!(invalid.diagnostic_code, "source_revalidation_failed");
        assert_eq!(invalid.operation, AppOperation::ExportSequence);
        fs::remove_file(invalid_path).unwrap();
    }

    #[test]
    fn ui0e_unsupported_cancellation_is_inert_and_preserves_committed_export() {
        let (service, source, inspection) = portable_service();
        let destination = portable_directory();
        let before = service.get_inspection(&inspection.session_id).unwrap();
        let response = service
            .export_sequence(ui0d3_request(
                &inspection,
                &destination,
                "Committed",
                CollisionPolicy::FailIfExists,
            ))
            .unwrap();
        let committed_bytes = fs::read(&response.output_path).unwrap();
        let session_count = service.session_count();

        for operation_id in [
            OperationId::new("arbitrary-a"),
            OperationId::new("arbitrary-b"),
        ] {
            let error = service.cancel_operation(&operation_id).unwrap_err();
            assert_eq!(error.contract_version, CONTRACT_VERSION);
            assert_eq!(error.category, AppErrorCategory::InternalError);
            assert_eq!(error.diagnostic_code, "cancellation_not_supported");
            assert_eq!(error.operation, AppOperation::CancelOperation);
            assert_eq!(error.session_id, None);
            assert_eq!(error.sequence_id, None);
        }

        assert_eq!(service.session_count(), session_count);
        assert_eq!(
            service.get_inspection(&inspection.session_id).unwrap(),
            before
        );
        assert_eq!(fs::read(&response.output_path).unwrap(), committed_bytes);
        assert_eq!(response.validation_status, ValidationStatus::Validated);

        fs::remove_dir_all(destination).unwrap();
        fs::remove_file(source).unwrap();
    }

    fn summary(readiness: Readiness) -> SequenceSummary {
        SequenceSummary {
            sequence_id: SequenceId::new("sequence"),
            display_name: "Sequence".into(),
            readiness,
            readiness_reason: ReadinessReason::new(
                ReadinessReasonCode::MissingChannelRouting,
                "generic",
            ),
            musical_track_count: Some(1),
            supported_event_families: Vec::new(),
            warning_count: 0,
            export_capability: None,
            diagnostics_available: true,
        }
    }

    fn matched_assessment() -> SequenceAssessment {
        SequenceAssessment {
            structural_ordinal: 0,
            generic_readiness: Readiness::PartiallySupported,
            match_state: SequenceMatchState::Matched,
            capability: Some(crate::app_contract::ProfileCapability {
                profile_id: "profile".into(),
                profile_version: 1,
                display_label: "Validated profile".into(),
            }),
            resolved_policy: Some(ResolvedProfilePolicy {
                profile_id: crate::compatibility::ProfileId::new("profile"),
                profile_version: crate::compatibility::ProfileVersion::new(1),
                sequence: crate::compatibility::ResolvedSequenceIdentity {
                    structural_ordinal: 0,
                    sequence_range: ByteRange::new(0, 0).unwrap(),
                },
                track_manifest: Vec::new(),
            }),
            diagnostic_code: None,
            technical_detail: None,
        }
    }

    #[test]
    fn empty_walk_has_empty_inventory() {
        let walk = walk_bounded_mixed_events(
            &[],
            MixedEventBounds { event_range: 0..0 },
            MixedEventTimingBasis::default(),
        )
        .unwrap();
        assert_eq!(inventory_families(&walk), (Vec::new(), 0, Vec::new()));
    }

    #[test]
    fn repeated_notes_count_logical_events_once_per_occurrence() {
        let bytes = [
            0x00, 0x90, 0x3c, 0x40, 0x20, 0x01, 0x00, 0x90, 0x3d, 0x41, 0x20, 0x01,
        ];
        let walk = walk_bounded_mixed_events(
            &bytes,
            MixedEventBounds {
                event_range: 0..bytes.len(),
            },
            MixedEventTimingBasis::default(),
        )
        .unwrap();
        let (families, count, patch_evidence) = inventory_families(&walk);
        assert_eq!(families, vec![EvidenceEventFamily::Note]);
        assert_eq!(count, 2);
        assert!(patch_evidence.is_empty());
    }

    #[test]
    fn matched_projection_sets_ready_reason_and_safe_capability() {
        let mut sequence = summary(Readiness::PartiallySupported);
        project_sequence_readiness(&mut sequence, &matched_assessment());
        assert_eq!(sequence.readiness, Readiness::Ready);
        assert_eq!(
            sequence.readiness_reason.code,
            ReadinessReasonCode::ValidatedCompatibilityProfile
        );
        assert_eq!(
            sequence
                .export_capability
                .as_ref()
                .map(|capability| capability.profile_id.as_str()),
            Some("profile")
        );
    }

    #[test]
    fn nonmatched_projection_preserves_generic_readiness_and_capability_absence() {
        let mut sequence = summary(Readiness::PartiallySupported);
        let mut assessment = matched_assessment();
        assessment.match_state = SequenceMatchState::NoMatch;
        assessment.capability = None;
        assessment.resolved_policy = None;
        project_sequence_readiness(&mut sequence, &assessment);
        assert_eq!(sequence.readiness, Readiness::PartiallySupported);
        assert_eq!(
            sequence.readiness_reason.code,
            ReadinessReasonCode::MissingChannelRouting
        );
        assert!(sequence.export_capability.is_none());
    }

    #[test]
    fn overall_readiness_handles_empty_all_ready_and_mixed_projects() {
        assert_eq!(overall_readiness(&[]), Readiness::Unknown);
        assert_eq!(
            overall_readiness(&[summary(Readiness::Ready), summary(Readiness::Ready)]),
            Readiness::Ready
        );
        assert_eq!(
            overall_readiness(&[
                summary(Readiness::Ready),
                summary(Readiness::PartiallySupported)
            ]),
            Readiness::PartiallySupported
        );
        assert_eq!(
            overall_readiness(&[
                summary(Readiness::Unsupported),
                summary(Readiness::Unsupported)
            ]),
            Readiness::Unsupported
        );
        assert_eq!(
            overall_readiness(&[summary(Readiness::Unsupported), summary(Readiness::Unknown)]),
            Readiness::Unknown
        );
    }
}
