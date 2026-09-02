# Independent Review of OS1 Design Authority Reconciliation

## Review status

**Review result: PATH B — TARGETED RE-SPECIFICATION REQUIRED**

**CORRECTIVE IMPLEMENTATION AUTHORIZED: NO**

The reconciliation is an honest and usable inventory for the next design
step. It is not a complete implementation authority. The combined committed
protocols, surviving uncommitted repository material, and explicitly preserved
historical facts do not contain enough exact detail to implement the approved
system without making new implementation-critical design choices.

This review did not repair the reconciliation, modify helper code, inspect a
reserve or candidate, inspect or create reference MIDI, begin provenance lock,
or run candidate intake.

## 1. Integrity verification

- Working directory:
  `/Users/kurtheiden/Developer/phoenix-studio-vision`
- `HEAD`: `cb61cbcb08cdc82901a3ae7990af76754a6b12ec`
- `main`: `cb61cbcb08cdc82901a3ae7990af76754a6b12ec`
- `origin/main`: `cb61cbcb08cdc82901a3ae7990af76754a6b12ec`
- `main...origin/main`: `0` ahead, `0` behind
- Latest subject: `Adopt OS1 reserve intake protocol`
- Staged files: none
- Modified tracked files: `docs/DECISIONS.md`, `docs/ROADMAP.md`
- Expected untracked files present: protected Experiment 032, the
  reconciliation, and the six metadata-helper files
- Unexpected status entries: none
- Experiment 032 SHA-256, verified without reading its contents:
  `57dcf5007a3fb7ca98efb51dc20a60f9485835e41d0ce1d228a2cbf5fbd6b747`
- Reconciliation line count: `416`
- Reconciliation SHA-256:
  `baa27d81cf9f6fd134ba7ed995d5d330a9cb3ef259791816a46ad601a886e69e`

The identity stop gate passed.

## 2. Sources and authority classes

The review used only:

- `docs/OS1_DESIGN_AUTHORITY_RECONCILIATION.md`;
- committed `docs/OS1_BLIND_VALIDATION_PROTOCOL.md`;
- committed `docs/OS1_RESERVE_INTAKE_PROTOCOL.md`;
- the OS1 sections of `docs/DECISIONS.md` and `docs/ROADMAP.md`, compared with
  their `HEAD` versions and relevant Git history;
- the uncommitted metadata-helper README, Swift source, Python tests, build
  script, and test script; and
- historical facts expressly recorded in the reconciliation.

The evidence divides correctly into five classes:

1. **Durable committed authority:** the blind-validation and reserve-intake
   protocols and the committed OS1 decisions/roadmap state.
2. **Historical approved requirements:** facts in the reconciliation about
   approved but lost `/tmp` designs and reviews. Their old hashes authenticate
   no current bytes.
3. **Surviving uncommitted state:** the helper draft and the later additions to
   `DECISIONS.md` and `ROADMAP.md`. These are evidence of pre-pause work, not
   final approved design authority.
4. **Open reconciliation gaps:** missing exact contracts required to translate
   the approved architecture into deterministic code and tests.
5. **New reconciliation statements:** the inventory, gap analysis, and
   nonauthorization conclusion authored after loss of the original files.

## 3. Provenance-honesty findings

### BLOCKER findings

None.

### IMPORTANT finding I-001 — implementation authority remains incomplete

The reconciliation accurately reports that complete schemas, canonical byte
forms, state transitions, validation rows, error mappings, ordered algorithms,
and test assertions were not preserved. Those are not replaceable by old
filenames, historical hashes, review verdicts, test-range names, or broad
architectural summaries. Corrective implementation from the available record
would require silent redesign. The reconciliation remains usable because it
states this limitation and denies authorization rather than hiding it.

### MINOR findings

None.

### Specific provenance checks

- No historical hash is represented as authenticating current bytes. The
  reconciliation labels every such digest historical at lines 21–25 and
  repeats the limitation prominently at lines 385–392.
- No lost design is treated as currently present. Loss and non-reconstruction
  are explicit at lines 3–7.
- No current implementation detail is promoted to approved final design. The
  helper is identified as an uncommitted, pre-final-design draft at lines
  156–162.
- Historical requirements are separately labeled category B at lines 300–305.
- Statements not fully preserved are framed as gaps, not filled with guessed
  schemas or behaviors.
- The phrase “durable repository authority” includes uncommitted repository
  files only under an explicit local definition and explicit qualification at
  lines 104–108, 137–148. This is not provenance misrepresentation.

## 4. Classification of all 28 reconciliation gaps

Classification totals:

- `CLOSED BY DURABLE AUTHORITY`: **0**
- `CLOSED BY EXPLICIT HISTORICAL APPROVED REQUIREMENT`: **8**
- `IMPLEMENTATION-CRITICAL AND STILL OPEN`: **20**
- `NON-IMPLEMENTATION-CRITICAL`: **0**
- `DUPLICATE / SUBSUMED BY ANOTHER GAP`: **0**

The absence of “closed by durable authority” does not mean the committed
protocols are irrelevant. They close broad safety/firewall policy, but none of
the 28 gaps as worded contains a complete detailed implementation contract in
those protocols.

### G-001 — IMPLEMENTATION-CRITICAL AND STILL OPEN

Complete artifact schemas and canonical byte contracts are absent. The
reconciliation itself says the closed field sets, types, key order, grammar,
bounds, and validation rules are absent at lines 205–209. Historical schema
names and selected numerical limits do not close this gap.

### G-002 — IMPLEMENTATION-CRITICAL AND STILL OPEN

The complete custody command contract is absent (reconciliation lines
210–212). Two preserved executable-review command signatures do not define the
rest of custody's commands, sequencing, outputs, exit behavior, or error
mapping.

### G-003 — IMPLEMENTATION-CRITICAL AND STILL OPEN

The historical statement at reconciliation lines 345–356 preserves the
`freeze-executable-review` interface, but not its review artifact schema,
identity calculations, deterministic-build decisions, validation failures, or
publication semantics. The complete contract remains absent as stated at lines
213–216.

### G-004 — IMPLEMENTATION-CRITICAL AND STILL OPEN

The `verify-executables` signature survives at reconciliation lines 352–356,
but its input schema, validation algorithm, output schema, and failures do not.
Lines 217–219 correctly leave it open.

### G-005 — IMPLEMENTATION-CRITICAL AND STILL OPEN

The historical record names four terminal root identities, continuity fields,
timestamps, and custody provenance, but supplies neither complete preflight nor
path-map schema or creation/validation algorithm. Reconciliation lines 220–223
correctly identify the missing decisions.

### G-006 — CLOSED BY EXPLICIT HISTORICAL APPROVED REQUIREMENT

The exact corrected enumerate interface is preserved at reconciliation lines
224–226 and again as category B at lines 336–339. This closes the interface
syntax only; G-007 and the artifact-schema gaps remain open.

### G-007 — IMPLEMENTATION-CRITICAL AND STILL OPEN

The historical record preserves a 12-stage validation order in summarized
form, reflected at reconciliation lines 336–339, but the complete parse,
comparison, identity, failure, and race-check mechanics are absent. The broad
sequence does not supply the exact algorithm required by lines 227–230.

### G-008 — IMPLEMENTATION-CRITICAL AND STILL OPEN

Selected MAP provenance fields and path meanings survive at reconciliation
lines 341–344, but the full closed schema, types, key order, canonicalization,
digest domain, publication rules, and validation rules do not. Lines 231–233
correctly leave the MAP contract open.

### G-009 — IMPLEMENTATION-CRITICAL AND STILL OPEN

Only the existence and role of a binding and its freeze-review digest survive.
No complete binding schema, construction algorithm, canonicalization,
publication, or validation contract survives. See reconciliation lines
234–236.

### G-010 — IMPLEMENTATION-CRITICAL AND STILL OPEN

The sole-external-authority role of `binding_sha256` survives at reconciliation
lines 326–329, but the complete freeze-review schema and procedure do not.
Therefore lines 237–239 remain open.

### G-011 — IMPLEMENTATION-CRITICAL AND STILL OPEN

Candidate-root continuity and evidence placement survive, but the run
authorization schema, creator, verification procedure, correlations, and
lifecycle do not. See reconciliation lines 240–241.

### G-012 — IMPLEMENTATION-CRITICAL AND STILL OPEN

The historical record preserves that copied bytes create no new use count, but
not reservation creation, exclusivity, validation, consumption transition, or
lifecycle. See reconciliation lines 242–243.

### G-013 — IMPLEMENTATION-CRITICAL AND STILL OPEN

The root-bound principle and several behavioral properties survive at
reconciliation lines 318–329, but the canonical root-identity representation,
byte/digest rules, chain checks, race ordering, and complete alternate-name
algorithm do not. See lines 244–247.

### G-014 — IMPLEMENTATION-CRITICAL AND STILL OPEN

The placement split itself is explicit at reconciliation lines 323–325, but
the gap also asks for complete placement and identity rules. Those identity
rules are not preserved. The gap is therefore not closed by the placement fact
alone.

### G-015 — CLOSED BY EXPLICIT HISTORICAL APPROVED REQUIREMENT

Reconciliation lines 318–322 expressly state that validation authenticates
only the supplied instance and performs no discovery, selection, copying,
renaming, repair, migration, or substitution. That behavioral non-agency rule
is complete at the level G-015 asks for.

### G-016 — CLOSED BY EXPLICIT HISTORICAL APPROVED REQUIREMENT

Reconciliation lines 326–329 expressly state that copying run or reservation
bytes creates no authority or lifecycle event. The supplied historical record
also expressly names no new run ID, use count, freeze-review authority, or
completion.

### G-017 — IMPLEMENTATION-CRITICAL AND STILL OPEN

The two line-class names survive, but their closed fields/key order and even
the names of all four approved rejection codes do not. See reconciliation
lines 259–261 and 359–364.

### G-018 — CLOSED BY EXPLICIT HISTORICAL APPROVED REQUIREMENT

Reconciliation lines 357–359 preserve UTF-8 JSONL, no BOM/whitespace/open
fields, zero-entry/zero-byte behavior, index correspondence, ID range, and the
three stated caps. These are exact enough to close the cardinality/indexing
gap, while schemas remain open under G-017/G-019.

### G-019 — IMPLEMENTATION-CRITICAL AND STILL OPEN

The field families survive, but the explicit privacy denylist, exact field
names/forms/types, key order, optionality, encodings, and source/runtime digest
placement do not. See reconciliation lines 266–269.

### G-020 — CLOSED BY EXPLICIT HISTORICAL APPROVED REQUIREMENT

Reconciliation lines 366–369 preserve descriptor `fgetxattr`, `ENOATTR` as
absence, the 8–4096 byte bound, retention of only bytes 0–7, `alis` rejection,
and the no-data-read-after-alias-decision rule. Error-enum mapping remains a
separate G-028 issue.

### G-021 — CLOSED BY EXPLICIT HISTORICAL APPROVED REQUIREMENT

Reconciliation lines 366–370 preserve a presence-only resource-fork query and
the prohibition on serializing resource-fork size/content. Error mapping and
field schema remain open elsewhere.

### G-022 — IMPLEMENTATION-CRITICAL AND STILL OPEN

Only the existence of RA01–RA19, two 64 KiB passes, object binding, and repeated
race checks survives. The 19 ordered steps and exact checks do not. See
reconciliation lines 276–278 and 370–372.

### G-023 — IMPLEMENTATION-CRITICAL AND STILL OPEN

The broad nonfatal/fatal categories survive at reconciliation lines 362–364,
but the exact outcome for each syscall/schema/probe/race condition and its
error/publication consequences do not. The gap expressly asks for the exact
matrix, so it remains open.

### G-024 — CLOSED BY EXPLICIT HISTORICAL APPROVED REQUIREMENT

Reconciliation lines 373–375 preserve an opaque mode-0600 temporary, complete
reread before IP01, exclusive publication, and retention/non-authority of a
failed pre-IP01 temporary. Later terminal steps and failure mappings remain
separate under G-027.

### G-025 — CLOSED BY EXPLICIT HISTORICAL APPROVED REQUIREMENT

Reconciliation lines 376–379 preserve the distinction between operational
success and later durable validity and state that byte-valid completion is the
terminal durable authority.

### G-026 — IMPLEMENTATION-CRITICAL AND STILL OPEN

The two schema identifiers and authoritative/nonauthoritative roles survive,
but neither closed schema nor complete creation rule does. See reconciliation
lines 289–291 and 378–380.

### G-027 — IMPLEMENTATION-CRITICAL AND STILL OPEN

The IP01–IP11 class names, broad operations, exact E09 source set, and IP11
event suppression survive. The precise failure mapping, artifact/event fields,
transition semantics, syscall-result handling, and complete ordered validation
contract do not. See reconciliation lines 292–294 and 379–382.

### G-028 — IMPLEMENTATION-CRITICAL AND STILL OPEN

Counts/ranges and stderr grammar survive, but the 42 validator rows, complete
V2 enum, exit mapping, and individual expected assertions for the three test
registries do not. Names and ranges cannot generate deterministic tests. See
reconciliation lines 295–298 and 382–384.

## 5. Completeness by subsystem

Totals:

- `COMPLETE`: **5**
- `INCOMPLETE`: **23**
- `CONTRADICTORY`: **0**
- `NOT REQUIRED FOR CORRECTIVE IMPLEMENTATION`: **0**

1. **Executable review and executable authorization — INCOMPLETE.** Missing
   closed review/identity/authorization schemas, digest calculations,
   validation procedure, publication rules, and error outcomes.
2. **Four-root/custody preflight and path map — INCOMPLETE.** Missing closed
   schemas, creators, canonical bytes, exact root identity representation,
   timestamp/provenance rules, correlations, and validation failures.
3. **MAP schema and canonicalization — INCOMPLETE.** Missing full fields/types,
   order, byte grammar, digest domain, size bounds, and validation/publication
   contract.
4. **Enumerate algorithm and access firewall — INCOMPLETE.** Durable protocol
   sections “Candidate-only enumeration boundary,” “Command and wrapper
   firewall,” and “Content firewall through selection review” close broad
   access policy, and the historical 12-stage order survives, but exact parsing,
   identity comparisons, race rechecks, and failure mapping do not.
5. **Binding creation and authority — INCOMPLETE.** Missing schema,
   canonicalization, creator, MAP correlation, publication, and validator.
6. **Freeze-review schema/validation — INCOMPLETE.** `binding_sha256` authority
   survives; all other closed fields, canonicalization, creation, and
   validation decisions are missing.
7. **Run authorization — INCOMPLETE.** Missing schema, issuer/creator,
   validation, correlations, timing, and lifecycle decisions.
8. **Consumption reservation and one-use semantics — INCOMPLETE.** Missing
   creation, exclusivity, state transitions, atomicity, use-count validation,
   and failure behavior.
9. **Inspect pre-candidate gate — INCOMPLETE.** Missing exact supplied inputs,
   validation order, root/evidence correlations, stop point, and error mapping
   before the first candidate access.
10. **Candidate access algorithm — INCOMPLETE.** RA01–RA19 are not preserved;
    the exact descriptor/path/stat/probe/hash/race sequence is unavailable.
11. **FinderInfo handling — COMPLETE.** Historical approved facts preserve the
    descriptor call, absence errno, length bounds, retained bytes, alias value,
    rejection, and access boundary. Schema/error mapping is assessed separately.
12. **Resource-fork presence handling — COMPLETE.** Historical approved facts
    preserve presence-only access and no size/content serialization. Schema and
    error mapping are assessed separately.
13. **Two-pass data-fork SHA-256/race detection — INCOMPLETE.** Two 64 KiB whole
    passes and repeated race checks survive, but the ordered snapshots,
    comparisons, retry policy, and per-failure outcomes do not.
14. **RECORD line schemas — INCOMPLETE.** Missing complete closed fields,
    types, exact key order, optionality, encodings, and four rejection-code
    names.
15. **RECORD canonicalization/order/bounds — INCOMPLETE.** JSONL/no-BOM/no-
    whitespace, ordering, IDs, and numerical caps survive, but exact canonical
    line grammar depends on missing line schemas and encodings.
16. **Expected rejection semantics — INCOMPLETE.** Stable nonregular/alias
    rejection is known to be nonfatal, but exact four-code mapping and line
    payloads are absent.
17. **Fatal error model / OS1_METADATA_HELPER_ERRORS_V2 — INCOMPLETE.** Missing
    complete enum membership, triggering condition for each member, exit
    mapping, precedence, and publication/event consequence.
18. **RECORD publication — INCOMPLETE.** Pre-IP01 temporary/reread/retention and
    exclusive publication survive, but integration with exact IP steps,
    failure transitions, directory/root checks, and terminal authority is
    missing.
19. **Operational event semantics — INCOMPLETE.** Nonauthoritative role and
    IP11 suppression survive; schema, creation triggers, contents, ordering,
    and failure behavior do not.
20. **Completion schema and creation — INCOMPLETE.** Identifier, sole creator,
    and authority axiom survive; closed schema, canonical bytes, exact source
    correlations, creation preconditions, and failures do not.
21. **IP01 through IP11 terminal publication sequence — INCOMPLETE.** Broad
    operation labels survive, but exact preconditions, syscall/error mapping,
    event transitions, and canonical verification details do not.
22. **E09 authority sources — COMPLETE.** The source set is expressly limited
    to MAP, binding, run authorization, and consumption reservation.
23. **candidate_root_identity continuity — COMPLETE.** The requirement is
    expressly stated across MAP, binding, freeze-review, and run authorization.
24. **Durable validator behavior — INCOMPLETE.** Read-only observation and
    non-agency are preserved, but the 42 validation rows, order, error mapping,
    and complete correlations are absent.
25. **Root-bound evidence identity rules — INCOMPLETE.** The governing
    principle and alternate-basename result survive, but the canonical root
    identity, chain/terminal/parent check algorithm, bounded bytes/digests, and
    race order do not.
26. **Path/basename non-authority rules — COMPLETE.** Historical facts
    expressly deny durable authority to final basename/exact absolute path and
    permit a supplied same-root alternate child only when all authority checks
    pass.
27. **Privacy denylist / serialization firewall — INCOMPLETE.** The durable
    intake protocol provides a strong broad denylist in “Permitted
    observations” and “Content firewall through selection review,” but the
    final RECORD's explicit field-level denylist, exact serializable forms, and
    closed-schema interaction are not preserved.
28. **Stable deterministic test registries and expected assertions —
    INCOMPLETE.** Registry sizes/ranges survive; individual test purposes,
    fixtures, inputs, expected bytes/errors/transitions, and coverage mapping do
    not.

## 6. Comparison with the surviving helper

The reconciliation's characterization is accurate.

- **Two-command V1 interface/schema:** confirmed by README lines 15–20 and
  source constants/argument parser (`OS1_METADATA_HELPER_MAP_V1`,
  `OS1_METADATA_HELPER_RECORD_V1`, and the old enumerate/inspect forms).
- **Exact stored root paths:** confirmed by MAP field `root_path` and inspect's
  reuse of that serialized path.
- **Partial/failure RECORD rows:** confirmed by `failureRecord`, the loop's
  `terminalArtifactError`, publication of accumulated lines, then throwing the
  terminal error.
- **Temporary cleanup:** confirmed by `publish` unlinking the temporary when
  `succeeded` is false.
- **No executable authorization:** confirmed; no corresponding argument,
  parser, schema, or validator exists.
- **No explicit preflight/path-map authority:** confirmed; neither input is
  accepted or parsed.
- **No binding/freeze/run/reservation validation:** confirmed; those artifacts
  and correlations are absent.
- **No terminal completion:** confirmed; no completion schema or creator exists.
- **No complete V2 error/RECORD contracts:** confirmed; source declares
  `OS1_METADATA_HELPER_ERRORS_V1` and the earlier RECORD schema, with a
  999,999-entry ceiling rather than the approved 4,096-record bound.

Additional evidence supports the reconciliation's statement that this draft
predates the final design: its README calls the old interface “frozen,” its
tests invoke only that interface, and Git status/history show the helper tree
as untracked while the last commit contains only the intake protocol decision.

No inaccurate helper characterization was found.

## 7. Sufficiency determination

The historical record is precise enough to preserve several individual
invariants: the corrected enumerate signature, placement split, validator
non-agency, copy non-creation semantics, RECORD cardinality/bounds,
FinderInfo/resource-fork rules, pre-IP01 publication properties, terminal
authority axiom, E09 sources, continuity set, and path/basename non-authority.

It is not precise enough to implement the system as a whole. At minimum, an
implementer would have to choose facts not present in any available source:

- exact closed schemas, key orders, field types/encodings, canonical bytes, and
  digest domains;
- precise custody creators, state transitions, correlations, atomicity,
  lifecycle, and validation failures;
- the full enumerate, inspect pre-gate, RA01–RA19, and IP01–IP11 algorithms;
- all rejection codes and the complete V2 error enum, precedence, exit mapping,
  and publication/event consequences;
- the 42 validator rows and every stable test's inputs and assertions; and
- exact completion and operational-event schemas and creation rules.

Reasonable implementation choices are still choices. Making them would be a
new design, even if they happened to resemble the lost approved artifacts.
Therefore the combined evidence is not complete, deterministic implementation
authority.

## 8. Required recovery path

**PATH B — TARGETED RE-SPECIFICATION REQUIRED**

The smallest closed ordered set of re-specification topics is:

1. **Canonical evidence model and error taxonomy.** Re-specify all closed
   artifact schemas, fields/types/order/encodings, canonical byte and digest
   rules, bounds, full `OS1_METADATA_HELPER_ERRORS_V2`, precedence, exit
   mapping, and privacy serialization denylist. All later algorithms depend on
   these definitions.
2. **Custody authority and lifecycle.** Re-specify executable review and
   authorization, four-root preflight/path map, binding, freeze review, run
   authorization, consumption reservation/one-use semantics, root-bound
   validation, creators, state transitions, correlations, publication, and the
   42-row validator registry. Preserve the surviving non-agency, placement,
   continuity, E09-source, copy, and path/basename rules without redesign.
3. **Enumeration and MAP production.** Re-specify the exact corrected
   enumerate validation/race order, access boundary, MAP schema use,
   canonicalization, binding handoff, publication, and failure outcomes. The
   corrected interface itself need not be redesigned.
4. **Inspect, candidate access, and RECORD production.** Re-specify the
   pre-candidate gate, RA01–RA19, descriptor/path/stat/probe/hash race checks,
   closed line schemas, four rejection codes, fatal/nonfatal matrix, exact
   metadata serialization, RECORD ordering/bounds, and publication behavior.
   Preserve the exact surviving FinderInfo, resource-fork, hash-pass, and
   numerical-bound facts.
5. **Terminal evidence publication.** Re-specify completion and operational-
   event schemas/creation, operational-success versus durable-validity
   transitions, and the complete IP01–IP11 sequence with failure/event mapping
   and final/reopen/root verification. Preserve the terminal-authority axiom
   and exact E09 source set.
6. **Deterministic conformance registry.** Re-specify the individual 42
   validator rows and the complete assertions/fixtures/expected bytes,
   errors, and transitions for `RECORD-CONTRACT-001..060`, `V6-001..077`, and
   the tests through `TERM-AUTH-189`, traced to topics 1–5.

Lost details that matter are the exact schemas, bytes, algorithms, state
transitions, failure mappings, registry rows, and assertions above. Lost line
counts, prose layout, review-log wording, and the three historical MINOR
findings' exact prose do not independently matter if the replacement authority
fully and deterministically specifies behavior and receives independent
approval.

No re-specification or implementation was performed by this review.

## 9. Severity summary

- `BLOCKER`: **0**
- `IMPORTANT`: **1**
- `MINOR`: **0**

The IMPORTANT finding is the grouped implementation-authority incompleteness
described in I-001. It requires targeted re-specification before implementation
but does not make the reconciliation materially false or unusable as the basis
for that next design step.

## 10. Final decision

The reconciliation contains no provenance misrepresentation and accurately
describes the surviving helper. It conservatively exposes, rather than hides,
the loss of implementation-critical detail.

**PATH B — TARGETED RE-SPECIFICATION REQUIRED**

**CORRECTIVE IMPLEMENTATION AUTHORIZED: NO**
