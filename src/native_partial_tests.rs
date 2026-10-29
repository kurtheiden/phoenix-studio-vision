//! Native bridge integration with synthetic, source-qualified 120 neighborhoods.
use crate::{app_contract::*, app_service::AppService, json_transport::dispatch_json};
use serde_json::{json, Value};
use std::{fs, path::Path};
fn record(bytes: &mut Vec<u8>, kind: u8, payload: &[u8]) {
    bytes.push(kind);
    bytes.extend((payload.len() as u32).to_be_bytes());
    bytes.extend(payload);
}
fn fixture() -> Vec<u8> {
    let original = crate::recovery120::tests::fixture(&crate::recovery120::tests::note(), 1);
    let root = crate::sequence_container::parse_root_record_stream(&original).unwrap();
    let mut bytes =
        crate::observed_layout120::tests::synthetic_marker(4, b"Portable partial", [0xfe, 0xff]);
    bytes.truncate(8 + 75 + 120 * 4);
    for (i, label) in [
        b"Meter Track".as_slice(),
        b"Tempo Track",
        b"Unresolved",
        b"Playable",
    ]
    .iter()
    .enumerate()
    {
        let at = 8 + 121 + 120 * i;
        bytes[at..at + label.len()].copy_from_slice(label);
        bytes[at - 39..at - 31].copy_from_slice(&[0x80, 0, 4, 0, 0, 4, 1, 0]);
        if i >= 2 {
            bytes[at - 33..at].fill(255);
            bytes[at - 33..at - 30].copy_from_slice(&[1, 0, 1]);
        }
    }
    record(&mut bytes, 7, &[0; 10]);
    let mut meter = vec![0; 38];
    meter[..2].copy_from_slice(&[0, 1]);
    meter[2..6].copy_from_slice(&1u32.to_be_bytes());
    meter[14..22].copy_from_slice(&[0, 255, 88, 4, 3, 2, 8, 8]);
    meter[22..29].copy_from_slice(&[135, 255, 255, 127, 255, 47, 0]);
    record(&mut bytes, 2, &meter);
    for index in [3, 4, 5] {
        bytes.extend(&original[root.records[index].record_range.clone()]);
    }
    let mut primary = vec![0; 14];
    primary[..2].copy_from_slice(&[0, 1]);
    primary[2..6].copy_from_slice(&1u32.to_be_bytes());
    primary.extend([0, 255, 32, 0]); // Retained as an unresolved whole primary-only Track.
    primary.extend([255, 0, 0, 0, 255, 47, 0]);
    record(&mut bytes, 2, &primary);
    for framed in &root.records[6..] {
        bytes.extend(&original[framed.record_range.clone()]);
    }
    bytes
}
fn inspect(app: &mut AppService, source: &Path) -> InspectProjectResponse {
    app.inspect_project(InspectProjectRequest {
        contract_version: 1,
        source_path: source.to_str().unwrap().into(),
        diagnostics_level: DiagnosticsLevel::Full,
    })
    .unwrap()
}
fn request(inspection: &InspectProjectResponse, destination: &Path) -> Value {
    json!({"operation":"partial_recovery","contract_version":1,"payload":{
        "session_id":inspection.session_id.as_str(),"sequence_id":inspection.sequences[0].sequence_id.as_str(),
        "recovery_inclusion":"include_muted","destination_folder":destination,"filename_stem":"Partial"}})
}
fn call(app: &mut AppService, request: &Value) -> Value {
    serde_json::from_slice(&dispatch_json(app, &serde_json::to_vec(request).unwrap())).unwrap()
}
#[test]
fn qualified_120_native_preview_consent_scope_mute_and_publication() {
    publication_safety(fixture());
}
#[test]
fn paired_and_zero_state_native_preview_consent_and_publication() {
    publication_safety(crate::experimental120::admission_tests::publication_fixture());
}
fn publication_safety(bytes: Vec<u8>) {
    assert!(crate::observed_layout120::observe_project_120(&bytes).is_ok());
    assert!(crate::sequence_container::parse_project_166(&bytes).is_err());
    let source = crate::app_service::tests::portable_path(&bytes);
    let destination = crate::app_service::tests::portable_directory();
    let mut app = AppService::new();
    let inspection = inspect(&mut app, &source);
    assert_eq!(inspection.sequences.len(), 1);
    assert_ne!(inspection.sequences[0].readiness, Readiness::Ready);
    let mut request = request(&inspection, &destination);
    let preview = call(&mut app, &request);
    assert_eq!(preview["ok"], true, "{preview}");
    let report = preview["result"].clone();
    assert_eq!(report["authorization"]["profile"], "layout120");
    assert_eq!(report["recovered_event_counts"]["notes"], 1);
    assert_eq!(
        report["recovered_event_counts"]["unresolved_source_objects"],
        1
    );
    assert_eq!(fs::read_dir(&destination).unwrap().count(), 0);
    for alteration in ["profile", "source", "scope", "counts"] {
        let mut altered = report.clone();
        match alteration {
            "profile" => altered["authorization"]["profile"] = json!("descriptor166"),
            "source" => altered["source_sha256"] = json!("forged"),
            "scope" => altered["sequence_ordinal"] = json!(1),
            _ => altered["recovered_event_counts"]["notes"] = json!(2),
        }
        request["payload"]["consent_report"] = altered;
        assert_eq!(
            call(&mut app, &request)["error"]["code"],
            "partial_consent_changed"
        );
        assert_eq!(fs::read_dir(&destination).unwrap().count(), 0);
    }
    request["payload"]["consent_report"] = report.clone();
    request["payload"]["recovery_inclusion"] = json!("exclude_muted");
    assert_eq!(
        call(&mut app, &request)["error"]["code"],
        "partial_mute_selection_unsupported"
    );
    request["payload"]["recovery_inclusion"] = json!("include_muted");
    let renewed = inspect(&mut app, &source);
    let mut stale = self::request(&renewed, &destination);
    stale["payload"]["consent_report"] = report.clone();
    assert_eq!(
        call(&mut app, &stale)["error"]["code"],
        "partial_consent_changed"
    );
    assert_eq!(fs::read_dir(&destination).unwrap().count(), 0);
    let mut other_bytes = bytes.clone();
    other_bytes[0] ^= 1;
    let other_path = crate::app_service::tests::portable_path(&other_bytes);
    let other = inspect(&mut app, &other_path);
    let mut crossed = self::request(&other, &destination);
    crossed["payload"]["consent_report"] = report;
    assert_eq!(
        call(&mut app, &crossed)["error"]["code"],
        "partial_consent_changed"
    );
    assert_eq!(fs::read_dir(&destination).unwrap().count(), 0);
    let published = call(&mut app, &request);
    assert_eq!(published["ok"], true, "{published}");
    assert!(Path::new(published["result"]["output_path"].as_str().unwrap()).is_file());
    assert!(Path::new(published["result"]["limitations_path"].as_str().unwrap()).is_file());
    let file_count = fs::read_dir(&destination).unwrap().count();
    assert_eq!(
        call(&mut app, &request)["ok"],
        false,
        "collision must refuse"
    );
    assert_eq!(fs::read_dir(&destination).unwrap().count(), file_count);
    fs::write(&source, &other_bytes).unwrap();
    request["payload"]["filename_stem"] = json!("Changed source");
    assert_eq!(
        call(&mut app, &request)["error"]["code"],
        "partial_unavailable"
    );
    assert_eq!(fs::read_dir(&destination).unwrap().count(), file_count);
    fs::remove_file(source).unwrap();
    fs::remove_file(other_path).unwrap();
    fs::remove_dir_all(destination).unwrap();
}
#[test]
fn complete_120_still_uses_complete_export_contract() {
    let bytes = crate::recovery120::tests::fixture(&crate::recovery120::tests::note(), 1);
    let expected = crate::recovery120::recover_project120(&bytes)
        .unwrap()
        .remove(0)
        .result
        .unwrap()
        .midi()
        .to_vec();
    let source = crate::app_service::tests::portable_path(&bytes);
    let destination = crate::app_service::tests::portable_directory();
    let mut app = AppService::new();
    let inspection = inspect(&mut app, &source);
    let result = app
        .export_sequence(ExportSequenceRequest {
            contract_version: 1,
            session_id: inspection.session_id.clone(),
            sequence_id: inspection.sequences[0].sequence_id.clone(),
            destination_folder: destination.to_str().unwrap().into(),
            filename_stem: "Complete".into(),
            collision_policy: CollisionPolicy::FailIfExists,
            operation_id: None,
        })
        .unwrap();
    assert_eq!(fs::read(result.output_path).unwrap(), expected);
    assert_eq!(
        call(&mut app, &request(&inspection, &destination))["ok"],
        false
    );
    fs::remove_file(source).unwrap();
    fs::remove_dir_all(destination).unwrap();
}

#[test]
fn consent_for_one_qualified_sequence_cannot_authorize_another() {
    let mut bytes = fixture();
    let second = fixture();
    let root = crate::sequence_container::parse_root_record_stream(&second).unwrap();
    for record in root
        .records
        .iter()
        .filter(|record| !matches!(record.record_type.value, 16 | 42))
    {
        bytes.extend(&second[record.record_range.clone()]);
    }
    let source = crate::app_service::tests::portable_path(&bytes);
    let destination = crate::app_service::tests::portable_directory();
    let mut app = AppService::new();
    let inspected = inspect(&mut app, &source);
    assert_eq!(inspected.sequences.len(), 2);
    let first = request(&inspected, &destination);
    let preview = call(&mut app, &first);
    assert_eq!(preview["ok"], true, "{preview}");
    let mut second = first;
    second["payload"]["sequence_id"] = json!(inspected.sequences[1].sequence_id.as_str());
    let second_preview = call(&mut app, &second);
    assert_eq!(second_preview["ok"], true, "{second_preview}");
    second["payload"]["consent_report"] = preview["result"].clone();
    assert_eq!(
        call(&mut app, &second)["error"]["code"],
        "partial_consent_changed"
    );
    assert_eq!(fs::read_dir(&destination).unwrap().count(), 0);
    fs::remove_file(source).unwrap();
    fs::remove_dir(destination).unwrap();
}
