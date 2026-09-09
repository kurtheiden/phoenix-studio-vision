# Review identity

Fresh independent design-authority approval review, 2026-09-09.
The reviewer did not author or revise the clarification. Repository contents,
not prior conversation reasoning or lost temporary material, supplied the
review evidence. Only this review file was created.

The proposed clarification closes both confirmed blockers. This approval is
limited to its two authority amendments; it is not approval of Topic 3 or
authorization to implement any part of OS1.

# Authority and artifact identities

The baseline gate passed before substantive review:

- HEAD, main, origin/main: `0bd2cbc63ae481271f7c2e3c0d22060d49e03aeb`.
- Subject: `Define OS1 custody authority and lifecycle`.
- main...origin/main: 0 ahead, 0 behind.
- Staged paths: none.

| Input | Lines | SHA-256 |
|---|---:|---|
| Topic-3 draft | 776 | `587564ea6d9659e6420c4adc207d9f68311eda36f9c44e02538cded67bbcb18b` |
| Blocker review | 314 | `d7aeb9277550a20c30255e41fa62718d924706dad1e9d54152b62d40975f3639` |
| Clarification reviewed | 512 | `6763a37804c0f57cc8e637746990844d92546d564ae20dd52c35f530b7e8d09c` |

All eleven requested documents were read in full. The nine upstream working
documents were verified equal to HEAD. References below use T1 for the
canonical schemas/errors, A1 for the approved Topic-1 error amendment, T2 for
the custody/lifecycle/registry design, C for the clarification, and BR for
the blocker review. Their approving reviews and the reconciliation/review
were considered alongside their exact normative rules. Broad earlier approval
claims were not treated as substitutes for missing operands or predicates.

The checkpoint's older baseline and topic-status statements are historical
handoff context. The current baseline and artifact gates above control this
review. The Topic-3 draft was consulted at sections 3–5 for blocker consequences
only; it remains unapproved and supplies no authority.

# U3-001 review

**CLOSED.** C section 3 explicitly amends the preserved G-006 syntax rather
than denying its historical normative status. Its ten mandatory options are,
in order: executable-authorization, preflight, path-map, root,
implementation-review-root, custody-record-root, private-output-root,
helper-source, helper-executable, map-output. All five original options retain
their meanings and relative order. Missing, duplicated, reordered, extra, or
wrong-arity options fail invocation validation.

The five additions are sufficient and minimal within the approved explicit
root/descriptor and raw-file model. Removing either raw-file operand leaves a
required current measurement without bytes. Removing any of the three added
root operands requires inferred-parent or serialized-locator acquisition that
this contract expressly excludes. No additional review, identities, build,
custody executable/source, or Git operand is required for enumerate's scope.
A different interface encoding is conceivable, but is not an existing
approved operand source that makes one of these five redundant.

| Operand or object | Required root | Independent compatibility result |
|---|---|---|
| --root | CANDIDATE_ROOT | Retains T1 MAP's explicit candidate-root locator source. |
| authorization | IMPLEMENTATION_REVIEW_ROOT | T2 sections 2–3 assign this placement. |
| helper source and executable | IMPLEMENTATION_REVIEW_ROOT | T2 section 5.3 and rows 018–021 require explicit raw immediate children. |
| preflight and path map | CUSTODY_RECORD_ROOT | T2 sections 2–3 and 8 assign this placement. |
| MAP output | PRIVATE_OUTPUT_ROOT | T1 MAP and T2 section 3 require this output parent. |

C section 3.3's exact lexical child rule is compatible with T2's explicit-root
plus immediate-child model. It rejects a disallowed operand form before
filesystem access; it does not reject a successfully acquired alternate root
identity merely because its original serialized locator differs. The special
case for root `/` is deterministic. Bounds, basename exclusions, absolute-byte
grammar, and the ban on normalization preserve T1/T2. Descriptor-relative
operations after basename extraction prevent a separate full-path reopen.

The root order is candidate, review, custody, private, exactly the root-option
order in the new signature. T2 section 4.2 and V02-003 require command-order
acquisition, not preflight-roots' command-specific order for every command.
Consequently no root-order exception is introduced. All four roots participate
in separation, including nonnesting, before serialized input acquisition.
Acquiring candidate-root directory metadata at this stage is not enumeration
or candidate-child access.

The possible unsupported-root-association failure was checked explicitly.
Authorization has no root component, and C correctly does not invent one.
Preflight carries all four root identities; path map carries and correlates
those same four identities. Authorization supplies authorization/review IDs,
repository identity and custody digests that correlate to preflight. Path map
supplies the matching authorization/preflight IDs. Fresh roots can therefore
be compared against the validated preflight/path-map components, and the
authorization descriptor's assigned review parent can be checked against that
same review-root component. This is the association already used by T2
sections 5.2 and 9 for bind-map's supplied chain. It is not a claim that
authorization alone proves the historical review root, nor cryptographic
authentication against coordinated hostile rewriting. T2's honest,
access-controlled threat model and preflight bootstrap remain unchanged.

Source and executable bytes are acquired in that order at V02-018–021 using
T2 section 5.3's complete stable acquisition. V02-028 compares source first,
then executable, against the respective authorization hashes. No comparison
consumes future data; no authorized digest string substitutes for measurement.
Raw, evidence, and root descriptors remain retained for the existing rechecks.

| Registry range | Application under the clarification |
|---|---|
| V02-015–017 | Supplied-chain correlation and fresh-root comparison; no invented authorization root field. |
| V02-018–021, 028 | Existing T3 current-helper allocation, acquisition, and measured identity checks. |
| V02-022–027 | Review-only and repository-only operations remain skipped. |
| V02-029 | Existing supplied provenance/root correlations follow current-helper checks. |
| V02-030–032 | No supplied MAP/lifecycle objects in enumerate; skipped. |
| V02-033–036 | Retained root/non-root final observations, identity and protected-state checks remain required. |
| V02-037, 040 | MAP output-parent and non-reservation absence checks remain required. |
| V02-038–039 | Reservation-only predicates remain skipped. |
| V02-041–042 | Timestamp, then construction, then completed-output integrity; not moved by this amendment. |

C section 3.4 explicitly leaves local enumeration and its integration with
final checks to Topic 3. That is an existing delegated task, not a missing
authority operand or permission to relocate registry rows. Its requirement
for authority/parent/absence checks before child enumeration does not purport
to replace final preproduction rechecks.

# U3-002 review

**CLOSED.** C section 4 expressly replaces the impossible unconditional
comparison. Computation on retained canonical MAP bytes is a legal
observational derivation; it creates no artifact or authority transition.
MAP-only mode does not claim verification against a declared digest.

T1 MAP retains exactly these 17 ordered fields: schema, map_id, preflight_id,
path_map_id, authorization_id, executable_review_id, helper_source_sha256,
helper_executable_sha256, custody_source_sha256, custody_executable_sha256,
candidate_root_path_b64u, candidate_root_identity, private_output_root_identity,
enumerated_at, entry_count, entries, object_sha256. map_entry_v1 remains
record_id, basename_b64u, kind. Only OS1_MAP_BINDING_V1 serializes
entries_sha256; computed_entries_sha256 is ephemeral derived data.

The entries domain remains the exact canonical array substring including
`[` and `]`, excluding LF. Empty entries therefore hash the bytes `[]`.
It is neither the MAP self-digest projection nor the complete MAP bytes.
All self-digest and complete-object domains remain intact.

Independent inspection of all sixteen T2 signatures gives this exhaustive
application set:

| Command | MAP input | Binding input | V02-030 action |
|---|---|---|---|
| bind-map | yes | no | Compute and retain only. |
| freeze-map-review | yes | yes | Compute and compare. |
| authorize-run | yes | yes | Compute and compare. |
| reserve-run | yes | yes | Compute and compare. |
| validate-binding | yes | yes | Compute and compare. |
| validate-freeze-review | yes | yes | Compute and compare. |
| validate-run-reservation | yes | yes | Compute and compare. |

The remaining nine commands are freeze-executable-review, verify-executables,
preflight-roots, create-path-map, validate-executable-review,
validate-review-identities, validate-executable-authorization,
validate-preflight, and validate-path-map. None supplies MAP; all skip 030.
Enumerate likewise has no MAP input. Future Topic-4/5 signatures are not
silently defined or excluded by this T2 inventory.

The conditional binding prerequisite is explicit: V02-014 binding is required
exactly when binding is an input. All serialized inputs complete rows 005–014
before row 030, regardless of their command-line ordering. V02-031 can consume
the derived observation and, where supplied, the established pair equality.
It still excludes a producer's future output.

The earlier generic digest/correlation language would otherwise be a plausible
competing owner. C section 4.2 expressly qualifies V02-014/015/029/031 for this
one comparison, and section 7 expressly gives that qualification precedence.
Thus an otherwise canonical, self-digest-valid binding whose entries digest
alone is wrong reaches 030 and fails EVIDENCE_DIGEST_MISMATCH, not an earlier
generic correlation error. Other digest failures, including its own invalid
self-digest or a wrong complete MAP reference digest, retain earlier checks.
This is a necessary consequence of the authorized V02-030 semantic/dependency
correction, not wholesale replacement of another row.

bind-map computes at 030, completes the remaining input/root/output checks,
acquires time at 041, and only then constructs binding using the retained
digest. Row 042 validates the completed output. No output-path read or future
object supplies a comparison operand. Wrong internally copied digest after
successful input validation is an implementation invariant; an ordinary wrong
supplied binding remains a supplied-evidence mismatch. Publication remains
after 042, and freeze authority still depends on the full binding chain.

# Error compatibility review

**PASS: the approved vocabulary remains exactly 59 members.** No new literal,
changed stderr envelope, changed exit status, or ordinary-error fallback is
introduced. C section 6 was checked against T1's semantic partitions and A1
sections 5–6, rather than against names alone.

| Concrete condition | Unique approved result |
|---|---|
| Invalid option/order/arity | INVALID_INVOCATION |
| FILE is not the exact lexical immediate child of its assigned explicit ROOT | PATH_GRAMMAR_INVALID |
| Root/parent chain cannot be acquired no-follow, statted, or established as directories | ROOT_CHAIN_INVALID |
| Successfully acquired root differs from frozen component | ROOT_AUTHORITY_MISMATCH |
| Root aliasing/nesting | ROOT_SEPARATION_FAILED |
| Raw helper open failure or successfully observed nonregular kind | EXECUTABLE_AUTHORITY_INPUT_OPEN_FAILED |
| Raw helper descriptor-stat syscall failure | OBJECT_STAT_FAILED |
| Raw helper bound/read/required rewind/reread/digest-byte acquisition failure | EXECUTABLE_AUTHORITY_INPUT_READ_FAILED |
| Successfully measured helper source/executable hash differs | EXECUTABLE_IDENTITY_MISMATCH |
| Later bound device/inode/kind discontinuity | OBJECT_CHANGED_OR_REPLACED |
| Later protected change with continuous identity | RACE_OR_MUTATION_DETECTED |
| Output parent fails assigned authority at its output-parent stage | OUTPUT_PARENT_UNAUTHORIZED |
| Required final already occupied before rename | OUTPUT_ALREADY_EXISTS |
| Computed entries digest differs from supplied binding field | EVIDENCE_DIGEST_MISMATCH |
| Impossible wrong digest in internally constructed binding at 042 | INTERNAL_INVARIANT_FAILED |

Lexical mismatch is a disallowed caller path form, not a successful filesystem
authority comparison. Root acquisition errors remain distinct from output
authority errors. Raw inputs are not serialized evidence, so EVIDENCE_* never
absorbs their I/O. Initial complete-byte acquisition and later detected bound
mutation retain A1's stage partition. Serialized inputs retain the existing
open/kind/bound/read and UTF-8/JSON/schema/canonical/digest/correlation/authority
sequence. All error names used in C section 6 are approved members.

Row 030 hashes already acquired bounded bytes and adds no ordinary filesystem
operation needing another error. MAP-only mode lacks a comparison by design,
not because an ordinary failure has been hidden as an invariant. Timestamp
and publication failures retain their specialized mappings. No reviewed
runtime condition lacks a unique approved result.

# Authority-effect review

**PASS.** C section 7 limits precedence to the G-006 enumerate operand contract
and V02-030's meaning, dependencies, and directly necessary comparison-owner
qualification. The 42 IDs and numeric order remain intact; no row is added.
All sixteen custody signatures are unchanged. Other correlations and validator
semantics remain required outside that one entries-digest relationship.

T1's schemas, field order/types, canonical bytes, digest domains, RECORD
limits, privacy, and pathname nonauthority remain intact. T2's root roles,
identity projection, lifecycle states, reservation behavior, publication and
terminal boundaries remain intact. Explicit paths do not grant authority;
current helper measurements do not broaden candidate access. There is no
hidden lookup, fixed helper filename, process-image authority, or new state.

Approval permits Topic 3 to consume these two clarified interfaces while
finishing its separately reviewed design. It does not approve the existing
draft, begin Topic 4, authorize candidate material access, or permit corrective
implementation. This review performs none of those actions.

# Blocker replay

These answers independently replay BR's A–H questions using C as prospective
authority, rather than adopting C's author closure table.

| U3-001 question | Independent answer |
|---|---|
| A: exact durable interface | G-006 preserved five options; prospective C 3.1 explicitly replaces that surface with the exact ten-option contract reviewed above. |
| B: status of historical syntax | Normative preserved syntax, explicitly amended; not dismissed as incidental historical evidence. |
| C: current measurements | Both complete current helper source and executable remain required and measured before authority use. Copying hashes alone remains insufficient. |
| D: necessary raw operands | Two explicit files beneath the separately supplied review root now exist; acquisition uses unchanged raw stages. All four root operands are present. |
| E: existing source of missing operands | None existed in schemas/process conventions. The new mandatory caller operands supply them without discovery; frozen root components, not locators, establish identity. |
| F: canonical receipt mechanism | C 3.1–3.4 now provides enumerate's missing receipt/order mechanism; T2's other signatures are not used to imply options. |
| G: complete invocation without clarification | No under old authority; yes under the explicit amendment. No additional required operand remains absent. |
| H: compatible unchanged interpretation | No old interpretation sufficed. The proposed explicit amendment supplies the necessary authority decision without weakening measurement or root checks. |

**U3-001: CLOSED.**

| U3-002 question | Independent answer |
|---|---|
| A: exact MAP schema | The 17 ordered fields listed above remain exact, with unchanged three-field map_entry_v1. |
| B: MAP entries_sha256 membership | Absent at top level and inside entries. No new MAP field is permitted. |
| C: serialized ownership | Only OS1_MAP_BINDING_V1 contains entries_sha256. An ephemeral computed digest is not serialization. |
| D: predicate/dependencies | MAP always supplies derivation bytes; explicitly supplied binding adds V02-014 binding and the comparison. Numeric position and observational mode remain. |
| E: available referent | Exactly supplied binding.entries_sha256 in comparison mode; no declared referent is required in MAP-only mode. |
| F: interpretations | MAP-field, whole-object-digest, self-comparison, and future-output readings are excluded. One signature-controlled conditional rule remains. |
| G: construction timing | Binding cannot exist as producer output at 030. Retained computation precedes timestamp; construction follows 041; output validation is 042. |
| H: unchanged T1/T2 interpretation | The original row was impossible. The explicit row and narrow overlap amendment makes the handoff coherent while leaving T1 and remaining T2 authority intact. |

**U3-002: CLOSED.**

# Findings

| Severity | Count |
|---|---:|
| BLOCKER | 0 |
| IMPORTANT | 0 |
| MINOR | 0 |

Unresolved implementation-critical decisions within the clarification: **0**.
Topic 3's delegated local algorithm still requires completion and independent
review; that remaining work is not approval by implication.

Validation completed successfully: cargo fmt --check; cargo clippy -- -D
warnings; cargo test --no-run; git diff --check. Test binaries were compiled,
not executed. These repository checks establish formatting/build hygiene,
not correctness of an unimplemented OS1 design.

Protected worktree checks preserve the starting DECISIONS.md SHA-256
`9f0014d36bb270790c6c0dd0f497bcee42e1d5f8dab4a926b7d05bef324ff8be`
and ROADMAP.md SHA-256
`c676fb7f0e11690945b8322ed9188d748920798a50ae8056b4afdacade7c0246`.
The draft, blocker review and clarification retain the gated identities above.
Experiment 032 was hash-checked only and retains
`57dcf5007a3fb7ca98efb51dc20a60f9485835e41d0ce1d228a2cbf5fbd6b747`.
The helper tree was neither read nor modified. Authentic projects, reference
MIDI, reserve artifacts and candidate material were not accessed. No staging,
commit, push, cleanup, reset or restore occurred. Staged paths remain none.

# Final verdict

Topic-1 schema compatibility: **PASS**.
Topic-2 registry compatibility with the explicit narrow amendment: **PASS**.
Public error vocabulary compatibility: **PASS**.

The reviewed 512-line clarification is approved for its stated narrow
authority effect. No Topic-3 edit is performed by this review, and the draft
itself remains unapproved.

U3-001 CLOSED: YES
U3-002 CLOSED: YES
CLARIFICATION APPROVED: YES
TOPIC 3 MAY BE EDITED TO COMPLETION: YES
TOPIC 4 MAY BEGIN: NO
CORRECTIVE IMPLEMENTATION AUTHORIZED: NO
