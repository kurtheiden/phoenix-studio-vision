# Objective

This spike implements the first deliberately bounded diagnostic parser for the
known local Track 7 event-chain region. It is a transition from controlled byte
forensics to narrow parser implementation. It does not emit Standard MIDI
Files, identify arbitrary bytes, or claim general Studio Vision parsing.

# Evidence boundary

Controlled experiments support the local repeating sequence:

`[timing VLQ] [pitch] [attack velocity] [release velocity] [duration VLQ]`

The fourth event and the immediately following fifth event have direct or
independent documented support. The next `24 7f 60 6b` structure is only
mechanically consistent and remains provisional. Timing values are described
as interval units; they are not called SMF delta-times, MIDI ticks, or Studio
Vision absolute positions.

The implementation accepts an explicit byte slice and start/end range. It
never scans the whole file for plausible events and never searches ahead to
repair malformed alignment.

# Implemented diagnostic model

`src/track7.rs` contains the smallest reusable model needed by the spike:

- `DecodedVlq` records a value and encoded length;
- `DiagnosticEvent` records timing/property offsets, timing interval, pitch,
  attack/release values, duration, encoded lengths, and provisional accumulated
  interval units;
- `decode_event` parses exactly one bounded structure;
- `walk_chain` repeatedly parses only the supplied range and reports the
  cursor on failure.

No channel, status, event-type, track-framing, or non-note fields were added.
The existing CLI remains unchanged because exposing an evidence-backed
start/end contract as a public command-line interface would be a new user
interface rather than a small inspection integration. Library-level use keeps
the spike isolated from established behavior.

# VLQ decoding

`decode_7bit_be_vlq` implements the observed 7-bit big-endian encoding. It
checks the supplied bound, detects truncation, and limits one value to four
encoded bytes. Focused tests cover:

| Bytes | Value |
|---|---:|
| `81 65` | 229 |
| `83 60` | 480 |
| `81 70` | 240 |
| `83 3a` | 442 |
| `81 75` | 245 |
| `6b` | 107 |

Tests also cover an unterminated value, an offset outside the range, and a
five-byte continuation sequence.

# Bounded event decoding

One call decodes one timing VLQ, reads exactly three property bytes, decodes
one duration VLQ, and returns the next cursor. Truncated timing, missing
properties, truncated duration, and range overruns become typed errors. The
parser does not assign note names or infer event ownership.

# Bounded chain walking

`walk_chain(bytes, start, end)` accumulates timing intervals with checked
saturating addition and retains each source offset. It stops at the explicit
end and reports the failing cursor/context if a structure cannot be decoded.
The API intentionally makes the evidence-backed region a caller decision.

# Controlled fixture results

The documented baseline fixture

`81 65 24 7f 5c 83 3a 83 60 26 7f 56 81 75 81 70 24 7f 60 6b`

produces three mechanical structures:

| Timing offset (fixture-relative) | Accumulated | Interval | Pitch | Attack | Release | Duration |
|---:|---:|---:|---:|---:|---:|---:|
| 0 | 229 | 229 | `0x24` | 127 | 92 | 442 |
| 7 | 709 | 480 | `0x26` | 127 | 86 | 245 |
| 14 | 949 | 240 | `0x24` | 127 | 96 | 107 |

The first two structures correspond to the controlled/documented fourth and
fifth events. The third is a mechanically parsed, provisional candidate; its
successful decode is not an independent Studio Vision row identification.

The position fixtures also decode mechanically:

| Artifact | First interval | Second interval | Third interval | Accumulated values |
|---|---:|---:|---:|---|
| Experiment 019 | 228 | 481 | 240 | 228, 709, 949 |
| Experiment 007 | 229 | 480 | 240 | 229, 709, 949 |
| Experiment 020 | 230 | 479 | 240 | 230, 709, 949 |
| Experiment 022 | 229 | 481 | 239 | 229, 710, 949 |

These fixtures verify the established one-for-one compensating timing
responses without encoding those conclusions into parser semantics.

# Authentic artifact results

The sole uncompressed Experiment 007 project was positively identified as
`/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline`.
Using the evidence-backed absolute range `0x00031c1d–0x00031c30`, the parser
reported:

| Timing offset | Accumulated | Interval | Pitch | Attack | Release | Duration |
|---:|---:|---:|---:|---:|---:|---:|
| `0x00031c1d` | 229 | 229 | `0x24` | 127 | 92 | 442 |
| `0x00031c24` | 709 | 480 | `0x26` | 127 | 86 | 245 |
| `0x00031c2b` | 949 | 240 | `0x24` | 127 | 96 | 107 |

The authentic result matches both experimentally identified structures and
the following provisional structure. No artifact was copied or modified.

# Position-control results

The same read-only bounded invocation was run on the positively identified
uncompressed artifacts for Experiments 019, 020, and 022. It reproduced the
forensic reports exactly:

- Experiment 019: fourth-event leading 228 and next interval 481;
- Experiment 007: 229 and 480;
- Experiment 020: 230 and 479;
- Experiment 022: fifth-event leading 481 and following 239.

Experiment 022's property bytes remain `26 7f 56 81 75`; only the adjacent
interval values differ as controlled. No authentic-artifact result contradicts
the current local model.

# What the parser does not claim

- It is not a general Studio Vision track parser.
- It does not discover Track 7 or identify event ranges heuristically.
- It does not prove that the intervals are SMF delta-times or establish their
  unit, ownership, or absolute-time reconstruction.
- It does not establish complete event boundaries, channel/status encoding,
  non-note event representation, or broader track framing.
- It does not prove that every byte sequence matching the local shape is a
  note; the third structure remains provisional.
- It does not write MIDI output.

# Evidence supported

- A bounded parser can safely decode the exact evidence-backed local model.
- The VLQ implementation handles all documented values and malformed bounds.
- The authentic baseline produces the two controlled structures and the
  provisional third structure at exact source offsets.
- Position experiments 019, 020, and 022 decode the established timing
  mutations exactly.
- The parser does not need whole-file heuristic recovery to reproduce the
  controlled evidence.

# Unknowns

- The start/end framing for a complete Track 7 stream remains unknown.
- The sixth and later property-like structures lack independent List Window
  matching in the current evidence.
- Timing ownership, internal units, status/channel representation, and event
  ordering beyond the bounded local chain remain unresolved.
- No valid Standard MIDI File has yet been recovered.

# Single recommended next step

Verify the provisional third structure against Studio Vision's sixth List
Window event. This is the highest-information next step because it can extend
the bounded chain with an independently documented row before expanding the
parser's range or attempting any MIDI reconstruction.
