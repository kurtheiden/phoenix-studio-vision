# Objective

UI0C4A adds internal per-sequence compatibility assessment storage to
`AppService`. It consumes one retained UI0C2 `ProfileEvidence` snapshot and
does not project readiness or perform export.

# Scope

The service now owns an immutable registry and automatically assesses every
discovered `SequenceId` after successful generic inspection. Profile internals
remain Core-only.

# Registry ownership

`AppService::new` uses the built-in compiled registry. `AppService::with_registry`
provides deterministic synthetic injection without global mutable state.

# Registry construction failure

Built-in construction is handled deliberately: if the fallible constructor ever
fails, generic inspection continues with a registry-error assessment for each
sequence and no capability or policy. The service does not silently substitute
an empty or guessed profile.

# Assessment lifecycle

After the session is stored, one owned `ProfileEvidence` value is built and
reused for every sequence ordinal. No second parse or source read occurs.
`CompatibilityRegistry::assess` is called independently for each retained
opaque `SequenceId` mapping.

# Multi-sequence behavior

Assessments are sequence-local. The current authentic project produces one
matched Ode sequence and non-matched states for all others; future profiles can
produce multiple independent matches in one session.

# Private assessment state

Each session stores a private `SequenceAssessment` containing structural ordinal,
generic readiness snapshot, match kind, optional safe capability, optional
immutable resolved policy, and bounded diagnostic fields. Only a safe
`SequenceAssessmentStatus` summary is exposed for Core tests/future service
layers; policy contents remain private.

# Session map

The map key is the session-scoped opaque `SequenceId`. Display names, profile IDs,
and ordinals alone cannot retrieve a policy. A `resolved_policy_for_sequence`
`pub(crate)` seam is reserved for later export/revalidation code.

# NoMatch

NoMatch stores no capability or policy, is not an error, adds no warning, and
leaves all UI0B readiness and export fields unchanged.

# Matched

Matched stores the safe `ProfileCapability` and complete immutable
`ResolvedProfilePolicy` only for that sequence. Nothing is copied into
`SequenceSummary` during UI0C4A.

# Rejected

Rejected stores no policy or capability and retains the bounded mismatch code
and technical detail internally. Generic readiness remains unchanged.

# Registry error

Ambiguity or built-in configuration failure stores `RegistryError`, no policy,
and no capability. Inspection remains available and no sequence becomes Ready.

# Resolved policy lifetime

Policies live only in the in-memory session and current Core version. They are
inspection-time assessment results, not export authorization. UI0C4B must fresh-
revalidate before any future use.

# SequenceId/session isolation

Status and policy lookup require both `SessionId` and `SequenceId`. Unknown
sessions and unknown sequence IDs return the existing deterministic typed errors;
identical identifier text in another session cannot cross the boundary.

# Idempotence

Assessment is synchronous, deterministic, and based on immutable retained
evidence and registry data. Repeated inspection assessment produces equivalent
status and policy values.

# Authentic regression

Optional authentic tests inspect all discovered sequences, verify exactly one
current built-in match, verify all other sequences are non-matched, and confirm
the target policy is private while the public summary remains conservative.
An injected empty registry confirms every sequence becomes NoMatch.

# Readiness projection

UI0C4A assessment remains private until UI0C4C. The projection now maps only a
complete matched assessment to `Ready`, `ValidatedCompatibilityProfile`, and a
safe `ProfileCapability`; NoMatch, Rejected, and RegistryError retain generic
readiness. Fresh UI0C4B revalidation remains mandatory before export.

# Export isolation

No source revalidation, serializer call, MIDI write, destination handling, or
export operation was added.

# Source-revalidation handoff

The existing source path, size, and SHA-256 remain retained. UI0C4B must reopen,
rehash, reparse, remap, and reassess before using any stored policy.

# Target-specific isolation

AppService contains no Ode names, hashes, offsets, channels, or Patch policy.
Only `compatibility_profiles` owns target-specific constants.

# Tests

The authentic AppService regression covers independent sequence status, exact
single-profile matching, readiness freeze, policy privacy, and empty-registry
injection. Existing registry tests continue to cover matching and policy
semantics.

# Explicit exclusions

No app-contract change, export implementation, revalidation change, parser
change, dependency, UI, FFI, serialization, or channel inference is part of
UI0C4A/UI0C4C.

# UI0C4A gate

The gate passes when every assessable sequence receives private independent
state, matched policies are sequence-keyed, NoMatch/Rejected/RegistryError are
distinct, authentic assessment matches only the target sequence, and all
app-facing readiness/export behavior remains frozen.

# Single recommended next step

Design the UI0D export handoff around immediate UI0C4B revalidation and fresh
owned source bytes.

# UI0C4B implementation status

AppService now provides an explicit Core-only revalidation operation. It
rereads the stored source path, requires exact size/SHA-256 identity, rebuilds
Descriptor166 evidence, reruns the registry for the retained structural
ordinal, and compares the fresh policy with the inspection policy. Missing,
mutated, structurally changed, unmatched, ambiguous, or policy-different
sources are refused; readiness and export remain frozen.
