# Objective and scope

Phoenix now implements the first exact-bounded Studio Vision mixed-event
walker for event regions supplied under the authenticated 166-byte descriptor
profile. It supports only Note, Patch, ordinary Controller, Channel Pressure,
Pitch Bend, the established direct/one-`ff 60` Patch transition, and the
established non-Patch `ff 60` Note-entry form. It also supports the authenticated
Bells form in which a length-bounded Patch core is followed by exactly one
ordinary Controller and one direct explicit Note.

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
- `ff 60` invokes the strict context-mediated Note entry;
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
two logical events. Patch position remains absolute; first-Note position adds
post-PC timing and, in the extended form, final timing. A context-mediated
Note entry preserves and adds its leading and final timing components.

For the Bells Patch-to-Controller-to-Note form, the walker transactionally
returns three ordered logical items with disjoint ranges: standalone Patch core,
ordinary Controller, and explicit Note. Patch position is stored absolute;
Controller position adds its timing delta; Note position adds its own delta to
the Controller position. Successful continuation begins after the Note in Note
state.

# Error and transactional behavior

`MixedEventWalkError` records absolute cursors/offsets for invalid bounds,
timing failures, arithmetic overflow, data without state, unsupported
statuses/tags, malformed known families, context mismatch, bound crossing,
high-bit data, and cursor non-progress. The API returns the complete walk or
`Err`; accumulated partial events are never exposed as successful recovery.

# No-scanning enforcement

No branch searches for a later status/tag, probes another decoder, backtracks,
or resynchronizes. Tests place an unsupported `ff` branch at the current cursor
and a valid-looking Note later; the original tag offset is returned. Repeated
`ff 60` syntax is rejected at the second record rather than recovered.
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
component truncations, and later-signature negative controls.

# Authentic Bells Tracks 3 and 4

Experiment 007 Track 3 `0x010a4d..0x0110c8` and Track 4
`0x01121b..0x01192a` now consume exactly. Their Patch/Controller/first-Note
positions are respectively `480/960/71040` and `180/208/71278`, with exact
adjacent source ownership and continued Note-state suffix walking. Existing
authenticated MIDI correlation remains complete for both tracks. Bells exact
mixed-event consumption therefore improves from 11/14 to 13/14 tracks.

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

SysEx, Poly Pressure, unknown statuses/tags, repeated or other `ff 60` forms,
other Patch/context layouts, Patch-to-multiple-Controller forms, context after a
Patch-following Controller, the 120-byte profile, recovery, and MIDI export
remain unsupported. Bells Track 6 repeated `ff 60` contexts, CC0-only Patch
translation, inclusion policy, compatibility/readiness, and general routing
remain separate unresolved work.

# Single recommended next step

Design MIDI writer/export integration over the validated project, sequence,
Meter/Tempo, and mixed-event representations without broadening this parser.
