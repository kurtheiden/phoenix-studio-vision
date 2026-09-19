# OS1 metadata helper independent review

Date: 2026-09-19. Verdict: **BLOCKED for authentic reserve intake**. This is a synthetic-only implementation review, not candidate selection or blind validation. No authentic reserve path, project, reference, or project content was accessed.

## Authority and identity

Review baseline: HEAD, main, and origin/main `7ade3c6426932bcc96802a8a9ad175443772cf7a`; staging empty. The existing modified `docs/ROADMAP.md` and `docs/DECISIONS.md` are working-tree planning updates, not new committed approval. The two existing untracked research documents were left untouched.

The governing authority is [OS1_BLIND_VALIDATION_PROTOCOL.md](OS1_BLIND_VALIDATION_PROTOCOL.md), followed by [OS1_RESERVE_INTAKE_PROTOCOL.md](OS1_RESERVE_INTAKE_PROTOCOL.md). The accepted 0.1 boundary is in [OPEN_SOURCE_RELEASE_PLAN.md](OPEN_SOURCE_RELEASE_PLAN.md). The helper README documents operating restrictions; it is not candidate-access authorization. The full OS1 Topics 4–6 design is outside this review.

| Reviewed item | SHA-256 |
|---|---|
| `tools/os1-metadata-helper/Sources/main.swift` | `55976c0c758a6c29799d94afce472f8cfccd5e8886b04f932098841b4e2d89ef` |
| `tools/os1-metadata-helper/build.sh` | `ad83093d37fe70b2b153f8a245f4fe01c739f48b3434afb14a22f6655d6c6c56` |
| `tools/os1-metadata-helper/test.sh` | `bd431c83a1aa3b01540c6164ca63bf365bc6be42b41aded04061a1ac26bc529a` |
| `tools/os1-metadata-helper/Tests/test_helper.py` | `f381b508dc764b90674ab3ba721924276c79d079102c93277fb766f069201568` |
| Synthetic review binary, basename `phoenix-os1-metadata-helper` | `0ca58226b7797306f02dc1b4a5ed267f61beda6d0c106e694f52b12c84fb8899` |

The review binary was built at `/private/tmp/phoenix-os1-helper-review.i4FbXr/phoenix-os1-metadata-helper` with Apple Swift 6.2.4, target `arm64-apple-macosx15.0`, on macOS 15.7.5. A second build with the same basename in another directory had the same SHA-256. A build with a different output basename had a different Mach-O identity and hash; future intake must bind the exact executable bytes, not infer identity from source alone. The temporary review binary is not an approved or durable intake binary.

## Implementation and synthetic verification

`enumerate` is immediate-child-only, uses no-follow stat for entries, and emits a private MAP with neutral IDs plus basename/root mapping. `inspect` opens regular files with `openat` and `O_NOFOLLOW`, reads only FinderInfo Type/Creator values, probes resource-fork presence, and performs two complete sequential data-fork SHA-256 passes with stat and path identity checks. It does not enumerate other xattr names or emit file bytes. Outputs are exclusive private files; ordinary failure messages contain fixed codes without paths. Nonregular entries and Finder `alis` type are refused. These properties were checked against the source and the focused synthetic suite.

The helper's `OS1_METADATA_HELPER_MAP_V1`, `OS1_METADATA_HELPER_RECORD_V1`, and `OS1_METADATA_HELPER_ERRORS_V1` are local formats, not the later full OS1 schemas. MAP holds sensitive paths and names and must remain private. RECORD rows contain neutral IDs and statuses; a nonzero run may publish a partial RECORD with an `ERROR` row. A saved file's existence is not success. The Python acceptance check also requires success exit status, row/ID accounting, verified inspected rows, and an independently computed saved-RECORD digest; it is test code, not a production validator.

`test.sh` reported **PASS: 57 checks, 0 failures, 1 unavailable**. The unavailable case is an overlapping in-read mutation/replacement race; deterministic before/after changes do not cover it. The tests cover synthetic source preservation, FinderInfo truncation to eight bytes, absent/present resource-fork status where supported, symlink/directory/FIFO and alias refusal, nonrecursive enumeration, two-pass digest, output privacy/collisions, MAP tampering, root replacement, partial failure evidence, and fixed error grammar. No broad repository test suite or authentic fixture was run.

Focused independent probes on synthetic files additionally established:

1. A regular file named `archive.zip` is `INSPECTED` and hashed. The helper cannot classify archive or disk-image content within the metadata firewall. Its candidate-only-root prerequisite must be established and independently reviewed before use.
2. `inspect` accepts a private MAP whose `helper_binary_sha256` was replaced with an all-zero value and whose unkeyed `canonical_map_sha256` was recomputed. Thus that MAP digest detects accidental change but does not authenticate the MAP or bind it to the executable. The operator must separately freeze and verify exact MAP bytes, executable SHA-256, source/build identity, command/exit outcome, and RECORD bytes before accepting intake evidence.

The source also relies on reviewed, resolved local path arrangements: lexical output/root checks do not establish physical separation through every symlinked ancestor or equivalent path. The candidate-only root, private output parent, path resolution, and quarantine are operating controls, not properties proved solely by the helper.

## Blocking conditions before authentic metadata intake

- An independent human acceptance must approve this exact source/build/test identity and a durable frozen binary; the temporary review build alone is insufficient.
- Freeze a concrete private command and evidence-acceptance procedure that verifies the actual binary digest, saved MAP digest and identity, saved RECORD digest, successful exit, complete row/ID accounting, and source preservation. It must reject any partial or changed output without silent retry. A self-reported MAP hash is insufficient.
- Establish the protocol's candidate-only root attestation, no-reference quarantine, owner/permission and later-reference attestations, resolved non-symlinked local paths, private non-cloud output, and independent role boundary before supplying a reserve path. Known archives, disk images, packages, aliases, and other noncandidate objects must be excluded or refused without opening their content.
- Separately authorize and review any actual intake execution. This review grants no reserve enumeration, candidate selection, Phoenix inspection, reference reveal, profile tuning, or Gate 1/5 pass.

The synthetic implementation is promising within those constraints, but the current helper and test result do not independently satisfy the complete authentic-intake acceptance boundary. No production parser or MIDI-export change is indicated by this review.
