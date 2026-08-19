# Objective

Implement Phase D2's authenticated, provenance-locked Ode to Clarke
nine-track integration as proof policy feeding the reusable D1 assembler.

# Scope

The implementation is focused integration coverage in
'tests/ode_multitrack_integration.rs'. It reads the established external source
fixture when present, validates the complete locked manifest, derives and walks
all nine exact event regions, flattens only Note/Patch events, applies four
authenticated Patch classifications, and assembles one ten-track Format 1 file
in memory.

It does not compare every event to the reference MIDI, write a proof file, add
CLI behavior, infer channels, or change production parsers/adapters/serializers.

# Provenance

The integration requires exactly 211,468 source bytes and SHA-256
'e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132'
before parsing. The source is then parsed from byte zero with
'parse_project_166'. The reference MIDI is not opened in D2.

# Manifest architecture

Test-local generic-shaped 'SequenceManifest', 'TrackRow', and
'PatchExpectation' values separate validation inputs from the concrete
'ode_manifest()' constant. The validation/assembly function consumes a
manifest argument, enabling in-memory mutation tests. All concrete hashes,
ranges, names, channels, counts, and Patch expectations remain authenticated
test policy.

No concrete value is added to 'sequence_container', 'mixed_event',
'midi_export', 'multitrack_export', or 'smf'.

# Sequence validation

After size/hash validation, D2 selects exactly one structurally parsed sequence
whose derived name bytes equal Ode to Clarke. It then requires exact sequence
range '0x02ef6f..0x03202c', name range '0x02f753..0x02f760', and resolved
ordinal descriptor/pair associations.

It decodes initial Tempo/Meter through the existing bounded decoders and
requires 500,000 MPQN plus source Meter '(4,2,8,8)'.

# Nine-row validation

The validator requires exactly nine track descriptors, pairs, bindings, and
manifest rows. Descriptor/pair ordinals must be unique and descriptor rows
strictly ordered. Every row validates descriptor ordinal/range/label, ordinal
binding, pair ordinal, primary range, valid human MIDI channel, and exact event
range.

Event end is derived, not used as navigation: 'primary.payload.end - 7' after
validating the established seven-byte tail shape. The derived range must equal
the locked row. Missing, extra, duplicate, reordered, or mismatched rows fail
before a successful result can exist.

# Exact walks

Each validated derived range is passed to 'walk_bounded_mixed_events' with
origin timing state zero. D2 requires 'consumed_range' equality and exact
logical inventory. Any family other than Note or the coupled Patch-to-Note
form is an error; nothing is skipped.

# Walk inventories

| Track | Logical events | Notes | Patches |
|---|---:|---:|---:|
| Track 1 | 92 | 91 | 1 |
| Track 2 | 212 | 211 | 1 |
| sys100loops | 322 | 322 | 0 |
| Track 4 | 179 | 179 | 0 |
| Track 5 | 134 | 134 | 0 |
| Track 3 | 85 | 84 | 1 |
| Track 6 | 60 | 60 | 0 |
| Track 3 #2 | 85 | 84 | 1 |
| Track 7 | 143 | 143 | 0 |
| **Total** | **1,312** | **1,308** | **4** |

# Patch classifications

Before supplying a Phase B classification, D2 verifies Patch tick, decoded
name, complete post-name context, and direct Program value:

- Track 1: tick 0, Empty Patch, Program 61, ProgramOnlyConfirmed;
- Track 2: tick 0, Stereoww Bs, Program 37, confirmed bank 81/1;
- Track 3: tick 480, Wavox, Program 29, confirmed bank 81/2;
- Track 3 #2: tick 530, Ming Dynasty, Program 23, ProgramOnlyConfirmed.

The test does not derive classifications from the reference MIDI or interpret
opaque bytes as general Patch semantics.

# Flattening

Each direct Note becomes one 'DecodedExportEvent' through the existing
constructor. Each coupled transition becomes one Patch event followed by its
first Note. Monotonic per-track source ordinals start at zero and remain unique;
existing constructors preserve positions, properties, and source ranges.

# D1 input construction

Nine 'MusicalTrackInput' values are created in manifest order with descriptor
label bytes, AuthenticatedOverride channels, flattened event slices,
StrictKnownOnly, and structural descriptor/pair context. The conductor uses
the parsed name and decoded Tempo/Meter with Identity480 and
HistoricalWhenKnownOtherwiseStandard.

'assemble_multitrack_sequence' is called exactly once after all rows validate
and flatten.

# Aggregate result

D1 succeeds with:

- 9 musical / 10 total tracks;
- 1,308 Notes and 1,308 generated Note Offs;
- 0 ordinary Controllers;
- 2 Bank Select MSB and 2 Bank Select LSB;
- 4 Program Changes;
- 0 Channel Pressure and 0 Pitch Bend;
- 1 Tempo and 1 Meter;
- no warnings.

Each per-track report retains the locked order, label, authenticated channel
provenance, and Note count.

# In-memory SMF structure

A test-local parser validates MThd length, Format 1, PPQN 480, declared ten
tracks, ten exact MTrk chunks, exact EOF, one final EOT per track, conductor
name/no channel messages, all nine musical names in manifest order, and channel
sets '1,2,10,10,10,1,10,15,10'.

This D2 structural validation now feeds D3's complete comparison in the same
focused test, avoiding a second manifest/walk/assembly implementation.

# Transactionality

The integration returns a Result and exposes no partial successful MIDI. One
mutation test independently verifies rejection of wrong project hash, wrong
sequence range, wrong descriptor range, wrong pair ordinal, wrong event range,
missing row, extra row, and mismatched Patch expectation. The authentic file
is never modified.

# Tests

The focused authentic test skips only when the established external source is
absent. When present, it executes the complete D2 gate. The reference MIDI is
not needed or read, and normal tests do not write any MIDI artifact.

# Production/proof boundary

All target-specific policy remains in integration coverage. Production modules
continue to provide generic structural parsing, exact walking, decoded
adaptation, transactional multitrack assembly, and SMF serialization without
Ode constants or authenticated routing claims.

# Explicit exclusions

- permanent multitrack proof writing/re-open;
- Logic Pro validation;
- general channel derivation;
- arbitrary sequences/projects;
- new event families or parser behavior;
- CLI/file-writing workflow.

# D2 gate

**PASS.** Source provenance, structural sequence location, all nine manifest
rows/ranges/walks, exact inventories, four Patch classifications, flattening,
D1 assembly/reporting, and ten-track in-memory structure all pass. Generic
production modules remain uncoupled.

# Single recommended next step

Implement D4's explicit proof write and independent disk re-open now that D3's
complete normalized comparison passes.
