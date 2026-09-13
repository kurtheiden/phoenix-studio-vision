# Prologue Master post-freeze research checkpoint

## Status, authority, and scope

This documentation-only checkpoint preserves the completed research following
`496a15a578c45369203e163fe486fe5d1d87c478` (`Document Steve Horowitz project
provenance`). See the [Steve provenance clarification](OS1_STEVE_HOROWITZ_PROVENANCE_CLARIFICATION.md).

Byte observations below come from read-only post-freeze analysis. Authentic
Studio Vision observations are project-owner reports from the running
application, including the latest inventory, Finder readback, and References
clarifications. Interpretations are distinguished from those observations.
Private testing is authorized; public redistribution is not. Steve's later
ChatGPT-generated MIDI derivatives remain uninspected and are not ground truth.
Neither Steve's terminology nor ChatGPT's historical format claim establishes
format identity; actual Studio Vision opening now provides independent
application evidence.

No Phoenix change, candidate change, reference export, or new experiment is
performed by this checkpoint. No 120-byte support is authorized here.

## Candidate identity and frozen blind observation

- Filename: `Prologue Master`.
- Data-fork size: **33,057 bytes**.
- SHA-256: `b3a3efd8179d3ffe2a5026b3a40eab74da3ce6e674bb06162a176df07389b733`.
- Private blind intake: `/private/tmp/phoenix-prologue-master-blind-xfbff4lj/Prologue Master`.
- Frozen observation directory: `/private/tmp/phoenix-prologue-observation-ffu4zojf`.
- Observation implementation: unchanged Phoenix 0.1.0 at the commit above,
  invoked once through `phoenix::json_transport::dispatch_json`, using
  `AppService::new()` and public `inspect_project` (contract 1, full diagnostics).

The first independent, reference-blind Phoenix response reported:

- label `Unrecognized`, recognized `false`, confidence `Unknown`;
- no accepted structural parse or recognized profile;
- zero sequences, overall readiness `unknown`, zero Ready/export-capable sequences;
- no export requested or performed and no MIDI output;
- caller exit status 0 and empty runtime stderr.

The response contained caution `unsupported_project_profile` and this detail:

```text
no established Studio Vision profile accepted the input: malformed 166-profile sequence candidate record 23 at 0x0000064e..0x000008f1: InvalidSequenceNameBounds { length_offset: 2637, declared: 36, containing_range: 2289..2304, derived_end: 2674 }
```

Frozen evidence anchors:

| Artifact | SHA-256 |
|---|---|
| `response.json` | `056ff684319c8e57ae32c2c4f6d5dc0cdead22beaed59b19fb2e3b603cb65f94` |
| `freeze.txt` | `a1c34abf04aaf6f0e88091a1b18f63422f144799499cad962d26d5de95629975` |
| `SHA256SUMS` | `1123d154216706885405671956034337847140b838b4d55f982ae95d967fdade` |

The private directory also preserves the exact request, stderr, caller,
invocation/build identity, candidate identity, and artifact hashes. These are
temporary private paths, not repository-hosted artifacts or a permanent backup.
Preserve the frozen observation as valid and immutable. No reference information
was revealed before freeze, no candidate-specific change was made, and Phoenix
was not rerun after its first result. Later evidence does not rewrite that result.

## Post-freeze structural observations

The eight-byte header is `00 1b 00 03 00 46 00 78`. A checked walk from offset 8
using `type:u8 | payload_length:u32 big-endian | payload` consumes **224 records
exactly to EOF**. Fifteen records have type `0x01`; each satisfies:

```text
total record bytes = 75 + 120 × byte_at(record_start + 5)
```

The count ranges from 3 to 11. This is observed geometry, not a complete semantic
descriptor definition. Meter Track and Tempo Track text recur at 120-byte
spacing. The following inventory uses one-based object ordinals and half-open
hexadecimal byte ranges; it does not label every object a Sequence.

| Object | Range | Total bytes | Count | Embedded name |
|---:|---|---:|---:|---|
| 1 | `064e..08f1` | 675 | 5 | Frame 1 |
| 2 | `0e80..1213` | 915 | 7 | Frames 2-4 |
| 3 | `17b6..1bc1` | 1,035 | 8 | Frames 5-6 |
| 4 | `3189..36fc` | 1,395 | 11 | Frames 7-8 |
| 5 | `3c4c..3eef` | 675 | 5 | Frames 9-10 |
| 6 | `426d..4768` | 1,275 | 10 | Frame 11 |
| 7 | `5846..5cc9` | 1,155 | 9 | Frame 12 |
| 8 | `6004..61b7` | 435 | 3 | Prologue |
| 9 | `6307..65aa` | 675 | 5 | Frame 1 |
| 10 | `65e6..6979` | 915 | 7 | Frames 2-4 |
| 11 | `69d1..6ddc` | 1,035 | 8 | Frames 5-6 |
| 12 | `6e2e..73a1` | 1,395 | 11 | Frames 7-8 |
| 13 | `73f2..7695` | 675 | 5 | Frames 9-10 |
| 14 | `76d1..7bcc` | 1,275 | 10 | Frame 11 |
| 15 | `7c37..80ba` | 1,155 | 9 | Frame 12 |

Objects 1-7 have associated following `0x02/0x29` payload records. Objects 9-15
lack corresponding local `0x02/0x29` records, while retaining matching names,
labels, and comment material. The groups also differ systematically at object
`+41` (`fe` versus `ff`) and `+43..+46` (zero versus `00 1f 5e 98`). The meanings
of those fields, including any pointer/reference interpretation, remain unknown.

Object 8 has Meter Track, Tempo Track, and Prologue labels. Its following
payload includes seven repeated `ff 31 03` patterns containing values `0000`
through `0006`, then a distinctive type-`0x05` boundary before the second group.
These distinctions motivate reference/coordination hypotheses but establish no
object class or event-tag semantics.

### Why the frozen Descriptor166 interpretation failed

The first type-`0x01` record is `0x064e..0x08f1`, count 5, total size 675.
Descriptor166 derives its name location as `1614 + 208 + 5*166 - 15 = 2637`
(`0x0a4d`). That offset is outside the following type-`0x07` record
`0x08f1..0x0900`; it actually lies inside a later type-`0x02` payload
(`0x09b7..0x0b2f`). Its byte `24` was interpreted as length 36.

Thus the diagnostic reflects incompatible layout assumptions, not demonstrated
corruption of an actual sequence-name field. The coherent framing and repeated
120-byte geometry differ from the supported 208-byte-preamble/166-byte model.
Committed [root-parser evidence](ROOT_SEQUENCE_CONTAINER_PARSER_DESIGN.md) also
records an older authentic 120-byte form. Shared framing does not establish
identical object semantics, and replacing a stride is insufficient for support.

## FINDER.DAT and historical metadata separation

`FINDER.DAT` is **920 bytes**, consisting of ten 92-byte entries. SHA-256:
`fe653f87b60bdae1326e0f54b07cbcbe918edde2df487927a7afb7e4506cc482`.

Matching length-prefixed family filenames, Type/Creator fields, and DOS-style
short names establish separately preserved classic Macintosh metadata. The
layout supports a PC Exchange/FAT-style transfer hypothesis; the exact transfer
history is not established. Prior ZIP enumeration found no AppleDouble
companions for the ten non-MIDI source names.

| Associated files | Type | Creator |
|---|---|---|
| Prologue Master | `MID2` | `MIDI` |
| Seven frame files and Prologue 1.1 Seq J | `Midi` | `MIDI` |
| Mark miller Sequence list | `WDBN` | `MSWD` |

The family is heterogeneous. These case-sensitive values must not be normalized;
in particular, **`MIDI` is not `MIDA`**. Metadata on other original entries does
not validate their data formats or the later ChatGPT derivatives.

## Controlled metadata restoration and authentic opening

A new disposable copy was created at
`~/Documents/Phoenix Research/Prologue Master Metadata Test/Prologue Master`.
Its data fork was verified identical before and after assigning only the
recovered Type `MID2` and Creator `MIDI`. Size remained 33,057 bytes and SHA-256
remained the candidate hash above. No resource fork was introduced. The original,
private blind intake, and frozen observation were left unchanged.

The owner subsequently reported classic Finder AppleScript readback:

```text
Path:    Unix:Desktop Folder:Prologue Master
Type:    MID2
Creator: MIDI
```

**Configuration clarification:** the SheepShaver startup disk itself is named
`Unix`. This path is on the native guest startup disk; its name is not evidence
that this file was opened from the host Shared/extfs filesystem. Earlier
planning in the conversation treated the volume label as shared-filesystem
evidence; that inference is superseded by this owner clarification. The generic
icon alone never established metadata loss at the transfer boundary.

Before restoration, Studio Vision Pro File -> Open did not normally display/
select the source, and dragging it onto the running application did not open it.
After exact `MID2`/`MIDI` restoration, File -> Open displayed and selected the
file, and Studio Vision successfully opened/parsed the project.

The normal Undefined Channels dialog appeared because referenced Sound Canvas
channels were absent from the current Studio Setup. The owner chose Cancel,
their established workflow for avoiding remapping and preserving original
assignments. This records the owner's workflow, not an independently measured
assignment-preservation experiment.

The unchanged data fork with its recovered metadata is therefore **authentic
Studio Vision-readable material**. This application evidence is independent of
Steve's belief and the historical ChatGPT claim. Exact originating version and
complete musical correctness are not established by opening alone.

## Authenticated visible object inventory

The owner confirmed exactly seven Sequences, in this order:

1. Frame 1
2. Frames 2-4
3. Frames 5-6
4. Frames 7-8
5. Frames 9-10
6. Frame 11
7. Frame 12

Frame 12 is the final Sequence. Segments contains exactly seven visible objects
with the same names. Templates contains zero visible objects: changing its
disclosure state revealed none.

There is no ordinary visible Sequence, Segment, or Template named Prologue.
The unmatched serialized name must not imply a special Studio Vision semantic
role. The owner notes that the cues were likely for a video game's prologue;
the name is likely content/project nomenclature. Object 8's role remains
unresolved; Templates does not explain it.

## Frames 9-10: Sequence and Segment observations

Opening `Sequences -> Frames 9-10` showed meter 4/4, tempo 102.00, Seq Len 10,
three tracks, Sound Canvas instrument assignments visibly ending `-3`, `-4`,
`-5`, matching comments, and visible musical phrase data.

Opening `Segments -> Frames 9-10` produced the normal sequence-style editor,
title Frames 9-10, with the same apparent material and properties. It also
explicitly showed Display Phrases and Silence 1 bar, with phrase data on all
three tracks.

| Track | Visible instrument suffix | Comment |
|---|---|---|
| Low Strings | `-3` | `#97` |
| Middle Strings | `-4` | `#56-24` |
| High Strings | `-5` | `#52+24` |

Instrument text is truncated; do not reconstruct exact full strings from it.
**Segment and Sequence expose the same apparent musical material: YES.**
This does not establish identical serialization, independent duplicate data,
reference direction, or which serialized group represents which UI category.

## Byte/UI correlation and References observation

Each authenticated frame name occurs exactly twice among the type-`0x01`
records; both groups preserve UI order. Frames 9-10 objects 5 and 13 both contain
the three track labels and have the three matching comment payloads in following
type-`0x07` records. Corresponding element values `00 03`, `00 04`, `00 05`
support the observed channel suffixes, without establishing the complete
instrument-assignment encoding.

Across both seven-name groups, object `+70` values `03 05 02 08 22 0e 04`
correlate with authenticated keys F G D C I E H. This strongly supports a
keyboard-assignment correlation but does not distinguish the groups. Numerical
matches for length alone do not establish field semantics. Known meter/tempo
patterns elsewhere in the file do not establish object-specific linkage or
inheritance for Frames 9-10.

The owner reports project-window columns Name, Key, MIDI, References, Comments.
The MIDI column visibly showed `(none)`. With `Segments -> Frames 9-10` selected,
its References cell was blank; References also appeared blank for all seven
visible Segments and all seven visible Sequences. This does **not** establish
that internal sharing or reference relationships are absent.

## Current interpretation and stopping point

- Authentic Studio Vision opening establishes readability of this unchanged
  data fork with recovered Finder metadata. Phoenix's frozen rejection therefore
  exposes a genuine cross-project limitation; retain it as valid evidence.
- Descriptor166 does not describe this coherent 120-byte layout. That mismatch
  is not evidence of artifact corruption.
- Two structurally distinct seven-name groups correlate strongly with the seven
  Sequences and seven Segments, but exact category-to-group assignment remains
  **UNRESOLVED**. Either direction and other shared/reference serialization
  relationships remain hypotheses.
- The unmatched record named Prologue has no ordinary visible category
  counterpart. Its role is **UNRESOLVED**; its name supplies no technical role.
- Empty Templates and blank visible References do not resolve the object model.
- The 120-byte semantics are **not sufficient for implementation**. No parser,
  profile, recognition, or SHA-gate change is authorized by these findings.

We stop before choosing or performing another experiment. A possible next step
is to assess whether a tightly controlled save/diff experiment on a disposable
copy is now the highest-information way to distinguish the two groups. This
checkpoint neither designs nor performs that experiment and does not authorize
editing/saving the current project.

The original, blind intake, frozen evidence, production code, tests, profiles,
and four protected unrelated local files remain outside this documentation
patch. No generated MIDI derivative was inspected or used as ground truth.
