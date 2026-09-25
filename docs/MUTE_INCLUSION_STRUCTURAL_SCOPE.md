# Mute and MIDI inclusion: structural scope after Experiments 035 and 036

Read-only evidence review, 2026-09-25. Baseline: `main`,
`f48744400ffa4e7c50f7614474d1815dc41ec146`. No implementation, experiment,
export, or commit is performed by this review.

## Decision

**B — IMPLEMENT NARROW SUBSET, as a bounded saved-mute-state interpretation,
not an unconditional MIDI-inclusion policy.** Two independently targeted tracks
now support the same state transition in the same bounded structure. They are
separate controlled interventions on a shared project lineage: Experiment 036
reuses Experiment 035 EDIT, not an independently sourced project or application
version. This does not establish independence across projects or versions. There is
no evidence of a different descriptor class that explains away the historical
Ode reference. Separate saved state from the desired export contract. Generic
export authorization and exact-profile policy must remain unchanged.

This is a recommendation for subsequent owner-reviewed implementation, not
implementation in this task. If “decoder” means deciding inclusion from project
bytes alone for every export, that stronger proposal is not supported.

## Corpus, ownership, and observation vocabulary

The reviewed structural corpus is the authenticated Experiment 007 project and
its returned Experiment 033-036 descendants, not every unrelated file in the
research collection. All 18 UI-authenticated sequence identities are covered:
133 musical descriptor slots; 122 slots have unambiguous equal-count ordinal
track/pair binding; Sequence I's remaining 11 do not. Older 120-byte forms and
unbound projects are outside the located-field evidence.

Baseline source: `Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline`
under `/Users/kurtheiden/Documents/Phoenix Research/`, 211468 bytes, SHA-256
`e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`.
The root walk, sequence names, descriptor positions, primary/secondary pairs,
terminal framing, and all candidate/assignment bytes were read directly.

For descriptor ordinal d >= 2 with start D, label start is D+15 and the candidate
is D-24 = label-minus-39 = preceding slice start+0x8e. Assignment candidate is
D-16 = label-minus-31 = preceding slice+0x96. Under the parser's existing slice
boundaries, the byte physically lies in the preceding descriptor slice. Its
association with the following named track is supported by the two distinct
controlled transitions, not by a claim that current slices are complete semantic
track records. Do not shift parser boundaries or attach it to the preceding
track's name. Musical pair ordinal is d-2 only when all counts/bindings validate.

Table classes are conservative observational fingerprints, **not decoded Studio
Vision subtypes**. All share the Descriptor166 container:

- N: unambiguous ordinal pair, nonempty bounded event region, and bytes
  candidate+1..+7 exactly `00 04 00 00 04 01 00` (the controlled local pattern).
- V: unambiguous pair with a different local pattern; raw candidate can be
  inventoried but is outside the proposed semantic subset.
- E: structurally empty event region; no mute semantics assigned by this subset.
- U: unequal descriptor/pair counts; no safe track/pair binding.

“Nonempty” below means bytes in the validated primary event region, not proof
that a complete event walk succeeds or that audible notes exist. Zero-length
regions are established from primary prefix and terminal grammar, not flags.
Unknown mute state is never filled from inclusion or the candidate byte.

## Complete baseline evidence table

Every row is Experiment 007. Sequence headings specify provenance: P is this
project's direct structural/byte evidence. O/B/G/T/Q/K additionally identify an
authenticated reference MIDI below. `in`/`out` refer only to that reference,
not a contemporaneous observation of this saved project's export state.
`?` means unknown. Mute is independently unknown for every baseline row.
Candidate and assignment values are hexadecimal; offsets are baseline absolute.
Nonempty shows event-region bytes (0 = structurally empty, ? = association unsafe).

### xForm — provenance P

| d / track | Class | Candidate @ offset | Mute observed | MIDI | Assignment | Nonempty bytes |
| --- | --- | --- | --- | --- | --- | ---: |
| 2 / Track 1 | N | `80 @ 006cc0` | ? | ? | `01` | 483 |
| 3 / Track 2 | N | `80 @ 006d66` | ? | ? | `02` | 1532 |
| 4 / Track 3 | V | `80 @ 006e0c` | ? | ? | `0d` | 4132 |
| 5 / Track 4 | N | `80 @ 006eb2` | ? | ? | `1c` | 1214 |
| 6 / Track 5 | N | `80 @ 006f58` | ? | ? | `03` | 1759 |
| 7 / Track 6 | N | `80 @ 006ffe` | ? | ? | `07` | 1100 |
| 8 / Track 7 | V | `80 @ 0070a4` | ? | ? | `0d` | 1578 |
| 9 / Track 8 | N | `80 @ 00714a` | ? | ? | `0f` | 564 |
| 10 / Track 9 | N | `80 @ 0071f0` | ? | ? | `01` | 7632 |
| 11 / Track 10 | N | `80 @ 007296` | ? | ? | `05` | 964 |
| 12 / Track 11 | N | `80 @ 00733c` | ? | ? | `1e` | 490 |
| 13 / Track 12 | N | `80 @ 0073e2` | ? | ? | `0a` | 755 |
| 14 / Track 11 #2 | N | `88 @ 007488` | ? | ? | `21` | 513 |

### Bells for her — provenance P/B

| d / track | Class | Candidate @ offset | Mute observed | MIDI | Assignment | Nonempty bytes |
| --- | --- | --- | --- | --- | --- | ---: |
| 2 / Track 1 | N | `80 @ 00e203` | ? | in | `03` | 6413 |
| 3 / Track 2 | N | `88 @ 00e2a9` | ? | out | `04` | 605 |
| 4 / Track 3 | V | `80 @ 00e34f` | ? | in | `23` | 1659 |
| 5 / Track 4 | N | `80 @ 00e3f5` | ? | in | `05` | 1807 |
| 6 / Track 5 | N | `80 @ 00e49b` | ? | in | `06` | 644 |
| 7 / Track 6 | N | `80 @ 00e541` | ? | in | `02` | 1329 |
| 8 / Track 7 | N | `88 @ 00e5e7` | ? | out | `01` | 1167 |
| 9 / Track 8 | V | `80 @ 00e68d` | ? | in | `23` | 6080 |
| 10 / Track 9 | N | `80 @ 00e733` | ? | in | `1f` | 1423 |
| 11 / Track 10 | E | `80 @ 00e7d9` | ? | out | `05` | 0 |
| 12 / Track 11 | N | `80 @ 00e87f` | ? | in | `0b` | 501 |
| 13 / Track 12 | V | `80 @ 00e925` | ? | in | `0d` | 54 |
| 14 / Track 13 | E | `80 @ 00e9cb` | ? | out | `02` | 0 |
| 15 / Track 14 | N | `80 @ 00ea71` | ? | in | `01` | 4270 |

### Situation — provenance P

| d / track | Class | Candidate @ offset | Mute observed | MIDI | Assignment | Nonempty bytes |
| --- | --- | --- | --- | --- | --- | ---: |
| 2 / Track 1 | N | `80 @ 01623f` | ? | ? | `03` | 690 |
| 3 / Track 2 | N | `80 @ 0162e5` | ? | ? | `1c` | 863 |
| 4 / Track 3 | V | `80 @ 01638b` | ? | ? | `0d` | 1092 |
| 5 / Track 4 | V | `80 @ 016431` | ? | ? | `0d` | 441 |
| 6 / Track 5 | N | `80 @ 0164d7` | ? | ? | `02` | 302 |
| 7 / Track 6 | N | `80 @ 01657d` | ? | ? | `01` | 244 |

### Sequence D — provenance P

| d / track | Class | Candidate @ offset | Mute observed | MIDI | Assignment | Nonempty bytes |
| --- | --- | --- | --- | --- | --- | ---: |
| 2 / Track 1 | N | `80 @ 017a12` | ? | ? | `04` | 771 |
| 3 / Track 2 | N | `80 @ 017ab8` | ? | ? | `04` | 796 |
| 4 / Track 3 | V | `80 @ 017b5e` | ? | ? | `0d` | 383 |
| 5 / Track 4 | V | `80 @ 017c04` | ? | ? | `0d` | 805 |
| 6 / Track 5 | N | `80 @ 017caa` | ? | ? | `08` | 755 |
| 7 / Track 6 | E | `80 @ 017d50` | ? | ? | `4d` | 0 |

### Sequence E — provenance P

| d / track | Class | Candidate @ offset | Mute observed | MIDI | Assignment | Nonempty bytes |
| --- | --- | --- | --- | --- | --- | ---: |
| 2 / Track 1 | N | `80 @ 019213` | ? | ? | `03` | 1694 |
| 3 / Track 2 | V | `80 @ 0192b9` | ? | ? | `0d` | 1424 |
| 4 / Track 3 | V | `80 @ 01935f` | ? | ? | `0d` | 635 |
| 5 / Track 4 | V | `80 @ 019405` | ? | ? | `0d` | 229 |
| 6 / Track 5 | E | `80 @ 0194ab` | ? | ? | `0d` | 0 |
| 7 / Track 6 | N | `80 @ 019551` | ? | ? | `08` | 936 |
| 8 / Track 7 | N | `88 @ 0195f7` | ? | ? | `05` | 457 |
| 9 / Track 8 | E | `80 @ 01969d` | ? | ? | `05` | 0 |

### Girl-U-Want — provenance P/G

| d / track | Class | Candidate @ offset | Mute observed | MIDI | Assignment | Nonempty bytes |
| --- | --- | --- | --- | --- | --- | ---: |
| 2 / Track 1 | N | `80 @ 01b2df` | ? | in | `05` | 905 |
| 3 / Track 2 | V | `80 @ 01b385` | ? | in | `0d` | 653 |
| 4 / Track 3 | N | `80 @ 01b42b` | ? | in | `03` | 524 |

### mission impossibl — provenance P

| d / track | Class | Candidate @ offset | Mute observed | MIDI | Assignment | Nonempty bytes |
| --- | --- | --- | --- | --- | --- | ---: |
| 2 / bass | N | `80 @ 01c171` | ? | ? | `03` | 683 |
| 3 / kick drum | V | `80 @ 01c217` | ? | ? | `0d` | 936 |
| 4 / stick thingy | V | `80 @ 01c2bd` | ? | ? | `0d` | 1129 |
| 5 / claps | V | `80 @ 01c363` | ? | ? | `0d` | 239 |
| 6 / Track 1 | N | `80 @ 01c409` | ? | ? | `03` | 488 |
| 7 / Track 2 | V | `80 @ 01c4af` | ? | ? | `0d` | 396 |
| 8 / Track 3 | N | `80 @ 01c555` | ? | ? | `15` | 240 |
| 9 / Track 4 | N | `80 @ 01c5fb` | ? | ? | `14` | 527 |
| 10 / Track 5 | N | `80 @ 01c6a1` | ? | ? | `0b` | 37 |
| 11 / Track 6 | V | `80 @ 01c747` | ? | ? | `0d` | 1572 |

### happyone — provenance P

| d / track | Class | Candidate @ offset | Mute observed | MIDI | Assignment | Nonempty bytes |
| --- | --- | --- | --- | --- | --- | ---: |
| 2 / Track 1 | N | `88 @ 01e91e` | ? | ? | `01` | 3036 |
| 3 / Track 2 | N | `88 @ 01e9c4` | ? | ? | `01` | 2199 |
| 4 / Track 3 | N | `88 @ 01ea6a` | ? | ? | `01` | 261 |
| 5 / Track 4 | N | `80 @ 01eb10` | ? | ? | `1c` | 2158 |
| 6 / Track 5 | E | `88 @ 01ebb6` | ? | ? | `03` | 0 |
| 7 / Track 6 | N | `80 @ 01ec5c` | ? | ? | `04` | 1246 |
| 8 / Track 7 | V | `80 @ 01ed02` | ? | ? | `0d` | 2966 |
| 9 / Track 8 | E | `80 @ 01eda8` | ? | ? | `0d` | 0 |
| 10 / Track 9 | N | `80 @ 01ee4e` | ? | ? | `02` | 168 |
| 11 / Track 9 #2 | N | `80 @ 01eef4` | ? | ? | `06` | 166 |
| 12 / Track 9 #3 | N | `80 @ 01ef9a` | ? | ? | `02` | 140 |

### Sequence I — provenance P

| d / track | Class | Candidate @ offset | Mute observed | MIDI | Assignment | Nonempty bytes |
| --- | --- | --- | --- | --- | --- | ---: |
| 2 / Track 1 | U | `80 @ 0229aa` | ? | ? | `05` | ? |
| 3 / Track 2 | U | `80 @ 022a50` | ? | ? | `0d` | ? |
| 4 / Track 5 | U | `80 @ 022af6` | ? | ? | `0d` | ? |
| 5 / Track 3 | U | `80 @ 022b9c` | ? | ? | `04` | ? |
| 6 / Track 4 | U | `80 @ 022c42` | ? | ? | `06` | ? |
| 7 / Track 6 | U | `88 @ 022ce8` | ? | ? | `08` | ? |
| 8 / Track 7 | U | `80 @ 022d8e` | ? | ? | `09` | ? |
| 9 / Track 8 | U | `88 @ 022e34` | ? | ? | `02` | ? |
| 10 / Track 9 | U | `80 @ 022eda` | ? | ? | `02` | ? |
| 11 / (blank) | U | `00 @ 022f80` | ? | ? | `05` | ? |
| 12 / Track 1 #2 | U | `80 @ 023026` | ? | ? | `05` | ? |

### newsong — provenance P

| d / track | Class | Candidate @ offset | Mute observed | MIDI | Assignment | Nonempty bytes |
| --- | --- | --- | --- | --- | --- | ---: |
| 2 / Track 2 | V | `80 @ 0248c6` | ? | ? | `4c` | 62 |
| 3 / Track 2 | N | `80 @ 02496c` | ? | ? | `01` | 34 |
| 4 / Track 3 | N | `80 @ 024a12` | ? | ? | `4c` | 664 |
| 5 / Track 4 | V | `80 @ 024ab8` | ? | ? | `0d` | 704 |
| 6 / Track 5 | E | `80 @ 024b5e` | ? | ? | `04` | 0 |

### Sequence K — provenance P/K

| d / track | Class | Candidate @ offset | Mute observed | MIDI | Assignment | Nonempty bytes |
| --- | --- | --- | --- | --- | --- | ---: |
| 2 / Track 1 | N | `80 @ 02576c` | ? | in | `01` | 215 |
| 3 / Track 2 | E | `80 @ 025812` | ? | out | `03` | 0 |

### Renaissance — provenance P

| d / track | Class | Candidate @ offset | Mute observed | MIDI | Assignment | Nonempty bytes |
| --- | --- | --- | --- | --- | --- | ---: |
| 2 / Track 1 | N | `80 @ 025d85` | ? | ? | `02` | 964 |
| 3 / Track 2 | N | `80 @ 025e2b` | ? | ? | `02` | 576 |
| 4 / Track 3 | E | `80 @ 025ed1` | ? | ? | `02` | 0 |
| 5 / Track 4 | N | `80 @ 025f77` | ? | ? | `09` | 38 |
| 6 / Track 5 | N | `80 @ 02601d` | ? | ? | `0a` | 164 |
| 7 / Track 6 | E | `80 @ 0260c3` | ? | ? | `0a` | 0 |

### Get on up & Dance — provenance P

| d / track | Class | Candidate @ offset | Mute observed | MIDI | Assignment | Nonempty bytes |
| --- | --- | --- | --- | --- | --- | ---: |
| 2 / Track 1 | N | `88 @ 026d6b` | ? | ? | `11` | 4153 |
| 3 / Track 2 | N | `80 @ 026e11` | ? | ? | `11` | 1558 |
| 4 / Track 2 #2 | N | `80 @ 026eb7` | ? | ? | `10` | 1558 |
| 5 / Track 3 | N | `80 @ 026f5d` | ? | ? | `12` | 3924 |
| 6 / Track 4 | N | `80 @ 027003` | ? | ? | `1c` | 942 |
| 7 / Track 5 | N | `80 @ 0270a9` | ? | ? | `3d` | 1341 |
| 8 / Track 6 | N | `80 @ 02714f` | ? | ? | `3d` | 1920 |
| 9 / Track 7 | N | `80 @ 0271f5` | ? | ? | `3d` | 1854 |
| 10 / Track 8 | N | `80 @ 02729b` | ? | ? | `3d` | 1066 |
| 11 / Track 9 | N | `80 @ 027341` | ? | ? | `02` | 230 |
| 12 / Track 10 | N | `80 @ 0273e7` | ? | ? | `05` | 150 |
| 13 / Track 11 | N | `80 @ 02748d` | ? | ? | `06` | 714 |
| 14 / Track 12 | N | `80 @ 027533` | ? | ? | `03` | 66 |
| 15 / Track 13 | N | `80 @ 0275d9` | ? | ? | `01` | 675 |
| 16 / Track 14 | E | `80 @ 02767f` | ? | ? | `02` | 0 |
| 17 / Track 15 | N | `80 @ 027725` | ? | ? | `1d` | 356 |

### Jurrasic Park — provenance P

| d / track | Class | Candidate @ offset | Mute observed | MIDI | Assignment | Nonempty bytes |
| --- | --- | --- | --- | --- | --- | ---: |
| 2 / Track 1 | V | `88 @ 02d80f` | ? | ? | `0d` | 323 |
| 3 / Track 2 | N | `80 @ 02d8b5` | ? | ? | `04` | 740 |
| 4 / Track 3 | N | `80 @ 02d95b` | ? | ? | `05` | 219 |
| 5 / Track 4 | N | `80 @ 02da01` | ? | ? | `06` | 2396 |
| 6 / Track 5 | E | `80 @ 02daa7` | ? | ? | `07` | 0 |
| 7 / Track 6 | N | `80 @ 02db4d` | ? | ? | `3d` | 127 |
| 8 / Track 7 | E | `80 @ 02dbf3` | ? | ? | `3d` | 0 |

### Ode to Clarke — provenance P/O

| d / track | Class | Candidate @ offset | Mute observed | MIDI | Assignment | Nonempty bytes |
| --- | --- | --- | --- | --- | --- | ---: |
| 2 / Track 1 | N | `88 @ 02f173` | ? | in | `03` | 576 |
| 3 / Track 2 | N | `80 @ 02f219` | ? | in | `05` | 1411 |
| 4 / sys100loops | N | `80 @ 02f2bf` | ? | in | `3d` | 1964 |
| 5 / Track 4 | N | `80 @ 02f365` | ? | in | `3d` | 1134 |
| 6 / Track 5 | N | `80 @ 02f40b` | ? | in | `3d` | 784 |
| 7 / Track 3 | N | `88 @ 02f4b1` | ? | in | `04` | 612 |
| 8 / Track 6 | N | `80 @ 02f557` | ? | in | `3d` | 400 |
| 9 / Track 3 #2 | N | `80 @ 02f5fd` | ? | in | `01` | 632 |
| 10 / Track 7 | V | `80 @ 02f6a3` | ? | in | `0d` | 916 |

### Over the Top — provenance P/T

| d / track | Class | Candidate @ offset | Mute observed | MIDI | Assignment | Nonempty bytes |
| --- | --- | --- | --- | --- | --- | ---: |
| 2 / Track 1 | N | `80 @ 032230` | ? | in | `04` | 734 |
| 3 / Track 2 | N | `80 @ 0322d6` | ? | in | `05` | 196 |
| 4 / Track 3 | N | `80 @ 03237c` | ? | in | `06` | 252 |

### Sequence Q — provenance P/Q

| d / track | Class | Candidate @ offset | Mute observed | MIDI | Assignment | Nonempty bytes |
| --- | --- | --- | --- | --- | --- | ---: |
| 2 / Track 1 | N | `80 @ 032ca1` | ? | in | `15` | 1051 |

### Sequence R — provenance P

| d / track | Class | Candidate @ offset | Mute observed | MIDI | Assignment | Nonempty bytes |
| --- | --- | --- | --- | --- | --- | ---: |
| 2 / Track 1 | N | `80 @ 03349e` | ? | ? | `16` | 260 |
| 3 / Track 2 | N | `80 @ 033544` | ? | ? | `17` | 404 |

## Reference provenance

All reference files were read and their SMF framing/name inventory checked.
Except K, paths below are relative to `Studio Vision MIDI Exports/Project 001/`
under the research root. Their structural manifest association is also recorded
in the six built-in exact compatibility profiles (`src/compatibility_profiles.rs`).

| Code | Reference | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| O | Ode to Clarke Multi All | 12141 | `4f63b34ef92204d4bc5eeb78dbbe7b94d005c1f9ceb57ea0f9809533ad590f29` |
| B | newest STUFF - Bells for her - provenance multitrack | 28120 | `ffbdbb6be208a2d607c9b0c55a12b72226a18d43b9494c2b46b058d4568fc2c3` |
| G | Girl-U-Want - SVP ref | 3082 | `6a5c2b30a2b39c9acdf33f6cc7eebfd742f9022d6f190dacb8d0f8de593944e0` |
| T | Over the Top - SVP ref | 1706 | `b828623da0524ae73e3709adbff4fdb210c2b7f56c483bd0a9ed9288a6c20b70` |
| Q | Sequence Q - provenance multi | 1633 | `e4416ce11296f99077c4b9a21ca9a7aa9a910cffff7be5745267d7a14f728040` |
| K | Controlled Save Experiments/Experiment 030 - Change initial Meter from 4-4 to 7-8/Sequence K 6-8 | 317 | `77cda3e7ceb707f034ead498592f148a3ed211708d5033533429e27b5a9a6db6` |

## Returned-save contexts and independently observed states

All non-Ode rows in the returned 033-036 projects have the same candidate,
assignment, label, event-region length, and observational class as the baseline
table after relocation. Their own contemporaneous MIDI inclusion and mute states
are unknown: the returned MIDIs export Ode only. Do not copy baseline-reference
inclusion into a claim about an unexported sequence in a later save.

The following compact table covers all Ode rows in each returned pair. All
have nonempty events; Track 7 is V (candidate+1 is `04`), the other eight are N.
“Other seven” means Track 2, sys100loops, Track 4, Track 5, Track 6, Track 3 #2,
and Track 7, individually identified in the baseline table. Their assignment
bytes are respectively `05,3d,3d,3d,3d,01,0d`, except the Track 2 edits below.

| Project(s) | Track(s) | Candidate | Independent mute observation | Actual paired MIDI | Assignment |
| --- | --- | --- | --- | --- | --- |
| 033 CTRL | Track 1 / Track 3 | 88 / 88 | ON / ON, later saved-project UI inspection | out / out | 03 / 04 |
| 033 CTRL | Other seven | 80 each | OFF, same later inspection | in each | as above |
| 033 EDIT | Track 1 / Track 3 | 88 / 88 | unknown; do not transfer CTRL screenshot | out / out | 03 / 04 |
| 033 EDIT | Other seven | 80 each | unknown | in each | Track 2 = 06; others unchanged |
| 034 CTRL, EDIT | Track 1 / Track 3 | 88 / 88 | no independent mute-state observation established in reviewed results | out / out in each MIDI | 03 / 04 |
| 034 CTRL, EDIT | Other seven | 80 each | unknown | in each | Track 2 = 05 / 03; others unchanged |
| 035 CTRL | Track 1 / Track 3 | 88 / 88 | ON / ON, owner-reported contemporaneous state | out / out | 03 / 04 |
| 035 CTRL | Other seven | 80 each | OFF, owner-reported | in each | as above |
| 035 EDIT / 036 control | Track 1 / Track 3 | 80 / 88 | OFF / ON, saved/reopened owner verification | in / out | 03 / 04 |
| 035 EDIT / 036 control | Other seven | 80 each | OFF, owner-reported | in each | as above |
| 036 EDIT | Track 1 / Track 3 | 80 / 80 | OFF / OFF, saved/reopened owner verification | in / in | 03 / 04 |
| 036 EDIT | Other seven | 80 each | unchanged OFF per owner intervention report | in each | as above |

Project identities and actual returned paths are in the linked experiment
records. Values were remeasured for every row of every returned project here.
The only baseline-relative candidate changes are Ode Track 1 in 035/036 EDIT
and Ode Track 3 in 036 EDIT; assignment changes are only Track 2 in 033/034 EDIT.
No other sequence acquired an inferred observed mute state through this survey.

## The apparent Ode 88 conflict

**Same position and same evidenced structural class: yes.** Baseline Ode
Tracks 1 and 3 have the same Descriptor166 layout, ordinal binding, nonempty
primary events, and local N fingerprint as the controlled transitions.

| Proposed explanation | Evidence assessment |
| --- | --- |
| Wrong track association | Controlled changes target distinct following labels/pairs while the other candidate stays fixed. Mapping to the preceding named track would misassign Track 1 to Tempo and Track 3 to Track 5. |
| Wrong absolute boundary | 035 and 036 relocate; structure-relative alignment preserves label-minus-39. Absolute-offset comparison would be wrong, but it does not explain the baseline reference. |
| Distinct subtype/class | No decoded subtype or structural difference distinguishes baseline Ode's two 88 rows from the controlled 88 rows. Claiming one would be invented. |
| Bit flags | Both interventions clear exactly bit 0x08 and preserve all other bits in that byte. Only whole-byte values 80 and 88 are experimentally tested. Other combinations remain unknown. |
| Save/version/context | Save metadata changes and relocations are measured; no version change or export override is established. Historical Multi All has no contemporaneous binding to the baseline's displayed mute state. Its procedure is explicitly unresolved. |

Thus there is an **export-provenance/contract limitation, not an evidenced
structural contradiction in the mute transition**. The historical reference
contains Tracks 1 and 3, but it does not establish that the exact saved 88 state
was exported without an intervening UI change. Conversely, it is not legitimate
to assert that historical unmuting occurred. Experiment 036 reproduces that
entire reference with both tracks unmuted; it demonstrates one sufficient
procedure, not the historical procedure.

Describe the measurement as “one-byte 88 -> 80, clearing bit 0x08.” Describe the
interpretation as a candidate mute flag tested only with the other bits fixed
at 0x80. It is observationally indistinguishable from a two-value enum on these
data; a universal bit-mask decoder is not established. Adjacent bytes have not
been shown to be part of a wider mute field or a decoded subtype selector.

## Bells cross-check

Bells Track 2 (d=3, candidate `0x00e2a9`, assignment `04`) and Track 7 (d=8,
candidate `0x00e5e7`, assignment `01`) both have `88`, the same N local pattern,
nonempty event regions (605 and 1167 bytes), and reliable ordinal bindings.
They satisfy the proposed structural/readout conditions of Ode Tracks 1/3.
Their authenticated MIDI omissions corroborate the interpretation, but **their
mute states were not independently observed**. This is not another mute-only
causal replication. Full event evidence for those omissions is retained in the
exact Bells profile; those authenticated omissions remain authoritative.

Bells Tracks 10 and 13 have `80`, structurally empty event regions, and no
corresponding reference MIDI tracks. The exact profile classifies these as
structural-empty omissions; their mute states were not independently observed.
Sequence K Track 2 supplies another `80`/empty omission. These rows do not
establish a causal mute response, and `80` alone cannot require an exported track.

## The 00 case

Sequence I descriptor ordinal 11 has a **blank label**, not an identified musical
track name. Its slice is `0x022f98..0x02303e`, label position `0x022fa7`, candidate
`0x022f80 = 00`, assignment candidate `0x022f88 = 05`. It uses the same 166-byte
physical form but belongs to an unresolved association class: 11 musical slots,
10 primary/secondary pairs. Candidate+6 is also `00`, not the controlled `01`.

The blank row's own event contents, mute state, and MIDI inclusion are unknown.
No authenticated Sequence I MIDI establishes its inclusion or omission. Do not
assume the blank slot is an inactive track, skip it to repair ordinal binding,
or infer empty events from its neighboring byte. **00 must remain unknown and
fail closed**, as must the whole sequence for any operation requiring track/pair
association. It is not evidence of an unmuted or muted alternate encoding.

## Exact proposed narrow subset and failure behavior

1. Validate the existing Descriptor166 grammar from the root stream: bounded
   typed records, 208-byte preamble, 166-byte descriptor stride, derived name
   record, preludes, Meter/Tempo records, complete primary/secondary track pairs,
   terminal record, and exact EOF. No signature scanning or fixed project offset.
2. Require equal musical-descriptor/pair counts, validated ordinal association,
   target d >= 2, and a nonblank bounded label. Labels identify provenance, not
   the substring “Track” or a hard-coded track name. Do not support Sequence I
   by guessing around its blank slot.
3. Derive c = D-24 with checked arithmetic. Require the candidate and local
   seven-byte fingerprint wholly bounded inside the sequence descriptor area.
   Require candidate+1..+7 = `00 04 00 00 04 01 00`, and a separately validated
   nonempty event region. This fingerprint is a deliberately conservative scope
   guard matching both interventions, not proof that those bytes encode a subtype.
4. Read **one byte** at c. Support **exactly 80 = mute OFF, 88 = mute ON within
   this subset**. Report source offset, raw value, structural binding, and scope.
   The observed difference mask is 08; first whitelist 80/88. Never evaluate
   arbitrary x using only `(x & 08)`. Every other value, including 00, and every
   other local fingerprint must yield Unknown, not OFF, ON, Include, or Omit.
5. This result is a bounded saved-state interpretation. It is not an independently
   observed UI state; new tracks remain marked inferred in evidence displays.
   Structurally empty tracks retain their independent empty-track handling and
   are outside this semantic subset. A malformed boundary or incomplete event
   walk must never become safe by declaring the track muted and skipping it.
   Raw state evidence may be displayed with an incomplete walk, but cannot
   authorize omission, successful export, or readiness.
6. Inclusion requires an explicit output contract. In the verified 035/036
   Multitrack/no-other-state-change context, ON correlated causally with omission
   and OFF with inclusion for the two tested nonempty tracks. Project bytes do
   not independently establish all export/solo/Instrument conditions. Generic
   export must remain fail-closed when that contract or routing/events are
   unresolved; no silent auto-omission is authorized by this proposal.
7. Exact-profile inclusion and exclusion policy remains authoritative after its
   existing provenance/manifest checks. Preserve Ode's nine-track reference
   contract, Bells' nonempty exclusions, and structural-empty exclusions.
   A state/profile difference is not grounds to rewrite the profile or reject
   its known reference behavior. No generic fallback may broaden a profile match.

The restriction to the controlled local fingerprint deliberately excludes
candidate+1=04 rows (including Ode Track 7 and Jurrasic Park Track 1), newsong's
candidate+6=02 row, and empty rows. Their raw byte observations stay available;
no meanings are invented for the differing neighboring bytes.

## What the current Partial sequences gain

**No sequence becomes Ready or gains export authorization from this rule alone.**
The generic service still requires authoritative routing, complete event handling,
and a resolved inclusion contract. “Potential mute-state evidence” is not a
readiness claim. Structural prefilter counts below do not certify event walks.

| Current Partial sequence | N-prefilter rows / all slots | 88 rows within that subset |
| --- | ---: | --- |
| xForm | 11 / 13 | Track 11 #2 |
| Situation | 4 / 6 | none |
| Sequence D | 3 / 6 | none |
| Sequence E | 3 / 8 | Track 7 |
| mission impossibl | 5 / 10 | none |
| happyone | 8 / 11 | Tracks 1, 2, 3 |
| Sequence I | 0 / 11 | none safely bound |
| newsong | 2 / 5 | none |
| Renaissance | 4 / 6 | none |
| Get on up & Dance | 15 / 16 | Track 1 |
| Jurrasic Park | 4 / 7 | none; its 88 Track 1 has another fingerprint |
| Sequence R | 2 / 2 | none |

Eleven sequences could gain some scoped state evidence; xForm, Sequence E,
happyone, and Get on up & Dance have in-subset set-bit rows of particular
inclusion interest. Sequence I gains no semantic track result. This review
proposes no further controlled experiment; the next action is owner review of
this state/policy separation and narrow implementation specification.

## Reproducibility and related records

Use the root/descriptor comparison recipe in the Experiment 035 record, adding
EXP36 EDIT as another project input. The candidate is D-24, assignment D-16;
classify only after the current sequence parser's ordinal and event-boundary
rules. Compare MIDI chunks by bound track identity, not filename inference.
The 133-row table is the compact durable result; no large raw-byte annex is added.

- [Experiment 035](CONTROLLED_TRACK1_MUTE_ON_TO_OFF.md)
- [Experiment 036 preregistration and appended results](CONTROLLED_TRACK3_MUTE_ON_TO_OFF.md)
- [Experiment 033](CONTROLLED_TRACK2_INSTRUMENT_ASSIGNMENT_CHANGE.md)
- [Experiment 034](CONTROLLED_TRACK2_INSTRUMENT_ASSIGNMENT_JUNO106.md)
- [Sequence structure and Sequence I limitation](SEQUENCE_CONTAINER_BOUNDARY_CORRELATION.md)
- [Parser contract](ROOT_SEQUENCE_CONTAINER_PARSER_IMPLEMENTATION.md)
- [Exact-profile policy](AUTHENTICATED_COMPATIBILITY_PROFILE_IMPLEMENTATION.md)
