# Objective

UI0D defines the Core-only handoff from an app-facing `Ready` sequence to one
transactional MIDI export. Inspection readiness remains advisory; export
authorization is created only by an immediate UI0C4B revalidation.

# Repository-derived contract

The committed application contract already supplies the intended operation:

```rust
AppService::export_sequence(
    request: ExportSequenceRequest,
) -> Result<ExportSequenceResponse, AppError>
```

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
2. **UI0D2 — Core export orchestration:** add `export_sequence`, validate request
   identity/readiness, invoke UI0C4B, adapt the fresh sequence, and call
   `assemble_multitrack_sequence` transactionally.
3. **UI0D3 — destination commit/report:** implement collision policy, atomic
   filesystem writing, output error mapping, and `ExportSequenceResponse`.

No slice changes the parser grammar, generic compatibility policy, readiness
projection, or revalidation guarantees.

# Required implementation tests

The later implementation must cover successful Ready-sequence handoff,
stale/same-size mutation refusal, source disappearance, wrong session/sequence,
non-Ready and sibling isolation, fresh profile mismatch, exact conversion-input
propagation, deterministic SMF bytes where existing fixtures permit it,
transactional failure with no partial output, collision behavior, and output
write errors.

# Explicit exclusions

UI0D does not include Swift/SwiftUI/AppKit or FFI transport, save dialogs/Finder
integration, batch export, audio recovery, DAW-project generation, instrument
remapping, additional compatibility profiles, general channel derivation, or
speculative parser support.

# Current design status

UI0D is designed only. No export operation, file writer, serializer call, or
application-contract change is implemented by this document.

# Single recommended next step

Implement UI0D1: build the owned conversion-ready handoff from UI0C4B's fresh
validated bytes and resolved policy without rereading the source.
