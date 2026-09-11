# Status and authority

**PROPOSED PREDECESSOR-AUTHORITY CLARIFICATION — NOT APPROVED.**

Authored 2026-09-11 at HEAD/main/origin-main
`fd7c8edd6fb961442a5a35a707dd9e0a060db7bb`, subject
`Approve U4-001 candidate size representability clarification`, divergence
0/0, with an empty index. Independent review and durable preservation are
required. This document does not approve itself or authorize Topic-4 resumption.

The durable checkpoint was read first. The twelve checkpoint-listed approved
design/review identities were verified by SHA-256, line count, and equality to
committed HEAD before reliance. The reconciliation pair also equaled HEAD.
Retained prospective wording in approved predecessors is qualified by their
approving reviews and the checkpoint's durable-preservation record.

The user supplied the independently confirmed U4-002 condition. The earlier
transcript-only blocker review supplies no normative rule here. The outcome
below is newly proposed authority derived from committed contracts. No lost
temporary artifact, unfinished Topic-4 design, helper implementation, or
candidate content supplies a decision.

References and section names below resolve to these exact baseline documents:

| Ref | Committed document | Lines |
|---|---|---:|
| CP | [OS1_TARGETED_RESPEC_RESUME_CHECKPOINT.md](OS1_TARGETED_RESPEC_RESUME_CHECKPOINT.md) | status only |
| T1 | [OS1_CANONICAL_EVIDENCE_SCHEMAS_AND_ERRORS_V1.md](OS1_CANONICAL_EVIDENCE_SCHEMAS_AND_ERRORS_V1.md) | 1034 |
| T1R | [OS1_CANONICAL_EVIDENCE_SCHEMAS_AND_ERRORS_V1_REVIEW.md](OS1_CANONICAL_EVIDENCE_SCHEMAS_AND_ERRORS_V1_REVIEW.md) | 204 |
| A1 | [OS1_TOPIC1_ERROR_TAXONOMY_CLARIFICATION_FOR_TOPIC2_V1.md](OS1_TOPIC1_ERROR_TAXONOMY_CLARIFICATION_FOR_TOPIC2_V1.md) | 383 |
| A1R | [OS1_TOPIC1_ERROR_TAXONOMY_CLARIFICATION_FOR_TOPIC2_V1_REVIEW.md](OS1_TOPIC1_ERROR_TAXONOMY_CLARIFICATION_FOR_TOPIC2_V1_REVIEW.md) | 263 |
| S1 | [OS1_TOPIC1_CANDIDATE_SIZE_REPRESENTABILITY_CLARIFICATION_U4_001_V1.md](OS1_TOPIC1_CANDIDATE_SIZE_REPRESENTABILITY_CLARIFICATION_U4_001_V1.md) | 463 |
| S1R | [OS1_TOPIC1_CANDIDATE_SIZE_REPRESENTABILITY_CLARIFICATION_U4_001_V1_REVIEW.md](OS1_TOPIC1_CANDIDATE_SIZE_REPRESENTABILITY_CLARIFICATION_U4_001_V1_REVIEW.md) | 273 |
| T2 | [OS1_CUSTODY_AUTHORITY_LIFECYCLE_AND_VALIDATOR_REGISTRY_V1.md](OS1_CUSTODY_AUTHORITY_LIFECYCLE_AND_VALIDATOR_REGISTRY_V1.md) | 1213 |
| T2R | [OS1_CUSTODY_AUTHORITY_LIFECYCLE_AND_VALIDATOR_REGISTRY_V1_REVIEW.md](OS1_CUSTODY_AUTHORITY_LIFECYCLE_AND_VALIDATOR_REGISTRY_V1_REVIEW.md) | 227 |
| C3 | [OS1_TOPIC3_AUTHORITY_CLARIFICATION_U3_001_U3_002_V1.md](OS1_TOPIC3_AUTHORITY_CLARIFICATION_U3_001_U3_002_V1.md) | 512 |
| C3R | [OS1_TOPIC3_AUTHORITY_CLARIFICATION_U3_001_U3_002_V1_REVIEW.md](OS1_TOPIC3_AUTHORITY_CLARIFICATION_U3_001_U3_002_V1_REVIEW.md) | 318 |
| T3 | [OS1_ENUMERATION_AND_MAP_PRODUCTION_V1.md](OS1_ENUMERATION_AND_MAP_PRODUCTION_V1.md) | 1154 |
| T3R | [OS1_ENUMERATION_AND_MAP_PRODUCTION_V1_REVIEW.md](OS1_ENUMERATION_AND_MAP_PRODUCTION_V1_REVIEW.md) | 369 |
| R | [OS1_DESIGN_AUTHORITY_RECONCILIATION.md](OS1_DESIGN_AUTHORITY_RECONCILIATION.md) | 416 |
| RR | [OS1_DESIGN_AUTHORITY_RECONCILIATION_REVIEW.md](OS1_DESIGN_AUTHORITY_RECONCILIATION_REVIEW.md) | 527 |

If approved, this supplement follows T1, A1, and S1 only on the Amendment
surface below. All other predecessor meanings and approvals remain intact.
Every new normative requirement here is prospective until independent approval.

# Purpose

Close only the missing public outcome for successful candidate-stat timestamp
components that cannot be faithfully represented by unchanged timestamp_v1.

# Scope and non-goals

This is a Topic-1 outcome-taxonomy supplement for T4 inspect only. It fixes one
external-observation predicate, one public result, and its narrow dependencies.
It does not change timestamp_v1, RECORD schemas, rejection codes/pairings, MAP,
custody, validator predicates, or U4-001's domain. It does not define RA01–RA19,
stat acquisition, snapshots, descriptor counts, RECORD storage/publication,
completion/events, fixtures, fault injection, or conformance registries.

No platform guarantee, timestamp normalization, content inference, access grant,
retry, cleanup, reservation transition, or corrective implementation is created.
No authentic/reference/reserve/blind-validation candidate is accessed.

# Confirmed predecessor gap and existing authority

T1 primitive i64s (line 125), timestamp_v1 (154–161), inspected-line sources
(532–535), and numeric-syscall-component rule (946–948) jointly require faithful
bounded seconds/nanoseconds. All four RECORD timestamps are required.
T1's rejection pairings (569–579) cannot reject an otherwise eligible non-alias
regular file. T1's mandatory partitions (712–788) bind later error selection.

A1 sections 5.1 and 5.3 reserve TIMESTAMP_ACQUISITION_FAILED for producer-clock
acquisition/conversion/range failure in T2/T3/T5. A1R section 7 confirms that
candidate stat time is not absorbed. T2 section 5.5 expressly separates these
domains. S1 Exact exclusions explicitly excludes timestamp representability;
S1R Narrowness review confirms that no timestamp outcome was introduced.

T3 sections 7 and 9 include regular entries and retain candidate stat times
only ephemerally for continuity; MAP serializes none of those timestamps.
T3's section-13 clock range rule concerns enumerated_at only. T3R's independent
trace confirms that distinction. C3 section 7 and C3R Authority-effect review
amend enumerate operands and MAP/binding digest comparison only.

T2 V02-042 checks completed internal construction, not live candidate timestamp
acquisition. T2R's producer-order review preserves that boundary. R/RR G-023
leave the exact inspect matrix to Topic 4; T1's cross-topic and amendment rules
prohibit using that delegation to extend fixed error or schema semantics.

Thus successful stat, stable identity, and unchanged components establish no
predecessor guarantee that the components fit timestamp_v1. No committed rule
normalizes this successful candidate observation. For size 0 and stable birth
components (0, 1000000000), with every other prerequisite satisfied, the
unchanged success schema, rejection set, and public errors supply no outcome.
This is a protocol hypothetical, not a claim that macOS or any filesystem
returns these values. The same gap includes both seconds bounds.

# Outcome analysis

These alternatives were evaluated before choosing the new rule:

| Alternative | Already authorized for this condition? | Compatibility and amendment cost |
|---|---|---|
| A. Existing rejection RECORD | No. The object is neither a nonregular kind nor a Finder alias. | Reusing a code would alter its closed kind/alias pairing and misstate the observation. |
| B. Existing fatal error | No; domains audited below. | Would broaden an existing operation, target, stage, or invariant meaning. |
| C. One new narrow fatal public error | Requires this explicit amendment. | Preserves component fidelity, all serialized schemas, four rejection pairings, and complete-RECORD coverage. Adds one disjoint public domain and local dependency. |
| D. One new narrow rejection code | Requires broader amendment. | Adds a fifth rejection value and a new regular-file rejection predicate, changing schema/validation acceptance and nonfatal policy. Disallowed by this task and unnecessary for closure. |
| E. Normalization/canonicalization | No approved candidate rule. | Carrying, borrowing, clamping, truncating, rounding, or reinterpreting changes observed components. Would need a new value-transformation policy contrary to the unchanged fidelity requirement and task scope. |
| F. Guarantee/assertion that successful observations always fit | No committed proof. | An implementation assertion cannot create an external guarantee. Declaring the probe impossible would add an unsupported platform premise and leave the conditional protocol case unhandled. |

C has the smallest compatible amendment surface. The reason is preservation of
the existing schema and outcome domains, not S1's prior choice of fatality.
The candidate's intrinsic property does not by itself imply a nonfatal policy;
such a policy would require the broader changes in D.

## Existing-error and name audit

The complete original T1 table, all four A1 additions, and S1's one addition
were independently enumerated: 60 distinct current members. The proposed
literal CANDIDATE_TIMESTAMP_UNREPRESENTABLE is absent from that union.
Its candidate target, timestamp field family, and representability condition
are explicit; it implies neither syscall failure nor clock acquisition.

| Existing member/family | Exact exclusion for the isolated condition |
|---|---|
| OBJECT_STAT_FAILED | Required stat syscall failed; here it succeeded. |
| TIMESTAMP_ACQUISITION_FAILED | A1 producer-clock domain and T2/T3/T5 allocation; not candidate stat. |
| CANDIDATE_SIZE_UNREPRESENTABLE | S1's exact out-of-range size predicate; size passes here. Timestamps remain excluded. |
| INTERNAL_INVARIANT_FAILED | T1 rule 1 requires an actual program-invariant violation; an external out-of-domain component is not one. Its broad table shorthand supplies no fallback. |
| OBJECT_IDENTITY_MISMATCH | Failed initial identity comparison; none occurs. |
| OBJECT_CHANGED_OR_REPLACED | Later device/inode/kind discontinuity; none occurs. |
| RACE_OR_MUTATION_DETECTED | Protected observation changes under retained identity; none occurs. |
| CANDIDATE_OPEN_FAILED / CANDIDATE_READ_FAILED / CANDIDATE_SEEK_FAILED | Respective operation failure; none occurs. |
| FINDERINFO_PROBE_FAILED / FINDERINFO_MALFORMED / RESOURCE_FORK_PROBE_FAILED | Respective probe failure or malformed FinderInfo length; none occurs. |
| HASH_LENGTH_MISMATCH / HASH_PASSES_DIFFER | Hashed-length or two-pass digest disagreement; none occurs. |
| Every EVIDENCE_* | Supplied serialized evidence acquisition/validation only; live stat is not that input. |
| EXECUTABLE_AUTHORITY_INPUT_OPEN_FAILED / EXECUTABLE_AUTHORITY_INPUT_READ_FAILED / EXECUTABLE_IDENTITY_MISMATCH / REPOSITORY_IDENTITY_FAILED | Raw executable-authority or repository target; not candidate timestamps. |
| ROOT_* / OUTPUT_PARENT_UNAUTHORIZED | Root acquisition/structure/identity/separation or output-parent authority; no such failure. |
| RESERVATION_UNAVAILABLE / RESERVATION_ALREADY_CONSUMED | Reservation availability/use; no such condition. |
| INVALID_INVOCATION / PATH_GRAMMAR_INVALID | Caller syntax/path defect; no such defect. |
| ENUMERATION_FAILED / ENTRY_LIMIT_EXCEEDED | T3 inventory operation/count; not the T4 observation. |
| OUTPUT_ALREADY_EXISTS / every TEMP_* / every FINAL_* | Specific output/publication stage; no such failure. Do not manufacture malformed output to reach one. |
| Every COMPLETION_* / OPERATIONAL_EVENT_FAILED | Completion/event operation; not this observation. |
| RECORD_INCOMPLETE | Count/order/coverage failure after individually valid lines; not this component's representability. |

SYMLINK_NOFOLLOW requires SYMLINK; DIRECTORY_NOT_CANDIDATE requires DIRECTORY;
UNSUPPORTED_FILE_KIND requires FIFO, SOCKET, CHARACTER_DEVICE, BLOCK_DEVICE,
or OTHER; FINDER_ALIAS_FILE requires REGULAR_FILE with Type 616c6973 (alis).
No pairing applies to this stable non-alias REGULAR_FILE. No fifth code is added.

# Chosen protocol outcome

**PROPOSED NORMATIVE RULE:** Add exactly one fatal public member:

| Literal | Semantic domain | Final terminal evidence | May raise |
|---|---|---|---|
| CANDIDATE_TIMESTAMP_UNREPRESENTABLE | At the local decision below, successful stable stat of an identity-valid, non-alias REGULAR_FILE candidate with representable size supplies at least one required RECORD timestamp component outside timestamp_v1. | none | T4 inspect only |

Inspect must select this member if and only if its Exact trigger holds. It
must stop without a rejection line or a successful inspection result. This
supplements the fatal taxonomy; it does not reclassify successful stat as I/O
failure. No other command acquires this runtime allocation.

# Exact timestamp_v1 representability predicate

Let T be the four required candidate timestamp sources for birth_time,
modification_time, metadata_change_time, and access_time. The first three use
T1's stable stat sources; access_time uses its pre-data-read stable stat source.
For each t in T, let S_t and N_t be the exact mathematical integer seconds and
nanoseconds returned by the successful stat observation selected as that source.

Define:

```text
representable(S, N) =
    (-9223372036854775808 <= S <= 9223372036854775807)
    AND (0 <= N <= 999999999)
```

Evaluate against exact observed components before bounded serialization or
lossy conversion. No host-language numeric bound replaces these protocol bounds.
Do not carry/borrow seconds, clamp, truncate, round, wrap, rescale, omit,
substitute, reinterpret signedness or epoch, or otherwise normalize components.
Do not first convert an out-of-range value and test the converted result.

Negative in-range seconds are valid. Seconds remain canonical i64s strings
with no plus, leading zero, or -0; nanoseconds remain canonical JSON integers.
The syscall epoch is unchanged. This clarification does not impose the
producer-clock epoch or a chronology requirement on candidate stat values.

# Exact trigger

The new member is mandatory if and only if all these conditions hold:

1. Every locally preceding authority/acquisition prerequisite has succeeded;
   no independently triggered fatal result has already terminated the command.
2. Applicable initial binding, identity, and protected-state continuity checks
   through this decision have succeeded for the successful source observations.
3. The stable observed kind is REGULAR_FILE and the approved FinderInfo decision
   establishes non-alias status: absent FinderInfo, or a valid first-eight-byte
   result whose Type is not 616c6973. Failed/malformed probing is not non-alias.
4. S1's candidate-size check has passed using the exact stable st_size in
   0..9007199254740991. Its prerequisites and failure result remain unchanged.
5. For at least one t in T:

```text
S_t < -9223372036854775808
OR S_t > 9223372036854775807
OR N_t < 0
OR N_t > 999999999
```

If every t passes, this member cannot be raised; the owning algorithm continues.
Passing means timestamp representability only, not overall inspection success.
Several failing components or timestamps still produce this one result; no
public branch, field name, or first-invalid-component order is introduced.

Stability means successful required observations through this decision, not an
atomic snapshot or proof against future mutation. Later operations need not be
attempted to prove they would succeed. Their hypothetical failures cannot
compete with a result already reached here.

The new domain excludes failed acquisition/stat, nonregular/alias candidates,
identity discontinuity, changed protected observations, producer clocks,
supplied serialized evidence, root metadata, raw executable-authority values,
output validation, internal fabrication, and conversion bugs affecting an
actually representable observation. These retain their exact existing domains;
this member is not a fallback for any other stat field or implementation limit.

# Boundary cases

These are mathematical authority probes with all prerequisites satisfied, not
platform claims, executable tests, or Topic-6 fixtures. Other timestamps pass.

| S | N | Result of this representation check |
|---:|---:|---|
| -9223372036854775808 | 0 | PASS |
| 9223372036854775807 | 999999999 | PASS |
| 0 | 0 | PASS |
| -1 | 999999999 | PASS |
| -9223372036854775809 | 0 | CANDIDATE_TIMESTAMP_UNREPRESENTABLE |
| 9223372036854775808 | 0 | CANDIDATE_TIMESTAMP_UNREPRESENTABLE |
| 0 | -1 | CANDIDATE_TIMESTAMP_UNREPRESENTABLE |
| 0 | 1000000000 | CANDIDATE_TIMESTAMP_UNREPRESENTABLE |

The failing examples remain unchanged. In particular (0, 1000000000) must not
be converted to (1, 0), nor (0, -1) to (-1, 999999999). The same table applies
to each of the four required timestamp fields.

# Local precedence

S1 already requires its size check after successful stat, applicable binding/
continuity, and non-alias determination, and before this candidate's resource
probe, first data read, hash passes, and successful-line construction. T1
requires the stat timestamp sources, including pre-data-read access time.
These contracts provide a compatible local interval without a new acquisition
procedure or contradictory predecessor dependency.

**NEW NARROW DEPENDENCY:** Place this timestamp decision after S1's check
passes and after the source observations and their required through-decision
continuity checks succeed, but before this candidate's resource-fork probe,
first data-fork read, hash passes, and successful-line construction. This added
relative placement is proposed authority, not a claim that predecessors already
ordered the missing check. It preserves every S1 dependency and outcome.

Topic 4 must supply the authorized acquisition and concrete failure points
within this boundary. No new syscall, count, snapshot recipe, repeated-stat
schedule, candidate traversal order, or storage stage is specified here.
No additional predecessor blocker prevents this local integration.

| Condition when reached at its authorized stage | Controlling outcome |
|---|---|
| Earlier authority/acquisition or other independent failure | Existing exact member; never overwritten by timestamp classification. |
| Required candidate stat syscall fails | OBJECT_STAT_FAILED; no successful components from that call. |
| Initial identity/binding comparison fails | OBJECT_IDENTITY_MISMATCH. |
| Later device/inode/kind discontinuity through the decision | OBJECT_CHANGED_OR_REPLACED. |
| Same identity but required protected observation changes through the decision | RACE_OR_MUTATION_DETECTED. |
| Stable nonregular kind with rejection prerequisites satisfied | Its unchanged nonregular rejection pairing; no timestamp check for successful regular-file RECORD. |
| FinderInfo fails or is malformed | FINDERINFO_PROBE_FAILED or FINDERINFO_MALFORMED. |
| Stable Finder alias with rejection prerequisites satisfied | FINDER_ALIAS_FILE, regardless of timestamp range. |
| S1 prerequisites pass and size is outside its range | CANDIDATE_SIZE_UNREPRESENTABLE; timestamp decision is not reached, even if its values would also be out of range. |
| Size passes and this trigger holds | CANDIDATE_TIMESTAMP_UNREPRESENTABLE; no later candidate probe/read/hash or successful-line construction. |
| Every timestamp passes | Continue; no new success guarantee. |
| Later resource/read/seek/length/digest operation fails | Its existing exact member; no retrospective timestamp error. |
| Later stat fails or identity/protected state changes after this check passed | Existing stat/identity/replacement/mutation outcome at that stage; do not replace the bound source and relabel the result as timestamp unrepresentability. |

This table does not waive any later continuity rule or change how access time
is protected. If multiple preceding prerequisites fail, the owning ordered
algorithm selects the reached failure under existing partitions. This document
orders only its new decision relative to those prerequisites and later work.

# Public error contract

Under the Exact trigger, the literal is exactly
CANDIDATE_TIMESTAMP_UNREPRESENTABLE, allocated only to T4 inspect. It writes
zero stdout bytes and exactly the following ASCII line followed by one LF byte
(0x0a), with no other stderr bytes:

```text
OS1_METADATA_HELPER_ERROR:CANDIDATE_TIMESTAMP_UNREPRESENTABLE
```

Exit status is exactly 70. This follows T1 Public stderr grammar: every V2
error other than INVALID_INVOCATION and PATH_GRAMMAR_INVALID exits 70. The
new member belongs to neither exception. No new exit rule is selected by
analogy with S1. Success remains 0; the two invocation/path exceptions remain
64; signal/host termination remains outside the public command contract.

No dynamic timestamp, sign, component, field name, value, pathname, basename,
ID, digest, OS diagnostic, candidate byte, or other metadata may be appended.
All four range branches and all four timestamp fields share identical public
bytes. No rejection-code alias, numeric ordinal, or alternate spelling exists.

# RECORD consequences

The failing candidate receives neither INSPECTED_REGULAR_FILE nor
EXPECTED_REJECTION. No ERROR line or incident artifact is introduced.
The invocation must not publish its RECORD or a valid terminal completion.
Earlier final evidence remains untouched.

Every successfully completed RECORD still covers exactly the MAP entry_count,
with MAP index i corresponding to line i and the same record ID. Do not omit
the failing candidate, renumber entries, shrink MAP, substitute a line, or
publish only the other candidates. A fatal invocation yields no complete
RECORD, not a successful RECORD with a coverage exception.

Any partial unpublished temporary bytes already produced remain nonauthoritative
residue under existing rules. They are not a partial successful RECORD and do
not authorize repair, cleanup, or completion. This rule is conditional on such
bytes existing; it does not require a temp, forbid earlier storage for other
candidates, or choose a Topic-4 buffering/write schedule. Semantic line
production and later storage mechanics remain separate obligations.

Zero MAP entries exercise no candidate predicate. The existing zero-entry,
zero-byte RECORD behavior is unchanged, subject to all other command checks.
All fields, canonical bytes, digests, four rejection pairings, 4096-entry cap,
1124/626-byte line maxima, and 4,603,904-byte complete-RECORD maximum remain.

# Error vocabulary effect

Independent enumeration of the authoritative additions gives:

| Set | Distinct members | Overlap with earlier sets |
|---|---:|---:|
| T1 complete original literal table | 55 | n/a |
| A1 section 5 four named additions | 4 | 0 |
| S1 CANDIDATE_SIZE_UNREPRESENTABLE | 1 | 0 |
| This proposed CANDIDATE_TIMESTAMP_UNREPRESENTABLE | 1 | 0 |

Current approved total: 55 + 4 + 1 = 60. Proposed effective total after approval:
55 + 4 + 1 + 1 = 61. Until approval, the effective approved total remains 60.
The name audit includes all current public literals, not only nearby names.

Retain OS1_METADATA_HELPER_ERRORS_V2. A1 and S1 establish additive supplements
under that same taxonomy identity; no predecessor requires a version change.
Existing members retain their exact spelling, semantic domain, and allocation.
T2/T3/T5 acquire no allocation for the new member. Topic 6 is not a runtime
operation allocation.

## Complete current-count qualifications

Upon approval, every current-total reference at the following anchors denotes
the combined 61-member vocabulary. Earlier measured totals and set-addition
arithmetic remain historical facts; original-member and amendment-subset counts
are not rewritten. Historical review verdicts are not revoked.

| Document | Baseline anchors qualified |
|---|---|
| T1 | Taxonomy rules 700; table aftermath 849; Public stderr grammar 865; Forward-compatibility audit 984; I6 closure 1010. |
| T1R | Regression gate 154. |
| A1 | Section 5 total 173; section 7 count assertions 301; section 10 checklist 362. |
| A1R | Section 9 arithmetic 198 and vocabulary count 202. |
| T2 | Status 16; G-002 1080; G-028 1090; sufficiency 1099; checklist 1198. |
| T2R | Section 7 184; G-028 212. |
| C3 | Compatibility 330; error sufficiency 337; self-audit 431. |
| C3R | Error compatibility 187. |
| T3 | Authority 57; errors 771; G-028 1054; compatibility 1126. |
| T3R | Compatibility table 248. |
| S1 | Error vocabulary total 309; current-count qualification 323 and its table 327–339; Amendment surface total interpretations 395 and 403. Its one-member addition and sixtieth-member description stay historical/subset facts. |
| S1R | Error/exit review arithmetic 158; review reports the approved total at that review. |
| CP | Topic-1 summary and U4-001 preservation/count paragraphs; future status reporting must reflect this separately approved addition and its preservation. CP itself is not normative authority. |

This table supplements S1's current-count qualification to 61, including its
references to earlier 55/59 assertions. It does not expand earlier amendments'
local applicability or turn C3's local sufficiency into a Topic-4 guarantee.

# Validator-registry effect

Independent enumeration confirms 42 unique contiguous IDs V02-001..V02-042.
No row, predicate, dependency, numeric order, or command applicability changes.
No V02-043 is introduced. The new predicate concerns live external candidate
observations, not supplied serialized evidence or T2 constructed authority.

T2 V02-041 retains its producer-clock domain; V02-042 retains internal output
integrity after construction. Neither acquires candidate timestamp values.
C3's corrected V02-030 remains unchanged. T3's analogous MAP construction check
cannot supply a candidate RECORD outcome.

Public-error recognition must consume the newly approved member and its T4-only
allocation under the existing envelope. This is vocabulary recognition, not a
new serialized-evidence validator predicate or authority created by stderr.
An out-of-range timestamp supplied in serialized RECORD remains governed by
T1 evidence-schema/canonical precedence. The new member in rejection_code is
invalid. Publication-stage checks retain TEMP_CONTENT_MISMATCH and
FINAL_REOPEN_FAILED at their existing stages; RECORD_INCOMPLETE retains its
individually-valid-line count/order/coverage domain.

A normalized or fabricated in-range timestamp may be schema-shaped. No new
claim is made that validators can infer falsified source components from bytes
alone. Producer fidelity and serialized-schema recognition remain distinct.
Future conformance work must cover this addition, but no test, fixture, fault
injection, registry row, or conformance count is specified here.

# Amendment surface

Only the changed effective meanings below and the current-count qualifications
have precedence over predecessors. Every other rule remains controlling.

| Authority anchor | Effective supplement or explicit preservation |
|---|---|
| T1 i64s and timestamp_v1 | Unchanged types, four bounds, order, epoch meaning, and numeric spelling. No normalization or larger domain. |
| T1 inspected-line timestamp sources and determinism | Unchanged four required sources and component fidelity. Add a producer outcome when faithful representation is impossible; no field omission/substitution. |
| T1 mandatory partition rule 6 and error table | Add the one disjoint successful candidate timestamp representation domain, T4-only allocation, and local dependencies. Failed stat retains OBJECT_STAT_FAILED unchanged. |
| T1 mandatory partition rules 1–5 and 7–10 | Unchanged invariant, supplied-input, identity, publication, and completeness domains. No fallback or broadened failure meaning. |
| T1 taxonomy fatality, Expected rejection 581–583; R G-023/category-B RECORD statement; RR G-023/section 8 item 4 | Supplement the prospective fatal set with this exact representation condition. Historical categories remain historical; no rejection pairing or generic I/O meaning changes. |
| T1 Public stderr grammar | Add the new literal to permissible enum membership; unchanged exact envelope, privacy, stdout and exit rules determine its public contract. |
| A1 sections 5, 5.1, 5.3 and A1R producer-timestamp boundary | TIMESTAMP_ACQUISITION_FAILED remains T2/T3/T5 clock-only. The statement that candidate failures retain object/race mappings now has this explicit additional successful-observation domain alongside S1; no existing mapping is broadened. |
| S1 Exact trigger, Exact exclusions, Precedence, integration; S1R domain/narrowness review | Entire size predicate and outcomes unchanged; timestamps remain excluded from that member. This document separately places its decision after S1 passes. Both size-range branches retain S1 priority. |
| T2 5.5, 15 V02-041/042 and T2R producer-order review; C3 section 7 | No changed predicate, allocation, acquisition, custody semantics, or 42-row registry. New public vocabulary membership only. |
| T3 sections 7, 9, 13 and T3R trace | No changed enumeration eligibility, local continuity, MAP fields, clock behavior, or publication. No timestamp screening or new error allocation to enumerate. |
| T1 cross-topic RECORD/error rows and Unresolved decisions Topic 4 | This exact outcome and narrow dependency are now fixed upstream upon approval. Full acquisition, RA01–RA19, remaining failure sequencing, and RECORD production stay Topic 4. |
| T1 Forward-compatibility/amendment rules; T1R Forward compatibility/final verdict; broader T2/T3 sufficiency statements | Qualify only the claim that no additional predecessor outcome is needed for this candidate domain. The explicit independently reviewed supplement is required; no schema insufficiency, revoked approval, or unrelated gap closure is claimed. |
| S1 Amendment surface and count table; all count anchors above | Preserve S1's amendment as one size-only addition; qualify combined current membership by this separate addition. |
| CP summary/resume status | Future preservation must record this separate approval and identity before authorized Topic-4 resumption. No checkpoint edit or resumption occurs here. |

No direct predecessor edit is needed for this standalone supplement. It claims
closure only for U4-002 after approval; it does not claim unfinished Topic 4
has no other blocker.

# Topic boundaries and compatibility with U4-001

- Topic 1 gains only this fatal domain, local dependencies, and count qualification.
- Topic 2 custody, lifecycle, validators, and producer clocks remain unchanged.
- Topic 3 enumeration/MAP remains unchanged; no candidate timestamp filtering.
- Topic 4 may later consume the approved, durably preserved clarification in a
  separately authorized task. It retains candidate access, RA01–RA19, exact
  source acquisition, remaining failure sequencing, and RECORD production.
- Topic 5 completion/events and their mechanics remain unchanged. Existing
  fatal-command rules prohibit valid completion; no event trigger or operation
  mapping, suppression, reservation transition, or publication design is added.
- Topic 6 fixtures, fault injection, and conformance design remain deferred.

S1 is not broadened or reinterpreted. Out-of-range size still stops at S1;
representable size permits reaching this separate timestamp check. Passing
both checks grants neither candidate access nor overall success. Existing
alias/nonregular rejection and continuity prerequisites remain controlling.

# Determinism and privacy

Given the same prerequisite results and exact timestamp components, this
predicate yields the same result regardless of language, locale, timezone,
platform formatting, overflow behavior, or which invalid field is examined
first. Its mathematical comparisons do not authorize normalization or retry.
The only added public information is one fixed failure class; no component,
field identity, count of failing timestamps, or dynamic candidate detail leaks.
No new serialized metadata, hidden state, pathname authority, or digest domain
is introduced.

# Self-review

This is author self-review, not independent approval. The following conclusions
apply to the proposed rules and were checked against unchanged predecessors.

| Adversarial check | Finding |
|---|---|
| Seconds below minimum / above maximum | Both select the sole new member when prerequisites pass. |
| Nanoseconds below 0 / above 999999999 | Both select the same member without normalization. |
| Exact seconds endpoints and nanosecond endpoints | Included; the PASS table covers all four endpoints. |
| Negative representable seconds | Accepted; (-1, 999999999) remains exact. |
| Any of four timestamps invalid; several invalid | Same result; no birth-only gap or competing literal. |
| Successful stat versus failed stat | Disjoint; failed syscall remains OBJECT_STAT_FAILED. |
| Producer-clock range defect | TIMESTAMP_ACQUISITION_FAILED allocation unchanged; not the new member. |
| Size fails as well | S1 wins before this decision; no size-domain broadening. |
| Initial identity or through-decision continuity fails | Existing identity/replacement/mutation result; no new fallback. |
| Stable nonregular/alias candidate | Existing rejection pairing; no timestamp rejection or fatality override. |
| Later continuity/mutation failure after range passes | Existing stage result; source is not replaced to manufacture this error. |
| Program cannot encode an actually in-range value | Implementation defect, outside the new external-observation domain. |
| Supplied malformed RECORD / malformed producer output | Existing target/stage partition; never manufactured to resolve this observation. |
| Public grammar and exit | One fixed ASCII stderr line plus LF, zero stdout, exit 70 derived from T1's all-other-V2 rule. |
| One failing entry in nonempty MAP | No complete/published RECORD; no omission, renumbering, substitution, or partial success. |
| Zero entries | Predicate never executes; zero-byte RECORD rule unchanged. |
| Vocabulary arithmetic/name audit | 55 distinct original + 4 distinct A1 + 1 S1 = 60; one absent, semantically disjoint proposed name yields 61 after approval. |
| Validator registry | 42 contiguous existing IDs; live predicate requires no row or predicate change. |
| Local placement | Compatible interval after S1 and before candidate resource/read/hash work; no complete inspect algorithm invented. |
| U4-001 compatibility | Exact trigger, size priority, exclusions, schema and output obligations preserved. |
| Topic 5/6 boundaries | No terminal/event mechanics, fixtures, test IDs, or implementation authored. |
| Privacy | Fixed class only; no new serialized fields or dynamic diagnostics. |
| Implementation-critical ambiguity within U4-002 | None found: target, all bounds, prerequisites, local precedence, literal, exit, bytes, and RECORD effects are fixed. Remaining mechanics retain their assigned owner. |

No new predecessor blocker was found while resolving this narrow condition.
This is not an assertion that all Topic-4 design decisions are complete.

# Approval status

This is a proposed predecessor-authority clarification. Independent review is
required. It does not approve itself, change current approved membership before
approval, or authorize Topic 4 to resume based on this document alone.
Durable preservation under CP and separate task authorization remain required.
Corrective implementation remains unauthorized under the complete-chain gate.
No predecessor, helper, candidate, or protected unrelated work is modified.
No staging, commit, push, implementation test, or Topic-5/6 work is performed.

U4-002 CLARIFICATION AUTHORED: YES

U4-002 CLARIFICATION APPROVED: NO

UNRESOLVED CLARIFICATION BLOCKERS: 0

TOPIC 4 MAY RESUME: NO

CORRECTIVE IMPLEMENTATION AUTHORIZED: NO

NEXT REQUIRED ACTION:
INDEPENDENT REVIEW OF U4-002 PREDECESSOR-AUTHORITY CLARIFICATION
