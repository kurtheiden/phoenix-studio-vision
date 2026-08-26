use phoenix::app_contract::{
    CollisionPolicy, DiagnosticsLevel, ExportSequenceRequest, InspectProjectRequest,
    CONTRACT_VERSION,
};
use phoenix::app_service::AppService;
use phoenix::compatibility::{
    PatchTranslationPolicy, ProfileMatch, ProfileMismatchReason, ResolvedTrackOutputDisposition,
};
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
    assess_named(path, "Ode to Clarke")
}

fn assess_named(path: &Path, sequence_name: &str) -> ProfileMatch {
    let (service, response) = inspect(path);
    let sequence = response
        .sequences
        .iter()
        .find(|sequence| sequence.display_name == sequence_name)
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
fn authentic_bells_profile_matches_complete_manifest() {
    let path = Path::new(SOURCE);
    if !path.is_file() {
        return;
    }
    let result = assess_named(path, "Bells for her");
    let ProfileMatch::Matched {
        capability,
        resolved_policy,
    } = result
    else {
        panic!("authenticated Bells source must match built-in profile");
    };
    assert_eq!(capability.profile_id, "studio_vision_bells_for_her_v1");
    assert_eq!(capability.profile_version, 1);
    assert_eq!(
        capability.display_label,
        "Validated research profile — Bells for her"
    );
    assert_eq!(resolved_policy.sequence.structural_ordinal, 1);
    assert_eq!(resolved_policy.track_manifest.len(), 14);
    let included = [1, 3, 4, 5, 6, 8, 9, 11, 12, 14];
    let nonempty_omitted = [2, 7];
    let empty_omitted = [10, 13];
    let mut included_ordinals = Vec::new();
    let mut nonempty_ordinals = Vec::new();
    let mut empty_ordinals = Vec::new();
    let mut channels = Vec::new();
    let mut patch_policies = Vec::new();
    for entry in &resolved_policy.track_manifest {
        let ordinal = entry.key.pair_ordinal + 1;
        match &entry.output {
            ResolvedTrackOutputDisposition::Included {
                midi_channel,
                patches,
            } => {
                included_ordinals.push(ordinal);
                channels.push(*midi_channel);
                patch_policies.push((ordinal, patches.clone()));
            }
            ResolvedTrackOutputDisposition::OmittedAuthenticatedNonempty { .. } => {
                nonempty_ordinals.push(ordinal)
            }
            ResolvedTrackOutputDisposition::OmittedStructuralEmpty => empty_ordinals.push(ordinal),
        }
    }
    assert_eq!(included_ordinals, included);
    assert_eq!(nonempty_ordinals, nonempty_omitted);
    assert_eq!(empty_ordinals, empty_omitted);
    for entry in &resolved_policy.track_manifest {
        match entry.output {
            ResolvedTrackOutputDisposition::OmittedAuthenticatedNonempty {
                decoded_event_count,
                ref decoded_event_families,
                ref patches,
            } => {
                let expected_count = match entry.key.pair_ordinal + 1 {
                    2 => 83,
                    7 => 165,
                    ordinal => panic!("unexpected nonempty omission {ordinal}"),
                };
                assert_eq!(decoded_event_count, expected_count);
                assert_eq!(
                    decoded_event_families,
                    &[
                        phoenix::compatibility::EvidenceEventFamily::Patch,
                        phoenix::compatibility::EvidenceEventFamily::Note
                    ]
                );
                assert_eq!(patches.len(), 1);
            }
            ResolvedTrackOutputDisposition::OmittedStructuralEmpty => {}
            ResolvedTrackOutputDisposition::Included { .. } => {}
        }
    }
    assert_eq!(channels, vec![1, 16, 2, 3, 1, 16, 12, 8, 10, 15]);
    assert_eq!(
        patch_policies,
        vec![
            (1, vec![PatchTranslationPolicy::ProgramOnly { program: 16 }]),
            (
                3,
                vec![PatchTranslationPolicy::BankSelectMsbAndProgram {
                    msb: 81,
                    program: 25
                }]
            ),
            (
                4,
                vec![PatchTranslationPolicy::BankSelectAndProgram {
                    msb: 81,
                    lsb: 1,
                    program: 34
                }]
            ),
            (
                5,
                vec![PatchTranslationPolicy::BankSelectAndProgram {
                    msb: 80,
                    lsb: 0,
                    program: 70
                }]
            ),
            (
                6,
                vec![PatchTranslationPolicy::BankSelectMsbAndProgram {
                    msb: 81,
                    program: 35
                }]
            ),
            (
                8,
                vec![PatchTranslationPolicy::BankSelectMsbAndProgram {
                    msb: 81,
                    program: 25
                }]
            ),
            (
                9,
                vec![PatchTranslationPolicy::BankSelectMsbAndProgram {
                    msb: 81,
                    program: 122
                }]
            ),
            (
                11,
                vec![PatchTranslationPolicy::BankSelectAndProgram {
                    msb: 80,
                    lsb: 0,
                    program: 12
                }]
            ),
            (12, Vec::new()),
            (14, Vec::new()),
        ]
    );
}

#[test]
fn authentic_bells_is_ready_and_exports_only_included_tracks() {
    let path = Path::new(SOURCE);
    if !path.is_file() {
        return;
    }
    let mut service = AppService::new();
    let response = service
        .inspect_project(InspectProjectRequest {
            contract_version: CONTRACT_VERSION,
            source_path: path.to_string_lossy().into_owned(),
            diagnostics_level: DiagnosticsLevel::Full,
        })
        .expect("authentic Bells source should inspect");
    let sequence = response
        .sequences
        .iter()
        .find(|sequence| sequence.display_name == "Bells for her")
        .expect("Bells sequence");
    assert_eq!(
        sequence
            .export_capability
            .as_ref()
            .map(|c| c.profile_id.as_str()),
        Some("studio_vision_bells_for_her_v1")
    );
    assert!(matches!(
        sequence.readiness,
        phoenix::app_contract::Readiness::Ready
    ));
    let destination = std::env::temp_dir().join(format!(
        "phoenix-bells-profile-{}-{}",
        std::process::id(),
        NEXT_TEMP.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(&destination).expect("temporary destination");
    let exported = service
        .export_sequence(ExportSequenceRequest {
            contract_version: CONTRACT_VERSION,
            session_id: response.session_id,
            sequence_id: sequence.sequence_id.clone(),
            destination_folder: destination.to_string_lossy().into_owned(),
            filename_stem: "Bells for her".into(),
            collision_policy: CollisionPolicy::FailIfExists,
            operation_id: None,
        })
        .expect("Bells export should be conversion-ready");
    assert_eq!(exported.musical_track_count, 10);
    assert_eq!(exported.total_smf_track_count, 11);
    assert_eq!(exported.counts.notes, 3_186);
    assert_eq!(exported.counts.programs, 8);
    assert_eq!(exported.counts.bank_select_msb, 7);
    assert_eq!(exported.counts.bank_select_lsb, 3);
    assert_eq!(exported.counts.controllers, 395);
    assert_eq!(exported.counts.pressure, 32);
    assert_eq!(exported.counts.pitch_bend, 102);
    assert!(destination.join("Bells for her.mid").is_file());
    let generated = fs::read(destination.join("Bells for her.mid")).expect("generated SMF");
    assert_eq!(&generated[..4], b"MThd");
    assert_eq!(&generated[8..10], &[0, 1]);
    assert_eq!(&generated[10..12], &[0, 11]);
    assert_eq!(&generated[12..14], &[1, 224]);
    let names = [
        b"Track 1".as_slice(),
        b"Track 3",
        b"Track 4",
        b"Track 5",
        b"Track 6",
        b"Track 8",
        b"Track 9",
        b"Track 11",
        b"Track 12",
        b"Track 14",
    ];
    let positions = names
        .iter()
        .map(|name| {
            generated
                .windows(name.len())
                .position(|window| window == *name)
                .expect("generated musical track name")
        })
        .collect::<Vec<_>>();
    assert!(positions.windows(2).all(|pair| pair[0] < pair[1]));
    fs::remove_dir_all(destination).ok();
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
    assert_eq!(resolved_policy.track_manifest.len(), 9);
    assert_eq!(
        resolved_policy
            .track_manifest
            .iter()
            .map(|track| match track.output {
                ResolvedTrackOutputDisposition::Included { midi_channel, .. } => midi_channel,
                _ => panic!("Ode tracks must remain included"),
            })
            .collect::<Vec<_>>(),
        vec![1, 2, 10, 10, 10, 1, 10, 15, 10]
    );
    assert_eq!(
        resolved_policy
            .track_manifest
            .iter()
            .map(|track| match &track.output {
                ResolvedTrackOutputDisposition::Included { patches, .. } => patches.len(),
                _ => panic!("Ode tracks must remain included"),
            })
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
