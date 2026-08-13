# Objective

Replicate and strengthen the Experiment 023 Program Change result with a
deliberately non-adjacent value. Experiment 024 changed only the first Patch
event's displayed value from `PC 23` to `PC 100`; the Patch name remained
`Ming Dynasty`. No parsing or Studio Vision artifact modification was
performed.

# Controlled change

The human-observed event was `Ode to Clarke` / `Track 3 #2` / `JD-800`, first
List Window event at `1·2·50`, type Patch. The operator manually entered 100
in the PC field, saved, reopened the file in Studio Vision, verified it, and
copied it back to the host. No intentional change was made to position, name,
instrument, notes, note positions, durations, velocities, or other track data.

# Experiment lineage

Experiment 024 was made from a fresh duplicate of Experiment 007, not from
Experiment 023. All primary alignment and difference results therefore compare
Experiment 024 directly with Experiment 007. Experiment 023 is used only as
independent prior evidence in the later three-state comparison.

# Preregistered prediction

Experiment 023 established that displayed `PC 23 -> PC 24` produced the aligned
change `0x17 -> 0x18`. Before Experiment 024 was inspected, the locked
prediction was: if the byte directly stores the displayed Program Change
value, then `PC 23 -> PC 100` must produce `0x17 -> 0x64`, because decimal 23
is `0x17` and decimal 100 is `0x64`. No prediction was registered for the
nearby unexplained three-byte field.

# Baseline and experiment identity

| Artifact | Exact path | Size | SHA-256 | Finder Type / Creator | Inode |
|---|---|---:|---|---|---:|
| Experiment 007 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline` | 211,468 | `e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132` | `MID2` / `MIDA` | 17242646 |
| Experiment 024 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 024 - Track 3-2 Program Change 100/newest STUFF baseline EXP24` | 211,468 | `6979037d31fc163f106ca201b4f15aeab76fae34dea3f623b2434e2033df4fed` | `MID2` / `MIDA` | 17359509 |

Both uncompressed data forks expose FinderInfo beginning `MID2MIDA`. The
Experiment 007 directory also contains an 89,525-byte StuffIt archive; its
type and content distinguish it from the uncompressed project. The two project
files have different digests and inodes and are distinct.

# Structural alignment

Track 3 #2 did not relocate or change size. Independent anchors occur at the
same offsets in both projects:

- unique literal `Ming Dynasty`: `0x31891`;
- unique first-note properties `55 64 7f 83 4d`: `0x318b5`;
- unique complete 585-byte 84-note chain: `0x318b5–0x31afe`;
- established 17-note sample: `0x31994` (one of two known musical hits, selected
  here by the unique name, first-note properties, and complete chain);
- stable post-chain context: `0x31afe`.

The broader marker/context, name, first-note boundary, complete note stream,
and post-context align unambiguously. Both files are 211,468 bytes. A
whole-file same-position comparison finds 1,767 changed bytes in 641 runs;
these global save differences are not assigned to the Patch edit.

# Note-chain stability

The complete established stream at `0x318b5–0x31afe` is byte-for-byte
identical. This verifies all 84 pitches, attack velocities, release velocities,
and durations, plus all 83 note-to-note timing fields. The preceding `90`
status/type-like byte at `0x318b4` and the post-chain context are also stable.

# Local binary differences

Within the same aligned window used for Experiment 023,
`0x31870–0x31b10`, exactly four bytes change in two runs:

| Relative to `0x31882` | Baseline offset | Experiment 024 offset | Baseline | Experiment 024 | Context / assessment |
|---:|---:|---:|---:|---:|---|
| `+0x01` | `0x31883` | `0x31883` | `c4` | `c7` | first byte of unexplained three-byte header/save field |
| `+0x02` | `0x31884` | `0x31884` | `b2` | `85` | second byte of unexplained three-byte header/save field |
| `+0x03` | `0x31885` | `0x31885` | `5c` | `1c` | third byte of unexplained three-byte header/save field |
| `+0x23` | `0x318a5` | `0x318a5` | `17` | `64` | fourth byte of stable `ff ff ff NN`; Program Change value |

The external aligned-diff CSV preserves exact contexts and three-state values.
No other byte changes in this bounded Patch/name/note neighborhood.

Outside that strict window, in the unresolved post-chain area before Track 7's
marker at `0x31c04`, six repeated pairs change at `0x31b39/0x31b3f`,
`0x31b5c/0x31b62`, `0x31b7f/0x31b85`, `0x31b9a/0x31ba0`,
`0x31bbd/0x31bc3`, and `0x31be8/0x31bee`. In each pair a save-dependent
`c5 -> c7` change is followed six bytes later by `17 -> 18` in Experiment 023
or `17 -> 64` in Experiment 024. The repeated PC-valued bytes are dependent
copies or references at high confidence, but their structure and ownership are
unknown. They do not alter the identification of `0x318a5` as the controlled
event field and are not used to extend the Patch-event boundary.

# Program Change field

The preregistered prediction is **CONFIRMED**:

`0x318a5: 0x17 -> 0x64`.

The byte remains the fourth byte of `ff ff ff NN`. The non-adjacent displayed
change maps exactly to the non-adjacent stored value while the name, position
candidates, note boundary, and complete note performance remain stable.

# Three-state comparison

All three files align at the same Program Change offset; no relocation
adjustment is needed:

| State | Displayed PC | Offset | Stored byte |
|---|---:|---:|---:|
| Experiment 007 baseline | 23 | `0x318a5` | `0x17` (23) |
| Experiment 023 | 24 | `0x318a5` | `0x18` (24) |
| Experiment 024 | 100 | `0x318a5` | `0x64` (100) |

Two independent controlled saves from the baseline have therefore produced
`PC 23 -> PC 24: 0x17 -> 0x18` and
`PC 23 -> PC 100: 0x17 -> 0x64` at the same aligned field. Across all three
states, the stored project value directly equals the Vision-displayed PC
number for this Patch event.

# Unexplained three-byte field

The corresponding aligned states at `0x31883–0x31885` are:

| State | Displayed PC | Bytes | 7-bit VLQ mechanical value |
|---|---:|---|---:|
| Experiment 007 | 23 | `c4 b2 5c` | 1,120,604 |
| Experiment 023 | 24 | `c7 85 24` | 1,163,940 |
| Experiment 024 | 100 | `c7 85 1c` | 1,163,932 |

The values do not equal PC, have no fixed delta, and do not change
monotonically with PC. The Experiment 023 to 024 field decreases by 8 while
PC increases by 76. It is not a length consequence because the files and
aligned region retain their sizes and offsets.

The same baseline `c4 b2` marker-family prefix changes at 34 aligned locations
throughout these saves, commonly to `c7 68`, `c7 69`, `c7 6b`, or other
values, and many corresponding third bytes differ by 8 between Experiments
023 and 024 without local payload relocation. This is strong evidence that the
field participates in broader save-dependent serialization or reference state,
not that it directly encodes PC. No small local checksum relation is supported:
the controlled PC changes are 1 and 77, whereas the mechanical field changes
are 43,336 and 43,328 from baseline. Exact semantics remain unexplained.

# Patch-name stability

The unique literal `Ming Dynasty` at `0x31891–0x3189c` and its immediate text
context are byte-for-byte unchanged. Together with both controlled Program
Change edits, this establishes that the Patch-name text and Program Change
field are independently editable/stored properties for this event instance.

# Program-number semantics

The project-file experiments independently establish direct equality between
Vision's displayed values and the project byte for displayed PC 23, 24, and
100 in this Patch event. They do not establish a universal UI convention for
other instruments, patchers, or Studio Vision versions.

Existing MIDI-export evidence is only a consistency check: the baseline export
contains Program Change data byte decimal 23 for this track at absolute time
530, matching displayed `PC 23` and project byte `0x17`. It supplies no
evidence of a one-based translation in this artifact. The project-field
identification does not depend on the export.

# Updated Patch-event model

- **A. Program Change value field identified: YES.** Two independent,
  preregistered controlled changes map at `0x318a5` exactly.
- **B. Stored project value directly equals displayed PC value: YES.** The
  three aligned states are 23/`17`, 24/`18`, and 100/`64`.
- **C. Patch-name text independent from Program Change field: YES.** The unique
  literal remains unchanged across both Program Change experiments.
- **D. Patch event fully bounded: PARTIAL.** Its name, program field, and exact
  following note boundary are local, but its start and complete field ownership
  are not established.
- **E. Event-type discriminator established: PARTIAL.** `90` identifies the
  following note stream and `ff ff ff NN` is Patch-specific evidence, but an
  exact Patch event-type field is not isolated.
- **F. Patch-event timing representation established: NO.** Position remained
  unchanged and no timing field has been isolated by control.
- **G. Unexplained three-byte field understood: NO.** Save-wide comparison
  disfavors direct PC coupling but does not establish its semantics.

# Evidence supported

- Experiment 024 is a distinct, same-size `MID2`/`MIDA` project descended
  directly from Experiment 007.
- Track 3 #2 remains at the same offsets and size.
- All 84 note rows and 83 note-to-note timing fields are unchanged.
- `Ming Dynasty` is unchanged.
- The locked `0x17 -> 0x64` prediction is confirmed at `0x318a5`.
- Three displayed PC states directly equal the three stored project bytes.
- The only other bounded local change is the unexplained three-byte field;
  file-wide evidence associates its family with save-level serialization.
- Six post-chain PC-valued dependent copies also track the edit outside the
  strict Patch/name/note window; their ownership remains unknown.

# Unknowns

The exact Patch-event start and end ownership, Patch event-type discriminator,
Patch timing representation, meanings of `ff ff ff`, semantics of the
three-byte marker/header field, and generality beyond this event, instrument,
patcher, and Studio Vision version remain unknown.

# Single recommended next step

Change only the first Patch event's position while keeping `Ming Dynasty`, PC,
instrument, and notes unchanged. Program value and name independence are now
strongly established; a position-only control offers the highest information
gain by isolating the unresolved Patch timing representation and helping bound
the event without implementing parsing.

Experiment 025 completed this recommendation. The absolute Patch position is
stored as a 7-bit VLQ at `0x31886–0x31887`, changing `84 12` = 530 to
`84 13` = 531 for the one-unit-later edit. The PC byte, name, and complete note
chain stayed unchanged. See `CONTROLLED_TRACK3_2_PATCH_POSITION_PLUS_ONE.md`.

Experiment 026 independently changed only the Patch name. The aligned payload
at `0x31891–0x3189c` became `Phoenix Test`, while the confirmed PC field stayed
`0x17`. This directly confirms name/PC independence. See
`CONTROLLED_TRACK3_2_PATCH_NAME_CHANGE.md`.
