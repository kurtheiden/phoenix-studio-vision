# Objective

Record the provenance-controlled natural evidence for Studio Vision Pitch
Bend runs while keeping observations, supported interpretation, and unknowns
separate.

# Provenance

The source is the untouched Experiment 007 `newest STUFF baseline` project
(SHA-256 `e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`)
and the full multitrack export of its active `Bells for her` sequence (SHA-256
`ffbdbb6be208a2d607c9b0c55a12b72226a18d43b9494c2b46b058d4568fc2c3`).
SMF chunk index 10 is independently mapped to Studio Vision Track 14 through
the already-established Controller population and Note properties, durations,
timing, and order; track-name text is not used as identity evidence.

# Evidence population

The export contains exactly 102 Pitch Bend events, all on exported MIDI
channel 15, in Studio Vision Track 14. Their project records form nine runs
containing 8, 6, 34, 10, 5, 6, 12, 6, and 15 events. All 102 stored values and
all 102 event-start timing deltas agree with the export projection.

# Run boundaries

The independently aligned half-open project ranges are:

| Run | Range | Events |
|---:|---|---:|
| 1 | `0x1541f..0x15439` | 8 |
| 2 | `0x15440..0x15453` | 6 |
| 3 | `0x154bb..0x15524` | 34 |
| 4 | `0x1552b..0x1554a` | 10 |
| 5 | `0x1555e..0x1556f` | 5 |
| 6 | `0x15576..0x15589` | 6 |
| 7 | `0x155b7..0x155dc` | 12 |
| 8 | `0x1588f..0x158a3` | 6 |
| 9 | `0x158aa..0x158d8` | 15 |

These bounds come from ordered traversal beginning at the proven Controller
anchor `0x15213`, with independently matching Controller and Note records
between runs. They were not found by searching for bend values.

# Run-entry representation

All nine run entries use:

```text
timing VLQ | e0 | pitch_lsb | pitch_msb
```

The first is `8a 51 e0 3f 3f` at `0x1541f`: delta 1,361 and raw value
8,127. `e0` is an observed Studio Vision family-entry discriminator. It is not
interpreted as a stored MIDI channel or literal MIDI status: the export uses
channel 15 and status `ee`.

# Continuation representation

The remaining 93 events omit `e0` and use:

```text
timing VLQ | pitch_lsb | pitch_msb
```

This form is meaningful only after the run entry has established Pitch Bend
state and only inside an exact caller-known Pitch Bend run bound. It is not
independently classifiable in arbitrary project bytes.

# Stored value representation

The project preserves the two exported seven-bit MIDI data bytes directly in
LSB-then-MSB order. For all 102 events:

```text
raw = pitch_lsb + (pitch_msb << 7)
```

Examples include `3f 3f` = 8,127, `3c 38` = 7,228, `00 00` = 0, and
`00 40` = 8,192. The observed raw range is 0 through 8,192. The report's
display conversion is `signed = raw - 8192`, yielding -8,192 through 0. That
conversion is derived interpretation, not a replacement for the stored bytes.
The absence of above-center values does not establish that they are unsupported.

# Timing semantics

Every leading 7-bit big-endian VLQ equals the delta from the previous Studio
Vision List-event start: 102/102. The event-start projection includes Notes,
Controllers, and Pitch Bends but does not treat exported Note Off messages as
separate List-event starts. A bounded run provides deltas only; absolute
musical position requires caller/container accumulation.

# Stateful family behavior

Observation establishes nine explicit `e0` entries and 93 compact same-family
continuations. A 1,586-tick gap inside run 3 remains a continuation because no
other List event intervenes. This establishes Pitch Bend family state for
these exact runs, not universal running status or global state rules.

# Neighboring event boundaries

Eight runs exit to independently matched Notes, with explicit `90` at each of
those specific transitions. Run 9 exits at `0x158d8` to an independently
decoded ordinary CC1=7 record beginning `83 5b ff 41 05 ...`. The continuation
grammar has no internal run-end marker; these boundaries remain caller
evidence.

# Current-cursor discrimination

At an observed run start, a bounded timing VLQ followed immediately by `e0`
identifies the entry without scanning: 9/9. Continuations have no local family
tag and require active Pitch Bend state plus the exact run bound. The run end
cannot be derived from continuation bytes alone.

# Comparison with Channel Pressure

Proven natural forms are:

```text
Channel Pressure entry: timing | d0 | value
Channel Pressure continuation: timing | value
Pitch Bend entry: timing | e0 | LSB | MSB
Pitch Bend continuation: timing | LSB | MSB
```

Both use event-start timing, an explicit observed family entry, compact
untagged continuations, and exact caller-known run ends. This makes a broader
stateful-family convention possible, but does not prove universal MIDI-like
running status, universal `d0`/`e0` semantics, or the behavior of other
families.

# Evidence supported

- 102 natural Pitch Bend events in one provenance-controlled track;
- nine exact run bounds and independently matched neighbors;
- 102/102 direct LSB/MSB values and 102/102 event-start deltas;
- nine `e0` entries and 93 untagged continuations;
- exact bounded, state-aware decoding is supportable for the observed form.

# Unknowns

Unknowns include isolated Pitch Bend representation, whether `e0` has the same
meaning elsewhere, re-entry behavior, autonomous run-bound discovery, bends
above center, and generality across projects, versions, devices, and channels.

# Decoder implications

Decode one exact caller-bounded run. Require `e0` after entry timing, preserve
both stored value bytes and their provenance, enter explicit continuation
state, parse timing plus two bytes to the exact bound, and return ordered
entries. Do not scan, discover the end, infer MIDI channel, or accumulate
absolute time.

# Experiment decision

No controlled experiment is needed for the observed bounded-run contract.
Natural evidence establishes timing, value mapping, entry discrimination,
state behavior, and exact boundaries. Further work would test generality.

# Single recommended next step

Implement the exact caller-bounded, state-aware Pitch Bend run decoder with
fixed authentic fixtures for all nine runs and focused malformed inputs.
