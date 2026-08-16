# Objective

Design, without implementing, a provenance-preserving decoder for one exact
caller-bounded Pitch Bend run using the observed `Bells for her` Track 14
contract.

# Scope and non-goals

Proposed name: `decode_bounded_pitch_bend_run`. The caller supplies one exact
half-open run range and explicitly requests the observed `e0` entry form. The
decoder does not discover or infer runs, scan, resynchronize, walk mixed
events, infer MIDI channel, assign universal status semantics, or accumulate
absolute musical time.

# Caller contract

The caller supplies the complete project byte slice and
`PitchBendRunBounds { run_range }`. The range must begin at the entry timing
VLQ and end immediately after the final continuation MSB. Its correctness is
external evidence. Invocation asserts that the expected family is the
observed stateful Pitch Bend run form.

# Proposed Rust representation

Use existing `LocatedVlq`, `LocatedByte`, and absolute half-open `Range<usize>`
conventions:

```rust
pub struct PitchBendRunBounds {
    pub run_range: Range<usize>,
}

pub struct PitchBendEntry<'a> {
    pub entry_range: Range<usize>,
    pub timing_delta: LocatedVlq<'a>,
    pub pitch_lsb: LocatedByte,
    pub pitch_msb: LocatedByte,
}

impl PitchBendEntry<'_> {
    pub fn raw_value(&self) -> u16 {
        u16::from(self.pitch_lsb.value)
            + (u16::from(self.pitch_msb.value) << 7)
    }
}

pub struct PitchBendRun<'a> {
    pub run_range: Range<usize>,
    pub entry_tag: LocatedByte,
    pub entries: Vec<PitchBendEntry<'a>>,
}
```

The two located bytes are authoritative. `raw_value()` is a derived
convenience implementing the established combination and does not replace raw
provenance. The natural population contains seven-bit bytes; the binary layer
should preserve the complete stored bytes and should not invent an
above-center restriction or use signed/centered value as primary storage.

Do not add MIDI channel, track, device, instrument, absolute tick, or a generic
running-status abstraction.

# Entry decoding

At `run_range.start`:

1. decode one bounded one-to-four-byte 7-bit big-endian timing VLQ;
2. require exact byte `e0` immediately after timing;
3. read one direct LSB byte and one direct MSB byte;
4. preserve the timing bytes/value/range, tag byte/offset, each data byte and
   offset, and the complete entry range;
5. enter Pitch Bend continuation state.

If the MSB ends exactly at `run_range.end`, accept a valid one-entry run.

# Continuation decoding

While the cursor is before the exact run end:

1. decode a bounded one-to-four-byte timing VLQ;
2. read exactly one LSB and one MSB;
3. preserve all field and entry provenance;
4. append the entry in order and advance exactly.

Continuation parsing is private to the successful run decoder. There is no
public arbitrary-continuation function. `timing | LSB | MSB` is safe only
under state established by `e0` and inside the exact caller-known bound.

# Timing contract

Each entry returns its encoded event-start delta and located raw timing bytes.
The decoder does not manufacture absolute position. A future container/walker
may checked-add deltas to its previous event-start state after successful
decoding; that state is outside this API.

# Exact-bound behavior

The supplied range is authoritative. The decoder never searches for `90`,
`ff 41`, `e0`, or another apparent family marker and never retries at another
offset. It parses the entry and then continuation grammar until the cursor is
exactly equal to the supplied end. Any incomplete field or timing failure is a
deterministic error.

Exact consumption validates a correct bound but cannot prove an arbitrary
oversized bound is semantically correct when unrelated bytes happen to fit the
continuation grammar. That is why bounds remain caller evidence. The known
oversized authentic Note and Controller fixtures should be chosen so they fail
naturally under this grammar; no next-family recognition is permitted.

# Acceptance rules

Accept only when:

- `start < end <= bytes.len()`;
- entry timing is a terminating one-to-four-byte bounded VLQ;
- the adjacent entry tag is exactly `e0`;
- entry LSB and MSB are both present;
- every remaining entry has a valid bounded timing VLQ plus two bytes;
- at least one entry exists; and
- parsing ends exactly at `run_range.end`.

A correct one-entry run and a correct multi-entry run are both valid. Do not
reject values above the observed center solely because they are absent from
this population. Preserve data bytes losslessly.

# Rejection and error model

Use deterministic variants consistent with `BoundedChannelPressureError`:

```text
InvalidBounds { start, end, size }
EntryTiming(VlqError)
MissingEntryTag { offset }
WrongEntryTag { offset, observed, expected }
MissingEntryLsb { offset }
MissingEntryMsb { offset }
ContinuationTiming { entry_index, cursor, source }
MissingContinuationLsb { entry_index, offset }
MissingContinuationMsb { entry_index, offset }
OffsetOverflow { offset }
```

The continuation index should use the same zero-based convention as Channel
Pressure. Errors preserve the first failing cursor or field offset. There is no
fallback parsing.

Required negative cases are empty, reversed, and out-of-input bounds;
unterminated and overlong entry timing; missing/wrong `e0`; missing entry LSB
or MSB; truncated and overlong continuation timing; missing continuation LSB
or MSB; and bounds ending mid-entry. Fixed authentic oversized fixtures should
include run 1 extended through its following Note and run 9 extended through
its following Controller. They must fail through timing/data completeness or
VLQ rules, not because the decoder recognizes `90` or `ff 41` as transitions.

# Authentic fixture plan

Tests should read the untouched baseline at its fixed path and use literal
ranges, never runtime scanning.

- Run 1, `0x1541f..0x15439`: 8 entries; first delta 1,361; first bytes after
  timing `e0 3f 3f`; first raw value 8,127; final raw value 0; following Note
  independently begins at `0x15439`.
- Run 3, `0x154bb..0x15524`: 34 entries; one- and two-byte timings, including
  delta 1,586; direct center `00 40` = 8,192; long continuation coverage.
- Run 9, `0x158aa..0x158d8`: 15 entries; first raw value 0; final raw value
  8,192; exact exit to ordinary CC1=7 at `0x158d8`.
- All nine exact ranges: event counts `8, 6, 34, 10, 5, 6, 12, 6, 15`, totaling
  102; validate every timing/value pair where practical, all `e0` offsets,
  contiguous ordered entry ranges, and exact consumption.

Synthetic positives should cover one entry, multiple entries, one- and
multi-byte timing, and preserved data bytes without a center restriction.
Synthetic negatives should cover every error category above. Continuation-form
bytes alone must fail because the entry `e0` is required.

# Comparison with Channel Pressure architecture

Proven Channel Pressure and Pitch Bend grammars differ in entry tag and data
width but share explicit family entry, compact continuations, event-start VLQ
timing, and exact caller-known run ends. This independently strengthens a
possible broader stateful-family convention.

Do not generalize the implementations yet. A shared abstraction could obscure
family-specific evidence, error names, payload width, and future variant
findings. Small deliberate duplication of the Channel Pressure architecture is
safer until another integration requirement proves a lossless common contract.

# Event-stream implications

A future walker positioned at an independently known run start can decode the
timing VLQ and inspect adjacent `e0` without scanning. Once the caller also
provides the exact run end, explicit active-family state makes continuations
deterministic. The walker still cannot discover the run end internally.

Pitch Bend is a second independent reason for future walker state to include an
active family/run mode. It does not solve generic Note discrimination,
Patch-to-first-Note ownership/timing, or generic mixed walking.

# Unsupported behavior and unknowns

Unsupported behavior includes discovery, isolated Pitch Bend decoding,
automatic end detection, absolute-time accumulation, MIDI conversion/output,
channel inference, generic running status, mixed walking, CLI output, and
project-wide integration. Unknowns remain those listed in
`PITCH_BEND_EVENT_REPRESENTATION.md`.

# Implementation gate

**YES.** The nine exact natural runs establish entry framing, continuation
state, direct byte/value mapping, timing, and caller-known boundaries with
102/102 agreement. No controlled experiment is required for this observed
contract.

# Single recommended next step

Implement `decode_bounded_pitch_bend_run` with exact consumption, provenance,
all nine fixed authentic run fixtures, and focused malformed-input coverage.
Do not add discovery or generic mixed-event walking.
