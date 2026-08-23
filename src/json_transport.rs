//! Strict, safe JSON transport for the owned application-service contract.
//!
//! This module contains no C ABI, raw pointers, service registry, or allocator
//! handoff. A caller supplies an existing [`AppService`] so its sessions live
//! across multiple synchronous dispatches.

use crate::app_contract::{
    AppError, CollisionPolicy, DiagnosticsLevel, ExportSequenceRequest, InspectProjectRequest,
    OperationId, SequenceId, SessionId,
};
use crate::app_service::AppService;
use serde::{Deserialize, Serialize};
use serde_json::Value;

const INVALID_UTF8: &str = "invalid_utf8";
const MALFORMED_JSON: &str = "malformed_json";
const MISSING_OPERATION: &str = "missing_operation";
const UNKNOWN_OPERATION: &str = "unknown_operation";
const MISSING_CONTRACT_VERSION: &str = "missing_contract_version";
const INVALID_REQUEST_FIELDS: &str = "invalid_request_fields";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EmptyPayload {}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct InspectProjectPayload {
    source_path: String,
    diagnostics_level: DiagnosticsLevel,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GetDiagnosticsPayload {
    session_id: String,
    diagnostics_level: DiagnosticsLevel,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ExportSequencePayload {
    session_id: String,
    sequence_id: String,
    destination_folder: String,
    filename_stem: String,
    collision_policy: CollisionPolicy,
    operation_id: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CancelOperationPayload {
    operation_id: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RequestEnvelope<P> {
    operation: String,
    payload: P,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct VersionedRequestEnvelope<P> {
    operation: String,
    contract_version: u32,
    payload: P,
}

#[derive(Serialize)]
struct SuccessEnvelope<T> {
    ok: bool,
    result: T,
}

#[derive(Serialize)]
struct FailureEnvelope<E> {
    ok: bool,
    error: E,
}

#[derive(Serialize)]
struct AppFailure {
    kind: &'static str,
    app_error: AppError,
}

#[derive(Serialize)]
struct TransportFailure<'a> {
    kind: &'static str,
    code: &'static str,
    message: &'a str,
}

/// Dispatch one UTF-8 JSON request through an existing synchronous service.
///
/// The returned bytes are always an owned UTF-8 JSON success, application
/// error, or transport-error envelope. Object key order is not contractual.
pub fn dispatch_json(service: &mut AppService, request: &[u8]) -> Vec<u8> {
    let request = match std::str::from_utf8(request) {
        Ok(request) => request,
        Err(_) => {
            return transport_error(INVALID_UTF8, "The request is not valid UTF-8.");
        }
    };
    let value: Value = match serde_json::from_str(request) {
        Ok(value) => value,
        Err(_) => {
            return transport_error(MALFORMED_JSON, "The request is not valid JSON.");
        }
    };
    let Some(object) = value.as_object() else {
        return transport_error(INVALID_REQUEST_FIELDS, "The request must be a JSON object.");
    };
    let Some(operation) = object.get("operation") else {
        return transport_error(MISSING_OPERATION, "The request is missing operation.");
    };
    let Some(operation) = operation.as_str() else {
        return transport_error(
            INVALID_REQUEST_FIELDS,
            "The operation field must be a string.",
        );
    };

    match operation {
        "get_api_info" => dispatch_api_info(service, value),
        "inspect_project" => dispatch_inspect_project(service, value),
        "get_diagnostics" => dispatch_get_diagnostics(service, value),
        "export_sequence" => dispatch_export_sequence(service, value),
        "cancel_operation" => dispatch_cancel_operation(service, value),
        _ => transport_error(UNKNOWN_OPERATION, "The requested operation is unknown."),
    }
}

fn dispatch_api_info(service: &AppService, value: Value) -> Vec<u8> {
    let envelope: RequestEnvelope<EmptyPayload> = match parse_fields(value) {
        Ok(envelope) => envelope,
        Err(response) => return response,
    };
    debug_assert_eq!(envelope.operation, "get_api_info");
    success(service.api_info())
}

fn dispatch_inspect_project(service: &mut AppService, value: Value) -> Vec<u8> {
    if !has_contract_version(&value) {
        return transport_error(
            MISSING_CONTRACT_VERSION,
            "The request is missing contract_version.",
        );
    }
    let envelope: VersionedRequestEnvelope<InspectProjectPayload> = match parse_fields(value) {
        Ok(envelope) => envelope,
        Err(response) => return response,
    };
    debug_assert_eq!(envelope.operation, "inspect_project");
    core_result(service.inspect_project(InspectProjectRequest {
        contract_version: envelope.contract_version,
        source_path: envelope.payload.source_path,
        diagnostics_level: envelope.payload.diagnostics_level,
    }))
}

fn dispatch_get_diagnostics(service: &AppService, value: Value) -> Vec<u8> {
    let envelope: RequestEnvelope<GetDiagnosticsPayload> = match parse_fields(value) {
        Ok(envelope) => envelope,
        Err(response) => return response,
    };
    debug_assert_eq!(envelope.operation, "get_diagnostics");
    core_result(service.get_diagnostics(
        &SessionId::new(envelope.payload.session_id),
        envelope.payload.diagnostics_level,
    ))
}

fn dispatch_export_sequence(service: &AppService, value: Value) -> Vec<u8> {
    if !has_contract_version(&value) {
        return transport_error(
            MISSING_CONTRACT_VERSION,
            "The request is missing contract_version.",
        );
    }
    let envelope: VersionedRequestEnvelope<ExportSequencePayload> = match parse_fields(value) {
        Ok(envelope) => envelope,
        Err(response) => return response,
    };
    debug_assert_eq!(envelope.operation, "export_sequence");
    core_result(service.export_sequence(ExportSequenceRequest {
        contract_version: envelope.contract_version,
        session_id: SessionId::new(envelope.payload.session_id),
        sequence_id: SequenceId::new(envelope.payload.sequence_id),
        destination_folder: envelope.payload.destination_folder,
        filename_stem: envelope.payload.filename_stem,
        collision_policy: envelope.payload.collision_policy,
        operation_id: envelope.payload.operation_id.map(OperationId::new),
    }))
}

fn dispatch_cancel_operation(service: &AppService, value: Value) -> Vec<u8> {
    let envelope: RequestEnvelope<CancelOperationPayload> = match parse_fields(value) {
        Ok(envelope) => envelope,
        Err(response) => return response,
    };
    debug_assert_eq!(envelope.operation, "cancel_operation");
    core_result(service.cancel_operation(&OperationId::new(envelope.payload.operation_id)))
}

fn has_contract_version(value: &Value) -> bool {
    value
        .as_object()
        .is_some_and(|object| object.contains_key("contract_version"))
}

fn parse_fields<T>(value: Value) -> Result<T, Vec<u8>>
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_value(value).map_err(|_| {
        transport_error(
            INVALID_REQUEST_FIELDS,
            "The request contains invalid or unexpected fields.",
        )
    })
}

fn core_result<T>(result: Result<T, AppError>) -> Vec<u8>
where
    T: Serialize,
{
    match result {
        Ok(result) => success(result),
        Err(app_error) => serialize(&FailureEnvelope {
            ok: false,
            error: AppFailure {
                kind: "app",
                app_error,
            },
        }),
    }
}

fn success<T>(result: T) -> Vec<u8>
where
    T: Serialize,
{
    serialize(&SuccessEnvelope { ok: true, result })
}

fn transport_error(code: &'static str, message: &str) -> Vec<u8> {
    serialize(&FailureEnvelope {
        ok: false,
        error: TransportFailure {
            kind: "transport",
            code,
            message,
        },
    })
}

fn serialize<T>(value: &T) -> Vec<u8>
where
    T: Serialize,
{
    serde_json::to_vec(value).expect("UI0F1 transport values must serialize to JSON")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_contract::{Warning, WarningScope, WarningSeverity, CONTRACT_VERSION};
    use crate::app_service::tests::{portable_directory, portable_path, portable_registry};
    use crate::export_handoff::tests::portable_project;
    use serde_json::{json, Value};
    use std::fs;

    fn dispatch(service: &mut AppService, request: Value) -> Value {
        serde_json::from_slice(&dispatch_json(
            service,
            &serde_json::to_vec(&request).unwrap(),
        ))
        .unwrap()
    }

    fn transport_code(response: &Value) -> &str {
        assert_eq!(response["ok"], false);
        assert_eq!(response["error"]["kind"], "transport");
        response["error"]["code"].as_str().unwrap()
    }

    fn app_error(response: &Value) -> &Value {
        assert_eq!(response["ok"], false);
        assert_eq!(response["error"]["kind"], "app");
        &response["error"]["app_error"]
    }

    fn portable_transport_service() -> (AppService, std::path::PathBuf) {
        let bytes = portable_project();
        let source = portable_path(&bytes);
        (AppService::with_registry(portable_registry(&bytes)), source)
    }

    fn inspect(service: &mut AppService, source: &std::path::Path) -> Value {
        dispatch(
            service,
            json!({
                "operation": "inspect_project",
                "contract_version": CONTRACT_VERSION,
                "payload": {
                    "source_path": source.to_string_lossy(),
                    "diagnostics_level": "full"
                }
            }),
        )
    }

    #[test]
    fn ui0f1_api_info_has_one_strict_bootstrap_shape() {
        let mut service = AppService::new();
        let response = dispatch(
            &mut service,
            json!({"operation": "get_api_info", "payload": {}}),
        );
        assert_eq!(response["ok"], true);
        assert_eq!(response["result"]["contract_version"], CONTRACT_VERSION);
        assert!(response["result"]["core_version"].is_string());

        let response = dispatch(
            &mut service,
            json!({
                "operation": "get_api_info",
                "contract_version": CONTRACT_VERSION,
                "payload": {}
            }),
        );
        assert_eq!(transport_code(&response), INVALID_REQUEST_FIELDS);
    }

    #[test]
    fn ui0f1_inspection_and_diagnostics_preserve_one_service_session() {
        let (mut service, source) = portable_transport_service();
        let inspection = inspect(&mut service, &source);
        assert_eq!(inspection["ok"], true);
        assert_eq!(inspection["result"]["contract_version"], CONTRACT_VERSION);
        assert_eq!(inspection["result"]["project"]["sequence_count"], 1);
        assert_eq!(inspection["result"]["sequences"][0]["readiness"], "ready");
        let session_id = inspection["result"]["session_id"].as_str().unwrap();

        let diagnostics = dispatch(
            &mut service,
            json!({
                "operation": "get_diagnostics",
                "payload": {
                    "session_id": session_id,
                    "diagnostics_level": "full"
                }
            }),
        );
        assert_eq!(diagnostics["ok"], true);
        assert_eq!(diagnostics["result"]["contract_version"], CONTRACT_VERSION);
        assert!(diagnostics["result"]["core_version"].is_string());
        assert!(diagnostics["result"]["structural_status"].is_string());
        assert!(diagnostics["result"]["source_sha256"].is_string());
        fs::remove_file(source).unwrap();
    }

    #[test]
    fn ui0f1_contract_mismatch_and_unknown_session_preserve_core_errors() {
        let mut service = AppService::new();
        let mismatch = dispatch(
            &mut service,
            json!({
                "operation": "inspect_project",
                "contract_version": CONTRACT_VERSION + 1,
                "payload": {
                    "source_path": "/not/read/ui0f1-version",
                    "diagnostics_level": "none"
                }
            }),
        );
        let error = app_error(&mismatch);
        assert_eq!(error["contract_version"], CONTRACT_VERSION);
        assert_eq!(error["category"], "internal_error");
        assert_eq!(error["diagnostic_code"], "contract_version_mismatch");
        assert_eq!(error["operation"], "inspect_project");

        let unknown = dispatch(
            &mut service,
            json!({
                "operation": "get_diagnostics",
                "payload": {
                    "session_id": "unknown-ui0f1-session",
                    "diagnostics_level": "summary"
                }
            }),
        );
        let error = app_error(&unknown);
        assert_eq!(error["category"], "internal_error");
        assert_eq!(error["diagnostic_code"], "unknown_session");
        assert_eq!(error["operation"], "get_diagnostics");
        for field in ["session_id", "sequence_id", "diagnostic_ref"] {
            assert!(error.get(field).is_some(), "missing {field}");
            assert!(error[field].is_null(), "{field} must be null");
        }
        assert!(error["display_message"].is_string());
        assert!(error["technical_message"].is_string());
    }

    #[test]
    fn ui0f1_export_round_trip_preserves_core_commit_response() {
        let (mut service, source) = portable_transport_service();
        let inspection = inspect(&mut service, &source);
        let session_id = inspection["result"]["session_id"].as_str().unwrap();
        let sequence_id = inspection["result"]["sequences"][0]["sequence_id"]
            .as_str()
            .unwrap();
        let destination = portable_directory();

        let exported = dispatch(
            &mut service,
            json!({
                "operation": "export_sequence",
                "contract_version": CONTRACT_VERSION,
                "payload": {
                    "session_id": session_id,
                    "sequence_id": sequence_id,
                    "destination_folder": destination.to_string_lossy(),
                    "filename_stem": "UI0F1 Portable",
                    "collision_policy": "fail_if_exists",
                    "operation_id": null
                }
            }),
        );
        assert_eq!(exported["ok"], true);
        assert_eq!(exported["result"]["session_id"], session_id);
        assert_eq!(exported["result"]["sequence_id"], sequence_id);
        assert_eq!(exported["result"]["validation_status"], "validated");
        let output_path = exported["result"]["output_path"].as_str().unwrap();
        assert_eq!(
            fs::read(output_path).unwrap()[..4],
            [b'M', b'T', b'h', b'd']
        );
        assert!(exported["result"]["counts"]["notes"].as_u64().unwrap() > 0);

        fs::remove_dir_all(destination).unwrap();
        fs::remove_file(source).unwrap();
    }

    #[test]
    fn ui0f1_cancel_is_the_existing_unsupported_core_operation() {
        let mut service = AppService::new();
        let response = dispatch(
            &mut service,
            json!({
                "operation": "cancel_operation",
                "payload": {"operation_id": "ui0f1-arbitrary"}
            }),
        );
        let error = app_error(&response);
        assert_eq!(error["category"], "internal_error");
        assert_eq!(error["diagnostic_code"], "cancellation_not_supported");
        assert_eq!(error["operation"], "cancel_operation");
    }

    #[test]
    fn ui0f1_malformed_and_strict_request_failures_are_transport_errors() {
        let mut service = AppService::new();
        let invalid_utf8: Value =
            serde_json::from_slice(&dispatch_json(&mut service, &[0xff, 0xfe])).unwrap();
        assert_eq!(transport_code(&invalid_utf8), INVALID_UTF8);

        let malformed: Value =
            serde_json::from_slice(&dispatch_json(&mut service, b"{\"operation\":")).unwrap();
        assert_eq!(transport_code(&malformed), MALFORMED_JSON);

        let cases = [
            (json!({}), MISSING_OPERATION),
            (
                json!({"operation": "not_an_operation", "payload": {}}),
                UNKNOWN_OPERATION,
            ),
            (
                json!({
                    "operation": "inspect_project",
                    "payload": {"source_path": "x", "diagnostics_level": "none"}
                }),
                MISSING_CONTRACT_VERSION,
            ),
            (
                json!({"operation": 7, "payload": {}}),
                INVALID_REQUEST_FIELDS,
            ),
            (
                json!({
                    "operation": "get_api_info",
                    "payload": {},
                    "unexpected": true
                }),
                INVALID_REQUEST_FIELDS,
            ),
            (
                json!({
                    "operation": "cancel_operation",
                    "payload": {"operation_id": "x", "unexpected": true}
                }),
                INVALID_REQUEST_FIELDS,
            ),
            (
                json!({
                    "operation": "get_diagnostics",
                    "contract_version": CONTRACT_VERSION,
                    "payload": {"session_id": "x", "diagnostics_level": "none"}
                }),
                INVALID_REQUEST_FIELDS,
            ),
            (
                json!({
                    "operation": "cancel_operation",
                    "contract_version": CONTRACT_VERSION,
                    "payload": {"operation_id": "x"}
                }),
                INVALID_REQUEST_FIELDS,
            ),
            (
                json!({
                    "operation": "inspect_project",
                    "contract_version": "1",
                    "payload": {"source_path": "x", "diagnostics_level": "none"}
                }),
                INVALID_REQUEST_FIELDS,
            ),
        ];
        for (request, expected) in cases {
            let response = dispatch(&mut service, request);
            assert_eq!(transport_code(&response), expected);
        }
    }

    #[test]
    fn ui0f1_warning_enums_and_optional_fields_have_stable_json() {
        let warning = Warning {
            code: "ui0f1_warning".into(),
            message: "Display text".into(),
            technical_detail: None,
            scope: WarningScope::GenericTrack,
            severity: WarningSeverity::DataLossRisk,
            diagnostic_ref: None,
            source_order: 4,
        };
        let value = serde_json::to_value(warning).unwrap();
        assert_eq!(value["scope"], "generic_track");
        assert_eq!(value["severity"], "data_loss_risk");
        assert!(value.get("technical_detail").is_some());
        assert!(value["technical_detail"].is_null());
        assert!(value.get("diagnostic_ref").is_some());
        assert!(value["diagnostic_ref"].is_null());
    }
}
