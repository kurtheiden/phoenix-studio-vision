//! Explicit opt-in only. Missing fixtures are unavailable coverage, never success.
use phoenix::{app_contract::*, app_service::AppService, bounded_sequence::*};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::PathBuf};
#[path = "support/reference_smf.rs"]
mod reference;

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
    // Authentic, source-bound positive fixtures, including the additional
    // zero-tick match discovered by the complete census. These facts are literal
    // expectations, independent of the census's fallback classification.
    for (sequence, pair, range, program, post, ticks, name_len) in [
        (
            1,
            6,
            0x124de..0x12508,
            58,
            *b"\x03I83\x04\xff\xff\xff",
            (320, 71477),
            13,
        ),
        (
            10,
            0,
            0x2598e..0x259b6,
            19,
            *b"\x03I34\x04\xff\xff\xff",
            (0, 2),
            14,
        ),
        (
            12,
            13,
            0x2d050..0x2d07d,
            15,
            *b"\x03I28\x04\xff\xff\xff",
            (1500, 41520),
            16,
        ),
    ] {
        let q = phoenix::program_only166::qualify_track(&bytes, sequence, pair).unwrap();
        let p = q.provenance();
        assert!(q.controller_prefix().is_empty());
        assert_eq!(p.composition_range, range);
        assert_eq!(p.pre_name_bytes, [0, 0, program, 0, program]);
        assert_eq!(p.post_name_bytes, post);
        assert_eq!(
            (p.previous_tick, p.patch_tick, p.note_tick),
            (0, ticks.0, ticks.1)
        );
        assert_eq!(p.logical_source_ordinal, 0);
        assert_eq!(p.raw_source_object_ordinal, 0);
        assert_eq!(&bytes[p.composition_range.clone()], p.composition_bytes);
        assert_eq!(p.post_name_range.start - p.pre_name_range.end - 1, name_len);
    }
    let census = phoenix::census::census_path(std::path::Path::new(&source)).unwrap();
    assert_eq!(census.metrics.complete_include_muted, 9);
    assert_eq!(census.metrics.complete_exclude_muted, 9);
    assert_eq!(census.metrics.application_authorized, 10);
    assert_eq!(census.metrics.proven_policy_only, 1);
    assert_eq!(census.metrics.sequences_with_decoding_blockers, 0);
    use phoenix::census::{Family, Outcome, Stage};
    assert_eq!(
        census.metrics.blocker_instances,
        BTreeMap::from([
            (Family::Translation, 14),
            (Family::Conductor, 3),
            (Family::Ordering, 3),
            (Family::Ownership, 1),
        ])
    );
    let expected_patches = [0, 2, 0, 2, 1, 0, 0, 0, 0, 0, 0, 2, 4, 0, 3, 0, 0, 0];
    let mut categories = BTreeMap::new();
    let mut ranges = std::collections::BTreeSet::new();
    for (ordinal, s) in census.sequences.iter().enumerate() {
        let complete = [0, 2, 5, 9, 10, 13, 15, 16, 17].contains(&ordinal);
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
            ("NonzeroPosition", 8),
            ("PayloadLength", 3),
            ("unsupported_standalone_patch", 3)
        ])
    );
    // All six new source instances must reach the production-backed census
    // through exactly the known wrapper, including the standalone case.
    for (sequence, pair) in [(1, 2), (1, 7), (1, 8), (2, 1), (6, 6), (6, 7)] {
        let q = phoenix::bounded_patch_translation::qualify_descriptor166_bank_patch(
            &bytes, sequence, pair,
        )
        .unwrap();
        let event = q.decoded_export_event();
        assert!(matches!(
            event.kind,
            phoenix::midi_export::DecodedExportEventKind::TargetedPatch {
                translation: phoenix::midi_export::PatchTranslation::ConfirmedBankSelectMsb {
                    msb: 81
                },
                ..
            }
        ));
        let rows: Vec<_> = census.sequences[sequence]
            .diagnostics
            .iter()
            .filter(|d| d.stage == Stage::PatchTranslation && d.pair_ordinal == Some(pair))
            .collect();
        assert!(!rows.is_empty());
        assert!(rows.iter().all(|d| d.outcome == Outcome::Passed));
    }
    // Composition, not a new wrapper: all retained bytes and raw contexts survive.
    let q = phoenix::bank_patch166_handoff::qualify_track(&bytes, 7, 5).unwrap();
    let p = q.provenance();
    assert_eq!((p.patch_tick, p.note_tick), (62, 360));
    assert_eq!(
        (
            p.logical_source_ordinal,
            p.raw_patch_ordinal,
            p.raw_note_ordinal
        ),
        (0, 1, 2)
    );
    assert_eq!(
        &bytes[p.initial_context_range.clone().unwrap()],
        p.initial_context_bytes.as_ref().unwrap()
    );
    let project = phoenix::sequence_container::parse_project_166(&bytes).unwrap();
    let bounds = project.sequences[12]
        .validated_track_event_bounds(9)
        .unwrap();
    let walk = phoenix::mixed_event::walk_bounded_mixed_events(
        &bytes,
        phoenix::mixed_event::MixedEventBounds {
            event_range: bounds.event_range,
        },
        Default::default(),
    )
    .unwrap();
    let mut logical = 0;
    let mut found = false;
    for item in &walk.items {
        if let phoenix::mixed_event::MixedEventItem::Patch(patch) = item {
            if patch.patch.representation_range.start == 0x2ca22 {
                let later =
                    phoenix::bounded_patch_translation::qualify_descriptor166_bank_patch_at(
                        &bytes, 12, 9, logical,
                    )
                    .unwrap();
                assert_eq!(later.source(), item);
                assert!(logical > 0);
                assert_eq!(patch.patch.program_change.value, 40);
                found = true;
            }
        }
        logical += item.logical_event_count() as u64;
    }
    assert!(found, "literal later standalone Patch range");
    for (sequence, pair) in [(7, 5), (12, 9)] {
        assert!(census.sequences[sequence]
            .diagnostics
            .iter()
            .filter(|d| d.stage == Stage::PatchTranslation && d.pair_ordinal == Some(pair))
            .all(|d| d.outcome == Outcome::Passed));
    }
    for stage in [Stage::TempoCoverage, Stage::MeterCoverage] {
        assert_eq!(
            census.sequences[9]
                .diagnostics
                .iter()
                .find(|d| d.stage == stage)
                .unwrap()
                .outcome,
            Outcome::Passed
        );
    }
    // Literal expected routing, independent of the qualifier's returned channel.
    let newsong =
        build_recovery_sequence_manifest(&bytes, 9, 480, RecoveryInclusion::IncludeMuted).unwrap();
    let wire = assemble_recovery_sequence(&newsong).unwrap().smf_bytes;
    let tr = reference::midi_tracks(&wire);
    assert_eq!(tr.len(), 5); // Conductor + four nonempty source tracks; empty fifth retained in coverage.
    let channels = |index: usize| {
        tr[index]
            .iter()
            .filter(|(_, s, _)| *s >= 128 && *s < 240)
            .cloned()
            .collect::<Vec<_>>()
    };
    assert_eq!(channels(1), [(0, 0xc8, vec![90]), (480, 0xcb, vec![120])]);
    assert_eq!(channels(2), [(0, 0xce, vec![9])]);
    for (pair, index, channel, count) in [(2, 3, 9, 102), (3, 4, 10, 115)] {
        let bounds = project.sequences[9]
            .validated_track_event_bounds(pair)
            .unwrap();
        let walk = phoenix::mixed_event::walk_bounded_mixed_events(
            &bytes,
            phoenix::mixed_event::MixedEventBounds {
                event_range: bounds.event_range,
            },
            Default::default(),
        )
        .unwrap();
        let mut expected = Vec::new();
        for item in walk.items {
            let phoenix::mixed_event::MixedEventItem::Event(e) = item else {
                panic!("Notes only");
            };
            let phoenix::mixed_event::MixedEventKind::Note(n) = e.event else {
                panic!("Notes only");
            };
            expected.push((
                e.position,
                0x90 + channel - 1,
                vec![n.pitch.value, n.attack_velocity.value],
            ));
            expected.push((
                e.position + n.duration.value,
                0x80 + channel - 1,
                vec![n.pitch.value, n.release_velocity.value],
            ));
        }
        let mut actual = channels(index);
        assert_eq!(actual.len(), count * 2);
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }
    assert!(census.sequences[9].proven_policy_only);
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
    assert_eq!(ready, 10);
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
        assert_eq!(passed, vec![0, 2, 5, 9, 10, 13, 15, 16, 17]);
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
        assert_eq!(
            previous.len(),
            25,
            "private pre-change ledger must contain all 25 outputs"
        );
        for (key, digest) in &previous {
            assert_eq!(
                digests.get(key),
                Some(digest),
                "authorized MIDI bytes changed: {key}"
            );
        }
        let added: Vec<_> = digests
            .keys()
            .filter(|key| !previous.contains_key(*key))
            .map(String::as_str)
            .collect();
        assert_eq!(
            added,
            [
                "application/2/None",
                "general/10/ExcludeMuted",
                "general/10/IncludeMuted",
                "general/2/ExcludeMuted",
                "general/2/IncludeMuted",
                "general/9/ExcludeMuted",
                "general/9/IncludeMuted"
            ]
        );
        assert_eq!(digests.len(), 32);
    }
    assert_eq!(fs::read(source).unwrap(), bytes, "source changed");
}
