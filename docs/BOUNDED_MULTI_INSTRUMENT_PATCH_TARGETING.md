# Bounded standalone Multi Patch targeting

Standalone Patch events on a Multi track now resolve their own saved Instrument
selector instead of inheriting a track-wide MIDI channel. Target resolution and
Program-only translation are separate from sequence readiness, mute policy and
export authorization.

The caller supplies an already-established ordinary track's 33-byte association
range and exact event region. The resolver reparses the same supplied source's
root records and accepts only a Patch-only, completely consumed region. It checks
that the association and primary event record belong to the same framed sequence;
it does not discover or replace the caller's descriptor/pair ownership contract.

The accepted association has a count of 2–15 big-endian routing-row entries and
an `ff`-filled unused tail. Entries must be distinct, nonzero and in scope.
Instrument rows must have the established 36-byte layout, with whole-table
agreement between physical ordinal and the identifier at +25. Device lookup uses
the source's own bounded 40-byte name-bearing records and identifier at +33;
missing or duplicate matches refuse. Channel is row +27 plus one, validated as
MIDI channel 1–16. No display-name suffix is interpreted.

The Program-only classifier requires a seven-bit final Program byte, a five-byte
pre-name context with selector, zero and repeated matching Program bytes, and a
bounded post-name label followed by `04 ff ff ff`. Accepted wrappers are:

- `f8` with the canonical ASCII `G` plus decimal raw Program label;
- `00` with an ASCII `I` plus two decimal digits. That label remains metadata,
  and does not supply or alter the final Program byte.

Other contexts, bank tails, malformed lengths and unrecognized event families
refuse transactionally. These predicates describe the bounded observed classes,
not a universal interpretation of `ff ff ff` or arbitrary Patch records.

The resolved DTO retains selector/list-entry association, routing/device identity,
channel, Program, timing, source ordinal/ranges, Patch name and context bytes.
Its owned handoff uses a distinct `TargetedProgram` event. The common MIDI adapter
emits one Program Change on that event's channel, with no CC0 or CC32. Ordinary
Notes, context-mediated Notes, both Controller families, legacy Patch translation
and Pitch Bend retain their existing behavior. The adapter's legacy track summary
assignment represents the first resolved target for this Patch-only helper; it
is not a common route or a fallback for any other Patch.

Controlled Echo Drops evidence established a mutable local list-entry lookup
with unchanged event selector. Native evidence covered the eight D-110 Patch
records and ten SETUPS initialization records, including the distinct I71 form.
Original SCHOOL PROJECTS and its CONTROL resave store different JD-800 channels
(1 and 15). Each resolves from its own source. This does not claim to reconstruct
historically effective OMS or hardware routing.

Synthetic tests cover independent source channels, multiple targets, both
wrappers, repeated/simultaneous events, timing/order/provenance, refusal boundaries,
and unchanged neighboring event-family serialization. The original Nothing FINAL
probe verifies 18/18 Patch handoffs, including Program 48 on original channel 1.
Its track-level experimental gate improves from 12/15 to 14/15. Arpegio's routing
context and complete conductor-envelope coverage remain blocked. No sequence
readiness, UI, publication authorization or mute behavior was changed, and no
Nothing FINAL MIDI file was generated.
