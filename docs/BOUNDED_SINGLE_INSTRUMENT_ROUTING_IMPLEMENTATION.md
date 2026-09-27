# Bounded single-Instrument MIDI routing implementation

## Scope and evidence

`bounded_routing::decode_bounded_routing(&[u8])` is a pure, read-only library
entry point. It evaluates the bounded F/Z predicate established by the preceding
routing rebaseline. It does not participate in application readiness or export.
It uses no filename, digest, profile, track-name whitelist, or sample offsets.
Synthetic, previously unseen Descriptor166 projects exercise the public entry
point in the tests.

The evidence basis is the authenticated Experiment 007 baseline, the EXP33 and
EXP34 controlled Instrument-assignment saves, the F/Z neighborhoods in the
18-sequence baseline, and the complex associations in `newsong` and the recovered
`chris stuff with audio` project. Existing observations and their limits are
recorded in `ROUTING_EXPERIMENT_RECONCILIATION.md`,
`CONTROLLED_TRACK2_INSTRUMENT_ASSIGNMENT_JUNO106.md`, and
`MUTE_INCLUSION_STRUCTURAL_SCOPE.md`. Fixture paths remain external, following
the repository's existing integration-test convention. Tests require the real
files; there is no fabricated or silently skipped authentic evidence.

## Module boundary

The new module reuses `parse_project_166`, `collect_routing_evidence`, existing
ordinal descriptor/pair authority, `validated_track_event_bounds`, and
`walk_bounded_mixed_events`. It reparses one supplied source, preventing callers
from combining another source's bindings, tables, or event walk. Parsing is
read-only. The existing provisional-routing module is unchanged.

Only `src/lib.rs` exposes the additional module. No application-service, UI,
compatibility-profile, inclusion, Patch translation, readiness, serializer, or
export-handoff implementation is changed. Existing exact-profile behavior
continues to supply authoritative export channels.

## Exact acceptance predicate

All of the following must succeed:

1. Descriptor166 parsing and existing equal-count ordinal ownership succeed.
   The descriptor has a bounded nonblank label. Unequal counts remain ambiguous.
2. Validated musical event bounds are nonempty. The existing bounded walker
   consumes exactly the entire event range. Empty regions return a separate
   refusal; this makes no inclusion or omission decision.
3. At label start L, the bounded 33-byte neighborhood has prefix `01` at L-33
   and first slot `00 i` at L-32..L-30. Exactly one complete tail is accepted:
   **F**, fifteen `ffff` slots; or **Z**, fifteen `0000` slots. Z does not assign
   universal inactive semantics to `0000`. The neighborhood is bounded within
   the sequence, not within the descriptor's nominal range: the established
   label-relative neighborhood begins before that range.
4. `1 <= i < table_count <= 99`. This is an implementation limit, not a limit
   claimed for Studio Vision.
5. All framed type-0x10 payloads are exactly 36 bytes. H1, physical position i,
   must equal H2, the unique record with payload +25 equal to i. Additionally,
   every type-0x10 record must have +25 equal to its zero-based physical position.
   Neither hypothesis is asserted universally.
6. Selected +27 is 0..15; the inferred MIDI channel is +27 + 1. Selected +26
   uniquely matches the observed type-0x2a identifier at +33. The applicability
   guard requires the observed 40-byte device payload shape and a bounded,
   nonblank length-prefixed name ending before +33. Malformed device records
   or multiple matches fail closed. This is not a universal device decoder.
7. Context checks run only on objects returned by the successful event walker:
   Patch pre-name context starts `00`; Controller context is exactly `00 i 00`;
   supported ff60 payloads start `57 7f 00`. Both contexts of double-context
   entries and the initial/following contexts of Patch/Note transitions are
   checked. No opaque context semantics are invented. The exhaustive event
   match ensures future event variants require an explicit routing decision.

Validation returns the first applicable refusal. In particular, empty regions
are reported before association analysis, and unresolved ownership before
musical or association analysis. No weaker fallback is attempted.

## Result and provenance

`TrackRoutingResult` retains structural sequence ordinal and descriptor ordinal.
Its `Result<BoundedRoutingResolution, RoutingRefusal>` distinguishes success from
explicit unsupported, ambiguous, malformed, empty, and incomplete outcomes.
Project/root framing errors reuse `RoutingEvidenceError` and are unmeasurable.

A successful resolution retains pair ordinal, descriptor and label ranges,
label bytes, association neighborhood range, F/Z form, first-slot bytes and
candidate value, table cardinality, selected type-0x10 record/payload ranges,
H1 and H2 positions, table-wide agreement, +25/+26/+27 values, selected device
position/range/name, one-based channel, event and consumed ranges, and every
validated guarded context's kind, source range, and bytes. Successful context
validation is represented by the success result and its complete context list;
Note/pressure/bend-only tracks legitimately have no such contexts.

## Acceptance and refusal coverage

Nine focused tests exercise the public entry point. Generated fixtures are
explicitly synthetic. They cover F/Z success, selected target, both channel
endpoints, maximum accepted table cardinality 99 with candidate 98, H1/H2
agreement, and provenance. Controlled EXP33/34 and authenticated corpus tests
are separate from synthetic mutation tests.

Refusals cover non-01 prefixes, nonzero first-slot high bytes, zero and
out-of-range candidates, empty/one-record/100-record tables, mixed F/Z tails,
residual/multiple slots, seven-Instrument and audio forms, H1/H2 disagreement,
duplicate and missing +25 targets, table-wide disagreement away from the
selected target, malformed payloads and framing, invalid channel, missing or
ambiguous device relationship, malformed device names/payloads, invalid event
bounds, empty events, incomplete walks, conflicting Patch/Controller/ff60
contexts, blank labels, and unbound ownership.

The authentic recovered seven-Instrument and audio project is refused earlier
by unresolved ownership. Synthetic cases separately prove association-level
rejection; the test does not pretend the authentic file reached that gate.
`newsong` has four nonempty association refusals and one empty region.

## Controlled evidence

| Save | Candidate / physical target | +26 | +27 | MIDI channel |
|---|---:|---:|---:|---:|
| EXP33 CTRL, Ode Track 2 | 5 | 12 | 1 | 2 |
| EXP33 EDIT, Ode Track 2 | 6 | 12 | 2 | 3 |
| EXP34 CTRL, Ode Track 2 | 5 | 12 | 1 | 2 |
| EXP34 EDIT, Ode Track 2 | 3 | 11 | 0 | 1 |

All four pass the full implemented predicate. This channel evidence does not
repair or authorize controlled-export inclusion differences.

## Authenticated corpus results

Counts are successful nonempty tracks / empty regions / other refusals.
Sequence I remains unmeasurable under existing ownership, including its blank
descriptor. These counts are routing observations, not output-track counts.

| Sequence | Success | Empty | Other refusal |
|---|---:|---:|---:|
| xForm | 11 | 0 | 2 incomplete walks |
| Bells for her | 12 | 2 | 0 |
| Situation | 6 | 0 | 0 |
| Sequence D | 5 | 1 | 0 |
| Sequence E | 6 | 2 | 0 |
| Girl-U-Want | 3 | 0 | 0 |
| mission impossibl | 10 | 0 | 0 |
| happyone | 7 | 2 | 2 incomplete walks |
| Sequence I | 0 | 0 | 11 ambiguous ownership |
| newsong | 0 | 1 | 4 association refusals |
| Sequence K | 1 | 1 | 0 |
| Renaissance | 4 | 2 | 0 |
| Get on up & Dance | 15 | 1 | 0 |
| Jurrasic Park | 5 | 2 | 0 |
| Ode to Clarke | 9 | 0 | 0 |
| Over the Top | 3 | 0 | 0 |
| Sequence Q | 1 | 0 | 0 |
| Sequence R | 2 | 0 | 0 |

The implementation reproduces all eight expected fully satisfied Partial
sequences: Situation, Sequence D, Sequence E, mission impossibl, Renaissance,
Get on up & Dance, Jurrasic Park, and Sequence R. In total, 100 tracks pass:
83 F and 17 Z. For example, Situation descriptor 2 is F, candidate 3, channel 1;
Situation descriptor 4 is Z, candidate 13, channel 10.

## Non-regression and gates

The explicit service regression test confirms six Ready and twelve Partially
Supported sequences, unchanged inspection/profile evidence and unchanged
existing provisional resolutions after bounded routing analysis. Profile
observed channels remain unset. The normal suite also covers exact profiles,
omissions, Patch policy, MIDI output, and export authorization.

Required validation commands:

```sh
cargo test --test bounded_routing -- --nocapture
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
git diff --check
```

Final validation: the focused suite passed 9 tests; `cargo fmt --check`, strict
Clippy, `cargo test`, and `git diff --check` passed. The full suite passed
396 tests with zero failures and two pre-existing ignored tests. The handoff
records the commit identity.
Raw corpus/gate measurements are under `/tmp/phoenix-bounded-routing-*.txt`;
no large measurement dump or external research fixture is added to the repo.

## Non-goals and remaining limitations

No inclusion/omission or saved-mute generalization; no new Patch/bank
translation; no generalized export authorization; no additional Ready sequence;
no Sequence I ownership solution; no newsong fallback; no multi-Instrument,
audio, high-number Instrument, universal slot, universal H1/H2, or unsupported
event-form solution; no UI change. This module's success is deliberately not
wired into existing readiness or export policy.

Remaining blockers are separate: **I** (inclusion/output scope), **B**
(Patch/bank translation), and **A** (generalized export authorization).
Bounded routing does not solve any of these families.
