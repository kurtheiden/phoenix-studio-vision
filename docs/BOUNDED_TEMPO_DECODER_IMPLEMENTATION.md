# Objective and scope

Implement the exact caller-bounded Studio Vision initial Tempo representation
defined by `BOUNDED_TEMPO_DECODER_DESIGN.md`: `00 ff 51 03 tt tt tt`.

The implementation decodes one supplied seven-byte half-open range. It does
not discover Tempo, parse a Tempo map, accept nonzero position forms, parse the
secondary copy, integrate with performance-event walking, emit MIDI, or expose
CLI behavior.

# Public API

The `tempo` module exports `InitialTempoBounds`, `InitialTempoEvent`,
`BoundedInitialTempoError`, and `decode_bounded_initial_tempo`.
`InitialTempoEvent` returns the exact range and seven `LocatedByte` fields:
`initial_position_byte`, `ff_tag`, `tempo_tag`, `payload_length`, and three
MPQN bytes. Every byte retains its value and absolute input offset. No generic
meta-event or Tempo-map abstraction was introduced.

# Exact-bound behavior

The decoder validates `start < end <= bytes.len()` before indexing, then
requires length exactly seven. Empty, reversed, and beyond-input ranges return
`InvalidBounds`. Every in-input range shorter or longer than seven returns
`WrongRepresentationLength`; no trailing bytes are ignored.

# Structural validation

After length validation, bytes 0–3 must be `00 ff 51 03` in order. Bytes 4–6
are preserved as the MPQN payload. Errors report the relevant range or
absolute offset and observed/expected values. There is no fallback parse.

# Initial position handling

The first field is `initial_position_byte` and must equal zero. A nonzero value
returns `UnsupportedInitialPositionByte`; it is never decoded as a VLQ, delta,
or absolute position. Position semantics remain **PARTIAL**.

# MPQN handling

`mpqn()` derives `(byte0 << 16) | (byte1 << 8) | byte2` as `u32` without
replacing the raw bytes. Tests cover zero, one, 500,000, 461,538, 594,059, and
the maximum unsigned 24-bit value 16,777,215. No semantic MPQN range is imposed.

# BPM convenience

`bpm()` returns `Some(60_000_000.0 / mpqn)` for nonzero MPQN and `None` for
zero. Tests cover exact 120 BPM and the non-integral natural and controlled
values. MPQN remains authoritative and BPM is not rounded to a nominal UI BPM.

# Authentic fixtures

The integration suite uses fixed paths and ranges without scanning:

- Experiment 007 `0xebd8..0xebdf`: `00 ff 51 03 09 10 8b`, MPQN 594,059;
- Experiment 002 `0x2f7dc..0x2f7e3`: `00 ff 51 03 07 a1 20`, MPQN 500,000;
- Experiment 004 `0x2f7dc..0x2f7e3`: `00 ff 51 03 07 0a e2`, MPQN 461,538.

Each asserts exact source bytes, event range, seven field values, seven
absolute offsets, MPQN, BPM behavior, and exact consumption.

# Synthetic positive coverage

Synthetic tests cover exact decoding, MPQN zero, one, maximum 24-bit MPQN,
exact 120 BPM, and a nonzero slice start proving absolute offsets.

# Malformed-input coverage

Tests cover empty, reversed, beyond-input, undersized/partial-payload,
oversized, nonzero initial-position, wrong `ff`, wrong `51`, and wrong `03`.
Length validation precedes indexing, so truncated payloads deterministically
return `WrongRepresentationLength`.

# No-scanning behavior

A synthetic input has a wrong supplied seven-byte range and a valid Tempo form
immediately afterward. The decoder returns `WrongFfTag` for the supplied range
and does not recover to the later form.

# Secondary-copy exclusion

No API parses natural `0xec15..0xec19` or controlled
`0x2f81a..0x2f81d`. Those bytes lack a complete containing boundary and remain
unsupported.

# Sequence-level architecture

The module is exported by the library but is not connected to discovery, the
mixed performance-event walker, CLI, MIDI export, or Tempo-map construction. A
future sequence-level parser must provide the exact known bound.

# Deliberately unsupported behavior

Unsupported behavior includes scanning, discovery, nonzero or variable-width
positions, mid-sequence Tempo changes, multiple-event maps, secondary-copy
decoding, sequence/MIDI-track identity, absolute ticks, deltas, generalized
meta events, MIDI emission, and automatic playback reconstruction.

# Remaining unknowns

General position meaning, nonzero forms, mid-sequence placement, complete
Tempo-map framing/discovery, secondary-copy ownership, multiple Tempo events,
and broader project/version variants remain unknown. A general Tempo-map
parser is not implementation-ready.

# Single recommended next step

Perform the planned read-only Meter structural correlation. Do not create a
Meter experiment unless that correlation shows one non-4/4 edit is necessary.
