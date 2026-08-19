# Objective

UI0C2C records factual decoded event-family inventory for safely associated
Descriptor166 tracks. It does not infer routing, Patch policy, readiness, or
export capability.

# Scope

Only the established Descriptor166 profile is eligible. Other profiles retain
conservative incomplete evidence.

# Input authority

The adapter consumes parser-owned structural snapshots and source bytes held by
the AppService session. It does not reinterpret Studio Vision bytes.

# Exact-range requirement

Each track must first pass `validated_track_event_bounds`. The broader
`event_containing_range` is never used as a fallback.

# Existing walker reuse

The adapter invokes `walk_bounded_mixed_events` with the validated absolute
event range and the existing zero timing basis. It requires exact consumed-range
equality and retains no partial walk on failure.

# Supported decoded families

The inventory maps existing walker variants to `EvidenceEventFamily`: Note,
Patch, Controller, Channel Pressure, and Pitch Bend. Context-mediated notes are
Notes; a Patch-to-Note transition records both Patch and Note facts. No new
event grammar or family is introduced.

# Deterministic family ordering

Families are emitted once each in Phoenix's canonical order: Patch, Note,
Controller, Channel Pressure, Pitch Bend. Ordering does not depend on source
event order or hash iteration.

# Logical event count

`decoded_event_count` is `MixedEventWalk::logical_event_count()`, including both
logical events in a Patch-to-Note transition. Raw bytes, tail bytes, and future
MIDI messages are not counted.

# Empty tracks

An exact empty range yields a successful empty walk, count zero, and no family
entries.

# Failure semantics

Bounds failure leaves the exact range unavailable. Walker failure may preserve a
validated exact range, but inventory is empty and count zero. No successfully
decoded prefix is reported.

# Patch-family versus Patch-evidence distinction

The presence of a decoded Patch is recorded only as a family fact. `patch_evidence`
remains empty; no bank semantics, translation, or compatibility policy is
derived.

UI0C2D now permits only the audit-approved Patch subset: bounded source range,
walk-item ordinal, and direct decoded program value. Bank fields remain absent.

# Channel non-inference

No event family populates `observed_channel`; it remains `None`.

# Evidence completeness

`evidence_complete` remains false because generic Patch evidence and routing are
not established. Exact bounds and family inventory alone cannot produce a
profile match.

# Synthetic tests

The existing bounded-walker synthetic suite remains authoritative for event
grammar, including empty ranges, repeated families, Patch transitions, and
failure transactionality. The AppService mapping preserves those semantics.

# Authentic regression

The optional Experiment 007 AppService test confirms at least one successful
non-empty inventory, exact ranges, conservative Patch/channel fields, unchanged
readiness, and no export capability. It does not assert Ode-specific policy.

# Compatibility/readiness isolation

The compatibility matcher and UI0B readiness mapping are unchanged. Incomplete
evidence cannot yield a capability, and the registry is not called.

# Broader-profile limitation

No 120-byte or arbitrary-profile inventory is attempted.

# Explicit exclusions

No channel inference, Patch translation, profile migration, export, UI, FFI,
serialization, dependency, or decoder grammar changes are part of UI0C2C.

# UI0C2C gate

The gate passes when exact bounds feed the existing walker, counts and canonical
families are deterministic, failures are all-or-nothing, and readiness and
policy remain conservative.

# Historical next step (completed)

Populate generic Patch evidence only from already-decoded Patch provenance while
continuing to leave channel routing and compatibility assessment isolated.

# Current status

The bounded Patch evidence slice is now complete and is covered by the UI0C2
completion audit; this document's remaining exclusions are intentional.
