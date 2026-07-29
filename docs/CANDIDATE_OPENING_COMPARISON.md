# Experimental Candidate Opening Comparison

This document compares the output of the experimental candidate opening
inspector for exactly three authentic Studio Vision projects: `SMF files`,
`dominion with samples`, and `analog only`. Each file was inspected read-only
with:

```text
phoenix --inspect-candidate-opening <file>
```

The inspector reports candidate structures only. This comparison does not
assign device, instrument, track, OMS, audio, or other semantic meanings to any
name or byte.

## Evidence categories

### Direct observations

The offsets, byte ranges, hexadecimal bytes, and printable ASCII sequences in
this document were emitted by the experimental inspector. SHA-256 values were
verified before and after inspection and agree with the existing sample
registry. Finder codes and sample identifiers are existing direct observations
from [`AUTHENTIC_PROJECT_SURVEY.md`](AUTHENTIC_PROJECT_SURVEY.md).

### User recollection

The user recalls that `SMF files` is likely MIDI-only and may contain only a
small number of MIDI tracks. The user states that `dominion with samples` is
known to contain references associated with digital audio. `analog only` was
selected as a small comparison sample. These statements are contextual
information only. They are not verified file-format facts and are not used to
interpret the candidate bytes.

### Hypotheses

Any claim that a byte or printable sequence represents a device, instrument,
track, OMS object, audio reference, record, field, count, or length would
require additional evidence, preferably controlled-save experiments. No such
claim is made here.

## Sample identity and preservation

| Filename | Existing ID | Size | SHA-256 before and after inspection | Finder Type / Creator |
| --- | --- | ---: | --- | --- |
| `SMF files` | project-009 | 881,556 bytes | `058b8398c58089891e66c38696f35766df9babf0d1aef43f1086f90e3d319c71` | `MID2` / `MIDA` |
| `dominion with samples` | project-015 | 134,204 bytes | `4ec77addb3b39dfb5791d853cbb6225df9da30a95431633137e02a6811a8b34b` | `MIDS` / `MIDA` |
| `analog only` | project-011 | 10,107 bytes | `f3c945c764280bae7bee96c2faab716b61a0743f6949df034d231a52257ab101` | `MID2` / `MIDA` |

The before and after digest matched for every file. No authentic sample was
modified.

## Candidate-region completeness and alignment

The inspector reported the complete documented candidate opening region for
all three files. It emitted the same aligned candidate offsets and exact ranges
for each file:

| Candidate offset | Exact candidate byte range |
| --- | --- |
| `0x0000000e` | `0x0000000e`--`0x0000003a` |
| `0x0000003b` | `0x0000003b`--`0x00000067` |
| `0x00000068` | `0x00000068`--`0x00000094` |
| `0x00000095` | `0x00000095`--`0x000000c1` |
| `0x000000c2` | `0x000000c2`--`0x000000ee` |
| `0x000000ef` | `0x000000ef`--`0x0000011b` |
| `0x0000011c` | `0x0000011c`--`0x00000148` |
| `0x00000149` | `0x00000149`--`0x00000175` |
| `0x00000176` | `0x00000176`--`0x000001a2` |
| `0x000001a3` | `0x000001a3`--`0x000001cf` |
| `0x000001d0` | `0x000001d0`--`0x000001fc` |

Alignment here means only that the inspector applied the currently documented
candidate offsets to each complete file. It does not establish record
boundaries or a common structure.

## Exact candidate bytes

### `SMF files` — project-009

| Range | Exact bytes in hexadecimal |
| --- | --- |
| `0x0000000e`--`0x0000003a` | `49 41 43 20 42 75 73 20 23 31 fe 01 74 4d a8 01 74 4d 9c 17 24 00 00 00 00 00 02 01 74 4e 16 00 00 7a a3 80 00 01 01 2a 00 00 00 28 06` |
| `0x0000003b`--`0x00000067` | `4a 44 2d 38 30 30 73 20 23 31 fe 01 74 4d a8 01 74 4d 9c 17 24 00 00 00 00 00 02 01 74 4e 16 00 01 78 7a 00 02 01 01 2a 00 00 00 28 06` |
| `0x00000068`--`0x00000094` | `4a 44 2d 39 39 30 73 20 23 31 fe 01 74 4d a8 01 74 4d 9c 17 24 00 00 00 00 00 02 01 74 4e 16 00 02 6b be 80 00 01 01 2a 00 00 00 28 08` |
| `0x00000095`--`0x000000c1` | `4a 75 6e 6f 2d 31 30 36 23 31 fe 01 74 4d a8 01 74 4d 9c 17 24 00 00 00 00 00 02 01 74 4e 16 00 03 73 5d 80 00 01 01 2a 00 00 00 28 07` |
| `0x000000c2`--`0x000000ee` | `4a 56 2d 31 30 38 30 36 23 31 fe 01 74 4d a8 01 74 4d 9c 17 24 00 00 00 00 00 02 01 74 4e 16 00 04 7a ec ff ff 01 01 2a 00 00 00 28 06` |
| `0x000000ef`--`0x0000011b` | `4a 56 2d 38 38 30 30 36 23 31 fe 01 74 4d a8 01 74 4d 9c 17 24 00 00 00 00 00 02 01 74 4e 16 00 05 73 ca ff 00 01 01 2a 00 00 00 28 08` |
| `0x0000011c`--`0x00000148` | `4a 56 2d 38 38 30 2f 32 23 31 fe 01 74 4d a8 01 74 4d 9c 17 24 00 00 00 00 00 02 01 74 4e 16 00 06 7b c7 00 ff 01 01 2a 00 00 00 28 0f` |
| `0x00000149`--`0x00000175` | `51 75 69 63 6b 74 69 6d 65 20 4d 75 73 69 63 01 74 4d 9c 17 24 00 00 00 00 00 02 01 74 4e 16 00 07 6c 71 ff ff 01 01 2a 00 00 00 28 05` |
| `0x00000176`--`0x000001a2` | `53 2d 37 36 30 74 69 6d 65 20 4d 75 73 69 63 01 74 4d 9c 17 24 00 00 00 00 00 02 01 74 4e 16 00 08 72 e4 ff ff 01 01 2a 00 00 00 28 0c` |
| `0x000001a3`--`0x000001cf` | `53 6f 75 6e 64 20 43 61 6e 76 61 73 73 69 63 01 74 4d 9c 17 24 00 00 00 00 00 02 01 74 4e 16 00 09 7e 51 ff ff 01 01 2a 00 00 00 28 16` |
| `0x000001d0`--`0x000001fc` | `53 74 75 64 69 6f 20 50 61 74 63 68 65 73 20 70 67 6d 20 63 68 67 00 00 00 00 02 01 74 4e 16 00 0b 68 c1 80 00 01 01 10 00 00 00 24 00` |

### `dominion with samples` — project-015

| Range | Exact bytes in hexadecimal |
| --- | --- |
| `0x0000000e`--`0x0000003a` | `4a 44 2d 38 30 30 b6 02 47 39 ee 02 d0 98 be 02 d0 98 b2 16 0a 00 00 00 00 00 02 02 d0 99 2c 00 01 6b 4c 80 00 01 01 2a 00 00 00 28 10` |
| `0x0000003b`--`0x00000067` | `4a 44 2d 39 39 30 20 77 2f 56 69 6e 74 61 67 65 d0 98 b2 16 0a 00 00 00 00 00 02 02 d0 99 2c 00 02 78 43 80 00 01 01 2a 00 00 00 28 08` |
| `0x00000068`--`0x00000094` | `4a 75 6e 6f 2d 31 30 36 2f 56 69 6e 74 61 67 65 d0 98 b2 16 0a 00 00 00 00 00 02 02 d0 99 2c 00 03 67 c7 80 00 01 01 2a 00 00 00 28 07` |
| `0x00000095`--`0x000000c1` | `4a 56 2d 31 30 38 30 36 2f 56 69 6e 74 61 67 65 d0 98 b2 16 0a 00 00 00 00 00 02 02 d0 99 2c 00 05 62 7c ff ff 01 01 2a 00 00 00 28 0c` |
| `0x000000c2`--`0x000000ee` | `4a 56 2d 38 38 30 20 4f 72 63 68 2e 74 61 67 65 d0 98 b2 16 0a 00 00 00 00 00 02 02 d0 99 2c 00 06 68 a5 ff 00 01 01 2a 00 00 00 28 0e` |
| `0x000000ef`--`0x0000011b` | `4a 56 2d 38 38 30 20 56 69 6e 74 61 67 65 67 65 d0 98 b2 16 0a 00 00 00 00 00 02 02 d0 99 2c 00 07 7b e7 00 ff 01 01 2a 00 00 00 28 0f` |
| `0x0000011c`--`0x00000148` | `51 75 69 63 6b 54 69 6d 65 20 4d 75 73 69 63 65 d0 98 b2 16 0a 00 00 00 00 00 02 02 d0 99 2c 00 08 73 d7 ff ff 01 01 2a 00 00 00 28 05` |
| `0x00000149`--`0x00000175` | `53 2d 37 36 30 54 69 6d 65 20 4d 75 73 69 63 65 d0 98 b2 16 0a 00 00 00 00 00 02 02 d0 99 2c 00 09 72 4b ff ff 01 01 2a 00 00 00 28 0c` |
| `0x00000176`--`0x000001a2` | `53 6f 75 6e 64 20 43 61 6e 76 61 73 73 69 63 65 d0 98 b2 16 0a 00 00 00 00 00 02 02 d0 99 2c 00 0a 6b 62 ff ff 01 01 2a 00 00 00 28 16` |
| `0x000001a3`--`0x000001cf` | `53 74 75 64 69 6f 20 50 61 74 63 68 65 73 20 70 67 6d 20 63 68 67 00 00 00 00 02 02 d0 99 2c 00 0b 76 4e 80 00 01 01 10 00 00 00 24 00` |
| `0x000001d0`--`0x000001fc` | `00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01 00 00 00 00 0b 00 00 7f 10 ff 80 64 00 7f 10 00 00 00 24 00 00 00 00 00` |

### `analog only` — project-011

| Range | Exact bytes in hexadecimal |
| --- | --- |
| `0x0000000e`--`0x0000003a` | `4a 44 2d 38 30 30 48 04 85 e5 90 04 13 e4 b0 03 f6 b5 58 4d 49 44 41 00 00 00 00 04 13 e4 b0 00 00 68 e0 80 00 01 01 2a 00 00 00 28 10` |
| `0x0000003b`--`0x00000067` | `4a 44 2d 39 39 30 20 77 2f 56 69 6e 74 61 67 65 f6 b5 58 4d 49 44 41 00 00 00 00 04 13 e4 b0 00 01 78 43 80 00 01 01 2a 00 00 00 28 08` |
| `0x00000068`--`0x00000094` | `4a 75 6e 6f 2d 31 30 36 2f 56 69 6e 74 61 67 65 f6 b5 58 4d 49 44 41 00 00 00 00 04 13 e4 b0 00 02 7c f9 80 00 01 01 2a 00 00 00 28 07` |
| `0x00000095`--`0x000000c1` | `4a 56 2d 31 30 38 30 36 2f 56 69 6e 74 61 67 65 f6 b5 58 4d 49 44 41 00 00 00 00 04 13 e4 b0 00 04 62 7c ff ff 01 01 2a 00 00 00 28 0c` |
| `0x000000c2`--`0x000000ee` | `4a 56 2d 38 38 30 20 4f 72 63 68 2e 74 61 67 65 f6 b5 58 4d 49 44 41 00 00 00 00 04 13 e4 b0 00 05 68 a5 ff 00 01 01 2a 00 00 00 28 0e` |
| `0x000000ef`--`0x0000011b` | `4a 56 2d 38 38 30 20 56 69 6e 74 61 67 65 67 65 f6 b5 58 4d 49 44 41 00 00 00 00 04 13 e4 b0 00 06 7b e7 00 ff 01 01 2a 00 00 00 28 0f` |
| `0x0000011c`--`0x00000148` | `51 75 69 63 6b 54 69 6d 65 20 4d 75 73 69 63 65 f6 b5 58 4d 49 44 41 00 00 00 00 04 13 e4 b0 00 07 76 fd ff ff 01 01 2a 00 00 00 28 05` |
| `0x00000149`--`0x00000175` | `53 2d 37 36 30 54 69 6d 65 20 4d 75 73 69 63 65 f6 b5 58 4d 49 44 41 00 00 00 00 04 13 e4 b0 00 08 61 3f ff ff 01 01 2a 00 00 00 28 0c` |
| `0x00000176`--`0x000001a2` | `53 6f 75 6e 64 20 43 61 6e 76 61 73 73 69 63 65 f6 b5 58 4d 49 44 41 00 00 00 00 04 13 e4 b0 00 09 6b 62 ff ff 01 01 2a 00 00 00 28 16` |
| `0x000001a3`--`0x000001cf` | `53 74 75 64 69 6f 20 50 61 74 63 68 65 73 20 70 67 6d 20 63 68 67 41 00 00 00 00 04 13 e4 b0 00 0a 6a e8 80 00 01 01 10 00 00 00 24 00` |
| `0x000001d0`--`0x000001fc` | `00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01 00 00 00 00 00 00 00 7f 10 ff 80 64 00 00 10 00 00 00 24 00 00 00 00 00` |

## Printable ASCII sequences

Every maximal printable ASCII sequence emitted by the inspector is listed
below. Single-character sequences are retained.

### `SMF files` — project-009

| Candidate range | Printable sequences (`offset`: `bytes`) |
| --- | --- |
| `0x0000000e`--`0x0000003a` | `0x0000000e`: `IAC Bus #1`; `0x0000001a`: `tM`; `0x0000001e`: `tM`; `0x00000022`: `$`; `0x0000002a`: `tN`; `0x0000002f`: `z`; `0x00000035`: `*`; `0x00000039`: `(` |
| `0x0000003b`--`0x00000067` | `0x0000003b`: `JD-800s #1`; `0x00000047`: `tM`; `0x0000004b`: `tM`; `0x0000004f`: `$`; `0x00000057`: `tN`; `0x0000005c`: `xz`; `0x00000062`: `*`; `0x00000066`: `(` |
| `0x00000068`--`0x00000094` | `0x00000068`: `JD-990s #1`; `0x00000074`: `tM`; `0x00000078`: `tM`; `0x0000007c`: `$`; `0x00000084`: `tN`; `0x00000089`: `k`; `0x0000008f`: `*`; `0x00000093`: `(` |
| `0x00000095`--`0x000000c1` | `0x00000095`: `Juno-106#1`; `0x000000a1`: `tM`; `0x000000a5`: `tM`; `0x000000a9`: `$`; `0x000000b1`: `tN`; `0x000000b6`: `s]`; `0x000000bc`: `*`; `0x000000c0`: `(` |
| `0x000000c2`--`0x000000ee` | `0x000000c2`: `JV-10806#1`; `0x000000ce`: `tM`; `0x000000d2`: `tM`; `0x000000d6`: `$`; `0x000000de`: `tN`; `0x000000e3`: `z`; `0x000000e9`: `*`; `0x000000ed`: `(` |
| `0x000000ef`--`0x0000011b` | `0x000000ef`: `JV-88006#1`; `0x000000fb`: `tM`; `0x000000ff`: `tM`; `0x00000103`: `$`; `0x0000010b`: `tN`; `0x00000110`: `s`; `0x00000116`: `*`; `0x0000011a`: `(` |
| `0x0000011c`--`0x00000148` | `0x0000011c`: `JV-880/2#1`; `0x00000128`: `tM`; `0x0000012c`: `tM`; `0x00000130`: `$`; `0x00000138`: `tN`; `0x0000013d`: `{`; `0x00000143`: `*`; `0x00000147`: `(` |
| `0x00000149`--`0x00000175` | `0x00000149`: `Quicktime Music`; `0x00000159`: `tM`; `0x0000015d`: `$`; `0x00000165`: `tN`; `0x0000016a`: `lq`; `0x00000170`: `*`; `0x00000174`: `(` |
| `0x00000176`--`0x000001a2` | `0x00000176`: `S-760time Music`; `0x00000186`: `tM`; `0x0000018a`: `$`; `0x00000192`: `tN`; `0x00000197`: `r`; `0x0000019d`: `*`; `0x000001a1`: `(` |
| `0x000001a3`--`0x000001cf` | `0x000001a3`: `Sound Canvassic`; `0x000001b3`: `tM`; `0x000001b7`: `$`; `0x000001bf`: `tN`; `0x000001c4`: `~Q`; `0x000001ca`: `*`; `0x000001ce`: `(` |
| `0x000001d0`--`0x000001fc` | `0x000001d0`: `Studio Patches pgm chg`; `0x000001ec`: `tN`; `0x000001f1`: `h`; `0x000001fb`: `$` |

### `dominion with samples` — project-015

| Candidate range | Printable sequences (`offset`: `bytes`) |
| --- | --- |
| `0x0000000e`--`0x0000003a` | `0x0000000e`: `JD-800`; `0x00000016`: `G9`; `0x0000002c`: `,`; `0x0000002f`: `kL`; `0x00000035`: `*`; `0x00000039`: `(` |
| `0x0000003b`--`0x00000067` | `0x0000003b`: `JD-990 w/Vintage`; `0x00000059`: `,`; `0x0000005c`: `xC`; `0x00000062`: `*`; `0x00000066`: `(` |
| `0x00000068`--`0x00000094` | `0x00000068`: `Juno-106/Vintage`; `0x00000086`: `,`; `0x00000089`: `g`; `0x0000008f`: `*`; `0x00000093`: `(` |
| `0x00000095`--`0x000000c1` | `0x00000095`: `JV-10806/Vintage`; `0x000000b3`: `,`; `0x000000b6`: `b|`; `0x000000bc`: `*`; `0x000000c0`: `(` |
| `0x000000c2`--`0x000000ee` | `0x000000c2`: `JV-880 Orch.tage`; `0x000000e0`: `,`; `0x000000e3`: `h`; `0x000000e9`: `*`; `0x000000ed`: `(` |
| `0x000000ef`--`0x0000011b` | `0x000000ef`: `JV-880 Vintagege`; `0x0000010d`: `,`; `0x00000110`: `{`; `0x00000116`: `*`; `0x0000011a`: `(` |
| `0x0000011c`--`0x00000148` | `0x0000011c`: `QuickTime Musice`; `0x0000013a`: `,`; `0x0000013d`: `s`; `0x00000143`: `*`; `0x00000147`: `(` |
| `0x00000149`--`0x00000175` | `0x00000149`: `S-760Time Musice`; `0x00000167`: `,`; `0x0000016a`: `rK`; `0x00000170`: `*`; `0x00000174`: `(` |
| `0x00000176`--`0x000001a2` | `0x00000176`: `Sound Canvassice`; `0x00000194`: `,`; `0x00000197`: `kb`; `0x0000019d`: `*`; `0x000001a1`: `(` |
| `0x000001a3`--`0x000001cf` | `0x000001a3`: `Studio Patches pgm chg`; `0x000001c1`: `,`; `0x000001c4`: `vN`; `0x000001ce`: `$` |
| `0x000001d0`--`0x000001fc` | `0x000001f0`: `d`; `0x000001f7`: `$` |

### `analog only` — project-011

| Candidate range | Printable sequences (`offset`: `bytes`) |
| --- | --- |
| `0x0000000e`--`0x0000003a` | `0x0000000e`: `JD-800H`; `0x00000020`: `XMIDA`; `0x0000002f`: `h`; `0x00000035`: `*`; `0x00000039`: `(` |
| `0x0000003b`--`0x00000067` | `0x0000003b`: `JD-990 w/Vintage`; `0x0000004d`: `XMIDA`; `0x0000005c`: `xC`; `0x00000062`: `*`; `0x00000066`: `(` |
| `0x00000068`--`0x00000094` | `0x00000068`: `Juno-106/Vintage`; `0x0000007a`: `XMIDA`; `0x00000089`: `|`; `0x0000008f`: `*`; `0x00000093`: `(` |
| `0x00000095`--`0x000000c1` | `0x00000095`: `JV-10806/Vintage`; `0x000000a7`: `XMIDA`; `0x000000b6`: `b|`; `0x000000bc`: `*`; `0x000000c0`: `(` |
| `0x000000c2`--`0x000000ee` | `0x000000c2`: `JV-880 Orch.tage`; `0x000000d4`: `XMIDA`; `0x000000e3`: `h`; `0x000000e9`: `*`; `0x000000ed`: `(` |
| `0x000000ef`--`0x0000011b` | `0x000000ef`: `JV-880 Vintagege`; `0x00000101`: `XMIDA`; `0x00000110`: `{`; `0x00000116`: `*`; `0x0000011a`: `(` |
| `0x0000011c`--`0x00000148` | `0x0000011c`: `QuickTime Musice`; `0x0000012e`: `XMIDA`; `0x0000013d`: `v`; `0x00000143`: `*`; `0x00000147`: `(` |
| `0x00000149`--`0x00000175` | `0x00000149`: `S-760Time Musice`; `0x0000015b`: `XMIDA`; `0x0000016a`: `a?`; `0x00000170`: `*`; `0x00000174`: `(` |
| `0x00000176`--`0x000001a2` | `0x00000176`: `Sound Canvassice`; `0x00000188`: `XMIDA`; `0x00000197`: `kb`; `0x0000019d`: `*`; `0x000001a1`: `(` |
| `0x000001a3`--`0x000001cf` | `0x000001a3`: `Studio Patches pgm chgA`; `0x000001c4`: `j`; `0x000001ce`: `$` |
| `0x000001d0`--`0x000001fc` | `0x000001f0`: `d`; `0x000001f7`: `$` |

## Byte-position comparison

The following comparison covers only bytes within the candidate ranges. An
“identical” position has the same byte value in all three files. A “different”
position is any position at which at least one of the three values differs.
The two sets are complementary within each listed range.

| Candidate range | Identical positions in all three | Different positions |
| --- | --- | --- |
| `0x0000000e`--`0x0000003a` | `0x00000025`--`0x00000027`, `0x0000002d`, `0x00000031`--`0x00000039` | `0x0000000e`--`0x00000024`, `0x00000028`--`0x0000002c`, `0x0000002e`--`0x00000030`, `0x0000003a` |
| `0x0000003b`--`0x00000067` | `0x0000003b`--`0x0000003d`, `0x00000040`, `0x00000047`, `0x00000052`--`0x00000054`, `0x0000005a`, `0x0000005c`, `0x00000060`--`0x00000066` | `0x0000003e`--`0x0000003f`, `0x00000041`--`0x00000046`, `0x00000048`--`0x00000051`, `0x00000055`--`0x00000059`, `0x0000005b`, `0x0000005d`--`0x0000005f`, `0x00000067` |
| `0x00000068`--`0x00000094` | `0x00000068`, `0x00000074`, `0x0000007f`--`0x00000081`, `0x00000087`, `0x0000008b`--`0x00000093` | `0x00000069`--`0x00000073`, `0x00000075`--`0x0000007e`, `0x00000082`--`0x00000086`, `0x00000088`--`0x0000008a`, `0x00000094` |
| `0x00000095`--`0x000000c1` | `0x00000095`, `0x0000009b`--`0x0000009c`, `0x000000a1`, `0x000000ac`--`0x000000ae`, `0x000000b4`, `0x000000ba`--`0x000000c0` | `0x00000096`--`0x0000009a`, `0x0000009d`--`0x000000a0`, `0x000000a2`--`0x000000ab`, `0x000000af`--`0x000000b3`, `0x000000b5`--`0x000000b9`, `0x000000c1` |
| `0x000000c2`--`0x000000ee` | `0x000000c2`--`0x000000c4`, `0x000000ce`, `0x000000d9`--`0x000000db`, `0x000000e1`, `0x000000e5`, `0x000000e7`--`0x000000ed` | `0x000000c5`--`0x000000cd`, `0x000000cf`--`0x000000d8`, `0x000000dc`--`0x000000e0`, `0x000000e2`--`0x000000e4`, `0x000000e6`, `0x000000ee` |
| `0x000000ef`--`0x0000011b` | `0x000000ef`--`0x000000f4`, `0x00000106`--`0x00000108`, `0x0000010e`, `0x00000114`--`0x0000011a` | `0x000000f5`--`0x00000105`, `0x00000109`--`0x0000010d`, `0x0000010f`--`0x00000113`, `0x0000011b` |
| `0x0000011c`--`0x00000148` | `0x00000133`--`0x00000135`, `0x0000013b`, `0x00000140`--`0x00000147` | `0x0000011c`--`0x00000132`, `0x00000136`--`0x0000013a`, `0x0000013c`--`0x0000013f`, `0x00000148` |
| `0x00000149`--`0x00000175` | `0x0000014f`--`0x00000157`, `0x00000160`--`0x00000162`, `0x00000168`, `0x0000016c`--`0x00000174` | `0x00000149`--`0x0000014e`, `0x00000158`--`0x0000015f`, `0x00000163`--`0x00000167`, `0x00000169`--`0x0000016b`, `0x00000175` |
| `0x00000176`--`0x000001a2` | `0x00000176`, `0x00000182`--`0x00000184`, `0x0000018d`--`0x0000018f`, `0x00000195`, `0x00000199`--`0x000001a1` | `0x00000177`--`0x00000181`, `0x00000185`--`0x0000018c`, `0x00000190`--`0x00000194`, `0x00000196`--`0x00000198`, `0x000001a2` |
| `0x000001a3`--`0x000001cf` | `0x000001a3`, `0x000001a5`, `0x000001ba`--`0x000001bc`, `0x000001c2`, `0x000001c8`--`0x000001c9`, `0x000001cb`--`0x000001cd` | `0x000001a4`, `0x000001a6`--`0x000001b9`, `0x000001bd`--`0x000001c1`, `0x000001c3`--`0x000001c7`, `0x000001ca`, `0x000001ce`--`0x000001cf` |
| `0x000001d0`--`0x000001fc` | `0x000001e6`--`0x000001e8`, `0x000001f4`, `0x000001f8`--`0x000001fa`, `0x000001fc` | `0x000001d0`--`0x000001e5`, `0x000001e9`--`0x000001f3`, `0x000001f5`--`0x000001f7`, `0x000001fb` |

Across the combined candidate ranges, 163 of 495 aligned positions have the
same byte value in all three files. Pairwise equality counts are:

- `SMF files` and `dominion with samples`: 192 of 495 positions.
- `SMF files` and `analog only`: 167 of 495 positions.
- `dominion with samples` and `analog only`: 349 of 495 positions.

These are byte-equality counts only. They are not similarity scores and do not
establish shared fields or records.

## Printable-sequence alignment observations

- In the first ten candidate ranges, every file has a printable sequence
  beginning exactly at the candidate offset.
- In the final candidate range, `SMF files` has a printable sequence beginning
  at `0x000001d0`. The other two files have no printable sequence there; their
  first printable sequence in that range begins at `0x000001f0`.
- `dominion with samples` and `analog only` have identical first printable
  sequences at the candidate offsets `0x0000003b`, `0x00000068`,
  `0x00000095`, `0x000000c2`, `0x000000ef`, `0x0000011c`, `0x00000149`,
  and `0x00000176`.
- Their first sequences differ at `0x0000000e` (`JD-800` versus `JD-800H`)
  and `0x000001a3` (`Studio Patches pgm chg` versus
  `Studio Patches pgm chgA`).
- `SMF files` has different first printable sequences from the other two files
  at several aligned candidate offsets. This observation does not identify an
  insertion, deletion, list order, or semantic difference.
- Short printable runs such as `tM`, `XMIDA`, `,`, `*`, `(`, and `$` occur at
  repeated relative positions within some files. Their printability is the
  only conclusion drawn here.

## Explicit unknowns

- The candidate ranges are inspector-defined research windows, not established
  records or fields.
- It is unknown why `dominion with samples` and `analog only` have 349
  pairwise-equal positions or why `SMF files` differs at many aligned
  positions.
- It is unknown whether any printable sequence is a name field, cached text,
  identifier, path fragment, or coincidentally printable binary data.
- The user recollections about MIDI tracks and digital-audio references are not
  connected to any candidate byte by this comparison.
- This comparison does not establish that `SMF files` is MIDI-only, that any
  sample has a particular number of tracks, or that any candidate byte encodes
  audio-related information.
- The meanings of Finder Types `MID2` and `MIDS` remain unknown. Their presence
  is metadata evidence only.
- No byte order, numeric encoding, string encoding, terminator, length, count,
  flag, pointer, checksum, padding rule, or boundary has been identified.
- The reason that some printable sequences begin at aligned offsets is
  unknown. Alignment alone does not establish common semantics.
- Controlled-save experiments changing one independently known property at a
  time would be required before associating byte differences with project
  content or application behavior.
