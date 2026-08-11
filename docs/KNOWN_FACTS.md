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
