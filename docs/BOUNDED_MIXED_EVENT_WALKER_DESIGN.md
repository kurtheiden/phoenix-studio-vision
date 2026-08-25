# Objective

Design the first strict mixed-event walker for an exact Studio Vision track
event range in the authenticated 166-byte descriptor profile. The walker
advances only from the current cursor, preserves absolute byte provenance, and
supports only Note, Patch, ordinary Controller, Channel Pressure, and Pitch
Bend evidence already established in
`CURRENT_CURSOR_STATE_EXIT_CORRELATION.md`.

This document is an implementation contract, not production code.

# Evidence boundary

The container layer supplies this already-validated half-open range:

```text
event_start = track_primary.payload.start + 14
event_end   = track_primary.payload.end - 7
```

It derives `event_end` only after validating the authenticated terminal form:

```text
ff aa bb cc ff 2f 00
```

The walker does not inspect that terminal form and does not derive, widen, or
search for its own bounds. The established current-cursor classifier is:

```text
cursor == event_end                         => success
otherwise decode one bounded timing VLQ

active Note / ChannelPressure / PitchBend:
    first post-VLQ byte 00..7f              => active-family continuation

first post-VLQ byte ff                      => strict known tagged/context branch
first post-VLQ byte 90, d0, or e0            => explicit family entry/re-entry
anything else                               => unsupported at this cursor
```

The rule reproduced every event in the authenticated Bells Track 9 and Track
14 populations without run bounds, scan-ahead, backtracking, or musical
plausibility.

# Supported scope

## Supported

- an exact caller-supplied event range from the 166-byte profile;
- Note explicit entry and compact continuation;
- ordinary `ff 41 05` Controller records;
- `d0` Channel Pressure entry and compact continuation;
- `e0` Pitch Bend entry and compact continuation;
- the established direct Patch-to-Note form;
- the established form with exactly one length-framed `ff 60` context record;
- the established Patch-core-to-one-Controller-to-direct-Note form;
- the established `ff 60` context-mediated Note entry outside Patch;
- the authenticated fixed two-context Note entry with payload lengths `6`
  then `7`;
- strict exact-end termination.

## Unsupported

- event-region discovery or terminal-tail discovery;
- the older 120-byte descriptor profile;
- SysEx, Poly Pressure, unknown statuses, and unknown `ff` tags;
- arbitrary or other repeated `ff 60` forms, Patch-to-multiple-Controller
  forms, context after a Patch-following Controller, other Patch contexts, or
  other Patch layouts;
- general Studio Vision channel semantics;
- malformed or out-of-domain structures;
- heuristic recovery, decoder probing, scan-ahead, or backtracking;
- MIDI writing or historical export policy.

## Preserved without semantic invention

- raw timing encodings and their absolute ranges;
- Patch/context bytes whose syntax is bounded but whose purpose is unknown;
- the low status nibble as an observed source byte, not an exported MIDI
  channel;
- all family bytes already preserved by bounded family representations.

# Responsibility boundaries

## Sequence/container parser

The container parser owns project, sequence, and track navigation. A thin
166-profile track-bound validator above the walker validates the seven-byte
tail and supplies the exact event range. It also owns sequence/track identity
used for diagnostics. It does not interpret performance-event families.

## Mixed-event walker

The walker owns the event cursor, accumulated position, compact-family state,
current-byte dispatch, coupled Patch/context transitions, and complete-range
transaction. It derives family representation bounds at the current cursor
and calls exact family primitives. It never rediscovers container bounds.

## Bounded family decoders

Family code validates and preserves one exact representation supplied at its
start and end. Existing exact-bound guarantees remain intact. The mixed walker
may use shared single-event primitives, but it must not make the existing
caller-bounded run decoders scan or accept partial ranges.

## Future MIDI export layer

The exporter receives lossless decoded events and established positions. It
owns MIDI channel selection, status emission, SMF policy, and conversion of
Studio Vision-specific context. The walker does not emit MIDI or infer export
channels.

# Input contract

The proposed public entry point is conceptually:

```rust
pub struct MixedEventBounds {
    pub event_range: Range<usize>,
}

pub struct MixedEventTimingBasis {
    pub previous_event_position: u32,
}

pub fn walk_bounded_mixed_events<'a>(
    bytes: &'a [u8],
    bounds: MixedEventBounds,
    timing_basis: MixedEventTimingBasis,
) -> Result<MixedEventWalk<'a>, MixedEventWalkError>;
```

The range is absolute in `bytes`, matching existing provenance conventions.
A pre-sliced byte slice is rejected as the primary API because it would either
lose absolute offsets or require an additional base offset. The caller supplies
the position preceding the first ordinary delta-timed event. For an
authenticated full track that basis is the established track origin; accepting
it explicitly avoids pretending that every future bounded subrange starts at
zero.

The walker validates non-reversed bounds and `end <= bytes.len()` before
indexing. An empty exact range succeeds without reading a timing VLQ.
Sequence/track labels are not required inputs; callers may attach them when
reporting an error without changing parsing semantics.

# Active state

Only compact families persist:

```rust
pub enum ActiveEventState {
    None,
    Note,
    ChannelPressure,
    PitchBend,
}
```

Controller and Patch are not continuation states. A Controller is a complete
tagged record and leaves state `None`. A Patch transition is one coupled
structure that establishes Note state after its explicit `90` entry. A
context-mediated Note entry likewise establishes Note state.

Explicit `90`, `d0`, and `e0` replace any prior state. A known `ff` branch
exits any compact state before its own grammar determines the resulting state.
At exact `event_end`, the current state is irrelevant and parsing succeeds.

# Cursor algorithm

The complete algorithm is:

```text
validate event_start <= event_end <= bytes.len
cursor = event_start
previous_position = caller timing basis
state = None
items = []

loop:
    if cursor == event_end:
        return complete walk consuming event_start..event_end
    if cursor > event_end:
        fail internal invariant

    timing = decode one VLQ at cursor, bounded by event_end
    branch = timing.end
    require branch < event_end
    first = bytes[branch]

    if first < 80:
        require state != None
        decode exactly one continuation using state-specific width/grammar
        position = checked(previous_position + timing.value)
        append one ordinary event
        previous_position = position
        cursor = exact decoded end
        require cursor advanced and cursor <= event_end
        continue

    if first == ff:
        classify only the exact known tag at branch
        ff 41 05:
            derive the exact Controller end from its fixed grammar
            call the bounded Controller decoder on cursor..derived_end
            position = checked(previous_position + timing.value)
            append Controller; state = None
        ff 7c:
            parse the bounded Patch core through declared payload_end
            classify only the exact post-core timing endpoint
            direct 90 or one ff 60:
                preserve the established coupled PatchToNote transition
            ff 41:
                decode exactly one ordinary Controller at payload_end
                require exactly one direct explicit Note at Controller end
                append disjoint Patch, Controller, Note items transactionally
            reject every other form without scanning
            state = Note; previous_position = first-Note position
        ff 60:
            parse the first length-bounded context and following timing
            direct 90:
                preserve the established single-context Note entry
            exact ff 60 when the first payload length is 6:
                require a second payload length of 7
                require one final timing VLQ and direct explicit 90 Note
                append one fixed-shape double-context Note transactionally
            reject every other form at the current cursor without scanning
            previous_position = position
        other ff tag:
            fail UnsupportedFfTag at branch
        advance to exact known branch end and require progress
        continue

    if first == 90:
        decode explicit Note after timing
        position = checked(previous_position + timing.value)
        append Note; state = Note
    else if first == d0:
        decode explicit Channel Pressure after timing
        position = checked(previous_position + timing.value)
        append Pressure; state = ChannelPressure
    else if first == e0:
        decode explicit Pitch Bend after timing
        position = checked(previous_position + timing.value)
        append Bend; state = PitchBend
    else:
        fail UnsupportedStatus at branch

    previous_position = position
    cursor = exact decoded end
    require cursor advanced and cursor <= event_end
```

There is no branch that tries another decoder after failure.

# Timing model

The representation distinguishes stored syntax from derived position:

- `Delta`: one raw timing VLQ and the checked accumulated position;
- `PatchAbsolute`: the raw Patch position VLQ and its direct absolute value;
- `PatchFirstNote`: Patch absolute position, post-PC timing VLQ, optional
  bounded `ff 60` context, optional final timing VLQ, and derived Note position;
- `PatchControllerNote`: Patch absolute position, Controller delta and checked
  position, then Note delta and checked position;
- `ContextMediatedNote`: leading delta VLQ, one bounded `ff 60` context, final
  timing VLQ, their checked sum, and accumulated Note position.
- `DoubleContextMediatedNote`: leading delta VLQ, a length-`6` context,
  inter-context delta VLQ, a length-`7` context, final delta VLQ, their checked
  sum, and accumulated Note position.

Ordinary Note, Controller, Pressure, and Bend positions add their timing value
to the preceding logical event start. Patch position remains absolute. In the
direct Patch form, the post-PC component is the complete first-Note interval.
In the established extended form, that interval is post-PC plus final timing.
Experiment 031 directly validates the latter ownership.

All additions use checked arithmetic. The encoded values and bytes remain
authoritative; accumulated position is derived. No bar/beat conversion, PPQN
conversion, or MIDI delta generation belongs here.

# Output representation

Return one lossless walk result:

```rust
pub struct MixedEventWalk<'a> {
    pub event_range: Range<usize>,
    pub items: Vec<MixedEventItem<'a>>,
    pub consumed_range: Range<usize>,
}

pub enum MixedEventItem<'a> {
    Event(TimedMixedEvent<'a>),
    Patch(BoundedPatchCore<'a>),
    PatchToNote(BoundedPatchToNoteTransition<'a>),
}

pub struct TimedMixedEvent<'a> {
    pub representation_range: Range<usize>,
    pub timing: MixedEventTiming<'a>,
    pub position: u32,
    pub event: MixedEventKind<'a>,
}

pub enum MixedEventKind<'a> {
    Note(BoundedNoteEvent<'a>),
    ContextMediatedNote(BoundedContextMediatedNoteEntry<'a>),
    DoubleContextMediatedNote(BoundedDoubleContextMediatedNoteEntry<'a>),
    Controller(BoundedControllerRecord<'a>),
    ChannelPressure(ChannelPressureEntry<'a>),
    PitchBend(PitchBendEntry<'a>),
}
```

`BoundedPatchToNoteTransition` is deliberately coupled. It preserves the
existing Patch representation, transition components/context, and first Note
as two logical musical events under one non-overlapping consumed range. This
avoids inventing disjoint source ownership where the existing Patch decoder's
bounded representation includes the first `90`. The walk result exposes a
logical-event iterator/count so authentic expectations still count Patch and
first Note separately.

The standalone `Patch` item is used only where a following ordinary Controller
has independent byte ownership. Its range ends exactly at declared
`payload_end`; the following Controller and direct Note are ordinary timed
items with adjacent, disjoint ranges. The branch validates all three before
adding them to a successful transactional walk.

Every representation preserves its absolute source range, raw timing bytes,
decoded timing, derived position, and family fields. `BoundedNoteEvent` must
also preserve whether `90` was explicit, the three property bytes, duration
VLQ, and their offsets. Existing family structs are reused wherever their
provenance and exact-bound contracts fit.

# Note grammar

Explicit entry:

```text
timing VLQ | 90 | pitch | attack | release | duration VLQ
```

Continuation in Note state:

```text
timing VLQ | pitch | attack | release | duration VLQ
```

The three properties are required data bytes for the supported grammar. The
duration is a bounded one-to-four-byte 7-bit big-endian VLQ. The event ends
exactly at the duration VLQ end. Explicit entry sets Note state; continuation
retains it. Missing properties, high-bit property bytes, truncated/overflowing
duration, or an end beyond `event_end` is an error at the responsible offset.

The current diagnostic `track7::decode_event` is not used directly because it
does not preserve the complete required provenance or explicit-entry status.
Its VLQ/property logic should be factored into a bounded single-Note primitive
shared by diagnostic and mixed walking rather than duplicated.

# Controller grammar

Ordinary Controller is:

```text
timing VLQ | ff 41 05 | five-byte established payload
```

At `ff 41 05`, the walker derives the exact end as timing width plus eight
bytes, checks it against `event_end`, then invokes
`decode_bounded_controller_record` with that exact range. The result already
preserves timing, tag, context, controller number, value, and provenance.
Controller timing is an ordinary delta. Controller clears compact state; the
next loop iteration must use an explicit status or known `ff` branch, and a
data byte with state `None` fails.

# Channel Pressure grammar

Entry:

```text
timing VLQ | d0 | value
```

Continuation in Pressure state:

```text
timing VLQ | value
```

The value is one data byte. Entry replaces state with Channel Pressure;
continuation retains it. A following high-bit branch ends the run dynamically.

The existing exact-run decoder remains public and unchanged in behavior. Its
single-entry validation/construction should be factored into a shared bounded
primitive used both by the run decoder and walker. The walker must not create
or guess a run range.

# Pitch Bend grammar

Entry:

```text
timing VLQ | e0 | LSB | MSB
```

Continuation in Bend state:

```text
timing VLQ | LSB | MSB
```

Both value bytes are data bytes. Entry replaces state with Pitch Bend;
continuation retains it. A following high-bit branch ends the run dynamically.
As with Pressure, factor a shared exact single-entry primitive while preserving
the current caller-bounded run decoder and its tests.

# Patch-to-Note grammar

Supported direct form:

```text
position VLQ
ff 7c | payload_length | payload ending in direct Program Change
post-PC timing VLQ
90 | pitch | attack | release | duration VLQ
```

Supported extended form:

```text
position VLQ
ff 7c | payload_length | payload ending in direct Program Change
post-PC timing VLQ
ff 60 | one-byte context_length | context payload
final timing VLQ
90 | pitch | attack | release | duration VLQ
```

A new bounded Patch-transition helper derives the exact `90` and transition
end from the current cursor. It accepts immediate `90` or exactly one `ff 60`
record followed by one bounded final timing VLQ and `90`. It rejects repeated
contexts, other tags, missing status, or any crossing of `event_end`. Once the
status bound is known, it wraps `decode_bounded_patch_representation`; it does
not duplicate Patch payload/name/Program Change decoding.

The helper then decodes the first Note properties and duration, returns the
coupled `BoundedPatchToNoteTransition`, and establishes Note state. It does not
assign a semantic name to `ff 60` or its payload.

## Patch-to-Controller-to-Note grammar

The additional authenticated Bells Tracks 3/4 form is exactly:

```text
position VLQ
ff 7c | payload_length | payload ending in direct Program Change
Controller timing VLQ | ff 41 | 05 | context[3] | number | value
Note timing VLQ | 90 | pitch | attack | release | duration VLQ
```

The factored Patch core ends at declared `payload_end`. Exactly one ordinary
Controller begins there and ends at its value byte. Exactly one direct explicit
Note begins at that Controller end. The three ranges are disjoint. Patch
position is absolute; Controller position adds its delta to Patch position;
Note position adds its delta to Controller position. The completed branch
leaves Note state.

`ff 60` after the Controller, a second Controller, malformed current bytes,
and any other intervening structure are rejected at the exact current cursor.
No later valid-looking `ff 41` or `90` is searched for or used for recovery.

# Context-mediated Note entry

Track 9 establishes this non-Patch syntax after Controller:

```text
leading timing VLQ
ff 60 | one-byte context_length | context payload
final timing VLQ
90 | pitch | attack | release | duration VLQ
```

Model `ff 60` as observed syntax, not a performance family and not Patch-only
semantics. The established single-context form remains
`BoundedContextMediatedNoteEntry`. Bells Track 6 additionally establishes one
fixed `BoundedDoubleContextMediatedNoteEntry`: exactly a length-`6` context,
inter-context timing, a length-`7` context, final timing, and a direct explicit
Note. The two contexts remain opaque and emit no independent musical events.
The representation is fixed-shape rather than a general repeated-context
collection. Any third context, different length sequence, alternate following
family, malformed current record, or bound crossing is rejected without
scanning or resynchronization.

# Active-state exits

After the timing VLQ:

| Active state | First byte | Result |
|---|---|---|
| `None` | `00..7f` | `DataWithoutActiveState` |
| Note | `00..7f` | one Note continuation |
| Channel Pressure | `00..7f` | one Pressure continuation |
| Pitch Bend | `00..7f` | one Bend continuation |
| any | `ff` | leave compact state; strict known tag/context dispatch |
| any | `90` | replace with Note state |
| any | `d0` | replace with Channel Pressure state |
| any | `e0` | replace with Pitch Bend state |
| any | other high-bit byte | `UnsupportedStatus` |
| any | exact `event_end` before timing | successful stop |

There is no implicit restoration of the state that preceded Controller or
Patch. A tagged record's grammar must establish what follows.

# Error model

`MixedEventWalkError` is a deterministic typed error carrying the current
cursor and relevant observed/expected offsets or bytes. It includes at least:

- `InvalidEventBounds`;
- `TimingVlq(VlqError)` and position-overflow errors;
- `DataWithoutActiveState`;
- `UnsupportedStatus`;
- `UnsupportedFfTag`;
- `MalformedKnownTag`;
- `MalformedNote`;
- `MalformedController(BoundedControllerError)`;
- `MalformedChannelPressure`;
- `MalformedPitchBend`;
- `MalformedPatch(BoundedPatchError)`;
- `PatchContextMismatch`;
- `EventPastBound`;
- `CursorDidNotAdvance`.

Length and range checks precede indexing. A known decoder failure is terminal;
it is never converted into another-family attempt. An error points to the
current structural cursor even when valid-looking bytes occur later.

# Transactional behavior

The first public API returns all items only after exact complete success:

```text
Result<MixedEventWalk, MixedEventWalkError>
```

It does not return partial events as successful recovery and is not initially
an iterator. This makes the guarantee unambiguous: a returned walk consumed
the caller's exact range and every item passed strict validation. Internal
events may be accumulated while parsing but are discarded on error. A future
diagnostic API may report a failing cursor separately without weakening this
contract.

# No-scanning invariant

Every successful advance is derived solely from:

- the exact current cursor and `event_end`;
- active compact state;
- one bounded timing VLQ;
- the byte at its exact end;
- a supported explicit status/tag; and
- that branch's exact bounded grammar.

No code may search for `90`, `d0`, `e0`, `ff 41`, `ff 60`, or `ff 7c`, probe
multiple decoders, or resynchronize after error. Mandatory tests place a
malformed/unsupported branch at the current cursor and a valid-looking event
later; the returned error must remain at the malformed cursor.

# Authentic acceptance fixtures

## Bells for her Track 9

Walk exactly `0x0143c8..0x014957` from the untouched Experiment 007 artifact.
Assert complete consumption and 184 logical events:

- 31 Notes;
- one Patch;
- 120 Controllers;
- 32 Channel Pressure events.

The test must derive the Pressure run dynamically, cover the Patch transition
and the separate `ff 60` context-mediated Note entry, require exact stop before
the seven-byte tail, and supply no family-run bounds.

## Bells for her Track 14

Walk exactly `0x014e26..0x015ed4`. Assert complete consumption and 601 logical
events:

- 227 Notes;
- 272 Controllers;
- 102 Pitch Bend events.

The test must derive all nine Bend runs and their exits, retain the final
Controller, stop exactly at `event_end`, and supply no run bounds.

Authentic offsets are regression assertions, not production locators.

# Synthetic tests

Focused tests must cover:

- empty event range and exact-end stop before attempting a timing VLQ;
- explicit Note entry and Note continuation;
- Note to Controller and Note to Bend;
- Controller to Note, Pressure, and Bend;
- Pressure continuation and Pressure to Note;
- Bend continuation, Bend to Note, and Bend to Controller;
- direct Patch-to-Note;
- Patch-to-one-Controller-to-direct-Note, including disjoint ownership and
  checked two-step timing;
- exactly one `ff 60`-mediated Note entry;
- exactly two `ff 60` contexts with payload lengths `6` then `7`, including
  exact component ownership and checked three-delta timing;
- unsupported `ff` tag and unsupported status;
- data byte with state `None`;
- truncated and overlong timing/duration VLQs;
- family payload crossing `event_end`;
- checked accumulated-position overflow;
- malformed current branch followed by a valid-looking later event, proving
  no scanning or recovery;
- malformed/truncated Patch-following Controller or Note, `ff 60` after that
  Controller, and a second Controller;
- exact returned item/consumed ranges and monotonic cursors for every success.
- wrong context lengths/tags, truncation at every double-context component,
  a third context, alternate following families, compact/alternate Note forms,
  later-signature decoys, and accumulated-position overflow.

# Existing decoder reuse

| Decoder | Decision | Reason |
|---|---|---|
| Note (`track7`) | FACTOR SHARED PRIMITIVE | VLQ/property logic is useful, but the current diagnostic event lacks full provenance and explicit-status representation. |
| Patch | WRAP | A strict transition helper derives the status bound, then the existing bounded decoder preserves Patch fields. |
| Controller | REUSE DIRECTLY | Its exact record width is derivable at the cursor and its decoder already requires exact bounds. |
| Channel Pressure | FACTOR SHARED PRIMITIVE | Current public API requires a complete run; walker needs one exact entry/continuation without weakening the run decoder. |
| Pitch Bend | FACTOR SHARED PRIMITIVE | Same run-bound distinction as Pressure. |

Refactoring must retain all current bounded decoder APIs and regression tests.
No decoder acquires search or fallback behavior.

# Implementation decomposition

One implementation checkpoint is appropriate, internally ordered as:

1. add walker types, errors, exact-bound validation, transactional loop, and
   no-scanning tests;
2. factor bounded Note, Pressure, and Bend single-event primitives and add
   explicit/continuation dispatch;
3. reuse the Controller decoder and add strict state replacement/clearing;
4. add the one-record `ff 60` syntax helper and Patch-to-Note wrapper;
5. add focused synthetic transitions, then authentic Track 9 and Track 14
   acceptance fixtures;
6. document the implemented contract and run the full validation suite.

These are implementation phases, not separate compatibility promises or
required micro-commits.

# Expected implementation files

The implementation task is expected to change:

- `src/mixed_event.rs` (new walker, output, errors, transition helpers);
- `src/lib.rs` (focused module export);
- `src/track7.rs` (shared exact single-Note primitive/provenance support);
- `src/channel_pressure.rs` (shared exact single-entry primitive);
- `src/pitch_bend.rs` (shared exact single-entry primitive);
- `tests/bounded_mixed_event_walker.rs` (synthetic and authentic acceptance);
- `docs/BOUNDED_MIXED_EVENT_WALKER_IMPLEMENTATION.md` (new);
- the narrowly affected durable status documents.

No change to Controller or Patch production modules is expected unless the
implementation demonstrates that a tiny shared boundary helper is required;
such a change must preserve their public behavior and remain within this
design.

# Implementation gate

**READY TO IMPLEMENT.**

The outer bounds, current-cursor classifier, supported family grammars,
state transitions, Patch/context forms, errors, provenance, transactional
behavior, no-scanning invariant, and authentic acceptance criteria are all
explicit. General mixed Studio Vision parsing remains outside this gate.

# Single recommended next step

Implement this exact-bounded walker in one checkpoint, validate it against the
184-event Track 9 and 601-event Track 14 fixtures, and reject every unsupported
branch at its current cursor. Do not begin MIDI export integration until that
implementation passes.
