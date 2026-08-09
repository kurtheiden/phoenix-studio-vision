# Objective

This report compares the authentic Studio Vision project `newest STUFF`, an
untouched save first created on the native Mac OS 9 Desktop, and two edited
saves written directly by Studio Vision to SheepShaver's `Unix` shared volume.
It is an observational provenance and file-comparison report. It does not
modify a source artifact, implement parser logic, identify binary-field
meaning, or establish an SMF payload.

# Experimental history

The Original was known to function in Studio Vision. Experiment 005 applied
the same-length edit `Track 7` to `Track A` and used `Save As...` directly to
`Unix`; the user reports that reopening it crashes Studio Vision/SheepShaver.
Experiment 006 applied the same edit and used `Save Copy As...` directly to
`Unix`; the user reports that it opens a project window whose contents cannot
be opened and whose window cannot be resized, although Studio Vision can quit
normally.

The user later reports that another `Save As...` attempt to the native Mac OS 9
filesystem also produced a broken file. A separate procedure duplicated a
known-good file with Finder on the native filesystem, changed `Track 7` to
`Track A`, and saved the duplicate when quitting Studio Vision. That edited
file reopened and functioned normally. These are distinct operations and are
not treated as equivalent save paths.

Experiment 007 involved no intentional project edit. Studio Vision saved it to
the native Mac OS 9 Desktop, where that native file reopened and functioned
normally. The Mac OS 9 Finder then copied it to `Unix`, and the host-side file
was copied to the research directory. In a later test, the file on `Unix` was
Finder-copied back to the native Mac OS 9 Desktop without an intentional edit
or resave. The user reports that this returned file opened and functioned
normally in Studio Vision. Thus this specific project remained usable after
the tested native Desktop to `Unix` to native Desktop Finder-copy round trip.

# Artifacts and provenance

The supplied Experiment 007 directory name
`Experiment 007 - Untouched Baseline Save` was not present. The parent
directory contained one closely matching directory,
`Experiment 007 - Untouched Baseline`. That exact observed path is used below.
At the time of the original comparison it contained the uncompressed project;
it now also contains a StuffIt preservation archive inventoried separately in
`CLASSIC_MAC_PRESERVATION_SURVEY.md`. Experiments 005 and 006 each contained
one file. The Original was positively identified by its requested basename and
by reproducing the hash recorded in the earlier reports.

Offsets are zero-based data-fork offsets. SHA-256 values in this table cover
only the data fork.

| Artifact | Exact path | Basename | Data-fork size | Data-fork SHA-256 |
|---|---|---|---:|---|
| Original | `/Users/kurtheiden/Documents/Phoenix Research/Opcode/MY MUSIC/newest STUFF` | `newest STUFF` | 203,422 (`0x31a9e`) | `7f97918efd8c8756058b72b4cada4a39a1f0f94655aa2982d4e03cfa8a876114` |
| Experiment 005, direct `Save As...` | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 005 - Same Length Track Name Change/newest STUFF Track A` | `newest STUFF Track A` | 211,468 (`0x33a0c`) | `746c7757983dc7fcfbbd9b84bffb21a08e4f17c49a3c19c6730adcd9c7c1c455` |
| Experiment 006, direct `Save Copy As...` | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 006 - Save Copy As/newest STUFF A` | `newest STUFF A` | 211,468 (`0x33a0c`) | `3386c85a9657d11647fe4a20103ff3a06df748c4025748c2e21e62aedddc4650` |
| Experiment 007, native-save-derived baseline | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline` | `newest STUFF baseline` | 211,468 (`0x33a0c`) | `e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132` |

# Original vs native-save-derived baseline

Experiment 007 is 8,046 bytes larger than the Original. Its data fork is not a
localized no-op save: at equal absolute positions through the Original length,
190,446 of 203,422 positions differ, in 7,317 disjoint runs. The common prefix
is one byte and the common suffix is four bytes. Thus the maximal non-common
middle spans Original `0x00000001–0x00031a99` and Experiment 007
`0x00000001–0x00033a07`. These measurements show broad reserialization and net
growth; they do not determine an insertion, deletion, or movement history.

`Ode to Clarke` is present once in each artifact, moving from `0x0002680d` in
the Original to `0x0002f753` in Experiment 007. The relevant observed labels
are:

| Label | Original | Experiment 007 |
|---|---:|---:|
| `Meter Track` | `0x0002686f` | `0x0002f04e` |
| `Tempo Track` | `0x000268e7` | `0x0002f0f4` |
| `Track 1` | `0x0002695f` | `0x0002f19a` |
| `Track 2` | `0x000269d7` | `0x0002f240` |
| `sys100loops` | `0x00026a4f` | `0x0002f2e6` |
| `Track 4` | `0x00026ac7` | `0x0002f38c` |
| `Track 5` | `0x00026b3f` | `0x0002f432` |
| `Track 3` | `0x00026bb7` | `0x0002f4d8` |
| `Track 6` | `0x00026c2f` | `0x0002f57e` |
| `Track 3 #2` | `0x00026ca7` | `0x0002f624` |
| `Track 7` | `0x00026d1f` | `0x0002f6ca` |

Successive Original locations differ by 120 (`0x78`) bytes. Successive
Experiment 007 locations differ by 166 (`0xa6`) bytes. `Track 12` and
`Track 8`, which follow in the Original sequence, were not observed in the
corresponding Experiment 007 sequence. The large size change, relocated label
region, and 166-byte cadence previously seen in the direct saves are therefore
also present in the healthy native-save-derived artifact.

The Original has Finder Type `MIDS`, a 286-byte resource fork, and no
BasiliskII attributes. Experiment 007 has Finder Type `MID2`, no resource fork,
and two BasiliskII Finder attributes. Finder Creator is `MIDA` in both.

# Native-save-derived baseline vs direct Save As

Experiments 007 and 005 have equal 211,468-byte data forks but different
SHA-256 values. At equal offsets, 1,800 bytes differ in 987 disjoint runs. The
first difference is `0x0000001e`, the last is `0x000339ed`, and no unequal run
is longer than three bytes. They share a 30-byte prefix and 30-byte suffix.
This is sparse, distributed variation within the same broad layout, not the
large structural difference observed between either save and the Original.

Both files place `Ode to Clarke` at `0x0002f753`. The complete relevant label
sequence has identical locations and the same 166-byte cadence; only the
intentional label differs, with `Track 7` in Experiment 007 and `Track A` in
Experiment 005 at `0x0002f6ca`. Across the sequence span
`0x0002f03f–0x0002f760`, 37 byte positions differ. In the 120-byte comparison
slice anchored 15 bytes before that label (`0x0002f6bb–0x0002f732`), seven
positions differ, including the expected printable `7`/`A` byte at
`0x0002f6d0`.

Their Finder Type and Creator, extended-attribute names and sizes, and absence
of a resource fork match. The bytes of `com.apple.FinderInfo` and
`org.BasiliskII.FinderInfo` differ as recorded in the metadata section.

# Native-save-derived baseline vs direct Save Copy As

Experiments 007 and 006 also have equal-size data forks and different SHA-256
values. Their exact whole-file comparison is the same in aggregate as the 007
versus 005 comparison: 1,800 unequal positions in 987 runs, a 30-byte common
prefix and suffix, first difference `0x0000001e`, last difference
`0x000339ed`, and a maximum unequal-run length of three bytes.

`Ode to Clarke`, all relevant label locations, and the 166-byte cadence match.
Experiment 007 contains `Track 7` and Experiment 006 contains `Track A` at
`0x0002f6ca`. The same 37 positions differ across the label-sequence span and
the same seven positions differ in the anchored 120-byte slice. Metadata names,
sizes, Type/Creator, and resource-fork absence match; two Finder-attribute
values differ.

Across all three saved data forks, 209,665 positions are equal. At 1,792
positions all three byte values differ, consistent with substantial
run-specific variation but not proof of its origin. The two direct saves equal
one another while Experiment 007 differs at five positions; one is the
intentional `Track 7`/`Track A` byte. Experiment 007 equals only Experiment 005
at three positions and only Experiment 006 at three positions. The remaining
four positions shared only by the edited direct saves may correlate with the
edit or another shared condition; their meaning is unresolved.

# Finder metadata and fork comparison

| Observation | Original | Experiment 005 | Experiment 006 | Experiment 007 |
|---|---|---|---|---|
| Finder Type | `MIDS` | `MID2` | `MID2` | `MID2` |
| Finder Creator | `MIDA` | `MIDA` | `MIDA` | `MIDA` |
| `com.apple.FinderInfo` | present, 32 bytes | present, 32 bytes | present, 32 bytes | present, 32 bytes |
| `org.BasiliskII.FinderInfo` | absent | present, 16 bytes | present, 16 bytes | present, 16 bytes |
| `org.BasiliskII.ExtendedFinderInfo` | absent | present, 16 bytes | present, 16 bytes | present, 16 bytes |
| resource fork | present, 286 bytes | absent | absent | absent |
| other extended attributes | none observed | none observed | none observed | none observed |

The Type/Creator bytes begin each FinderInfo value. The complete leading bytes
were Original `4d4944534d4944410140009c00c00000`, Experiment 005 and 006
`4d4944324d4944410000000000000000`, and Experiment 007
`4d4944324d4944410100000000000000`. The BasiliskII FinderInfo bytes were
identical in Experiments 005 and 006
(`4d4944324d4944410000ffffffff0000`) but differed in Experiment 007
(`4d4944324d4944410100007b01080000`). All three
`org.BasiliskII.ExtendedFinderInfo` values were 16 zero bytes.

The Original resource-fork SHA-256 is
`4469ca9b058366d644af55f13c5c52f68313dad72b5491d071ed3743bfae9432`.
No resource fork was observed on any saved artifact. Consequently, the
host-side Experiment 007 copy does not preserve resource-fork information that
the direct-to-`Unix` saves lack. It does preserve differing FinderInfo flag and
BasiliskII FinderInfo values, but not additional attribute names or sizes.
Because there is no measurement of the native Desktop file before Finder
copying, the stage at which its resource fork or metadata representation
changed is unknown.

# Save-destination observations

Changes also present in the healthy native-save-derived Experiment 007 are the
8,046-byte growth, broad reserialization relative to the Original, relocation
of `Ode to Clarke` and its label sequence, change from 120- to 166-byte observed
label cadence, Finder Type `MID2`, Creator `MIDA`, absence of the Original's
resource fork, and presence of the two BasiliskII Finder attributes. These
observations show that the large serialization change is not unique to saving
directly to `Unix` and is compatible with a save that first functioned on the
native Mac OS 9 Desktop.

No broad data-layout feature measured here was found only in the direct-to-
`Unix` artifacts. Differences found in both direct artifacts but not
Experiment 007 comprise five individual data-fork positions and identical
values in two Finder attributes; they are observations, not an established
signature of the save destination.

The printable `Track 7`/`Track A` byte correlates directly with the intentional
edit. Four other data-fork positions share a value only between the two edited
direct saves and may correlate with that edit, another shared condition, or
chance. The 1,792 all-distinct positions may be run-specific, but one artifact
per run cannot establish that classification. The remaining sparse byte and
FinderInfo-value differences are unresolved.

The behavioral evidence is a strong correlation in a small sample: the native
Desktop save reopened successfully, the same artifact remained usable after
the tested Finder-copy round trip through `Unix`, and the two files saved
directly to `Unix` behaved abnormally in different ways. A later native
`Save As...` attempt also produced a broken file, while a native Finder
duplicate edited and saved during Studio Vision quit reopened and functioned
normally. This supports the usability of the tested Finder-copy workflow and
save-on-quit procedure for their specific artifacts. It weakens a simple
hypothesis that the `Unix` destination alone explains the earlier failures,
but does not establish the cause of any failure or generalize either successful
procedure to other projects.

# Reassessment of Experiments 005 and 006

`CONTROLLED_SAVE_EXPERIMENT_005.md` correctly treated the cause of the broad
rewrite, metadata changes, and resource-fork loss as unknown. Its structural
measurements remain observations of Experiment 005, but interpretations that
use that single save to weaken serialization invariants must now be qualified:
Experiment 007 shows that the same size, relocation, and cadence changes occur
without the track edit and in a save first made on the native filesystem. Thus
those changes cannot be attributed specifically to the rename or to the direct
destination from the available evidence. Its recommended next step also did
not control save destination and should no longer be preferred.

`SAVE_AS_VS_SAVE_COPY_AS.md` correctly found that Experiments 005 and 006 share
a broad layout and warned that method, run, copying, and other state were not
separated. Its statement that both edited saves broadly differ from the
Original remains accurate, but no longer distinguishes the two direct saves:
the untouched native-save-derived baseline has that same broad layout. Its
prior behavioral section is superseded by the later Experiment 006 open test
reported in this task. Its recommendation to test Experiment 006 has therefore
already been carried out and is no longer the next unresolved comparison.

# Evidence supported

- Experiment 007 is derived by Finder copying from a native Desktop save that
  reopened successfully before copying. After the file on `Unix` was
  Finder-copied back to the native Desktop, the returned file also opened and
  functioned normally in Studio Vision without an intentional edit or resave.
- All three saved artifacts have the same size, relevant string locations,
  observed 166-byte label cadence, Finder Type/Creator, extended-attribute
  names and sizes, and absent resource fork.
- Broad reserialization relative to the Original is present in the untouched,
  native-save-derived Experiment 007 and is not unique to Experiments 005 or
  006.
- Experiment 007 and each direct save share more than 99% of same-position
  data-fork bytes and differ only in short, distributed runs within the shared
  layout.
- The direct saves exhibit abnormal reopening behavior, while the native
  Desktop source of Experiment 007 and the returned round-trip file opened and
  functioned normally.

# Evidence weakened

- The intentional rename is not supported as the source of the 8,046-byte
  growth, relocated sequence, or cadence change, because Experiment 007 has no
  intentional edit and reproduces them.
- Direct saving to `Unix` is not supported as the source of those broad
  serialization changes, because they are already present in Experiment 007.
- The earlier use of Experiments 005 and 006 alone to characterize save-method
  differences is weakened by the newly recognized uncontrolled destination and
  the shared structure in Experiment 007.
- Resource-fork absence and the BasiliskII attribute layout do not distinguish
  the direct saves from the native-save-derived host artifact.
- A simple destination-only explanation for the earlier failures is weakened
  by the reported broken native `Save As...` result and the verified-working
  native duplicate saved during Studio Vision quit.

# Unknowns

- Whether direct saving to `Unix` caused or contributed to either abnormal
  reopen result is unknown.
- Why the later native `Save As...` attempt failed while the native duplicate
  saved during quit remained functional is unknown.
- The exact stage at which Finder metadata was translated and the Original
  resource fork was lost is unknown.
- The meanings and origins of the sparse differences among the three saved
  data forks are unknown.
- Whether four non-label positions shared only by the two edited saves relate
  to the intentional rename is unknown.
- One artifact for each save path does not distinguish deterministic
  destination effects from run-specific state.
- No observation identifies event-bearing data or recovers a valid Standard
  MIDI File.

# Single recommended next step

Repeat the same native Desktop to `Unix` to native Desktop Finder-copy and
read-only reopen procedure with one additional known-good, unedited Studio
Vision project, recording hashes and fork metadata at every observable stage.
This tests whether the observed workflow is reproducible without treating one
successful artifact as a general result.
