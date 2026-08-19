# Objective

Define a stable, owned, versioned Phoenix Core application-service contract
for a future native macOS client. The contract is operation-oriented and
platform-neutral; it does not expose Studio Vision parser structures, borrowed
bytes, serializer primitives, or authenticated Ode policy.

# Architectural boundary

Core internals parse, decode, assess, validate, and export. The future app
service owns the inspected-file/session lifecycle, maps internal results into
owned DTOs, selects compatibility policy, maps errors, and coordinates safe
transactional file operations. Swift presents those DTOs and never interprets
SVP bytes or chooses MIDI channels/Patch mappings.

The first service module can be conceptually `app_service`; this design does
not add it yet.

# Contract versioning

Use an integer `contract_version`, initially `1`, returned by `GetApiInfo` and
included in every serialized response envelope. Also include a `schema_version`
per DTO family when a family may evolve independently.

The frontend must reject a major contract version it does not understand and
show an actionable incompatibility error. Additive optional fields and new
reason/warning enum values are compatible when unknown values are preserved as
`unknown(code, payload)` and do not change existing meanings. Removing,
renaming, changing field meaning, changing ownership/encoding, or altering
transaction/error semantics requires a new contract version. No plugin-style
negotiation is needed for v0.

# Project/session identity

Choose **B: an opaque Core-issued session handle**. `InspectProject` reads and
hashes the selected file, owns the resulting parsed/assessment state, and
returns a random-looking `SessionId` scoped to that Core instance. A later
export carries the session and sequence IDs; Core revalidates the source
identity before exporting. A changed/replaced file invalidates the session
rather than silently exporting stale borrowed data.

The app holds the security-scoped URL only while Core performs the operation;
Core stores no borrowed reference to it. Session expiry/disposal can be
automatic after a bounded idle period; explicit disposal is optional for v0.
Multiple windows can use independent sessions later.

# Inspect operation

Conceptual request:

```text
InspectProjectRequest {
  contract_version
  source_path: UTF-8 path supplied after user authorization
  diagnostics: None | Summary | Full
}
```

Conceptual response:

```text
InspectProjectResponse {
  contract_version
  session_id
  project: ProjectSummary
  sequences: [SequenceSummary]
  warnings: [Warning]
  diagnostics_available: bool
}
```

The path is an operation capability, not a durable project identity. Core
returns owned UTF-8 strings and arrays in deterministic order.

# ProjectSummary

Required owned fields:

```text
ProjectSummary {
  display_name
  byte_size
  identification: IdentificationSummary
  recognized_studio_vision: bool
  profile_label: optional string
  sequence_count
  overall_readiness: Readiness
  warning_count
  diagnostics_available: bool
}
```

SHA-256, Finder evidence, and internal profile details belong in diagnostics,
not the primary summary.

# SequenceId

Use a Core-issued opaque UTF-8 token (or fixed byte token in a binary ABI),
unique within a session and never derived from sequence name or raw offset.
It is valid only with its `SessionId` and for that inspection lifetime. Core
can map it internally to parser structures and revalidate before export.

# SequenceSummary

```text
SequenceSummary {
  sequence_id
  display_name
  readiness
  readiness_reason: ReadinessReason
  musical_track_count: optional integer
  supported_event_families: [EventFamilySummary]
  warning_count
  export_capability: None | ProfileCapability
  diagnostics_available: bool
}
```

No offsets, descriptor ordinals, pair identities, raw Patch context, or channel
assignments cross this boundary.

# Readiness

Use exactly four stable values:

- `READY`: Core has a complete explicit policy and can export transactionally
  without known silent musical loss.
- `PARTIALLY_SUPPORTED`: Core recognizes the sequence but known content or
  routing cannot be exported completely; Export is disabled.
- `UNSUPPORTED`: Core positively identifies a required unsupported profile,
  structure, or family; Export is disabled.
- `UNKNOWN`: Core cannot safely classify the sequence or completeness.

Operation failures such as unreadable input are errors, not readiness values.
Only Core assigns readiness; Swift never derives it from warning text.

# Reason codes

Reason codes are stable machine values with severity, export-enabled flag,
display detail, and optional diagnostic reference. Initial codes:

| Code | Meaning | Export |
|---|---|---|
| `ValidatedCompatibilityProfile` | Complete provenance-locked policy matched | enabled |
| `MissingChannelRouting` | Required channel/routing is unknown | disabled |
| `UnsupportedEventFamily` | Required event family has no safe translation | disabled |
| `UnsupportedPatchTranslation` | Patch cannot be classified safely | disabled |
| `UnsupportedProjectProfile` | Profile is outside supported parser contract | disabled |
| `IncompleteSequenceStructure` | Structural association/bounds are unresolved | disabled |
| `UnknownStructure` | Core cannot classify safely | disabled |

Unknown future codes must render as a generic unsupported/unknown explanation.

# Warnings

```text
Warning {
  code
  message
  technical_detail: optional string
  scope: Project | Sequence | GenericTrack
  severity: Informational | Caution | DataLossRisk
  diagnostic_ref: optional string
}
```

Warnings are deterministically ordered by scope, code, then source order. A
`DataLossRisk` warning is never compatible with `READY` for an export that
would silently omit musical content. Informational metadata differences may
coexist with READY when the policy explicitly preserves musical equivalence.

# Diagnostics

Diagnostics are requested separately or returned through a bounded reference,
not embedded as a huge default payload:

```text
Diagnostics {
  core_version
  contract_version
  source_sha256
  identification_evidence
  recognized_profile
  structural_status
  unsupported_families
  compatibility_profile_id/version
  technical_errors
  export_report: optional ExportReport
}
```

Core should later provide `GetDiagnostics(session_id, level)` and a deterministic
copyable text/JSON rendering. It must not include arbitrary raw file content or
unrelated local paths.

# Compatibility profiles

Core owns an isolated registry:

```text
CompatibilityProfile {
  profile_id
  profile_version
  label
  matcher: Core-only provenance/structure predicate
  export_policy: Core-only decoded-event policy
}
```

Matching requires complete project/sequence/track evidence. Filename or name
alone never matches. A mismatch returns a non-ready assessment; there is no
fallback to guessed routing. The authenticated Ode profile remains a proof
policy outside generic parser modules and is not implemented in UI0 production
code. Future general routing can replace the profile at this seam.

# Export request

```text
ExportSequenceRequest {
  contract_version
  session_id
  sequence_id
  destination_folder: UTF-8 path supplied after user authorization
  filename_stem: optional UTF-8 string
  collision_policy
  operation_id: optional caller token
}
```

The frontend supplies no channel, Patch, Tempo/Meter, event-range, PPQN, or
profile policy. Core chooses and validates the applicable profile.

# Collision policy

Support `FAIL_IF_EXISTS` and `GENERATE_UNIQUE_NAME` in v0. Use the first as the
default for predictable preservation; the app may offer the second explicitly.
`REPLACE_WITH_CONFIRMATION_TOKEN` is deferred. Core returns the actual committed
path and never overwrites silently.

# Export response

```text
ExportSequenceResponse {
  session_id
  sequence_id
  sequence_display_name
  output_path
  compatibility_profile_id/version
  musical_track_count
  total_smf_track_count
  counts: { notes, generated_note_offs, controllers, bank_select_msb,
            bank_select_lsb, programs, pressure, pitch_bend, tempo, meter }
  warnings
  untranslated_metadata_count
  validation_status: Validated
}
```

Returning this response means the transaction completed. Partial bytes are
never success. Future operation responses can add `sequences_attempted`,
`sequences_exported`, per-sequence results, and failures without changing the
single-sequence shape.

# Future multi-sequence reporting

Reserve an additive `BatchExportSummary` with attempted/exported sequence IDs,
per-sequence success/error records, and optional audio references. UI0 does not
implement batch behavior.

# Audio-reference seam

Reserve, but leave empty, an additive record:

```text
AudioReferenceSummary { display_name, path_hint, status, provenance_confidence }
```

Core must not fabricate it. The UI must not show resurrection/audio language
until real evidence exists.

# Error envelope

Every failed operation returns:

```text
AppError {
  contract_version
  category
  display_message
  technical_message
  operation
  session_id: optional
  sequence_id: optional
  diagnostic_code
  diagnostic_ref: optional
}
```

Stable categories are `FileUnreadable`, `NotRecognized`, `UnsupportedProfile`,
`SequenceUnsupported`, `MissingRouting`, `UnsupportedEventFamily`,
`ExportValidationFailed`, `DestinationExists`, `OutputIoFailed`, `Cancelled`,
and `InternalError`. Success with warnings is distinct from an operation error.

# FFI transport

Choose **B: JSON payloads over a tiny C ABI** for v0. The ABI can expose
`api_info`, `call(request_json, response_out)`, and `free_buffer`, with one
UTF-8 JSON envelope per operation. JSON is simple to inspect from Swift,
debuggable in fixtures, naturally additive, and keeps Rust ownership behind a
small boundary. It has no performance concern for project summaries/reports.

This will likely require adding a serialization dependency (for example
`serde`/`serde_json`) when UI0 implementation begins; this design task adds
none. If dependency policy rejects that later, retain the owned domain DTOs and
replace only the transport encoding.

# Memory ownership

Rust allocates each response buffer and returns pointer plus length. The caller
must invoke `free_buffer(pointer, length, allocator_version)` exactly once;
Rust owns deallocation. Responses are valid UTF-8 JSON, contain no interior NUL
requirement, and are length-delimited rather than C-string terminated. Request
bytes are caller-owned for the call. No borrowed Rust pointer crosses a call.

# macOS sandbox/file ownership

For v0 choose **A: authorized POSIX path strings**, not file descriptors. Swift
uses `fileImporter`/`NSOpenPanel` or a drop URL, starts security-scoped access,
passes the path to a synchronous Core operation, and stops access after return.
The destination folder is selected similarly. Core performs final collision
checking and atomic writing while access is active, returning the committed
path. No remembered bookmarks are needed in v0. A future handle-based API can
be added if sandbox coordination proves insufficient.

# Operations

Keep the initial surface coarse:

- `GetApiInfo`
- `InspectProject`
- `GetDiagnostics`
- `ExportSequence`
- `CancelOperation` (reserved seam; initially may return unsupported)

No explicit session-dispose call is required initially; Core can expire
sessions and release memory after export or idle timeout.

# Sync/async model

Use a synchronous Rust service call initially. Swift invokes it from `Task` or
a background actor, never the main actor. This keeps the ABI and ownership
simple while preserving a future asynchronous wrapper. Core returns only after
success, failure, or cancellation checkpoint.

# Progress

Reserve an optional stage field in operation status, with values `Reading`,
`Hashing`, `Identifying`, `Parsing`, `Assessing`, `Exporting`, `Validating`, and
`Writing`. UI0 does not require callbacks or percentages; stage reporting can
be added in a later ABI version or polled operation API.

# Cancellation

Accept an optional caller-generated `operation_id` and cooperative cancellation
registry in the domain design, but do not require a callback ABI in v0. Swift
can cancel its Task before/after a synchronous call; Core must check the token
at future bounded stages before committing output. Cancellation never returns
partial success.

# Thread safety

Make the service instance session-aware but serialized per session. Independent
sessions are safe on separate worker instances; one session cannot be exported
concurrently. This is simple for one window and leaves multi-window ownership
explicit rather than promising every parser object is Send/Sync.

# Deterministic behavior

Enums have stable snake-case strings plus numeric codes. Sequence order follows
Core structural order; warning and report arrays are deterministic. All text is
UTF-8. Absent optional values are omitted/null according to one schema rule,
never represented by magic empty strings. JSON readers ignore unknown additive
fields and preserve unknown enum values as `unknown(code)`.

# Security and privacy

All processing is local. Core uploads nothing, emits no telemetry, and avoids
unrelated local paths in errors/diagnostics. Source bytes remain in the chosen
file scope. Diagnostics are user-requested and copyable, not automatically
sent anywhere.

# Testing strategy

Future service tests should include:

- synthetic API/version, enum, warning/error, ownership, deterministic-order,
  malformed-request, destination-collision, and response-schema tests;
- authentic inspection, Ode-profile readiness, mismatch-disabled-export, and
  successful report integration tests;
- FFI round trips for request/response bytes, version mismatch, malformed JSON,
  UTF-8, buffer freeing, and allocator ownership.

No tests are added in UI0 design.

# Implementation decomposition

1. **UI0A:** owned domain DTOs/enums and deterministic serialization-independent
   tests.
2. **UI0B:** application-service inspection, session store, readiness, and
   diagnostics mapping.
3. **UI0C:** Core-only compatibility-profile registry seam.
4. **UI0D:** path-based transactional export service and collision policy.
5. **UI0E:** stable error/report mapping and cooperative cancellation seam.
6. **UI0F:** JSON transport plus tiny C ABI and explicit allocator functions.
7. **UI0G:** contract/FFI fixtures and a future Swift smoke harness.

# Implementation gate

| Area | Result |
|---|---|
| A. Owned DTO model | YES |
| B. Project/session identity | YES |
| C. Sequence identity | YES |
| D. Readiness model | YES |
| E. Compatibility-profile isolation | YES |
| F. Export request/report | YES |
| G. Error model | YES |
| H. Diagnostics model | YES |
| I. FFI transport recommendation | YES |
| J. macOS path/sandbox ownership | YES |
| K. Cancellation/progress seam | YES |
| L. UI0 implementation-ready | PARTIAL |

UI0 is not implementation-ready until the repository chooses the exact owned
DTO Rust module and confirms the C ABI/JSON serialization dependency policy.
That is one bounded implementation decision, not a parser/export blocker.

# Unknowns

- Exact DTO Rust module names and serialization crate approval.
- C ABI symbol naming, library packaging, and Swift binding generation.
- Minimum supported macOS/Xcode versions and sandbox entitlements.
- Whether Core or a host process owns all destination writes in a signed app.
- Session expiry limits and future multi-window scheduling.

# UI0A/UI0B status

UI0A is implemented as owned Rust DTOs and deterministic contract tests.
UI0B now provides the synchronous `AppService` inspection/session layer: it
reads one path, hashes and identifies the input using existing helpers,
structurally parses only the established `Descriptor166` profile, enumerates
owned sequence summaries, and returns bounded diagnostics. No generic sequence
is marked Ready because routing and compatibility profiles remain unresolved.

# Single recommended next step

Design UI0C's Core-only compatibility-profile registry seam without embedding
authenticated Ode constants in generic service or contract modules.

# UI0C registry seam

The planned registry is Core-only and declarative. It matches exact project
provenance plus internal structural sequence/track evidence, then returns only
the existing `ProfileCapability` and validated readiness through the app
contract. Channel assignments, Patch translations, ranges, and resolved policy
handles remain private. A mismatch never falls back to a guessed policy, and
export-time revalidation is mandatory.

# UI0C2 evidence boundary

The application service may retain an internal structural snapshot and produce
generic compatibility evidence for Core. That evidence is not an app DTO,
does not expose offsets or channels to the frontend, and remains separate from
registry assessment and readiness.

# UI0C2 completion status

The evidence bridge is now complete for its established scope. Descriptor166
validated bounds, deterministic bounded-walker inventory, and the conservative
generic Patch observation subset are available to Core; routing, bank policy,
profile assessment, and export remain separate future operations.

# UI0C4 assessment handoff

The next service seam assesses every discovered `SequenceId` independently and
retains any resolved profile policy privately. No profile internals cross the
contract, and fresh source identity validation remains mandatory before export.
