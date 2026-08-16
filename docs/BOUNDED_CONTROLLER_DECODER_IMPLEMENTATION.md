# Objective and scope

Phoenix now implements the designed decoder for one exact caller-bounded
ordinary Studio Vision Controller record. It adds representation decoding only:
no discovery, scanning, whole-track parsing, timeline accumulation, CLI output,
MIDI export, or artifact mutation.

# Public API

`src/controller.rs` exposes:

- `ControllerRecordBounds` with one exact half-open `record_range`;
- `BoundedControllerRecord`;
- `BoundedControllerError`;
- `decode_bounded_controller_record(bytes, bounds)`.

The result reuses the bounded Patch decoder's `LocatedVlq`, `LocatedBytes`, and
`LocatedByte` provenance types.

# Exact-bound behavior

The decoder validates `timing VLQ | ff 41 | 05 | context[3] | number | value`
strictly inside the supplied range. Bounds must be nonempty and inside the
input. Parsing starts exactly at `record_range.start`, and the value must end
exactly at `record_range.end`. The decoder never searches, retries, skips, or
resynchronizes. Both truncation and trailing bytes fail deterministically.

# Fields returned

The result contains the complete record range, decoded timing delta with raw
bytes/range, tag range, located payload length, payload range, borrowed opaque
context with range, and located controller number and value. Number and value
remain structural `u8` values without a decoder-level 0–127 restriction.

# Timing behavior

The returned `timing_delta` is only the encoded delta from the previous Studio
Vision event start. Its byte width and source range are preserved. The decoder
does not return or manufacture an absolute tick or musical position; a future
container caller must accumulate event state.

# Context preservation

All three context bytes are borrowed and returned byte-for-byte with an
absolute range. Authentic fixtures cover `00 23 00`, `00 05 00`, `00 02 00`,
`00 1f 00`, and `00 01 00`. No channel, track, instrument, device, identifier,
or other semantics are inferred.

# Authentic fixtures

Fixed-offset tests read the untouched Experiment 007 baseline without scanning:

- Track 3: `0x10a6d..0x10a77`, CC7=127, delta 480;
- Track 4: `0x1123a..0x11243`, CC7=127, delta 28;
- Track 6: `0x11eac..0x11eb6`, CC7=127, delta 130;
- Track 9: fixed CC7 and CC1 records covering delta zero, one-byte nonzero,
  and two-byte timing;
- Track 14: `0x15213..0x1521d`, CC1=1, delta 2,030.

Tests assert values, raw timing width, and exact ranges for every returned
field. Track 9 and Track 14 demonstrate that the same decoder handles CC1 and
CC7; there are no parameter-specific parsing paths.

# Malformed-input coverage

Focused tests cover invalid, reversed, out-of-input, and empty bounds;
unterminated and over-four-byte timing; missing/truncated/wrong tag bytes;
missing/wrong payload length; context truncation at every partial width;
missing number; missing value; and a valid record followed by a trailing byte
inside the supplied bound. Errors retain useful offsets and observed bytes.

# Patch-bank separation

An authentic bounded Patch bank tail is explicitly rejected because it is not
an `ff 41 05` record. Patch-derived CC0/CC32 SMF messages remain Patch state;
MIDI controller number never selects this decoder.

# Deliberately unsupported behavior

The implementation does not locate records, infer bounds, parse containers or
tracks, accumulate time, interpret context, emit MIDI, integrate with the CLI,
or claim support for every CC number, project, Studio Vision version, or device
configuration.

# Remaining unknowns

Context semantics, surrounding container ownership, discovery, and grammar
generality beyond the 395 natural `Bells for her` CC1/CC7 records remain
unknown. Out-of-MIDI-range stored bytes have no established semantic meaning,
although the binary decoder preserves them structurally.

# Single recommended next step

Design the caller/container integration that supplies proven Controller bounds
and accumulated event-start state, without adding heuristic scanning.
