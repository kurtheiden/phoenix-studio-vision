# Objective

UI0C3 migrates the authenticated Ode to Clarke compatibility policy into an
isolated compiled-in Core profile. It returns a safe capability and resolved
policy only after the generic registry has matched exact evidence.

# Scope

This is one provenance-locked research profile. It does not change AppService
readiness, enable export, or claim general Studio Vision compatibility.

# Profile identity

The stable identity is `studio_vision_ode_to_clarke_v1`, version `1`, with the
honest label `Validated research profile — Ode to Clarke`.

# Provenance authority

The profile requires the authenticated source SHA-256
`e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`, exact size
211,468 bytes, parser profile `descriptor166`, and the complete project
sequence count of 18. Filename and path do not participate.

# Target sequence identity

The target is structural sequence ordinal 14 with range `0x02ef6f..0x03202c`,
raw name `Ode to Clarke`, name range `0x02f753..0x02f760`, 11 descriptors, and
9 track pairs. Name equality alone is insufficient.

# Track manifest

The isolated profile contains all nine authenticated rows, keyed by descriptor
and pair ordinals. Each row carries the exact descriptor, primary, and event
ranges, raw label bytes, and a validated channel policy. The manifest is
complete and ordered by structural identity; no row is optional and no default
row exists.

# Channel policy

Authenticated channels are structural policy values, not inferred routing:
`1, 2, 10, 10, 10, 1, 10, 15, 10`. Generic evidence still reports no observed
channel; the resolved profile policy supplies these values only after an exact
match.

# Patch translation policy

The four proof-established Patch observations are keyed by exact source range,
walk-item ordinal, and direct decoded program. Track 1 and Track 3 #2 use
ProgramOnly; Track 2 and Track 3 use the authenticated BankSelectAndProgram
translations (81/1 and 81/2 respectively). Generic bank evidence remains
optional/opaque; the profile's bank translation is authenticated export policy,
not a newly inferred decoder fact.

The generic matcher regressions prove that absent bank observations can match,
matching observed values agree, and contradictory observed values reject.

# Resolved policy

`ProfileMatch::Matched` returns the safe capability plus an immutable resolved
sequence identity, all nine channel assignments, and four Patch translations.
No serializer or export operation runs in this phase.

# Built-in registry construction

`built_in_compatibility_registry` constructs the isolated profile and passes it
through `CompatibilityRegistry::new`, so duplicate identities, malformed
channels, duplicate rows, and inconsistent Patch declarations cannot bypass
generic definition validation.

# Exact-match behavior

The authentic fixture matches only when project provenance, target sequence
identity, complete track manifest, exact event bounds, Patch observations, and
policy declarations all agree. The authenticated test verifies capability,
sequence identity, channel vector, and four Patch policies.

# NoMatch behavior

Different source bytes, a filename-only copy without the same bytes, or an
unrelated same-name input cannot become a candidate. An identical byte copy
under a different filename still matches.

# Rejected behavior

After exact provenance identifies the profile candidate, changed structural
sequence or Patch evidence returns `Rejected` with the appropriate mismatch
reason. Another sequence in the same project never inherits the target
capability.

# Filename independence

The authenticated test copies the source to a differently named temporary path
and confirms byte-identical matching. No filename or display name lookup exists
in the profile.

# Another-sequence isolation

Selecting another structural sequence ordinal produces no match/rejection, never
`Matched`, even though the project provenance is identical.

# Authentic regression

`tests/authenticated_compatibility_profile.rs` skips cleanly when the external
fixture is absent. When present it exercises exact matching, renamed identity,
one-byte mutation, same-name unrelated input, structural drift, Patch drift, and
resolved policy contents.

# Target-specific isolation

All Ode constants live in `src/compatibility_profiles.rs`. Generic parser,
evidence, AppService, MIDI, and multitrack modules remain target-independent;
the historical Track 3 #2 research decoder in `src/patch.rs` is unchanged.

# Relationship to existing proof

The existing D2/D3/D4/D5 proof tests remain authoritative and unchanged. UI0C3
migrates their authenticated manifest/policy into a declarative production
profile without replacing the independent proof path.

# UI0C5 Bells profile

The built-in registry now includes the provenance-locked
`studio_vision_bells_for_her_v1` profile. It matches the exact authenticated
project/sequence and complete 14-row structural manifest: ten Included output
rows, Tracks 2 and 7 as authenticated nonempty omissions, and Tracks 10 and
13 as structural-empty omissions. Included channels and Patch translations
are fixed authenticated override policy; omitted rows receive neither
channels nor Patch translations. This is compatibility-profile output policy,
not a decoded Vision mute, enable, playback, routing, or inclusion field.
Handoff emits ten musical tracks in parser structural order and freshly
revalidates every omitted row. The authentic generated-export proof parses
both SMFs and compares Notes and every supported channel-event family
event-by-event. The comparison target is normalized musical reconciliation,
not byte equality; historical zero-velocity Note-Off substitutions remain an
explicit reference exception.

# Explicit exclusions

No general Studio Vision inclusion/routing semantics, runtime profile loading,
FFI, or public contract change was added. Bells readiness and export are
available only after its exact provenance-locked profile match.

# UI0C3/UI0C5 gate

The gate passes when the isolated profile validates, the authentic fixture
matches exactly, renamed identical bytes match, mutated bytes and same-name
unrelated input do not, structural/Patch drift rejects, and no generic service
behavior changes.

# Single recommended next step

Keep Bells-specific normalized proof and future profile declarations separate
from any general Studio Vision routing or inclusion recovery.
