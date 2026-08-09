# Objective

This report analyzes Experiment 017, a controlled duration-only edit of the
established C1 drum note from 442 to 441 while pitch, attack velocity, release
velocity, position, and note count remained unchanged. The work tests for a
duration representation near the confirmed pitch/velocity bytes. It is
read-only evidence gathering and does not claim complete event framing.

# Experimental provenance

Experiment 007 is the known-good baseline. Experiment 017 was made from a
fresh native Finder duplicate of that baseline. In Studio Vision's List
Window, the user edited only the fourth event at position `25·4·469`, changing
duration from 442 to 441 while retaining pitch C1, attack velocity 127, release
velocity 92, and note count. The user saved on quit, reopened and verified the
project as functional, quit without saving, and Finder-copied it through
`Unix` into the research directory.

Directory inspection positively identified the uncompressed project as
`/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 017 - Single MIDI Note Duration Down One/newest STUFF baseline EXP17`.
The reported basename was therefore confirmed rather than assumed.

# Artifact inventory

| Artifact | Exact path | Size | SHA-256 |
|---|---|---:|---|
| Experiment 007 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline` | 211,468 | `e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132` |
| Experiment 017 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 017 - Single MIDI Note Duration Down One/newest STUFF baseline EXP17` | 211,468 | `a7cb579f310bca31712d92ee7cb882327838ef86e05006e9e2a5c26c2f805006` |

Experiment 017 has Finder Type `MID2`, Creator `MIDA`, and the same observed
metadata shape as the clean controlled saves: `com.apple.FinderInfo` (32
bytes), `org.BasiliskII.FinderInfo` (16 bytes), and
`org.BasiliskII.ExtendedFinderInfo` (16 bytes). No other extended attribute or
resource fork was observed.

# Pitch and velocity anchor verification

The preregistered pair at `0x00031c1f–0x00031c20` is exactly `24 7f` in
Experiment 017. Pitch remains direct C1 value `0x24`, and attack velocity
remains direct decimal-127 value `0x7f`.

The 81 pitch anchors were reconstructed from positions where Experiment 010 is
`0x23`, Experiment 007 is `0x24`, and Experiment 009 is `0x25`. Experiment 017
contains `0x24` at every anchor.

| Experiment 017 value | Count |
|---|---:|
| `0x24` | 81 |
| `0x23` | 0 |
| `0x25` | 0 |
| other | 0 |

Both preregistered stability tests succeed without exception.

# Direct duration-value search

No literal 16-bit or 32-bit integer representation changes from `0x01ba`
(442) to `0x01b9` (441) in either byte order. No `01 ba -> 01 b9` or
`ba 01 -> b9 01` sequence was found.

One exact transformed numeric representation was found. At
`0x00031c22–0x00031c23`, baseline bytes `83 3a` become `83 39`. Under the
standard 7-bit big-endian variable-length quantity calculation:

- `83 3a` decodes as `((0x83 & 0x7f) << 7) | (0x3a & 0x7f) = 442`;
- `83 39` decodes as `((0x83 & 0x7f) << 7) | (0x39 & 0x7f) = 441`.

Thus the field reflects the exact known duration change by one. Only its
low-order seven-bit component changes, `0x3a -> 0x39`. This is not a literal
fixed-width `0x01ba -> 0x01b9` representation, but it is an exact direct
numeric relationship under a documented mechanical encoding.

Across the whole file, `0x00031c23` is the only byte that decreases by one.
It is also the only such position stable in all inspected pitch and velocity
controls.

# Pitch/velocity neighborhood analysis

The compact nine-byte local sequence at `0x00031c1b–0x00031c23` is:

| Artifact or controlled state | Bytes |
|---|---|
| Experiment 007 baseline | `81 3b 81 65 24 7f 5c 83 3a` |
| Experiment 009, pitch C#1 | `81 3b 81 65 25 7f 5c 83 3a` |
| Experiment 010, pitch B0 | `81 3b 81 65 23 7f 5c 83 3a` |
| Experiment 013, velocity 126 | `81 3b 81 65 24 7e 5c 83 3a` |
| Experiment 014, velocity 125 | `81 3b 81 65 24 7d 5c 83 3a` |
| Experiment 015, velocity 124 | `81 3b 81 65 24 7c 5c 83 3a` |
| Experiment 016, velocity 121 | `81 3b 81 65 24 79 5c 83 3a` |
| Experiment 017, duration 441 | `81 3b 81 65 24 7f 5c 83 39` |

Relative to pitch byte `0x00031c1f`, independently controlled evidence now
locates:

- relative `+0`: pitch, directly matching the tested note number;
- relative `+1`: attack velocity, directly matching five tested values;
- relative `+2`: stable `0x5c`, numerically equal to the reported unchanged
  release velocity 92, but not yet independently edited;
- relative `+3..+4`: a 7-bit variable-length value that exactly follows the
  tested duration 442 to 441 change.

Within the tight structure `0x00031c1b–0x00031c23`, the only Experiment 017
difference is `0x00031c23: 3a -> 39`. In a wider 130-byte inspection window,
three additional differences at `0x00031c05–0x00031c07` occur, but those
positions are previously variable save output and have no duration-specific
numeric relationship.

The compact adjacency and independent responses provide the first strong
evidence for a larger note-data structure around the pitch/velocity pair. They
do not yet establish its start, end, ownership, timing fields, or full event
semantics.

Experiment 018 subsequently changed only release velocity from 92 to 91 and
produced exact sequence `24 7f 5b 83 3a`. This independently confirms the
intervening byte as release velocity while pitch, attack velocity, and duration
remain stable. See `CONTROLLED_NOTE_RELEASE_VELOCITY_CHANGE.md`.

# Dense-anchor neighborhood analysis

Consistent eight-byte windows on both sides of all 80 dense pitch anchors in
`0x00031c1f–0x00031f98` were compared. Exactly one neighborhood changes: the
first anchor at `0x00031c1f`. Its only difference is relative `+4`,
`0x00031c23: 3a -> 39`.

The other 79 bounded neighborhoods are byte-identical between Experiments 007
and 017. No second pitch-bearing structure responds, and no competing
duration-like field is observed in those windows. The changed neighborhood is
the same first anchor previously established by direct velocity edits.

# Whole-file comparison

Experiments 007 and 017 both contain 211,468 bytes. They differ at 1,761
same-position bytes in 632 disjoint runs. The first difference is
`0x0000001e`, the last is `0x00033a06`, the maximum unequal run is 36 bytes,
the common prefix is 30 bytes, and the common suffix is five bytes.

`Track 7` remains at `0x0002f6ca`, `Ode to Clarke` remains at
`0x0002f753`, and the established labels and 166-byte cadence remain aligned.
Same-position comparison is therefore appropriate without inferring
insertions or deletions.

Of the 1,761 unequal offsets, 1,760 had already varied in at least one of
Experiments 005, 006, or 008–016. Only `0x00031c23` is new relative to the
entire prior control set. There are no other Experiment-017-only unresolved
positions.

The evidence classifies as:

- recurring or previously variable save-output positions: 1,760;
- strong duration-correlated candidate: one byte at `0x00031c23`, within the
  two-byte field `0x00031c22–0x00031c23`;
- Experiment-017-only unresolved positions: zero after separating the exact
  duration candidate.

# Duration-correlated candidates

The strongest and only control-stable candidate is
`0x00031c22–0x00031c23: 83 3a -> 83 39`. It exactly represents 442 to 441 as a
7-bit big-endian variable-length quantity, occurs three bytes after the
confirmed pitch byte and two bytes after the confirmed velocity byte, and is
the sole changed field in the tight local structure.

No literal 16-bit or 32-bit duration candidate was found. No weak candidate
survives control filtering: all other differences occur at previously variable
positions, and none supplies another numeric decrease by one.

# Evidence supported

- The established pitch/attack-velocity pair remains exactly `24 7f`.
- All 81 established pitch anchors remain `0x24`.
- No literal fixed-width `0x01ba -> 0x01b9` representation was found.
- The adjacent field `83 3a -> 83 39` exactly represents duration 442 to 441
  under a 7-bit big-endian variable-length quantity calculation.
- `0x00031c23` is the only whole-file byte decrease by one and the only new
  position after control subtraction.
- Exactly one of 80 dense pitch-anchor neighborhoods changes, at the first
  anchor previously tied to direct velocity.
- The compact sequence independently tracks pitch, attack velocity, and
  duration. Its intervening stable byte `0x5c` also equals the reported release
  velocity 92, though that relationship has not yet been controlled.
- The evidence begins to support a larger note-data structure around the
  established pair, but does not establish complete framing.

# Unknowns

- The stable `0x5c` byte has not been independently tested as release velocity.
- The meanings of the four bytes preceding the pitch remain unknown.
- The full structure boundaries, timing/position representation, channel,
  ownership, ordering, and note-on/note-off semantics remain unknown.
- It is unknown whether all durations use the observed variable-length form or
  whether the field width changes at encoding boundaries.
- No valid Standard MIDI File has yet been recovered.

# Single recommended next step

Experiment 018 completed the release-velocity test and exactly produced
`0x00031c21: 5c -> 5b`; see
`CONTROLLED_NOTE_RELEASE_VELOCITY_CHANGE.md`. The next controlled experiment
should move only the same note from `25·4·469` to `25·4·468`, retaining the
confirmed five-byte musical-property sequence, to search the adjacent stable
bytes for a timing-correlated response.
