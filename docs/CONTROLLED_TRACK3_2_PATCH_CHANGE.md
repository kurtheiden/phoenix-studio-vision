# Objective

Test the preregistered prediction that the local Track 3 #2 Patch candidate
byte directly stores the displayed Program Change value by comparing the
authentic Experiment 007 baseline with Experiment 023, where the human
operator changed only `PC 23` to `PC 24` in the first List Window event.

# Controlled change

The observed context was `Ode to Clarke` / `Track 3 #2` / `JD-800`, first
event at `1·2·50`, type Patch. `Ming Dynasty` remained unchanged. No
intentional note, timing, duration, velocity, instrument, or other track-data
change was made. Both source artifacts were inspected read only.

# Preregistered prediction

Before Experiment 023 was inspected, the baseline candidate at `0x318a5` was
recorded as `0x17` (decimal 23). If it directly represents the displayed PC
value, the aligned experiment byte should be `0x18` (decimal 24), with the
surrounding Patch structure stable except for ordinary save noise or dependent
serialization changes. The result was evaluated without changing this rule.

# Baseline and experiment identity

| Artifact | Path | Size | SHA-256 | Finder Type / Creator |
|---|---|---:|---|---|
| Experiment 007 baseline | `.../Experiment 007 - Untouched Baseline/newest STUFF baseline` | 211,468 | `e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132` | `MID2` / `MIDA` |
| Experiment 023 | `.../Experiment 023 - Track 3-2 Patch Change/newest STUFF baseline EXP23` | 211,468 | `7b88c3424d7a0a2b419ef500571f9c5f186239d9dd57409129a1e15cd041e45b` | `MID2` / `MIDA` |

The files have different SHA-256 digests and inodes and are therefore distinct.
Both expose FinderInfo/legacy FinderInfo containing `MID2MIDA`; neither was
identified from filename alone. Experiment 023 contains the established Track
3 #2 name, note-chain, and framing evidence described below.

# Structural alignment

No relocation or size change occurred. Independent anchors align at the same
absolute offsets in both files:

- literal `Ming Dynasty`: `0x31891`;
- first-note properties: `0x318b5`;
- complete 585-byte 84-note chain: `0x318b5–0x31afe`;
- prior 17-note sample: `0x31994`;
- post-chain context: `0x31afe`.

The baseline bytes `2c c4 b2 5c` at `0x31882` become `2c c7 85 24`; therefore
the formerly described full `2c c4 b2` marker is not invariant under this save.
The leading `2c`, following `84 12 ff 7c ...`, name, program neighborhood, note
stream, and post-context provide an unambiguous same-offset alignment.

Both files are 211,468 bytes. A whole-file same-position comparison finds
1,771 changed bytes in 641 runs, consistent with ordinary Studio Vision save
variation documented by prior experiments. Those global changes are not
assigned to the Patch edit.

# Note-chain stability

The entire established note stream is byte-for-byte identical from `0x318b5`
through `0x31afe`. This covers all 84 notes, including every pitch, attack,
release, duration, and all 83 note-to-note timing VLQs. The first-note `90`
status/type-like byte at `0x318b4` and post-chain bytes also remain stable.

# Local binary differences

Within the aligned window `0x31870–0x31b10`, exactly four bytes change in two
runs:

| Relative to `0x31882` | Baseline / experiment offset | Baseline | Experiment | Assessment |
|---:|---|---:|---:|---|
| `+0x01` | `0x31883` / `0x31883` | `c4` | `c7` | unexplained header/dependent/save candidate |
| `+0x02` | `0x31884` / `0x31884` | `b2` | `85` | unexplained header/dependent/save candidate |
| `+0x03` | `0x31885` / `0x31885` | `5c` | `24` | unexplained header/dependent/save candidate |
| `+0x23` | `0x318a5` / `0x318a5` | `17` | `18` | direct Program Change field, high confidence |

The exact contexts are retained in the external aligned-diff CSV. The three
adjacent header bytes cannot be classified as checksum, index, length, or
ordinary save noise from this single comparison. They are not plausible direct
PC storage because they change as a three-byte group to values unrelated to
23/24 while the isolated candidate changes exactly as predicted.

# Program Change field

The preregistered prediction is **CONFIRMED**:

`0x318a5: 0x17 -> 0x18`.

The byte remains the fourth byte of the stable local sequence `ff ff ff NN`.
No relocation adjustment is required. The one-unit displayed change produces
the predicted one-unit stored change, while the name, position candidates,
note-type byte, and all note data remain stable. This identifies the direct
Program Change value field in this event instance.

# Patch-name stability

The complete literal `Ming Dynasty` at `0x31891–0x3189c` is byte-for-byte
unchanged. Its surrounding name/header bytes through `0x318a1` are also stable.
The controlled result supports the Patch-name text and Program Change value as
independently stored or independently editable properties in this event
representation. It does not prove how other patches or name tables behave.

# Event-position observations

The Patch remained at `1·2·50`. All bytes from immediately after the program
field (`0x318a6`) through the first-note status (`0x318b4`) are stable,
including previously noted candidates `c5 4c`, `a3 4a`, and `81 25`. Bytes
before the name at `0x31886–0x31890` are also stable. These are position/timing
stability observations only; the
experiment does not isolate which field, if any, encodes Patch position.

# MIDI export consistency

Existing independent research found one Track 3 #2 Program Change in the
Studio Vision MIDI export at absolute time 530 with data byte decimal 23. The
controlled project-byte change from decimal 23 to 24 is consistent with an
ordinary 7-bit MIDI Program Change data value. Because Studio Vision displayed
`PC 23` when both project and export used 23, this artifact supplies no evidence
for subtracting one from the displayed value. General UI numbering behavior is
not inferred beyond this observation.

# Patch-event structural model

- **A. Program Change value field: YES.** `0x318a5` changes uniquely as
  preregistered from `17` to `18`.
- **B. Patch-name independently stored/editable: YES.** Literal `Ming Dynasty`
  and its local text bytes remain unchanged while the program byte changes.
- **C. Patch event fully bounded: PARTIAL.** Its name and program field are
  local, and the first note boundary is exact, but Patch start and complete
  field ownership are not established.
- **D. Event-type discriminator: PARTIAL.** Stable `90` identifies the note
  stream; the Patch-specific `ff ff ff NN` relationship is supported, but an
  exact Patch type field is not isolated.
- **E. Patch timing representation: NO.** Candidate bytes remain stable as
  expected for an unchanged position, but no controlled position change or
  decoded timing field exists.

# Evidence supported

- Experiment 023 is a distinct, same-size `MID2` / `MIDA` project save.
- Track 3 #2 remains at the same offsets and aligns by multiple anchors.
- The complete 84-note stream and all note timing are byte-identical.
- Exactly four bytes change in the bounded Patch/note neighborhood.
- The predicted direct field changes `0x318a5: 17 -> 18`.
- `Ming Dynasty` remains byte-identical.
- Three nearby header bytes also change but have no assigned semantics.
- Patch position candidates remain stable; Patch timing remains unidentified.

# Unknowns

The exact Patch-event start, the meanings of `ff ff ff`, the three changed
header bytes, Patch timing/position field ownership, the changed marker-family
bytes, and generality to other Patch values, tracks, or Studio Vision versions
remain unknown.

# Single recommended next step

Experiment 024 completed this recommendation with displayed `PC 100` from a
fresh Experiment 007 duplicate. The aligned field changed
`0x318a5: 17 -> 64`, exactly as preregistered, while the name and complete note
chain remained unchanged. The nearby three-byte field became `c7 85 1c` and
remains unexplained; file-wide comparison favors save-dependent serialization
over direct PC coupling. See `CONTROLLED_TRACK3_2_PROGRAM_CHANGE_100.md`.

Change only the first Patch event's position while keeping name, PC,
instrument, and notes unchanged. This is now the smallest high-information
test of the unresolved Patch timing representation.
