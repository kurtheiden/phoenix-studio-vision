# Review identity

Independent design-authority review of U4-002, completed 2026-09-11.
This artifact faithfully preserves the completed session review and its
approval. It is not a new review or a revision of the approved clarification.

Reviewed clarification:
`docs/OS1_TOPIC1_CANDIDATE_TIMESTAMP_REPRESENTABILITY_CLARIFICATION_U4_002_V1.md`

- Exactly 524 lines.
- SHA-256:
  `af7acbc6a04526f00be4a92a4251620e5a0f74adb6768502bdc5a4f1ea888abd`.

Initial and final review baseline: HEAD, main, and origin/main all equaled
`fd7c8edd6fb961442a5a35a707dd9e0a060db7bb`, subject
`Approve U4-001 candidate size representability clarification`.
Divergence remained 0/0 and staging remained empty. The working directory was
`/Users/kurtheiden/Developer/phoenix-studio-vision`.

# Scope and method

The review independently derived the outcome from committed predecessor
authority before comparing the proposal. Author self-review, prior transcript
conclusions, and the uncommitted U3 blocker review supplied no normative rule.
The review was read-only and transcript-only: no files were modified, no review
artifact was created then, and no staging, commit, push, or implementation test
occurred. Protected helper implementation contents and authentic/reference/
reserve/blind-validation candidate contents were not accessed.

The review covered outcome alternatives, exact mathematical bounds, all four
required timestamps, semantic partitions, precedence, public output, RECORD
totality, vocabulary and validator counts, amendment coverage, privacy, and
topic boundaries. Mathematical boundary probes were not implementation tests
or Topic-6 fixtures. No platform was assumed to produce outside-domain values.

# Authority verified

The durable checkpoint was read first. Relevant committed provisions and
approving reviews were independently consumed from:

- `OS1_CANONICAL_EVIDENCE_SCHEMAS_AND_ERRORS_V1.md` and its review.
- `OS1_TOPIC1_ERROR_TAXONOMY_CLARIFICATION_FOR_TOPIC2_V1.md` and its review.
- `OS1_TOPIC1_CANDIDATE_SIZE_REPRESENTABILITY_CLARIFICATION_U4_001_V1.md`
  and its review.
- `OS1_CUSTODY_AUTHORITY_LIFECYCLE_AND_VALIDATOR_REGISTRY_V1.md` and its review.
- `OS1_TOPIC3_AUTHORITY_CLARIFICATION_U3_001_U3_002_V1.md` and its review.
- `OS1_ENUMERATION_AND_MAP_PRODUCTION_V1.md` and its review.
- `OS1_DESIGN_AUTHORITY_RECONCILIATION.md` and its independent review.

All documents are under `docs/`. All twelve checkpoint-listed design/review
identities matched their recorded line counts, SHA-256 values, and committed
HEAD bytes. The checkpoint and reconciliation pair also matched HEAD.
The checkpoint is status/handoff evidence, not an independent amendment.
Retained prospective wording in approved predecessors is qualified by their
approving reviews and durable preservation.

# Independently derived outcome

PASS. Predecessors leave the isolated successful, stable, identity-valid,
non-alias regular-candidate observation without an applicable outcome when size
is representable but a required timestamp component is outside timestamp_v1.
One new narrowly scoped fatal error is the narrowest valid authority repair.

| Alternative | Independent determination |
|---|---|
| Existing rejection RECORD | None of the four closed kind/alias pairings applies. |
| Existing fatal error | No current semantic domain covers this isolated successful observation. |
| New narrow fatal error | Preferred: preserves schemas, source fidelity, rejection policy, and complete-RECORD requirements. |
| New narrow rejection | Requires a broader schema and nonfatal-policy amendment, including a fifth code and regular-file pairing. |
| Normalization/canonicalization | Changes observed components and violates unchanged source fidelity. |
| Protocol/platform impossibility guarantee | No inspected authority supplies one; an implementation assertion cannot create it. |

This conclusion follows the unchanged contracts independently. U4-001's
selection of fatality is not itself proof. An intrinsic candidate property
does not necessarily require a nonfatal outcome.

# Error-name review

PASS. `CANDIDATE_TIMESTAMP_UNREPRESENTABLE` precisely identifies the candidate
target, timestamp family, and representation failure. It implies neither a
failed syscall nor producer-clock acquisition.

`OBJECT_STAT_FAILED` remains failed stat; `TIMESTAMP_ACQUISITION_FAILED`
remains the T2/T3/T5 producer-clock domain; `CANDIDATE_SIZE_UNREPRESENTABLE`
remains size-only. An external outside-domain value is not an actual program
invariant violation, so `INTERNAL_INVARIANT_FAILED` supplies no fallback.
Initial identity, later replacement, and protected-state mutation errors
remain distinct. Evidence-input, output/publication, candidate I/O, FinderInfo,
resource-probe, hash, root, reservation, and completion/event errors retain
their exact targets and stages. No malformed output may be manufactured to
reach another error.

The four rejection domains remain unchanged: SYMLINK_NOFOLLOW for SYMLINK;
DIRECTORY_NOT_CANDIDATE for DIRECTORY; UNSUPPORTED_FILE_KIND for FIFO, SOCKET,
CHARACTER_DEVICE, BLOCK_DEVICE, or OTHER; FINDER_ALIAS_FILE for REGULAR_FILE
with Finder Type 616c6973. None covers the stated non-alias regular candidate.

# Trigger and timestamp-domain review

PASS. For exact mathematical integer seconds S and nanoseconds N:

```text
representable iff
    -9223372036854775808 <= S <= 9223372036854775807
    AND 0 <= N <= 999999999

outside-domain iff
    S < -9223372036854775808
    OR S > 9223372036854775807
    OR N < 0
    OR N > 999999999
```

All four required candidate timestamps are covered: birth_time,
modification_time, metadata_change_time, and pre-data-read access_time.
With the clarification's prerequisites satisfied, any outside-domain component
requires the sole new fatal member. Multiple invalid components still yield
the same public result. Passing establishes representability only.

No normalization, carry/borrow, clamping, rounding, wrapping, truncation,
rescaling, omission, substitution, epoch change, signedness reinterpretation,
or lossy conversion is permitted. Checks precede bounded conversion.
Negative in-range seconds remain valid; timestamp_v1 and its canonical spelling
are unchanged. No host-language bound replaces the protocol bounds.

# Boundary-case review

PASS. These mathematical comparisons assume every other prerequisite passes
and apply independently to each of the four timestamp fields.

| S | N | Result |
|---:|---:|---|
| -9223372036854775808 | 0 | PASS |
| 9223372036854775807 | 999999999 | PASS |
| 0 | 0 | PASS |
| -1 | 999999999 | PASS |
| -9223372036854775809 | 0 | CANDIDATE_TIMESTAMP_UNREPRESENTABLE |
| 9223372036854775808 | 0 | CANDIDATE_TIMESTAMP_UNREPRESENTABLE |
| 0 | -1 | CANDIDATE_TIMESTAMP_UNREPRESENTABLE |
| 0 | 1000000000 | CANDIDATE_TIMESTAMP_UNREPRESENTABLE |

All four outside-domain branches are covered, with inclusive valid endpoints.
(0, 1000000000) cannot become (1, 0); (0, -1) cannot become (-1, 999999999).
No claim is made that any platform produces these values.

# Local precedence review

PASS. U4-001 provides a compatible interval after successful acquisition,
applicable identity/continuity checks, and non-alias determination, before
resource/data/hash work. U4-002 explicitly adds the timestamp decision after
U4-001 size representability passes and before later resource-fork probing,
first data-fork read, hash passes, and successful-line construction.

This relative placement is approved new supplemental authority, not a claim
that predecessors already ordered the missing check. It preserves every
U4-001 dependency and both size-range failure branches. If size and timestamps
would both be outside their domains, U4-001 wins and this check is not reached.
FinderInfo failure/malformation and stable alias rejection remain earlier
controlling outcomes. Nonregular rejection pairings remain controlling too.

Earlier acquisition/identity/continuity failures retain their exact errors.
Later failures after the check passes retain their stage-specific outcomes;
bound sources cannot be replaced to manufacture timestamp unrepresentability.
Hypothetical failures of unattempted operations have no precedence.
No contradictory predecessor dependency was found. Stability is observation-
based through the decision, not an atomic snapshot or future-mutation proof.
Topic 4 retains the authorized source acquisition and concrete failure points.

# Public error contract review

PASS. The exact literal is `CANDIDATE_TIMESTAMP_UNREPRESENTABLE`, allocated
only to T4 inspect. Stdout is zero bytes. Stderr is exactly the following ASCII
text followed by one LF byte, with no other bytes:

```text
OS1_METADATA_HELPER_ERROR:CANDIDATE_TIMESTAMP_UNREPRESENTABLE
```

Equivalently, the escaped byte notation is
`OS1_METADATA_HELPER_ERROR:CANDIDATE_TIMESTAMP_UNREPRESENTABLE\n`.
Exit 70 follows directly from existing Topic-1 authority: every V2 error except
INVALID_INVOCATION and PATH_GRAMMAR_INVALID exits 70. The new member is neither
exception. No exit rule is inferred merely by analogy with U4-001.
No dynamic metadata, component/field identifier, value, path, ID, hash, OS
diagnostic, or candidate content is emitted.

# RECORD semantics review

PASS. No successful or rejection RECORD line is produced for the triggering
candidate. The failing invocation publishes neither its RECORD nor a valid
terminal completion. Earlier final evidence remains untouched.

Every successfully completed RECORD retains exact MAP entry_count coverage,
MAP-index-to-line correspondence, order, IDs, schemas, four rejection
codes/pairings, and bounds. There is no omission, substitute rejection, ERROR
line, renumbering, MAP shrinkage, or authoritative partial output. Zero entries
still yield the existing zero-byte RECORD behavior subject to other checks.
The 1124/626-byte line maxima and 4,603,904-byte complete maximum remain.

Conditional treatment of existing unpublished residue creates no buffering,
temporary-file creation, cleanup, retry, or publication procedure. Partial
unpublished bytes remain nonauthoritative. Source fidelity and schema-shaped
bytes are distinct: validators cannot infer falsified external sources from
normalized but internally consistent bytes alone.

# Vocabulary arithmetic

PASS. Independent enumeration found 55 distinct original Topic-1 members,
four distinct additions in the error-taxonomy amendment, and one U4-001
addition: 55 + 4 + 1 = 60 approved members before U4-002.
U4-002 adds exactly one distinct member, yielding 61 after approval.
No member is removed or renamed. Retaining OS1_METADATA_HELPER_ERRORS_V2
follows the approved additive-amendment precedent; no version change is required.
Vocabulary recognition does not grant a new operation or event allocation.

# Validator-registry review

PASS. Independently counted exactly 42 unique contiguous rows, V02-001 through
V02-042. No new row, predicate, dependency, command applicability, or ordering
change is required. C3's approved V02-030 correction remains intact.
V02-041 remains producer-clock acquisition; V02-042 remains constructed-output
integrity and does not acquire or classify live candidate timestamp values.
Expanded public-error recognition is distinct from changing validator predicates.
Malformed supplied RECORD timestamps retain evidence-schema/canonical errors;
publication checks and RECORD_INCOMPLETE retain their existing domains.

# Amendment-surface review

PASS. The proposal addresses timestamp bounds/sources, successful versus failed
stat, mandatory partitions, complete vocabulary/count statements, producer-
clock allocation, U4-001 exclusions and precedence, validator boundaries,
cross-topic delegation, and forward-compatibility claims.
Explicit count qualifications prevent older complete tables from becoming
competing current totals. Historical subset counts and review verdicts remain
historical. Symbolic vocabulary references do not authorize event behavior.
No normative omission or hidden contradiction was found.

# Topic-boundary review

PASS. Topic 3 enumeration/MAP remains unchanged. Topic-4 mechanics remain
delegated: RA01–RA19, full acquisition, remaining failure sequencing,
resource/data/hash mechanics, and RECORD storage/publication are not designed
here. Topic 5 completion/events and Topic 6 fixtures/fault injection/conformance
remain unchanged. Corrective implementation remains unauthorized.

Adversarial review found no self-approval, circular reliance on U4-001,
unsupported platform premise, normalization loophole, ambiguous mathematical
type, missing timestamp, endpoint defect, precedence/alias contradiction,
semantic overlap, broadened rejection/invariant domain, totality contradiction,
count/version error, privacy leak, later-topic mechanics leakage, or
implementation-critical ambiguity.

# Findings

BLOCKERS: 0

IMPORTANT FINDINGS: 0

MINOR FINDINGS: 1

The sole MINOR is editorial: proposal line 380 labels S1R line 158 as
"Error/exit review arithmetic"; the line actually belongs to
"Vocabulary-count review". The cited line number and arithmetic are correct.
This does not affect implementation behavior. No correction is required for
approval. The optional label correction was not made: the approved clarification
is intentionally preserved unchanged to retain its exact independently reviewed
identity.

# Final verdict

U4-002 predecessor authority is approved and closed for the exact 524-line
clarification identity recorded above, including its narrow fatal domain and
local dependencies. Approval does not resume Topic 4 or authorize implementation.

U4-002 INDEPENDENT CLARIFICATION REVIEW: APPROVED

BLOCKERS: 0

IMPORTANT FINDINGS: 0

MINOR FINDINGS: 1

U4-002 AUTHORITY CLOSED: YES

TOPIC 4 MAY RESUME: NO

CORRECTIVE IMPLEMENTATION AUTHORIZED: NO

# Preservation status

The completed independent review required durable preservation of the unchanged
approved clarification and this review. This artifact is created only in the
subsequently authorized preservation task, together with the checkpoint update.
The independent review's final target/authority identities and protected
document hashes were unchanged; its index was empty and baseline synchronized.

The user has subsequently inserted a project-management gate before any further
Topic-4 authorization. This process change does not alter the review verdict.
U4-001 remains approved and preserved. Topic 4 has not resumed. Preservation
does not itself perform the release-path rebaseline or any later design work.

NEXT REQUIRED ACTION:
PHOENIX 0.1 RELEASE-PATH REBASELINE BEFORE FURTHER OS1 TOPIC-4 WORK
