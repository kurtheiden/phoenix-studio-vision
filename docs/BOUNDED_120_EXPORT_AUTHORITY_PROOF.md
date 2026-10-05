# Bounded 120-layout ownership, routing and accounting proof

`authority120::prove_project120` formalizes existing evidence as necessary
same-source prerequisites. It does not serialize MIDI, publish files, create
AppService capabilities, change readiness, or change the native app.

**Passing these proof layers is NECESSARY but not by itself SUFFICIENT for MIDI
export. All other bounded recovery gates must also pass.**

## Ownership chain

Construction reparses one immutable source using the existing positive
120-layout observation and semantic association. Every observed candidate is
retained, including association refusals. The chain is:

source → observed candidate → uniquely bounded semantic Sequence association
→ consuming ordinary slot → local primary/secondary pair → complete event walk.

The existing marker, label, prelude, contiguous pair-run, terminal, overlap and
exact consuming-slot/pair cardinality guards remain authoritative. Neither a
blank label nor an empty event walk permits dropping a slot. The exact established
ZERO-STATE slot consumes no pair and remains a distinct Track, with its state
range retained. See [semantic bridge](BOUNDED_120_LAYOUT_SEMANTIC_BRIDGE.md) and
[controlled initialization evidence](EXP039_NEVER_USED_TRACK_INITIALIZATION.md).

The proof holds the original source slice, Sequence bounds/name and immutable
Track proofs. Callers cannot supply a pair, event walk, routing table or fabricated
proof. Returned walks retain decoded event/context bytes and source ranges.
Pair and absent-pair state ranges survive. Candidate ordinal identifies selection
within that exact source, not a persistent app session or a globally unique ID.

The existing pair bounds helper validates the established 14-byte primary
prefix/event-start geometry and exact seven-byte terminal grammar. Here it is
only a bounds checker: the ownership premise is established by the 120 association,
not by a counterfeit Descriptor166 container or its routing resolver. Full walk
consumption and accounting are additionally required. The source/normalized ordinal
correspondence is a bounded engineering inference, not a universal pointer decoder.

## Routing

Nonempty ordinary paired Tracks require the established label-relative 33-byte
association inside the owned Sequence, before that Track's primary. Single
Instrument routing accepts only `01 00 i` with an all-ff or all-zero unused tail.
It requires nonzero in-scope `i`, no more than99 routing rows, exact36-byte type10
payloads, whole-table physical-position/+25 identifier agreement, valid selected
+27 channel0–15, and a unique selected+26/device+33 relationship. All type2a device
payloads must have the established40-byte bounded nonblank name shape. The
channel is selected+27+1; names do not supply channels. Duplicate/missing devices,
wrong IDs, malformed rows and invalid channels refuse.

These are the established post-ownership table/device checks documented in
[bounded single-Instrument routing](BOUNDED_SINGLE_INSTRUMENT_ROUTING_IMPLEMENTATION.md).
The 120 association supplies its own ownership premise. The Descriptor166 resolver
is unchanged and is not called. Shared production context validation verifies
Patch/Controller/context agreement, including the narrowly established
[zero-context Note case](BOUNDED_ZERO_CONTEXT_NOTE_ROUTING.md); conflicting contexts
refuse. Explicit MIDI Controllers retain their status-derived channels and are
not forced to share Note routing.

Multi associations use the existing [bounded standalone Multi Patch
resolver](BOUNDED_MULTI_INSTRUMENT_PATCH_TARGETING.md), after ownership and accounting.
Its selector/list-entry, same-source routing/device/channel, complete Patch-only
walk and exact translation guards remain required. It does not become a fallback
for unsupported mixed-event Multi Tracks. Each resolved Patch retains its own
channel/provenance. Empty paired and absent-pair Tracks get `NoEvents`, not an
invented MIDI channel.

Single-Instrument Patch **translation** remains a separate later gate. Routing
agreement does not interpret unknown bank tails, playback state or Patch semantics.

## Accounting

For this bounded ordinary primary subprofile, the four-byte big-endian field
at payload+2..+6 must equal:

`decoded logical source events + explicitly decoded context records`.

This is a per-Track gate, not merely a Sequence-total coincidence. The decoder
must first accept the entire independently bounded event region. Any unknown
status/tag, malformed context, ambiguous extent, unconsumed byte or arithmetic
failure refuses even when the count happens to match.

The exhaustive supported accounting decisions are:

| Decoder item | Logical source events | Separate context records |
|---|---:|---:|
| Ordinary Note, either Controller family, Pitch Bend, Channel Pressure | 1 | 0 |
| Context-mediated Note | 1 | 1 |
| Double-context Note | 1 | 2 |
| Standalone Patch | 1 | 0 |
| Patch-to-Note without initial context | 2 | 1 if the optional following decoded context exists; otherwise0 |

The initial-context Patch-to-Note composition has no certified general raw-count
relation in this particular proof subprofile, so it refuses `UnsupportedAccounting`.
This does not modify that composition's decoder or claim it is invalid. It cannot
be admitted to this proof by inventing a context-count rule. None of the eight
validated targets requires it. A new decoder enum variant requires an explicit
accounting decision at compile time.

Timing VLQs, Note releases emitted in MIDI, bank-message expansion, headers,
secondary bookkeeping and sequence-terminal diagnostics are not counted as
additional musical source events. A bank Patch remains one source Patch even
if its eventual translation emits CC0 plus Program Change. Empty paired Tracks
require declared0, no decoded events and no contexts. Legitimate no-event-data
Tracks have **no declared count**; only their separately proven ZERO-STATE rule
applies. Conductor uses its existing separate bounded count/envelope contract;
ordinary accounting is not generalized to conductor or historical unrelated fields.

Evidence: [terminal diagnostic documentation](NON_EVENT_TERMINAL_DIAGNOSTIC.md)
records NOTHING's exact declared counts and H's298=284+14; [mixed-event
implementation](BOUNDED_MIXED_EVENT_WALKER_IMPLEMENTATION.md) records Clarinet's
269=259+10. The current recorded sweep independently reconciles all64 paired
ordinary Tracks across the eight supported Sequences, with7 legitimate no-event
Tracks separate. Older count-like-field uncertainty does not authorize a
universal count interpretation.

## Non-authorities and refusal boundaries

Success proves ownership/accounting/routing prerequisites only. It does not prove
conductor completeness, single-Instrument Patch translation, source-order MIDI
serialization, sequence-terminal coverage, mute export policy, original timbres,
hardware equivalence, AppService source freshness or destination durability.
All these required downstream gates remain independent. Count reconciliation
cannot demote ff40, f0 or unknown contexts to diagnostics. Unknown material that
could affect musical events, routing, playback state or timing remains fail-closed.
The later builder must use this exact source and successful private proofs;
AppService must still mint its own fresh source/Sequence-bound authority.

No production logic uses project names, hashes, Sequence ordinals or artifact
filenames as acceptance criteria. Authentic material stays external, never copied
into repository fixtures. The optional authentic test authenticates the external
source when available; absence is reported explicitly and portable tests still run.

## Validation

The authentic local test ran on the current external343,875-byte SCHOOL source
with its established SHA-256. All15 candidates remain visible. The eight supported
Sequences pass all applicable proofs:64 ordinary paired Tracks reconcile and7
legitimate no-event-data Tracks remain separate. Seven blocked candidates continue
to refuse: two cardinality shortages, minuet f0, M-vegetarian del ff40, kickin'dance
ordinary terminal bounds, second Wind ensemble complex single association, and
M-vegetarian#2 ff40. These are existing refusals; independent downstream conductor,
Patch and source-order blockers remain unchanged. No MIDI was serialized or
listening artifact rewritten in this proof task.

Portable tests cover contexts, raw shortage/surplus, paired emptiness vs absent
pair, cardinality shortage/excess, wrong secondary framing, malformed/ambiguous
routing/device/channel/context data, source isolation, uncovered extents and
musically unknown material. The original Descriptor166/application path remains
unchanged.
