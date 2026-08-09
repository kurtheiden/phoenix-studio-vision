# Objective

This report performs the first controlled comparison between an untouched
Studio Vision baseline and a same-length track rename where both compared
projects were produced through native Mac OS 9 filesystem operations and were
verified by the user to function normally in Studio Vision. It is a read-only
evidence comparison. It does not modify an artifact, implement parser logic,
or infer binary-field semantics.

# Experimental provenance

Experiment 007 is the untouched baseline derived from the authentic
`newest STUFF` project. Studio Vision saved it to the native Mac OS 9 Desktop
with no intentional project edit, and it opened and functioned normally there.
The project also remained functional after a Finder-copy round trip from the
native filesystem through SheepShaver's `Unix` shared volume and back.

Experiment 008 used a different procedure after another native `Save As...`
attempt produced a broken file. The user duplicated a known-good Studio Vision
file with Finder on the native Mac OS 9 filesystem, opened the duplicate,
changed exactly `Track 7` to `Track A`, quit Studio Vision, and chose to save
when prompted during quitting. The resulting file reopened and functioned
normally in Studio Vision. It was then Finder-copied to `Unix` and copied into
the research directory.

The comparison therefore concerns an untouched native-save-derived baseline
and a verified-working native Finder duplicate modified by save-on-quit. It
does not treat `Save As...`, direct-to-`Unix` saving, and save-on-quit as
equivalent operations. The StuffIt archive beside Experiment 007 was excluded.

# Artifact inventory

Each directory was inspected before selecting the artifacts. Experiment 007
contained the uncompressed project and a `.sit` archive; only the uncompressed
project was used. Experiment 008 contained one file, whose basename was not
assumed.

| Artifact | Exact path | Basename | Data-fork size | Data-fork SHA-256 |
|---|---|---|---:|---|
| Experiment 007 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline` | `newest STUFF baseline` | 211,468 (`0x33a0c`) | `e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132` |
| Experiment 008 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 008 - Native Duplicate Edit Save/newest STUFF Track A native` | `newest STUFF Track A native` | 211,468 (`0x33a0c`) | `ccc53da15cdd9044ee475e1a1ffd92be6301f70a0a64f3718df361aa6c412ffb` |

Both artifacts have Finder Type `MID2`, Finder Creator `MIDA`, a 32-byte
`com.apple.FinderInfo`, a 16-byte `org.BasiliskII.FinderInfo`, and a 16-byte
`org.BasiliskII.ExtendedFinderInfo`. Neither has a resource fork, and no other
extended attributes were observed. Their `com.apple.FinderInfo` values are
identical, as are their zero-filled extended FinderInfo values. Their
BasiliskII FinderInfo values differ: Experiment 007 has
`4d4944324d4944410100007b01080000`, while Experiment 008 has
`4d4944324d4944410100ffffffff0000`.

The relevant printable labels remain aligned:

| Label | Experiment 007 | Experiment 008 |
|---|---:|---:|
| `Meter Track` | `0x0002f04e` | `0x0002f04e` |
| `Tempo Track` | `0x0002f0f4` | `0x0002f0f4` |
| `Track 1` | `0x0002f19a` | `0x0002f19a` |
| `Track 2` | `0x0002f240` | `0x0002f240` |
| `sys100loops` | `0x0002f2e6` | `0x0002f2e6` |
| `Track 4` | `0x0002f38c` | `0x0002f38c` |
| `Track 5` | `0x0002f432` | `0x0002f432` |
| `Track 3` | `0x0002f4d8` | `0x0002f4d8` |
| `Track 6` | `0x0002f57e` | `0x0002f57e` |
| `Track 3 #2` | `0x0002f624` | `0x0002f624` |
| `Track 7` / `Track A` | `0x0002f6ca` | `0x0002f6ca` |
| `Ode to Clarke` | `0x0002f753` | `0x0002f753` |

Successive track-label locations differ by 166 (`0xa6`) bytes in both files.

# Whole-file comparison

The data forks are the same size but are not byte-identical. At identical
absolute positions, 1,208 of 211,468 bytes differ, approximately 0.57%, in 412
disjoint runs. The first difference is `0x0000001e`, the last is
`0x0003380a`, and the maximum unequal-run length is four bytes. The common
prefix is 30 bytes and the common suffix is 513 bytes.

The unequal-run length distribution is four one-byte runs, 21 two-byte runs,
386 three-byte runs, and one four-byte run. Differences occur in every
`0x4000`-byte bin spanning the file. At the same time, file size, all relevant
label offsets, the observed 166-byte cadence, and `Ode to Clarke` remain
aligned. This supports sparse, distributed byte variation within the same
broad serialization layout. No insertion or deletion is inferred.

# Track-label comparison

Experiment 007 contains `Track 7` at `0x0002f6ca`; Experiment 008 contains
`Track A` at the identical offset. The ten preceding label offsets establish a
166-byte cadence. Using the established relative label position `0x0f` gives
an aligned comparison structure from `0x0002f6bb` through `0x0002f760`, 166
bytes inclusive. This is a cadence-supported comparison structure, not a
proven semantic record boundary.

Every difference in that aligned structure is:

| Absolute offset | Relative offset | Experiment 007 | Experiment 008 | Classification |
|---:|---:|---:|---:|---|
| `0x0002f6d0` | `0x15` | `37` (`7`) | `41` (`A`) | expected printable rename |
| `0x0002f709` | `0x4e` | `c4` | `da` | non-label, run-varying |
| `0x0002f70a` | `0x4f` | `b2` | `f1` | non-label, run-varying |
| `0x0002f70b` | `0x50` | `60` | `34` | non-label, run-varying |
| `0x0002f715` | `0x5a` | `c4` | `da` | non-label, run-varying |
| `0x0002f716` | `0x5b` | `b2` | `ef` | non-label, run-varying |
| `0x0002f717` | `0x5c` | `a0` | `78` | non-label, run-varying |

The printable byte directly represents the observed `7` to `A` label change.
The other six positions correlate with this comparison but do not reproduce
the same edited values in Experiments 005 and 006. At each of those positions,
Experiments 005, 006, and 008 have different values. Their recurring locations
within save outputs may merit later study, but the present evidence supports
classifying their values as run-specific rather than rename-deterministic.

# Reproduced edit-correlated differences

The prior three-way comparison identified five positions where Experiment 007
differed and Experiments 005 and 006 shared a value. One was the printable
rename byte. The other four were outside the aligned track structure:

| Absolute offset | Relative position | Experiment 007 | Experiment 008 | Experiments 005/006 | Reproduced by 008 |
|---:|---|---:|---:|---:|---|
| `0x0002efa3` | outside | `9f` | `9f` | `95` | no |
| `0x0002efa5` | outside | `4d` | `4d` | `00` | no |
| `0x0002efa7` | outside | `a1` | `a1` | `97` | no |
| `0x0002efa9` | outside | `c9` | `c9` | `7c` | no |
| `0x0002f6d0` | aligned `0x15` | `37` | `41` | `41` | yes |

Thus Experiment 008 reproduces none of the four previously interesting
non-label changes. It matches the untouched baseline at all four positions.
Across the complete file, `0x0002f6d0` is the only position where Experiment
007 has one value and Experiment 008, Experiment 005, and Experiment 006 all
share another value. The independently reproduced evidence therefore isolates
the expected printable rename byte; it does not identify an additional
edit-correlated non-label byte.

# Candidate numeric windows

The following are mechanical unsigned big-endian readings within the aligned
166-byte comparison structure beginning at `0x0002f6bb`. They are not assigned
any semantic role.

| Relative window | Experiment 007 bytes | Experiment 007 value | Experiment 008 bytes | Experiment 008 value | Changed |
|---|---|---:|---|---:|---|
| `0x25–0x28` | `00 02 02 f7` | `0x202f7` (131,831) | `00 02 02 f7` | `0x202f7` (131,831) | no |
| `0x5b–0x5e` | `b2 a0 00 00` | `0xb2a00000` (2,996,830,208) | `ef 78 00 00` | `0xef780000` (4,017,618,944) | yes |
| `0x4b–0x4e` | `00 00 2c c4` | `0x2cc4` (11,460) | `00 00 2c da` | `0x2cda` (11,482) | yes |
| `0x31–0x34` | `00 05 00 25` | `0x50025` (327,717) | `00 05 00 25` | `0x50025` (327,717) | no |

The two changed windows include run-varying bytes. Experiments 005 and 006 do
not reproduce Experiment 008's resulting values: at `0x5b–0x5e` their values
are respectively `0xc7ac0000` and `0x42900000`, and at `0x4b–0x4e` they are
`0x2cdb` and `0x2cb7`. Conversely, both unchanged windows have the same bytes
in all four saved artifacts. These observations strengthen stability evidence
for those two byte windows only; they establish no field meaning.

# Run-specific differences

Of the 1,208 unequal Experiment 007/008 positions, only the printable rename
byte exactly matches the edited value independently observed in both prior
edited artifacts. The remaining 1,207 positions do not meet that
edit-correlation test. Among them, Experiment 008 happens to equal only
Experiment 005 at 23 positions and only Experiment 006 at three positions;
three positions have an equal 005/006 value different from both 007 and 008;
and 1,178 fall into other value patterns. These counts are mechanical equality
classes, not semantic categories.

The widespread short-run distribution, the distinct values among edited runs
inside the aligned track structure, and the lack of three-edited-file
reproduction support treating the large population as run-specific variation
for this comparison. Individual causes remain unresolved; distribution alone
does not establish timestamps, identifiers, checksums, addresses, or any other
meaning.

# Evidence supported

- Experiments 007 and 008 are same-size, verified-working Studio Vision files
  with aligned broad serialization and identical relevant label locations.
- The expected printable `Track 7` to `Track A` byte changes from `37` to `41`
  at the same absolute and relative position.
- That printable byte is the only whole-file Experiment 007/008 difference
  whose Experiment 008 value is independently identical in Experiments 005 and
  006.
- Experiment 008 does not reproduce the four prior non-label positions shared
  only by Experiments 005 and 006; it matches Experiment 007 at all four.
- Two candidate numeric windows remain byte-identical across all four saved
  artifacts, while two others exhibit different values across save runs.
- A verified-working edited native save can differ from the verified-working
  untouched baseline in sparse, distributed non-label bytes without disrupting
  the observed broad alignment.
- The broken native `Save As...` attempt weakens a simple hypothesis that the
  `Unix` destination alone explains earlier save failures.

# Unknowns

- The semantic meaning, if any, of every non-label differing position is
  unknown.
- The mechanism producing the distributed run-specific bytes is unknown.
- It is unknown why the separate native `Save As...` attempt produced a broken
  file while saving the edited Finder duplicate during quit produced a working
  file.
- The 166-byte cadence supports alignment but does not establish complete
  logical record boundaries.
- Equality or change in a candidate numeric window does not establish its
  width, signedness, purpose, or relationship to any other region.
- One successful save-on-quit controlled edit does not establish that the
  procedure is generally reliable.
- No observation identifies event-bearing data or recovers a valid Standard
  MIDI File.

# Single recommended next step

Repeat the `Track 7` to `Track A` edit once more from a fresh native Finder
duplicate of the same known-good Experiment 007 baseline, save only during
Studio Vision quit, and verify reopenability before Finder copying. A second
verified-working native save-on-quit replicate would test whether the six
non-label bytes in the aligned structure and the wider 1,207-byte population
vary again while the isolated printable rename byte remains deterministic.
