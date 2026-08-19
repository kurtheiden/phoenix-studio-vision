# Objective

UI0C2B exposes the smallest parser-facing seam needed to turn an established
Descriptor166 track payload into exact mixed-event bounds. It validates the
terminal structure before deriving the event range and leaves decoding,
compatibility matching, and export to later phases.

# Scope

The helper is explicitly limited to the established 166-byte sequence profile.
It does not claim a universal Studio Vision terminator and is not applied to the
unsupported 120-byte profile or other unvalidated versions.

# Established Descriptor166 rule

For a parsed track pair, the candidate event start is the parser-established
payload start plus the 14-byte primary prefix. The exact event end is the
payload end minus seven bytes, but only after validating the final seven bytes.

# Result type

`TrackEventBounds` owns four half-open `Range<usize>` values: primary record,
payload, exact event, and terminal-tail ranges. The event range is the narrow
range supplied to the bounded mixed-event walker; the existing containing range
is unchanged.

# Error model

`TrackEventBoundsError` distinguishes missing pairs, malformed relationships,
short payloads, arithmetic failures, an event start beyond the derived end, and
invalid terminal grammar. No error is converted into a containing-range
approximation.

# Checked arithmetic

The helper validates payload containment, recomputes the 14-byte prefix start,
uses checked addition/subtraction, and requires `event_start <= event_end`.
Empty event ranges are valid when the prefix and terminal structure are valid.

# Terminal grammar

The seven-byte suffix must satisfy:

`ff ?? ?? ?? ff 2f 00`

Only bytes 0, 4, 5, and 6 are constrained. The three middle bytes remain
opaque and are preserved only as part of the source payload.

# Opaque middle bytes

Synthetic tests accept multiple middle-byte values. The helper does not assign
meaning to them and does not search for an earlier terminal-shaped sequence.

# Empty tracks

When no event bytes occur after the prefix, `event_range.start ==
event_range.end` and the validated tail immediately follows it.

# Relationship to containing range

`TrackRecordPair::event_containing_range` remains the parser's broader payload
range from candidate start through payload end. `TrackEventBounds` proves the
narrow event range and tail relationship without changing that existing API.

# Parser API

`SequenceContainer::validated_track_event_bounds(pair_ordinal)` delegates to
`TrackRecordPair::validated_event_bounds()`. It consumes only already parsed
record/payload facts and does not depend on `mixed_event` or perform heuristic
scanning.

# AppService evidence integration

The retained UI0C2 structural snapshot now records `exact_event_range` when the
validated helper succeeds. A failure leaves the field absent and does not make
the session unusable.

# Evidence completeness

Exact bounds are only one evidence component. `evidence_complete` remains
`false` because decoded family inventory, generic Patch evidence, and routing
evidence are still intentionally absent. Compatibility matching continues to
require complete evidence.

# Tests

Parser tests cover valid non-empty and empty ranges, opaque middle bytes, each
constrained suffix byte, earlier terminal-shaped bytes, pair-not-found errors,
start-after-end failures, and containing-range relationships. The optional
authentic inspection test confirms exact ranges are populated while readiness,
families, Patch evidence, and channels remain unchanged.

# Authentic regression

The existing optional Experiment 007 inspection path supplies corpus-level
regression coverage without embedding Ode export policy. A broader 166-byte
census remains documented research evidence rather than a new fixture dependency.

# Broader-profile limitation

The 120-byte profile and other Studio Vision versions remain unestablished. No
helper call or documentation here promotes the seven-byte rule beyond
Descriptor166.

# Explicit exclusions

UI0C2B does not migrate an authenticated profile, populate decoded families or
Patch policy, infer channels, change readiness, enable export, or add transport,
UI, or dependency code.

# UI0C2B gate

PASS requires validated Descriptor166 bounds, exact suffix checking, checked
arithmetic, empty-track support, unchanged containing-range semantics, safe
AppService population, and unchanged conservative readiness. The next evidence
slice is bounded event-family inventory.

# Historical next step (completed)

Implement UI0C2C by walking these validated ranges with the existing bounded
mixed-event walker, while keeping Patch evidence and channel routing
conservative.

# Current status

The bounds helper is consumed by the bounded family/Patch inventory adapter;
the Descriptor166 scope and broader-profile limitation remain unchanged.

# UI0C2C handoff

The exact event range is now consumed by AppService through the existing bounded
walker. Successful walks populate deterministic family inventory and logical
counts; exact bounds remain useful even when a walk fails, but no partial events
are retained.
