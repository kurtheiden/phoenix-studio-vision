# Contributing to Phoenix

Phoenix is in an exploratory phase. Contributions should be small, evidence
based, and avoid asserting undocumented file-format details as fact.

Before opening a substantial change, start a discussion describing the
observation, supporting samples, and proposed scope. Do not submit proprietary,
personal, or otherwise non-redistributable sample files.

## Required pre-investigation reconciliation

Before proposing any experiment, forensic investigation, parser change or
recovery expansion, consult the [current research-status cross-reference](docs/DECISIONS.md#current-research-status-cross-reference)
and record:

1. The exact unanswered question and expected benefit to general-purpose recovery.
2. Relevant research IDs, experiments, reports and original evidence locators.
3. Later code, tests, PRs and decisions that supersede earlier recommendations.
4. What is established and what remains genuinely unresolved.
5. Whether known private evidence is inaccessible in the current environment.
6. Why the existing evidence cannot answer the question.
7. The precise new evidence or action needed, with falsifiable acceptance and
   refusal criteria.

If earlier private work is known but unavailable, require a status check before
proposing a repeated experiment; do not equate inaccessible evidence with an
uninvestigated question. Preserve historical reports unchanged. Any implementation
that resolves a tracked gap must update its current-status pointer in the same
change, linking the evidence to the implementation and remaining limits. Keep
private music, authentic source bytes and private reports out of public changes.
This checklist grants no permission to access private evidence or run experiments.

## Development checks

Run these commands before submitting a change:

```sh
cargo fmt --check
cargo test
```

Code should preserve input files, handle malformed data without panicking, and
include tests for new behavior. Document architectural or project-policy
changes in `docs/DECISIONS.md`.

By contributing, you agree that your contribution is licensed under the
project's current license. Licensing will be reviewed before significant
outside contributions are accepted.

