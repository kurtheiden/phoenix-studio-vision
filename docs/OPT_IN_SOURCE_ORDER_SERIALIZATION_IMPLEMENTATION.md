# Opt-in source-order SMF serialization

## Scope and API

Native Sequence R Track 2 starts with Note On, CC0, then Program Change at tick
zero. Family-priority serialization moves the Patch before the Note, changing
the instrument state at the Note's start. Source ordering is therefore a
separate, explicit musical-track serialization capability.

`src/smf.rs` adds `MusicalTrackOrdering::{ExistingPriority, SourceOrder}` and:

- `serialize_musical_track_with_ordering(events, ordering)`
- `serialize_named_musical_track_with_ordering(name, events, ordering)`

Existing `serialize_musical_track` and `serialize_named_musical_track` retain
family-priority behavior without caller changes. Existing multitrack assembly
and exact-profile exports continue to use those entry points. No application
or export-authorization caller opts into the new mode in this change.

## Ordering and provenance

Existing priority remains `(absolute_tick, message_priority, stable_ordinal)`.
Source order uses a stable sort by `(absolute_tick, stable_ordinal)` on a copy.
No source ordinals, messages, timing, or caller-owned values are recomputed.
Both modes share the existing delta encoding, channel serialization, optional
track-name emission and minimum End-of-Track handling.

The source-order contract uses existing adapter provenance: a source event's
start has ordinal `2n`; its generated Note Off has ordinal `2n+1`. An adapted
Patch expansion shares the source ordinal and has a defined input sequence:
CC0, optional CC32, Program Change. Stable sorting preserves that intra-source
sequence. No extra sub-order field is necessary.

Equal tick/ordinal groups are accepted only as same-channel CC0→PC or
CC0→CC32→PC, in the supplied order. Other duplicate-ordinal groups, including
reversed expansions, return `AmbiguousSourceOrder`; the serializer does not
repair them by guessing. Supporting the already established two-bank adapter
expansion does not establish any new source Patch semantics.

## Generated endings and refusal boundary

For notes in source order, an earlier Note's generated end precedes a later
same-tick retrigger because `2n+1 < 2(n+1)`. A zero-duration Note preserves its
own start→end, then any later source event. The legacy mode retains its prior
end-before-start behavior, including for zero duration.

The opt-in mode checks same-tick Note ties by channel and key. An ending after
an unrelated same-pitch start returns `UnsafeNoteOffOrder`; only the adapter's
own even/odd zero-duration pair is accepted in that position. A velocity-zero
Note On is treated as an ending for this safety check. Multiple same-pitch
starts at one tick without an intervening end are conservatively refused.
Unrelated channels/keys are independent. These checks are not a universal MIDI
overlapping-voice model or source authenticity validation: callers must retain
the adapter's ordinal and expansion provenance. The legacy mode does not acquire
these new refusal conditions.

## Authenticated reconciliation

Source: `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline`

- 211,468 bytes.
- SHA-256 `e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`.

Native export: `/Users/kurtheiden/Documents/Phoenix Research/Studio Vision MIDI Exports/Project 001/Seq R Test`

- 1,025 bytes; Format 1; three tracks; 480 PPQN.
- SHA-256 `a97c0c3e99e95bc0e97941c8823e90f81758c94112c7dd1a64ae707250eca887`.

The tests require both files, verify their identities before decoding, and never
silently skip missing evidence. The untouched export procedure is owner
provenance. No source or MIDI evidence file is committed.

Production bounded classification/walking and adaptation feed the new serializer.
An independent test SMF reader expands running status and retains event order.
Complete channel streams reconcile:

| Musical track | Channel | Channel messages | Notes | CC0 | PC | CC32 | Ordinary CC |
|---|---:|---:|---:|---:|---:|---:|---:|
| Track 1 | 3 | 76 | 37 | 1 | 1 | 0 | 0 |
| Track 2 | 4 | 126 | 62 | 1 | 1 | 0 | 0 |
| Total | | 202 | 99 | 2 | 2 | 0 | 0 |

Track 2 tick zero is exactly `93 3c 7f`, `b3 00 50`, `c3 10`.
Track 1 needs no ending normalization. Track 2 has one native velocity-zero
Note On ending, compared to the corresponding generated Note Off at the same
tick/channel/key under Phoenix's established Note-ending normalization. Every
other channel byte, absolute tick and event order matches. This is not a
byte-identical complete SMF comparison; optional metadata and EOT padding retain
the existing documented contract.

## Tests and integration limitations

`tests/source_order_serialization.rs` contains 14 focused tests covering both
API variants, unchanged default priority, Patch sub-order, chronological ticks,
generated ends, zero duration, retriggers, ambiguous/refused ties, channel/key
isolation, velocity-zero endings and both authenticated complete streams.
Existing exact-profile expected outputs are unchanged.

This implements only the source-order serialization step. Sequence R remains
non-Ready. Bounded inclusion, a complete sequence/conductor manifest, assembly
integration and fresh bounded export authorization remain separate future work.
Routing/Patch applicability, inclusion, readiness and exact-profile precedence
are unchanged. No generalized export is authorized by selecting this enum.

## Validation results

- Focused source-order suite: 14 passed, none failed or ignored.
- `cargo fmt --check`: passed.
- `cargo clippy --all-targets --all-features -- -D warnings`: passed.
- `cargo test`: 419 passed, 0 failed, 2 existing intentionally ignored tests.
- `git diff --check`: passed.

The ignored tests require an owner-only authorization manifest or explicitly
write a research export; neither authenticated source-order test is ignored.
