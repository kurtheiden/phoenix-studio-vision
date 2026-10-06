# Native macOS batch recovery validation

## Purpose and baseline

Record completed end-to-end batch recovery through the actual Phoenix macOS
application. Validation baseline: `aad84e1cf366d5a4a5db622111b04be9eb147d42`,
`Clarify recovery destination chooser`.

Mechanical evidence is transcribed from
`/tmp/phoenix-native-ui-batch-recovery-validation.txt`. It is not recomputed here.
This record complements the existing single-Sequence native UI validation.

## Authentic project and native workflow

Source project:

`/Users/kurtheiden/Documents/Phoenix Research/Authentic Studio Vision Projects/SCHOOL PROJECTS`

The owner performed the following workflow in the actual Phoenix macOS app:

- Opened SCHOOL PROJECTS; Phoenix recognized the project.
- Observed 15 visible Sequences: eight recoverable and seven **Not yet recoverable**.
- Confirmed the initial recovery selection was empty.
- Clicked **Select All Recoverable**; Phoenix displayed **8 selected**.
- Exactly the eight Ready Sequences were selected; blocked Sequences remained unselected.
- The primary action became **Recover 8 Sequences…**.
- Chose one new empty destination folder.
- Phoenix recovered the selected Sequences sequentially and reported **“8 sequences recovered.”**
- Finder showed eight MIDI files; **Reveal in Finder** worked.

Live native-UI destination:

`/Users/kurtheiden/Documents/Phoenix Research/Phoenix Exports/SCHOOL PROJECTS Batch Test`

The UI observations are owner evidence. The mechanical validation independently
checked the on-disk outputs without modifying, regenerating or listening to them.

## Exact mechanical evidence

Validated references reside under:

`/Users/kurtheiden/Documents/Phoenix Research/Experimental MIDI Recovery/SCHOOL PROJECTS`

Exactly eight regular MIDI files were present, with no additional directory
entries. All eight corresponded to the established recoverable SCHOOL Sequences.
Unnamed mappings use established source-order identity: Sequences 8, 9 and 13,
with previously established reference identities H, unnamed #9 and M respectively.
Correspondence was not inferred from filename similarity alone.

The table below is reproduced exactly from the mechanical validation report.
Each SHA-256 is the identical hash of both the output and reference; sizes are
shown as output/reference bytes.

| Sequence | Phoenix output filename | Reference filename | Size (output/reference bytes) | SHA-256 (output = reference when identical) | Byte identical | Complete SMF readback |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | DANCING.mid | SCHOOL - DANCING.mid | 64131/64131 | edc52673db86f6c0b6e1307f80d6c827d12be7c17c7cf9bead489cedf6577c22 | YES | YES |
| 2 | NOTHING.mid | SCHOOL - NOTHING.mid | 26523/26523 | 8912e1ed99bee8aa8684db9b1ed87ed6f980a640ad2153b4d62f2a68fdb0c23c | YES | YES |
| 5 | Nothing FINAL.mid | SCHOOL - Nothing FINAL.mid | 44633/44633 | 4366b0b3f83eadc4e36166ae48258bc0d0282f29713c783da489f1881eae4ca3 | YES | YES |
| 8 | Sequence 8.mid | SCHOOL - Sequence H.mid | 2857/2857 | b7b0d39da264919f273e36886eae4913a5ff86dc8d6122a9576ca96c31935cc5 | YES | YES |
| 9 | Sequence 9.mid | SCHOOL - Unnamed Sequence 09.mid | 1098/1098 | a4421055e4f6d6915e7b04e196b87590f3689735a768a0d25ca9b62feca00fc2 | YES | YES |
| 10 | Wind ensemble.mid | SCHOOL - Wind ensemble.mid | 10862/10862 | 0c6bd6e3b9c9ab272dcee6fe9d6479062495ca23db7eb5916ead5325fcb665b6 | YES | YES |
| 12 | Tuba thing.mid | SCHOOL - Tuba thing.mid | 787/787 | 4085abaa837942e426a6c95895b414ce77345488ba2e2fcb4d73d4470f97bcc5 | YES | YES |
| 13 | Sequence 13.mid | SCHOOL - Sequence M.mid | 61/61 | e8f988e8a7b983588ef12dd1c0f26a5fb9ac102f6fcace85529f59f8a39ec21c | YES | YES |

All eight passed complete Standard MIDI File readback: valid headers, expected
format 1, valid PPQN division, all declared Track chunks satisfied, valid
MIDI/meta/SysEx framing, valid End-of-Track handling and no unread trailing bytes.
All eight were byte-for-byte identical to their corresponding previously
validated Phoenix recovery artifacts. Zero blocked Sequences produced output.

## Result and bounded interpretation

**NATIVE UI END-TO-END BATCH MIDI PARITY — PASS**

The Phoenix macOS native UI selected all eight safely recoverable Sequences
from the authentic SCHOOL PROJECTS Studio Vision project, recovered them
through one batch operation to a single destination folder, and produced
eight MIDI files byte-for-byte identical to the corresponding previously
validated recovery artifacts.

The seven Sequences Phoenix classified as not yet recoverable remained visible
and produced no MIDI outputs.

This result does not claim that the seven blocked Sequences were recovered,
universal Studio Vision support, support for every Studio Vision project,
historical instrument/timbre equivalence, or byte equivalence to Studio Vision's
native MIDI exports unless separately established for a particular artifact.
The comparison here is to validated Phoenix recovery artifacts.

No private musical contents are copied into the repository. This is a
documentation-only evidence record; recovery research remains frozen.
