# OS1 metadata helper durable binary review candidate

Date: 2026-09-20. Purpose: synthetic-only evidence for independent human acceptance. **Authentic reserve intake remains BLOCKED.** This record does not authorize candidate access, selection, Phoenix inspection, or reference reveal.

## Observed identity and build

- At the start, `HEAD`, `main`, and `origin/main` were `e76b0671c051e7413e6b32251ea74a867984a774`; branch `main`; staging empty. The four unrelated local files had their previously recorded SHA-256 values and were left untouched.
- Reviewed source `tools/os1-metadata-helper/Sources/main.swift`: SHA-256 `55976c0c758a6c29799d94afce472f8cfccd5e8886b04f932098841b4e2d89ef`.
- Build script `tools/os1-metadata-helper/build.sh`: SHA-256 `ad83093d37fe70b2b153f8a245f4fe01c739f48b3434afb14a22f6655d6c6c56`.
- Test script `tools/os1-metadata-helper/test.sh`: SHA-256 `bd431c83a1aa3b01540c6164ca63bf365bc6be42b41aded04061a1ac26bc529a`; focused test source `tools/os1-metadata-helper/Tests/test_helper.py`: SHA-256 `f381b508dc764b90674ab3ba721924276c79d079102c93277fb766f069201568`.
- Acceptance runner `tools/os1-metadata-helper/acceptance/accept_inspect.py`: SHA-256 `16f1ce215e3100012cfb469e31f804e120b31a6dad3019b2535da8e2934dce6c`; its test file `test_accept_inspect.py`: SHA-256 `6e4c28841b78f41d64c2c42388334d753349a15beccf76cde9f585bb26a1ef6f`.
- Toolchain: Apple Swift 6.2.4 (`swiftlang-6.2.4.1.4 clang-1700.6.4.2`), target `arm64-apple-macosx15.0`; macOS 15.7.5 (build `24G624`).
- Exact durable build command, run from repository root: `tools/os1-metadata-helper/build.sh /Users/kurtheiden/.local/share/phoenix-os1-review-candidates/2026-09-20/phoenix-os1-metadata-helper`. Exit status: `0`. The script's temporary `Sources/BuildIdentity.generated.swift` was absent before and after the build.
- Executable: `/Users/kurtheiden/.local/share/phoenix-os1-review-candidates/2026-09-20/phoenix-os1-metadata-helper`; SHA-256 `0ca58226b7797306f02dc1b4a5ed267f61beda6d0c106e694f52b12c84fb8899`. Verified regular Mach-O arm64 file, 136280 bytes, owner `kurtheiden`, mode `0700`; immediate review directory mode `0700`. The resolved path equals the written path, with no symlinked ancestors. The location is on the local APFS Data volume, outside the repository and named cloud storage roots. Independent review must still confirm it is not subject to a sync service or unintended access before any authentic use.

## Synthetic verification

- Exact focused command: `tools/os1-metadata-helper/test.sh /Users/kurtheiden/.local/share/phoenix-os1-review-candidates/2026-09-20/phoenix-os1-metadata-helper`. Exit `0`: `PASS`, 57 passed, 0 failed, 1 unavailable. The unavailable case is overlapping in-read mutation/replacement; the race was not established and is not claimed covered.
- Acceptance test command: `python3 -m unittest discover -s tools/os1-metadata-helper/acceptance -p 'test_*.py'`. Exit `0`: 12 tests, `OK`. These are synthetic runner tests, not authentic intake.
- Reproducibility comparison: the same build script exited `0` for `/private/tmp/phoenix-os1-compare.4kQ7cC/phoenix-os1-metadata-helper`, a separate private synthetic build directory with the same executable basename. Both SHA-256 digests were `0ca58226b7797306f02dc1b4a5ed267f61beda6d0c106e694f52b12c84fb8899`; `cmp -s` exited `0`. This establishes matching bytes for these two builds on this toolchain, not general reproducibility across hosts or toolchains. The comparison binary is temporary and is not the durable review candidate.

## Outstanding human and operating gates

The [independent review](OS1_METADATA_HELPER_INDEPENDENT_REVIEW_2026_09_19.md) and [operator checklist](OS1_MINIMUM_0_1_BLIND_INTAKE_OPERATOR_CHECKLIST.md) retain the **BLOCKED** verdict. An independent human must accept this exact source, build, tests, unavailable coverage, and durable executable identity. Before any authentic path is supplied, the operators must freeze concrete private commands and evidence handling; verify storage, resolved paths, permissions, candidate-only root, reference quarantine, provenance and later-reference attestations, and independent roles; and separately authorize the actual execution. A successful synthetic test or matching build digest does not meet those conditions. Later intake, selection, provenance lock, Phoenix observation, and reference reveal each retain their protocol gates.
