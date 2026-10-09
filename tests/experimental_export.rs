use phoenix::app_contract::*;
use phoenix::app_service::AppService;
use std::path::PathBuf;
const SOURCE: &str =
    "/Users/kurtheiden/Documents/Phoenix Research/Authentic Studio Vision Projects/COMIC BOOK";
fn folder() -> PathBuf {
    let p = std::env::temp_dir().join(format!(
        "phoenix-cca027-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir(&p).unwrap();
    p
}
fn inspect(service: &mut AppService, path: &str) -> InspectProjectResponse {
    service
        .inspect_project(InspectProjectRequest {
            contract_version: CONTRACT_VERSION,
            source_path: path.into(),
            diagnostics_level: DiagnosticsLevel::Full,
        })
        .unwrap()
}
fn request(
    response: &InspectProjectResponse,
    name: &str,
    dir: &std::path::Path,
) -> ExportSequenceRequest {
    ExportSequenceRequest {
        contract_version: CONTRACT_VERSION,
        session_id: response.session_id.clone(),
        sequence_id: response
            .sequences
            .iter()
            .find(|s| s.display_name == name)
            .unwrap()
            .sequence_id
            .clone(),
        destination_folder: dir.to_string_lossy().into_owned(),
        filename_stem: name.into(),
        collision_policy: CollisionPolicy::FailIfExists,
        operation_id: None,
    }
}
#[test]
fn authentic_note_only_export_is_explicit_incomplete_and_not_ready() {
    for (name, notes) in [
        ("pick up the pace", 64),
        ("nitemare b4 xmas", 167),
        ("sampler xperimnt", 239),
    ] {
        let mut service = AppService::new();
        let response = inspect(&mut service, SOURCE);
        let dir = folder();
        let req = request(&response, name, &dir);
        let plan = service
            .preview_experimental_sequence(&req.session_id, &req.sequence_id)
            .unwrap();
        assert_eq!(plan.report().included_notes, notes);
        assert_eq!(plan.report().omitted_known_notes, 0);
        assert_eq!(plan.report().unresolved_semantic_event_count, None);
        assert_eq!(
            plan.report().tracks.len(),
            if name == "sampler xperimnt" { 5 } else { 1 }
        );
        assert!(service
            .export_experimental_sequence(req.clone(), &plan, false)
            .is_err());
        assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 0);
        assert!(service.export_sequence(req.clone()).is_err());
        let result = service
            .export_experimental_sequence(req.clone(), &plan, true)
            .unwrap();
        assert_eq!(result.label, phoenix::experimental_export::LABEL);
        assert!(result.output_path.contains("EXPERIMENTAL INCOMPLETE"));
        let midi = std::fs::read(&result.output_path).unwrap();
        assert_eq!(&midi[..4], b"MThd");
        let report = std::fs::read(&result.limitations_path).unwrap();
        let json: serde_json::Value = serde_json::from_slice(&report).unwrap();
        assert_eq!(json["included_notes"], notes);
        assert!(json["unresolved_semantic_event_count"].is_null());
        assert!(midi
            .windows(b"NOT VERIFIED".len())
            .any(|w| w == b"NOT VERIFIED"));
        assert!(midi.windows(b"conductor".len()).any(|w| w == b"conductor"));
        let hash = std::fs::read(&result.output_path).unwrap();
        assert!(service
            .export_experimental_sequence(req.clone(), &plan, true)
            .is_err());
        assert_eq!(std::fs::read(&result.output_path).unwrap(), hash);
        assert!(service.export_sequence(req).is_err());
        let after = inspect(&mut service, SOURCE);
        assert_eq!(
            response
                .sequences
                .iter()
                .map(|s| s.readiness)
                .collect::<Vec<_>>(),
            after
                .sequences
                .iter()
                .map(|s| s.readiness)
                .collect::<Vec<_>>()
        );
    }
}
#[test]
fn changed_source_and_wrong_selection_cannot_reuse_plan() {
    let dir = folder();
    let copy = dir.join("copy");
    std::fs::copy(SOURCE, &copy).unwrap();
    let mut service = AppService::new();
    let response = inspect(&mut service, copy.to_str().unwrap());
    let req = request(&response, "pick up the pace", &dir);
    let plan = service
        .preview_experimental_sequence(&req.session_id, &req.sequence_id)
        .unwrap();
    let mut bad = req.clone();
    bad.sequence_id = SequenceId::new("wrong");
    assert!(service
        .export_experimental_sequence(bad, &plan, true)
        .is_err());
    let mut bad = req.clone();
    bad.collision_policy = CollisionPolicy::GenerateUniqueName;
    assert!(service
        .export_experimental_sequence(bad, &plan, true)
        .is_err());
    let mut bytes = std::fs::read(&copy).unwrap();
    bytes[0] ^= 1;
    std::fs::write(&copy, bytes).unwrap();
    assert!(service
        .export_experimental_sequence(req, &plan, true)
        .unwrap_err()
        .0
        .contains("identity changed"));
    assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 1);
}
#[test]
fn other_event_or_ownership_candidates_refuse_without_omitting_events() {
    let mut service = AppService::new();
    let response = inspect(&mut service, SOURCE);
    for name in ["SETUP.MID", "manners", "calm intro music"] {
        let row = response
            .sequences
            .iter()
            .find(|s| s.display_name == name)
            .unwrap();
        assert!(
            service
                .preview_experimental_sequence(&response.session_id, &row.sequence_id)
                .is_err(),
            "{name}"
        );
    }
}

#[test]
fn occupied_report_or_midi_is_never_overwritten_and_invalid_names_refuse() {
    let mut service = AppService::new();
    let response = inspect(&mut service, SOURCE);
    let dir = folder();
    let req = request(&response, "pick up the pace", &dir);
    let plan = service
        .preview_experimental_sequence(&req.session_id, &req.sequence_id)
        .unwrap();
    let mut invalid = req.clone();
    invalid.filename_stem = "../escape".into();
    assert!(service
        .export_experimental_sequence(invalid, &plan, true)
        .is_err());
    assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 0);
    let report = dir.join("EXPERIMENTAL INCOMPLETE - pick up the pace.limitations.json");
    std::fs::write(&report, b"OWNER FILE").unwrap();
    assert!(service
        .export_experimental_sequence(req, &plan, true)
        .is_err());
    assert_eq!(std::fs::read(&report).unwrap(), b"OWNER FILE");
    assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 1);
    let dir = folder();
    let req = request(&response, "pick up the pace", &dir);
    let midi = dir.join("EXPERIMENTAL INCOMPLETE - pick up the pace.mid");
    std::fs::write(&midi, b"OWNER MIDI").unwrap();
    let error = service
        .export_experimental_sequence(req, &plan, true)
        .unwrap_err();
    assert!(error.0.contains("limitations retained at"));
    assert_eq!(std::fs::read(&midi).unwrap(), b"OWNER MIDI");
    assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 2);
}
