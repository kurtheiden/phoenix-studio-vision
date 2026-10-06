# Native macOS recovery validation

## Purpose and baseline

Record completed end-to-end validation of Phoenix's bounded supported recovery
path through the actual native macOS application. Application implementation
baseline: `643d0eb5c993d8f884557d36bca965aa3d7e45fd`,
`Integrate bounded 120-layout recovery with AppService`.

## Authentic source

Source project (read-only):

`/Users/kurtheiden/Documents/Phoenix Research/Authentic Studio Vision Projects/SCHOOL PROJECTS`

- Size: 343875 bytes.
- SHA-256: `bfd4fa1208e2cd884ec51ccfb1131d5c02723d7c90acafc98bd89597e1a20331`.

No private musical data is copied into this document or the repository.

## Native application observations and procedure

The owner opened the authentic project in Phoenix's macOS application. Phoenix
recognized it as a Studio Vision project and classified it **Partially supported**.
All 15 Sequences were visible: eight safely recoverable/exportable and seven
visible but blocked.

The owner selected `DANCING.MID`. It displayed **Ready**, with the statement
“This sequence can be safely recovered as MIDI.” The **Export MIDI** control was
enabled. The owner exported the Sequence through the actual Phoenix macOS UI to:

`/Users/kurtheiden/Documents/Phoenix Research/Phoenix Exports/DANCING.mid`

No Studio Vision, Mac OS 9 or SheepShaver participated in that Phoenix
recovery/export workflow.

## Mechanical comparison

The native UI output was compared directly, byte-for-byte, against the previously
mechanically validated and owner-listened recovery artifact:

`/Users/kurtheiden/Documents/Phoenix Research/Experimental MIDI Recovery/SCHOOL PROJECTS/SCHOOL - DANCING.mid`

| Property | Native UI export | Validated recovery artifact |
| --- | --- | --- |
| Byte size | 64131 | 64131 |
| SHA-256 | `edc52673db86f6c0b6e1307f80d6c827d12be7c17c7cf9bead489cedf6577c22` | `edc52673db86f6c0b6e1307f80d6c827d12be7c17c7cf9bead489cedf6577c22` |

**Byte identical: YES.**

Independent structural parsing of the native UI export established:

- Valid Standard MIDI File: YES.
- SMF format: 1.
- Division: 480 PPQN.
- MIDI Tracks: 11.
- Complete readback: YES, including all Track boundaries and end-of-Track events.

Neither MIDI file was altered, regenerated or listened to during this mechanical
comparison.

## Result and bounded interpretation

**NATIVE UI END-TO-END MIDI PARITY — PASS**

The MIDI exported from the Phoenix macOS UI is byte-for-byte identical to the
previously validated and owner-listened DANCING recovery artifact.

An authentic Studio Vision project was opened in Phoenix's macOS application, a
safely recoverable Sequence was selected, and the normal native application
workflow produced the same MIDI as the established recovery path. This validates
the bounded supported recovery path and its native application handoff.

This result does not establish that every Studio Vision project is recoverable,
that all 15 SCHOOL Sequences are recoverable, historical instrument/timbre
reconstruction, universal Studio Vision format support, or byte equivalence to a
native Studio Vision MIDI export. The comparison target here is the validated
Phoenix recovery artifact. Recovery research remains frozen.
