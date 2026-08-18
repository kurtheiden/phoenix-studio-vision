# Objective and scope

Implement the exact caller-bounded Studio Vision initial Meter representation
defined by `BOUNDED_METER_DECODER_DESIGN.md`:

```text
00 ff 58 04 nn dd xx yy
```

The implementation decodes one supplied eight-byte half-open range. It does
not discover Meter, parse a Meter map or secondary copy, interpret nonzero
positions, integrate with performance events, or implement MIDI export policy.

# Public API

The `meter` module exports `InitialMeterBounds`, `InitialMeterEvent`,
`BoundedInitialMeterError`, and `decode_bounded_initial_meter`.
`InitialMeterEvent` returns the exact event range plus eight `LocatedByte`
fields: `initial_position_byte`, `ff_tag`, `meter_tag`, `payload_length`,
`numerator`, `denominator_exponent`, `third_payload`, and `fourth_payload`.

# Exact grammar and bounds

The decoder accepts only exact `00 ff 58 04 nn dd xx yy`. It validates
`start < end <= bytes.len()` before indexing and then requires exactly eight
bytes. Empty, reversed, or beyond-input ranges return `InvalidBounds`.
In-input shorter or longer ranges return `WrongRepresentationLength`; trailing
bytes are never ignored.

# Validation order and errors

After bounds and length validation, bytes 0–3 are checked in order:

1. byte 0 must be `00`, otherwise `UnsupportedInitialPositionByte`;
2. byte 1 must be `ff`, otherwise `WrongFfTag`;
3. byte 2 must be `58`, otherwise `WrongMeterTag`;
4. byte 3 must be `04`, otherwise `WrongPayloadLength`.

Errors preserve the exact range or absolute offset plus observed and expected
values. Length-first validation makes truncated payload failures deterministic
without separate unreachable truncation variants.

# Provenance behavior

All eight source bytes are returned individually as `LocatedByte` values with
absolute input offsets. `event_range` is the exact consumed caller bound. Raw
payload bytes remain authoritative; no derived value replaces them.

# Initial position handling

The first field is conservatively named `initial_position_byte` and must equal
zero. A nonzero byte is unsupported rather than decoded as an absolute tick,
delta, VLQ, or general Meter-map position. General position semantics remain
unresolved.

# Denominator derivation

`denominator()` derives `2^dd` through
`1_u64.checked_shl(u32::from(denominator_exponent.value))`. It returns `None`
for an exponent too large for `u64` without panicking or rejecting the
otherwise valid record. The exponent byte is always preserved.

# Payload policy

`numerator` and `denominator_exponent` are direct located bytes.
`third_payload` and `fourth_payload` are also preserved without additional
validation. The implementation assigns no historical SMF `cc` meaning to the
third byte and does not require the fourth byte to equal `08`.

# Authentic fixtures

Fixed-offset integration tests read external artifacts without scanning:

- Experiment 007 Bells 4/4, `0xeb80..0xeb88`;
- Experiment 007 `Sequence K` 6/8, `0x258df..0x258e7`;
- Experiment 030 Bells 7/8, `0xeb80..0xeb88`; and
- Experiment 007 `mission impossibl` 10/8, `0x1c864..0x1c86c`.

Every fixture asserts the exact source bytes and event range, all eight values
and absolute offsets, numerator, exponent, derived denominator, payload
preservation, and exact consumption.

# Synthetic coverage

Unit tests cover direct numerator/exponent preservation, denominator
derivation, arbitrary `third_payload = a5`, arbitrary
`fourth_payload = 37`, a nonzero event start with absolute provenance, and a
high exponent returning `None` safely.

Malformed tests cover empty, reversed, beyond-input, shortened, seven-byte
truncated, nine-byte oversized, nonzero-position, wrong `ff`, wrong `58`, and
wrong `04` cases with deterministic errors.

# No-scanning guarantee

A regression input contains a wrong caller-supplied eight-byte range followed
by a valid Meter representation. Decoding rejects the supplied range at its
wrong `ff` byte and does not discover or recover to the later form.

# Explicit exclusions

The implementation adds no Meter discovery, Meter-map parser, secondary-copy
parser, sequence/container discovery, mixed-event integration, MIDI writer,
historical `xx -> cc` conversion, sequence identity, track identity, absolute
tick, event delta, or nonzero position support.

# Remaining unknowns

Unknowns remain nonzero/mid-sequence position encoding, general Meter-map
framing and walking, complete secondary-copy ownership, the universal semantic
meaning of the third payload, universal historical SMF `cc` conversion, and
behavior of unobserved Meter structures.

# Relationship to future sequence/container parsing

The bounded decoder does not locate records. A future sequence/container
parser must establish and supply the exact eight-byte primary range. Meter
remains in the sequence-level Meter/Tempo area and outside performance-event
walking.

# Relationship to future SMF export policy

Binary recovery exposes `nn`, `dd`, `xx`, and `yy`; it does not generate
`nn dd cc bb`. Historical mappings `08 -> 18` and `06 -> 0c` remain evidence
for a future exporter, separate from decoding. That layer may use known
mappings and a documented standards-valid fallback without changing this API.

# Single recommended next step

Establish sequence/container discovery and integration capable of supplying
exact bounds and ownership to the implemented bounded family decoders.
