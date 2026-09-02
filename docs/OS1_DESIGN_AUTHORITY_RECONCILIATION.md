# Purpose

The original independently approved OS1 design-authority artifacts were stored
only under `/tmp`. They were lost during the planned pause/reboot before they
were durably copied into this repository. They are not present in the current
filesystem, and this document is not a recovery or byte-for-byte reconstruction
of any of them.

Repository implementation and Git history remained intact. At reconciliation
start, `HEAD`, `main`, and `origin/main` were all
`cb61cbcb08cdc82901a3ae7990af76754a6b12ec` (`Adopt OS1 reserve intake
protocol`) with `0/0` divergence and no staged changes. The expected
pre-pause uncommitted metadata-helper and documentation work also survived.

This document inventories the authority that is still durable in the
repository, records historical provenance facts supplied for the lost approved
design cycle, and identifies the authority gaps that must be reviewed before
corrective implementation. It does not authorize reserve or candidate access,
reference creation or inspection, provenance lock, intake, or implementation.

# Historical approved authority

Every digest in this section is **HISTORICAL**. It identifies bytes reported as
present during the completed design/review cycle, not any file currently in the
filesystem and not this document.

## Root-bound evidence identity amendment v1

- Historical artifact: `/tmp/phoenix-os1-root-bound-evidence-identity-amendment-v1.md`
- Historical artifact line count and SHA-256: not supplied in the surviving
  record
- Historical independent-review filename and line count: not supplied in the
  surviving record
- Historical review SHA-256: **HISTORICAL**
  `23f6d7ccb87c51dbaa4d07e246bd1cb75df96d98b9d1af23ecc642afe800f87e`
- Historical verdict: `APPROVED`

The approved rule was that OS1 validation evidence is root-bound and
content/correlation-bound, not exact-path-bound.

## Terminal RECORD authority v3.1

- Historical artifact: `/tmp/phoenix-os1-terminal-record-authority-decision-v3.1.md`
- Historical line count: not supplied in the surviving record
- Historical SHA-256: **HISTORICAL**
  `49ba5fc9abd2685aeb89e0f81befe90c6f596d2528d20fb6a9f093c50c74e0f9`
- Historical review:
  `/tmp/phoenix-os1-terminal-record-authority-decision-v3.1-review.log`
- Historical review line count: not supplied in the surviving record
- Historical review SHA-256: **HISTORICAL**
  `e6cb6c574a22528754376d580b1424c55bc4e79a82e9e146b3c0274a930ad304`
- Historical verdict: `APPROVED`; `BLOCKER 0`; `IMPORTANT 0`; `MINOR 3`;
  no unresolved implementation-critical decisions

## Completed metadata RECORD contract v1

- Historical artifact:
  `/tmp/phoenix-os1-metadata-record-contract-amendment-v1-complete.md`
- Historical line count: `359`
- Historical SHA-256: **HISTORICAL**
  `6358bdc9f87d39801f525ede3dd87a237ee1bd11505c6efd1368334529fbad0f`
- Historical review:
  `/tmp/phoenix-os1-metadata-record-contract-amendment-v1-complete-review.log`
- Historical review line count: not supplied in the surviving record
- Historical review SHA-256: **HISTORICAL**
  `78a33d9107ca714c803278e31eeacaa8bc0f87a8809b542c16085511e3e177e4`
- Historical verdict: `APPROVED`; `BLOCKER 0`; `IMPORTANT 0`; `MINOR 3`;
  no unresolved implementation-critical decisions

## MAP-binding amendment design v6

- Original reviewed v6 historical SHA-256: **HISTORICAL**
  `b38daee8efd1bf20acf9e179a9b09ac82d788a0f4b015d1c8e0c271920bfadf0`
- First-review historical SHA-256: **HISTORICAL**
  `9d83820842104ebe10aaa77b1cf6d03448bfe3acb00d0fa8c34daa2591b937b4`
- Corrected historical artifact:
  `/tmp/phoenix-os1-map-binding-amendment-design-v6.md`
- Corrected historical line count: `337`
- Corrected historical SHA-256: **HISTORICAL**
  `bea6056994bbd4249cbb1e9db1bebc535e160f0c83423354b38deaca26b8d66a`
- Historical final re-review:
  `/tmp/phoenix-os1-map-binding-amendment-design-v6-rereview.log`
- Historical final re-review line count: `181`
- Historical final re-review SHA-256: **HISTORICAL**
  `6b20a1a96c824184803c820abc8d9c22fd0bf1df990ddf4c5c9159b7708f7d11`
- Historical final verdict: `OS1 MAP-BINDING V6 APPROVED`;
  `MAP-BINDING V6 APPROVED: YES`; B1/B2/B3 closed/pass; prior subsystem
  regression none; executable-interface finding closed; `BLOCKER 0`;
  `IMPORTANT 0`; `MINOR 0`; implementability pass; no unresolved
  implementation-critical decisions
- Historical scope: `V6-001` through `V6-077`, 77 tests
- Historical authorization boundary: design chain ready for corrective
  implementation, but corrective implementation was not authorized by the
  review itself and had not begun

Only sections 5.1, 8, 14, and 15 were historically reported changed between
the first-reviewed and corrected v6. The first review's IMPORTANT finding was
that an enumerate interface carrying only a preflight ID could not, without a
hidden lookup, prove that the MAP output parent was the authorized
`PRIVATE_OUTPUT_ROOT`. Its MINOR finding concerned executable-interface
wording/reference. Both were reported closed by the corrected design and final
re-review.

# Durable repository authority already present

The following repository text independently preserves parts of the required
authority. “Durable” here means present in repository files; where noted, a
passage is surviving uncommitted work rather than committed baseline authority.

- `docs/OS1_BLIND_VALIDATION_PROTOCOL.md`, “Status and purpose” through
  “Candidate-selection boundary,” is committed governing authority for
  blindness classification, candidate eligibility, provenance lock,
  quarantine, Phoenix-only observation, freeze/reveal ordering, comparison,
  integrity, gate criteria, stop conditions, records, and the prohibition on
  selecting or inspecting a candidate in that protocol task.
- `docs/OS1_RESERVE_INTAKE_PROTOCOL.md`, “Status and authority” and “Roles and
  concealment boundary,” makes reserve intake subordinate to the blind
  protocol, separates roles, and denies Phoenix, Studio Vision, reference,
  copying, and provenance-lock authority.
- That intake protocol's “Candidate-only enumeration boundary,” “Command and
  wrapper firewall,” “Permitted observations,” “Content firewall through
  selection review,” and “SHA-256 boundary” specify candidate-only,
  nonrecursive/no-follow metadata access; the bounded FinderInfo and
  resource-fork observations; full-data-fork hashing limits; forbidden content
  features and tools; and conservative unsupported-object handling.
- Its owner-attestation, blindness, development-exposure, reference,
  logical-family, representative, cohort/selection, replacement, privacy, and
  incidental-exposure sections preserve the nontechnical custody and selection
  policy.
- Its “Mandatory execution order,” “Stop and containment conditions,” and
  “Private intake record” preserve the A–L phase boundary, independent-review
  gates, incident handling, and requirement to keep candidate-specific evidence
  outside the repository.
- `docs/DECISIONS.md`, “preregister the OS1 blind validation protocol” and
  “require metadata-only intake for untouched OS1 reserves,” records the
  accepted governing decisions at the committed baseline.
- `docs/DECISIONS.md`, “isolate the OS1 reserve metadata helper,” is surviving
  uncommitted documentation. It records the standalone Swift/Darwin boundary,
  no Phoenix linkage, nonrecursive no-follow enumeration, descriptor-bound
  inspection, bounded FinderInfo/resource-fork behavior, two-pass SHA-256, and
  the independent-review requirement.
- `docs/ROADMAP.md`, OS1A through OS1C-R, includes surviving uncommitted wording
  that the protocol passed review while the synthetic-only helper still awaits
  source/binary/schema review and reserve access remains prohibited.
- `tools/os1-metadata-helper/README.md` is surviving uncommitted documentation,
  not approved final-design authority. It records the old two-command helper
  interface, private/exclusive output intent, bounded observations, synthetic
  testing, and the explicit no-use-before-review boundary.

No current repository document was found that durably and unambiguously
specifies the complete approved executable-review, preflight/path-map,
MAP/binding, freeze-review, run-authorization, consumption-reservation,
completed RECORD, terminal-completion, later validation, or operational-event
contracts described by the historical approved-design record.

# Surviving implementation state

The pre-pause uncommitted helper survives at
`tools/os1-metadata-helper/Sources/main.swift`, with its README, build/test
scripts, and synthetic Python tests. It is an implementation-review draft that
predates the completed authority chain, not evidence that the later approved
design was implemented.

The draft implements an old `enumerate --root ... --map-output ...` and
`inspect --map ... --record-output ...` interface. It uses
`OS1_METADATA_HELPER_MAP_V1`, `OS1_METADATA_HELPER_RECORD_V1`, and
`OS1_METADATA_HELPER_ERRORS_V1`; stores an exact root path and device/inode in
the MAP; enumerates immediate children without following them; inspects regular
files by retained descriptors; bounds FinderInfo and resource-fork probes; and
performs two whole-data-fork SHA-256 passes with race checks. Its tests cover
synthetic enumeration, nonexposure, hashes, xattrs, output permissions,
collisions, tampering, replacement/mutation races, and bounded errors.

Known differences from the later approved design remain in the draft:

- enumerate does not accept or validate executable authorization, preflight,
  or path-map evidence and cannot prove the authorized candidate and private
  output root identities;
- inspect does not consume the approved MAP/binding/freeze-review/run
  authorization/consumption-reservation authority chain;
- the draft binds later inspection to the MAP's serialized exact root path;
- its schemas, cardinality, record IDs, line forms, canonical key ordering,
  size caps, error enum, and fatal/nonfatal outcomes are not the approved final
  contracts;
- it can publish a partial RECORD containing an `ERROR` row after a fatal
  artifact failure;
- its publication helper unlinks a failed temporary and lacks the approved
  directory fsync, final root/parent rechecks, reopen validation, terminal
  completion creation, and operational-event rules; and
- its tests are the earlier synthetic suite, not the historical
  `RECORD-CONTRACT-001` through `060`, `V6-001` through `077`, or
  `TERM-AUTH-189` registry.

Corrective implementation would therefore have to replace or extend these
interfaces and contracts, not merely patch the originally reported v6
enumerate finding.

# Reconciliation gaps

Each numbered item is implementation-critical authority that is not already
durably and unambiguously specified in repository documentation. The only
available support for it is the historical approved-design record supplied to
this reconciliation; details absent from that record must not be inferred.

1. **G-001 — Complete schemas and canonical bytes.** The complete closed field
   sets, types, exact key order, byte grammar, size bounds, and validation rules
   for every executable-review, authorization, preflight, path-map, MAP,
   binding, freeze-review, run, reservation, RECORD, completion, and event
   artifact are absent.
2. **G-002 — Custody command contract.** The full command set, argument grammar,
   sequencing, outputs, exit behavior, and error mapping for
   `phoenix-os1-custody` are absent.
3. **G-003 — Executable-review artifact contract.** The complete contract for
   `freeze-executable-review`, including the four source/build/executable
   inputs, review output, review identities, and deterministic-build checks, is
   absent.
4. **G-004 — Executable authorization contract.** The complete
   `verify-executables` validation and `authorization-output` contract is
   absent.
5. **G-005 — Preflight and path-map contracts.** Their complete schemas,
   creation authority, identity rules, four terminal root identities,
   timestamps, repository/custody continuity, and custody-tool provenance are
   absent.
6. **G-006 — Corrected enumerate interface.** The repository does not specify
   the approved `enumerate --executable-authorization FILE --preflight FILE
   --path-map FILE --root ROOT --map-output FILE` interface.
7. **G-007 — Enumerate validation algorithm.** The full approved closed order
   from invocation grammar through acquisition/correlation, helper identity,
   no-follow root and output-parent authorization, separation/absence/race
   checks, child enumeration, and final rechecks is absent.
8. **G-008 — MAP contract.** The complete MAP schema and canonicalization are
   absent, including copied authority identities and the precise treatment of
   the supplied root path and current root/output identities.
9. **G-009 — Binding contract.** The complete binding schema, construction,
   canonical bytes, MAP correlation, publication, and validation rules are
   absent.
10. **G-010 — Freeze-review contract.** The complete freeze-review schema and
    procedure are absent, including `binding_sha256` as the sole external
    MAP/binding pair authority.
11. **G-011 — Run authorization contract.** Its schema, creation/verification
    procedure, identity/correlation checks, and lifecycle are absent.
12. **G-012 — Consumption reservation contract.** Its schema, creation,
    exclusivity/use-count behavior, validation, and lifecycle are absent.
13. **G-013 — Root-bound evidence algorithm.** The complete bounded canonical
    byte and digest rules, frozen authorized-root representation, no-follow
    chain/terminal/parent checks, alternate-basename rules, and race checks are
    absent.
14. **G-014 — Evidence location split.** The repository does not durably define
    the complete placement and identity rules for MAP/binding/RECORD under
    `PRIVATE_OUTPUT_ROOT` and freeze-review/run/reservation/completion under
    `CUSTODY_RECORD_ROOT`.
15. **G-015 — Validator non-agency contract.** The full mechanical guarantees
    that validation only authenticates explicitly supplied instances and never
    discovers, selects, copies, renames, repairs, migrates, or substitutes
    evidence are absent.
16. **G-016 — Copy/non-creation semantics.** The exact rules by which copied
    run/reservation bytes create no new run ID, use count, freeze-review
    authority, or completion are absent.
17. **G-017 — Completed RECORD schema.** The exact ordered
    `INSPECTED_REGULAR_FILE` and `EXPECTED_REJECTION` closed schemas and the
    four rejection codes are absent.
18. **G-018 — RECORD cardinality and indexing.** The durable repository lacks
    the approved zero-entry/zero-byte rule, MAP-index-to-line mapping,
    `R-000001` through `R-004096`, 4096-line limit, 4096-byte outer-line cap,
    1631-byte maximum valid success line, and 6,680,576-byte total maximum.
19. **G-019 — RECORD metadata/privacy contract.** The complete allowlist and
    explicit denylist, exact serialization of four stat times, kind, size,
    Finder Type/Creator, resource-fork presence, two hashes, and source/runtime
    digests are absent.
20. **G-020 — FinderInfo/alias ordering.** The exact descriptor `fgetxattr`,
    `ENOATTR`, 8-to-4096-byte, first-eight-byte, `alis` rejection, and
    no-candidate-data-read-after-alias-decision algorithm is absent.
21. **G-021 — Resource-fork contract.** The complete presence-only probe and
    prohibition on serialized resource-fork size/content are not specified as
    part of the final RECORD algorithm.
22. **G-022 — RA01–RA19.** The complete ordered inspection algorithm, including
    two 64 KiB SHA passes, object bindings, and every repeated race check, is
    absent.
23. **G-023 — Fatal/nonfatal matrix.** The exact classification of stable
    nonregular/alias rejection versus fatal I/O, race, mutation, malformed
    evidence, and probe failures is absent.
24. **G-024 — RECORD publication.** The complete opaque-0600 temporary,
    complete-reread-before-IP01, exclusive publication, and failed
    pre-IP01-temp retention/non-authority rules are absent.
25. **G-025 — Terminal authority model.** The durable repository lacks the
    distinction between `OPERATIONAL_SUCCESS` and
    `DURABLE_EVIDENCE_VALIDITY`, and the axiom “BYTE-VALID COMPLETION IS THE
    TERMINAL DURABLE AUTHORITY.”
26. **G-026 — Completion and operational event schemas.** The complete
    `OS1_RECORD_COMPLETION_V1` and nonauthoritative
    `OS1_OPERATIONAL_EVENT_V1` schemas and creation rules are absent.
27. **G-027 — IP01–IP11.** The exact publication operation classes, failure
    mapping, required fsync/rename/final/reopen/root verifications, E09 sources,
    and IP11 operational-event suppression are absent.
28. **G-028 — Validator registry and error/test contracts.** The 42-row
    validator registry, complete `OS1_METADATA_HELPER_ERRORS_V2` enum and exit
    mapping, exact stderr grammar, and full stable test definitions through
    `RECORD-CONTRACT-060`, `V6-077`, and `TERM-AUTH-189` are absent.

# Preserved approved requirements

Provenance category **A** means durable repository authority. Category **B**
means only the historical approved-design record supplied for this
reconciliation. A B item is preserved as a historical requirement, but its
missing complete contract remains a reconciliation gap where identified above.

- **A:** The blind-validation protocol governs; reserve intake is subordinate
  and grants no candidate inspection, Studio Vision/reference access,
  provenance-lock, or Phoenix-run authority.
- **A:** Intake is candidate-only, nonrecursive, no-follow, metadata-bounded,
  private, independently reviewed, and separated from selection, lock, and
  blind execution.
- **A:** Permitted observations and complete-data-fork SHA-256 have narrow
  identity/provenance/duplicate/ranking purposes; content inference and
  content-bearing output are forbidden.
- **A:** FinderInfo is limited to Type/Creator; resource fork is presence-only;
  arbitrary xattr names/values and candidate content features are forbidden.
- **A:** Family grouping, representative choice, eligibility, complete neutral
  ordering, fallback promotion, quarantine, privacy, and stop/containment rules
  are fixed before candidate access.
- **A:** The helper is standalone, synthetic-only, and not authorized for
  reserve use before independent review. The more detailed helper statement is
  surviving uncommitted repository documentation.
- **B:** Evidence is root-bound and content/correlation-bound, not
  exact-path-bound; a final basename or exact absolute pathname is not durable
  authority.
- **B:** A validator authenticates only the explicitly supplied filesystem
  instance using frozen root identity, current no-follow properties, bounded
  canonical bytes, applicable digests, and complete correlations; it performs
  no discovery, selection, copying, renaming, repair, migration, or
  substitution.
- **B:** MAP, binding, and RECORD belong under `PRIVATE_OUTPUT_ROOT`;
  freeze-review, run authorization, consumption reservation, and completion
  belong under `CUSTODY_RECORD_ROOT`.
- **B:** A same-root alternate immediate-child basename can validate only when
  explicitly supplied and every authority check passes.
- **B:** `binding_sha256` in freeze-review is the sole external MAP/binding pair
  authority; `validate-record` is read-only observational; copying run or
  reservation bytes creates no authority or lifecycle event.
- **B:** Candidate-root identity continuity is required across MAP, binding,
  freeze-review, and run authorization.
- **B:** The corrected enumerate interface explicitly supplies executable
  authorization, preflight, path map, root, and MAP output, and follows the
  historically recorded closed validation order before enumeration.
- **B:** Parsing supplied preflight/path-map evidence is not candidate-root
  filesystem access. Serialized paths are not durable authority.
- **B:** MAP provenance includes preflight and executable IDs, copied authorized
  root identities, the validated explicit root bytes/current root identity,
  and the authorized current MAP-output-parent identity.
- **B:** The approved executable-review interfaces are:

  ```text
  phoenix-os1-custody freeze-executable-review \
    --implementation-review-root ROOT --helper-source FILE --custody-source FILE \
    --helper-build-command FILE --custody-build-command FILE \
    --helper-executable-a FILE --helper-executable-b FILE \
    --custody-executable-a FILE --custody-executable-b FILE --review-output FILE

  phoenix-os1-custody verify-executables \
    --implementation-review-root ROOT --review-identities FILE \
    --helper-executable FILE --custody-executable FILE --authorization-output FILE
  ```

- **B:** A complete RECORD is exact ordered UTF-8 JSONL with no BOM,
  whitespace, or open fields; zero MAP entries produce a zero-byte RECORD; MAP
  index equals RECORD line index; IDs and caps are as recorded in G-018.
- **B:** RECORD lines are `INSPECTED_REGULAR_FILE` or `EXPECTED_REJECTION` with
  one of four approved rejection codes; stable nonregular/alias results are
  nonfatal, while I/O/race/mutation/malformed-evidence/probe failures are fatal.
- **B:** The RECORD metadata model, FinderInfo/alias rule, resource-fork rule,
  two-pass 64 KiB hashes, RA01–RA19 algorithm, bounds, publication semantics,
  and V2 error grammar are the historically approved requirements summarized
  in G-019 through G-024 and G-028.
- **B:** `metadata-helper inspect` is the sole completion creator. Operational
  success and later durable evidence validity are distinct; byte-valid
  completion is terminal durable authority.
- **B:** Completion uses `OS1_RECORD_COMPLETION_V1`; operational history uses
  nonauthoritative `OS1_OPERATIONAL_EVENT_V1`; terminal publication follows
  IP01–IP11 and E09 draws exactly from MAP, binding, run authorization, and
  consumption reservation.
- **B:** The historical deterministic registries contain 42 validator rows,
  `RECORD-CONTRACT-001` through `060`, `V6-001` through `077`, and tests through
  `TERM-AUTH-189`. Their individual definitions are not preserved here.

# Historical identity limitation

> **The historical SHA-256 values authenticate artifacts that no longer exist
> in the current filesystem. They cannot authenticate this reconciliation
> document or any later reconstruction.**

No digest in the historical section is asserted to match current bytes. A new
digest computed for this document identifies only this new document.

# Implementation-authority status

**CORRECTIVE IMPLEMENTATION AUTHORIZED: NO**

The durable repository authority and historical approved-requirement record
have not yet been independently judged sufficient to replace the lost design
chain. The gaps above include complete schemas, algorithms, registries, and
failure mappings that cannot safely be reconstructed from names or summaries.

# Required next step

Require an independent review of this reconciliation document. That review
must decide whether:

1. the combined durable repository authority plus explicitly recorded
   historical approved requirements is complete enough to become a new
   implementation authority without silently redesigning anything; or
2. one or more design questions must be re-specified and independently
   approved before implementation.

The independent review must not treat historical hashes as authentication of
this document or of a reconstruction. Until that review reaches an explicit
authority decision, no corrective implementation may begin.
