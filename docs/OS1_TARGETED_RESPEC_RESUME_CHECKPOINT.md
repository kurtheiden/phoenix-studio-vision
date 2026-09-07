# OS1 Targeted Re-Specification Resume Checkpoint

Checkpoint date: 2026-09-03.

This is a status and handoff document. It is not new design authority and does
not supersede or alter any approved design authority.

## Durable baseline

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
  while Topics 3–5 retain assigned producer enforcement.
- G-028: `PARTIALLY CLOSED`; Topic 2 closes the normative registry, ordering,
  and error mapping, while Topic 6 retains deterministic fixtures/assertions.

## Topic 3 — NEXT

Status: **NOT STARTED**

Title: **Enumeration and MAP production**

This is the exact next targeted re-specification topic. Its design must consume
the durable Topic-1 and Topic-2 authority without changing either. Topic 3 has
not begun merely because Topic 2 is approved.

## Topics 3–6

- Topic 3 — Enumeration and MAP production: **NOT STARTED — NEXT**
- Topic 4 — Inspect, candidate access, and RECORD production: **NOT STARTED**
- Topic 5 — Terminal completion and operational-event publication:
  **NOT STARTED**
- Topic 6 — Deterministic conformance-test registries: **NOT STARTED**

Every topic follows the same discipline:

1. author design
2. independent review
3. narrow correction if required
4. fresh independent re-review
5. approval
6. copy the final approving review from `/tmp` into the repository
   byte-for-byte
7. commit design and review together
8. push and verify synchronization
9. only then begin the next topic

## Implementation authorization

**CORRECTIVE IMPLEMENTATION AUTHORIZED: NO**

Corrective implementation must not begin merely because Topics 1 and 2 are
approved.
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

**AUTHOR OS1 TARGETED RE-SPECIFICATION TOPIC 3:**
**ENUMERATION AND MAP PRODUCTION**

Before authoring Topic 3:

1. verify repository synchronization and worktree preservation
2. read the reconciliation and its independent review
3. read the approved Topic-1 design, amendment, and reviews
4. read the approved Topic-2 design and review
5. treat Topic-1 schemas/errors and Topic-2 custody/lifecycle/validator rules
   as closed authority
6. identify only the reconciliation gaps delegated to Topic 3
7. author Topic 3 without beginning implementation
