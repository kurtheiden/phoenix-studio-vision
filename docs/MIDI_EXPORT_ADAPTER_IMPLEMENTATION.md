# Objective

Implement Phase B of `MIDI_EXPORT_DESIGN.md`: pure adaptation of already
decoded musical values into MIDI-domain scheduled events, deterministic Note
Off generation, explicit export policy, reporting, and synthetic integration
with the Phase A SMF serializer.

# Scope

`src/midi_export.rs` sits strictly between decoded Phoenix event values and
`src/smf.rs`. It consumes decoded fields or narrow owned synthetic values and
produces `ScheduledEvent`, `TimeSignature`, Tempo MPQN, and report structures.
It performs no byte scanning, event-bound discovery, descriptor parsing,
channel inference, file I/O, CLI work, or SMF chunk serialization.

No authentic artifact or Ode channel manifest is read or embedded.

# Module and API

The module exposes:

- `DecodedExportEvent` and `DecodedExportEventKind` for Note, Controller,
  Channel Pressure, Pitch Bend, Patch, and explicit unsupported-family input;
- constructors borrowing established `BoundedNoteEvent`, `BoundedNoteBody`,
  `BoundedControllerRecord`, `ChannelPressureEntry`, `PitchBendEntry`, and
  `BoundedPatchRepresentation` fields without re-parsing them;
- `adapt_track` for transactional track conversion;
- `adapt_meter` / `adapt_meter_values` and `adapt_tempo` /
  `adapt_tempo_mpqn`;
- `adapt_text` and `adapt_conductor`;
- export policies, counts, warnings, untranslated-metadata entries, and typed
  errors.

The owned adapter event is deliberately narrow, not a new project or DAW
model. It carries absolute position, stable source ordinal, optional source
range, and only the musical fields required for export.

# Timing policy

`TimingPolicy::Identity480` explicitly states the only supported conversion:
decoded position units equal MIDI ticks and PPQN is 480. `Unsupported` returns
`UnsupportedTimingConversion`. No universal default or arbitrary scaling is
implemented.

# Channel assignment

Every track requires `Some(ChannelAssignment)` containing the Phase A
human-facing `MidiChannel`. Absence is `UnknownChannel`; there is no default.
The provenance enum permits `ParsedRouting`, `AuthenticatedOverride`, and
`Synthetic`. Phase B tests use only `Synthetic`.

# Note and Note Off scheduling

One decoded Note creates:

- Note On at its absolute position with pitch and attack velocity;
- explicit `8n` Note Off at checked `position + duration`, preserving release
  velocity.

Zero duration is valid. Phase A ordering puts the same-tick Note Off before
Note On. Notes are independent: overlapping equal pitches are neither paired
afterward nor truncated.

Stable ordinals are encoded without collision as:

```text
source MIDI event = source_ordinal * 2
generated Note Off = source_ordinal * 2 + 1
```

Multiplication/addition is checked. Duplicate source ordinals are rejected.
Thus simultaneous generated endings remain ordered by their originating source
events and cannot collide with source-event ordinals.

# Family mappings

- Controller maps number/value directly to Control Change on the assigned
  channel. Its opaque context is not emitted and is recorded as untranslated
  metadata.
- Channel Pressure maps its value directly to `Dn`.
- Pitch Bend maps stored LSB/MSB directly to `En`, with no signed or cents
  conversion.
- Every MIDI data field is validated through `MidiDataByte`; high bits are
  never masked.

# Patch policy

Patch translation must be classified explicitly upstream:

- `ProgramOnlyConfirmed` emits Program Change;
- `ConfirmedBankSelect { msb, lsb }` emits CC0, CC32, and Program Change at
  the Patch absolute position;
- `UnsupportedOpaque` returns `UnsupportedPatchTranslation`.

The only first-version policy is `StrictKnownOnly`. The adapter never inspects
opaque Patch bytes to derive a bank form or interpret a sentinel. Phase A
same-tick priorities order the bank messages before Program Change.

# Meter policy

`HistoricalWhenKnownOtherwiseStandard` maps:

- third payload 8 to `cc=24`;
- third payload 6 to `cc=12`;
- any other third payload to `cc=24` with a warning;
- a MIDI-valid fourth payload directly to `bb`;
- an out-of-range fourth payload to `bb=8` with a warning.

`KnownHistoricalOnly` rejects an unknown third payload with
`UnsupportedMeterMapping`. Numerator, denominator exponent, `cc`, and `bb` are
then validated into the Phase A `TimeSignature`; numerator zero remains an SMF
validation error.

# Tempo

Initial Tempo remains the unsigned MPQN value. The adapter accepts
`1..=0x00ff_ffff` and returns the validated value suitable for
`serialize_conductor_track`. Zero and larger values are wrapped Phase A
`InvalidTempo` errors. Mid-sequence Tempo maps are excluded.

# Text decision

Phase B preserves valid UTF-8 bytes unchanged and rejects interior NUL. The
repository has no MacRoman conversion dependency, and this task did not add
one casually. Invalid UTF-8 therefore returns a typed
`TextConversion::MacRomanDeferred` error. Full MacRoman-to-UTF-8 conversion is
isolated at `adapt_text` for a later explicit dependency/manual-table decision.

# Report

`ExportCounts` covers Notes, generated Note Offs, ordinary Controllers, CC0,
CC32, Program Changes, Channel Pressure, Pitch Bend, Tempo, and Meter.
`ExportTrackResult` includes the channel assignment, scheduled events, counts,
warnings, and untranslated metadata. `ConductorResult` contains name bytes,
validated Tempo, `TimeSignature`, PPQN 480, counts, and warnings.

`ExportReport` aggregates conductor/track counts, warnings, channel
assignments, and untranslated metadata for the later workflow without GUI
wording.

# Errors

`MidiExportError` covers unsupported timing, position/ordinal overflow,
duplicate source ordinal, unknown channel, contextual wrapped SMF validation,
unsupported Patch translation, unsupported Meter mapping, text conversion,
and unsupported event families. Event validation errors retain source ordinal
and optional source range. Filesystem errors are absent.

# Transactional behavior

`adapt_track` builds its result locally and returns it only after every event
has translated. Any unsupported family, invalid field, duplicate ordinal,
overflow, or unsafe Patch classification returns `Err` and no partial success.
Warnings occur only for the explicitly allowed Meter fallback policy.

# Synthetic tests

`tests/midi_export_adapter.rs` covers:

- exact Note On/Off positions, pitch, attack, and release velocity;
- zero-duration ordering, overflow, invalid data, equal-ending generated
  ordinals, and overlapping same-pitch Notes;
- Controller mapping on channels 1 and 16 plus invalid number/value;
- direct Pressure and Bend mapping plus invalid bytes;
- Program-only and confirmed-bank Patch mappings and unsupported Patch error;
- unknown channel/timing, duplicate ordinal, unsupported family, and
  transactional failure;
- historical 4/4 and 6/8 Meter mappings, fallback warnings, strict rejection,
  and invalid fourth-byte fallback;
- MPQN 500000/461538 plus zero/24-bit rejection;
- UTF-8 preservation, NUL rejection, and explicit MacRoman deferral.

# Serializer integration

A synthetic test adapts a conductor and one channel-16 Patch/Note track,
serializes both into an in-memory Format 1 SMF, and independently checks header,
track count, PPQN 480, exact chunk boundaries, conductor Tempo/Meter, Program,
Note On, generated release-velocity Note Off, exactly one final EOT per track,
and complete file consumption. No `.mid` file is written.

# Explicit exclusions

Phase B excludes authentic project reads, the Ode manifest, project/sequence/
track navigation, routing inference, Patch-byte classification, arbitrary
timing scaling, Note pairing/repair, full legacy text conversion, SMF chunk
serialization, CLI/file writing, DAW validation, and unsupported event
families.

# Deviations from MIDI_EXPORT_DESIGN.md

MacRoman conversion is deferred behind a typed error rather than implemented.
Adding a new encoding dependency was explicitly disallowed without review, and
valid UTF-8/ASCII behavior is complete. No other design deviation is present.

# Single recommended next step

Design Phase C's one-track authentic proof integration: validate all manifest
provenance keys, structurally locate only `Ode to Clarke` Track 3, flatten its
already decoded Patch/Note walk into this adapter, and keep file output outside
the proof until in-memory comparison passes.
