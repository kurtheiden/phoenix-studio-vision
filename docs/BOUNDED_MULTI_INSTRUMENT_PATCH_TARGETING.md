# Bounded standalone Multi Patch targeting

Standalone Patch events on a Multi track now resolve their own saved Instrument
selector instead of inheriting a track-wide MIDI channel. Target resolution and
bounded Patch translation are separate from sequence readiness, mute policy and
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

Other contexts, unclassified bank tails, malformed lengths and unrecognized event
families refuse transactionally; the additional evidenced bank forms are below. These predicates describe the bounded observed classes,
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


## Demonstrated standalone Multi bank forms

The previous classifier refused banked standalone cores even when decoding and
per-Patch targeting were complete. The same public bounded recovery helper now
classifies two additional structural families; no Sequence/project identity,
source offset/hash, Track/Patch/Instrument name or ordinal selects behavior.
The shared MIDI adapter accepts an explicit target channel with the existing
Patch translation DTO, reusing its bank/Program handling. Existing Program-only
handoffs remain `TargetedProgram` and retain their prior output.

Evidence combines controlled bank-value experiments 028/029 and the owner's
untouched native Sequence H Multitrack export. The owner confirmed the actual
reference filename `Sequence H`, exported from the original SCHOOL PROJECTS
without intentional Patch/bank/Instrument/mute/other setting changes. Reference:
2,641 bytes, SHA-256
`78b27be7e35408957600349cce1ba0d0135561704e172f83e735bedd0c2e4b0d`;
Format 1, 480 PPQN, eight Tracks. Independent structural parsing identifies four
CC0/Program pairs on the final Track 2, with zero CC32 throughout. Details are in
`/tmp/phoenix-sequence-h-native-multitrack-patch-validation.txt` and
`CONTROLLED_TRACK2_BANK_MSB_CHANGE.md` / `CONTROLLED_TRACK2_BANK_LSB_CHANGE.md`.

For direct seven-bit Program p and resolved selector s:

| Family | Five pre-name bytes | Bounded post-name metadata | Exact tail | MIDI |
|---|---|---|---|---|
| A/f8 | s, 00, p, f8, p | length-prefixed A plus one/two ASCII digits | 04 ff 51 ff | CC0=81, then Program p |
| I/08 | s, 00, (p OR 80), 08, p | length-prefixed I plus one/two ASCII digits | 04 ff 50 ff | CC0=80, then Program p |

Labels remain metadata; their values never supply Program, bank or channel.
Existing bounded field framing permits the demonstrated two/three-byte label
widths without interpreting text. Both families emit no CC32 and introduce no
LSB default. Each pair uses its own resolved target channel and decoded tick.
The adapter appends bank then Program with the Patch source ordinal; the stable
source-order serializer preserves intra-Patch order and simultaneous Patch order.
Resolved provenance now additionally retains the explicit translation class.

All prior association/table/device/selector/channel and complete Patch-only walk
requirements remain. Supplied event bounds must end exactly before the primary
record's established seven-byte Track suffix, so an otherwise valid prefix cannot
conceal later Patches. The caller still supplies established descriptor ownership
and validated ordinary terminal bounds. Cross-sequence ownership, malformed or
truncated cores, invalid Programs, changed tails, unsupported wrappers, ambiguous
or missing routing and mixed-event Multi paths refuse transactionally. This does
not establish arbitrary banks, universal ff optionality, LSB-only behavior,
fully specified bank translation for other wrappers, or historical live MIDI
latch state. Ordinary Track and Sequence terminal predicates are unchanged.

## Current authentic export checks

Sequence H's **Patch export blocker is removed**. Native comparison passes all
four pairs: ticks 0/0/0/480, channels 2/3/1/4, CC0 81/81/81/80, Programs
7/13/23/31, CC0-before-Program and no CC32. Twelve ordinary serialized Track
probes independently match the complete source handoff (280 attacks, 280
releases, four CC0, four Programs), including saved-muted material and four
legitimate no-event-data Tracks; conductor serialization also verifies.

**Sequence H remains blocked** at its newly reached Sequence terminal gate:
`UnsupportedMaterial`, framed type00 record [248314,248347), payload
[248319,248347), bytes
`fffffffffffffffffffffffffffffffffffffffefffefffefffeffff`.
Earlier readiness predictions had not reached this gate because Patch translation
refused first. It is not bypassed or investigated here. No full Sequence H MIDI
or listening artifact is produced; successful Track probes are not a full export.

The same Patch blocker is also removed on source #10 Wind ensemble Track 4
(slot10) and #14 Wind ensemble's first Track 4 (slot8). #10 remains blocked by
Clarinet's context-length mismatch. #14 retains conductor-envelope and four
single-Instrument association refusals. Similar single-Instrument/mixed-event/
zero-context Patch forms in #7/#11/#15 retain their previous refusals.

All 15 current candidates were rerun for breadth: six remain fully exportable;
no additional complete Sequence export passes. DANCING, Nothing FINAL,
NOTHING.MID, unnamed #9, Tuba thing and Sequence M are byte-identical to their
existing listening artifacts and independently mechanically verified again.
No musical data is discarded or invented, and native saved-mute omissions are
not adopted as recovery policy.

Focused synthetic tests use unseen names/Programs and cover bank values, per-
Patch targets, timing, no CC32, source-order serialization, lookalike/tail/wrapper
refusals, invalid Programs, truncated framing, missing/duplicate routing,
invalid channels, mixed events and incomplete event ownership. A separately
hash-authenticated native test parses MIDI structurally and compares serialized
bank/Program messages. Formatting, strict all-target/all-feature Clippy, full
cargo tests and diff whitespace checks pass; detailed logs/results are in
`/tmp/phoenix-standalone-multi-patch-blocker-removal.txt`.
