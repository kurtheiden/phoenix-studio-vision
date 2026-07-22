# Studio Vision Project File Structural Survey

This document records a byte-level, read-only survey of one authentic Studio
Vision project already identified in the Phoenix research notes as
`newest-stuff-001`. It describes observations only. Candidate boundaries are
measurement aids, not inferred sections, records, or fields.

## Evidence source and method

- Inspected path: `samples/newest STUFF`.
- The file was read without modification. It was not opened in Studio Vision,
  imported, converted, repaired, or rewritten.
- SHA-256 and the whole-file byte histogram cover the complete data fork.
- Printable strings are maximal runs of bytes from `0x20` through `0x7e`, with
  a minimum length of four bytes.
- Local entropy, printable-string byte share, and zero-byte share were measured
  in fixed, nonoverlapping 4,096-byte windows beginning at offset zero. The
  final window contains 3,505 bytes.
- Repetition observations use exact 16-byte sequences. Overlapping sequences
  were counted independently.

## File and Finder metadata

### Observations

- Filename: `newest STUFF`.
- Filesystem object: Regular file.
- Filename extension: None.
- Logical size and data-fork size: 171,953 bytes (`0x29fb1`).
- Last data-fork offset: `0x29fb0`.
- Data-fork SHA-256:
  `c44d415a4b69d56abd5680652ed99039a4f9ca9afd281898601ccc14026aebec`.
- Filesystem creation time: `1997-04-11 07:07:09 +0100`.
- Filesystem modification time: `1997-04-11 07:07:09 +0100`.
- Finder Type: `MID2` (`4d 49 44 32`).
- Finder Creator: `MIDA` (`4d 49 44 41`).
- Extended attributes:
  - `com.apple.FinderInfo` -- 32 bytes.
  - `com.apple.quarantine` -- 15 bytes.
- No `com.apple.ResourceFork` extended attribute was present at the inspected
  path.

### Evidence

The first eight bytes of `com.apple.FinderInfo` are:

```text
4d 49 44 32 4d 49 44 41
```

These bytes are the direct source of the Finder Type and Creator observations.
The complete FinderInfo bytes are:

```text
4d 49 44 32 4d 49 44 41 01 00 00 00 00 00 00 00
00 00 00 00 00 00 00 00 fa e0 91 8d 00 00 00 00
```

### Unknowns

- The meanings of FinderInfo bytes after the Type and Creator values are not
  examined here.
- No conclusion about the original presence or absence of a resource fork is
  drawn from its absence on this inspected copy.
- Filesystem timestamps are recorded as observed metadata; their transfer
  history and provenance have not been independently established.

## Entropy and byte-characteristic summary

### Whole-file observations

- Shannon entropy: 6.346559 bits per byte.
- Printable ASCII runs of at least four bytes: 10,023.
- Bytes in those runs: 66,444 of 171,953 bytes (38.64%).
- Longest reported printable run: 219 bytes at `0x01332d`. Its mixture of
  punctuation and letters is recorded as a printable-byte observation only.

### Fixed-window observations

Selected 4 KiB windows illustrate the measured range:

| Offset range | Entropy | Printable-run bytes | Zero bytes |
| --- | ---: | ---: | ---: |
| `0x000000`--`0x000fff` | 2.9888 | 4.57% | 62.35% |
| `0x001000`--`0x001fff` | 3.8140 | 18.38% | 56.30% |
| `0x002000`--`0x002fff` | 4.0791 | 8.84% | 43.09% |
| `0x003000`--`0x003fff` | 6.0085 | 41.58% | 9.59% |
| `0x004000`--`0x004fff` | 5.4664 | 72.58% | 4.17% |
| `0x007000`--`0x007fff` | 6.5967 | 10.47% | 7.50% |
| `0x008000`--`0x008fff` | 5.7361 | 29.83% | 24.15% |
| `0x00d000`--`0x00dfff` | 5.8717 | 77.37% | 0.10% |
| `0x012000`--`0x012fff` | 5.1448 | 36.50% | 23.56% |
| `0x018000`--`0x018fff` | 6.5848 | 34.84% | 2.59% |
| `0x01e000`--`0x01efff` | 5.1826 | 21.73% | 28.32% |
| `0x021000`--`0x021fff` | 5.8367 | 81.45% | 2.12% |
| `0x029000`--`0x029fb0` | 5.6947 | 40.00% | 23.28% |

Across all fixed windows, observed entropy ranges from 2.9888 to 6.6031 bits
per byte, printable-run byte share ranges from 4.57% to 81.45%, and zero-byte
share ranges from 0.10% to 62.35%.

## Printable strings by offset range

The following are representative literal strings from the complete scan. The
grouping is by byte offset only; it does not assign meaning to a containing
binary region. Strings resembling names or labels are included verbatim, while
the thousands of punctuation-heavy printable runs are summarized by their
count above.

### `0x000000`--`0x001fff`

- `0x00000e`: `IAC Bus #1`
- `0x00003b`: `JD-800s #1`
- `0x000068`: `JD-990s #1`
- `0x000095`: `Juno-106#1`
- `0x000149`: `Quicktime Music`
- `0x0001d0`: `Studio Patches pgm chg`
- `0x000f9b`: `VISN`
- `0x00104b`: `Studio Vision Pro`
- `0x001070`: `HHard Disk 2149:Music:Opcode:Studio Vision Pro:KURT SV3.0 Bundle 12-16-95`
- `0x001111`: `General MIDI`
- `0x001199`: `@Macintosh HD:System Folder:OMS Folder:Factory Names:General MIDI`
- `0x00130a`: `Roland`
- `0x00193a`: `Studio 4`
- `0x001943`: `Modem/Printer `
- `0x001a4e`: `Opcode`
- `0x001a66`: `Studio Patches`

### `0x002000`--`0x002fff`

- `0x002469`: `Console 1`
- `0x00251d`: `Console 2`
- `0x0025d1`: `Console 3`
- `0x002685`: `Console 4`
- `0x002875`: `Meter Track`
- `0x0028ed`: `Tempo Track`
- `0x002965`: `Track 1`
- `0x0029dd`: `Track 2`
- `0x002e8d`: `Track 12`
- `0x002f05`: `Track 11 #2`

### `0x003000`--`0x007fff`

- `0x003019`: `Xperiment- :D  !`
- `0x0031d4`: `Velo-Crunch`
- `0x0032f9`: `Golden Sands`
- `0x005111`: `Sharp Bass`
- `0x0057ac`: `Empty Patch`
- `0x00590d`: `Taj Mahal`
- `0x005cde`: `Dist Gtr 1`
- `0x00640d`: `Wet Fretls`
- `0x007cbe`: `Crunch Split`

### `0x008000`--`0x00ffff`

- `0x00800c`: `Time flies`
- `0x0082cd`: `Wave Bells`
- `0x008a0b`: `Bells for her`
- `0x008a6d`: `Meter Track`
- `0x008ae5`: `Tempo Track`
- `0x008b5d`: `Track 1`
- `0x0090fd`: `Track 13`
- `0x009284`: `Lingering Bells`
- `0x00acaf`: `SquareLead 1`
- `0x00b010`: `Pipe Organ 2`
- `0x00c056`: `ORBit Pad`
- `0x00c3fc`: `That's Funky`
- `0x00e8d3`: `JP-8 Pad`
- `0x00ed3d`: `Running Pad`
- `0x00fc71`: `Meter Track`
- `0x00fce9`: `Tempo Track`

### `0x010000`--`0x017fff`

- `0x0100c8`: `Sharp Bass`
- `0x0103e8`: `Analog Seq`
- `0x011191`: `Meter Track`
- `0x011209`: `Tempo Track`
- `0x011281`: `Track 1`
- `0x0115e8`: `Pick Bass`
- `0x011972`: `The Big Spin`
- `0x01226f`: `Echo Drops`
- `0x012ab7`: `Chorused Organ`
- `0x013c08`: `D-50 Stack`
- `0x014f26`: `mission impossibl`
- `0x0150f0`: `kick drum`
- `0x015168`: `stick thingy`
- `0x016770`: `Flute mod`
- `0x0168e7`: `Brass Sect 2`
- `0x017305`: `Meter Track`

### `0x018000`--`0x01ffff`

- `0x019035`: `OBig Poly`
- `0x019969`: `Moist Bass`
- `0x01ac35`: `MG Solo`
- `0x01aec1`: `Meter Track`
- `0x01af39`: `Tempo Track`
- `0x01b571`: `Blade Racer`
- `0x01bdaf`: `Hihat Tekno`
- `0x01c1d6`: `4 Hits 4 You`
- `0x01cdf7`: `Gt. Fretnoise`
- `0x01ce32`: `Classic Sweeper`
- `0x01d7f7`: `Velocity Pad !`
- `0x01de49`: `Space Harp`
- `0x01e5af`: `Square Keys`
- `0x01e707`: `Get on up & Dance`
- `0x01f070`: `Saw Mass`

### `0x020000`--`0x027fff`

- `0x0201a7`: `Saw Mass`
- `0x021026`: `Multi Bass`
- `0x022068`: `Bark Bass`
- `0x023f74`: `Mondo Chord1`
- `0x024039`: `Raw Oscillators`
- `0x024611`: `Big Machine Dies`
- `0x024987`: `Thick Matrix`
- `0x024bdf`: `Jurrasic Park`
- `0x024c41`: `Meter Track`
- `0x025296`: `Velo Tekno 1`
- `0x0255e8`: `Auto TB-303`
- `0x025731`: `Belly Lead`
- `0x025d98`: `Ode to Clarke`
- `0x0263b9`: `Empty Patch`
- `0x0266b2`: `Stereoww Bs`
- `0x027da2`: `Wavox`

### `0x028000`--`0x029fb0`

- `0x0282d2`: `Ming Dynasty`
- `0x028a2e`: `Over the Top`
- `0x028a90`: `Meter Track`
- `0x028b08`: `Tempo Track`
- `0x028b80`: `Track 1`
- `0x028d7f`: `Clk AnalogBs`
- `0x0290b3`: `Intentions`
- `0x02935c`: `Meter Track`
- `0x0293d4`: `Tempo Track`
- `0x029ca3`: `Timpani 2  /`
- `0x029dde`: `BD  Roll   /`

## Recurring binary observations

- The first 11 printable names begin at offsets `0x00000e`, `0x00003b`,
  `0x000068`, `0x000095`, `0x0000c2`, `0x0000ef`, `0x00011c`, `0x000149`,
  `0x000176`, `0x0001a3`, and `0x0001d0`. Consecutive starting offsets differ
  by exactly `0x2d` bytes.
- In the range beginning at `0x002875`, `Meter Track`, `Tempo Track`, and the
  following `Track` strings recur at starting offsets separated by `0x78`
  bytes. Comparable `Meter Track`/`Tempo Track`/`Track` sequences occur at
  multiple later offsets listed above.
- The exact 16-byte sequence
  `ff ff 80 00 00 14 00 c8 00 c8 00 00 00 00 80 00` occurs 106 times. Its
  first observed starts include `0x00289d`, `0x002915`, `0x002a7d`,
  `0x008a95`, `0x008b0d`, `0x008fbd`, `0x0090ad`, and `0x009125`.
- The exact 16-byte sequence
  `ff ff ff ff ff ff ff ff ff ff ff ff ff ff 54 72` occurs 93 times. Its
  first observed starts include `0x002957`, `0x0029cf`, `0x002a47`,
  `0x002abf`, `0x002b37`, `0x002baf`, `0x002c27`, and `0x002c9f`.
- Several high-frequency 16-byte results overlap by one byte and therefore
  describe the same longer recurring byte neighborhood. Counts are occurrence
  counts, not inferred record counts.
- Exact repeated printable strings include `Meter Track`, `Tempo Track`,
  numbered `Track` labels, `Saw Mass`, `Empty Patch`, and other strings listed
  at multiple offsets above.

## Candidate boundaries and approximate offset map

These boundaries are candidates based only on fixed-window measurements and
visible changes in strings or repetition. Their 4 KiB alignment reflects the
measurement method and does not establish an exact on-disk boundary.

| Approximate range | Direct observations |
| --- | --- |
| `0x000000`--`0x002fff` | Three lowest-entropy 4 KiB windows at the start of the file; 43.09%--62.35% zero bytes; device, path, console, and track strings; `0x2d`- and `0x78`-spaced string offsets. |
| `0x003000`--`0x007fff` | Entropy rises to 5.4664--6.5967; zero-byte share falls to 4.17%--9.59%; many punctuation-heavy printable runs and scattered short labels. |
| `0x008000`--`0x00afff` | Entropy changes from 5.7361 to 4.8263 and then 5.3167; zero-byte share reaches 24.15% in the first window; repeated track strings and exact 16-byte patterns occur. |
| `0x00b000`--`0x011fff` | Entropy varies from 5.8717 to 6.6031 except for adjacent lower windows; printable-run share ranges from 16.28% to 77.37%; repeated labels and binary patterns continue. |
| `0x012000`--`0x017fff` | The first window has 23.56% zero bytes and 5.1448 entropy, followed by alternating windows with 5.6210--6.2080 entropy and 7.35%--24.54% zero bytes. |
| `0x018000`--`0x01efff` | The first window rises to 6.5848 entropy and 2.59% zero bytes; later windows fall to 5.1826 entropy and reach 32.64% zero bytes. |
| `0x01f000`--`0x023fff` | The first window changes to 6.0120 entropy and 62.13% printable-run bytes; the range contains the file's highest printable share, 81.45%, at `0x021000`--`0x021fff`. |
| `0x024000`--`0x029fb0` | Entropy remains between 5.6947 and 6.1758; zero-byte share ranges from 5.10% to 24.51%; repeated labels and patterns continue through the final partial window. |

The largest adjacent 4 KiB entropy changes occur at candidate boundaries
`0x003000` (4.0791 to 6.0085), `0x00b000` (5.3167 to 6.4286), `0x013000`
(5.1448 to 6.1255), `0x009000` (5.7361 to 4.8263), and `0x018000`
(5.6818 to 6.5848). These are ranked measurement changes only.

## Similar or repetitive regions

- The opening offsets through `0x0001e5` contain 11 similarly spaced
  printable-name starts.
- Ranges around `0x002875`, `0x008a6d`, `0x00fc71`, `0x011191`, `0x01256f`,
  `0x0142f3`, `0x014f88`, `0x017305`, `0x01aec1`, `0x01ca0e`, `0x01d580`,
  `0x01d9f1`, `0x01e769`, `0x024c41`, `0x025dfa`, `0x028a90`, `0x02935c`,
  and `0x029a30` contain recurring `Meter Track`, `Tempo Track`, and nearby
  numbered `Track` strings.
- The high-frequency exact 16-byte patterns occur in several of those ranges
  and also at other offsets. Similarity here means byte equality or repeated
  literal strings only.
- Punctuation-heavy printable runs recur across much of `0x003000` through the
  end of the file. Their printability does not establish that they are text.

## Conclusions limited to observations

- The inspected data fork is not uniform: fixed-window entropy, zero-byte
  share, printable-run share, exact repeated bytes, and literal-string density
  vary substantially by offset.
- Exact byte sequences and similarly spaced printable strings recur at many
  offsets.
- The measured changes permit an approximate observational map, but do not
  establish section boundaries, record layouts, field types, or semantics.

## Unknown regions and limitations

- Every binary region remains semantically unidentified.
- The bytes preceding, between, and following printable strings are
  unexamined beyond the reported aggregate measurements and exact-pattern
  search.
- The source and meaning of high-entropy, low-entropy, zero-rich,
  punctuation-rich, and repetitive regions are unknown.
- No record layout, byte order, pointer, length field, offset table,
  compression, checksum, timestamp, event representation, or section grammar
  has been inferred.
- Candidate boundaries are limited by the chosen 4,096-byte window size and
  alignment. Measurements with another window size could identify different
  change points.
- Exact 16-byte repetition can arise from overlapping or nested byte runs; no
  structural unit is assigned to an occurrence.
- Relationships between these bytes and Studio Vision application resources,
  preferences, OMS artifacts, or other project files remain unknown.
