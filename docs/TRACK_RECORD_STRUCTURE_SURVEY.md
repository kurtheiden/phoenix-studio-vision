# Objective

This survey records structural observations about the repeated 120-byte sequence
identified in `FIRST_MIDI_RECOVERY_SPIKE.md`. It treats every 120-byte slice as
opaque. It does not identify a field, propose parser behavior, or assign meaning
to any observed value.

# Method

The inspected data fork was the externally held `Opcode/MY MUSIC/newest STUFF`.
Its observed size was 203,422 bytes (`0x00031a9e`) and its SHA-256 was
`7f97918efd8c8756058b72b4cada4a39a1f0f94655aa2982d4e03cfa8a876114`,
matching the artifact recorded by the first recovery spike.

The earlier spike documents names at a 120-byte cadence beginning with `Meter
Track` at absolute offset `0x0002686f`. Inspection shows that this name and each
following name begins at relative offset `0x0f` in a 120-byte slice. The survey
therefore uses `0x00026860` as the first boundary and advances by exactly
`0x78`. Thirteen complete slices are present through `0x00026e77`. The earlier
spike's bounded candidate region ends at `0x00026e2f`; completing the last
observed 120-byte slice accounts for the larger end bound here.

For each relative byte position, the 13 observed bytes were compared directly.
A position is called constant only when all 13 bytes are identical. Otherwise
it is called variable. Printable strings are maximal runs of at least four
ASCII bytes in the inclusive range `0x20` through `0x7e`.

Candidate numeric readings were computed mechanically at non-overlapping,
naturally aligned two-byte and four-byte boundaries, in both byte orders. This
is a reporting convention, not evidence that those boundaries or widths are
fields. A nonzero reading below the data-fork size is described as
“in-file-range”; that arithmetic property alone does not establish a pointer.

# Record inventory

Offsets are zero-based absolute data-fork offsets. SHA-256 covers exactly the
120 bytes beginning at the listed offset. Labels reproduce the observed
printable run and are used only to distinguish records.

| Index | Absolute range | Printable label | Record SHA-256 |
|---:|---|---|---|
| 0 | `0x00026860–0x000268d7` | `Meter Track` | `b17fb88f6586c9e932e205ec4208c9b44a9678ab7ca99c4b52a3cf596e343ee6` |
| 1 | `0x000268d8–0x0002694f` | `Tempo Track` | `25430c1c032e1f3b13700b3fd7ab967bc18f142f1fa27a9ff5a31fe368881a8f` |
| 2 | `0x00026950–0x000269c7` | `Track 1` | `1c0ee5f440a3f62b750f864492fd6b89440519e0288b3d63315c341abc5d33bb` |
| 3 | `0x000269c8–0x00026a3f` | `Track 2` | `eaf4e7ecf9c182334b41a9e04f203f1529731b97709b653a517b3d52bed2bc20` |
| 4 | `0x00026a40–0x00026ab7` | `sys100loops` | `dcadf942139f1d684dffc21512c07538d903916a9d7a007e2c5940a4fd96e535` |
| 5 | `0x00026ab8–0x00026b2f` | `Track 4` | `c093ce30a3a2755c45f3c75ec4e621458b7106b5d0c48b0e698c6b53607fc20d` |
| 6 | `0x00026b30–0x00026ba7` | `Track 5` | `8201a9cd33b030df771ed479fdee94e8fa5ca558e431d42c39ed5b1ec84974f1` |
| 7 | `0x00026ba8–0x00026c1f` | `Track 3` | `7b4404fdc8e9b3e11401d32d5cd5d0bcad9916bf0a10124a91cf48d0fd2be22a` |
| 8 | `0x00026c20–0x00026c97` | `Track 6` | `2b81d35a86fc29a118687c953ca28c10fc563394ff812eae952a2b214387bf7c` |
| 9 | `0x00026c98–0x00026d0f` | `Track 3 #2` | `575d7ae661b1f6f48ee5a494b65d79ca412d98620ad276d23bd491122853f676` |
| 10 | `0x00026d10–0x00026d87` | `Track 7` | `0f8d66f440045aeb3b6b85ff3ecb31b024aad7cba13b6de65ae11ca53d90ea4d` |
| 11 | `0x00026d88–0x00026dff` | `Track 12` | `afe2f5af6652f857263f6671897723ef8baa5e99922a487d8c08189898798efc` |
| 12 | `0x00026e00–0x00026e77` | `Track 8` | `7e5023a0ef8a6b3147e30413abd4a93234507883787176f9ea8f8c036f4d8a05` |

No two complete records have the same SHA-256.

# Constant byte positions

The following relative positions are identical across all 13 records. Bytes
are shown exactly as observed.

| Relative position(s) | Constant byte(s) |
|---|---|
| `0x1a–0x1d` | `00 00 00 00` |
| `0x21` | `04` |
| `0x23` | `00` |
| `0x25` | `00` |
| `0x2d` | `00` |
| `0x31` | `00` |
| `0x33` | `00` |
| `0x38` | `ff` |
| `0x3b` | `00` |
| `0x41` | `00` |
| `0x43–0x4d` | `00 00 80 00 15 00 00 00 00 00 04` |
| `0x51–0x59` | `00 00 00 00 00 00 00 00 00` |
| `0x5b` | `00` |
| `0x5f` | `00` |
| `0x61–0x65` | `00 04 00 00 04` |
| `0x67` | `00` |

This accounts for 44 constant positions. Constancy in this sample does not
establish that a byte is invariant in other projects or record sequences.

# Variable byte positions

The other 76 relative positions differ in at least one of the 13 records:

`0x00–0x19`, `0x1e–0x20`, `0x22`, `0x24`, `0x26–0x2c`, `0x2e–0x30`,
`0x32`, `0x34–0x37`, `0x39–0x3a`, `0x3c–0x40`, `0x42`, `0x4e–0x50`,
`0x5a–0x5e`, `0x60`, `0x66`, and `0x68–0x77`.

The following byte-position variability map covers all 120 relative offsets.
`C` means identical across all records and `V` means at least one observed byte
differs.

```text
relative  00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f
0x00      V  V  V  V  V  V  V  V  V  V  V  V  V  V  V  V
0x10      V  V  V  V  V  V  V  V  V  V  C  C  C  C  V  V
0x20      V  C  V  C  V  C  V  V  V  V  V  V  V  C  V  V
0x30      V  C  V  C  V  V  V  V  C  V  V  C  V  V  V  V
0x40      V  C  V  C  C  C  C  C  C  C  C  C  C  C  V  V
0x50      V  C  C  C  C  C  C  C  C  C  V  C  V  V  V  C
0x60      V  C  C  C  C  C  V  C  V  V  V  V  V  V  V  V
0x70      V  V  V  V  V  V  V  V
```

# Candidate numeric fields

The numeric observations below are deliberately described as candidate
readings. The source bytes remain opaque.

## Repeated aligned readings

This table lists aligned windows outside the printable-name area that have a
value repeated in at least two records. Counts are numbers of records. Hex is
used so that byte order remains visible. The table reports both interpretations
of the same bytes; it does not select one.

| Relative window | 16-bit big-endian repeats | 16-bit little-endian repeats |
|---|---|---|
| `0x1e–0x1f` | `0x0000` ×4; `0x6401` ×7; `0x6402` ×2 | `0x0000` ×4; `0x0164` ×7; `0x0264` ×2 |
| `0x20–0x21` | `0x0004` ×4; `0x6c04` ×6; `0x5104` ×2 | `0x0400` ×4; `0x046c` ×6; `0x0451` ×2 |
| `0x22–0x23` | `0xff00` ×5; `0xfe00` ×7 | `0x00ff` ×5; `0x00fe` ×7 |
| `0x24–0x25` | `0x1400` ×5; `0x8300` ×7 | `0x0014` ×5; `0x0083` ×7 |
| `0x26–0x27` | `0x1401` ×5; `0x0201` ×5; `0x0202` ×2 | `0x0114` ×5; `0x0102` ×5; `0x0202` ×2 |
| `0x28–0x29` | `0x6c01` ×5; `0xdc02` ×3 | `0x016c` ×5; `0x02dc` ×3 |
| `0x2a–0x2b` | `0xec00` ×5; `0x7d00` ×4; `0xfd00` ×2 | `0x00ec` ×5; `0x007d` ×4; `0x00fd` ×2 |
| `0x2c–0x2d` | `0x5400` ×5 | `0x0054` ×5 |
| `0x2e–0x2f` | `0x0000` ×5 | `0x0000` ×5 |
| `0x30–0x31` | `0x0000` ×5 | `0x0000` ×5 |
| `0x32–0x33` | `0x0300` ×5; `0x0500` ×5; `0x0900` ×2 | `0x0003` ×5; `0x0005` ×5; `0x0009` ×2 |
| `0x34–0x35` | `0x0a00` ×6 | `0x000a` ×6 |
| `0x36–0x37` | `0x00ff` ×7; `0x00fe` ×4 | `0xff00` ×7; `0xfe00` ×4 |
| `0x38–0x39` | `0xff80` ×8; `0xff00` ×5 | `0x80ff` ×8; `0x00ff` ×5 |
| `0x3a–0x3b` | `0x0000` ×8; `0x8300` ×5 | `0x0000` ×8; `0x0083` ×5 |
| `0x3c–0x3d` | `0x1400` ×8; `0x7401` ×5 | `0x0014` ×8; `0x0174` ×5 |
| `0x3e–0x3f` | `0xc800` ×8; `0xdb01` ×5 | `0x00c8` ×8; `0x01db` ×5 |
| `0x40–0x41` | `0xc800` ×8; `0xdd00` ×5 | `0x00c8` ×8; `0x00dd` ×5 |
| `0x42–0x43` | `0x0000` ×8; `0x1700` ×5 | `0x0000` ×8; `0x0017` ×5 |
| `0x4e–0x4f` | `0x5c49` ×6; `0x5c50` ×2; `0x5613` ×2; `0x561d` ×2 | `0x495c` ×6; `0x505c` ×2; `0x1356` ×2; `0x1d56` ×2 |
| `0x50–0x51` | `0x8000` ×2 | `0x0080` ×2 |
| `0x5a–0x5b` | `0x3000` ×3; `0x3900` ×2; `0x4100` ×2 | `0x0030` ×3; `0x0039` ×2; `0x0041` ×2 |
| `0x5c–0x5d` | `0x0168` ×3; `0x01ab` ×2; `0x01e7` ×2 | `0x6801` ×3; `0xab01` ×2; `0xe701` ×2 |
| `0x5e–0x5f` | `0x8000` ×7; `0x0000` ×5 | `0x0080` ×7; `0x0000` ×5 |
| `0x60–0x61` | `0x8000` ×10; `0x8800` ×3 | `0x0080` ×10; `0x0088` ×3 |
| `0x66–0x67` | `0x0000` ×2; `0x0100` ×11 | `0x0000` ×2; `0x0001` ×11 |
| `0x68–0x69` | `0x45ff` ×4 | `0xff45` ×4 |
| `0x6a–0x77`, each aligned 16-bit window | `0x0000` ×2; `0xffff` ×11 | `0x0000` ×2; `0xffff` ×11 |

At four-byte alignment, repeated readings occur at `0x00`, `0x04`, `0x08`,
`0x0c`, `0x10`, `0x14`, `0x18`, `0x1c`, `0x20`, `0x24`, `0x28`, `0x2c`,
`0x30`, `0x34`, `0x38`, `0x3c`, `0x40`, `0x4c`, `0x50`, `0x58`, `0x5c`,
`0x60`, `0x64`, `0x68`, `0x6c`, `0x70`, and `0x74`. The repetitions are
expected in part from the constant bytes and repeated byte patterns documented
above. They do not independently establish 32-bit fields.

Representative four-byte observations that preserve both byte orders are:

| Relative window | Observed big-endian readings (count) | Same bytes, little-endian (count) |
|---|---|---|
| `0x1c–0x1f` | `0x00000000` ×4; `0x00006401` ×7; `0x00006402` ×2 | `0x00000000` ×4; `0x01640000` ×7; `0x02640000` ×2 |
| `0x28–0x2b` | `0x6c01ec00` ×5; `0xdc027d00` ×3; five other values ×1 | `0x00ec016c` ×5; `0x007d02dc` ×3; five other values ×1 |
| `0x30–0x33` | `0x00000300` ×5; eight other values ×1 | `0x00030000` ×5; eight other values ×1 |
| `0x4c–0x4f` | `0x00045c49` ×6; `0x00045c50` ×2; `0x00045613` ×2; `0x0004561d` ×2; one other value ×1 | corresponding byte reversals with the same counts |
| `0x50–0x53` | `0x80000000` ×2; eleven other values ×1 | `0x00000080` ×2; eleven other values ×1 |
| `0x58–0x5b` | `0x00003000` ×3; `0x00003900` ×2; `0x00004100` ×2; six other values ×1 | corresponding byte reversals with the same counts |
| `0x5c–0x5f` | `0x01680000` ×3; `0x01ab8000` ×2; `0x01e78000` ×2; six other values ×1 | corresponding byte reversals with the same counts |
| `0x60–0x63` | `0x80000400` ×10; `0x88000400` ×3 | `0x00040080` ×10; `0x00040088` ×3 |
| `0x64–0x67` | `0x00040100` ×11; `0x00040000` ×2 | `0x00010400` ×11; `0x00000400` ×2 |
| `0x68–0x6b` | `0x45ffffff` ×4; nine other values ×1 | `0xffffff45` ×4; nine other values ×1 |

## Values that resemble file offsets

Every unsigned 16-bit reading is at most 65,535, below this file's 203,422-byte
length. Consequently every nonzero 16-bit reading is mechanically
in-file-range in both byte orders. This test cannot discriminate candidates in
this artifact and supplies no evidence that any such value is an offset.

Among aligned 32-bit readings, the nonzero in-file-range observations are:

| Relative window | Byte order | In-file-range values observed |
|---|---|---|
| `0x0c–0x0f` | big | `0x00000054` |
| `0x14–0x17` | little | `0x00003120`, `0x00003220`, `0x00003620`, `0x00003720`, `0x00003820` |
| `0x18–0x1b` | little | `0x00000032`, `0x00006b63`, `0x00007370` |
| `0x1c–0x1f` | big | `0x00006401`, `0x00006402` |
| `0x28–0x2b` | big | `0x00018803` |
| `0x2c–0x2f` | little | `0x00000054` |
| `0x30–0x33` | big | `0x00000300` |
| `0x30–0x33` | little | `0x000000b4`, `0x00030000` |
| `0x38–0x3b` | little | `0x000080ff` |
| `0x40–0x43` | little | `0x000000c8` |
| `0x50–0x53` | little | `0x00000008`, `0x00000038`, `0x0000004c`, `0x00000068`, `0x00000080`, `0x00000088`, `0x00000094`, `0x000000a0`, `0x000000ac`, `0x000000b4`, `0x000000c4`, `0x000000f8` |
| `0x58–0x5b` | big | `0x00002c00`, `0x00002d00`, `0x00002e00`, `0x00003000`, `0x00003100`, `0x00003300`, `0x00003900`, `0x00004100`, `0x00004b00` |
| `0x5c–0x5f` | little | `0x00004a01`, `0x00005901`, `0x00006801` |
| `0x64–0x67` | little | `0x00000400`, `0x00010400` |
| `0x68–0x6b` | little | `0x00000003`, `0x00000045` |

These values merely fall between byte zero and EOF. No target correlation,
boundary relationship, or controlled change establishes any as a file offset.

# Printable strings

Each record contains exactly one maximal printable run of at least four bytes.
Every run begins at relative offset `0x0f`. The absolute locations are:

| Record | Relative range | Absolute range | Exact bytes as ASCII |
|---:|---|---|---|
| 0 | `0x0f–0x19` | `0x0002686f–0x00026879` | `Meter Track` |
| 1 | `0x0f–0x19` | `0x000268e7–0x000268f1` | `Tempo Track` |
| 2 | `0x0f–0x15` | `0x0002695f–0x00026965` | `Track 1` |
| 3 | `0x0f–0x15` | `0x000269d7–0x000269dd` | `Track 2` |
| 4 | `0x0f–0x19` | `0x00026a4f–0x00026a59` | `sys100loops` |
| 5 | `0x0f–0x15` | `0x00026ac7–0x00026acd` | `Track 4` |
| 6 | `0x0f–0x15` | `0x00026b3f–0x00026b45` | `Track 5` |
| 7 | `0x0f–0x15` | `0x00026bb7–0x00026bbd` | `Track 3` |
| 8 | `0x0f–0x15` | `0x00026c2f–0x00026c35` | `Track 6` |
| 9 | `0x0f–0x18` | `0x00026ca7–0x00026cb0` | `Track 3 #2` |
| 10 | `0x0f–0x15` | `0x00026d1f–0x00026d25` | `Track 7` |
| 11 | `0x0f–0x16` | `0x00026d97–0x00026d9e` | `Track 12` |
| 12 | `0x0f–0x15` | `0x00026e0f–0x00026e15` | `Track 8` |

# Unknowns

- The evidence does not establish the purpose of the 120-byte slices.
- The evidence does not establish whether the first or last observed slice is
  the boundary of a larger collection.
- The evidence does not establish the meaning, width, signedness, byte order,
  or alignment of any numeric-looking bytes.
- The evidence does not establish that any in-file-range value is an offset.
- The evidence does not establish whether constant positions remain constant
  in another artifact or after a controlled edit.
- The evidence does not establish how these slices relate to event-bearing
  project data.

# Recommended next investigation

Perform a controlled rename-only save experiment on a copy of the same project:
change one selected label to another value of the same byte length, preserve all
other user-visible settings, and compare the original and saved data forks
byte-for-byte. Record every changed range before testing whether the observed
120-byte cadence and relative string location remain stable. This experiment
would test the structural boundary evidence while minimizing the number of
intentionally changed bytes; it should not be used to assign meaning to any
other changed position without additional controls.
