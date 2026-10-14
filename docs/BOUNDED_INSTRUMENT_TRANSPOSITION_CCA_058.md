# Bounded Instrument transposition — CCA-058

Source-derived pitch processing is now integrated with the established bounded 120 single-Instrument routing proof and its complete/experimental recovery builders. It is not a general decoder for all Instrument processing, transpose maps, layers or other project layouts.

## Evidence reconciliation

`AUTHENTIC_SWEEPER_MIDI_REFERENCE_STUDY_2026_10_12.md` records 2,294 Sweeper Notes selected through Instrument rows84/86, with payload+34=F4 and authentic output twelve semitones below raw source pitches. SCHOOL PROJECTS M-vegetarian del supplies a second project: its row85 has the same encoding and its317-note c-b13 dyna tom lo stream independently matches after −12. Additional held-out parts kick/snare and agogo/bongo contain225 and158 matching Note streams, using the same source rule.

Existing pitch Experiments009/010 establish raw Note pitch bytes; they do not establish Instrument processing. Existing routing Experiments033/034 establish assignment-mediated output changes while retaining musical data and Instrument definitions; transpose was kept fixed, so these are not controlled transpose edits. Existing bounded routing research supplies the guarded association/table/device chain. No completed experiment was rerun or new native experiment initiated. Nonzero F4 behavior is independently corroborated; encodings beyond00/F4 are not authorized as arbitrary signed semitones.

## Runtime rule and provenance

`instrument_transposition` accepts only a selected type-0x10 record with exactly36 payload bytes and an agreeing Instrument row. The existing same-source routing proof establishes structural selection, unique device, channel, bounds and context applicability. Payload+34 decodes00 to0 and F4 to−12. Unknown encodings refuse, rather than defaulting to zero. There is no channel10 rule or names/hashes/reference lookup.

Raw walker values and the complete recovery's raw handoff remain untouched. A cloned adaptation handoff receives effective Note pitch. Normal adaptation consequently uses the same effective key for Note On and its generated Note Off. The selected Instrument's association range, row, record range, encoding offset/value and semitone value are retained. Every changed Note separately records source ordinal/range, original pitch, exported pitch and applied semitones; experimental sidecars retain these fields. Zero-transpose tracks retain Instrument metadata and unchanged raw values.

Every original pitch must be0..127 and every effective pitch must remain0..127. The entire transformation is transactional; no clamping, wrapping, partial output or lost provenance is permitted. Unsupported selected encodings fail the routing proof; out-of-range changed Notes fail adaptation. Existing experimental refusal/whole-track ledger behavior remains authoritative. Missing changed-Note provenance also refuses.

No changes to Note timing/duration/velocity, Controller targets, FF60 interpretation, Patch/program/bank assignments, Pitch Bend or effects. Multi Patch tracks contain no Notes under their existing grammar and are untouched. The generic raw Note and Controller decoders are unchanged. Original/reference MIDI files are never read by this module or supplied to conversion; authentic comparisons exist only in tests.

## Validation design

Portable fixtures exercise unrelated source/part/device identities, a selected row moved to a newly inserted Instrument record on the same output channel, zero-transpose negative controls, unknown encodings, pitch12→0 and127→115, all underflow pitches0..11, raw pitch128 rejection, malformed record/row mismatch, missing provenance, transactional failure and unchanged non-Note events. Complete public in-memory recovery retains raw handoff pitch while emitting effective Note On/Off pairs.

Authentic SCHOOL validation resolves original semantic associations and bounds through production routines, decodes original source notes, applies the production rule and compares317+225+158 note onsets/pitches/attack/channels against native MIDI. References never provide converter output events. Further values or layouts require independent evidence before admission.

Sweeper public experimental export validation compares against existing CCA-057: exactly four chunks change and only2294 Note On/Off pitch pairs shift. All other chunks, including verified Pipe, remain identical; counts, raw-object accounting, channels, ordering, timing, velocities, programs, controllers and effects remain unchanged. Native reference independently validates all affected Note On events. Publication uses a fresh destination and no-overwrite behavior.
