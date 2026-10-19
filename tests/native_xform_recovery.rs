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
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Fixture {
    root: PathBuf,
    source: PathBuf,
    bytes: Vec<u8>,
}
impl Fixture {
    fn new(name: &str) -> Self {
        let root = std::env::temp_dir().join(format!(
            "phoenix-cca101-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        let bytes = fs::read(format!(
            "/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/{name}"
        ))
        .unwrap();
        let source = root.join("renamed-copy.svp");
        fs::write(&source, &bytes).unwrap();
        Self {
            root,
            source,
            bytes,
        }
    }
    fn inspect(&self, app: &mut AppService) -> InspectProjectResponse {
        inspect(app, &self.source)
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.root).unwrap();
    }
}
fn inspect(app: &mut AppService, p: &Path) -> InspectProjectResponse {
    app.inspect_project(InspectProjectRequest {
        contract_version: 1,
        source_path: p.to_str().unwrap().into(),
        diagnostics_level: DiagnosticsLevel::Summary,
    })
    .unwrap()
}
fn request(f: &Fixture, i: &InspectProjectResponse, name: &str) -> ExportSequenceRequest {
    ExportSequenceRequest {
        contract_version: 1,
        session_id: i.session_id.clone(),
        sequence_id: i
            .sequences
            .iter()
            .find(|s| s.display_name == name)
            .unwrap()
            .sequence_id
            .clone(),
        destination_folder: f.root.to_str().unwrap().into(),
        filename_stem: "recovered".into(),
        collision_policy: CollisionPolicy::FailIfExists,
        operation_id: None,
    }
}
fn music(t: &[(u32, u8, Vec<u8>)], normalize: bool) -> Vec<(u32, u8, Vec<u8>)> {
    t.iter()
        .filter(|(_, s, _)| *s >= 128)
        .map(|(t, s, d)| {
            if normalize && *s >> 4 == 9 && d[1] == 0 {
                (*t, 0x80 | (*s & 15), vec![d[0], 64])
            } else {
                (*t, *s, d.clone())
            }
        })
        .collect()
}
#[test]
fn published_both_modes_default_renamed_copy_and_native_readback() {
    let f = Fixture::new("CCA073 CONTROL");
    assert_eq!(f.bytes.len(), 211468);
    assert_eq!(
        format!("{:x}", Sha256::digest(&f.bytes)),
        "e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132"
    );
    let n = fs::read("/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/073 CTRL xForm.mid")
        .unwrap();
    assert_eq!(
        format!("{:x}", Sha256::digest(&n)),
        "872272ce43c389474fdb408416ad2406fa9a63de5e1d8bb94e0f6bbced7a1241"
    );
    let native = reference::midi_tracks(&n);
    let mut app = AppService::new();
    let i = f.inspect(&mut app);
    let x = &i.sequences[0];
    assert_eq!(x.readiness, Readiness::Ready);
    assert!(x.export_capability.is_none());
    assert_eq!(
        x.bounded_export_capability.as_ref().unwrap().contract_id,
        "authenticated_xform_recovery"
    );
    for (index, mode) in [
        None,
        Some(RecoveryInclusionOption::IncludeMuted),
        Some(RecoveryInclusionOption::ExcludeMuted),
    ]
    .into_iter()
    .enumerate()
    {
        let mut r = request(&f, &i, "xForm");
        r.filename_stem = format!("recovered-{index}");
        let receipt = match mode {
            None => app.export_sequence(r).unwrap(),
            Some(m) => app.export_sequence_with_recovery_inclusion(r, m).unwrap(),
        };
        let bytes = fs::read(&receipt.output_path).unwrap();
        let tracks = reference::midi_tracks(&bytes);
        let musical = if mode == Some(RecoveryInclusionOption::ExcludeMuted) {
            12
        } else {
            13
        };
        assert_eq!(tracks.len(), musical + 1);
        assert_eq!(receipt.musical_track_count, musical as u32);
        assert_eq!(receipt.total_smf_track_count, (musical + 1) as u32);
        let engine_mode = if musical == 12 {
            RecoveryInclusion::ExcludeMuted
        } else {
            RecoveryInclusion::IncludeMuted
        };
        let expected = assemble_recovery_sequence(
            &build_recovery_sequence_manifest(&f.bytes, 0, 480, engine_mode).unwrap(),
        )
        .unwrap();
        assert_eq!(bytes, expected.smf_bytes);
        for j in 1..=12 {
            assert_eq!(music(&tracks[j], false), music(&native[j], true));
            assert_eq!(
                tracks[j].iter().find(|(_, s, _)| *s == 3),
                native[j].iter().find(|(_, s, _)| *s == 3)
            );
        }
        let conductor = |t: &Vec<(u32, u8, Vec<u8>)>| {
            t.iter()
                .filter(|(_, s, _)| matches!(s, 3 | 81 | 88))
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(conductor(&tracks[0]), conductor(&native[0]));
        if musical == 13 {
            assert_eq!(music(&tracks[13], false).len(), 142);
        }
        assert!(!fs::read_dir(&f.root).unwrap().any(|e| e
            .unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with(".phoenix-export-")));
    }
    assert_eq!(fs::read(&f.source).unwrap(), f.bytes);
    // Publication collision never overwrites the source or an existing destination.
    let mut r = request(&f, &i, "xForm");
    r.filename_stem = "recovered-0".into();
    let existing = fs::read(f.root.join("recovered-0.mid")).unwrap();
    assert_eq!(
        app.export_sequence(r.clone()).unwrap_err().diagnostic_code,
        "destination_exists"
    );
    assert_eq!(existing, fs::read(f.root.join("recovered-0.mid")).unwrap());
    r.collision_policy = CollisionPolicy::GenerateUniqueName;
    assert!(app
        .export_sequence_with_recovery_inclusion(r, RecoveryInclusionOption::ExcludeMuted)
        .unwrap()
        .output_path
        .ends_with("recovered-0 2.mid"));
    let mut invalid = request(&f, &i, "xForm");
    invalid.filename_stem = "../escape".into();
    assert_eq!(
        app.export_sequence(invalid).unwrap_err().diagnostic_code,
        "invalid_filename_stem"
    );
    let mut invalid = request(&f, &i, "xForm");
    invalid.destination_folder = f.source.to_str().unwrap().into();
    assert_eq!(
        app.export_sequence(invalid).unwrap_err().diagnostic_code,
        "invalid_destination_folder"
    );
}
#[test]
fn changed_deleted_cross_session_edit_and_unsupported_preferences_refuse() {
    let f = Fixture::new("CCA073 CONTROL");
    let mut app = AppService::new();
    let i = f.inspect(&mut app);
    let r = request(&f, &i, "xForm");
    let j = f.inspect(&mut app);
    let mut crossed = r.clone();
    crossed.session_id = j.session_id;
    assert_eq!(
        app.export_sequence(crossed).unwrap_err().diagnostic_code,
        "unknown_sequence"
    );
    for legacy in ["Sequence R", "Bells for her", "Ode to Clarke"] {
        let sibling = request(&f, &i, legacy);
        assert_eq!(
            app.export_sequence_with_recovery_inclusion(
                sibling,
                RecoveryInclusionOption::IncludeMuted
            )
            .unwrap_err()
            .diagnostic_code,
            "unsupported_recovery_inclusion"
        );
    }
    let blocked = request(&f, &i, "Situation");
    assert!(app.export_sequence(blocked).is_err());
    let mut altered = f.bytes.clone();
    altered[0] ^= 1;
    fs::write(&f.source, &altered).unwrap();
    assert_eq!(
        app.export_sequence(r.clone()).unwrap_err().diagnostic_code,
        "source_identity_changed"
    );
    let mutated = f.inspect(&mut app);
    assert!(mutated.sequences[0]
        .bounded_export_capability
        .as_ref()
        .map_or(true, |c| c.contract_id != "authenticated_xform_recovery"));
    fs::remove_file(&f.source).unwrap();
    assert_eq!(
        app.export_sequence(r).unwrap_err().diagnostic_code,
        "source_revalidation_failed"
    );
    assert!(!f.root.join("recovered.mid").exists());
    let edit = Fixture::new("CCA073 EDIT");
    let i = edit.inspect(&mut app);
    assert!(i.sequences[0].bounded_export_capability.is_none());
    assert_eq!(
        app.export_sequence(request(&edit, &i, "xForm"))
            .unwrap_err()
            .diagnostic_code,
        "sequence_not_export_capable"
    );
}
#[test]
fn strict_json_preferences_and_omission_defaults() {
    let f = Fixture::new("CCA073 CONTROL");
    let mut app = AppService::new();
    let i = f.inspect(&mut app);
    let mut v = serde_json::json!({"operation":"export_sequence","contract_version":1,"payload":{"session_id":i.session_id.as_str(),"sequence_id":i.sequences[0].sequence_id.as_str(),"destination_folder":f.root,"filename_stem":"json","collision_policy":"generate_unique_name","operation_id":null}});
    for bad in [
        serde_json::json!(null),
        serde_json::json!(true),
        serde_json::json!("unknown"),
        serde_json::json!({}),
        serde_json::json!(0),
    ] {
        v["payload"]["recovery_inclusion"] = bad;
        let response: serde_json::Value =
            serde_json::from_slice(&dispatch_json(&mut app, &serde_json::to_vec(&v).unwrap()))
                .unwrap();
        assert_eq!(response["ok"], false);
        assert!(!f.root.join("json.mid").exists());
    }
    for mode in [None, Some("include_muted"), Some("exclude_muted")] {
        v["payload"]
            .as_object_mut()
            .unwrap()
            .remove("recovery_inclusion");
        if let Some(m) = mode {
            v["payload"]["recovery_inclusion"] = serde_json::json!(m);
        }
        let response: serde_json::Value =
            serde_json::from_slice(&dispatch_json(&mut app, &serde_json::to_vec(&v).unwrap()))
                .unwrap();
        assert_eq!(response["ok"], true, "{response}");
        assert_eq!(
            response["result"]["total_smf_track_count"],
            if mode == Some("exclude_muted") {
                13
            } else {
                14
            }
        );
    }
    // Capability strings and arbitrary additional fields are not client authority.
    v["payload"]["capability"] = serde_json::json!("authenticated_xform_recovery");
    let response: serde_json::Value =
        serde_json::from_slice(&dispatch_json(&mut app, &serde_json::to_vec(&v).unwrap())).unwrap();
    assert_eq!(response["ok"], false);
}
