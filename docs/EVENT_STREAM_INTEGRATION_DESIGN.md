# Objective

Define the narrowest evidence-supported caller/container contract for walking
an already-bounded Studio Vision event region in order, accumulating event-start
time, and stopping whenever the next event cannot be classified and bounded at
the current cursor. This is a design only; it does not authorize discovery,
scanning, recovery, or a generic track parser.

# Existing bounded decoders

## Note

`track7::decode_event(bytes, cursor, end)` treats `cursor` as the start of a
timing VLQ and `end` as a containing upper bound. It decodes:

```text
timing VLQ | pitch | attack | release | duration VLQ
```

It returns semantic values, absolute offsets, VLQ widths, and a derived next
cursor. It does not require an exact per-event end, return raw borrowed ranges,
validate a Note status/type marker, or prove that a structurally plausible
candidate is a Note. `walk_chain` repeats this grammar only because the caller
asserts an evidence-bounded consecutive Note chain. For validated consecutive
Notes, the leading timing value equals the later Note's start-to-start interval.
The first Note's leading timing ownership and general mixed-event use remain
incomplete.

## Patch

`patch::decode_bounded_patch_representation` still requires the exact absolute-
position VLQ start and an exclusive caller boundary immediately after a known
first-Note `90` status. It decodes the absolute Patch position, `ff 7c`, the
payload through direct PC, a neutral post-PC VLQ component, preserved trailing
bytes, and the terminal Note-status transition.

Experiment 031 now supplies a structural caller-side derivation for the
established corpus. After the post-PC timing VLQ, either `90` follows directly,
or one length-framed `ff 60 | u8 length | payload` record is followed by a
final timing VLQ and `90`. The current production decoder remains unchanged;
a future integration layer may derive `note_status_end` with this strict
grammar and must reject unsupported optional forms rather than scan.

## Ordinary Controller

`controller::decode_bounded_controller_record` requires one exact half-open
record range. It returns a located encoded event-start delta, tag/payload
provenance, opaque context, number/value, and complete record range. It consumes
the bound exactly. Once the current cursor is known, the walker can derive the
end as `cursor + timing_width + 8` after bounded VLQ decoding and exact
`ff 41 05` validation. No scan or external per-record end is structurally
needed.

# Record decoder versus stream walker

A record decoder receives an already justified record representation and
decodes only its local fields. It neither chooses its type nor owns timeline or
container state.

An event-stream walker receives an exact containing region plus initial timing
state. It owns the cursor, classifies only at that cursor, derives a justified
next boundary, calls the appropriate bounded decoder, commits the new cursor
and timing state only after success, and stops at the first unsupported
structure. It must not use decoder failure as permission to try other offsets.

# Timing-state model

The walker state should conceptually contain:

```text
region: exact caller-supplied half-open range
cursor: next event start, initially region.start
previous_event_start: caller-supplied absolute time, or None at a proven origin
decoded events: ordered results with record ranges and event-start provenance
```

Use checked accumulation and fail without changing state on overflow.

- Controller: `event_start = previous_event_start + timing_delta`. At a proven
  track origin, a zero first delta can establish zero under an explicit caller
  origin contract. The decoder itself remains delta-only.
- Consecutive Note chain: for Notes whose leading timing ownership is already
  established, add the interval to the previous Note start. This is valid only
  inside a caller-asserted, evidence-backed Note chain; it is not yet a generic
  mixed-stream rule.
- Patch: its leading position is absolute, so set the Patch event start to that
  decoded value rather than adding it. Do not use `post_pc_timing_component`
  alone to position the following Note. Track 3 #2 and Track 9 demonstrate
  compound Patch-to-first-Note timing.

No single uniform `delta()` trait should be imposed across these families.

# Current-cursor event discrimination

The classifier may decode one bounded VLQ at `cursor`, then inspect only bytes
immediately following that VLQ. It must not search forward.

- **Controller — SUPPORTED.** Exact following bytes `ff 41 05` identify the
  known ordinary Controller grammar and its fixed remaining width.
- **Channel Pressure — PARTIAL.** After a timing VLQ, adjacent `d0` identifies
  the one established run entry. Its later `timing VLQ | value` continuations
  are classifiable only while explicit Channel Pressure run state is active
  inside the independently bounded run; they are not stateless records.
- **Pitch Bend — PARTIAL.** After a timing VLQ, adjacent `e0` identifies all
  nine observed run entries. The 93 `timing VLQ | LSB | MSB` continuations are
  classifiable only under active Pitch Bend state inside one of the nine exact
  caller-known run bounds. No continuation identifies the run end.
- **Patch — SUPPORTED FOR THE ESTABLISHED TRANSITION GRAMMAR.** Exact following
  bytes `ff 7c` identify Patch after its absolute-position VLQ. Payload length,
  post-PC VLQ, immediate `90` or one validated length-framed `ff 60` extension,
  final VLQ, and `90` derive the next cursor without scanning. Other optional
  forms remain unsupported.
- **Note — UNRESOLVED.** After the timing VLQ, the next byte is pitch/property
  data, not a dedicated `ff` marker. Range plausibility for pitch, velocities,
  and duration is validation inside a known Note chain, not a collision-free
  event-type discriminator in a mixed stream. A leading `90` exists at some
  Patch-to-first-Note transitions, but ordinary consecutive Notes omit it and
  its ownership does not define a generic Note record.

Trying each decoder at one cursor would still be heuristic classification when
grammars can accept overlapping byte shapes; therefore fallback dispatch is
not permitted.

# Unknown-event behavior

If exact current-cursor classification or end derivation is unsupported, return
an error such as:

```text
UnsupportedEvent {
    cursor,
    previous_event_start,
    timing_prefix: optional located VLQ,
    inspected_prefix: bounded raw bytes,
    raw_remainder: cursor..region.end,
}
```

The cursor and accumulated time remain unchanged. `raw_remainder` may borrow
only the already supplied containing region. It is diagnostic preservation,
not a search space. There is no skip, retry, alternate-offset probe, or
resynchronization.

# Track 9 mixed-stream case study

The established `Bells for her` Track 9 event span is
`0x143c8..0x14957` and represents 184 Studio Vision List events: 31 Notes, one
Patch, 120 ordinary Controllers, and 32 Channel Pressure events.

At `0x143c8`, the walker can derive and decode the zero-delta CC7 record through
`0x143d1`, committing absolute event start zero under the known track-origin
contract. At `0x143d1`, `8f 00 | ff 7c` identifies the Patch and its absolute
position 1,920. The transition then uses post-PC `87 99 02` = 117,890, one
length-framed `ff 60 07` context record, final timing `81 40` = 192, and `90`
at `0x143fc`. The summed 118,082 units equal the independently established
Patch-to-first-Note interval. Experiment 031's controlled +1 result confirms
the same timing ownership in the other established extended form.

Thus the established grammar now derives the first Note status and timing
without scanning. Independently bounded event-order correlation establishes
the Channel Pressure run at `0x1478c..0x147ce`: entry `82 20
d0 01`, followed by 31 timing/value continuations, with 32/32 timing and value
matches. A bounded run decoder can safely process that region with active-family
state even though a generic walker cannot autonomously reach it.

The run exits through `83 56 90 ...`, matching delta 470 to the following Note.
This proves an explicit `90` at this Pressure-to-Note transition and makes that
specific boundary strong. It suggests that some family transitions may carry
status-like markers. It does not prove `90` on every Note transition or on
consecutive Notes, so the generic Note-discriminator conclusion is unchanged.

# Note boundary assessment

**PARTIAL.** Within an explicitly asserted consecutive Note-chain range,
timing/property/duration widths derive the next cursor exactly, and the existing
walker has extensive sequential validation. A generic mixed-event walker
cannot safely decide that the bytes after a timing VLQ are a Note rather than
another untagged or unknown family. Experiment 031 resolves the special first
Note after the established Patch forms, but not ordinary Note-state exit.

Mixed walking still requires a collision-resistant state-exit/current-family
rule after the timing prefix. Property plausibility is insufficient.

# Patch boundary assessment

**SUPPORTED FOR THE ESTABLISHED BOUNDED GRAMMAR.** `ff 7c`, payload length,
post-PC timing, and either immediate `90` or one length-framed `ff 60` record
plus final timing derive `note_status_end` without scanning. Direct forms use
the post-PC timing as the complete interval. Extended forms sum post-PC and
final timing; Experiment 031 confirms `81 25 -> 81 26` owns the +1 Note edit.
The existing decoder API remains caller-bounded. Other tags, repeated context
records, and broader Patch layouts must be rejected.

# Controller boundary assessment

**SUPPORTED.** From the current cursor, a bounded VLQ yields its width; exact
`ff 41 05` validation then fixes the record end at `cursor + timing_width + 8`.
The existing decoder should continue receiving that derived exact bound rather
than weakening its contract. This supports a Controller-only walker inside an
exact Controller-only region.

# Integration-readiness matrix

| Walker | Readiness | Boundary |
|---|---|---|
| Controller-only bounded sequence | **YES** | Exact region and initial timing state required; each record length is derivable. |
| Exact-bounded Channel Pressure run | **YES** | Entry `d0` establishes state; continuations are safe only inside the caller-known run. |
| Exact-bounded Pitch Bend run | **YES** | Entry `e0` establishes state; two-byte continuations are safe only inside the caller-known run. |
| Consecutive Note-chain | **YES** | Only for a caller-asserted evidence-backed Note chain with a known start/end and initial timing basis. |
| Mixed Note + Controller | **NO** | Controller is tagged, but Note has no proven collision-free current-cursor discriminator in a mixed region. |
| Mixed Patch + Note + Controller | **PARTIAL** | Patch handoff is derivable for the established grammar; generic Note/state exit remains unresolved. |
| Full Track 9 including Channel Pressure | **PARTIAL** | Patch handoff and outer bounds are known; Pressure exit and generic current-cursor state classification remain unresolved. |

The bounded mixed-event walker remains **PARTIAL and not implementation-ready**
until current-cursor state exit is established.

# Highest-value blocker

The single broadest blocker is current-cursor state-exit classification after a
timing VLQ. Patch-to-first-Note is no longer part of this blocker. Evidence is
still needed to distinguish established Note, Channel Pressure, and Pitch Bend
continuations from a new tagged/status-bearing family without heuristic
fallback.

# Proposed architecture

Keep the existing record decoders unchanged. A future integration layer should
have:

1. an immutable exact event-region bound and explicit initial timing basis;
2. transactional walker state (`cursor`, `previous_event_start`);
3. a current-cursor classifier that decodes at most the immediate timing prefix
   and fixed adjacent discriminator bytes;
4. explicit active-family/run state for justified compact continuations, never
   inferred from arbitrary untagged bytes;
5. family-specific boundary derivation before invoking an exact-bound decoder;
6. a decoded-event enum carrying family result, absolute event start, and the
   source range used to derive that start;
7. a terminal unsupported-event result preserving the bounded remainder.

Currently, a Controller-only profile, the existing explicitly asserted
Note-chain profile, the implemented exact-bounded Channel Pressure decoder,
and the implemented exact-bounded Pitch Bend run decoder meet this contract.
Both stateful families require an explicit active-family mode and exact run bounds.
Do not expose a generic mixed profile until current-cursor state exit is
established. Patch absolute timing and its direct/extended first-Note interval
must remain family-specific updates, not be coerced into one uniform delta.

# Sequence-level Meter and Tempo separation

The established initial Meter and Tempo representations belong to one
sequence-level Meter/Tempo structural area, not this performance-event stream.
A future sequence parser may supply exact eight-byte Meter and seven-byte Tempo
bounds to their independently implemented bounded decoders. The mixed walker
must not search for `ff 58` or `ff 51`, treat either leading zero as a
performance-event delta, parse unresolved secondary copies, or fold
Meter/Tempo into Note, Controller, Channel Pressure, or Pitch Bend state.
General Meter-map and Tempo-map walking remain unsupported.

# Container-boundary correlation

All 18 authenticated sequences now have exact length-derived sequence record
streams. A checked top-level record walk starts after the opaque eight-byte
root header, consumes Experiment 007 exactly through EOF, and reaches the first
type-`0x01` sequence preamble at `0x006abc` without signature scanning. The
sequence layer can then derive initial Meter and Tempo ranges at primary-record
payload `+14` and exact type-`0x02` containing ranges for ordered track
primaries. This resolves hard-coded per-sequence and per-track containing
offsets inside the bounded chain.

Track-primary bounds plus follow-up tail correlation now supply the exact
performance-event end for the authenticated 166-byte profile. All 132 track
primaries end in `ff aa bb cc ff 2f 00`, and all 15 zero-count tracks contain
only this seven-byte structure after payload `+14`. After validating the tail,
the event region is payload `+14 .. payload.end - 7`. The variable middle
bytes remain opaque, and this rule is not established for the older 120-byte
profile.

This outer boundary does not solve internal dispatch. Experiment 031 resolves
Patch-to-first-Note for the established grammar, but stateful Pressure/Bend run
ends and generic current-cursor state exit remain partial. A future integration
layer must not treat successful container walking as permission to scan or
heuristically dispatch inside a track.

# Evidence supported

- Controller classification, length derivation, and event-start delta
  accumulation are ready inside a Controller-only bounded region.
- Consecutive known Notes have derivable local ends and validated start-to-start
  intervals inside asserted Note chains.
- `ff 7c` identifies the Patch family locally and Patch position is absolute.
- Direct and one-`ff 60` extended Patch transitions derive first-Note status
  and interval without scanning for the established corpus.
- The exact Channel Pressure run uses one explicit `d0` entry and 31 compact
  state-dependent continuations; all 32 timing/value pairs agree.
- Nine exact Pitch Bend runs use explicit `e0` entries and 93 compact
  state-dependent continuations; all 102 timing/LSB/MSB tuples agree.
- The specific following Note transition contains explicit `90`, without
  establishing a universal Note-transition rule.
- Unknown structures can be preserved and reported without moving the cursor.
- Track 9 demonstrates that resolving Patch handoff still does not supply a
  mixed walker: later state exits require their own current-cursor grammar.

# Unknowns

Unknowns include current-cursor Note/Pressure/Bend state exit, isolated or
re-entered Channel Pressure and Pitch Bend forms, universal family-state rules,
internal run-end discovery, `ff 60` semantics, other optional Patch context
forms, and broader-version generality. The outer event-region end and
established Patch handoff are resolved; stateful-run bounds are not.

# Single recommended next step

Perform read-only current-cursor state-exit correlation after timing VLQs
across established Note, Channel Pressure, and Pitch Bend transitions. Generic
mixed walking remains out of scope until that shared boundary is resolved. The
implementation profile remains
limited to the established 166-byte descriptor form; the older 120-byte form
is not selected from an opaque root word.
