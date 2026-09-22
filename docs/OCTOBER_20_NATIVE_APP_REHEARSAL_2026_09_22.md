# October 20 native-app rehearsal — September 22, 2026

## Purpose and evidence boundary

This document records Kurt's manual rehearsal of the native Phoenix macOS
application on September 22, 2026, in preparation for the October 20
GameSoundCon IASIG AI Working Group presentation. The results below are
user-observed behavior from this rehearsal. They apply to the identified
authentic source project and do not establish general Studio Vision
compatibility.

## Environment

- The native Phoenix macOS application was built and launched on Kurt's Mac
  through Xcode **Product → Run**.
- The Xcode Cargo PATH fix had been committed and pushed as `f67f3d0`.
- Phoenix displayed **Phoenix Core connected** and **Application contract
  version 1**.
- The MIDI export workflow required neither SheepShaver nor Codex.

## Source and project inspection

The rehearsal used the authentic, untouched **newest STUFF baseline** project
from Experiment 007. Phoenix recognized it as a Studio Vision project and
reported:

- project status: **Partially supported**;
- 18 Sequences displayed;
- three Ready Sequences and 15 partially supported Sequences;
- 18 warnings in the project display; and
- 15 warnings in the expandable warning section.

The difference between the two displayed warning counts was observed but was
not diagnosed during this rehearsal.

## Successful native-app exports

All exports used the native Phoenix application and the destination
`~/Documents/Phoenix Demo Exports/`. Finder showed the three resulting MIDI
files together in that folder.

### Ode to Clarke

- Readiness: **Ready**.
- Project tracks displayed: 9.
- Sequence warnings displayed: 0.
- Exported file: `Ode to Clarke.mid`.
- The success panel reported 9 musical tracks and 10 SMF tracks.
- **Reveal in Finder** succeeded.

### Bells for her

- Readiness: **Ready**.
- Project tracks displayed: 14.
- Sequence warnings displayed: 0.
- Exported file: `Bells for her.mid`.
- The success panel reported 10 musical tracks and 11 SMF tracks.
- The success panel reported 395 untranslated metadata items.
- **Reveal in Finder** succeeded.

The 14 project tracks must not be described as 14 exported musical tracks.
The observed export result reported 10 musical tracks.

### Sequence K

- Readiness: **Ready**.
- Project tracks displayed: 2.
- `Sequence K.mid` appeared in the export folder.
- **Reveal in Finder** succeeded.

The export-result panel's track counts and warning details were not captured,
so this rehearsal establishes no values for them.

## Unsupported-Sequence safety check

Kurt selected **xForm** and observed:

- status: **Partially supported**;
- 13 tracks;
- one warning;
- the explanation, “Phoenix can inspect this sequence, but general MIDI
  routing is not established.”; and
- a disabled **Export MIDI** button.

This demonstrated conservative export gating for that Sequence in the tested
project.

## Limits of this rehearsal

- This was one authentic source project, not cross-project compatibility
  validation.
- The rehearsal established native-app export and Finder reveal. It did not
  evaluate musical playback quality in Logic.
- It established no new OS1 blind-validation eligibility.
- Steve Horowitz's Prologue remains inspection-only and not MIDI-exportable.
- No distributable, signed, or notarized application was validated.
- The results do not establish that every original project track or every
  untranslated metadata item was recovered as MIDI.

## Presentation observations for future work

These are observations and desired directions, not implemented changes:

- Expanded warnings dominate the initial window.
- The fixed-height Sequence list requires scrolling.
- Export success is functional but feels anticlimactic. Kurt would like a more
  upbeat Phoenix-themed completion experience after functionality is secure.
- Future completion reporting should distinguish exported Sequence counts,
  musical-track counts, SMF-track counts, untranslated metadata, and external
  audio references when each is supported by evidence.
