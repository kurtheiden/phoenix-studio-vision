# Objective

UI0C2 adapts AppService's retained, established parser facts into owned generic
`ProfileEvidence` for a later compatibility-registry query. It intentionally
does not perform that query or alter readiness.

# Scope

The adapter retains a private structural snapshot when `parse_project_166`
succeeds and exposes a Core-facing `profile_evidence(session_id)` method. It
does not read new formats, infer routing, classify Patches, export MIDI, or
create a profile instance.

# Session structural snapshot

Each parsed session now retains an owned project/sequence/track snapshot beside
the source bytes, hash, response, and diagnostics. It contains fixed-width
compatibility ranges and copied bytes only; no borrowed `SequenceContainer`
survives the inspection call.

# Parser profile identity

Successful 166-byte parsing records the generic `ParserProfileId` token
`descriptor166`. Unrecognized sessions have no profile evidence and return a
bounded `profile_evidence_unavailable` error rather than fabricated structure.

# Sequence structural identity

Sequence ordinals follow parser order as checked `u32` values. Sequence ranges,
descriptor count, pair count, and the exact parser name bytes/name range are
copied into `SequenceEvidence`. The opaque UI `SequenceId` remains separate;
the private session map resolves it to its structural ordinal for UI0C3/UI0C4.

# Sequence raw name evidence

Evidence uses the parser's raw name bytes and range, never the display-string
conversion. Invalid legacy text therefore cannot silently become a different
profile identity.

# Descriptor/pair evidence

Ordinal associations copy descriptor ordinal/range, pair ordinal, and primary
record range. Unresolved associations do not invent bindings; their sequence
snapshot contains no false track rows.

# Track evidence availability

Labels are copied from established raw descriptor label bytes. Channel evidence
is always `None`; no general MIDI routing source is assumed. Patch evidence and
decoded families are empty until a bounded generic walk/decoder adapter is
explicitly established.

# Exact range derivation

The current parser exposes `event_containing_range`, not a generic exact event
range. The authenticated D2 test derives exact bounds from a proof-specific
seven-byte tail, which is not a reusable parser fact. UI0C2 therefore records
`exact_event_range: None` and `evidence_complete: false`; it never substitutes
the containing range. UI0C1 complete profile matching rejects such incomplete
evidence.

# Event-family inventory

No broad walk is performed in UI0C2. The adapter reports no decoded family or
count rather than claiming an inventory it cannot establish safely. A future
bounded-walk adapter may fill these fields and mark evidence complete only
after exact ranges are available.

# Patch evidence

No Patch facts or translation policy are generated. The generic Patch decoder
and D2 coupled Patch transition are not reinterpreted here.

# Channel evidence

`observed_channel` remains `None` for every generic snapshot. Authenticated
profiles may later supply channels through resolved policy; UI0C2 does not infer
or default them.

# Evidence completeness

Completeness is explicit at track level. Structural rows can be preserved for
diagnostics and future matching, but incomplete rows cannot satisfy a complete
compatibility profile. This is conservative and preserves the UI0B readiness
result.

# SequenceId mapping

The private session map records each generated `SequenceId` alongside its
structural ordinal. Unknown sessions and unknown sequence IDs fail with typed,
deterministic errors. Structural ordinals are not added to app DTOs.

# Ownership

`ProfileEvidence` and all nested compatibility values own their vectors,
strings, and ranges. Tests retrieve evidence after request/path locals are
dropped. No parser lifetime crosses the service boundary.

# Authentic fixture policy

The optional external Experiment 007 test verifies only generic hash/size,
parser identity, sequence order/name evidence, and conservative incomplete
track facts. It does not contain channels, Patch classifications, profile IDs,
Ready assertions, or MIDI comparison and skips when absent.

# Parser/accessor changes

No parser or identification source changed. UI0C2 consumes existing public
sequence fields and stores an owned snapshot.

# Compatibility-type changes

UI0C1's generic types gained only the minimal completeness representation:
`TrackEvidence.exact_event_range` is optional and `evidence_complete` is
explicit. Complete synthetic matcher tests still require `Some` plus true;
missing evidence rejects safely.

# Readiness non-integration

`profile_evidence` never calls `CompatibilityRegistry::assess`, creates no
`ProfileCapability`, and does not change `SequenceSummary` or project
readiness. All UI0B results remain unchanged.

# Tests

AppService tests cover unavailable evidence for readable unknown input,
optional authentic structural evidence, source identity equality, session
ownership, and unchanged conservative readiness. Existing synthetic registry
tests continue to require complete evidence for successful matches.

# Explicit exclusions

No authenticated target constants, registry assessment, profile instance,
channel derivation, Patch translation, export, UI, FFI, serde, dependency, or
binary parser behavior was added.

# Initial UI0C2 gate (historical baseline)

The initial adapter was **PARTIAL by design**: source provenance, parser identity, sequence
identity, names, descriptor/pair facts, and ownership are available, while
exact event ranges, decoded families, Patch evidence, and channels remain
unavailable generically. No unsafe inference was made.

# Single recommended next step

Establish a reusable exact event-range/evidence adapter from existing bounded
parser facts before attempting UI0C3 profile migration.

# UI0C2 completion status

The later UI0C2A–UI0C2D slices completed that recommendation: Descriptor166
validated bounds now feed the existing bounded walker, deterministic family and
logical-count inventory is retained, and the bounded generic Patch subset is
copied from the same successful walk. `observed_channel` remains absent and
`evidence_complete` remains false by design because those facts belong to
compatibility-profile matching. See `PROFILE_EVIDENCE_ADAPTER_COMPLETION_AUDIT.md`.

# UI0C2A investigation

Termination evidence confirms a reusable boundary rule for the established
166-byte profile: validate `ff aa bb cc ff 2f 00` at primary payload end and
use payload `+14 .. payload.end - 7`. The current adapter remains conservative
until a parser-facing bounds helper owns those checks; the rule is not
generalized to the unsupported 120-byte profile.

# UI0C2B implementation

The Descriptor166 parser now exposes validated `TrackEventBounds`. The adapter
uses its exact event range when the suffix grammar is valid and leaves evidence
incomplete otherwise. This does not populate decoded families, Patch evidence,
or channels, and it does not alter readiness.

# UI0C2C implementation

Validated Descriptor166 ranges now feed the existing bounded mixed-event walker.
The adapter records canonical decoded family presence and exact logical event
counts. Walker failures produce no partial inventory; Patch evidence and channel
routing remain intentionally absent.

# UI0C2D implementation

Patch observations from successful walks now populate only generic source range,
walk-item ordinal, and direct program facts. Bank values, channels, and
translation policy remain absent; evidence stays incomplete.
