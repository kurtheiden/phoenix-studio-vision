# Objective

This report analyzes Experiment 012, a second verified-working velocity-only
edit of the same C1 drum note, and tests whether the byte isolated by
Experiment 011 responds reproducibly to another known velocity. The work is
read-only evidence gathering. It does not modify an artifact, implement parser
logic, or claim a complete event representation.

# Experimental provenance

Experiment 007 is the untouched verified-working baseline: pitch C1 and
velocity 127. Experiment 009 changed the same note's pitch to C#1 while leaving
velocity unchanged; Experiment 010 changed it to B0. Experiment 011 retained
C1 and changed velocity from 127 to 124. Those controls established 81
pitch-bearing positions and isolated a velocity-correlated byte immediately
after one pitch anchor.

Experiment 012 was created from a fresh native Finder duplicate of the same
known-good baseline. The user retained pitch C1 and changed only velocity from
127 to 121, leaving timing, duration, and note count unchanged. Studio Vision
saved the edit when the user quit and selected Save. The project reopened and
functioned normally, was quit without saving, and was Finder-copied through
SheepShaver's `Unix` shared volume into the research folder. The Experiment 007
StuffIt archive was excluded.

All five directories were inspected. Their uncompressed project basenames are
`newest STUFF baseline`, `newest STUFF baseline copy`,
`newest STUFF baseline EXP10`, `newest STUFF baseline EXP11`, and
`newest STUFF baseline EXP12` for Experiments 007, 009, 010, 011, and 012,
respectively. The reported Experiment 012 basename was confirmed.

# Artifact inventory

The requested full inventory for Experiments 007, 011, and 012 is:

| Artifact | Exact path | Basename | Data-fork size | Data-fork SHA-256 |
|---|---|---|---:|---|
| Experiment 007 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline` | `newest STUFF baseline` | 211,468 (`0x33a0c`) | `e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132` |
| Experiment 011 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 011 - Single MIDI Note Velocity Change/newest STUFF baseline EXP11` | `newest STUFF baseline EXP11` | 211,468 (`0x33a0c`) | `8f9a18f629a58a4eede289b181fae6ba1b61ef2361d28a2eb17cc0248747ccd6` |
| Experiment 012 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 012 - Second Single MIDI Note Velocity Change/newest STUFF baseline EXP12` | `newest STUFF baseline EXP12` | 211,468 (`0x33a0c`) | `d20da4c64bf13e6c044e9d36493da90a5c6c256671373010a289f320162b3fec` |

All three have Finder Type `MID2`, Finder Creator `MIDA`, a 32-byte
`com.apple.FinderInfo`, a 16-byte `org.BasiliskII.FinderInfo`, and a 16-byte
`org.BasiliskII.ExtendedFinderInfo`. No other extended attributes or resource
forks were observed. Their `com.apple.FinderInfo` values and zero-filled
extended FinderInfo values are identical. Experiment 007 has BasiliskII
FinderInfo `4d4944324d4944410100007b01080000`; Experiments 011 and 012
both have `4d4944324d4944410100ffffffff0000`.

The relevant labels have identical offsets in Experiments 007, 011, and 012:

| Label | Absolute offset |
|---|---:|
| `Meter Track` | `0x0002f04e` |
| `Tempo Track` | `0x0002f0f4` |
| `Track 1` | `0x0002f19a` |
| `Track 2` | `0x0002f240` |
| `sys100loops` | `0x0002f2e6` |
| `Track 4` | `0x0002f38c` |
| `Track 5` | `0x0002f432` |
| `Track 3` | `0x0002f4d8` |
| `Track 6` | `0x0002f57e` |
| `Track 3 #2` | `0x0002f624` |
| `Track 7` | `0x0002f6ca` |
| `Ode to Clarke` | `0x0002f753` |

All data forks have equal sizes, and the relevant labels and established
166-byte cadence remain aligned. Experiment 007 versus 012 has 1,805 unequal
same-position bytes in 647 runs, first difference `0x0000001e`, last
difference `0x00033a06`, maximum run 36 bytes, common prefix 30 bytes, and
common suffix five bytes. No insertion or deletion is inferred.

# Pitch-anchor stability

The established pitch set was reconstructed as the 81 positions where
Experiment 010 is `0x23`, Experiment 007 is `0x24`, Experiment 009 is `0x25`,
and Experiments 005, 006, and 008 retain `0x24`. Experiment 012 contains
`0x24` at every position.

| Experiment 012 value at the 81 pitch anchors | Count |
|---|---:|
| `0x24` | 81 |
| `0x23` | 0 |
| `0x25` | 0 |
| any other value | 0 |

There are no exceptions. The preregistered pitch-stability prediction succeeds
for a second independent velocity-only edit.

All 80 dense pitch anchors in `0x00031c1f–0x00031f98` remain `0x24`. The wider
range `0x00031c0f–0x00031fa8`, including exterior margins, is byte-identical
between Experiments 007 and 012. The prior negative finding of no velocity
response near the dense anchors is independently reproduced.

# Isolated pitch/velocity pair

The central pair at `0x0002f76f–0x0002f770` was inspected before broad
searching. Experiment 012 contains exactly `24 2d`.

| Experiment | Pitch | Velocity | Observed pair |
|---|---|---:|---|
| 007 | C1 | 127 | `24 0f` |
| 009 | C#1 | unchanged | `25 0f` |
| 010 | B0 | unchanged | `23 0f` |
| 011 | C1 | 124 | `24 2d` |
| 012 | C1 | 121 | `24 2d` |

The first byte remains the expected `0x24`, independently confirming that the
adjacent pitch anchor is stable during velocity-only edits. The second byte
reproducibly differs from baseline but takes the same `0x2d` value at velocities
124 and 121. It does not participate in a wider changed field: the compact
range `0x0002f761–0x0002f77f` is byte-identical between Experiments 011 and
012, including the complete pair.

Thus `0x0002f770` independently responds in the sense that a second
velocity-edited artifact reproduces its non-baseline state. It does not provide
an ordered response to the two edited velocity magnitudes.

# Velocity-series analysis

The observed series is:

| Velocity | Observed byte at `0x0002f770` | Change from preceding velocity point |
|---:|---:|---:|
| 127 | `0x0f` (15) | — |
| 124 | `0x2d` (45) | +30 encoded for -3 velocity |
| 121 | `0x2d` (45) | 0 encoded for -3 velocity |

This series does not match direct velocity (`7f`, `7c`, `79`), seven-bit
inverse velocity (`00`, `03`, `06`), eight-bit complement (`80`, `83`, `86`),
difference from 127, a fixed multiplication, a right or left shift, the high
or low velocity nibble, or signed reinterpretation of the observed bytes.

No single affine relationship `encoded = a * velocity + b` fits all three
points: the velocity inputs are equally spaced by -3, but the encoded outputs
change by +30 and then zero. In particular, the two-point relationship from
velocities 127 and 124 would predict `0x4b` (75) at velocity 121 under a linear
continuation; Experiment 012 instead contains `0x2d`. This independently
falsifies that simple extrapolation.

A threshold, quantized state, edit marker, lookup result, or many more complex
functions could fit the three observations, but none is selected by this
evidence. Modulo or piecewise formulas can be constructed after the fact and
are not treated as supported encodings. The byte is reproducibly
velocity-correlated, while its relationship to velocity magnitude remains
unknown.

# Experiment 011 vs Experiment 012

The two velocity-only artifacts have equal sizes and aligned labels. Their data
forks differ at 85 same-position bytes in 42 disjoint runs. The first difference
is `0x00000fdc`, the last is `0x000332e4`, the maximum run is five bytes, the
common prefix is 4,060 bytes, and the common suffix is 1,831 bytes.

There are no differences in the compact region around
`0x0002f76f–0x0002f770` and no difference anywhere in
`0x0002f740–0x0002f78f`. No position that is stable in every non-velocity
experiment differs between Experiments 011 and 012.

Two Experiment 011/012 bytes decrease by three, at `0x000227ef` and
`0x0002470b`, both `9d` to `9a`. They are not velocity candidates: both offsets
have different values across Experiments 007, 008, 009, and 010. Their numeric
relationship is therefore observed within the known run-varying population,
not isolated to the velocity series.

The lack of a control-stable 011/012 difference means no byte currently
distinguishes velocity 124 from 121. The same candidate field responds in both
velocity experiments only as the shared non-baseline value `2d`.

# Whole-file velocity search

Experiment 007 versus 012 contains no direct `0x7f` to `0x79` change and no
single-byte decrease by six. Mechanical searches at every byte alignment found
no two- or four-byte unsigned integer decreasing by six in either byte order
while the corresponding bytes remained stable in Experiments 008, 009, 010,
and 011.

After subtracting the union of positions already variable in Experiments 005,
006, 008, 009, 010, and 011, no new Experiment 012-only difference remains.
The `0x0002f770` candidate is not new to Experiment 012; it exactly reproduces
Experiment 011 and is therefore classified as reproducible velocity-correlated
evidence.

These negative searches do not show that velocity magnitude is absent from the
file. They show that neither the direct MIDI byte nor the tested simple numeric
relationships isolate it in this comparison.

# Save-run controls

The 1,805 Experiment 007/012 unequal positions classify as:

| Class | Count | Observation |
|---|---:|---|
| exactly reproduced Experiment 011 value, excluding `0x0002f770` | 1,734 | common output across the two velocity saves |
| other previously variable locations | 70 | offset varied in at least one of Experiments 005, 006, 008, 009, 010, or 011 |
| stable pitch anchors | 0 changed; 81 stable | all remain baseline `24` |
| reproducible velocity-correlated difference | 1 | `0x0002f770`, `0f` baseline and `2d` in both velocity experiments |
| new unresolved differences | 0 | no Experiment 012-only position survives control subtraction |

The 1,734 shared values include broad save-output behavior and are not inferred
to represent velocity. The 70 other locations are already variable across
prior artifacts. The velocity candidate is separated because it is stable in
all non-velocity controls, changes only in both velocity experiments, and lies
immediately after the independently established isolated pitch anchor.

# Evidence supported

- Experiment 012 is a same-size, verified-working artifact with aligned broad
  structure and unchanged relevant labels.
- All 81 pitch-bearing positions remain `0x24`, including all 80 dense-region
  anchors and the isolated `0x0002f76f` anchor.
- The dense pitch region plus margins is again completely unchanged during a
  velocity-only edit.
- Experiment 012 independently reproduces `0x2d` at `0x0002f770`, immediately
  following the stable isolated pitch byte.
- The two-byte pair is increasingly consistent with a pitch/velocity-bearing
  structure: the first byte tracks pitch and the second distinguishes baseline
  from both tested velocity-edited states.
- There is no evidence that `0x0002f770` is an ordered scalar encoding of
  velocity: velocities 124 and 121 produce the same value.
- The simple linear relationship suggested by the first two points is
  independently falsified by Experiment 012.
- No direct `7f` to `79`, bytewise -6, control-stable multi-byte -6, or other
  new velocity-magnitude candidate was found.
- Experiment 011 and 012 differ only at previously variable positions; no
  control-stable byte distinguishes their two velocity values.

# Unknowns

- It is unknown whether `0x2d` represents a threshold, quantized category,
  edit state, lookup output, or another property correlated with velocity.
- The actual magnitude encoding for velocity 127, 124, and 121 remains
  unknown.
- It is unknown whether the adjacent pair is a field pair, mapping entry,
  cached state, or another structure.
- The reason the 80 dense pitch-bearing bytes lack nearby velocity responses
  remains unknown.
- No event boundary, timing, duration, channel, track ownership, or
  note-on/note-off semantics have been established.
- No valid Standard MIDI File has yet been recovered.

# Single recommended next step

From a fresh native Finder duplicate of Experiment 007, change only the same C1
drum note's velocity from 127 to 126. After the verified-working save-on-quit
procedure, test whether `0x0002f770` remains baseline `0f` or enters the edited
`2d` state while all 81 pitch anchors remain `24`. This boundary-focused test
is the narrowest way to distinguish an immediate edit-state transition from a
coarser velocity threshold without assuming an unsupported numeric formula.
