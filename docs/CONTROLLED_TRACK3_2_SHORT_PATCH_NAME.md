# Objective

Determine the framing and length behavior of the confirmed editable Patch-name
field in the first Patch event of `Ode to Clarke` / `Track 3 #2` / `JD-800` by
changing only `Ming Dynasty` (12 characters) to `Phoenix` (7 characters). No
parsing was implemented and no Studio Vision artifact was modified.

# Controlled change

The operator changed only the first List Window Patch event's editable name:

`Ming Dynasty -> Phoenix`.

The event remained at `1·2·50` with displayed `PC 23`. Experiment 027 was
saved, reopened and verified in Studio Vision, and transferred using the
established controlled-save procedure. No intentional instrument, note,
note-position, pitch, duration, velocity, or other track-data change was made.

# Experiment lineage

Experiment 027 was created from a fresh Experiment 007 duplicate. It does not
descend from Experiments 023–026. All primary results compare Experiment 027
directly with Experiment 007. Experiment 026 is used only as an independent
equal-length name-content control, and the other controlled saves are used only
for property and save-variation controls.

# Preregistered prediction

Before Experiment 027 was inspected, the locked prediction was: if baseline
`0x31890 = 0x0c` is the Patch-name length byte, changing the name from 12 to 7
characters must change the corresponding aligned byte `0x0c -> 0x07`. No
prediction was registered for padding, relocation, removed bytes, record
lengths, or other serialization consequences. Stable predictions were that
Patch position would remain 530, PC would remain `0x17`, and the known note
performance would remain musically unchanged.

# Baseline and experiment identity

| Artifact | Exact path | Size | SHA-256 | Finder Type / Creator | Inode |
|---|---|---:|---|---|---:|
| Experiment 007 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline` | 211,468 | `e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132` | `MID2` / `MIDA` | 17242646 |
| Experiment 027 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 027 - Track 3-2 Short Patch Name/newest STUFF baseline EXP27` | 211,463 | `866085a09d9d5fbffd6af5f22ff2f4a52c94358341a211fff29a3799587d65b8` | `MID2` / `MIDA` | 17362286 |

Both uncompressed data forks expose FinderInfo beginning `MID2MIDA` and the
same three FinderInfo-related extended attributes. The baseline directory's
separate `.sit` is identified as a StuffIt archive, not the project data fork.
Different hashes and inodes confirm distinct files. Experiment 027 is exactly
five bytes smaller than baseline, matching the five-character name reduction.

# Structural alignment

Alignment was established separately on both sides of the variable-length
name:

- before the name, the absolute-position field remains at
  `0x31886–0x31887`, and the name-length byte remains at `0x31890`;
- the baseline name occupies `0x31891–0x3189c`; experiment `Phoenix` occupies
  `0x31891–0x31897`;
- the following `03 I38 04` context begins at baseline `0x3189d` and
  experiment `0x31898`, exactly five bytes earlier;
- `ff ff ff 17`, first-note properties, the unique complete 84-note chain,
  post-chain context, Track 7 context, and later event-chain anchors all align
  at experiment offsets exactly five bytes below baseline.

Track 3 #2 therefore retains its pre-name start but becomes five bytes shorter
at the variable-length name. All following structure relocates by `-5`; the
same delta persists through the remainder of the file except for ordinary
save-dependent field contents and relocation-adjusted references.

# Length-byte result

The preregistered prediction is **CONFIRMED** at the same structural and
absolute offset:

`0x31890: 0x0c -> 0x07`.

Controlled evidence now supports:

- 12-character `Ming Dynasty` -> length byte `0x0c`;
- 12-character `Phoenix Test` -> length byte `0x0c`;
- 7-character `Phoenix` -> length byte `0x07`.

The byte is therefore confirmed as the name payload length for this
representation.

# Patch-name storage

The name is stored directly as ASCII immediately after the length byte:

| State | Length offset/value | Payload offsets | ASCII | Hexadecimal |
|---|---|---|---|---|
| baseline | `0x31890 = 0x0c` | `0x31891–0x3189c` | `Ming Dynasty` | `4d 69 6e 67 20 44 79 6e 61 73 74 79` |
| Experiment 027 | `0x31890 = 0x07` | `0x31891–0x31897` | `Phoenix` | `50 68 6f 65 6e 69 78` |

This is a controlled one-byte-length-prefixed variable-length string. The
Pascal-style description is now justified for this observed representation:
one length byte is followed immediately by exactly that many text bytes.

# Shorter-name structural behavior

Studio Vision removes the five unused bytes entirely. There is no padding,
zero fill, residual `Dynasty`, or fixed-width slot. The next field, beginning
`03 49 33 38 04`, moves from baseline `0x3189d` to experiment `0x31898`, and
all subsequent primary Patch and Note structure moves by the same `-5` delta.
The total project size also decreases by five bytes.

Variable-length name classification:

- **A. Preceding byte is a length field: YES.**
- **B. Payload occupies exactly the specified byte count: YES.**
- **C. Name storage is variable-length: YES.**
- **D. Padding to a fixed width: NO.**
- **E. Following structure relocates with name length: YES.**
- **F. Larger Patch/container size updated: YES.** Two local size fields and
  several downstream offset-like fields respond by exactly `-5`; exact broader
  container ownership remains partial.

# Program Change relocation or stability

The confirmed PC field remains `0x17`, but relocates with the following
structure:

- baseline: `0x318a5` inside `ff ff ff 17` at `0x318a2–0x318a5`;
- Experiment 027: `0x318a0` inside `ff ff ff 17` at `0x3189d–0x318a0`;
- offset delta: `-5`.

This preserves the direct Program Change result while demonstrating that its
absolute offset depends on preceding variable-length data.

# Note-chain relocation or stability

The first-note status moves from baseline `0x318b4` to experiment `0x318af`.
First-note properties and the complete unique 585-byte chain move from
`0x318b5–0x31afe` to `0x318b0–0x31af9`, exactly `-5`. The relocated chain is
byte-for-byte identical, covering all 84 pitches, attack velocities, release
velocities, and durations plus all 83 note-to-note timing fields. There is no
musical-data change, only relocation.

# Patch timing stability

The primary Patch absolute-position VLQ precedes the variable-length name and
therefore remains at baseline offset `0x31886–0x31887`, unchanged as
`84 12` = 530 for displayed `1·2·50`. The interval component following the
name relocates from baseline `0x318a6–0x318a7` to experiment
`0x318a1–0x318a2` and remains `c5 4c` = 8,908. The stable 165 component moves
from `0x318b2–0x318b3` to `0x318ad–0x318ae` and remains `81 25`. Their sum
remains 9,073; no timing semantics are reinterpreted.

# Candidate size/length fields

Experiment 026 left these fields stable during an equal-length content change;
Experiment 027 changes them exactly with the five-byte reduction:

| Field | Baseline offset | Experiment offset | Baseline | Experiment | Delta | Assessment |
|---|---:|---:|---:|---:|---:|---|
| broad local container-size candidate | `0x31874–0x31877` | same | `00 00 02 8d` = 653 | `00 00 02 88` = 648 | -5 | high numeric response; exact container ownership partial |
| name-bearing Patch payload length | `0x3188a` | same | `1b` = 27 | `16` = 22 | -5 | high; counts bytes `0x3188b` through the PC byte, ending before the interval component |
| Patch-name length | `0x31890` | same | `0c` = 12 | `07` = 7 | -5 | confirmed |

The `0x3188a` value exactly equals the span from the following byte through the
confirmed PC field: baseline `0x3188b–0x318a5` is 27 bytes; experiment
`0x3188b–0x318a0` is 22 bytes. It is therefore identified as a local
name-bearing Patch payload length with high confidence.

Six downstream big-endian offset-like fields also decrease by five under
structural alignment: 110->105, 222->217, 326->321, 403->398, 578->573, and
636->631. Their precise ownership remains unknown. Known companion marker
bytes continue to show save variation and are not classified as name fields.

# Updated Patch-event field map

| Structural field | Baseline offset/range | Experiment 027 offset/range | Encoding | Evidence | Confidence |
|---|---|---|---|---|---|
| broad container-size candidate | `0x31874–0x31877` | same | 32-bit big-endian, 653/648 | Experiment 027 | high response; partial ownership |
| absolute Patch position | `0x31886–0x31887` | same | 7-bit VLQ, 530 | Experiment 025 plus stability | high |
| name-bearing payload length | `0x3188a` | same | one byte, 27/22 | Experiment 027 | high |
| Patch-name length | `0x31890` | same | one-byte length, 12/7 | Experiments 026/027 | high |
| Patch-name payload | `0x31891–0x3189c` | `0x31891–0x31897` | variable-length ASCII | Experiments 026/027 | high |
| Program Change | `0x318a5` | `0x318a0` | direct byte, 23 | Experiments 023/024 plus relocation | high |
| interval component | `0x318a6–0x318a7` | `0x318a1–0x318a2` | 7-bit VLQ, 8,908 | Experiment 025 plus relocation | high response; partial ownership |
| note status / first properties | `0x318b4` / `0x318b5` | `0x318af` / `0x318b0` | `90` / note properties | established note controls | high |

The earliest strongly supported Patch-related byte is now the responding broad
size candidate at `0x31874`; the confirmed primary Patch position begins at
`0x31886`. Unresolved bytes remain between the broad size field and position,
at `0x31888–0x31889`, inside the 27/22-byte payload before the name length,
between name and PC, and after the interval component. The first-note boundary
is exact but complete semantic ownership of every intervening byte remains
partial.

# Event-type discriminator

Event-type discrimination remains **PARTIAL**. `ff ff ff` relocates with the
variable-length Patch payload immediately before the confirmed PC field and is
therefore more securely associated with the Patch representation; `90` still
identifies the following Note stream. However, no controlled Patch-versus-Note
type change establishes the exact discriminator width, ownership, or generality.

# Implementation readiness

- **A. Identify this known Track 3 #2 Patch representation: YES.** Multiple
  confirmed fields and the exact following Note chain provide structural
  anchors.
- **B. Safely decode absolute position: YES.** Confirmed 7-bit VLQ.
- **C. Safely decode Patch name: YES.** Confirmed one-byte length followed by
  that many ASCII bytes.
- **D. Safely decode Program Change: YES.** Confirmed direct byte located after
  structurally aligned intervening fields.
- **E. Determine end of variable-length name: YES.** Length byte plus payload.
- **F. Determine full Patch-event boundary: PARTIAL.** Primary start and exact
  Note boundary are local, but interval/metadata ownership and type semantics
  remain incomplete.
- **G. Bounded parser spike justified: YES.** A diagnostic parser explicitly
  bounded to this known Track 3 #2 representation can validate and report the
  confirmed position, variable-length name, PC, offsets, and transition to the
  known Note chain across Experiments 007 and 023–027. This does not justify a
  general Studio Vision Patch parser or MIDI emission.

# Evidence supported

- Experiment 027 is a distinct `MID2/MIDA` project and is exactly five bytes
  smaller than baseline.
- `0x31890: 0c -> 07` confirms the locked name-length prediction.
- `Phoenix` is stored directly as seven ASCII bytes after the length.
- No padding or residual bytes remain; every following anchor moves by `-5`.
- The PC remains 23, the Patch position remains 530, and both interval
  components retain their values.
- The complete 84-note chain relocates by `-5` but remains byte-identical.
- Local size fields change 653->648 and 27->22; six downstream offset-like
  fields also decrease by five.
- The name representation and known fields are sufficient for a narrowly
  bounded diagnostic parser spike, not a general parser.

# Unknowns

Exact ownership and endpoint of the 653/648 container, meanings of unresolved
metadata, exact ownership of the compound Patch-to-first-note interval,
complete Patch-event end semantics, exact event-type discriminator width and
generality, downstream offset/reference ownership, and generality beyond this
known event/environment remain unknown.

# Single recommended next step

Implement a bounded, diagnostic-only Patch decoder spike for this known Track
3 #2 representation and validate it against Experiments 007 and 023–027. It
should accept explicit bounds/anchors, decode only the confirmed position,
one-byte-length-prefixed ASCII name, and direct PC fields, report all source
offsets and relocation, and verify the transition to the known Note chain. It
must not scan whole files, infer a general Patch grammar, or emit MIDI.
