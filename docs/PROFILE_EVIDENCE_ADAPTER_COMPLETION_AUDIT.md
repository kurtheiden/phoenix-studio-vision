# Objective

This audit evaluates the accumulated UI0C2 ProfileEvidence adapter work as one
Core-only bridge from an inspected AppService session to owned generic
compatibility evidence. It does not perform profile matching, readiness
promotion, or export.

# Original UI0C2 contract

UI0C2 is responsible for preserving established source and structural facts:
source provenance, parser-profile identity, deterministic sequence identity,
raw names and ranges, descriptor/pair associations, safely validated event
bounds, deterministic decoded-family inventory, and the conservative generic
Patch observations established by the bounded decoder. Routing, authenticated
profile policy, and export remain later phases.

# Completion matrix

| Requirement | Required | Implementation / tests | Status | Notes |
|---|---|---|---|---|
| Source SHA-256, size, parser profile | Yes | `AppService::profile_evidence`; app-service tests | PASS | Owned and sourced from the session. |
| Fully owned evidence | Yes | `ProfileEvidence` DTOs; ownership tests | PASS | No parser lifetime crosses the boundary. |
| Deterministic structural sequence ordinal | Yes | Structural snapshot; app-service tests | PASS | Parser order, not display name. |
| Opaque `SequenceId` to ordinal mapping | Yes | Private session map; app-service tests | PASS | Session-scoped and name-independent. |
| Sequence range and raw name/range | Yes | Structural snapshot; sequence tests | PASS | Raw bytes, not display normalization. |
| Descriptor/pair counts and ordinal associations | Yes | Structural snapshot; sequence tests | PASS | Unresolved associations are omitted. |
| Descriptor/pair ranges and raw labels | Yes | Structural snapshot; sequence tests | PASS | Fixed-width checked conversion. |
| Exact event range where established | Yes | Descriptor166 bounds helper; parser tests | PASS | Validated terminal grammar only. |
| Deterministic decoded family inventory | Yes | Existing bounded walker adapter; app-service tests | PASS | Canonical distinct-family order and logical count. |
| Conservative generic Patch observations | Yes | Shared walker extraction; app-service tests and audit | PASS | Only ordinal, range, and direct program. |
| Observed channel | No | Deliberately absent | PASS | Requires later generic or authenticated routing evidence. |
| Generic bank semantics | No | Optional Patch fields remain `None` | PASS | Not required by the UI0C2 contract. |
| Generic policy completeness assertion | No | `evidence_complete = false` | PASS | AppService does not assert authenticated policy; the matched profile supplies it. |
| Readiness/profile/export integration | No | Explicitly deferred | PASS | UI0C3/UI0D responsibilities. |

# Source provenance

The session retains the inspected source bytes, exact SHA-256, and byte size.
`profile_evidence` copies those values and records `descriptor166` only after
the established parser succeeds. Fixed-width conversions are checked.

# Ownership

All evidence strings, byte vectors, identifiers, ranges, and nested records are
owned. The service stores an owned structural snapshot; returned evidence does
not borrow request strings, parser containers, or temporary slices.

# Sequence structural identity

Sequence ordinals are assigned from parser order. The opaque session-scoped
`SequenceId` is resolved internally to that ordinal, so duplicate display names
remain representable and cannot collide.

Raw sequence name bytes and their exact parser ranges are retained separately
from the user-facing display name. Sequence and name ranges preserve half-open
semantics through checked conversion to compatibility `ByteRange`.

# Descriptor/pair evidence

Ordinal `TrackAssociations` are copied with descriptor ordinal/range, pair
ordinal, primary range, and raw descriptor label bytes. Unresolved associations
do not create guessed track rows or structural identities.

# Exact Descriptor166 event bounds

`SequenceContainer::validated_track_event_bounds` is the sole authority for
exact event bounds. It validates the established Descriptor166 seven-byte
terminal grammar (`ff ?? ?? ?? ff 2f 00`), uses checked arithmetic, supports
empty event streams, and leaves the broader containing range unchanged. The
adapter does not subtract seven itself or search heuristically. The rule is not
applied to the unsupported 120-byte profile.

# Decoded event inventory

For a validated exact range, the adapter invokes the existing bounded
mixed-event walker and requires exact range exhaustion. It records the
walker's logical event count and canonical distinct family order. Empty ranges
produce zero/empty inventory; bounds or walk failures never retain a partially
decoded prefix.

# Generic Patch evidence

The bounded Patch audit permits only facts independently supported by the
shared walker/decoder: deterministic walk-item ordinal, absolute representation
range, and direct decoded program value. Equal observations remain separate and
source ordered. Bank fields, channel, instrument identity, and translation
policy remain absent.

# Channel non-inference

`observed_channel` remains `None`. No channel is inferred from event bytes,
track labels, or Patch observations.

# Evidence-complete semantics

`TrackEvidence.evidence_complete` remains false in generic AppService output
because the service does not assert authenticated routing or policy. A matched
profile validates the explicit structural/Patch evidence and supplies its own
complete resolved policy; generic evidence alone still cannot promote readiness.

# Failure semantics

Bounds failure yields no exact range or inventory. A walk failure after valid
bounds may retain the exact range but yields zero/empty families and Patch
evidence. Unknown sessions and unavailable structural snapshots return typed
errors. No failure promotes readiness.

# Readiness isolation

Inspection readiness remains the conservative UI0B result. No generic sequence
becomes `Ready` because evidence is available.

# Compatibility-registry isolation

`AppService` does not call `CompatibilityRegistry::assess`, instantiate a
profile, or return a profile capability. UI0C2 only prepares the evidence seam.

# Export isolation

No export operation or output path is enabled. No channel, Patch translation,
serializer, or file-writing behavior was added.

# Profile-scope limitations

Exact event bounds and walking are established only for Descriptor166. The
120-byte profile and other Studio Vision versions remain unsupported or
unproven. General routing and bank semantics remain open research/profile work.

# Test coverage

Coverage includes source identity and ownership, sequence ordering and duplicate
names, checked ranges and terminal grammar, empty tracks, unresolved
associations, deterministic family/count inventory, all-or-nothing walk failure,
bounded generic Patch observations, no channel inference, unchanged readiness,
and absent export capability. Optional external Descriptor166 tests skip when
the fixture is unavailable and assert only target-independent structure.

# Remaining unknowns

Generic channel derivation, bank semantics, additional profiles, and the
authenticated Ode profile migration remain intentionally deferred. They are
not UI0C2 completion blockers.

# UI0C2 completion decision

**COMPLETE.** All responsibilities assigned to UI0C2 are implemented with
conservative evidence boundaries. The remaining unknowns are explicitly later
phase work rather than unfinished adapter obligations.

# Checkpoint readiness

The accumulated implementation, tests, and documentation are ready for one
coherent checkpoint review. No source, test, dependency, UI, FFI, or export
changes are required by this audit.

# Single recommended next step

Checkpoint the complete UI0C2 ProfileEvidence adapter implementation before
beginning UI0C3 authenticated compatibility-profile migration.

# UI0C3 handoff

The isolated profile migration may consume this evidence through the generic
registry. AppService still reports conservative UI0B readiness until a later
assessment-handoff task explicitly adds revalidation and policy selection.
