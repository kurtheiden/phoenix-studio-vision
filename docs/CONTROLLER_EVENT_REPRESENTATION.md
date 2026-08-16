# Objective

Record the provenance-controlled evidence for Studio Vision ordinary Controller
events while separating observations, supported interpretation, and unknowns.

# Provenance

The source is the untouched Experiment 007 `newest STUFF baseline` project
(SHA-256 `e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`)
and the full multitrack export of its active `Bells for her` sequence
(SHA-256 `ffbdbb6be208a2d607c9b0c55a12b72226a18d43b9494c2b46b058d4568fc2c3`).
The provenance is reinforced by sequence, track, Patch, Note, and timing
correlations.

# Evidence population

The SMF contains 405 Control Changes. Ten CC0/CC32 messages are generated from
Patch bank state. Excluding those leaves 395 ordinary Controllers: one CC7 on
each of Tracks 3, 4, and 6; 32 CC7 and 88 CC1 on Track 9; and 272 CC1 on Track
14. The project contains 395 corresponding identified `ff 41` records.

# Shared record representation

```text
timing VLQ | ff 41 | 05 | context[3] | controller_number | controller_value
```

The timing field is an observed one-to-four-byte 7-bit big-endian VLQ. The
fixed `05` counts the three context bytes, number, and value. A record is
therefore `timing_width + 8` bytes.

# Timing semantics

On Track 9, all 120 encoded timing values equal the exported delta from the
previous Studio Vision event start in the ordered track stream. This is an
event delta, not an absolute musical position. Absolute position requires
prior caller/container state and cannot be reconstructed from one record.

# Controller number and value fields

The fourth and fifth payload bytes directly match controller number and value.
Track 9 matches 120/120 ordered pairs across CC7 and CC1, and independent
tracks match their exports. These are structural `u8` fields; MIDI-aware
callers may separately validate the nominal 0–127 range.

# Opaque context bytes

The first three payload bytes are preserved losslessly and remain opaque.
Observed associations are:

- Track 3: `00 23 00`
- Track 4: `00 05 00`
- Track 6: `00 02 00`
- Track 9: `00 1f 00`
- Track 14: `00 01 00`

They do not establish channel, track, instrument, device, or reference
semantics. A decoder must not assign those meanings.

# Interleaving with Note events

Controllers and Notes share the ordered event-start timing stream. Track 9
Notes interrupt Controller runs, and the following Controller delta is measured
from the intervening Note start rather than the preceding Controller.

# Independent track validation

Track 3 has CC7=127 at tick 960 with record start `0x10a6d`, timing `83 60` =
480 after its Patch, and context `00 23 00`. Track 4 has CC7=127 at tick 208,
record start `0x1123a`, timing `1c` = 28 after its Patch, and context `00 05
00`. Track 6 has CC7=127 at tick 130, record start `0x11eac`, timing `81 02` =
130, and context `00 02 00`. Track 14 contributes 272 ordered CC1 records with
context `00 01 00`.

# Patch-derived bank messages are distinct

Ordinary Controllers use the `ff 41 05` record. Patch bank state is stored
inside the Patch representation and may produce CC0/CC32 during SMF export.
An exported CC0 or CC32 is not automatically an ordinary `ff 41` Controller.
Future code must preserve this architectural distinction.

# Evidence supported

Evidence strongly supports the event boundary, timing mechanics and delta
semantics, tag, payload length, opaque context width, direct number/value bytes,
and Note interleaving. It supports one shared ordinary Controller grammar for
natural CC1 and CC7 across the identified `Bells for her` tracks.

# Unknowns

Context semantics, surrounding container ownership, discovery, and generality
across every CC number, project/version, and device configuration remain
unknown. Malformed out-of-MIDI-range stored values have not been observed.

# Decoder implications

Phoenix should implement one generic number/value Controller record, not CC1-
and CC7-specific decoders: both numbers occupy the same field in identical
framing. The decoder must be caller-bounded, exact-consuming,
provenance-preserving, and perform no discovery or resynchronization.

# Experiment 030 decision

No Experiment 030 is needed. The 395 natural records distinguish number and
value, cover multiple contexts and timing widths, and demonstrate Note
interleaving. A value edit would not resolve the remaining context/container
unknowns.

# Single recommended next step

Implement the bounded ordinary Controller decoder and authentic regression
fixtures described in `BOUNDED_CONTROLLER_DECODER_DESIGN.md`.
