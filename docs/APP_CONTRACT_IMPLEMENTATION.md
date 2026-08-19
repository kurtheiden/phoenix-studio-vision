# Objective

UI0A implements the owned, transport-independent Rust domain model described
by the application-service contract. It does not inspect files, hold sessions,
match profiles, export MIDI, write paths, or cross an FFI boundary.

# Scope

The module contains owned API/version, opaque identifiers, readiness/reasons,
warnings, identification/project/sequence summaries, event-family summaries,
profile capability, inspect/export requests and responses, counts, diagnostics,
future audio references, collision/validation values, and application errors.

# Module

`src/app_contract.rs` is exposed as `phoenix::app_contract`. It imports no
Studio Vision parser, mixed-event, adapter, serializer, or proof module.

# Contract version

`CONTRACT_VERSION` is the deliberate value `1`. Public cross-boundary enums
provide explicit stable numeric codes and names; no Rust discriminant or Debug
format is treated as ABI. Transport-level unknown-value handling is deferred to
UI0F.

# Owned identity types

`SessionId`, `SequenceId`, and `OperationId` are owned `String` newtypes with
constructors, borrowed accessors, equality/order/hash traits, and no parser
offset semantics. UI0A does not generate IDs; UI0B will issue session-scoped
values.

# Readiness and reason codes

`Readiness` has exactly Ready, PartiallySupported, Unsupported, and Unknown.
`ReadinessReasonCode` and owned `ReadinessReason` carry deliberate stable code,
severity, export-enabled policy, display detail, and optional diagnostic
reference. Only the validated compatibility-profile reason enables export.

# Warnings

`Warning` owns code/message/technical detail/diagnostic reference, scope,
severity, and explicit source order. `compare_warnings` ranks scope, code, then
source order; message text is not semantic ordering.

# Project/sequence summaries

`IdentificationSummary`, `ProjectSummary`, `EventFamilySummary`,
`ProfileCapability`, and `SequenceSummary` are owned app-facing values. Counts
and file sizes use fixed-width `u32`/`u64`; sequence order is supplied by Core
and is not sorted by this DTO layer. Duplicate display names are valid.

# Requests/responses

`InspectProjectRequest/Response` and `ExportSequenceRequest/Response` contain
owned UTF-8 paths, opaque IDs, policy values, summaries, reports, and warnings.
Export requests contain no channel, Patch, event-range, or timing fields.
Response success uses `ValidationStatus::Validated` and conceptually means a
complete transaction, although UI0A performs no operation.

# Export counts

`ExportCounts` uses explicit `u64` fields for Notes, generated Note Offs,
Controllers, bank-select MSB/LSB, Programs, Pressure, Bend, Tempo, and Meter.
`checked_add` returns `CountOverflow` rather than wrapping.

# Diagnostics

`Diagnostics` carries bounded owned strings/lists for Core/contract version,
hash, identification evidence, profile, structural status, unsupported
families, technical errors, and an optional export response. It contains no
raw bytes or borrowed parser values.

# Errors

`AppErrorCategory` and `AppOperation` implement deliberate stable code/name
helpers. `AppError` carries display and technical text, operation, optional
session/sequence context, category, and diagnostic identifiers. UI0E will map
internal Phoenix errors into this envelope.

# Future audio seam

`AudioReferenceSummary` reserves display name, optional path hint, status, and
provenance confidence. UI0A creates no recovered audio values and the UI must
not display audio claims from an empty seam.

# Stable codes/names

Readiness, reason, severity, scope, event-family, diagnostics-level, collision,
validation, error-category, and operation enums expose explicit helpers where
they may cross a future ABI. The transport will add unknown additive enum
handling later; the Rust domain stays strongly typed here.

# Deterministic ordering

Sequence arrays preserve caller/Core structural order. Warning ordering is
explicitly testable through `compare_warnings`; it uses scope rank, code, and
source order, never display message text.

# Ownership guarantees

Every field crossing the conceptual application boundary is owned. Tests build
DTOs from temporary strings/vectors, return them, and use them after the source
locals are gone. No unsafe code or borrowed lifetime appears in the module.

# Transport independence

No serde, JSON, FFI, CBOR, MessagePack, or other transport dependency was
added. UI0F may later encode these DTOs as JSON over a small C ABI after a
separate dependency/ABI decision.

# Explicit exclusions

No file inspection, session store, compatibility matcher, channel/Patch policy,
MIDI serialization, export I/O, cancellation registry, progress callbacks,
Swift/Xcode code, CLI behavior, or Ode-specific constant exists here.

# Tests

`tests/app_contract.rs` covers version/API info, opaque IDs, stable readiness and
reason behavior, warning order, owned summaries, duplicate names, requests and
responses, collision codes, checked count aggregation, diagnostics/errors,
future audio placeholders, stable enum identities, and ownership lifetime
proofs.

# UI0A gate

PASS: all DTOs are owned, versioned, deterministic, strongly typed, transport
independent, parser-free, and free of authenticated target constants. Existing
tests remain green; no Cargo dependency changed.

# Single recommended next step

Implement UI0B application-service inspection/session/readiness mapping without
adding FFI, serialization dependencies, or UI code.
