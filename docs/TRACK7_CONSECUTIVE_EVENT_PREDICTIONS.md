# Objective

This report extends the bounded Track 7 diagnostic walk after the independently
verified sixth List Window event. It records parser-derived predictions for
the next consecutive rows before those rows are manually inspected in Studio
Vision. No new controlled experiment, source-artifact modification, MIDI
emission, or whole-file heuristic scan was used.

# Evidence boundary

The supported local model is:

`[timing 7-bit big-endian VLQ] [pitch] [attack] [release] [duration VLQ]`

The fourth and fifth events are controlled/documented. The sixth event is now
independently verified after a prior parser prediction. Timing values remain
provisional interval units: this report does not convert them to displayed
Studio Vision positions or call them SMF delta-times. Later structures are
labelled parser-predicted candidate events until Studio Vision confirms them.

# Confirmed starting alignment

The authentic Experiment 007 artifact was positively identified as:

`/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline`

The bounded walk begins at `0x00031c1d` and confirms the first three structures:

| Event / SVP row | Timing offset | Timing bytes | Interval | Property offset | Properties | Duration | Next cursor | Accumulated |
|---|---:|---|---:|---:|---|---:|---:|---:|
| fourth | `0x31c1d` | `81 65` | 229 | `0x31c1f` | `24 7f 5c` | 442 (`83 3a`) | `0x31c24` | 229 |
| fifth | `0x31c24` | `83 60` | 480 | `0x31c26` | `26 7f 56` | 245 (`81 75`) | `0x31c2b` | 709 |
| sixth | `0x31c2b` | `81 70` | 240 | `0x31c2d` | `24 7f 60` | 107 (`6b`) | `0x31c31` | 949 |

# Sixth-event independent verification

Before the Studio Vision check, Phoenix predicted from the third binary
structure:

`81 70 | 24 7f 60 6b`

Studio Vision then independently showed the sixth Track 7 List Window event as
position `26·2·229`, pitch C1, duration 107, attack velocity 127, and release
velocity 96. All four musical-property predictions succeeded exactly:

- pitch `24` = C1 under the established mapping;
- attack `7f` = 127;
- release `60` = 96;
- duration `6b` = 107.

This property structure is now independently verified rather than merely
provisional. The leading interval `81 70` = 240 remains timing-related but
its ownership and conversion to the displayed position remain unresolved.

# Sequential decoding method

The existing `track7::decode_event` implementation was reused without code
changes. The walk starts at the known sixth-event timing offset and advances
strictly to each returned cursor. The explicit bound is `0x00031f99`; the
first failed cursor is `0x00031f96`, where the remaining bounded bytes are
insufficient for the required property structure. No bytes were skipped, no
resynchronization was attempted, and no whole-file candidate scan was
performed.

All 137 structures from the sixth event through the final clean candidate
decoded mechanically within this bound. Each candidate passed these separate
plausibility checks: timing VLQ decoded, duration VLQ decoded, pitch byte was
within `0x00–0x7f`, and both velocity-like bytes were within `0–127`.
These checks do not establish event semantics.

# Parser-predicted candidate events

The first five candidates after the now-confirmed sixth event are:

| Candidate row | Timing offset | Timing bytes | Interval | Property offset | Pitch byte | Attack | Release | Duration bytes | Duration | Next cursor | Accumulated |
|---:|---:|---|---:|---:|---:|---:|---:|---|---:|---:|---:|
| 7 | `0x31c31` | `83 60` | 480 | `0x31c33` | `26` | 121 | 125 | `47` | 71 | `0x31c37` | 1429 |
| 8 | `0x31c37` | `81 70` | 240 | `0x31c39` | `26` | 127 | 62 | `81 56` | 214 | `0x31c3e` | 1669 |
| 9 | `0x31c3e` | `81 70` | 240 | `0x31c40` | `24` | 118 | 105 | `55` | 85 | `0x31c44` | 1909 |
| 10 | `0x31c44` | `81 7b` | 251 | `0x31c46` | `24` | 127 | 92 | `83 3a` | 442 | `0x31c4b` | 2160 |
| 11 | `0x31c4b` | `83 60` | 480 | `0x31c4d` | `26` | 127 | 86 | `81 75` | 245 | `0x31c52` | 2640 |

All five are parser-predicted candidate events, not confirmed Studio Vision
rows. Their bytes and values are retained exactly so a manual check cannot be
retrofit to a later observation.

The remaining 131 clean candidates continue sequentially from `0x31c52` to
the final clean candidate ending at `0x31f96`. They all pass the same narrow
mechanical plausibility checks. Their full source offsets, byte lengths, and
decoded fields are available by rerunning the bounded `decode_event` walk over
the explicit range; no semantic claim is made for them here.

# Preregistered Studio Vision predictions

The following table is the preregistration target. It was derived only from
the binary walk and the established pitch-byte mapping; later displayed rows
were not consulted.

| List Window row | Predicted pitch | Attack | Release | Duration | Leading interval |
|---:|---|---:|---:|---:|---:|
| 7 | D1 (`0x26`) | 121 | 125 | 71 | 480 |
| 8 | D1 (`0x26`) | 127 | 62 | 214 | 240 |
| 9 | C1 (`0x24`) | 118 | 105 | 85 | 240 |
| 10 | C1 (`0x24`) | 127 | 92 | 442 | 251 |
| 11 | D1 (`0x26`) | 127 | 86 | 245 | 480 |

The leading intervals are reported separately. They are not translated into
SVP displayed positions because that conversion is not established.

# Natural stopping point

The preregistration used the explicit bound `0x00031f99`, where the next
candidate could not be completed. Subsequent full-list validation extended the
explicit range to include visible row 143. The candidate beginning at
`0x00031f96` then decoded cleanly and returned cursor `0x00031f9c`; the earlier
stop was therefore a bound limit, not a structural failure in the visible
dataset. See `TRACK7_FULL_LIST_VALIDATION.md`.

No resynchronization or byte skipping was used. Parsing beyond `0x00031f9c`
was not required for the screenshot validation and remains outside its
evidence-backed bound.

# What would confirm the predictions

For each row 7–11, manually inspect the corresponding untouched Experiment 007
Track 7 List Window row and compare pitch, attack velocity, release velocity,
and duration exactly. A match in all four fields would independently verify
that candidate structure. Timing values should remain a separate comparison;
even a musical-property match would not establish timing ownership.

The subsequent full screenshot validation confirmed rows 7–11 exactly: all 20
preregistered musical-property fields matched. See
`TRACK7_FULL_LIST_VALIDATION.md`; the original prediction table above is
preserved unchanged.

# What would falsify or weaken the model

A disagreement in one or more musical properties would weaken the claim that
the local property sequence continues unchanged after the sixth event. A
consistent property match with a different displayed position would leave the
property model supported but keep timing conversion unresolved. A structural
failure before a candidate row would narrow the valid chain boundary rather
than justify resynchronization.

# Unknowns

- Exact timing semantics and conversion from accumulated intervals to displayed
  Studio Vision positions remain unknown.
- Event framing beyond the bounded chain, channel/status representation,
  non-note events, and Track 7 start/end framing remain unresolved.
- Candidate rows 7–11 and later have not been independently verified in Studio
  Vision.
- No MIDI output has been generated and no valid Standard MIDI File has yet
  been recovered.

# Single recommended next step

Manually verify rows 7–11 in Studio Vision against this fixed table before any
new controlled edit or parser expansion. This directly tests the highest-value
new predictions while preserving the current evidence boundary.
