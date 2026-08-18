# Objective

Design, without implementing, a provenance-preserving decoder for one exact
caller-bounded Studio Vision initial Tempo representation.

# Scope and non-goals

Proposed name: `decode_bounded_initial_tempo`. The caller supplies the complete
input and one exact seven-byte half-open range. The only accepted grammar is:

```text
00 ff 51 03 tt tt tt
```

The decoder does not scan, search for `ff 51`, discover Tempo structures,
parse sequence containers, parse the secondary `51 | MPQN` copy, infer
absolute time or delta semantics, accept nonzero position forms, reconstruct a
Tempo map, or participate in generic mixed performance-event walking.

# Caller contract

The caller supplies `InitialTempoBounds { event_range }`. The range must have
exactly seven bytes, start at the established zero initial-position byte, and
end immediately after the third MPQN byte. A future sequence-level parser is
responsible for locating and supplying this evidence-backed boundary.

# Proposed Rust representation

Reuse the existing `LocatedByte` and absolute `Range<usize>` conventions. The
smallest consistent API is conceptually:

```rust
use std::ops::Range;

pub struct InitialTempoBounds {
    pub event_range: Range<usize>,
}

pub struct InitialTempoEvent {
    pub event_range: Range<usize>,
    pub initial_position_byte: LocatedByte,
    pub ff_tag: LocatedByte,
    pub tempo_tag: LocatedByte,
    pub payload_length: LocatedByte,
    pub mpqn_byte_0: LocatedByte,
    pub mpqn_byte_1: LocatedByte,
    pub mpqn_byte_2: LocatedByte,
}

impl InitialTempoEvent {
    pub fn mpqn(&self) -> u32 {
        (u32::from(self.mpqn_byte_0.value) << 16)
            | (u32::from(self.mpqn_byte_1.value) << 8)
            | u32::from(self.mpqn_byte_2.value)
    }

    pub fn bpm(&self) -> Option<f64> {
        let mpqn = self.mpqn();
        (mpqn != 0).then(|| 60_000_000.0 / f64::from(mpqn))
    }
}

pub fn decode_bounded_initial_tempo(
    bytes: &[u8],
    bounds: InitialTempoBounds,
) -> Result<InitialTempoEvent, BoundedInitialTempoError>;
```

The seven `LocatedByte` fields preserve every structural and payload byte with
its absolute offset. `event_range` preserves the exact representation bound.
`mpqn()` and `bpm()` are derived conveniences; they do not replace raw bytes.
An additional borrowed seven-byte field would duplicate the complete set and
is not necessary because the range plus located fields preserve all bytes and
provenance.

Do not add sequence identity, MIDI track, absolute tick, event delta, a generic
Tempo-map state, or secondary-copy fields.

# Decode order

After bounds validation:

1. require exact range length seven;
2. read and preserve byte 0 as `initial_position_byte`; require `00`;
3. read and preserve byte 1 as `ff_tag`; require `ff`;
4. read and preserve byte 2 as `tempo_tag`; require `51`;
5. read and preserve byte 3 as `payload_length`; require `03`;
6. read and preserve bytes 4–6 individually as MPQN bytes;
7. derive the unsigned big-endian MPQN only on request; and
8. return only after the cursor equals `event_range.end` exactly.

There is no recovery path, alternate offset, tag search, or trailing-byte
acceptance.

# Position-field contract

`initial_position_byte` is deliberately conservative. Evidence establishes
only that it immediately precedes `ff 51`, equals zero in every known primary
example, and accompanies an initial Tempo at sequence start. The decoder must
require `00`. A nonzero byte is `UnsupportedInitialPositionByte`, not a value
to decode as an absolute tick, delta, or VLQ.

# MPQN contract

The three located payload bytes are authoritative. Derive:

```text
mpqn = (byte0 << 16) | (byte1 << 8) | byte2
```

No additional range validation is justified: every three-byte combination is
an unsigned 24-bit value. In particular, structurally preserve MPQN zero.
If `bpm()` exists, it must return `None` for zero and otherwise
`Some(60_000_000.0 / mpqn)`, avoiding division by zero and infinity.

# Exact-bound behavior

Validate bounds before indexing. Deterministic behavior is:

- empty range: `InvalidBounds`;
- reversed range: `InvalidBounds`;
- end beyond input: `InvalidBounds`;
- valid in-input range shorter than seven: `WrongRepresentationLength`;
- valid in-input range longer than seven: `WrongRepresentationLength`;
- nonzero byte 0: `UnsupportedInitialPositionByte`;
- wrong byte 1: `WrongFfTag`;
- wrong byte 2: `WrongTempoTag`;
- wrong byte 3: `WrongPayloadLength`;
- missing any MPQN byte in a short bound: rejected by
  `WrongRepresentationLength` before field access.

This ordering makes a six-byte representation, any partial payload, and an
eight-byte oversized representation deterministic. It does not silently
ignore bytes outside the exact grammar.

# Proposed error model

Illustrative variants are:

```text
InvalidBounds { start, end, size }
WrongRepresentationLength { start, end, observed, expected }
UnsupportedInitialPositionByte { offset, observed, expected }
WrongFfTag { offset, observed, expected }
WrongTempoTag { offset, observed, expected }
WrongPayloadLength { offset, observed, expected }
```

All errors identify the exact source offset or range. Length-first validation
means separate unreachable `TruncatedMpqnByteN` variants would add no
determinism; future tests should describe six-byte and partial forms as
truncated payload cases while asserting `WrongRepresentationLength`.

# Authentic future regression fixtures

Fixtures use fixed offsets and never scan:

- Experiment 007 natural Bells:
  `0x0000ebd8..0x0000ebdf`, `00 ff 51 03 09 10 8b`, MPQN 594,059;
- Experiment 002 controlled 120 BPM:
  `0x0002f7dc..0x0002f7e3`, `00 ff 51 03 07 a1 20`, MPQN 500,000;
- Experiment 004 controlled 130 BPM:
  `0x0002f7dc..0x0002f7e3`, `00 ff 51 03 07 0a e2`, MPQN 461,538.

Each test should assert the event range, every located byte value/offset, the
derived MPQN, and BPM behavior. Authentic fixture access follows the existing
repository policy for external research artifacts.

# Synthetic future tests

Positive cases:

- exact valid seven-byte form;
- low MPQN, for example `00 00 01` = 1;
- high MPQN `ff ff ff` = 16,777,215;
- MPQN zero preserved with `bpm() == None`;
- representative BPM calculation, including 500,000 MPQN = 120 BPM.

Negative cases:

- empty, reversed, and beyond-input bounds;
- six-byte truncated representation;
- every shorter partial-payload bound;
- eight-byte oversized representation;
- nonzero first byte;
- wrong `ff` byte;
- wrong `51` byte;
- wrong `03` length byte.

Tests must invoke exact ranges directly, assert deterministic variants and
offsets, and never locate fixtures by scanning.

# Sequence-level architecture

Tempo is represented in a sequence-level Meter/Tempo structural area. It is
not part of the known Note chains, Controller records, Channel Pressure runs,
Pitch Bend runs, or generic mixed performance-event stream. The bounded
decoder should remain independent of those modules. A future sequence parser
may invoke it only after establishing the exact containing range.

# Secondary-copy exclusion

Natural `51 09 10 8b` at `0x0000ec15..0x0000ec19` and controlled value bytes
at `0x0002f81a..0x0002f81d` correlate with Tempo. Their broader containing
structure is unresolved. This design neither parses nor validates them and
does not assume they must be reconstructed for MIDI recovery.

# Implementation gate

- A. stored Tempo value understood: **YES**.
- B. initial primary representation boundary understood: **YES**.
- C. initial position semantics fully understood: **NO**; bounded zero/start
  support is **PARTIAL**.
- D. bounded initial-Tempo decoder implementation-ready: **YES**.
- E. general Tempo-map parser implementation-ready: **NO**.

# Experiment decision

No controlled experiment is needed to implement this bounded initial form.
Do not broaden an implementation to answer general Tempo-map questions.

# Single recommended next step

Implement `decode_bounded_initial_tempo` exactly as bounded here, with the
three fixed authentic fixtures and focused synthetic positive/negative cases.
Meter is suitable for a separate next read-only correlation task; because all
known natural values are 4/4, that task may conclude that one controlled Meter
change is required before a Meter decoder can be designed.
