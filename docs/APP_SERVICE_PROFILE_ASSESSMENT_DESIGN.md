# Objective

UI0C4 defines the Core-only handoff from an inspected `AppService` session to
the compiled-in `CompatibilityRegistry`. It assesses every discovered sequence
independently, retains matched policy privately, and leaves frontend-safe
readiness projection and export to later phases.

# Scope

UI0C4A is an assessment/storage slice. It does not parse a second time, expose
profile internals, promote `Ready`, or write MIDI. The authenticated Ode profile
is one registry entry, not an application workflow or a project-wide default.

# Multi-sequence invariant

Every `SequenceId` in an inspection response is independently assessed. Core
does not search by name, inherit a profile from another sequence, or assume one
profile per session. A session can retain zero, one, or many resolved policies,
including policies from different profiles in a future registry.

# Assessment lifecycle

Assessment runs automatically after generic inspection has built one owned
`ProfileEvidence` value. The service resolves each opaque `SequenceId` to its
structural ordinal and calls the registry once for each sequence. This reuses
the existing snapshot and bounded walks rather than reparsing. Unsupported
projects remain valid inspection results; a NoMatch is not an inspection error.

# Registry ownership

`AppService` owns one immutable registry for its lifetime. Production
construction uses the built-in registry; a test constructor accepts an explicit
immutable registry. There is no global mutable state or runtime profile loading.
Registry construction failure is retained as a service configuration error and
prevents capability/readiness promotion without invalidating generic inspection.

# Session assessment state

Each private session stores an assessment record keyed by `SequenceId`:

```text
SequenceAssessment {
    structural_ordinal
    generic_readiness
    match_state
    optional_profile_capability
    optional_resolved_policy
    optional_bounded_diagnostic
}
```

`match_state` distinguishes NoMatch, Rejected, Matched, and registry/configuration
failure. The resolved policy is never keyed by display name or exposed as an app
DTO.

# Resolved policy lifetime

On a match, Core stores both profile identity/version and the immutable
`ResolvedProfilePolicy` for that exact sequence. The policy belongs only to the
in-memory session and current Core version. It is an optimization and a record
of inspection-time assessment, not permanent export authorization.

# Source identity

The session retains original path, byte size, and SHA-256. Content identity
(size plus hash, with SHA-256 authoritative) is the final revalidation authority.
Modification time or inode metadata may be diagnostic hints later, but cannot
replace content verification. No Finder alias/bookmark behavior is added in v0.

# Inspection-time trust

An inspection-time match means: “this source and sequence matched a known
profile when inspected.” It permits Core to retain a private capability/policy
candidate and, in a later projection phase, show profile readiness. It does not
authorize writing.

# Export-time revalidation

Before UI0D uses a stored policy, Core must reopen the stored authorized path,
recompute size and SHA-256, reparse the established profile, rebuild relevant
evidence, resolve the same `SequenceId` to the same structural identity, and
re-run the registry assessment. Only fresh matching evidence may enter MIDI
assembly. A changed hash/size, missing path, structural drift, or profile-version
change fails before export; stale policy is never silently used.

Missing/replaced source maps to a typed revalidation/export-validation failure
(`FileUnreadable` with `source_revalidation_failed` when the path cannot be
opened, or `ExportValidationFailed` with a bounded source-identity diagnostic
when content differs). A renamed file is not followed automatically. A fresh
inspection of an independently selected byte-identical copy may match normally.

# ProfileMatch mapping

`Matched` stores the capability and resolved policy for only the selected
sequence. `NoMatch` stores no policy and leaves generic UI0B readiness intact.
`Rejected` stores no policy, retains a bounded technical diagnostic, and leaves
the sequence conservative. Registry ambiguity/configuration failure stores no
policy and never produces readiness.

# NoMatch behavior

NoMatch is an expected enhancement result, not an `AppError`. It must not make a
recognized project disappear or create a noisy warning for every unsupported
sequence.

# Rejected behavior

An exact-provenance candidate with structural or Patch drift is distinct from
NoMatch. Core may retain a caution/diagnostic such as
`profile_track_manifest_mismatch` or `profile_patch_policy_mismatch`, without
showing expected offsets or authenticated constants in the normal UI.

# Ambiguity behavior

`AmbiguousProfiles` is a registry/configuration failure. No profile is selected,
no policy is stored, and no sequence becomes Ready. Full diagnostics retain the
bounded `profile_ambiguous_match` code; declaration order never chooses a winner.

# Readiness handoff

UI0C4A stores assessment but does not change `SequenceSummary.readiness`,
`readiness_reason`, `export_capability`, or project readiness. This gives the
assessment path an independently testable checkpoint. UI0C4C may project a
matched capability only after this state and revalidation contract are tested.

# Multi-sequence project readiness

For the eventual projection, `Ready` means every discovered sequence in the
project's assessed scope is Ready. `PartiallySupported` means at least one
sequence is Ready or PartiallySupported but not all are Ready. `Unsupported`
means no sequence is usable and the project is positively unsupported; `Unknown`
means safe classification is unavailable. Thus a project with 3 Ready, 4
Partial, and 3 Unsupported sequences is PartiallySupported. The existing
sequence array already lets the UI display per-sequence and aggregate counts;
no redundant count field is needed.

# Profile capability projection

Only the safe `ProfileCapability` (`profile_id`, version, display label) may
cross the app boundary. Structural ranges, source hash, channels, Patch policy,
and resolved handles remain private. The matched capability is attached only to
the exact `SequenceId` in the later readiness projection.

# Session policy storage

The private map is `SequenceId -> SequenceAssessment`. It supports sequence A
with no policy, B with profile X, and C with profile Y in one session. The same
textual identifier from another session cannot retrieve a policy because lookup
is scoped by the owning session.

# Idempotence

Assessment over unchanged retained evidence is synchronous, deterministic, and
idempotent. Repeated calls produce identical match states, capabilities, and
resolved policies; registry matching has no mutable state.

# Registry construction failure

The implementation should expose a fallible explicit constructor for injected
registries and a built-in construction path that maps impossible definition
failure to a typed Core configuration error. It must not silently continue with
an empty registry or use an unchecked fallback profile.

# App contract impact

The committed contract is sufficient. `SequenceSummary` already carries
readiness, reason, optional capability, and diagnostics availability; the
project summary derives its aggregate from the sequence array. No policy handle,
offset, hash, or new DTO field is required for UI0C4A.

# General-conversion precedence

Profiles are not the permanent definition of Ready. Future Core conversion
mechanisms should produce one internal, fully validated conversion capability.
When a generally proven conversion path exists, Core prefers it; otherwise an
exact compatibility profile may supply the capability. The UI sees only the
result, never the selection mechanism.

# Future batch export

The architecture preserves “Export all Ready sequences”: enumerate Ready
`SequenceId` values, submit each to Core, revalidate source/policy under a safe
transaction model, and report per-sequence outcomes. Batch behavior and output
collision policy remain outside UI0C4.

# Testing strategy

Synthetic tests should cover injected/empty/built-in registries, one matched
sequence among many, duplicate names, multiple simultaneous policies, NoMatch,
Rejected, ambiguity, idempotence, session isolation, and DTO/policy privacy.
Future revalidation tests must mutate or replace a matched source and verify
stale-policy refusal. Authentic tests should assess all sequences independently,
match only the target ordinal, and leave all other sequences unmatched without
export or readiness projection.

# Implementation decomposition

**UI0C4A — assessment storage:** own/inject the registry, assess every
`SequenceId`, retain private per-sequence outcomes, and keep readiness unchanged.

**UI0C4B — source revalidation:** implement one-shot fresh identity/structure
revalidation and stale-policy refusal, still without export.

**UI0C4C — readiness projection:** map exact matched outcomes to Ready and safe
ProfileCapability, apply the mixed-project rule, and test multi-sequence UI DTOs.

**UI0D — export:** use fresh revalidation plus the resolved policy for
transactional authenticated MIDI export.

# UI0C4 gate

The design is ready when all sequences are independently assessable, policies
are session-private and sequence-keyed, NoMatch/Rejected/Ambiguous outcomes are
distinct, source identity is authoritative, and export-time revalidation is
mandatory. Readiness remains an explicit later projection.

# Unknowns

Future decisions include exact diagnostic DTO projection, whether a refresh
operation is useful, and whether one revalidation can safely cover a batch.
Neither affects the single-sequence assessment handoff.

# Single recommended next step

Implement UI0C4A: inject/own the registry, assess every inspected `SequenceId`,
and retain private per-sequence outcomes without changing readiness or export.

# UI0C4A implementation status

The assessment/storage slice is implemented: AppService owns the built-in or
injected registry, reuses one evidence snapshot, and stores private outcomes
per SequenceId. Readiness and export remain intentionally frozen.
