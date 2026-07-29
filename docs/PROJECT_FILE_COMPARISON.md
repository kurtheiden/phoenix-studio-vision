# Studio Vision Project File Comparison

This document compares two authentic Studio Vision project files available in
the local, ignored `samples/` workspace. It records byte-level and filesystem
observations only. Offset ranges are measurement results, not inferred
sections, records, fields, or parser structures.

## Evidence sources and method

- Sample A: `samples/New Dance Tracks`.
- Sample B: `samples/New Dance Tracks 2`.
- Both files were read without modification.
- File sizes and timestamps were obtained from the local filesystem; SHA-256
  values cover each complete data fork.
- FinderInfo and quarantine values were read from extended attributes.
- Entropy is Shannon entropy over byte values.
- Window summaries use fixed, nonoverlapping 4,096-byte windows beginning at
  offset zero. Final windows are shorter.
- Printable strings are maximal runs of bytes from `0x20` through `0x7e`, with
  a minimum length of four bytes. Printable-string byte shares count only
  bytes in such runs.
- Repetition counts use every overlapping occurrence of each exact 16-byte
  sequence.
- Cross-file matching used a bytewise sequence comparison with automatic
  suppression of frequent values disabled. The reported ranges are exact,
  contiguous matches selected by that comparison. Approximate matching ranges
  describe their locations, not approximate byte equality.

## File and Finder metadata

| Observation | Sample A | Sample B |
| --- | --- | --- |
| Filename | `New Dance Tracks` | `New Dance Tracks 2` |
| Filesystem object | Regular file | Regular file |
| Filename extension | None | None |
| Logical/data-fork size | 16,487 bytes (`0x4067`) | 62,238 bytes (`0xf31e`) |
| Last data-fork offset | `0x4066` | `0xf31d` |
| SHA-256 | `b92d7b92fc67b05a04a5f93077b11696ae29cde76bdeced1ba1920cbbd73ca36` | `e2394569e865dd89c16138c5733eb2b86255eff1ef914ca5f54c6b8761743dd9` |
| Creation time | `1997-03-30 03:09:00 +0100` | `1998-09-25 07:52:21 +0100` |
| Modification time | `1997-03-30 03:09:00 +0100` | `1998-09-25 07:52:21 +0100` |
| Observed mode | `-rwx------` | `-rwx------` |
| Finder Type | `MID2` | `MID2` |
| Finder Creator | `MIDA` | `MIDA` |

Both files have a 32-byte `com.apple.FinderInfo` extended attribute. Their
complete observed values are:

```text
Sample A:
4d 49 44 32 4d 49 44 41 01 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00 fa d0 87 bc 00 00 00 00

Sample B:
4d 49 44 32 4d 49 44 41 01 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00 fd 9d fa 25 00 00 00 00
```

The first 24 bytes and final four bytes are equal. Bytes `0x18`--`0x1b` of
the attributes differ. This comparison does not assign meanings to those
bytes beyond the directly decoded Type and Creator values.

Both files also have a 15-byte `com.apple.quarantine` attribute with the same
value, printable as `0081;00000000;;`. Neither inspected path returned a
`com.apple.ResourceFork` extended attribute.

## Entropy and byte-characteristic summaries

### Whole-file comparison

| Observation | Sample A | Sample B |
| --- | ---: | ---: |
| Shannon entropy | 5.004089 bits/byte | 6.044894 bits/byte |
| Zero bytes | 6,190 (37.54%) | 12,003 (19.29%) |
| Printable runs | 538 | 3,010 |
| Bytes in printable runs | 3,576 (21.69%) | 16,313 (26.21%) |
| Longest printable run | 124 bytes at `0x002ae3` | 124 bytes at `0x0032b4` |

The longest printable run is byte-for-byte equal in the two files. It begins
`MU@xxRa@xxMU@xxJ[` and contains punctuation and letters. Its printability is
the only observation made here.

### Fixed-window comparison

| Offset range | Sample A entropy | Sample A printable | Sample A zeros | Sample B entropy | Sample B printable | Sample B zeros |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `0x000000`--`0x000fff` | 2.9998 | 4.69% | 62.40% | 2.8957 | 5.76% | 64.16% |
| `0x001000`--`0x001fff` | 3.7713 | 18.82% | 57.25% | 3.7633 | 20.78% | 57.64% |
| `0x002000`--`0x002fff` | 5.7156 | 35.38% | 14.45% | 5.4211 | 17.77% | 18.21% |
| `0x003000`--`0x003fff` | 5.6644 | 28.00% | 16.28% | 5.4437 | 53.25% | 7.20% |
| Sample A final: `0x004000`--`0x004066` | 4.1862 | 11.65% | 29.13% | n/a | n/a | n/a |
| Sample B `0x004000`--`0x004fff` | n/a | n/a | n/a | 5.8348 | 14.94% | 10.64% |
| Sample B `0x005000`--`0x00dfff` | n/a | n/a | n/a | 5.4242--6.3735 | 14.94%--41.58% | 5.35%--22.19% |
| Sample B `0x00e000`--`0x00efff` | n/a | n/a | n/a | 5.5059 | 0.24% | 3.93% |
| Sample B final: `0x00f000`--`0x00f31d` | n/a | n/a | n/a | 3.2877 | 2.76% | 51.88% |

The combined Sample B range in the table reports the minimum and maximum of
the individual 4 KiB measurements from `0x004000` through `0x00dfff`; it is
not an entropy calculation over the combined bytes.

## Printable-string comparison

The files have 279 distinct printable strings in common under the stated scan
rule. Directly observed examples include:

| String | Sample A offset(s) | Sample B offset(s) |
| --- | --- | --- |
| `KURT SV3.0 Bundle 12-16-95` | `0x000fe4`, `0x0010be` | `0x000fb7`, `0x001089` |
| `General MIDI` | `0x001111`, `0x0011df` | `0x0010dc`, `0x0011aa` |
| `Factory Names` | `0x001178` | `0x001143` |
| `JD-800` | `0x0012b0`, `0x001322` | `0x0011c1`, `0x001233` |
| `Roland` | `0x00130a` and later offsets | `0x00121b` and later offsets |
| `Meter Track` | present | present |
| `Tempo Track` | present | present |
| `Track 1` | present | present |
| `Studio Patches` | present | present |
| `Stereoww Bs` | present | present |

Representative strings observed only in Sample A include:

- `Studio Vision Pro` at `0x00104b`.
- `Hard Disk 2149` at `0x000fbc`.
- `Macintosh HD` at `0x0010e9`.
- `IAC Bus #1` at `0x0011f6` and `0x001268`.
- `Quicktime Music` at `0x000149`.

Representative strings observed only in Sample B include:

- `Galaxy Plus Editors` at `0x00101e`.
- `System Drive` at `0x0010b4`.
- `JD-990 w/Vintage` at `0x00003b` and `0x00127b`.
- `QuickTime Music` at a later scanned offset; its capitalization differs
  from Sample A's `Quicktime Music`.
- `keep on keepin'` at a later scanned offset.

Some early printable runs combine adjacent letters and punctuation differently
between files. For example, Sample A has `IAC Bus #1>` at `0x00000e`, while
Sample B has `JD-800H` at `0x00000e`. The comparison does not treat either run
as a decoded field.

## Repeated binary regions

The most frequent exact 16-byte sequence in each file is 16 zero bytes. With
overlapping occurrences counted, it occurs 1,517 times in Sample A and 1,666
times in Sample B. Early observed starts are `0x0001fc` in Sample A and
`0x0001cf` in Sample B.

Sixteen `ff` bytes occur 138 times in Sample A and 754 times in Sample B under
the same overlapping count. Early observed starts are `0x001e6a` in Sample A
and `0x001ef6` in Sample B.

The following exact 16-byte sequences recur at regularly spaced starts in
both files:

| Sequence | Count in A | Early A starts | Count in B | Early B starts |
| --- | ---: | --- | ---: | --- |
| `24 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00` | 85 | `0x01fb`, `0x0224`, `0x024d` | 85 | `0x01ce`, `0x01f7`, `0x0220` |
| `10 00 00 00 24 00 00 00 00 00 00 00 00 00 00 00` | 85 | `0x01f7`, `0x0220`, `0x0249` | 85 | `0x01ca`, `0x01f3`, `0x021c` |
| `00 24 00 00 00 00 00 00 00 00 00 00 00 00 00 00` | 85 | `0x01fa`, `0x0223`, `0x024c` | 85 | `0x01cd`, `0x01f6`, `0x021f` |

Consecutive starts in these examples differ by `0x29` bytes within each file.
The Sample B starts shown are `0x2d` bytes earlier than the corresponding
Sample A starts. These are spacing and offset observations only.

## Approximate matching offset ranges

The cross-file comparison selected 14,543 bytes among its exact contiguous
matching blocks. Its sequence similarity ratio was 0.369463. That ratio is a
description of this comparison run, not a percentage of a shared file
structure; frequent zero and `ff` bytes can contribute matches.

The largest selected exact matching ranges were:

| Sample A range | Sample B range | Exact length |
| --- | --- | ---: |
| `0x002ebb`--`0x0031fe` | `0x003c19`--`0x003f5c` | 836 bytes |
| `0x001a74`--`0x001d72` | `0x001af9`--`0x001df7` | 767 bytes |
| `0x003bb6`--`0x003df7` | `0x00631a`--`0x00655b` | 578 bytes |
| `0x002be5`--`0x002e22` | `0x0034b0`--`0x0036ed` | 574 bytes |
| `0x003e4d`--`0x004024` | `0x006906`--`0x006add` | 472 bytes |
| `0x003284`--`0x0033be` | `0x004415`--`0x00454f` | 315 bytes |
| `0x000d56`--`0x000e64` | `0x000d29`--`0x000e37` | 271 bytes |
| `0x00035b`--`0x000424` | `0x00032e`--`0x0003f7` | 202 bytes |
| `0x000ec7`--`0x000f83` | `0x000e9a`--`0x000f56` | 189 bytes |

Additional selected exact matches of 40 bytes recur throughout the early
ranges. Examples map Sample A `0x0002b7`--`0x0002de` to Sample B
`0x00028a`--`0x0002b1`, and Sample A `0x0004b0`--`0x0004d7` to Sample B
`0x000483`--`0x0004aa`. Each of those Sample B ranges begins `0x2d` bytes
earlier than its Sample A counterpart.

## Regions that differ significantly

- The complete files differ in size by 45,751 bytes. Sample B continues from
  `0x004067` through `0x00f31d`, where Sample A has no corresponding bytes at
  the same or any later file offset.
- In the aligned 4 KiB window `0x003000`--`0x003fff`, printable-run share is
  28.00% in Sample A and 53.25% in Sample B, a difference of 25.25 percentage
  points. Zero-byte share differs by 9.08 percentage points in that window.
- Sample B's `0x00e000`--`0x00efff` window has a printable-run share of 0.24%,
  below every measured Sample A window (whose minimum is 4.69%).
- The differing SHA-256 values establish that the complete data forks are not
  byte-for-byte equal.
- The early printable strings differ in content and offset even where nearby
  exact binary runs are present.

## Regions that appear structurally consistent by measurement

"Consistent" here means only that the listed byte characteristics or exact
bytes are similar; it does not identify a structure.

- The first two aligned 4 KiB windows in both files have entropy within 0.11
  bits/byte of each other and zero-byte shares within 1.76 percentage points.
- Both files contain the same Finder Type and Creator bytes and identical
  FinderInfo bytes outside `0x18`--`0x1b`.
- Both files contain the same 85 occurrences of each of the three listed
  recurring 16-byte sequences, with early occurrence spacing of `0x29` bytes.
- Numerous exact matches in the early file ranges are displaced by `0x2d`
  bytes between Sample A and Sample B.
- Exact contiguous matches hundreds of bytes long occur at later, differently
  displaced offsets, including the six largest ranges in the matching table.
- The scan found 279 distinct printable strings common to both files.

## Explicit unknowns and limits

- The meanings of all data-fork byte ranges, repeated sequences, offsets,
  printable strings, and differences are unknown here.
- This comparison does not identify headers, sections, records, fields,
  indexes, event data, or parser boundaries.
- The reason for the filenames' similarity and the history or relationship of
  the two projects have not been independently established.
- The Studio Vision or Studio Vision Pro version that created or last wrote
  either file is not established by this comparison.
- Whether either file still opens in original software was not tested.
- The presence, absence, or meaning of audio or MIDI content was not tested.
- No conclusion about the original presence or absence of resource forks is
  drawn from their absence on these inspected copies.
- FinderInfo bytes after the Type and Creator values are not interpreted.
- Filesystem dates, modes, quarantine attributes, and other metadata may
  reflect copying or transfer; their provenance is not established.
- Printable runs can occur by chance in binary data and are not necessarily
  text fields.
- Entropy values describe byte distributions only. They do not establish
  compression, encryption, corruption, or semantic content.
- The selected cross-file matching blocks are algorithm-dependent. Repeated
  bytes permit alternative alignments, and unreported shorter matches also
  exist.
- These two samples do not establish properties shared by all Studio Vision
  project files.
