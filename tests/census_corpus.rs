//! Explicit opt-in only. Missing fixtures are unavailable coverage, never success.
use phoenix::{app_contract::*, app_service::AppService, bounded_sequence::*};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::PathBuf};

#[test]
#[ignore = "requires approved Experiment 007 and a private pre-change byte ledger"]
fn approved_baseline_exports_remain_identical() {
    let source = std::env::var("PHOENIX_CENSUS_BASELINE")
        .expect("UNAVAILABLE coverage: PHOENIX_CENSUS_BASELINE required");
    let ledger = PathBuf::from(
        std::env::var("PHOENIX_CENSUS_LEDGER")
            .expect("UNAVAILABLE coverage: private PHOENIX_CENSUS_LEDGER required"),
    );
    let bytes = fs::read(&source).expect("UNAVAILABLE coverage: approved baseline unreadable");
    let mut app = AppService::new();
    let inspected = app
        .inspect_project(InspectProjectRequest {
            contract_version: 1,
            source_path: source.clone(),
            diagnostics_level: DiagnosticsLevel::Full,
        })
        .unwrap();
    assert_eq!(inspected.sequences.len(), 18);
    let census = phoenix::census::census_path(std::path::Path::new(&source)).unwrap();
    assert_eq!(census.metrics.complete_include_muted, 6);
    assert_eq!(census.metrics.complete_exclude_muted, 6);
    assert_eq!(census.metrics.application_authorized, 9);
    assert_eq!(census.metrics.proven_policy_only, 0);
    assert_eq!(census.metrics.sequences_with_decoding_blockers, 0);
    use phoenix::census::{Family, Outcome, Stage};
    assert_eq!(
        census.metrics.blocker_instances,
        BTreeMap::from([
            (Family::Translation, 25),
            (Family::Conductor, 5),
            (Family::Routing, 4),
            (Family::Ordering, 3),
            (Family::Ownership, 1),
        ])
    );
    let expected_patches = [0, 6, 1, 2, 1, 0, 2, 1, 0, 0, 1, 2, 6, 0, 3, 0, 0, 0];
    let mut categories = BTreeMap::new();
    let mut ranges = std::collections::BTreeSet::new();
    for (ordinal, s) in census.sequences.iter().enumerate() {
        let complete = [0, 5, 13, 15, 16, 17].contains(&ordinal);
        for p in [&s.general_include_muted, &s.general_exclude_muted] {
            assert_eq!(
                p.outcome,
                if complete {
                    Outcome::Passed
                } else {
                    Outcome::Failed
                }
            );
        }
        let patches: Vec<_> = s
            .diagnostics
            .iter()
            .filter(|d| d.stage == Stage::PatchTranslation && d.outcome == Outcome::Failed)
            .collect();
        assert_eq!(
            patches.len(),
            expected_patches[ordinal],
            "sequence {ordinal}"
        );
        for patch in patches {
            *categories.entry(patch.reason_code.as_str()).or_insert(0) += 1;
            let range = patch.source_range.as_ref().expect("located Patch refusal");
            assert!(
                ranges.insert((range.start, range.end)),
                "duplicate Patch instance"
            );
            assert_eq!(patch.blocker_family, Some(Family::Translation));
            assert!(patch.blocks_general_recovery);
            assert!(patch.prerequisites.is_empty());
            for stage in [Stage::EventWalk, Stage::SourceAccounting, Stage::Routing] {
                assert_eq!(
                    s.diagnostics
                        .iter()
                        .find(|d| d.stage == stage && d.pair_ordinal == patch.pair_ordinal)
                        .unwrap()
                        .outcome,
                    Outcome::Passed
                );
            }
            for stage in [Stage::Translation, Stage::TrackExport] {
                let d = s
                    .diagnostics
                    .iter()
                    .find(|d| d.stage == stage && d.pair_ordinal == patch.pair_ordinal)
                    .unwrap();
                assert_eq!(d.outcome, Outcome::NotEvaluated);
                assert!(!d.prerequisites.is_empty());
            }
        }
        if ![0, 13].contains(&ordinal) {
            assert_eq!(s.application_include_muted.outcome, Outcome::NotApplicable);
            assert_eq!(s.application_exclude_muted.outcome, Outcome::NotApplicable);
        }
    }
    assert_eq!(
        categories,
        BTreeMap::from([
            ("NonzeroPosition", 14),
            ("PayloadLength", 5),
            ("unsupported_standalone_patch", 5),
            ("UnsupportedProgram", 1)
        ])
    );
    let output = ledger.parent().unwrap().join("regression-midi");
    fs::create_dir(&output).unwrap();
    let mut digests = BTreeMap::new();
    let mut ready = 0;
    for (ordinal, sequence) in inspected.sequences.iter().enumerate() {
        let probe = &census.sequences[ordinal];
        if sequence.readiness != Readiness::Ready {
            let error = app
                .export_sequence(ExportSequenceRequest {
                    contract_version: 1,
                    session_id: inspected.session_id.clone(),
                    sequence_id: sequence.sequence_id.clone(),
                    destination_folder: output.to_str().unwrap().into(),
                    filename_stem: format!("blocked-{ordinal}"),
                    collision_policy: CollisionPolicy::FailIfExists,
                    operation_id: None,
                })
                .unwrap_err();
            assert_eq!(
                probe.application_default.outcome,
                phoenix::census::Outcome::Failed
            );
            assert_eq!(probe.application_default.reason_code, error.diagnostic_code);
            continue;
        }
        assert_eq!(
            probe.application_default.outcome,
            phoenix::census::Outcome::Passed
        );
        ready += 1;
        for mode in [
            None,
            Some(RecoveryInclusionOption::IncludeMuted),
            Some(RecoveryInclusionOption::ExcludeMuted),
        ] {
            if mode.is_some() && ![0, 13].contains(&ordinal) {
                continue;
            }
            let key = format!("application/{ordinal}/{mode:?}");
            let request = ExportSequenceRequest {
                contract_version: 1,
                session_id: inspected.session_id.clone(),
                sequence_id: sequence.sequence_id.clone(),
                destination_folder: output.to_str().unwrap().into(),
                filename_stem: format!("sequence-{ordinal}-{}", digests.len()),
                collision_policy: CollisionPolicy::FailIfExists,
                operation_id: None,
            };
            let mode_probe = match mode {
                None => &probe.application_default,
                Some(RecoveryInclusionOption::IncludeMuted) => &probe.application_include_muted,
                Some(RecoveryInclusionOption::ExcludeMuted) => &probe.application_exclude_muted,
            };
            let result = match mode {
                None => app.export_sequence(request),
                Some(mode) => app.export_sequence_with_recovery_inclusion(request, mode),
            };
            assert_eq!(
                mode_probe.outcome,
                if result.is_ok() {
                    Outcome::Passed
                } else {
                    Outcome::Failed
                }
            );
            if let Err(e) = &result {
                assert_eq!(mode_probe.reason_code, e.diagnostic_code);
            }
            result.unwrap();
            assert_eq!(mode_probe.reason_code, "passed");
            // Only this test's freshly created files exist in the private directory.
            let name = fs::read_dir(&output)
                .unwrap()
                .map(|e| e.unwrap().path())
                .find(|p| {
                    p.file_stem().unwrap().to_string_lossy()
                        == format!("sequence-{ordinal}-{}", digests.len())
                })
                .unwrap();
            digests.insert(
                key,
                format!("{:x}", Sha256::digest(fs::read(name).unwrap())),
            );
        }
    }
    assert_eq!(ready, 9);
    for mode in [
        RecoveryInclusion::IncludeMuted,
        RecoveryInclusion::ExcludeMuted,
    ] {
        let mut passed = Vec::new();
        for ordinal in 0..18 {
            let probe = match mode {
                RecoveryInclusion::IncludeMuted => &census.sequences[ordinal].general_include_muted,
                RecoveryInclusion::ExcludeMuted => &census.sequences[ordinal].general_exclude_muted,
            };
            let actual = build_recovery_sequence_manifest(&bytes, ordinal, 480, mode);
            if let Err(e) = &actual {
                assert_eq!(probe.outcome, phoenix::census::Outcome::Failed);
                assert_eq!(probe.stage, e.stage);
            }
            if let Ok(m) = actual {
                assert_eq!(probe.outcome, phoenix::census::Outcome::Passed);
                let out = assemble_recovery_sequence(&m).unwrap();
                digests.insert(
                    format!("general/{ordinal}/{mode:?}"),
                    format!("{:x}", Sha256::digest(&out.smf_bytes)),
                );
                passed.push(ordinal);
            }
        }
        assert_eq!(passed, vec![0, 5, 13, 15, 16, 17]);
    }
    fs::remove_dir_all(output).unwrap();
    if std::env::var_os("PHOENIX_CENSUS_CAPTURE").is_some() {
        assert!(!ledger.exists(), "never overwrite pre-change ledger");
        fs::write(ledger, serde_json::to_vec_pretty(&digests).unwrap()).unwrap();
    } else {
        let previous: BTreeMap<String, String> = serde_json::from_slice(
            &fs::read(ledger).expect("UNAVAILABLE coverage: pre-change ledger missing"),
        )
        .unwrap();
        assert_eq!(digests, previous, "authorized MIDI bytes changed");
    }
    assert_eq!(fs::read(source).unwrap(), bytes, "source changed");
}
