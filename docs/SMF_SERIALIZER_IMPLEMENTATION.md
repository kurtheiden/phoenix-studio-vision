# Objective

Implement Phase A of `MIDI_EXPORT_DESIGN.md`: small, pure, deterministic SMF
Format 1 primitives and synthetic tests. The implementation accepts only
MIDI-domain values and returns bytes or typed errors.

# Scope

The existing `smf` module now contains two deliberately separate facilities:
its earlier opaque track-chunk comparison API and the new serializer API. The
serializer supports a fixed six-byte Format 1 header, PPQN division, `MTrk`
chunks, explicit-status channel messages, the four required meta events,
absolute-tick scheduling, and automatic End of Track.

It imports no Studio Vision parser module, reads no project bytes, infers no
channel, performs no I/O, and contains no authentic project manifest data.

# Public API

Validated inputs:

- `MidiChannel::new(1..=16)`;
- `MidiDataByte::new(0..=127)`;
- `TimeSignature::new(...)` with nonzero numerator;
- `ChannelMessage` for the supported channel voice messages;
- `ScheduledEvent { absolute_tick, stable_ordinal, message }`.

Serialization primitives:

- `encode_midi_vlq`;
- `serialize_channel_message`;
- `serialize_track_name`, `serialize_set_tempo`,
  `serialize_time_signature`, and `serialize_end_of_track`;
- `serialize_musical_track`;
- `serialize_conductor_track`;
- `serialize_format1`.

`SerializedTrack` has private storage and can only be constructed by the
module, while `as_bytes` permits inspection. This prevents callers from
constructing a track without the enforced terminal event.

# MIDI channel convention

The public convention is human-facing channels 1 through 16. The zero-based
status nibble remains private and is derived only during status serialization.
There is no default channel and values 0 or 17 are typed errors.

# Supported channel messages

Every event writes an explicit status byte; running status is never emitted:

- `8n` Note Off with release velocity;
- `9n` Note On with attack velocity;
- `Bn` Control Change;
- `Cn` Program Change;
- `Dn` Channel Pressure;
- `En` Pitch Bend in LSB/MSB order.

Every key, velocity, controller, value, program, pressure, LSB, and MSB is a
validated seven-bit `MidiDataByte`. High bits are never masked.

# Supported meta events

- Track Name: `ff 03 <VLQ length> <bytes>`;
- Set Tempo: `ff 51 03 tt tt tt`, nonzero 24-bit MPQN;
- Time Signature: `ff 58 04 nn dd cc bb`, nonzero seven-bit numerator and
  seven-bit remaining fields;
- End of Track: `ff 2f 00`.

The conductor helper emits name, tempo, meter, and EOT at tick zero in exactly
that order. It derives none of those values from Studio Vision structures.

# VLQ limits

SMF delta and meta-length VLQs accept exactly `0..=0x0fff_ffff`. Encoding is
minimal and uses at most four bytes. Larger timing deltas and payload lengths
return `MidiVlqOverflow`.

# Deterministic ordering

Musical events are sorted by the explicit key:

```text
(absolute_tick, policy_priority, stable_ordinal)
```

Policy priority is implemented by an explicit match, not enum discriminants:
CC0, CC32, Program Change, other CC, Pitch Bend, Channel Pressure, Note Off,
then Note On. Equal-priority events use the caller-supplied stable ordinal.
Absolute ticks are converted to checked deltas after sorting; simultaneous
events receive zero delta.

# EOT invariant

`serialize_musical_track` accepts channel messages only. It appends exactly one
EOT at the latest scheduled tick with delta zero. Empty tracks end at tick zero.
The conductor helper also appends exactly one final EOT. Because EOT is not a
scheduled-message variant and `SerializedTrack` fields are private, callers
cannot omit it, duplicate it, or put channel messages after it.

# Error behavior

`SmfSerializeError` covers invalid channel/data/PPQN, invalid Tempo and Meter
numerator, MIDI VLQ overflow, zero or overflowing track count, track payload
length overflow, complete SMF length overflow, and an internal nonmonotonic
ordering invariant. Errors retain offending values or counts. No filesystem
or `std::io::Error` is present.

PPQN accepts positive values with the high bit clear. This rejects zero and
SMPTE-form divisions. Only Format 1 is emitted, and at least one track is
required.

# Synthetic test coverage

`tests/smf_serializer.rs` covers:

- exact Format 1 header, two-track count, PPQN 480, and invalid divisions;
- all required VLQ boundaries and overflow;
- channel 1/16 statuses and invalid channel 0/17;
- every channel message byte form and invalid MIDI data;
- Track Name including a two-byte length VLQ, Tempo limits, Time Signature,
  and EOT;
- an exact tiny `MTrk` chunk and checked payload length;
- absolute-to-delta conversion, simultaneous events, maximum legal delta, and
  overflowing delta;
- deliberately scrambled same-tick priorities and two stable ordinals within
  the ordinary-CC priority;
- exact conductor-track bytes;
- exactly one final EOT.

# Independent validation

A test-only parser independently walks one complete two-track synthetic SMF.
It reads fixed header fields and chunk lengths directly, independently decodes
event VLQs, checks explicit legal status/data widths, recognizes only the
supported meta events, requires exactly one final EOT per track, and requires
exact complete-file consumption. It does not call serializer decoding helpers
or add an external MIDI dependency.

# Explicit exclusions

Phase A excludes Format 0/2, SMPTE division, running status, Note scheduling,
Studio Vision adaptation, channel manifests/inference, arbitrary Tempo/Meter
maps, other meta events, SysEx, file writing, CLI behavior, authentic MIDI
generation, and DAW validation.

# Deviations from MIDI_EXPORT_DESIGN.md

None. The PPQN primitive is more general than the first 480-PPQN profile: it
accepts any validated positive metrical `u16` division with its high bit clear.
This is the designed serializer boundary and does not add timing conversion.

# Single recommended next step

Implement Phase B only: a pure MIDI-domain adapter/scheduler with Note Off
generation and conversion/report errors, tested entirely with synthetic
decoded values before any authentic project integration.
