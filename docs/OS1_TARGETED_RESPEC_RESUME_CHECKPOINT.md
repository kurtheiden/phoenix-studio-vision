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
- 55 `OS1_METADATA_HELPER_ERRORS_V2` members
- maximum schema-derived successful RECORD line: 1124 bytes including LF
- maximum complete RECORD: 4,603,904 bytes
- gap accounting: 3 fully closed, 15 partially closed, 2 outside Topic 1, and
  0 unaddressed
- digest graph acyclic
- forward compatibility passed
- all I1–I6 closed

## Topic 2 — NEXT

Status: **NOT STARTED**

Title: **Custody authority, lifecycle, and validator registry**

This is the exact next design task. Topic 2 must consume Topic-1 schemas and
error authority without changing them.

Topic 2 should close the reconciliation gaps delegated to custody authority,
lifecycle and state transitions, root/preflight/path-map authority creation and
validation, executable review/authorization lifecycle, validator registry,
one-use/run/reservation authority mechanics, and related custody procedures.
This checkpoint does not invent a detailed Topic-2 specification.

The first future action is:

**AUTHOR OS1 TARGETED RE-SPECIFICATION TOPIC 2**

Then:

**INDEPENDENTLY REVIEW TOPIC 2**

Then, only if approved:

**COPY THE APPROVING REVIEW INTO THE REPOSITORY BYTE-FOR-BYTE**

Then:

**COMMIT AND PUSH THE TOPIC-2 DESIGN AND ITS APPROVING REVIEW TOGETHER**

Do not begin Topic 3 until Topic 2 has a durable approved checkpoint.

## Topics 3–6

- Topic 3 — Enumeration and MAP production: **NOT STARTED**
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

Corrective implementation must not begin merely because Topic 1 is approved.
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

**BEGIN OS1 TARGETED RE-SPECIFICATION TOPIC 2:**
**CUSTODY AUTHORITY, LIFECYCLE, AND VALIDATOR REGISTRY**

Before authoring Topic 2:

1. verify repository synchronization and worktree preservation
2. read the reconciliation and its independent review
3. read the approved Topic-1 design and review
4. treat Topic-1 serialized schemas, canonical-byte rules, digest domains,
   privacy boundaries, and public error vocabulary as closed authority
5. identify only the reconciliation gaps delegated to Topic 2
6. author Topic 2 without beginning implementation
