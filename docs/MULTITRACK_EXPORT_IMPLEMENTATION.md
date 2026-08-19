# Objective

Implement Phase D1 as pure, reusable, transactional assembly of one conductor
and an ordered collection of already-decoded musical tracks into one in-memory
SMF Format 1 plus an aggregate report.

# Scope

`src/multitrack_export.rs` orchestrates existing Phase B adaptation and Phase A
serialization. It contains no project parser, raw-byte logic, channel
inference, authenticated manifest, file I/O, CLI behavior, or target-specific
constant.

# Architecture

The module is a thin layer above `midi_export` and `smf`:

```text
decoded values + explicit policy
  -> conductor/track adaptation
  -> conductor/named-track serialization
  -> Format 1 assembly
  -> bytes + report
```

It does not change or duplicate the lower layers. The authenticated Phase D
integration can later validate a proof manifest, flatten exact walks, and then
supply this generic input.

# Public API

- `MultitrackSequenceInput` borrows sequence metadata, policy, and ordered
  tracks.
- `MusicalTrackInput` borrows a name, opaque diagnostic context, and decoded
  events while carrying explicit channel assignment and Patch policy.
- `assemble_multitrack_sequence` returns `MultitrackExportResult` or one
  context-preserving `MultitrackExportError`.
- `MultitrackExportResult` owns complete SMF bytes and a
  `MultitrackExportReport`.

# Sequence input

The sequence input contains raw name bytes, initial MPQN, four initial Meter
values, timing policy, Meter policy, and a borrowed slice of musical tracks.
The implemented supported timing remains explicit `Identity480`; unsupported
timing fails through the conductor adapter.

# Track input

Each track supplies raw name bytes, `MidiChannel` plus assignment provenance,
a borrowed ordered `DecodedExportEvent` slice, strict Patch policy, and an
opaque caller context string used only in reports/errors. It contains no
Studio Vision range type or identity semantics.

# Track ordering

The assembler never sorts tracks. Conductor is output Track 0; each musical
track follows at input index plus one. Names, channels, counts, and source
ordinals do not affect track position.

# Conductor assembly

The module calls `adapt_conductor`, then `serialize_conductor_track`. It does
not reimplement text, Tempo, Meter, PPQN, ordering, or EOT rules. Adaptation or
serialization failure aborts the sequence.

# Musical-track assembly

For each input track, the module adapts its name with the existing narrow text
policy, calls `adapt_track`, and passes the successful scheduled events to
`serialize_named_musical_track`. Channel assignment, Patch policy, Note Off
scheduling, MIDI value validation, event ordering, and EOT remain lower-layer
responsibilities.

# Empty tracks

An explicitly supplied empty musical track is valid and remains in order. It
contains Track Name at tick zero followed by the serializer's one final EOT,
and its musical counts are zero.

# Duplicate names/channels

Duplicate names and duplicate channels are accepted. Ordered opaque context
and input index distinguish reports/errors; neither name nor channel is
treated as a unique track key.

# Transactionality

No result type contains partial bytes. The function returns immediately on a
conductor or track failure. Format 1 assembly occurs only after every supplied
track has adapted and serialized successfully. A failed later track therefore
cannot yield a successful file containing earlier tracks, and unsupported
Patch translation fails the whole sequence.

# Aggregate report

Each successful track report owns context, adapted name, channel assignment,
adapter-derived counts, warnings, and untranslated metadata. Sequence totals
begin with the conductor's Tempo/Meter counts and use `ExportCounts::add_assign`
over successful adapter results. Aggregate warnings/metadata likewise derive
from those results, never an independent input recount.

The report also records the adapted sequence name, musical track count, total
SMF track count, and ordered per-track reports.

# Error model

`MultitrackExportError` distinguishes:

- conductor adaptation;
- musical-track adaptation with zero-based input index and opaque context;
- conductor serialization;
- musical-track serialization with index/context;
- final Format 1 assembly.

Existing `MidiExportError` and `SmfSerializeError` values are wrapped without
flattening. Filesystem errors are absent.

# Format 1 assembly

After complete lower-layer success, `serialize_format1` receives conductor
first and the musical chunks in exact caller order. The result therefore has
exactly `1 + musical_track_count` MTrk chunks and PPQN supplied by the validated
conductor result.

# Synthetic tests

`tests/multitrack_export.rs` covers:

- one musical track / two total chunks;
- three ordered musical tracks and all supported Phase B families;
- exact generated Note Off tick and release velocity;
- empty track preservation;
- duplicate names and duplicate channels;
- adapter-derived per-track and aggregate counts;
- a failing later track with index/context;
- unsupported Patch whole-sequence failure;
- conductor failure.

# Independent parse validation

A test-local parser independently validates MThd, Format 1, PPQN 480, declared
and observed MTrk counts, chunk lengths, MIDI VLQs, legal channel data, names,
conductor Tempo/Meter, caller order, channels, representative messages,
generated Note Off, exactly one final EOT per track, and exact EOF consumption.
It does not call serializer decoding helpers.

# Production/proof boundary

Sequence assembly, ordered reports, and contextual errors are reusable
production infrastructure. Project/reference hashes, structural manifests,
authenticated channels, exact ranges, target Patch classifications, comparison
inventories, and artifact paths remain future proof-integration inputs.

# Explicit exclusions

- authentic project/reference access;
- manifest validation or Studio Vision structure;
- channel derivation;
- file writing and CLI/UI;
- new message/meta families;
- sequence-duration padding;
- general legacy text conversion.

# Deviations from design

None. The opaque track context is a caller string rather than a Studio Vision
identity type, keeping D1 format-neutral while preserving error/report context.

# Implementation gate

**PASS.** Reusable ordered assembly, all supported family flow, empty and
duplicate tracks, Note Off preservation, report consistency, transactionality,
unsupported Patch failure, and independent full-file parsing all pass without
authentic coupling.

# Single recommended next step

Implement D2's authenticated Ode manifest validation and nine-track
walk/flatten integration as proof policy feeding this assembler.
