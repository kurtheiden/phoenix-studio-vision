# Studio Vision Audio Reference Findings

This document records a read-only search for literal audio-reference evidence
in one authentic Studio Vision project. It reports observed bytes, strings,
metadata, and prior application behavior without assigning binary structures
or field meanings.

## Evidence categories

- **Direct project-file observations** are measurements and literal bytes read
  from the inspected sample.
- **Observed application behavior** is the previously reported Studio Vision
  behavior from the SheepShaver session described below.
- **Unknowns** are questions not resolved by these observations.

No claim in this document is based on a decoded binary layout.

## Primary sample and method

- Research filename: `dominion with samples`.
- Inspected path:
  `/Users/kurtheiden/Documents/Phoenix Research/Authentic Studio Vision Projects/dominion with samples`.
- The file was inspected without modification.
- Logical/data-fork size: 134,204 bytes (`0x20c3c`).
- Last data-fork offset: `0x20c3b`.
- SHA-256:
  `4ec77addb3b39dfb5791d853cbb6225df9da30a95431633137e02a6811a8b34b`.
- Finder Type bytes: `MIDS` (`4d 49 44 53`).
- Finder Creator bytes: `MIDA` (`4d 49 44 41`).
- Printable strings were scanned as maximal runs of ASCII bytes `0x20`
  through `0x7e`, with a minimum length of four bytes.
- Raw case-insensitive byte searches were also made for `mercwork`, `audio`,
  `OMS`, `bundle`, `AIFF`, `WAVE`, `Sound Designer`, and `SDII`.
- Colon- and slash-containing printable runs were reviewed as path-like
  candidates. Their punctuation alone was not treated as proof of a path.
- The top-level filenames in the authorized authentic-project directory were
  checked for a file named `mercwork`.

## Literal audio-filename candidate

The only observed filename candidate that can be compared directly with the
reported missing-audio prompt is `mercwork`. The exact lowercase ASCII bytes
`6d 65 72 63 77 6f 72 6b` occur three times:

| Offset | Directly observed surrounding bytes |
| --- | --- |
| `0x014cd4` | Preceded immediately by `08`; followed immediately by `00 ff ff ff 7c` |
| `0x020b89` | Preceded immediately by `08`; followed immediately by `00 00 02 00` |
| `0x020bf9` | Preceded immediately by `08`; followed immediately by the ASCII bytes `rive49Pro` and then `00` |

The complete maximal printable run at `0x020bf9` is
`mercworkrive49Pro`. The byte scan alone does not establish whether that run
is one string, adjacent values without an ASCII delimiter, or incidental
printability. In particular, this document does not interpret the preceding
`08` bytes.

No literal `.aif`, `.aiff`, `.wav`, `.wave`, `.sd2`, `AIFF`, `WAVE`,
`Sound Designer`, or `SDII` was found in the complete data fork. This does not
establish that `mercwork` originally had no extension; it records only that no
such literal extension was observed in this sample.

No top-level file named `mercwork` was present in the authorized authentic
project collection at inspection time.

## Comparison with observed SheepShaver behavior

During the previously reported SheepShaver session, Studio Vision requested a
missing digital audio file named `mercwork`. The literal project bytes match
that reported prompt text exactly in spelling and case at all three offsets
listed above.

The session also reportedly showed a separate request for an OMS bundle, and
cancelling the missing-audio prompt caused Studio Vision or SheepShaver to
crash. Those are observed application behaviors from that session, not results
reproduced during this file inspection.

The exact filename match is evidence that the prompt text also exists
literally in the project data fork. It does not, by itself, establish what
surrounding bytes mean, how Studio Vision locates a file, whether the audio is
external, or whether audio data is also embedded.

## Other audio-related strings

The sample contains the following literal runs near its beginning:

| String | Offset |
| --- | ---: |
| `Audio-1` | `0x000f79` |
| `Audio-2` | `0x000fa2` |
| `Audio-3` | `0x000fcb` |
| `Audio-4` | `0x000ff4` |
| `Audio-5` | `0x00101d` |
| `Audio-6` | `0x001046` |
| `Audio-7` | `0x00106f` |
| `Audio-8` | `0x001098` |
| `Audio-9` | `0x0010c1` |
| `Audio-10` | `0x0010ea` |
| `Audio-11` | `0x001113` |
| `Audio-12` | `0x00113c` |
| `Audio-13` | `0x001165` |
| `Audio-14` | `0x00118e` |
| `Audio-15` | `0x0011b7` |
| `Audio-16` | `0x0011e0` |
| `Audio/Video` | `0x00122a` and `0x0012dd` |

The `Audio-1` through `Audio-16` starts are separated by `0x29` bytes. These
values are recorded as audio-related printable strings, not as filenames or
decoded records. `Audio/Video` also occurs within a colon-delimited run listed
below; its meaning is not assigned here.

Other human-readable strings in the sample include names such as
`D&I Loop 21 94bpm`, `Reservist Revolt`, `OPENING FAUCET`, and
`dominion set`. The present inspection found no direct evidence that these are
audio filenames, so they are not classified as such.

## Path-like strings and classic Macintosh path syntax

Two long, human-readable runs contain colon delimiters resembling classic
Macintosh path syntax:

| Printable run and offset | Literal path-like substring |
| --- | --- |
| `0x0012dc`: `AAudio/Video:Opcode:Galaxy Plus Editors:KURT SV3.0 Bundle 12-16-95` | `Audio/Video:Opcode:Galaxy Plus Editors:KURT SV3.0 Bundle 12-16-95` begins at `0x0012dd` |
| `0x0013ff`: `>Backup 2GB:System Folder:OMS Folder:Factory Names:General MIDI` | `Backup 2GB:System Folder:OMS Folder:Factory Names:General MIDI` begins at `0x001400` |

The leading `A` and `>` bytes are part of the maximal printable runs under the
stated scan rule. No meaning is assigned to them. Both literal substrings use
colons between readable components. The first also contains the slash in
`Audio/Video`.

Many other printable runs contain colons or slashes amid punctuation-heavy
binary data. They were not recorded as human-readable paths solely because of
those characters. No path-like printable run containing `mercwork` was
observed.

## Volume-name candidates

`Backup 2GB` occurs at `0x00134f` and again at `0x001400`. The second occurrence
is the first readable component of the colon-delimited substring
`Backup 2GB:System Folder:OMS Folder:Factory Names:General MIDI`.

`Hillary:` occurs at `0x020ba9`, 24 bytes after the start of the `mercwork`
occurrence at `0x020b89`. The colon is part of the eight-byte printable run.
The bytes immediately preceding the `H` are `00 00 00 00 08`; the bytes
immediately following the colon are `1b 47 a0 00`.

The final printable run `mercworkrive49Pro` includes the suffix `rive49Pro`
beginning at `0x020c01`. The scan found no preceding ASCII `D`, separator, or
terminator between `mercwork` and `rive49Pro`.

`Backup 2GB`, `Hillary:`, and `rive49Pro` are recorded as candidates for
further comparison because of their literal form or byte proximity. This
document does not establish that any is a volume name or locator component.

## OMS and bundle references

The following observations are kept separate from the `mercwork` findings:

| Literal string or substring | Offset(s) |
| --- | --- |
| `KURT SV3.0 Bundle 12-16-95` | `0x001252`, `0x001304`, `0x001324` |
| `Galaxy Plus Editors` | `0x0012b9`, and within the run beginning at `0x0012dc` |
| `Backup 2GB` | `0x00134f`, `0x001400` |
| `OMS Folder` | `0x001419` |
| `Factory Names` | `0x0013de`, and within the run beginning at `0x0013ff` |
| `General MIDI` | `0x001376`, `0x001442`, and within the run beginning at `0x0013ff` |

The complete readable colon-delimited substrings and offsets are reported in
the preceding path-like-string section. These strings are consistent with the
reported session request containing the name `KURT SV3.0 Bundle 12-16-95`, but
the inspection does not establish which bytes Studio Vision used for that
request or whether the word `Bundle` has a file-format-specific meaning.

## Explicit unknowns and limits

- The binary structures containing all reported strings are unknown.
- It is unknown whether any occurrence of `mercwork` is a stored file
  reference, a display value, duplicated metadata, or another kind of value.
- The meanings of the bytes before, after, and between the `mercwork`
  occurrences are unknown.
- It is unknown whether `mercworkrive49Pro` represents one value or multiple
  adjacent values.
- It is unknown whether `Hillary:`, `Backup 2GB`, or `rive49Pro` identifies a
  volume, directory, file, machine, or something else.
- No complete literal path containing `mercwork` was observed. A locator may
  be absent, encoded nonliterally, split across bytes, stored elsewhere, or not
  recognized by this scan.
- It is unknown whether the sample embeds audio data, refers to external audio,
  or supports both behaviors.
- It is unknown whether the absence of a literal filename extension reflects
  the original filename, classic Mac metadata, a shortened display value, or
  another cause.
- The `Audio-1` through `Audio-16` strings are not established as filenames,
  tracks, slots, records, or any other structure.
- The OMS- and bundle-related strings are not established as audio references.
- The observed SheepShaver crash does not establish whether Studio Vision,
  SheepShaver, the system configuration, or another condition caused it.
- This inspection did not launch Studio Vision, move or relink media, or alter
  the authentic sample.
- This inspection did not review Opcode documentation or test behavior beyond
  the previously reported SheepShaver observations.
- Findings from this single sample do not establish behavior for all Studio
  Vision or Studio Vision Pro projects.
