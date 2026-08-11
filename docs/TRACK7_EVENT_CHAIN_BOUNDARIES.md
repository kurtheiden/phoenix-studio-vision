# Objective

This investigation tests whether the 143 visible Track 7 rows have local
binary boundaries before the established row-4 anchor and after row 143. It is
read-only and does not infer unsupported framing.

# Evidence boundary

The authentic baseline is the uncompressed project
`.../Experiment 007 - Untouched Baseline/newest STUFF baseline`. The bounded
parser is used only from known local offsets; no whole-file resynchronization is
performed.

# Existing validated sequence

The prior strict validation starts at row 4 timing offset `0x31c1d`, with
properties at `0x31c1f`, and ends after row 143 at cursor `0x31f9c`. Rows 4–143
are 140 consecutive candidates with 560/560 property-field matches.

# Rows 1–3 ground truth

The external ground-truth CSV records:

| Row | Position | Pitch | Duration | Attack | Release |
|---:|---|---|---:|---:|---:|
| 1 | `25·3·240` | D1 | 76 | 127 | 127 |
| 2 | `25·4·0` | D1 | 86 | 127 | 127 |
| 3 | `25·4·240` | D1 | 187 | 127 | 54 |

# Immediate predecessor analysis

Working backward locally from row 4 gives unambiguous property matches for
rows 3 and 2. Row 1's property bytes also match, but its preceding timing VLQ
is not unambiguously identifiable in the local bytes.

# Rows 1–3 binary alignment

| Row | Timing offset/bytes/value | Property offset/bytes | Duration/value | Result |
|---:|---|---|---|---|
| 1 | not established | `0x31c0c`: `26 7f 7f 4c` | `4c` = 76 | four properties match |
| 2 | `0x31c10`: `81 70` = 240 | `0x31c12`: `26 7f 7f 56` | `56` = 86 | exact; row 2−row 1 = 240 |
| 3 | `0x31c16`: `81 70` = 240 | `0x31c18`: `26 7f 36 81 3b` | `81 3b` = 187 | exact; row 3−row 2 = 240 |

Row 4 then begins at `0x31c1d` with `81 65` = 229 and properties
`24 7f 5c 83 3a`; row 4−row 3 is 229. Thus all three rows are immediately
preceding property structures, while only rows 2–4 have established leading
timing fields.

# Pre-sequence bytes

The conservative window before row 1 contains:

`82 f1 30 90 | 26 7f 7f 4c | 81 70 ...`

The four property bytes at `0x31c0c` are strong local evidence for row 1. The
preceding `82 f1 30 90` does not form an unambiguous instance of the established
timing/property/duration sequence at that boundary. Nearby values are not
called a count, header, or track identifier merely because one byte could be
143.

# Final-event boundary

The final visible candidate is row 143: timing `81 70` at `0x31f96`, properties
`24 7f 7f 76` at `0x31f98`, duration 118, and next cursor `0x31f9c`.

# Post-sequence bytes

Immediately after the cursor are:

`ff fa b9 2f ff 2f 00 29 00 00 00 69 00 06 00 00 00 00 ...`

The first four bytes can be consumed syntactically as a VLQ, but the next
property byte would be `ff`, outside the observed pitch/velocity byte range.
Therefore conservative event validation stops at `0x31f9c`; this is not a
claim that the bytes are a terminator, footer, or SMF end marker. A different
structure may begin there.

# 143-event reconciliation

The screenshots reconcile to exactly 143 unique rows. Rows 1–3 now have local
property matches, and rows 4–143 have strict sequential binary candidates.
Thus all 143 visible property structures are mechanically accounted for. The
initial timing field for row 1 remains unresolved, so this is not yet a claim
that all 143 rows have independently framed timing prefixes.

# Complete timing evidence

Rows 2–4 add exact start-to-start checks: 240, 240, and 229 respectively. Row
1 has no tested prior displayed start and no unambiguous local timing prefix.
Together with the prior 140/140 result, 142 adjacent displayed differences are
now exact where the later event has an established timing field.

# Framing hypotheses

**Strongly supported:** a contiguous local property chain covers rows 1–143;
rows 2–143 have established timing/property/duration decoding; the conservative
post-row-143 boundary is at `0x31f9c`.

**Plausible:** bytes before `0x31c0c` and after `0x31f9c` belong to a containing
record or neighboring structure.

**Unknown:** actual track header/footer, initial timing ownership, event count
encoding, channel/status representation, and whether post-sequence bytes are a
footer, another event type, or unrelated data.

# Evidence supported

- All 143 screenshot rows are represented by matching local property bytes.
- Rows 2–143 form a strict sequential event-property chain.
- No heuristic backward scan was used to assign row 1's timing.
- The final conservative stop is caused by an out-of-range property byte after
  the last validated event, not by a demonstrated end marker.

# Unknowns

Exact Track 7 start/end framing, row-1 timing prefix, semantics of surrounding
bytes, and generality beyond this bounded region remain unresolved.

Comparative inspection found five other bounded, unidentified multi-structure
candidate chains with a nearby `2c c4 b2` marker and analogous
`ff fa ?? ?? ff 2f 00 29 ...` post-context. This makes the local bytes
plausibly record-related, but does not identify Track 7's container.

# Single recommended next step

Perform a narrow byte/record comparison at `0x31c08–0x31c0c` and
`0x31f9c–0x31fb0` across an independently preserved native save or adjacent
track record to identify framing without treating either context as note data.
