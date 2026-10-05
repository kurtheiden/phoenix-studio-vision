use super::*;
use crate::recovery120::tests::{fixture, note};
fn directory() -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "phoenix-app120-{}-{}",
        std::process::id(),
        next_export_temp_filename()
    ));
    fs::create_dir(&path).unwrap();
    path
}
fn inspect(service: &mut AppService, path: &Path) -> InspectProjectResponse {
    service
        .inspect_project(InspectProjectRequest {
            contract_version: CONTRACT_VERSION,
            source_path: path.to_string_lossy().into_owned(),
            diagnostics_level: DiagnosticsLevel::Full,
        })
        .unwrap()
}
fn request(response: &InspectProjectResponse, index: usize, dir: &Path) -> ExportSequenceRequest {
    ExportSequenceRequest {
        contract_version: CONTRACT_VERSION,
        session_id: response.session_id.clone(),
        sequence_id: response.sequences[index].sequence_id.clone(),
        destination_folder: dir.to_string_lossy().into_owned(),
        filename_stem: format!("sequence-{index}"),
        collision_policy: CollisionPolicy::FailIfExists,
        operation_id: Some(OperationId::new("test120")),
    }
}
#[test]
fn portable_app120_fresh_export_identity_and_destination_safety() {
    let dir = directory();
    let path = dir.join("source");
    let bytes = fixture(&note(), 1);
    fs::write(&path, &bytes).unwrap();
    let mut service = AppService::new();
    let response = inspect(&mut service, &path);
    assert_eq!(response.sequences.len(), 1);
    assert_eq!(response.sequences[0].readiness, Readiness::Ready);
    let req = request(&response, 0, &dir);
    let exported = service.export_sequence(req.clone()).unwrap();
    let expected = crate::recovery120::recover_project120(&bytes)
        .unwrap()
        .remove(0)
        .result
        .unwrap()
        .midi()
        .to_vec();
    assert_eq!(fs::read(exported.output_path).unwrap(), expected);
    assert!(service.export_sequence(req.clone()).is_err());
    let mut wrong = req.clone();
    wrong.sequence_id = SequenceId::new("unknown");
    assert!(service.export_sequence(wrong).is_err());
    let mut stale = req.clone();
    stale.session_id = SessionId::new("unknown");
    assert!(service.export_sequence(stale).is_err());
    let session = service.sessions.get_mut(&response.session_id).unwrap();
    session.sequence_ordinals.insert(req.sequence_id.clone(), 1);
    assert!(service.export_sequence(req.clone()).is_err());
    service
        .sessions
        .get_mut(&response.session_id)
        .unwrap()
        .sequence_ordinals
        .insert(req.sequence_id.clone(), 0);
    fs::write(&path, fixture(&note(), 2)).unwrap();
    assert!(service.export_sequence(req).is_err());
    let blocked = inspect(&mut service, &path);
    assert_ne!(blocked.sequences[0].readiness, Readiness::Ready);
    assert!(blocked.sequences[0].bounded_export_capability.is_none());
    assert!(service.export_sequence(request(&blocked, 0, &dir)).is_err());
    let row = &mut service
        .sessions
        .get_mut(&blocked.session_id)
        .unwrap()
        .response
        .sequences[0];
    row.readiness = Readiness::Ready;
    row.bounded_export_capability = Some(BoundedExportCapability::validated120());
    assert!(service.export_sequence(request(&blocked, 0, &dir)).is_err());
    fs::remove_dir_all(dir).unwrap();
}
#[test]
#[ignore = "requires external authentic SCHOOL and validated artifacts"]
fn external_school_appservice_all_rows_exports_and_parity() {
    let path = Path::new("/Users/kurtheiden/Documents/Phoenix Research/Authentic Studio Vision Projects/SCHOOL PROJECTS");
    let artifact = Path::new(
        "/Users/kurtheiden/Documents/Phoenix Research/Experimental MIDI Recovery/SCHOOL PROJECTS",
    );
    let files = [
        (0, "DANCING"),
        (1, "NOTHING"),
        (4, "Nothing FINAL"),
        (7, "Sequence H"),
        (8, "Unnamed Sequence 09"),
        (9, "Wind ensemble"),
        (11, "Tuba thing"),
        (12, "Sequence M"),
    ];
    let dir = directory();
    let mut service = AppService::new();
    let response = inspect(&mut service, path);
    assert_eq!(response.sequences.len(), 15);
    assert_eq!(
        response
            .sequences
            .iter()
            .filter(|s| s.readiness == Readiness::Ready)
            .count(),
        8
    );
    for index in 0..15 {
        let req = request(&response, index, &dir);
        if let Some((_, name)) = files.iter().find(|(i, _)| *i == index) {
            assert!(response.sequences[index]
                .bounded_export_capability
                .is_some());
            let result = service.export_sequence(req).unwrap();
            assert_eq!(
                fs::read(result.output_path).unwrap(),
                fs::read(artifact.join(format!("SCHOOL - {name}.mid"))).unwrap(),
                "{name}"
            );
        } else {
            assert!(response.sequences[index]
                .bounded_export_capability
                .is_none());
            assert!(service.export_sequence(req).is_err());
        }
    }
    fs::remove_dir_all(dir).unwrap();
}
#[test]
fn portable120_json_transport_preserves_existing_contract() {
    let dir = directory();
    let path = dir.join("source");
    fs::write(&path, fixture(&note(), 1)).unwrap();
    let mut service = AppService::new();
    let inspect = serde_json::json!({"operation":"inspect_project","contract_version":CONTRACT_VERSION,"payload":{"source_path":path.to_string_lossy(),"diagnostics_level":"full"}});
    let response: serde_json::Value = serde_json::from_slice(
        &crate::json_transport::dispatch_json(&mut service, &serde_json::to_vec(&inspect).unwrap()),
    )
    .unwrap();
    assert_eq!(response["ok"], true);
    let result = &response["result"];
    assert_eq!(
        result["sequences"][0]["bounded_export_capability"]["contract_id"],
        "source_layout120_bounded_sequence"
    );
    let export = serde_json::json!({"operation":"export_sequence","contract_version":CONTRACT_VERSION,"payload":{"session_id":result["session_id"],"sequence_id":result["sequences"][0]["sequence_id"],"destination_folder":dir.to_string_lossy(),"filename_stem":"transport","collision_policy":"fail_if_exists"}});
    let response: serde_json::Value = serde_json::from_slice(
        &crate::json_transport::dispatch_json(&mut service, &serde_json::to_vec(&export).unwrap()),
    )
    .unwrap();
    assert_eq!(response["ok"], true, "{response}");
    assert_eq!(response["result"]["counts"]["notes"], 1);
    fs::remove_dir_all(dir).unwrap();
}
