# Pure bounded sequence manifest and assembly bridge

## Purpose and authority boundary

`bounded_sequence` implements Step 2: source-derived construction evidence and
pure in-memory SMF assembly. It does not authorize application export. No
readiness, compatibility-profile, export-handoff, or app-service policy changes
are part of this implementation. Exact-profile precedence is unchanged.

The public entry points are `build_bounded_sequence_manifest(bytes,
sequence_ordinal, division)` and `assemble_bounded_sequence(&manifest)`.
Sequence ordinals select source objects; they are not applicability predicates.
All manifest fields are private, with immutable audit accessors. Callers cannot
supply fabricated bindings, routing results, mute states, or translated events.
The assembly function accepts only a successfully constructed manifest.

## Structural applicability and exhaustive accounting

The builder reparses the same immutable project bytes through
`parse_project_166`, then composes saved-mute evidence, bounded routing, the
bounded CC0 Patch classifier, the mixed-event walker, MIDI adaptation and SMF
serialization. It neither looks up compatibility profiles nor accepts filenames,
paths, hashes, project identities or known sequence names.

Only the established Descriptor166 sequence/container form is supported.
Every musical descriptor must have exactly one ordinal-associated primary /
secondary record pair. Both counts and all ordinal bindings are checked.
Additional musical pairs cannot disappear through descriptor iteration; a count
mismatch refuses the entire sequence. Unexpected records inside the sequence
are rejected by structural parsing. The sequence terminal must have exactly
eight `ff` payload bytes. The selected sequence scope ends at that terminal;
other project sequences are not its musical objects.

Track manifests retain source and descriptor ordinals, descriptor/label ranges,
resolved label/channel, complete routing evidence, saved-mute evidence, event
bounds and consumed range, secondary-record ownership range, and translated
source-located events. Primary performance regions are derived by the existing
validated terminal-boundary helper. Paired musical secondary records retain
their established container ownership; this implementation does not introduce
a second performance stream or a universal interpretation of their opaque data.

## Inclusion and event completeness

Every musical track must have a nonblank bounded label, a nonempty event region,
a complete walk, bounded saved Mute OFF, and successful bounded routing. Any
Mute ON, Unknown, unbound, empty, unsupported or ambiguous track refuses the
whole manifest. No track is silently omitted. This does not establish Mute ON
omission, empty-track export, solo/selection semantics or universal Studio Vision
inclusion behavior.

A successful parser walk alone is insufficient. The builder explicitly accepts
ordinary bounded Notes and the existing accepted CC0-only Patch-to-Note
composition. Every Patch must match the classifier's source ordinal and succeed.
Both the Patch and its first Note become explicit export events. Parsed
Controllers, Channel Pressure, Pitch Bend, other context-mediated compositions
and standalone Patches refuse. Translated-event count must equal logical walk
count, and every classified Patch must have been consumed.

Source ordinals count logical events, including both members of Patch/Note
items. Source ranges remain attached. Before returning success the builder runs
strict adaptation and source-order serialization checks, so unsupported MIDI
values, ambiguous ties and unsafe Note-ending order cannot hide in an accepted
manifest. This preflight is pure; assembly repeats deterministic construction.

## Initial-only conductor completeness

The guard is deliberately an exact bounded envelope, not a general conductor
map grammar. It comes from source records, not native MIDI contents:

- No type-`09` prelude is accepted.
- Both primary payloads are exactly 38 bytes, with prefix
  `00 01 00 00 00 01 00 00 00 00` at +0..+10.
- The four opaque prefix bytes +10..+14 are preserved by the source scope and
  are not interpreted as timing or events. The established primary event
  boundary is +14; no scanning occurs.
- The existing decoders validate the complete initial event: eight bytes for
  Meter, seven for Tempo, including zero initial position, tags and lengths.
- Immediately after each event comes `87 ff ff 7f ff 2f 00`, followed only by
  zero padding to payload end. There is no remaining event-bearing space.
- The entire paired secondary payload is checked: 40 bytes for Meter and 39
  for Tempo. It has the observed single-entry prefix, bounded length fields,
  `00 ff ff ff 2f c5 00 00 00 00` framing, exactly one value copy matching the
  primary, and exactly three trailing `ff` bytes. Additional content or a
  contradictory copy refuses.
- The manifest retains all four full record ranges and both decoded events.

The observed secondary length words at +6 and +10 equal payload length minus
13, the +14 word is one, and the +18 word equals payload length minus 28.
These are envelope equality checks, not a generalized semantic interpretation
of secondary records. The guard admits variable tempo/meter values only through
the established decoders and strict adapter. Zero MPQN, invalid meter values,
unknown historical meter mappings and either meter fallback warning refuse.
Missing meter evidence never produces a default 4/4.

Tests flip every non-event, non-opaque conductor-envelope byte individually.
They also append complete additional tempo/meter events with correctly updated
record lengths, corrupt initial evidence, alter secondary values and exercise
both meter fallback routes. All refuse.

## Timing and assembly

Division must explicitly equal the established `Identity480` contract. No
source PPQN field has been newly decoded, and no unsupported division is
silently scaled or defaulted. The manifest records 480; the bridge passes that
validated value to Format 1 serialization.

Assembly uses the existing conductor adapter with `KnownHistoricalOnly`,
`adapt_text`, `adapt_track` with parsed-routing provenance and
`StrictKnownOnly`, and `serialize_named_musical_track_with_ordering` with
`MusicalTrackOrdering::SourceOrder`. The conductor precedes all musical tracks,
which retain source order. The accepted Patch adapter emits CC0=80 immediately
before Program Change, without CC32. Existing Note-ending normalization and
minimum End-of-Track policy remain in force. No SMPTE Offset, Instrument Name,
native tail padding, filesystem publication or export capability is added.

## Native reconciliation and identity independence

The mandatory evidence test authenticates the source at 211,468 bytes and
SHA-256 `e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`.
It authenticates `Seq R Test` at 1,025 bytes and SHA-256
`a97c0c3e99e95bc0e97941c8823e90f81758c94112c7dd1a64ae707250eca887` before
comparison. These assertions exist only in tests.

The production builder accepts the structurally selected sequence and the
production bridge constructs its MIDI entirely in memory. The strict test
reader compares Format 1, three tracks, 480 PPQN, conductor name/tempo/meter,
track names, channels, every absolute channel-event tick, ordering and bytes.
Tempo is 472440 MPQN and meter reconciles as 4/4. Track 1 has 76 messages on
channel 3; Track 2 has 126 on channel 4: 202 total, comprising 99 Notes and their
endings, two CC0 messages and two Program Changes. There are no CC32 or ordinary
Controllers. Track 2 tick zero is exactly:

```
93 3c 7f
b3 00 50
c3 10
```

One native velocity-zero Note On ending is normalized to the existing generated
Note Off representation. No other channel-byte normalization is allowed.
Whole-file identity is intentionally not required: native-only metadata and
EOT tick 17280 are outside the musical-equivalence contract.

Independent synthetic projects construct accepted manifests with unrelated
sequence, track, device and Patch names, different source bytes and hashes, and
one musical track. A name mutation remains accepted. There is no production
filename/path/hash input, hashing, corpus lookup or sequence-name condition.
The number two is not an applicability gate.

## Refusals and authorization verification

Focused tests cover unresolved ownership, blank labels, empty tracks, incomplete
walks, Mute ON/Unknown, invalid routing, a parsed but untranslated Controller,
unsupported Patch, ambiguous source order, conductor preludes, additional tempo
and meter content, missing/invalid initial events, zero tempo, unknown meter
mapping, meter fallback, contradictory secondary content, unexpected sequence
objects and unsupported divisions. Mutating only the second native musical
track to Mute ON rejects the entire manifest.

A mandatory app-service test builds and assembles the manifest, then verifies
Sequence R still has no resolved export policy or export capability and is not
Ready. An application export attempt returns `sequence_not_export_capable`
before publication. The same test verifies the unchanged six exact-profile
Ready sequences: Bells for her, Girl-U-Want, Sequence K, Ode to Clarke, Over the
Top and Sequence Q. Existing export-handoff and full-suite tests remain intact.

## Limits and Step 3

This is a narrowly bounded musical construction contract. It does not decode
all opaque project fields, generalize conductor maps or secondary-record
semantics, authorize other event families, infer arbitrary inclusion behavior,
or reproduce all native SMF metadata. Unrecognized required structures refuse.
The conductor envelope deliberately rejects other valid Studio Vision forms.

Step 3 remains a separate design and implementation task for bounded export
authorization: define how fresh source-derived evidence may enter readiness and
export-handoff policy, retain exact-profile precedence, and specify application
capabilities, revalidation and publication tests. The next action is a read-only
review of this manifest/assembly boundary and a concrete Step 3 authorization
proposal. Do not connect the manifest to application export without that
separate policy work.
