# Objective

This report compares the authentic Studio Vision project `newest STUFF` used by
the First MIDI Recovery Spike with Controlled Save Experiment 005. The
intentional user-visible edit was the same-length rename `Track 7` to `Track A`
inside `Ode to Clarke`. This is an evidence-gathering comparison only. It does
not implement parser logic, identify MIDI data, or infer field meaning from one
controlled save.

# Files and provenance

Offsets below are zero-based data-fork offsets. SHA-256 covers the data fork.
Both source artifacts were read only.

| Evidence label | Exact path | Data-fork size | SHA-256 |
|---|---|---:|---|
| original authentic project | `/Users/kurtheiden/Documents/Phoenix Research/Opcode/MY MUSIC/newest STUFF` | 203,422 (`0x31a9e`) | `7f97918efd8c8756058b72b4cada4a39a1f0f94655aa2982d4e03cfa8a876114` |
| Experiment 005 save | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 005 - Same Length Track Name Change/newest STUFF Track A` | 211,468 (`0x33a0c`) | `746c7757983dc7fcfbbd9b84bffb21a08e4f17c49a3c19c6730adcd9c7c1c455` |

The original path and hash exactly match the artifact named in
`FIRST_MIDI_RECOVERY_SPIKE.md`, `TRACK_RECORD_STRUCTURE_SURVEY.md`, and
`TRACK_RECORD_REFERENCE_SURVEY.md`. The repository file
`samples/newest STUFF` is a different 171,953-byte artifact with SHA-256
`c44d415a4b69d56abd5680652ed99039a4f9ca9afd281898601ccc14026aebec`;
it was not substituted for the recorded original.

Finder metadata and extended attributes were observed as follows:

| Observation | Original | Experiment 005 |
|---|---|---|
| Finder Type | `MIDS` (`4d 49 44 53`) | `MID2` (`4d 49 44 32`) |
| Finder Creator | `MIDA` (`4d 49 44 41`) | `MIDA` (`4d 49 44 41`) |
| `com.apple.FinderInfo` | present, 32 bytes | present, 32 bytes |
| other extended attributes | `com.apple.ResourceFork` | `org.BasiliskII.FinderInfo` (32 bytes), `org.BasiliskII.ExtendedFinderInfo` (zero bytes) |
| resource fork | present, 286 bytes | absent |

The original FinderInfo begins `4d 49 44 53 4d 49 44 41`; the controlled
FinderInfo and BasiliskII FinderInfo begin `4d 49 44 32 4d 49 44 41`.
Finder metadata and resource-fork differences are provenance observations, not
data-fork differences.

# Whole-file comparison

The data forks are not equal. The controlled file is 8,046 bytes larger.

Two exact comparison conventions are useful because the save substantially
rewrote the serialization:

- At identical absolute offsets through the shorter file, 190,445 of 203,422
  byte positions differ. The remaining 12,977 same-position bytes are equal.
  Counting the controlled file's 8,046-byte excess separately gives 198,491
  bytes that are either unequal at the same position or have no same-position
  counterpart. The unequal same-position bytes form 7,318 disjoint runs.
- The longest common prefix is one byte (`0x00000000`). The longest common
  suffix is four bytes. Thus the exact maximal changed middle spans are
  original `0x00000001–0x00031a99` (203,417 bytes) and controlled
  `0x00000001–0x00033a07` (211,463 bytes). This is the only unambiguous single
  replacement range obtainable from common-prefix/common-suffix anchoring.

The size difference proves net growth of 8,046 bytes. It does not establish a
unique insertion/deletion history: common byte values recur extensively, and
no format-aware boundaries establish which portions were inserted, deleted,
moved, or reserialized. Accordingly, this report does not claim a count of
insertions or deletions beyond the observed net size change.

The previously surveyed `Ode to Clarke` label sequence does not retain its
120-byte cadence. In the original, the 13 slice boundaries run from
`0x00026860` through `0x00026e77`, with labels at relative `0x0f` and boundaries
120 (`0x78`) bytes apart. In the controlled file, the corresponding sequence
from `Meter Track` through `Track A` has label offsets `0x0002f04e`,
`0x0002f0f4`, `0x0002f19a`, `0x0002f240`, `0x0002f2e6`, `0x0002f38c`,
`0x0002f432`, `0x0002f4d8`, `0x0002f57e`, `0x0002f624`, and `0x0002f6ca`.
Every successive difference is 166 (`0xa6`) bytes. `Track 12` and `Track 8`
were not observed in that controlled sequence. The controlled save therefore
weakens the hypothesis that 120 bytes is a serialization-invariant record
length.

# Track-record comparison

The previously surveyed original slice containing `Track 7` is exactly
`0x00026d10–0x00026d87`; its label begins at relative `0x0f`, absolute
`0x00026d1f`. The controlled `Track A` label is uniquely observed at
`0x0002f6ca`. Using the same relative-label anchor gives the 120-byte comparison
slice `0x0002f6bb–0x0002f732`.

The controlled slice is called a *comparison slice*, not a proven complete
record: the surrounding controlled labels use a 166-byte cadence.

| Slice | SHA-256 |
|---|---|
| original `0x00026d10–0x00026d87` | `0f8d66f440045aeb3b6b85ff3ecb31b024aad7cba13b6de65ae11ca53d90ea4d` |
| controlled `0x0002f6bb–0x0002f732` | `6052cf2db836b18a6ebb52919c11864e1411c64f80e6342e24f8c15bc26893df` |

Thirty-nine byte positions differ in the anchored 120-byte comparison. Exact
relative ranges and before/after bytes are:

| Relative range | Length | Before | After |
|---|---:|---|---|
| `0x15` | 1 | `37` (`7`) | `41` (`A`) |
| `0x20` | 1 | `51` | `48` |
| `0x2c` | 1 | `6d` | `6c` |
| `0x2e–0x30` | 3 | `01 64 2c` | `00 5d cd` |
| `0x34` | 1 | `0a` | `25` |
| `0x36` | 1 | `00` | `ff` |
| `0x4d–0x50` | 4 | `04 5c 50 38` | `2c db c7 6c` |
| `0x59–0x5e` | 6 | `00 41 00 01 e7 80` | `2c db c7 ac 00 00` |
| `0x60` | 1 | `80` | `00` |
| `0x62` | 1 | `04` | `01` |
| `0x64–0x66` | 3 | `00 04 01` | `01 00 00` |
| `0x68–0x77` | 16 | `24 ff ff ff ff ff ff ff ff ff ff ff ff ff ff ff` | `78 00 00 00 78 00 00 00 78 08 00 01 00 01 00 00` |

Only relative `0x15` is an expected printable label-byte change. The other 38
changed positions are outside the printable label. Therefore the controlled
save did not preserve all non-label bytes within this anchored slice.

# Candidate-window comparison

All numeric interpretations below are mechanical unsigned big-endian readings.
They are not described as offsets or pointers.

| Relative window | Before bytes | After bytes | Before value | After value | Changed |
|---|---|---|---:|---:|---|
| `0x25–0x28` | `00 02 02 f7` | `00 02 02 f7` | `0x202f7` (131,831) | `0x202f7` (131,831) | no |
| `0x5b–0x5e` | `00 01 e7 80` | `c7 ac 00 00` | `0x1e780` (124,800) | `0xc7ac0000` (3,349,938,176) | yes |
| `0x4b–0x4e` | `00 00 04 5c` | `00 00 2c db` | `0x45c` (1,116) | `0x2cdb` (11,483) | yes |
| `0x31–0x34` | `00 05 00 0a` | `00 05 00 25` | `0x5000a` (327,690) | `0x50025` (327,717) | yes |

For each changed value, the same 32-byte destination-window convention from
`TRACK_RECORD_REFERENCE_SURVEY.md` was applied: 16 bytes before the numeric
location and 16 bytes beginning there, clipped to the file.

- Relative `0x5b–0x5e`: the before value selects original
  `0x0001e770–0x0001e78f`, bytes
  `70 3f 66 24 81 75 81 71 33 62 37 82 28 81 71 3a 5c 39 82 19 81 70 3f 6c 7d 72 81 70 2e 6c 33 81`.
  The window is byte-diverse (20 distinct bytes), has no zero or `ff` run, and
  has several short printable runs. The after value is outside the
  211,468-byte controlled file, so no after destination window exists.
- Relative `0x4b–0x4e`: the before value selects original
  `0x0000044c–0x0000046b`, bytes
  `00 00 00 01 15 00 00 0e 04 02 00 7f 10 ff 80 64 00 7f 10 00 00 00 24 00 00 00 00 00 00 00 00 00`.
  It is zero-rich (longest zero run 9), contains one `ff`, and has no printable
  run of four bytes. The after value selects controlled
  `0x00002ccb–0x00002cea`, bytes
  `07 00 00 00 00 80 00 00 07 00 00 00 00 80 00 00 07 00 00 00 00 80 00 00 07 00 00 00 00 80 00 00`.
  It is a repeated eight-byte structure, is zero-rich (longest run 4), has no
  `ff`, and has no printable run.
- Relative `0x31–0x34`: both values are outside their respective data forks;
  therefore neither a before nor after destination window exists.

The unchanged `0x25–0x28` value still selects different bytes after the whole
file rewrite. Original `0x000202e7–0x00020306` is
`6c 64 42 81 36 31 60 6c 30 00 27 54 68 19 00 25 56 6a 81 1b 78 33 75 66 42 3a 27 78 7e 41 81 36`;
controlled at the same numeric location is
`81 61 4b 4e 2f 79 05 3c 64 46 5f 7c 4a 60 29 61 76 35 55 4d 81 47 02 48 64 1f 86 44 81 63 41 5a`.
Both are byte-diverse and neither contains a long zero or `ff` run. This is an
observation about destination contents, not evidence that the value references
them.

# Changes outside the edited record

Outside-record changes are extensive, not localized. Common-prefix/suffix
anchoring places the complete non-common middle at original absolute
`0x00000001–0x00031a99` and controlled absolute
`0x00000001–0x00033a07`. The controlled span is 8,046 bytes longer. At equal
absolute positions, 190,375 unequal positions lie outside the original
`Track 7` slice; those positions form 7,300 runs after excluding that absolute
slice. These run counts describe same-position comparison only and are not
format-aware edit operations.

Direct observations within the broad outside-record span include:

- many nearby printable strings, including device-like names, sequence and
  track labels, `Ode to Clarke`, `Meter Track`, and `Tempo Track`;
- repeated fixed-looking byte structures, including the controlled 166-byte
  label cadence and repeated zero-rich groups such as the window at
  `0x00002ccb–0x00002cea`;
- zero-rich and `ff`-rich areas, including the padding adjacent to track labels;
- large byte-diverse regions with short incidental printable runs.

The original weak candidate region around `0x0002649f–0x000265e3`, the
original 120-byte label sequence, and the earlier candidate destination regions
all fall inside the broad rewritten span. Some byte-diverse outside-record
areas could plausibly be event-bearing because byte diversity does not exclude
that possibility. No exact SMF payload, event framing, controlled note change,
or other evidence in this experiment identifies any such area as MIDI data.

# Evidence supported

- The exact original artifact and its prior provenance measurements are
  reproduced.
- `Track 7` remains at relative `0x0f` in the previously documented original
  120-byte slice, and `Track A` is at relative `0x0f` in the controlled anchored
  comparison slice.
- The expected single printable byte changed from `37` to `41`.
- The strongest-ranked `0x25–0x28` big-endian candidate is stable within the
  anchored slice in this experiment.
- The experiment supports the prior reports' caution: numeric coincidences and
  destination-window appearance alone do not establish references or fields.
- The first spike's negative direct-SMF result is neither contradicted nor
  converted into a positive MIDI identification by this rename-only save.

# Evidence weakened or eliminated

- A file-size-preserving save hypothesis is eliminated: the controlled data
  fork is 8,046 bytes larger despite the same-length label edit.
- A hypothesis that the original 120-byte cadence is serialization-invariant is
  weakened: the corresponding controlled label sequence has a 166-byte cadence.
- A hypothesis that only the printable label byte changes within the
  corresponding anchored slice is eliminated for this save: 38 non-label byte
  positions also changed.
- The ranked `0x5b–0x5e`, `0x4b–0x4e`, and `0x31–0x34` numeric candidates are
  weakened as stable record-associated values because all three changed during
  a rename-only save, and two resulting values are outside the controlled file.
- The `0x25–0x28` candidate's unchanged source bytes do not preserve its
  destination contents because the file was rewritten. This weakens any
  inference based only on the earlier destination-window coincidence.
- The structure survey's 120-byte slices remain valid observations about the
  original artifact, but this experiment eliminates treating that width as a
  cross-save invariant on present evidence.

# Unknowns

- It is unknown which differences are consequences of the intentional rename
  and which are consequences of Studio Vision's save process, environment,
  format version, Finder Type change, or other state.
- It is unknown whether the controlled 166-byte cadence denotes records, and
  whether the anchored 120-byte controlled slice is a complete logical unit.
- It is unknown why `Track 12` and `Track 8` are absent from the controlled
  `Ode to Clarke` label sequence.
- A unique insertion/deletion/move history cannot be derived from these two
  opaque byte strings without format-aware anchors.
- No meaning, width, signedness, byte order beyond the requested mechanical
  reading, or relationship is established for any candidate window.
- No outside-record changed region is established as event-bearing or MIDI
  data.
- This experiment does not provide enough evidence to recover a valid Standard
  MIDI File.

# Single recommended next step

Repeat the same-length `Track 7` to `Track A` rename from a freshly verified
copy of the exact original artifact, while preserving the original Finder Type
and the same emulator/application save environment, then compare the two
independent controlled saves against each other before comparing either with
the original. Reproducible controlled-save changes would separate deterministic
serialization effects from run-specific rewrite noise without assigning field
meaning.
