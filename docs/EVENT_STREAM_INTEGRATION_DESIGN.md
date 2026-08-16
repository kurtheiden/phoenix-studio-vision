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

`patch::decode_bounded_patch_representation` requires the exact absolute-
position VLQ start and an exclusive caller boundary immediately after a known
first-Note `90` status. It decodes the absolute Patch position, `ff 7c`, the
payload through direct PC, a neutral post-PC VLQ component, opaque regions, and
the terminal Note-status transition. It returns borrowed raw fields and
absolute ranges.

The local payload end is derivable, but the complete supplied representation
end is not: variable opaque pre-Note bytes make `note_status_end` caller
knowledge. The returned range also includes the first Note status, so it is not
a proven standalone Patch-event ownership boundary. The post-PC component is
not always the whole Patch-to-first-Note interval.

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
- **Patch — PARTIAL.** Exact following bytes `ff 7c` identify the observed
  Patch family after its absolute-position VLQ, and the payload end is
  derivable. The variable pre-Note tail and complete representation/event end
  are not derivable without the caller-supplied `note_status_end`.
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
`0x143c8..0x14956` and represents 184 Studio Vision List events: 31 Notes, one
Patch, 120 ordinary Controllers, and 32 Channel Pressure events.

At `0x143c8`, the walker can derive and decode the zero-delta CC7 record through
`0x143d1`, committing absolute event start zero under the known track-origin
contract. At `0x143d1`, `8f 00 | ff 7c` identifies the Patch and its absolute
position 1,920. The shared Patch decoder, however, requires a caller-known
boundary after the first Note status at `0x143fc`; a track-region-only walker
cannot derive that boundary because opaque pre-Note context is variable.
Moreover, the bytes between PC and that Note encode a compound timing
relationship, so the following Note's absolute start cannot be obtained from
the returned neutral post-PC component alone.

Therefore a walker limited to Patch, Note, and Controller decodes exactly the
initial Controller and then stops at the Patch start `0x143d1`. It does not
reach a Channel Pressure event. Current evidence establishes 32 such exported
events but not the exact project offset or grammar of the first one, so no
first-pressure offset is recorded.

# Note boundary assessment

**PARTIAL.** Within an explicitly asserted consecutive Note-chain range,
timing/property/duration widths derive the next cursor exactly, and the existing
walker has extensive sequential validation. A generic mixed-event walker
cannot safely decide that the bytes after a timing VLQ are a Note rather than
another untagged or unknown family. It also lacks a general rule for the first
Note after a Patch, where status and compound timing occur outside the ordinary
consecutive-Note shape.

Mixed walking requires a collision-resistant Note discriminator or independently
established container metadata that declares the next type, plus a precise
Patch-to-first-Note ownership/timing handoff. Property plausibility is
insufficient.

# Patch boundary assessment

**PARTIAL.** `ff 7c`, payload length, name length, and payload end support exact
local semantic parsing. They do not derive the variable post-PC/pre-Note span
or a standalone Patch end. The current API deliberately asks the caller for
`note_status_end` and returns a representation that owns the transition status.
That is safe for bounded decoding but insufficient for autonomous mixed-stream
continuation.

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
| Consecutive Note-chain | **YES** | Only for a caller-asserted evidence-backed Note chain with a known start/end and initial timing basis. |
| Mixed Note + Controller | **NO** | Controller is tagged, but Note has no proven collision-free current-cursor discriminator in a mixed region. |
| Mixed Patch + Note + Controller | **NO** | Adds non-derivable Patch-to-first-Note boundary and compound timing ownership. |
| Full Track 9 including Channel Pressure | **NO** | Patch/Note handoff already blocks progress; Channel Pressure project grammar is also unknown. |

# Highest-value blocker

The single broadest blocker is a generic Note boundary/type discriminator in a
mixed event stream, including the special first-Note handoff after Patch.
Notes dominate known event populations, and their property shape can derive an
end only after Note identity has been justified. Without that justification,
mixed Note/Controller walking would turn plausibility into heuristic parsing.

# Proposed architecture

Keep the existing record decoders unchanged. A future integration layer should
have:

1. an immutable exact event-region bound and explicit initial timing basis;
2. transactional walker state (`cursor`, `previous_event_start`);
3. a current-cursor classifier that decodes at most the immediate timing prefix
   and fixed adjacent discriminator bytes;
4. family-specific boundary derivation before invoking an exact-bound decoder;
5. a decoded-event enum carrying family result, absolute event start, and the
   source range used to derive that start;
6. a terminal unsupported-event result preserving the bounded remainder.

Initially, only a Controller-only profile and the existing explicitly asserted
Note-chain profile meet this contract. Do not expose a generic mixed profile
until the Note discriminator/handoff is established. Patch absolute timing must
remain a family-specific update, not be coerced into delta accumulation.

# Evidence supported

- Controller classification, length derivation, and event-start delta
  accumulation are ready inside a Controller-only bounded region.
- Consecutive known Notes have derivable local ends and validated start-to-start
  intervals inside asserted Note chains.
- `ff 7c` identifies the Patch family locally and Patch position is absolute.
- Unknown structures can be preserved and reported without moving the cursor.
- Track 9 demonstrates why record decoders alone do not supply a mixed walker:
  the first Patch-to-Note transition already requires external boundary and
  compound-timing knowledge.

# Unknowns

Unknowns include a collision-resistant mixed-stream Note discriminator,
first-Note ownership after Patch, complete Patch end, compound Patch-to-Note
timing, Channel Pressure and Pitch Bend project grammars, exact Track 9 first
pressure offset, and general track-container framing. The event-region bound
itself remains caller evidence, not something this design discovers.

# Single recommended next step

Perform a read-only, event-by-event correlation of the known Track 9 Note,
Controller, Patch, and exported Channel Pressure order against the already
bounded `0x143c8..0x14956` project span, specifically to identify the first
non-Controller/non-Note record at the exact expected event position and test
whether its current-cursor bytes provide an explicit discriminator and length.
This natural mixed-stream evidence is higher-information than a controlled
edit and may expose the missing boundary convention without Experiment 030.
