# Controlled Save Experiment 001: Track Name Change

This document records a read-only comparison of `newest STUFF` and
`newest STUFF altered` from Controlled Save Experiment 001. The user recalls
that the only project edit was changing `Track 7` to `Track 7 TEST` and then
saving under a new filename. That description is experimental context, not a
file-format conclusion. All findings below are direct measurements of the two
files.

No project file was opened, resaved, or modified during this comparison.

## File inventory

| Measurement | `newest STUFF` | `newest STUFF altered` |
| --- | ---: | ---: |
| Data-fork size | 203,422 bytes | 211,468 bytes |
| SHA-256 | `7f97918efd8c8756058b72b4cada4a39a1f0f94655aa2982d4e03cfa8a876114` | `244f69731644b4d7ecc6743986efc5aefffc49adaa2a54d9f11d7028b0e649b6` |
| Finder Type | `MIDS` | `MID2` |
| Finder Creator | `MIDA` | `MIDA` |

The altered data fork is 8,046 bytes larger. The different digests establish
that the data forks are not byte-identical.

## Finder metadata and resource forks

Direct observations from the files' extended attributes are:

- `newest STUFF` has `com.apple.FinderInfo` and
  `com.apple.ResourceFork` attributes. Its 32-byte FinderInfo value begins
  `4d 49 44 53 4d 49 44 41`, which contains the Type and Creator values
  `MIDS` and `MIDA`. Its resource fork is 286 bytes and has SHA-256
  `928f342013fc1c0e0ded95c5e46d7bb01f851def540a6498cf8d9534d9c363ca`.
- `newest STUFF altered` has `com.apple.FinderInfo`,
  `org.BasiliskII.ExtendedFinderInfo`, and `org.BasiliskII.FinderInfo`
  attributes. Its 32-byte `com.apple.FinderInfo` value begins
  `4d 49 44 32 4d 49 44 41`, which contains `MID2` and `MIDA`.
- No `com.apple.ResourceFork` attribute was present on
  `newest STUFF altered` at the time of inspection. The absence of that
  attribute establishes only that a resource fork was not available through
  that representation; it does not establish why.
- The observed Finder Type changed from `MIDS` to `MID2`. The comparison does
  not establish what either Type represents or why the save changed it.

## Byte-level comparison

### Same-offset comparison

A positional comparison was made over the 203,422 offsets present in both
data forks:

| Result at the same numeric offset | Byte count | Contiguous runs |
| --- | ---: | ---: |
| Different | 190,447 | 7,317 |
| Identical | 12,975 | 7,317 |

The first byte, at `0x00000000`, is identical. The first differing offset is
`0x00000001`. The final four bytes are identical, but they occur at different
offsets: `0x00031a9a`--`0x00031a9d` in `newest STUFF` and
`0x00033a08`--`0x00033a0b` in `newest STUFF altered`.

The largest same-offset differing run is 1,992 bytes at
`0x0000e165`--`0x0000e92c`. The largest same-offset identical run is 57 bytes
at `0x00001972`--`0x000019aa`. Thus, some local spans remain identical at the
same offsets, but there is no long unchanged prefix or suffix at matching
numeric offsets.

This run summary is a complete accounting of every offset shared by the two
files: every shared offset belongs to exactly one of the two rows above. A
literal list of all 190,447 differing offsets would obscure the result without
adding evidence.

### Insertions and deletions

The altered file contains 8,046 more bytes than the original file. In a
strict same-offset comparison, `0x00031a9e`--`0x00033a0b` exists only in
`newest STUFF altered` because the original has already reached end of file.

That size difference does **not** by itself establish one 8,046-byte insertion,
nor does it establish that no bytes were deleted elsewhere. Repeated zero
bytes, repeated project records, and the broad rewrite allow multiple valid
sequence alignments. Consequently, this experiment can report the exact size
difference and same-offset changes, but cannot uniquely classify particular
byte spans as inserted or deleted without additional structural evidence.

## Printable-string differences

For this comparison, a printable string is a maximal run of at least four
bytes in the inclusive ASCII range `0x20`--`0x7e`. This mechanical definition
also admits accidental printable runs in binary data.

| Measurement | `newest STUFF` | `newest STUFF altered` |
| --- | ---: | ---: |
| Printable-string instances | 11,800 | 10,035 |
| Distinct printable-string values | 8,379 | 7,031 |

Comparing string values as multisets found 1,977 instances present only in the
original inventory and 212 instances present only in the altered inventory.
Those totals show that the printable changes are not limited to one string.

The exact printable sequence `Track 7 TEST` occurs once in
`newest STUFF altered`, beginning at `0x0002f6ca`, and does not occur in
`newest STUFF`. The exact sequence `Track 7` occurs nine times in
`newest STUFF`, at:

```text
0x00003156  0x00009375  0x00012f1a  0x00017ccd  0x0001b8c7
0x0001f30c  0x00025a83  0x00026d1f  0x0002b867
```

In `newest STUFF altered`, the bytes `Track 7` occur eight times, at:

```text
0x000070cb  0x0000e60e  0x0001961e  0x0001ed29
0x00022db5  0x0002721c  0x0002dc1a  0x0002f6ca
```

The final occurrence is the prefix of `Track 7 TEST`; it is not a separate
maximal printable string. Other newly observed printable values include
`MacOS9HD`, `Desktop Folder`, and
`MacOS9HD:System Folder:OMS Folder:Factory Names:General MIDI`. Other values
not present in the altered inventory include `System Drive`, `Primary 1GB:`,
and `System Drive:System Folder:OMS Folder:Factory Names:General MIDI`.
These are literal string differences only; this comparison does not assign a
field or purpose to them.

## Documented candidate opening region

The experimental inspector described in
[`DEVICE_TABLE_RESEARCH.md`](DEVICE_TABLE_RESEARCH.md) currently reports
candidate ranges from `0x0000000e` through `0x000001fc`, inclusive. Both files
are long enough for the complete documented region.

At the same numeric offsets within those 495 bytes, 376 bytes differ and 119
bytes are identical. The differing bytes form these 34 inclusive ranges:

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

Therefore, the observed save changed bytes both inside and outside the
documented candidate opening region. The track-name strings listed above all
occur outside that region.

## Unchanged regions

Direct comparison establishes three different forms of equality:

- 12,975 bytes are equal at the same numeric offsets, distributed among
  7,317 runs.
- The one-byte prefix at offset `0x00000000` is equal.
- The four-byte suffixes are equal as byte sequences, although their numeric
  offsets differ because the file sizes differ.

These observations do not establish that similarly placed or equal byte runs
represent the same logical structures. Conversely, the broad inequality does
not establish that every logical value changed; serialization or surrounding
data may have changed while some project content remained the same.

## Explicit unknowns

- The comparison does not establish which occurrence of `Track 7` in the
  original, if any, corresponds to `Track 7 TEST` in the altered file.
- It does not establish whether the altered file was produced by rewriting
  existing structures, inserting data, deleting data, changing serialization,
  changing application state, or a combination of these operations.
- It does not establish why the Finder Type, resource-fork availability,
  paths, opening candidate bytes, and many other printable strings differ.
- It does not establish whether metadata differences arose during the Studio
  Vision save, later file transfer, or preservation in the external research
  collection.
- It does not establish a binary field, record size, table boundary, pointer,
  length value, encoding rule, or semantic meaning for any changed byte.
- A controlled experiment with pre-save and post-save copies made on the same
  preserved filesystem, plus additional one-variable saves, is required to
  distinguish repeatable save behavior from changes unique to this pair.
