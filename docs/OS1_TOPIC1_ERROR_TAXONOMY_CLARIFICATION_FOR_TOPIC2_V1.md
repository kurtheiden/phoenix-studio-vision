# OS1 Topic-1 Error-Taxonomy Clarification for Topic 2 V1

# 1. Purpose and authority status

This document is a prospective, narrow amendment to Topic-1 error-taxonomy
authority prompted by authoring OS1 Targeted Re-Specification Topic 2. It
addresses only U2-001, U2-002, and U2-003 recorded in the unapproved Topic-2
draft.

This document is not approved authority until it receives an independent
review and explicit approval. It does not edit or silently reinterpret
`OS1_CANONICAL_EVIDENCE_SCHEMAS_AND_ERRORS_V1.md` or its approving review. If
approved, it supplements only the public error vocabulary and its semantic
partition as stated here.

The Topic-2 draft remains unapproved and unchanged. This amendment does not
continue Topic-2 lifecycle design, begin its review, or authorize
implementation. Topics 3 through 6 remain outside scope. Reserve, authentic
candidate, Studio Vision, reference MIDI, and provenance-lock activity remain
unauthorized.

**CORRECTIVE IMPLEMENTATION AUTHORIZED: NO**

# 2. Exact blocker analysis

The analysis uses only the durable reconciliation, its review, the approved
Topic-1 design/review, the durable resume checkpoint, and the unapproved
Topic-2 draft. It uses no lost `/tmp` artifact.

## 2.1 U2-001 — raw executable-authority input I/O

Topic-2 executable review must explicitly open and completely read raw helper
source, custody source, helper/custody build-command files, A/B helper and
custody executables, and later current helper/custody executables. These files
are inputs used to construct or verify Topic-1 `file_digest_v1` and executable
digest fields. They are not serialized OS1 evidence objects.

Approved Topic-1 mandatory semantic-partition rule 2 says that `EVIDENCE_*`
applies only while acquiring or validating an explicitly supplied serialized
evidence input. It expressly excludes other producer inputs. Therefore
`EVIDENCE_OPEN_FAILED`, `EVIDENCE_NOT_REGULAR_FILE`,
`EVIDENCE_SIZE_LIMIT_EXCEEDED`, and `EVIDENCE_READ_FAILED` cannot classify raw
source, build-command, or executable I/O.

Rule 1 says `INTERNAL_INVARIANT_FAILED` is only for a program invariant not
representable by another member and never wraps an ordinary OS, parse,
authorization, or publication failure. Raw-file open/read failure is ordinary
OS I/O and cannot use that fallback.

Rule 6 assigns specialized candidate errors only to candidate operations.
These executable-authority files are not candidates, so `CANDIDATE_OPEN_FAILED`
and `CANDIDATE_READ_FAILED` cannot apply. The existing
`EXECUTABLE_IDENTITY_MISMATCH` describes a comparison mismatch after identity
acquisition; it does not describe inability to acquire bytes.

Consequently U2-001 has no compatible current member.

## 2.2 U2-002 — repository identity operation

Topic-1 executable-review and preflight schemas require
`repository_commit: git_oid_sha1`, and Topic 2 must acquire the current reviewed
repository identity and lexically validate its exact 40-lowercase-hex form.
Failure to open/resolve/read that identity is not serialized-evidence I/O.
Successful acquisition of a different well-formed commit can use
`EVIDENCE_CORRELATION_MISMATCH` when compared with an already validated
artifact, but acquisition failure and a lexically invalid acquired value occur
before such a comparison.

Rule 2 bars `EVIDENCE_OPEN_FAILED`, `EVIDENCE_READ_FAILED`,
`EVIDENCE_SCHEMA_INVALID`, and `EVIDENCE_CANONICAL_BYTES_INVALID`, because the
repository observation is not a supplied serialized evidence object. Rule 1
bars `INTERNAL_INVARIANT_FAILED`. `EXECUTABLE_IDENTITY_MISMATCH` concerns
source/runtime/review executable identity, not Git repository identity.

Consequently U2-002 has no compatible current member.

## 2.3 U2-003 — required timestamp acquisition

Every Topic-2-produced Topic-1 authority schema has a required timestamp
component. Topic 2 therefore must acquire a timestamp before constructing
canonical bytes. Failure to acquire clock data is ordinary OS failure, not a
malformed supplied serialized artifact, an authority mismatch, a publication
failure, or an invariant violation.

Rule 2 excludes every `EVIDENCE_*` error and rule 1 excludes
`INTERNAL_INVARIANT_FAILED`. None of the existing specialized root, object,
candidate, reservation, publication, completion, or event errors names this
producer-input operation.

Consequently U2-003 has no compatible current member.

# 3. Existing-vocabulary coverage analysis

The following are all plausible existing families or members. Exclusion based
on a binding semantic partition is incompatibility, not naming awkwardness.

| Existing error/family | Raw source/build/executable open/read | Repository acquisition/lexical validation | Timestamp acquisition | Determination |
|---|---|---|---|---|
| `INVALID_INVOCATION` | no | no | no | Arguments may be valid; runtime acquisition failed. |
| `PATH_GRAMMAR_INVALID` | no | no | no | Applies only to malformed caller path form, not a valid path whose operation fails; timestamp has no path. |
| `EVIDENCE_OPEN_FAILED` | no | no | no | Rule 2 restricts it to supplied serialized evidence. |
| `EVIDENCE_NOT_REGULAR_FILE` | no for open/read failure | no | no | Rule 2 restriction; kind mismatch is also distinct from open/read. |
| `EVIDENCE_SIZE_LIMIT_EXCEEDED` | no for open/read failure | no | no | Rule 2 restriction and wrong semantic stage. |
| `EVIDENCE_READ_FAILED` | no | no | no | Rule 2 restriction. |
| `EVIDENCE_UTF8_INVALID` through `EVIDENCE_CANONICAL_BYTES_INVALID` | no | no | no | These parse supplied serialized evidence; raw bytes, a Git observation, and a clock result are not such evidence. |
| `EVIDENCE_DIGEST_MISMATCH` | no | no | no | Requires acquired bytes and a digest comparison. |
| `EVIDENCE_CORRELATION_MISMATCH` | no for acquisition | only after a well-formed current commit is acquired and differs | no | It remains correct for comparison, but cannot cover acquisition/lexical failure. |
| `EVIDENCE_UNAUTHORIZED` | no | no | no | Requires canonical, digest-valid, correlated evidence lacking verdict/state/authority. |
| `EXECUTABLE_IDENTITY_MISMATCH` | only a successfully acquired source/runtime/review identity mismatch; not open/read | no | no | Extending it to I/O or repository/time would incompatibly change “mismatch.” |
| `ROOT_*` and `OUTPUT_PARENT_UNAUTHORIZED` | no | no | no | Reserved for root acquisition/identity/separation/parent domains. |
| `OBJECT_STAT_FAILED` | no for open/read; yes for the separate descriptor-stat syscall | no | no | Topic 1 already defines required object/path stat failure. This amendment adds T2 applicability but does not use it for open, wrong-kind, or read failure. |
| `OBJECT_IDENTITY_MISMATCH`, `OBJECT_CHANGED_OR_REPLACED`, `RACE_OR_MUTATION_DETECTED` | only after successful observations | only after successful observations if an applicable binding exists | no | They cannot represent initial acquisition failure. |
| `CANDIDATE_*`, `FINDERINFO_*`, resource-fork and hash errors | no | no | no | Specialized candidate domain excludes executable-authority/repository/time operations. |
| `TEMP_*`, `FINAL_*` | no | no | no | Apply to producer output publication, not producer input acquisition. |
| `COMPLETION_*`, `OPERATIONAL_EVENT_FAILED` | no | no | no | Topic-5 specialized outputs; wrong operation and scope. |
| `INTERNAL_INVARIANT_FAILED` | no | no | no | Rule 1 expressly forbids ordinary OS/acquisition failure. |

Concrete coverage conclusions:

- raw source open: no current member;
- raw source read: no current member;
- build-command open: no current member;
- build-command read: no current member;
- helper executable open/read: no current member for acquisition;
- custody executable open/read: no current member for acquisition;
- repository identity acquisition: no current member;
- repository identity lexical validation: no current member;
- required timestamp acquisition: no current member.

# 4. Decision

**B. MINIMAL ERROR-VOCABULARY AMENDMENT REQUIRED**

A compatible clarification-only result is impossible. Assigning these domains
to `EVIDENCE_*` would contradict rule 2; assigning them to
`INTERNAL_INVARIANT_FAILED` would contradict rule 1; and using an identity,
candidate, object-stat, or publication error would change its operation or
stage incompatibly.

Four new errors are the minimum that preserves deterministic stage identity:

1. raw executable-authority input open failure;
2. raw executable-authority input complete-read failure;
3. repository identity acquisition or lexical-validation failure; and
4. required timestamp acquisition failure.

Open and read remain distinct because the approved taxonomy already treats
those as different observable operation stages for serialized evidence and
candidate data. Repository resolution and lexical validation are one bounded
public domain because both mean that no valid current `git_oid_sha1` was
acquired; no downstream comparison may begin. Splitting them would add detail
not needed by Topic 2. Timestamp acquisition is unrelated to all three.

# 5. Exact normative amendment

If independently approved, the closed vocabulary is amended by exactly these
four literals:

| New literal | Exact semantic meaning | Final evidence from failing command | May raise |
|---|---|---|---|
| `EXECUTABLE_AUTHORITY_INPUT_OPEN_FAILED` | An explicitly supplied nonserialized helper/custody source, build-command, or executable input required to create or revalidate executable authority cannot be opened no-follow, or a successful descriptor stat reports a kind other than the required regular file. Descriptor-stat syscall failure is expressly excluded. | none | T2,T3,T4 |
| `EXECUTABLE_AUTHORITY_INPUT_READ_FAILED` | After such a raw input is opened as a regular file, it exceeds the Topic-1 representable complete-byte-count bound, or its complete bytes cannot be read, rewound when a complete reread is required, or stably acquired for its required complete digest. | none | T2,T3,T4 |
| `REPOSITORY_IDENTITY_FAILED` | The current repository Git object identity required for executable review/preflight cannot be opened, resolved, or read, or the acquired value is not exactly the required `git_oid_sha1` lexical form. | none | T2 |
| `TIMESTAMP_ACQUISITION_FAILED` | A producer cannot acquire a required `timestamp_v1` value with seconds and nanoseconds in their Topic-1 ranges before constructing an authority object. | none | T2,T3,T5 |

If approved, the existing `OBJECT_STAT_FAILED` row's `May raise` allocation is
amended from `T3,T4,T5` to exactly `T2,T3,T4,T5`. Its existing semantic meaning,
“required object/path stat failed,” is unchanged. This allocation supplies T2
with the already-correct generic result for raw executable-authority input
descriptor-stat syscall failure; it does not make wrong kind a stat failure.

The resulting `OS1_METADATA_HELPER_ERRORS_V2` vocabulary contains exactly
**59 members**. Its taxonomy identifier remains
`OS1_METADATA_HELPER_ERRORS_V2`; this amendment changes membership/count only
after approval and does not introduce another taxonomy identifier.

Every new member is fatal to the current command, produces no valid terminal
completion, writes zero stdout, and uses the unchanged public stderr envelope:

```text
OS1_METADATA_HELPER_ERROR:<ENUM>\n
```

Each new member exits with status `70`. None is an expected candidate
rejection or a RECORD line.

## 5.1 Exact precedence

1. Invocation and path grammar retain first precedence.
2. For a serialized evidence object, existing `EVIDENCE_*` rules retain
   exclusive precedence; the new executable-authority input errors never
   apply.
3. For an explicitly supplied raw executable-authority input, no-follow open
   syscall failure is `EXECUTABLE_AUTHORITY_INPUT_OPEN_FAILED`. After open, a
   descriptor-stat syscall failure is exclusively `OBJECT_STAT_FAILED`. A
   successful descriptor stat that reports a kind other than regular file is
   exclusively `EXECUTABLE_AUTHORITY_INPUT_OPEN_FAILED`; wrong kind is not a
   stat syscall failure. After a successful regular-file stat, failure while
   acquiring complete bytes, including an unrepresentable complete byte count,
   is `EXECUTABLE_AUTHORITY_INPUT_READ_FAILED`.
4. After raw bytes are successfully acquired, unequal source/runtime/review
   executable identities remain `EXECUTABLE_IDENTITY_MISMATCH`; later bound
   object replacement/mutation retains the existing object/race errors.
5. Repository open, resolution, read, and lexical validation use only
   `REPOSITORY_IDENTITY_FAILED`. After a valid `git_oid_sha1` is acquired, its
   disagreement with a validated predecessor remains
   `EVIDENCE_CORRELATION_MISMATCH`.
6. Clock acquisition/range failure before typed timestamp construction uses
   only `TIMESTAMP_ACQUISITION_FAILED`. A timestamp already present in supplied
   serialized evidence remains governed by existing schema/canonical errors.
7. Publication, root, reservation, candidate, completion, and operational-event
   partitions remain unchanged and exclude all four new members.
8. `INTERNAL_INVARIANT_FAILED` remains unavailable for every failure covered
   here.

## 5.2 Descriptor-stat decision — Option B

Topic 1 defines `OBJECT_STAT_FAILED` by operation: a required object/path stat
failed. That generic operation rule already covers descriptor stat regardless
of the surrounding T3/T4 workflow; only its original `May raise` allocation
omitted T2. Retaining it requires an applicability update, not a semantic
reinterpretation or new conflict.

The new OPEN member already combines no-follow open syscall failure with a
successful kind observation that rejects a nonregular object. Absorbing the
intervening descriptor-stat syscall failure would expand OPEN from its named
operation/result cases and collide with the exact existing stat domain.
Therefore Option B best preserves specialized-versus-generic precedence:

- open syscall failure -> `EXECUTABLE_AUTHORITY_INPUT_OPEN_FAILED`;
- descriptor-stat syscall failure -> `OBJECT_STAT_FAILED`;
- successful stat reporting wrong kind ->
  `EXECUTABLE_AUTHORITY_INPUT_OPEN_FAILED`;
- complete read, bound, rewind/reread, or digest-byte acquisition failure ->
  `EXECUTABLE_AUTHORITY_INPUT_READ_FAILED`.

The same mapping applies in T2, T3, and T4. Wrong kind is a successful stat
result, not a stat syscall failure. No fifth error is needed.

No existing serialized schema, field, field order/type, canonical byte,
digest domain, RECORD rule/limit, or privacy rule changes.

## 5.3 Cross-topic applicability audit

`May raise` allocates an already-defined semantic failure to a topic whose
authorized procedure must perform that operation. Allocation neither grants
filesystem access nor establishes a procedure that the owning topic does not
already require. Topic 6 tests runtime errors but is not itself allocated a
runtime error.

| Error | T2 | T3 | T4 | T5 | Durable-authority reason |
|---|---|---|---|---|---|
| `EXECUTABLE_AUTHORITY_INPUT_OPEN_FAILED` | yes | yes | yes | no | T2 creates/revalidates executable authority. Topic-1 MAP authority requires authorization/current-helper identity in T3, and RECORD authority requires the current validated helper in T4. Existing `EXECUTABLE_IDENTITY_MISMATCH` independently confirms executable checking belongs to T2/T3/T4. T5 completion/event authority consumes validated helper correlations and is not allocated current-executable identity checking by durable authority. |
| `EXECUTABLE_AUTHORITY_INPUT_READ_FAILED` | yes | yes | yes | no | The same required raw executable acquisition operations have distinct post-open complete-byte acquisition failures. No durable T5 rule independently reopens or rehashes the helper/custody executable. |
| `REPOSITORY_IDENTITY_FAILED` | yes | no | no | no | Current `repository_commit` acquisition belongs to executable review/preflight in T2. MAP, RECORD, completion, and operational-event schemas contain no repository-identity field, and durable authority does not require T3–T5 to acquire the repository identity independently. |
| `TIMESTAMP_ACQUISITION_FAILED` | yes | yes | no | yes | T2 authority objects require lifecycle timestamps; T3 MAP requires `enumerated_at`; T5 completion/event schemas require `completed_at`/`observed_at`. T4 RECORD has filesystem stat timestamps acquired through candidate stat, not a producer clock timestamp; its failures retain Topic-1 object/race mappings. |
| existing `OBJECT_STAT_FAILED` | yes | yes | yes | yes | The semantic domain remains every required non-root object/path stat syscall failure. Adding T2 closes raw executable-authority descriptor-stat acquisition; T3–T5 retain their approved allocation. Root-chain/output-parent stat remains excluded by existing Topic-1 specialization. |

The T5 helper fields in completion are correlations to a helper already
validated by the custody/run/inspect authority chain. Their presence does not,
without later durable authority saying otherwise, require T5 to open or read a
raw executable. If a later design attempted to add such an operation, it would
need separate authority rather than silently relying on this allocation.

# 6. Deterministic failure matrix

| Concrete failure domain | Exact sole error | Why no ambiguity remains |
|---|---|---|
| No-follow open syscall fails for any raw helper/custody source, build-command, or executable authority input | `EXECUTABLE_AUTHORITY_INPUT_OPEN_FAILED` | No descriptor exists; stat/read errors cannot apply. |
| Required descriptor-stat syscall fails for any opened raw executable-authority input | `OBJECT_STAT_FAILED` | Exact existing stat-operation domain; new OPEN error expressly excludes stat syscall failure. |
| Successful descriptor stat reports any kind other than regular file | `EXECUTABLE_AUTHORITY_INPUT_OPEN_FAILED` | This is wrong-kind rejection, not stat syscall failure; read has not begun. |
| Complete read syscall fails after successful regular-file stat | `EXECUTABLE_AUTHORITY_INPUT_READ_FAILED` | Post-open/post-stat byte-acquisition stage is exclusive. |
| Raw input exceeds the representable complete-byte-count bound | `EXECUTABLE_AUTHORITY_INPUT_READ_FAILED` | Complete digest input cannot be represented; open and stat succeeded. |
| Required rewind or reread syscall fails | `EXECUTABLE_AUTHORITY_INPUT_READ_FAILED` | It is a post-open complete-byte acquisition failure. |
| Required complete digest bytes cannot be acquired despite a usable regular descriptor | `EXECUTABLE_AUTHORITY_INPUT_READ_FAILED` | Acquisition failure precedes digest comparison; later detected object change retains object/race errors. |
| Repository identity cannot be opened/resolved/read | `REPOSITORY_IDENTITY_FAILED` | No valid comparison operand exists; `EVIDENCE_*` is excluded. |
| Acquired repository identity fails `git_oid_sha1` lexical grammar | `REPOSITORY_IDENTITY_FAILED` | Same bounded acquisition operation; no serialized schema is being parsed. |
| Required timestamp cannot be acquired or represented in Topic-1 ranges | `TIMESTAMP_ACQUISITION_FAILED` | Producer clock domain excludes evidence/schema and invariant errors. |

A raw input that is successfully read but hashes differently does not use a
new error: it remains `EXECUTABLE_IDENTITY_MISMATCH`. A valid current repository
OID that disagrees with predecessor authority remains
`EVIDENCE_CORRELATION_MISMATCH`. These post-acquisition predicates make the
boundary complete and nonoverlapping.

# 7. Compatibility audit

| Authority surface | Change? | Determination |
|---|---|---|
| Topic-1 serialized schemas | no | No object or field changes. |
| Field order/types | no | Unchanged. |
| Canonical bytes | no | Unchanged. |
| Digest domains | no | Unchanged. |
| RECORD grammar | no | Unchanged. |
| RECORD limits | no | 1124-byte successful line and 4,603,904-byte complete RECORD remain exact. |
| Privacy rules | no | New stderr literals contain no dynamic detail. |
| Pathname nonauthority | no | Errors neither serialize nor authorize a pathname. |
| Exit grammar | no | Existing success/64/70 rule assigns all new non-invocation errors to 70. |
| Stderr grammar | no | Existing prefix, enum slot, and LF remain exact. |
| Existing error meanings | no | No existing member gains or loses a semantic case; `OBJECT_STAT_FAILED` gains T2 allocation for the same required-stat meaning. |
| Existing tests conceptually | additive only | Any test asserting exactly 55 members must change to 59 after approval; existing per-member expectations remain unchanged, and four new membership/grammar/precedence cases become required under future Topic 6. |
| Cross-topic `May raise` allocation | additive only | Runtime allocation is exactly T2/T3/T4 for executable-authority input open/read, T2 for repository identity, T2/T3/T5 for producer timestamp acquisition, and T2/T3/T4/T5 for existing `OBJECT_STAT_FAILED`. |

The amendment changes only vocabulary membership/count and adds four disjoint
semantic partitions. This is intentionally an amendment, not a claim that the
approved 55-member identity remains unchanged.

# 8. Topic-2 sufficiency audit

The amendment is sufficient for Topic 2 to close all three recorded blockers:

- U2-001 gains exact raw-input open and read outcomes across source,
  build-command, and executable roles.
- U2-002 gains one exact pre-comparison repository-identity outcome while
  preserving correlation mismatch after successful acquisition.
- U2-003 gains one exact producer timestamp-acquisition outcome.

No additional error-taxonomy choice is needed for these domains. Topic 2 must,
after this amendment is independently approved, update its draft mappings,
registry rows, count statements, compatibility audit, and completion verdict.
That later edit is not performed here.

Topics 3 through 5 can apply the same errors where their already-authorized
operations reach the same domains without another applicability amendment:
T3/T4 raw current-helper acquisition and T3/T5 producer timestamp acquisition.
No later topic gains a new operation from this statement. No new schema,
canonical form, digest domain, hidden state, or incompatible reinterpretation
is required. T5 raw executable acquisition, T4 producer-clock acquisition, and
T3–T5 repository acquisition remain unallocated because durable authority does
not require those operations.

Raw executable-authority input stat uses the same `OBJECT_STAT_FAILED` mapping
in T2, T3, and T4. This removes both the former T2 gap and T3/T4 ambiguity
without adding a fifth error or changing the stat member's meaning.

Unresolved implementation-critical decisions within this amendment: **0**.

# 9. Scope audit

This document decides no creator, command argument, artifact placement,
root-chain algorithm, executable-review procedure, preflight, binding, freeze,
run, reservation, publication, crash, replay, validator-row, enumeration,
candidate-access, RECORD, completion/event, or test-fixture behavior. References
to operation stages exist only to make the four error domains mutually
exclusive.

It does not enter Topic 2 beyond error allocation and does not enter Topics 3
through 6. It introduces no hidden state, authority source, pathname authority,
privacy access, or threat-model expansion.

# 10. Independent-review checklist

- Verify all authority sources are current durable repository documents.
- Confirm Topic-1 rules 1 and 2 really exclude clarification-only mappings.
- Confirm every plausible existing error was considered without semantic
  abuse.
- Confirm four is the minimum count preserving raw open/read stage identity.
- Confirm repository acquisition and lexical failure can safely share one
  bounded public domain.
- Confirm each matrix row maps to exactly one member.
- Confirm post-acquisition mismatch retains existing errors.
- Confirm resulting membership is exactly 59 and stderr/exit grammar is
  unchanged.
- Confirm `May raise` is exactly T2/T3/T4 for both executable-input errors, T2
  for repository identity, and T2/T3/T5 for timestamp acquisition.
- Confirm existing `OBJECT_STAT_FAILED` is allocated to T2/T3/T4/T5 and is the
  sole raw-input descriptor-stat syscall result, while successful wrong kind
  remains the new OPEN result.
- Confirm T5 helper correlations do not silently become a raw-executable
  revalidation operation and T4 stat times do not become producer-clock input.
- Confirm no schema, canonical-byte, digest, RECORD, privacy, or pathname rule
  changes.
- Confirm Topic 2 can close U2-001 through U2-003 after approval.
- Confirm there is no lifecycle, implementation, later-topic, reserve,
  candidate, reference, or provenance activity.

**TOPIC-1 ERROR CLARIFICATION DESIGN COMPLETE: YES**

**INDEPENDENT REVIEW REQUIRED: YES**

**TOPIC 2 REMAINS UNAPPROVED: YES**

**CORRECTIVE IMPLEMENTATION AUTHORIZED: NO**
