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
- nonempty all-ff terminal content; diagnostic forms are `ff+ fe ff*` or
  exactly two leading zero bytes followed by `ff+ fe ff+`. The notation means
  one or more `ff` before a single `fe`, then zero or more `ff` for the unprefixed
  form, or one or more trailing `ff` for the two-zero-prefixed form. The additional
  unprefixed repeated-marker grammar is `ff+ (fe ff+){2,}`, described below.

The diagnostic forms yield `TerminalDiagnostic`, retaining record/payload ranges,
exact source bytes and the classification **UNINTERPRETED NON-EVENT TERMINAL
MATERIAL**, with the message:

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

## NOTHING.MID and Sequence M terminal generalization

After bounded initial-conductor recovery, these two targets passed every
preceding musical gate but refused the older content predicate. A focused,
read-only check of the same authenticated 343,875-byte SCHOOL source established
their complete local neighborhoods and exhaustive preceding paired-record
coverage. No historical semantics were investigated or assigned.

| Target | Complete local Sequence | Paired-record region | Terminal record / payload | Exact terminal bytes |
|---|---|---|---|---|
| #2 NOTHING.MID | 62504..85509 | 64034..85480, 11 pairs | 85480..85509 / 85485..85509 | `0000fffffffffffffffffffffffffffffffffffffffeffff` |
| #13 unnamed, owner UI Sequence M | 281057..281671 | 281507..281660, 2 pairs | 281660..281671 / 281665..281671 | `fffffffffffe` |

Ranges are absolute decimal and half-open. Each neighborhood is one type01,
one type07, contiguous type02/type29 pairs, then a separate type00 record.
NOTHING.MID has two conductor pairs, nine ordinary pairs and one legitimate
no-event-data Track; its 12 metadata slots have complete correspondence. Every
ordinary primary's declared event count matches the complete bounded event walk,
totaling 3,354 logical events. Its last ordinary event range ends at 85382 and
its last secondary record ends at 85480, before the terminal payload begins.
Sequence M has two complete conductor pairs and one legitimate no-event-data
ordinary Track; all three metadata slots are accounted for. Its last conductor
event ends at 281610, and the last secondary ends at 281660.

Neither terminal overlaps any recovered event range. No ownership/count/length
evidence calls for additional musical data in either terminal. The supported
fresh mixed-event walker recognizes no complete event prefix at any bounded
start/end inside either payload; neither contains the entry statuses or bounded
tags needed for supported initial conductor events. Stateful Track continuation
is not inherited across a closed type02/type29 pair into a type00 terminal.
The absence of a recognized event alone is not acceptance evidence: framing,
exhaustive preceding coverage, complete musical gates and the whole-payload
terminal-family predicate are all required.

The narrow extension admits a final `fe` in the existing unprefixed family and
the exact two-zero prefix only in the demonstrated internal-`fe` family with
trailing `ff`. That extension did not admit one/three leading zeros, all-zero data,
two-zero-prefixed all-ff content, a prefixed final `fe`, multiple deviations,
event statuses/tags or arbitrary unknown suffixes. These bytes remain
uninterpreted non-event terminal material, not claimed padding. All bytes and
record/payload provenance survive in the diagnostic; none enters MIDI.

### Conversion and regression verification

Both targets now pass correspondence, complete event decoding, routing, Patch,
Controller, Pitch Bend, source ordering, conductor, terminal coverage and final
serialization. Zero-event families are fully accounted for without defaults.
All ordinary Tracks are retained, including no-event-data Tracks and saved-muted
musical Tracks. Sequence M has no invented route, Program, Key or Note.

Fresh artifacts were exclusively created in the owner's Experimental MIDI
Recovery/SCHOOL PROJECTS directory, preserving existing listening files:

- `SCHOOL - NOTHING.mid`: Format 1, 480 PPQN, 11 SMF Tracks, 26,523 bytes;
  3,354 logical events = 2,926 Notes + 15 Controllers + seven Patches + 406
  Pitch Bends. Readback matches 2,926 attacks, 2,926 generated releases, 15 CCs,
  seven Programs and 406 Bends, totaling 6,280 channel messages. Initial Tempo
  is 1,000,000 MPQN (60 BPM), Meter 4/4 and Key 0 accidentals/major. SHA-256:
  `8912e1ed99bee8aa8684db9b1ed87ed6f980a640ad2153b4d62f2a68fdb0c23c`.
- `SCHOOL - Sequence M.mid`: Format 1, 480 PPQN, conductor plus one retained
  empty musical Track, 61 bytes; zero logical/channel events. Initial Tempo
  is 545,454 MPQN (~110 BPM), Meter 4/4, with no source Key event. SHA-256:
  `e8f988e8a7b983588ef12dd1c0f26a5fb9ac102f6fcace85529f59f8a39ec21c`.

Independent readback of the actual generated files checks SMF/chunk boundaries,
PPQN, conductor metadata, every channel message's tick/channel/data against the
owned source handoff, generated Note releases, source order, Track names and EOT.
No new owner listening result is claimed.

DANCING retains its diagnostic and byte-identical conversion; Nothing FINAL,
unnamed #9 and Tuba thing remain mechanically valid and byte-identical to their
previous verified artifacts. The separate kickin'dance Track 8 zero terminal
tail still refuses `InvalidTerminalGrammar`; ordinary Track bounds are unchanged.

Twelve focused terminal tests pass, including incomplete/overlapping/reordered
coverage, additional unrecovered pairs, unsupported placement, malformed framing,
plausible Note/Controller/Pitch Bend/Patch/conductor/context material and arbitrary
suffix refusals. Full `cargo test` passes (552 passed, seven ignored, zero failed),
along with formatting, all-target/all-feature Clippy with warnings denied and
diff checks. The authentic source, existing listening files and unrelated
owner/Xcode state remain unchanged. No production readiness or publication
authorization is added by this coverage diagnostic.


## Repeated isolated non-event terminal markers

The bounded classifier now also accepts the unprefixed whole-payload grammar
`ff+ (fe ff+){2,}`: a nonempty leading ff run, at least two isolated fe bytes,
and a nonempty ff run after each fe, including the final one. This is a structural
extension, independent of project/Sequence identity, ordinals, offsets, hashes or
Track/Patch names. It assigns no historical semantics to either byte.
Existing all-ff, single-fe and exact two-zero-prefixed single-fe forms are unchanged.
The previous single-fe unprefixed form may still end in fe; the repeated form may
not. Multiple fe in the prefixed form remain unsupported.

Evidence review `/tmp/phoenix-sequence-h-terminal-blocker-review.txt` established
exhaustive musical coverage before classifying the new material. The diagnostic
remains **UNINTERPRETED NON-EVENT TERMINAL MATERIAL** with the existing message,
record/payload ranges and complete exact source bytes. It is never synthesized
into MIDI or silently removed from recovery evidence.

The existing source framing and ordered/exhaustive conductor/ordinary pair coverage
checks are unchanged. Complete correspondence, event decoding, count reconciliation,
routing and all other musical gates remain caller prerequisites: this coverage
component checks framed record completeness, not musical semantics. Incomplete
recovery cannot be authorized by a byte-pattern match. The complete recovery
assessment invokes the terminal check only after all musical gates pass.

Refusals include leading fe, adjacent fe, a repeated body's final fe without ff,
zero ff before the first fe, unapproved prefixes, other byte values and mixed
possibly musical terminal material. Existing malformed/framing/ownership/placement,
extra unrecovered event-data pair, overlapping/incomplete/reordered coverage and
ordinary Track terminal-bound refusals remain unchanged. Unsupported event forms,
decoder refusals and unresolved declared-count deficits must fail the preceding
musical assessment; the terminal classifier is not a fallback around them.
This does not accept arbitrary ff/fe strings or general unknown trailing data.

### Complete Sequence H export verification

Authenticated SCHOOL PROJECTS remains 343,875 bytes, SHA-256
`bfd4fa1208e2cd884ec51ccfb1131d5c02723d7c90acafc98bd89597e1a20331`.
All 12 ordinary Tracks are retained: eight fully consumed event-bearing Tracks
and four legitimate no-event-data Tracks. Recovered logical events total 284:
280 Notes and four Patches, with zero source Controllers/Bends/Pressure. The raw
ordinary count fields total 298; every per-Track difference is exactly accounted
for by its already-decoded contexts, 14 in total (not 14 extra musical events).
There is no residual count deficit or unconsumed event-bearing region. The initial
Meter and Tempo are completely decoded; no source Key event is invented.

The separate type00 record [248314,248347), payload [248319,248347), retains
`fffffffffffffffffffffffffffffffffffffffefffefffefffeffff` diagnostically after
all ten conductor/ordinary type02/type29 pairs. It overlaps no event range and
no bounded subrange matches the supported event grammar. Semantics remain unknown.

The complete current bounded recovery/export pipeline now passes correspondence,
ordinary bounds and decoding, routing, Controller/Pitch Bend requirements, standalone
Multi Patch bank translation, source order, conductor, terminal and Format1 assembly.
Sequence H is now experimentally exportable, with no next export blocker found.

Persistent listening artifact: `SCHOOL - Sequence H.mid` in the owner's Experimental
MIDI Recovery/SCHOOL PROJECTS directory; 2,857 bytes, SHA-256
`b7b0d39da264919f273e36886eae4913a5ff86dc8d6122a9576ca96c31935cc5`.
Exclusive file creation preserves existing artifacts. Independent readback of the
actual file passes Format1, 480 PPQN, 13 Tracks, conductor metadata, all 280 attacks
and 280 releases, four CC0 and four Programs, ticks/channels/data/source order,
Track names and EOT/chunk/file boundaries. Saved-muted material is preserved.

The four initializations continue to match the owner-attested untouched native
Sequence H MIDI: ticks 0/0/0/480, channels 2/3/1/4, CC0 81/81/81/80 and Programs
7/13/23/31. CC0 precedes Program and no CC32 is present. This compares Patch
semantics, not full-file byte equality with Studio Vision's saved-mute output scope.
No owner listening result is claimed.

DANCING, Nothing FINAL, NOTHING.MID, unnamed #9, Tuba thing and Sequence M remain
byte-identical and independently mechanically valid; prior terminal diagnostics
retain exact bytes/ranges/messages. Other SCHOOL failures remain unchanged,
including ordinary kickin'dance Track 8 and broader conductor cases. SCHOOL's
current complete experimental exports increase from six to seven of fifteen.

Fourteen focused terminal tests pass, covering repeated two/four-marker positives,
exact diagnostic provenance, no synthesized events, prior supported forms,
lookalikes/prefix/suffix refusals, event content, overlapping/incomplete coverage,
extra unrecovered pairs and malformed/ambiguous framing/placement. Full cargo tests,
formatting, strict all-target/all-feature Clippy and whitespace checks pass.
Detailed audit and artifact identities:
`/tmp/phoenix-repeated-fe-terminal-blocker-removal.txt`.
