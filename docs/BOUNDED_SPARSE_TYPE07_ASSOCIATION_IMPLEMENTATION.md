# Bounded sparse type-07 association — CCA-005

## Supported rule

The existing positive 120-profile discriminator (75 + 120 × count, supported marker/name/bounds guards) and supported initial prelude must pass first. Additional type-07 records immediately after that prelude form a bounded sparse list. Track views begin at candidate +106 with stride 120; labels begin at view +15. Exactly four bytes at view +85..88 identify participation when any byte is nonzero. This is a presence predicate, not a decoded pointer.

The leading Meter/Tempo slots must have zero windows. Ordinary participating slots, in ascending structural order, own successive additional records in physical order. Payload bytes remain opaque. Accept only the evidenced payload lengths 2, 3, 6, 8, 10, 12 and 15 with exact five-byte framing, contiguous record bounds and payload extent. Labels must terminate before the presence window. Require exact subset/list cardinality: missing or extra entries, conflicting special participation, unsupported lengths, overlapping labels and records outside the contiguous list refuse. No owner is selected by text, name, offset or specimen identity.

Retain Track ordinal, presence bytes/range, full record and payload provenance in the semantic association and diagnostic details. When independent pair cardinality refuses, no complete Sequence association/proof is returned. Recognition of metadata never manufactures missing event ownership.

## Independent ownership and exclusions

After the list, unchanged contiguous type-02/type-41 pairing and terminal checks apply. Independently established Track inventory, special positions, exact EXP039 absent-event-data state and consuming-slot/pair cardinality still determine musical ownership. No musical event, routing context, conductor guard, source order, timing, event-count check or export authorization is relaxed.

The initial type-07 prelude remains a separate rule: the existing opaque ten-byte form and evidenced 27/35-byte counted-field forms are unchanged. With no additional records, the previous grammar remains unchanged, including opaque nonzero windows. This implementation does not accept arbitrary type-07 structures, missing/unmatched pairs, inferred Segment ownership, unsupported layouts or metadata inserted among musical pairs. It adds no MIDI meaning for metadata or partial-export capability.

## Evidence

[CCA-003](COMPATIBILITY_ENGINEERING_DECISION_CCA_003.md) selected this as implementation-ready association work. The [durable Prologue checkpoint](PROLOGUE_POST_PM_R13_RECOVERY_RESEARCH_CHECKPOINT.md) preserves PM-R11/12/12A conclusions. PM-R12 conserved Track order, sparse masks and all 64 ordered payloads across original 120 and normalized 166 representations. PM-R12A's prospectively locked Frames 7-8 / Strings prediction (#90 versus #49) was confirmed by the subsequent read-only owner observation; three visible rows corroborated the unchanged mapping. Payload text validates the evidence but is not a parser predicate.

Original Prologue: 33,057 bytes, SHA-256 `b3a3efd8179d3ffe2a5026b3a40eab74da3ce6e674bb06162a176df07389b733`. Normalized comparison: 58,119 bytes, SHA-256 `0618cd5087987d7f694ca7532e1cc8a01b4adf7e3043ec648783d71e7e5ad714`. No specimen was rewritten. Historical PM-R12A A14/B50/C0 means 14 occurrences had independent pair ownership while 50 retained a Frame-level cardinality dependency; it does not mean all 64 have full performance authority.

## Regression and authentic observations

Focused regressions cover sparse gaps and opaque/high-bit payloads, deterministic Track/Sequence identity, exact record accounting, unchanged pair payloads, same-source routing and event order, conflicting masks/counts, unsupported lengths, label overlap, misplaced records and missing terminals. Existing dense, longer-prelude, conductor and cardinality tests remain intact. The CCA-004 authentic bridge assertion now expects the newly implemented association to pass instead of the former metadata refusal.

The opt-in read-only `sparse_type07_authentic_cross_layout_and_remaining_guards_read_only` test pins both hashes, verifies all 64 records across 14 Frames against normalized framing, masks and raw labels, and directly tests unchanged authority/conductor refusals. Supply `PHOENIX_CCA005_ORIGINAL` and `PHOENIX_CCA005_NORMALIZED`. No MIDI serialization/export is called.

Sparse interruption removed: **14/14**. Frame 1, Frames 7-8 and Frame 11 now pass ordinary ownership/routing/accounting, including CCA-004 C0 support, then remain blocked by **conductor Envelope**. The other **11** serialized Frame records remain blocked by **Cardinality**. The separately serialized `Prologue` record remains at its prior type-02/type-05 neighborhood refusal. No complete recovery is authorized or independently verified by this work.

Validation passed: `cargo fmt --check`, all-target/all-feature Clippy with warnings denied, `cargo test` (**598 passed, 10 ignored**), focused sparse tests and the separately invoked authentic cross-layout regression, plus `git diff --check`. No staging, commit, push, authentic save or MIDI export occurred.
