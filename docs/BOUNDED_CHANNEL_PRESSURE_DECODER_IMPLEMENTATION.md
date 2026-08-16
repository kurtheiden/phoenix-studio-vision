# Objective and scope

Phoenix now implements one provenance-preserving decoder for the exact
caller-bounded Channel Pressure run established in `Bells for her` Track 9. It
does not discover runs, walk mixed events, or generalize the observed stateful
form into universal running status.

# Public API

`channel_pressure::decode_bounded_channel_pressure_run` accepts project bytes
and `ChannelPressureRunBounds { run_range }`. It returns the exact run range,
the located `d0` entry tag, and ordered entries containing exact ranges, a
`LocatedVlq` timing delta, and a `LocatedByte` pressure value.

# Exact-bound behavior

The half-open caller range is authoritative. Empty, reversed, and
out-of-input ranges fail. Parsing begins exactly at the supplied start and
must consume exactly to its end. The decoder never scans for family markers or
tries alternate boundaries.

# Entry state

The first bounded entry must be `timing VLQ | d0 | pressure value`. The timing
VLQ is limited to one through four bytes by the shared bounded decoder. The
`d0` byte and its absolute offset are preserved. A complete first entry ending
at the supplied bound is a valid one-entry run.

# Continuation state

Only after the `d0` entry succeeds does the decoder repeatedly accept `timing
VLQ | pressure value` continuations. Continuation errors report their
zero-based entry index and cursor or missing-value offset. There is no public
stateless continuation decoder.

# Timing behavior

Every entry exposes the encoded event-start delta plus its raw bytes and exact
range. The decoder does not accumulate absolute musical position; that remains
future caller/container state.

# Pressure-value handling

Pressure values are direct preserved bytes with absolute offsets. The binary
decoder deliberately imposes no `0..127` semantic restriction and assigns no
MIDI-channel meaning to `d0`.

# Authentic Track 9 fixture

The fixed-offset regression fixture decodes `0x1478c..0x147ce` from the
untouched Experiment 007 baseline. It asserts all 32 established timing/value
pairs, including entry `82 20 d0 01` (delta 288, value 1) and final
continuation `0a 00` (delta 10, value 0), exact provenance, ordered contiguous
entry ranges, and exact total consumption.

# Boundary adjacency

The authentic test separately decodes the preceding CC1 record at
`0x14783..0x1478c`, checks that the run starts at its end, and confirms that
the known following Note bytes begin at `0x147ce`. These neighbor checks do not
make the Channel Pressure decoder parse either family. The earlier
`0x14782` preceding-record start was corrected: that byte belongs to the prior
Controller record.

# Malformed-input coverage

Tests cover empty, reversed, and out-of-input bounds; truncated and overlong
entry timing; missing or wrong `d0`; missing first value; truncated and
overlong continuation timing; missing continuation value; and bounds ending
mid-entry. An authentic oversized range containing the following Note fails
under continuation grammar rather than treating `90` as an end marker.

# Why continuations are not independently decodable

The bytes `timing VLQ | pressure value` have no independent family
discriminator. They are accepted only while state established by the required
`d0` entry is active and only inside the exact caller-supplied run range. A
test confirms continuation-form bytes alone fail the entry-tag requirement.

# Deliberately unsupported behavior

The implementation does not discover run starts or ends, scan or
resynchronize, decode isolated pressure events, infer MIDI channel, expose a
generic running-status abstraction, accumulate absolute time, walk mixed
events, integrate with project discovery, or produce CLI output.

# Remaining unknowns

Unknowns include isolated representation, whether `d0` has the same meaning
elsewhere, re-entry behavior, other transition forms, values outside the
natural population, and generality across projects, versions, and devices.

# Single recommended next step

Perform a read-only natural correlation of the provenance-controlled `Bells
for her` Pitch Bend population, prioritizing a bounded region with independently
aligned neighbors before considering further mixed-stream integration.
