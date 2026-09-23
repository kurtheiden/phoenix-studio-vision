# Experiment 034 - Track 2 Instrument JV-1080-2 to Juno-106

Status: **PREREGISTERED; CONTROLLED ARTIFACTS NOT YET CREATED**

Preregistration date: 2026-09-23.

Experiment 034 is the next unused number in the reviewed controlled-save
collection. It is a new cross-device Instrument-assignment experiment. It does
not resume or change Experiment 032 or Experiment 033, and it is not an
independent MIDI-channel edit.

## Objective and evidence boundary

Change exactly one track's assignment to an existing Instrument to test the
Experiment 033 assignment-candidate lookup across device families. The sole
intentional UI change is:

`JV-1080-2 -> Juno-106`

Selecting another Instrument selects that Instrument's existing output,
channel, and processing definition. This experiment does not edit a channel
field independently and does not predict a universal Studio Vision routing
grammar.

## Source and target identities

- Source project, read-only:
  `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline`.
- Source data fork: 211,468 bytes; SHA-256
  `e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`;
  recorded Finder Type/Creator `MID2` / `MIDA`.
- Sequence: `Ode to Clarke`; baseline range `0x02ef6f..0x03202c`.
- Target: `Track 2`; descriptor ordinal 3, range `0x02f231..0x02f2d7`;
  pair ordinal 1, primary range `0x02fb42..0x0300df`;
  exact event range `0x02fb55..0x0300d8`. These are half-open baseline
  anchors and must be structurally revalidated after each save.
- Owner-confirmed original assignment: `JV-1080-2`; its existing displayed
  Output Device is `JV-1080`, channel 2.
- Proposed existing Instrument: `Juno-106`. Its displayed Output Device,
  channel, Drum Instrument state, Transpose Map, transpose, velocity scaling,
  range, mute/solo, and any other processing settings must be recorded before
  execution. The proposed output channel is expected to be 1 only if the
  owner verifies that setting before the edit.
- Authenticated reference MIDI, read-only:
  `/Users/kurtheiden/Documents/Phoenix Research/Studio Vision MIDI Exports/Project 001/Ode to Clarke Multi All`.
  Size 12,141 bytes; SHA-256
  `4f63b34ef92204d4bc5eeb78dbbe7b94d005c1f9ceb57ea0f9809533ad590f29`.
  Format 1, PPQN 480, conductor plus nine musical tracks; reference Track 2
  uses channel 2.

The Experiment 033 candidate is a hypothesis, not a decoded reference. The
candidate at `0x02f221` is predicted to be an assignment value, not a direct
MIDI channel. Existing type-`0x10` and type-`0x2a` records must be inspected
read-only and treated as opaque evidence during execution.

## Fixed variables and preflight

Before making any edit, record the source identity, Finder Type/Creator,
available resource-fork metadata, Studio Vision version/environment, and the
visible state of both Instruments. Confirm that Track 2 is assigned solely to
`JV-1080-2`, not `Multi`.

Do not edit or create either Instrument definition. Keep the Juno-106 Output
Device, channel, Drum Instrument toggle, Transpose Map, layers/overflow,
velocity scaling, transpose, range, mute/solo, patch-name setup, and every
other setting fixed. Keep OMS setup and application environment fixed.

Keep Track 2's notes, pitches, start times, durations, attack/release
velocities, controllers, Patch name `Stereoww Bs`, Patch position tick 0,
bank CC0=81 and CC32=1, Program 37, track name, length, loop, quantize/shift,
mute/solo, and playback state fixed. Keep all other tracks, assignments,
events, names, tempo, meter, and ordering fixed.

Experiment 033 observed yellow track-level Mute indicators on `Track 1` and
`Track 3` in the saved CONTROL project. Those two tracks were absent from
both Experiment 033 MIDI exports. This experiment must record the same mute
indicators and must not toggle them. It must not silently change the original
full-export gate or assume that the Multitrack checkbox includes muted tracks.
The historical nine-track `Multi All` procedure remains unestablished.

## Fresh lineage, controls, and artifact names

Create TWO independent Finder duplicates directly from Experiment 007. One is
the no-edit CONTROL and one is the EDIT project. Neither may descend from
Experiment 033, another experiment, or the other duplicate.

Use this external directory and retain all artifacts together:

`/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 034 - Track 2 Instrument JV-1080-2 to Juno-106/`

| Role | Filename |
| --- | --- |
| No-edit saved project | `EXP34 CTRL` |
| No-edit MIDI export | `EXP34 CTRL MID` |
| Assignment-edited project | `EXP34 EDIT` |
| Assignment-edited MIDI export | `EXP34 EDIT MID` |

Preserve any extension Studio Vision creates. Record exact paths, data-fork
sizes, SHA-256 values, Finder Type/Creator and resource-fork metadata, source
lineage, Studio Vision version, save action, export action, export options, and
before/after/reopened Instrument and mute-state observations. Never overwrite
an existing artifact.

## Owner execution protocol

1. Verify the Experiment 007 path, size, SHA-256, and Finder metadata. Create
   the two fresh duplicates and document their initial identities.
2. Open CONTROL. Confirm `Ode to Clarke / Track 2 / JV-1080-2`; record the
   Juno-106 definition and its displayed output channel. Record Track 1 and
   Track 3 mute indicators and all visible export-dialog options. Do not change
   any option. Save once using the established normal Save workflow, close,
   reopen, and verify the original assignment and mute states.
3. Export the complete selected `Ode to Clarke` sequence as Format-1 MIDI with
   Multitrack checked and otherwise unchanged options. Do not alter mute
   states or select tracks to force inclusion. If Track 1 or Track 3 is absent,
   record that fact; do not call the export a complete nine-track control.
4. Close CONTROL without further saves. Open the independent EDIT duplicate
   and confirm the original assignment. In the Track 2 Instrument popup choose
   the existing `Juno-106`; make no other edit.
5. Verify that the Instrument definition, OMS setup, Track 1/Track 3 mute
   indicators, events, Patch, and fixed settings remain unchanged. Save once,
   close, reopen, and verify `Track 2 -> Juno-106` and the saved mute states.
6. Export the same selected sequence with exactly the same options. Retain the
   export even if it omits muted tracks, but classify inclusion against the
   authenticated nine-track reference rather than silently waiving the gate.
7. Transfer and retain all four artifacts with provenance. Do not repair,
   normalize, rename, overwrite, or discard any result.

## Preregistered predictions

### A. Assignment candidate

The byte at project offset `0x02f221` will change from `05` in Experiment 007
and CONTROL to `03` in EDIT. This is a locked prediction for the candidate
lookup, not a claim that the byte is a one-byte field or that it semantically
belongs to the current Track 2 descriptor slice.

The existing candidate type-`0x10` and type-`0x2a` records will remain
byte-identical between baseline, CONTROL, and EDIT. If the proposed lookup is
correct, the value `03` selects the existing Juno-106-associated record while
the device/channel fields remain unchanged. No exact field width or ordinal /
identifier interpretation is predicted.

### B. MIDI response and musical stability

If the selected Juno-106 definition displays channel 1 and the assignment is
retained after reopening, all 425 Track 2 channel messages are predicted to
move from channel 2 to channel 1: two Control Changes, one Program Change,
215 Note On, and 207 Note Off messages. Logical status changes are `B1 -> B0`,
`C1 -> C0`, `91 -> 90`, and `81 -> 80`, including running status.

Track 2's 211 normalized notes, pitches, absolute timing, durations, attack
and release velocities, controllers, Patch, bank values, Program 37, event
families, and counts are predicted unchanged apart from channel status.
All unaffected tracks retain their assignments, channels, musical events,
timing, counts, and inclusion state. The conductor, PPQN, track names/order,
and export options remain equivalent.

If the owner verifies a Juno-106 channel other than 1 before execution, the
MIDI prediction must be classified **not applicable** and the experiment must
not proceed under this preregistration.

## Structural comparison and serialization-noise controls

Verify all identities before interpretation. Compare baseline-to-CONTROL,
baseline-to-EDIT, and CONTROL-to-EDIT at whole-file, record, descriptor,
candidate, primary/prelude, terminal, Patch, and event-range levels. Retain
raw changed runs before assigning semantics. Confirm root framing, sequence
range, descriptor count, pair count, labels, target ranges, and relocation.

The no-edit CONTROL is mandatory. Use it to identify ordinary Save As and
serialization rewrites. Compare the localized candidate separately from the
widespread primary/prelude `+0x06` and terminal changes already observed in
Experiment 033 and earlier saves. Do not subtract broad differences as noise
without a structural comparison. If control and authenticated reference MIDI
do not agree on retained tracks, channel inventory, Patch data, timing, and
inclusion, preserve the discrepancy and do not claim the full-export gate
passed.

## Outcome classifications

**SUCCESS** requires all of the following:

- reopened EDIT retains Track 2 -> Juno-106 and the Instrument definition is
  unchanged;
- candidate `0x02f221` is `05` in baseline/CONTROL and `03` in EDIT;
- candidate type-`0x10` and type-`0x2a` records are unchanged;
- target MIDI moves channel 2 -> 1 with musical data unchanged;
- unaffected tracks pass structural and MIDI negative controls; and
- inclusion is reported explicitly, with the original nine-track full-export
  gate passed only if all nine authenticated musical tracks are present.

**FAILURE** means the saved/reopened assignment is Juno-106 but the candidate
does not change as predicted, candidate records change, MIDI remains on
channel 2 or moves to an unexpected channel, or target/unaffected musical data
changes beyond channel status.

**INCONCLUSIVE** means the assignment cannot be verified after reopen, Juno-106
settings or OMS identity are ambiguous, structural alignment is insufficient,
serialization noise cannot be separated, export is incomplete, or the
full-export gate cannot be evaluated safely.

**ABORT BEFORE SAVE** if selecting Juno-106 requires editing or creating its
Instrument definition, changing OMS or output-device configuration, changing
the Juno-106 channel, converting Patch or musical events, modifying another
track, changing Track 1/Track 3 mute states, or changing any export setting.
Abort if the target is Multi, the source identity differs, an existing Juno-106
is unavailable, or any remapping prompt requires acceptance.

After save, preserve unexpected results and classify them; do not repair,
overwrite, discard, or silently substitute artifacts.

## Evidence boundary and future use

This experiment tests an Instrument-assignment intervention across device
families. It does not independently edit MIDI channel, prove a universal
Instrument table, prove field width or identifier semantics, establish the
historical Multi All export procedure, or authorize a routing decoder. A
candidate becomes a decoded Instrument reference only if it is structurally
bound to the target, resolves under one validated rule to independently
identified Juno-106 and JV-1080-2 definitions, remains stable in controls, and
survives independent replication across unaffected tracks.

Do not use a positive result to change `observed_channel`, compatibility
profiles, readiness, or export policy under this preregistration. Any later
implementation requires a separate review.

## References

- [Experiment 033 findings and returned-artifact analysis](CONTROLLED_TRACK2_INSTRUMENT_ASSIGNMENT_CHANGE.md)
- [Controlled-save methodology](CONTROLLED_SAVE_EXPERIMENTS.md)
- [Controlled-save comparison summary](CONTROLLED_SAVE_EXPERIMENTS_SUMMARY.md)
- [Track 2 bank control](CONTROLLED_TRACK2_BANK_LSB_CHANGE.md)
- [Ode channel and structural correlation](ODE_TO_CLARKE_CHANNEL_CORRELATION.md)
- [Reference MIDI inventory](FIRST_MIDI_RECOVERY_SPIKE.md)
- [OMS routing evidence and Experiment 032 abort](OMS_STUDIO_SETUP_FORENSICS.md)

## Post-experiment results — 2026-09-23

The owner returned four artifacts in the following directory:

`/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 034 - Track 2 Instrument JV-1080-2 to Juno-106/Returned from MacOS9/`

All four are regular files. Available Finder metadata is shown below; no
resource fork was present for any returned file.

| Artifact | Exact path | Size | SHA-256 | Finder Type/Creator |
| --- | --- | ---: | --- | --- |
| EXP34 CTRL | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 034 - Track 2 Instrument JV-1080-2 to Juno-106/Returned from MacOS9/EXP34 CTRL` | 211,468 | `0f811a95b782fc5af3339a18d107f373d337913bdceb7cdf2109878b68603288` | `MID2` / `MIDA` |
| EXP34 CTRL MID | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 034 - Track 2 Instrument JV-1080-2 to Juno-106/Returned from MacOS9/EXP34 CTRL MID` | 10,514 | `9979ed6d5fc58edb85c3c03e5e43b4c7015a353af6075ece8ba94ac49cbf5059` | `Midi` / `MIDA` |
| EXP34 EDIT | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 034 - Track 2 Instrument JV-1080-2 to Juno-106/Returned from MacOS9/EXP34 EDIT` | 211,468 | `6c87431d619b3934d8deb365c6e7cc8273e9a980f1422307fdfab5ae944091c0` | `MID2` / `MIDA` |
| EXP34 EDIT MID | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 034 - Track 2 Instrument JV-1080-2 to Juno-106/Returned from MacOS9/EXP34 EDIT MID` | 10,515 | `dbeb7c5f592d052c2c76da69f32fea4699a61b6d1befed0e412d51d1088ffbf7` | `Midi` / `MIDA` |

The authenticated Experiment 007 source remains the 211,468-byte project with
SHA-256 `e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`;
the authenticated reference MIDI remains the 12,141-byte file with SHA-256
`4f63b34ef92204d4bc5eeb78dbbe7b94d005c1f9ceb57ea0f9809533ad590f29`.

### Verified structural and MIDI results

- The candidate at `0x02f221` is `05` in the Experiment 007 baseline, `05` in
  CONTROL, and `03` in EDIT.
- All 85 type-`0x10` records are byte-identical across baseline, CONTROL, and
  EDIT.
- The 12 type-`0x2a` records are identical between CONTROL and EDIT, but both
  saved projects differ from baseline in shared save-generated fields. The
  literal preregistered prediction of byte identity across all three projects
  is therefore **CONTRADICTED**; this is not an EDIT-specific type-`0x2a`
  change.
- Track 2's Patch range and complete 211-note chain are byte-identical across
  all three projects. All nine documented Ode musical event ranges are
  unchanged.
- All 425 Track 2 MIDI channel messages move from channel 2 to channel 1.
  The two Control Changes, one Program Change, 215 Note On messages, and 207
  Note Off messages retain their musical data, timing, counts, bank values,
  and Program 37.
- Both returned MIDI files are Format 1 with the conductor plus seven musical
  tracks. Track 1 and Track 3 are absent from both. The original nine-musical-
  track full-export gate was **NOT MET**. The previously observed Track 1 and
  Track 3 mute-state correlation remains a plausible explanation, not a proven
  export rule.

The localized `0x02f221` change and the channel response are separated from
widespread save-generated primary/prelude and terminal rewrites. The no-edit
CONTROL shows that those broader rewrites are not evidence of the Instrument
intervention itself.

### Prediction outcomes and experiment classification

| Preregistered prediction | Outcome |
| --- | --- |
| Candidate `05 -> 03` | **SUPPORTED** |
| Type-`0x10` records unchanged | **SUPPORTED** |
| Type-`0x2a` records byte-identical across all three projects | **CONTRADICTED** as written; shared CONTROL/EDIT save noise observed |
| Track 2 channel `2 -> 1` across all 425 messages | **SUPPORTED** |
| Target notes, timing, velocities, Patch, bank, Program, and counts unchanged | **SUPPORTED** |
| Unaffected retained tracks unchanged | **SUPPORTED** |
| All nine musical tracks included | **CONTRADICTED**; seven musical tracks were exported |

Overall preregistered **SUCCESS: NOT MET**. The full experiment is
**INCONCLUSIVE under its original gates** because the type-`0x2a` literal
identity prediction failed and the nine-track export gate was not met. The
localized cross-device Instrument-assignment/channel response is
**STRONGLY SUPPORTED**.

### Evidence boundary after Experiments 033 and 034

Together, Experiments 033 and 034 support an assignment-correlated project
candidate that changes when Track 2 is assigned to another existing Instrument,
and they support the resulting MIDI channel response while preserving Track 2
musical content. They do not establish a general Instrument-reference decoder,
field width, ordinal-versus-identifier semantics, or the historical nine-track
Multi All export procedure. They do not authorize Experiment 035 or any parser,
profile, readiness, or export-policy change.
