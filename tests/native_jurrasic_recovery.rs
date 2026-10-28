//! CCA-105: native equivalence covers four audible tracks only. The extra
//! saved-muted track is audited against source Notes, not claimed native-equal.
use phoenix::{
    app_contract::*, app_service::AppService, bounded_sequence::*, json_transport::dispatch_json,
};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};
#[path = "support/reference_smf.rs"]
mod reference;
const HASH: &str = "e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132";
const NATIVE_HASH: &str = "5c26d869e9d9ed136a619d4856c7b50a21772f94f7ebb94dc112a23c27a97558";
fn optional_bytes(env: &str, default: &str, hash: &str) -> Option<Vec<u8>> {
    let path = std::env::var(env).unwrap_or_else(|_| default.into());
    let b = match fs::read(&path) {
        Ok(b) => b,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            eprintln!("Optional authenticated fixture unavailable: {path}");
            return None;
        }
        Err(e) => panic!("{path}: {e}"),
    };
    assert_eq!(format!("{:x}", Sha256::digest(&b)), hash);
    Some(b)
}
fn source() -> Option<Vec<u8>> {
    optional_bytes("PHOENIX_CCA105_SOURCE","/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline",HASH)
}
fn native() -> Option<Vec<u8>> {
    optional_bytes("PHOENIX_CCA105_REFERENCE","/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/MIDI Exports from SVP/Jurrasic Park Multitrack",NATIVE_HASH)
}
fn music(t: &[(u32, u8, Vec<u8>)], normalize: bool) -> Vec<(u32, u8, Vec<u8>)> {
    t.iter()
        .filter(|(_, s, _)| *s >= 128)
        .map(|(tick, s, d)| {
            if normalize && s >> 4 == 9 && d[1] == 0 {
                (*tick, 0x80 | (*s & 15), vec![d[0], 64])
            } else {
                (*tick, *s, d.clone())
            }
        })
        .collect()
}
#[test]
fn authenticated_native_four_tracks_and_separate_muted_source_notes() {
    let (Some(b), Some(n)) = (source(), native()) else {
        return;
    };
    assert_eq!(b.len(), 211468);
    let n = reference::midi_tracks(&n);
    assert_eq!(n.len(), 5);
    assert_eq!(
        n[1..].iter().map(|t| music(t, false).len()).sum::<usize>(),
        701
    );
    assert_eq!(
        n[1..]
            .iter()
            .flat_map(|t| music(t, true))
            .filter(|(_, s, _)| s >> 4 == 9)
            .count(),
        241
    );
    for mode in [
        RecoveryInclusion::IncludeMuted,
        RecoveryInclusion::ExcludeMuted,
    ] {
        let m = build_recovery_sequence_manifest(&b, 13, 480, mode).unwrap();
        let out = assemble_recovery_sequence(&m).unwrap();
        let g = reference::midi_tracks(&out.smf_bytes);
        assert_eq!(m.source_track_count(), 7);
        assert_eq!(m.empty_tracks().len(), 2);
        let offset = usize::from(mode == RecoveryInclusion::IncludeMuted);
        assert_eq!(g.len(), 5 + offset);
        let metadata = |t: &Vec<(u32, u8, Vec<u8>)>| {
            t.iter()
                .filter(|(_, s, _)| matches!(s, 3 | 81 | 88))
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(metadata(&g[0]), metadata(&n[0]));
        assert_eq!(
            n[0].iter().find(|(_, s, _)| *s == 81).unwrap().2,
            vec![7, 161, 32]
        );
        assert_eq!(
            n[0].iter().find(|(_, s, _)| *s == 88).unwrap().2,
            vec![4, 2, 24, 8]
        );
        for j in 1..=4 {
            assert_eq!(music(&g[j + offset], false), music(&n[j], true));
            assert_eq!(
                g[j + offset].iter().find(|(_, s, _)| *s == 3),
                n[j].iter().find(|(_, s, _)| *s == 3)
            );
            // Native Instrument Name metadata and padded EOT are not musical
            // channel events; Phoenix retains minimal EOT and Track Names.
            assert!(n[j].iter().any(|(_, s, _)| *s == 4));
            assert!(!g[j + offset].iter().any(|(_, s, _)| *s == 4));
            assert_eq!(n[j].last().unwrap(), &(42240, 47, vec![]));
            assert_eq!(g[j + offset].last().unwrap().1, 47);
            assert!(g[j + offset].last().unwrap().0 < 42240);
        }
        // Native SMPTE Offset is outside this conductor's musical contract.
        assert!(n[0].iter().any(|(_, s, _)| *s == 84));
        assert!(!g[0].iter().any(|(_, s, _)| *s == 84));
        if offset == 1 {
            use phoenix::mixed_event::*;
            let project = phoenix::sequence_container::parse_project_166(&b).unwrap();
            let bounds = project.sequences[13]
                .validated_track_event_bounds(0)
                .unwrap()
                .event_range;
            let walk = walk_bounded_mixed_events(
                &b,
                MixedEventBounds {
                    event_range: bounds,
                },
                Default::default(),
            )
            .unwrap();
            assert_eq!(walk.items.len(), 47);
            let mut expected = Vec::new();
            for (i, item) in walk.items.iter().enumerate() {
                let MixedEventItem::Event(e) = item else {
                    panic!("not a Note")
                };
                let MixedEventKind::Note(note) = &e.event else {
                    panic!("not a Note")
                };
                expected.push((
                    e.position,
                    2 * i,
                    0x99,
                    vec![note.pitch.value, note.attack_velocity.value],
                ));
                expected.push((
                    e.position + note.duration.value,
                    2 * i + 1,
                    0x89,
                    vec![note.pitch.value, note.release_velocity.value],
                ));
            }
            expected.sort_by_key(|e| (e.0, e.1));
            assert_eq!(
                music(&g[1], false),
                expected
                    .into_iter()
                    .map(|(t, _, s, d)| (t, s, d))
                    .collect::<Vec<_>>()
            );
            assert_eq!(m.disposition(0), Some(RecoveryTrackDisposition::Included));
        } else {
            assert_eq!(
                m.disposition(0),
                Some(RecoveryTrackDisposition::ExcludedSavedMute)
            );
        }
    }
}
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Fixture {
    root: PathBuf,
    source: PathBuf,
    bytes: Vec<u8>,
}
impl Fixture {
    fn new() -> Option<Self> {
        let bytes = source()?;
        let root = std::env::temp_dir().join(format!(
            "phoenix-cca105-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        let source = root.join("renamed-copy.svp");
        fs::write(&source, &bytes).unwrap();
        Some(Self {
            root,
            source,
            bytes,
        })
    }
    fn inspect(&self, app: &mut AppService) -> InspectProjectResponse {
        inspect(app, &self.source)
    }
    fn request(&self, i: &InspectProjectResponse, ordinal: usize) -> ExportSequenceRequest {
        ExportSequenceRequest {
            contract_version: 1,
            session_id: i.session_id.clone(),
            sequence_id: i.sequences[ordinal].sequence_id.clone(),
            destination_folder: self.root.to_str().unwrap().into(),
            filename_stem: "Jurrasic".into(),
            collision_policy: CollisionPolicy::FailIfExists,
            operation_id: None,
        }
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.root).unwrap();
    }
}
fn inspect(app: &mut AppService, path: &Path) -> InspectProjectResponse {
    app.inspect_project(InspectProjectRequest {
        contract_version: 1,
        source_path: path.to_str().unwrap().into(),
        diagnostics_level: DiagnosticsLevel::Summary,
    })
    .unwrap()
}
#[test]
fn renamed_source_authorizes_general_recovery_and_publishes_both_modes() {
    let Some(f) = Fixture::new() else { return };
    let mut app = AppService::new();
    let i = f.inspect(&mut app);
    assert_eq!(
        i.sequences
            .iter()
            .filter(|s| s.readiness == Readiness::Ready)
            .count(),
        11
    );
    assert_eq!(
        i.sequences[13]
            .bounded_export_capability
            .as_ref()
            .unwrap()
            .contract_id,
        "generalized_sequence_recovery"
    );
    assert_eq!(
        i.sequences[0]
            .bounded_export_capability
            .as_ref()
            .unwrap()
            .contract_id,
        "generalized_sequence_recovery"
    );
    for (index, mode) in [
        None,
        Some(RecoveryInclusionOption::IncludeMuted),
        Some(RecoveryInclusionOption::ExcludeMuted),
    ]
    .into_iter()
    .enumerate()
    {
        let mut r = f.request(&i, 13);
        r.filename_stem = format!("Jurrasic-{index}");
        let receipt = match mode {
            None => app.export_sequence(r).unwrap(),
            Some(mode) => app
                .export_sequence_with_recovery_inclusion(r, mode)
                .unwrap(),
        };
        let bytes = fs::read(&receipt.output_path).unwrap();
        let count = if mode == Some(RecoveryInclusionOption::ExcludeMuted) {
            5
        } else {
            6
        };
        assert_eq!(reference::midi_tracks(&bytes).len(), count);
        assert_eq!(receipt.total_smf_track_count, count as u32);
        let expected = assemble_recovery_sequence(
            &build_recovery_sequence_manifest(
                &f.bytes,
                13,
                480,
                if count == 5 {
                    RecoveryInclusion::ExcludeMuted
                } else {
                    RecoveryInclusion::IncludeMuted
                },
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(bytes, expected.smf_bytes);
    }
    let mut collision = f.request(&i, 13);
    collision.filename_stem = "Jurrasic-0".into();
    let existing = fs::read(f.root.join("Jurrasic-0.mid")).unwrap();
    assert_eq!(
        app.export_sequence(collision).unwrap_err().diagnostic_code,
        "destination_exists"
    );
    assert_eq!(fs::read(f.root.join("Jurrasic-0.mid")).unwrap(), existing);
    assert_eq!(fs::read(&f.source).unwrap(), f.bytes);
}
#[test]
fn revalidation_selection_json_and_legacy_preference_guards() {
    let Some(f) = Fixture::new() else { return };
    let mut app = AppService::new();
    let i = f.inspect(&mut app);
    let r = f.request(&i, 13);
    let j = f.inspect(&mut app);
    let mut cross = r.clone();
    cross.session_id = j.session_id;
    assert_eq!(
        app.export_sequence(cross).unwrap_err().diagnostic_code,
        "unknown_sequence"
    );
    let mut recovered = f.request(&i, 2);
    recovered.filename_stem = "Recovered-sibling".into();
    assert!(app.export_sequence(recovered).is_ok());
    assert_eq!(
        app.export_sequence_with_recovery_inclusion(
            f.request(&i, 14),
            RecoveryInclusionOption::IncludeMuted
        )
        .unwrap_err()
        .diagnostic_code,
        "unsupported_recovery_inclusion"
    );
    let mut v = serde_json::json!({"operation":"export_sequence","contract_version":1,"payload":{"session_id":i.session_id.as_str(),"sequence_id":i.sequences[13].sequence_id.as_str(),"destination_folder":f.root,"filename_stem":"JSON","collision_policy":"fail_if_exists","operation_id":null}});
    for bad in [
        serde_json::json!(null),
        serde_json::json!(true),
        serde_json::json!("unknown"),
        serde_json::json!({}),
    ] {
        v["payload"]["recovery_inclusion"] = bad;
        let out: serde_json::Value =
            serde_json::from_slice(&dispatch_json(&mut app, &serde_json::to_vec(&v).unwrap()))
                .unwrap();
        assert_eq!(out["ok"], false);
    }
    for mode in [None, Some("include_muted"), Some("exclude_muted")] {
        v["payload"]
            .as_object_mut()
            .unwrap()
            .remove("recovery_inclusion");
        if let Some(mode) = mode {
            v["payload"]["recovery_inclusion"] = serde_json::json!(mode);
        }
        v["payload"]["filename_stem"] = serde_json::json!(format!("json-{mode:?}"));
        let out: serde_json::Value =
            serde_json::from_slice(&dispatch_json(&mut app, &serde_json::to_vec(&v).unwrap()))
                .unwrap();
        assert_eq!(out["ok"], true, "{out}");
        assert_eq!(
            out["result"]["total_smf_track_count"],
            if mode == Some("exclude_muted") { 5 } else { 6 }
        );
    }
    v["payload"]["capability"] = serde_json::json!("generalized_sequence_recovery");
    let out: serde_json::Value =
        serde_json::from_slice(&dispatch_json(&mut app, &serde_json::to_vec(&v).unwrap())).unwrap();
    assert_eq!(out["ok"], false);
    let mut altered = f.bytes.clone();
    altered[0] ^= 1;
    fs::write(&f.source, &altered).unwrap();
    assert_eq!(
        app.export_sequence(r.clone()).unwrap_err().diagnostic_code,
        "source_identity_changed"
    );
    let changed = f.inspect(&mut app);
    assert!(changed.sequences[13].bounded_export_capability.is_some());
    fs::remove_file(&f.source).unwrap();
    assert_eq!(
        app.export_sequence(r).unwrap_err().diagnostic_code,
        "source_revalidation_failed"
    );
    assert!(!f.root.join("Jurrasic.mid").exists());
}

#[test]
fn renamed_sequence_qualifies_but_incomplete_accounting_never_authorizes() {
    let Some(f) = Fixture::new() else { return };
    let project = phoenix::sequence_container::parse_project_166(&f.bytes).unwrap();
    let s = &project.sequences[13];
    let name = s.sequence_name.bytes.range.start;
    let count = s.track_pairs[0].primary.payload.range.start + 5;
    for offset in [name, count] {
        let mut altered = f.bytes.clone();
        altered[offset] ^= 1;
        fs::write(&f.source, &altered).unwrap();
        let mut app = AppService::new();
        let i = f.inspect(&mut app);
        if offset == name {
            assert!(i.sequences[13].bounded_export_capability.is_some());
        } else {
            assert!(i.sequences[13].bounded_export_capability.is_none());
            assert!(app.export_sequence(f.request(&i, 13)).is_err());
        }
        if offset == count {
            assert!(build_recovery_sequence_manifest(
                &altered,
                13,
                480,
                RecoveryInclusion::IncludeMuted
            )
            .is_err());
        }
    }
    assert!(!f.root.join("Jurrasic.mid").exists());
}
