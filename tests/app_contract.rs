use phoenix::app_contract::*;

#[test]
fn version_and_api_info_are_explicit() {
    assert_eq!(CONTRACT_VERSION, 1);
    assert_eq!(ApiInfo::new("0.1.0").contract_version, 1);
}

#[test]
fn identifiers_are_owned_opaque_values() {
    let name = String::from("same display name");
    let id = SequenceId::new(name.clone());
    drop(name);
    assert_eq!(id.as_str(), "same display name");
    assert_ne!(SessionId::new("a"), SessionId::new("b"));
    assert_ne!(OperationId::new("a"), OperationId::new("b"));
    assert_eq!(SequenceId::new("one"), SequenceId::new("one"));
}

#[test]
fn readiness_and_reason_codes_are_deliberate() {
    assert_eq!(Readiness::Ready.stable_name(), "ready");
    assert_eq!(Readiness::PartiallySupported.stable_code(), 2);
    assert_eq!(Readiness::Unsupported.stable_code(), 3);
    assert_eq!(Readiness::Unknown.stable_code(), 4);
    let ready = ReadinessReason::new(
        ReadinessReasonCode::ValidatedCompatibilityProfile,
        "validated",
    );
    assert!(ready.export_enabled);
    let loss = ReadinessReason::new(
        ReadinessReasonCode::MissingChannelRouting,
        "routing unknown",
    );
    assert!(!loss.export_enabled);
    assert_eq!(loss.severity, ReasonSeverity::DataLossRisk);
}

#[test]
fn warnings_have_explicit_deterministic_order() {
    let make = |scope: WarningScope, code: &str, source_order: u32| Warning {
        code: code.into(),
        message: "display text differs freely".into(),
        technical_detail: None,
        scope,
        severity: WarningSeverity::Caution,
        diagnostic_ref: None,
        source_order,
    };
    let mut warnings = [
        make(WarningScope::Sequence, "z", 0),
        make(WarningScope::Project, "z", 9),
        make(WarningScope::Sequence, "a", 3),
        make(WarningScope::Sequence, "a", 1),
    ];
    warnings.sort_by(compare_warnings);
    assert_eq!(
        warnings
            .iter()
            .map(|warning| (
                warning.scope.stable_code(),
                warning.code.as_str(),
                warning.source_order
            ))
            .collect::<Vec<_>>(),
        vec![(1, "z", 9), (2, "a", 1), (2, "a", 3), (2, "z", 0)]
    );
}

#[test]
fn owned_project_and_sequence_summaries_outlive_inputs() {
    fn build() -> (ProjectSummary, SequenceSummary) {
        let display = String::from("project");
        let sequence_name = String::from("duplicate");
        let project = ProjectSummary {
            display_name: display,
            byte_size: 4_294_967_296,
            identification: IdentificationSummary {
                recognized: true,
                label: "Studio Vision".into(),
                confidence: "high".into(),
                profile_label: None,
            },
            recognized_studio_vision: true,
            profile_label: None,
            sequence_count: 2,
            overall_readiness: Readiness::Ready,
            warning_count: 0,
            diagnostics_available: false,
        };
        let sequence = SequenceSummary {
            sequence_id: SequenceId::new("session-sequence-0"),
            display_name: sequence_name,
            readiness: Readiness::Ready,
            readiness_reason: ReadinessReason::new(
                ReadinessReasonCode::ValidatedCompatibilityProfile,
                "ready",
            ),
            musical_track_count: Some(1),
            supported_event_families: vec![EventFamilySummary {
                family: EventFamily::Note,
                count: 4,
                supported: true,
            }],
            warning_count: 0,
            export_capability: Some(ProfileCapability {
                profile_id: "profile".into(),
                profile_version: 1,
                display_label: "Validated profile".into(),
            }),
            diagnostics_available: false,
        };
        (project, sequence)
    }
    let (project, sequence) = build();
    assert_eq!(project.byte_size, 4_294_967_296);
    assert_eq!(sequence.display_name, "duplicate");
}

#[test]
fn request_response_types_are_owned_and_policy_limited() {
    let request = ExportSequenceRequest {
        contract_version: CONTRACT_VERSION,
        session_id: SessionId::new("session"),
        sequence_id: SequenceId::new("sequence"),
        destination_folder: "/tmp/output".into(),
        filename_stem: "sequence".into(),
        collision_policy: CollisionPolicy::FailIfExists,
        operation_id: Some(OperationId::new("operation")),
    };
    assert_eq!(request.collision_policy.stable_name(), "fail_if_exists");
    let response = ExportSequenceResponse {
        session_id: request.session_id,
        sequence_id: request.sequence_id,
        sequence_display_name: "sequence".into(),
        output_path: "/tmp/output/sequence.mid".into(),
        compatibility_profile: None,
        musical_track_count: 1,
        total_smf_track_count: 2,
        counts: ExportCounts {
            notes: 2,
            generated_note_offs: 2,
            tempo: 1,
            meter: 1,
            ..ExportCounts::default()
        },
        warnings: Vec::new(),
        untranslated_metadata_count: 0,
        validation_status: ValidationStatus::Validated,
    };
    assert_eq!(response.counts.notes, 2);
}

#[test]
fn counts_aggregate_with_overflow_protection() {
    let left = ExportCounts {
        notes: u64::MAX,
        ..ExportCounts::default()
    };
    let right = ExportCounts {
        notes: 1,
        ..ExportCounts::default()
    };
    assert_eq!(left.checked_add(&right), Err(CountOverflow));
    let one = ExportCounts {
        notes: 1,
        ..ExportCounts::default()
    };
    assert_eq!(one.checked_add(&one).unwrap().notes, 2);
}

#[test]
fn diagnostics_audio_and_error_models_are_owned() {
    let diagnostics = Diagnostics {
        core_version: "0.1.0".into(),
        contract_version: CONTRACT_VERSION,
        source_sha256: Some("hash".into()),
        identification_evidence: vec!["local".into()],
        recognized_profile: None,
        structural_status: Some("complete".into()),
        unsupported_families: Vec::new(),
        compatibility_profile: None,
        technical_errors: Vec::new(),
        export_report: None,
    };
    let audio = AudioReferenceSummary {
        display_name: "future.wav".into(),
        path_hint: None,
        status: AudioReferenceStatus::Unknown,
        provenance_confidence: ProvenanceConfidence::Unknown,
    };
    let error = AppError {
        contract_version: CONTRACT_VERSION,
        category: AppErrorCategory::DestinationExists,
        display_message: "Choose another destination.".into(),
        technical_message: "destination exists".into(),
        operation: AppOperation::ExportSequence,
        session_id: Some(SessionId::new("session")),
        sequence_id: None,
        diagnostic_code: "destination_exists".into(),
        diagnostic_ref: None,
    };
    assert_eq!(diagnostics.source_sha256.as_deref(), Some("hash"));
    assert_eq!(audio.status, AudioReferenceStatus::Unknown);
    assert_eq!(error.category.stable_name(), "destination_exists");
}

#[test]
fn every_public_cross_boundary_enum_has_stable_identity() {
    assert_eq!(CollisionPolicy::GenerateUniqueName.stable_code(), 2);
    assert_eq!(DiagnosticsLevel::Full.stable_code(), 3);
    assert_eq!(
        AppOperation::CancelOperation.stable_name(),
        "cancel_operation"
    );
    assert_eq!(AppErrorCategory::InternalError.stable_code(), 11);
    assert_eq!(EventFamily::Meter.stable_name(), "meter");
}
