# Objective

UI0C4C projects the private UI0C4A per-sequence compatibility assessment into
the existing owned application DTOs. It does not perform revalidation or
export.

# Authoritative state

Projection consumes each session's `SequenceAssessment`, keyed by opaque
`SequenceId`. It does not inspect names, parser offsets, or profile constants.

# Matched sequence

Only an assessment with `Matched`, a capability, and a resolved policy projects
to `Readiness::Ready`, `ReadinessReasonCode::ValidatedCompatibilityProfile`,
and the safe `ProfileCapability`. The policy itself remains Core-private.

# Non-matched sequences

NoMatch, Rejected, and RegistryError retain their generic UI0B readiness and
have no export capability. A matched profile never grants capability to another
sequence in the same project.

# Warnings

The generic `missing_channel_routing` data-loss warning is removed only for a
sequence whose authenticated policy supplies that routing. Other warnings stay
owned, ordered, and visible; a Ready sequence is not left with a contradictory
data-loss warning.

# Project readiness

`Ready` requires every discovered sequence to be Ready. A mixed project with at
least one Ready or PartiallySupported sequence is `PartiallySupported`. A
project containing only positively Unsupported sequences is `Unsupported`; an
empty or otherwise safely unclassifiable set is `Unknown`.

# Freshness and export boundary

Projection is an inspection-time UI result, not export authorization. UI0D must
still call UI0C4B immediately before export and use the fresh owned bytes and
policy returned by that handoff.

# Tests

Focused service-unit tests cover matched/non-matched projection and empty,
all-ready, mixed, unsupported, and unknown project aggregates. The optional
authentic regression verifies only the exact matched sequence becomes Ready and
other sequences remain conservative.

# Explicit exclusions

No app-contract, parser, decoder, export, serialization, FFI, UI, or dependency
changes are part of UI0C4C.

# UI0C4C gate

The readiness projection is complete when only exact private matches become
Ready with safe capability, mixed-project aggregation follows the documented
rule, and freshness/export boundaries remain unchanged.

# Single recommended next step

Design the UI0D export handoff around immediate UI0C4B revalidation and fresh
owned source bytes.
