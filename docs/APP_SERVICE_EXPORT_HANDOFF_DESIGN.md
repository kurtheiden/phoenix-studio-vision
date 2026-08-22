# Objective

UI0D defines the Core-only handoff from an app-facing `Ready` sequence to one
transactional MIDI export. Inspection readiness remains advisory; export
authorization is created only by an immediate UI0C4B revalidation.

# Repository-derived contract

The committed application contract already supplies the intended final public
operation, which UI0D3 will expose after destination commit exists:

```rust
AppService::export_sequence(
    request: ExportSequenceRequest,
) -> Result<ExportSequenceResponse, AppError>
```

UI0D2 does not return `ExportSequenceResponse`: that type requires an
`output_path`, and creating one before destination commit would fabricate
success or pull UI0D3 behavior into UI0D2.

`ExportSequenceRequest` carries only `SessionId`, `SequenceId`, contract
version, destination folder/stem, collision policy, and an optional operation
identifier. It does not carry parser offsets, profile policy, source hashes, or
track manifests. `ExportSequenceResponse` owns the output path, safe profile
capability, counts, warnings, and validation status.

# Export request identity

Core resolves both the session and opaque `SequenceId`. The sequence is never
selected by display name, ordinal supplied by the caller, or profile ID. A
sequence from another session is an `unknown_sequence` failure.

# Readiness versus authorization

`Ready` means that inspection-time Core assessment found a complete explicit
conversion policy. It is useful UI guidance, not authorization. A request for
an otherwise non-Ready sequence fails with a bounded
`sequence_not_export_capable` validation error; siblings in the same project
are not affected.

# Fresh-source boundary

Immediately after request validation and before conversion, Core invokes the
UI0C4B revalidation handoff for the exact `SessionId`/`SequenceId`. It rereads
the stored path, requires exact size and SHA-256 identity, reparses, rebuilds
evidence, reassesses the same structural sequence, and compares profile identity
and resolved policy. Cached inspection bytes or policy alone can never authorize
conversion.

# Fresh ownership

UI0C4B returns an owned `FreshValidatedSequence` containing the freshly read
source bytes, fresh SHA-256, structural ordinal, and equivalent resolved policy.
The export adapter must consume those same owned bytes and policy for the rest
of the call. The request is owned by the service call, and the resulting SMF
bytes/report are owned until the destination write succeeds.

# Conversion handoff

The existing pure boundary is:

```text
fresh owned bytes + structural ordinal + resolved policy
    -> one internal Descriptor166 decode/adapter pass
    -> owned MIDI-domain sequence/tracks/events
    -> MultitrackSequenceInput
    -> assemble_multitrack_sequence
    -> complete in-memory Format 1 bytes + report
```

`assemble_multitrack_sequence` remains the sole transactional assembler. It
must not receive Studio Vision bytes, infer channels, or discover Patch policy.
The new narrow adapter supplies sequence name, initial Tempo/Meter, ordered
decoded events, authenticated channel assignments, and Patch translation policy
from the freshly validated policy. It must validate complete track coverage and
exact structural identity before assembly.

The current `FreshValidatedSequence` intentionally contains source bytes and
policy but not borrowed parser structures. UI0D implementation must add only an
owned conversion-ready internal representation or build one from those same
bytes in the same call; it must not reread the path or create a second source of
truth.

# UI0D2 prepared result

UI0D2 introduces only this crate-internal owned success boundary (field names
may follow Rust module conventions, but ownership must remain equivalent):

```rust
pub(crate) struct PreparedExportSequence {
    pub(crate) session_id: SessionId,
    pub(crate) sequence_id: SequenceId,
    pub(crate) sequence_display_name: String,
    pub(crate) compatibility_profile: ProfileCapability,
    pub(crate) result: MultitrackExportResult,
}
```

`MultitrackExportResult` already owns the complete SMF bytes and assembler
report, including track counts, event counts, warnings, and untranslated
metadata. `PreparedExportSequence` must not duplicate those report values. A
successful eligible sequence has a concrete safe capability, so the internal
field is non-optional even though the final public response retains its
versioned optional shape.

The prepared result does not retain source bytes, parser structures, resolved
policy, output path, destination state, collision state, open handles, or
`operation_id`. Those values are either consumed before assembly or remain in
the caller's request for UI0D3.

The UI0D2 operation is crate-internal:

```rust
pub(crate) fn prepare_export_sequence(
    &self,
    request: &ExportSequenceRequest,
) -> Result<PreparedExportSequence, AppError>
```

It accepts the existing full request by reference to avoid a second request
model and duplicate version/identity validation. UI0D2 reads only contract,
session, and sequence identity. It leaves `destination_folder`,
`filename_stem`, `collision_policy`, and `operation_id` untouched in the same
request for UI0D3; destination values cannot affect preparation.

# UI0D2 operation sequence

The successful UI0D2 path is exactly:

```text
request/version validation
    -> session resolution
    -> exact SequenceId resolution
    -> inspection-time Ready/capability eligibility check
    -> immediate UI0C4B revalidation
    -> UI0D1 build_conversion_ready_sequence
    -> assemble_multitrack_sequence
    -> PreparedExportSequence
```

No filesystem activity other than UI0C4B's required fresh source reread occurs;
there is no destination access, output write, or public success response.

# Export error operation identity

Every error surfaced by `prepare_export_sequence` has
`AppOperation::ExportSequence`, including errors created inside UI0C4B. The
preferred minimal implementation is to parameterize the private revalidation
implementation (and its unknown-sequence helper) with the calling
`AppOperation`. Existing revalidation callers continue to pass
`GetDiagnostics`; UI0D2 passes `ExportSequence`. This reuses one revalidation
path and its diagnostic codes without post-hoc error rewriting or duplicated
validation.

# Output boundary

The existing request/response contract includes destination folder, filename
stem, collision policy, and output path. Therefore v0 UI0D keeps filesystem
commit at the AppService boundary: assemble the complete in-memory SMF first,
then perform collision checking and an atomic destination write, and finally
return `ExportSequenceResponse`. The frontend owns file pickers,
security-scoped access, and presentation; it does not write MIDI bytes or
choose policy.

No destination file is created when revalidation, decoding, adaptation,
serialization, or collision validation fails. A future byte-only API would be a
separate contract revision, not an implicit alternate path.

UI0D3 will retain the original `ExportSequenceRequest`, call UI0D2 preparation,
then combine its untouched destination/collision fields with the returned
`PreparedExportSequence`. Only after destination validation, collision
resolution, and atomic commit of `result.smf_bytes` may UI0D3 construct the
public `ExportSequenceResponse`, including the real `output_path` and values
derived from `result.report`.

# UI0D3 destination contract

`filename_stem` is one filename component, not a path. Phoenix removes one
final case-insensitive `.mid` suffix when present, then requires the remaining
stem to be non-empty, different from `.` and `..`, and free of `/`, `\`, and
NUL. Whitespace is otherwise preserved. Phoenix does not repeatedly strip
suffixes: `Song.mid.mid` has normalized stem `Song.mid` and therefore produces
candidate `Song.mid.mid`, while `.mid` is invalid. Every candidate receives
exactly one canonical lowercase `.mid` suffix after normalization.

`destination_folder` must already exist and resolve through normal filesystem
semantics to a directory. Phoenix does not create it or canonicalize it as an
authorization mechanism. Relative and absolute paths follow normal Rust/OS
semantics. A folder symlink that resolves normally to a directory is accepted.
An existing final entry is a collision whether it is a file, directory, or
symlink; Phoenix never intentionally replaces it.

`FailIfExists` has one candidate, `<stem>.mid`. A collision returns
`DestinationExists` / `destination_exists` without modifying the entry.
`GenerateUniqueName` tries, in order, `<stem>.mid`, `<stem> 2.mid`,
`<stem> 3.mid`, through `<stem> 10000.mid`. It uses the lowest candidate that
can be committed without replacement. If all 10,000 candidates collide, it
returns `DestinationExists` / `destination_name_exhausted`. No current policy
permits overwrite.

# UI0D3 atomic no-overwrite commit

The Rust standard library has no portable rename-with-no-replace operation:
`std::fs::rename` may replace an existing destination. UI0D3 therefore uses a
same-directory hard-link publication strategy and never falls back to ordinary
rename:

1. Create a Phoenix-owned temporary file in the selected destination directory
   with `OpenOptions::create_new(true)`. Its private name combines a Phoenix
   prefix with collision-resistant process/time/counter material; allocation
   retries at most 128 names.
2. Write exactly `PreparedExportSequence.result.smf_bytes`, flush, call
   `sync_all`, and close the file.
3. Call `std::fs::hard_link(temp, candidate)`. This creates the final directory
   entry atomically and fails if any entry already occupies the candidate.
   Because both paths are in one directory, they are on the same filesystem.
   Successful hard-link publication is the irreversible UI0D3 commit point:
   the complete, closed, synced bytes are now the committed output at
   `candidate`.
4. Attempt to remove the temporary name. Phoenix never removes the committed
   candidate to compensate for failure of this private-name cleanup.

For `GenerateUniqueName`, an `AlreadyExists` result from the hard-link commit is
a collision and advances to the next candidate. For `FailIfExists`, it returns
`destination_exists`. Other hard-link failures are `output_commit_failed`;
filesystems that cannot create the required hard link are unsupported for that
attempt rather than receiving a replacement-prone fallback.

Before publication, every failure attempts to remove Phoenix's temporary
artifact. Failure of that cleanup does not replace the primary error and may be
included in bounded technical detail; no user-visible final candidate has been
created. After publication, failure to remove the temporary name does not turn
the completed export into an error. Phoenix preserves the committed candidate
unconditionally, leaves the second private hard link when it cannot remove it,
and returns the otherwise successful response with one additional
`temporary_cleanup_failed` warning. Preserving the complete recovered output
takes precedence over hiding or destroying it. `sync_all` defines the file
durability boundary; portable directory-fsync or stronger crash-consistency is
not promised.

Temporary names are private implementation details and are never returned as
`output_path`. Tests identify them by the private Phoenix prefix rather than an
exact nonce. Phoenix retries at most 128 names only when `create_new` returns
`AlreadyExists`. Any other creation error fails immediately as
`OutputIoFailed` / `temporary_file_allocation_failed`; permission, read-only
filesystem, invalid-path, and unrelated I/O errors are not retried as name
collisions.

# UI0D3 errors and response mapping

All UI0D3 errors use `AppOperation::ExportSequence` and retain session and
sequence identity when available:

| Condition | Category | Diagnostic |
|---|---|---|
| Invalid filename stem | `ExportValidationFailed` | `invalid_filename_stem` |
| Missing, unreadable, or non-directory destination folder | `OutputIoFailed` | `invalid_destination_folder` |
| `FailIfExists` collision | `DestinationExists` | `destination_exists` |
| All 10,000 unique candidates collide | `DestinationExists` | `destination_name_exhausted` |
| All 128 temporary names collide, or another temp creation error occurs | `OutputIoFailed` | `temporary_file_allocation_failed` |
| Byte write failure | `OutputIoFailed` | `output_write_failed` |
| Flush or `sync_all` failure | `OutputIoFailed` | `output_sync_failed` |
| Hard-link publication or other pre-success commit failure | `OutputIoFailed` | `output_commit_failed` |
| Response count/order cannot fit its public integer field | `InternalError` | `export_response_overflow` |

UI0D3 maps the assembler totals exactly: `notes`, `generated_note_offs`,
`controllers`, `bank_select_msb`, `bank_select_lsb`, `pitch_bend`, `tempo`, and
`meter` retain their names; internal `program_changes` becomes public
`programs`, and internal `channel_pressure` becomes public `pressure`.
Musical and total track counts use checked `usize`-to-`u32` conversion;
untranslated metadata length and warning source order use checked conversion to
their public integer types. Any overflow is checked before destination access
and returns `InternalError` / `export_response_overflow`.

Assembler warnings preserve report order and map without exposing Rust variant
names:

| Internal warning | Public code | Severity | Scope | Message/technical detail |
|---|---|---|---|---|
| `MeterClocksFallback` | `meter_clocks_fallback` | `Caution` | `Sequence` | Message: `Phoenix used the standard MIDI clocks-per-click value for this sequence.` Technical detail: `source clocks-per-click <source>; used <used>`. |
| `MeterThirtySecondsFallback` | `meter_thirty_seconds_fallback` | `Caution` | `Sequence` | Message: `Phoenix used the standard MIDI notated-32nd-notes value for this sequence.` Technical detail: `source notated 32nd notes <source>; used <used>`. |

Mapped warnings have `diagnostic_ref: None`; `source_order` is their zero-based
order in `report.warnings`. UI0D3 invents no warning for future internal
variants. A post-publication temporary cleanup failure appends this service
warning after every assembler warning:

| Public code | Severity | Scope | Message/technical detail |
|---|---|---|---|
| `temporary_cleanup_failed` | `Caution` | `Sequence` | Message: `The MIDI export succeeded, but Phoenix could not remove a private temporary filesystem entry.` Technical detail contains the bounded OS error and may include only the private filename component, not its full path. `diagnostic_ref` is `None`. |

If there are `N` assembler warnings, their source orders are `0` through
`N - 1` and the possible cleanup warning's source order is `N`. Before any
destination access, UI0D3 requires `u32::try_from(N)` to succeed. That single
check proves both every assembler index and the one additional cleanup index
are representable; failure is `InternalError` /
`export_response_overflow`. The cleanup warning changes no counts, profile,
track totals, untranslated-metadata count, output path, or validation status.

The complete operation order is:

```text
prepare_export_sequence
    -> checked response-data mapping except output_path
    -> filename and destination validation
    -> collision candidate selection/attempt
    -> atomic no-overwrite hard-link commit
    -> best-effort private temp-name cleanup
    -> actual committed UTF-8 output_path
    -> infallible ExportSequenceResponse construction
```

The destination originates as UTF-8 `String` and candidates add only the
documented ASCII suffixes, so the joined output path remains representable by
the String-based contract. A defensive conversion failure is treated as
`OutputIoFailed` / `invalid_destination_folder` before commit. `output_path` is
the actual successfully committed candidate, never a hypothetical base path.
The response uses `Some(prepared.compatibility_profile)`, mapped report counts
and warnings, the checked untranslated-metadata count, and
`ValidationStatus::Validated`. `temporary_cleanup_failed` means only that a
second private pathname could not be removed; the MIDI export itself succeeded
and `output_path` remains the committed candidate.

`operation_id` remains an opaque reserved caller token. It has no current role
in authorization, naming, collision handling, idempotency, cancellation,
logging contract, or response construction. Production writes the already
assembled bytes and performs no output reread or MIDI reparse; tests may reread
the file to verify exact identity.

# UI0D3 test ownership

Portable tests own exact-byte success and complete response mapping; actual
committed `output_path`; preparation failure before destination access;
missing, unreadable, and non-directory destinations; every invalid stem and
single-suffix `.mid` normalization; preserving an existing entry under
`FailIfExists`; deterministic lowest-gap unique naming; final-commit collision
retry; bounded unique-name and temporary-name exhaustion through private
testable helpers; injected write, sync, publication, and cleanup failures where
the injection remains below the public contract; absence of partial final
artifacts and cleanup of Phoenix temporary names in ordinary success and
pre-publication failures where cleanup succeeds; operation metadata;
warning/count mapping; and `operation_id` independence. Every successful write
need not be reread redundantly: the public successful-export regression compares
the committed MIDI byte-for-byte with the prepared SMF, collision/naming tests
independently prove destination selection and no-overwrite behavior, and the
post-publication cleanup-failure regression verifies that the preserved final
bytes remain exact.
Authentic Ode coverage remains optional and additive.

The cleanup tests distinguish the commit point explicitly: successful
publication plus successful temp removal is ordinary success without a cleanup
warning; successful publication plus injected temp-removal failure preserves
the exact final bytes, never removes the final candidate, and succeeds with
`temporary_cleanup_failed` appended at source order `N`. They also prove
response representability was preflighted before publication. Pre-publication
cleanup failure retains its primary write/sync/publication error, and a
non-`AlreadyExists` temp-creation error fails immediately without consuming all
128 attempts. One fully synced temp file is reused without rewriting or
reassembly across `GenerateUniqueName` publication collisions; candidate
selection stops at the first successful hard link.

# Failure model

Stable service outcomes are:

| Condition | Category | Diagnostic |
|---|---|---|
| Unknown session | `InternalError` | `unknown_session` |
| Unknown sequence in session | `InternalError` | `unknown_sequence` |
| No matched policy/Ready sequence | `ExportValidationFailed` | `sequence_not_export_capable` |
| Missing/unreadable source | `FileUnreadable` | `source_revalidation_failed` |
| Size or SHA-256 changed | `ExportValidationFailed` | `source_identity_changed` |
| Fresh sequence/profile no longer matches | `ExportValidationFailed` | `profile_no_longer_matches` or bounded mismatch code |
| Rejected/ambiguous registry result | `ExportValidationFailed` | profile mismatch or `profile_ambiguous_match` |
| Decode/adaptation/assembly failure | `ExportValidationFailed` | `conversion_failed` with bounded technical context |
| Destination already exists | `DestinationExists` | stable destination diagnostic |
| Filesystem write failure | `OutputIoFailed` | stable output diagnostic |

Parser and converter error details remain technical diagnostics; parser-internal
types do not cross the app contract directly.

# TOCTOU protection

The required path is:

```text
inspect -> advisory Ready
       -> user requests ExportSequence
       -> fresh source read and UI0C4B revalidation
       -> conversion from those same fresh owned bytes
       -> complete in-memory assembly
       -> collision-safe write
       -> owned export response
```

If the source disappears, is replaced, changes length/hash, structurally drifts,
or no longer matches the profile, no conversion or write occurs. The small
post-revalidation filesystem TOCTOU window is minimized by retaining the fresh
bytes and never rereading before assembly; stronger platform file handles are a
future macOS concern.

# Mixed-project semantics

Authorization is strictly sequence-scoped. A project with one Ready, one
PartiallySupported, and one Unsupported sequence can export only the requested
Ready `SequenceId`. Requesting either sibling fails without changing its summary
or the session's other policies. Batch export is outside UI0D.

# Policy constraints

The resolved compatibility policy supplies every track channel and authenticated
Patch translation. The adapter must not derive channels, invent Patch defaults,
or accept incomplete/extra rows. Generic parser facts and profile policy remain
separate layers.

# Proposed implementation slices

1. **UI0D1 — conversion-ready handoff:** define an owned internal representation
   that is produced from `FreshValidatedSequence` bytes and policy and contains
   the exact decoded values required by `MultitrackSequenceInput`.
2. **UI0D2 — Core export preparation:** add crate-internal
   `prepare_export_sequence`, validate request identity/readiness, invoke
   UI0C4B, adapt the fresh sequence, call `assemble_multitrack_sequence`
   transactionally, and return `PreparedExportSequence`.
3. **UI0D3 — public destination commit/report:** expose `export_sequence`, call
   UI0D2, implement collision policy and atomic filesystem writing, map output
   errors, and construct `ExportSequenceResponse` with the committed path.

No slice changes the parser grammar, generic compatibility policy, readiness
projection, or revalidation guarantees.

# Required implementation tests

UI0D2 tests own contract/session/sequence validation, non-Ready and sibling
isolation, stale mutation and source disappearance, fresh profile mismatch,
conversion/assembly error mapping, `ExportSequence` operation metadata, exact
prepared-result identity/capability/report propagation, deterministic in-memory
SMF bytes where fixtures permit, and proof that destination fields cause no
filesystem access.

UI0D3 tests own destination validation, collision behavior and unique naming,
transactional no-partial-output guarantees, atomic write failures, final report
mapping, and the real `output_path` in `ExportSequenceResponse`. These concerns
are not duplicated in UI0D2 tests.

# Explicit exclusions

UI0D does not include Swift/SwiftUI/AppKit or FFI transport, save dialogs/Finder
integration, batch export, audio recovery, DAW-project generation, instrument
remapping, additional compatibility profiles, general channel derivation, or
speculative parser support.

# Current design status

UI0D1, UI0D2, and UI0D3 are implemented. Destination output remains restricted
to the designed single-sequence, compatibility-profile-gated operation.

# Single recommended next step

Review the designed UI0E stable error/report and explicit unsupported v0
cancellation contract. Do not reopen UI0D3 transaction semantics or broaden
UI0D beyond single-sequence MIDI export.
