# Bounded explicit Program entries in authenticated 120 recovery

CCA-004 implements the rule selected by [CCA-003](COMPATIBILITY_ENGINEERING_DECISION_CCA_003.md). It does not change the supported conductor envelope, sparse metadata association, cardinality, or complete-export authorization.

## Acceptance boundary

Within an independently owned ordinary 120-layout event body, the single-Instrument path accepts the existing bounded timing VLQ followed by explicit `C0` and exactly one seven-bit Program value. Timing is accumulated by the existing walker. `C0` is a canonical project tag; its low nibble does not override the channel supplied by the independently validated same-source Instrument/device route. The entry contributes one declared event and no context record.

The decoder accounts for the complete timing/status/value source range and resets compact continuation state. Another entry must independently satisfy the existing grammar. Missing/high-bit Program data, `C1`–`CF`, implicit Program continuation, overflow, unsupported successors, and unexplained trailing bytes remain refusals. The generic walker, Descriptor166 export path and Multi-Instrument path do not gain this acceptance.

`authority120` selects the crate-private walker only for the single-Instrument association selector; full routing and event-count proof must still succeed. `recovery120` hands the Program value and absolute position to the existing Program-only MIDI adapter with the proven Track route. No bank or Patch-name interpretation is invented.

## Evidence and validation

PM-R13 established the original-to-normalized bridge, preserved in the [durable Prologue checkpoint](PROLOGUE_POST_PM_R13_RECOVERY_RESEARCH_CHECKPOINT.md). Nine explicit entries in Frame 1, Frames 7-8 and Frame 11 correspond exactly to normalized bounded `FF 7C` Patch cores, preserving Program values, timing and all other body bytes. This is a structural rule, with no specimen names or offsets in production acceptance.

Focused tests cover Program values 0/127, timing, exact consumption and provenance, route/channel handoff, malformed variants, unchanged non-Program behavior, ownership/count refusal and conductor refusal. An opt-in read-only authentic regression pins both specimen hashes and verifies the nine-entry bridge, exact body consumption and same-source channels (3, 1, 3). It performs no MIDI export. Supply `PHOENIX_CCA004_ORIGINAL` and `PHOENIX_CCA004_NORMALIZED` to run the ignored bridge test.

The authentic scoped decoder succeeds for 70, 24 and 180 declared events respectively. Production Prologue recovery still refuses at sparse metadata association before this decoder; applying the prior observational metadata interpretation exposes the unchanged conductor Envelope blocker. No additional complete recovery or verified MIDI export is claimed.

Required validation: `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test`, focused Program tests, the opt-in authentic bridge test, and `git diff --check`.

CCA-004 validation passed: formatting, all-target/all-feature Clippy with warnings denied, full tests (594 passed, 9 ignored), seven focused Program regressions, the separately invoked authentic bridge regression, and diff whitespace checks. Preservation fingerprint checks matched all unrelated baseline repository files and 156 external artifacts/reports; HEAD, branch and index remained unchanged. No authentic MIDI exports, staging, commit or push occurred.
