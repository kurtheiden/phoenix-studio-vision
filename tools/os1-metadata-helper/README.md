# OS1 reserve metadata helper

This standalone macOS helper is retained for the bounded Phoenix 0.1
blind-validation intake route under the
[release-scope rebaseline](../../docs/OS1_TARGETED_RESPEC_RESUME_CHECKPOINT.md).
It is not Phoenix, a parser, a selection engine, or a reference tool, and does
not claim to implement the entire historical OS1 Topics 4–6 framework.
Permission, eligibility, neutral selection, concealment and independent review
remain governed by the [reserve-intake protocol](../../docs/OS1_RESERVE_INTAKE_PROTOCOL.md)
and [blind-validation protocol](../../docs/OS1_BLIND_VALIDATION_PROTOCOL.md).

## Commands and controlled paths

Build into an existing private external directory:

```text
tools/os1-metadata-helper/build.sh /absolute/private/path/phoenix-os1-metadata-helper
```

The existing build script temporarily generates `Sources/BuildIdentity.generated.swift`
and removes it on exit; do not run it over a pre-existing file at that path.
It also uses a Swift module cache under the temporary directory.

The interfaces remain:

```text
phoenix-os1-metadata-helper enumerate --root ROOT --map-output MAP.json
phoenix-os1-metadata-helper inspect --map MAP.json --record-output RECORD.jsonl
```

Use reviewed, resolved, local private directories with separate source and
output locations. Output parents must already be private mode-0700 directories;
outputs are created exclusively as mode 0600, never overwritten. Keep outputs
outside the source root, repository and cloud-managed storage. Avoid symlinked,
ambiguous or equivalent-path arrangements: these are operating restrictions,
not a claim that the helper resolves every filesystem alias or path case.
Candidate-only directories must exclude known archives, disk images and other
noncandidate regular files, except deliberately identified synthetic test cases.
The helper hashes regular files; it does not classify their contents.

## Privacy and source integrity

Enumeration is immediate-child-only and no-follow. Inspection retains one
`O_RDONLY | O_NOFOLLOW | O_CLOEXEC` candidate descriptor and performs two complete
sequential CryptoKit SHA-256 passes with stat and path-identity checks.
Candidate access is read-only; the helper writes its separate evidence outputs.
FinderInfo output contains only Type/Creator bytes 0–7. Resource-fork attribute
length may be queried internally to determine presence; neither length nor fork
contents are emitted, and fork contents are not read. Arbitrary xattr names and
values are not enumerated or read; `OTHER_XATTRS_PRESENT` is omitted.

Intake must not expose candidate strings/previews, parser results, reference
results, resource-fork contents or arbitrary private metadata. Success emits
nothing on stdout/stderr; failure emits a fixed error without paths or OS text.
MAP includes names and root paths: it is private evidence, not public output.
Do not use Finder previews, content classifiers or Phoenix during metadata intake.

Around later authorized Phoenix observation and export, preserve before/after
candidate identity, size and complete-data-fork hash evidence. Unexpected
changes invalidate the result or require explicit assessment before acceptance.
Normal access-time effects alone do not demonstrate Phoenix source mutation.
The helper is not a filesystem snapshot and does not prove historical identity.

## Acceptance and freeze

Output-file existence does **not** establish successful completion. Check the
actual exit status, readable saved records, expected MAP entry count/IDs and
row outcomes. `INSPECTED` rows require verified integrity; explicit refusals
must remain refusals. Nonzero execution can leave a partial RECORD with an
`ERROR` row: preserve it as failure evidence, never successful intake. Missing,
truncated or altered records are not accepted. Do not silently retry failures.

Privately bind the exact MAP and RECORD files, their hashes, execution outcome
and tool/build identity together for each run. MAP artifact IDs restart and are
not globally unique; RECORD alone does not bind itself to a particular MAP.
Keep prior outputs intact and verify saved bytes before accepting them.

`OS1_METADATA_HELPER_MAP_V1` and `OS1_METADATA_HELPER_RECORD_V1` are the helper's
existing formats. They do not claim conformance to fuller later OS1 schemas.
The Python suite's acceptance checks demonstrate caller obligations; they are
not a new production schema or a complete reusable evidence validator.

Metadata intake completion does **not** freeze Phoenix's later blind observation.
Separately save, check, digest/identify, accept and freeze the complete Phoenix
observation before the reference holder reveals reference information. Record
that ordering, preserve failures and compare only after the freeze.

## Synthetic-only verification and rehearsal

Until separate candidate-access authorization, use only test-generated synthetic
inputs. Never blindly run an unrestricted repository test suite: some Phoenix
tests reference authentic material. The helper's focused procedure is:

```text
tools/os1-metadata-helper/test.sh /absolute/private/path/phoenix-os1-metadata-helper
```

Its fixtures are generated in a disposable private temporary directory. Helper
subprocesses have a 20-second timeout. Non-mutation cases check source bytes,
size and successfully installed metadata. Tests deliberately changing sources
are labelled as test-induced changes, not helper mutation. The suite reports
PASS/FAIL separately from UNAVAILABLE coverage. Missing metadata setup or a
host bypassing permission denial is not a passed test of that behavior.
Timing-dependent in-read mutation/replacement coverage is explicitly unavailable;
deterministic between-observation hash comparisons do not claim race coverage.
Review unavailable coverage before approving any subsequent real use.

For the later rehearsal, use fictional provenance and separate synthetic
source/output/reference locations. Exercise enumeration, complete intake
acceptance, mock neutral selection, Phoenix observation of synthetic input,
source rechecks, saved observation acceptance/freeze, then simulated reference
reveal/comparison. Conservative Unsupported is valid; do not add a fake production
export profile. Test alias/nonregular refusal, source identity changes, partial
failure evidence, altered records and output collisions. A premature simulated
reveal makes the exercise nonqualifying. Preserve source and existing output
bytes except explicitly declared test mutations.

Passing focused helper tests does not itself complete that rehearsal or permit
real intake. Before real use, identify the reviewed source/binary, toolchain,
build command, test outcomes (including unavailable coverage), private operating
boundary and independent approval. Candidate access and blind execution remain
separate explicit gates.
