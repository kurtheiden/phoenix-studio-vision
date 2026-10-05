# Objective and scope

Phoenix now implements the first exact-bounded Studio Vision mixed-event
walker for event regions supplied under the authenticated 166-byte descriptor
profile. It supports only Note, Patch, ordinary Controller, Channel Pressure,
Pitch Bend, the established direct/one-`ff 60` Patch transition, the established
non-Patch single-`ff 60` Note-entry form, and the authenticated fixed
double-context form with opaque payload lengths `6` then `7`, plus the bounded
`6` then `8` variant described below. It also supports the authenticated Bells
form in which a length-bounded Patch core is followed by exactly one ordinary
Controller and one direct explicit Note.

It does not discover track bounds, parse the seven-byte track tail, support the
older 120-byte profile, scan for event signatures, recover malformed input, or
write MIDI.

# Public API

`mixed_event::walk_bounded_mixed_events` accepts the complete source bytes,
`MixedEventBounds` with one absolute half-open event range, and
`MixedEventTimingBasis` with the preceding logical event position. It returns
`MixedEventWalk` only after exact consumption. Empty exact ranges succeed
without reading a VLQ.

# State machine

`ActiveEventState` contains only `None`, `Note`, `ChannelPressure`, and
`PitchBend`. At every non-terminal cursor the walker decodes one bounded timing
VLQ and classifies exactly its next byte:

- a data byte continues the active compact family;
- `90`, `d0`, and `e0` explicitly enter or replace compact state;
- `ff 41 05` decodes one Controller and clears compact state;
- `ff 7c` invokes the strict Patch-to-Note transition;
- `ff 60` invokes the strict single- or fixed double-context Note entry;
- every other status/tag is an error at the current cursor.

The loop requires monotonic progress and stops before attempting another VLQ
when its cursor equals the supplied event end.

# Decoder reuse and refactoring

- Controller uses `decode_bounded_controller_record` directly after its exact
  end is derived from the known grammar.
- Patch uses `decode_bounded_patch_representation` after the transition helper
  derives the exact `90` bound. Patch payload parsing was not duplicated.
- `track7` exposes a provenance-preserving single-Note primitive and Note-body
  primitive for enclosing transition timing.
- Channel Pressure and Pitch Bend expose single-entry/continuation primitives
  shared with their unchanged caller-bounded run decoders.

All earlier bounded APIs remain available and their regression tests pass.
Patch payload/name/Program parsing is factored through `BoundedPatchCore`, whose
range ends exactly at declared `payload_end`; the legacy Note-coupled decoder
composes that core without changing its accepted direct or one-`ff 60` forms.

# Output and timing

Ordinary results retain absolute ranges, raw timing VLQs, decoded fields, and
checked accumulated positions. Pressure and Bend entries preserve whether
their entry status was explicit.

Patch-to-Note is returned as one coupled `BoundedPatchToNoteTransition` with
two logical events. The raw stored Patch component is preserved while derived
Patch position adds it to the retained previous event position. First-Note
position then adds post-PC timing and, in the extended form, final timing. A
single-context Note entry preserves and adds its leading and final timing
components. The fixed double-context form preserves its leading,
inter-context, and final timings
and adds all three with checked arithmetic. Both forms produce one logical
Note; their opaque contexts produce no musical events.

For the Bells Patch-to-Controller-to-Note form, the walker transactionally
returns three ordered logical items with disjoint ranges: standalone Patch core,
ordinary Controller, and explicit Note. `PositionedPatch` preserves the raw
core and supplies the separately derived event position. Controller position
adds its timing delta to derived Patch position; Note position adds its own
delta to the Controller position. Successful continuation begins after the
Note in Note state.

# Error and transactional behavior

`MixedEventWalkError` records absolute cursors/offsets for invalid bounds,
timing failures, arithmetic overflow, data without state, unsupported
statuses/tags, malformed known families, context mismatch, bound crossing,
high-bit data, and cursor non-progress. The API returns the complete walk or
`Err`; accumulated partial events are never exposed as successful recovery.

# No-scanning enforcement

No branch searches for a later status/tag, probes another decoder, backtracks,
or resynchronizes. Tests place an unsupported `ff` branch at the current cursor
and a valid-looking Note later; the original tag offset is returned. Supported
repeated forms have exactly length `6`, timing, length `7` or the evidenced
length `8`, timing, and a direct explicit Note. The new `6`-then-`8` variant
additionally requires both context payloads to start `57 7f 00`. All other
repeated `ff 60` syntax is rejected at the current record rather than recovered.
The new branch likewise rejects `ff 60` after Controller, multiple Controllers,
malformed current records, and valid-looking later signatures without scanning
or resynchronization.

# Synthetic coverage

Tests cover empty and nonzero absolute ranges, explicit/continuing Notes,
Controller/Pressure/Bend transitions, compact continuations, direct Patch,
one `ff 60` Note entry, exact termination, invalid bounds, unsupported
branches, data without state, truncated timing, malformed duration/payload,
position overflow, provenance, and no scanning.
Focused cases additionally cover malformed/truncated Patch-following
Controllers and Notes, unsupported intervening contexts/controllers, exact
component truncations, and later-signature negative controls. Double-context
coverage requires exact `6`-then-`7` lengths, exact component ownership,
checked three-delta timing, a direct explicit Note, and rejection of wrong
lengths/tags, truncation, a third context, alternate families, compact or
alternate Note entry, overflow, and later-signature decoys.

# Authentic Bells Tracks 3 and 4

Experiment 007 Track 3 `0x010a4d..0x0110c8` and Track 4
`0x01121b..0x01192a` now consume exactly. Their Patch/Controller/first-Note
positions are respectively `480/960/71040` and `180/208/71278`, with exact
adjacent source ownership and continued Note-state suffix walking. Existing
authenticated MIDI correlation remains complete for both tracks. Bells exact
mixed-event consumption therefore improves from 11/14 to 13/14 tracks.

# Authentic Bells Track 6

Experiment 007 Track 6 `0x011eac..0x0123dd` now consumes exactly as 184 logical
events: 182 Notes, one Patch, and one ordinary CC7. Its residual population
contains six established single-context Notes and three fixed double-context
Notes beginning at `0x0121ba`, `0x0122c7`, and `0x012310`. Each double form has
exactly a length-`6` context followed by a length-`7` context and a direct
explicit Note. Bells structural mixed-event consumption is therefore 14/14.

Track 6's CC7 position is 130 and its raw Patch component remains 160. Retaining
the Controller basis derives Patch position 290 and first-Note position 71,903.
All 182 Note starts, endings, and explicit release velocities reconcile with
the authenticated MIDI. Patch translation, inclusion,
compatibility/readiness, and routing policy remain outside this timing change;
Bells is not profile-ready or export-ready.

# Authentic Track 9

Experiment 007 range `0x0143c8..0x014957` consumes exactly as 184 logical
events: 31 Notes, one Patch, 120 Controllers, and 32 Channel Pressure events.
Exactly one Pressure entry carries `d0`; 31 continuations are derived. The
extended Patch transition and separate context-mediated Note entry are both
preserved without a run bound.

# Authentic Track 14

Range `0x014e26..0x015ed4` consumes exactly as 601 events: 227 Notes, 272
Controllers, and 102 Pitch Bend events. Exactly nine Bend entries carry `e0`,
deriving all nine runs and exits. The final Controller is retained.

# Design conformance

The implementation follows `BOUNDED_MIXED_EVENT_WALKER_DESIGN.md`. Large Rust
enum variants use `Box` indirection for warning-free Clippy; parsing semantics
and provenance are unchanged. No grammar deviation was required.

# Remaining unsupported grammar

SysEx, Poly Pressure, unknown statuses/tags, arbitrary or other repeated
`ff 60` forms,
other Patch/context layouts, Patch-to-multiple-Controller forms, context after a
Patch-following Controller, the 120-byte profile, recovery, and MIDI export
remain unsupported. CC0-only Patch translation, inclusion policy,
compatibility/readiness, and general routing remain separate unresolved work.

# Single recommended next step

Resolve Bells Patch translation and inclusion policy only through separate
evidence-gated work; do not broaden the mixed-event grammar.

# Bounded 6-to-8 double-context Note extension

The existing general double-context path now supports a first context of exactly
six bytes followed by a second context of exactly eight bytes. For this new
variant, both length-framed `ff 60` payloads must start `57 7f 00`; their remaining
bytes stay opaque. The immediate successor must be the existing explicit `90`
Note body. Existing `6`-then-`7` acceptance and timing remain unchanged.

The evidence review at `7cf6ba27e29b8b7efe0ca0ed947a00b17a43fca4` found one
matching refused instance in authenticated SCHOOL PROJECTS. The same Track
already contained an accepted `6`-then-`7` double-context entry with identical
first payload, intervening/final deltas and Note properties, and an accepted
single eight-byte context. Complete sequential accounting establishes 259 Notes
plus ten contexts against the raw count of 269. Existing authenticated Bells
Track 6 evidence independently supports the reused three-delta timing arithmetic;
all 182 attacks/releases, including three supported double-context Notes, match
its native reference. This is structural evidence for the bounded extension,
not a claim of native Wind ensemble playback equivalence.

The implementation reuses checked addition of the leading, intervening and
final delta VLQs. It preserves both contexts, tags, declared lengths, exact
source bytes/ranges and all timing components in `DoubleContextMediatedNote`.
Pitch, attack/release velocities and duration come from the existing Note body.
No context semantics, musical events or channel values are invented. Channel
resolution still requires complete same-source ownership and the existing
Instrument/table/device/context guards; decoding alone does not authorize export.

Malformed/truncated framing, unsupported lengths, wrong prefixes for the new
variant, extra contexts, missing or alternate-family immediate successors,
invalid Note properties/duration, invalid timing/overflow, incomplete subsequent
coverage, unresolved ownership/routing and unsafe source order remain refused.
The first-context-seven shape currently refused in SCHOOL #15 is explicitly
outside this rule. No project name, ordinal, hash, source offset, Track name or
opaque-payload suffix is an acceptance predicate.

The current complete bounded SCHOOL pipeline passes source #10 Wind ensemble:
all 19 ordinary Tracks retained, 17 complete event-bearing Tracks and two paired
empty regions; 1,178 logical events (1,166 Notes, one source Controller, eleven
Patches). Clarinet's entire event region consumes exactly as 259 Notes and ten
retained contexts. Its new Note has tick 112,501, pitch 69, attack velocity 88,
release velocity 64, duration 80 and resolved channel 5. Conductor, routing,
Patch/bank translation, source order, sequence terminal and serialization pass.

Independent Format-1/480-PPQN SMF readback verifies all 1,166 attacks/releases,
six Control Changes (one source Controller plus five Patch CC0 messages), eleven
Program Changes, conductor events, channels/timing/order and Track/EOT bounds.
Output is 10,862 bytes, SHA-256
`0c6bd6e3b9c9ab272dcee6fe9d6479062495ca23db7eb5916ead5325fcb665b6`.
SCHOOL exportability increases from seven to eight of fifteen. The prior seven
verified artifacts remain byte-identical; all other refusal inventories remain
unchanged, including #15's different context form. The verified listening artifact is
`/Users/kurtheiden/Documents/Phoenix Research/Experimental MIDI Recovery/SCHOOL PROJECTS/SCHOOL - Wind ensemble.mid`.
Human listening of this new artifact has not yet been recorded.

Focused tests cover both supported lengths, independent F/Z channels, timing,
Note properties, absolute context provenance and equal-tick source order, plus
malformed/truncated contexts/Notes, wrong prefixes/lengths, extra layers,
unsupported suffixes, timing errors/overflow, unresolved/ambiguous routing,
ownership shortage and the excluded first-context-seven form.
