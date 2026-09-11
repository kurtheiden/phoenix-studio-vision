# OS1 Topic-1 Candidate Size Representability Clarification U4-001 V1

# Status and provenance

**DRAFT — NOT APPROVED AUTHORITY — INDEPENDENT REVIEW REQUIRED**

Authored 2026-09-11 for the independently confirmed U4-001 authority gap.
At the authoring gate, HEAD, main, and origin/main were all
`f1cafd0b73136cf7138dd6dbdbde37507286b48c`, with divergence 0/0 and an empty
index. The working directory was
`/Users/kurtheiden/Developer/phoenix-studio-vision`.

The committed checkpoint and relevant committed authority listed below were
consumed. All ten checkpoint-listed design/review file identities matched
their exact line counts and SHA-256 values before reliance. Prospective status
wording retained in approved predecessor designs is qualified by their exact
approving reviews and the checkpoint's durable-preservation record.

The U4-001 independent review exists as session output, not a committed review
artifact. The user's confirmed finding supplies this task's blocker premise;
it supplies no additional normative rule. No lost temporary document, helper
implementation, unfinished Topic-4 design, or candidate contents supplied a
decision. This document is new prospective authority, not recovered authority.

Protected modified/untracked work is outside the amendment. Experiment 032
was verified by digest only. The helper tree was neither read nor modified.
Only this new document is authored; no predecessor is edited.

# Authority and precedence

References below name committed files at the authoring checkpoint. Line
anchors later in this document refer to that exact commit, not future edits.

| Ref | Document |
|---|---|
| CP | [OS1_TARGETED_RESPEC_RESUME_CHECKPOINT.md](OS1_TARGETED_RESPEC_RESUME_CHECKPOINT.md) |
| T1 | [OS1_CANONICAL_EVIDENCE_SCHEMAS_AND_ERRORS_V1.md](OS1_CANONICAL_EVIDENCE_SCHEMAS_AND_ERRORS_V1.md) |
| T1R | [OS1_CANONICAL_EVIDENCE_SCHEMAS_AND_ERRORS_V1_REVIEW.md](OS1_CANONICAL_EVIDENCE_SCHEMAS_AND_ERRORS_V1_REVIEW.md) |
| A1 | [OS1_TOPIC1_ERROR_TAXONOMY_CLARIFICATION_FOR_TOPIC2_V1.md](OS1_TOPIC1_ERROR_TAXONOMY_CLARIFICATION_FOR_TOPIC2_V1.md) |
| A1R | [OS1_TOPIC1_ERROR_TAXONOMY_CLARIFICATION_FOR_TOPIC2_V1_REVIEW.md](OS1_TOPIC1_ERROR_TAXONOMY_CLARIFICATION_FOR_TOPIC2_V1_REVIEW.md) |
| T2 | [OS1_CUSTODY_AUTHORITY_LIFECYCLE_AND_VALIDATOR_REGISTRY_V1.md](OS1_CUSTODY_AUTHORITY_LIFECYCLE_AND_VALIDATOR_REGISTRY_V1.md) |
| T2R | [OS1_CUSTODY_AUTHORITY_LIFECYCLE_AND_VALIDATOR_REGISTRY_V1_REVIEW.md](OS1_CUSTODY_AUTHORITY_LIFECYCLE_AND_VALIDATOR_REGISTRY_V1_REVIEW.md) |
| C3 | [OS1_TOPIC3_AUTHORITY_CLARIFICATION_U3_001_U3_002_V1.md](OS1_TOPIC3_AUTHORITY_CLARIFICATION_U3_001_U3_002_V1.md) |
| C3R | [OS1_TOPIC3_AUTHORITY_CLARIFICATION_U3_001_U3_002_V1_REVIEW.md](OS1_TOPIC3_AUTHORITY_CLARIFICATION_U3_001_U3_002_V1_REVIEW.md) |
| T3 | [OS1_ENUMERATION_AND_MAP_PRODUCTION_V1.md](OS1_ENUMERATION_AND_MAP_PRODUCTION_V1.md) |
| T3R | [OS1_ENUMERATION_AND_MAP_PRODUCTION_V1_REVIEW.md](OS1_ENUMERATION_AND_MAP_PRODUCTION_V1_REVIEW.md) |
| R | [OS1_DESIGN_AUTHORITY_RECONCILIATION.md](OS1_DESIGN_AUTHORITY_RECONCILIATION.md) |
| RR | [OS1_DESIGN_AUTHORITY_RECONCILIATION_REVIEW.md](OS1_DESIGN_AUTHORITY_RECONCILIATION_REVIEW.md) |

Upon independent approval, this clarification supplements T1's mandatory
semantic partition with exactly one candidate-size representability domain
and one public error. Its precedence is limited to the amendment surface
enumerated below. T1 as supplemented by A1 and this document controls error
membership. All other T1/A1 rules and all T2/C3/T3 algorithms remain intact.
Historical approvals and counts remain accurate reports of their own targets.

Normative requirements in this draft are prospective. Self-review does not
make them approved authority. CP's approval and durable-preservation sequence
must finish before a separately authorized Topic-4 task consumes this rule.

# Scope

Decide the public outcome for a successfully observed, stable size outside
T1's RECORD `size_bytes` domain on an otherwise eligible non-alias regular
candidate. Fix its exact domain, local precedence, output consequences, and
vocabulary accounting. This is a Topic-1 outcome-taxonomy amendment only.

# Explicit exclusions

This document does not author RA01–RA19, a candidate syscall sequence, an
inspect command signature, publication mechanics, terminal events, fixtures,
or conformance registries. It does not resume Topics 4–6 or authorize code.
It changes no enumeration eligibility, custody procedure, root authority,
reservation lifecycle, existing rejection code, or serialized evidence schema.
It authorizes no candidate access, reference access, provenance lock, content
inference, size-based selection, or general file-size policy.

# Confirmed U4-001 conflict

T3 section 7, lines 402–426, includes every immediate name except `.` and `..`
and forbids selection or kind classification by size. Section 9 retains size
ephemerally for continuity; MAP does not serialize a candidate size ceiling.

T1's inspected-line table, line 531, requires `size_bytes` to equal stable
`st_size` and belong to `0..9007199254740991`. T1 lines 569–583 restrict the
four expected rejections to stable nonregular kinds or Finder alias files.
The example `9007199254740992` therefore fits neither a faithful successful
line nor any approved rejection.

T1 mandatory partition rules 1, 2, and 6 prevent borrowing an invariant,
serialized-evidence, or failed-syscall error. A1 section 5 expressly covers an
unrepresentable raw executable-authority byte count, but section 5.1 excludes
candidate observations from those new members. A1's producer-clock range
error is also a different target. T2's V02-042 covers constructed-output
integrity, not this external observation; T3's analogous MAP rule cannot
create a RECORD outcome. The uncovered domain requires an explicit amendment.

# Alternatives considered

| Criterion | A: new expected-rejection code | B: new fatal inspect error |
|---|---|---|
| Inventory and correspondence | Could preserve one line per MAP entry, but only by extending the rejection schema's accepted values. | Preserves the complete MAP and every successful RECORD's exact coverage; the failing command cannot publish a partial RECORD. |
| Existing outcome meaning | Would add a new nonfatal candidate class beyond the four kind/alias decisions. | Adds one evidence-production failure while preserving every existing success and rejection predicate. |
| Intrinsic characteristic versus inspection limitation | Size is an observed property, but this boundary does not prove the file has an excluded kind, alias identity, or unusable content. | Expresses inability of the prescribed protocol to represent required metadata, even when the OS and hashing machinery work. |
| One candidate versus whole command | A fifth rejection could permit other candidates to complete; that is a new product policy. | One candidate causes the defect, but under the unchanged line schemas the command cannot produce its required complete RECORD. |
| Privacy and determinism | Could be private and deterministic, but adds a durable per-candidate classification. | One constant stderr line, no size/name/ID, no new evidence fields, and one exact predicate. |
| Narrowness | Requires a fifth rejection literal, a new regular-file pairing, and amendments to four-code schema/validation statements. | Requires one additional taxonomy member and one disjoint partition; no schema, pairing, or RECORD byte bound changes. |
| Mandatory partitions | Requires explicit rejection authority; no existing rejection can be reused. | Fits a new specialized external-observation domain without changing existing error meanings. |
| Later integration | Requires new nonfatal line construction and continued inspection behavior. | Requires a local pre-construction check with the fixed fatal result specified below. |
| Preservation | Expands the meaning of completed expected-rejection evidence. | Leaves successful inspection and all four expected rejections unchanged. |

The predecessor rules do not assert that every intrinsic candidate property
must be nonfatal, or that fatal errors require broken implementation machinery.
A1 already treats some successful external observations that cannot supply a
representable protocol value as fatal, in its own separate domains. That is an
architectural comparison, not permission to reuse A1's literals for candidates.

A fifth rejection is possible only through a broader explicit schema-policy
change. It is not chosen: the narrow defect concerns representing required
inspection evidence, and a fatal error preserves more closed authority.
Clamping, omission, changed bounds, fabricated kinds, and reuse of an existing
error were also considered and excluded because each violates an unchanged
contract. Neither option is selected for implementation convenience.

# Chosen semantic outcome

**B — FATAL INSPECT COMMAND ERROR.**

When the exact trigger below holds, inspect must fail with
`CANDIDATE_SIZE_UNREPRESENTABLE`. It must not produce a rejection line, treat
the candidate as successfully inspected, omit it, or continue to obtain a
successful command result. This is a protocol-representation failure, not a
claim about project format, candidate content, storage capacity, or OS failure.

# Canonical vocabulary change, if any

Add exactly one member to `OS1_METADATA_HELPER_ERRORS_V2`:

| Literal | Semantic domain | Final terminal evidence | May raise |
|---|---|---|---|
| `CANDIDATE_SIZE_UNREPRESENTABLE` | Successful stable stat of an identity-valid, non-alias REGULAR_FILE candidate supplies an exact size outside RECORD's `0..9007199254740991` domain, after the prerequisites below pass. | none | T4 only, inspect |

The taxonomy identifier remains unchanged. There are no aliases, alternative
spellings, numeric enum ordinals, or second literal for the lower bound.
Existing members retain their spelling, domain, and topic allocations.
Allocation permits mapping the specified condition; it grants no operation
or filesystem access independently of the owning topic's authority.

# Exact trigger

Let S be the exact mathematical integer reported by the successful candidate
stat observation selected as the stable source of RECORD `size_bytes`.
The new member is mandatory if and only if all of these conditions hold:

1. Inspect's preceding authority and candidate-acquisition prerequisites pass.
2. The candidate has valid bound identity and observed kind REGULAR_FILE.
3. The approved FinderInfo decision establishes that it is not a Finder alias:
   FinderInfo is absent, or its valid first-eight-byte result has Type other
   than `616c6973`. A failed/malformed probe does not establish non-alias status.
4. The stat syscall succeeds and the identity and protected-state continuity
   checks required through this decision point pass, including stability of S.
5. No independent fatal failure has already terminated the invocation.
6. `S < 0` or `S > 9007199254740991`.

Evaluate the range against the exact observed value before any conversion to
the bounded RECORD integer. Wrapping, rounding, unsigned reinterpretation of
a negative value, truncation, clamping, or converting first and checking later
cannot implement this predicate. No programming-language number limit supplies
the bound; T1's protocol is the sole source.

Both endpoints, 0 and 9007199254740991, pass this check. Passing it establishes
only size representability, not inspection success or exemption from any other
required check. The example 9007199254740992 fails it.

No consumed committed authority guarantees that a successful platform stat
can never report a negative size. If an API can report such an integer, the
same predicate and same error apply. This is not an assertion that any
particular platform does report one. The lower-bound branch closes the same
representation domain; it does not invent a negative-size file kind or an
additional error. Conversion bugs affecting an actually in-range S are not
external size unrepresentability and remain outside this member.

# Exact exclusions

The new member never classifies:

- a failed stat syscall, candidate open/read/seek operation, FinderInfo probe,
  resource-fork probe, hashing operation, or identity/mutation comparison;
- a stable nonregular candidate or a Finder alias, regardless of its size;
- malformed supplied serialized evidence, including an out-of-range size in
  a supplied RECORD line;
- raw executable-authority input size, evidence-file size, root metadata,
  producer-clock values, MAP byte size, or output-file publication checks;
- candidate timestamp representability or any other stat field's domain;
- a physical allocation, sparse-file, resource-fork, available-memory,
  timeout, project-format, or discretionary maximum-size policy;
- internal construction mistakes, fabricated schema values, or a failure to
  represent an in-range observation caused by the chosen implementation.

No excluded case is assigned a new meaning here. Any applicable approved
domain remains controlling; absence of such a domain does not permit this
member as a fallback. This document claims closure only for U4-001.

# Precedence

This is a local dependency order for the new predicate, not RA01–RA19.
Topic 4 must place the decision after successful stat, applicable binding and
continuity validation, and the non-alias decision; it must place it before
that candidate's resource-fork probe, first data-fork read, hash passes, and
successful-line construction. These are new integration constraints only for
this representation check. They specify no syscall count or snapshot recipe.

An earlier fatal result terminates the command and cannot be replaced by this
error. When observations needed for this decision expose both an out-of-range
S and a failed identity/continuity prerequisite, the existing prerequisite
error wins. A later size change from an in-range bound observation is mutation,
not permission to replace the bound S and report this member instead.

| Nearby condition at its prescribed stage | Result relative to the new member |
|---|---|
| Authority/acquisition fails before this decision | Existing exact error; no size outcome is evaluated. |
| Candidate stat syscall fails | OBJECT_STAT_FAILED; no successful S exists. |
| Initial bound identity comparison fails | OBJECT_IDENTITY_MISMATCH; do not classify size. |
| Later device/inode/kind discontinuity | OBJECT_CHANGED_OR_REPLACED; do not classify size. |
| Same bound identity but protected observation changes | RACE_OR_MUTATION_DETECTED; do not classify size. |
| Stable nonregular kind with its required rejection prerequisites satisfied | Existing SYMLINK_NOFOLLOW, DIRECTORY_NOT_CANDIDATE, or UNSUPPORTED_FILE_KIND pairing; new member is inapplicable. |
| FinderInfo query/read fails or is malformed | Existing FINDERINFO_PROBE_FAILED or FINDERINFO_MALFORMED; non-alias prerequisite did not pass. |
| Stable Finder Type is alis with its required rejection prerequisites satisfied | FINDER_ALIAS_FILE; new member is inapplicable even when size is out of range. |
| All new-member prerequisites pass; S outside range | CANDIDATE_SIZE_UNREPRESENTABLE; stop before later candidate probes/reads/construction. |
| All new-member prerequisites pass; S within range | Continue the owning algorithm; this member cannot be raised. |
| Later resource probe, read, seek, hash-length, or digest comparison fails after the range check passed | Existing exact error; no retrospective size error. |

Thus a hypothetical failure of an operation never attempted has no precedence.
If several earlier prerequisites fail, Topic 4's ordered failure-point matrix
selects the first under the existing partitions; this new member cannot compete
with any of them. Topic 4 still owns ordering among unrelated operations and
between candidates. It cannot postpone this check past this candidate's data
read to manufacture a read/hash result for the already-established size case.

Stability means passing the owning algorithm's required observations through
the decision, not proving an atomic snapshot, predicting future changes, or
adding an exception to the existing observation-based threat model.

# RECORD consequences

No RECORD line encodes this fatal outcome. Both existing line schemas and
the four rejection literals/pairings remain exactly unchanged. The failing
invocation must not publish its RECORD or a valid terminal completion. Earlier
final evidence remains untouched. Already written unpublished RECORD bytes
remain nonauthoritative residue under the existing no-cleanup rules; they do
not become a partial successful RECORD or a new incident artifact.

For every successfully completed RECORD, MAP index i still maps to line i,
with the same ID and exact entry_count coverage. A fatal invocation produces
no complete RECORD; this is the existing command-failure branch, not a missing
line exception. Do not skip the candidate, substitute a line, renumber IDs,
shrink MAP, emit an ERROR row, or publish lines for only the other candidates.
Zero-entry/zero-byte behavior is unchanged because no candidate triggers this
predicate in an empty MAP.

# Error/exit consequences

The fatal outcome writes zero stdout bytes and exactly these ASCII bytes to
stderr, followed by one LF byte (0x0a):

```text
OS1_METADATA_HELPER_ERROR:CANDIDATE_SIZE_UNREPRESENTABLE
```

Exit status is exactly 70. No additional stderr bytes, candidate size, sign,
name, path, ID, digest, OS diagnostic, or dynamic detail is emitted. The
literal discloses only this bounded failure class and does not distinguish the
two range branches. Existing success/64/70 and signal/host-termination rules
are unchanged. No operational-event trigger, suppression, cleanup, retry, or
reservation transition is created here; Topic 5 retains its assigned scope.

# Validator consequences

The public-error vocabulary recognizer must accept the new exact literal
under the existing envelope and associate it with fatal inspect exit 70.
An operation-allocation check must permit it only for T4 inspect. Neither
recognition nor receipt of stderr creates serialized evidence authority.

No V02-001..V02-042 predicate, ID, dependency, or command allocation changes.
In particular V02-042 remains a constructed-output invariant check and does
not acquire candidate stat values. No forty-third validator row is added.

RECORD validators retain the existing schemas and four rejection pairings.
The new literal in rejection_code or a new ERROR line is schema-invalid; an
out-of-range size_bytes remains outside the schema. Supplied evidence uses
T1's evidence-parse precedence, not the new producer error. Publication-stage
validation retains its TEMP_CONTENT_MISMATCH/FINAL_REOPEN_FAILED domains.
Individually valid lines with missing coverage retain RECORD_INCOMPLETE.

A clamped in-range value may be schema-shaped: validators cannot infer its
false external source from bytes alone. Producer fidelity and whole-chain
validity remain distinct from schema recognition. This amendment makes no new
claim that a validator detects fabricated, internally consistent evidence
without access authorized by its own contract.

Future Topic 4 must recognize the new producer predicate. Future conformance
coverage must recognize membership, allocation, range edges, exclusions, and
the local precedence above. No fixture, test ID, test count, or executable
test is defined or changed in this task.

# Taxonomy/schema count consequences

After approval, the effective vocabulary is the original T1 55 plus A1's four
plus this one: **60 members**. Its identifier remains
OS1_METADATA_HELPER_ERRORS_V2. The original 55 and A1's addition of exactly four
remain historical/subset facts, not claims that either document added sixty.
T2/T3 runtime allocations are unchanged; the sixtieth member is T4-only.

Unchanged counts/bounds: 14 top-level serialized schemas, two RECORD line
classes, four rejection codes, 42 V02 registry rows, 4096 entries/lines,
1124-byte maximum success line, 626-byte maximum rejection line, and
4,603,904-byte maximum complete RECORD. All byte maxima include their existing
LF rules. The historical 4096-byte outer line ceiling remains unchanged.
RECORD size_bytes remains exactly 0..9007199254740991.

Every current-total count statement affected in the consumed approved chain
is enumerated here. Each row has the same amended interpretation: the live
combined vocabulary is 60; the cited document's measured former total remains
historical, and its local error allocations do not expand. Section names or
quoted phrases plus baseline lines are stable anchors for each occurrence.

| Document | Exact count anchors at baseline | Prior wording |
|---|---|---|
| T1 | Taxonomy rules line 700; after enum table 849; Public stderr grammar 865; Forward-compatibility audit 984; I6 closure 1010 | Exactly 55 / one of 55 / 55-member vocabulary; already supplemented to 59 by A1, now supplemented to 60. |
| T1R | Regression gate line 154 | Public vocabulary remains exactly 55. |
| A1 | Section 5 line 173; section 7 tests row 301; section 10 checklist 362 | Resulting 59; assertions of total 55 must change to 59; confirm exactly 59. Total-membership assertions now require 60, preserving prior per-member assertions. |
| A1R | Section 9 arithmetic 198 and VOCABULARY COUNT 202 | 55 + 4 = 59 and count 59; former arithmetic remains true, effective union adds one. |
| T2 | Status line 16; G-002 row 1080; G-028 row 1090; sufficiency paragraph 1099; checklist 1198 | Exactly 59 / 59-member / approved 59 errors. |
| T2R | Section 7 line 184; G-028 row 212 | Exactly 59 / 59-error mapping. |
| C3 | Compatibility table 330; Error compatibility section 337; self-audit row 431 | 59 unchanged; existing 59 sufficient; no accidental 60th error. The intentional T4-only addition does not change C3's local sufficiency. |
| C3R | Error compatibility review line 187 | Approved vocabulary remains exactly 59. |
| T3 | Authority section 57; error section 771; G-028 row 1054; compatibility row 1126 | Exactly 59 / 59 approved / 59 unchanged. |
| T3R | Compatibility table line 248 | PASS: exactly 59; independently counted 55 plus four. |
| CP | Approved Topic-1 summary line 77 | 59 members after A1; future current-status reporting must include this separately approved addition. |

T2 line 11's "original 55-member", A1 line 306's description of the amendment
to the original 55, and A1R lines 162/191's prior insufficiency/original-55
statements are unchanged historical/subset statements. The phrase "four new"
in A1 and its review still means A1's four, not this amendment. No unrelated
numeric statement, historical review verdict, ID, or registry is renumbered.

# Cross-topic consequences

- Topic 1 gains one disjoint fatal domain and the count qualification above.
- Topic 2 retains all procedures, lifecycle rules, and 42 validators.
- Topic 3 retains every inclusion, stat, continuity, MAP, and handoff rule;
  it must not apply this T4-only error or screen candidate sizes.
- Topic 4 remains paused. After approval and durable preservation, its
  separately authorized design must consume the fixed rule below.
- Topic 5 retains terminal/event design and cannot turn this fatal inspect
  outcome into a valid completion. No Topic-5 design is authored here.
- Topic 6 retains test design; only the eventual vocabulary coverage obligation
  is identified, with no registry or fixture authored here.

# Topic-4 integration rule

Topic 4 must map the exact trigger exclusively to
CANDIDATE_SIZE_UNREPRESENTABLE, at a named decision point satisfying this
document's local dependency order. It must define its own stat/binding/
continuity mechanics and integrate this point without omitting existing
checks. It must not choose rejection versus fatality, substitute another
literal, introduce a separate negative-size result, or weaken RECORD fidelity.
Neither this paragraph nor the new error completes any other Topic-4 gap.

# Compatibility analysis

The new domain starts only after successful external acquisition and valid
identity/non-alias/stability observations. Failed stat remains OBJECT_STAT_FAILED;
program defects remain under the restricted invariant rule; EVIDENCE_* remains
serialized-input-only. Candidate open/read/seek, FinderInfo/resource probes,
identity, mutation, and hash disagreement retain their exact meanings.
The new member supplements mandatory partition rule 6; it does not rename a
stat failure or broaden any existing candidate error.

Fatality supplements the prior enumerated fatal categories with one explicit
representation condition. It is not a reinterpretation of I/O failure or a
general claim that all schema-production difficulties are this error.
The four rejection predicates, successful in-range size semantics, privacy
allowlist, canonical bytes, digest graph, pathname nonauthority, and output
placement are unchanged. A static public literal adds no dynamic metadata.

# Amendment surface

Only the following semantic supplements and the complete count table above
have precedence over prior current-authority statements. Reference names
resolve to exact documents in Authority and precedence.

| Document and anchor | Prior rule | Amended interpretation |
|---|---|---|
| T1, OS1_METADATA_HELPER_ERRORS_V2 taxonomy rules/table | Closed original vocabulary, supplemented by A1 to 59; each member fatal with fixed allocation. | Add exactly CANDIDATE_SIZE_UNREPRESENTABLE with the domain, T4-only allocation, and fatal consequences defined here; effective union 60. |
| T1, mandatory semantic partition rule 6 | Candidate failures use exact specialized domains; successful out-of-range size has no member. | Supplement with this one successful-observation representability domain and its local precedence. All existing clauses remain. |
| T1, Expected rejection, lines 581–583 | Stable nonregular/alias outcomes nonfatal; I/O/race/mutation/malformed-evidence/probe failures fatal. | Add this exact size-representation condition to fatal outcomes. No fifth rejection or altered pairing. |
| T1, Public stderr grammar | Exactly one enum literal plus LF; non-invocation errors exit 70. | Permit the new literal for inspect with the same bytes/exit grammar and no dynamic detail. |
| T1, Cross-topic boundary table RECORD/error rows; Unresolved decisions Topic 4 | Topic 4 selects outcomes and concrete mappings within fixed domains. | This one outcome and its local dependencies are fixed upstream; Topic 4 retains all other delegated algorithmic work. |
| T1, Forward-compatibility audit; T1R, Forward compatibility and Final verdict | Later topics could proceed without another schema/error change. | Qualify only the error-sufficiency claim for U4-001: this explicit addition is required. Prior approvals are not revoked; schemas remain sufficient and unchanged. |
| A1, section 5 resulting-membership paragraph and section 7 count compatibility | Four additive errors yield the effective 59; original domains unchanged. | Preserve A1's four exact domains; the combined vocabulary additionally includes this distinct T4-only member. |
| R, G-023 and category-B paragraph beginning "RECORD lines are"; RR, G-023 and section 8 item 4 | Historical nonfatal/fatal categories; later exact matrix delegated. | Preserve historical facts; supplement the prospective fatal set with this condition and bind the later matrix to its chosen result. No other gap is closed here. |
| T2/T2R, C3/C3R, T3/T3R current-vocabulary statements listed above | Current combined total 59 with existing local mappings. | Total 60 only; no new local runtime allocation, changed predicate, or algorithm. C3's two authority corrections remain closed. |
| CP, Topic-1 summary and resume instruction | Current count 59; Topic 4 next after synchronized Topic-3 preservation. | CP is status, not normative design. For U4-001, current-count reporting and the next Topic-4 resume must additionally reflect this clarification's independent approval and durable preservation. |

The inspected-line size_bytes rule, RECORD coverage/bounds, T2 V02-042, and T3
section 7 are deliberately not amended. They motivate the new outcome but
their exact obligations remain. No direct predecessor edit is required by the
standalone-amendment precedent established by A1 and C3.

# Fresh self-review

This is author self-review, not independent approval. The completed draft was
checked against the unchanged predecessor predicates rather than against the
convenience of the selected literal.

| Adversarial check | Result under this draft |
|---|---|
| Original stable non-alias S=9007199254740992, all prerequisites pass | Exactly CANDIDATE_SIZE_UNREPRESENTABLE, fatal exit 70, no RECORD line or final RECORD/completion. |
| Same prerequisites, S=-1 if returned successfully | Same sole error; no unsigned conversion or second code. |
| S=0 or S=9007199254740991 | New predicate false; no new success guarantee or failure. |
| Out-of-range observation plus stat failure | No successful value from that call; existing stat error wins. |
| Out-of-range observation plus failed bound identity or continuity | Existing prerequisite error wins; stable-input premise fails. |
| Stable alias or nonregular object with out-of-range stat size | Existing exact rejection pairing remains; no new error. |
| FinderInfo failure before a non-alias decision | Existing FinderInfo error, not size classification. |
| Range passes, later read/seek/hash/resource failure | Existing exact semantic domain; no new fallback. |
| Raw executable-authority size exceeds its bound | Existing A1 READ domain, unchanged. |
| Invented rejection/error line or oversized serialized size_bytes | Existing closed-schema validation rejects it; no evidence parser uses the new producer error. |
| Clamped in-range line | Prohibited source falsification, even if schema-shaped; no invented validator detection guarantee. |
| One failing candidate among several MAP entries | No omission or partial successful RECORD; command fails. Every completed RECORD still covers all MAP entries in order. |
| Changes to enumeration, custody, schema bounds, or existing domains | None; only one additive taxonomy member and local integration constraints. |
| Topic-4 mapping discretion for this case | None; condition, literal, fatality, outputs, and nearby precedence are fixed. |
| Other Topic-4 decisions and future tests | Remain delegated; no unrelated gap claimed closed. |

Vocabulary audit searches cover "appropriate", "as needed", "if necessary",
"unsupported", "invalid", "handle", and "reasonable". No discretionary
implementation rule uses these words. Existing enum names retain their exact
definitions; schema-invalid means failing T1's closed constraints; valid
identity/stability means the owning algorithm's required checks passed through
this decision. No new implementation-critical ambiguity was found within
U4-001. This statement makes no completeness claim for unfinished Topic 4.

# Implementation authorization status

This draft makes U4-001 deterministic only under its prospective rules.
The approved authority gap remains pending independent review and durable
preservation. No corrective implementation, helper modification, candidate
access, Topic-4 resumption, or Topic-5/6 work is authorized. No file is staged,
committed, or pushed by this task. Existing protected work remains untouched.

**U4-001 CLARIFICATION DRAFT COMPLETE: YES**

**U4-001 CLOSED BY DRAFT: YES**

**READY FOR INDEPENDENT REVIEW: YES**

**TOPIC 4 MAY RESUME: NO**

**CORRECTIVE IMPLEMENTATION AUTHORIZED: NO**

# Required next action

**INDEPENDENT REVIEW OF U4-001 PREDECESSOR-AUTHORITY CLARIFICATION**
