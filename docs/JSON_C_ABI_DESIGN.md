# UI0F JSON-over-C ABI design

## Objective and boundary

UI0F is a synchronous, versioned transport for the already implemented owned
`AppService` contract. It does not add application operations or reproduce
parser, compatibility, session, export, error, or cancellation policy. The v0
operation surface is exactly `get_api_info`, `inspect_project`,
`get_diagnostics`, `export_sequence`, and `cancel_operation`.

Crate-internal preparation/revalidation helpers, parser values, compatibility
registry internals, and raw session maps never cross this boundary. Paths are
UTF-8 JSON strings containing the existing Core `String` paths. URL/bookmark
and security-scoped-access ownership remains in the native platform layer.
Core display and technical text is opaque owned UTF-8; clients must not parse
`technical_message` or manufacture new path/diagnostic fields from it.

## Dispatcher and operation mapping

Choose one generic dispatcher, not one exported function per operation. This
keeps operation growth in versioned JSON rather than the C symbol table while
retaining explicit request and response DTOs.

Every request is one JSON object:

```json
{
  "operation": "inspect_project",
  "contract_version": 1,
  "payload": {}
}
```

The discriminator values are the explicit `AppOperation::stable_name()`
vocabulary, except that the public API-info operation retains the already
committed `get_api_info` value: `get_api_info`, `inspect_project`,
`get_diagnostics`, `export_sequence`, and `cancel_operation`. They are never
derived from `Debug` output.

| Operation | Payload | Successful `result` |
|---|---|---|
| `get_api_info` | `{}` | existing `ApiInfo` |
| `inspect_project` | envelope `contract_version` plus existing `InspectProjectRequest` fields in `payload`, without a duplicated inner version | existing `InspectProjectResponse` |
| `get_diagnostics` | `{ "session_id": string, "diagnostics_level": enum }` | existing `Diagnostics` |
| `export_sequence` | envelope `contract_version` plus existing `ExportSequenceRequest` fields in `payload`, without a duplicated inner version | existing `ExportSequenceResponse` |
| `cancel_operation` | `{ "operation_id": string }` | JSON `null` if Core ever succeeds; v0 returns the existing `AppError` |

The versioned v0 operations are exactly `inspect_project` and
`export_sequence`, matching the existing request DTOs that carry
`contract_version`. For each, the dispatcher requires the envelope field to be
present with the required JSON integer representation, then constructs the
existing Core request DTO with that value. It does not decide whether the
number is supported. The Core operation is the sole semantic version authority
and returns its existing `InternalError` / `contract_version_mismatch`
`AppError` with the actual operation identity when the number is unsupported.
The other three operations use their exact table payloads and do not accept an
envelope `contract_version`.

`get_api_info` has exactly one bootstrap request shape:

```json
{"operation":"get_api_info","payload":{}}
```

It contains no `contract_version`; supplying one is an unknown envelope field
and therefore `invalid_request_fields`. Its `ApiInfo` result discovers the
currently supported application contract version. The transport does not
expose `get_inspection`, which is a service helper rather than an
`AppOperation`.

## JSON results and errors

A successful operation has one envelope:

```json
{"ok":true,"result":{}}
```

A valid operation that fails in Core preserves its `AppError` exactly:

```json
{"ok":false,"error":{"kind":"app","app_error":{}}}
```

A failure before a valid Core call is a separate transport error:

```json
{
  "ok": false,
  "error": {
    "kind": "transport",
    "code": "malformed_json",
    "message": "The request is not valid JSON."
  }
}
```

Transport `code` is the stable machine value; `message` is diagnostic text and
is not a branching contract. Initial lowercase snake-case codes are
`null_request`, `invalid_request_length`, `invalid_utf8`, `malformed_json`,
`missing_operation`, `unknown_operation`, `missing_contract_version`,
`invalid_request_fields`, `invalid_handle`, and `internal_panic`. Null/invalid
response out-parameters cannot carry JSON and instead use the C status
described below. A missing required version is `missing_contract_version`; a
wrong JSON type or value outside the required numeric representation is
`invalid_request_fields`. A supported-type numeric value always reaches Core,
including an unsupported application contract version.

## Versioning

`PHOENIX_ABI_VERSION` starts at `1` and is independent of
`app_contract::CONTRACT_VERSION`. The ABI version covers symbol signatures,
C layouts, integer widths, calling convention, and request/response buffer
ownership. `contract_version` covers JSON application semantics and Core DTOs.
Neither number substitutes for the other. Any allocator or buffer-layout
change requires a new ABI version even if application JSON is unchanged.

## JSON serialization policy

Later implementation should put explicit `serde` derives and rename
attributes directly on the public, transport-independent DTOs in
`app_contract`: that module is already the deliberate owned boundary and this
avoids a second mirrored application contract. Only the dispatcher request,
small operation payload adapters, transport error, and result-envelope types
are private transport types.
Internal service/parser/profile types receive no serialization derives.

Fields use `snake_case`. Enums use explicit lowercase `snake_case` strings
matching stable Core names; implementation must add explicit names where a
Core enum currently exposes only a numeric helper. JSON never uses Rust enum
discriminants or `Debug`. Optional fields always serialize as JSON `null`, not
by omission, so one v0 value has one deterministic shape. Later serde code
must not use `skip_serializing_if` or equivalent omission for stable DTO
options. In particular, JSON preserves all `AppError` fields:
`contract_version`, `category`, `display_message`, `technical_message`,
`operation`, `session_id`, `sequence_id`, `diagnostic_code`, and
`diagnostic_ref`.

V0 rejects unknown request fields and wrong field types. This catches client
typos and makes accepted input unambiguous; additive request evolution must use
a new contract version or a new operation shape. Later serde implementation
must enforce unknown-field rejection on the dispatcher envelope, every private
operation payload adapter, and every other closed transport request wrapper.
Request-side unknown enum strings may be rejected in v0. Clients must ignore
unknown response object fields and handle unknown enum/code strings as an
unknown value with safe UI behavior. They must not assume the documented enum
set is exhaustive. Existing values never change meaning.

## Exact C ABI

UI0F implementation owns a version-controlled handwritten public C header. It
defines only fixed-width C types and `size_t`:

```c
typedef uint64_t phoenix_service_handle_t;

typedef struct phoenix_buffer {
    uint8_t *ptr;
    size_t len;
} phoenix_buffer_t;

uint32_t phoenix_abi_version(void);
int32_t phoenix_service_create(phoenix_service_handle_t *out_handle);
int32_t phoenix_service_destroy(phoenix_service_handle_t handle);
int32_t phoenix_call(phoenix_service_handle_t handle,
                     const uint8_t *request_ptr,
                     size_t request_len,
                     phoenix_buffer_t *out_response);
int32_t phoenix_free_buffer(phoenix_buffer_t buffer);
```

The stable C status values are `0` (`PHOENIX_STATUS_OK`), `1`
(`PHOENIX_STATUS_INVALID_ARGUMENT`), `2` (`PHOENIX_STATUS_INVALID_HANDLE`),
and `3` (`PHOENIX_STATUS_INTERNAL_FAILURE`). For `phoenix_call`, status 0 means
`out_response` contains a valid Rust-owned JSON envelope representing success,
a Core `AppError`, a transport validation failure, an invalid handle, or a
recoverable contained panic. A nonzero status means no trustworthy JSON is
available. When the caller supplied a valid writable `out_response`, every
nonzero return zeros it. An invalid, stale, or destroyed handle therefore
returns status 0 plus an `invalid_handle` transport envelope whenever
`out_response` satisfies its caller preconditions; `INVALID_HANDLE` is not an
alternative `phoenix_call` result in that case.

`phoenix_service_create` returns `OK` and writes a nonzero handle on success,
`INVALID_ARGUMENT` for a detectably null output argument, and
`INTERNAL_FAILURE` for a contained infrastructure failure or token exhaustion.
`phoenix_service_destroy` returns `OK` after valid destruction,
`INVALID_HANDLE` for zero, unknown, or already destroyed tokens, and
`INTERNAL_FAILURE` for a contained infrastructure failure. The output pointer
validity requirements below remain caller preconditions; arbitrary invalid
addresses are not detectably invalid arguments.

For successful creation, the caller must provide a non-null `out_handle`
properly aligned for `phoenix_service_handle_t`, pointing to writable storage
for one complete value, and valid for the call. As with `out_response`, Rust
can detect nullness but cannot prove arbitrary foreign storage writable.

Request bytes are caller-owned and borrowed only for the synchronous call.
They are a UTF-8 pointer plus exact byte length, never a NUL-terminated string.
A caller passing `request_len > 0` must supply a non-null pointer aligned for
`uint8_t`, pointing to at least `request_len` readable bytes, and valid for the
duration of `phoenix_call`. A caller must supply a non-null `out_response`
properly aligned for `phoenix_buffer_t`, pointing to writable storage for one
complete value, and valid for the duration of the call.

Rust can check nullness, `request_len <= isize::MAX` where slice
representability requires it, and address-range arithmetic that can be checked
without dereferencing. It cannot prove that an arbitrary foreign pointer is
actually readable or writable. Violating these pointer validity preconditions,
including passing fabricated addresses, may cause undefined behavior; UI0F
does not promise recovery from them, and `catch_unwind` does not make them
safe. A detectably null `out_response` returns `INVALID_ARGUMENT` without
dispatch. A null request pointer with nonzero length becomes `null_request`;
zero-length input is `malformed_json`, with null plus zero accepted as the
empty input representation. A representationally impossible length is
`invalid_request_length`. Embedded NUL is an ordinary byte and makes the JSON
invalid where JSON syntax disallows it.

Every successful JSON response buffer is contractually nonempty valid UTF-8
and has no terminator promise. Rust converts its exact response byte vector to
`Box<[u8]>`; the original data pointer plus exact length supplies the slice
metadata required to reconstruct that allocation, with alignment suitable for
`u8` and no `Vec` capacity assumption. No capacity or allocator-version field
is needed.

`phoenix_free_buffer` reconstructs only a buffer originally returned by
Phoenix with both fields unchanged. It returns `OK` for that buffer and for
`{NULL, 0}`, and `INVALID_ARGUMENT` for the explicitly detectable null/nonzero
mismatch. Only this function may release response memory; Swift, libc `free`,
and another allocator must never do so. The caller must not modify `ptr` or
`len` and must free a valid buffer exactly once. Fabricated non-null pairs,
modified lengths, and double frees violate caller preconditions and cannot be
promised as detectable errors. The implementation must not dereference the
response contents while freeing them.

## Service lifetime and concurrency

Choose explicit service handles rather than one implicit process-global
`AppService`. The small create/destroy cost buys explicit Swift ownership,
test isolation, independent window/project state, and a future path to more
than one client. The handle registry is infrastructure, not application state;
it contains shared service-entry ownership objects rather than holding
`AppService` values under the registry lock for a complete Core operation.

Tokens are nonzero and monotonically generated for the process lifetime. A
successfully destroyed value is never intentionally reused. If the usable
`uint64_t` space would wrap or is exhausted, creation returns
`INTERNAL_FAILURE`; stale values are never silently recycled.

Lookup acquires the registry lock, obtains shared ownership of an entry, and
briefly coordinates admission through that entry's lifecycle state. It
releases the registry lock before potentially long Core work. Each entry owns
one `AppService` plus per-handle serialization and lifecycle state. The fixed
acquisition order when both are needed is registry lock, then entry
lifecycle/serialization lock; reverse acquisition is prohibited. Destroy uses
that ordering to prevent new calls from being admitted, then releases the
registry lock, coordinates with already admitted or active calls, waits for
them as necessary, and finally invalidates the entry. Successful destroy
guarantees that no call associated with that token can subsequently begin Core
execution, and later lookup fails.

Calls on one handle are synchronous and serialized. Separate handles may run
concurrently and own independent service/session state only after UI0F proves
that each entry is safe behind the chosen cross-thread synchronization
primitive. The implementation must establish the required `AppService: Send`
property with a build assertion; a `Mutex` alone is not evidence. Same-handle
reentrant use remains unsupported and must not be required by a client. UI1
calls Core off the main actor.

If a standard-library registry or entry lock is poisoned by unwinding, ABI
code must not `unwrap()` and cause an uncaught secondary panic. It may recover
the owned poisoned value only when invariants remain valid; otherwise it
returns a contained `INTERNAL_FAILURE`. An `internal_panic` JSON envelope is
reserved for an original `phoenix_call` panic that was recovered while the
remaining transport state is still trustworthy. The chosen poison path must be
tested.

## Panic and allocation boundary

Every exported function must be an `extern "C"` no-unwind boundary whose body
is inside `catch_unwind`. This contains ordinary Rust unwinding only, so the
UI0F ABI artifact must use a panic-unwind strategy for contained-panic mapping
to be contractual. A `panic = "abort"` build cannot provide JSON/status
recovery. `catch_unwind` does not catch process aborts, make invalid foreign
pointers safe, or promise containment of hardware faults or foreign
exceptions.

`phoenix_call` maps a recovered Rust panic to `internal_panic` only when it can
still serialize and allocate that transport envelope. Otherwise it returns
`INTERNAL_FAILURE` with a zero output buffer. Create, destroy, and free map a
contained internal panic to `INTERNAL_FAILURE` where their more specific
statuses do not apply. Later implementation uses fallible reservation where
Rust APIs permit. Recoverable serialization/allocation failures may yield an
`INTERNAL_FAILURE` or a transport envelope when possible, but Rust's global
allocator may abort on OOM; allocator abort is outside the recoverable
contract.

## Cancellation and platform boundary

`cancel_operation` transports its `operation_id` as an owned JSON string and
calls `AppService::cancel_operation`. V0 therefore always returns the existing
application error: `internal_error` / `cancellation_not_supported`, operation
`cancel_operation`, with null session and sequence IDs. The transport creates
no cancellation registry and does not change completed export behavior.

Swift retains security-scoped access for the full synchronous call. UI0F
accepts only path strings; it does not add URLs, bookmarks, file descriptors,
file pickers, or persistent authorization.

## Later implementation and tests

UI0F implementation will require runtime `serde` with `derive` and runtime
`serde_json`. They encode/decode production requests and responses, so neither
is dev-only. No dependency is approved or added by this design.

Cargo will later need to retain the normal Rust `rlib` and add `staticlib` for
the macOS/Xcode bridge. A static library is the simplest native-app link unit;
`cdylib` adds runtime embedding/signing concerns and is not required for v0.
XCFramework packaging remains deferred. The implementation prompt must
explicitly approve the Cargo dependency and crate-type changes.

UI0F implementation also owns the public header. It must include `<stdint.h>`
and `<stddef.h>`, include guards, C++ `extern "C"` guards, status constants,
the handle/buffer definitions, exported declarations, and ownership,
nullability, and pointer-precondition comments. It is version-controlled with
the implementation rather than created by UI0G.

Minimum implementation coverage:

- ABI version and stable C status/layout behavior;
- monotonic nonzero token generation, no process-lifetime reuse, and exhaustion
  through a bounded/testable generator seam;
- create, valid destroy, null/unknown/double destroy, use after destroy,
  destroy racing with active work, and destroy coordinating with admitted or
  queued same-handle work so no Core call begins after successful destroy;
- fixed lock ordering where testable, poisoned registry/entry handling, and a
  compile-time assertion of the required `AppService: Send` property;
- API-info, inspection, diagnostics, portable export, and unsupported-cancel
  JSON round trips;
- the exact `get_api_info` shape, rejection of its `contract_version`,
  Core-owned `contract_version_mismatch`, strict unknown fields for the
  envelope and every payload adapter, unknown request enums, and explicit-null
  option behavior;
- exact lossless `AppError` field/category/code/operation/context mapping;
- null request/out pointers, zero length, impossible/invalid length, malformed
  UTF-8/JSON, embedded NUL, missing/unknown operation, missing/wrong contract
  version, and wrong payload types; pointer tests stop at nullness and bounded
  representability and do not fabricate readability claims;
- invalid-handle `phoenix_call` returning status 0 plus JSON with a usable
  output argument;
- response allocation/free, null-buffer no-op, repeated allocate/free cycles,
  and correct no-leak/no-double-free caller paths under sanitizing tools where
  available;
- panic containment under the required unwind build strategy and the
  `INTERNAL_FAILURE` fallback;
- session state persisting across calls on one handle, independent state for
  multiple handles, and serialized same-handle calls.

UI0F adds Rust-side ABI/layout tests and build/export-symbol verification where
practical. UI0G starts only after this ABI and its header are compiled. It
compiles C and C++ smoke programs against the header, links the C smoke program
against the Phoenix static library, verifies external symbol/signature use,
freezes cross-language JSON fixtures, and supplies the first Swift/native smoke
harness. UI0F includes no Swift code or packaging beyond what is strictly
needed to compile/test the ABI.

## Explicit exclusions

UI0F does not include SwiftUI/AppKit UI, Swift model types or async wrappers,
security-scoped bookmarks, file pickers, progress UI, genuine cancellation,
batch export, audio recovery, DAW export, remapping, installer/notarization,
XCFramework packaging, or broad parser/profile work.

## Design gate

The dispatcher, service lifetime, JSON schema, version split, allocation,
panic, malformed-input, dependency, crate-output, and UI0G boundaries are
specified. The overall design is approved; UI0F1 is implemented below while
the C ABI remains deferred.

## UI0F1 implementation status

UI0F1 implements the safe Rust JSON transport only. Runtime `serde` with
`derive` and `serde_json` are now dependencies. Public application DTOs carry
only the serialization/deserialization directions needed by transport, with
explicit stable enum strings and explicit JSON `null` for optional response
fields. Internal parser, service-state, and compatibility-policy types remain
unserialized.

The serialization dependency graph is pinned and locked for the crate's
declared Rust 1.70 MSRV. UI0F1 compiles and its complete test suite passes with
Rust 1.70.0. Historical Clippy lint differences are not part of the MSRV
contract; the current default toolchain's Clippy with warnings denied remains
the project lint gate.

`json_transport::dispatch_json(&mut AppService, &[u8]) -> Vec<u8>` strictly
parses the dispatcher envelope and private operation payloads, dispatches only
the five documented public operations, and returns owned success, Core
`AppError`, or transport-error JSON. Core remains the version authority for
`inspect_project` and `export_sequence`; non-versioned operations reject an
envelope version. Reusing one caller-owned `AppService` preserves sessions
across dispatches.

Portable tests cover API info, inspection-to-diagnostics session persistence,
export commit/report fidelity, unsupported cancellation, Core error fidelity,
strict field/type/version shapes, invalid UTF-8/malformed JSON, explicit enum
strings, and explicit-null options. UI0F1 contains no unsafe code, C ABI symbol,
raw pointer, handle registry, synchronization, response-allocation handoff,
header, crate-type change, Swift, or UI0G work.

The remaining slices are:

- **UI0F2:** Implemented C ABI service handles, length-delimited
  request/response buffers, exact `Box<[u8]>` ownership/freeing, public header,
  and static-library output.
- **UI0F3:** Implemented ABI concurrency/lifecycle races, panic/poison
  containment, layout/symbol hardening, and the finalized C-boundary validation
  matrix.

## UI0F2 implementation status

UI0F2 implements the five version-1 C symbols in `c_abi`, process-local
monotonic service tokens, atomic call admission relative to destroy,
same-handle serialization, and quiescent destruction. Each exported operation
contains ordinary Rust unwinding; basic poisoned-lock paths return the stable
internal-failure status without unwrapping ABI-path locks. `phoenix_call`
reuses UI0F1 `dispatch_json` exactly once for valid calls and returns
exact-length Rust-owned boxed response bytes released only through
`phoenix_free_buffer`.

The version-controlled `include/phoenix.h` defines the public layouts,
statuses, ownership rules, and pointer preconditions. Cargo retains `rlib` and
adds `staticlib` output with release unwinding. Focused portable tests cover
the ABI version/layout, service and buffer lifecycle, persistent JSON sessions,
real portable export, unsupported cancellation, bounded pointer/length cases,
and C/C++ header syntax.

UI0F3 covers concurrent race/stress proof, poison and panic fault injection,
token-exhaustion injection, bounded allocator-lifecycle validation, and
conditional external symbol/layout hardening. UI0G still owns external
static-library linkage, frozen cross-language fixtures, Swift, and native-app
validation.

## UI0F3 implementation status

UI0F3 hardens the existing ABI without changing its public symbols, layouts,
JSON envelopes, or allocator contract. Private registry-parameterized helpers
let tests poison and exhaust isolated state without damaging the process-global
registry. Deterministic channel/barrier tests prove same-handle serialization,
separate-handle concurrency, active and queued call quiescence, bounded
admission/destroy races, and no post-destroy admission. Opaque session IDs now
include a per-`AppService` namespace so one handle cannot accidentally resolve
an unrelated handle's same-ordinal session.

Focused injection tests cover registry, service, and lifecycle poison plus
ordinary panics after admission, during dispatch, before publication, and in
panic-response fallback. They preserve the status/output rules, release every
admission exactly once, and leave destruction bounded. Token exhaustion,
bounded service/buffer cycles, a narrow Miri-compatible boxed-buffer path,
C/C++ compile-time layout assertions, release unwind configuration, and
conditional archive-symbol inspection complete the Rust-side hardening.

UI0G remains responsible for compiling and linking an external C consumer,
freezing cross-language fixtures, invoking the ABI from Swift, and validating
the native application boundary. UI0F3 adds no Swift, Xcode, XCFramework,
packaging, notarization, or external client.
