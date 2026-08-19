# Objective

Design the shortest non-throwaway path from one already validated Studio Vision
sequence to a playable Standard MIDI File (SMF). The exporter consumes Phoenix
container, initial Meter/Tempo, and mixed-event results. It does not inspect raw
project bytes, discover event bounds, reinterpret opaque parser fields, or aim
for byte identity with Studio Vision's export.

Success has three separate meanings: structural SMF validity, musical
equivalence for supported events, and practical DAW interoperability.
Historical Studio Vision organization is comparison evidence only.

# Scope

The first version supports the established 166-byte sequence profile and the
currently decoded initial Tempo, initial Meter, Note, ordinary Controller,
Channel Pressure, Pitch Bend, and safely translatable Patch fields. It supports
one sequence per SMF, no mid-sequence Tempo/Meter maps, SysEx, Poly Pressure,
unknown mixed-event branches, or guessed routing.

# Architectural boundary

The four responsibilities remain strict:

1. The Studio Vision parser validates root/sequence/track structure and returns
   bounded decoded representations with provenance.
2. A small export adapter selects one `SequenceContainer`, decodes its known
   ranges, walks each exact event range, applies explicit channel/text/meter/
   Patch policy, and constructs format-neutral export values.
3. The pure SMF serializer accepts only MIDI-domain values and emits chunks,
   delta VLQs, channel messages, and supported meta events. It never receives
   or reads SVP bytes.
4. The CLI/file layer selects paths, reads the project, invokes parsing and
   conversion, writes bytes, and owns I/O errors and user messages.

The adapter is the only new seam required. It must not create a broad project
model or copy provenance-heavy parser types into a second parser hierarchy.

# Initial SMF format

Use **SMF Format 1**. One Studio Vision sequence can contain multiple named
tracks, independent MIDI channels, and sequence-level Tempo/Meter. Format 1
naturally preserves those tracks and permits a conventional conductor track.
It also matches Studio Vision's authenticated multitrack exports. Format 0
would merge tracks, make names ambiguous, and require an avoidable structural
loss even though it can preserve channel messages musically.

# Export input model

The minimum owned/borrowed adapter result is conceptually:

```text
MidiExportSequence
  name: MidiText
  ticks_per_quarter: u16
  initial_tempo_mpqn: u32
  initial_meter: MidiTimeSignature
  tracks: Vec<MidiExportTrack>

MidiExportTrack
  source_identity: descriptor/pair ordinals and ranges
  name: Option<MidiText>
  channel: MidiChannel
  events: Vec<DecodedExportEvent>
```

`DecodedExportEvent` holds absolute Studio Vision position, source order, and
one already decoded Note, Controller, Pressure, Bend, or translatable Patch.
The adapter should borrow values where practical. Generated Note Offs and final
scheduled MIDI events are owned. Channel and lossy-text behavior enter as an
explicit `MidiExportPolicy`, never as parser conclusions.

# Timing division and scaling

The authenticated evidence establishes a four-beat coordinate with 480 Studio
Vision position units per beat/quarter, decoded positions equal to correlated
SMF absolute ticks, and authenticated exports declaring division 480. Patch
absolute positions, Note/Controller/Pressure/Bend accumulated positions, Note
durations, and export ticks agree directly across the correlated populations.

- A. Studio Vision internal position units per quarter understood: **YES for
  the authenticated 166-profile corpus** (480).
- B. SMF PPQN used by authenticated exports understood: **YES** (480).
- C. Lossless integer mapping from decoded positions to SMF ticks: **YES for
  this supported profile**, identity mapping.

The first exporter therefore writes division 480 and performs no scaling.
This is a profile fact, not a universal Studio Vision constant. A future
profile must supply an independently established rational conversion or fail
with `UnsupportedTimingConversion`; it must not default to a common PPQN.

# Absolute positions and delta times

Conversion is performed after all source and generated events have been merged
and totally ordered. Starting with `previous_tick = 0`, each event delta is
`event.tick - previous_tick` using checked arithmetic, then `previous_tick` is
updated. The first event at zero and simultaneous events therefore encode delta
zero. Decreasing positions are an error.

SMF delta time is limited to the four-byte MIDI VLQ maximum `0x0fff_ffff`.
A larger individual delta is `MidiVlqOverflow`; it is not split by invented
events. Source position/duration addition uses checked `u32` arithmetic and
fails on overflow. Stable source ordinal and generated-event ordinal complete
the ordering; no map iteration order is observable.

# Note scheduling

For every Phoenix Note at `position`:

- emit Note On `9n pitch attack_velocity` at `position`;
- checked-add `position + duration` and schedule Note Off
  `8n pitch release_velocity` at that tick.

The first policy uses explicit `8n` Note Off because Phoenix has a distinct
release velocity and this encoding preserves it. Velocity-zero Note On would
discard that value and make genuine zero-attack Note Ons harder to distinguish.
Pitch, attack, and release must be in `0..=127`; duration may be zero, in which
case ordering still places the Note Off before its Note On at the same tick.
Overlapping equal-pitch notes are emitted as represented; the exporter does not
pair, truncate, or otherwise repair them.

# Controller

An ordinary `BoundedControllerRecord` supplies controller number and value but
not channel. After the track adapter has obtained a proven or explicitly
provided channel, emit `Bn number value` unchanged. Both data bytes must be
`0..=127`; opaque three-byte context is retained in provenance/reporting and
has no SMF emission.

# Channel Pressure

`ChannelPressureEntry.pressure_value` maps directly to `Dn value` on the
track's resolved channel. Authenticated values are already MIDI-domain bytes.
Any value above 127 is rejected during export even though the structural parser
deliberately preserves arbitrary bytes.

# Pitch Bend

`PitchBendEntry.pitch_lsb` and `pitch_msb` are already the exact MIDI LSB/MSB
data-byte order, with `raw = lsb + (msb << 7)`. Emit `En lsb msb` unchanged
after validating both bytes are at most 127. No cent, signed, or range scaling
is applied, preserving the established 14-bit value exactly.

# Patch / Program Change

`BoundedPatchRepresentation.program_change` maps directly to `Cn program`
after `0..=127` validation. Patch position is its decoded absolute position.

Bank selection is narrower. Controlled evidence identifies direct CC0 and
CC32 bytes for the established `ff <cc0> <cc32>` tail, and authentic exports
show those messages before Program Change. However optional/sentinel semantics
remain partial. The first exporter may emit Bank Select only when the adapter
recognizes that exact independently confirmed three-byte form; it emits CC0,
then CC32, then Program Change. `ff ff ff` and every other tail are reported as
untranslated Patch metadata, never guessed as banks.

Device/instrument name and opaque Patch contexts have no channel-message
equivalent. They remain in the export report. Under strict first-version
policy, an untranslatable Patch field produces `UnsupportedPatchTranslation`
rather than silent loss. A future explicit metadata-loss policy may warn and
continue, but is not the proof path.

# Tempo

The initial decoded unsigned 24-bit MPQN is emitted at tick zero as
`ff 51 03 tt tt tt`. In Format 1 it belongs only in conductor Track 0. MPQN zero
is structurally representable by the parser but invalid for this musical export
policy and is rejected. No mid-sequence Tempo events are designed here.

# Meter

Emit initial Time Signature at tick zero in conductor Track 0 as
`ff 58 04 nn dd cc bb`. Numerator and denominator exponent map directly.

The narrowest musically faithful and standards-valid policy is an explicit
choice: `HistoricalWhenKnownOtherwiseStandard`. It maps source third payload
`08 -> cc 18` and `06 -> cc 0c`; otherwise it uses standards-valid `cc = 24`
(24 MIDI clocks per metronome click) and records a warning that historical
Studio Vision click reconstruction is unknown. It uses the established fourth
payload directly as `bb` when MIDI-valid; otherwise it uses `bb = 8` (32nd
notes per MIDI quarter) with a warning. This preserves `nn/dd`, hence musical
meter, without falsely claiming historical byte identity. Strict historical
comparison can request `KnownHistoricalOnly` and fail on an unmapped value.

# Track and sequence names

Sequence and descriptor names are raw legacy bytes. Parsing remains byte
preserving. The first deterministic SMF text policy is:

1. Preserve valid UTF-8 bytes unchanged.
2. Otherwise decode each byte using MacRoman to Unicode, then encode UTF-8,
   recording that conversion in the report.
3. Interior NUL or an unavailable/failed conversion is `TextConversion`.

Blank track labels omit Track Name rather than inventing one. Nonblank labels
become a zero-tick Track Name (`ff 03`). The conductor Track Name is the
sequence name; musical tracks use their descriptor labels. The filename is a
CLI concern and is not the sole sequence-name representation.

# MIDI channel source

Decoded performance events contain no MIDI channel, and source discriminators
`90`, `d0`, and `e0` have a zero low nibble that is proven not to be the export
channel. Ordinary Controller context is also opaque. Current classification:

- event-local channel: **UNKNOWN**;
- parser-exposed descriptor/routing channel: **UNKNOWN**;
- channel from authenticated export correlation for identified tracks:
  **PROVEN for those tracks**, but not derivable by the current parser;
- explicit caller-supplied mapping keyed by validated sequence plus descriptor
  and pair identity: **DERIVABLE export policy**, provided its provenance is
  checked and absence is an error.

The architecture therefore requires a `ChannelAssignment` per included track,
with provenance `ParsedRouting` (future) or `AuthenticatedOverride` (proof
target). It rejects missing/ambiguous assignments and never assumes channel 1.
`ODE_TO_CLARKE_CHANNEL_CORRELATION.md` independently establishes a complete
nine-track, single-channel manifest keyed by project hash plus sequence,
descriptor, pair, primary, and event ranges. This permits the first proof
without weakening the future parsed-routing seam. It is proof-target policy,
not Studio Vision format knowledge.

# Track inclusion

Include only ordinally established descriptor/pair bindings whose exact event
range validates and walks completely. Omit structurally empty event ranges;
record them as omitted empty tracks. Do not manufacture a track for an
unassociated descriptor or pair. A descriptor/pair mismatch such as Sequence I
is an export error unless the caller selects only independently established
bindings.

A nonempty track containing an unsupported family/branch fails the whole
sequence export. A track containing only unsupported material is likewise a
failure, not an empty MIDI track. Blank-name, otherwise valid event-bearing
tracks are included without a Track Name event.

# Conductor track

Track 0 contains, all at tick zero in this order: sequence Track Name, Set
Tempo, Time Signature, then End of Track. It contains no channel events,
invented markers, instrument metadata, or DAW-specific events.

# Event ordering

The total key is `(absolute_tick, priority, source_ordinal, generated_ordinal)`.
At an identical tick, priority is:

1. conductor/name meta events (only within their own track);
2. Bank Select MSB (CC0);
3. Bank Select LSB (CC32);
4. Program Change;
5. other Control Change;
6. Pitch Bend;
7. Channel Pressure;
8. Note Off;
9. Note On;
10. End of Track.

Patch setup precedes articulation and Note Off precedes Note On, preventing a
same-pitch retrigger from being immediately ended. Other same-tick controls
precede Note On so the new note observes the represented state. Within a class,
source order is stable. This is deterministic export policy; it does not claim
Studio Vision historically used the same byte order.

# End of Track

Each track ends with `ff 2f 00`. For a musical track its EOT is emitted at the
latest emitted event tick, including generated Note Offs, with delta zero after
that event. The conductor EOT is tick zero. The first version does not extend
tracks to an opaque project tail or Studio Vision sequence duration. This is
the minimum standards-valid policy and avoids inventing duration semantics.

# Serializer

Implement the small serializer directly in Phoenix. `Cargo.toml` currently has
one focused dependency (`sha2`), and the required SMF subset is compact enough
to validate exhaustively without adding a dependency. The serializer supports:

- `MThd`, fixed six-byte header, Format 1, checked `u16` track count, PPQN 480;
- `MTrk` with checked `u32` payload length and big-endian integers;
- four-byte-maximum MIDI VLQ delta times;
- Note On/Off, Control Change, Program Change, Channel Pressure, Pitch Bend;
- Track Name, Set Tempo, Time Signature, End of Track.

Always write explicit status bytes initially. Running status is unnecessary
and complicates validation. Serialization writes to `Vec<u8>` or an abstract
byte sink and has no filesystem API. An external crate would be acceptable
later only if it supports exact Format 1/chunk control, explicit Note Off
release velocity, checked VLQs and lengths, deterministic ordering, no lossy
normalization, Rust 1.70, and a compatible license.

# Unsupported-data policy

The first implementation fails the whole selected sequence on any unsupported
nonempty structure, event family, missing channel, or unsafe Patch translation.
It may omit only a proven structurally empty track, with a report entry. There
is no silent event or track omission. A later opt-in recovery mode may skip an
entire track with warnings, but skipping individual events is not acceptable
because it can change musical state and timing semantics.

# Export report

On success return bytes plus a report containing sequence name/identity, SMF
format/division, tracks written and empty tracks omitted, counts of Notes,
generated Note Offs, Controllers, Bank Selects, Program Changes, Pressure,
Pitch Bend, Tempo, and Meter, applied channel assignments and their provenance,
text conversions, Patch metadata retained but not emitted, and warnings.

On failure return the typed error plus a partial diagnostic report of examined
structures, never partial MIDI bytes presented as success. A future project
workflow can aggregate successful reports into “X sequences were saved as .MID
files” without involving the serializer in GUI design.

# Error model

Pure conversion/serialization errors include:

- `UnsupportedTimingConversion`;
- `PositionOverflow` / `NonMonotonicPosition`;
- `UnknownChannel` / `AmbiguousChannel`;
- `InvalidMidiDataValue` and `InvalidTempo`;
- `TrackCountOverflow` / `SmfLengthOverflow`;
- `MidiVlqOverflow`;
- `UnsupportedPatchTranslation`;
- `UnsupportedMeterMapping` for strict historical policy;
- `TextConversion`;
- `UnsupportedTrackAssociation` / `UnsupportedEvent`.

Each error carries sequence/track identity, source provenance where available,
and the offending value. Parser errors remain parser errors wrapped with
context. `MidiWriteError` (create/write/flush/rename) belongs solely to the CLI
or file-writing layer and wraps `std::io::Error` separately.

# Authentic validation strategy

Validation has three levels:

1. **Syntax:** independently parse the generated file; verify header format,
   division, track count/chunk lengths, legal VLQs/data bytes, and EOTs.
2. **Musical equivalence:** normalize Phoenix and authenticated Studio Vision
   exports to absolute events, ignoring track-byte organization and running
   status. Compare supported track/name/channel, Note starts/durations/pitches/
   attack/release velocities, ordinary Controllers, proven banks/programs,
   Pressure, Bend, Tempo, and Meter. Explicitly report Studio Vision-only or
   unsupported metadata; never require byte identity.
3. **Practical:** open in a modern DAW and manually verify conductor metadata,
   track structure/names/channels, note lengths/velocities, playback, and lack
   of import warnings.

# First authentic target

Choose **`Ode to Clarke`** from the authenticated Experiment 007 project and
compare against `Ode to Clarke Multi All` (Format 1). Its project provenance,
sequence/container, multiple Patch/Note tracks, exact event streams, 480-tick
timing, Tempo/Meter, names, programs/banks, and several track channels are
independently correlated. It exercises the desired multitrack architecture
with fewer families and less unsupported surface than `Bells for her`.

The first proof should export only every `Ode to Clarke` descriptor/pair whose
complete walk, channel, and Patch policy are independently established; strict
mode must fail rather than quietly omit any other nonempty pair. If complete
sequence coverage cannot meet that rule, begin Phase C with independently
established Track 3 and do not call it an end-to-end sequence success.

`Bells for her` is the second target: it is the stronger Controller, Pressure,
and Bend acceptance fixture, but its needed Track 9/14 channels currently come
from export correlation rather than decoded routing, so it does not remove the
shared channel blocker.

# Implementation decomposition

1. **A — pure SMF primitives (IMPLEMENTED):** chunks, explicit-status messages, meta events,
   VLQs, lengths, total ordering, and independent-parser synthetic tests.
2. **B — decoded adapter and scheduler (IMPLEMENTED):** identity 480 mapping, Note Off
   generation, all direct event-family mappings, policy/report/errors, using
   synthetic decoded values.
3. **C — one authentic `Ode to Clarke` Track 3 proof (COMPLETE):** structural navigation,
   exact walk, authenticated channel override, Program/bank policy, independent
   normalized comparison, and DAW smoke test.
4. **D — strict `Ode to Clarke` multitrack Format 1 (COMPLETE):**
   reusable assembly, authenticated nine-track manifest/walk integration, and
   independent complete normalized comparison, artifact write/re-open, and
   user-observed Logic Pro 12 validation are complete.
5. **E — `Bells for her` family coverage:** Controller, Pressure, Bend, and
   authenticated multitrack comparison.
6. **F — CLI/file writing:** only after the pure conversion succeeds; atomic
   destination behavior and aggregate reporting are separate.

This ordering gets authentic sound in Phase C while preserving the same types
and serializer used by the final multitrack export.

# Implementation gate

| Area | Gate | Reason |
|---|---|---|
| A. SMF container serialization | YES — IMPLEMENTED | pure Format 1 subset and independent synthetic parser validation |
| B. Note export | YES — IMPLEMENTED ADAPTER | explicit release-velocity Note Off scheduling |
| C. Controller export | YES — IMPLEMENTED ADAPTER | direct mapping with required explicit channel |
| D. Channel Pressure export | YES — IMPLEMENTED ADAPTER | direct mapping with required explicit channel |
| E. Pitch Bend export | YES — IMPLEMENTED ADAPTER | exact LSB/MSB with required explicit channel |
| F. Patch / Program Change | PARTIAL — IMPLEMENTED SAFE SUBSET | confirmed program/bank classifications only; opaque forms fail |
| G. Tempo export | YES | initial MPQN maps directly |
| H. Meter export | YES | musical fields plus explicit safe policy |
| I. MIDI channel assignment | PARTIAL | proven overrides exist; parser derivation absent |
| J. timing / PPQN mapping | YES | authenticated identity mapping at 480 |
| K. first authentic end-to-end export | YES for the `Ode to Clarke` proof target | complete provenance-locked channel manifest exists; general export remains partial |

The prior channel blocker for the first proof is resolved by the complete
authenticated manifest. Reliable parser-derived channel assignment remains a
blocker for arbitrary sequences and production-general export.

# Unknowns

- The Studio Vision descriptor/routing field that determines MIDI channel.
- Complete channel assignments for arbitrary sequences and tracks.
- Patch no-bank/sentinel optionality outside exact confirmed forms.
- Universal historical Meter `cc` conversion and nonstandard `bb` behavior.
- Legacy text encoding declaration rather than deterministic MacRoman policy.
- Mid-sequence Tempo/Meter maps and sequence-end duration semantics.
- Unsupported families, especially SysEx and Poly Pressure.
- Applicability of 480-unit identity mapping outside the supported profile.

# Single recommended next step

Preserve the completed bounded Ode proof; choose the next export scope in a
separate evidence-bounded task.
