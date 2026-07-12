# Contributing to Phoenix

Phoenix is in an exploratory phase. Contributions should be small, evidence
based, and avoid asserting undocumented file-format details as fact.

Before opening a substantial change, start a discussion describing the
observation, supporting samples, and proposed scope. Do not submit proprietary,
personal, or otherwise non-redistributable sample files.

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

