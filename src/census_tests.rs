use super::*;
use crate::{
    bounded_sequence::tests::{synthetic, NOTE},
    sequence_container::parse_project_166,
};
use std::{
    fs,
    sync::atomic::{AtomicU64, Ordering},
};
static NEXT: AtomicU64 = AtomicU64::new(0);
fn fixture(events: &[u8], count: u32) -> Vec<u8> {
    let mut b = synthetic(events);
    let p = parse_project_166(&b).unwrap();
    let start = p.sequences[0].track_pairs[0].primary.payload.range.start;
    b[start + 1] = 1;
    b[start + 2..start + 6].copy_from_slice(&count.to_be_bytes());
    b
}
fn row(s: &SequenceCensus, stage: Stage) -> &Diagnostic {
    s.diagnostics.iter().find(|d| d.stage == stage).unwrap()
}
#[test]
fn independent_conductor_and_track_failures_and_prerequisite_skips() {
    let mut b = fixture(&[0, 0xff, 0x01], 1);
    let p = parse_project_166(&b).unwrap();
    let tempo = p.sequences[0].initial_tempo_range.start;
    b[tempo + 2] = 0x52;
    let report = census_bytes(&b);
    let s = &report.sequences[0];
    assert_eq!(row(s, Stage::Tempo).outcome, Outcome::Failed);
    assert_eq!(
        row(s, Stage::TempoCoverage).prerequisites,
        vec![row(s, Stage::Tempo).id]
    );
    assert_eq!(row(s, Stage::EventWalk).outcome, Outcome::Failed);
    assert!(row(s, Stage::EventWalk).cursor.is_some());
    assert_eq!(row(s, Stage::Translation).outcome, Outcome::NotEvaluated);
    assert_eq!(
        row(s, Stage::Translation).prerequisites,
        vec![row(s, Stage::EventWalk).id]
    );
    assert_eq!(row(s, Stage::SavedMute).outcome, Outcome::NotEvaluated);
    assert_eq!(row(s, Stage::Routing).outcome, Outcome::NotEvaluated);
}
#[test]
fn accounting_is_independent_of_routing_and_policy() {
    let mut b = fixture(NOTE, 2);
    let p = parse_project_166(&b).unwrap();
    let label = p.sequences[0].descriptors[2]
        .label
        .as_ref()
        .unwrap()
        .range
        .start;
    b[label - 31] = 99;
    let report = census_bytes(&b);
    let s = &report.sequences[0];
    assert_eq!(row(s, Stage::Routing).outcome, Outcome::Failed);
    assert_eq!(row(s, Stage::SourceAccounting).outcome, Outcome::Failed);
    assert_eq!(row(s, Stage::TrackExport).outcome, Outcome::NotEvaluated);
}
#[test]
fn unknown_mute_is_evidence_failure_not_invented_policy() {
    let mut b = fixture(NOTE, 1);
    let p = parse_project_166(&b).unwrap();
    let offset = p.sequences[0].descriptors[2].range.start - 24;
    b[offset] = 0;
    let report = census_bytes(&b);
    let s = &report.sequences[0];
    assert_eq!(row(s, Stage::SavedMute).outcome, Outcome::Failed);
    assert_eq!(row(s, Stage::SavedMute).reason_code, "UnsupportedValue");
    assert!(!s.proven_policy_only); // No application probe in pure API.
    assert_eq!(row(s, Stage::EventWalk).outcome, Outcome::Passed);
}
#[test]
fn exact_empty_track_is_accounted_and_bad_empty_is_not_applicable_to_nothing() {
    let mut b = fixture(&[], 0);
    let p = parse_project_166(&b).unwrap();
    let pair = &p.sequences[0].track_pairs[0];
    let tail = pair.validated_event_bounds().unwrap().tail_range;
    let secondary = pair.secondary.record_range.clone();
    b[tail].copy_from_slice(&[255, 255, 255, 127, 255, 47, 0]);
    let payload = [
        0, 1, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 4, 15, 255, 255, 255, 47,
        197, 255, 255, 255,
    ];
    let mut record = vec![41, 0, 0, 0, 31];
    record.extend(payload);
    b.splice(secondary, record);
    let report = census_bytes(&b);
    let s = &report.sequences[0];
    assert_eq!(row(s, Stage::EmptyTrack).outcome, Outcome::Passed);
    assert_eq!(row(s, Stage::EventWalk).outcome, Outcome::NotApplicable);
    let p = parse_project_166(&b).unwrap();
    let offset = p.sequences[0].track_pairs[0].secondary.payload.range.start;
    b[offset] = 7;
    let report = census_bytes(&b);
    let s = &report.sequences[0];
    assert_eq!(row(s, Stage::EmptyTrack).outcome, Outcome::Failed);
    assert_eq!(row(s, Stage::EventWalk).outcome, Outcome::Passed);
    assert_eq!(row(s, Stage::SourceAccounting).outcome, Outcome::Passed);
    assert_eq!(row(s, Stage::Routing).outcome, Outcome::NotEvaluated);
    assert_eq!(
        row(s, Stage::Routing).prerequisites,
        vec![row(s, Stage::EmptyTrack).id]
    );
}
#[test]
fn deterministic_and_sanitized_and_malformed_structure() {
    let b = fixture(NOTE, 1);
    let a = census_bytes(&b);
    assert_eq!(a, census_bytes(&b));
    let json = serde_json::to_string(&a).unwrap();
    for forbidden in [
        "Other",
        "New!",
        "Test",
        "sha256",
        "device_name",
        "source_path",
    ] {
        assert!(!json.contains(forbidden));
    }
    let bad = census_bytes(&[0; 3]);
    assert!(bad.sequences.is_empty());
    assert_eq!(bad.diagnostics[0].outcome, Outcome::Failed);
    assert_eq!(bad.metrics.application_authorized, 0);
}
#[test]
fn production_probe_parity_uses_actual_builders_and_public_application_export() {
    let root = std::env::temp_dir().join(format!(
        "phoenix-census-synthetic-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir(&root).unwrap();
    let source = root.join("source");
    for (case, events) in [NOTE, &[0, 0xff, 0x01][..]].into_iter().enumerate() {
        let b = fixture(events, 1);
        fs::write(&source, &b).unwrap();
        let report = census_path(&source).unwrap();
        let s = &report.sequences[0];
        assert_eq!(report, census_path(&source).unwrap());
        for (mode, probe) in [
            (RecoveryInclusion::IncludeMuted, &s.general_include_muted),
            (RecoveryInclusion::ExcludeMuted, &s.general_exclude_muted),
        ] {
            let actual = build_recovery_sequence_manifest(&b, 0, 480, mode)
                .and_then(|m| assemble_recovery_sequence(&m));
            match actual {
                Ok(_) => assert_eq!(probe.outcome, Outcome::Passed),
                Err(e) => assert_eq!(*probe, manifest_probe(e)),
            }
        }
        let mut app = AppService::new();
        let i = app
            .inspect_project(InspectProjectRequest {
                contract_version: 1,
                source_path: source.to_str().unwrap().into(),
                diagnostics_level: DiagnosticsLevel::Full,
            })
            .unwrap();
        let request = ExportSequenceRequest {
            contract_version: 1,
            session_id: i.session_id,
            sequence_id: i.sequences[0].sequence_id.clone(),
            destination_folder: root.to_str().unwrap().into(),
            filename_stem: format!("out-{case}"),
            collision_policy: CollisionPolicy::FailIfExists,
            operation_id: None,
        };
        let prepared = app.prepare_export_sequence(&request);
        let actual = app.export_sequence(request);
        match actual {
            Ok(_) => {
                assert_eq!(s.application_default.outcome, Outcome::Passed);
                let p = prepared.unwrap();
                assert_eq!(
                    fs::read(root.join(format!("out-{case}.mid"))).unwrap(),
                    p.result.smf_bytes
                );
            }
            Err(e) => {
                assert_eq!(s.application_default.outcome, Outcome::Failed);
                assert_eq!(s.application_default.reason_code, e.diagnostic_code);
            }
        }
        assert_eq!(fs::read(&source).unwrap(), b);
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn saved_on_does_not_hide_decode_errors_and_policy_only_requires_complete_recovery() {
    let root = std::env::temp_dir().join(format!(
        "phoenix-census-policy-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir(&root).unwrap();
    let source = root.join("source");
    for (events, complete) in [(NOTE, true), (&[0, 0xff, 0x01][..], false)] {
        let mut b = fixture(events, 1);
        let p = parse_project_166(&b).unwrap();
        let offset = p.sequences[0].descriptors[2].range.start - 24;
        b[offset] = 0x88;
        fs::write(&source, &b).unwrap();
        let c = census_path(&source).unwrap();
        let s = &c.sequences[0];
        assert_eq!(s.proven_policy_only, complete);
        assert_eq!(
            s.general_include_muted.outcome,
            if complete {
                Outcome::Passed
            } else {
                Outcome::Failed
            }
        );
        assert_eq!(
            s.general_exclude_muted.outcome,
            if complete {
                Outcome::Passed
            } else {
                Outcome::Failed
            }
        );
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn unresolved_ownership_keeps_physical_pair_probes_without_fabricating_binding() {
    let mut b = fixture(NOTE, 1);
    let p = parse_project_166(&b).unwrap();
    let s = &p.sequences[0];
    let pair = &s.track_pairs[0];
    let extra = b[pair.primary.record_range.start..pair.secondary.record_range.end].to_vec();
    let pos = s.terminal_record.record_range.start;
    b.splice(pos..pos, extra);
    let report = census_bytes(&b);
    let s = &report.sequences[0];
    assert_eq!(row(s, Stage::Ownership).outcome, Outcome::Failed);
    let raw: Vec<_> = s
        .diagnostics
        .iter()
        .filter(|d| {
            d.descriptor_ordinal.is_none()
                && d.pair_ordinal.is_some()
                && d.stage == Stage::EventWalk
        })
        .collect();
    assert_eq!(raw.len(), 2);
    assert!(raw.iter().all(|d| d.outcome == Outcome::Passed));
    assert!(s
        .diagnostics
        .iter()
        .filter(|d| d.descriptor_ordinal.is_some())
        .all(|d| d.outcome == Outcome::NotEvaluated));
}

const CC: &[u8] = &[20, 255, 65, 5, 0, 1, 0, 7, 108];
fn patch_core(pre: [u8; 5], post: &[u8], program: u8, delta: u8, name: &[u8]) -> Vec<u8> {
    let mut payload = pre.to_vec();
    payload.push(name.len() as u8);
    payload.extend(name);
    payload.extend(post);
    payload.push(program);
    let mut e = vec![delta, 255, 124, payload.len() as u8];
    e.extend(payload);
    e
}
fn cc0(program: u8, delta: u8, name: &[u8]) -> Vec<u8> {
    let mut e = patch_core(
        [0, 0, program | 128, 8, program],
        &[3, b'X', b'Y', b'Z', 4, 255, 80, 255],
        program,
        delta,
        name,
    );
    e.extend(NOTE);
    e
}
fn program_only() -> Vec<u8> {
    let mut e = patch_core(
        [0, 0, 36, 0, 36],
        &[3, b'I', b'5', b'5', 4, 255, 255, 255],
        36,
        1,
        b"Unseen name!",
    );
    e.extend([1, 144, 70, 102, 90, 12]);
    e
}
fn numeric(opening: bool) -> Vec<u8> {
    let (pre, post, program) = if opening {
        ([0, 0, 1, 0, 1], vec![1, b'2', 4, 255, 255, 255], 1)
    } else {
        ([0, 0, 63, 8, 31], vec![2, b'3', b'2', 4, 255, 255, 255], 63)
    };
    let mut e = patch_core(pre, &post, program, 1, b"Unseen name");
    e.extend([1, 144, 60, 100, 64, 33]);
    e
}
fn with_prelude(mut b: Vec<u8>) -> Vec<u8> {
    let p = parse_project_166(&b).unwrap();
    let pos = p.sequences[0].meter_primary.record_range.start;
    let payload = [0, 1, 0, 1, 0, 1, 60, 15];
    let mut record = vec![9, 0, 0, 0, payload.len() as u8];
    record.extend(payload);
    b.splice(pos..pos, record);
    b
}
#[test]
fn mixed_family_controller_gate_failure_is_counted_once() {
    let mut e = CC.to_vec();
    e[8] = 255;
    // A source-owned context Note, then a numeric Patch and a context Controller.
    e.extend([0, 255, 96, 6, 35, 96, 0, 1, 1, 1, 0, 144, 60, 100, 64, 12]);
    e.extend(numeric(false));
    e.extend([
        0, 255, 96, 7, 35, 96, 0, 114, 108, 131, 76, 0, 255, 65, 5, 0, 1, 0, 1, 58,
    ]);
    let b = fixture(&e, 7);
    let report = census_bytes(&b);
    let s = &report.sequences[0];
    assert_eq!(row(s, Stage::EventWalk).outcome, Outcome::Passed);
    assert_eq!(row(s, Stage::Routing).outcome, Outcome::Passed);
    assert_eq!(row(s, Stage::SourceAccounting).outcome, Outcome::Passed);
    let cause = row(s, Stage::ControllerTranslation);
    assert_eq!(cause.outcome, Outcome::Failed);
    assert_eq!(cause.reason_code, "MidiData");
    for stage in [
        Stage::ContextNoteTranslation,
        Stage::ContextControllerTranslation,
        Stage::PatchTranslation,
    ] {
        let d = row(s, stage);
        assert_eq!(d.outcome, Outcome::NotEvaluated);
        assert_eq!(d.prerequisites, vec![cause.id]);
    }
    assert_eq!(report.metrics.blocker_instances.values().sum::<usize>(), 1);
}
#[test]
fn prelude_accounting_failure_is_a_prerequisite_not_a_second_blocker() {
    let report = census_bytes(&with_prelude(fixture(NOTE, 2)));
    let s = &report.sequences[0];
    let cause = row(s, Stage::SourceAccounting);
    assert_eq!(cause.outcome, Outcome::Failed);
    let prelude = row(s, Stage::Prelude);
    assert_eq!(prelude.outcome, Outcome::NotEvaluated);
    assert_eq!(prelude.prerequisites, vec![cause.id]);
    assert_eq!(report.metrics.blocker_instances.values().sum::<usize>(), 1);
    // A bad MIDI Controller does not itself prevent the prelude's Note inventory.
    let mut e = CC.to_vec();
    e[8] = 255;
    e.extend(NOTE);
    let report = census_bytes(&with_prelude(fixture(&e, 2)));
    assert_eq!(
        row(&report.sequences[0], Stage::Prelude).outcome,
        Outcome::Passed
    );
}
#[test]
fn contradictory_zero_count_keeps_safe_walk_route_and_one_accounting_blocker() {
    let report = census_bytes(&fixture(NOTE, 0));
    let s = &report.sequences[0];
    assert_eq!(row(s, Stage::EventWalk).outcome, Outcome::Passed);
    assert_eq!(row(s, Stage::Routing).outcome, Outcome::Passed);
    let cause = row(s, Stage::SourceAccounting);
    assert_eq!(cause.outcome, Outcome::Failed);
    assert_eq!(cause.reason_code, "declared_source_object_count_mismatch");
    let empty = row(s, Stage::EmptyTrack);
    assert_eq!(empty.outcome, Outcome::NotEvaluated);
    assert_eq!(empty.prerequisites, vec![cause.id]);
    assert_eq!(
        report.metrics.blocker_instances.get(&Family::Accounting),
        Some(&1)
    );
    assert_eq!(report.metrics.blocker_instances.values().sum::<usize>(), 1);
}
#[test]
fn patch_categories_have_literal_independent_expectations() {
    let standalone = patch_core(
        [0, 0, 144, 8, 16],
        &[3, b'X', b'Y', b'Z', 4, 255, 80, 255],
        16,
        0,
        b"Unseen name!",
    );
    for (events, count, reason) in [
        (cc0(16, 1, b"Unseen name!"), 2, "NonzeroPosition"),
        (cc0(16, 0, b"Other name!"), 2, "PayloadLength"),
        (standalone, 1, "unsupported_standalone_patch"),
        (cc0(42, 0, b"Unseen name!"), 2, "UnsupportedProgram"),
    ] {
        let b = fixture(&events, count);
        let report = census_bytes(&b);
        let s = &report.sequences[0];
        assert_eq!(
            row(s, Stage::EventWalk).outcome,
            Outcome::Passed,
            "{reason}"
        );
        assert_eq!(row(s, Stage::Routing).outcome, Outcome::Passed, "{reason}");
        assert_eq!(
            row(s, Stage::SourceAccounting).outcome,
            Outcome::Passed,
            "{reason}"
        );
        let patch = row(s, Stage::PatchTranslation);
        assert_eq!(patch.outcome, Outcome::Failed, "{reason}");
        assert_eq!(patch.reason_code, reason);
        assert!(patch.source_range.is_some());
        assert_eq!(s.general_include_muted.stage, "Patch");
        assert_eq!(s.general_include_muted.reason_code, reason);
        assert_eq!(
            report.metrics.blocker_instances.get(&Family::Translation),
            Some(&1)
        );
    }
}
#[test]
fn supported_patch_alternatives_are_not_false_blockers() {
    let mut program = CC.to_vec();
    program.extend(program_only());
    let bank = patch_core(
        [0, 0, 42, 8, 42],
        &[2, b'9', b'9', 4, 255, 80, 255],
        42,
        0,
        b"Fresh",
    );
    let mut bank_note = bank.clone();
    bank_note.extend(NOTE);
    let mut bank_standalone = bank;
    bank_standalone.extend(CC);
    bank_standalone.extend(NOTE);
    let mut terminal = NOTE.to_vec();
    terminal.extend(patch_core(
        [0, 0, 36, 0, 36],
        &[3, b'I', b'5', b'5', 4, 255, 255, 255],
        36,
        1,
        b"Unseen name!",
    ));
    for (events, count, label) in [
        (program, 3, "program-only"),
        (numeric(true), 2, "numeric"),
        (bank_note, 2, "bank-to-note"),
        (bank_standalone, 3, "standalone-bank"),
        (terminal, 2, "terminal-program"),
    ] {
        let report = census_bytes(&fixture(&events, count));
        let s = &report.sequences[0];
        assert_eq!(
            row(s, Stage::PatchTranslation).outcome,
            Outcome::Passed,
            "{label}: {:?}",
            s.diagnostics
        );
        assert_eq!(
            s.general_include_muted.outcome,
            Outcome::Passed,
            "{label}: {:?}",
            s.general_include_muted
        );
        assert_eq!(
            report.metrics.blocker_instances.get(&Family::Translation),
            None,
            "{label}"
        );
    }
}
#[test]
fn multi_patch_successful_program_only_mismatch_refuses_without_fallback() {
    let mut e = CC.to_vec();
    e.extend(program_only());
    e.extend(numeric(false));
    let b = fixture(&e, 5);
    assert!(crate::program_only166::qualify_track(&b, 0, 0).is_ok());
    let proof = crate::program_only166::qualify_track(&b, 0, 0).unwrap();
    let matches = proof.provenance().logical_source_ordinal == 3;
    assert!(!matches);
    assert!(!patch_fallback(
        Some(matches),
        || panic!("numeric fallback must not run"),
        || panic!("bank fallback must not run")
    ));
    let report = census_bytes(&b);
    let s = &report.sequences[0];
    let patches: Vec<_> = s
        .diagnostics
        .iter()
        .filter(|d| d.stage == Stage::PatchTranslation)
        .collect();
    assert_eq!(patches.len(), 2);
    assert_eq!(patches[0].outcome, Outcome::Passed);
    assert_eq!(patches[1].outcome, Outcome::Failed);
    assert_eq!(patches[1].reason_code, "NonzeroPosition");
    assert_eq!(s.general_include_muted.stage, "Patch");
    assert_eq!(s.general_include_muted.reason_code, "NonzeroPosition");
    assert_eq!(row(s, Stage::Translation).outcome, Outcome::NotEvaluated);
    assert_eq!(
        report.metrics.blocker_instances.get(&Family::Translation),
        Some(&1)
    );
}
