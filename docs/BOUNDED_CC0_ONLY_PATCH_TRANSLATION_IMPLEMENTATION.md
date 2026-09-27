# Bounded CC0-only Patch translation

## Evidence and scope

The owner's untouched native Sequence R Multitrack export supplies the missing
output discrimination for the observed `ff 50 ff` form. This implementation
adds a pure library classifier, not a sequence-specific policy or export path.

Authenticated source:

- Path: `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline`
- Size: 211,468 bytes.
- SHA-256: `e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`.

Native reference (no filename extension):

- Path: `/Users/kurtheiden/Documents/Phoenix Research/Studio Vision MIDI Exports/Project 001/Seq R Test`
- Size: 1,025 bytes.
- SHA-256: `a97c0c3e99e95bc0e97941c8823e90f81758c94112c7dd1a64ae707250eca887`.
- Format 1, three tracks, 480 PPQN.

The untouched-export procedure is owner provenance. Tests independently verify
both identities and parse the native MIDI. Neither fixture is committed.
Related evidence: controlled bank-value experiments 028/029, the bounded Patch
decoder, and `tests/bells_cc0_only_patch_evidence.rs`. This does not generalize
Bells' exact-profile authority or decode a universal bank sentinel.

## Architecture and provenance

`bounded_patch_translation::classify_bounded_cc0_patches(&[u8])` derives bounded
routing and complete event walks from the same immutable source. It accepts no
external decoded objects, forged bindings, filenames, hashes, or profiles as
applicability inputs. The only existing production change is the new module
export in `src/lib.rs`.

Each descriptor result retains sequence/descriptor ordinals and either a typed
track-level refusal or routing provenance plus per-Patch classification. Every
Patch on an accepted track receives an explicit result. A refused track does
not imply that it has zero Patch events.

An accepted `BoundedCc0Patch` retains the entire decoded Patch/Note transition:
source ranges, borrowed name/context bytes, initial ff60 context if present,
program and timing fields, and first Note. Construction is private. Its logical
source ordinal counts preceding events, including Notes contained in preceding
Patch/Note items. `decoded_export_event()` supplies the existing
`ConfirmedBankSelectMsb { msb: 80 }` representation to the adapter. The track's
bounded routing result supplies the channel; this is not an inclusion decision.

## Exact acceptance boundary

Track gates reuse bounded routing without relaxation: Descriptor166 framing,
established unambiguous binding, bounded nonblank label, nonempty validated
events, complete walk, accepted association/table/device/context guards, and
inferred MIDI channel.

Patch gates require:

1. A walker-validated Patch-to-explicit-Note transition; standalone, chained,
   or context-mediated successors are refused.
2. Absolute Patch position zero.
3. Payload length exactly 27; bounded ASCII name length exactly 12.
4. Direct program byte p is seven-bit and in `{16, 35}`.
5. Pre-name context exactly `00 00 (p | 0x80) 08 p`.
6. Post-name context exactly eight bytes:
   `03 <three ASCII bytes> 04 ff 50 ff`.
7. No post-Patch ff60/final-context timing or pre-Note opaque context. If an
   initial ff60 context participates, its payload length is exactly seven and
   starts `57 7f 00`. The existing walker also validates its placement/timing.

Name text, the three ASCII bytes' exact values, sequence/track names, source
hash, and corpus identity are not gates. Those three bytes remain opaque.
Synthetic fixtures use unseen project/track/Patch names and `XYZ` in that field.
ASCII means the existing decoder's ASCII scope, not an invented label syntax.
The apparent bit relationship in pre-name context is only an exact byte guard.

Failures retain typed reasons: unsupported transition, nonzero position,
payload/name length, invalid/unsupported program, pre/post-name context, or
initial context. Existing framing and routing failures remain explicit.

## MIDI semantics and ordering boundary

For an accepted Patch, emit CC0=80 immediately followed by the direct Program
Change, at its decoded position on the bounded channel. Emit no CC32. There is
no LSB default and no generic interpretation of `ff` outside this exact form.

The classifier's source ordinal and the existing adapter preserve logical
source order. Reconciliation confirms the following native tick-zero order:

| Track | Channel | Ordered status/data bytes |
|---|---:|---|
| Track 1 | 3 | `b2 00 50`, `c2 23` |
| Track 2 | 4 | `93 3c 7f`, `b3 00 50`, `c3 10` |

The native reference contains no CC32 anywhere in either musical track. The
adapter produces one CC0, zero CC32, and one PC per track. Its tick-zero message
order matches the reference, including Track 2's preceding Note On.

**Downstream limitation:** the legacy `serialize_musical_track` sorts equal-tick
messages by family priority before source ordinal. Passing the complete Track 2
adapter output through that serializer moves CC0/PC ahead of the preceding
Note. This task does not change that established exact-profile behavior or wire
this classifier into export. The reconciliation test checks native ordering at
the classifier/adapter boundary and separately checks serialized bank/PC bytes
and the absence of CC32. It does not claim complete serialized-track equality.
A future generalized export integration must preserve the established source
order explicitly. The API documentation warns consumers of this limitation.

## Corpus and policy effects

Across the authenticated 18-sequence project (133 descriptor results):

- Exactly two Patches pass: Sequence R Track 1/d2, source ordinal 0, program 35,
  channel 3; Track 2/d3, source ordinal 1, program 16, channel 4.
- 56 other Patches on routing-accepted complete tracks are refused by this
  classifier. They retain their previous independently authorized exact-profile
  behavior, if any; this classifier is not a replacement or fallback.
- 33 descriptor tracks fail existing track-level routing gates. Their Patch
  counts are not inferred from partial walks or ambiguous ownership.
- Two Patches newly gain this bounded structural classification. No incidental
  extra acceptance or unexpected refusal was found.

Explicit regression checks retain six Ready and twelve PartiallySupported
sequences and unchanged inspection/profile evidence. No inclusion, saved-mute,
Patch policy in exact profiles, readiness, UI, or export authorization changed.

## Tests and validation

Nine focused tests cover synthetic acceptance/provenance, bank-tail/ASCII
refusals, pre-name/program refusals, payload/name length refusals, timing and
transition refusals (including initial ff60 length 8), track-level gates,
authenticated native reconciliation, exact corpus acceptance, and unchanged
readiness/profile evidence. Real fixtures are required and hash-checked;
missing evidence is never silently skipped.

Commands:

```sh
cargo test --test bounded_cc0_patch_translation -- --nocapture
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
git diff --check
```

Focused tests passed 9/9. The full suite passed 405 tests with zero failures
and two pre-existing ignored tests; formatting, strict Clippy, and whitespace
checks passed. Commit identity is recorded in the task handoff. Raw logs
remain under `/tmp/phoenix-cc0-*.txt`, outside the repository.

## Blocker consequence and limitations

B (Patch/bank translation) is resolved for Sequence R under this exact bounded
form at the semantic classifier/adapter boundary. Production use still needs
an appropriate source-order-preserving export integration. No universal bank
optionality, additional program values, other bank values, or unsupported event
forms are admitted.

I (inclusion/output scope) and A (generalized export authorization) remain
separate unresolved blockers. Sequence R is not made Ready by this change.
