# Bounded non-event terminal diagnostic

The experimental Source120 recovery assessment previously required every byte
of a framed type00 terminal payload to be ff. DANCING.MID passed all musical
checks but had one fe within an otherwise ff payload after its last complete
Track primary/secondary record pair. Its terminal payload [62480,62504) is
separate from all recovered event data; its framed record is [62475,62504).
The byte at62501 remains uninterpreted. We do not call it padding or claim to
understand its semantics.

## Bounded coverage policy

`bounded_terminal::assess_terminal_coverage` replaces the private assessment's
terminal-content veto with a reusable production coverage check. It requires:

- exact sequence bounds in the existing root record stream;
- contiguous type01/type07, at least two type02/type29 pairs, type00 framing;
- complete, ordered equality between supplied recovered conductor/ordinary
  record ranges and every paired record before the terminal;
- nonempty all-ff terminal content, or an otherwise ff payload with exactly one
  internal fe followed by ff.

The latter form yields `TerminalDiagnostic`, retaining record/payload ranges,
source bytes and the message:

> Uninterpreted terminal material remains outside recovered MIDI event data.

This is coverage evidence, not export authorization or proof of musical decoding.
Callers must establish complete Track correspondence and all event, routing,
translation, conductor and serialization gates. A diagnostic may accompany
experimental recovery only after those gates pass. No event is invented from
terminal bytes, and no known event-bearing material is discarded.

Malformed framing, overlaps or reordered/incomplete supplied coverage,
interrupted pair neighborhoods, extra event-bearing records, and unevidenced or
possibly musical terminal content remain refused. Arbitrary unknown terminal
content is not automatically accepted. Descriptor166 manifest policy and
production readiness/publication authorization are unchanged. This does not
establish a universal Studio Vision terminal grammar.

## DANCING.MID verification

The existing bounded SCHOOL assessment/assembly was invoked for DANCING.MID
only; no intervening whole-project survey or new event grammar was introduced.
It used existing correspondence, walker, same-source routing, multi-instrument
Patch, compact conductor, adapter and provenance-aware source-order serialization.
All ten ordinary Tracks pass: nine event-data Tracks and one legitimate
no-event-data Track retained as an empty MIDI Track. All musical content is
included regardless of historical mute state; mute state is not serialized.

Recovered logical events:7,845 =6,954 Notes +37 Controllers +8 Patch +846 Pitch Bend.
The terminal diagnostic retains its original bytes and does not enter the MIDI.
The experimental artifact is outside the repository:
`/tmp/phoenix-school-experimental-DANCING.mid`.

Independent bounded SMF readback passed: Format1,480 PPQN,11 Tracks, initial
MPQN400000 (150 BPM),4/4, initial key annotation0 accidentals/major. Every intended
channel message matches the recovery handoff in order, tick, channel and data:
6,954 NoteOns +6,954 NoteOffs +37 CCs +8 Programs +846 Bends =14,799 messages.
No CC0/CC32 messages are present. All eight source Patch events retain their
established Program-only translation. The reviewed duplicate Note pair remains
two attacks and two releases. Chunk bounds, explicit statuses, delta times,
Track names and End-of-Track structures pass. The file is64,131 bytes.

Six focused synthetic tests cover the diagnostic/provenance, unchanged all-ff
acceptance and MIDI content, incomplete/overlapping/reordered coverage, interrupted
or unsupported record kinds, possible musical terminal bytes and framing refusal.
The complete regression suite, formatting, clippy and diff checks pass.

The artifact is ready for owner listening validation. Mechanical verification
is not an assertion of final musical correctness, original instrument sounds,
or equivalence to a native Studio Vision export.
