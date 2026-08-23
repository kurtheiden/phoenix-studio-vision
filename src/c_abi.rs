//! Synchronous C ABI for the UI0F1 JSON transport.
//!
//! The exported functions are the only foreign boundary. Registry entries own
//! `AppService` values, calls are admitted atomically with respect to destroy,
//! and the global registry lock is never held while Core work runs.

use crate::app_service::AppService;
use crate::json_transport::{dispatch_json, transport_error_json};
use std::collections::HashMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr;
use std::slice;
use std::sync::{Arc, Condvar, Mutex, OnceLock};

pub const PHOENIX_ABI_VERSION: u32 = 1;

pub const PHOENIX_STATUS_OK: i32 = 0;
pub const PHOENIX_STATUS_INVALID_ARGUMENT: i32 = 1;
pub const PHOENIX_STATUS_INVALID_HANDLE: i32 = 2;
pub const PHOENIX_STATUS_INTERNAL_FAILURE: i32 = 3;

pub type PhoenixServiceHandle = u64;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PhoenixBuffer {
    pub ptr: *mut u8,
    pub len: usize,
}

impl PhoenixBuffer {
    const EMPTY: Self = Self {
        ptr: ptr::null_mut(),
        len: 0,
    };
}

struct ServiceRegistry {
    next_token: Option<PhoenixServiceHandle>,
    entries: HashMap<PhoenixServiceHandle, Arc<ServiceEntry>>,
}

impl ServiceRegistry {
    fn new() -> Self {
        Self {
            next_token: Some(1),
            entries: HashMap::new(),
        }
    }
}

struct ServiceEntry {
    service: Mutex<AppService>,
    lifecycle: Mutex<Lifecycle>,
    quiescent: Condvar,
}

impl ServiceEntry {
    fn new(service: AppService) -> Self {
        assert_send::<AppService>();
        Self {
            service: Mutex::new(service),
            lifecycle: Mutex::new(Lifecycle {
                destroying: false,
                admitted_calls: 0,
            }),
            quiescent: Condvar::new(),
        }
    }
}

struct Lifecycle {
    destroying: bool,
    admitted_calls: usize,
}

struct Admission {
    entry: Arc<ServiceEntry>,
}

impl Drop for Admission {
    fn drop(&mut self) {
        // Lifecycle mutations contain no fallible work. Recovering this small
        // state on decrement is necessary so a contained panic cannot strand
        // a destroy waiter forever.
        let mut lifecycle = match self.entry.lifecycle.lock() {
            Ok(lifecycle) => lifecycle,
            Err(poisoned) => poisoned.into_inner(),
        };
        if lifecycle.admitted_calls > 0 {
            lifecycle.admitted_calls -= 1;
        }
        if lifecycle.admitted_calls == 0 {
            self.entry.quiescent.notify_all();
        }
    }
}

enum AdmissionResult {
    Admitted(Admission),
    InvalidHandle,
    InternalFailure,
}

enum DestroyResult {
    Destroyed,
    InvalidHandle,
    InternalFailure,
}

static REGISTRY: OnceLock<Mutex<ServiceRegistry>> = OnceLock::new();

fn registry() -> &'static Mutex<ServiceRegistry> {
    REGISTRY.get_or_init(|| Mutex::new(ServiceRegistry::new()))
}

fn assert_send<T: Send>() {}

fn register_service(service: AppService) -> Result<PhoenixServiceHandle, ()> {
    let entry = Arc::new(ServiceEntry::new(service));
    let mut registry = registry().lock().map_err(|_| ())?;
    let token = registry.next_token.ok_or(())?;
    if token == 0 || registry.entries.contains_key(&token) {
        return Err(());
    }
    registry.entries.insert(token, entry);
    registry.next_token = token.checked_add(1);
    Ok(token)
}

fn admit_call(handle: PhoenixServiceHandle) -> AdmissionResult {
    if handle == 0 {
        return AdmissionResult::InvalidHandle;
    }

    // Fixed order: registry, then entry lifecycle. Neither is held while the
    // service mutex or Core dispatcher runs.
    let registry = match registry().lock() {
        Ok(registry) => registry,
        Err(_) => return AdmissionResult::InternalFailure,
    };
    let entry = match registry.entries.get(&handle) {
        Some(entry) => Arc::clone(entry),
        None => return AdmissionResult::InvalidHandle,
    };
    let mut lifecycle = match entry.lifecycle.lock() {
        Ok(lifecycle) => lifecycle,
        Err(_) => return AdmissionResult::InternalFailure,
    };
    if lifecycle.destroying {
        return AdmissionResult::InvalidHandle;
    }
    lifecycle.admitted_calls = match lifecycle.admitted_calls.checked_add(1) {
        Some(count) => count,
        None => return AdmissionResult::InternalFailure,
    };
    drop(lifecycle);
    drop(registry);

    AdmissionResult::Admitted(Admission { entry })
}

fn destroy_service(handle: PhoenixServiceHandle) -> DestroyResult {
    if handle == 0 {
        return DestroyResult::InvalidHandle;
    }

    let entry = {
        let mut registry = match registry().lock() {
            Ok(registry) => registry,
            Err(_) => return DestroyResult::InternalFailure,
        };
        let entry = match registry.entries.get(&handle) {
            Some(entry) => Arc::clone(entry),
            None => return DestroyResult::InvalidHandle,
        };
        let mut lifecycle = match entry.lifecycle.lock() {
            Ok(lifecycle) => lifecycle,
            Err(_) => return DestroyResult::InternalFailure,
        };
        if lifecycle.destroying {
            return DestroyResult::InvalidHandle;
        }
        lifecycle.destroying = true;
        registry.entries.remove(&handle);
        drop(lifecycle);
        drop(registry);
        entry
    };

    let mut lifecycle = match entry.lifecycle.lock() {
        Ok(lifecycle) => lifecycle,
        Err(_) => return DestroyResult::InternalFailure,
    };
    while lifecycle.admitted_calls != 0 {
        lifecycle = match entry.quiescent.wait(lifecycle) {
            Ok(lifecycle) => lifecycle,
            Err(_) => return DestroyResult::InternalFailure,
        };
    }
    DestroyResult::Destroyed
}

fn request_representation_is_valid(request_ptr: *const u8, request_len: usize) -> bool {
    if request_len > isize::MAX as usize {
        return false;
    }
    request_len == 0 || (request_ptr as usize).checked_add(request_len).is_some()
}

unsafe fn publish_response(response: Vec<u8>, out_response: *mut PhoenixBuffer) -> i32 {
    if response.is_empty() {
        return PHOENIX_STATUS_INTERNAL_FAILURE;
    }
    let response = response.into_boxed_slice();
    let len = response.len();
    let ptr = Box::into_raw(response) as *mut u8;
    // SAFETY: The exported caller contract requires `out_response` to point to
    // aligned writable storage for one `PhoenixBuffer` for the whole call.
    unsafe {
        ptr::write(out_response, PhoenixBuffer { ptr, len });
    }
    PHOENIX_STATUS_OK
}

fn publish_admitted_response(_admission: Admission, publish: impl FnOnce() -> i32) -> i32 {
    // The admission guard remains owned by this scope until publication has
    // returned. Unwinding through `publish` also drops it exactly once.
    publish()
}

unsafe fn call_inner(
    handle: PhoenixServiceHandle,
    request_ptr: *const u8,
    request_len: usize,
    out_response: *mut PhoenixBuffer,
) -> i32 {
    if request_ptr.is_null() && request_len != 0 {
        return unsafe {
            publish_response(
                transport_error_json(
                    "null_request",
                    "The request pointer is null for a nonzero length.",
                ),
                out_response,
            )
        };
    }
    if !request_representation_is_valid(request_ptr, request_len) {
        return unsafe {
            publish_response(
                transport_error_json(
                    "invalid_request_length",
                    "The request length cannot be represented safely.",
                ),
                out_response,
            )
        };
    }

    let admission = match admit_call(handle) {
        AdmissionResult::Admitted(admission) => admission,
        AdmissionResult::InvalidHandle => {
            return unsafe {
                publish_response(
                    transport_error_json(
                        "invalid_handle",
                        "The service handle is invalid or no longer active.",
                    ),
                    out_response,
                )
            };
        }
        AdmissionResult::InternalFailure => return PHOENIX_STATUS_INTERNAL_FAILURE,
    };

    let request = if request_len == 0 {
        &[]
    } else {
        // SAFETY: Representability, nullness, and address arithmetic were
        // checked above. The foreign caller must additionally guarantee that
        // the non-null pointer refers to `request_len` readable bytes for this
        // synchronous call; Rust cannot validate that foreign allocation.
        unsafe { slice::from_raw_parts(request_ptr, request_len) }
    };
    let response = {
        let mut service = match admission.entry.service.lock() {
            Ok(service) => service,
            Err(_) => return PHOENIX_STATUS_INTERNAL_FAILURE,
        };
        dispatch_json(&mut service, request)
    };
    publish_admitted_response(admission, || unsafe {
        publish_response(response, out_response)
    })
}

#[no_mangle]
pub extern "C" fn phoenix_abi_version() -> u32 {
    catch_unwind(|| PHOENIX_ABI_VERSION).unwrap_or(0)
}

/// Create one process-local service handle.
///
/// # Safety
///
/// A non-null `out_handle` must be aligned, writable for one `u64`, and valid
/// for the duration of this call.
#[no_mangle]
pub unsafe extern "C" fn phoenix_service_create(out_handle: *mut PhoenixServiceHandle) -> i32 {
    if out_handle.is_null() {
        return PHOENIX_STATUS_INVALID_ARGUMENT;
    }
    // SAFETY: The caller precondition above requires writable aligned storage.
    unsafe {
        ptr::write(out_handle, 0);
    }
    match catch_unwind(AssertUnwindSafe(|| register_service(AppService::new()))) {
        Ok(Ok(handle)) => {
            // SAFETY: The same caller-provided storage remains valid.
            unsafe {
                ptr::write(out_handle, handle);
            }
            PHOENIX_STATUS_OK
        }
        Ok(Err(())) | Err(_) => PHOENIX_STATUS_INTERNAL_FAILURE,
    }
}

#[no_mangle]
pub extern "C" fn phoenix_service_destroy(handle: PhoenixServiceHandle) -> i32 {
    match catch_unwind(AssertUnwindSafe(|| destroy_service(handle))) {
        Ok(DestroyResult::Destroyed) => PHOENIX_STATUS_OK,
        Ok(DestroyResult::InvalidHandle) => PHOENIX_STATUS_INVALID_HANDLE,
        Ok(DestroyResult::InternalFailure) | Err(_) => PHOENIX_STATUS_INTERNAL_FAILURE,
    }
}

/// Dispatch one length-delimited request and return a Rust-owned JSON buffer.
///
/// # Safety
///
/// `out_response` must be non-null, aligned, writable for one `PhoenixBuffer`,
/// and valid for this call. For nonzero `request_len`, `request_ptr` must point
/// to that many readable bytes for the call. Arbitrary foreign pointer validity
/// cannot be checked by Rust.
#[no_mangle]
pub unsafe extern "C" fn phoenix_call(
    handle: PhoenixServiceHandle,
    request_ptr: *const u8,
    request_len: usize,
    out_response: *mut PhoenixBuffer,
) -> i32 {
    if out_response.is_null() {
        return PHOENIX_STATUS_INVALID_ARGUMENT;
    }
    // SAFETY: The caller contract requires writable aligned output storage.
    unsafe {
        ptr::write(out_response, PhoenixBuffer::EMPTY);
    }

    match catch_unwind(AssertUnwindSafe(|| unsafe {
        call_inner(handle, request_ptr, request_len, out_response)
    })) {
        Ok(status) => status,
        Err(_) => {
            // Keep a nonzero fallback deterministic even if constructing the
            // recoverable panic envelope itself panics.
            // SAFETY: The validated output storage remains valid for the call.
            unsafe {
                ptr::write(out_response, PhoenixBuffer::EMPTY);
            }
            match catch_unwind(AssertUnwindSafe(|| unsafe {
                publish_response(
                    transport_error_json(
                        "internal_panic",
                        "Phoenix contained an internal Rust panic.",
                    ),
                    out_response,
                )
            })) {
                Ok(status) => status,
                Err(_) => {
                    // SAFETY: The validated output storage remains valid.
                    unsafe {
                        ptr::write(out_response, PhoenixBuffer::EMPTY);
                    }
                    PHOENIX_STATUS_INTERNAL_FAILURE
                }
            }
        }
    }
}

/// Release one unchanged buffer returned by `phoenix_call`.
///
/// # Safety
///
/// A non-null buffer must be an unchanged Phoenix allocation and must be freed
/// exactly once. Fabricated pairs, modified lengths, and double frees violate
/// the caller contract.
#[no_mangle]
pub unsafe extern "C" fn phoenix_free_buffer(buffer: PhoenixBuffer) -> i32 {
    if buffer.ptr.is_null() {
        return if buffer.len == 0 {
            PHOENIX_STATUS_OK
        } else {
            PHOENIX_STATUS_INVALID_ARGUMENT
        };
    }
    match catch_unwind(AssertUnwindSafe(|| {
        let slice = ptr::slice_from_raw_parts_mut(buffer.ptr, buffer.len);
        // SAFETY: The caller contract requires the exact pointer and length
        // returned by Phoenix, unchanged and not previously freed. Those are
        // precisely the slice metadata produced by `Box<[u8]>` transfer.
        unsafe {
            drop(Box::from_raw(slice));
        }
    })) {
        Ok(()) => PHOENIX_STATUS_OK,
        Err(_) => PHOENIX_STATUS_INTERNAL_FAILURE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_contract::CONTRACT_VERSION;
    use crate::app_service::tests::{portable_directory, portable_path, portable_registry};
    use crate::export_handoff::tests::portable_project;
    use serde_json::{json, Value};
    use std::fs;
    use std::mem::{align_of, size_of};
    use std::process::{Command, Stdio};
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::mpsc;
    use std::thread;

    fn create_service() -> PhoenixServiceHandle {
        let mut handle = 0;
        assert_eq!(
            unsafe { phoenix_service_create(&mut handle) },
            PHOENIX_STATUS_OK
        );
        assert_ne!(handle, 0);
        handle
    }

    fn portable_service() -> (PhoenixServiceHandle, std::path::PathBuf) {
        let bytes = portable_project();
        let source = portable_path(&bytes);
        let service = AppService::with_registry(portable_registry(&bytes));
        (register_service(service).unwrap(), source)
    }

    fn call_bytes(
        handle: PhoenixServiceHandle,
        request_ptr: *const u8,
        request_len: usize,
    ) -> (i32, PhoenixBuffer, Vec<u8>) {
        let mut buffer = PhoenixBuffer::EMPTY;
        let status = unsafe { phoenix_call(handle, request_ptr, request_len, &mut buffer) };
        let bytes = if status == PHOENIX_STATUS_OK {
            assert!(!buffer.ptr.is_null());
            assert_ne!(buffer.len, 0);
            // SAFETY: A successful call returned this readable Phoenix buffer,
            // and it remains owned until the free below.
            unsafe { slice::from_raw_parts(buffer.ptr, buffer.len).to_vec() }
        } else {
            assert_eq!(buffer, PhoenixBuffer::EMPTY);
            Vec::new()
        };
        (status, buffer, bytes)
    }

    fn call_json(handle: PhoenixServiceHandle, request: &Value) -> Value {
        let request = serde_json::to_vec(request).unwrap();
        let (status, buffer, bytes) = call_bytes(handle, request.as_ptr(), request.len());
        assert_eq!(status, PHOENIX_STATUS_OK);
        assert_eq!(buffer.len, bytes.len());
        let response = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(unsafe { phoenix_free_buffer(buffer) }, PHOENIX_STATUS_OK);
        response
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

    #[test]
    fn ui0f2_version_status_layout_and_send_contract_are_stable() {
        assert_send::<AppService>();
        assert_eq!(phoenix_abi_version(), 1);
        assert_eq!(PHOENIX_STATUS_OK, 0);
        assert_eq!(PHOENIX_STATUS_INVALID_ARGUMENT, 1);
        assert_eq!(PHOENIX_STATUS_INVALID_HANDLE, 2);
        assert_eq!(PHOENIX_STATUS_INTERNAL_FAILURE, 3);
        assert_eq!(size_of::<PhoenixServiceHandle>(), 8);
        assert_eq!(size_of::<PhoenixBuffer>(), size_of::<usize>() * 2);
        assert_eq!(align_of::<PhoenixBuffer>(), align_of::<usize>());
    }

    #[test]
    fn ui0f2_create_tokens_are_nonzero_monotonic_and_not_reused() {
        assert_eq!(
            unsafe { phoenix_service_create(ptr::null_mut()) },
            PHOENIX_STATUS_INVALID_ARGUMENT
        );
        let first = create_service();
        assert_eq!(phoenix_service_destroy(first), PHOENIX_STATUS_OK);
        let second = create_service();
        assert!(second > first);
        assert_eq!(phoenix_service_destroy(second), PHOENIX_STATUS_OK);
    }

    #[test]
    fn ui0f2_api_info_uses_owned_nonempty_response_buffer() {
        let handle = create_service();
        let response = call_json(handle, &json!({"operation": "get_api_info", "payload": {}}));
        assert_eq!(response["ok"], true);
        assert_eq!(response["result"]["contract_version"], CONTRACT_VERSION);
        assert_eq!(phoenix_service_destroy(handle), PHOENIX_STATUS_OK);
    }

    #[test]
    fn ui0f2_inspection_then_diagnostics_preserves_handle_state() {
        let (handle, source) = portable_service();
        let inspection = call_json(
            handle,
            &json!({
                "operation": "inspect_project",
                "contract_version": CONTRACT_VERSION,
                "payload": {
                    "source_path": source.to_string_lossy(),
                    "diagnostics_level": "full"
                }
            }),
        );
        let session_id = inspection["result"]["session_id"].as_str().unwrap();
        let diagnostics = call_json(
            handle,
            &json!({
                "operation": "get_diagnostics",
                "payload": {
                    "session_id": session_id,
                    "diagnostics_level": "full"
                }
            }),
        );
        assert_eq!(diagnostics["ok"], true);
        assert_eq!(diagnostics["result"]["contract_version"], CONTRACT_VERSION);
        assert_eq!(phoenix_service_destroy(handle), PHOENIX_STATUS_OK);
        fs::remove_file(source).unwrap();
    }

    #[test]
    fn ui0f2_portable_export_and_cancel_reuse_core_operations() {
        let (handle, source) = portable_service();
        let inspection = call_json(
            handle,
            &json!({
                "operation": "inspect_project",
                "contract_version": CONTRACT_VERSION,
                "payload": {
                    "source_path": source.to_string_lossy(),
                    "diagnostics_level": "full"
                }
            }),
        );
        let session_id = inspection["result"]["session_id"].as_str().unwrap();
        let sequence_id = inspection["result"]["sequences"][0]["sequence_id"]
            .as_str()
            .unwrap();
        let destination = portable_directory();
        let exported = call_json(
            handle,
            &json!({
                "operation": "export_sequence",
                "contract_version": CONTRACT_VERSION,
                "payload": {
                    "session_id": session_id,
                    "sequence_id": sequence_id,
                    "destination_folder": destination.to_string_lossy(),
                    "filename_stem": "UI0F2 Portable",
                    "collision_policy": "fail_if_exists",
                    "operation_id": null
                }
            }),
        );
        assert_eq!(exported["ok"], true);
        let output_path = exported["result"]["output_path"].as_str().unwrap();
        assert_eq!(&fs::read(output_path).unwrap()[..4], b"MThd");

        let cancelled = call_json(
            handle,
            &json!({
                "operation": "cancel_operation",
                "payload": {"operation_id": "ui0f2-arbitrary"}
            }),
        );
        let error = app_error(&cancelled);
        assert_eq!(error["diagnostic_code"], "cancellation_not_supported");
        assert_eq!(error["operation"], "cancel_operation");

        assert_eq!(phoenix_service_destroy(handle), PHOENIX_STATUS_OK);
        fs::remove_dir_all(destination).unwrap();
        fs::remove_file(source).unwrap();
    }

    #[test]
    fn ui0f2_destroy_rejects_invalid_and_call_after_destroy_is_json() {
        assert_eq!(phoenix_service_destroy(0), PHOENIX_STATUS_INVALID_HANDLE);
        assert_eq!(
            phoenix_service_destroy(u64::MAX - 17),
            PHOENIX_STATUS_INVALID_HANDLE
        );
        let handle = create_service();
        assert_eq!(phoenix_service_destroy(handle), PHOENIX_STATUS_OK);
        assert_eq!(
            phoenix_service_destroy(handle),
            PHOENIX_STATUS_INVALID_HANDLE
        );
        let response = call_json(handle, &json!({"operation": "get_api_info", "payload": {}}));
        assert_eq!(transport_code(&response), "invalid_handle");
    }

    #[test]
    fn ui0f2_destroy_waits_for_response_publication() {
        let handle = create_service();
        let admission = match admit_call(handle) {
            AdmissionResult::Admitted(admission) => admission,
            _ => panic!("new service handle was not admitted"),
        };
        let entry = Arc::clone(&admission.entry);
        let (publication_started_tx, publication_started_rx) = mpsc::channel();
        let (release_publication_tx, release_publication_rx) = mpsc::channel();
        let call = thread::spawn(move || {
            publish_admitted_response(admission, || {
                publication_started_tx.send(()).unwrap();
                release_publication_rx.recv().unwrap();
                PHOENIX_STATUS_OK
            })
        });
        publication_started_rx.recv().unwrap();

        let (destroyed_tx, destroyed_rx) = mpsc::channel();
        let destroy = thread::spawn(move || {
            destroyed_tx.send(phoenix_service_destroy(handle)).unwrap();
        });
        loop {
            let lifecycle = entry.lifecycle.lock().unwrap();
            if lifecycle.destroying {
                assert_eq!(lifecycle.admitted_calls, 1);
                break;
            }
            drop(lifecycle);
            thread::yield_now();
        }
        assert!(matches!(
            destroyed_rx.try_recv(),
            Err(mpsc::TryRecvError::Empty)
        ));

        release_publication_tx.send(()).unwrap();
        assert_eq!(call.join().unwrap(), PHOENIX_STATUS_OK);
        assert_eq!(destroyed_rx.recv().unwrap(), PHOENIX_STATUS_OK);
        destroy.join().unwrap();
    }

    #[test]
    fn ui0f2_request_pointer_and_length_failures_are_bounded() {
        let handle = create_service();
        let (status, buffer, bytes) = call_bytes(handle, ptr::null(), 4);
        assert_eq!(status, PHOENIX_STATUS_OK);
        let response: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(transport_code(&response), "null_request");
        assert_eq!(unsafe { phoenix_free_buffer(buffer) }, PHOENIX_STATUS_OK);

        let (status, buffer, bytes) = call_bytes(handle, ptr::null(), 0);
        assert_eq!(status, PHOENIX_STATUS_OK);
        let response: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(transport_code(&response), "malformed_json");
        assert_eq!(unsafe { phoenix_free_buffer(buffer) }, PHOENIX_STATUS_OK);

        let excessive = (isize::MAX as usize).checked_add(1).unwrap();
        let dangling = ptr::NonNull::<u8>::dangling().as_ptr();
        let (status, buffer, bytes) = call_bytes(handle, dangling, excessive);
        assert_eq!(status, PHOENIX_STATUS_OK);
        let response: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(transport_code(&response), "invalid_request_length");
        assert_eq!(unsafe { phoenix_free_buffer(buffer) }, PHOENIX_STATUS_OK);

        let request = br#"{"operation":"get_api_info","payload":{}}"#;
        assert_eq!(
            unsafe { phoenix_call(handle, request.as_ptr(), request.len(), ptr::null_mut()) },
            PHOENIX_STATUS_INVALID_ARGUMENT
        );
        assert_eq!(phoenix_service_destroy(handle), PHOENIX_STATUS_OK);
    }

    #[test]
    fn ui0f2_buffer_free_contract_handles_valid_and_null_forms() {
        assert_eq!(
            unsafe { phoenix_free_buffer(PhoenixBuffer::EMPTY) },
            PHOENIX_STATUS_OK
        );
        assert_eq!(
            unsafe {
                phoenix_free_buffer(PhoenixBuffer {
                    ptr: ptr::null_mut(),
                    len: 1,
                })
            },
            PHOENIX_STATUS_INVALID_ARGUMENT
        );
        let handle = create_service();
        let request = br#"{"operation":"get_api_info","payload":{}}"#;
        let (status, buffer, bytes) = call_bytes(handle, request.as_ptr(), request.len());
        assert_eq!(status, PHOENIX_STATUS_OK);
        assert_eq!(buffer.len, bytes.len());
        assert_eq!(unsafe { phoenix_free_buffer(buffer) }, PHOENIX_STATUS_OK);
        assert_eq!(phoenix_service_destroy(handle), PHOENIX_STATUS_OK);
    }

    #[test]
    fn ui0f2_repeated_sequential_lifecycle_is_stable() {
        for _ in 0..8 {
            let handle = create_service();
            let response = call_json(handle, &json!({"operation": "get_api_info", "payload": {}}));
            assert_eq!(response["ok"], true);
            assert_eq!(phoenix_service_destroy(handle), PHOENIX_STATUS_OK);
        }
    }

    fn compiler_available(compiler: &str) -> bool {
        Command::new(compiler)
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok()
    }

    fn syntax_check_header(compiler: &str, standard: &str, suffix: &str) {
        static NEXT_SOURCE: AtomicU64 = AtomicU64::new(0);
        let source = std::env::temp_dir().join(format!(
            "phoenix-ui0f2-header-{}-{}.{}",
            std::process::id(),
            NEXT_SOURCE.fetch_add(1, Ordering::Relaxed),
            suffix
        ));
        fs::write(
            &source,
            b"#include \"phoenix.h\"\nint main(void) { phoenix_buffer_t b = {0}; return (int)(b.len + phoenix_abi_version()); }\n",
        )
        .unwrap();
        let include = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("include");
        let status = Command::new(compiler)
            .arg(standard)
            .arg("-fsyntax-only")
            .arg("-I")
            .arg(include)
            .arg(&source)
            .status()
            .unwrap();
        fs::remove_file(source).unwrap();
        assert!(status.success(), "{compiler} rejected phoenix.h");
    }

    #[test]
    fn ui0f2_public_header_is_valid_c_and_optional_cpp() {
        if compiler_available("cc") {
            syntax_check_header("cc", "-std=c11", "c");
        }
        if compiler_available("c++") {
            syntax_check_header("c++", "-std=c++11", "cpp");
        }
    }
}
