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
use crate::compatibility::{
    ByteRange, CompatibilityRegistry, EvidenceEventFamily, ParserProfileId, PatchEvidence,
    ProfileEvidence, ProfileMatch, ResolvedProfilePolicy, SequenceEvidence, TrackEvidence,
};
use crate::identification::{identify, read_finder_metadata};
use crate::inspection::inspect;
use crate::mixed_event::{
    walk_bounded_mixed_events, MixedEventBounds, MixedEventItem, MixedEventKind,
    MixedEventTimingBasis,
};
use crate::sequence_container::{parse_project_166, TrackAssociations};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

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
    pub(crate) resolved_policy: ResolvedProfilePolicy,
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
                next_session: 1,
                registry: None,
                registry_error: Some(format!("{error:?}")),
            },
        }
    }

    pub fn with_registry(registry: CompatibilityRegistry) -> Self {
        Self {
            sessions: HashMap::new(),
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
        let assessment = session
            .assessments
            .get(sequence_id)
            .ok_or_else(|| self.unknown_sequence(session_id, sequence_id))?;
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
            .ok_or_else(|| self.unknown_sequence(session_id, sequence_id))
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

    /// Internal handoff for a future exporter. The returned bytes and policy
    /// belong to the same successful revalidation call, avoiding a stale
    /// policy or a second source read at the export boundary.
    #[allow(clippy::result_large_err)]
    pub(crate) fn revalidated_policy_for_sequence(
        &self,
        session_id: &SessionId,
        sequence_id: &SequenceId,
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
                .ok_or_else(|| self.unknown_session(AppOperation::GetDiagnostics))?;
            let assessment = session
                .assessments
                .get(sequence_id)
                .ok_or_else(|| self.unknown_sequence(session_id, sequence_id))?;
            let Some(policy) = assessment.resolved_policy.clone() else {
                return Err(self.error(
                    AppErrorCategory::ExportValidationFailed,
                    "This sequence has no freshly validated compatibility policy.",
                    "the inspection assessment did not retain a matched profile policy".into(),
                    AppOperation::GetDiagnostics,
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
                AppOperation::GetDiagnostics,
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
                AppOperation::GetDiagnostics,
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
                AppOperation::GetDiagnostics,
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
                AppOperation::GetDiagnostics,
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
                    AppOperation::GetDiagnostics,
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
                AppOperation::GetDiagnostics,
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
                AppOperation::GetDiagnostics,
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
                AppOperation::GetDiagnostics,
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
                    AppOperation::GetDiagnostics,
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
                    AppOperation::GetDiagnostics,
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
                AppOperation::GetDiagnostics,
                "profile_policy_changed",
                Some(session_id.clone()),
                Some(sequence_id.clone()),
            ));
        }
        Ok(FreshValidatedSequence {
            source_bytes: fresh_bytes,
            source_sha256: fresh_sha256,
            structural_ordinal: ordinal,
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
            .ok_or_else(|| self.unknown_sequence(session_id, sequence_id))
    }

    fn allocate_session_id(&mut self) -> SessionId {
        let value = format!("session-{:08}", self.next_session);
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
    fn unknown_sequence(&self, session_id: &SessionId, sequence_id: &SequenceId) -> AppError {
        self.error(
            AppErrorCategory::InternalError,
            "The selected sequence is not available in this inspection session.",
            "no sequence exists for the supplied session and sequence identifiers".into(),
            AppOperation::GetDiagnostics,
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
                MixedEventKind::Note(_) | MixedEventKind::ContextMediatedNote(_) => {
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
mod tests {
    use super::*;

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
                tracks: Vec::new(),
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
