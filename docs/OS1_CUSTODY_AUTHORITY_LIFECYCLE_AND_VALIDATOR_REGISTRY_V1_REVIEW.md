# Fresh Independent Approval Review of Final OS1 Topic 2

## 1. Identity and authority gate

The review gate passed before substantive review.

- Topic-2 design: 1213 lines; SHA-256
  `406c8703da2454423308fbd347d2c0f2126228ba9c90a9ac00282e597a06869c`.
- `HEAD`, `main`, and `origin/main`:
  `9db577163bd8035b82c7da5da99ac22e2548ed86`.
- divergence: `0/0`.
- staged paths: none.

The immediately preceding 229-line approval review and the earlier 394- and
370-line reviews were read only as historical review evidence. Durable
repository authority was used for the regression review. No lost temporary
design was treated as authority.

## 2. Findings

| Severity | Count |
|---|---:|
| BLOCKER | 0 |
| IMPORTANT | 0 |
| MINOR | 0 |

No finding prevents approval.

## 3. Final preflight root-order correction

**PREFLIGHT ROOT-ORDER FINDING CLOSED: YES.**

The canonical signature is exactly:

```text
phoenix-os1-custody preflight-roots \
  --implementation-review-root ROOT --candidate-root ROOT \
  --private-output-root ROOT --custody-record-root ROOT \
  --executable-authorization FILE --custody-source FILE \
  --custody-executable FILE --preflight-output FILE
```

The root options occur in one exact order:

1. `IMPLEMENTATION_REVIEW_ROOT`;
2. `CANDIDATE_ROOT`;
3. `PRIVATE_OUTPUT_ROOT`;
4. `CUSTODY_RECORD_ROOT`.

Section 4.2 applies the generic command-line-order rule. Section 5.6 applies
the same command-argument-order rule to producers. V02-003 uses command-line
order. Its dependency row uses canonical signature order. Section 8.1 names
the four roles in the same sequence and expressly states that it is the
canonical command-argument order. The producer walkthrough abbreviates the
same sequence as review, candidate, private, custody. No other preflight order
appears.

If multiple supplied roots independently fail, section 8.1 requires the first
failure in that sequence to produce the sole observed `ROOT_CHAIN_INVALID` and
forbids attempting later roots. This agrees with Topic-1 root specialization
and adds no priority system. Option A therefore preserves the general rule;
there is no preflight exception or hidden override.

## 4. Producer order and validator registry

The normative registry contains exactly 42 unique contiguous rows,
`V02-001` through `V02-042`. Its numeric order is executable:

1. invocation and paths;
2. root acquisition and separation;
3. serialized-evidence open, parse, canonicalization, digest and correlation;
4. applicable raw executable and fixed repository acquisition;
5. remaining input authority and lifecycle correlations;
6. final root/object identity and protected-state rechecks;
7. output-parent and mutually exclusive availability checks;
8. V02-041 producer timestamp acquisition;
9. complete canonical output construction and ID/self-digest derivation;
10. V02-042 constructed-output validation;
11. publication and post-publication verification.

No row consumes a future observation. V02-015 and V02-031 expressly exclude a
producer's not-yet-constructed output. The row-by-row dependency table matches
the registry. Conditional evidence, repository, MAP, freeze, and reservation
rows state their applicability. Observational validators execute applicable
input and closing continuity rows but skip producer-only V02-037 through
V02-042; they cannot acquire time, construct output, or publish.

## 5. Command surfaces and lifecycle regression

The command inventory remains exactly eight producers:

1. `freeze-executable-review`;
2. `verify-executables`;
3. `preflight-roots`;
4. `create-path-map`;
5. `bind-map`;
6. `freeze-map-review`;
7. `authorize-run`;
8. `reserve-run`.

It also remains exactly eight observational validators:

1. `validate-executable-review`;
2. `validate-review-identities`;
3. `validate-executable-authorization`;
4. `validate-preflight`;
5. `validate-path-map`;
6. `validate-binding`;
7. `validate-freeze-review`;
8. `validate-run-reservation`.

All names, required arguments, argument ordering, input roots, output roots,
and descriptor-relative opens are explicit. There is no optional authority,
environment/config fallback, implicit CWD, daemon, database, session state, or
filesystem discovery. No seventeenth operation is implied.

`bind-map` and `validate-binding` still supply exactly the necessary
implementation-review, custody-record, and private-output roots.
Authorization opens under the review root; preflight and path map under the
custody root; MAP and binding under the private root. `bind-map` publishes
through the retained private root. Current candidate-root access is neither
needed nor invented.

Executable review remains ordered from raw inputs through A/B comparison,
fixed repository identity, final rechecks, output checks, timestamp,
construction, review publication, and identities publication as commit point.
Authorization, preflight, path map, binding, freeze, run, and reservation
lifecycles retain exact inputs, correlations, timestamp, construction,
publication, final verification, and crash behavior.

The forward state machine remains binding to freeze review to run authorization
to consumption reservation. It is immutable and has no reverse transition or
hidden mutable state. Reservation remains use reservation, not inspect
completion.

## 6. Repository, timestamp, publication, and crash regression

The repository model is unchanged and deterministic. `.git`, `refs`, and
`heads` retain device ID, inode/file ID, kind, and mode. `HEAD` and `main`
retain those fields plus size, mtime, and ctime. Atime, birthtime, and every
other stat field are ignored.

The five fixed objects are acquired descriptor-relative/no-follow and retained
through the final recheck after correlations and immediately before output
checks. The error partition remains unique:

- repository open, wrong kind, bound/read, representation, resolution, or
  lexical failure: `REPOSITORY_IDENTITY_FAILED`;
- initial or later descriptor-stat syscall failure: `OBJECT_STAT_FAILED`;
- later device/inode/kind discontinuity: `OBJECT_CHANGED_OR_REPLACED`;
- later protected-field mutation with stable identity:
  `RACE_OR_MUTATION_DETECTED`;
- stable valid OID disagreement: `EVIDENCE_CORRELATION_MISMATCH`.

The producer timestamp model is unchanged: one successful
`clock_gettime(CLOCK_REALTIME)` observation per command after preconstruction
checks and before construction, using POSIX epoch signed `i64s` seconds and
nanoseconds in `0..999999999`. There is no retry, rounding, normalization, or
alternate clock. Equal/backward values create no ordering authority. Review
and identities share one timestamp. Failure maps solely to
`TIMESTAMP_ACQUISITION_FAILED`.

The no-cleanup publication model is unchanged. Complete canonical output bytes
precede self-digest and deterministic temp-name derivation. Temporary creation
is exclusive; write, fsync, reread, rename, directory fsync, final file/root,
and reopen stages remain ordered and separately mapped. Temporary residue is
nonauthoritative and immutable. A failed invocation cannot resume. Fresh
invocation revalidates and reacquires time; identical temp identity fails
`TEMP_CREATE_FAILED`, and progress is not promised. No cleanup, removal,
alternate-name search, overwrite, or repair exists.

The crash table agrees with this prose for pre-temp, pre-rename, post-rename,
directory-sync, final-verification, review-pair, and reservation states.

## 7. Topic-1, privacy, determinism, and scope audit

- Topic-1 schemas, fields and ordering are unchanged.
- Canonical bytes and digest domains are unchanged.
- RECORD grammar remains unchanged, including the 1124-byte line maximum with
  LF and 4,603,904-byte complete maximum.
- Stderr and exit grammar remain unchanged.
- Pathname nonauthority and root-bound/content-correlation-bound identity are
  preserved.
- The approved public vocabulary remains exactly 59 members, with one
  deterministic mapping for every Topic-2 failure.
- Validators remain observational and non-agentic.
- Candidate paths, names, contents, arbitrary xattrs, and resource-fork content
  are not exposed.
- No randomness, fallback, hidden lookup, ambient Git, locale dependence,
  implicit clock, cleanup, daemon, database, or global state exists.
- The honest/access-controlled threat model is unchanged; no signature, MAC,
  key, network trust, hostile-kernel/admin, or Byzantine-filesystem claim is
  added.
- Topic 3 enumeration/MAP production, Topic 4 candidate/RECORD production,
  Topic 5 terminal publication, and Topic 6 fixtures/assertions remain outside
  Topic 2.

## 8. Gap accounting

| Gap | Disposition | Reason |
|---|---|---|
| G-002 | **CLOSED BY TOPIC 2** | All sixteen command contracts, argument orders, authority inputs, validation order, publication, and failures are exact. |
| G-003 | **CLOSED BY TOPIC 2** | Executable review, repository, timestamp, projection, publication, and revalidation are complete. |
| G-004 | **CLOSED BY TOPIC 2** | Executable authorization inputs, current identities, correlations, placement, and rejection are complete. |
| G-005 | **CLOSED BY TOPIC 2** | Four-root preflight acquisition now has one exact order; separation, provenance, path-map authority, publication, and errors are complete. |
| G-009 | **CLOSED BY TOPIC 2** | Binding command roots and custody-side MAP authority are explicit without defining MAP production. |
| G-010 | **CLOSED BY TOPIC 2** | Freeze pair authority, verdict, creation, placement, and validation are complete. |
| G-011 | **CLOSED BY TOPIC 2** | Run scope, complete chain, issuance, placement, and stale rejection are complete. |
| G-012 | **CLOSED BY TOPIC 2** | Reservation exclusivity, one-use, replay, crash, and validation are complete without claiming completion. |
| G-013 | **CLOSED BY TOPIC 2** | Root acquisition, projection, comparison, rechecks, errors, and pathname nonauthority are complete. |
| G-014 | **PARTIALLY CLOSED — Topics 3–5 retain assigned producer enforcement** | Topic-2 placement and identity authority is closed. |
| G-028 | **PARTIALLY CLOSED — Topic 6 retains deterministic fixtures/assertions** | Topic 2 closes the normative 42-row registry, ordering, and 59-error mapping. |

## 9. Implementation readiness and verdict

Unresolved Topic-2 implementation-critical decisions: **0**.

**Implementation-readiness answer: YES.** A different engineer can implement
every Topic-2-owned behavior from durable authority and this design without
making an implementation-critical design decision.

```text
TOPIC 2 APPROVED: YES
TOPIC 2 MAY BE DURABLY PRESERVED AFTER THIS REVIEW: YES
TOPIC 3 MAY BEGIN AFTER DURABLE PRESERVATION: YES
CORRECTIVE IMPLEMENTATION AUTHORIZED: NO
```
