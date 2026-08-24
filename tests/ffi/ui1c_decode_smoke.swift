import Foundation

func check(_ condition: @autoclosure () -> Bool, _ message: String) {
    guard condition() else { fputs("UI1D decode failure: \(message)\n", stderr); exit(1) }
}

let inspectionJSON = #"""
{
  "session_id":"opaque-session",
  "project":{"display_name":"Synthetic","byte_size":42,"recognized_studio_vision":true,"sequence_count":4,"overall_readiness":"partially_supported","warning_count":2,"future_project_field":true},
  "sequences":[
    {"sequence_id":"sequence-ready","display_name":"Ready Sequence","readiness":"ready","readiness_reason":{"code":"validated_compatibility_profile","severity":"informational","export_enabled":true,"display_detail":"Synthetic ready detail.","diagnostic_ref":null,"future_reason":1},"musical_track_count":3,"supported_event_families":[],"warning_count":0,"export_capability":{"profile_id":"hidden","profile_version":1,"display_label":"Synthetic capability","future_capability":true},"diagnostics_available":true},
    {"sequence_id":"sequence-partial","display_name":"Partial Sequence","readiness":"partially_supported","readiness_reason":{"code":"missing_channel_routing","severity":"data_loss_risk","export_enabled":false,"display_detail":"Synthetic partial detail.","diagnostic_ref":"hidden"},"musical_track_count":null,"supported_event_families":[],"warning_count":1,"export_capability":null,"diagnostics_available":true},
    {"sequence_id":"sequence-unsupported","display_name":"Unsupported Sequence","readiness":"unsupported","readiness_reason":{"code":"unsupported_project_profile","severity":"caution","export_enabled":false,"display_detail":"Synthetic unsupported detail.","diagnostic_ref":null},"musical_track_count":1,"supported_event_families":[],"warning_count":1,"export_capability":null,"diagnostics_available":false},
    {"sequence_id":"sequence-unknown","display_name":"Unknown Sequence","readiness":"unknown","readiness_reason":{"code":"unknown_structure","severity":"caution","export_enabled":false,"display_detail":"Synthetic unknown detail.","diagnostic_ref":null},"musical_track_count":null,"supported_event_families":[],"warning_count":0,"export_capability":null,"diagnostics_available":true,"future_sequence_field":true}
  ],
  "warnings":[
    {"code":"do_not_show","message":"First synthetic warning.","technical_detail":"hidden","scope":"project","severity":"caution","diagnostic_ref":null,"source_order":9},
    {"code":"also_hidden","message":"Second synthetic warning.","technical_detail":null,"scope":"sequence","severity":"data_loss_risk","diagnostic_ref":"hidden","source_order":1,"future_warning":true}
  ],
  "diagnostics_available":true,
  "future_top_level":true
}
"""#.data(using: .utf8)!

let decoder = JSONDecoder()
let inspection = try decoder.decode(ProjectInspection.self, from: inspectionJSON)
check(inspection.overallReadiness == .partiallySupported, "project readiness")
check(inspection.sequences.map(\.readiness) == [.ready, .partiallySupported, .unsupported, .unknown], "all readiness values")
check(inspection.sequences[0].readiness.displayName == "Ready", "ready presentation")
check(inspection.sequences[1].readiness.displayName == "Partially supported", "partial presentation")
check(inspection.sequences[2].readiness.displayName == "Not currently exportable", "unsupported presentation")
check(inspection.sequences[3].readiness.displayName == "Readiness unknown", "unknown presentation")
check(inspection.sequences[1].readinessReason.displayDetail == "Synthetic partial detail.", "Core display detail")
check(inspection.sequences[1].readinessReason.severity == .dataLossRisk, "reason severity")
check(inspection.sequences[0].exportCapability?.displayLabel == "Synthetic capability", "optional export capability")
check(inspection.sequences[1].exportCapability == nil, "absent export capability")
check(inspection.warnings.map(\.message) == ["First synthetic warning.", "Second synthetic warning."], "warning order")
check(inspection.warnings.map(\.severity) == [.caution, .dataLossRisk], "warning severity")

let zeroJSON = #"{"session_id":"zero","project":{"display_name":"newest STUFF","byte_size":12,"recognized_studio_vision":true,"sequence_count":0,"overall_readiness":"unknown","warning_count":1},"sequences":[],"warnings":[{"code":"mutable","message":"Synthetic zero-sequence warning.","technical_detail":null,"scope":"project","severity":"caution","diagnostic_ref":null,"source_order":0}],"diagnostics_available":true}"#.data(using: .utf8)!
let zero = try decoder.decode(ProjectInspection.self, from: zeroJSON)
check(zero.recognizedStudioVision && zero.sequences.isEmpty && zero.overallReadiness == .unknown, "recognized zero sequence")

let unrecognizedJSON = #"{"session_id":"unrecognized","project":{"display_name":"LICENSE","byte_size":7,"recognized_studio_vision":false,"sequence_count":0,"overall_readiness":"unsupported","warning_count":1},"sequences":[],"warnings":[{"code":"mutable","message":"Synthetic unrecognized warning.","technical_detail":null,"scope":"project","severity":"caution","diagnostic_ref":null,"source_order":0}],"diagnostics_available":true}"#.data(using: .utf8)!
let unrecognized = try decoder.decode(ProjectInspection.self, from: unrecognizedJSON)
check(!unrecognized.recognizedStudioVision && unrecognized.warnings.count == 1, "successful unrecognized inspection")

struct DiagnosticsSmokeEnvelope: Decodable {
    let ok: Bool
    let result: DiagnosticsSummary
}

let diagnosticsJSON = #"{"ok":true,"result":{"core_version":"0.1.0","contract_version":1,"source_sha256":null,"identification_evidence":["hidden"],"recognized_profile":"Synthetic profile","structural_status":"Synthetic structural status","unsupported_families":["synthetic_family"],"compatibility_profile":{"profile_id":"hidden","profile_version":1,"display_label":"Validated research profile"},"technical_errors":[],"export_report":null,"future_diagnostic":true},"future_envelope":true}"#.data(using: .utf8)!
let diagnostics = try decoder.decode(DiagnosticsSmokeEnvelope.self, from: diagnosticsJSON)
check(diagnostics.ok, "summary diagnostics envelope")
check(diagnostics.result.coreVersion == "0.1.0", "Core version")
check(diagnostics.result.recognizedProfile == "Synthetic profile", "recognized profile")
check(diagnostics.result.structuralStatus == "Synthetic structural status", "structural status")
check(diagnostics.result.unsupportedFamilies == ["synthetic_family"], "unsupported families")
check(diagnostics.result.compatibilityProfile?.displayLabel == "Validated research profile", "compatibility display label")
print("UI1D Swift decoder smoke passed")
