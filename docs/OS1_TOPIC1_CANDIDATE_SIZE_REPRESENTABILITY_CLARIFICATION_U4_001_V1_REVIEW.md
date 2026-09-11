# Review identity

Independent design-authority review of the U4-001 predecessor clarification,
completed 2026-09-11. This artifact faithfully preserves the approved session
review in the repository structure requested by the preservation task. It is
not a new review, a revision of the clarification, or new normative authority.
The conclusions were re-derived from committed predecessor authority rather
than accepted from the draft's author self-review.

Review target:
[OS1_TOPIC1_CANDIDATE_SIZE_REPRESENTABILITY_CLARIFICATION_U4_001_V1.md](OS1_TOPIC1_CANDIDATE_SIZE_REPRESENTABILITY_CLARIFICATION_U4_001_V1.md).

# Repository baseline

The independent review verified at start and finish:

- Working directory: `/Users/kurtheiden/Developer/phoenix-studio-vision`.
- HEAD, main, and origin/main:
  `f1cafd0b73136cf7138dd6dbdbde37507286b48c`.
- Divergence against both main and origin/main: `0/0`.
- Index: empty.
- Complete working-tree status: the two pre-existing modified documents and
  the eight untracked files listed under Preservation consequence below.

The clarification identity and protected document digests remained unchanged.
No file was changed and no review file was created during the independent
review itself. No tests, staging, commits, or pushes were performed. Protected
helper implementation and candidate contents were not inspected.

# Target identity

The target matched both before and after independent review:

- Target lines: **463**.
- Target SHA-256:
  `d656d2e25719457bdfc80c7128ef24d0c803f9b819cbb55e42ad2975361e75c3`.

Approval is limited to that exact target and its stated narrow authority
effect. The target was unapproved when reviewed; author self-review was not
treated as approval evidence.

# Authority inspected

The committed authority at the baseline was independently consumed, including
the relevant provisions of each design as qualified by its approving review:

- [OS1_TARGETED_RESPEC_RESUME_CHECKPOINT.md](OS1_TARGETED_RESPEC_RESUME_CHECKPOINT.md).
- [OS1_CANONICAL_EVIDENCE_SCHEMAS_AND_ERRORS_V1.md](OS1_CANONICAL_EVIDENCE_SCHEMAS_AND_ERRORS_V1.md)
  and [its approving review](OS1_CANONICAL_EVIDENCE_SCHEMAS_AND_ERRORS_V1_REVIEW.md).
- [OS1_TOPIC1_ERROR_TAXONOMY_CLARIFICATION_FOR_TOPIC2_V1.md](OS1_TOPIC1_ERROR_TAXONOMY_CLARIFICATION_FOR_TOPIC2_V1.md)
  and [its approving review](OS1_TOPIC1_ERROR_TAXONOMY_CLARIFICATION_FOR_TOPIC2_V1_REVIEW.md).
- [OS1_CUSTODY_AUTHORITY_LIFECYCLE_AND_VALIDATOR_REGISTRY_V1.md](OS1_CUSTODY_AUTHORITY_LIFECYCLE_AND_VALIDATOR_REGISTRY_V1.md)
  and [its approving review](OS1_CUSTODY_AUTHORITY_LIFECYCLE_AND_VALIDATOR_REGISTRY_V1_REVIEW.md).
- [OS1_TOPIC3_AUTHORITY_CLARIFICATION_U3_001_U3_002_V1.md](OS1_TOPIC3_AUTHORITY_CLARIFICATION_U3_001_U3_002_V1.md)
  and [its approving review](OS1_TOPIC3_AUTHORITY_CLARIFICATION_U3_001_U3_002_V1_REVIEW.md).
- [OS1_ENUMERATION_AND_MAP_PRODUCTION_V1.md](OS1_ENUMERATION_AND_MAP_PRODUCTION_V1.md)
  and [its approving review](OS1_ENUMERATION_AND_MAP_PRODUCTION_V1_REVIEW.md).
- [OS1_DESIGN_AUTHORITY_RECONCILIATION.md](OS1_DESIGN_AUTHORITY_RECONCILIATION.md)
  and [its independent review](OS1_DESIGN_AUTHORITY_RECONCILIATION_REVIEW.md).

All ten checkpoint-listed predecessor design/review file identities matched
their exact committed line counts and SHA-256 values. The draft's citations
and interpretations were checked rather than treated as proof.

# Independent analysis

The review independently confirmed the seam: Topic 3 includes the candidate
without size-based filtering; Topic 1 requires faithful stable st_size within
0..9007199254740991; its four rejection pairings do not cover the stated
non-alias regular file. The clarification supplies exactly one new fatal
domain rather than repurposing a predecessor error.

The controlling provisions are Topic-1 RECORD rules beginning at baseline
line 477, its mandatory error partitions beginning at line 713, the approved
prior amendment section 5, and Topic-3 inclusion rules in section 7.

# Outcome choice

**PASS.** A fifth rejection would require a new canonical rejection value and
regular-file pairing. One fatal domain preserves those closed schemas. One
candidate prevents the command from producing its required complete RECORD
under the unchanged schemas.

Predecessors do not require every intrinsic candidate characteristic to yield
a rejection. The prior amendment already distinguishes successful external
observations that cannot supply representable protocol values from syscall
failures. The chosen fatal inspect result preserves existing success and
rejection semantics.

# Error-domain review

**PASS.** CANDIDATE_SIZE_UNREPRESENTABLE is triggered only by the specified
successful external candidate-size observation after its prerequisites pass.
The domain excludes failed stat, identity/mutation failures, internal defects,
supplied-evidence parsing, and executable-authority inputs. It cannot operate
as an uncategorized fallback or expand OBJECT_STAT_FAILED,
INTERNAL_INVARIANT_FAILED, or existing candidate errors.

# Range review

**PASS.** The protocol range remains exactly 0..9007199254740991.

| Stable S, with all prerequisites satisfied | Result |
|---|---|
| -1 | CANDIDATE_SIZE_UNREPRESENTABLE, fatal inspect error |
| 0 | Representability check passes |
| 9007199254740991 | Representability check passes |
| 9007199254740992 | CANDIDATE_SIZE_UNREPRESENTABLE, fatal inspect error |

The negative branch closes the same protocol domain as the upper-bound-outside
case. No inspected predecessor guarantees its impossibility, and the draft
makes no platform-specific claim. Passing the range check does not guarantee
inspection success. No programming-language limit supplies the bound.

# Precedence review

**PASS.** Successful acquisition, identity/continuity, and non-alias
determination precede the new check. Resource probing, data reads, hashing,
and successful-line construction follow it. No preserved predecessor sequence
contradicts this local constraint.

Every nearby row was checked: earlier fatal prerequisites retain their errors;
stable nonregular/alias outcomes retain their rejection pairings. Later size
changes remain mutation. Hypothetical failures of unattempted operations
cannot displace the size error. The new member does not compete with earlier
failed prerequisites or replace later independent failures after the range
check passed.

# RECORD completeness review

**PASS.** Complete RECORDs still cover every MAP entry in order. This fatal
branch publishes no RECORD and emits no rejection RECORD line; unpublished
residue remains nonauthoritative. No omission, replacement line, renumbering,
partial success, or zero-entry change is permitted. Earlier final evidence
remains untouched. No valid terminal completion is produced for the failure.

# Public error contract review

**PASS.** The approved outcome is CANDIDATE_SIZE_UNREPRESENTABLE, a fatal
inspect result with exit **70** and **zero stdout bytes**. Stderr is exactly
the following ASCII text followed by one LF byte, 0x0a:

```text
OS1_METADATA_HELPER_ERROR:CANDIDATE_SIZE_UNREPRESENTABLE
```

There are no additional stderr bytes or dynamic metadata. Allocation is T4
inspect only. No operational-event trigger or Topic-5 mapping is authorized.

# Vocabulary-count review

**PASS.** Independent enumeration of the committed tables produced:

- Original Topic 1: 55 distinct members.
- Prior approved amendment: four distinct additions, with no duplication.
- Pre-U4-001 effective vocabulary: 59 members.
- Proposed distinct addition: CANDIDATE_SIZE_UNREPRESENTABLE.
- Approved effective vocabulary: **55 + 4 + 1 = 60 members**.

Retaining OS1_METADATA_HELPER_ERRORS_V2 is consistent with the prior approved
additive amendment. No governing rule requires a taxonomy-name change.
Searches independently confirmed the current-total occurrences identified by
the draft; historical counts remain historical and checkpoint status requires
refresh during preservation.

One indirect reference received particular scrutiny: the Topic-1 operational-
event schema's error field references the V2 vocabulary symbolically. Its
membership recognition follows the amended set; its field definition and
digest rules remain unchanged. Vocabulary membership does not authorize an
operational event or supply a Topic-5 operation-to-error mapping.

# Validator review

**PASS.** Independently counted 42 unique rows, V02-001 through V02-042.
None requires modification; no new V02 row is added. V02-042 remains the
constructed-output invariant check. RECORD parsing retains its existing
schemas and rejection values; public-error recognition must consume the
expanded vocabulary. Future implementation/test recognition is distinct from
changing validator design authority.

Validators cannot prove that an in-range value was not clamped from serialized
bytes alone. The draft preserves the distinction between schema validation
and source fidelity rather than claiming detection of fabricated evidence.

# Amendment-surface review

**PASS — COMPLETE.** Every amendment-surface row was checked against its
predecessor anchor. The count inventory identifies the affected current-total
statements. Historical wording remains historical, and checkpoint status
requires refresh during preservation. No normative omission or blocking
contradiction was found. No predecessor edit is required to change an existing
schema, rejection pairing, enumeration procedure, or custody procedure.

# Topic-boundary review

**PASS.** The clarification fixes the new outcome and local dependencies
without defining RA01–RA19, terminal events, fixtures, enumeration, or custody
procedures. Topic 4 receives a fixed fatality, literal, range, output contract,
and nearby precedence; its remaining operation-specific mechanics stay
delegated. This approval does not resume Topic 4.

# Narrowness review

**PASS.** No general file-size policy, sparse-file policy, timestamp outcome,
retry, cleanup, or reservation transition is introduced. The rule does not
create a physical-allocation, project-format, memory-availability, or
resource-fork-size policy. Existing schemas, four rejection codes, enumeration,
custody, and the 42-validator registry remain unchanged.

# Internal-consistency review

**PASS.** No implementation-critical ambiguity remains within U4-001.
"Stable" explicitly means required observations through the decision point;
the remaining observation mechanics are legitimately delegated to Topic 4.
The checked terminology and dependency order do not require an atomic
snapshot or a guarantee about future mutations. The review makes no claim
that unfinished Topic 4 is otherwise complete.

# Findings

No blocker, important finding, or minor finding requires revision.

**BLOCKERS: 0**

**IMPORTANT FINDINGS: 0**

**MINOR FINDINGS: 0**

# Approval verdict

Approved for the exact target identity and its stated narrow authority effect.
U4-001 has exactly one outcome under the approved clarification.

**U4-001 INDEPENDENT REVIEW: APPROVED**

**U4-001 AUTHORITY CLOSED: YES**

**TOPIC 4 MAY RESUME: NO**

**CORRECTIVE IMPLEMENTATION AUTHORIZED: NO**

# Preservation consequence

The review-stage TOPIC 4 MAY RESUME: NO records that durable preservation was
still outstanding, not an approval defect. The review's next required action
was **DURABLY PRESERVE APPROVED U4-001 CLARIFICATION AND REVIEW**.
After the separately authorized preservation commit is pushed and verified
synchronized, the updated checkpoint governs the next Topic-4 design task.
This artifact neither performs that design task nor authorizes implementation.

Complete final status recorded by the independent review, before creation of
this preservation artifact:

```text
On branch main
Your branch is up to date with 'origin/main'.

Changes not staged for commit:
	modified:   docs/DECISIONS.md
	modified:   docs/ROADMAP.md

Untracked files:
	docs/CONTROLLED_TRACK3_2_MIDI_CHANNEL_CHANGE.md
	docs/OS1_TOPIC1_CANDIDATE_SIZE_REPRESENTABILITY_CLARIFICATION_U4_001_V1.md
	docs/OS1_TOPIC3_BLOCKER_REVIEW_U3_001_U3_002.md
	tools/os1-metadata-helper/README.md
	tools/os1-metadata-helper/Sources/main.swift
	tools/os1-metadata-helper/Tests/test_helper.py
	tools/os1-metadata-helper/build.sh
	tools/os1-metadata-helper/test.sh

no changes added to commit
```
