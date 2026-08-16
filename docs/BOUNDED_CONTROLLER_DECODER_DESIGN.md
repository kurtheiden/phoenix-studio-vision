# Objective

Design, without implementing, a strict decoder for one caller-bounded ordinary
Studio Vision Controller record supported by the `Bells for her` evidence.

# Scope and boundary contract

Proposed name: `decode_bounded_controller_record`. The caller supplies an exact
half-open `record_range`: its start is the timing VLQ and its end is one byte
after the controller value. It must satisfy `start < end <= bytes.len()`.

The decoder consumes only that range. It does not search for `ff 41`, infer
bounds, retry, skip bytes, or resynchronize. Discovery, container parsing,
event ordering, and timeline accumulation remain caller responsibilities.

# Supported grammar

```text
timing VLQ | ff 41 | 05 | context[3] | controller_number | controller_value
```

Return the timing value and raw width/range, tag range, located payload-length
byte, payload range, byte-exact context/range, located number/value bytes, and
complete record range. Offsets are absolute in the input, matching bounded
Patch provenance conventions.

# Strict acceptance and rejection

Acceptance requires:

1. Valid in-input caller bounds.
2. A bounded one-to-four-byte 7-bit big-endian VLQ terminating in the range.
3. The next two bytes exactly `ff 41`.
4. The next byte exactly `05`.
5. Exactly five payload bytes: context[3], number, and value.
6. The value ending exactly at `record_range.end`.

Exact consumption rejects trailing bytes in an oversized bound. An undersized
bound yields truncation. Reject unterminated or greater-than-four-byte timing,
missing/truncated/wrong tag, missing/wrong length, truncated payload, overflow,
and every read outside the supplied range. Non-minimal terminated VLQs remain
accepted because current shared mechanics do not establish canonical encoding
as a validity rule.

Accept every structural `u8` number and value. MIDI's nominal 0–127 constraint
belongs to a separate semantic consumer; imposing it here would invent an
unsupported project-format validity rule.

# Timing contract

`timing_delta.value` is the encoded delta from the previous Studio Vision event
**start** in the ordered track stream. Its raw bytes and absolute range preserve
provenance. It is not an absolute tick, musical position, or delta from the
previous Controller specifically.

A container walker may accumulate absolute event starts. The bounded decoder
must not accept or manufacture that state.

# Opaque context policy

`context` is exactly three borrowed bytes and an absolute range, returned
losslessly and never interpreted. Observed Track 3/4/6/9/14 associations (`00
23 00`, `00 05 00`, `00 02 00`, `00 1f 00`, `00 01 00`) are fixture values,
not channel, track, instrument, device, or identifier fields.

# Ordinary Controller versus Patch bank state

This decoder accepts only an exact `ff 41 05` ordinary Controller record.
CC0/CC32 stored in a Patch representation remain Patch state even when SMF
export emits MIDI Control Changes. MIDI controller number alone must never
route Patch-derived bank state into this decoder.

# Proposed Rust representation

Illustrative only:

```rust
use std::ops::Range;

pub struct ControllerRecordBounds {
    pub record_range: Range<usize>,
}

pub struct BoundedControllerRecord<'a> {
    pub record_range: Range<usize>,
    pub timing_delta: LocatedVlq<'a>,
    pub event_tag_range: Range<usize>,
    pub payload_length: LocatedByte,
    pub payload_range: Range<usize>,
    pub context: LocatedBytes<'a>,
    pub controller_number: LocatedByte,
    pub controller_value: LocatedByte,
}

pub fn decode_bounded_controller_record<'a>(
    bytes: &'a [u8],
    bounds: ControllerRecordBounds,
) -> Result<BoundedControllerRecord<'a>, BoundedControllerError>;
```

Reuse or generalize existing `LocatedVlq`, `LocatedByte`, and `LocatedBytes`
concepts. Do not add channel, absolute position, track identity,
controller-specific enums, or Patch/bank fields.

# Proposed error model

Distinguish invalid bounds, timing VLQ failure, missing/wrong tag (with offset
and observed bytes), missing/wrong payload length, truncated payload (with
required/available extent), and trailing bytes/exact-consumption failure. Reuse
`VlqError` where compatible. Fail in grammar order after validating bounds and
never probe beyond the range.

# Future validation fixtures

Authentic fixtures should cover:

- Track 3 CC7=127, context `00 23 00`, multi-byte delta 480;
- Track 4 CC7=127, context `00 05 00`, single-byte delta 28;
- Track 6 CC7=127, context `00 02 00`, multi-byte delta 130;
- Track 9 multiple CC7/CC1 records, including delta zero, single- and
  multi-byte timing, and caller-side Controller/Note interleaving;
- Track 14 CC1 records with context `00 01 00`.

Synthetic negatives should cover invalid/out-of-input bounds, unterminated and
over-four-byte timing, wrong/truncated tag, wrong/missing `05`, truncation in
each payload position, and a valid record with extra bytes inside its bound.
Assert semantic values plus exact ranges/raw bytes. No fixture should locate a
record by scanning.

# Generalization boundary

Strong evidence supports 395 natural CC1/CC7 records across the identified
tracks. It does not prove every CC number or every Studio Vision project,
version, or device configuration. A generic number/value record is still the
narrowest faithful design: CC1 and CC7 vary only in fields demonstrated to
carry number/value. Separate decoders would add unsupported distinctions.

# Implementation gate

**YES.** Evidence is sufficient for a bounded ordinary Controller decoder
without another controlled experiment. This does not authorize discovery,
whole-track parsing, context semantics, or universal format claims.

# Single recommended next step

Implement `decode_bounded_controller_record` with exact-bound consumption and
add the authentic and malformed regression fixtures above.
