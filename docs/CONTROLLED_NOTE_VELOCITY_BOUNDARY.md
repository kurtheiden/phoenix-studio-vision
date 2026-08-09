# Objective

This report analyzes Experiment 013, a verified-working velocity-only edit of
the established C1 drum note from velocity 127 to exactly 126. Its primary test
is the boundary behavior of the velocity-correlated byte immediately following
the isolated pitch anchor. The work is read-only evidence gathering. It does
not modify an artifact, implement parser logic, or claim a complete event
record.

# Experimental provenance

Experiment 007 is the untouched verified-working baseline at C1, velocity 127.
Experiments 011 and 012 retained C1 and changed the same note to velocities 124
and 121. Experiment 013 was created from a fresh native Finder duplicate of
the same baseline; the user changed only velocity from 127 to 126, leaving
pitch, timing, duration, and note count unchanged.

Studio Vision saved Experiment 013 when the user quit and selected Save. The
project reopened and functioned normally, was quit without saving, and was
Finder-copied through SheepShaver's `Unix` shared volume into the research
folder. All four supplied directories were inspected before selecting their
uncompressed project files. Experiment 007's StuffIt archive was excluded, and
the reported Experiment 013 basename was confirmed.

# Artifact inventory

| Artifact | Exact path | Basename | Data-fork size | Data-fork SHA-256 |
|---|---|---|---:|---|
| Experiment 007 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline` | `newest STUFF baseline` | 211,468 (`0x33a0c`) | `e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132` |
| Experiment 011 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 011 - Single MIDI Note Velocity Change/newest STUFF baseline EXP11` | `newest STUFF baseline EXP11` | 211,468 (`0x33a0c`) | `8f9a18f629a58a4eede289b181fae6ba1b61ef2361d28a2eb17cc0248747ccd6` |
| Experiment 012 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 012 - Second Single MIDI Note Velocity Change/newest STUFF baseline EXP12` | `newest STUFF baseline EXP12` | 211,468 (`0x33a0c`) | `d20da4c64bf13e6c044e9d36493da90a5c6c256671373010a289f320162b3fec` |
| Experiment 013 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 013 - Velocity 126 Boundary Test/newest STUFF baseline EXP13` | `newest STUFF baseline EXP13` | 211,468 (`0x33a0c`) | `9f6b977f5c49fe4f64422a6031b6829fd167b06accd9e6629ed69e74c00fb03a` |

All four have Finder Type `MID2`, Finder Creator `MIDA`, a 32-byte
`com.apple.FinderInfo`, a 16-byte `org.BasiliskII.FinderInfo`, and a 16-byte
`org.BasiliskII.ExtendedFinderInfo`. No other extended attributes or resource
forks were observed. Their `com.apple.FinderInfo` and zero-filled extended
FinderInfo values are identical. Experiment 007 has BasiliskII FinderInfo
`4d4944324d4944410100007b01080000`; Experiments 011, 012, and 013 have
`4d4944324d4944410100ffffffff0000`.

All four data forks have equal sizes. `Track 7` remains at `0x0002f6ca` and
`Ode to Clarke` at `0x0002f753`; the complete previously reported label
sequence and 166-byte cadence remain aligned. Broad structure is aligned, so
same-position comparisons are used without inferring insertions or deletions.

# Pitch-anchor verification

The 81 pitch-bearing positions were reconstructed from the bidirectional
conditions: Experiment 010 `0x23`, Experiment 007 `0x24`, Experiment 009
`0x25`, and Experiments 005, 006, and 008 `0x24`. Experiment 013 retains
`0x24` at all 81 positions.

| Experiment 013 value | Count |
|---|---:|
| `0x24` | 81 |
| `0x23` | 0 |
| `0x25` | 0 |
| any other value | 0 |

The preregistered pitch prediction succeeds without exception. The isolated
pitch byte at `0x0002f76f` and all 80 dense-region pitch bytes remain
independently stable during the velocity-126 edit.

# Velocity 126 boundary result

The pair at `0x0002f76f–0x0002f770` was inspected before the broad comparison.
Experiment 013 contains exactly `24 0f`.

| Experiment | Velocity | Pair |
|---|---:|---|
| 007 | 127 | `24 0f` |
| 013 | 126 | `24 0f` |
| 011 | 124 | `24 2d` |
| 012 | 121 | `24 2d` |

Velocity 126 therefore retains the baseline state at `0x0002f770`. The
observed transition from `0f` to `2d` lies below tested velocity 126; among the
tested points, the differing states are bounded by velocity 126 (`0f`) and
velocity 124 (`2d`). This is consistent with a threshold or quantized state
between those tested values, but does not establish monotonic behavior at
untested velocity 125 or elsewhere.

The byte is reproducibly velocity-correlated because both lower tested
velocity files have `2d`, while baseline, both pitch controls, the rename
control, and Experiment 013 have `0f`. It is not a direct scalar velocity byte
and its encoding remains unknown.

# Velocity series

| Experiment | Pitch | Velocity | `0x0002f76f` | `0x0002f770` |
|---|---|---:|---:|---:|
| 007 | C1 | 127 | `24` | `0f` |
| 013 | C1 | 126 | `24` | `0f` |
| 011 | C1 | 124 | `24` | `2d` |
| 012 | C1 | 121 | `24` | `2d` |
| 009 | C#1 | unchanged | `25` | `0f` |
| 010 | B0 | unchanged | `23` | `0f` |

The first byte continues to respond only to pitch in these controls. The
second byte shows two tested states rather than an ordered magnitude: `0f` at
127 and 126, and `2d` at 124 and 121. The data strengthen a two-state
interpretation over the rejected affine interpretation, but one untested
intervening value and only two observations per state do not establish a
threshold rule.

The broad search also found a direct-value candidate beside the first dense
pitch anchor. At `0x00031c1f–0x00031c20`, the observed pairs are:

| Experiment | Intentional state | Pair |
|---|---|---|
| 007 | C1, velocity 127 | `24 7f` |
| 009 | C#1, velocity unchanged | `25 7f` |
| 010 | B0, velocity unchanged | `23 7f` |
| 011 | C1, velocity 124 | `24 7f` |
| 012 | C1, velocity 121 | `24 7f` |
| 013 | C1, velocity 126 | `24 7e` |

Experiment 013 supplies an exact direct `7f` to `7e` response one byte after a
confirmed pitch-bearing byte, with surrounding local bytes unchanged. This is
strong Experiment-013-specific direct velocity-value evidence. It is not a
coherent series-wide direct encoding because Experiments 011 and 012 retain
`7f` despite their lower velocities. The result supports a second positional
candidate but not a universal scalar field at that offset.

# Whole-file comparison

Experiment 007 and 013 have equal 211,468-byte data forks. They differ at 1,766
same-position bytes in 632 disjoint runs. The first difference is
`0x0000001e`, the last is `0x00033a06`, the maximum unequal run is 36 bytes,
the common prefix is 30 bytes, and the common suffix is five bytes. Relevant
labels and broad structure remain aligned.

There is exactly one direct `0x7f` to `0x7e` transition, at `0x00031c20`.
There are two whole-file single-byte decreases by one: that direct candidate
and `ff` to `fe` at `0x0002f6f2`. The latter is part of a compact new
Experiment-013-only change spanning `0x0002f6f2–0x0002f6fd`; nine positions
differ within that 12-byte span. Its baseline bytes
`ff ff 80 00 00 14 00 c8 00 c8 00 00` become
`fe ff 00 97 00 74 02 8b 03 0b 00 23`. The compact change has no established
numeric relationship to velocity 126 and is classified as unresolved.

All 80 dense pitch bytes remain `0x24`, but the immediately following direct
candidate at `0x00031c20` changes to `0x7e`. No other byte in
`0x00031c1f–0x00031f98` differs from baseline.

Experiment 013 versus 011 differs at 753 bytes in 426 runs; first difference
`0x0000002c`, last `0x00033809`, maximum run 13 bytes, common prefix 44 bytes,
and common suffix 514 bytes. Experiment 013 versus 012 differs at 735 bytes in
422 runs with the same first/last offsets, maximum run, prefix, and suffix.

# Control filtering

Using Experiments 005, 006, 008, 009, 010, 011, and 012 as the prior-control
union, the 1,766 baseline/Experiment-013 differences classify as:

| Class | Count | Observation |
|---|---:|---|
| exactly reproduced Experiment 008 value | 145 | recurring save output |
| other previously variable locations | 1,611 | offset varied in another prior artifact |
| established pitch anchors | 0 changed; 81 stable | all remain `24` |
| direct velocity-value candidate | 1 | `0x00031c20`, baseline `7f`, Experiment 013 `7e` |
| new compact unresolved positions | 9 | within `0x0002f6f2–0x0002f6fd` |

The categories sum to all 1,766 unequal positions. The isolated
`0x0002f770` state does not appear in the baseline/013 diff because Experiment
013 retains baseline `0f`; its importance comes from the velocity series.

At positions stable in pitch-only and rename controls, the velocity experiments
identify 11 relevant positions: the nine compact Experiment-013 changes,
`0x0002f770`, and `0x00031c20`. The two established candidates behave
differently:

- `0x0002f770` is `0f` at velocities 127 and 126, and `2d` at 124 and 121;
- `0x00031c20` is `7e` only at velocity 126 and `7f` in all other inspected
  artifacts.

The exact numeric match at `0x00031c20` is a strong direct-value candidate,
while its lack of response at velocities 124 and 121 argues against treating
that fixed offset as a complete ordered velocity series. The nine compact
positions are new and unresolved. All remaining differences fall in the
previously variable save-output population.

Experiment 014 subsequently tested velocity 125. It retained `0f` at
`0x0002f770`, bracketing the tested state transition between 125 and 124, and
independently produced the preregistered direct value `7d` at `0x00031c20`.
The complete comparison is in `CONTROLLED_NOTE_VELOCITY_125.md`.

Experiment 015 then independently repeated velocity 124. It contained `0f` at
`0x0002f770` and direct value `7c` at `0x00031c20`; neither Experiment 011
candidate reproduced. This weakens the provisional state-boundary
interpretation and extends the direct sequence. See
`CONTROLLED_NOTE_VELOCITY_124_REPLICATION.md`.

Experiment 016 independently repeated velocity 121 and likewise contained
`0f` at `0x0002f770`, while `0x00031c20` matched direct value `79`. Experiment
012's `2d`/`7f` pair did not replicate. See
`CONTROLLED_NOTE_VELOCITY_121_REPLICATION.md`.

# Evidence supported

- Experiment 013 is a verified-working, same-size artifact with aligned broad
  structure and unchanged relevant labels.
- All 81 confirmed pitch-bearing bytes remain `0x24` when pitch remains C1.
- The central boundary pair is `24 0f`; velocity 126 retains the baseline state
  at `0x0002f770`.
- The observed `0f`/`2d` state change lies between tested velocities 126 and
  124. This is consistent with, but does not prove, a threshold or quantized
  representation.
- `0x00031c20`, immediately after a confirmed dense pitch byte, changes exactly
  `7f` to `7e` for velocity 127 to 126 while local surrounding bytes remain
  stable.
- The original Experiments 011 and 012 retain `7f`, while their independent
  replications produce direct values `7c` and `79`; applicability to the
  anomalous artifacts remains unresolved.
- The isolated pair and dense pair provide two distinct pitch/velocity
  positional relationships, neither yet constituting a complete event record.
- Nine compact Experiment-013-only differences remain unresolved; all other
  differences occur at previously variable positions.

# Unknowns

- Whether `0x0002f770` represents a threshold, quantized category, edit state,
  or another property remains unknown.
- Why the original Experiments 011 and 012 do not contain the direct values
  reproduced by Experiments 015 and 016 is unknown.
- The meaning of the compact `0x0002f6f2–0x0002f6fd` change is unknown.
- It is unknown whether the two pitch/velocity pairs are event fields,
  mappings, cached state, or other representations.
- No timing, duration, channel, track ownership, note-on/note-off framing, or
  complete event boundary has been established.
- No valid Standard MIDI File has yet been recovered.

# Single recommended next step

Experiment 014 completed the velocity-125 test: `0x0002f770` remains `0f`, and
`0x00031c20` independently matches direct value `7d`. See
`CONTROLLED_NOTE_VELOCITY_125.md`. Experiment 015's velocity-124 replication
is reported in `CONTROLLED_NOTE_VELOCITY_124_REPLICATION.md`; the velocity-121
replication is reported in `CONTROLLED_NOTE_VELOCITY_121_REPLICATION.md`.
