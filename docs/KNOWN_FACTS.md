# Known Facts

This document records direct observations from local samples. Observations do
not establish a file signature, structure, or parser behavior.

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
