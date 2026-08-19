# Objective

UI0C defines the smallest Core-only seam for recognizing an explicitly
validated export policy. It lets a future service ask whether an inspected
sequence is covered by a complete compatibility profile without teaching the
generic parser, UI contract, or frontend any Studio Vision target constants.

# Scope

This is a design for a compiled-in registry and declarative profile matchers.
It does not implement a registry, move the authenticated Ode policy, enable
export, add dependencies, or change the UI0A/UI0B contract.

# Registry boundary

Use one focused Core module, preferably `src/compatibility.rs`, until several
independent profiles justify a subdirectory. It owns four internal concepts:

* `CompatibilityRegistry` — an immutable ordered collection of compiled
  profiles, queried by `AppService`.
* `CompatibilityProfile` — identity, display metadata, match expectations, and
  a declarative export policy.
* `ProfileMatch` — `NoMatch`, `Matched`, or `Rejected` with bounded diagnostic
  context.
* `ProfilePolicy` — the resolved, Core-only channel/Patch/timing policy handed
  to the later export layer.

The registry receives generic evidence produced by the inspection/parser path.
It never opens a file, scans raw bytes, or asks the frontend for a name,
channel, offset, or Patch value.

# Profile identity

Use Core newtypes `ProfileId(String)` and `ProfileVersion(u32)`. Their values
are stable identifiers, not parser facts. Conversion to UI0A's owned
`ProfileCapability { profile_id, profile_version, display_label }` occurs only
after a complete match. A profile ID may be specific, for example
`studio_vision_ode_to_clarke_v1`, while its user-facing label must say
`Validated research profile — Ode to Clarke`, not imply general Studio Vision
support.

# Match evidence

The safe generic input is an owned internal `ProfileEvidence` assembled by Core
from one inspected session:

```text
ProfileEvidence {
    source_sha256
    source_byte_size
    parser_profile
    sequences: [SequenceEvidence]
}

SequenceEvidence {
    structural_ordinal
    sequence_range
    name_bytes
    name_range
    descriptor_count
    pair_count
    tracks: [TrackEvidence]
}

TrackEvidence {
    descriptor_ordinal
    descriptor_range
    pair_ordinal
    primary_range
    exact_event_range
    label_bytes
    decoded_event_families
    decoded_event_count
    patch_evidence
}
```

These ranges and bytes are internal evidence, never app-facing identity. The
evidence adapter must use facts already derived by the established parser and
bounded walkers; it must not rediscover structure in the registry.

# Project provenance

The current research profile requires all of:

1. exact SHA-256;
2. exact byte size;
3. the established parser profile; and
4. the profile's complete sequence/track evidence.

SHA-256 is the primary provenance lock, but size remains an explicit cheap
consistency check and diagnostics field. A hash mismatch is an immediate
`NoMatch`; there is no close-enough or filename fallback.

# Sequence identity

Sequence names never identify a profile. A profile expectation must match the
sequence structural ordinal, exact sequence range, exact name bytes and name
range, descriptor count, pair count, and every required track row. The service
session's opaque `SequenceId` is used only to select the internally mapped
sequence; the registry revalidates the structural identity before returning a
match.

# Track manifest

Track expectations are internal rows keyed by structural identity, not names:

```text
TrackExpectation {
    descriptor_ordinal
    descriptor_range
    pair_ordinal
    primary_range
    exact_event_range
    expected_label_bytes: optional bytes
    channel_policy_id
    patch_policy_id: optional id
}
```

The row set must have exact cardinality, unique ordinals, complete association,
and exact ranges. Missing, extra, reordered, or shifted rows reject the
candidate. Labels may corroborate identity but can never be the sole key.

# Channel policy

Channel assignment is declarative and complete. Each authenticated row names a
structural track key and a validated human MIDI channel; there is no default,
name-only lookup, or inferred channel. A missing row, duplicate key, invalid
channel, or observed mismatch rejects the entire profile. The resulting
assignments remain inside `ProfilePolicy` and never enter app DTOs.

# Patch policy

Patch translation is also declarative. A `PatchExpectation` is keyed by the
track key and source Patch identity/ordinal, and names only an independently
confirmed classification such as program-only or confirmed CC0/CC32/program.
The matcher validates the decoded Patch evidence before enabling that
translation. There is no default Program Change, guessed bank, opaque-tail
fallback, or reference-MIDI inference at runtime.

# Match results

The registry returns:

* `NoMatch` when provenance does not identify a candidate (for example a hash
  mismatch or no profile parser family);
* `Matched(ProfileCapability, ResolvedPolicyHandle)` only after every required
  expectation succeeds; or
* `Rejected(ProfileId, ProfileMismatchReason)` when a candidate's provenance
  is close enough to identify the intended profile but structural, channel, or
  Patch evidence contradicts it.

`Rejected` is not a partial policy. It must never produce an export capability.
The service maps it conservatively to `Unknown` with an
`profile_*_mismatch` diagnostic when corruption or structural drift cannot be
classified more precisely; it may use `UnsupportedProjectProfile` when the
profile is positively outside the supported structure. Both outcomes disable
export.

# Multiple-profile behavior

The registry has deterministic declaration order for diagnostics, but order
does not select a profile. Exactly one match is required. Zero matches is
`NoMatch`; two or more matches is an internal configuration error with
`profile_ambiguous_match`. A first-match policy is prohibited.

# Readiness integration

UI0B's generic assessment remains the baseline. After generic parsing, the
service asks the registry about each sequence:

* `Matched` becomes `Readiness::Ready`,
  `ValidatedCompatibilityProfile`, and a `Some(ProfileCapability)`.
* `NoMatch` retains the generic readiness, normally
  `PartiallySupported`, `Unsupported`, or `Unknown`.
* `Rejected` becomes conservative non-ready state with a bounded diagnostic;
  it never falls back to a weaker profile or guessed policy.

Names, filenames, or resemblance to the authenticated target never trigger
Ready.

# Project readiness

Use an explicit mixed-state rule: project readiness is `Ready` only when every
discovered sequence that is in scope for the selected project assessment is
Ready. A mixture of Ready and non-ready sequences is `PartiallySupported`; a
positively unsupported project with no Ready sequence is `Unsupported`; an
unclassifiable assessment is `Unknown`. The UI receives this result and does
not calculate it.

# Registry ownership

`AppService` should own an immutable `CompatibilityRegistry` instance, supplied
at construction with a production default. This avoids global mutable state,
makes synthetic registries straightforward to test, and leaves room for one
service instance per future application process. Registry entries are compiled
into Core; there is no runtime discovery.

# Resolved policy lifetime

On a match, the private session stores both the profile identity/version and an
immutable resolved policy handle. The handle is never placed in
`SessionId`, `SequenceId`, or app DTOs. This avoids reconstructing policy from
display data while allowing diagnostics to report the capability identity.

# Export-time revalidation

UI0D must not trust a stale match. Before export it must re-read the source,
verify byte size and SHA-256, reparse the applicable structural profile, map the
selected `SequenceId` to the same structural identity, rebuild evidence, and
re-match the registry. Any replacement, drift, or policy-version change fails
the transaction before MIDI assembly.

# Ode profile migration

The existing authenticated Ode constants should eventually move from proof
test policy into an isolated compiled profile module, for example
`src/compatibility_profiles/ode_to_clarke.rs`, only after generic validators
exist. The module would contain the project hash/size, sequence identity, nine
track rows, authenticated channels, Patch classifications, and policy version.

Reusable generic validators should be extracted from the D2 integration for
range/ordinal/event-inventory checks. D2/D3 tests remain the provenance and
comparison evidence; they should call the same profile matcher rather than
duplicate nine-track assembly. No constants move during UI0C design.

# Naming

The profile's internal ID may name the target for stable diagnostics. The
display label must remain honest: `Validated research profile — Ode to Clarke`
and a detail that it applies only to the authenticated source/provenance. It
must not say `Studio Vision supported` without qualification.

# Traits versus declarative profiles

Prefer declarative structs plus generic matching functions. A trait with
callbacks would make provenance review and deterministic testing harder and
would invite hidden policy. A trait is unnecessary until a genuinely dynamic
policy algorithm is independently established.

# Runtime profile loading

Runtime-loaded or third-party profiles are out of scope for v0. Profiles are
compiled into Core, versioned with Phoenix, code-reviewed, and covered by
synthetic plus authentic tests. This avoids an untrusted policy/plugin surface.

# Diagnostics

Bounded internal codes include:

* `profile_hash_mismatch`
* `profile_size_mismatch`
* `profile_sequence_identity_mismatch`
* `profile_track_manifest_mismatch`
* `profile_channel_policy_mismatch`
* `profile_patch_policy_mismatch`
* `profile_ambiguous_match`

Normal UI messages say only that a validated profile did not match. Full
diagnostics may include the failing generic field and observed/expected
categories, but raw file bytes and internal offsets remain out of the normal
contract.

# Contract impact

The committed UI0A contract is sufficient. `ProfileCapability`, `Readiness`,
`ReadinessReason`, and `SequenceSummary.export_capability` already carry the
safe result. No new DTO, channel field, policy handle, or offset is required.

# AppService integration

The later change is a narrow branch after generic inspection:

```text
read/hash -> identify -> parse/enumerate -> generic readiness
                                      \-> registry assess -> optional Ready/capability
```

Unsupported projects remain inspectable. The registry cannot replace generic
parsing, and a registry failure cannot erase generic warnings or diagnostics.

# UI0D handoff

Successful export receives an internal resolved policy containing profile ID /
version, validated sequence structural identity, every channel assignment,
validated Patch translations, and profile timing/export settings. This is a
Core-to-export-layer value only; the frontend supplies none of it.

# Generalization seam

Policy precedence is: (1) a future general, proven conversion path; (2) an
exact compatibility profile when general proof is unavailable; (3) non-ready
inspection. Both mechanisms produce the same app-facing Ready/capability shape,
so the UI does not know why a sequence became exportable. A general path must
still satisfy the same no-silent-loss and export-time revalidation rules.

# Testing strategy

## Synthetic registry tests

Cover an empty registry, one exact match, hash/size mismatch, sequence range or
name-range mismatch, descriptor/pair/event-range mismatch, missing or extra
rows, channel mismatch, Patch mismatch, duplicate matching profiles, no
name-only match, and absence of fallback policy. Assert that capability is
returned only after complete success.

## Authentic profile tests

Against the optional external fixture, require exact source provenance and the
complete sequence/track manifest. A renamed file with identical bytes may
match; a copied or modified byte variant may match only if its bytes are truly
identical. Another sequence in the same project, a shifted range, a missing
row, or a name-only unrelated file must not become Ready. Fixtures remain
outside the repository and are never modified.

# Implementation decomposition

1. **UI0C1:** Core-only profile IDs, evidence types, declarative policies,
   matcher results, and synthetic registry tests.
2. **UI0C2:** an evidence adapter from AppService's owned session/parser state
   with no new binary interpretation.
3. **UI0C3:** migrate the authenticated Ode constants into an isolated profile
   module and reuse generic manifest validators.
4. **UI0C4:** merge registry outcomes into sequence/project readiness and
   diagnostics while preserving generic inspection.
5. **UI0C5:** authentic exact-match/mismatch tests, then hand the resolved
   policy to UI0D's transactional export service.

# UI0C gate

| Area | Result |
|---|---|
| Registry boundary | YES |
| Profile identity | YES |
| Generic match evidence | YES |
| Project provenance | YES |
| Sequence identity | YES |
| Track manifest | YES |
| Channel policy | YES |
| Patch policy | YES |
| Readiness integration | YES |
| Resolved policy lifetime | YES |
| Export-time revalidation | YES |
| Existing app contract sufficient | YES |
| UI0C implementation-ready | YES |

# Unknowns

The exact generic evidence adapter and the final extraction boundary from the
D2 test helper should be settled during UI0C1/UI0C2. In particular, the parser
does not yet expose every exact event-range or decoded-Patch fact through a
general owned inspection record. This is an implementation decomposition
question, not permission to infer missing evidence.

# Single recommended next step

Implement UI0C1: generic Core-only profile identity, evidence, declarative
policy, match-result types, and synthetic tests with no authenticated Ode
constants or AppService wiring.

# Initial UI0C2 status (historical baseline)

The generic service adapter may now build owned evidence from parsed sessions,
but it does not assess profiles or alter readiness. Since the parser exposes a
containing track range rather than a reusable exact event range, incomplete
track evidence must remain non-matchable until a safe range seam exists.

# Exact-range seam status (historical baseline)

Existing cross-track evidence confirms the seven-byte terminal grammar for the
authenticated 166-byte profile, but the parser API still exposes only a
containing range. A future validated bounds helper must own tail validation and
checked arithmetic before UI0C2 marks evidence complete; no broader-format rule
is implied.

# UI0C2 completion status

The later evidence slices established the reusable Descriptor166 bounds and
bounded-walker facts, so the adapter handoff is complete. Evidence remains
non-matchable until a later phase supplies authenticated routing and complete
profile policy; this is intentional registry isolation, not an adapter gap.
