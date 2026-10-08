# Bounded saved-mute 04-guard implementation (CCA-010)

Baseline: `360bff267d7865d8748b1561db13a938b3ac17a5` on `main`.

## Supported rule

The existing Descriptor166 saved-mute decoder additionally accepts exactly
`04 04 00 00 04 01 00` as the seven neighboring guard bytes. The existing
`00 04 00 00 04 01 00` form remains accepted. The candidate is at descriptor
start minus 24, equivalently bounded label start minus 39: `80` means inferred
saved Mute OFF and `88` means inferred saved Mute ON. Other values remain unknown.

All existing applicability checks remain: independently resolved ordinal Track
binding, nonblank bounded label, candidate bounds, validated nonempty event body,
and exact complete nonzero mixed-event walk. This extends one guard predicate;
it does not extend layouts, ownership rules, routing, timing, conductor handling,
or export authority. No filename or specimen identity gates production behavior.

## Evidence

[Original saved-mute implementation](BOUNDED_SAVED_MUTE_IMPLEMENTATION.md) and
[controlled inclusion scope](MUTE_INCLUSION_STRUCTURAL_SCOPE.md) document the
established 00-guard rule and its inclusion limitations. Controlled Experiments
035 and 036 demonstrated visible ON to OFF transitions (`88` to `80`) and their
specific native MIDI inclusion behavior; they do not authorize a universal
mute-based omission policy.

PM-R10 independently demonstrated visible Ode to Clarke Track 7 Mute OFF to ON,
with candidate `80` to `88` at `0x2f6a3`, invariant exact 04 guard, and preserved
168 original event bodies, 168 secondary payloads, routing and ownership.
Its source CONTROL is 211468 bytes, SHA-256
`e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`;
PMR10 EDIT is 211468 bytes, SHA-256
`681a0446c074d87273cee70cf80829984084effcc8d0fdcb200015fa3af03ab8`.

Research references:

- `/tmp/phoenix-prologue-saved-mute-04-guard-pm-r09.txt`
- `/tmp/phoenix-prologue-saved-mute-04-guard-pm-r10-preregistration.txt`
- `/tmp/phoenix-prologue-saved-mute-04-guard-pm-r10-analysis.txt`
- `/tmp/phoenix-mute-reconciliation-cca-009.txt`

CCA-009 reconciled the completed research and identified this exact guard as
proven but not implemented. The implementation uses that established evidence,
not a new experiment or a broader interpretation of the prefix.

## Exclusions and export policy

Arbitrary 04-prefixed forms, other candidate values, empty event bodies,
unresolved binding and incomplete event walks remain refused. The generic
bounded recovery builder still requires saved Mute OFF; recognizing Mute ON
never authorizes silently dropping or including a Track. Existing exact-profile
authority and whole-sequence refusal rules are unchanged. Saved state is not a
new Standard MIDI File mapping or a new partial-export capability.

## Regression coverage

`tests/saved_mute.rs` checks all 256 candidate values under both guards, mutations
of every guard byte, provenance, unresolved binding, blank labels, empty bodies,
truncated events and incomplete consumption. Existing 00-guard controlled and
relocation regressions remain. The PM-R10 authentic comparison verifies artifact
hashes, the causal transition, all 168 primary event bodies and secondary
payloads, Track associations, names, ranges and decoded routing equivalence.
External authentic tests explicitly skip when their research files are absent;
they ran against the available specimens for this validation.

`tests/bounded_sequence.rs` verifies that an exact 04-guard OFF fixture retains
sequence identity, Track ownership, routing, timing and events, while the same
fixture with ON still refuses at mute. Existing conductor and ownership refusal
coverage remains unchanged.

## Authentic impact and limitations

The authenticated Experiment 007 newest STUFF baseline contains 24 exact-04
candidate slots. Existing checks now establish 21 previously unknown states:
20 OFF and one ON across ten sequences (xForm, Bells for her, Situation,
Sequence D, Sequence E, Girl-U-Want, mission impossibl, newsong, Jurrasic Park,
and Ode to Clarke). Two Sequence I slots remain unbound; happyone Track 7 remains
unknown because its event walk is incomplete. These are inferred natural states,
not 21 independent causal experiments.

Five newly decoded rows belong to existing exact profiles; sixteen belong to
unsupported contexts. No current first export blocker is removed and no
additional complete recovery is verified. Independent conductor, terminal,
event, ownership and routing refusals remain. Jurrasic Park's newly decoded ON
state continues to be excluded by the general OFF-only recovery contract.
No authentic specimens were modified and no production MIDI export was made.

## Validation

Focused saved-mute tests: 11 passed. The focused exact-04 bounded-sequence
OFF/ON regression passed. Full validation passed: `cargo fmt --check`,
`cargo clippy --all-targets --all-features -- -D warnings`, `cargo test`
(602 passed, 10 ignored, zero failures), and `git diff --check`. These checks do
not authorize any broader acceptance rule.
