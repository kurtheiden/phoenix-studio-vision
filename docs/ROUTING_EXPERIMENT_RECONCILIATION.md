# Routing Experiment Reconciliation

This document reconciles the original routing-related records without
repeating their full protocols or artifact analyses. It distinguishes selecting
an existing Instrument from editing a fixed Instrument definition or channel.

## Common source

The controlled-save series uses the authenticated Experiment 007 `newest STUFF`
project (211,468 bytes,
SHA-256 `e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`).
The authenticated Ode reference is the 12,141-byte `Ode to Clarke Multi All`
MIDI export (SHA-256
`4f63b34ef92204d4bc5eeb78dbbe7b94d005c1f9ceb57ea0f9809533ad590f29`).

## Chronological inventory

### Experiment 003 — Change One Instrument Assignment

The original record says only that one track Instrument assignment changed. It
does not identify the target track, replacement Instrument, an independent
channel edit, or a MIDI export. The available result is the project
`Experiment 003 - Change one instrument assignment/newest STUFF changed instrument`
(SHA-256
`b8aa42825f6d478bef873a0e1319c667791bf622e0023e06e5ace37e8700e8dd`).

The saved project differs broadly because Studio Vision rewrites the file on
save. Output-specific observations include:

| Range | Baseline → result |
| --- | --- |
| `0x0000e64d` | `b2 → be` |
| `0x000161ff` | `b2 → be` |
| `0x0001e8df` | `31 → 32` |
| `0x0002f325` | `67 → be` |
| `0x0002f605` | `01 → 02` |
| `0x0002f766` | `02 → 01` |
| `0x0002f7cf` | `02 → 01` |
| `0x00032018..0x00032019` | `00 92 → ff ff` |

Later structural alignment places `0x0002f605` at the name-minus-31
candidate before `Ode to Clarke / Track 3 #2`. That is an assignment-correlated
observation, not proof of a channel or Instrument-reference field. No MIDI
result exists, so Experiment 003 cannot establish an exported channel.

The owner's recollection of changing a channel through an Instrument/channel
popup is compatible with this experiment if an Instrument selection carried a
different channel. The surviving record does not establish that a fixed
Instrument's channel was edited independently.

### Experiment 032 — Proposed Independent Channel Edit

The preregistered action was `Ode to Clarke / Track 3 #2 / JD-800`, changing
MIDI channel `15 → 14` while keeping the Instrument definition and output
device fixed. Studio Vision did not provide the required independently editable
control in the available session, so the abort condition triggered before save.

No CONTROL, EDIT, project, or MIDI artifact exists. Its byte and MIDI
predictions were never tested. This does not contradict Experiment 003; the
experiments specify different interventions.

### Experiment 033 — JV-1080-2 to JV-1080-3 assignment

The owner changed only `Ode to Clarke / Track 2`'s existing Instrument
assignment from `JV-1080-2` (JV-1080 channel 2) to `JV-1080-3` (JV-1080 channel
3). The returned artifacts are in the `Returned from MacOS9` subdirectory of
the Experiment 033 folder:

| Artifact | SHA-256 |
| --- | --- |
| `EXP33 CTRL` | `b1c82a7085f98f054f7320e2800aba523447c3c987c1aa983865518b7a7dd368` |
| `EXP33 CTRL MID` | `9979ed6d5fc58edb85c3c03e5e43b4c7015a353af6075ece8ba94ac49cbf5059` |
| `EXP33 EDIT` | `91eba17f0e23c39ee480e7b2c2ae585d2b23198bf69cf10b5f5fc9b91bd701d7` |
| `EXP33 EDIT MID` | `717fb6064aa423c333d8d411d0a4d9d1aee13578d9d4f2b71f3bc27e24ad5a79` |

The Track 2 candidate at `0x0002f221` changed `05 → 06`; all type-0x10
records remained byte-identical. The proposed selected records are ordinal 5,
`0x0002f1..0x00031a`, payload `05 0c 01`, and ordinal 6,
`0x00031a..0x000343`, payload `06 0c 02`. The type-0x2a JV-1080 record and
its name/identifier remained unchanged.

All 425 Track 2 MIDI channel messages changed from channel 2 to channel 3
with musical data unchanged. Both returned MIDI exports contained the
conductor plus seven musical tracks; Track 1 and Track 3 were absent. The
original nine-musical-track export gate was not met. Their observed Track 1 /
Track 3 mute correlation remains plausible but unproven as an export rule.

### Experiment 034 — JV-1080-2 to Juno-106 assignment

The owner changed only the same Track 2 Instrument assignment from `JV-1080-2`
to the existing `Juno-106` (displayed channel 1). The four returned artifacts
are in the Experiment 034 `Returned from MacOS9` subdirectory:

| Artifact | SHA-256 |
| --- | --- |
| `EXP34 CTRL` | `0f811a95b782fc5af3339a18d107f373d337913bdceb7cdf2109878b68603288` |
| `EXP34 CTRL MID` | `9979ed6d5fc58edb85c3c03e5e43b4c7015a353af6075ece8ba94ac49cbf5059` |
| `EXP34 EDIT` | `6c87431d619b3934d8deb365c6e7cc8273e9a980f1422307fdfab5ae944091c0` |
| `EXP34 EDIT MID` | `dbeb7c5f592d052c2c76da69f32fea4699a61b6d1befed0e412d51d1088ffbf7` |

The same candidate changed `05 → 03`. The proposed selected type-0x10 record
is ordinal 3, `0x00029f..0x0002c8`, payload `03 0b 00`; the proposed Juno-106
type-0x2a record is `0x0000e9..0x000116`, identifier `0b`. The records remained
unchanged between CONTROL and EDIT.

All 425 Track 2 MIDI channel messages changed from channel 2 to channel 1,
with notes, timing, Patch, bank, Program, and counts unchanged. The same
seven-musical-track limitation applied; the nine-track gate was not met.

## Evidence boundary

**Proven:** selecting another existing Instrument can change a track's emitted
MIDI channel while preserving its musical events. Experiments 033 and 034
demonstrate this for same-device and cross-device assignments. The Phase 1
collector can safely expose bounded assignment candidates and type-0x10 /
type-0x2a observations without making them authoritative.

**Provisional:** the name-minus-31 candidate, type-0x10 ordinal/device/channel
fields, and type-0x2a names/identifiers form a consistent lookup hypothesis.
The candidate is not proven to be a serialized Instrument reference, and its
field width and ordinal-versus-identifier semantics remain unresolved.

**Unresolved:** no artifact proves an independently editable fixed-Instrument
channel control; no serialized chain is established from track assignment to
Instrument identity to device/channel; and mute state versus MIDI export scope
is not causally resolved.

Do not repeat Experiments 033 or 034 merely to demonstrate assignment-mediated
channel changes. That response is already supported by both returned artifact
pairs.

## Evidence required for an authoritative resolver

Before promoting the provisional collector, Phoenix needs an independently
bound chain of:

1. track assignment to a serialized Instrument identity;
2. that identity to a bounded Instrument/device record;
3. the record to an independently observed output channel; and
4. a replication in which ordinal, identifier, and Instrument-identity
   interpretations no longer coincide.

The evidence must preserve fixed musical events and output-device settings,
include no-edit controls, and classify track inclusion separately. Existing
Experiments 003, 033, and 034 provide regression fixtures and assignment/MIDI
correlation, but cannot supply this missing independent binding. No new
experiment should be authorized until a specific independently editable UI
control or another source of identity evidence is documented.
