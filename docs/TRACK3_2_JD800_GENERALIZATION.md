# Objective

Test whether an independently observed consecutive 17-note Studio Vision List
Window sample for `Ode to Clarke` / `Track 3 #2` / `JD-800` is predicted by the
validated Track 7 event representation, and whether the matching bytes have a
repeatable local container relationship. The authentic Experiment 007 baseline
was inspected read-only. No MIDI was emitted and parser code was not changed.

# Independent ground truth

Studio Vision reported `85 Events`. The preregistered sample was the 17 notes
supplied for positions `18·4·241` through `27·4·466`. It was originally
described to the investigation as the beginning of the List Window. Complete
screenshots obtained later established that it is actually note indices 33–49,
or complete event indices 34–50, after one Patch event and 32 earlier notes.
This provenance correction does not change the consecutive sample's values or
its original comparison result. It remains independent ground truth from a
different track and instrument than Track 7 / JV-1080-10.
The project contains 18 Studio Vision Sequences; this report does not treat
other event-chain regions as tracks in `Ode to Clarke` without evidence.

The inspected uncompressed project is `newest STUFF baseline`, 211,468 bytes,
SHA-256 `e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`.

# Preregistered binary predictions

Pitch uses the already established Track 7 mapping. Duration is
`beats × 480 + units`. All VLQs are 7-bit big-endian.

| Row | Pitch | Attack | Release | Duration/value | Complete expected properties | Previous-to-current start difference / VLQ |
|---:|---:|---:|---:|---|---|---|
| 1 | `4d` | `7b` | `76` | 1655 / `8c 77` | `4d 7b 76 8c 77` | not tested |
| 2 | `4b` | `7d` | `3e` | 1156 / `89 04` | `4b 7d 3e 89 04` | 1630 / `8c 5e` |
| 3 | `4d` | `69` | `3b` | 257 / `82 01` | `4d 69 3b 82 01` | 1703 / `8d 27` |
| 4 | `50` | `72` | `54` | 240 / `81 70` | `50 72 54 81 70` | 249 / `81 79` |
| 5 | `4d` | `67` | `35` | 1432 / `8b 18` | `4d 67 35 8b 18` | 220 / `81 5c` |
| 6 | `50` | `6c` | `38` | 290 / `82 22` | `50 6c 38 82 22` | 1406 / `8a 7e` |
| 7 | `52` | `6c` | `32` | 249 / `81 79` | `52 6c 32 81 79` | 276 / `82 14` |
| 8 | `50` | `6e` | `72` | 3793 / `9d 51` | `50 6e 72 9d 51` | 231 / `81 67` |
| 9 | `55` | `4e` | `7f` | 461 / `83 4d` | `55 4e 7f 83 4d` | 7967 / `be 1f` |
| 10 | `54` | `4c` | `51` | 443 / `83 3b` | `54 4c 51 83 3b` | 483 / `83 63` |
| 11 | `50` | `5c` | `33` | 480 / `83 60` | `50 5c 33 83 60` | 438 / `83 36` |
| 12 | `4b` | `5a` | `3c` | 301 / `82 2d` | `4b 5a 3c 82 2d` | 463 / `83 4f` |
| 13 | `4d` | `71` | `3f` | 1470 / `8b 3e` | `4d 71 3f 8b 3e` | 284 / `82 1c` |
| 14 | `4b` | `55` | `41` | 257 / `82 01` | `4b 55 41 82 01` | 1465 / `8b 39` |
| 15 | `4d` | `6b` | `36` | 281 / `82 19` | `4d 6b 36 82 19` | 243 / `81 73` |
| 16 | `50` | `64` | `2a` | 278 / `82 16` | `50 64 2a 82 16` | 209 / `81 51` |
| 17 | `54` | `60` | `7f` | 3608 / `9c 18` | `54 60 7f 9c 18` | 238 / `81 6e` |

# Existing candidate test

The five previously recorded candidates were tested first, from their
documented mechanically clean starts. Corresponding events were compared in
fixed order for 17 rows. No search-ahead or resynchronization was used.

| Candidate | Pitch | Attack | Release | Duration | Timing rows 2–17 | Complete properties | Complete including timing |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1, `0x2fb7a` | 0/17 | 1/17 | 1/17 | 0/17 | 0/16 | 0/17 | 0/16 |
| 2, `0x301d8` | 0/17 | 0/17 | 0/17 | 0/17 | 0/16 | 0/17 | 0/16 |
| 3, `0x30a32` | 0/17 | 0/17 | 1/17 | 0/17 | 0/16 | 0/17 | 0/16 |
| 4, `0x30f4c` | 0/17 | 0/17 | 0/17 | 0/17 | 0/16 | 0/17 | 0/16 |
| 5, `0x31677` | 0/17 | 0/17 | 0/17 | 0/17 | 0/16 | 0/17 | 0/16 |

Every candidate failed on all four row-1 properties. None is Track 3 #2 under
the preregistered representation.

# Track location

Only after those failures, a whole-file search used the preregistered 117-byte
17-event sequence. It produced two exact hits, and the same two hits persisted
when prefixes of 2, 3, 4, 5, and 8 complete events were tested:

- `0x313fa`, in the marker-framed region beginning near `0x312fc`;
- `0x31994`, in a separate marker-framed region beginning near `0x31882`.

The original investigation selected the first hit because `00 00 00 55` (85)
occurs near its marker. That inference was wrong. Complete-list ground truth
now distinguishes the hits: the first note must be C#5 with attack 100, and the
track patch is Ming Dynasty. Those observations match the second region, whose
note chain begins at `0x318b5` and whose pre-note region contains literal
`Ming Dynasty`. The first region begins with C#5 attack 78 and contains
`Wavox`; it is a separate unidentified region despite sharing the later
17-note sequence.

The validated sample's first properties are at `0x31994`. A timing VLQ `82 00`
(256) immediately precedes them at `0x31992`. The next timing begins at
`0x31999`, properties at `0x3199b`, and follows without intervening bytes. The
17 notes are one consecutive timing/property sequence through `0x31a09`.

# Property validation

At corrected offset `0x31994`, all fields match without resynchronization:

| Property | Exact matches |
|---|---:|
| Pitch | 17/17 |
| Attack velocity | 17/17 |
| Release velocity | 17/17 |
| Duration | 17/17 |
| Complete property rows | 17/17 |
| Individual property fields | 68/68 |

The Track 7 property model independently predicted note data for a different
track and different instrument.

# Timing validation

For rows 2–17, every binary timing value immediately before the property
structure equals the current displayed start minus the previous displayed
start under 4 beats per measure and 480 units per beat: **16/16**. There are no
mismatches. Row 1's preceding `82 00` is reported but not interpreted as an
incoming displayed-position difference.

# Event-count analysis

This report originally selected the `0x312fc` marker because its marker-minus-
eight value is 85. Complete ground truth corrects Track 3 #2 to marker
`0x31882`, where the analogous value at `0x3187a` is 86, while Studio Vision
shows 85 total events. Track 7's analogous value remains 143 for 143 displayed
notes. The corrected 86/85 result contradicts the earlier claim that this is
an exact repeated List Window event-count field. The `0x312f4` value 85 belongs
to the separate Wavox region.

# Complete candidate extent

Beginning with the corrected first-note property structure at `0x318b5`, 84
consecutive note-property structures decode and now match the complete Studio
Vision screenshots. The preregistered sample is note structures 33–49. Thus the
run contains 32 notes before it, the 17 validated subset, and 35 after it.

The final property structure begins at `0x31af9` and ends at cursor `0x31afe`.
Bytes immediately after it are:

`ff fb 8b 7d ff 2f 00 29 00 00 00 eb 00 06 00 00 ...`

The next conservative property candidate is invalid, so note decoding stops.
The complete List Window explains the count exactly as one Patch plus 84 notes;
it does not imply a missing note. The Patch is partially localized in the
pre-note bytes but is not yet represented by a complete decoded event grammar.

# Boundary comparison with Track 7

| Relationship | Track 3 #2 | Track 7 | Assessment |
|---|---|---|---|
| Marker-minus-eight value | 86 at `0x3187a`; marker `0x31882` | 143 at `0x31bfc`; marker `0x31c04` | same placement, not UI-count equality |
| Marker | `2c c4 b2` | `2c c4 b2` | exact |
| First clean properties | `90` then properties at `0x318b5` | `90` then properties at `0x31c0c` | repeated treatment |
| Later events | timing, pitch, attack, release, duration | same | exact representation |
| Post-chain | `ff fb 8b 7d ff 2f 00 29 ...` | `ff fa b9 2f ff 2f 00 29 ...` | repeated framing shape |
| Alignment | count begins on 4-byte boundary; marker +8 | same | exact relationship |

The pre-marker payload/header bytes and post-chain variable bytes differ.
Possible length/reference fields remain unproved. The repeated relationships
support framing for these two identified tracks, not all Studio Vision tracks.

# Sequence association

**Strongly supported:** Studio Vision ground truth places Track 3 #2 and Track
7 in `Ode to Clarke`; known exported `Ode to Clarke` track metadata also names
both tracks, and the project metadata records place their labels consecutively.

**Plausible:** the two identified binary regions belong to the same broad
event-record family because they repeat the count/marker/event/post-context
relationships.

**Unknown:** no validated pointer, reference, or higher-level binary container
links either event region to the `Ode to Clarke` Sequence or links the two
regions to each other. Because the project has 18 Sequences, metadata order and
nearby file location are not promoted into a Sequence-container claim.

# Reassessment of previous candidates

Track 3 #2 was not one of the five candidates. The prior heuristic recorded
only long runs beginning at a mechanically clean timing/property boundary. The
Track 3 #2 record begins with non-note/header material and a property structure
whose first timing ownership is ambiguous; its strict note walk also accounts
for 84 rather than the nearby 85. Those conditions prevented it from meeting
the previous candidate-start rule.

The five candidates remain credible unidentified members of a repeated
event-like record family. Their marker and post-context evidence is unchanged;
their track, Sequence, and event-type identities remain unknown.

# Engineering conclusions

- **A. YES:** all 68/68 note-property fields and 17/17 complete rows match.
- **B. YES:** all 16/16 testable displayed start differences match timing.
- **C. YES:** a preregistered multi-event signature located the region without
  a hard-coded offset; later complete-list first-note and Patch-name ground
  truth selected the correct one of two hits. This is evidence-backed
  identification, not a general discovery parser.
- **D. NO:** the corrected Track 3 #2 field is 86 for 85 UI events, so exact
  repeated event-count semantics are contradicted.
- **E. YES:** the identified regions repeat the marker, marker-relative field
  placement, event representation, and post-chain shape. Field semantics
  beyond those relationships remain limited.

# Evidence supported

- Track 3 #2 is identified at first-note property offset `0x318b5`; the
  validated 17-note sample begins at `0x31994`.
- The 17 preregistered rows match 68/68 property fields and 16/16 timing tests.
- A second exact 17-note byte hit begins at `0x313fa`, but its surrounding
  region is Wavox and its first-note attack differs; it remains unidentified.
- The corrected marker-relative values are 86 for Track 3 #2 and 143 for Track
  7; the earlier 85/143 event-count conclusion is withdrawn.
- Thirty-five note structures decode after the validated rows, and 84 total
  consecutive note-property structures are bounded in the selected region.
- Parser and MIDI-emission code are unchanged.

# Unknowns

The reason for the duplicate musical prefix, the Wavox region's identity, the
Track 3 #2 Patch record's exact framing/timing ownership, the meaning of the
nearby 86, length/reference fields, complete container grammar, and higher-
level Sequence ownership remain unknown.

# Single recommended next step

Perform one controlled Patch-only edit on a disposable copy, leaving all notes
and event positions unchanged, to isolate the Patch type/program/name fields
and timing ownership before implementing mixed-event parsing.
