# Objective

Record the byte-exact evidence for one Studio Vision initial Tempo
representation and separate that bounded result from unresolved general Tempo
map structure.

# Provenance

The authentic source is the untouched Experiment 007 project
`newest STUFF baseline`, size 211,468 bytes, SHA-256
`e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`.
The natural event belongs to its `Bells for her` sequence.

The provenance-controlled full multitrack export is `newest STUFF - Bells for
her - provenance multitrack`, size 28,120 bytes, SHA-256
`ffbdbb6be208a2d607c9b0c55a12b72226a18d43b9494c2b46b058d4568fc2c3`.
It is a valid format-1 SMF with division 480 and eleven track chunks. Its
descriptive modern filename was applied after export and is not embedded
Studio Vision provenance.

Controlled evidence comes from the same-offset Experiment 002 no-edit save and
Experiment 004 single Tempo edit from 120 BPM to 130 BPM. Their corrected
primary and secondary offsets were independently re-read after the external
correlation report was corrected.

# Natural Bells for her evidence

The authentic project half-open range `0x0000ebd8..0x0000ebdf` is exactly:

```text
00 ff 51 03 09 10 8b
```

Observed bytes and offsets are:

| Range or offset | Bytes | Observation |
|---|---|---|
| `0x0000ebd8` | `00` | byte immediately before the Tempo tag |
| `0x0000ebd9..0x0000ebdb` | `ff 51` | two tag bytes |
| `0x0000ebdb` | `03` | payload-length byte |
| `0x0000ebdc..0x0000ebdf` | `09 10 8b` | three-byte payload |

The SMF contains exactly one Tempo meta-event, in track/chunk 0 at absolute
tick 0. Its event bytes at file offset `0x30` are also
`00 ff 51 03 09 10 8b`; its payload is therefore byte-equal to the project
payload. SMF organization is export evidence and is not assumed to reproduce
Studio Vision container organization.

# Controlled Tempo evidence

The corrected exact primary half-open range in both controlled files is
`0x0002f7dc..0x0002f7e3`:

```text
Experiment 002, 120 BPM: 00 ff 51 03 07 a1 20
Experiment 004, 130 BPM: 00 ff 51 03 07 0a e2
```

The complete primary MPQN payload is
`0x0002f7e0..0x0002f7e3`. The inclusive same-offset diff run is only
`0x0002f7e1–0x0002f7e2`, because the shared high payload byte remains `07`.
That historical diff range must not be mistaken for the complete payload.

Experiment 004 was not pre-registered with a binary prediction. Its early
comparison report established output-specific differences but did not assign
the Tempo bytes. Natural value equality, sequence-level structural placement,
and repetition of the complete primary and secondary forms now provide the
independent support for that assignment. Unrelated save differences retain no
Tempo interpretation.

# Primary representation

Natural and controlled evidence agree on this exact seven-byte initial form:

```text
initial_position_byte | ff | 51 | 03 | mpqn[3]
```

Only `initial_position_byte == 00` is established. `ff 51` is the Tempo tag,
`03` is the payload length, and the following three bytes are the stored MPQN
integer. The established primary boundary includes all seven bytes and excludes
the following structure.

# Stored MPQN value

The payload is an unsigned 24-bit big-endian microseconds-per-quarter-note
integer:

```text
mpqn = (byte0 << 16) | (byte1 << 8) | byte2
```

All available semantic states agree:

| State | Payload | MPQN |
|---|---|---:|
| controlled 120 BPM | `07 a1 20` | 500,000 |
| controlled 130 BPM | `07 0a e2` | 461,538 |
| natural nominal 101 BPM | `09 10 8b` | 594,059 |

The three raw payload bytes are authoritative and must retain their offsets.

# BPM conversion

BPM is derived rather than stored directly:

```text
bpm = 60,000,000 / mpqn
```

The observed UI-to-storage relationship is
`floor(60,000,000 / nominal_integer_bpm)` where the division is non-integral:
130 produces 461,538 and nominal 101 produces 594,059; 120 produces the exact
integer 500,000. A decoder may expose floating-point BPM as a convenience, but
MPQN remains the stored value. A zero MPQN has no finite BPM conversion; the
binary evidence does not justify rejecting it structurally.

# Initial position field

The byte immediately before `ff 51` is `00` in all established primary
examples, and all represent initial Tempo at sequence start. Evidence does not
distinguish absolute position zero, event delta zero, or a zero location in a
separate Tempo map. Durable code and documentation should therefore call it
`initial_position_byte`, require zero for this bounded form, and assign no
general nonzero semantics.

# Sequence-level structural context

The natural primary occurs after the literal `Bells for her` and in a local
area containing paired Meter and Tempo structures. This is sequence-level
structure, not an identified Note chain, Controller chain, Channel Pressure
run, Pitch Bend run, or generic mixed performance-event stream. A future
sequence parser must supply the exact seven-byte Tempo bound.

# Secondary value-bearing copy

The natural project has `51 09 10 8b` at
`0x0000ec15..0x0000ec19`; the value bytes occupy
`0x0000ec16..0x0000ec19`. The controlled value bytes occupy corrected range
`0x0002f81a..0x0002f81d` and change from `07 a1 20` to `07 0a e2`.

This secondary form is correlated, but its complete containing-record boundary
and purpose are unresolved. It is outside the bounded initial decoder. Current
evidence does not establish that Phoenix must parse, emit, or reconstruct it to
recover MIDI Tempo.

# SMF export relationship

Standard SMF Tempo uses `ff 51 03 tt tt tt`. The project primary contains the
same tag, length, and MPQN payload after its leading zero byte. Direct storage
is established by repeated project bytes, the controlled value change, and
natural export equality—not merely because the SMF contains `ff 51`.

# Evidence supported

- Stored Tempo value: **YES**, unsigned 24-bit big-endian MPQN.
- Initial primary representation boundary: **YES**, exact seven-byte form.
- Natural/controlled structure agreement: **YES**.
- Initial position semantics fully understood: **NO**; only zero/start is
  established, so operational support is **PARTIAL**.
- Bounded initial-Tempo decoder implementation readiness: **YES**.
- General Tempo-map parser implementation readiness: **NO**.

# Unknowns

Unknowns include Tempo-map discovery, mid-sequence Tempo positions, whether a
general position is absolute or delta, nonzero position widths or encodings,
multiple Tempo events, broader primary framing variants, secondary-copy
ownership/boundaries, and complete Tempo-map reconstruction.

# Decoder implications

Decode only an exact caller-supplied seven-byte half-open range matching
`00 ff 51 03 tt tt tt`. Preserve every byte and absolute offset, derive MPQN,
optionally derive finite BPM, and consume the bound exactly. Do not scan,
discover, parse the secondary copy, attach sequence identity, manufacture an
absolute tick, or enter the performance-event walker.

# Experiment decision

**NO CONTROLLED EXPERIMENT NEEDED** for the bounded initial form. A future
mid-sequence Tempo edit would answer a broader map-position question and is not
a prerequisite for this decoder.

# Single recommended next step

Implement the exact caller-bounded initial Tempo decoder from
`BOUNDED_TEMPO_DECODER_DESIGN.md` with fixed authentic and synthetic malformed
fixtures. Separately, Meter is suitable for the next read-only correlation
because its structure is adjacent; existing all-4/4 evidence is unlikely to
establish all value fields without one later controlled Meter change.
