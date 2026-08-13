# Objective

Identify the project-file representation affected by moving only the first
Patch event in `Ode to Clarke` / `Track 3 #2` / `JD-800` one Studio Vision
timing unit later, from `1·2·50` to `1·2·51`. No parsing was implemented and no
Studio Vision artifact was modified.

# Controlled change

The human operator changed only the first List Window event's position:

`1·2·50 -> 1·2·51`.

The event remained type Patch with name `Ming Dynasty` and displayed `PC 23`.
After saving, the project was reopened in Studio Vision and those values were
verified before it was copied to the host. No intentional note, note-position,
pitch, duration, velocity, instrument, or other track-data change was made.

# Experiment lineage

Experiment 025 was created from a fresh duplicate of Experiment 007. It does
not descend from Experiment 023 or 024. All primary alignment and difference
results compare Experiment 025 directly with Experiment 007; the PC-only saves
are used only as independent controls for field stability and save variation.

# Preregistered predictions

Before Experiment 025 was inspected, two predictions were locked:

1. an unknown project representation controlling Patch position should change
   by `+1`, without preregistering offset, width, byte order, or encoding;
2. conditionally, if the displayed Patch-to-first-note interval is stored, its
   value should change from 9,073 to 9,072 because the Patch moves one unit
   later while the first note remains at `6·1·3`.

Stable controls were also preregistered: the confirmed PC byte should remain
`0x17`, literal `Ming Dynasty` should remain unchanged, and the complete
84-note performance stream should remain musically unchanged.

# Baseline and experiment identity

| Artifact | Exact path | Size | SHA-256 | Finder Type / Creator | Inode |
|---|---|---:|---|---|---:|
| Experiment 007 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline` | 211,468 | `e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132` | `MID2` / `MIDA` | 17242646 |
| Experiment 025 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 025 - Track 3-2 Patch Position Plus One/newest STUFF baseline EXP25` | 211,468 | `16737c2a0f31e97a4411a1a48d9217d9feebfb23260eedfe0e86ee9ca38959ad` | `MID2` / `MIDA` | 17360405 |

Both uncompressed data forks expose FinderInfo beginning `MID2MIDA` and the
same three FinderInfo-related extended attributes. The baseline directory's
separate 89,525-byte `.sit` file is identified as a StuffIt archive, not the
project data fork. Different hashes and inodes confirm distinct project files.

# Structural alignment

Track 3 #2 did not relocate or change size. Both files are 211,468 bytes, and
independent anchors remain at the same offsets:

- unique literal `Ming Dynasty`: `0x31891`;
- confirmed `ff ff ff 17` PC context: `0x318a2`;
- unique first-note properties: `0x318b5`;
- unique complete 585-byte 84-note chain: `0x318b5–0x31afe`;
- established 17-note sample: `0x31994`, selected from two musical hits by the
  unique name, first note, and complete-chain context;
- stable post-chain context: `0x31afe`.

A whole-file same-position comparison finds 1,768 changed bytes in 642 runs.
The broad population is ordinary save variation; the aligned anchors and equal
file size show no local relocation or expansion.

# Controlled-data stability

Literal `Ming Dynasty`, the confirmed PC field `0x318a5 = 0x17`, the first-note
boundary, and the entire note stream at `0x318b5–0x31afe` are byte-for-byte
unchanged. This covers all 84 pitches, attack velocities, release velocities,
and durations, plus all 83 established note-to-note timing fields. Thus the
project bytes are consistent with the operator's observation that note
positions remained unchanged; the project comparison cannot independently
observe the UI but finds no note-timing change.

# Local binary differences

Within the established aligned window `0x31870–0x31b10`, four bytes change in
three runs:

| Relative to `0x31882` | Baseline / Experiment 025 offset | Baseline | Experiment 025 | Interpretation | Confidence |
|---:|---:|---|---|---|---|
| `+0x01..+0x02` | `0x31883–0x31884` | `c4 b2` | `c7 85` | part of save-variable three-byte marker/header field (`c4 b2 5c -> c7 85 5c`) | high as save-variable; exact semantics unknown |
| `+0x05` | `0x31887` within `0x31886–0x31887` | `84 12` = 530 | `84 13` = 531 | absolute Patch position, 7-bit big-endian VLQ | high |
| `+0x25` | `0x318a7` within `0x318a6–0x318a7` | `c5 4c` = 8,908 | `c5 4b` = 8,907 | Patch-to-first-note interval component | high for timing response; partial ownership |

The contexts are respectively `2c [c4 b2 5c] 84 12 ff 7c`,
`c4 b2 5c [84 12] ff 7c 1b`, and
`ff ff ff 17 [c5 4c] ff 60 07 57 ...`. The two timing fields were stable at
baseline values in PC-only Experiments 023 and 024. The save-variable field
changed in those controls and is rejected as a simple timing representation.

In the unresolved post-chain region before Track 7, six dependent structures
also copy the absolute Patch position as fixed-width big-endian
`00 00 02 12 -> 00 00 02 13` (530 to 531), with changed low bytes at
`0x31b3d`, `0x31b60`, `0x31b83`, `0x31b9e`, `0x31bc1`, and `0x31bec`.
Their neighboring cached PC byte remains `0x17`. Save-dependent companion
bytes `c5 -> c7` also occur in each structure. These copies corroborate the
position value but do not extend the primary Patch-event boundary or establish
the copied structures' ownership.

# Patch position field

The primary structural prediction is **CONFIRMED**. At `0x31886–0x31887`, the
standard 7-bit big-endian VLQ changes:

`84 12` = 530 -> `84 13` = 531.

Using the established coordinate conversion,
`1·2·50 = (1-1)×4×480 + (2-1)×480 + 50 = 530`, and `1·2·51 = 531`.
This field therefore stores the Patch event's absolute displayed position in
the same numeric timing unit used by the validated note coordinates. Six
fixed-width copies elsewhere independently reproduce the same numeric change.

# Patch-to-first-note interval test

The conditional prediction is **SUPPORTED**. The local VLQ at
`0x318a6–0x318a7` changes `c5 4c` = 8,908 to `c5 4b` = 8,907. A later stable
local VLQ at `0x318b2–0x318b3` remains `81 25` = 165. Their mechanical sums
are therefore:

- baseline: 8,908 + 165 = 9,073;
- Experiment 025: 8,907 + 165 = 9,072.

The controlled `-1` response and exact preregistered totals support a compound
Patch-to-first-note interval representation. Exact ownership and the reason
the interval is split across noncontiguous fields remain unresolved; it is not
promoted to a single conventional note-timing field.

# Comparison with note timing

- **Same units: YES.** The absolute Patch VLQ equals the 530/531 coordinate
  calculated in the established 480-units-per-beat system, one-for-one.
- **Same encoding: YES for the primary absolute field.** `84 12/84 13` uses
  the same standard 7-bit big-endian VLQ mechanics as note timing fields.
- **Absolute versus delta: established difference.** The Patch has an absolute
  position VLQ, whereas validated note-to-note fields are start-to-start
  intervals preceding later note properties.
- **First-note relation: PARTIAL.** `c5 4c/c5 4b` plus stable `81 25` exactly
  tracks the Patch-to-first-note interval, but the split representation and
  field ownership are not decoded.
- **Record structure: different.** The Patch timing/name/PC area contains
  metadata and multiple unresolved fields rather than the compact repeating
  note timing/property chain. The first note begins only at `0x318b4/0x318b5`.

# Program Change stability

The confirmed Program Change field at `0x318a5` remains `0x17`, decimal 23,
inside unchanged `ff ff ff 17`. This preserves the independent results from
Experiments 023 and 024 and rules out reinterpreting that byte as timing.

# Patch-name stability

The unique literal `Ming Dynasty` at `0x31891–0x3189c` remains byte-for-byte
unchanged. The position-only result preserves the conclusion that the literal
name is independently stored/editable from the direct Program Change value for
this event instance.

# Updated Patch-event boundaries

The earliest strongly supported primary Patch-event byte is now `0x31886`, the
start of its absolute position VLQ. Strongly supported Patch content continues
through the interval component ending at `0x318a7`. Within that span:

- `0x31886–0x31887`: absolute position VLQ;
- `0x31888–0x31890`: unresolved metadata;
- `0x31891–0x3189c`: literal Patch name;
- `0x3189d–0x318a1`: unresolved name/record metadata;
- `0x318a2–0x318a5`: Patch-specific `ff ff ff` context plus confirmed PC;
- `0x318a6–0x318a7`: interval component;
- `0x318a8–0x318b1`: unresolved intervening metadata;
- `0x318b2–0x318b3`: stable 165 interval component candidate;
- `0x318b4`: note-stream status/type-like byte `90`;
- `0x318b5`: first-note properties begin.

The timing, name, and PC fields now form one coherent contiguous local span,
but complete Patch ownership through `0x318b3` is only partial because several
bytes and the split interval remain unresolved. Patch and Note material appear
contiguous with intervening structural metadata; no gap or relocation is
observed, but a complete record grammar is not established.

# Event-type discriminator

No new event-type discriminator is isolated. Stable `ff ff ff` immediately
before the confirmed PC remains a plausible Patch-specific discriminator, and
`90` at `0x318b4` strongly identifies the following note stream. The timing
control does not vary or independently compare an event-type byte, so Patch
event-type discrimination remains **PARTIAL**.

# Evidence supported

- Experiment 025 is a distinct same-size `MID2/MIDA` project descended from a
  fresh Experiment 007 duplicate.
- Track 3 #2 remains at the same offsets and size.
- The Patch name, PC byte, all 84 note properties, and all 83 note-to-note
  timing fields remain unchanged.
- Absolute Patch position is a 7-bit VLQ at `0x31886–0x31887`, changing exactly
  530 to 531 in the established Studio Vision timing units.
- A second local VLQ changes 8,908 to 8,907; together with stable 165 it
  supports the preregistered interval change 9,073 to 9,072.
- Six fixed-width dependent copies also change 530 to 531.
- The primary Patch span is more precise, but complete ownership, the split
  interval grammar, and event-type discrimination remain incomplete.

Updated model:

- **A. Program Change value field still confirmed: YES.**
- **B. Patch position/timing field identified: YES.**
- **C. Patch timing uses the same units as note timing: YES.**
- **D. Patch timing uses the same encoding as note timing: YES** for the
  primary absolute VLQ; surrounding representation differs.
- **E. Patch-to-first-note interval storage identified: PARTIAL.**
- **F. Patch-name text still independent of Program Change value: YES.**
- **G. Patch event fully bounded: PARTIAL.**
- **H. Event-type discriminator established: PARTIAL.**

# Unknowns

The exact meanings of unresolved bytes inside `0x31888–0x318b3`, why the
Patch-to-first-note interval is split into 8,908 and 165, ownership of the
stable 165 field, complete Patch end ownership, exact semantics of the
save-variable three-byte field and dependent copies, and a definitive Patch
event-type discriminator remain unknown. Generality beyond this event and
Studio Vision environment is not established.

# Single recommended next step

Change only the literal Patch-name text to a deliberately different same-length
name while keeping position, PC, instrument, and notes unchanged. Timing and PC
are now established; a same-length name control offers the highest information
gain for determining literal field ownership and narrowing the Patch boundary
without introducing layout-length ambiguity.
