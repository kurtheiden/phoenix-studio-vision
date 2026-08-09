# Objective

This report analyzes Experiment 018, a controlled release-velocity-only edit
of the established C1 drum note from 92 to 91. It tests the remaining
unverified byte inside the compact five-byte note-data candidate. The work is
read-only evidence gathering and does not claim that the five bytes constitute
an entire event.

# Experimental provenance

Experiment 007 is the known-good baseline. Experiment 018 was created from a
fresh native Finder duplicate of that baseline. In Studio Vision's List
Window, the user edited only the fourth event, changing release velocity from
92 to 91 while preserving position `25·4·469`, pitch C1, attack velocity 127,
duration 442, and note count. The user saved on quit, reopened and verified the
project as functional, quit without saving, and Finder-copied it through
`Unix` into the research directory.

Directory inspection positively identified the uncompressed project as
`/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 018 - Single MIDI Note Release Velocity Down One/newest STUFF baseline EXP18`.
It is 211,468 bytes and has SHA-256
`d948b5b391833d4184b85db1c85e310ec3212db5ddf626227a19e0302c736851`.
Finder Type is `MID2`, Creator is `MIDA`, and the observed attributes are
`com.apple.FinderInfo` (32 bytes), `org.BasiliskII.FinderInfo` (16 bytes), and
`org.BasiliskII.ExtendedFinderInfo` (16 bytes). No other attribute or resource
fork was observed.

# Preregistered five-byte result

Before broad comparison, `0x00031c1f–0x00031c23` was inspected. Experiment 018
contains exactly:

`24 7f 5b 83 3a`

The complete preregistered prediction succeeds:

| Relative field | Baseline | Experiment 018 | Expected result |
|---|---|---|---|
| pitch, `0x00031c1f` | `24` | `24` | stable C1 |
| attack velocity, `0x00031c20` | `7f` | `7f` | stable 127 |
| release-velocity candidate, `0x00031c21` | `5c` | `5b` | exact 92 to 91 |
| duration, `0x00031c22–0x00031c23` | `83 3a` | `83 3a` | stable 442 |

Only the preregistered third byte changes.

# Pitch-anchor verification

The 81 pitch anchors were reconstructed from positions where Experiment 010 is
`0x23`, Experiment 007 is `0x24`, and Experiment 009 is `0x25`. Experiment 018
contains `0x24` at all 81 positions.

| Experiment 018 value | Count |
|---|---:|
| `0x24` | 81 |
| `0x23` | 0 |
| `0x25` | 0 |
| other | 0 |

There are no exceptions.

# Attack-velocity and duration controls

Attack velocity remains `0x7f` at `0x00031c20`, exactly matching unchanged
decimal value 127. The duration field remains `83 3a` at
`0x00031c22–0x00031c23`. Its established 7-bit big-endian variable-length
quantity calculation is:

`((0x83 & 0x7f) << 7) | (0x3a & 0x7f) = 442`

Both independently established fields remain stable during the
release-velocity-only edit.

# Direct release-velocity search

The whole-file comparison contains exactly one direct `0x5c -> 0x5b` change,
at `0x00031c21`. It is also the only single-byte decrease by one. A second
baseline-differing `0x5b` appears at `0x000246b1`, but that position is
previously variable save output and does not survive control filtering.

`0x00031c21` is therefore the only control-stable direct release-velocity
candidate. It is immediately adjacent to the independently confirmed pitch,
attack-velocity, and duration fields. Numeric identity alone would not prove
ownership, but the preregistered response and independent field controls make
this evidence deterministic for the tested structure.

# Controlled five-byte series

| Experiment | Intentional change | Five-byte structure |
|---|---|---|
| 007 | baseline | `24 7f 5c 83 3a` |
| 009 | pitch C1 to C#1 | `25 7f 5c 83 3a` |
| 010 | pitch C1 to B0 | `23 7f 5c 83 3a` |
| 013 | attack velocity 127 to 126 | `24 7e 5c 83 3a` |
| 014 | attack velocity 127 to 125 | `24 7d 5c 83 3a` |
| 015 | attack velocity 127 to 124 | `24 7c 5c 83 3a` |
| 016 | attack velocity 127 to 121 | `24 79 5c 83 3a` |
| 017 | duration 442 to 441 | `24 7f 5c 83 39` |
| 018 | release velocity 92 to 91 | `24 7f 5b 83 3a` |

The candidate release-velocity byte remains `0x5c` through every unrelated
clean controlled edit and changes only in Experiment 018. Each manipulation
affects only its predicted byte or duration component within this five-byte
window.

# Dense-anchor neighborhood analysis

The same eight-byte windows on both sides of all 80 dense pitch anchors in
`0x00031c1f–0x00031f98` were compared. Exactly one neighborhood changes: the
first anchor at `0x00031c1f`. Its only difference is relative `+2`,
`0x00031c21: 5c -> 5b`.

The other 79 neighborhoods are byte-identical. No competing `5c -> 5b` change
occurs beside another pitch anchor, and no second dense structure responds to
the intentional edit.

# Whole-file comparison

Experiments 007 and 018 both contain 211,468 bytes. They differ at 1,765
same-position bytes in 632 disjoint runs. The first difference is
`0x0000001e`, the last is `0x00033a06`, the maximum unequal run is 36 bytes,
the common prefix is 30 bytes, and the common suffix is five bytes.

`Track 7` remains at `0x0002f6ca`, `Ode to Clarke` remains at
`0x0002f753`, and the established label cadence remains aligned. Same-position
comparison is used without inferring insertions or deletions.

Of the 1,765 unequal offsets, 1,764 had already varied in Experiments 005, 006,
or 008–017. Only `0x00031c21` is new relative to the complete prior control
set. Classification is therefore:

- recurring or previously variable save-output positions: 1,764;
- release-velocity-correlated candidate: one byte at `0x00031c21`;
- Experiment-018-only unresolved positions: zero after separating that exact
  candidate.

# Emerging note-data structure

Controlled evidence now supports the contiguous baseline sequence
`24 7f 5c 83 3a` field by field:

- **Pitch (`24`)**: independently changed one semitone upward and downward,
  producing exact direct values `25` and `23`.
- **Attack velocity (`7f`)**: independently changed to 126, 125, 124, and 121,
  producing exact direct values `7e`, `7d`, `7c`, and `79` in the clean series.
- **Release velocity (`5c`)**: independently changed from 92 to 91 and produced
  the exact direct value `5b`, with all other fields stable.
- **Duration (`83 3a`)**: independently changed from 442 to 441 and produced
  exact 7-bit VLQ value `83 39`, with all other fields stable.

The four musical properties can therefore be considered independently
supported, in this order, within one contiguous five-byte structure for the
tested note. This establishes neither the entire note event nor its external
framing. It does not yet locate timing, channel, ownership, status, record
boundaries, or relationships to neighboring notes.

# Preceding-byte observations

The four bytes immediately preceding pitch are `81 3b 81 65`, giving the local
nine-byte sequence `81 3b 81 65 24 7f 5c 83 3a`. They remain byte-identical in
Experiments 007, 009, 010, and 013–018 across pitch, attack-velocity,
release-velocity, and duration edits.

Mechanically, the bytes form two well-formed 7-bit big-endian variable-length
quantities:

- `81 3b` decodes to `187`;
- `81 65` decodes to `229`.

They cannot form one standard four-byte VLQ because `0x3b` terminates the
first value. The observed values do not directly equal the reported position
components `25`, `4`, or `469`, and prior research established no repeatable
timing representation connecting them to the note position. They remain
plausible adjacent numeric fields but are not assigned timing semantics.

# Evidence supported

- Experiment 018 exactly matches preregistered sequence `24 7f 5b 83 3a`.
- `0x00031c21` changes exactly `5c -> 5b` for release velocity 92 to 91.
- Pitch remains `0x24`, attack velocity remains `0x7f`, and duration remains
  `83 3a`, mechanically decoding to 442.
- All 81 pitch anchors remain `0x24`.
- `0x00031c21` is the only control-stable direct release-velocity candidate and
  the only new offset after prior-control subtraction.
- Exactly one of 80 dense-anchor neighborhoods changes, and only at relative
  `+2` beside the first anchor.
- Pitch, attack velocity, release velocity, and duration now have independent
  controlled support within one contiguous five-byte structure.
- The stable preceding bytes mechanically decode as VLQs 187 and 229, without
  supported timing semantics.

# Unknowns

- The full event boundaries and the meanings of preceding and following bytes
  remain unknown.
- The note position/timing representation has not been identified.
- Channel, ownership, status, ordering, and note-on/note-off semantics remain
  unknown.
- It is unknown whether every note uses the same field sequence and duration
  encoding width.
- No valid Standard MIDI File has yet been recovered.

# Single recommended next step

From a fresh native Finder duplicate of Experiment 007, move only the same note
from position `25·4·469` to `25·4·468` in the List Window while retaining C1,
attack velocity 127, release velocity 92, duration 442, and note count.
Preregister the confirmed five-byte sequence `24 7f 5c 83 3a` as stable, then
test the preceding `81 3b 81 65` values and other compact neighboring bytes for
one control-stable timing-correlated response. This is the narrowest experiment
for extending the confirmed musical-property structure toward event timing.
