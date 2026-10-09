use phoenix::app_contract::*;
use phoenix::app_service::AppService;
use phoenix::experimental166::TrackIdentity166;
use std::path::{Path, PathBuf};
const PROJECT:&str="/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline";
const NOTE: &[u8] = &[0, 0x90, 60, 127, 113, 102];
fn record(b: &mut Vec<u8>, tag: u8, p: &[u8]) {
    b.push(tag);
    b.extend((p.len() as u32).to_be_bytes());
    b.extend(p);
}
fn synthetic(events: &[u8]) -> Vec<u8> {
    let mut b = vec![0; 8];
    for i in 0..2 {
        let mut p = vec![0; 36];
        p[25] = i;
        p[26] = 7;
        p[27] = 3;
        record(&mut b, 0x10, &p);
    }
    let mut device = vec![0; 40];
    device[0] = 4;
    device[1..5].copy_from_slice(b"Test");
    device[33] = 7;
    record(&mut b, 0x2a, &device);
    let start = b.len();
    let mut p = vec![0; 3 * 166 + 172];
    p[0] = 3;
    record(&mut b, 1, &p);
    let label = start + 208 + 2 * 166 + 15;
    b[label..label + 5].copy_from_slice(b"Other");
    b[label - 39..label - 31].copy_from_slice(&[128, 0, 4, 0, 0, 4, 1, 0]);
    b[label - 33] = 1;
    b[label - 31] = 1;
    b[label - 30..label].fill(0xff);
    let mut name = vec![0; 11];
    name.extend([4, b'N', b'e', b'w', b'!']);
    record(&mut b, 7, &name);
    conductor_pair(&mut b, &[0, 255, 88, 4, 4, 2, 8, 8]);
    conductor_pair(&mut b, &[0, 255, 81, 3, 7, 53, 120]);
    let mut p = vec![0; 14];
    p[2..6].copy_from_slice(&1u32.to_be_bytes());
    p.extend(events);
    p.extend([0xff, 0, 0, 0, 0xff, 0x2f, 0]);
    record(&mut b, 2, &p);
    record(&mut b, 0x29, &[]);
    record(&mut b, 0, &[255; 8]);
    b
}
fn conductor_pair(b: &mut Vec<u8>, event: &[u8]) {
    let mut primary = vec![0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 12, 34, 56, 78];
    primary.extend(event);
    primary.extend([135, 255, 255, 127, 255, 47, 0]);
    primary.resize(38, 0);
    record(b, 2, &primary);
    let n = event.len() as u8;
    let mut secondary = vec![
        0,
        1,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        n + 19,
        0,
        0,
        0,
        n + 19,
        0,
        0,
        0,
        1,
        0,
        0,
        0,
        n + 4,
        0,
        255,
        255,
        255,
        47,
        197,
        0,
        0,
        0,
        0,
    ];
    secondary.push(event[2]);
    secondary.extend(&event[4..]);
    secondary.extend([255; 3]);
    record(b, 41, &secondary);
}

fn folder() -> PathBuf {
    let p = Path::new("/tmp").join(format!(
        "phoenix-cca031-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir(&p).unwrap();
    p
}
fn inspect(s: &mut AppService, p: &Path) -> InspectProjectResponse {
    s.inspect_project(InspectProjectRequest {
        contract_version: CONTRACT_VERSION,
        source_path: p.to_string_lossy().into(),
        diagnostics_level: DiagnosticsLevel::Full,
    })
    .unwrap()
}
fn req(r: &InspectProjectResponse, id: SequenceId, p: &Path) -> ExportSequenceRequest {
    ExportSequenceRequest {
        contract_version: CONTRACT_VERSION,
        session_id: r.session_id.clone(),
        sequence_id: id,
        destination_folder: p.to_string_lossy().into(),
        filename_stem: "Selected".into(),
        collision_policy: CollisionPolicy::FailIfExists,
        operation_id: None,
    }
}
fn selected(s: &AppService, r: &InspectProjectResponse) -> Vec<TrackIdentity166> {
    s.experimental_tracks166(&r.session_id, &r.sequences[0].sequence_id)
        .unwrap()
}
#[test]
fn portable_explicit_identity_note_values_and_independent_verified_refusal() {
    let dir = folder();
    let path = dir.join("source");
    let mut bytes = synthetic(NOTE);
    // Unresolved whole-sequence terminal remains a normal refusal.
    let n = bytes.len();
    bytes[n - 1] = 254;
    std::fs::write(&path, &bytes).unwrap();
    let mut s = AppService::new();
    let r = inspect(&mut s, &path);
    let ids = selected(&s, &r);
    let plan = s
        .preview_experimental_selected166(&r.session_id, &r.sequences[0].sequence_id, &ids)
        .unwrap();
    assert_eq!(plan.report().included_notes, 1);
    assert_eq!(plan.report().tracks[0].channel, Some(4));
    assert!(plan
        .report()
        .limitations
        .iter()
        .any(|x| x.contains("UNRESOLVED")));
    let q = req(&r, r.sequences[0].sequence_id.clone(), &dir);
    assert!(s.export_sequence(q.clone()).is_err());
    assert!(s
        .export_experimental_sequence(q.clone(), &plan, false)
        .is_err());
    let out = s
        .export_experimental_sequence(q.clone(), &plan, true)
        .unwrap();
    let midi = std::fs::read(out.output_path).unwrap();
    assert!(midi
        .windows(7)
        .any(|w| w == [0x93, 60, 127, 102, 0x83, 60, 113]));
    assert!(midi.windows(15).any(|w| w == b"SELECTED TRACKS"));
    assert!(s
        .export_experimental_sequence(q.clone(), &plan, true)
        .is_err());
    assert!(s.export_sequence(q).is_err());
    assert_eq!(
        r.sequences[0].readiness,
        inspect(&mut s, &path).sequences[0].readiness
    );
}
#[test]
fn selection_scope_and_source_freshness_cannot_be_forged() {
    let dir = folder();
    let path = dir.join("source");
    std::fs::write(&path, synthetic(NOTE)).unwrap();
    let mut s = AppService::new();
    let r = inspect(&mut s, &path);
    let ids = selected(&s, &r);
    let sid = &r.sequences[0].sequence_id;
    assert!(s
        .preview_experimental_selected166(&r.session_id, sid, &[])
        .is_err());
    assert!(s
        .preview_experimental_selected166(&r.session_id, sid, &[ids[0].clone(), ids[0].clone()])
        .is_err());
    for field in 0..3 {
        let mut bad = ids.clone();
        match field {
            0 => bad[0].descriptor_ordinal = 99,
            1 => bad[0].sequence_range.end += 1,
            _ => bad[0].source_sha256 = "wrong".into(),
        };
        assert!(s
            .preview_experimental_selected166(&r.session_id, sid, &bad)
            .is_err());
    }
    let plan = s
        .preview_experimental_selected166(&r.session_id, sid, &ids)
        .unwrap();
    let mut b = std::fs::read(&path).unwrap();
    b[0] ^= 1;
    std::fs::write(&path, b).unwrap();
    assert!(s
        .export_experimental_sequence(req(&r, sid.clone(), &dir), &plan, true)
        .is_err());
    assert_eq!(std::fs::read_dir(dir).unwrap().count(), 1);
}
#[test]
fn malformed_event_unsupported_family_channel_conductor_and_ownership_refuse() {
    let good = synthetic(NOTE);
    let parsed = phoenix::sequence_container::parse_project_166(&good).unwrap();
    let seq = &parsed.sequences[0];
    let mut variants = Vec::new();
    for n in 0..NOTE.len() {
        variants.push(synthetic(&NOTE[..n]));
    }
    variants.push(synthetic(&[0, 0xd0, 12]));
    let mut b = good.clone();
    b[seq.track_pairs[0].primary.payload.range.start + 5] = 2;
    variants.push(b);
    let mut b = good.clone();
    b[seq.initial_tempo_range.start] = 1;
    variants.push(b);
    let mut b = good.clone();
    b[seq.tempo_secondary.payload.range.end - 4] ^= 1;
    variants.push(b);
    let mut b = good.clone();
    for table in parsed.records.iter().filter(|x| x.record_type.value == 16) {
        b[table.payload.range.start + 27] = 255;
    }
    variants.push(b);
    let mut b = good.clone();
    b[seq.descriptors[2].label.as_ref().unwrap().range.start - 33] = 2;
    variants.push(b);
    let mut b = good.clone();
    b[parsed.records[seq.candidate_record_index]
        .payload
        .range
        .start] = 4;
    variants.push(b);
    let dir = folder();
    for (i, b) in variants.iter().enumerate() {
        let path = dir.join(format!("v{i}"));
        std::fs::write(&path, b).unwrap();
        let mut s = AppService::new();
        if let Ok(r) = s.inspect_project(InspectProjectRequest {
            contract_version: CONTRACT_VERSION,
            source_path: path.to_string_lossy().into(),
            diagnostics_level: DiagnosticsLevel::Full,
        }) {
            if let Some(row) = r.sequences.first() {
                if let Ok(ids) = s.experimental_tracks166(&r.session_id, &row.sequence_id) {
                    assert!(
                        s.preview_experimental_selected166(&r.session_id, &row.sequence_id, &ids)
                            .is_err(),
                        "variant {i}"
                    );
                }
            }
        }
    }
}
#[test]
fn authentic_selected_subsets_have_explicit_exclusions_and_actual_service_outputs() {
    let mut s = AppService::new();
    let r = inspect(&mut s, Path::new(PROJECT));
    for (name, ordinals, expected) in [
        ("Situation", vec![4, 5, 6, 7], 367),
        ("Sequence D", vec![4, 5], 176),
        ("happyone", vec![2, 3, 4], 893),
    ] {
        let row = r
            .sequences
            .iter()
            .find(|row| row.display_name == name)
            .unwrap();
        let all = s
            .experimental_tracks166(&r.session_id, &row.sequence_id)
            .unwrap();
        let ids: Vec<_> = all
            .iter()
            .filter(|id| ordinals.contains(&id.descriptor_ordinal))
            .cloned()
            .collect();
        let mut requested = ids.clone();
        requested.reverse();
        let plan = s
            .preview_experimental_selected166(&r.session_id, &row.sequence_id, &requested)
            .unwrap();
        assert_eq!(plan.report().included_notes, expected);
        let scope = plan.report().selected_scope166.as_ref().unwrap();
        assert_eq!(scope.selected.len() + scope.excluded.len(), all.len());
        assert_eq!(scope.selected, ids);
        assert!(scope.excluded.iter().all(|t| !t.reason.is_empty()));
        let dir = folder();
        let q = req(&r, row.sequence_id.clone(), &dir);
        assert!(s.export_sequence(q.clone()).is_err());
        let out = s
            .export_experimental_sequence(q.clone(), &plan, true)
            .unwrap();
        println!(
            "CCA031_OUTPUT {} {} {}",
            name, out.output_path, out.limitations_path
        );
        assert!(s.export_sequence(q).is_err());
    }
    let row = r
        .sequences
        .iter()
        .find(|r| r.display_name == "Sequence I")
        .unwrap();
    assert!(s
        .experimental_tracks166(&r.session_id, &row.sequence_id)
        .is_err());
    let row = r
        .sequences
        .iter()
        .find(|r| r.display_name == "Situation")
        .unwrap();
    let all = s
        .experimental_tracks166(&r.session_id, &row.sequence_id)
        .unwrap();
    assert!(s
        .preview_experimental_selected166(&r.session_id, &row.sequence_id, &all[..1])
        .is_err());
    let after = inspect(&mut s, Path::new(PROJECT));
    assert_eq!(
        r.sequences.iter().map(|r| r.readiness).collect::<Vec<_>>(),
        after
            .sequences
            .iter()
            .map(|r| r.readiness)
            .collect::<Vec<_>>()
    );
}

#[test]
fn non_equivalent_same_tick_note_order_refuses_without_dropping_a_note() {
    let mut second = NOTE.to_vec();
    second[3] = 100;
    let mut bytes = synthetic(&[NOTE, second.as_slice()].concat());
    let parsed = phoenix::sequence_container::parse_project_166(&bytes).unwrap();
    let count_offset = parsed.sequences[0].track_pairs[0]
        .primary
        .payload
        .range
        .start
        + 2;
    bytes[count_offset..count_offset + 4].copy_from_slice(&2u32.to_be_bytes());
    let dir = folder();
    let path = dir.join("source");
    std::fs::write(&path, bytes).unwrap();
    let mut s = AppService::new();
    let r = inspect(&mut s, &path);
    let ids = selected(&s, &r);
    let error = s
        .preview_experimental_selected166(&r.session_id, &r.sequences[0].sequence_id, &ids)
        .unwrap_err();
    assert!(error.0.contains("AmbiguousSourceOrder"));
    assert_eq!(std::fs::read_dir(dir).unwrap().count(), 1);
}

#[test]
fn all_three_existing_120_experimental_midi_bytes_remain_identical() {
    use sha2::{Digest, Sha256};
    let path = Path::new(
        "/Users/kurtheiden/Documents/Phoenix Research/Authentic Studio Vision Projects/COMIC BOOK",
    );
    let mut s = AppService::new();
    let r = inspect(&mut s, path);
    for (name, hash) in [
        (
            "pick up the pace",
            "ddae904317712a6ba43697f258642aeabb30c6e7cdf72e3144a8dd2cf5d18e89",
        ),
        (
            "nitemare b4 xmas",
            "af7caa08710c70a3fa41c975ead8e743b10b3c730e53213cdf8e501b49f7ef7d",
        ),
        (
            "sampler xperimnt",
            "cb1aa26a045f16e67daec97996d033054621dd627810b12789db1b0c271f83b6",
        ),
    ] {
        let row = r.sequences.iter().find(|r| r.display_name == name).unwrap();
        let plan = s
            .preview_experimental_sequence(&r.session_id, &row.sequence_id)
            .unwrap();
        assert!(plan.report().selected_scope166.is_none());
        let out = s
            .export_experimental_sequence(req(&r, row.sequence_id.clone(), &folder()), &plan, true)
            .unwrap();
        assert_eq!(
            format!(
                "{:x}",
                Sha256::digest(std::fs::read(out.output_path).unwrap())
            ),
            hash
        );
    }
}
