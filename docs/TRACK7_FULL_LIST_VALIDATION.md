# Objective

This report validates the bounded Track 7 event-chain model against the three
overlapping Studio Vision Pro List Window screenshots. It transcribes the
visible rows, reconciles the displayed event count, compares rows under strict
sequential alignment, and tests displayed positions against binary timing
intervals. It does not modify source artifacts, emit MIDI, or broaden the
parser into a whole-file heuristic.

# Evidence sources

The screenshots were positively found in:

`/Users/kurtheiden/Documents/Phoenix Research/Track 7 Ground Truth/`

with the expected names `Track 7 List 1.png`, `Track 7 List 2.png`, and
`Track 7 List 3.png`. The authentic uncompressed baseline was positively
identified as:

`/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline`

The existing bounded parser was used without code changes. The external,
machine-readable artifacts are `track7-list-ground-truth.csv` and
`track7-binary-validation.csv` in the screenshot directory. Neither is in the
repository.

# Screenshot transcription

Overlap reconciliation produced 143 unique rows. The transcription preserves
displayed positions including `.120`, `.229`, `.240`, and `.469`, and records
the numeric duration, attack, and release columns. No field was changed to
make a binary value fit; the first three rows are retained as a visible prefix
before the established binary alignment.

The first rows are:

| Index | Position | Pitch | Duration | Attack | Release | Source |
|---:|---|---|---:|---:|---:|---|
| 1 | `25·3·240` | D1 | 76 | 127 | 127 | List 1 |
| 2 | `25·4·0` | D1 | 86 | 127 | 127 | List 1 |
| 3 | `25·4·240` | D1 | 187 | 127 | 54 | List 1 |
| 4 | `25·4·469` | C1 | 442 | 127 | 92 | List 1 |

The complete 143-row transcription is in the external CSV, with first-seen
screenshot provenance and notes for the alignment boundary.

# Event-count reconciliation

The screenshots contain 52, 52, and 52 visible row lines. There are 13
duplicate rows across the two overlap regions, yielding exactly 143 unique
rows. Thus the displayed `143 Events` count is fully accounted for by the
screenshots. No top or bottom row is missing from the visible transcription.

This establishes 143 visible List Window rows, but does not independently prove
that the Studio Vision event count means only note rows; that interpretation
remains a UI-level unknown.

# Alignment method

The alignment was not selected by screenshot-value searching. The controlled
fourth event is the anchor:

- transcription row 4, `25·4·469`, corresponds to binary timing offset
  `0x00031c1d` and property offset `0x00031c1f`;
- row 5 corresponds to `0x00031c24` / `0x00031c26`;
- row 6 corresponds to `0x00031c2b` / `0x00031c2d`.

The parser then advances only through each returned cursor. Binary candidate
indices 1–140 therefore correspond to transcription rows 4–143. The absolute
Track 7 event numbers above the visible window are unknown; “fourth/fifth/sixth”
is the controlled-experiment terminology for this locally anchored sequence,
not a claim about the first event in the complete track.

# Preregistered rows 7–11

The preceding preregistration was preserved unchanged. The screenshot rows
match all five predicted rows and all 20 musical-property fields:

| Row | Predicted / observed pitch | Attack | Release | Duration | Result |
|---:|---|---:|---:|---:|---|
| 7 | D1 | 121 | 125 | 71 | exact |
| 8 | D1 | 127 | 62 | 214 | exact |
| 9 | C1 | 118 | 105 | 85 | exact |
| 10 | C1 | 127 | 92 | 442 | exact |
| 11 | D1 | 127 | 86 | 245 | exact |

Preregistered result: **20 / 20 fields correct**. This result is kept
separate from the later full-dataset comparison, which uses the supplied
screenshots as ground truth.

# Binary chain decoding

The bounded walk used the explicit evidence-backed range beginning at
`0x00031c1d`. It decoded 140 consecutive candidates corresponding to rows
4–143. Each record in the external validation CSV retains timing-field offset,
VLQ bytes and value, property offset, property bytes, duration bytes and
value, accumulated interval, and next cursor.

The first three visible rows precede the established parser start and are not
silently resynchronized. Boundary follow-up found matching local property
bytes for rows 1–3 at `0x31c0c`, `0x31c12`, and `0x31c18`; rows 2 and 3 also
have immediately preceding `81 70` timing fields, while row 1's timing prefix
remains ambiguous. The strict 140-candidate comparison below is intentionally
preserved as the historical full-list validation result.

# Full property comparison

Across the 140 strictly aligned rows (560 property fields):

| Field | Matches / comparisons |
|---|---:|
| Pitch | 140 / 140 |
| Attack velocity | 140 / 140 |
| Release velocity | 140 / 140 |
| Duration | 140 / 140 |
| Overall | 560 / 560 |

Complete four-field row matches: **140 / 140**. Rows with at least one
mismatch: zero. First mismatch: none.

# Consecutive structure validation

The five-field pattern persists across every screenshot row with an established
binary counterpart: 140 consecutive rows, from the controlled fourth row
through transcription row 143. All timing and duration VLQs decode, every
pitch-like byte is in `0x00–0x7f`, and every velocity-like byte is 0–127.

The final visible row begins at `0x00031f96` and its returned cursor is
`0x00031f9c`. There is no structural failure within the 143-row evidence
bound. The earlier `0x31f96` stop belonged to the preceding task's smaller
`0x31f99` bound and is not a failure of this full-list validation.

# Studio Vision position model

The displayed positions are mechanically consistent with four beats per
measure and 480 units per beat. Coordinates were calculated with one-based
measure and beat components converted as:

`(measure − 1) × 4 × 480 + (beat − 1) × 480 + subdivision`

This preserves the displayed components and gives, for example, the controlled
coordinates for `25·4·469`, `26·1·469`, and `26·2·229`.

# Timing interval comparison

For every aligned consecutive pair from row 3→4 through row 142→143, the
displayed coordinate difference equals the timing interval of the later binary
candidate:

- tested pairs: 140;
- exact matches: 140;
- difference distribution: `{0: 140}`;
- constant offset required: zero.

This includes measure boundaries, beat boundaries, and unusual subdivisions
such as 120, 229, 240, and 469. The controlled examples are part of the same
full-dataset relationship: 229, 480, and 240 are the displayed start-to-start
differences for the fourth, fifth, and sixth events.

The tested relationship is therefore:

`timing interval for event N = displayed start(N) − displayed start(N−1)`

for all 140 aligned candidates. This is strong evidence for a start-to-start
interval relationship in this region, but it does not name the format's timing
semantics or prove SMF delta-time encoding.

# Duration relationship

For the 139 pairs where a current duration and next displayed start are both
available, the quantity

`next_start − (current_start + current_duration)`

never equals zero and has a broad distribution from −29 to 620 units; four
values are negative. This is consistent with overlapping notes and supplies
no competing constant relationship. The exact start-to-start equality remains
the supported timing observation; duration does not need to participate in
that interval relationship.

# Evidence supported

- The three screenshots reconcile to exactly 143 unique visible rows, matching
  the displayed event count.
- The first three visible rows precede the established parser alignment; rows
  4–143 align strictly to 140 consecutive binary candidates.
- Preregistered rows 7–11 match 20/20 fields.
- Full strict sequential comparison matches 560/560 pitch, attack, release,
  and duration fields, with 140 complete row matches and no mismatch.
- The five-field event structure persists across all 140 available aligned
  rows.
- Under the tested four-beat/480-unit coordinate model, every paired displayed
  position difference equals the later event's timing interval.
- No duration-based alternative relationship was supported.

# Boundary follow-up

The final visible candidate begins at `0x00031f96` and returns cursor
`0x00031f9c`. Bytes after that cursor begin `ff fa b9 2f ff 2f ...`; a
conservative event check therefore rejects the next property byte (`ff`).
This is not an identified end marker. See `TRACK7_EVENT_CHAIN_BOUNDARIES.md`.

# Contradictions or mismatches

No property mismatch or structural failure occurs within the 143 transcribed
rows. Parsing beyond the final returned cursor `0x00031f9c` was not required
and would need a new evidence-backed boundary.

# Unknowns

- Whether `143 Events` includes only notes or other event types is unknown.
- Absolute Track 7 event numbers above the visible window are unknown.
- Timing ownership, internal units, and relation to the complete Studio Vision
  track framing remain unresolved.
- The conversion is validated for displayed positions in this region, but it
  is not yet a claim about all Studio Vision projects or tracks.
- Channel/status representation, non-note events, track start/end framing, and
  Standard MIDI File reconstruction remain unknown.

# Single recommended next step

Capture and independently transcribe the next rows beyond the current visible
window, beginning at the first post-window parser boundary, before extending
the parser or attempting MIDI reconstruction. This preserves strict alignment
while testing whether the same 140-row agreement continues past the current
ground-truth boundary.
