# Objective

Inventory the MIDI event families emitted by the available Studio Vision MIDI
exports, assess current Phoenix coverage, and prioritize recovery by shared
event representation rather than by individual parameter number. Export
provenance is tracked independently: only the `Ode to Clarke` set is presently
mapped to the authentic `newest STUFF` project.

# Source scope

The positively identified uncompressed project is:

`/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline`

It is 211,468 bytes, SHA-256
`e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`,
with Finder type `MID2` and creator `MIDA`.

The export directory contains seven valid SMFs in three filename-based sets.
Those filenames are not, by themselves, Studio Vision sequence identities:

| Export | Format | Tracks | Size | SHA-256 |
|---|---:|---:|---:|---|
| `ANALOG.MID #2` | 0 | 1 | 38,874 | `8115784c95850f55fc9addc711536e1a72e115d3ca1b539d879c995adc736f8e` |
| `ANALOG.MID #2 Multitrack` | 1 | 14 | 33,704 | `bda7a60314c4e1552acc105599d26739f44190365ec3054a093547d98c4bb59e` |
| `BATTL2GS.MID` | 0 | 1 | 19,848 | `de902b71506481ac481504ec8fa6b9b4782391d28a7ee185f759839817526c88` |
| `BATTL2GS.MID Multitrack` | 1 | 18 | 20,817 | `8c7deba333c9f97d5347f93a6b04429c28cb1d3e6ab6377aa4328c7ecbe0e2b1` |
| `Ode to Clarke` | 0 | 1 | 8,644 | `eb37711a81eee7d78877bfe2ca67712ac2b98067cbec9e23f9f8e739380bf5a6` |
| `Ode to Clarke Multi All` | 1 | 10 | 12,141 | `4f63b34ef92204d4bc5eeb78dbbe7b94d005c1f9ceb57ea0f9809533ad590f29` |
| `Ode to Clarke Multitrack` | 1 | 8 | 10,514 | `9979ed6d5fc58edb85c3c03e5e43b4c7015a353af6075ece8ba94ac49cbf5059` |

All use division 480. Counts below use one richest format-1 export per export
set: `ANALOG.MID #2 Multitrack`, `BATTL2GS.MID Multitrack`, and `Ode to Clarke
Multi All`. This avoids double-counting related format-0/subset files, but it
does **not** imply that all three sets came from `newest STUFF`. For
reproducibility, the raw seven-file view contains 13,176 positive Note Ons,
13,176 note endings, 10,225 Control Changes, 76 Program Changes, 880 Pitch
Bends, and 4 SysEx events.

## Export provenance correction

Direct Studio Vision UI observation establishes these 18 active sequences in
the untouched baseline:

| Letter | Active sequence | Letter | Active sequence |
|---|---|---|---|
| A | `xForm` | J | `newsong` |
| B | `Bells for her` | K | `Sequence K` |
| C | `Situation` | L | `Renaissance` |
| D | `Sequence D` | M | `Get on up & Dance` |
| E | `Sequence E` | N | `Jurrasic Park` |
| F | `Girl-U-Want` | O | `Ode to Clarke` |
| G | `mission impossibl` | P | `Over the Top` |
| H | `happyone` | Q | `Sequence Q` |
| I | `Sequence I` | R | `Sequence R` |

Neither `ANALOG.MID #2` nor `Analog Seq` is an active sequence. A filename,
SMF track-name event, or readable project string is therefore insufficient to
identify an active sequence. Identity requires UI ground truth or independent
musical/structural correlation.

| Export set | Provenance classification | Basis |
|---|---|---|
| `Ode to Clarke` (three files) | **PROVEN FROM NEWEST STUFF** | UI sequence/track identity plus extensive exact Note, Patch, timing, instrument-context, and project-binary correlations |
| `ANALOG.MID #2` (two files) | **UNRESOLVED** | export filename/SMF metadata only; no musical or structural mapping to any of the 18 active sequences |
| `BATTL2GS.MID` (two files) | **UNRESOLVED** | export filename/SMF metadata only; no musical or structural mapping to any of the 18 active sequences |

The two unresolved sets are compatible with exports from another Studio
Vision project. Current evidence neither proves nor disproves that origin.

# Important limitations of MIDI export evidence

The exports show what Studio Vision emitted, not necessarily every original
event object or its project encoding. Export can insert initialization data,
translate durations into Note Off messages, use zero-velocity Note On as Note
Off, duplicate metadata across chunks, flatten tracks, and normalize device
state. Meta events may be generated from sequence, track, instrument, or
application state rather than serialized event records. Only the Ode set is
known to cover one of the project's 18 active sequences. The aggregate counts
also include two source-unresolved export sets and therefore must not be
reported as `newest STUFF` coverage. No raw exported event stream or complete
SysEx payload was found verbatim in the authentic project.

# Channel-event family inventory

The three-export-set non-duplicative inventory is:

| Family | Events | Sequences | Tracks | Channels | Representative evidence |
|---|---:|---:|---:|---|---|
| Note starts | 6,109 | 3 | 36 | 1–16 | pitches across orchestral, drum, synth, and sampler tracks; attack 7–127 |
| Note endings | 6,109 | 3 | 36 | 1–16 | 4,465 explicit Note Off; 1,644 zero-velocity Note On |
| Program Change | 38 | 3 | 30 | 1–16 | values 0–125; several mid-sequence changes plus initial Patches |
| Control Change | 5,112 | 3 | 29 | 1–16 | 14 controller numbers, values 0–127 |
| Pitch Bend | 440 | 1 | 2 | 3, 5 | `ANALOG.MID #5` and `ANALOG.MID #2`; values 0–16,383 including center 8,192 |
| Channel Pressure | 0 | 0 | 0 | — | not present |
| Polyphonic Key Pressure | 0 | 0 | 0 | — | not present |

This table remains the historical three-export-set snapshot. The later
provenance-controlled `Bells for her` export adds 32 Channel Pressure events on
Track 9/channel 12, 102 Pitch Bend events on Track 14/channel 15, and no
Polyphonic Key Pressure. Pressure correlates to one exact stateful run; Pitch
Bend correlates to nine exact stateful runs. They do not retroactively change
the older three-set counts.

These aggregate counts remain useful evidence about event families emitted by
Studio Vision, but not as an inventory of `newest STUFF`. By export set, the
Controller counts are: source-unresolved `ANALOG` 4,865;
source-unresolved `BATTL2GS` 243; and proven `Ode to Clarke` 4. The proven Ode
events are two CC0/CC32 bank-select pairs already correlated with Patch
representations; they do not provide a controller-heavy ordinary-Controller
region.

Positive Note On count is the musical note count. A zero-velocity Note On is
reported as a note ending, not another Note start. Explicit Note Off release
velocity ranges 8–127; the zero-velocity form carries zero by definition.

# Control Change inventory

Control Change is one MIDI event family with controller number and value
fields. The inventory does not create fourteen separate parser targets.

| CC | Standard name | Count | Sequences | Tracks | Value range | Pattern |
|---:|---|---:|---:|---:|---|---|
| 0 | Bank Select MSB | 6 | 2 | 5 | 0–81 | paired with CC32 and PC |
| 6 | Data Entry MSB | 2,766 | 1 | 6 | 14–118 | dense NRPN/RPN parameter streams |
| 7 | Channel Volume | 31 | 2 | 25 | 100–127 | initialization clusters |
| 10 | Pan | 31 | 2 | 25 | 0–127 | initialization clusters |
| 11 | Expression | 140 | 2 | 13 | 18–127 | initialization plus automation |
| 32 | Bank Select LSB | 4 | 2 | 3 | 0–2 | paired with CC0 and PC |
| 64 | Sustain | 2 | 1 | 1 | 0–127 | one on/off pair on `piano hi` |
| 91 | Reverb send | 32 | 2 | 26 | 40–127 | initialization clusters |
| 93 | Chorus send | 32 | 2 | 26 | 0–127 | initialization clusters |
| 98 | NRPN LSB | 529 | 1 | 6 | 8–102 | selector preceding CC6 values |
| 99 | NRPN MSB | 529 | 1 | 6 | 1 | selector preceding CC98/CC6 |
| 100 | RPN LSB | 494 | 1 | 4 | 127 | recurring null-selection/reset tail |
| 101 | RPN MSB | 494 | 1 | 4 | 127 | recurring null-selection/reset tail |
| 121 | Reset All Controllers | 22 | 1 | 16 | 0 | per-part initialization |

CC1 Modulation is absent from these exports. The dominant data is not fourteen
unrelated behaviors: 4,812 of 5,112 events are CC6/98/99/100/101 components of
NRPN/RPN-style sequences. CC7/10/11/91/93 and CC121 recur in track-start or
patch-change initialization clusters. CC0/32 occur next to Program Change.
Only CC64 is a simple isolated performance on/off pair in this sample.

# System and meta-event inventory

| Family | Count | Sequences | Tracks | Classification |
|---|---:|---:|---:|---|
| SysEx | 2 | 2 | 2 | musical/device state; payloads 10 and 25 bytes |
| Tempo (`FF 51`) | 3 | 3 | 3 | playback-critical sequence/export metadata; 600,000, 461,538, 500,000 µ/qn |
| Time signature (`FF 58`) | 4 | 2 | 2 | meter data; all observed 4/4, three repeated in `ANALOG` |
| SMPTE offset (`FF 54`) | 3 | 3 | 3 | synchronization/export metadata; all `60 00 00 00 00` |
| Track name (`FF 03`) | 42 | 3 | 42 | export metadata derived from sequence/track names |
| Instrument name (`FF 04`) | 56 | 3 | 39 | export metadata; `Sound Canvas`, channel labels, Juno/JV/JD/S-760 |
| Copyright (`FF 02`) | 1 | 1 | 1 | low-value export metadata |
| End of track (`FF 2F`) | 42 | 3 | 42 | SMF framing generated by export |

No key signatures, generic text, lyrics, markers, cues, sequence-specific meta
events, channel pressure, or poly pressure occur. Tempo and meter affect
playback and are musically recoverable concepts, but their SMF meta events may
be synthesized from Studio Vision's Tempo/Meter tracks rather than direct
one-to-one project event records. Track/instrument names are useful project
metadata but are not MIDI performance events. End-of-track is export framing.

The original three-set material contains **12 distinct semantic families**:
Note, Program/Patch, Controller, Pitch Bend, SysEx, tempo, meter, SMPTE offset,
track name, instrument name, copyright, and end-of-track. At wire-message
level, splitting Note On and Note Off makes 13. The later `Bells for her`
export adds Channel Pressure as a thirteenth observed semantic family across
the expanded evidence set.

# Current Phoenix coverage

| Family | Status | Boundary of claim |
|---|---|---|
| Notes | IMPLEMENTED | bounded timing/property/duration decoder and extensive controlled/authentic validation; discovery and general track ownership unresolved |
| Patch / Program / bank | IMPLEMENTED | caller-bounded shared Patch decoder; PC, CC0, and CC32 values established; bank optionality not exposed |
| Control Change | IMPLEMENTED BOUNDED | exact-bound `ff 41 05` decoder with authentic CC1/CC7 fixtures; discovery and container integration remain absent |
| Pitch Bend | IMPLEMENTED BOUNDED RUN | exact-bound decoder and all nine authentic `e0`-entered Track 14 runs; discovery and broader forms absent |
| Channel Pressure | IMPLEMENTED BOUNDED RUN | exact-bound decoder and authentic 32-event `d0`-entered stateful-run fixture; discovery and broader forms absent |
| Poly Pressure | NOT PRESENT | absent in available exports |
| SysEx | UNINVESTIGATED | two export examples; no project encoding |
| Tempo | IMPLEMENTED BOUNDED INITIAL | exact seven-byte decoder with direct MPQN and authentic fixtures; general map unknown |
| Meter / time signature | IMPLEMENTED BOUNDED INITIAL | exact eight-byte decoder; numerator/denominator and payload provenance established; general map and universal historical `cc` policy unknown |
| Export metadata | PARTIALLY UNDERSTOOD | strings and metadata tables observed; no general semantic parser |

# Shared Controller-family assessment

**ESTABLISHED FOR THE OBSERVED CC1/CC7 SCOPE.** The later
provenance-controlled `Bells for her` export supplies 395 natural ordinary
Controllers across Tracks 3, 4, 6, 9, and 14. Every corresponding project
record uses `timing VLQ | ff 41 | 05 | opaque context[3] | controller number |
value`; Track 9 matches 120/120 ordered number/value pairs and event-start
deltas. CC1 and CC7 therefore belong to one generic record family.

This does not establish every CC number or every Studio Vision project,
version, or device configuration. Source-unresolved export populations remain
unusable for project correlation, and the context bytes have no assigned
channel, track, instrument, device, or reference semantics.

# Binary candidate visibility

This was intentionally a lightweight survey, not a decoding campaign.

| Family | Visibility | Observation |
|---|---|---|
| Notes | STRONG | multiple long project chains already align exactly with export properties and timing |
| Patch / Program | STRONG | four authentic events and controlled position/name/PC/bank saves |
| Controllers | STRONG for bounded `Bells for her` CC1/CC7; unresolved elsewhere | 395 ordinary events correlate exactly; source-unresolved populations remain out of scope |
| Pitch Bend | STRONG for bounded `Bells for her`; unresolved for older `ANALOG` set | 102 events form nine exact stateful Track 14 runs with direct LSB/MSB and timing agreement |
| SysEx | WEAK | two distinctive Roland payloads; neither full payload nor payload without terminator appears verbatim in the project |
| Tempo | STRONG for bounded initial `Bells for her` form | natural and controlled seven-byte forms agree; direct MPQN and exact primary boundary established |
| Meter | STRONG for bounded initial form | natural 4/4/6/8/10/8 project forms and controlled 7/8 establish framing and fields; 4/4, 6/8, and 7/8 have provenance-correlated exports |
| Pressure | STRONG for one bounded `Bells for her` run | 32 direct values and event-start deltas match exactly; entry/continuation state is established only for Track 9 |

Controller, Channel Pressure, and Pitch Bend now have strong bounded natural
evidence. SysEx likely needs explicit List Window or one controlled payload edit
because export framing may transform raw data.

# Recovery-value ranking

- **CRITICAL — Notes:** pitch, timing, velocity, and duration are the musical
  performance; substantially solved within bounded known chains.
- **HIGH — Patch/Program/bank:** determines sound selection; value fields are
  substantially solved and bounded decoding exists.
- **HIGH — Controllers:** 5,112 events include expression, sustain, bank,
  mix/effects settings, and dense parameter automation.
- **HIGH — Pitch Bend:** 440 events represent continuous pitch gestures that
  cannot be reconstructed from Notes.
- **HIGH — Tempo:** controls absolute playback speed.
- **MEDIUM — Meter:** important for bar/beat reconstruction and editing, but
  does not change raw MIDI tick playback.
- **MEDIUM — SysEx:** only two events, but they may configure device state or
  partial structures essential to correct timbre.
- **LOW — names, SMPTE offset, copyright, EOT:** useful project/export context
  but usually not needed for basic musical playback; SMPTE importance rises
  for synchronization workflows.

# Recommended family order

1. **Sequence/container discovery and integration.** Supply exact bounds and
   ownership to the proven bounded family decoders without heuristic scans.
2. **SysEx representation.** Only two events occur. Preserve raw bytes and
   timing; expect one targeted List Window capture or controlled payload edit.

Patch bank-removal is not ahead of these families because CC0/CC32 values are
already established and opaque preservation loses no bytes.

# Estimated remaining controlled experiments

- **Likely minimum: 0 for established bounded families.** Controller, Channel
  Pressure, Pitch Bend, bounded initial Tempo, and bounded initial Meter need
  no further controlled edit for their established contracts.
- **Additional conditional experiment: 0–1 for SysEx.** Use one payload-byte
  change only if natural/List Window evidence cannot map its representation.
- Broader generality, optional Patch bank state, or Tempo-map positioning may
  justify later experiments only when those scopes become current priorities.

These are family-level experiments, not one experiment per controller number
or parameter. Discovery and whole-project association are separate structural
milestones and may use natural evidence rather than edits.

# Families likely solvable without controlled edits

- **Controller:** dense NRPN/RPN and initialization sequences may expose one
  repeated record naturally, but only after their source project is established
  or equivalent provenance-controlled material is exported from `newest STUFF`.
- **Pitch Bend:** the provenance-controlled `Bells for her` population has now
  established nine exact bounded runs without an edit; the older 440-event
  `ANALOG` population remains source-unresolved and outside this claim.
- **Additional Patch instances:** the shared bounded representation already
  works when caller bounds are known.
- **Track/instrument names:** exact strings and regular metadata records may be
  recoverable diagnostically without event edits, though semantic ownership
  remains separate.

# Families requiring controlled edits

- **Tempo:** no edit is needed for the bounded initial form. General
  mid-sequence Tempo-map positioning remains unsupported and is a separate
  question.
- **Meter:** no further edit is needed. Natural 4/4/6/8/10/8 evidence,
  controlled 7/8, and correlated 4/4/6/8/7/8 exports establish the bounded
  initial record. Universal historical `cc` policy remains partial but does
  not block musical Meter recovery or standards-valid export.
- **SysEx:** only two distinct payloads exist and neither is verbatim in the
  project. If List Window ground truth cannot map them, one small payload-byte
  change is likely necessary.
- **Bank optionality:** only a controlled removal can prove leading `ff` and
  `ff ff ff`; this is real ambiguity, but low priority for musical recovery.

# Experiment 030 priority assessment

**UNNECESSARY FOR THE CONTROLLER GRAMMAR.** The 395 provenance-controlled
natural records distinguish timing, tag, length, context, number, and value
across CC1/CC7 and multiple tracks. A value edit would not resolve context or
container semantics. Bank-removal optionality remains a separate Patch issue.

# MIDI recovery coverage snapshot

| event_family | present_in_exports | occurrence_count | sequences | tracks | current_status | musical_importance | shared_decoder_leverage | natural_ground_truth_quality | controlled_experiment_needed | priority |
|---|---|---:|---:|---:|---|---|---|---|---|---:|
| Note | yes | 6,109 starts | 3 | 36 | IMPLEMENTED bounded | CRITICAL | high | strong | no for known grammar | covered |
| Patch/Program/bank | yes | 38 PC | 3 | 30 | IMPLEMENTED bounded; transition grammar correlated | HIGH | high | strong | no further Patch experiment | covered |
| Controller | yes; 395 ordinary `Bells for her` events proven, plus Patch bank exports | 5,112 in the earlier three-set inventory, plus 405 in the later provenance export | 4 export sets | 29 earlier plus identified `Bells for her` tracks | IMPLEMENTED BOUNDED | HIGH | very high | strong for CC1/CC7 scope | no | covered |
| Pitch Bend | yes; 102 proven `Bells for her` events plus older unresolved population | 440 in earlier snapshot plus 102 in later provenance export | 2 export sets | 2 earlier plus identified Track 14 | IMPLEMENTED BOUNDED RUN | HIGH | high | strong for one nine-run population | no for observed contract | covered |
| Tempo | yes; bounded initial `Bells for her` form proven | 3 in earlier snapshot plus 1 later provenance export | 4 export sets | 4 conductor tracks | IMPLEMENTED BOUNDED INITIAL | HIGH | medium | strong for initial form | no for bounded initial form | covered |
| Meter | yes; natural and controlled initial forms correlated | initial events across 18 project sequences; 4/4, 6/8, and 7/8 export-correlated | 18 project sequences | sequence-level | IMPLEMENTED BOUNDED INITIAL | MEDIUM | medium | strong for initial form | no | covered |
| SysEx | yes | 2 | 2 | 2 | UNINVESTIGATED | MEDIUM | medium | weak | likely 0–1 | 4 |
| Channel Pressure | yes in later provenance export | 32 | 1 | 1 | IMPLEMENTED BOUNDED RUN | HIGH | medium | strong for one stateful run | no | covered |
| Poly Pressure | no | 0 | 0 | 0 | NOT PRESENT | conditional | unknown | none | no current need | none |
| Names/export metadata | yes | 98 name events | 3 | 42 | PARTIAL | LOW | medium | moderate | not for raw strings | later |

# Evidence supported

- Seven exports form three filename-based sets; only the Ode set is proven to
  represent one of the 18 active `newest STUFF` sequences.
- The earlier three-set inventory lacks pressure, but the later
  provenance-controlled `Bells for her` export adds 32 Channel Pressure events;
  Polyphonic Key Pressure remains absent.
- The non-duplicative view contains 6,109 Notes, 5,112 Controllers, 440 Pitch
  Bends, 38 Program Changes, and two SysEx events.
- Fourteen CC numbers share one MIDI family; most traffic is structured
  NRPN/RPN automation.
- One shared ordinary Controller representation is proven for 395 natural
  CC1/CC7 records in `Bells for her`; broader generality remains unproven.
- Notes, bounded Patch semantics, and bounded ordinary Controller CC1/CC7
  representation decoding are implemented within their documented scopes.
- Experiment 031 establishes direct Patch-to-Note transition after the post-PC
  VLQ and an extended form using one length-framed `ff 60` payload plus final
  timing VLQ. Navigation is deterministic for the established corpus; `ff 60`
  semantics and unsupported optional forms remain unknown.
- Controller discovery and container integration remain unsupported.
- One exact-bounded Channel Pressure run decoder is implemented and validates
  all 32 natural entries; isolated events, discovery, and universal
  running-state semantics remain unsupported.
- One exact-bounded Pitch Bend run decoder validates all nine ranges and 102
  natural events with direct LSB/MSB storage; isolated events, discovery, and
  universal running-state semantics remain unsupported.
- The bounded initial Tempo decoder implements `00 ff 51 03 | MPQN[3]` with
  exact bounds, all-byte provenance, direct unsigned 24-bit big-endian MPQN,
  fixed authentic fixtures, and no scanning. General Tempo maps remain unknown.
- The bounded initial Meter decoder implements `00 ff 58 04 nn dd xx yy` with
  exact bounds, all-byte provenance, safe optional denominator derivation,
  four fixed authentic fixtures, and no scanning. Historical `xx -> cc`
  conversion is exact for correlated `08` and `06` values but remains outside
  decoding and is not universal.

# Unknowns

The 15 active sequences without proven project-correlated exports may contain additional families
or values. Export
initialization may not correspond one-to-one with project events. Broader
Controller/Pitch Bend forms plus SysEx project encoding, broader Meter/Tempo
map variants, state-exit/current-cursor classification, sequence/track
associations, and framing variants remain unknown. No claim is made about
pressure or bend support in Studio Vision generally.

# Single recommended next step

Correlate current-cursor state exit after timing VLQs across established Note,
Channel Pressure, and Pitch Bend transitions. Track-local end and bounded
Patch-to-first-Note navigation are established, but stateful run subdivision
remains unresolved; do not fold heuristic scanning into container navigation.
