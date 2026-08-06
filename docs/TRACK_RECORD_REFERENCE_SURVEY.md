# Objective

This survey asks whether numeric interpretations of bytes in the previously
identified 120-byte record sequence coincide with other locations in the
authentic `newest STUFF` data fork. It records arithmetic coincidences and the
bytes observed near candidate destinations. It does not identify a field,
implement parsing, or assign meaning to any value.

# Method

The inspected artifact was the externally held `Opcode/MY MUSIC/newest STUFF`.
Its observed data-fork size was 203,422 bytes (`0x00031a9e`) and its SHA-256 was
`7f97918efd8c8756058b72b4cada4a39a1f0f94655aa2982d4e03cfa8a876114`.
Those measurements match `FIRST_MIDI_RECOVERY_SPIKE.md` and
`TRACK_RECORD_STRUCTURE_SURVEY.md`. The artifact was read only.

The survey reused the 13 complete 120-byte slices from `0x00026860` through
`0x00026e77`. Records are numbered 0 through 12 in file order. Every sliding
two-byte window at relative positions `0x00–0x76` and every sliding four-byte
window at `0x00–0x74` was interpreted as an unsigned big-endian and unsigned
little-endian integer. Thus, the census covered 3,094 16-bit interpretations
and 3,042 32-bit interpretations. Sliding windows intentionally include
unaligned and overlapping byte sequences; inclusion is not a field-boundary
claim.

A value was called *in-range* when `0 <= value < 203422`. Zero was counted in
the census but excluded as a candidate destination. Values at the same relative
position were compared for repetition, order, and range. “Systematic” was
tested as strict increase, strict decrease, or a nonzero constant difference
across all 13 records in file order. No interpretation met that test.

For candidate destinations, a 32-byte window was inspected: 16 bytes before
the numeric location and 16 bytes beginning there, clipped at file boundaries.
The observations below report printable ASCII runs of at least four bytes,
longest zero and `ff` runs, and local Shannon entropy in bits per byte. Entropy
is only a byte-distribution measurement over this small window.

To keep arithmetic coincidence separate from plausible follow-up evidence, a
four-byte family was inspected as a candidate family when at least five records
produced nonzero in-range values at the same relative position. Overlapping
families created from the same source bytes were considered together. Families
formed wholly from constant bytes, printable label bytes, or evident long
zero/`ff` padding were retained in the census but not ranked as strong.

# Candidate numeric values

## Exhaustive census

| Width and byte order | Interpretations | In-range including zero | Nonzero in-range | Distinct nonzero values | Relative positions with a nonzero in-range value |
|---|---:|---:|---:|---:|---:|
| 16-bit big-endian | 1,547 | 1,547 | 1,160 | 255 | 102 |
| 16-bit little-endian | 1,547 | 1,547 | 1,160 | 255 | 102 |
| 32-bit big-endian | 1,521 | 413 | 191 | 68 | 28 |
| 32-bit little-endian | 1,521 | 401 | 179 | 72 | 28 |

Every 16-bit interpretation is at most 65,535 and therefore falls within this
203,422-byte file. The in-range test supplies no discrimination for 16-bit
values. The big- and little-endian 16-bit censuses have equal counts and equal
numbers of distinct values because reversing two bytes maps the complete set
one-to-one. Repeated 16-bit observations are concentrated in the printable
label bytes, zero/`ff` runs, and the repeated byte patterns already tabulated in
`TRACK_RECORD_STRUCTURE_SURVEY.md`. None changes monotonically across all 13
records.

The following table is the complete position-level inventory of nonzero
in-range 32-bit observations. `N` is the number of records with an in-range
reading, `D` is the number of distinct readings, and `value×count` records
repetition. Singleton values are represented by the observed range.

| Order | Relative positions and observations |
|---|---|
| big | `0c N2 D1 00054×2`; `0d N2 D2 05465–05472`; `1b N9 D1 00064×9`; `1c N9 D2 06401×7, 06402×2`; `1e N4 D1 00004×4`; `1f N4 D2 004fd–004ff, 004ff×3`; `25 N7 D4 201dc–202f7, 201dc×4`; `26 N1 00001`; `27 N1 00188`; `28 N1 18803`; `2d N8 D8 03e7a–17916`; `2f N5 D1 00003×5`; `30 N5 D1 00300×5`; `31 N6 D2 0003c–3000a, 3000a×5`; `32 N1 03c00`; `35 N11 D2 0feff×4, 0ffff×7`; `3a N8 D1 01400×8`; `42 N8 D1 00080×8`; `43 N13 D1 08000×13`; `4a N13 D1 00004×13`; `4b N13 D2 00456×5, 0045c×8`; `57 N13 D9 0002c–0004b, 00030×3, 00039×2, 00041×2`; `58 N13 D9 02c00–04b00, 03000×3, 03900×2, 04100×2`; `5b N13 D9 14a00–23280, 16800×3, 1ab80×2, 1e780×2`; `5e N5 D2 08000–08800, 08000×4`; `63 N13 D2 00400×2, 00401×11`; `66 N2 D2 00300–045ff`; `67 N1 30000` |
| little | `14 N5 D5 03120–03820`; `15 N6 D6 00031–03231`; `16 N1 00032`; `17 N1 03223`; `18 N7 D3 00032–07370, 06b63×2, 07370×4`; `19 N6 D2 0006b×2, 00073×4`; `21 N1 0fd04`; `22 N1 000fd`; `2a N1 30388`; `2b N10 D6 00303–05800, 05400×5`; `2c N5 D1 00054×5`; `2f N1 0b478`; `30 N6 D2 000b4–30000, 30000×5`; `33 N11 D6 00400–03c00, 00a00×6`; `38 N8 D1 080ff×8`; `3f N8 D1 0c800×8`; `40 N8 D1 000c8×8`; `41 N5 D1 01700×5`; `46 N13 D1 01500×13`; `47 N13 D1 00015×13`; `4f N13 D13 0081c–0f849`; `50 N13 D12 00008–000f8, 00080×2`; `5c N5 D3 04a01–06801, 06801×3`; `61 N13 D1 00400×13`; `64 N13 D2 00400×2, 10400×11`; `66 N1 30000`; `67 N2 D2 00300–04500`; `68 N2 D2 00003–00045` |

The two rows enumerate every 32-bit relative position that produced at least
one nonzero in-range value. They do not describe those values as locations
encoded by the records.

# Referenced-region observations

The heading uses “referenced-region” only as the name required for this survey
section. The observations establish numeric destinations, not references.

## Relative `0x25–0x28`, big-endian

Seven records produced four values in the narrow range `0x201dc–0x202f7`, a
span of 284 bytes. Records 2, 3, 5, and 7 all produced `0x201dc`; record 9
produced `0x201de`; record 12 produced `0x2021c`; and record 10 produced
`0x202f7`.

| Value | Records | Direct 32-byte-window observation |
|---|---|---|
| `0x201dc` | 2, 3, 5, 7 | entropy 4.539; 21 printable bytes; longest zero run 1; no `ff`; printable run `` `)ux> `` |
| `0x201de` | 9 | entropy 4.515; 22 printable bytes; longest zero run 1; no `ff`; two short printable runs |
| `0x2021c` | 12 | entropy 4.191; 23 printable bytes; longest zero run 1; no `ff`; three short printable runs |
| `0x202f7` | 10 | entropy 4.476; 25 printable bytes; longest zero run 1; no `ff`; two printable runs |

All four windows are byte-diverse and contain recurring bytes in the `0x81–82`
range interspersed with printable bytes and isolated zeroes. No long zero or
`ff` run was observed. The content does not identify a format.

## Relative `0x5b–0x5e`, big-endian

All 13 records produced one of nine in-range values. Repetitions were
`0x16800` in records 2, 3, and 5; `0x1ab80` in records 7 and 12; and `0x1e780`
in records 9 and 10. The other values were `0x23280`, `0x15180`, `0x1e3c0`,
`0x15900`, `0x14a00`, and `0x17e80`.

| Value(s) | Direct 32-byte-window observation |
|---|---|
| `0x16800` | entropy 4.301; 22 printable bytes; no zero or `ff`; four short printable runs |
| `0x1ab80` | entropy 4.140; 20 printable bytes; longest zero run 1; two short printable runs |
| `0x1e780` | entropy 4.054; 23 printable bytes; no zero or `ff`; five short printable runs |
| `0x15180`, `0x23280` | entropy 3.688 and 3.162; 24 and 19 printable bytes; no zero or `ff` runs |
| `0x14a00`, `0x15900`, `0x17e80` | entropy 1.809–2.267; each has a longest zero run of 9; `0x17e80` also has one `ff` |
| `0x1e3c0` | entropy 2.789; eight printable bytes; longest zero run 8; one `ff`; printable run `ck 4` |

The destinations do not form one narrow region and their observed content is
heterogeneous. The repeated values and coverage of all records are evidence
for a controlled test, but the destination windows do not establish a common
structure.

## Relative `0x2d–0x30`, big-endian family

At `0x2d`, eight records produced eight values spanning `0x03e7a–0x17916`.
Their windows ranged from entropy 2.833 to 4.301. One (`0x17916`) contains the
printable run `Meter Track` and seven `ff` bytes; two contain zero-rich binary
data; the remaining five contain byte-diverse data with multiple short
printable runs. The wide destination span and heterogeneous observations do
not show a common region.

The overlapping positions `0x2f` and `0x30` yield `0x00003` and `0x00300` in
the same five records. The `0x00300` window has entropy 1.223, a 21-byte zero
run, and no printable run. These results arise from overlapping source bytes
and provide no independent corroboration.

## Relative `0x31–0x34`, big-endian

Five records produce `0x3000a`; record 0 produces `0x0003c`. The `0x3000a`
window has entropy 3.519, 24 printable bytes, one isolated zero, and three
short printable runs. The `0x0003c` window overlaps the printable string
`JD-990 w/Vintage5` near the beginning of the file. The two destinations are
widely separated, and the five repeated observations alone do not establish a
relationship.

## Relative `0x4b–0x4e`, big-endian

All records produce either `0x00456` (five records) or `0x0045c` (eight
records), two destinations six bytes apart. Their overlapping windows each
have entropy 2.353, nine zero bytes in the longest aggregate window region,
one `ff`, and no printable run. This is a directly observed common region. The
small values and source overlap with repeated bytes at `0x4c–0x4d` leave open
whether the coincidence is structural or arithmetic.

## Relative `0x57–0x5a`, big-endian family

All records yield nine small values at `0x57` (`0x2c–0x4b`) and the same
byte-derived values shifted by eight bits at `0x58` (`0x2c00–0x4b00`). The
`0x57` destinations lie among printable device-like strings near the beginning
of the file. The `0x58` destinations are heterogeneous: `0x2e00` has a
30-byte zero run; `0x2c00` has an 18-byte `ff` run; `0x3000` overlaps `ack 4`;
and `0x3900`, `0x4100`, and `0x4b00` are printable-dense byte sequences. The
one-byte shift producing two scales is observable evidence of an overlapping
interpretation, not evidence selecting either scale.

## Relative `0x63–0x66`, big-endian

Records 0 and 12 produce `0x00400`; the other 11 produce `0x00401`. The two
overlapping destination windows are zero-rich (longest zero runs 13 and 12),
contain one `ff`, and have entropy 1.993 and 2.174. A little-endian overlapping
window at `0x64` similarly produces `0x00400` twice and `0x10400` 11 times;
the `0x10400` window is also zero-rich, with entropy 1.872. The paired results
show source-byte regularity but do not select a byte order or destination.

## Other repeated destination families

- Big-endian `0x1b–0x1c` produces `0x00064` nine times and `0x06401` or
  `0x06402` nine times. The source overlaps the zero padding immediately after
  the printable label. Target `0x64` overlaps `Juno-106/Vin`; `0x6401` and
  `0x6402` are byte-diverse windows.
- Big-endian `0x35`, `0x3a`, `0x42–0x43`, and `0x5e` produce repeated
  `0x0feff/0x0ffff`, `0x01400`, `0x00080/0x08000`, and
  `0x08000/0x08800`. Their source bytes overlap constant or repeated
  zero/`ff` patterns. The target windows vary from zero-rich to byte-diverse.
- Little-endian `0x38–0x41` produces repeated `0x080ff`, `0x0c800`,
  `0x000c8`, and `0x01700`. These are overlapping interpretations of one
  repeated source-byte family. Destination `0x1700` is a visible `78 43`
  repetition; the other windows differ.
- Little-endian `0x46–0x47` produces constants `0x01500` and `0x00015` in
  all records. Both source windows are wholly constant across the sample.
- Little-endian `0x4f–0x50` produces an in-range value in every record, but
  `0x4f` has 13 distinct values and `0x50` mostly produces one-byte-sized
  destinations. The latter windows overlap printable strings near file start.
- Little-endian `0x5c` produces `0x06801` three times plus `0x05901` and
  `0x04a01`. All three destination windows are printable-dense and
  byte-diverse. Only five records produce in-range readings.

# Strong candidates

“Strong” here means strongest for a future controlled experiment, not an
established reference field.

1. **Relative `0x25–0x28`, big-endian.** Seven in-range observations converge
   within 284 bytes, four records repeat exactly `0x201dc`, and all four
   inspected destinations are byte-diverse without padding runs. This is the
   clearest common-region coincidence in a variable source window.
2. **Relative `0x5b–0x5e`, big-endian.** All 13 records produce in-range
   values, three destinations repeat across record groups, and inspected
   windows include both byte-diverse and zero-rich regions. Full record
   coverage makes controlled change behavior directly testable, although the
   destinations span 59,520 bytes.
3. **Relative `0x4b–0x4e`, big-endian.** All records select one of two nearby
   zero-rich destinations, six bytes apart. The strong repetition is tempered
   by small values and repeated source bytes.
4. **Relative `0x31–0x34`, big-endian.** Five records repeat `0x3000a` and
   reach the same printable-dense window. One other record reaches a distant
   file-start string, so the family is less consistent than the first three.

# Weak candidates

- Relative `0x2d–0x30` big-endian is variable and reaches several kinds of
  content, including another `Meter Track` occurrence, but its eight distinct
  values span a wide region and do not converge.
- Relative `0x57–0x5a` big-endian covers all records and repeats values, but
  adjacent sliding windows yield the same source bytes at two scales and the
  destination content is heterogeneous.
- Relative `0x63–0x66` big-endian and overlapping `0x64–0x67`
  little-endian show strong repetition but no evidence chooses between the
  competing byte orders or their zero-rich destinations.
- Relative `0x5c–0x5f` little-endian reaches three printable-dense regions,
  but only five records are in-range and three share one value.
- Relative `0x1b–0x1f` big-endian yields repeated destinations but overlaps
  post-label padding and produces small values.

# Eliminated candidates

Elimination means excluded from the ranked experiment list under this survey's
criteria, not proved incapable of carrying information.

- No 16-bit interpretation is retained solely because it is in-range: all
  3,094 interpretations pass that test, so it cannot distinguish a candidate.
- No candidate shows strict increase, strict decrease, or a constant nonzero
  step across all records in file order.
- Sliding windows overlapping relative `0x0f–0x19` are excluded from ranking
  when their in-range values are direct numeric readings of printable label
  bytes or their adjacent terminators.
- Families wholly constant across all records, including little-endian
  `0x46–0x47` and big-endian `0x4a`, cannot correlate record-to-record
  variation in this sample and are excluded from ranking.
- Values created predominantly by long zero or `ff` source runs, including the
  many candidates at relative `0x00–0x0e` and `0x69–0x77`, are excluded from
  ranking. Their repetitions are already explained by directly observed
  padding-like runs.
- Little-endian `0x4f–0x50` is excluded from the strong list because the
  adjacent window transforms source patterns of the form `XX 00 00 00` into
  small file-start values. The resulting printable target strings do not
  establish a relationship.

# Unknowns

- It is unknown whether any numeric interpretation corresponds to a stored
  location, index, count, identifier, measurement, flag combination, or none
  of these.
- It is unknown whether byte order or field alignment is uniform within these
  records.
- It is unknown whether a candidate destination should be absolute, relative
  to another base, scaled, masked, signed, or not location-like at all.
- It is unknown whether short printable runs in byte-diverse regions are
  structured text or chance occurrences.
- It is unknown whether the candidate families remain stable in another
  project or after a controlled save.
- No candidate is sufficiently evidenced to recover or generate an SMF.

# Recommended next experiment

Perform one controlled same-length track-label change on a copy of `newest
STUFF`, then compare the original and saved data forks. Before interpreting any
changed byte, test whether the 13-record cadence remains stable and record the
before/after bytes at relative windows `0x25–0x28`, `0x5b–0x5e`,
`0x4b–0x4e`, and `0x31–0x34`. If a candidate value changes, inspect both its
before and after numeric destinations using the same 32-byte-window method.
This single-variable experiment can test whether the ranked coincidences track
a controlled record change without assigning meaning in advance.
