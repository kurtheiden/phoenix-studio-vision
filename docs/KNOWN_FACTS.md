# Known Facts

This document records direct observations from local samples. Observations do
not establish a file signature, structure, or parser behavior.

## Exact-bounded mixed-event walker validation

- The implemented 166-profile walker consumes Bells Track 9
  `0x0143c8..0x014957` as exactly 184 logical events: 31 Notes, one Patch, 120
  Controllers, and 32 Channel Pressure events.
- It consumes Bells Track 14 `0x014e26..0x015ed4` as exactly 601 events: 227
  Notes, 272 Controllers, and 102 Pitch Bend events.
- One explicit `d0` entry derives the Track 9 Pressure run; nine explicit `e0`
  entries derive all Track 14 Bend runs. No run-end table or scan is used.
- Authenticated Studio Vision exports declare division 480, and correlated
  decoded positions/durations use the same 480-units-per-quarter tick basis;
  the supported 166-profile export mapping is therefore lossless identity.
- Current decoded Note, Controller, Channel Pressure, Pitch Bend, and Patch
  representations do not contain the exported MIDI channel. The observed
  source entry discriminators are not channel-bearing statuses, and Controller
  context remains opaque.
- Independent parsing of `Ode to Clarke Multi All` establishes one channel per
  musical SMF track: Track 1=1, Track 2=2, sys100loops=10, Track 4=10, Track
  5=10, Track 3=1, Track 6=10, Track 3 #2=15, and Track 7=10. Equal descriptor/
  pair counts bind these to Ode pair ordinals 0 through 8.
- No relative byte or high/low nibble in the nine complete 166-byte Ode track
  descriptors matches every one-based or zero-based authenticated channel.
  General SVP channel storage remains unknown. A hash-and-range-locked
  per-track manifest is sufficient policy for the first Ode export proof but
  is not Studio Vision format knowledge.
- The `smf` module now implements pure Format 1 serialization for validated
  PPQN, explicit Note Off/On, Control Change, Program Change, Channel Pressure,
  Pitch Bend, Track Name, initial Tempo, Time Signature, and automatic EOT.
  It uses four-byte-limited MIDI VLQs, deterministic absolute-tick ordering,
  and private EOT-safe track construction without Studio Vision dependencies.
- The provenance-locked Ode Track 3 proof structurally derives and exactly
  walks `0x031300..0x031564`, yielding one Patch and 84 Notes. Phoenix and the
  authenticated Studio Vision track 6 match one-for-one for every Note's
  channel, pitch, start, end, and attack velocity; all 82 explicit reference
  Note Off release velocities also match. Two historical endings use Note On
  velocity zero, while Phoenix preserves the decoded release velocities in
  explicit Note Offs.
- The resulting 856-byte Format 1, PPQN-480, two-track proof has SHA-256
  `6b6553566eeee1e5e47ffe24b3ed4d0fdc7fed933d7f40811778ac6bb4108317`.
  Its channel-1 CC0=81, CC32=2, Program=29 at tick 480 and its initial
  Tempo/Meter match the authenticated export under independent parsing.
- On 2026-08-19 the user opened the proof in Logic Pro 12. Logic displayed the
  expected single musical track as `Track 3`; playback continued correctly
  while instrument patches were changed, with no audible glitches or obvious
  timing, hanging-note, or misplaced-note problems observed. This validates
  the bounded one-track proof in that DAW and does not establish multitrack or
  full-sequence export.
- Read-only Phase D inventory confirms all nine Ode event ranges walk exactly:
  1,312 logical events comprising 1,308 Notes and four Patch events, with no
  ordinary Controller, Channel Pressure, Pitch Bend, or other mixed family.
  All 1,308 Notes match the authenticated reference one-for-one for pitch,
  start, end/duration, and attack; all 1,291 explicit reference Note Off
  releases match, while 17 historical velocity-zero endings carry no release
  value to compare.
- The authenticated Ode reference has no later Tempo/Meter, SysEx, or other
  unsupported channel event. Its additional SMPTE Offset and Instrument Name
  metas and uniform tick-94,080 EOT padding are metadata/policy differences,
  not missing musical events. Descriptor order, pair order, and reference
  musical-track order agree across all nine ASCII-named tracks.
- The reusable `multitrack_export` layer now assembles one adapted conductor
  plus arbitrary caller-ordered decoded musical tracks transactionally. It
  preserves empty tracks and duplicate names/channels, aggregates only
  adapter-derived reports, and returns no successful bytes when any later
  track or Patch translation fails. Independent synthetic parsing verifies
  complete Format 1 structure without authentic-project coupling.
- Authenticated D2 integration now validates Experiment 007's locked source
  hash, Ode sequence identity, nine descriptor/pair/range/channel rows, four
  Patch expectations, and all nine exact walks before invoking D1 once. The
  resulting in-memory Format 1 file has 10 tracks in locked order and the
  expected channel sets; its report totals 1,312 logical source events, 1,308
  Notes/Note Offs, two bank pairs, four Programs, one Tempo, and one Meter.
- Authenticated D3 comparison independently parses that in-memory result and
  the Studio Vision reference with running-status and exact EOF/EOT checks.
  All 1,308 Notes match one-for-one; all 1,291 explicit reference releases
  match; exactly 17 reference endings use velocity-zero Note On. Track
  order/names/channels, conductor state, four Patch translations, and zero
  ordinary Controller/Pressure/Bend inventories match. Remaining differences
  are established metadata, release-representation, raw-encoding/order, and
  tick-94,080 reference EOT policies.
- D4 persisted those exact validated bytes only at the approved external proof
  path. The 12,184-byte Format 1, PPQN-480, ten-track file has SHA-256
  `14d855f9d6c8e609365ac8d45335ca1e6c36fd9ede8299d01fba9d5d0f4a72eb`.
  Its disk bytes equal the D3 buffer, independently parse to exact EOF, and
  repeat the full normalized comparison successfully.
- On 2026-08-19 the user opened that exact multitrack proof in Logic Pro 12.
  All nine expected musical tracks were visible in the established order; the
  user reported that it looked and sounded good, with no playback problem.
  Logic's Event List/filter showed nine top-level MIDI regions/tracks, not an
  independent count of the 1,308 Notes established by automated D3/D4
  comparison. This completes the bounded authenticated Ode proof cycle only.
- Synthetic tests independently parse a complete generated two-track SMF and
  verify header/chunk bounds, delta VLQs, legal explicit messages, final EOTs,
  and exact file consumption. No authentic MIDI artifact is generated.
- The `midi_export` module now transactionally converts already decoded Note,
  ordinary Controller, Channel Pressure, Pitch Bend, and explicitly classified
  Patch values into SMF scheduled events under an explicit channel and
  `Identity480` timing policy. It does not parse or locate source bytes.
- Each Note produces a Note On and checked generated `8n` Note Off preserving
  release velocity. Stable source ordinals are even and their generated Note
  Off ordinals odd; duplicate/overflowing source ordinals fail.
- Meter historical mappings `08 -> 24` and `06 -> 12`, standards fallback
  warnings, initial Tempo validation, export counts/reporting, and synthetic
  adapter-to-Format-1 integration are implemented. Invalid non-UTF-8 text is
  explicitly deferred rather than silently converted.
- The walker returns output only after exact event-range consumption and
  rejects unknown branches at the current cursor.

Unknown families, other context forms, the 120-byte profile, and MIDI export
remain outside this implemented contract.

## `newest-stuff-001`

- The local sample identifier is `newest-stuff-001`.
- The file has no extension.
- The file size is 171953 bytes.
- Its SHA-256 digest is
  `c44d415a4b69d56abd5680652ed99039a4f9ca9afd281898601ccc14026aebec`.
- Finder metadata identified the type code `MID2` and creator code `MIDA`.
- Printable device-related strings occur within the first 256 bytes, including:
  - `IAC Bus #1`
  - `JD-800s #1`
  - `JD-990s #1`
  - `Juno-106#1`
  - `JV-1080`
  - `JV-880`
- Repeated byte patterns surround several device names.
- No interpretation of those structures is confirmed yet.

## Artifact inventory observations

- Finder creator code `MIDA` was observed on `newest-stuff-001`, the
  `StudioVision.app` application, and the `Studio Vision Setup` artifact.
- Finder type code `APPL` was observed on the OMS 2.3.8 installer and the
  `StudioVision.app` application.
- Finder type code `MIDS` was observed on `Studio Vision Setup`; type code
  `MID2` was observed on `newest-stuff-001`.
- Resource forks were directly present as extended attributes on the OMS 2.3.8
  installer, `StudioVision.app`, and `Studio Vision Setup`.
- The OMS 2.3.8 installer resource fork is 295885 bytes, the
  `StudioVision.app` resource fork is 2112471 bytes, and the
  `Studio Vision Setup` resource fork is 286 bytes.
- The `StudioVision.app` data fork was identified by `file` as a PowerPC PEF
  executable. Its data fork begins with `Joy!peffpwpc`, and its `cfrg` resource
  contains `pwpc` and `StudioVision PPC`.
- The Studio Vision Pro 4.5 `Documentation` directory inventory contains five
  PDF 1.2 documents and one non-document `Icon\r` entry. Sizes, SHA-256
  digests, filesystem dates, and directly observable PDF metadata were
  recorded for those entries.

These observations do not establish general Finder-code meanings, resource
fork semantics, file-format structures, parser behavior, or compatibility.

## Bounded Track 7 parser spike

- A new `track7` library module decodes only an explicitly supplied local
  sequence of timing VLQ, three property bytes, and duration VLQ.
- The observed values `81 65`, `83 60`, `81 70`, `83 3a`, `81 75`, and `6b`
  decode mechanically as 229, 480, 240, 442, 245, and 107.
- The authentic Experiment 007 local range at offsets `0x00031c1d` through
  `0x00031c30` produced the documented values 229/442, 480/245, and 240/107,
  with provisional accumulated intervals 229, 709, and 949.
- Read-only runs on Experiments 019, 020, and 022 reproduced their controlled
  timing changes exactly.
- The third decoded structure `81 70 | 24 7f 60 6b` was predicted before the
  corresponding Studio Vision check and then matched the sixth List Window
  event at position `26·2·229`: C1, attack 127, release 96, duration 107.
- The leading value 240 remains timing-related/provisional; the independent
  check does not establish its ownership or conversion to displayed position.
- Three overlapping Track 7 List Window screenshots reconcile to 143 unique
  rows. Rows 4–143 align strictly to 140 consecutive binary candidates with
  560/560 musical-property field agreement.
- Under a four-beat/480-unit coordinate calculation, all 140 displayed
  start-to-start differences equal the later candidate's timing interval.
- The final visible aligned candidate begins at `0x00031f96` and returns
  cursor `0x00031f9c`; no structural failure occurs within the screenshot
  evidence bound.
- Boundary follow-up found matching property bytes for rows 1–3 at `0x31c0c`,
  `0x31c12`, and `0x31c18`. Rows 2 and 3 have `81 70` timing prefixes; row
  1's preceding timing field remains ambiguous.
- Bytes after the final cursor begin `ff fa b9 2f ff 2f ...`; conservative
  pitch/velocity plausibility stops the note model at `0x31f9c`. No footer,
  terminator, or track-framing semantics are established.

These are bounded diagnostic observations, not evidence of complete Track 7
framing, SMF delta-time semantics, channel/status encoding, or MIDI export.

Five additional bounded candidate chains were found by the same conservative
multi-structure discovery test at `0x2fb7a–0x300d8`, `0x301d8–0x30976`,
`0x30a32–0x30e98`, `0x30f4c–0x31254`, and `0x31677–0x317fe`. Nearby candidates
repeatedly contain `2c c4 b2` before the chain and
`ff fa ?? ?? ff 2f 00 29 ...` after it; identities and field semantics remain
unknown.

Independent complete Studio Vision ground truth for `Ode to Clarke` / `Track
3 #2` / `JD-800` reconciles to one Patch event plus 84 notes. All 84 notes
strictly match the project chain at `0x318b5–0x31afe`: 84/84 pitch, attack,
release, duration, and complete rows (336/336 fields), plus 83/83 note-to-note
timing intervals. The earlier 17-note sample is note indices 33–49 at
`0x31994`; its 68/68 properties and 16/16 timing results remain valid.

The prior identification of the `0x312fc` / Wavox region as Track 3 #2 was
wrong because that 17-note sample had been misdescribed as the start of the
List Window. The correct region has marker `0x31882`, literal `Ming Dynasty`,
and first-note properties `55 64 7f 83 4d` at `0x318b5`.

The corrected Track 3 #2 region has `00 00 00 56` (86) eight bytes before its
marker despite 85 displayed total events. Track 7 has 143 at the analogous
location and 143 displayed notes. The corrected two-track evidence therefore
contradicts interpreting that field as an exact total List Window event count,
while repeated local chain framing remains supported.

Track 3 #2 stores literal `Ming Dynasty` in its pre-note region and has
`ff ff ff 17` nearby; an existing Studio Vision SMF export independently has a
Program Change data value 23 at the Patch position. This partially identifies
the Patch representation but does not establish complete Patch field or timing
framing.

Experiment 023 changed only displayed `PC 23` to `PC 24`. The preregistered
local candidate changed exactly `0x318a5: 17 -> 18` at the same offset. Literal
`Ming Dynasty`, the complete 84-note stream, all note timing, first-note
boundary, and post-chain context remained byte-identical. This identifies the
direct Program Change value field for this event instance and supports the
literal name as independently stored/editable from the program value.

Three adjacent context bytes at `0x31883–0x31885` also changed (`c4 b2 5c ->
c7 85 24`) and remain unexplained. Consequently `2c c4 b2` is not an invariant
marker across this controlled save. Complete Patch-event framing, its exact
type field, and Patch timing representation remain unestablished.

Experiment 024 was independently created from Experiment 007 and changed only
displayed `PC 23` to `PC 100`. The locked prediction was confirmed at the same
aligned field: `0x318a5: 17 -> 64`. The unique `Ming Dynasty` literal, complete
585-byte 84-note stream, all 83 note-to-note timing fields, first-note boundary,
and post-chain context remain byte-identical. The three aligned displayed and
stored states are therefore 23/`17`, 24/`18`, and 100/`64` for this event.

The nearby three-byte field is `c7 85 1c` in Experiment 024. Its three-state
values (`c4 b2 5c`, `c7 85 24`, `c7 85 1c`) do not track PC by equality, fixed
delta, or monotonic change. Related three-byte marker-family changes occur
widely across both saves, supporting a save-dependent serialization/reference
classification but not establishing exact semantics. Patch framing, type, and
timing remain partial or unknown.

Experiment 025 moved only the same Patch event from `1·2·50` to `1·2·51` in a
fresh Experiment 007 duplicate. The primary absolute-position field at
`0x31886–0x31887` changed from 7-bit VLQ `84 12` = 530 to `84 13` = 531,
exactly matching the established 4/4, 480-units-per-beat coordinate. A local
interval component at `0x318a6–0x318a7` changed `c5 4c` = 8,908 to `c5 4b`
= 8,907; with stable `81 25` = 165, the sum changes from the preregistered
9,073 to 9,072. Six post-chain fixed-width copies also change 530 to 531.

The confirmed PC field remains `0x17`, literal `Ming Dynasty` remains
unchanged, and the complete 84-note stream plus all 83 note-to-note timing
fields are byte-identical. Patch timing uses the same numeric unit and 7-bit
VLQ mechanics as established note timing, while its primary field is absolute
rather than a note-to-note interval. The primary Patch span is now strongly
supported from `0x31886` through `0x318a7`, but unresolved metadata, compound
interval ownership, complete end framing, and the Patch event-type
discriminator remain partial.

Experiment 026 changed only the editable Patch name from `Ming Dynasty` to the
equal-length `Phoenix Test` in a fresh Experiment 007 duplicate. The aligned
payload at `0x31891–0x3189c` changed directly between the two 12-byte ASCII
strings without relocation. The preceding byte at `0x31890` remains `0x0c`,
strongly suggesting a Pascal-style length prefix, but equal-length evidence
does not establish its behavior or a general fixed/variable field width.

The Patch absolute-position VLQ remains `84 12` = 530, the direct PC field
remains `0x17`, both Patch-to-first-note timing components remain stable, and
the complete 84-note stream plus 83 note-to-note timing fields are
byte-identical. The name payload remained stable in PC-only Experiments 023/024
and position-only Experiment 025, changing only in Experiment 026. No
additional literal name copy or name-specific downstream reference was found.
Position, name, and PC now occupy independently controlled fields in one
coherent local span, but name-length framing, complete event end ownership,
compound interval grammar, and the event-type discriminator remain partial.

Experiment 027 changed only the Patch name from 12-character `Ming Dynasty` to
7-character `Phoenix` in a fresh Experiment 007 duplicate. The project becomes
exactly five bytes smaller. At the same pre-name offset, the locked prediction
is confirmed: `0x31890: 0c -> 07`, followed immediately by exactly seven ASCII
bytes. No padding or residual name bytes remain; all following Patch and Note
structure relocates by `-5`.

The PC byte moves from `0x318a5` to `0x318a0` and remains `0x17`. The first-note
properties and complete chain move from `0x318b5–0x31afe` to
`0x318b0–0x31af9` and remain byte-identical across all 84 note properties and
83 timing fields. Absolute Patch position remains `84 12` = 530 at its fixed
pre-name offset. A local payload-length field changes `1b` = 27 to `16` = 22,
exactly spanning its following bytes through PC, and a broader 32-bit size
candidate changes 653 to 648. Six downstream offset-like fields also decrease
by five. These results establish a one-byte-length-prefixed variable-length
ASCII name with no fixed-width padding and justify a bounded diagnostic parser
spike for this known representation, not a general Patch parser.

The bounded Track 3 #2 Patch decoder spike accepts an explicit start and end,
does not scan, and decodes only confirmed absolute position, one-byte-length-
prefixed ASCII name, direct PC, and the transition to `0x90`. Read-only tests
derive the expected semantic states from Experiments 007 and 023–027. The
five 12-character states reach Note status at `0x318b4`; Experiment 027 derives
the relocated offset `0x318af`. Malformed bounds/VLQs, truncated or non-ASCII
names, inconsistent payload length, unexpected known context, and missing Note
transition are rejected without recovery. General Patch discovery/grammar,
unknown fields, event-type semantics, interval ownership, and MIDI emission
remain unsupported.

An independent authentic event is now strongly identified at `0x31300` as
`Ode to Clarke` / `Track 3` / `JV-1080`: MIDI PC 29 at tick 480, four complete
following note rows, literal `Wavox`, and the project timing fields agree. It
repeats a two-byte absolute-position VLQ plus `ff 7c`, one-byte-length-prefixed
ASCII name, direct PC, two-byte post-PC VLQ, and transition to Note status
`0x90`. Its payload-length relationship and pre-name, post-name, and pre-Note
context differ from Track 3 #2. The unchanged bounded decoder therefore fails
at `0x31305`. This is partial generalization evidence for the semantic fields,
not evidence for one invariant local Patch layout.

Track 1 / Juno-106 is independently identified in the authentic project from
91 exported notes: Patch start `0x2f833`, one-byte position VLQ `00`, literal
`Empty Patch`, direct PC `3d` = 61 at `0x2f84f`, post-PC `cb 78` = 9,720, and
Note status `0x2f852`. All 91 pitches, attacks, and durations and all 90 timing
fields match; 89/91 release velocities match, with two SMF zero-velocity
note-offs corresponding to project `0x40`.

Across Track 1, Track 3, and Track 3 #2, `ff 7c`, one-byte local payload length
through PC, one-byte-length-prefixed ASCII name, direct PC, two-byte post-PC
VLQ, and transition to `0x90` recur. Track 3 exports CC0=81/CC32=2 and stores
`ff 51 02`; the two no-bank-export events store `ff ff ff`. Position width,
post-name width, and pre-Note context vary. This supports a common semantic
core, not a common fixed layout.

The proposed shared Patch contract is explicitly bounded by a caller-supplied
position start and an exclusive end immediately after the known `0x90` Note
status. It uses the one-byte payload length to locate PC as the final payload
byte, preserves five-byte pre-name, variable post-name, and variable pre-Note
contexts as borrowed bytes with absolute ranges, and performs no scanning or
recovery. Variable-width VLQs are returned with raw provenance; post-PC timing
is not assumed to be the complete interval. This is a design, not implemented
parser behavior.

`decode_bounded_patch_representation` now implements that contract. It derives
all three authentic events and Experiments 007/023–027, including variable
position width, variable post-name width, and zero/12-byte pre-Note context.
Opaque bytes are borrowed with absolute ranges; PC is payload-derived and
post-PC timing remains neutral. The strict Track 3 #2 decoder remains
independent and retains its previous behavior. No discovery, general parser,
CLI, or MIDI emission was added.

Authentic Track 2 / JV-1080 is independently identified by its 211-note MIDI
stream and decodes unchanged with bounds `0x2fb55..0x2fb75`: position 0, name
`Stereoww Bs`, PC 37 at `0x2fb71`, post-PC 1,920, empty pre-Note context, and
status `0x2fb74`. Its post-name tail `ff 51 01` matches exported CC0=81 and
CC32=1, strengthening—but not proving—the bank-field interpretation. It adds
opaque values but no new framing variant.

Experiment 028 changed only Track 2 CC32 from 1 to 2 in a fresh Experiment 007
duplicate. At the aligned post-name tail, the locked prediction was uniquely
confirmed: `0x2fb6e..0x2fb70: ff 51 01 -> ff 51 02`; only `0x2fb70` changed
inside the bounded Patch representation. Position 0, payload 25, name
`Stereoww Bs`, PC 37, post-PC timing 1,920, Note boundary, and the entire
211-note binary stream remained stable. This directly identifies the final
tail byte with CC32 for this representation; CC0 and `ff` semantics remain
partial/unknown.

Experiment 029 changed only Track 2 CC0 from 81 to 82 in another fresh
Experiment 007 duplicate. The locked prediction was uniquely confirmed at the
same aligned tail: `ff 51 01 -> ff 52 01`, with only `0x2fb6f: 51 -> 52`
changing inside the bounded representation. The entire 211-note chain and all
other Patch fields remain stable. Together Experiments 028/029 establish
direct CC0 and CC32 value storage for this representation; leading `ff` and
no-bank optionality remain unproven.

Studio Vision UI ground truth establishes the untouched baseline's active
sequence inventory: A `xForm`; B `Bells for her`; C `Situation`; D `Sequence
D`; E `Sequence E`; F `Girl-U-Want`; G `mission impossibl`; H `happyone`; I
`Sequence I`; J `newsong`; K `Sequence K`; L `Renaissance`; M `Get on up &
Dance`; N `Jurrasic Park`; O `Ode to Clarke`; P `Over the Top`; Q `Sequence
Q`; R `Sequence R`. Neither `ANALOG.MID #2` nor `Analog Seq` is an active
sequence. A readable project string, export filename, or SMF track-name event
does not establish active-sequence identity without UI or independent
structural evidence.

The Project 001 MIDI-export directory contains seven SMFs in three
filename-based sets. Only the three-file `Ode to Clarke` set is proven to come
from `newest STUFF`, through UI identity and extensive Note/Patch/timing/binary
correlation. The `ANALOG.MID #2` and `BATTL2GS.MID` sets have unresolved source
project and sequence provenance and may come from another project. The
non-duplicative aggregate still contains 6,109 Note starts and endings, 5,112
Control Changes, 440 Pitch Bends, 38 Program Changes, and two SysEx events, but
those are Studio Vision export-format observations, not `newest STUFF`
coverage. Of the 5,112 Controllers, 4,865 are in unresolved `ANALOG`, 243 in
unresolved `BATTL2GS`, and four in proven `Ode to Clarke`.

The provenance-controlled full multitrack export of the untouched baseline's
`Bells for her` sequence contains 405 Control Changes. Ten are CC0/CC32 export
messages derived from Patch bank state; the remaining 395 ordinary Controllers
correspond one-for-one to project records of the form `timing VLQ | ff 41 | 05
| opaque context[3] | controller number | controller value`. Track 9 matches
120/120 number/value pairs and 120/120 event-start deltas across CC1 and CC7.
Tracks 3, 4, and 6 independently match CC7=127, and Track 14 supplies 272
matching CC1 records. Context meanings and broader generality remain unknown.

The production `controller` module now decodes one exact caller-bounded
ordinary Controller record and returns timing, tag, payload, context, number,
value, and absolute byte provenance. Authentic fixed-offset tests cover Tracks
3, 4, 6, 9, and 14, both CC1 and CC7, and one- and two-byte timing including
zero. Malformed bounds/framing fail without scanning. Discovery, absolute time,
context semantics, and broader grammar generality remain unsupported.

The existing bounded decoders do not yet compose into a generic mixed-event
walker. Controller records have a current-cursor tag and derivable total length;
consecutive Notes have derivable ends only inside a caller-asserted Note chain;
and Patch decoding still requires a caller-known boundary after the first-Note
status. In Track 9, a track-region-only walker can decode the initial Controller
at `0x143c8..0x143d1` but must stop at the Patch beginning `0x143d1`; it cannot
derive the Patch-to-first-Note handoff or reach an identified Channel Pressure
record without guessing.

Independent event-order correlation later bounds the Track 9 Channel Pressure
run at `0x1478c..0x147ce`. Its entry is `82 20 d0 01` (delta 288, direct value
1), followed by 31 `timing VLQ | direct value` continuations. Values and
event-start deltas match the export 32/32. The run begins exactly after the
known CC1=0 record and ends before `83 56 90 ...`, the aligned following Note.
This establishes one stateful natural run, not isolated pressure encoding,
universal `d0` meaning, MIDI channel semantics, or global running status.

The production `channel_pressure` module now decodes one exact caller-bounded
run. It requires the `d0` entry, preserves tag/timing/value provenance, parses
continuations only in established run state, and consumes the supplied range
exactly. Authentic fixed-offset tests cover all 32 timing/value pairs and both
neighbors; malformed and oversized bounds fail without scanning. Run discovery
and generic mixed-event walking remain unsupported.

Independent Track 14 correlation establishes 102 natural Pitch Bend events in
nine exact project runs. Every run begins `timing VLQ | e0 | LSB | MSB`; the
remaining 93 events use `timing VLQ | LSB | MSB`. Project bytes preserve the
exported seven-bit data bytes directly, with `raw = LSB + (MSB << 7)`, and
values and event-start deltas match 102/102. Eight runs end at independently
matched Notes and the ninth at an ordinary Controller. Exact caller run bounds
remain necessary; `e0` has no assigned MIDI-channel/status semantics, isolated
forms and broader generality are unknown, and no controlled experiment is
needed for the observed bounded contract.

The production `pitch_bend` module now decodes one exact caller-bounded run,
requires `e0` at entry, preserves timing/LSB/MSB/tag provenance, derives raw
value without replacing stored bytes, and accepts continuations only in
established run state. Fixed authentic tests cover all nine ranges and all 102
timing/value tuples plus Note/Controller adjacency and malformed bounds. Run
discovery and generic mixed-event walking remain unsupported.

The provenance-controlled `Bells for her` export contains one Tempo meta-event
at tick 0 with payload `09 10 8b` = 594,059 microseconds per quarter note. The
authentic project range `0xebd8..0xebdf` is exactly
`00 ff 51 03 09 10 8b`. Controlled Experiment 002/004 range
`0x2f7dc..0x2f7e3` is respectively `00 ff 51 03 07 a1 20` at 120 BPM and
`00 ff 51 03 07 0a e2` at 130 BPM. The three-byte payload is direct unsigned
24-bit big-endian MPQN; natural and controlled primary framing agrees.

All established primary examples have a leading zero and represent initial
Tempo at sequence start. The byte is not established generally as absolute
position or delta. The primary lies in a sequence-level Meter/Tempo structural
area, outside the known performance-event streams. Later container correlation
bounds the correlated secondary `51 | MPQN` copy inside its type-`0x29`
record, but the record's purpose and field semantics remain unknown. General
Tempo map discovery and mid-sequence positioning remain unknown.

The production `tempo` module now decodes only an exact caller-bounded
seven-byte initial form. It requires `00 ff 51 03`, preserves all seven bytes
as `LocatedByte` values with absolute offsets, derives unsigned 24-bit
big-endian MPQN, and returns no BPM for MPQN zero. Fixed authentic tests cover
the natural 594,059 MPQN and controlled 500,000/461,538 MPQN states. Synthetic
tests cover the full 24-bit endpoints, exact-bound failures, every fixed
structural byte, nonzero slice provenance, and no scanning. Position semantics
remain partial; secondary-copy and general Tempo-map parsing remain absent.

All 18 authenticated sequences contain one initial primary Meter form with
exact grammar `00 ff 58 04 nn dd xx yy`. Natural `Bells for her` 4/4 is
`0xeb80..0xeb88 = 00 ff 58 04 04 02 08 08`; natural `Sequence K` 6/8 is
`0x258df..0x258e7 = 00 ff 58 04 06 03 06 08`; controlled Experiment 030
`Bells for her` 7/8 is `0xeb80..0xeb88 = 00 ff 58 04 07 03 06 08`.
The project also has a structurally supported `mission impossibl` 10/8 form at
`0x1c864..0x1c86c = 00 ff 58 04 0a 03 06 08`.

Provenance-controlled SMF events establish direct numerator and denominator-
exponent export and direct observed `yy = 08` to `bb = 08`. Studio Vision
converts the third project payload: natural 4/4 maps `xx 08 -> cc 18`, while
natural 6/8 and controlled 7/8 both map `xx 06 -> cc 0c`. The general semantic
meaning and universal historical conversion of `xx` remain partial, but all
source bytes can be preserved and standards-valid Meter can be exported from
the established musical fields.

Every primary leading byte is zero and every correlated event is initial at
tick zero; absolute/delta semantics and nonzero forms remain unknown. Meter is
sequence-level structure beside Tempo, outside performance-event streams.
Nearby `58 nn dd xx yy` copies correlate with the primaries. Later container
correlation bounds each inside a type-`0x29` record, but the record's purpose
and field semantics remain unresolved. The exact eight-byte initial primary is
ready for a caller-bounded decoder; general Meter-map parsing is not.

The production `meter` module now implements that exact caller-bounded initial
form. It requires `00 ff 58 04`, preserves all eight bytes with absolute
provenance, exposes all four payload bytes without imposing historical export
semantics, and derives `2^dd` safely as `Option<u64>`. Fixed authentic tests
cover natural 4/4, natural 6/8, controlled 7/8, and project-only natural 10/8.
Synthetic tests enforce exact bounds, structural errors, arbitrary third and
fourth payload preservation, high-exponent safety, and no scanning. General
Meter-map and secondary-copy parsing remain absent.

Read-only correlation of all 18 authenticated sequences establishes a repeated
sequence/container chain in Experiment 007. Each sequence has a 208-byte
preamble whose byte `+5` equals total descriptor count, followed by 166-byte
descriptors beginning with Meter and Tempo descriptors. Descriptor labels begin
at `+15`; the Pascal sequence-name length field occurs at
`descriptor_start + count * 166 - 15`.

After the sequence name, records use `type:u8 | big-endian u32 payload length |
payload`. Optional `0x09` records precede Meter primary `0x02`, Meter secondary
`0x29`, Tempo primary `0x02`, and Tempo secondary `0x29`. Both bounded primary
events begin at payload `+14`. Track `0x02`/`0x29` pairs follow, then a terminal
`0x00` record. For all 17 adjacent sequence pairs, the terminal record end is
exactly the next 208-byte preamble start.

Track primary records now have exact length-derived containing ranges and
their order strongly matches local descriptor order across independently
identified Bells and Ode tracks. `Sequence I` has eleven track descriptors but
only ten record pairs, so inactive/blank descriptor handling remains partial.
The primary length alone does not derive inner performance-event ends or
Pressure/Bend run bounds. Later tail correlation establishes the 166-profile
event end only after separately validating the final seven-byte structure;
Pressure/Bend run bounds remain partial.

Project-root correlation establishes an eight-byte opaque root header followed
at `0x00000008` by the same checked `type:u8 | big-endian u32 length | payload`
grammar. In Experiment 007, 527 consecutive top-level records consume exactly
through EOF. The 109th boundary is `0x00006abc`, where the first of 18
authenticated type-`0x01` sequence preambles begins. The immediately preceding
type-`0x2f` record is exactly `0x00006aab..0x00006abc`, with length 12 and
payload `00 83 00 02 02 66 01 30 00 24 00 00`. Comparable projects likewise
place one length-12 type-`0x2f` immediately before their first type-`0x01`
sequence. The root walk is cross-validated from offset eight to exact EOF in
the older authentic sample and all controlled project states through
Experiment 030. The root-header words and record-type semantics remain opaque;
no pointer interpretation or content-signature locator is required.

The approved narrow parser design preserves the eight-byte root header and all
generic records, classifies type-`0x01` candidates only after full local
validation, and supplies exact initial Meter/Tempo ranges plus track-primary
containing bounds. Its first semantic profile supports only the established
208-byte preamble/166-byte descriptor form. The older 120-byte descriptor form
remains generically frameable but is deterministically unsupported for
sequence classification; the fourth root word is not promoted to a universal
descriptor-width selector. Sequence I's 11 descriptors and 10 pairs are
accepted as unresolved associations rather than repaired speculatively.

The production `sequence_container` module implements this design. Generic
root framing preserves unknown and zero-length records and consumes exact EOF;
the separate `parse_project_166` API strictly validates every type-`0x01`
candidate. Authentic tests derive all 527 Experiment 007 records, 18 sequences,
Bells and Ode bounds, and the Sequence I mismatch. The older sample's 495
records frame generically while semantic 166-profile parsing rejects its first
candidate. No scanning, 120-byte fallback, mixed-event parsing, or exact inner
event-end inference is present.

Read-only follow-up correlation now establishes the exact inner event end for
the authenticated 166-byte profile without changing that production parser.
All 132 track-primary payloads end with a seven-byte structural form
`ff aa bb cc ff 2f 00`; all 15 zero-count tracks contain only this form after
payload `+14`. Independently bounded Ode Note chains and the last Bells Track
14 Controller end exactly where it begins. The performance-event region is
therefore payload `+14 .. payload.end - 7` after validating the tail grammar.
The three middle bytes remain opaque, and the result is not generalized to the
older 120-byte profile.

Byte-exact inspection corrects Bells Track 9's event end to `0x014957`, not
the earlier `0x014956`: byte `0x014956 = 2f` is the last property byte of the
final Note, followed by tail `ff f6 fd 6b ff 2f 00`. Exact track termination
does not by itself solve internal family transitions. Controller records have
exact next cursors; internal Note-run exit and stateful Channel Pressure/Pitch
Bend run exit remain partial.

Controlled Experiment 031 moved only Ode Track 3 #2's first Note from `6·1·3`
to `6·1·4`; its primary range and length remained unchanged. Final timing
`81 25` (165) became `81 26` (166), while post-PC `c5 4c` (8,908) and the
first `90` offset were unchanged. The following Note interval independently
became `81 63 -> 81 62` (227 -> 226), confirming a one-unit-later first Note
with the next Note fixed.

The intervening bytes are an explicit `ff 60 07` record with seven payload
bytes, followed by the separate final timing VLQ. Ode Tracks 1/2/3 establish a
direct Patch form whose post-PC VLQ is followed by `90`; Ode Track 3 #2 and
Bells Track 9 establish an extended form with the length-framed `ff 60` record,
final timing VLQ, then `90`. The direct timing is the complete interval; the
extended interval is the sum of post-PC and final timing. Navigation is exact
for this bounded corpus without scanning. The semantic purpose of `ff 60`,
other optional forms, broader-version generality, and current-cursor state-exit
grammar remain unknown.

Current-cursor correlation across the complete Bells Track 9 and Track 14
event regions analyzes 785 event-to-next/termination transitions. After a
timing VLQ, all 356 same-family Note/Channel Pressure/Pitch Bend continuations
begin with a data byte below `0x80`; all 393 `ff` cases are tagged Controller,
Patch, or length-framed Note-entry context branches; and all 34 other high-bit
cases are explicit `90`, `d0`, or `e0` entries. Two final events stop at exact
`event_end` before the seven-byte tail.

Under active family state, a first post-VLQ data byte continues the family with
its known payload width, while a high-bit byte exits to strict tagged/status
classification. This rule mechanically reproduces all 184 Track 9 events and
all 601 Track 14 events, including the 31 Pressure continuations and all nine
Pitch Bend run boundaries, without run bounds, scanning, backtracking, or
musical plausibility. It establishes the first bounded mixed-walker design gate
for the supported families only. Unknown tags/statuses, unobserved families,
other profiles, and malformed high-bit data remain unsupported rather than
guessed.
