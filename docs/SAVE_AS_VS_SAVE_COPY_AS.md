# Objective

This report determines whether Studio Vision's `Save As...` and
`Save Copy As...` operations produced materially different project files after
the same intentional edit, `Track 7` to `Track A` inside `Ode to Clarke`. It is
a provenance and controlled-save comparison. It does not modify an artifact,
implement parser logic, identify binary-field meaning, or test either file in
Studio Vision.

# Artifacts and provenance

Offsets are zero-based data-fork offsets. SHA-256 values in the artifact table
cover only the data fork.

The supplied Experiment 006 directory exists, but the supplied basename
`newest STUFF Track A` does not. The directory contains one project artifact,
whose exact on-disk basename is `newest STUFF A`. That path discrepancy is
retained as provenance evidence; the file was not renamed.

| Artifact | Exact path | Data-fork size | Data-fork SHA-256 |
|---|---|---:|---|
| Original | `/Users/kurtheiden/Documents/Phoenix Research/Opcode/MY MUSIC/newest STUFF` | 203,422 (`0x31a9e`) | `7f97918efd8c8756058b72b4cada4a39a1f0f94655aa2982d4e03cfa8a876114` |
| Experiment 005 — `Save As...` | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 005 - Same Length Track Name Change/newest STUFF Track A` | 211,468 (`0x33a0c`) | `746c7757983dc7fcfbbd9b84bffb21a08e4f17c49a3c19c6730adcd9c7c1c455` |
| Experiment 006 — `Save Copy As...` | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 006 - Save Copy As/newest STUFF A` | 211,468 (`0x33a0c`) | `3386c85a9657d11647fe4a20103ff3a06df748c4025748c2e21e62aedddc4650` |

The Original size and hash independently reproduce the provenance recorded by
`FIRST_MIDI_RECOVERY_SPIKE.md`, `TRACK_RECORD_STRUCTURE_SURVEY.md`,
`TRACK_RECORD_REFERENCE_SURVEY.md`, and
`CONTROLLED_SAVE_EXPERIMENT_005.md`. The Experiment 005 size and hash also
independently reproduce its prior report.

Complete fork and extended-attribute measurements are:

| Observation | Original | Experiment 005 | Experiment 006 |
|---|---|---|---|
| Finder Type | `MIDS` (`4d 49 44 53`) | `MID2` (`4d 49 44 32`) | `MID2` (`4d 49 44 32`) |
| Finder Creator | `MIDA` (`4d 49 44 41`) | `MIDA` (`4d 49 44 41`) | `MIDA` (`4d 49 44 41`) |
| `com.apple.FinderInfo` | present, 32 bytes | present, 32 bytes | present, 32 bytes |
| `org.BasiliskII.FinderInfo` | absent | present, 16 bytes | present, 16 bytes |
| `org.BasiliskII.ExtendedFinderInfo` | absent | present, 16 bytes | present, 16 bytes |
| resource fork | present | absent | absent |
| resource-fork size | 286 bytes | — | — |
| resource-fork SHA-256 | `4469ca9b058366d644af55f13c5c52f68313dad72b5491d071ed3743bfae9432` | — | — |
| other extended attributes | none observed | none observed | none observed |

The resource fork is exposed as `com.apple.ResourceFork` on the Original; it is
not an additional independent attribute. Both controlled files have the same
three extended-attribute names, sizes, and bytes. Their decoded attribute
SHA-256 values are `9699f46bd0efeeb423aebc59b541971b33ff4dac3fb7148e1c948bb1c097045a`
for `com.apple.FinderInfo`,
`a0707b1d631e22b51d0c9e28fbc3b4269743a447897b45e8879d14ecd3796758`
for `org.BasiliskII.FinderInfo`, and
`374708fff7719dd5979ec875d56cd2286f6d3cf7ec317a3b25632aab28ec37bb`
for `org.BasiliskII.ExtendedFinderInfo`.

# Original vs Save As

Independent measurement confirms the findings relevant to this comparison in
`CONTROLLED_SAVE_EXPERIMENT_005.md`:

- Experiment 005 is 8,046 bytes larger than the Original.
- The data forks have a one-byte common prefix and four-byte common suffix. At
  identical absolute positions through the Original length, 190,445 bytes
  differ in 7,318 disjoint runs. This is broad reserialization behavior, not a
  localized label-byte change.
- `Ode to Clarke` moves from `0x0002680d` to `0x0002f753`.
- The corresponding Original label sequence is `Meter Track` at
  `0x0002686f`, `Tempo Track` at `0x000268e7`, `Track 1` at `0x0002695f`,
  `Track 2` at `0x000269d7`, `sys100loops` at `0x00026a4f`, `Track 4` at
  `0x00026ac7`, `Track 5` at `0x00026b3f`, `Track 3` at `0x00026bb7`,
  `Track 6` at `0x00026c2f`, `Track 3 #2` at `0x00026ca7`, and `Track 7` at
  `0x00026d1f`. Consecutive label offsets differ by 120 (`0x78`) bytes.
- The Experiment 005 counterparts are at `0x0002f04e`, `0x0002f0f4`,
  `0x0002f19a`, `0x0002f240`, `0x0002f2e6`, `0x0002f38c`, `0x0002f432`,
  `0x0002f4d8`, `0x0002f57e`, `0x0002f624`, and `Track A` at
  `0x0002f6ca`. Consecutive offsets differ by 166 (`0xa6`) bytes.
- Finder Type changes from `MIDS` to `MID2`; Finder Creator remains `MIDA`.
- The 286-byte Original resource fork is absent. The two BasiliskII Finder
  attributes are present on Experiment 005 and absent on the Original.

These are direct comparisons. They do not identify why the save changed the
serialization or metadata.

# Original vs Save Copy As

Experiment 006 exhibits the same broad comparison measurements, with a distinct
data-fork hash:

- Experiment 006 is 8,046 bytes larger than the Original.
- The data forks have a one-byte common prefix and four-byte common suffix. At
  identical absolute positions through the Original length, 190,435 bytes
  differ in 7,316 disjoint runs. The behavior is broad reserialization rather
  than localization around the edited label.
- `Ode to Clarke` occurs once, at `0x0002f753`, identical to Experiment 005's
  location.
- The corresponding label sequence occurs at `0x0002f04e`, `0x0002f0f4`,
  `0x0002f19a`, `0x0002f240`, `0x0002f2e6`, `0x0002f38c`, `0x0002f432`,
  `0x0002f4d8`, `0x0002f57e`, `0x0002f624`, and `0x0002f6ca` in the same order
  listed under Original vs Save As. `Track A` occurs once, at `0x0002f6ca`.
  Each consecutive label difference is 166 (`0xa6`) bytes.
- Finder Type is `MID2`; Finder Creator is `MIDA`.
- No resource fork is present. `com.apple.FinderInfo` and both BasiliskII Finder
  attributes are present with the sizes recorded above.

Thus Experiment 006 independently reproduces Experiment 005's size change,
relevant string locations, label cadence, Finder Type/Creator, resource-fork
absence, and extended-attribute layout. Its data-fork bytes are not identical
to Experiment 005.

# Save As vs Save Copy As

| Test | Direct result |
|---|---|
| data forks byte-identical | no |
| data-fork sizes identical | yes; both 211,468 bytes |
| data-fork hashes identical | no; `746c7757…c1c455` versus `3386c85a…c4650` |
| relevant track-label locations identical | yes |
| observed label cadence identical | yes; 166 (`0xa6`) bytes |
| `Ode to Clarke` location identical | yes; `0x0002f753` |
| `Track A` location identical | yes; `0x0002f6ca` |
| Finder Type/Creator identical | yes; `MID2`/`MIDA` |
| resource forks identical | yes in observed state; both absent |
| extended attributes identical | yes; names, sizes, and bytes are identical |

At identical absolute positions, the controlled data forks differ at 1,798 of
211,468 bytes (approximately 0.85%), grouped into 982 disjoint runs. Their
first difference is at `0x0000001e`, their last at `0x000339ed`, and every
unequal run is one to three bytes long. Differences occur in every `0x4000`-byte
bin spanning the file; they are distributed rather than confined to the edited
track-label area.

The corresponding label-sequence span `0x0002f03f–0x0002f760` differs at 36
positions, but all relevant printable labels retain identical bytes and
offsets. The 120-byte slice anchored 15 bytes before `Track A`,
`0x0002f6bb–0x0002f732`, differs at six positions. Its SHA-256 is
`6052cf2db836b18a6ebb52919c11864e1411c64f80e6342e24f8c15bc26893df`
in Experiment 005 and
`2357fb4fdabcc5d61e4968092d453eb79ede31351661295214c9c3779b27b2fc`
in Experiment 006.

These measurements establish distinct serializations at the byte level. They
do not show substantially different serialization layouts: size, relevant
structure locations, cadence, metadata, and more than 99% of same-position
data-fork bytes agree. No semantic meaning is assigned to the dispersed
one-to-three-byte differences.

# Finder metadata and resource-fork comparison

The Original differs from both controlled files in the same observed ways:

- Original Finder Type is `MIDS`; both controlled types are `MID2`.
- Finder Creator is `MIDA` on all three artifacts.
- Original `com.apple.FinderInfo` is 32 bytes and differs from the identical
  32-byte controlled values.
- Original has a 286-byte resource fork with SHA-256
  `4469ca9b058366d644af55f13c5c52f68313dad72b5491d071ed3743bfae9432`;
  neither controlled file has a resource fork.
- Both controlled files have identical 16-byte `org.BasiliskII.FinderInfo` and
  16-byte `org.BasiliskII.ExtendedFinderInfo` attributes; the Original has
  neither.

No Finder metadata, resource-fork, or extended-attribute difference was
observed between Experiment 005 and Experiment 006.

# Controlled-experiment implications

Changes common to both save methods are directly supported: identical
8,046-byte data-fork growth, broad reserialization relative to the Original,
the same moved `Ode to Clarke` and track-label sequence, the same 166-byte
cadence, the expected `Track A`, Finder Type `MID2`, unchanged Creator `MIDA`,
resource-fork absence, and identical Finder-related attributes.

Changes unique to `Save As...` are limited by this evidence to its particular
1,798-byte pattern relative to Experiment 006 and its distinct SHA-256. Changes
unique to `Save Copy As...` are likewise limited to the complementary byte
values and distinct hash. Because there is one artifact from each method, the
comparison does not establish that these dispersed byte differences are caused
by the method rather than by separate save runs or other uncontrolled state.

Copying between drives can preserve, remove, or translate forks and extended
attributes depending on the filesystem and copy path. The observed controlled
files have identical Finder-related attributes and both lack resource forks,
but no pre-copy measurements are available. Therefore copying remains a
possible source of metadata state; it is not evidenced as the cause of a
specific difference here.

The cause of the 1,798 differing data-fork bytes remains unknown. Their
distribution shows that they are not only the intentional printable label
change, but distribution alone does not distinguish save method, run-specific
state, copying, or another cause.

# Behavioral observations

- The user reports that Experiment 005 (`Save As...`) crashes SheepShaver /
  Studio Vision when reopened. This report does not infer why.
- Experiment 006 (`Save Copy As...`) has not yet been tested for reopenability.
  This report does not infer whether it will reopen successfully.

Neither file was opened in Studio Vision during this investigation.

# Evidence supported

- Both save operations produced files with the same size, relevant structural
  locations, observed 166-byte label cadence, Finder Type/Creator, resource-fork
  state, and extended attributes.
- Both controlled files differ broadly from the exact Original in closely
  matching measured ways.
- The two controlled data forks are not byte-identical and have different
  SHA-256 values.
- Their differences are sparse, short, and distributed: 1,798 bytes in 982
  runs, with no run longer than three bytes.
- The evidence supports a shared broad serialization layout across the two save
  methods, while also establishing run-level byte differences.

# Evidence weakened or eliminated

- The hypothesis that `Save As...` and `Save Copy As...` necessarily produce
  byte-identical data forks for this edit is eliminated by their hashes and
  direct comparison.
- The hypothesis that the two methods produced substantially different broad
  layouts is weakened by their identical size, string positions, cadence, and
  metadata, plus greater than 99% same-position byte equality.
- A hypothesis that the Experiment 005 broad rewrite is unique to `Save As...`
  is weakened: Experiment 006 shows the same size growth, relocated strings,
  cadence, and nearly the same broad data-fork organization.
- The Original's 120-byte cadence is again shown not to persist in either
  controlled save.

# Unknowns

- It is unknown whether the 1,798 data-fork differences are method-dependent,
  run-dependent, or introduced by another uncontrolled condition.
- It is unknown whether any differing bytes encode time, identifiers, internal
  addresses, state, checksums, or something else; no field meaning is inferred.
- It is unknown at what stage the controlled Finder attributes were created or
  the Original resource fork was lost.
- It is unknown whether copying between drives altered any artifact because no
  before-and-after copy measurements are available.
- The reason for the Experiment 006 basename discrepancy is unknown.
- Experiment 006 reopenability is unknown.
- Experiment 005's reported crash cause is unknown.
- No observation in this comparison identifies event-bearing data or permits
  recovery of a valid Standard MIDI File.

# Single recommended next step

Test Experiment 006 for reopenability once in the same SheepShaver / Studio
Vision environment, without resaving it, then immediately record only whether
the open succeeds or crashes and remeasure the artifact afterward to confirm
that the test did not alter its data fork, resource-fork state, or extended
attributes. This directly resolves the remaining behavioral comparison without
assigning a cause to the byte differences.
