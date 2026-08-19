# Objective

Determine whether the existing bounded Patch representation provides enough
format-level evidence for conservative generic Descriptor166 `patch_evidence`.

# Existing Patch decoder inventory

`decode_bounded_patch_representation` is the shared caller-bounded decoder used
by the mixed-event walker. It returns absolute provenance for the position VLQ,
`ff 7c` marker, payload length/range, opaque pre-name and post-name context,
length-prefixed ASCII name, direct final-payload program byte, post-PC VLQ, and
opaque pre-Note context. It requires an explicit start and Note-status bound;
it performs no discovery or scanning.

`decode_known_track3_2_patch` is a separate strict research decoder with
Track-3-#2-specific constants and is not used as generic evidence authority.
Export Patch translation types in `midi_export` are policy, not decoder facts.

# Mixed walker Patch recognition path

`walk_bounded_mixed_events` recognizes the bounded `ff 7c` dispatch, derives the
caller-bounded payload/post-PC/Note transition, invokes the shared bounded
decoder, and emits `MixedEventItem::PatchToNote`. The walker supplies the
already-decoded `BoundedPatchRepresentation` and its exact representation range.
No file offset, filename, track name, or Ode constant participates.

# Evidence sources

The repository contains four naturally occurring authenticated Patch events:
Track 1, Track 2, Track 3, and Track 3 #2. Independent binary/MIDI correlation
and bounded tests validate their common semantic core. Controlled Experiments
023/024 independently change the direct program byte; Experiments 025–027
validate position/name framing; Experiments 028/029 show bank-tail changes but
leave bank semantics intentionally opaque.

# Evidence table

| Fact | Evidence source | Scope | Independent reproductions | Generic Descriptor166 evidence? |
|---|---|---|---:|---|
| Patch-to-Note item exists | bounded walker + four authentic events | shared bounded representation | 4 events / 4 tracks | YES |
| `ff 7c` dispatch follows position VLQ | shared decoder and four events | common semantic core | 4 | YES |
| one-byte length-prefixed ASCII name | decoder tests and four events | common semantic core | 4 | YES |
| final payload byte is direct program value | MIDI correlation + Experiments 023/024 | observed Patch representation | 4 events, 2 controls | YES |
| Patch representation source range | explicit walker bounds and decoded range | caller-bounded observation | every successful walk | YES |
| post-PC VLQ is decoded timing component | four events | common decoded field, semantics bounded | 4 | not emitted here |
| bank-tail bytes equal CC0/CC32 | Track 2/3 correlation + Experiments 028/029 | compact banked representation | controlled but optionality unresolved | NO |
| opaque context semantics | correlations only | representation-specific | insufficient | NO |
| Patch start/container ownership | caller bounds only | event-stream context | incomplete | NO |
| channel/routing meaning | MIDI export policy | authenticated proof policy | not decoder fact | NO |

# Format-level established facts

The shared decoder and walker establish a conservative Descriptor166 observation
of a bounded Patch-to-Note representation. A successful walk can therefore
report the source representation range, a deterministic observation ordinal,
and the direct decoded program byte. These are observations, not export
translations.

# Target-specific established facts

Names, exact offsets, authenticated channels, bank/program export mappings, and
Ode track identities remain proof policy or research evidence. They do not enter
generic AppService evidence.

# Observed but unproven facts

The meanings of opaque pre/post-name bytes, bank-tail optionality, complete
Patch ownership, and compound post-PC timing remain unresolved. Bank MSB/LSB
fields therefore remain `None` in generic evidence.

# Unknowns

No evidence establishes a universal Patch record, device/instrument identity,
channel field, or applicability to the unsupported 120-byte profile.

# Genericity boundary

Only `source_ordinal`, `source_range`, and `decoded_program` are permitted in
generic `PatchEvidence`. `source_ordinal` is the deterministic bounded-walk
item order; it is not a Studio Vision ordinal. Bank fields remain absent.

# Go/no-go decision

**GO, bounded subset only.** The shared decoder is independently reproduced
across four authentic events on four tracks, with controlled direct-program
confirmation. This supports a conservative generic evidence record without
generalizing bank or routing semantics.

# If blocked: smallest experiment/research needed

No blocker remains for the bounded subset. A future bank-policy task would need
a controlled bank-removal/optional-sentinel experiment before populating bank
fields.

# If allowed: exact fields permitted in generic evidence

Populate one record per decoded `PatchToNote`: deterministic source item
ordinal, the decoder's absolute `representation_range`, direct
`program_change.value`, and `None` for both bank fields.

# Explicit exclusions

No second Patch parser, byte scanner, channel inference, bank translation,
compatibility profile, readiness change, export, 120-byte generalization, or
decoder grammar change is authorized.

# Single recommended next step

Implement the bounded PatchEvidence extraction from the already-successful
mixed-event walk and retain evidence incompleteness until routing is established.
