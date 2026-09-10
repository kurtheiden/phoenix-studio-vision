# OS1 Targeted Re-Specification Resume Checkpoint

Checkpoint date: 2026-09-03.

Preservation update: 2026-09-10, after independent Topic-3 approval.
Pre-preservation HEAD, main, and origin/main were
`cb25005545bf5f8c464e2edc3930174efb40547c`
(`Preserve Topic 3 authority clarification`), divergence `0/0`.
This preservation commit, `Define OS1 enumeration and MAP production`, makes
the approved Topic-3 design/review pair durable with this checkpoint.

This is a status and handoff document. It is not new design authority and does
not supersede or alter any approved design authority.

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
- 59 `OS1_METADATA_HELPER_ERRORS_V2` members after the approved additive
  Topic-1 amendment below
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

Topic 4 is the NEXT targeted re-specification topic after this preservation
commit is pushed and synchronization is verified. This preservation task does
not begin it. The approving review's `TOPIC 4 MAY BEGIN: NO` records the scope
of that review task, not an outstanding Topic-3 approval defect.

- Topic 4 — Inspect, candidate access, and RECORD production:
  **INCOMPLETE / NOT STARTED — NEXT**
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

## Evidence/provenance boundaries

At this checkpoint:

- no OS1 reserve artifact has been selected or accessed for this phase;
- no authentic Studio Vision candidate has been selected or run;
- no blind reference MIDI has been inspected or created;
- provenance lock has not begun; and
- corrective implementation has not begun.

These are status statements based on the current documented state, not claims
beyond it.

## Resume instruction

**NEXT ACTION:**

**AUTHOR OS1 TARGETED RE-SPECIFICATION TOPIC 4:**
**INSPECT, CANDIDATE ACCESS, AND RECORD PRODUCTION**

After this preservation commit is pushed and synchronization is verified,
a separate Topic-4 task must:

1. verify repository synchronization and worktree preservation
2. read the reconciliation and its independent review
3. read the approved Topic-1 design, amendment, and reviews
4. read the approved Topic-2 design and review
5. read the approved U3 clarification and approving review
6. verify the exact committed Topic-3 design/review identities above and read
   that approved pair as authority
7. identify only the reconciliation gaps delegated to Topic 4, preserving
   Topics 1–3 and the narrow U3 amendments as closed authority

Topics 4–6 remain incomplete. No Topic-4 design decisions are made by this
checkpoint. Corrective implementation and authentic/reference/candidate access
remain unauthorized.
