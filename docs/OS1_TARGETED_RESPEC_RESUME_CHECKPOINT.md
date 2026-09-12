# OS1 Targeted Re-Specification Resume Checkpoint

Checkpoint date: 2026-09-03.

Preservation update: 2026-09-10, after independent Topic-3 approval.
Pre-preservation HEAD, main, and origin/main were
`cb25005545bf5f8c464e2edc3930174efb40547c`
(`Preserve Topic 3 authority clarification`), divergence `0/0`.
This preservation commit, `Define OS1 enumeration and MAP production`, makes
the approved Topic-3 design/review pair durable with this checkpoint.

Preservation update: 2026-09-11, after independent U4-001 clarification approval.
The preceding paragraph records the historical Topic-3 preservation. Before
this U4-001 preservation, HEAD, main, and origin/main were synchronized at
`f1cafd0b73136cf7138dd6dbdbde37507286b48c`, divergence `0/0`, with an empty index.
The commit `Approve U4-001 candidate size representability clarification`
preserves the unchanged approved clarification, its independent-review
artifact, and this minimum checkpoint update. It does not resume Topic 4.

Preservation update: 2026-09-11, after independent U4-002 clarification approval.
Before preservation, HEAD, main, and origin/main were synchronized at
`fd7c8edd6fb961442a5a35a707dd9e0a060db7bb`, divergence `0/0`, with an empty index.
The commit `Approve U4-002 candidate timestamp representability clarification`
preserves the unchanged approved clarification, its independent review, and
this checkpoint update. U4-001 remains approved and durably preserved.
Earlier references below to "this commit" describe their historical preservation
updates unless explicitly identified as U4-002. The new project-management gate
supersedes all earlier permission to resume Topic 4 after preservation alone.
Topic 4 has not resumed.

This is a status and handoff document. It is not new design authority and does
not supersede or alter any approved design authority.

## Phoenix 0.1 release-scope rebaseline — 2026-09-12

The release-path rebaseline and minimum OS1 scope decision are complete.
Phoenix 0.1 no longer requires completion of the entire remaining OS1
Topics 4–6 framework before blind validation. This project-management decision
supersedes the blanket six-topic completion/review prerequisite and earlier
next-action/resume prerequisites below **for the minimum 0.1 route only**.
The full-framework requirements remain applicable to work claiming that full
contract. Approved Topics 1–3 and U4-001/U4-002 remain valid, unchanged, and
neither withdrawn nor rewritten.

For 0.1, remaining OS1 work is limited to:

- eligible, neutral independent-candidate selection;
- candidate/source identity correlation;
- source read-only preservation with before/after integrity evidence;
- safe refusal of unintended or ambiguous objects;
- metadata-intake privacy and blindness protection;
- complete, interpretable Phoenix observation records;
- distinction between complete and incomplete/failed observations;
- durable freeze of Phoenix observations before reference reveal;
- honest preservation of failures and post-freeze reference comparison; and
- a synthetic rehearsal plus focused representative verification.

The following are **not Phoenix 0.1 release prerequisites**:

- complete RA01–RA19 or IP01–IP11 implementation;
- full canonical MAP/RECORD/completion/event production for the minimum route;
- mandatory implementation of every previously specified timestamp, fork,
  xattr, and metadata field;
- four-root custody continuity or generalized root-bound evidence validation;
- alternate-basename, relocation, copy, replay, or historical-identity machinery;
- executable A/B authorization artifact chains or automated binding/run/
  reservation authority chains;
- atomic one-use reservation/lifecycle automation or operational-event
  publication;
- exhaustive temporary-file, crash, or filesystem-race handling;
- complete validator/conformance registry coverage or exhaustive exact-byte/
  error-precedence matrices;
- automatic clarification → independent review → preservation cycles for every
  newly noticed edge case; or
- completion of Topics 4–6 merely to satisfy the former blanket framework.

New edge cases do not automatically create design-authority work. Ordinarily,
use conservative failure behavior and focused implementation-level tests unless
a concrete Phoenix 0.1 release risk requires more. Reduced/minimum-route
artifacts must not claim conformance to fuller OS1 schemas or contracts they
do not implement. This decision qualifies prior technical prerequisites only
as stated above; it does not reopen predecessor technical authority.

The blind-validation protocol's essential requirements remain binding:
permission, eligibility, neutrality, concealment, source integrity,
freeze-before-reveal, and honest comparison. Metadata-intake completion does
not itself freeze the later Phoenix observations.

**Candidate access, Topic 4 implementation, corrective implementation, and
blind execution remain unauthorized and require separate explicit gates.**
This record grants none of those permissions. The immediate purpose is to move
Phoenix toward its first independent blind validation, rather than continue
general OS1 framework completion.

## Historical baseline at checkpoint creation

Commit:
`10cc691e2bb49969436ad8667d30e97d7271ee36`

Subject:
`Define OS1 canonical evidence schemas and errors`

At checkpoint creation, `main` and `origin/main` were synchronized with
divergence `0/0`.

## Why targeted re-specification exists

The original approved OS1 design chain contained implementation-critical
authority that had existed only in `/tmp` and was lost after a reboot/pause.
The durable reconciliation at
`docs/OS1_DESIGN_AUTHORITY_RECONCILIATION.md` and its review at
`docs/OS1_DESIGN_AUTHORITY_RECONCILIATION_REVIEW.md` determined that corrective
implementation must remain unauthorized until six targeted re-specification
topics are completed and independently approved.

The lost temporary documents were not reconstructed byte-for-byte.

## Topic sequence

1. Topic 1: Canonical evidence schemas and error taxonomy
2. Topic 2: Custody authority, lifecycle, and validator registry
3. Topic 3: Enumeration and MAP production
4. Topic 4: Inspect, candidate access, and RECORD production
5. Topic 5: Terminal completion and operational-event publication
6. Topic 6: Deterministic conformance-test registries

## Topic 1 — COMPLETE AND DURABLE

Status: **APPROVED AND COMMITTED**

Design:
`docs/OS1_CANONICAL_EVIDENCE_SCHEMAS_AND_ERRORS_V1.md`

- 1034 lines
- SHA-256:
  `078d9cec16103f63b961dc53b03f68ae563de999e3ca9a3ab6ab04ad98f5f14e`

Final independent review:
`docs/OS1_CANONICAL_EVIDENCE_SCHEMAS_AND_ERRORS_V1_REVIEW.md`

- 204 lines
- SHA-256:
  `fa4d36080b5f75294561ed41546a4ae67d533b0a64aa52153b3e40b346e6baf1`

Approval result:

- `TOPIC 1 APPROVED`
- `BLOCKER 0`
- `IMPORTANT 0`
- `MINOR 0`
- unresolved Topic-1 implementation-critical decisions: `0`

Approved Topic-1 summary:

- 14 top-level serialized schemas
- one RECORD stream grammar
- 61 `OS1_METADATA_HELPER_ERRORS_V2` members: original 55, four from the
  approved Topic-2 amendment, one from U4-001, and one from U4-002
- maximum schema-derived successful RECORD line: 1124 bytes including LF
- maximum complete RECORD: 4,603,904 bytes
- gap accounting: 3 fully closed, 15 partially closed, 2 outside Topic 1, and
  0 unaddressed
- digest graph acyclic
- forward compatibility passed
- all I1–I6 closed

Approved additive Topic-1 error-taxonomy amendment:
`docs/OS1_TOPIC1_ERROR_TAXONOMY_CLARIFICATION_FOR_TOPIC2_V1.md`

- 383 lines
- SHA-256:
  `b0443f34180aa2dd5aa721bbd5f77333fc4d9a1c39bf03aca938fdb5ba144f53`

Final independent amendment review:
`docs/OS1_TOPIC1_ERROR_TAXONOMY_CLARIFICATION_FOR_TOPIC2_V1_REVIEW.md`

- 263 lines
- SHA-256:
  `0530e4106ae73406633a86acfda3ac30a833f1e67a54f76f694e9cf5046c6f1a`

The amendment and approving review are committed durable Topic-1 authority.
They add four errors without changing Topic-1 schemas, canonical bytes, digest
domains, privacy, pathname nonauthority, RECORD grammar, stderr, or exit
grammar.

### Approved U4-001 predecessor clarification — PRESERVED BY THIS COMMIT

Approved clarification:
`docs/OS1_TOPIC1_CANDIDATE_SIZE_REPRESENTABILITY_CLARIFICATION_U4_001_V1.md`

- 463 lines
- SHA-256:
  `d656d2e25719457bdfc80c7128ef24d0c803f9b819cbb55e42ad2975361e75c3`

Approved independent review, faithfully preserved from the session review:
`docs/OS1_TOPIC1_CANDIDATE_SIZE_REPRESENTABILITY_CLARIFICATION_U4_001_V1_REVIEW.md`

- 273 lines
- SHA-256:
  `9ca129bb9bed1992c2e1daf48302948fab7b02e8c43ea6ad8d0c0e3db4bc7e63`

Approval result:

- `U4-001 INDEPENDENT REVIEW: APPROVED`
- `BLOCKERS: 0`
- `IMPORTANT FINDINGS: 0`
- `MINOR FINDINGS: 0`
- `U4-001 AUTHORITY CLOSED: YES`
- `CORRECTIVE IMPLEMENTATION AUTHORIZED: NO`

U4-001 is closed by the approved Topic-1 supplement, not a Topic-4 local
mapping choice. At U4-001 approval the effective vocabulary became 60, retaining
`OS1_METADATA_HELPER_ERRORS_V2`. The new T4-inspect-only member is
`CANDIDATE_SIZE_UNREPRESENTABLE`: the clarification's exact successful,
stable, non-alias regular-candidate size condition is fatal, exits 70, emits
only the fixed stderr line, and produces no rejection line or published RECORD.
Both out-of-domain range branches use that one member. Existing RECORD bounds,
four rejection codes, enumeration, custody, and 42 V02 rows remain unchanged.

Precedence is Topic 1, then its approved Topic-2 error amendment, then this
narrow U4-001 supplement for its exact domain, local dependencies, and count
qualifications. Its Amendment surface controls; no unrelated predecessor rule
is reopened. Topic 4 must consume the exact approved clarification and review
unchanged. The U4-001 review-stage `TOPIC 4 MAY RESUME: NO` reflected pending
durable preservation only. Historically, that checkpoint allowed a separate
Topic-4 task after push and synchronization. The U4-002 project-management gate
now requires the release-path rebaseline before any further authorization.

### Approved U4-002 predecessor clarification — PRESERVED BY THIS COMMIT

Approved clarification, intentionally unchanged including its editorial MINOR:
`docs/OS1_TOPIC1_CANDIDATE_TIMESTAMP_REPRESENTABILITY_CLARIFICATION_U4_002_V1.md`

- 524 lines
- SHA-256:
  `af7acbc6a04526f00be4a92a4251620e5a0f74adb6768502bdc5a4f1ea888abd`

Approved independent review, faithfully preserved from the session review:
`docs/OS1_TOPIC1_CANDIDATE_TIMESTAMP_REPRESENTABILITY_CLARIFICATION_U4_002_V1_REVIEW.md`

- 303 lines
- SHA-256:
  `b6a0a2efb2f3714bf0f3588becdff054ac248b3da61715afdb408514181b88f9`

Approval result:

- `U4-002 INDEPENDENT CLARIFICATION REVIEW: APPROVED`
- `BLOCKERS: 0`
- `IMPORTANT FINDINGS: 0`
- `MINOR FINDINGS: 1`
- `U4-002 AUTHORITY CLOSED: YES`
- `TOPIC 4 MAY RESUME: NO`
- `CORRECTIVE IMPLEMENTATION AUTHORIZED: NO`

The one MINOR is an editorial section-label mismatch at clarification line 380:
S1R line 158 belongs to Vocabulary-count review, not Error/exit review.
The line number and arithmetic are correct; it cannot affect implementation
behavior and requires no correction for approval. The exact reviewed bytes
remain unchanged.

U4-002 is closed by the approved Topic-1 supplement. It adds exactly the
T4-inspect-only fatal member `CANDIDATE_TIMESTAMP_UNREPRESENTABLE`, producing
exit 70, zero stdout, and only its fixed error line plus LF on stderr.
All four required candidate stat timestamps and all four outside-domain
seconds/nanoseconds branches are covered. `timestamp_v1` remains unchanged;
no normalization is authorized. No successful or rejection line represents
this failure, and no complete RECORD or valid completion is published for it.
RECORD totality and four rejection codes/pairings remain unchanged.

The effective approved vocabulary is now 55 + 4 + 1 + 1 = 61 members,
retaining `OS1_METADATA_HELPER_ERRORS_V2`. The validator registry remains
exactly 42 rows, V02-001 through V02-042, with unchanged predicates and
applicability. U4-001's entire size domain remains approved and preserved.
Precedence is T1, A1, U4-001, then U4-002 only on its stated Amendment surface.
The timestamp check follows U4-001 size representability and precedes later
resource/data/hash work; FinderInfo/alias outcomes remain earlier controlling
outcomes. This is an approved local dependency, not a full Topic-4 algorithm.

Topic 3 remains unchanged. Topic-4 mechanics, Topic-5 completion/events, and
Topic-6 conformance design remain delegated. No corrective implementation is
authorized. Durable preservation does not resume Topic 4: the release-path
rebaseline below is required before authorizing additional Topic-4 work.

## Topic 2 — COMPLETE AND DURABLE

Status: **APPROVED AND COMMITTED**

Title: **Custody authority, lifecycle, and validator registry**

Design:
`docs/OS1_CUSTODY_AUTHORITY_LIFECYCLE_AND_VALIDATOR_REGISTRY_V1.md`

- 1213 lines
- SHA-256:
  `406c8703da2454423308fbd347d2c0f2126228ba9c90a9ac00282e597a06869c`

Final independent review:
`docs/OS1_CUSTODY_AUTHORITY_LIFECYCLE_AND_VALIDATOR_REGISTRY_V1_REVIEW.md`

- 227 lines
- SHA-256:
  `e0ded24de01d51695b8881fa929142cd1426a38aca1291de03461ca44849e55f`

Approval result:

- `TOPIC 2 APPROVED: YES`
- `BLOCKER 0`
- `IMPORTANT 0`
- `MINOR 0`
- unresolved Topic-2 implementation-critical decisions: `0`

Topic-2 gap state:

- G-002: `CLOSED BY TOPIC 2`
- G-003: `CLOSED BY TOPIC 2`
- G-004: `CLOSED BY TOPIC 2`
- G-005: `CLOSED BY TOPIC 2`
- G-009: `CLOSED BY TOPIC 2`
- G-010: `CLOSED BY TOPIC 2`
- G-011: `CLOSED BY TOPIC 2`
- G-012: `CLOSED BY TOPIC 2`
- G-013: `CLOSED BY TOPIC 2`
- G-014: `PARTIALLY CLOSED`; Topic-2 placement/identity authority is closed,
  Topic 3 now closes MAP enforcement as recorded below; Topics 4–5 retain
  their assigned producer enforcement.
- G-028: `PARTIALLY CLOSED`; Topic 2 closes the normative registry, ordering,
  and error mapping, while Topic 6 retains deterministic fixtures/assertions.

## Topic 3 — COMPLETE AND DURABLE

Status: **INDEPENDENTLY APPROVED — PRESERVED BY THIS COMMIT**

Title: **Enumeration and MAP production**

Topics 1 and 2 remain approved and durable. Topic-3 authoring exposed authority
blockers U3-001 and U3-002, and a fresh independent blocker review confirmed
both. The narrow clarification below and its approving review are already
approved, committed, and durable, with `BLOCKER 0`, `IMPORTANT 0`, and
`MINOR 0`.

Approved clarification:
`docs/OS1_TOPIC3_AUTHORITY_CLARIFICATION_U3_001_U3_002_V1.md`

- 512 lines
- SHA-256:
  `6763a37804c0f57cc8e637746990844d92546d564ae20dd52c35f530b7e8d09c`

Approving review:
`docs/OS1_TOPIC3_AUTHORITY_CLARIFICATION_U3_001_U3_002_V1_REVIEW.md`

- 318 lines
- SHA-256:
  `2ebb33947885ceb87ecf3341acd2224c61c2a65fb8f60be5fb022b45324ad838`

**U3-001 CLOSED: YES**

**U3-002 CLOSED: YES**

U3-001 and U3-002 remain closed and are fully integrated into the independently
approved Topic-3 design. The exact approved design and approving review below
are preserved unchanged by this commit; no further Topic-3 design work is
performed here.

Approved design:
`docs/OS1_ENUMERATION_AND_MAP_PRODUCTION_V1.md`

- 1154 lines
- SHA-256:
  `09aa74d6d0efbdc0e438bc1662f5b910b4b66dfbbeeb03aeb380add3d9a8a0b4`

Final independent approving review:
`docs/OS1_ENUMERATION_AND_MAP_PRODUCTION_V1_REVIEW.md`

- 369 lines
- SHA-256:
  `32e1aebc71de5fa0d063951e9e8e426293539f41afb45b0e9d2d26be9bb21220`

Approval result:

- `TOPIC 3 REVIEW COMPLETE: YES`
- `TOPIC 3 APPROVED: YES`
- `U3-001 APPROVED: YES`
- `U3-002 APPROVED: YES`
- `0 BLOCKER / 0 IMPORTANT / 0 MINOR`
- unresolved Topic-3 implementation-critical decisions: `0`
- `CORRECTIVE IMPLEMENTATION AUTHORIZED: NO`

Current Topic-3 gap dispositions:

- G-006: `CLOSED` through the approved U3 clarification and Topic-3 integration.
- G-007: `CLOSED` by approved Topic 3.
- G-008: `CLOSED` by Topic 1 plus approved Topic 3, consuming Topic 2 and U3.
- G-014: `PARTIALLY CLOSED` — MAP enforcement is closed; RECORD remains
  Topic 4; completion/events remain Topic 5.
- G-028: `PARTIALLY CLOSED` — Topics 4/5 retain operation-specific
  integrations; Topic 6 retains fixtures/assertions/conformance definitions.

The confirming blocker review is
`docs/OS1_TOPIC3_BLOCKER_REVIEW_U3_001_U3_002.md`:

- 314 lines
- SHA-256:
  `d7aeb9277550a20c30255e41fa62718d924706dad1e9d54152b62d40975f3639`

The blocker review remains unchanged, uncommitted review evidence only and is
excluded from this preservation commit. Approved Topic-3 authority does not
depend on that working artifact, `/tmp`, or the current conversation.

## Topics 4–6

Historically, the U4-001 checkpoint designated Topic 4 as the next targeted
re-specification topic after synchronized preservation. U4-001 and U4-002 are
now approved and preserved, but the user has inserted a Phoenix 0.1 release-path
rebaseline before authorizing further Topic-4 work. Topic 4 has not resumed.
The Topic-3 review's `TOPIC 4 MAY BEGIN: NO` records that review task's scope,
not an outstanding Topic-3 approval defect.

- Topic 4 — Inspect, candidate access, and RECORD production:
  **INCOMPLETE — NOT RESUMED — RELEASE-PATH REBASELINE REQUIRED BEFORE FURTHER WORK**
- Topic 5 — Terminal completion and operational-event publication:
  **INCOMPLETE / NOT STARTED**
- Topic 6 — Deterministic conformance-test registries:
  **INCOMPLETE / NOT STARTED**

Every topic follows the same discipline:

1. author design
2. independent review
3. narrow correction if required
4. fresh independent re-review
5. approval
6. preserve the exact final approving review in the repository
   byte-for-byte
7. commit design and review together
8. push and verify synchronization
9. only then begin the next topic

## Implementation authorization

**CORRECTIVE IMPLEMENTATION AUTHORIZED: NO**

Corrective implementation must not begin merely because Topics 1, 2, and 3
are approved and durable.
It remains unauthorized until:

- Topics 1–6 are all designed;
- each is independently reviewed and approved;
- each approved design/review pair is durably committed; and
- a final design-authority-chain review explicitly determines that corrective
  implementation is ready.

## Current unapproved implementation state

Pre-existing metadata-helper implementation work exists under
`tools/os1-metadata-helper/`. It is **NON-AUTHORITATIVE AND UNAPPROVED**. It
must not be committed or treated as design authority until the complete
targeted re-specification chain authorizes corrective implementation.
The helper tree remains protected: do not read or modify it during preservation
or targeted re-specification work.

## Protected unrelated work

`docs/DECISIONS.md` and `docs/ROADMAP.md` are pre-existing modified files
outside this checkpoint commit.

`docs/CONTROLLED_TRACK3_2_MIDI_CHANNEL_CHANGE.md` is protected untracked
Experiment 032 material. Its expected SHA-256 is
`57dcf5007a3fb7ca98efb51dc20a60f9485835e41d0ce1d228a2cbf5fbd6b747`.
Its contents must not be read during OS1 checkpoint or re-specification work
unless separately authorized.

All pre-existing protected modified/untracked work remains preserved outside
the U4-001 and U4-002 preservation commits, including the unchanged U3 blocker
review and every file under `tools/os1-metadata-helper/`. None is staged or incorporated into this
authority package; helper implementation contents remain protected from reads.

## Evidence/provenance boundaries

At this checkpoint:

- no OS1 reserve artifact has been selected or accessed for this phase;
- no authentic Studio Vision candidate has been selected or run;
- no blind reference MIDI has been inspected or created;
- provenance lock has not begun; and
- corrective implementation has not begun.

These are status statements based on the current documented state, not claims
beyond it.

## Current project-management gate

**NEXT REQUIRED ACTION:**
**PHOENIX 0.1 RELEASE-PATH REBASELINE BEFORE FURTHER OS1 TOPIC-4 WORK**

The rebaseline will classify remaining work as REQUIRED, VALUABLE, or OPTIONAL
against the shortest credible path to a usable Phoenix 0.1 release.
This preservation task does not perform that rebaseline. Synchronizing the
preservation commit alone does not authorize more Topic-4 work.

## Conditional future Topic-4 resume instruction

The historical next design task was Topic 4: Inspect, candidate access, and
RECORD production. It remains incomplete and has not resumed. Only after the
release-path rebaseline and separate authorization for further Topic-4 work,
a future task must:

1. verify repository synchronization and worktree preservation
2. read the reconciliation and its independent review
3. read the approved Topic-1 design, amendment, and reviews
4. verify and read the exact approved U4-001 clarification/review identities
   above; consume the clarification unchanged at its stated precedence
   and also verify/read the approved U4-002 clarification and review unchanged
5. read the approved Topic-2 design and review
6. read the approved U3 clarification and approving review
7. verify the exact committed Topic-3 design/review identities above and read
   that approved pair as authority
8. identify only the reconciliation gaps delegated to Topic 4, preserving
   Topics 1–3, the narrow U3 amendments, U4-001, and U4-002 as closed authority

Topics 4–6 remain incomplete. No Topic-4 design decisions are made by this
checkpoint. Corrective implementation and authentic/reference/candidate access
remain unauthorized. Any later access must be explicitly permitted by approved
Topic-4 authority and satisfy every applicable existing custody/access gate;
neither this checkpoint nor U4-001/U4-002 preservation grants such access.
