# Bounded event-composition extensions

## Evidence and scope

The read-only timing/state audit at baseline
`f2f4ddb43042e3720d96e184c8bbda3c3feee8c0` reproduced ten rejected musical
tracks/pairs in authenticated Experiment 007. Full-prefix replay showed that
existing bounded primitives could consume these cases without changing their
established timing semantics. The source is 211,468 bytes, SHA-256
`e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`.
Private source/music files are not included in Git.

This change adds structural decoder coverage only. There are no authenticated
MIDI references for these exact ten cases. Their complete consumption and
internally continuous timing are not new reference-proven MIDI recovery.
Existing Bells reference correlations support the reused timing rules:
Patch/Controller/first-Note ticks are 480/960/71040 on Track 3 and
180/208/71278 on Track 4. Bells Track 6 retains the preceding Controller at
130 when deriving Patch tick 290 and first Note tick 71903.

## Added grammar and ownership

Starting at the current cursor within exact validated musical-event bounds:

- An existing bounded Patch core may be followed by one through three ordinary
  Controllers and a direct explicit `90` Note.
- After one or two Controllers, the terminating Note may instead use one
  existing single `ff 60` context with payload length 7 or 8, immediately
  followed by its timing and explicit Note.
- A successfully decoded Patch core may terminate the event range only when
  its declared core ends exactly at the validated event end.

The Patch owns its core, each Controller owns its timing and record, and the
context-mediated Note owns its context and both timing components. Existing
raw bytes, opaque contexts, values and absolute provenance are preserved.
No sequence name, source hash or specimen offset affects production dispatch.

All additions use checked arithmetic. Patch position adds its raw component to
the retained previous event position; each Controller delta is added once;
the Note adds its direct delta or the existing leading-plus-final context
components. Controllers clear compact state. Only the required terminating
explicit Note establishes Note continuation state. The branch is transactional:
an incomplete chain returns an error, not partial decoded output.

The terminal form returns the existing standalone `PositionedPatch` with all
core fields. It fabricates no trailing timing, pre-Note context or Note state.
The enclosing seven-byte track tail is outside the validated event range.
A core ending before that range's end must still satisfy established successor
grammar; it cannot succeed by treating remaining bytes as terminal padding.

## Fail-closed boundary

The new Controller branch rejects a fourth Controller, a context after three
Controllers, context lengths other than 7/8, additional contexts,
context-to-Patch/Controller, consecutive Patches, `f0`, pressure, bend, compact
data and wrong Note status. Missing/truncated Notes, malformed Controllers,
invalid timing, arithmetic overflow and bound violations fail. No signature
scanning, recovery or resynchronization is introduced.

Terminal acceptance requires a complete core and exact end equality. Truncated
cores, extra timing, unsupported trailing bytes and track-tail bytes supplied
as event bytes do not qualify. Existing supported nonterminal forms remain
supported.

## Authentic full-prefix results

Bounds and Patch starts below are absolute hexadecimal, half-open where ranged.
Final tick means final accumulated event position, not sequence duration or the
last Note's ending.

| Sequence / track or pair | Event bounds | Patch start | Logical events | Final tick |
|---|---|---|---:|---:|
| xForm / Track 6 | a1c7..a613 | a1c7 | 135 | 96151 |
| xForm / Track 11 | d51b..d705 | d51b | 72 | 63360 |
| xForm / Track 12 | d82c..db1f | d82c | 123 | 140154 |
| xForm / Track 11 #2 | dcc7..dec8 | dcc7 | 73 | 63363 |
| Sequence I / unbound pair 1 | 231c8..234c0 | 231c8 | 106 | 32400 |
| Get on up & Dance / Track 2 | 28a48..2905e | 28a48 | 255 | 101286 |
| Get on up & Dance / Track 2 #2 | 291bc..297d2 | 291bc | 255 | 101292 |
| xForm / Track 1, terminal Patch | 7671..7854 | 7835 | 65 | 99818 |
| newsong / second Track 2, terminal Patch | 24d67..24d89 | 24d67 | 1 | 0 |
| Get on up & Dance / Track 9, terminal Patch | 2c95e..2ca44 | 2ca22 | 28 | 69120 |

The terminal Patch timing bases/components are respectively 94080+5738,
0+0 and 53768+15352. No trailing timing component exists in these event bounds.

All ten now consume completely; none merely advances to another failure.
All 115 previously successful musical-pair walks retain identical decoded
results, compared before/after including positions, values, raw bytes and
provenance. Total successful pairs increase from 115/132 to 125/132.
The permanent corpus test locks the ten event counts/final ticks and exact
remaining failure membership. The before/after comparison used temporary
external tooling against the baseline and rebuilt library; no second production
parser or private fixture was added.

Remaining failures are xForm Track 2 (`f0`) and Track 9 (context-to-Controller),
happyone Tracks 6 (context-to-Patch) and 7 (`f0`), newsong's first Track 2
(consecutive Patches), Renaissance Track 5 and Sequence R Track 1
(context-to-Patch). These are deliberately unchanged.

Get on up & Dance reaches 16/16 complete musical ranges. Sequence I reaches
10/10 complete pairs but still has unresolved 11-descriptor/10-pair ownership.
Five of the former seven event-failing Partial sequences retain event failures.

## Authorization and diagnostic consequences

Ready/Partial remains **6/12**. No AppService authorization, compatibility
profile, routing, inclusion, MIDI export policy, ownership, UI or FFI rule
changes. Existing authenticated export/reference tests remain the authority
for supported MIDI recovery.

The unchanged saved-mute collector now passes its complete-event-walk guard
for additional rows. xForm changes from ON/OFF/Unknown counts 0/4/9 to 1/8/4;
newsong from 0/1/4 to 0/2/3; Get on up & Dance from 1/11/4 to 1/14/1.
xForm Track 11 #2 now yields inferred saved ON. This is not independently
observed historical mute behavior or export omission authority. Sequence I
remains entirely Unknown because binding is unresolved. Saved-mute semantics
are unchanged; only their structural prerequisite is newly satisfied.

## Validation

Focused tests cover chain lengths, context lengths, nonzero accumulated timing,
Controller and Note provenance, opaque context preservation, compact Note
continuation after the required explicit Note, malformed/truncated successors,
unsupported statuses, no scanning, overflow and exact terminal boundaries.
Existing one-Controller/direct-Note behavior remains covered. The obsolete test
requiring rejection of every second Controller is replaced by explicit bounded
positive and negative cases.

Run `cargo fmt --all -- --check`,
`cargo clippy --all-targets --all-features -- -D warnings`,
`cargo test --test bounded_mixed_event_walker --test saved_mute`,
`cargo test`, and `git diff --check` for this checkpoint.

Checkpoint results: 31 mixed-event tests and 8 saved-mute tests passed; the
full suite passed 382 tests with zero failures and two ignored tests. Formatting,
all-target/all-feature Clippy with warnings denied, and diff whitespace checks
passed. The authenticated readiness test remained 6 Ready / 12 Partial, and
existing export/profile/reference regressions passed. Pre-existing unrelated
worktree files were verified unchanged by content hashes.
