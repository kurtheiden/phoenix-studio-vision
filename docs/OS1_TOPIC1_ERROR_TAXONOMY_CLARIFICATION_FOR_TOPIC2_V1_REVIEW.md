# Final Fresh Independent Re-Review of Topic-1 Error-Taxonomy Amendment V1

# 1. Review identity and scope

Reviewed amendment:
`docs/OS1_TOPIC1_ERROR_TAXONOMY_CLARIFICATION_FOR_TOPIC2_V1.md`

- line count: `383`
- SHA-256:
  `b0443f34180aa2dd5aa721bbd5f77333fc4d9a1c39bf03aca938fdb5ba144f53`

The identity gate passed before substantive review.

The earlier 265-line review and 259-line re-review were read only as historical
review evidence and independently identity-verified. The unapproved Topic-2
draft was identity-verified at 834 lines and consulted only as needed for
U2-001 through U2-003. The durable resume checkpoint, reconciliation/review,
and approved Topic-1 design/review were read as governing authority. No lost
`/tmp` design was used.

This review performed no correction, repository edit, implementation, reserve
or OS1 candidate access, reference MIDI activity, or provenance lock.

# 2. Final decision

The amendment is necessary, minimal, deterministic, internally consistent,
correctly allocated, compatible with durable Topic 1, sufficient for Topic 2,
and forward-compatible for Topics 3–5. No unresolved implementation-critical
taxonomy decision remains.

**TOPIC-1 ERROR AMENDMENT APPROVED: YES**

# 3. I-001 — independent allocation verification

## 3.1 Executable-authority input open/read

Both new executable-input members have the same exact allocation:

| Topic | Applies | Independent authority determination |
|---|---|---|
| T2 | YES | T2 creates/revalidates executable authority from explicit raw source, build-command, helper-executable, and custody-executable inputs. |
| T3 | YES | MAP authority carries authorization/current-helper identity and T3 must acquire the current helper identity before production. Existing executable mismatch was already allocated to T3. |
| T4 | YES | RECORD authority requires the current validated helper and T4 must acquire that helper identity before candidate/RECORD work. Existing executable mismatch was already allocated to T4. |
| T5 | NO | T5 consumes validated helper correlations in completion/event authority. Durable authority neither assigns current executable mismatch to T5 nor requires T5 independently to open/read/rehash a raw helper or custody executable. |

Verified allocation:

- `EXECUTABLE_AUTHORITY_INPUT_OPEN_FAILED`: `T2,T3,T4`
- `EXECUTABLE_AUTHORITY_INPUT_READ_FAILED`: `T2,T3,T4`

## 3.2 Repository identity

Only T2 independently acquires the current repository identity for executable
review/preflight. T3 MAP, T4 RECORD, and T5 completion/event schemas contain no
repository identity and consume prior authority rather than reacquiring it.

Verified allocation:

- `REPOSITORY_IDENTITY_FAILED`: `T2`

## 3.3 Producer timestamp

T2 lifecycle objects have required producer timestamps. T3 MAP requires
`enumerated_at`. T5 completion/event require `completed_at` and `observed_at`.
T4 RECORD contains candidate filesystem stat times, not a producer-clock
timestamp; failure there remains under object/stat/race rules.

Verified allocation:

- `TIMESTAMP_ACQUISITION_FAILED`: `T2,T3,T5`

The amendment correctly states that `May raise` follows an already-authorized
operation and grants no filesystem or lifecycle authority. T6 is correctly
excluded because conformance testing does not make a runtime error a T6
operation.

**I-001 CLOSED**

# 4. I-002 — independent descriptor-stat verification

Approved Topic 1 defines `OBJECT_STAT_FAILED` by operation: a required
object/path stat failed. Option B preserves that existing meaning. Adding T2 to
its existing T3/T4/T5 allocation is an applicability extension for the same
operation, not a semantic reinterpretation.

For an explicit raw executable-authority input:

1. A no-follow open syscall failure occurs before any descriptor/stat/read
   result and uses `EXECUTABLE_AUTHORITY_INPUT_OPEN_FAILED`.
2. Once a descriptor exists, failure of the required descriptor-stat syscall
   is exactly a required object stat failure and uses only
   `OBJECT_STAT_FAILED`.
3. A successful stat that reports a nonregular kind is not a stat syscall
   failure. It is failure to acquire the required usable raw input and uses
   `EXECUTABLE_AUTHORITY_INPUT_OPEN_FAILED`.
4. Only after successful regular-file stat can complete-byte acquisition begin;
   its failures use `EXECUTABLE_AUTHORITY_INPUT_READ_FAILED`.

The amendment expressly excludes descriptor-stat syscall failure from the new
OPEN member and expressly excludes wrong-kind observation from the existing
stat-failure member. `OBJECT_STAT_FAILED` is now allocated to exactly
`T2,T3,T4,T5`, so the same result is available in every applicable workflow.

There is no stale Option-A wording, T2 omission, or T3/T4 ambiguity.

**I-002 CLOSED**

# 5. Exact raw-input operation matrix

| Operation/result | Sole public error | Excluded alternatives |
|---|---|---|
| no-follow open syscall failure | `EXECUTABLE_AUTHORITY_INPUT_OPEN_FAILED` | No descriptor exists, so stat/read/identity/race cannot apply. |
| descriptor-stat syscall failure | `OBJECT_STAT_FAILED` | New OPEN expressly excludes the syscall failure; evidence errors are serialized-input-only. |
| successful stat reports wrong kind | `EXECUTABLE_AUTHORITY_INPUT_OPEN_FAILED` | Stat succeeded, read did not begin, and `EVIDENCE_NOT_REGULAR_FILE` is serialized-input-only. |
| complete read failure | `EXECUTABLE_AUTHORITY_INPUT_READ_FAILED` | Open and regular stat succeeded. |
| representable byte-count bound exhaustion | `EXECUTABLE_AUTHORITY_INPUT_READ_FAILED` | It prevents complete digest-byte acquisition after usable-object acquisition. |
| required rewind/reread failure | `EXECUTABLE_AUTHORITY_INPUT_READ_FAILED` | It is post-open/post-stat byte acquisition, not candidate seek or publication reread. |
| required complete digest bytes cannot be acquired | `EXECUTABLE_AUTHORITY_INPUT_READ_FAILED` | No digest comparison exists yet; later mutation remains under existing errors. |
| later device/inode/kind discontinuity | `OBJECT_CHANGED_OR_REPLACED` | Initial acquisition succeeded; OPEN/STAT/READ stages are past. |
| later same-object protected observation changes | `RACE_OR_MUTATION_DETECTED` | Identity persists; this is not initial byte acquisition. |

All rows are mutually exclusive by operation stage and observed result.

# 6. Wrong-kind and existing-error integrity

No existing generic wrong-kind member governs raw executable-authority inputs.
`EVIDENCE_NOT_REGULAR_FILE` is restricted by Topic-1 rule 2 to supplied
serialized evidence. `OBJECT_STAT_FAILED` means failure to perform a required
stat, not a successfully returned undesired kind. Candidate/root errors belong
to different targets. The new OPEN member may therefore own successful
wrong-kind rejection without collision.

The OPEN member covers exactly raw no-follow open failure and successful
wrong-kind observation. It excludes descriptor-stat failure, byte reads,
bound exhaustion, rewind/reread, digest-byte acquisition, and later mutation.

The READ member begins only after successful regular-file stat and covers
complete read, representable bound, required rewind/reread, and stable complete
digest-byte acquisition failures. Successfully acquired unequal executable
identity remains `EXECUTABLE_IDENTITY_MISMATCH`; later replacement/mutation
retains the object/race errors.

No existing member is removed, renamed, or semantically changed.

# 7. Repository and timestamp boundaries

`REPOSITORY_IDENTITY_FAILED` validly groups repository open, resolution, read,
and `git_oid_sha1` lexical failure as one pre-comparison operation: acquire one
valid current repository identity. After a valid identity exists, disagreement
with predecessor authority is `EVIDENCE_CORRELATION_MISMATCH`. Malformation in
a supplied serialized object remains under evidence schema/canonical errors.

`TIMESTAMP_ACQUISITION_FAILED` validly groups producer clock acquisition,
conversion, and Topic-1 representability/range failure before object
construction. Malformed serialized timestamps remain evidence schema/canonical
failures. Candidate stat-time acquisition is not absorbed.

Both boundaries are deterministic and nonoverlapping.

# 8. Necessity and minimality

The existing 55-member vocabulary cannot cover raw executable-authority input
open/read, current repository identity acquisition, or producer timestamp
acquisition without violating Topic-1 partitions. Rule 2 reserves
`EVIDENCE_*` for explicitly supplied serialized evidence. Rule 1 forbids
`INTERNAL_INVARIANT_FAILED` for ordinary OS/acquisition failure. Existing
candidate, root, publication, completion, event, identity-comparison, and race
members apply to other targets or later stages.

Four new members remain minimal. Raw open and read are distinct system stages;
repository and timestamp are distinct target domains; neither can merge with
raw input errors. Repository substeps can share one pre-comparison member, and
timestamp substeps can share one pre-construction member. Existing
`OBJECT_STAT_FAILED` closes the intervening stat stage, so no fifth member is
needed.

**NECESSITY: PASS**

**MINIMALITY: PASS — FOUR NEW MEMBERS**

# 9. Determinism and vocabulary integrity

Topic-1 precedence remains authoritative. Serialized evidence uses only its
existing `EVIDENCE_*` sequence. Raw inputs use OPEN, STAT, READ, then existing
identity/race results in operation order. Repository and timestamp domains are
separate. `INTERNAL_INVARIANT_FAILED` remains unavailable for ordinary I/O.

No reachable failure domain addressed by this amendment lacks an enum, and no
single semantic failure has two applicable enums.

Exactly four members are added to the original 55:

1. `EXECUTABLE_AUTHORITY_INPUT_OPEN_FAILED`
2. `EXECUTABLE_AUTHORITY_INPUT_READ_FAILED`
3. `REPOSITORY_IDENTITY_FAILED`
4. `TIMESTAMP_ACQUISITION_FAILED`

No member is removed or renamed. `55 + 4 = 59`.

**DETERMINISTIC MAPPING: PASS**

**VOCABULARY COUNT: 59**

# 10. Topic-1 regression audit

The amendment changes no serialized schema, field name/order/type, canonical
byte, digest domain, RECORD grammar, privacy rule, pathname nonauthority,
stderr grammar, or exit grammar. New errors use the existing public envelope
and exit 70. The maximum successful RECORD line remains 1124 bytes including
LF; maximum complete RECORD remains 4,603,904 bytes.

The only changes are four additive, disjoint public members and allocation of
existing `OBJECT_STAT_FAILED` to T2 for its unchanged operation.

**TOPIC-1 REGRESSION: NONE**

# 11. Topic-2 sufficiency and forward compatibility

Approval resolves U2-001, U2-002, and U2-003 without another Topic-1 taxonomy
decision. Topic 2 can map raw inputs, repository identity, timestamp
acquisition, and descriptor stat exhaustively.

T3/T4 can use the same raw-input errors and existing stat error; T3/T5 can use
the timestamp error. T3–T5 do not reacquire repository identity under current
durable authority. No further applicability-only amendment, schema, canonical
form, digest domain, hidden state, or incompatible reinterpretation is needed.

**TOPIC-2 SUFFICIENCY: PASS**

**FORWARD COMPATIBILITY: PASS**

# 12. Scope and adversarial audit

The amendment allocates errors but authorizes no operation. It defines no
Topic-2 lifecycle mechanics, T3 enumeration/MAP production, T4 inspect/RECORD
production, T5 terminal publication, or T6 test cases. It introduces no hidden
state, privacy expansion, pathname authority, or threat-model expansion.

Adversarial search found no stale T2-only new-member allocation, stale Option-A
text, pre-I-002 stat wording, contradictory matrix, unmapped stat failure,
wrong-kind ambiguity, T2 stat omission, T5 executable over-allocation, T4
timestamp over-allocation, repository over-allocation, hidden fifth-error need,
or operation-as-authority statement.

Unresolved implementation-critical decisions: **0**.

Findings:

- `BLOCKER`: **0**
- `IMPORTANT`: **0**
- `MINOR`: **0**

# 13. Final verdict

I-001 and I-002 are closed. The four-error amendment is complete,
deterministic, minimal, compatible, and ready to become durable approved
authority through the separately authorized preservation process.

**TOPIC-1 ERROR AMENDMENT APPROVED: YES**

**TOPIC 2 MAY RESUME AUTHORING AFTER DURABLE PRESERVATION: YES**

**CORRECTIVE IMPLEMENTATION AUTHORIZED: NO**
