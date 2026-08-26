# Objective

UI0C1 implements generic, synthetic-only compatibility-profile infrastructure
for Phoenix. It validates declarative policy against owned structural evidence
without migrating the authenticated Ode profile or wiring readiness/export.

# Scope

The new `compatibility` module contains profile IDs, fixed-width ranges,
evidence, expectations, channel/Patch policy, resolved policy, match results,
registry construction, and exact matching. It does not read files, parse
Studio Vision bytes, inspect AppService sessions, or serialize MIDI.

# Module

`src/compatibility.rs` is exposed as a reusable Core module. It imports only
the safe `ProfileCapability` DTO for successful result conversion; it does not
import parser, walker, adapter, or proof-policy modules.

# Profile identity

`ProfileId`, `ProfileVersion`, and `ParserProfileId` are owned stable values.
Profile IDs/version pairs must be unique in a registry. A display label is
required on each profile and is copied into `ProfileCapability` only after a
complete match.

# Structural range

`ByteRange` uses validated `u64` half-open `[start, end_exclusive)` semantics.
It has no slice lifetime or `usize` persistence and reports invalid reversed
ranges at construction.

# Evidence model

`ProfileEvidence`, `SequenceEvidence`, and `TrackEvidence` own all strings,
bytes, vectors, and structural facts. Track keys combine descriptor and pair
ordinals. Optional observed channel evidence allows a future adapter to verify
an independently observed route; absent routing is intentionally permitted for
an authenticated policy to supply.

# Profile expectations

`ProjectExpectation` requires exact SHA-256, byte size, parser profile, and
declared sequence count. SHA-256 format is validated as 64 hexadecimal bytes.
`SequenceExpectation` requires exact ordinal, sequence/name ranges and bytes,
descriptor/pair counts, and a complete track manifest. Each `TrackExpectation`
has a closed disposition: `Included` owns its mandatory channel and translated
Patch policies; `Omitted` owns either exact authenticated-nonempty evidence or
the parameterless structural-empty policy. The latter two are output policy,
not decoded Studio Vision inclusion semantics.

# Channel policy

`TrackChannelPolicy` validates human MIDI channels 1..=16. Every Included track
must have one keyed policy; omitted tracks cannot carry one. There is no
default or name lookup.

# Patch policy

The declarative variants are `ProgramOnly`, `BankSelectMsbAndProgram`, and
`BankSelectAndProgram`. `PatchExpectation` binds the source ordinal/range and
decoded values to the translation. The MSB-only form permits an absent decoded
MSB under authenticated profile authority, rejects a contradictory present
MSB, and requires decoded LSB to be absent. Construction rejects a policy whose
translation does not match its expected evidence. There is no guessed bank,
optional-LSB, sentinel, or opaque fallback.

# Resolved policy

Successful matching produces an owned `ResolvedProfilePolicy` containing the
profile identity, matched sequence identity, and every structural row in
canonical observed order. Its closed resolved disposition either carries the
Included channel/Patch translations or exact nonempty/structural-empty omission
authority. The complete manifest participates in source-revalidation equality.

# Match results

`ProfileMatch` has exactly `NoMatch`, `Matched { capability, resolved_policy }`,
and `Rejected { profile_id, reason }`. Provenance failures (hash, size, parser
profile, or sequence cardinality) produce `NoMatch`. Once exact provenance is a
candidate, structural, channel, or Patch contradictions produce `Rejected`.
Rejected results never carry partial policy.

# NoMatch versus Rejected

The distinction prevents a structurally drifted copy of an identified profile
from silently falling through to another policy. Bounded mismatch codes cover
hash, size, parser profile, sequence identity, track manifest, channel policy,
Patch policy, and ambiguity.

# Registry construction

`CompatibilityRegistry::new` accepts an explicit profile vector, validates it,
then stores an immutable deterministic order by profile ID/version. `empty`
supports no-profile operation and synthetic tests. There is no runtime loading,
global mutable state, or plugin discovery.

# Definition validation

Construction rejects invalid SHA-256, invalid channels, duplicate profile
identity, duplicate sequence ordinals, duplicate track keys, duplicate Patch
source ordinals, and Patch translations inconsistent with decoded evidence.

# Ambiguity

All profiles are assessed. If two fully match, assessment returns
`AmbiguousProfiles`; declaration order never chooses a winner. One match is
returned only after all other candidates have been checked.

# Exact cardinality

The matcher requires declared sequence count, selected sequence identity, and
track count to match exactly. Duplicate observed track keys, missing rows,
extra rows, shifted ranges, or changed structural ordinals reject the profile.

# Matcher behavior

The generic API is `registry.assess(&evidence, selected_sequence_ordinal)`.
Names and labels are corroborating fields only. Filename is never read or
accepted. Channel and Patch evidence are complete when supplied and never
inferred by the registry.

# Contract isolation

Only successful `ProfileCapability` data crosses toward the app contract.
Readiness, SequenceSummary, SessionId, offsets, channels, Patch details, and
resolved policy remain untouched in UI0C1.

# General-routing seam

The module is one possible source of a future Ready result, not the only one.
A later general proven conversion engine may bypass profiles; both paths must
retain no-silent-loss and export-time revalidation rules.

# Tests

`tests/compatibility_registry.rs` covers empty/no-match behavior, exact
capability/policy results, provenance and structural mismatches, cardinality,
channels, both Patch policies, definition validation, ambiguity, ownership,
and fixed-width ranges. All data is synthetic and generic.

# Explicit exclusions

No authenticated Ode values, AppService wiring, readiness integration, export,
FFI, serde, UI, file access, runtime profiles, or parser behavior changes were
introduced.

# UI0C1 gate

PASS when the generic registry validates declarative definitions, distinguishes
NoMatch from Rejected, returns complete capability/policy only on exact success,
rejects ambiguity without first-wins behavior, and remains target-independent.

# Single recommended next step

Implement UI0C2: build generic `ProfileEvidence` from AppService's owned
inspection/session state without migrating the authenticated Ode profile or
changing readiness yet.

# Initial UI0C2 handoff (historical baseline)

The AppService adapter now supplies owned provenance and structural evidence
snapshots. Exact event ranges, decoded families, Patch evidence, and channels
remain explicitly incomplete until reusable parser facts are established;
complete profile matching rejects those rows safely.

# UI0C2A investigation

The exact-range blocker is scoped: a validated seven-byte suffix rule is
supported for the established 166-byte profile by a 132-primary census and
empty-track evidence. Implementing the parser-facing bounds helper remains a
separate structural change; incomplete evidence must continue to reject
matching.

## UI0C2B implementation

Exact Descriptor166 bounds are now available to the evidence snapshot through a
validated parser helper. Registry matching remains unchanged: `Some` exact
bounds do not satisfy `evidence_complete`, so no profile can match until later
evidence slices are present.

## UI0C2C family inventory

Decoded family facts and logical counts are now available for complete bounded
walks, but `evidence_complete` remains false because Patch evidence and routing
are still absent. Registry matching is unchanged.

## UI0C2 completion status

The adapter now supplies exact Descriptor166 ranges, deterministic decoded
family/count facts, and the bounded generic Patch subset. The registry still
requires `evidence_complete` and therefore remains unwired to AppService until
authenticated profile migration supplies routing and policy evidence.

## UI0C3 authenticated profile status

The isolated `compatibility_profiles` module now supplies the validated Ode
research profile through the generic fallible registry constructor. It is not
wired into AppService readiness or export; see
`AUTHENTICATED_COMPATIBILITY_PROFILE_IMPLEMENTATION.md`.

The generic Patch matcher permits an authenticated banked translation to carry
bank values in resolved policy while observed generic bank fields remain
`None`; if bank evidence is present, any value must still agree exactly.

Focused synthetic regressions now cover absent banks, matching complete or
partial observations, contradictory MSB/LSB observations, and mandatory
program identity.
