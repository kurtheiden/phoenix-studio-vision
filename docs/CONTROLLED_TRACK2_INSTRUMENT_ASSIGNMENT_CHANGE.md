# Experiment 033 - Track 2 Instrument JV-1080-2 to JV-1080-3

Status: **PREREGISTERED; CONTROLLED ARTIFACTS NOT YET CREATED**

Preregistration date: 2026-09-23.

Experiment 033 is the next unused number in the reviewed repository and
external Controlled Save Experiments collection. This is a new experiment;
it neither resumes nor changes Experiment 032.

## Objective and evidence boundary

Test whether changing one track's assignment to an existing Instrument reveals
a serialized track-to-Instrument association. The sole intentional UI change is
`JV-1080-2 -> JV-1080-3`. This is NOT an independent MIDI-channel edit: choosing
another Instrument selects its existing routing and processing definition.
No exact project offset, numeric encoding, or pointer format is predicted.

## Source and target identities

- Source project, read-only:
  `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline`.
- Source data fork: 211,468 bytes; SHA-256
  `e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`;
  recorded Finder Type/Creator `MID2` / `MIDA`.
- Sequence: `Ode to Clarke`, baseline range `0x02ef6f..0x03202c`.
- Track: `Track 2`; descriptor ordinal 3, range `0x02f231..0x02f2d7`;
  pair ordinal 1, primary range `0x02fb42..0x0300df`;
  event range `0x02fb55..0x0300d8`. Ranges are half-open baseline anchors,
  not assumed unchanged offsets in later saves.
- Owner-confirmed baseline assignment: `JV-1080-2`.
- Owner-confirmed Instrument definitions: `JV-1080-2` uses Output Device
  `JV-1080`, channel 2; `JV-1080-3` uses Output Device `JV-1080`, channel 3.
  Neither shows the Drum Instrument indicator in the provided screenshot.
- Authenticated reference MIDI, read-only:
  `/Users/kurtheiden/Documents/Phoenix Research/Studio Vision MIDI Exports/Project 001/Ode to Clarke Multi All`.
  Size 12,141 bytes; SHA-256
  `4f63b34ef92204d4bc5eeb78dbbe7b94d005c1f9ceb57ea0f9809533ad590f29`.
  Format 1, PPQN 480, conductor plus nine musical tracks. Target is SMF track 2
  (conductor is track 0), named `Track 2`, channel 2.

The screenshot supports visible settings, not hidden Transpose Map contents
or a serialized reference. Drum Track, Drum Instrument, and Transpose Map are
distinct controls. Do not toggle any of them. Undefined/italic OMS assignments
must not be repaired or remapped as part of this experiment.

## Fixed variables

Keep all source events and values unchanged: 211 notes, pitches, start times,
durations, attack/release velocities, controllers, Patch name `Stereoww Bs`,
Patch position tick 0, bank CC0=81 and CC32=1, and Program value 37. Preserve
names, track order, tempo, meter, loops, quantize/shift settings, track drum
designation, mute/solo, playback and inclusion settings, and all other tracks.

Do not edit either Instrument definition: Output Device, channel, Drum
Instrument toggle, Transpose Map, layers/overflow, velocity scaling, transpose,
range, mute/solo, or any other setting. Keep OMS setup, application environment,
and export settings fixed. A displayed track color change inherited from the
selected Instrument is an expected consequence, not an additional manual edit.
Do not alter Patch events to accommodate the new Instrument.

## Required artifacts and naming

Retain the two existing before controls at their original paths. Place all new
artifacts together, using the established transfer workflow, under:

`/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 033 - Track 2 Instrument JV-1080-2 to JV-1080-3/`

| Role | Filename |
| --- | --- |
| No-edit saved project | `newest STUFF baseline EXP33 CONTROL` |
| No-edit complete sequence MIDI | `Ode to Clarke Multi All EXP33 CONTROL` |
| Assignment-edited project | `newest STUFF baseline EXP33` |
| Edited complete sequence MIDI | `Ode to Clarke Multi All EXP33` |

Preserve any extension Studio Vision creates. Never overwrite an existing
artifact; if these names are occupied, stop and resolve provenance first.
Preserve Finder/resource-fork metadata. Record each exact path, data-fork size
and SHA-256, Type/Creator and available resource-fork metadata, source lineage,
Studio Vision version/environment, save command, and export action/options.
Capture before/after/reopened target assignments and both Instrument settings.
Capture any warnings or remapping prompts without accepting a routing change.

## Owner execution protocol

1. Verify the source identity and create TWO fresh Finder duplicates directly
   from Experiment 007, named for CONTROL and edited project above. Neither
   may descend from another experiment or from the other duplicate.
2. Open CONTROL. Confirm `Ode to Clarke / Track 2 / JV-1080-2` and record the
   two Instrument definitions, including visible processing settings. Make no
   project edit. Save once using the established normal Save workflow, close,
   reopen, and verify the assignment remains `JV-1080-2`.
3. Export ONLY the complete `Ode to Clarke` sequence as one Format-1 multitrack
   MIDI file, including all nine musical tracks and conductor. Record options.
   Do not substitute an isolated-track or selected-track export.
4. Close CONTROL without further saves. Open the independent edited duplicate;
   confirm it still has the baseline target assignment. In the target track's
   Instrument popup, choose existing `JV-1080-3`. Make no other edit.
5. Verify that events/Patch and fixed settings have not changed. Save once by
   the same method, close, reopen, and verify `Track 2 -> JV-1080-3` with the
   existing Instrument definition still `JV-1080`, channel 3.
6. Export the complete Ode sequence with exactly the same multitrack options.
   Close without further saves. Transfer and retain all four new artifacts
   with provenance and screenshots. Do not normalize or repair the files.

If export requires changing track inclusion or another project setting to
obtain the complete sequence, stop rather than introducing a second variable.

## Locked predictions and classifications

### A. Assignment-correlated project delta

Predict a bounded change associated with Track 2, possibly in its descriptor,
event Instrument associations, a referenced record, or dependent metadata.
Do not assume assignment lives exclusively in a track descriptor.

- Supported candidate: structural comparison isolates an edit-specific field
  or linked set, distinct from the control save and plausibly bound to Track 2.
- Inconclusive: changes cannot be separated from save noise or bound safely.
- Not supported in the inspected data fork: retained assignment and MIDI
  response exist, but no distinct project-data candidate is found. This does
  not exclude resource-fork or environment storage.

One edited save yields candidates, not demonstrated repeatability. Independent
replication is required before claiming a stable encoding.

### B. Causal MIDI response with event stability

Predict target channel 2 -> 3 for all 425 channel messages: 2 Control Changes,
1 Program Change, 215 Note On and 207 Note Off messages. Logical status changes
are `B1 -> B2`, `C1 -> C2`, `91 -> 92`, and `81 -> 82`, including running status.
Predict unchanged families, data values, absolute ticks, and normalized 211
notes. Four velocity-zero Note On endings account for the difference between
note and explicit Note Off counts; compare like representations and preserve
the established project-versus-SMF release-velocity caveat.

All other musical tracks are negative controls: their channels, event data,
timing, counts, and inclusion must remain unchanged. Conductor tempo/meter,
PPQN, track names/order, and complete export scope must remain equivalent.
Instrument-name metadata may reflect the selected Instrument; report it
separately from musical events rather than demanding byte-identical MIDI.

- Confirmed for this intervention: reopened assignment is retained, target
  messages use channel 3, musical semantics are unchanged, and negative
  controls pass.
- Refuted clean-response prediction: retained assignment yields another
  channel, changed musical events, or changed negative controls.
- Inconclusive: incomplete export, unverified reopened state, baseline/control
  mismatch, remapping, or insufficient alignment prevents causal comparison.

Neither confirmation nor failure alone establishes the general SMF-export
effect of Drum Instrument flags, Transpose Maps, or undefined OMS devices.

## Read-only analysis and serialization controls

Verify identities before analysis. Compare baseline-to-CONTROL,
baseline-to-edited, and CONTROL-to-edited, retaining raw differences before
assigning semantics. The control export must first match the authenticated
baseline's musical content, channel inventory, and inclusion. Otherwise stop
causal interpretation and record the environmental/export discrepancy.

Structurally align the sequence, all descriptors/pairs, target Patch, and
complete note chain `0x02fb75..0x0300d8`, accounting for relocation and changed
container lengths. Compare decoded values as well as aligned bytes. Consult
prior ordinary-save controls and Track 2 bank experiments; do not blindly
subtract all offsets that changed in one control. If noise remains ambiguous,
require another independently recorded control/replication before elevating
a candidate. Do not run that follow-up under an unrecorded protocol change.

## Evidence required to establish an Instrument reference

A small delta, a value equal to 2 or 3, a matching name, or the expected MIDI
channel response alone is insufficient. Establish all of the following before
calling a candidate a decoded track-to-Instrument reference:

1. Bind the candidate structurally to the target track/event assignment.
2. Independently bound and identify the relevant Instrument definitions; show
   that the baseline and edited values resolve to `JV-1080-2` and `JV-1080-3`
   under the SAME explicit reference rule, with validated scope and bounds.
3. Distinguish a reference from a direct channel, ordinal coincidence,
   checksum, save-generated identifier, or dependent serialization field.
4. Verify Instrument definitions themselves stayed fixed and explain any
   dependent rewrites without using names or numeric suffixes as sole proof.
5. Obtain an independently preregistered replication or additional assignment
   mapping that tests the proposed rule and unaffected associations.

If Instrument definitions cannot be located, report an assignment-correlated
candidate only. Future experiments may separate Instrument identity from
channel; this experiment intentionally does not do so.

## Abort and preservation rules

Before the edited save, abort if source identity/assignment differs, the track
is Multi, either required Instrument is unavailable, expected definitions or
drum states differ, a remap/OMS repair is required, assignment causes a Patch
replacement/event conversion, or any additional setting must be edited.
Abort the clean-isolation attempt if known processing differences would alter
the performance; do not edit the definitions to compensate. Unknown hidden
processing is a limit tested by the MIDI comparison, not proof of equivalence.

After save, if assignment fails to persist or musical output differs beyond
predictions, preserve all results and classify the failure/inconclusiveness.
Do not repair, overwrite, discard, or silently substitute artifacts.

## Success boundary and references

Success is a bounded assignment candidate, or stronger evidence connecting
that association to an Instrument definition, together with classified MIDI
and control results. It does not establish independent channel storage,
general routing, OMS port decoding, track inclusion/playback fields, or new
export readiness. No parser, routing rule, compatibility profile, or readiness
change follows automatically.

- [Controlled-save methodology](CONTROLLED_SAVE_EXPERIMENTS.md)
- [Ode channel and structural correlation](ODE_TO_CLARKE_CHANNEL_CORRELATION.md)
- [Track 2 bank and note-chain evidence](CONTROLLED_TRACK2_BANK_LSB_CHANGE.md)
- [Reference MIDI inventory](FIRST_MIDI_RECOVERY_SPIKE.md)
- [OMS evidence and Experiment 032 abort](OMS_STUDIO_SETUP_FORENSICS.md)
- Local `Studio Vision Pro 4.5 ƒ/Documentation/MIDI Reference Manual.pdf`,
  printed pp. 207-208 and 294-298: track assignment, Drum Track, Drum
  Instrument, Transpose Map, Instrument line settings, and Output semantics.

## Post-experiment observations and results - 2026-09-23

This section records evidence obtained after execution. The preregistration,
including its original status line, predictions, and gates above, is preserved
as written; its pre-execution status is historical, not the current outcome.
The owner reports that both projects were saved and reopened successfully and
both MIDI exports were created. No additional experiment was performed for
the mute-state observation below.

### Returned artifact identities

The verified regular files are in:

`/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 033 - Track 2 Instrument JV-1080-2 to JV-1080-3/Returned from MacOS9/`

| Returned filename | Bytes | SHA-256 of data fork |
| --- | ---: | --- |
| `EXP33 CTRL` | 211,468 | `b1c82a7085f98f054f7320e2800aba523447c3c987c1aa983865518b7a7dd368` |
| `EXP33 CTRL MID` | 10,514 | `9979ed6d5fc58edb85c3c03e5e43b4c7015a353af6075ece8ba94ac49cbf5059` |
| `EXP33 EDIT` | 211,468 | `91eba17f0e23c39ee480e7b2c2ae585d2b23198bf69cf10b5f5fc9b91bd701d7` |
| `EXP33 EDIT MID` | 10,514 | `717fb6064aa423c333d8d411d0a4d9d1aee13578d9d4f2b71f3bc27e24ad5a79` |

These returned names differ from the proposed names above. The same-named
project files in the parent Experiment 033 folder are original Finder-side
duplicates, not the returned saved results. Returned projects expose Finder
Type/Creator `MID2` / `MIDA`; MIDI exports expose `Midi` / `MIDA`. No returned
file exposes a `com.apple.ResourceFork` attribute.

### Saved mute states and observed export setting

**OBSERVED (owner-supplied screenshots):** the saved `EXP33 CTRL` project,
Sequence `Ode to Clarke`, displays yellow `M` indicators for exactly `Track 1`
and `Track 3`. Their displayed Instrument assignments are `Juno-106` and
`JV-1080-1`, respectively. The other seven musical rows show no track Mute
indicator. These are track-level observations, not an inspection of Instrument
mute/solo settings or a decoded native mute field.

The supplied screenshot with the Export as MIDI File dialog shows
`Multitrack` checked. The owner reports inspecting without changing or saving
the project, then canceling the dialog without changing options or exporting.
This establishes the setting at inspection, not a retrospective record of
every option or state when the earlier files were exported.

The two screenshot attachments are identified by their visible content: the
dialog-over-Tracks view and the unobscured Tracks view. Neither was copied,
renamed, or modified as part of this documentation update.

**ESTABLISHED CORRELATION:** the two muted tracks in the saved CONTROL UI are
exactly the two tracks absent from both returned MIDI files relative to the
authenticated nine-musical-track `Ode to Clarke Multi All` reference.

**PLAUSIBLE EXPLANATION, NOT A CAUSALLY PROVEN RULE:** export may exclude muted
tracks. No independent mute-only intervention or authenticated before/after
export under such an intervention is available here. The correlation does not
establish a general Studio Vision export rule, the EDIT project's displayed
mute state, Instrument-level muting, or the cause of the historical omission.

The historical nine-track `Multi All` procedure and its inclusion settings
remain unestablished. Its filename is not evidence of an export option. The
MIDI Reference Manual, printed pp. 402-403, documents the Multitrack checkbox
as selecting Type 1 output and preserving separate tracks, while Export to
Clipboard uses the current selection. Its illustrated file-export dialog
does not show a `Multi All` option. The reviewed export section does not
explicitly establish muted-track inclusion behavior.

### MIDI comparison

**OBSERVED:** the authenticated reference and both returned exports are
Format 1, PPQN 480, with valid chunk framing. The reference has a conductor
and nine musical tracks; each returned file has a conductor and seven.
All retained CONTROL track payloads are byte-identical to their reference
counterparts. `EXP33 CTRL MID` is also byte-identical to the historical
`Ode to Clarke Multitrack` export, demonstrating that the same omission
pattern predates Experiment 033 without establishing its cause.

CONTROL-to-EDIT changes every one of Track 2's 425 channel messages from
channel 2 to channel 3: two Control Changes, one Program Change, 215 Note On,
and 207 Note Off messages. After excluding only the channel nibble, all target
events, absolute ticks, data bytes, families, and counts match. This preserves
the 211 normalized notes, their timing/durations and velocities, and the
bank/Program values CC0=81, CC32=1, PC37 at tick 0. There are 400 changed
physical status bytes; running status accounts for the larger logical count.
The conductor and the other six included musical tracks are byte-identical
between CONTROL and EDIT.

### Structural comparison and unresolved candidate

**OBSERVED:** Experiment 007, returned CONTROL, and returned EDIT retain the
same 527 root-record boundaries, types, and lengths. Ode's sequence range,
11 descriptors, and primary/secondary pairs align without relocation.

| Comparison | Changed bytes | Contiguous differing runs |
| --- | ---: | ---: |
| Experiment 007 to CONTROL | 1,237 | 425 |
| Experiment 007 to EDIT | 1,771 | 641 |
| CONTROL to EDIT | 1,262 | 600 |

The target Patch framing `0x02fb55..0x02fb75`, including established Patch
name/bank/Program fields, and complete 211-note chain `0x02fb75..0x0300d8`
are byte-identical across all three projects. All nine Ode musical event
ranges and their secondary records are also byte-identical, including those
for the two tracks absent from the returned MIDI.

A localized candidate at `0x02f221` is `05` in Experiment 007 and CONTROL,
and `06` in EDIT. It is also `05` in inspected prior Experiments 002, 003,
006, 008, 028, and 031. Under the existing descriptor boundaries, this byte
is at `Track 1` descriptor-relative `+0x96`, before the Track 2 descriptor.
Its ownership is unresolved: proximity and change correlation do not bind
it to Track 2 or prove an Instrument reference.

EDIT also changes relative `+0x06` in all 168 primary records and all 27
prelude records, together with terminal-record rewrites. Comparable patterns
occur in earlier experiments. They are not target-specific channel evidence;
the single no-edit control does not fully isolate these serialization effects.
No candidate has been resolved to independently identified Instrument
definitions under one validated reference rule. Repeatability, reference
encoding, direct versus dependent storage, and general routing remain unknown.

### Preregistered classifications retained

- **Prediction A: INCONCLUSIVE.** A localized candidate exists, but target
  ownership, meaning, and repeatability are not established.
- **Prediction B: INCONCLUSIVE under the original full-export gate.** The
  observed target channel response and retained-track stability match the
  prediction, but CONTROL lacks two tracks required by the authenticated
  baseline inclusion gate. The later mute-state observation supplies a
  plausible explanation; it does not retrospectively waive that gate.
- **Target musical-data stability: ESTABLISHED for the compared data.** The
  target project Patch/note bytes and channel-normalized MIDI events agree.

This remains an Instrument-assignment intervention, not independent channel
editing. No parser, profile, readiness, export behavior, or Experiment 032
status is changed by these observations.

## Structural findings - 2026-09-23

Classification: **SUPPORTED ASSIGNMENT-CORRELATED CANDIDATE, NOT A DECODED
INSTRUMENT REFERENCE.** This subsequent read-only investigation strengthens
the candidate assessment without rewriting the preregistration or the earlier
results. The original full-export gate remains unsatisfied.

### Candidate location and semantic ownership

The authenticated Experiment 007, returned CONTROL, and returned EDIT project
identities were reverified. At `0x02f221`, their values are respectively
`05`, `05`, and `06`, with unchanged root-record framing.

The containing type-`0x01` root record is `0x02ef6f..0x02f742`. The current
Track 1 descriptor slice is `0x02f18b..0x02f231`, placing the candidate at
slice-relative `+0x96`. The Track 2 slice begins at `0x02f231`, and its name
begins at `0x02f240`: the candidate is exactly 31 bytes before that name.
These are measured structural locations. A byte falling inside a named
descriptor slice does not prove semantic ownership by that track, nor does
this evidence alone authorize shifting the parser's descriptor boundaries.

The preceding byte is `00`, so the observation is also compatible with a
two-byte value `0005 -> 0006`. Field width and endianness are not established
by observing only the changed byte.

### Cross-track correspondence and proposed lookup

At the same name-minus-31 position, the nine Ode musical tracks yield:

| Track | Experiment 007 / CONTROL | EDIT | Displayed Instrument correspondence |
| --- | ---: | ---: | --- |
| Track 1 | 3 | 3 | Juno-106 |
| Track 2 | 5 | 6 | JV-1080-2 -> JV-1080-3 |
| sys100loops | 61 | 61 | S-760-10 |
| Track 4 | 61 | 61 | S-760-10 |
| Track 5 | 61 | 61 | S-760-10 |
| Track 3 | 4 | 4 | JV-1080-1 |
| Track 6 | 61 | 61 | S-760-10 |
| Track 3 #2 | 1 | 1 | JD-800 |
| Track 7 | 13 | 13 | JV-1080-10 |

The displayed assignments come from the supplied UI evidence and owner
confirmation; the numeric values are independently measured project bytes.
All unaffected entries retain their values across the three project files.

There are 85 bounded type-`0x10` records, all byte-identical across baseline,
CONTROL, and EDIT. In every record, payload byte `+25` equals its zero-based
ordinal. The proposed lookup selects the following records for Track 2:

| Selected ordinal | Record range | Payload bytes `+25`, `+26`, `+27` |
| ---: | --- | --- |
| 5 | `0x0002f1..0x00031a` | `05 0c 01` |
| 6 | `0x00031a..0x000343` | `06 0c 02` |

An independently framed name-bearing type-`0x2a` record contains the name
`JV-1080` and candidate identifier `0c` at payload byte `+33`. Under the
proposed interpretation, type-`0x10` payload `+26` selects that device and
payload `+27` is a zero-based channel. The two rows then correspond to
JV-1080 channels 2 and 3.

Applying this same proposed rule to all nine Ode tracks matches all nine
authenticated baseline channels and the displayed device contexts. In EDIT,
it predicts only Track 2's channel change. The other selected ordinals yield
Juno-106/channel 1 (3), JV-1080/channel 1 (4), S-760/channel 10 (61),
JD-800/channel 15 (1), and JV-1080/channel 10 (13). All 85 candidate device
values resolve to the name-bearing records under this rule; all 85 candidate
channel bytes lie within `0..15`.

These observations support the lookup hypothesis, but the proposed device
and channel field meanings are not independently controlled. Literal strings
`JV-1080-2` and `JV-1080-3` were not found in the baseline data fork. The
records are structurally bounded; their identities as those particular
Instrument definitions are NOT independently proven. Neither name agreement,
UI order, nor matching channels alone establishes those identities.

### Controls, alternatives, and serialization noise

The candidate remains `05` in the inspected, aligned saved variants from
Experiments 001-031. Experiment 003's recorded Instrument-assignment edit
changes the corresponding name-minus-31 position before `Track 3 #2`:
`0x02f605` is `01` in the baseline and `02` in Experiment 003. Its precise
before/after UI assignment provenance is insufficient for independent semantic
validation. It corroborates this field family without proving the rule.

A direct one-based or zero-based MIDI-channel interpretation of the candidate
is contradicted: Track 2 uses value 5 for channel 2, while value 61 accompanies
channel 10. A general Instrument-reference interpretation fits better, but
ordinal versus stored identifier is unresolved because the candidate table's
ordinal and payload identifier values coincide. A stable dependent field or
other assignment-related encoding is not yet excluded.

The stable controls, unchanged neighboring candidate values, and cross-track
lookup fit weigh against ordinary save-generated metadata. They do not prove
repeatability or eliminate all dependent serialization explanations.

The project-wide primary/prelude `+0x06` changes and terminal-record rewrites
remain separate observations, with comparable patterns in earlier saves.
They must not be conflated with the localized `0x02f221` candidate or promoted
to channel fields. No new reference claim follows from subtracting those
rewrites as presumed noise.

### Future option only - cross-device replication

A possible separately preregistered follow-up is a fresh-baseline change of
`Ode to Clarke / Track 2` from existing `JV-1080-2` to existing `Juno-106`,
after verifying that its settings permit an interpretable comparison. The
lookup hypothesis predicts `0x02f221: 05 -> 03`, unchanged candidate Instrument
records, and output channel 2 -> 1. A cross-device, nonadjacent assignment is
more discriminating than another adjacent JV-1080 channel assignment.

This is a future research option, NOT an authorized or executed experiment.
It requires its own preregistration, fresh duplicate/no-edit controls, fixed
event and environment conditions, and abort conditions for Patch/event
changes or required Instrument/OMS edits. Even a matching result would not
alone settle field width, ordinal versus identifier, independently establish
Instrument-record identities, or justify general routing support.
