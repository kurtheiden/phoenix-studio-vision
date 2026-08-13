# Objective

Test whether the opaque Track 2 Patch tail stores exported bank-select values
by changing only Bank Select LSB / CC32 from 1 to 2 in a fresh duplicate of
Experiment 007. No Studio Vision artifact or decoder implementation was
modified during the analysis.

# Controlled change

`Ode to Clarke` / `Track 2` / `JV-1080` retained Patch position `1·1·0`
(tick 0), name `Stereoww Bs`, CC0 81, PC 37, and all Note data. The sole
intentional change was CC32 `1 -> 2`.

# Experiment lineage

Experiment 028 was created directly from a fresh Experiment 007 duplicate. It
does not descend from another controlled experiment, so all comparisons use
Experiment 007 as the baseline.

# Preregistered prediction

Baseline Track 2 exported CC0=81 / CC32=1 and stored tail `ff 51 01`; authentic
Track 3 exported CC0=81 / CC32=2 and stored `ff 51 02`. The locked prediction
was therefore that changing only CC32 `1 -> 2` would produce
`ff 51 01 -> ff 51 02`, with `ff 51` stable. No prediction was made for
save-dependent bytes elsewhere.

# Artifact identity

| Artifact | Exact path | Size | SHA-256 | Finder type / creator |
|---|---|---:|---|---|
| Experiment 007 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline` | 211,468 | `e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132` | `MID2` / `MIDA` |
| Experiment 028 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 028 - Track 2 Bank Select LSB Change/newest STUFF baseline EXP28` | 211,468 | `013c0ed13cd965524bf24f9b7831890d955294d27e96c25210eb874f25f21a17` | `MID2` / `MIDA` |

The differing digests establish distinct files. The uncompressed projects,
not any archive, were used.

# Structural alignment

The length-prefixed `Stereoww Bs`, Patch position, PC, first Note properties,
and complete known Note chain align Track 2 at the same offsets in both files.
The Patch representation remains `0x2fb55..0x2fb75`, with Note status at
`0x2fb74`; neither the region nor the file changes size or relocates.

# Controlled-data stability

Both bounded representations decode as position 0, payload length 25, name
`Stereoww Bs`, PC 37, post-PC timing 1,920, empty pre-Note context, and Note
status `90` at `0x2fb74`. The binary Note chain `0x2fb75..0x300d8` is
byte-identical, proving stability of all 211 stored pitches, attacks, releases,
and durations and all 210 note-to-note timing fields. The known export caveat
remains: four SMF zero-velocity note-offs differ from project release values,
but baseline and Experiment 028 project bytes do not differ.

# Local Patch diff

Within the bounded Patch representation exactly one byte changes:

| Baseline offset | Experiment offset | Baseline | Experiment | Structural location | Assessment |
|---:|---:|---:|---:|---|---|
| `0x2fb70` | `0x2fb70` | `01` | `02` | final post-name byte; CC32 candidate | controlled CC32 value, high confidence |

The two preceding tail bytes at `0x2fb6e..0x2fb6f` remain `ff 51`; every other
bounded Patch byte is identical. Immediately before the Patch start, the
known save-dependent marker-family field at `0x2fb52..0x2fb54` changes
`c4 b2 8c -> c7 7e 60`. It is outside the decoder bounds, has varied in prior
saves, and remains unexplained save-level context rather than bank evidence.

# Bank-tail prediction result

**CONFIRMED.** At aligned offsets `0x2fb6e..0x2fb70`, the exact change is
`ff 51 01 -> ff 51 02`. The sentinel/candidate prefix `ff` and candidate CC0
byte `51` remain unchanged; only the candidate CC32 byte changes `01 -> 02`.

# Existing decoder result

The unchanged `decode_bounded_patch_representation` succeeds with explicit
bounds `0x2fb55..0x2fb75` and returns:

- position `00` = 0 at `0x2fb55`;
- payload length 25 at `0x2fb58`;
- pre-name context `00 01 25 f8 a5`;
- name `Stereoww Bs`;
- post-name context `02 33 38 04 ff 51 02` at `0x2fb6a..0x2fb71`;
- PC 37 at `0x2fb71`;
- post-PC `8f 00` = 1,920;
- empty pre-Note context and Note status `90` at `0x2fb74`.

No decoder contract or production code changed.

# Five-state bank evidence

| State | Independent/export bank evidence | Project tail |
|---|---|---|
| Track 1 | no bank select | `ff ff ff` |
| Track 2 baseline | CC0=81, CC32=1 | `ff 51 01` |
| Track 2 Experiment 028 | controlled CC32=2; CC0 remains 81 | `ff 51 02` |
| Track 3 | CC0=81, CC32=2 | `ff 51 02` |
| Track 3 #2 | no bank select | `ff ff ff` |

Experiment 028 converts the final-byte/CC32 relationship from correlation to
controlled evidence for this Track 2 event. It also strengthens the CC0
candidate because `51` stays stable while CC32 alone changes, but CC0 itself
has not yet been independently manipulated.

# Bank semantics assessment

- **A. Second-to-last byte directly encodes CC0=81: PARTIAL.** Multiple
  authentic states agree and it remains stable in the CC32-only save, but no
  controlled CC0 change exists.
- **B. Final byte directly encodes CC32: YES** for this representation. The
  locked single-variable prediction is uniquely confirmed.
- **C. Meaning of leading `ff`: NO.** It is stable but has not been isolated;
  sentinel, absence, or other semantics remain possible.
- **D. `ff ff ff` means no bank select: PARTIAL.** Two authentic no-bank
  exports correlate, without a controlled removal experiment.
- **E. Evidence supports optional diagnostic bank candidates: YES.** A
  diagnostic may label the last two bytes as evidence-backed candidates while
  retaining raw provenance and explicit uncertainty for CC0.
- **F. Evidence supports first-class shared-decoder bank semantics: PARTIAL.**
  CC32 is controlled, but CC0 and `ff` absence/sentinel behavior are not.

# Decoder-policy decision

Keep the shared decoder unchanged and preserve the complete post-name context
as opaque bytes. Document the confirmed CC32 subfield and the still-partial
CC0/absence interpretation. Optional diagnostic interpretation can be designed
later without changing structural parsing; promotion to first-class semantic
fields is premature.

# Evidence supported

- Track 2 remains structurally aligned and unchanged in size.
- The only bounded Patch change is `0x2fb70: 01 -> 02`.
- The locked `ff 51 01 -> ff 51 02` prediction is confirmed.
- Direct CC32 storage is established for this Track 2 representation.
- Position, name, PC, payload framing, timing, status transition, and the full
  Note performance remain stable.
- The existing bounded decoder accepts Experiment 028 unchanged and preserves
  the new opaque context byte-exactly.

# Unknowns

The exact semantics of `ff`, controlled behavior of CC0, representation of
absent bank select, broader generality, neighboring save-dependent fields, and
whether other patchers/devices share these bank subfields remain unresolved.

# Single recommended next step

Run one controlled Track 2 CC0/MSB-only change from a fresh Experiment 007
duplicate, keeping CC32=1, PC, position, name, and notes unchanged. This is the
smallest high-information test of the still-partial `51` byte and would decide
whether both bank bytes are ready for diagnostic semantic exposure.
