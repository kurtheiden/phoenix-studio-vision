# Objective

Design, without implementing, a provenance-preserving decoder for one exact
caller-bounded Channel Pressure run using the observed `Bells for her` Track 9
contract.

# Scope and non-goals

Proposed name: `decode_bounded_channel_pressure_run`. The caller supplies an
exact half-open run range and explicitly invokes the observed entry-form
decoder. The decoder does not discover a run, infer its end, scan for `d0`,
`90`, or `ff 41`, resynchronize, parse a whole track, infer MIDI channel, or
accumulate absolute musical time.

This is not an ordinary Controller decoder. Controller records are self-tagged
with `ff 41 05`; pressure uses one tagged run entry followed by state-dependent
compact continuations.

# Caller contract

The caller asserts:

- `run_range.start` is the first timing VLQ of the observed entry form;
- `run_range.end` is exactly after the last pressure value;
- the run begins with `timing VLQ | d0 | pressure_value`;
- every remaining byte belongs to zero or more `timing VLQ | pressure_value`
  continuations.

The decoder validates this assertion locally and consumes only the range.
There is no loose containing bound or search window.

# Parsing state machine

1. Validate nonempty in-input bounds.
2. Decode one bounded one-to-four-byte timing VLQ at the exact start.
3. Require `d0` immediately after it and preserve that byte/offset.
4. Read one direct pressure byte and emit the first ordered entry.
5. Enter internal `ChannelPressureContinuation` state.
6. Until the exact run end, decode one bounded timing VLQ and one direct value,
   emitting an ordered continuation entry.
7. Succeed only when the cursor equals `run_range.end` exactly.

The continuation state is internal proof supplied by successful entry parsing
plus the caller's exact run bound. The decoder never tries continuation parsing
before validating `d0`.

# Continuations are not self-identifying

`timing VLQ | pressure_value` has no independent family discriminator. It is
safe here only because the same decoder already accepted the `d0` run entry and
has not left the exact caller-bounded run. No public API should decode one
arbitrary continuation without this state. A future event walker may carry
active-family state, but it must obtain that state from a justified entry or
equivalent caller evidence.

# Timing contract

Each entry returns its encoded event-start delta as a located VLQ with decoded
value, raw bytes, and absolute source range. Each direct pressure value is a
located byte. The decoder preserves order but does not sum deltas or return
absolute ticks. Accumulation belongs to a future event-stream walker with a
known preceding event start.

# Exact-bound behavior

The decoder never determines the end by looking for another family marker.
Every byte inside the supplied bound must be consumed by entry or continuation
grammar. A bound ending inside a VLQ or before a value fails at that exact
cursor. A one-entry run is structurally accepted when its first value ends at
the bound; natural evidence contains 32 entries but does not establish a
minimum larger than one.

No decoder-level 0–127 rule is imposed on pressure values. This matches current
Phoenix structural decoders and avoids inventing malformed-project semantics.
Consequently, exact run bounds remain essential: arbitrary extra bytes that
happen to form complete timing/value pairs cannot always be distinguished from
continuations by local syntax alone.

For the authentic oversized fixture that includes the following Note bytes,
continuation parsing eventually encounters an incomplete continuation at the
supplied end and rejects it. The decoder does not identify or search for `90`.
More generally, callers—not byte-value heuristics—own run-end correctness.

# Acceptance and rejection

Accept:

- a valid exact run containing only the entry;
- a valid exact multi-entry run;
- one-to-four-byte terminated VLQs, including non-minimal encodings because the
  shared primitive does not enforce canonical width;
- any `u8` direct pressure value at the binary layer.

Reject deterministically:

- invalid, reversed, out-of-input, or empty bounds;
- unterminated or over-four-byte entry timing;
- absent or wrong entry byte where `d0` is required;
- missing first value;
- unterminated or over-four-byte continuation timing;
- missing continuation value;
- a bound ending mid-entry;
- the known oversized authentic bound containing the following Note when it
  cannot be consumed as complete continuation pairs;
- arithmetic overflow in range calculations.

Failures stop at the current cursor. There is no skip, alternate grammar,
retry, or recovery. Since the caller supplies the semantic run boundary, a
syntactically complete but falsely extended pair sequence is not detectable
without additional evidence and must not motivate speculative restrictions.

# Proposed Rust API

Illustrative only, reusing current located provenance concepts:

```rust
use std::ops::Range;

pub struct ChannelPressureRunBounds {
    pub run_range: Range<usize>,
}

pub struct ChannelPressureEntry<'a> {
    pub entry_range: Range<usize>,
    pub timing_delta: LocatedVlq<'a>,
    pub pressure_value: LocatedByte,
}

pub struct ChannelPressureRun<'a> {
    pub run_range: Range<usize>,
    pub entry_tag: LocatedByte,
    pub entries: Vec<ChannelPressureEntry<'a>>,
}

pub fn decode_bounded_channel_pressure_run<'a>(
    bytes: &'a [u8],
    bounds: ChannelPressureRunBounds,
) -> Result<ChannelPressureRun<'a>, BoundedChannelPressureError>;
```

An `Entry`/`Continuation` enum is redundant: `entries[0]` is necessarily the
tagged entry and all later elements are continuations. Keeping `entry_tag` at
run level makes state establishment explicit without duplicating kind data.
No MIDI channel, track, device, instrument, absolute time, payload length, or
opaque context field is included.

# Proposed error model

The future error type should distinguish invalid bounds, entry timing VLQ
failure, missing/wrong entry tag with offset/observed byte, missing first value,
continuation timing failure with entry index/cursor, missing continuation value
with index/offset, and checked-offset overflow. Reuse `VlqError` and current
located types where clean. Error order follows the state machine.

# Authentic future fixtures

Fixed-offset tests should read the untouched baseline without scanning:

- complete `0x1478c..0x147ce` run: exactly 32 ordered entries and exact total
  consumption;
- first entry: `82 20 d0 01`, delta 288, value 1, exact timing/tag/value ranges;
- representative one-byte continuations, including deltas 8, 7, 27, and 42;
- final continuation `0a 00`, delta 10, value 0, ending exactly at `0x147ce`;
- preceding Controller `0x14783..0x1478c`, CC1=0, proving adjacency;
- following Note beginning `0x147ce` with delta `83 56` = 470, `90`, and
  matching properties, proving the exclusive run end.

Synthetic tests should cover every rejection above, plus a valid single-entry
run and valid multi-entry run. Tests must pass fixed bounds directly and never
locate `d0` or neighboring markers by scanning.

# Event-stream implications

This run proves that at least one family uses an explicit entry discriminator
and compact same-family continuations. A future walker therefore needs explicit
active-family/run state rather than assuming every event is self-describing.
This does not establish a universal Studio Vision running-status system.

The following `timing VLQ | 90 | Note properties` proves an explicit marker at
this Pressure-to-Note transition. It makes the end of this known run externally
clear, but does not prove that all transitions into Notes carry `90`, that
consecutive Notes repeat it, or that generic Note classification is solved.

# Implementation gate

**YES.** The natural evidence is sufficient to implement the bounded,
state-aware decoder for this observed run contract without a controlled
experiment. This gate does not cover isolated pressure events, arbitrary
continuation decoding, run discovery, generic mixed walking, or universal
status semantics.

# Unknowns

Unknowns include isolated/re-entry forms, whether `d0` is invariant, other
transition families, global state rules, broader artifacts, and syntactically
valid falsely extended caller bounds. Note/Patch mixed-stream blockers remain.

# Single recommended next step

Implement `decode_bounded_channel_pressure_run` with exact-bound consumption,
explicit entry state, authentic Track 9 fixtures, and focused malformed-input
tests.
