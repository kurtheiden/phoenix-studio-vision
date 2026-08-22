# Objective

UI0E stabilizes the owned Core error/report boundary and makes the v0
cancellation behavior honest before UI0F adds any transport. It does not add
serialization, an ABI, Swift code, concurrency, or asynchronous execution.

# Existing owned boundary

UI0E reuses the existing `AppError`, `Warning`, `Diagnostics`,
`InspectProjectResponse`, and `ExportSequenceResponse`. No new public report or
error DTO is required. `AppError` already owns `contract_version`, category,
user-facing display message, diagnostic technical message, operation,
optional session/sequence identity, diagnostic code, and optional diagnostic
reference.

The stable `AppErrorCategory` set is `FileUnreadable`, `NotRecognized`,
`UnsupportedProfile`, `SequenceUnsupported`, `MissingRouting`,
`UnsupportedEventFamily`, `ExportValidationFailed`, `DestinationExists`,
`OutputIoFailed`, `Cancelled`, and `InternalError`. Several categories are
reserved by the domain model and need not be emitted by every v0 operation.

# Current AppService diagnostic inventory

`ApiInfo` is currently infallible. Current `InspectProject` errors are:

| Category | Code |
|---|---|
| `InternalError` | `contract_version_mismatch` |
| `FileUnreadable` | `file_read_failed` |

`GetDiagnostics` and the existing diagnostic/revalidation helpers use:

| Category | Code |
|---|---|
| `InternalError` | `unknown_session` |
| `InternalError` | `unknown_sequence` |
| `InternalError` | `profile_evidence_unavailable` |
| `InternalError` | `profile_evidence_size_overflow` |
| `ExportValidationFailed` | `no_validated_profile_policy` |
| `FileUnreadable` | `source_revalidation_failed` |
| `ExportValidationFailed` | `source_identity_changed` |
| `ExportValidationFailed` | `source_revalidation_failed` |
| `ExportValidationFailed` | `source_sequence_identity_changed` |
| `ExportValidationFailed` | `profile_registry_configuration` |
| `ExportValidationFailed` | `profile_ambiguous_match` |
| `ExportValidationFailed` | `profile_no_longer_matches` |
| `ExportValidationFailed` | `profile_policy_changed` |

`ExportSequence` uses:

| Category | Code |
|---|---|
| `InternalError` | `contract_version_mismatch` |
| `InternalError` | `unknown_session` |
| `InternalError` | `unknown_sequence` |
| `ExportValidationFailed` | `sequence_not_export_capable` |
| `ExportValidationFailed` | `no_validated_profile_policy` |
| `FileUnreadable` | `source_revalidation_failed` |
| `ExportValidationFailed` | `source_identity_changed` |
| `ExportValidationFailed` | `source_revalidation_failed` |
| `ExportValidationFailed` | `source_sequence_identity_changed` |
| `ExportValidationFailed` | `profile_registry_configuration` |
| `ExportValidationFailed` | `profile_ambiguous_match` |
| `ExportValidationFailed` | `profile_no_longer_matches` |
| `ExportValidationFailed` | `profile_policy_changed` |
| `ExportValidationFailed` | `conversion_failed` |
| `ExportValidationFailed` | `invalid_filename_stem` |
| `OutputIoFailed` | `invalid_destination_folder` |
| `InternalError` | `export_response_overflow` |
| `OutputIoFailed` | `temporary_file_allocation_failed` |
| `OutputIoFailed` | `output_write_failed` |
| `OutputIoFailed` | `output_sync_failed` |
| `DestinationExists` | `destination_exists` |
| `DestinationExists` | `destination_name_exhausted` |
| `OutputIoFailed` | `output_commit_failed` |

Exact profile rejection currently reaches the stable `ProfileMismatchReason`
codes `profile_sequence_identity_mismatch`, `profile_track_manifest_mismatch`,
`profile_channel_policy_mismatch`, and `profile_patch_policy_mismatch` through
AppService. Registry ambiguity independently emits `profile_ambiguous_match`;
profile matching does not currently construct
`ProfileMismatchReason::AmbiguousMatch`.

The domain model also defines the reserved reason codes
`profile_hash_mismatch`, `profile_size_mismatch`, and
`profile_parser_profile_mismatch`. Current matching classifies those provenance
outcomes as `NoMatch`, which AppService reports as
`profile_no_longer_matches`, so the reserved reason codes are not emitted by
the v0 service paths. They remain defined domain codes without being claimed as
currently reachable.

The repeated `source_revalidation_failed` code is intentional: paired with
`FileUnreadable` it means the fresh source could not be read, while paired with
`ExportValidationFailed` it means fresh bytes could not be structurally
revalidated. The full category/code/operation tuple distinguishes them.

`source_revalidated` is a successful revalidation-status code, not an
`AppError` code. `missing_channel_routing`, `unsupported_project_profile`,
`meter_clocks_fallback`, `meter_thirty_seconds_fallback`, and
`temporary_cleanup_failed` are warning codes, not failed-operation codes.

# Stable code policy

`AppErrorCategory` + `diagnostic_code` + `AppOperation` is the stable
machine-readable error identity. Diagnostic codes and `Warning.code` are
lowercase snake_case strings authored explicitly by Core; consumers must not
derive them from Rust enum names, display text, or debug formatting. New codes
may be added without changing old meanings. An existing code must not silently
change category, operation, or semantic condition; a deliberate incompatible
change requires a contract revision.

`display_message` and `Warning.message` are user-facing and may receive wording
improvements without changing machine identity. `technical_message`,
`Warning.technical_detail`, diagnostics text arrays, and diagnostic references
are diagnostic-only. Consumers must not branch on their wording.

# Technical-detail policy

Technical detail may include bounded parser/converter context, hashes where a
diagnostic field already exposes them, and bounded OS error text needed to
diagnose file operations. It must not contain arbitrary source-file bytes or
make Rust type/debug names part of contract semantics. Full local paths should
not be added casually; a private temporary filename component is permitted for
the existing cleanup warning, while its user-facing message contains no path.
Existing OS/parser error rendering is diagnostic context and is not promised to
be path-redacted; UI0F must transport it as opaque text rather than interpret
or promote it into machine semantics.

UI0D3 already bounds individual filesystem error strings to 512 characters.
Other existing parser and compatibility messages are finite but do not yet
share a repository-wide numeric maximum. UI0E must preserve useful bounded
context and may centralize truncation where tests show a real unbounded path;
this design does not falsely promise a global byte limit that current code does
not enforce.

# Report consistency

Inspection, diagnostics, and export retain their existing owned DTOs and these
cross-response rules:

- warning codes follow the stable-code policy; severity and scope are explicit;
- warning order is deterministic, and `source_order` records the originating
  order within its operation;
- summary warning counts describe the warnings projected into that summary;
- `diagnostics_available` truthfully indicates whether bounded diagnostics may
  be requested, and a requested level only removes detail;
- `ProfileCapability` is exposed only from a complete Core match;
- `ValidationStatus::Validated` appears only after a successful committed
  export;
- export counts, untranslated-metadata count, warnings, profile, and actual
  committed path come from the completed export transaction;
- success with warnings remains success, not `AppError`.

# Operation and identity invariants

Errors created for `inspect_project`, `get_diagnostics`, and `export_sequence`
must carry `InspectProject`, `GetDiagnostics`, and `ExportSequence`
respectively. Diagnostic/revalidation helper errors retain their caller's
operation. The current public operation paths satisfy this invariant.

`unknown_session` remains `InternalError` / `unknown_session`. Because no
session was resolved, it carries no trusted session or sequence context.
`unknown_sequence` remains `InternalError` / `unknown_sequence` and carries the
resolved session plus supplied sequence identifier. A sequence identifier from
another session is indistinguishable from any other absent identifier and
returns `unknown_sequence`; sibling state is never substituted.

# V0 cancellation model

V0 chooses explicit unsupported cancellation. `AppService` is synchronous,
has no operation registry or cross-thread coordination, UI0D3 treats
`operation_id` as inert, and UI0F is planned as a synchronous transport. Adding
threads or pretending cancellation occurred would broaden UI0E without making
the operation safe.

UI0E's smallest future implementation exposes the reserved Core operation as
the repository-consistent equivalent of:

```rust
pub fn cancel_operation(
    &self,
    operation_id: &OperationId,
) -> Result<(), AppError>
```

It always returns `InternalError` / `cancellation_not_supported` with
`AppOperation::CancelOperation`, no session or sequence identity, and bounded
technical context stating that synchronous v0 operations have no cancellation
registry. It does not resolve the token, so an unknown token has the same
result; `no_active_operation` is not invented. No successful cancellation DTO
is needed.

`operation_id` remains an opaque reserved caller token. It provides no
cancellation, idempotency, deduplication, logging, ordering, authorization, or
response behavior. UI1 must run synchronous Core calls off the main actor,
disable conflicting actions, and await completion; it must not display a
working Cancel action in v0.

Real cancellation later requires a new contract version or an additive
negotiated capability plus defined registry ownership, thread safety, token
lifetime, cancellable operations, and checkpoints. Merely changing the v0
unsupported operation to report cancellation is not a silent implementation
detail.

# Irreversible export precedence

The UI0D3 preservation boundary is permanent: successful
`hard_link(temp, candidate)` is the irreversible commit point. No present or
future cancellation may delete, roll back, or hide that committed output.

V0 has no cancellation checkpoints. A future cooperative design may check
before temp allocation, during bounded preparation/write stages, and
immediately before publication. Cancellation observed before publication must
clean up only Phoenix's private temp artifact and return no success. Once
`hard_link` succeeds, the export completes successfully with its real
`output_path`; a concurrent or late cancellation request is ignored for that
completed operation. Post-publication temp cleanup retains the existing
success-warning behavior.

# UI0E implementation boundary

The smallest UI0E implementation is:

1. codify the existing error/category/code/operation invariants in focused
   contract tests;
2. centralize error construction only where needed to prevent identity drift;
3. add the explicit always-unsupported `cancel_operation` behavior and tests;
4. audit report/warning invariants without changing successful UI0D behavior.

UI0E adds no serializer, dispatcher, C ABI, allocator, buffer, Swift code,
thread, async runtime, cancellation registry, progress callback, or new export
feature.

# UI0F handoff

UI0F receives only the stable owned Core DTOs, successful operation responses,
and `AppError` behavior fixed by UI0E. UI0E does not decide JSON field names,
serde attributes, C symbol names, allocator ABI, response-buffer ownership, or
library/XCFramework packaging. Those remain a separate UI0F dependency and ABI
decision.

# Status and next step

UI0E is designed but not implemented. The next step is a focused read-only
review of this contract before implementing its invariant tests and explicit
unsupported cancellation operation.
