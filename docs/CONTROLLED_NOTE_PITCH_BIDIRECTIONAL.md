# Objective

This report tests the bidirectional pitch prediction established by Experiment
009. The same drum note in `Track 7` of `Ode to Clarke` was changed from the
Experiment 007 baseline C1 to C#1 in Experiment 009 and from C1 to B0 in
Experiment 010. Timing, duration, velocity, and note count were unchanged. The
work is a read-only evidence comparison; it does not modify an artifact,
implement parser logic, or assign event semantics.

# Experimental provenance

Experiment 007 is the untouched, verified-working native-save baseline derived
from the authentic `newest STUFF` project. Experiment 009 and Experiment 010
were each created from fresh native Finder duplicates of the same known-good
baseline. In each edited experiment, the user made one intentional pitch edit,
quit Studio Vision, chose Save when prompted, reopened and verified that the
project functioned normally, quit without saving, and Finder-copied the file
through SheepShaver's `Unix` volume into the research folder.

Experiment 009 changed the selected drum note upward from C1 to C#1.
Experiment 010 changed the same note downward from C1 to B0. Both therefore use
the verified-working native duplicate and save-on-quit procedure. The StuffIt
archive beside Experiment 007 was excluded.

# Artifact inventory

All three directories were inspected before selecting the project files.
Experiment 007 contained an uncompressed project and a StuffIt archive;
Experiments 009 and 010 each contained one uncompressed project. Experiment
010's reported basename was confirmed exactly.

| Artifact | Exact path | Basename | Data-fork size | Data-fork SHA-256 |
|---|---|---|---:|---|
| Experiment 007 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline` | `newest STUFF baseline` | 211,468 (`0x33a0c`) | `e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132` |
| Experiment 009 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 009 - Single MIDI Note Pitch Change/newest STUFF baseline copy` | `newest STUFF baseline copy` | 211,468 (`0x33a0c`) | `2b08f822c65bf21b1eeda8f509e6b9c162414f210bb2fbf037650a3541c8ce87` |
| Experiment 010 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 010 - Single MIDI Note Down One Semitone/newest STUFF baseline EXP10` | `newest STUFF baseline EXP10` | 211,468 (`0x33a0c`) | `146d4a9da37aed1c1d6085ead4af9c57ad6d51e4e021001ae442f5d1fc9d2a42` |

All three artifacts have Finder Type `MID2`, Finder Creator `MIDA`, a 32-byte
`com.apple.FinderInfo`, a 16-byte `org.BasiliskII.FinderInfo`, and a 16-byte
`org.BasiliskII.ExtendedFinderInfo`. No other extended attributes or resource
forks were observed. The `com.apple.FinderInfo` values and zero-filled extended
FinderInfo values are identical. Experiment 007 has BasiliskII FinderInfo
`4d4944324d4944410100007b01080000`; Experiments 009 and 010 both have
`4d4944324d4944410100ffffffff0000`.

The relevant labels are aligned identically in all three files:

| Label | Absolute offset |
|---|---:|
| `Meter Track` | `0x0002f04e` |
| `Tempo Track` | `0x0002f0f4` |
| `Track 1` | `0x0002f19a` |
| `Track 2` | `0x0002f240` |
| `sys100loops` | `0x0002f2e6` |
| `Track 4` | `0x0002f38c` |
| `Track 5` | `0x0002f432` |
| `Track 3` | `0x0002f4d8` |
| `Track 6` | `0x0002f57e` |
| `Track 3 #2` | `0x0002f624` |
| `Track 7` | `0x0002f6ca` |
| `Ode to Clarke` | `0x0002f753` |

The established 166-byte (`0xa6`) label cadence is unchanged.

# Whole-file comparisons

All three data forks have identical sizes and retain aligned labels and broad
serialization structure. Direct same-position measurements are:

| Comparison | Unequal bytes | Disjoint runs | First difference | Last difference | Maximum run | Common prefix | Common suffix |
|---|---:|---:|---:|---:|---:|---:|---:|
| Experiment 007 vs 009 | 1,885 | 733 | `0x0000001e` | `0x00033a06` | 36 bytes | 30 bytes | 5 bytes |
| Experiment 007 vs 010 | 1,886 | 728 | `0x0000001e` | `0x00033a06` | 36 bytes | 30 bytes | 5 bytes |
| Experiment 009 vs 010 | 1,008 | 484 | `0x00000fdc` | `0x0003380a` | 7 bytes | 4,060 bytes | 513 bytes |

These are aligned same-position comparisons. No insertion or deletion is
inferred.

# Previously identified pitch candidates

Experiment 009 identified 81 positions satisfying all of these mechanical
conditions:

- Experiment 007 was `0x24`;
- Experiment 009 was `0x25`; and
- Experiments 005, 006, and 008 all retained `0x24`.

The exact set was reconstructed from those conditions rather than transcribed
manually. At all 81 positions, Experiment 010 contains `0x23`.

| Experiment 010 value at the 81 positions | Count |
|---|---:|
| `0x23` | 81 |
| `0x24` | 0 |
| `0x25` | 0 |
| any other value | 0 |

Under the explicitly stated hypothesis where MIDI note 35 is B0, note 36 is
C1, and note 37 is C#1, these hexadecimal bytes match the predicted downward
and upward semitone values exactly. This supports a pitch-bearing numeric
representation. It does not by itself prove that the bytes are framed MIDI
events or that Studio Vision universally uses this note-naming convention.

# Dense candidate region

Eighty of the prior 81 candidates occupy the 890-byte range
`0x00031c1f–0x00031f98`. Every one changes from baseline `0x24` to `0x23` in
Experiment 010.

| Dense-region outcome | Count |
|---|---:|
| Experiment 010 `0x23` | 80 |
| retains baseline `0x24` | 0 |
| Experiment 010 `0x25` | 0 |
| another value | 0 |

All 70 previously observed `81 70 24` local forms become `81 70 23` in
Experiment 010 and `81 70 25` in Experiment 009. All four `83 60 24` forms
likewise become `83 60 23` and `83 60 25`. The remaining six candidate bytes
have other immediate prefixes but follow the same three-point relationship.

Representative aligned examples are:

| Offset | Experiment 010 context | Experiment 007 context | Experiment 009 context |
|---:|---|---|---|
| `0x00031c1f` | `81 3b 81 65 23 7f 5c 83 3a` | `81 3b 81 65 24 7f 5c 83 3a` | same context with central `25` |
| `0x00031c2d` | `81 75 81 70 23 7f 60 6b 83` | `81 75 81 70 24 7f 60 6b 83` | same context with central `25` |
| `0x00031f98` | `7f 50 81 70 23 7f 7f 76 ff` | `7f 50 81 70 24 7f 7f 76 ff` | same context with central `25` |

The surrounding bytes in these examples remain fixed across the stated local
windows. The repeated forms and perfect bidirectional response strongly
support the region as pitch-bearing. The variable spacing between occurrences
and the multiplicity of 80 responsive bytes still do not establish event
boundaries or explain why one intentional edit changes this many stored
values.

# Isolated candidate

At `0x0002f76f`, the baseline `0x24` becomes `0x25` in Experiment 009 and
`0x23` in Experiment 010. Experiments 005, 006, and 008 retain `0x24`. This
isolated post-structure candidate therefore satisfies the same perfect
bidirectional relationship as all 80 dense-region candidates.

At nearby `0x0002f777`, Experiment 007 is `0x01`, while both Experiments 009
and 010 are `0x02`; Experiments 005, 006, and 008 retain `0x01`. The repeated
value across both pitch edits is directly observed, but no semantic meaning is
assigned. Because `01` to `02` changes occur widely in Experiment 009 output,
this nearby byte is not treated as direct pitch evidence.

# Bidirectional pitch relationship

Every previously identified position forms the exact three-point relationship:

| Experiment 010, B0 | Experiment 007, C1 | Experiment 009, C#1 | Matching positions |
|---:|---:|---:|---:|
| `0x23` | `0x24` | `0x25` | 81 of 81 |

Experiment 009 and Experiment 010 consequently differ at all 81 positions,
with `0x25` versus `0x23`. There are no counterexamples in the predicted set.
The symmetric numerical response to independent upward and downward edits,
together with baseline values in all three non-pitch controls, strongly
supports a pitch-bearing numeric representation at these positions.

This is stronger than a one-direction correlation because the downward value
was predicted before inspecting Experiment 010. It still does not establish
whether the 81 bytes are direct note values, repeated dependent values, a drum
mapping, cached material, or a mixture of representations.

# Save-run controls

Relative to Experiment 007, Experiment 010 has 1,886 unequal positions. They
classify mechanically as:

| Class | Count | Observation |
|---|---:|---|
| exactly reproduced Experiment 008 value | 307 | common save-on-quit output, not specific to either intentional edit |
| other previously variable locations | 919 | offset varied in 005, 006, or 008, but value is not the reproduced 008 value |
| bidirectional pitch positions | 81 | baseline `24`, Experiment 009 `25`, Experiment 010 `23`, prior controls `24` |
| new unresolved differences | 579 | absent from the 005/006/008 baseline comparisons and not in the pitch set |

The first two classes total 1,226 positions already implicated in prior
save-output variation. Their values and locations provide negative/control
evidence against interpreting the entire whole-file diff as musical content.
The 579 new differences remain unresolved; uniqueness to one save does not
establish a relationship to the intentional edit.

The pitch set is distinguished from these populations by a preregistered
numeric prediction, exact agreement at all positions, stable same-position
alignment, and baseline values in all non-pitch controls. No other class is
assigned field meaning.

# Evidence supported

- All three verified-working artifacts have equal sizes, aligned broad
  structure, and identical relevant label locations.
- Experiment 010 contains `0x23` at all 81 previously identified pitch
  candidates; none retains or takes another value.
- All 80 dense-region candidates respond bidirectionally, including every
  `81 70 XX` and `83 60 XX` family member.
- The isolated `0x0002f76f` candidate also responds exactly as predicted.
- All 81 positions form `0x23 ← 0x24 → 0x25`, while Experiments 005, 006, and
  008 remain at baseline `0x24`.
- This perfect predicted three-point relationship strongly supports a
  pitch-bearing numeric representation.
- No result within the predicted set contradicts direct numeric pitch storage.
  However, 81 responsive positions for one intentional note edit are evidence
  against a simple assumption that each occurrence independently represents
  the single edited event.
- The save-run controls isolate 1,226 Experiment 010 differences at previously
  variable positions and leave 579 additional differences unresolved.

# Unknowns

- It is unknown why one note edit changes 81 pitch-bearing positions.
- It is unknown whether the bytes are direct note values, repeated dependent
  values, a drum mapping, cached data, or another representation.
- No event framing, time, duration, velocity, channel, track ownership, or
  note-on/note-off relationship has been established.
- It is unknown whether the isolated `0x0002f76f` position and dense region
  serve the same role despite their identical numeric response.
- The meaning of `0x0002f777` and the 579 new unresolved differences is
  unknown.
- The mechanism and deterministic extent of save-on-quit variation remain
  unknown.
- A pitch-bearing numeric representation is strongly supported, but no valid
  Standard MIDI File has yet been recovered.

# Single recommended next step

From a fresh native Finder duplicate of Experiment 007, change only the same
drum note's velocity by exactly one known unit while retaining C1 pitch, time,
duration, and note count. After the verified-working save-on-quit procedure,
first confirm that all 81 pitch-bearing positions remain `0x24`, then search
the surrounding dense-region structures for a separately reproduced one-unit
response. This is the narrowest next experiment for distinguishing event-like
records from repeated pitch-dependent storage while retaining the established
pitch anchor.
