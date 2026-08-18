# Objective

Design, without implementing, a provenance-preserving decoder for one exact
caller-bounded Studio Vision initial Meter representation.

# Scope and non-goals

Proposed name: `decode_bounded_initial_meter`. The caller supplies the complete
input and one exact eight-byte half-open range. The only accepted grammar is:

```text
00 ff 58 04 nn dd xx yy
```

The decoder does not scan, discover Meter structures, parse a Meter map, parse
the secondary copy, interpret nonzero position forms, generate SMF `cc`, or
participate in mixed performance-event walking.

# Caller contract

The caller supplies `InitialMeterBounds { event_range }`. The range begins at
the established zero initial-position byte and ends immediately after the
fourth payload byte. It must contain exactly eight bytes. A future
sequence/container parser is responsible for locating and supplying this
evidence-backed boundary.

# Proposed Rust representation

Reuse `LocatedByte` and absolute `Range<usize>` conventions:

```rust
use std::ops::Range;

pub struct InitialMeterBounds {
    pub event_range: Range<usize>,
}

pub struct InitialMeterEvent {
    pub event_range: Range<usize>,
    pub initial_position_byte: LocatedByte,
    pub ff_tag: LocatedByte,
    pub meter_tag: LocatedByte,
    pub payload_length: LocatedByte,
    pub numerator: LocatedByte,
    pub denominator_exponent: LocatedByte,
    pub third_payload: LocatedByte,
    pub fourth_payload: LocatedByte,
}

impl InitialMeterEvent {
    pub fn denominator(&self) -> Option<u64> {
        1_u64.checked_shl(u32::from(self.denominator_exponent.value))
    }
}

pub fn decode_bounded_initial_meter(
    bytes: &[u8],
    bounds: InitialMeterBounds,
) -> Result<InitialMeterEvent, BoundedInitialMeterError>;
```

The range and eight located fields preserve the complete representation and
absolute provenance. `denominator()` is a derived convenience. It returns
`None` when the exponent cannot fit a `u64` shift; the raw byte remains
available and the record remains structurally valid.

Do not add sequence identity, absolute tick, event delta, MIDI track, a generic
Meter map, secondary-copy fields, semantic names for `xx`, or automatic SMF
export policy.

# Decode order

After validating bounds:

1. require exact range length eight;
2. preserve byte 0 as `initial_position_byte` and require `00`;
3. preserve byte 1 as `ff_tag` and require `ff`;
4. preserve byte 2 as `meter_tag` and require `58`;
5. preserve byte 3 as `payload_length` and require `04`;
6. preserve bytes 4–7 as numerator, denominator exponent, third payload, and
   fourth payload;
7. return only after consuming `event_range.end` exactly.

There is no alternate offset, search, recovery, fallback parser, or acceptance
of trailing bytes.

# Position-field contract

`initial_position_byte` is deliberately conservative. Evidence establishes
only that it equals zero in every authenticated primary and accompanies an
initial Meter at sequence start. Require `00`. A nonzero value is
`UnsupportedInitialPositionByte`, not a VLQ, delta, or absolute position.

# Numerator contract

Preserve `numerator` directly with its absolute offset. Authenticated examples
include `04`, `06`, `07`, and `0a`. Do not reject zero or another byte value on
musical-validity grounds: the binary grammar establishes a byte field, not a
universal semantic validity policy.

# Denominator-exponent contract

Preserve `denominator_exponent` directly. Derive the musical denominator as
`2^dd` only through an overflow-safe convenience such as
`1_u64.checked_shl(u32::from(dd))`. A high exponent returning `None` does not
invalidate otherwise correct framing and must not discard source provenance.

# Third- and fourth-payload contracts

Preserve `third_payload` (`xx`) and `fourth_payload` (`yy`) as uninterpreted
located bytes. Do not label `xx` as clocks per metronome or impose a universal
conversion. Do not require `yy == 08`; every observed value is `08`, but the
framing evidence does not make other values structurally invalid.

# Exact-bound behavior

Validate bounds before indexing. Deterministic behavior is:

- empty range: `InvalidBounds`;
- reversed range: `InvalidBounds`;
- end beyond input: `InvalidBounds`;
- valid in-input range shorter than eight: `WrongRepresentationLength`;
- valid in-input range longer than eight: `WrongRepresentationLength`;
- nonzero byte 0: `UnsupportedInitialPositionByte`;
- wrong byte 1: `WrongFfTag`;
- wrong byte 2: `WrongMeterTag`;
- wrong byte 3: `WrongPayloadLength`.

Length validation precedes field access, so a seven-byte truncated payload or
any shorter form deterministically returns `WrongRepresentationLength`.

# Proposed error model

Repository-consistent variants are:

```text
InvalidBounds { start, end, size }
WrongRepresentationLength { start, end, observed, expected }
UnsupportedInitialPositionByte { offset, observed, expected }
WrongFfTag { offset, observed, expected }
WrongMeterTag { offset, observed, expected }
WrongPayloadLength { offset, observed, expected }
```

Errors identify the exact range or absolute source offset. Separate truncated
payload variants are unnecessary because length-first validation makes them
unreachable.

# Authentic future regression fixtures

Use fixed paths and ranges without scanning:

- Experiment 007 natural Bells 4/4:
  `0x0000eb80..0x0000eb88`, `00 ff 58 04 04 02 08 08`;
- Experiment 007 natural `Sequence K` 6/8:
  `0x000258df..0x000258e7`, `00 ff 58 04 06 03 06 08`;
- Experiment 030 controlled Bells 7/8:
  `0x0000eb80..0x0000eb88`, `00 ff 58 04 07 03 06 08`;
- Experiment 007 natural `mission impossibl` 10/8 project-only form:
  `0x0001c864..0x0001c86c`, `00 ff 58 04 0a 03 06 08`.

Each authentic test should assert the exact event range, all eight byte values,
all eight absolute offsets, numerator, denominator exponent, derived
denominator, and direct preservation of `xx` and `yy`. Fixture identity and
access should follow existing external-artifact policy.

# Synthetic future tests

Positive cases:

- exact valid eight-byte form;
- direct numerator and denominator-exponent preservation;
- denominator derivation for `dd = 02` and `dd = 03`;
- arbitrary `xx` and arbitrary `yy` preserved without rejection;
- nonzero slice start proving absolute offsets;
- high denominator exponent preserved with `denominator() == None`.

Negative cases:

- empty, reversed, and beyond-input bounds;
- seven-byte truncated and shorter representations;
- nine-byte and longer oversized representations;
- nonzero first byte;
- wrong `ff`, wrong `58`, and wrong `04`.

Tests should assert deterministic error variants and exact offsets.

# No-scanning contract

A future test must provide a structurally wrong exact eight-byte range while a
valid `00 ff 58 04 ...` representation exists elsewhere in the same input. The
decoder must reject the supplied range and must not find or recover to the
other representation.

# Sequence-level architecture

Initial Meter belongs to the same sequence-level Meter/Tempo structural area
as initial Tempo. It is not a Note, Controller, Channel Pressure, or Pitch Bend
performance record and must not enter the mixed-event walker. A future
sequence/container parser supplies the exact bound to this decoder.

# Secondary-copy exclusion

Correlated local forms `58 nn dd xx yy` repeat primary payloads, including
natural Bells `0x0000ebbd..0x0000ebc2` and natural `Sequence K`
`0x0002591c..0x00025921`. Their containing-record boundary and role remain
unresolved. This design does not parse, validate, or reconstruct them.

# SMF export-policy separation

The decoder recovers `00 ff 58 04 nn dd xx yy`. An SMF writer emits
`ff 58 04 nn dd cc bb`. These are separate layers.

Firmly correlated historical mappings are `xx 08 -> cc 18` and
`xx 06 -> cc 0c`; observed `yy` exports directly as `bb`. Keep the lookup out
of core binary decoding. A future exporter may use evidence-backed mappings
where known and a documented standards-valid fallback otherwise. This design
does not define that full export policy.

# Implementation gate

- A. exact primary boundary understood: **YES**.
- B. framing understood: **YES**.
- C. numerator understood: **YES**.
- D. denominator exponent understood: **YES**.
- E. third payload preservation understood: **YES**.
- F. third payload general semantics understood: **PARTIAL**.
- G. fourth payload preservation understood: **YES**.
- H. bounded initial Meter decoder implementation-ready: **YES**.
- I. general Meter-map parser implementation-ready: **NO**.
- J. standards-valid SMF Meter export possible: **YES**.

# Experiment decision

**NO FURTHER METER EXPERIMENT NEEDED.** A universal historical semantic label
for `xx` is not required for bounded binary recovery or standards-valid Meter
export.

# Single recommended next step

This design is implemented as documented in
`BOUNDED_METER_DECODER_IMPLEMENTATION.md`. The next structural target is
sequence/container discovery and integration capable of supplying exact bounds
to the bounded decoders.
