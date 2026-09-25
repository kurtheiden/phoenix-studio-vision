# Bounded saved mute-state interpretation

Implements the narrow saved-state interpretation from
[Mute/inclusion structural scope](MUTE_INCLUSION_STRUCTURAL_SCOPE.md),
[Experiment 035](CONTROLLED_TRACK1_MUTE_ON_TO_OFF.md), and
[Experiment 036](CONTROLLED_TRACK3_MUTE_ON_TO_OFF.md).

## API and boundary

`phoenix::saved_mute::collect_saved_mute_evidence(&[u8])` returns owned,
source-located observations for every musical descriptor. `SavedMuteState`
expresses `On`, `Off`, or `Unknown(SavedMuteUnknown)` with a failure reason.
Sequence/descriptor ordinals, label bytes, descriptor range, optional bound
pair ordinal, raw candidate/offset, neighboring guard, and event range preserve
provenance. ON/OFF is an inferred saved state, not an independently observed UI
state or authorization to include/omit a MIDI track.

The public collector always calls `parse_project_166` on the supplied bytes;
it accepts no caller-constructed parsed context. Malformed/truncated framing
returns the existing `Project166Error` without a partial result. Track-local
unsupported conditions return Unknown. Empty generic roots yield no observations.

The byte is read at descriptor start minus 24 (label start minus 39), inside
the preceding descriptor slice. Checked subtraction/addition and slice bounds
are required, and both derivations must agree. Ownership uses the parser's
validated ordinal binding of the following musical descriptor to its pair;
it does not attach the byte to the preceding descriptor's name.

Decoding requires all of:

- Descriptor166 framing and an unambiguous ordinal binding;
- a bounded, nonblank label (empty/ASCII-whitespace-only labels are unsupported);
- a validated, nonempty event region with the existing terminal grammar;
- neighboring bytes exactly `00 04 00 00 04 01 00`;
- candidate exactly `80` (OFF) or `88` (ON);
- a successful existing mixed-event walk consuming the entire region and
  containing at least one logical event.

No other byte value is interpreted using bit 08. The neighboring pattern remains
an applicability guard, not a decoded subtype. Sequence I's unresolved bindings,
blank labels, empty regions, invalid tails, unsupported values, differing guards,
and incomplete walks cannot produce ON/OFF. Unknown evidence may retain raw
bytes already safely obtained. Structurally empty handling is independent.

The task explicitly requires incomplete walks to yield Unknown, which is stricter
than reporting the earlier byte-only structural prefilter. The existing walker
and its timing basis are reused without changing event semantics.

## Separation from behavior

The new module is a core evidence API only. It is not called by AppService,
compatibility matching, export handoff, MIDI serialization, or readiness code.
No UI, JSON, or C ABI contract changed. There is no conversion from this state
to export disposition. Existing authenticated profiles remain authoritative:
Ode's reference policy includes its two inferred-ON baseline tracks; Bells'
policy omits its two inferred-ON nonempty tracks. An incomplete walk cannot be
bypassed using mute state.

## Authentic fixture observations

External fixtures are read only and checked against their recorded SHA-256;
none are added to Git. Experiment 035's Track 1 transition and Experiment 036's
Track 3 transition decode correctly across relocation, with the unaffected
controlled track retaining its state. Baseline Ode Tracks 1/3 and Bells Tracks
2/7 decode ON within the subset, without changing their differing output policies.
Bells' inferred state is not relabeled as an independently observed mute state.

The actual baseline walk yields these results for the twelve Partial sequences:

| Sequence | ON | OFF | Unknown |
| --- | ---: | ---: | ---: |
| xForm | 0 | 4 | 9 |
| Situation | 0 | 4 | 2 |
| Sequence D | 0 | 3 | 3 |
| Sequence E | 1 | 2 | 5 |
| mission impossibl | 0 | 5 | 5 |
| happyone | 3 | 4 | 4 |
| Sequence I | 0 | 0 | 11 |
| newsong | 0 | 1 | 4 |
| Renaissance | 0 | 3 | 3 |
| Get on up & Dance | 1 | 11 | 4 |
| Jurrasic Park | 0 | 4 | 3 |
| Sequence R | 0 | 1 | 1 |

Eleven sequences gain some state evidence; Sequence I remains entirely Unknown.
The four previously identified sequences require this more precise report:

- xForm Track 11 #2: raw 88 and matching guard, but **Unknown** because the
  existing event walker cannot complete its region.
- Sequence E Track 7: inferred saved ON.
- happyone Tracks 1, 2, 3: inferred saved ON.
- Get on up & Dance Track 1: inferred saved ON.

These are scoped interpretations, not new authenticated UI behavior. The baseline
retains six Ready and twelve PartiallySupported sequences. None becomes newly
exportable. Other incomplete walks also reduce counts from the earlier structural
prefilter; the implementation does not repair or broaden the event walker.

## Tests

`tests/saved_mute.rs` covers all 256 candidate values, every single-byte guard
mutation for both supported states, blank/unbound descriptors, empty regions,
invalid terminal bounds, incomplete events and partial walks, every truncated
prefix of a synthetic project, overflowing record lengths, relocation/reparsing,
authenticated transitions, the complete authentic inventory counts, unchanged
profile dispositions, and unchanged readiness. Authentic cases skip explicitly
when the private fixture is absent; all were available in this workspace.

Existing authenticated MIDI reconciliation and exact-profile regression tests
remain the authority for output equivalence. No new music/project fixture or
production export is created by this implementation task.

Validation in this workspace: eight focused tests passed; full `cargo test`
reported 379 passed, zero failed, and two existing ignored tests. `cargo fmt
--check`, `cargo clippy --all-targets --all-features -- -D warnings`, and
`git diff --check` passed. Documentation links/whitespace and unrelated-file
SHA-256 preservation checks passed. Nothing was staged or committed.
