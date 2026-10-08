# Bounded SysEx preservation (CCA-017)

## Evidence and scope

CCA-016's exact-consumption verification authorizes the narrow source event
`timing-VLQ F0 07 <six seven-bit bytes> F7` in independently bounded,
Track-owned Descriptor166 musical bodies. CCA-015 identifies the original
failures. Original composition and event-family research is retained in
[REMAINING_BOUNDED_EVENT_FORMS.md](REMAINING_BOUNDED_EVENT_FORMS.md),
[BOUNDED_MIXED_EVENT_WALKER_IMPLEMENTATION.md](BOUNDED_MIXED_EVENT_WALKER_IMPLEMENTATION.md)
and [MIDI_EVENT_FAMILY_INVENTORY.md](MIDI_EVENT_FAMILY_INVENTORY.md).

Detailed original evidence remains in
`/tmp/phoenix-happyone-sysex-cca-016.txt` and
`/tmp/phoenix-next-musical-recovery-cca-015.txt`. The material proof is preserved
here: 28 explicit entries in happyone, each with encoded length seven, six
seven-bit bytes and final F7, reached monotonically through existing bounded
event decoding. Every successor is explicitly F0 or 90. Complete body geometry
and source counts agree; no signature scanning or payload interpretation is
needed. Earlier source-unresolved native SysEx examples do not establish this
rule and are not used as happyone reference exports.

The authentic Experiment 007 data fork is 211468 bytes, SHA-256
`e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`, at:

`/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline`

No private musical project or reference export is added to Git.

## Source decoder and provenance

`src/mixed_event.rs` adds `BoundedSysExEntry` and `MixedEventKind::SysEx` to the
existing generic walker. It retains the complete event range, original timing
VLQ and its bytes, status and length offsets, and a borrowed payload slice with
absolute source range. The payload includes F7. One entry counts as one musical
event, with existing checked delta accumulation and source ordering.

Acceptance requires explicit F0 at the current event cursor, exactly the
single-byte length 07, exactly seven bounded payload bytes, six low-bit bytes,
and final F7. A successor, if present, must independently begin with the
existing timing VLQ and explicit F0 or 90. Existing source compact state is
retained; implicit immediate successors remain refused. CCA-016 found retaining
and resetting state indistinguishable for these bodies because all successors
are explicit. This implementation preserves existing state rather than
importing SMF running-status rules into the SVP source.

Other lengths, overlong length encodings, early F7/high-bit payload data,
missing F7, truncation, overflow, unsupported successors, fragments and escaped
F7 entries remain refusals. No synthesizer parameter or manufacturer semantics
are interpreted, and no payload-value, filename or sequence-name predicate is
used. Bounds, Track ownership, routing, conductor applicability and terminal
validation remain independently necessary.

## Bounded handoff and SMF serialization

The generic Descriptor166 sequence builder uses `DecodedExportEvent::from_sysex`
to retain source range, tick, ordinal and an exact owned copy of payload bytes.
The MIDI adapter validates the payload into `BoundedSysExPayload`. Its private
seven-byte array prevents malformed payload construction at the writer boundary.
The existing scheduled-message architecture gains a SysEx variant; the legacy
`ChannelMessage` type name is retained, but this variant carries no channel.

The writer emits explicit F0, length 07 and the original seven payload bytes.
Only timing is encoded through existing SMF scheduling; payload bytes are never
normalized or reinterpreted. SysEx-containing tracks require `SourceOrder`;
legacy message-priority ordering is refused for them. The generic builder already
uses source ordering. Same-tick SysEx/Note order and generated Note endings retain
the existing checked ordering/provenance rules.

The diagnostic family inventory truthfully reports SysEx. This is not a new
authenticated export or omission capability: existing profile handoff refuses
SysEx, omission inventory refuses it, and it is excluded from the canonical
omission-authority family set. Existing profiles are untouched. The 120-layout
walker does not acquire F0 support; its authority and handoff explicitly refuse
a SysEx item. The 120 readback expectation helper only gains exhaustive handling
of the new scheduled-message variant; its parser/acceptance rules are unchanged.

## Authentic exact-consumption results

Ranges are absolute hexadecimal and half-open. Counts below are source musical
objects, not the expanded Note On/Off message count in SMF.

| happyone Track | Event body | SysEx | Other musical objects | Source count accounting | Last event tick |
|---|---|---:|---|---|---:|
| Track 6, pair 5 | `[0x21236,0x21714)` | 26 | 138 Notes, one Patch | 165 musical objects + one retained initial FF60 context = 166 | 62160 |
| Track 7, pair 6 | `[0x217df,0x22375)` | 2 | 519 Notes | 521 musical objects = 521 | 64320 |

Track 6 primary/secondary ranges are `[0x21223,0x2171b)` /
`[0x2171b,0x217cc)`; Track 7 ranges are `[0x217cc,0x2237c)` /
`[0x2237c,0x223f8)`. Independently validated seven-byte primary tails stay
outside the musical ranges. Source counts are BE-u32 at primary-relative +7:
`[0x2122a,0x2122e)` = 166 and `[0x217d3,0x217d7)` = 521. The initial Track 6
FF60 context `[0x21237,0x21242)` is retained project context, not an invented
MIDI event. Counting context objects in raw source counts is independently
documented in existing SCHOOL PROJECTS research.

First SysEx positions are Track 6 tick 33900 (delta 540, F0 at 0x21478) and
Track 7 tick 12092 (delta 92, F0 at 0x219dc). All 28 entries have fixed golden
range/status/delta/tick/payload assertions in `tests/bounded_sysex.rs`, derived
from CCA-016's recorded table. Subsequent Notes decode correctly; no unsupported
form or trailing unexplained byte remains in either body.

The same structural rule also completes xForm Track 2, with two SysEx entries.
Across the authenticated project, musical walks increase from 128/132 to
131/132. The remaining walk failure is xForm Track 9's context-to-Controller
form at cursor 0xc490 / successor 0xc49c. No context timing shortcut is added.

A temporary before/after comparison against CCA-016's observational walker
compares every known item (excluding newly represented SysEx) by complete debug
representation, including all values, positions, raw bytes and source ranges.
All 131 complete walks match; the remaining refusal is identical. This includes
all 128 previously successful walks and the existing known musical material in
the three newly completed walks. The comparison accounts for 30 new SysEx items.
Evidence log: `/tmp/phoenix-cca017-comparison.log`.

Existing bounded routing succeeds for 103 ordinary Tracks rather than 100;
happyone is nine event-bearing routing successes plus two empty Tracks.
Existing saved-mute logic now derives OFF for the three formerly incomplete
Tracks, without changed predicates: happyone ON/OFF/Unknown becomes 3/6/2;
xForm becomes 1/11/1. Exact-04 guard decoded inventory becomes 22 states across
11 sequences rather than 21 across ten. No inclusion policy changes follow.

## Regression coverage and validation

New focused coverage includes all authentic SysEx rows, payload/source provenance,
exact body consumption/count reconciliation, timing/order, consecutive SysEx,
explicit Note re-entry and later compact Notes, all truncation boundaries,
unsupported lengths and successors, invalid payload bytes/terminators, position
overflow, adapter validation, source-order-only serialization, and complete
synthetic Descriptor166 handoff/SMF byte readback. SysEx tests also preserve mute,
conductor, terminal and ownership refusals.

Existing corpus frontier/routing/mute population assertions are updated only for
newly completed walks. Existing Notes, Controllers, Patches, Pressure, Bend,
source-order and authenticated export tests remain present and pass. No decoder
or handler is removed, and no compatibility profile is rewritten.

Validation:

- `cargo fmt --check`: PASS.
- `cargo clippy --all-targets --all-features -- -D warnings`: PASS.
- `cargo test`: **614 passed, 10 ignored, zero failures**.
- `git diff --check`: PASS.

Full validation includes the focused tests and authenticated reference/regression
checks. Logs: `/tmp/phoenix-cca017-clippy.log` and
`/tmp/phoenix-cca017-full-tests.log`.

## Remaining recovery limits

Both happyone event walks complete, but the sequence remains refused first at
`conductor prelude`. Its two preludes have no proven generic compatibility rule;
its 26-byte FF terminal differs from the accepted generic terminal. Patch/context
translation and saved-Mute-ON inclusion requirements on Tracks 1/2/3 remain
independent. Track/device routing and complete export authorization must still
pass; SysEx recognition does not supply missing context or omission authority.

No additional complete sequence recovery or new happyone native-reference
reconciliation is claimed. The synthetic SMF test is in-memory validation, not
an authentic recovery claim. No new authentic/reference MIDI export or Studio
Vision experiment is performed. Owner files, historical research, specimens and
frozen releases remain unchanged. No staging, commit or push.
