# Two narrow remaining event-composition forms

## Evidence and scope

The read-only audit against `882216ff62456d6c17fbed634eabd80f48886200`
examined five non-f0 failures in authenticated Experiment 007. It approved two
narrow forms and left context-to-Controller unsupported. This implementation
changes only bounded decoder coverage; it does not authorize new exports.

The authentic project is 211,468 bytes, SHA-256
`e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`.
No private project/music files are added to Git. No authenticated MIDI-output
reference was found for any of the five audit targets. Existing primitive and
Bells/Ode timing evidence supports reuse of bounded decoders; it does not
constitute independent MIDI-output correlation for these new compositions.

## Initial zero-context, direct Patch, explicit Note

Applicability requires the validated musical-event beginning, compact state
None, zero leading timing, exactly one bounded ff60 context with length 7 or 8,
and an immediate existing direct Patch-to-explicit-90-Note representation.
The explicit Note is mandatory. A terminal Patch, intervening Controller,
second Patch or post-Patch context does not qualify.

`BoundedPatchToNoteTransition.initial_context` retains the leading timing and
complete opaque context, including its raw bytes and absolute ranges. The
transition's enclosing range includes that context. The existing nested Patch
and Note representations retain their own exact ranges and fields. The optional
field is None for every previously supported Patch-to-Note representation.
No new semantic interpretation is assigned to the context payload.

Zero leading timing is a guard, not a generalized context-timing rule. The
existing direct Patch-to-Note decoder adds the Patch delta once to the retained
position, then adds the direct Note delta once, using checked arithmetic.
Only the explicit Note establishes Note state. Ordinary walking resumes after
that Note, preserving existing compact-Note behavior.

The helper is transactional. It accepts only a single direct Patch-to-Note
result; broader existing Patch transitions do not qualify. Other established
initial context/Note forms continue through the unchanged normal dispatch.
Nonzero leading timing, other context lengths, malformed/truncated context,
additional contexts, context-to-Controller, intervening events, wrong/missing
Note status, f0, malformed Patch, overflow and invalid bounds fail closed.
There is no scanning or resynchronization.

## Exact terminal two-Patch form

Normal monotonic parsing must reach a valid bounded Patch core. Its declared
end is the timing start of exactly one additional bounded Patch core. The second
core must end exactly at the validated musical-event end.

Both existing standalone Patch-core representations are retained with all
program/payload/opaque fields and source provenance. Position is computed as
previous position plus first Patch delta, then second Patch delta, with checked
arithmetic. No Note state, trailing timing or successor is fabricated.

This is not a recursive or arbitrary-length Patch chain. Malformed cores,
third Patch, successor event, trailing timing, extra bytes, track-tail bytes
inside the supplied event range and non-exact termination are rejected.
The enclosing track tail remains outside validated event bounds. Existing
single terminal Patch and nonterminal Patch forms remain supported.

## Authentic results

Offsets are absolute hexadecimal; ranges are half-open. Final position is the
last accumulated event position, not duration or final Note ending.

| Sequence / track | Event range | Result | Timing |
|---|---|---|---|
| newsong / first descriptor labelled Track 2 | 24cd9..24d17 | Complete, 2 Patches | Patch ticks 0, 480; final 480 |
| Renaissance / Track 5 | 269f6..26a9a | Complete, 20 events | Patch 8467; first Note 20911; final 33824 |
| Sequence R / Track 1 | 336c0..337c4 | Complete, 38 events | Patch 0; first Note 1; final 15117 |
| happyone / Track 6 | 21236..21714 | Initial composition supported; whole walk fails | Patch 62; first Note 360; later f0 at 21478 |
| xForm / Track 9 | b168..cf38 | Remains unsupported | Context-to-Controller at c490 / c49c |

Full-prefix walking, not suffix-only parsing, establishes these results.
Musical-range consumption increases **125/132 to 128/132**. newsong becomes
5/5, Renaissance 6/6 and Sequence R 2/2 complete. happyone Track 6 remains
incomplete: the next failure is UnsupportedStatus at cursor 21476, status
21478, observed f0. No partial event list is returned as successful recovery.

The exact remaining four failures are:

- xForm Track 2: f0 at 7a27, cursor 7a25.
- xForm Track 9: context-to-Controller at c49c, cursor c490.
- happyone Track 6: f0 at 21478, cursor 21476.
- happyone Track 7: f0 at 219dc, cursor 219db.

Context-to-Controller remains unsupported because a mechanically successful
exploratory parse did not establish the context's timing/semantic ownership.
No new f0 grammar or SysEx meaning is inferred. The two original f0 failures
and the newly exposed third one remain explicit errors.

## Regression and authorization boundary

Before/after comparisons of all 132 authentic musical pairs show all 125
previous successes retain identical event values, timing, raw bytes and source
ranges. The comparison accounts only for the newly added optional
`initial_context: None` field; it does not ignore other differences.
Exactly the three predicted ranges newly succeed. Temporary comparison tools
and snapshots were outside the repository.

Ready/Partial remains **6/12**. No AppService export authorization, routing,
inclusion, saved-mute semantics, Patch-output/bank policy, compatibility profile,
descriptor ownership, readiness, UI or FFI contract changes.

The unchanged saved-mute collector gains complete-walk evidence: Renaissance
ON/OFF/Unknown becomes 0/4/2 (from 0/3/3), and Sequence R becomes 0/2/0
(from 0/1/1). These are inferred saved states, not independently observed
historical behavior or permission to include/omit MIDI tracks. Other sequence
mute counts remain unchanged. Sequence I ownership remains unresolved.

## Tests

New focused tests cover both context lengths, zero-only timing, retained
nonzero timing bases, context/Patch/Note provenance, compact Note continuation,
initial-position/None-state applicability, malformed/truncated components,
unsupported/intervening successors, no resynchronization, overflow, exact
terminal pairing, third-Patch rejection and exclusion of track-tail bytes.
Authentic tests lock event counts, final positions, per-sequence consumption
and all four remaining error cursors/statuses. Saved-mute fixture expectations
are updated only for newly complete event walks.

Validation commands: `cargo fmt --all -- --check`,
`cargo clippy --all-targets --all-features -- -D warnings`,
`cargo test --test bounded_mixed_event_walker --test saved_mute`, `cargo test`,
and `git diff --check`.

Checkpoint results: 36 mixed-event tests and 8 saved-mute tests passed. The
full suite passed 387 tests with zero failures and two ignored tests. Formatting,
all-target/all-feature Clippy with warnings denied, and diff whitespace checks
passed. Existing authenticated export/profile regressions passed and readiness
remained 6 Ready / 12 Partial. Pre-existing unrelated files remained
byte-identical to the initial content-hash snapshot.
