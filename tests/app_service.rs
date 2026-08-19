use phoenix::app_contract::{
    AppErrorCategory, AppOperation, DiagnosticsLevel, InspectProjectRequest, Readiness,
    CONTRACT_VERSION,
};
use phoenix::app_service::AppService;
use phoenix::compatibility::EvidenceEventFamily;
use phoenix::mixed_event::{walk_bounded_mixed_events, MixedEventBounds, MixedEventTimingBasis};
use phoenix::sequence_container::{parse_project_166, TrackAssociations};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_TEMP_FILE: AtomicU64 = AtomicU64::new(1);

fn temp_file(bytes: &[u8]) -> PathBuf {
    let nonce = NEXT_TEMP_FILE.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!("phoenix-app-service-{nonce}.bin"));
    fs::write(&path, bytes).expect("write temporary input");
    path
}

fn request(path: &Path, level: DiagnosticsLevel) -> InspectProjectRequest {
    InspectProjectRequest {
        contract_version: CONTRACT_VERSION,
        source_path: path.to_string_lossy().into_owned(),
        diagnostics_level: level,
    }
}

#[test]
fn service_starts_empty_and_generates_distinct_opaque_sessions() {
    let mut service = AppService::new();
    assert_eq!(service.session_count(), 0);
    let first_path = temp_file(b"not a Studio Vision project");
    let second_path = temp_file(b"another readable file");
    let first = service
        .inspect_project(request(&first_path, DiagnosticsLevel::Summary))
        .expect("readable file should produce an assessment");
    let second = service
        .inspect_project(request(&second_path, DiagnosticsLevel::Summary))
        .expect("readable file should produce an assessment");
    assert_ne!(first.session_id, second.session_id);
    assert_eq!(first.project.overall_readiness, Readiness::Unknown);
    assert!(first.session_id.as_str().starts_with("session-"));
    assert_eq!(service.session_count(), 2);
    fs::remove_file(first_path).ok();
    fs::remove_file(second_path).ok();
}

#[test]
fn missing_file_is_typed_inspection_error() {
    let mut service = AppService::new();
    let path = std::env::temp_dir().join("phoenix-app-service-does-not-exist");
    let error = service
        .inspect_project(request(&path, DiagnosticsLevel::None))
        .expect_err("missing file must fail");
    assert_eq!(error.category, AppErrorCategory::FileUnreadable);
    assert_eq!(error.operation, AppOperation::InspectProject);
    assert_eq!(error.diagnostic_code, "file_read_failed");
}

#[test]
fn wrong_contract_version_is_rejected_deterministically() {
    let mut service = AppService::new();
    let path = temp_file(b"input");
    let mut request = request(&path, DiagnosticsLevel::None);
    request.contract_version = CONTRACT_VERSION + 1;
    let error = service
        .inspect_project(request)
        .expect_err("version mismatch");
    assert_eq!(error.category, AppErrorCategory::InternalError);
    assert_eq!(error.operation, AppOperation::InspectProject);
    assert_eq!(error.diagnostic_code, "contract_version_mismatch");
    fs::remove_file(path).ok();
}

#[test]
fn readable_unrecognized_file_is_owned_and_diagnostics_are_bounded() {
    let mut service = AppService::new();
    let path = temp_file(&[0, 1, 2, 3, 4, 5]);
    let response = service
        .inspect_project(request(&path, DiagnosticsLevel::None))
        .expect("readable arbitrary input should be inspectable");
    assert!(!response.project.recognized_studio_vision);
    assert_eq!(response.project.sequence_count, 0);
    assert_eq!(response.project.overall_readiness, Readiness::Unknown);
    assert_eq!(response.sequences.len(), 0);
    let none = service
        .get_diagnostics(&response.session_id, DiagnosticsLevel::None)
        .expect("diagnostics session");
    assert!(none.source_sha256.is_none());
    assert!(none.identification_evidence.is_empty());
    let full = service
        .get_diagnostics(&response.session_id, DiagnosticsLevel::Full)
        .expect("full diagnostics session");
    assert_eq!(full.source_sha256.as_deref().map(str::len), Some(64));
    assert!(!full.technical_errors.is_empty());
    fs::remove_file(path).ok();
}

#[test]
fn unknown_session_is_a_stable_error() {
    let service = AppService::new();
    let error = service
        .get_diagnostics(
            &phoenix::app_contract::SessionId::new("session-99999999"),
            DiagnosticsLevel::Summary,
        )
        .expect_err("unknown session");
    assert_eq!(error.category, AppErrorCategory::InternalError);
    assert_eq!(error.diagnostic_code, "unknown_session");
}

#[test]
fn evidence_for_unrecognized_session_is_unavailable_not_fabricated() {
    let mut service = AppService::new();
    let path = temp_file(b"not a project");
    let response = service
        .inspect_project(request(&path, DiagnosticsLevel::None))
        .expect("readable input should be assessed");
    let error = service
        .profile_evidence(&response.session_id)
        .expect_err("no parser structure means no fabricated evidence");
    assert_eq!(error.diagnostic_code, "profile_evidence_unavailable");
    fs::remove_file(path).ok();
}

#[test]
fn source_identity_is_retained_for_future_revalidation() {
    let mut service = AppService::new();
    let path = temp_file(b"identity");
    let response = service
        .inspect_project(request(&path, DiagnosticsLevel::Summary))
        .expect("inspection");
    let (stored_path, size, hash) = service
        .source_identity(&response.session_id)
        .expect("identity");
    assert_eq!(stored_path, path.to_string_lossy());
    assert_eq!(size, 8);
    assert_eq!(hash.len(), 64);
    fs::remove_file(path).ok();
}

#[test]
fn optional_authentic_fixture_preserves_structural_sequence_order() {
    let path = PathBuf::from(
        "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline",
    );
    if !path.is_file() {
        return;
    }
    let mut service = AppService::new();
    let response = service
        .inspect_project(request(&path, DiagnosticsLevel::Summary))
        .expect("authentic fixture should inspect");
    let readiness_before = response.project.overall_readiness;
    assert!(!response.sequences.is_empty());
    assert!(response
        .sequences
        .windows(2)
        .all(|pair| pair[0].sequence_id < pair[1].sequence_id));
    let evidence = service
        .profile_evidence(&response.session_id)
        .expect("established parser should produce owned evidence");
    let bytes = fs::read(&path).expect("fixture bytes");
    let parsed = parse_project_166(&bytes).expect("fixture parser");
    let (_, source_size, source_hash) = service
        .source_identity(&response.session_id)
        .expect("source identity");
    assert_eq!(evidence.source_byte_size, source_size);
    assert_eq!(evidence.source_sha256, source_hash);
    assert_eq!(evidence.source_sha256.len(), 64);
    assert_eq!(evidence.parser_profile.as_str(), "descriptor166");
    assert_eq!(
        service
            .get_inspection(&response.session_id)
            .unwrap()
            .project
            .overall_readiness,
        readiness_before
    );
    assert_eq!(evidence.sequences.len(), response.sequences.len());
    if let Ok(name) = std::str::from_utf8(&evidence.sequences[0].name_bytes) {
        assert_eq!(name, response.sequences[0].display_name);
    }
    let tracks: Vec<_> = evidence
        .sequences
        .iter()
        .flat_map(|sequence| sequence.tracks.iter())
        .collect();
    assert!(!tracks.is_empty());
    assert!(tracks.iter().all(|track| {
        !track.evidence_complete
            && track.exact_event_range.is_some()
            && track.patch_evidence.iter().all(|patch| {
                patch.decoded_bank_msb.is_none()
                    && patch.decoded_bank_lsb.is_none()
                    && track.exact_event_range.is_some_and(|range| {
                        patch.source_range.start() >= range.start()
                            && patch.source_range.end_exclusive() <= range.end_exclusive()
                    })
            })
            && track.observed_channel.is_none()
    }));
    assert!(tracks.iter().any(|track| !track.patch_evidence.is_empty()));
    assert!(tracks.iter().any(|track| {
        track.decoded_event_count > 0 && !track.decoded_event_families.is_empty()
    }));
    assert!(tracks.iter().any(|track| {
        track
            .decoded_event_families
            .contains(&EvidenceEventFamily::Patch)
    }));
    assert!(tracks
        .iter()
        .all(|track| { track.decoded_event_count > 0 || track.decoded_event_families.is_empty() }));
    let mut compared = false;
    for (sequence_index, sequence) in parsed.sequences.iter().enumerate() {
        let TrackAssociations::Ordinal(bindings) = &sequence.track_associations else {
            continue;
        };
        for pair in &sequence.track_pairs {
            let Ok(bounds) = pair.validated_event_bounds() else {
                continue;
            };
            let Ok(direct) = walk_bounded_mixed_events(
                &bytes,
                MixedEventBounds {
                    event_range: bounds.event_range.clone(),
                },
                MixedEventTimingBasis::default(),
            ) else {
                continue;
            };
            let track_index = bindings
                .iter()
                .position(|binding| binding.pair_ordinal == pair.pair_ordinal)
                .expect("ordinal pair binding");
            assert_eq!(
                evidence.sequences[sequence_index].tracks[track_index].decoded_event_count,
                direct.logical_event_count() as u64
            );
            compared = true;
            break;
        }
        if compared {
            break;
        }
    }
    assert!(compared, "at least one direct bounded walk should succeed");
}
