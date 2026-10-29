# Generalized partial120 admission

Baseline: `ae0eb1aa34db644606e6ccf6e525ab24004b3bdf`. Implemented 2026-10-29.

## Scope and evidence

`experimental120::build` previously required exactly one ordinary primary-only
body and rejected every `no_event_data` descriptor. It now accepts the unchanged
semantic bridge's successful associations: zero or one primary-only body, any
number of proven paired ordinary Tracks, and exact EXP039 no-data descriptors.

[EXP039](EXP039_NEVER_USED_TRACK_INITIALIZATION.md) establishes only the exact
eight-byte state `00 00 04 00 00 04 00 00` as consuming no physical body within
the supported neighborhood. Blank names, absent Instruments and empty music are
not substitutes. [CCA-039](BOUNDED_PRIMARY_ONLY_TRACK_OWNERSHIP_CCA_039.md)
separately supports the existing bounded primary-only association. Its cap is
retained. Semantic association, cardinality, adjacency, special positions,
preludes and all decoder/routing/conductor/terminal rules are unchanged.

Paired, primary-only and no-data Tracks are merged by descriptor ordinal. A
no-data Track emits only its available name and MIDI EOT, with no channel or
musical events. Its ledger retains descriptor-view, raw label and exact state
provenance. `primary_range` is absent for these Tracks; the zero object count is
structurally established, not read from an invented primary. Existing physical
Tracks retain the same serialized ledger fields; new no-data provenance is
omitted when inapplicable. Empty paired Tracks still consume their real pairs.

Unresolved placeholders, prefix termination, playable-Note requirements,
readback, incomplete disclosures, source revalidation, consent and exclusive
publication are unchanged. No scan, restart, repaired source, project-specific
exception or additional primary-only grammar was introduced. Complete recovery
code and behavior were not changed.

## Five-project comparison

The same inventory IDs and authenticated data-fork hashes were used before and
after. These are previously exposed audit cases, not blind validation or proof
of five independent musical lineages. Outputs and source mappings remain private.

| Inventory ID | Recognized before/after | Enumerated sequences before/after | Complete before/after | Partial before/after | Unavailable before/after |
|---|---|---|---|---|---|
| project-001 | No / No | Unknown / Unknown | 0 / 0 | 0 / 0 | Unknown / Unknown |
| project-002 | Yes / Yes | 20 / 20 | 0 / 0 | 1 / 1 | 19 / 19 |
| project-003 | Yes / Yes | Unknown / Unknown | 0 / 0 | 0 / 0 | Unknown / Unknown |
| project-008 | Yes / Yes | 15 / 15 | 8 / 8 | 0 / 0 | 7 / 7 |
| project-011 | Yes / Yes | 1 / 1 | 1 / 1 | 0 / 0 | 0 / 0 |

Recognition remains 4/5; semantic enumeration remains 3/5. Among the 36
enumerated sequences: nine complete, one partial and 26 unavailable, unchanged.
No additional authentic sequence qualified. Synthetic positives establish the
new admission behavior, not an authentic recovery gain.

| Export class | Notes before/after | Controllers before/after | Program Changes before/after |
|---|---|---|---|
| Nine complete | 16,812 / 16,812 | 132 / 132 | 48 / 48 |
| Existing partial | 3,921 / 3,921 | 5,870 / 5,870 | 15 / 15 |
| Newly qualified | 0 / 0 | 0 / 0 | 0 / 0 |

Controller totals include Bank Select, matching the prior audit's output-count
convention. Complete and partial rows are disjoint.

The 26 unavailable enumerated sequences now first refuse partial qualification
at 22 existing conductor-envelope checks (13 Tempo, nine Meter) and four semantic
association checks (three cardinality shortages, one incomplete neighborhood).
These are partial-builder first refusals, distinct from the prior complete-path
first-refusal categories. They do not establish downstream recoverability.
The nine complete sequences remain excluded from partial output as already
verified. No conductor relaxation was added to manufacture an eligibility gain.

## Validation

Focused synthetic tests cover source order, failed Tracks alongside independently
proved Notes, empty paired bodies, interleaved named/unnamed EXP039 descriptors,
all eight near-state mutations, blank/unassigned nonqualifying descriptors,
shortage/surplus, multiple primary-only bodies, missing secondary/foreign
adjacency, prefix truncation and stop boundaries, routing/count integrity,
conductor counts/timing and the playable-Note requirement. The native JSON
publication regression also runs the new paired/no-data fixture through consent,
stale/cross-source reports, source changes, mute refusal and collision protection.

Before-change MIDI hashes were captured from the expected HEAD. All 25 protected
Experiment 007 outputs match. All nine audited complete MIDI files also match,
including actual AppService publications. Sweeper's existing partial MIDI,
serialized report and limitations sidecar are byte-identical before/after; its
authentic native JSON publication regression passes separately.

Strict all-target/all-feature Clippy, formatting, native Xcode build, external C
and Swift bridge checks, Swift model decoding and 131 workflow checks pass.
The full Rust suite passes: 829 passed, zero failed, 26 ignored. The authentic
native partial-publication and protected-output tests passed separately.

All five audit sources and the excluded Experiment 007 control retain identical
data hashes, size, mode, owner/group, inode, modification/change times and full
extended-attribute snapshots. Read access time is not an immutability check.
Every pre-existing modified/untracked file matches its initial hash. No source,
reference MIDI or private mapping was added to Git; nothing was staged, committed
or pushed. No SheepShaver or new controlled experiment was used.
