use phoenix::app_contract::{DiagnosticsLevel, InspectProjectRequest, CONTRACT_VERSION};
use phoenix::app_service::AppService;
use phoenix::compatibility::{ProfileMatch, ProfileMismatchReason};
use phoenix::compatibility_profiles::built_in_compatibility_registry;
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

const SOURCE: &str = "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline";

static NEXT_TEMP: AtomicU64 = AtomicU64::new(1);

fn inspect(path: &Path) -> (AppService, phoenix::app_contract::InspectProjectResponse) {
    let mut service = AppService::new();
    let response = service
        .inspect_project(InspectProjectRequest {
            contract_version: CONTRACT_VERSION,
            source_path: path.to_string_lossy().into_owned(),
            diagnostics_level: DiagnosticsLevel::Full,
        })
        .expect("authentic source should inspect");
    (service, response)
}

fn assess(path: &Path) -> ProfileMatch {
    let (service, response) = inspect(path);
    let sequence = response
        .sequences
        .iter()
        .find(|sequence| sequence.display_name == "Ode to Clarke")
        .expect("target sequence");
    let ordinal = service
        .sequence_ordinal_for_id(&response.session_id, &sequence.sequence_id)
        .expect("sequence identity mapping");
    let evidence = service
        .profile_evidence(&response.session_id)
        .expect("owned evidence");
    built_in_compatibility_registry()
        .expect("built-in profile validates")
        .assess(&evidence, ordinal)
        .expect("assessment should not be ambiguous")
}

#[test]
fn authentic_profile_matches_complete_policy() {
    let path = Path::new(SOURCE);
    if !path.is_file() {
        return;
    }
    let ProfileMatch::Matched {
        capability,
        resolved_policy,
    } = assess(path)
    else {
        panic!("authenticated source must match built-in profile");
    };
    assert_eq!(capability.profile_id, "studio_vision_ode_to_clarke_v1");
    assert_eq!(capability.profile_version, 1);
    assert_eq!(
        capability.display_label,
        "Validated research profile — Ode to Clarke"
    );
    assert_eq!(resolved_policy.sequence.structural_ordinal, 14);
    assert_eq!(resolved_policy.tracks.len(), 9);
    assert_eq!(
        resolved_policy
            .tracks
            .iter()
            .map(|track| track.midi_channel)
            .collect::<Vec<_>>(),
        vec![1, 2, 10, 10, 10, 1, 10, 15, 10]
    );
    assert_eq!(
        resolved_policy
            .tracks
            .iter()
            .map(|track| track.patches.len())
            .sum::<usize>(),
        4
    );
}

#[test]
fn renamed_identical_bytes_match_and_mutated_bytes_do_not() {
    let source = Path::new(SOURCE);
    if !source.is_file() {
        return;
    }
    let bytes = fs::read(source).expect("source bytes");
    let nonce = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
    let renamed = std::env::temp_dir().join(format!("phoenix-ode-renamed-{nonce}"));
    fs::write(&renamed, &bytes).expect("renamed copy");
    assert!(matches!(assess(&renamed), ProfileMatch::Matched { .. }));

    let mutated = std::env::temp_dir().join(format!("phoenix-ode-mutated-{nonce}"));
    let mut changed = bytes;
    changed[0] ^= 1;
    fs::write(&mutated, changed).expect("mutated copy");
    assert!(matches!(assess(&mutated), ProfileMatch::NoMatch));
    fs::remove_file(renamed).ok();
    fs::remove_file(mutated).ok();
}

#[test]
fn same_name_unrelated_input_and_other_sequence_never_match() {
    let source = Path::new(SOURCE);
    if !source.is_file() {
        return;
    }
    let unrelated = std::env::temp_dir().join("phoenix-ode-same-name-unrelated");
    fs::write(&unrelated, b"Ode to Clarke").expect("unrelated input");
    let mut unrelated_service = AppService::new();
    let unrelated_response = unrelated_service
        .inspect_project(InspectProjectRequest {
            contract_version: CONTRACT_VERSION,
            source_path: unrelated.to_string_lossy().into_owned(),
            diagnostics_level: DiagnosticsLevel::None,
        })
        .expect("readable unrelated input");
    assert!(unrelated_response.sequences.is_empty());
    let (service, response) = inspect(source);
    let target = response
        .sequences
        .iter()
        .find(|sequence| sequence.display_name == "Ode to Clarke")
        .unwrap();
    let ordinal = service
        .sequence_ordinal_for_id(&response.session_id, &target.sequence_id)
        .unwrap();
    let mut unrelated_evidence = service.profile_evidence(&response.session_id).unwrap();
    unrelated_evidence.source_sha256 = "0".repeat(64);
    assert!(matches!(
        built_in_compatibility_registry()
            .unwrap()
            .assess(&unrelated_evidence, ordinal)
            .unwrap(),
        ProfileMatch::NoMatch
    ));
    fs::remove_file(unrelated).ok();

    let (service, response) = inspect(source);
    let other = response
        .sequences
        .iter()
        .find(|sequence| sequence.display_name != "Ode to Clarke")
        .expect("another sequence");
    let other_ordinal = service
        .sequence_ordinal_for_id(&response.session_id, &other.sequence_id)
        .unwrap();
    let result = built_in_compatibility_registry()
        .unwrap()
        .assess(
            &service.profile_evidence(&response.session_id).unwrap(),
            other_ordinal,
        )
        .unwrap();
    assert!(!matches!(result, ProfileMatch::Matched { .. }));
}

#[test]
fn structural_and_patch_drift_reject_after_provenance_candidate() {
    let source = Path::new(SOURCE);
    if !source.is_file() {
        return;
    }
    let (service, response) = inspect(source);
    let sequence = response
        .sequences
        .iter()
        .find(|sequence| sequence.display_name == "Ode to Clarke")
        .unwrap();
    let ordinal = service
        .sequence_ordinal_for_id(&response.session_id, &sequence.sequence_id)
        .unwrap();
    let mut evidence = service.profile_evidence(&response.session_id).unwrap();
    evidence.sequences[ordinal as usize].sequence_range =
        phoenix::compatibility::ByteRange::new(1, 2).unwrap();
    assert!(matches!(
        built_in_compatibility_registry()
            .unwrap()
            .assess(&evidence, ordinal)
            .unwrap(),
        ProfileMatch::Rejected {
            reason: ProfileMismatchReason::SequenceIdentityMismatch,
            ..
        }
    ));

    let mut evidence = service.profile_evidence(&response.session_id).unwrap();
    evidence.sequences[ordinal as usize].tracks[0].patch_evidence[0].decoded_program ^= 1;
    assert!(matches!(
        built_in_compatibility_registry()
            .unwrap()
            .assess(&evidence, ordinal)
            .unwrap(),
        ProfileMatch::Rejected {
            reason: ProfileMismatchReason::PatchPolicyMismatch,
            ..
        }
    ));
}
