# Objective and scope

Phoenix now implements one provenance-preserving decoder for an exact
caller-bounded Pitch Bend run using the observed `Bells for her` Track 14
contract. It does not discover runs, parse mixed streams, or generalize the
observed state into universal running status.

# Public API

`pitch_bend::decode_bounded_pitch_bend_run` accepts project bytes and
`PitchBendRunBounds { run_range }`. It returns the exact run range, located
`e0` entry tag, and ordered entries containing exact ranges, located timing
deltas, located LSBs, and located MSBs.

# Exact-bound behavior

The caller's half-open range is authoritative. Empty, reversed, and
out-of-input ranges fail. Parsing starts exactly at the supplied start and
continues through complete timing/LSB/MSB continuations to the exact end. The
decoder never scans, retries at another offset, or infers the run end.

# Entry state

The first entry must be `timing VLQ | e0 | LSB | MSB`. Timing uses the shared
one-to-four-byte bounded VLQ decoder. The entry tag and both data bytes retain
absolute offsets. A complete entry ending at the bound is a valid one-entry
run.

# Continuation state

Only after `e0` succeeds does the decoder accept repeated `timing VLQ | LSB |
MSB` entries. Continuation errors report the zero-based entry index and failing
cursor or field offset. No public stateless continuation decoder exists.

# Timing behavior

Every entry returns the encoded event-start delta, raw timing bytes, and exact
range. Absolute musical position is not accumulated; that remains future
caller/container state.

# Stored LSB/MSB handling

Both direct project bytes are preserved independently as `LocatedByte` values.
The binary layer imposes no observed-center limit and structurally preserves
full-byte values rather than using data values to infer boundaries.

# Derived raw-value handling

`PitchBendEntry::raw_value()` computes `LSB + (MSB << 7)` as `u16`. It is
derived from the two authoritative located bytes and does not replace them or
create signed/centered primary storage.

# Authentic nine-run fixtures

Fixed-offset tests decode all nine Track 14 ranges with expected counts 8, 6,
34, 10, 5, 6, 12, 6, and 15. They validate all 102 event-start deltas and raw
values, direct LSB/MSB bytes, every `e0` entry, contiguous ordered entry ranges,
and exact final consumption. Detailed checks cover run 1's delta 1,361/value
8,127 entry and zero ending, run 3's one-/two-byte timing including delta 1,586
and center 8,192, and run 9's zero-to-center population.

# Neighbor-boundary validation

Separate provenance tests confirm each of the first eight run ends is the
start of its established following Note with adjacent `90` after timing. Run 9
ends at `0x158d8`, the start of an independently decoded CC1=7 record. The
Pitch Bend decoder parses neither neighbor.

# Malformed-input coverage

Tests cover empty, reversed, and out-of-input bounds; truncated and overlong
entry timing; missing or wrong `e0`; missing entry LSB/MSB; truncated and
overlong continuation timing; missing continuation LSB/MSB; mid-entry bounds;
and standalone continuation bytes. Valid one- and multi-entry runs are positive
fixtures.

Run 1 extended through following Note bytes and run 9 extended into following
Controller bytes fail through missing continuation fields. The decoder does
not recognize `90` or `ff 41` as terminators.

# Why continuations are not independently decodable

`timing VLQ | LSB | MSB` has no independent family discriminator. It is
accepted only after the required `e0` entry establishes Pitch Bend state and
only inside the exact caller-supplied run range. A standalone continuation
fixture fails the entry-tag requirement.

# Comparison with Channel Pressure implementation

Both implementations preserve an explicit family-entry tag and decode compact
continuations within an exact bound, but remain separate modules with
family-specific data width and errors. No shared generic running-status
abstraction was introduced.

# Deliberately unsupported behavior

Unsupported behavior includes run discovery, isolated Pitch Bend decoding,
automatic end detection, scanning/resynchronization, channel inference,
absolute-time accumulation, generic mixed walking, project/track discovery,
CLI output, and MIDI export.

# Remaining unknowns

Unknowns include isolated representation, whether `e0` has the same meaning
elsewhere, re-entry behavior, bends above center in natural projects,
autonomous run-bound discovery, and broader project/version/device/channel
generality.

# Single recommended next step

Make Tempo the next evidence target: correlate the provenance-controlled
`Bells for her` Tempo event with the existing bounded controlled tempo-save
evidence before designing a decoder.
