# Objective

Test whether the independently observed first 17 Studio Vision List Window
rows for `Ode to Clarke` / `Track 3 #2` / `JD-800` are predicted by the
validated Track 7 event representation, and whether the matching bytes have a
repeatable local container relationship. The authentic Experiment 007 baseline
was inspected read-only. No MIDI was emitted and parser code was not changed.

# Independent ground truth

Studio Vision reported `85 Events`. The preregistered rows were the 17 rows
supplied for positions `18·4·241` through `27·4·466`. This is independent
ground truth from a different track and instrument than Track 7 / JV-1080-10.
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

The first region uniquely has the independently expected count relationship:
the four bytes at `0x312f4` are `00 00 00 55` (85). The analogous field in the
second region is `00 00 00 56` (86). The first region is therefore positively
identified as Track 3 #2. The second exact musical-prefix hit is retained as an
unidentified separate region; convenience or byte identity alone is not used
to assign it to a track.

Track 3 #2 row 1 properties begin at `0x313fa`. A VLQ-like field `82 00`
(256) immediately precedes them at `0x313f8`; its incoming musical semantics
are not inferred. Row 2 timing begins at `0x313ff`, properties at `0x31401`,
and follows row 1 without intervening bytes. Rows 1–17 are one consecutive
timing/property sequence through `0x3146f`.

# Property validation

At `0x313fa`, all fields match without resynchronization:

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

The selected Track 3 #2 marker is at `0x312fc`; eight bytes earlier, the
big-endian four-byte value at `0x312f4` is 85. Track 7's marker is at
`0x31c04`; eight bytes earlier, `0x31bfc` contains 143. These equal the two
independently observed Studio Vision List Window counts. The identical
relative placement and independently correct unequal values are strong
evidence for a repeated event-count field.

The unidentified second exact prefix has 86 at the same marker-relative
position. It is not relabeled Track 3 #2.

# Complete candidate extent

Beginning with the clean first property structure at `0x3131b`, 84 consecutive
note-property structures can be mechanically decoded. Row 1 of the visible
preregistered set is structure 33. Thus the local run contains 32 structures
before it, the 17 validated structures, and 35 further structures after row 17.
Later decoded musical values are not Studio Vision-validated ground truth.

The final decoded property structure begins at `0x3155f` and ends at
`0x31564`. Bytes immediately after it are:

`ff fb 8b 7d ff 2f 00 29 00 00 00 eb 00 06 00 00 ...`

The next conservative property candidate is invalid, so decoding stops there.
Exactly 85 note-property structures cannot be bounded: 84 are accounted for.
The nearby count of 85 may include a differently represented event, but that
meaning is not assigned without evidence.

# Boundary comparison with Track 7

| Relationship | Track 3 #2 | Track 7 | Assessment |
|---|---|---|---|
| Count then marker | 85 at marker − 8; marker `0x312fc` | 143 at marker − 8; marker `0x31c04` | repeated relationship |
| Marker | `2c c4 b2` | `2c c4 b2` | exact |
| First clean properties | no established timing for record's first property | no established timing for row 1 | similar treatment |
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
  a hard-coded offset, and the independently expected 85 field selected it
  from the two exact hits. This is evidence-backed identification, not a
  general discovery parser.
- **D. YES:** analogous marker-relative fields contain 85 and 143 for two
  independently identified tracks with those Studio Vision counts.
- **E. YES:** the identified regions repeat the marker, marker-relative count,
  event representation, and post-chain shape. Field semantics beyond those
  relationships remain limited.

# Evidence supported

- Track 3 #2 is identified at row-1 property offset `0x313fa`.
- The 17 preregistered rows match 68/68 property fields and 16/16 timing tests.
- A second exact 17-row byte hit exists at `0x31994` but has an analogous count
  of 86 and remains unidentified.
- The unequal 85/143 values recur at the same structural position.
- Thirty-five note structures decode after the validated rows, and 84 total
  consecutive note-property structures are bounded in the selected region.
- Parser and MIDI-emission code are unchanged.

# Unknowns

The reason for the second exact musical-prefix copy, the identity of its
region, the non-note contribution (if any) to the 85-event count, the first
record property's timing ownership, length/reference fields, complete
container grammar, and higher-level Sequence ownership remain unknown.

# Single recommended next step

Resolve the exact 85-event boundary by identifying the one differently
represented event implied by the 85 count versus 84 consecutive note-property
structures. Do not implement general discovery until that boundary is
explained.
