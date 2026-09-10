# Independent approval review of OS1 Topic 3

## Review identity and baseline

Review date: **2026-09-10** (Europe/Lisbon).
Reviewer role: **INDEPENDENT REVIEWER**, not the Topic-3 author.

Reviewed document: [OS1_ENUMERATION_AND_MAP_PRODUCTION_V1.md](OS1_ENUMERATION_AND_MAP_PRODUCTION_V1.md),
abbreviated T3 below.

- Exact reviewed line count: **1154**.
- Exact reviewed SHA-256:
  `09aa74d6d0efbdc0e438bc1662f5b910b4b66dfbbeeb03aeb380add3d9a8a0b4`.
- HEAD, main, and origin/main:
  `cb25005545bf5f8c464e2edc3930174efb40547c`.
- Baseline subject: `Preserve Topic 3 authority clarification`.
- Divergence: **0 ahead / 0 behind**.
- Staging: **empty**.

These identities were verified before substantive review. Existing modified
DECISIONS.md and ROADMAP.md and untracked working artifacts were preserved.
The review target remains untracked; this review does not commit or preserve
the approved pair in Git.

## Exact authority set and independence

The following eleven documents were read independently in full. All eleven
working files were verified byte-identical to this baseline's committed files.
These references identify the exact authority set used; the checkpoint is
status/handoff evidence and does not independently amend design authority.

| Reference | Document |
|---|---|
| R | [OS1_DESIGN_AUTHORITY_RECONCILIATION.md](OS1_DESIGN_AUTHORITY_RECONCILIATION.md) |
| RR | [OS1_DESIGN_AUTHORITY_RECONCILIATION_REVIEW.md](OS1_DESIGN_AUTHORITY_RECONCILIATION_REVIEW.md) |
| T1 | [OS1_CANONICAL_EVIDENCE_SCHEMAS_AND_ERRORS_V1.md](OS1_CANONICAL_EVIDENCE_SCHEMAS_AND_ERRORS_V1.md) |
| T1R | [OS1_CANONICAL_EVIDENCE_SCHEMAS_AND_ERRORS_V1_REVIEW.md](OS1_CANONICAL_EVIDENCE_SCHEMAS_AND_ERRORS_V1_REVIEW.md) |
| A1 | [OS1_TOPIC1_ERROR_TAXONOMY_CLARIFICATION_FOR_TOPIC2_V1.md](OS1_TOPIC1_ERROR_TAXONOMY_CLARIFICATION_FOR_TOPIC2_V1.md) |
| A1R | [OS1_TOPIC1_ERROR_TAXONOMY_CLARIFICATION_FOR_TOPIC2_V1_REVIEW.md](OS1_TOPIC1_ERROR_TAXONOMY_CLARIFICATION_FOR_TOPIC2_V1_REVIEW.md) |
| T2 | [OS1_CUSTODY_AUTHORITY_LIFECYCLE_AND_VALIDATOR_REGISTRY_V1.md](OS1_CUSTODY_AUTHORITY_LIFECYCLE_AND_VALIDATOR_REGISTRY_V1.md) |
| T2R | [OS1_CUSTODY_AUTHORITY_LIFECYCLE_AND_VALIDATOR_REGISTRY_V1_REVIEW.md](OS1_CUSTODY_AUTHORITY_LIFECYCLE_AND_VALIDATOR_REGISTRY_V1_REVIEW.md) |
| C | [OS1_TOPIC3_AUTHORITY_CLARIFICATION_U3_001_U3_002_V1.md](OS1_TOPIC3_AUTHORITY_CLARIFICATION_U3_001_U3_002_V1.md) |
| CR | [OS1_TOPIC3_AUTHORITY_CLARIFICATION_U3_001_U3_002_V1_REVIEW.md](OS1_TOPIC3_AUTHORITY_CLARIFICATION_U3_001_U3_002_V1_REVIEW.md) |
| Checkpoint | [OS1_TARGETED_RESPEC_RESUME_CHECKPOINT.md](OS1_TARGETED_RESPEC_RESUME_CHECKPOINT.md) |

The approved clarification has 512 lines and SHA-256
`6763a37804c0f57cc8e637746990844d92546d564ae20dd52c35f530b7e8d09c`.
Its approving review has 318 lines and SHA-256
`2ebb33947885ceb87ecf3341acd2224c61c2a65fb8f60be5fb022b45324ad838`.
The checkpoint's earlier draft identity is historical handoff context; the
1154-line identity above controls this review.

Also read in full, as review evidence only:
[OS1_TOPIC3_BLOCKER_REVIEW_U3_001_U3_002.md](OS1_TOPIC3_BLOCKER_REVIEW_U3_001_U3_002.md).
Its identity was verified before review: **314 lines**, SHA-256
`d7aeb9277550a20c30255e41fa62718d924706dad1e9d54152b62d40975f3639`.
That review explains the former blockers; it supplies no replacement authority.

I did not author or revise T3. I did not accept T3 section 21's self-audit,
earlier broad sufficiency claims, or the clarification's author closure table
as evidence of correctness. Conclusions below were re-derived from the exact
normative contracts, their approved qualifications, and T3's operative rules.
No lost temporary document, implementation behavior, or prior conversation
reasoning supplied a missing rule. Only this new review document was authored.

## Review method

I traced each input and derived value to its supplying operand, validated
predecessor, syscall observation, or expressly delegated Topic-3 decision.
I compared the command byte-for-byte with C, independently counted the MAP
fields, registry rows, and original-plus-amended errors, and checked error
names used by T3 against that closed vocabulary.

I then traced the successful path and first-failure exits through acquisition,
enumeration, rechecks, construction, publication, and the separate binding
handoff. Adversarial cases included absent operands, crossed roots, wrong raw
helper bytes, duplicate/invalid names, overflow, hard links, name replacement,
same-object mutation, empty MAP, digest-domain confusion, temp/final occupancy,
rename collision, and failures after publication. This is design analysis,
not execution of an OS1 implementation or Topic-6 fixtures.

## Findings

| Severity | Count | Disposition |
|---|---:|---|
| BLOCKER | 0 | No implementation-critical contradiction, ambiguity, or authority gap found. |
| IMPORTANT | 0 | No meaningful correctness or clarity defect requiring correction found. |
| MINOR | 0 | No editorial finding recorded. |

Unresolved **Topic-3 implementation-critical decisions: 0**.
No new cross-topic clarification is required for this reviewed design.
The following cited determinations explain that conclusion independently.

## U3-001 verdict

**APPROVED — correctly and completely integrated.** T3 section 4.1,
lines 108–130, matches C section 3.1's sole command exactly:

```text
phoenix-os1-metadata-helper enumerate \
  --executable-authorization FILE \
  --preflight FILE \
  --path-map FILE \
  --root ROOT \
  --implementation-review-root ROOT \
  --custody-record-root ROOT \
  --private-output-root ROOT \
  --helper-source FILE \
  --helper-executable FILE \
  --map-output FILE
```

The original authorization, preflight, path-map, root, and map-output options
retain their meanings and relative order. Exactly three root and two raw-file
options are added. All ten occur once, with one operand each; aliases,
defaults, reordered options, extra inputs, and fallback forms are rejected.

T3 sections 4.2–4.3, lines 134–205, preserve C sections 3.2–3.4's assignments:
authorization and both raw files under review; preflight/path map under
custody; final MAP under private; explicit --root as candidate. Absolute-byte
grammar and exact lexical immediate-child relationships precede filesystem
access. Root acquisition order is candidate, review, custody, private, matching
the signature and T2 section 4.2. Every subsequent child lookup uses its
retained assigned-root descriptor.

Both current raw files are actually acquired and completely hashed, source
before executable, through T2 section 5.3's open/stat/kind/read and stability
checks. V02-028 compares those measurements with the corresponding authorization
digests in the same order. Expected digest strings cannot replace measurement.
Authorization's placement is correlated through preflight/path map; T3
section 5.1 does not invent an authorization root field.

No review, identities, build, custody raw input, or Git observation was added.
PATH, CWD, environment, process-image discovery, serialized-locator lookup,
and conventional filename/digest searches are expressly excluded. The
measured executable is the explicit file, not an asserted running-process
measurement; that procedural trust limit is preserved at T3 lines 1090–1095.

## U3-002 verdict

**APPROVED — correctly and completely integrated.** T3 sections 4.4 and
18.2–18.3, lines 209–232 and 974–1029, implement C section 4's corrected
V02-030, including its exclusive comparison ownership and dependencies.

The row remains V02-030, observational, in its original numeric position and
private-root assignment. It computes over the exact canonical entries value,
from opening `[` through closing `]`, without LF. Empty entries hash `[]`.
The observation is in memory, not a new MAP member or external authority.

Comparison occurs only when the command explicitly supplies binding, after
that binding passes V02-014, and uses exactly binding.entries_sha256.
Inequality is EVIDENCE_DIGEST_MISMATCH. V02-014/015/029/031 cannot perform this
same comparison earlier or substitute a correlation error; their other
self/reference-digest and correlation obligations remain intact.

Independent signature inspection confirms the seven-command set in T3
lines 980–986: bind-map computes only; freeze-map-review, authorize-run,
reserve-run, validate-binding, validate-freeze-review, and
validate-run-reservation compute and compare. The remaining nine T2 commands
skip the row. Enumerate also skips it because its MAP is output, not input.

T3 section 13 step 7's producer-side computation is expressly separate from
V02-030, ephemeral, and not a fabricated binding comparison. bind-map later
recomputes independently from supplied MAP bytes. Its binding is constructed
only after V02-041, copies the retained entries digest, and is validated at
V02-042 before publication. No future output or hidden handoff is consumed.

## Independent algorithm trace

| Phase | T3 citation | Independent determination |
|---|---|---|
| Invocation and lexical paths | sections 4.1–4.2; 16 steps 1–2 | Exact ten-option contract and all path relationships precede filesystem access. No operand or root is inferred. |
| Root acquisition | sections 4.3, 6 | Four retained no-follow chains, role-sensitive projections, immediate restats, terminal separation and nonnesting follow T2 sections 3–4. Candidate root metadata acquisition grants no child access. |
| Serialized inputs | sections 4.3, 5.1; 16 steps 3–4 | A, F, P each finish V02-005..014 before the next. Enumerated F/A and P/F equalities use only real fields. Fresh complete roots are compared with correlated frozen components at V02-017. |
| Current-helper gate | sections 4.3, 5; 16 step 5 | Source and executable measurements exist before V02-028. V02-022..027 and 030..032 are inapplicable; no Git, review, or future MAP is consumed. V02-029 follows measured equality. |
| Candidate-access/output gate | section 5.2; 16 step 6 | Retained private parent is rechecked and authorized; final absence is observed before the inventory. This local precondition does not move or replace numbered final rows. |
| Immediate inventory | sections 6–7; 16 steps 7–8 | Sequential duplicate-descriptor directory streams are rewound, errno distinguishes EOF from error, copied raw names are bounded, streams are closed, and original root descriptor is retained. No recursion or child open. |
| Ordering and classification | sections 7–9, 12 | Unsigned decoded-byte lexicographic order with shorter-prefix-first; no Unicode/locale/base64 collation. Successful no-follow file-type bits select one of eight kinds. Hard links remain distinct names/IDs. |
| Local rechecks | section 11; 16 step 9 | A complete second inventory must match before sorted child restats; each child identity precedes protected state. Candidate directory identity/mode and local mtime/ctime are then checked. No retry, omission, or partial MAP. |
| Final registry checks | section 5.1; 16 step 10 | V02-033 root stats, 034 non-root stats, 035 identities, then 036 protected fields are separate ordered phases. V02-037/040 follow; reservation rows are skipped. |
| Timestamp | section 13; 16 step 11 | Exactly one CLOCK_REALTIME sample after applicable checks, with signed i64 seconds and bounded integral nanoseconds. No normalization, chronology requirement, or retry. Failure precedes construction. |
| Construction and digests | section 13 and field table 13.1 | Every field has a defined source; primary ID, self-digest, complete-object digest, and array digest have distinct explicit domains. No circular dependency exists. |
| V02-042 | lines 700–705; 16 step 12 | Completed shape, values, sources, bounds, roots, count/order/IDs, canonical bytes and digests are checked before any temp creation. Internal construction defects use INTERNAL_INVARIANT_FAILED. |
| Temporary publication | section 14 steps 1–6 | Recheck availability; exclusive deterministic 0600 temp; capture its descriptor identity; write, fsync, complete reread; then repeat candidate, root/input, parent and absence checks. No MAP reconstruction after changed observations. |
| Final publication | section 14 steps 7–11 | Exclusive same-parent rename cannot overwrite. Directory fsync, final file/identity, dedicated final root, and no-follow reopen/byte validation must all pass before success. |
| Return and handoff | sections 14, 17–18 | Empty success output and exit 0 only after final verification. Separate explicit bind-map invocation owns binding; enumerate creates no freeze/run/reservation/RECORD/completion/event. |

No phase needs candidate data bytes, an unavailable predecessor, or a future
output. T3 section 16 agrees with the more detailed sections; the early local
availability gate and later publication rechecks are not extra registry rows.

## Enumeration, race, and error adjudication

T3 sections 7–8 include every immediate non-dot entry, including hidden and
invalid-UTF-8 names, without extension, format, content, or inode filtering.
Malformed/duplicate records fail ENUMERATION_FAILED; a 4097th valid non-dot
entry fails ENTRY_LIMIT_EXCEEDED. Names are never truncated or deduplicated.
IDs are index plus one, R-000001 through R-004096. An empty directory produces
a complete MAP with entry_count 0 and entries []; the zero-byte RECORD rule
does not apply to MAP.

The identity/state distinction is correct. T3 section 11, lines 552–576,
assigns successful later device/inode/kind discontinuity to
OBJECT_CHANGED_OR_REPLACED before comparing mode/size/mtime/ctime. Equal
identity with changed protected state uses RACE_OR_MUTATION_DETECTED. A failed
stat, including ENOENT, supplies no comparison and uses OBJECT_STAT_FAILED.
Namespace inequality detected earlier by the complete inventory pass instead
uses RACE_OR_MUTATION_DETECTED. These are distinct ordered observations.

T1 mandatory partition rule 6 and T1R's candidate-stat approval control over
the older taxonomy table's broad ENUMERATION_FAILED description. T3 correctly
uses OBJECT_STAT_FAILED for candidate stat. Root stat remains ROOT_CHAIN_INVALID;
dedicated post-publication root verification uses FINAL_ROOT_VERIFICATION_FAILED.
Final-basename absence stat targets an ordinary non-root object, so its
non-ENOENT syscall failure correctly remains OBJECT_STAT_FAILED.

T3 sections 5.1, 11, and 15 preserve stat-before-comparison and identity-before-
state precedence. Raw open/wrong-kind, descriptor-stat, complete-read, and
measured-hash mismatch follow A1 sections 5–6. Supplied evidence retains T1's
parse precedence. Temp acquisition/reread/content failures and final
file/root/reopen failures retain their distinct target/stage partitions.
Preobserved occupancy is OUTPUT_ALREADY_EXISTS; attempted rename collision is
FINAL_RENAME_FAILED. Ordinary I/O is never an internal-invariant fallback.

The 0600 exclusive temp and exclusive rename prohibit replacing any occupant,
including symlinks. T3 section 17 correctly leaves pre-rename residue
nonauthoritative and named post-rename residue unavailable for overwrite or
resumed publication. Fresh invocation performs a fresh gate and timestamp;
deterministic collision is a failure, not permission for cleanup or name search.

Race protection is observation-based, as T2 section 1 requires. T3 lines
578–589 and section 20 explicitly exclude atomic-snapshot, ABA-detection,
hostile-kernel, and content-lease claims. The last candidate recheck precedes
remaining pre-rename checks; later changes are not falsely certified absent.
Retained-descriptor root semantics are inherited from T2, not replaced by
pathname authority. No additional TOCTOU guarantee or hidden traversal is
required outside that approved model. Topic 4 must establish its own later
candidate acquisition; MAP serializes no historical child inode.

## Compatibility and completeness verdicts

| Requested verdict | Result | Controlling evidence and reason |
|---|---|---|
| Topic-1 compatibility, including A1 | PASS | T1 MAP/components/canonical and digest rules, mandatory partitions, A1 sections 5–8; T3 sections 4, 9, 13, 15 retain shapes, byte domains, error applicability and public grammar. |
| Topic-2 compatibility | PASS | T2 sections 3–5, 9, 14–15; C sections 3–4 and 7; T3 sections 4–6, 14, 18 retain root, lifecycle, timestamp and publication contracts with only the approved narrow amendments. |
| MAP schema | PASS: exactly 17 fields | T3 lines 662–678 match T1's ordered MAP table field-for-field; entries remain exactly record_id, basename_b64u, kind. No entries_sha256, child identity, size/time, or content hash is added. |
| Validator registry | PASS: exactly 42 rows | Independently counted T2 section 15's unique contiguous V02-001..042 rows. T3 applies/skips them, uses C's corrected 030, and adds local producer work without another row. |
| Public error vocabulary | PASS: exactly 59 members | Independently counted T1's 55 plus A1's four distinct additions. T3's runtime errors are members of that union; no new member or changed stderr/exit grammar exists. |
| Privacy/candidate access | PASS | T3 sections 3, 9–10 prohibit candidate-child descriptors, content reads/hashes, FinderInfo/xattrs/resource probes and readlink. Only names and allowlisted ephemeral stat observations are acquired. |
| Determinism | PASS | T3 sections 6–8, 13, 15–16, 20 fix stream handling, raw-byte order, IDs, sources, digest projections, clock use and first-failure ordering. Same validated values, observations and timestamp produce the same bytes. |
| Algorithm completeness | PASS | The trace above reaches successful final verification and explicit custody handoff without a missing field source, circular dependency, unresolved alternative, or prohibited observation. |

T3's map_id decision is within delegated scope: T1's MAP table explicitly
assigns map_id to Topic 3, whereas T2 section 2.1 lists only eight Topic-2
primary IDs. T3 lines 628–643 define its domain tag and LF-bearing projection
without changing those eight rules. Self-digest omits only object_sha256;
complete-object digest includes it and LF; entries digest covers only the
bracketed array without LF. These domains cannot substitute for one another.

T3 section 18.1's strict decoded-name ordering is an array-value constraint
under T1's explicit delegation of MAP ordering to Topic 3. Its V02-012
EVIDENCE_SCHEMA_INVALID result does not change V02-013's object-key/lexical
canonical-byte errors. This adds neither a schema field nor cross-topic
authority. Binding can validate conformity to ordering without proving the
historical completeness or syscall behavior of enumeration.

The existing private candidate_root_path_b64u is correctly derived from exact
--root bytes after identity equality. It participates in integrity without
authorizing a root; base64url is not claimed as encryption. Additional review,
custody, output paths and ephemeral child observations are not serialized.
Public stdout/stderr cannot expose paths, names, hashes, IDs, or OS diagnostics.

Candidate content hashing and RECORD remain Topic 4; completion/events and
terminal responsibility remain Topic 5; fixtures, fault injections, expected
vectors and conformance assertions remain Topic 6. T3 sections 3, 12, 18–20
consume their boundaries without beginning their algorithms. No new authority
service, hidden state, signature, lookup, candidate access permission, or
cross-topic responsibility is invented.

## Independently derived gap dispositions

| Gap | Review disposition | Comparison with T3 section 19 |
|---|---|---|
| G-006 | CLOSED through approved C, correctly integrated by T3 | Agree. R/RR's preserved five-option syntax is narrowly amended by C; T3 supplies exactly the approved complete ten-option contract. |
| G-007 | CLOSED BY TOPIC 3 upon this approval | Agree. T3 sections 4–16 specify the gate, inventory, race checks, construction, publication and ordered failures. |
| G-008 | CLOSED BY T1 plus TOPIC 3, consuming T2/C | Agree. T1 fixes shape/bytes; T3 fixes source population, current root/path semantics, ordering/IDs, digests, validation/publication and explicit binding handoff. |
| G-014 | PARTIALLY CLOSED | Agree. T2 placement/identity authority and T3 MAP enforcement are closed. RECORD producer enforcement remains Topic 4; completion/event enforcement remains Topic 5. |
| G-028 | PARTIALLY CLOSED | Agree. T1/A1 close errors, T2/C close the 42-row registry, T3 closes enumerate integration. Topic 6 retains fixtures/assertions/conformance definitions; Topics 4–5 retain their operation-specific integrations. |

These results follow R's gap definitions, RR section 8's delegations, T1's
cross-topic table, and T2 sections 19–20. No later-topic gap is over-closed.
Zero unresolved Topic-3 decisions does not mean the six-topic OS1 authority
chain is complete or implementation is authorized.

## Validation and protected-worktree record

All four requested repository checks completed successfully (exit 0):

- `git diff --check`.
- `cargo fmt --check`.
- `cargo clippy -- -D warnings`.
- `cargo test --no-run`.

These are formatting/build checks only, not proof of the unimplemented OS1
algorithm. Test binaries were compiled, not executed. The ordinary test suite
was not run. The new untracked review also receives a separate whitespace
check because ordinary git diff does not include untracked files.

After those checks, T3 was reverified at exactly 1154 lines and SHA-256
`09aa74d6d0efbdc0e438bc1662f5b910b4b66dfbbeeb03aeb380add3d9a8a0b4`.
The blocker review retained its exact 314-line identity and hash above.
HEAD/main/origin-main retained the baseline, divergence remained 0/0, and
staging remained empty.

The protected helper tree was neither read nor modified. No authentic project,
reserve candidate, reference MIDI, provenance data, or blind-validation
candidate material was accessed. Experiment 032 was verified by hash only:
`57dcf5007a3fb7ca98efb51dc20a60f9485835e41d0ce1d228a2cbf5fbd6b747`.
No existing authority document, Topic-3 design, DECISIONS.md, or ROADMAP.md was
edited. No code was implemented, and nothing was staged, committed, or pushed.

DECISIONS.md retained SHA-256
`9f0014d36bb270790c6c0dd0f497bcee42e1d5f8dab4a926b7d05bef324ff8be`;
ROADMAP.md retained SHA-256
`c676fb7f0e11690945b8322ed9188d748920798a50ae8056b4afdacade7c0246`.
The sole additional worktree status entry is this review file. Complete
ordinary `git status` at verification:

```text
On branch main
Your branch is up to date with 'origin/main'.

Changes not staged for commit:
  (use "git add <file>..." to update what will be committed)
  (use "git restore <file>..." to discard changes in working directory)
	modified:   docs/DECISIONS.md
	modified:   docs/ROADMAP.md

Untracked files:
  (use "git add <file>..." to include in what will be committed)
	docs/CONTROLLED_TRACK3_2_MIDI_CHANNEL_CHANGE.md
	docs/OS1_ENUMERATION_AND_MAP_PRODUCTION_V1.md
	docs/OS1_ENUMERATION_AND_MAP_PRODUCTION_V1_REVIEW.md
	docs/OS1_TOPIC3_BLOCKER_REVIEW_U3_001_U3_002.md
	tools/

no changes added to commit (use "git add" and/or "git commit -a")
```

## Final approval verdict

**TOPIC 3 APPROVED: YES**, limited to the exact reviewed 1154-line design and
its delegated enumeration/MAP-production scope. U3-001 and U3-002 are correctly
integrated, and no correction or new authority decision is required.

Durable preservation of the approved pair is a separate action not performed
here. Topic 4 may not begin in this task; corrective implementation remains
unauthorized under the checkpoint's complete-chain gate.

TOPIC 3 REVIEW COMPLETE: YES
BLOCKER: 0
IMPORTANT: 0
MINOR: 0
UNRESOLVED IMPLEMENTATION-CRITICAL DECISIONS: 0
U3-001 APPROVED: YES
U3-002 APPROVED: YES
TOPIC 3 APPROVED: YES
TOPIC 4 MAY BEGIN: NO
CORRECTIVE IMPLEMENTATION AUTHORIZED: NO
