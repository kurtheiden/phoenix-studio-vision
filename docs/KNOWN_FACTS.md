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
