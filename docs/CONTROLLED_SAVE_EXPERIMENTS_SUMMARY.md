# Controlled Save Experiments Summary

This document compares the four results in the external Controlled Save
Experiments collection with their common original project, `newest STUFF`.
Each experiment was performed independently. Phoenix inspected every file
read-only.

The described user actions are experimental context. File sizes, digests,
metadata, byte values, offsets, and printable strings are direct observations.
No observation below assigns an undocumented binary meaning to any byte.

## Method

The data forks were compared byte-for-byte at the same numeric offsets. Because
the original and saved files have different sizes and contain many repeated
byte sequences, this positional comparison does not claim a unique sequence
alignment. A changed-offset count means that the byte in a saved file differs
from the original byte at the same offset.

The no-edit result from Experiment 002 was also compared directly with the
three edited results. Those four files have the same length, making their
same-offset differences useful for separating common save behavior from
output-specific observations.

A printable string is defined mechanically as a maximal run of at least four
bytes in the inclusive ASCII range `0x20`--`0x7e`. Such runs can occur
accidentally in binary data and are not treated as decoded fields.

## Original project

| Measurement | Direct observation |
| --- | --- |
| Filename | `newest STUFF` |
| Size | 203,422 bytes |
| SHA-256 | `7f97918efd8c8756058b72b4cada4a39a1f0f94655aa2982d4e03cfa8a876114` |
| Finder Type / Creator | `MIDS` / `MIDA` |
| Resource fork | Present as `com.apple.ResourceFork`; 286 bytes; SHA-256 `928f342013fc1c0e0ded95c5e46d7bb01f851def540a6498cf8d9534d9c363ca` |
| Printable strings | 11,800 instances; 8,379 distinct values |

The original in Experiment 001 is the common source identified by the
experiment methodology. The other experiment directories contain only their
saved results.

## Experiment measurements

### Experiment 001 — Track Name Change

Context: the track name `Track 7` was changed to `Track 7 TEST`.

| Measurement | Direct observation |
| --- | --- |
| Result filename | `newest STUFF altered` |
| Size | 211,468 bytes; 8,046 bytes larger than the original |
| SHA-256 | `244f69731644b4d7ecc6743986efc5aefffc49adaa2a54d9f11d7028b0e649b6` |
| Finder Type / Creator | `MID2` / `MIDA` |
| Resource fork | No `com.apple.ResourceFork` attribute available |
| Changed original offsets | 190,447 of 203,422; 7,317 contiguous differing runs; first `0x00000001`, last `0x00031a9d` |
| Candidate opening region | 376 bytes differ and 119 are identical; all 495 bytes are present |
| Printable strings | 10,035 instances; 7,031 distinct values |
| Original string multiset difference | 1,977 removed instances and 212 added instances |

Compared with the no-edit save, this result differs at 799 offsets in 400
runs. Twenty offsets in nine runs differ only in this result among the four
saved files. They are listed in the output-specific table below.

The maximal printable string `Track 7 TEST` begins at `0x0002f6ca` and occurs
only in this result. At `0x0002f6d1`--`0x0002f6d5`, this result contains
`20 54 45 53 54` (` TEST`), while all three other saved files contain five
zero bytes. This is a direct byte observation consistent with the experiment
context; the comparison does not identify a containing binary field.

### Experiment 002 — Save As with No Edits

Context: the project was saved under a new filename without project edits.

| Measurement | Direct observation |
| --- | --- |
| Result filename | `newest STUFF no edits` |
| Size | 211,468 bytes; 8,046 bytes larger than the original |
| SHA-256 | `81af90ce487ab05f0e07ab75ad968085841dcf5ce19bc119739fcdde184a2cc5` |
| Finder Type / Creator | `MID2` / `MIDA` |
| Resource fork | No `com.apple.ResourceFork` attribute available |
| Changed original offsets | 190,447 of 203,422; 7,316 contiguous differing runs; first `0x00000001`, last `0x00031a9d` |
| Candidate opening region | 376 bytes differ and 119 are identical; all 495 bytes are present |
| Printable strings | 10,035 instances; 7,030 distinct values |
| Original string multiset difference | 1,976 removed instances and 211 added instances |

Eight individual offsets differ only in this result among the four saved
files. Those offsets are listed below. Calling them “Save As-only” describes
their observed membership in this collection; it does not assign a cause.

### Experiment 003 — Change One Instrument Assignment

Context: one track instrument assignment was changed.

| Measurement | Direct observation |
| --- | --- |
| Result filename | `newest STUFF changed instrument` |
| Size | 211,468 bytes; 8,046 bytes larger than the original |
| SHA-256 | `b8aa42825f6d478bef873a0e1319c667791bf622e0023e06e5ace37e8700e8dd` |
| Finder Type / Creator | `MID2` / `MIDA` |
| Resource fork | No `com.apple.ResourceFork` attribute available |
| Changed original offsets | 190,425 of 203,422; 7,340 contiguous differing runs; first `0x00000001`, last `0x00031a9d` |
| Candidate opening region | 376 bytes differ and 119 are identical; all 495 bytes are present |
| Printable strings | 10,034 instances; 7,029 distinct values |
| Original string multiset difference | 1,977 removed instances and 211 added instances |

Compared with the no-edit save, this result differs at 1,313 offsets in 613
runs. Nine offsets in eight runs differ only in this result among the four
saved files. The printable scanner reports `oBnd` removed once and `8oBnd`
added once relative to the no-edit result. These literal runs are not assigned
instrument or record semantics.

### Experiment 004 — Change One Tempo Value

Context: the tempo was changed from 120 BPM to 130 BPM.

| Measurement | Direct observation |
| --- | --- |
| Result filename | `newest STUFF tempo to 130` |
| Size | 211,468 bytes; 8,046 bytes larger than the original |
| SHA-256 | `ba1a99a058f7c6ad50f5a559b518e0f72bfa99660bc199e5dbfd8f31e46ee633` |
| Finder Type / Creator | `MID2` / `MIDA` |
| Resource fork | No `com.apple.ResourceFork` attribute available |
| Changed original offsets | 190,424 of 203,422; 7,338 contiguous differing runs; first `0x00000001`, last `0x00031a9d` |
| Candidate opening region | 376 bytes differ and 119 are identical; all 495 bytes are present |
| Printable strings | 10,035 instances; 7,030 distinct values |
| Original string multiset difference | 1,976 removed instances and 211 added instances |

Compared with the no-edit save, this result differs at 1,246 offsets in 613
runs. Eleven offsets in nine runs differ only in this result among the four
saved files. No user-facing tempo string was identified by the mechanical
printable scan.

## Inserted and deleted bytes

Every saved result is exactly 8,046 bytes larger than the original. In a
strict positional comparison, `0x00031a9e`--`0x00033a0b` exists in every
saved result after the original has reached end of file.

This does not prove a single 8,046-byte insertion or the absence of deletions
elsewhere. The broad rewrite and repeated byte sequences permit multiple valid
alignments. No specific inserted or deleted structure is therefore claimed.

## Candidate opening region

The documented candidate opening region is `0x0000000e`--`0x000001fc`,
inclusive. All four saved results are byte-identical to one another throughout
this region. Relative to the original, the same 376 offsets differ in every
result, in these 34 inclusive runs:

```text
0x00000e-0x00002c  0x00002f-0x000030  0x00003a-0x000040
0x000042-0x000059  0x00005c-0x00005d  0x000067-0x000086
0x000088-0x00008a  0x000094            0x000096-0x0000b3
0x0000b5-0x0000b9  0x0000c1            0x0000c3
0x0000c5-0x0000e0  0x0000e2-0x0000e6  0x0000ee
0x0000f0-0x00010d  0x00010f-0x000112  0x00011b
0x00011f-0x00013a  0x00013c-0x00013f  0x000148-0x000167
0x000169-0x00016b  0x00016d            0x000175-0x000194
0x000196-0x000199  0x0001a2-0x0001ae  0x0001b2-0x0001c1
0x0001c3-0x0001c5  0x0001cf            0x0001d1-0x0001de
0x0001e6-0x0001ee  0x0001f0-0x0001f4  0x0001f7
0x0001fb-0x0001fc
```

The other 119 candidate-region offsets are identical across the original and
all four results. The candidate-region evidence therefore separates a common
save-related observation from the three edit contexts, but does not explain
why the save changes these bytes.

## Output-specific byte observations

This table lists every offset at which one saved result differs while the
other three saved results contain the same byte or bytes. Ranges are inclusive.
These are output-specific observations, not decoded edit fields.

| Result | Offset or range | Result bytes | Other three saved files |
| --- | --- | --- | --- |
| Save As, no edits | `0x00006ab1` | `97` | `83` |
| Save As, no edits | `0x00006ab3` | `1e` | `02` |
| Save As, no edits | `0x00006ab5` | `7a` | `66` |
| Save As, no edits | `0x00006ab7` | `4c` | `30` |
| Save As, no edits | `0x0001c7ae` | `31` | `32` |
| Save As, no edits | `0x0002cc53` | `b2` | `be` |
| Save As, no edits | `0x0002dc59` | `b2` | `be` |
| Save As, no edits | `0x000336be` | `43` | `36` |
| Track name | `0x00006b04` | `b2` | `be` |
| Track name | `0x00006bda` | `b2` | `be` |
| Track name | `0x0001ef5b` | `31` | `32` |
| Track name | `0x00023e9b` | `33` | `31` |
| Track name | `0x0002ef8f` | `b2` | `be` |
| Track name | `0x0002efa8` | `03` | `02` |
| Track name | `0x0002f6d1`--`0x0002f6d5` | `20 54 45 53 54` | `00 00 00 00 00` |
| Track name | `0x000332c3` | `00` | `fe` |
| Track name | `0x000332cd`--`0x000332d4` | `01 34 01 03 02 09 03 7e` | `00 e2 00 6c 01 b7 02 e7` |
| Instrument assignment | `0x0000e64d` | `b2` | `be` |
| Instrument assignment | `0x000161ff` | `b2` | `be` |
| Instrument assignment | `0x0001e8df` | `31` | `32` |
| Instrument assignment | `0x0002f325` | `67` | `be` |
| Instrument assignment | `0x0002f605` | `02` | `01` |
| Instrument assignment | `0x0002f766` | `02` | `01` |
| Instrument assignment | `0x0002f7cf` | `02` | `01` |
| Instrument assignment | `0x00032018`--`0x00032019` | `00 92` | `ff ff` |
| Tempo | `0x000106df` | `67` | `be` |
| Tempo | `0x0001c862` | `31` | `32` |
| Tempo | `0x0001d714` | `31` | `32` |
| Tempo | `0x0001e839` | `31` | `32` |
| Tempo | `0x000269f3` | `b2` | `be` |
| Tempo | `0x0002a9c8` | `b2` | `be` |
| Tempo | `0x0002f7e1`--`0x0002f7e2` | `0a e2` | `a1 20` |
| Tempo | `0x0002f81b`--`0x0002f81c` | `0a e2` | `a1 20` |
| Tempo | `0x0003166b` | `b2` | `be` |

Many additional offsets differ between each edited result and the no-edit
result but are not unique to one output. In particular, 680 offsets differ
from the no-edit result in all three edited results; the three edited byte
values are not necessarily equal at those offsets. This prevents treating all
799, 1,313, or 1,246 pairwise differences as specific representations of the
described edits.

## Printable-string comparison

Relative to the no-edit result, the mechanical string inventories differ as
follows:

| Result | Removed instances | Added instances | Literal differences |
| --- | ---: | ---: | --- |
| Track name | 2 | 2 | Removed `Track 7` and `6h,ia`; added `Track 7 TEST` and `b,,ia` |
| Instrument assignment | 2 | 1 | Removed `oBnd` and `6h,ia`; added `8oBnd` |
| Tempo | 1 | 1 | Removed `6h,ia`; added `\\,ia` |

`6h,ia`, `b,,ia`, `8oBnd`, and `\\,ia` are reported exactly as the
printable scanner observed them. Their appearance does not establish that they
are intentional text. The `Track 7` multiset count decreases by one only in
Experiment 001, while `Track 7 TEST` appears only there.

## Comparative findings

### Direct observations common to all four saves

- Every result is 211,468 bytes, 8,046 bytes larger than the original.
- Every result has Finder Type `MID2` and Creator `MIDA`; the original is
  `MIDS` / `MIDA`.
- No result exposes a `com.apple.ResourceFork` attribute; the original does.
- All four results differ from the original at 190,413 shared offsets,
  although their replacement bytes are not always equal.
- At 189,220 shared offsets, every result contains the same byte and that byte
  differs from the original. Those offsets form 7,770 runs.
- The four results are mutually identical at 202,187 of the 203,422 offsets
  covered by the original and at 7,922 of the 8,046 later offsets.
- Their complete candidate opening regions are mutually byte-identical.

The largest run in which every saved result contains the same bytes and those
bytes differ from the original is 1,687 bytes at
`0x0000563c`--`0x00005cd2`. Other large examples are 1,129 bytes at
`0x00019f9b`--`0x0001a403` and 1,051 bytes at
`0x00012aa1`--`0x00012ebb`.

### Experiment-specific direct observations

- Experiment 002 is the sole outlier at the eight Save As rows in the
  output-specific table.
- Experiment 001 is the sole outlier at 20 offsets in nine runs, including
  the five bytes spelling ` TEST` after `Track 7`.
- Experiment 003 is the sole outlier at nine offsets in eight runs.
- Experiment 004 is the sole outlier at 11 offsets in nine runs.
- When each result is compared directly with the original, only three offsets
  differ exclusively in Experiment 001 while the other three results equal
  the original: `0x0002f6d1`--`0x0002f6d3`. No offset meets that narrower
  criterion for Experiments 002, 003, or 004.

### Regions identical across every file

Within the original file's length, 12,967 same-offset bytes are identical in
the original and all four results, distributed among 7,312 runs. The largest
such run is 57 bytes at `0x00001972`--`0x000019aa`; the next largest are 39
bytes at `0x0000169c`--`0x000016c2` and 36 bytes at
`0x000018b8`--`0x000018db`.

These are positional equality observations. They do not establish logical
structure boundaries.

### Regions rewritten regardless of edit

The 189,220 positions where all four saved results share a replacement byte
different from the original are the strongest direct evidence for broadly
repeatable save-related rewriting in this collection. The candidate opening
region is a bounded example: all 376 changed positions have identical saved
values across the four results.

“Save-related” is a comparative description, not an internal-format
interpretation. More no-edit repetitions are needed to determine whether every
observed replacement is deterministic across separate saves.

## Hypotheses requiring further experiments

- The bytes shared by all four results may reflect deterministic serialization
  performed by StudioVision PPC when saving this source project. Repeated
  no-edit saves are required to test repeatability.
- Output-specific offsets near the literal `Track 7 TEST` may be associated
  with the contextual track-name edit. Only the literal bytes themselves are
  directly identifiable; nearby differences require controlled replication.
- Some widespread differences between saved results may be indirect
  consequences of serialization, ordering, generated values, or application
  state rather than representations of the user-visible edits. The present
  files do not distinguish these possibilities.

## Explicit unknowns

- The comparison does not establish binary field meanings, record sizes,
  pointers, counts, checksums, timestamps, identifiers, or structure
  boundaries.
- It does not establish a unique inserted/deleted-byte alignment between the
  original and saved files.
- It does not establish why the Finder Type changes or why the saved files do
  not expose the original resource-fork attribute.
- It does not establish whether any metadata difference arose during saving,
  transfer, or preservation of the collection.
- It does not establish why independently saved results differ at hundreds of
  offsets even when compared with the no-edit result.
- It does not establish which output-specific instrument or tempo bytes, if
  any, directly encode the contextual user-visible change.
- Additional repeated Save As controls and repeated instances of each
  one-variable edit are required to distinguish stable edit-correlated changes
  from run-to-run save variation.
