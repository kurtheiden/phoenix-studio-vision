# Production bounded 120-layout recovery builder

`recovery120::recover_project120(source)` returns every observed candidate in
source order, with either an immutable complete in-memory recovery result or a
stage/refusal reason. It does not write files, integrate with AppService, create
an application capability, or alter the JSON/C/Swift/UI contract.

## Mandatory prerequisites and remaining gates

The builder calls the committed
[120 authority proof layer](BOUNDED_120_EXPORT_AUTHORITY_PROOF.md). No caller may
supply a fabricated association, decoded walk, route or accounting result.
Ownership, ordinary event-region consumption, per-Track raw/logical/context
reconciliation and applicable routing must pass before recovered Track events
participate. Legitimate no-event-data Tracks follow the separate established
ZERO-STATE rule; empty paired Tracks retain their pairs and declared-zero proof.

Passing these proofs remains necessary but insufficient. The builder also
requires the existing compact conductor contract (including key information,
initial-event counts and complete envelopes), exact single-Instrument Patch
translation or established standalone Multi-Patch translation, strict MIDI
adaptation, source-order serialization with Note provenance, exhaustive paired
record coverage and bounded sequence-terminal classification. No unknown event,
unsupported routing, unresolved bank, f0/ff40, conflicting context, count deficit,
conductor uncertainty or playback-changing order ambiguity is downgraded.

Ordinary event handoff uses the existing decoded source pitch/velocity/duration,
position and source ordinals. Controller status-derived channels remain distinct
from Note routing where established. Patch-to-Note supplies both logical events.
The single-Instrument Program-only classifier is the narrow already-established
G/f8 form (`00 00 p f8 p`, bounded G/raw-Program label, `04 ff ff ff` tail), a
subset of the production standalone Multi predicate. It does not interpret a
name as a Program or admit other wrappers/banks. Multi Tracks use the existing
production resolver and its per-Patch target channels; no mixed-event Multi
fallback is introduced.

Existing identity480 timing and PPQN480 policy remain unchanged. Conductor key
serialization uses the existing key-aware serializer. Musical Track serialization
uses the existing source-order/provenance path, including only its established
equivalent-duplicate Note exception. Saved-muted material remains preserved; no
mute filter is added. Empty Tracks serialize as named empty MIDI Tracks and have
no synthetic channel assignment in the recovery report.

## Success result and identity

`Recovery120` retains the exact immutable source-bound `SequenceProof120`, full
source SHA-256, candidate ordinal, Sequence range/name, decoded conductor and
opaque conductor envelope provenance, terminal diagnostic with exact bytes/range,
ordered Track handoffs/scheduled events/counts, aggregate counts and deterministic
MIDI bytes. Private construction and read-only access prevent replacing proof,
source identity or MIDI after verification. Names/ordinals are identity metadata,
not acceptance predicates. No project hash/name/artifact lookup occurs at runtime.

The retained original source/proof contains metadata/context provenance even
where the MIDI handoff intentionally has fewer fields. A consumer may derive a
display fallback for an unnamed Sequence; this builder never invents a historical
UI name. Count summaries distinguish source Patch events from generated bank
messages and Note releases. Existing adapted warnings or untranslated metadata
cause refusal under this established strict pipeline; bounded terminal diagnostics
remain attached to successful recovery rather than silently discarded.

## Mechanical readback

The private `readback120` module structurally reads the generated SMF independently
of the SMF serializer's chunk writer. It checks Format1/MThd length, PPQN, Track
count, chunk extents, bounded VLQs, accumulated ticks, channel data bytes, metadata,
Track names, conductor tempo/meter/key, exact channel-message sequence, one final
EOT at the expected last tick and complete file consumption. Channel expectations
are formed directly from scheduled message variants in source order, including
bank-before-Program order; all attacks and generated releases are included.

Readback intentionally accepts the explicit-status representation produced by
Phoenix; it is not an import API for arbitrary MIDI/running status/SysEx. A mismatch
refuses the entire Sequence. This establishes serializer-to-handoff integrity;
it does not replace the source authority proof or claim independent interpretation
of Studio Vision. Local authentic parity additionally compares complete output
bytes to all eight previously independently verified artifacts.

## Explicit non-authorities

The result is a complete bounded in-memory recovery, not a permission to write or
publish it. Later AppService integration must independently bind and freshly
revalidate source/session/Sequence identity before destination handoff. No source
path or output destination is accepted by this builder. No profile-specific
omission policy, new recovery capability, original timbre reconstruction or exact
Studio Vision/hardware playback is claimed. The Descriptor166 app/export code is
unchanged. Production recovery requires no `/tmp` code/report or pregenerated MIDI.

## Validation

Portable tests cover deterministic Note/key/conductor/terminal recovery, required
accounting, unknown musical material, empty Tracks without invented routing,
independent conductor/terminal/source-order refusals, narrow Patch boundaries and
readback corruption/truncation/trailing-byte rejection.

The authentic external SCHOOL test authenticated the existing source and ran all
15 candidates: eight supported results pass authority and every remaining gate;
all eight outputs pass structural/semantic readback and exactly match their
validated persistent listening artifacts. The seven blocked candidates remain
refused by existing cardinality, event-decoding, ordinary terminal-bound or
routing guards. Independent downstream conductor/Patch/order blockers are neither
investigated nor removed. No private music or generated MIDI is copied into the
repository. The test reports missing external source explicitly; authentic parity
must actually run locally before a productionization checkpoint is accepted.
