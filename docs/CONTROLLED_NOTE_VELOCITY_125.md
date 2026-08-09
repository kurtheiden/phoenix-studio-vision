# Objective

This report analyzes Experiment 014, a controlled velocity-only edit of the
established C1 drum note from velocity 127 to exactly 125. It tests two
predictions preregistered after Experiment 013: the state at `0x0002f770` and
the exact direct-value prediction `0x7d` at `0x00031c20`. The analysis is
read-only and does not claim a complete MIDI event record.

# Experimental provenance

Experiment 014 was created from a fresh native Finder duplicate of the
known-good Experiment 007 baseline. The user changed only the same note's
velocity from 127 to 125, saved on Studio Vision quit, reopened and verified
the project as functional, quit without saving, and Finder-copied the file
through `Unix` into the research directory.

Directory inspection positively identified the uncompressed project as
`/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 014 - Velocity 125 Boundary Test/newest STUFF baseline EXP14`.
It is 211,468 bytes and has SHA-256
`137d404cdd09e434e2e41cafb2a386e7497b6eb82e7e1280a41e927d15649106`.
Finder Type is `MID2` and Creator is `MIDA`. Its extended attributes are
`com.apple.FinderInfo` (32 bytes), `org.BasiliskII.FinderInfo` (16 bytes), and
`org.BasiliskII.ExtendedFinderInfo` (16 bytes); no other attributes or resource
fork were observed. This matches the metadata shape of Experiments 007, 011,
012, and 013. All five project data forks are 211,468 bytes.

# Preregistered candidate results

The two locations were inspected before the broad comparison:

| Candidate | Experiment 014 bytes | Result |
|---|---|---|
| `0x0002f76f–0x0002f770` | `24 0f` | C1 pitch byte retained; state remains `0f` |
| `0x00031c1f–0x00031c20` | `24 7d` | C1 pitch byte retained; preregistered direct value succeeds exactly |

Velocity 125 therefore retains the upper tested state at the isolated
candidate, while the dense candidate equals velocity 125 decimal exactly.
These observations are kept separate: one is a fixed-position state and the
other is an exact numeric match.

# Pitch-anchor verification

The 81 pitch anchors were reconstructed from positions where Experiment 010 is
`0x23`, Experiment 007 is `0x24`, and Experiment 009 is `0x25`. Experiment 014
contains `0x24` at all 81.

| Experiment 014 value | Count |
|---|---:|
| `0x24` | 81 |
| `0x23` | 0 |
| `0x25` | 0 |
| other | 0 |

The pitch representation remains stable without exception during the
velocity-only edit.

# Isolated boundary series

| Velocity | Experiment | `0x0002f770` |
|---:|---|---|
| 127 | 007 | `0f` |
| 126 | 013 | `0f` |
| 125 | 014 | `0f` |
| 124 | 011 | `2d` |
| 121 | 012 | `2d` |

The tested state transition is now bracketed between velocity 125 (`0f`) and
124 (`2d`). This strengthens a fixed-position, velocity-correlated two-state
observation. It does not establish monotonic behavior outside the tested
values or explain whether the state is a threshold, quantization, lookup
result, edit state, or another correlated property.

# Direct-value candidate

| Velocity | Experiment | Pair at `0x00031c1f–0x00031c20` |
|---:|---|---|
| 127 | 007 | `24 7f` |
| 126 | 013 | `24 7e` |
| 125 | 014 | `24 7d` |
| 124 | 011 | `24 7f` |
| 121 | 012 | `24 7f` |

The preregistered prediction `24 7d` succeeds exactly. Together, velocities
127, 126, and 125 form the independently predicted direct sequence `7f`,
`7e`, `7d` at a byte immediately following a confirmed pitch anchor, with the
surrounding four bytes on either side stable in the inspected controls.

The fixed position does not form a complete velocity series: velocity 124 and
121 artifacts contain `7f`, not `7c` and `79`. The exact 125 result is strong
direct numeric evidence, but numeric identity alone does not prove event
ownership or show why the lower-velocity saves use the baseline value here.

Experiment 015 subsequently repeated velocity 124 independently and produced
`24 7c`, exactly extending the direct sequence, while the isolated pair was
`24 0f`. Experiment 011's `24 7f` and `24 2d` values therefore did not
replicate. See `CONTROLLED_NOTE_VELOCITY_124_REPLICATION.md`.

Experiment 016 subsequently repeated velocity 121 and produced `24 79`, while
the isolated pair was `24 0f`. Experiment 012's two candidate values also did
not replicate. See `CONTROLLED_NOTE_VELOCITY_121_REPLICATION.md`.

# Search for moving velocity representation

A whole-file control-filtered search looked for each edited direct value at a
position that differs from baseline while Experiments 008, 009, and 010 remain
baseline-valued. Experiment 013 has one such `0x7e` position and Experiment
014 has one such `0x7d` position: both are the fixed `0x00031c20` candidate.
Experiments 011 and 012 have no such introduced `0x7c` or `0x79` position
anywhere in their data forks.

Within the dense region, raw numeric matches occur at many background
positions, but they do not change consistently with the experiment and are not
treated as velocity representations. No evidence was found that the direct
value moves to a different pitch anchor in the velocity-124 or velocity-121
save.

Experiment 014 introduces one other `0x7d` relative to baseline at
`0x0002d5f6`. That location is part of a serialized `7b 00 7c 00 7d 00 7e 00
7f` sequence also present in Experiments 009–013; it is recurring save-output
structure, not an Experiment-014-specific velocity candidate.

# Dense pitch-anchor neighborhood analysis

Consistent three-byte windows on each side of all 80 dense anchors were
compared across Experiments 007, 013, 014, 011, and 012. Seventy-nine anchor
neighborhoods are byte-identical across the velocity series. Only the first
anchor neighborhood changes:

| Pitch anchor | Relative position | Values for velocities 127, 126, 125, 124, 121 |
|---|---:|---|
| `0x00031c1f` | `+1` (`0x00031c20`) | `7f, 7e, 7d, 7f, 7f` |

No anchor identity changes, reordering, activation pattern, or second changing
pitch-adjacent byte was observed in these bounded windows. There are multiple
incidental pitch/value-looking pairs in the region, but only the first
anchor's neighbor responds under control filtering.

# Whole-file comparison

Experiments 007 and 014 both contain 211,468 bytes. They differ at 1,766
same-position bytes in 635 disjoint runs. The first difference is
`0x0000001e`, the last is `0x00033a06`, the maximum unequal run is 36 bytes,
the common prefix is 30 bytes, and the common suffix is five bytes.

`Track 7` remains at `0x0002f6ca` and `Ode to Clarke` at `0x0002f753`; the
known label sequence and 166-byte cadence remain aligned. Same-position
comparison is therefore appropriate, without inferring insertions or
deletions.

There is exactly one direct `0x7f -> 0x7d` transition, at `0x00031c20`.
There are two single-byte decreases by two: that candidate and a previously
variable byte at `0x00001053`. Two baseline-differing `0x7d` bytes occur, but
the other, at `0x0002d5f6`, belongs to recurring serialization described
above.

# Velocity-series synthesis

No fixed absolute position follows the complete five-point direct series
`7f, 7e, 7d, 7c, 79`. The fixed dense candidate follows its first three
consecutive values exactly, including Experiment 014's independently predicted
value, then returns to `7f` in the two lower-velocity artifacts. The isolated
candidate follows the state series `0f, 0f, 0f, 2d, 2d`.

The two observations can coexist without being assigned semantics: one is an
exact direct-value response over three experiments, and one is a reproducible
state response with a tested boundary between 125 and 124. No supported
transformation explains all five direct values, and no moving direct value was
found for 124 or 121.

The compact Experiment-013 region at `0x0002f6f2–0x0002f6fd` forms a third
pattern in Experiment 014:

| Artifact | Twelve bytes |
|---|---|
| 007, 011, 012 | `ff ff 80 00 00 14 00 c8 00 c8 00 00` |
| 013 | `fe ff 00 97 00 74 02 8b 03 0b 00 23` |
| 014 | `fe ff 00 97 00 74 01 d5 01 dd 00 15` |

The Experiment 014 pattern partly matches Experiment 013 and partly differs.
The multi-byte changes do not supply a demonstrated ordered relationship to
velocity 125 and remain unresolved.

# Control filtering

Of the 1,766 baseline/Experiment-014 unequal positions, 1,763 had already
varied in at least one of Experiments 005, 006, 008–013. Three positions are
new relative to that full control set: `0x00031bfb`, `0x0003202a`, and
`0x0003202b`. They lack a demonstrated relationship to velocity and are
unresolved save-run differences.

The exact `0x7d` candidate at `0x00031c20` is not new relative to all controls
because Experiment 013 already established that the position varies. Its
ordered `7f -> 7e -> 7d` behavior and pitch-adjacent location distinguish it
from the recurring population. The isolated `0x0002f770` state is likewise
separated by its reproducible velocity-series behavior.

# Evidence supported

- Both preregistered pitch bytes remain `0x24`, and all 81 established pitch
  anchors remain stable.
- Velocity 125 produces `24 0f` at the isolated pair, bracketing the tested
  state transition between velocities 125 and 124.
- Velocity 125 produces exactly the preregistered `24 7d` dense pair.
- The fixed dense byte follows direct values `7f`, `7e`, `7d` for velocities
  127, 126, and 125 with an otherwise stable local neighborhood.
- The original Experiments 011 and 012 remain `7f` at the fixed candidate and
  contain no control-stable moving `7c` or `79`; later replications do produce
  the predicted direct values at the fixed candidate.
- Seventy-nine of 80 bounded dense anchor neighborhoods remain byte-identical;
  only the first anchor's following byte varies.
- The Experiment-013 compact region takes a third unresolved pattern in
  Experiment 014.
- The evidence supports pitch-bearing bytes, a fixed velocity-correlated
  state, and an exact direct-value candidate, but not a complete event record.

# Unknowns

- Why the original Experiments 011 and 012 do not contain the direct values
  reproduced by Experiments 015 and 016 is unknown.
- The meaning and relationship of the `0f`/`2d` state and direct-value byte are
  unknown.
- Whether either pitch-adjacent pair is an event field, mapping, cache, or
  another representation is unknown.
- The three newly variable positions and compact third pattern remain
  unresolved.
- Timing, duration, channel, track ownership, note-on/note-off semantics, and
  complete event framing have not been established.
- No valid Standard MIDI File has yet been recovered.

# Single recommended next step

Experiment 015 completed the velocity-124 replication and produced direct
`0x7c` rather than Experiment 011's `0x7f`; see
`CONTROLLED_NOTE_VELOCITY_124_REPLICATION.md`. Experiment 016 completed the
velocity-121 replication; see `CONTROLLED_NOTE_VELOCITY_121_REPLICATION.md`.
The next experiment should make one exact duration-only edit to the same note
while holding pitch and velocity stable.
