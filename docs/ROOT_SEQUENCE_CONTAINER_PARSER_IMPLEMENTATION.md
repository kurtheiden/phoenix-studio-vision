# Implemented scope

Phoenix now implements the approved read-only root-record and sequence-
container parser for the explicit 208-byte-preamble/166-byte-descriptor form.
It performs generic root framing separately from semantic classification and
does not scan or parse mixed performance events.

# Public API

`src/sequence_container.rs` exposes:

- `frame_record_at` for one exact-cursor generic record;
- `parse_root_record_stream` for lossless root framing from offset 8 to EOF;
- `parse_project_166` for strict semantic classification under
  `SequenceContainerProfile::Descriptor166`;
- raw/provenance representations for root headers, generic records,
  descriptors, names, sequences, track pairs, and associations;
- deterministic root and semantic error types.

The module is exported through `src/lib.rs`. No CLI behavior changed.

# Supported semantic profile

Only the established 208-byte preamble and 166-byte descriptor profile is
implemented. Production code contains no authentic sequence count, record
count, name, or offset. Root word 3 is not interpreted as a descriptor width.

# Root-header handling

At least eight bytes are required. Range `0..8` and all raw bytes are borrowed
with provenance. Four big-endian `u16` values are exposed only as ordinal raw
word conveniences, each retaining its own bytes and range. No observed value
is required or assigned semantics.

# Generic framing

`FramedRecord` preserves the record range, located type byte, four located
length bytes, derived unsigned big-endian length, and borrowed payload. Header
availability, conversion, addition, and payload bounds are checked before
slicing. Zero-length and unknown-type records are valid.

# Top-level walk

`parse_root_record_stream` begins exactly at offset 8, advances only by each
declared record end, and requires exact EOF. It retains every generic record.
It never searches for another plausible header after failure.

# Sequence validation

`parse_project_166` treats each top-level type `0x01` as a candidate and
requires the 208-byte preamble, descriptor count of at least two, checked
166-byte descriptor ranges, derived Pascal name inside the following
type-`0x07` record, optional `0x09` preludes, required
`0x02/0x29/0x02/0x29` Meter/Tempo order, primary payload capacity, complete
track `0x02/0x29` pairs, and type-`0x00` terminal. Failure is reported at that
candidate and is not downgraded to opaque data.

Descriptor and name text is never used for recognition. Non-UTF-8 names remain
valid raw bytes; an optional UTF-8 view is only a convenience.

# Sequence I mismatch behavior

Equal descriptor/pair counts produce explicit ordinal bindings. Unequal counts
produce `TrackAssociations::Unresolved` with both counts while retaining every
descriptor and pair. Authentic Sequence I therefore parses with 11 track
descriptors and 10 pairs; no blank descriptor is discarded and no inactive
identity is inferred.

# Older 120-byte behavior

The older `samples/newest STUFF` file succeeds under generic root framing: 495
records consume exactly to EOF. `parse_project_166` rejects its first
type-`0x01` candidate under the selected profile. There is no 120-byte fallback
and no profile selection from root-header values.

# Meter and Tempo bound supply

Validated sequences expose exact Meter range `primary payload +14..+22` and
Tempo range `primary payload +14..+21`. These are inputs for the existing
bounded decoders. This module does not duplicate their tag/value decoding or
parse secondary copies.

# Track containing bounds

Every track pair preserves exact primary/secondary record and payload ranges.
The primary exposes `candidate_event_start = payload start + 14` and a clearly
named containing range through payload end. No exact inner performance-event
end is claimed.

# Error behavior

Errors cover truncated roots and record headers, record-length overflow,
payloads beyond input, incomplete top-level consumption, malformed sequence
candidates, descriptor count/range arithmetic, invalid names, missing/wrong
required records, short primary payloads, malformed track pairs, and malformed
terminals. Errors retain relevant offsets, ranges, observed values, and roles.

# Provenance behavior

Root bytes/words, record headers/payloads, preambles, descriptors/labels,
sequence names, Meter/Tempo ranges, and track records all retain absolute
half-open source provenance. Borrowed raw bytes remain authoritative beside
derived integers and conveniences.

# Authentic fixtures

The Experiment 007 integration fixture verifies 527 generic records, exact EOF,
18 sequences, the first sequence boundary, final zero-length type-`0x05`
record, Bells Meter/Tempo ranges, Bells Track 9/14 primary containers, Ode to
Clarke ranges, and Sequence I mismatch preservation.

The older authentic sample verifies 495 generic records to exact EOF plus
deterministic rejection by the 166-byte semantic parser.

# Synthetic tests

Focused tests cover arbitrary root bytes, raw words, unknown/zero-length
records, exact EOF, every truncated root/header width, payload overrun,
overflow arithmetic, malformed trailing/candidate structures, descriptor and
name failures, required ordering, primary capacity, track pairs, terminals,
equal/mismatched associations, arbitrary/blank labels, non-UTF-8 names, bound
derivation, and containing-range semantics.

# No-scanning guarantee

A synthetic regression places a malformed type-`0x01` candidate before a
later valid-looking sequence. The semantic parser returns the first candidate's
exact error and never recovers to the later bytes.

# Explicit exclusions

The implementation excludes the older 120-byte semantic form, automatic
profile selection, active-track inference, exact inner event ends, mixed-event
parsing, Controller discovery, Pressure/Bend run discovery, Patch/Note
transitions, Meter/Tempo maps, MIDI export, and CLI integration.

# Remaining unknowns

Root-word and record-type meanings, the older sequence profile, Sequence I's
inactive identity, descriptor fields, legacy text encoding, track event tails,
family-run boundaries, and mixed-event transitions remain unresolved.

# Single recommended next step

Perform a read-only correlation of exact track-local event termination and
family-transition structure inside the now automatically located track-primary
containers. Do not expand the container parser itself into a mixed-event
walker.
