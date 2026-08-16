# Objective

Record the provenance-controlled natural evidence for the observed Studio
Vision Channel Pressure run while separating byte observations, supported
interpretation, and unknowns.

# Provenance

The source is the untouched Experiment 007 `newest STUFF baseline` project
(SHA-256 `e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`)
and the full multitrack export of its active `Bells for her` sequence
(SHA-256 `ffbdbb6be208a2d607c9b0c55a12b72226a18d43b9494c2b46b058d4568fc2c3`).
Track 9 is independently identified by Patch, Note, Controller, and timing
correlations. The export contains exactly the 184 Studio Vision List events
accounted for in the bounded Track 9 event region.

# Evidence population

Track 9 contains 32 exported Channel Pressure events, all on SMF channel 12.
They are List-event ordinals 106–137 in the event-start projection that excludes
MIDI Note endings and Patch-derived CC0. All 32 correspond to one contiguous
project run.

# Run boundaries

The exact project run is `0x1478c..0x147ce` (66 bytes). Its start is fixed by
the end of the independently decoded preceding CC1 record. Its end is fixed by
the independently aligned following Note. The bounds were derived from event
order, not by searching for pressure values.

# Run-entry representation

The first event is:

```text
timing VLQ | d0 | pressure_value
82 20      | d0 | 01
delta 288       | value 1
```

`d0` is an observed entry discriminator with no assigned MIDI-channel
semantics. The exported event uses MIDI channel 12 and SMF status `db`, so the
project byte is not a literal export status/channel encoding.

# Continuation representation

The next 31 events omit `d0` and use:

```text
timing VLQ | pressure_value
```

This form is interpretable only after the explicit run entry has established
Channel Pressure continuation state and only inside the exact caller-known run
bound. It is not independently classifiable in arbitrary project bytes.

# Timing semantics

All 32 leading 7-bit big-endian VLQs equal the exported delta from the previous
Studio Vision event start: 32/32 exact. The first delta is 288 from preceding
CC1 at tick 131,236 to pressure at tick 131,524. Continuation deltas are from
the preceding pressure start. The following Note delta is 470 from the final
pressure at tick 131,912 to the Note at tick 132,382. Channel Pressure therefore
participates in the same ordered event-start timing stream as Controllers and
Notes. A bounded run does not independently provide absolute position.

# Direct pressure-value field

The byte after each entry's timing field (and after `d0` for the first entry)
matches the exported pressure value directly: 32/32 exact, spanning the
observed values 0–79. No payload-length byte or opaque context field is present
in this run.

# Stateful family behavior

Observation establishes one explicit family entry followed by 31 compact
same-family continuations. This supports explicit active-family state for this
run. It does not establish a universal Studio Vision running-status mechanism,
isolated pressure encoding, re-entry behavior, or transitions involving every
family.

# Preceding Controller boundary

The exact preceding record is `0x14782..0x1478c`:

```text
11 ff 41 05 00 1f 00 01 00
```

It matches exported CC1=0 and ends exactly where `82 20 d0 01` begins.

# Following Note boundary

The run ends exactly before:

```text
0x147ce: 83 56 90 6c 33 34 8b 31 ...
```

`83 56` is the expected delta 470; `90` is present at this specific
Pressure-to-Note transition; and the following properties match pitch 108,
attack 51, release 52, and duration 1,457. This proves this transition, not that
every Note transition has `90` or that consecutive Notes repeat it.

# Current-cursor discrimination

**PARTIAL.** At the run start, decoding the timing VLQ exposes adjacent `d0`,
which identifies the observed run entry and gives a derivable one-byte value.
Continuations have no local discriminator. They are safe only under already
active Channel Pressure state within the exact run bound.

# Evidence supported

- one exact 32-event natural Channel Pressure run;
- 32/32 direct values and 32/32 event-start deltas;
- explicit `d0` entry followed by compact timing/value continuations;
- exact preceding Controller and following Note boundaries;
- no observed payload length, context field, or literal MIDI channel;
- a specific explicit `90` transition from the pressure run to a Note.

# Unknowns

Unknowns include isolated pressure representation, whether `d0` always denotes
this family, whether it may reappear within a run, all other transition forms,
global active-family rules, broader projects/versions/devices, and values
outside the natural population.

# Decoder implications

Decode one exact caller-bounded run, require `d0` only on entry, and treat all
later timing/value pairs as continuations because run state is already active.
Return raw timing/value provenance and the entry tag. Do not scan for the end,
infer MIDI channel, or accumulate absolute time. This must be a run decoder,
not a stateless decoder for arbitrary continuation records.

# Experiment decision

No controlled experiment is needed for the observed run contract. Thirty-two
natural events establish bounds, entry form, continuations, direct value, and
timing. Additional natural or controlled cases would test generality, not the
bounded contract documented here.

# Single recommended next step

Implement the exact-bounded, state-aware Channel Pressure run decoder with the
authentic 32-event run and malformed bounded fixtures.
