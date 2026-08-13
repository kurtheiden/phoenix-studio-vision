# Objective

Identify and confirm the project-file representation of the editable Patch-name
text in the first Patch event of `Ode to Clarke` / `Track 3 #2` / `JD-800` by
changing only `Ming Dynasty` to the equal-length `Phoenix Test`. No parsing was
implemented and no Studio Vision artifact was modified.

# Controlled change

The operator changed only the first List Window Patch event's editable name:

`Ming Dynasty -> Phoenix Test`.

Both strings contain exactly 12 ASCII characters including their space. The
event remained at `1·2·50` with displayed `PC 23`. After saving, the project
was reopened in Studio Vision and verified to show `1·2·50`, Patch,
`Phoenix Test`, and `PC 23` before being copied to the host. No intentional
instrument, note, note-position, pitch, duration, velocity, or other track-data
change was made.

# Experiment lineage

Experiment 026 was created from a fresh Experiment 007 duplicate. It does not
descend from Experiments 023–025. All primary alignment and difference results
compare Experiment 026 directly with Experiment 007; prior experiments are
used only as independent property controls and save-variation evidence.

# Preregistered predictions

Before Experiment 026 was inspected, the primary prediction was locked: the
aligned payload at baseline `0x31891–0x3189c` should change directly from the
12 ASCII bytes for `Ming Dynasty` to the 12 ASCII bytes for `Phoenix Test`,
without shifting following structure. Stable predictions were that the Patch
position field would remain 530, the Program Change field would remain `0x17`,
and the first-note boundary and complete 84-note stream would remain at their
established structural positions and musically unchanged. No prediction was
made for known save-dependent fields.

# Baseline and experiment identity

| Artifact | Exact path | Size | SHA-256 | Finder Type / Creator | Inode |
|---|---|---:|---|---|---:|
| Experiment 007 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline` | 211,468 | `e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132` | `MID2` / `MIDA` | 17242646 |
| Experiment 026 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 026 - Track 3-2 Patch Name Change/newest STUFF baseline EXP26` | 211,468 | `4da9e3c3dd6eb943f69b6afa7b19d35df783886c08077bfb899d43d047d0005b` | `MID2` / `MIDA` | 17361616 |

Both uncompressed data forks expose FinderInfo beginning `MID2MIDA` and the
same three FinderInfo-related extended attributes. The baseline directory's
separate 89,525-byte `.sit` is identified as a StuffIt archive, not the project
data fork. Different hashes and inodes confirm distinct project files.

# Structural alignment

Track 3 #2 did not relocate or change size. Both files are 211,468 bytes, and
independent anchors remain at the same offsets:

- absolute-position VLQ `84 12` = 530: `0x31886–0x31887`;
- baseline `Ming Dynasty` / experiment `Phoenix Test`: `0x31891–0x3189c`;
- confirmed `ff ff ff 17` PC context: `0x318a2–0x318a5`;
- first-note status and properties: `0x318b4` / `0x318b5`;
- unique complete 585-byte note chain: `0x318b5–0x31afe`;
- established 17-note sample: `0x31994`, selected from two musical hits by the
  unique first-note and complete-chain context;
- stable post-chain context: `0x31afe`.

A whole-file same-position comparison finds 1,775 changed bytes in 635 runs.
The equal size, aligned independent anchors, unchanged following bytes, and
unique complete note chain establish same-offset alignment without relying on
a whole-file name search.

# Controlled-data stability

The absolute Patch position remains `84 12` = 530 at `0x31886–0x31887`. The
confirmed PC field remains `0x17` at `0x318a5`. The complete stream at
`0x318b5–0x31afe` is byte-for-byte unchanged, covering all 84 pitches, attack
velocities, release velocities, and durations plus all 83 note-to-note timing
fields. No unexpected musical-data change was found.

# Patch-name field

The preregistered prediction is **CONFIRMED** at the same aligned payload range,
`0x31891–0x3189c`:

| State | ASCII | Hexadecimal |
|---|---|---|
| Experiment 007 | `Ming Dynasty` | `4d 69 6e 67 20 44 79 6e 61 73 74 79` |
| Experiment 026 | `Phoenix Test` | `50 68 6f 65 6e 69 78 20 54 65 73 74` |

All 12 payload bytes change directly, and the following byte remains at
`0x3189d`; no insertion, deletion, padding change, or relocation occurs. The
old and new complete strings each occur once in their respective project and
only in this aligned primary field, but structural alignment—not string search
alone—identifies the field.

# Name-field framing

The immediately preceding byte at `0x31890` is stable `0x0c`, decimal 12,
followed by exactly the 12 name bytes. The next byte at `0x3189d` is `0x03`,
followed by literal `I38` at `0x3189e–0x318a0`, and `0x318a1` is `0x04`.
The compact aligned context is:

`... 00 17 0c [12-byte Patch name] 03 49 33 38 04 ff ff ff 17 ...`

This is strong structural evidence that `0x0c` is a Pascal-style one-byte
length prefix for the Patch name, reinforced by the immediately following
`03 I38` pattern. There is no null terminator after the name, and no padding or
delimiter change is observed. However, the equal-length control leaves the
`0x0c` unchanged and cannot establish whether it changes with name length,
whether 12 is a fixed maximum, or how record size/relocation behaves. Therefore
the 12-byte payload location is confirmed, while general field width and
string-length framing remain **PARTIAL** pending a different-length control.
No pointer or record-length field is identified by this experiment.

# Local binary differences

Within the established aligned window `0x31870–0x31b10`, exactly 15 bytes
change in two runs:

| Relative to `0x31882` | Baseline / Experiment 026 offset | Baseline | Experiment 026 | Classification | Confidence |
|---:|---:|---|---|---|---|
| `+0x01..+0x03` | `0x31883–0x31885` | `c4 b2 5c` | `c7 84 e0` | known save-variable marker/header field | high as save-variable; semantics unknown |
| `+0x0f..+0x1a` | `0x31891–0x3189c` | `Ming Dynasty` bytes | `Phoenix Test` bytes | intentional editable Patch-name payload | high |

No dependent structural byte changes within the primary Patch/name/note
window. In particular, the `0x0c` candidate prefix, `03 I38 04` following
context, position, PC, interval components, note status, and complete note
chain are stable. Exact byte-level evidence is preserved in the external CSV.

# Prior-experiment controls

The Patch-name payload remains `Ming Dynasty` at the same offsets in PC-only
Experiments 023 and 024 and position-only Experiment 025. It changes only in
Experiment 026. Conversely, the controlled timing fields change only in
Experiment 025, and the confirmed PC field changes only in Experiments 023 and
024. All bytes immediately surrounding the name—prefix `0x0c`, following
`03 I38 04`, and then `ff ff ff`—remain stable across all five aligned states.
This orthogonal control pattern ties the payload specifically to the editable
Patch-name property with high confidence.

# Patch timing stability

The primary absolute-position field remains `84 12` = 530 at
`0x31886–0x31887`, matching displayed `1·2·50`. The interval component remains
`c5 4c` = 8,908 at `0x318a6–0x318a7`, and the later candidate remains
`81 25` = 165 at `0x318b2–0x318b3`; their sum remains 9,073. The text-only
edit supplies stability controls and does not alter the timing interpretation.

# Program Change stability

The aligned direct PC field remains `0x17` at `0x318a5`, inside unchanged
`ff ff ff 17`. Across the controlled states the field is:

- Experiment 007, displayed 23: `0x17`;
- Experiment 023, displayed 24: `0x18`;
- Experiment 024, displayed 100: `0x64`;
- Experiment 025, displayed 23: `0x17`;
- Experiment 026, displayed 23: `0x17`.

This preserves the independent direct Program Change conclusion.

# Repeated copies or references

`Phoenix Test` has no additional literal copy in Experiment 026. The six known
downstream Patch-related structures retain fixed-width position 530 and cached
PC 23; they contain no name payload. Their companion `c5 -> c7` bytes, plus
other downstream marker/count changes, recur as save output in prior controls
and do not correlate specifically with the name edit. No name-specific index,
reference, fixed-width copy, or dependent structural change is identified.

# Updated Patch-event field map

Three independently manipulated fields now form one coherent ordered primary
span:

| Field | Offset/range | Width | Encoding | Controlled evidence | Confidence |
|---|---|---:|---|---|---|
| absolute Patch position | `0x31886–0x31887` | 2 bytes here | 7-bit big-endian VLQ, 530/531 | Experiment 025 | high |
| Patch-name length candidate | `0x31890` | 1 byte | `0x0c`, likely Pascal length | structural/equal-length Experiment 026 | partial |
| editable Patch-name payload | `0x31891–0x3189c` | 12 bytes here | ASCII | Experiment 026 | high |
| Program Change | `0x318a5` | 1 byte | direct displayed value | Experiments 023/024 | high |
| interval component | `0x318a6–0x318a7` | 2 bytes here | 7-bit big-endian VLQ, 8,908/8,907 | Experiment 025 | high response; partial ownership |

Unresolved gaps remain at `0x31888–0x3188f`, `0x3189d–0x318a1`, and
`0x318a8–0x318b1`; `0x318b2–0x318b3` is the stable 165 interval component
candidate. Note status begins at `0x318b4`, and first-note properties begin at
`0x318b5`. The confirmed properties support a coherent primary Patch span from
`0x31886` through at least `0x318a7`, but unresolved field ownership and the
compound interval still prevent a justified complete record end/grammar.

Patch-name storage classification:

- **A. Editable text directly stored: YES.**
- **B. Exact payload location identified: YES.**
- **C. Field width established: PARTIAL.** Twelve payload bytes are established
  for both tested names; fixed/maximum/variable width is unknown.
- **D. String framing/length mechanism established: PARTIAL.** `0x0c` is a
  strong Pascal-length candidate but was not varied.
- **E. Independent from Program Change: YES.**
- **F. Independent from absolute position: YES.**
- **G. Additional copies/references identified: NO.**

Overall Patch-event model:

- **A. Program Change field confirmed: YES.**
- **B. Absolute Patch position field confirmed: YES.**
- **C. Patch-name field confirmed: YES.**
- **D. Patch-to-first-note timing relationship understood: PARTIAL.**
- **E. Patch event fully bounded: PARTIAL.**
- **F. Event-type discriminator established: PARTIAL.**
- **G. Enough evidence for a bounded Patch decoder: PARTIAL.** A diagnostic
  extractor hard-bounded to this known representation could safely report the
  confirmed position, 12-byte name payload, and PC fields. A coherent Patch
  event decoder or general Studio Vision Patch parser is not yet justified
  because name-length framing, event end, interval ownership, and event type
  remain incomplete.

# Event-type discriminator

Event-type discrimination does not improve beyond **PARTIAL**. Stable
`ff ff ff` immediately before the controlled PC field is more securely inside
the coherent Patch property span and remains a plausible Patch-specific
discriminator. The `90` at `0x318b4` still strongly identifies the following
note stream. But no controlled edit varied an event type, and no same-structure
Patch-versus-Note comparison isolates a definitive discriminator width or
value. No claims extend to other event types.

# Evidence supported

- Experiment 026 is a distinct same-size `MID2/MIDA` project descended from a
  fresh Experiment 007 duplicate.
- Track 3 #2 remains at the same offsets and size.
- The locked 12-byte ASCII replacement is confirmed exactly at
  `0x31891–0x3189c` with no following shift.
- Position 530, PC 23, both timing components, all 84 note properties, and all
  83 note-to-note timing fields remain stable.
- The stable preceding `0x0c` is a strong but not yet controlled Pascal-length
  candidate; no null termination or padding is observed.
- The name payload changes only in the name-only experiment across five
  controlled states.
- No additional literal name copy or name-specific downstream reference is
  identified.
- Position, name, and PC now occupy confirmed fields in one coherent local
  span, but complete framing and event type remain partial.

# Unknowns

Whether `0x0c` varies as a length byte, fixed versus variable name capacity,
shorter/longer-name padding or relocation, record-size consequences, meanings
of unresolved gaps, complete Patch end ownership, compound interval grammar,
exact `ff` ownership/type semantics, downstream structure ownership, and
generality beyond this event and environment remain unknown.

# Single recommended next step

Change only the Patch name to a deliberately shorter value. This is the
smallest highest-information control for testing whether `0x31890` is a length
byte, whether the payload is variable-width or padded, whether following fields
relocate, and whether any record-size field changes. Do not implement a Patch
decoder until that framing evidence is available.
