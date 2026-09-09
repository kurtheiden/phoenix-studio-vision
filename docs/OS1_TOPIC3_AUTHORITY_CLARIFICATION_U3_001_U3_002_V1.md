# OS1 Topic-3 Authority Clarification U3-001 / U3-002 V1

## 1. Status, purpose, and identity

**AUTHORING PASS ONLY — NOT YET APPROVED AUTHORITY.**

This prospective normative clarification resolves exactly the two authority
seams confirmed by the fresh independent Topic-3 blocker review. Normative
requirements below take effect only after independent review and explicit
approval. Author self-audit is not independent approval. This document does
not edit or replace Topic 1, its amendment, Topic 2, or the Topic-3 draft.

Authoring date: 2026-09-09. Before authoring, HEAD, main, and origin/main
were verified equal to `0bd2cbc63ae481271f7c2e3c0d22060d49e03aeb`, subject
`Define OS1 custody authority and lifecycle`, with divergence 0/0 and no
staged paths. The two required identity gates passed:

| Document | Lines | SHA-256 |
|---|---:|---|
| Topic-3 draft | 776 | `587564ea6d9659e6420c4adc207d9f68311eda36f9c44e02538cded67bbcb18b` |
| Fresh blocker review | 314 | `d7aeb9277550a20c30255e41fa62718d924706dad1e9d54152b62d40975f3639` |

No implementation, Topic-3 revision, Topic-4 work, reserve/candidate access,
authentic project access, reference MIDI activity, or provenance activity is
authorized by this authoring pass.

## 2. Repository authority and scope

The following documents were read in full in this conversation. Their current
identities were checked before authoring; the nine upstream working documents
equal HEAD. No lost temporary artifact or previous conversation reasoning
supplies authority. The draft and blocker review are claim/review evidence,
not normative authority for the choices made here.

| Reference | Repository document |
|---|---|
| Checkpoint | [OS1_TARGETED_RESPEC_RESUME_CHECKPOINT.md](OS1_TARGETED_RESPEC_RESUME_CHECKPOINT.md) |
| R | [OS1_DESIGN_AUTHORITY_RECONCILIATION.md](OS1_DESIGN_AUTHORITY_RECONCILIATION.md) |
| RR | [OS1_DESIGN_AUTHORITY_RECONCILIATION_REVIEW.md](OS1_DESIGN_AUTHORITY_RECONCILIATION_REVIEW.md) |
| T1 | [OS1_CANONICAL_EVIDENCE_SCHEMAS_AND_ERRORS_V1.md](OS1_CANONICAL_EVIDENCE_SCHEMAS_AND_ERRORS_V1.md) |
| T1R | [OS1_CANONICAL_EVIDENCE_SCHEMAS_AND_ERRORS_V1_REVIEW.md](OS1_CANONICAL_EVIDENCE_SCHEMAS_AND_ERRORS_V1_REVIEW.md) |
| A1 | [OS1_TOPIC1_ERROR_TAXONOMY_CLARIFICATION_FOR_TOPIC2_V1.md](OS1_TOPIC1_ERROR_TAXONOMY_CLARIFICATION_FOR_TOPIC2_V1.md) |
| A1R | [OS1_TOPIC1_ERROR_TAXONOMY_CLARIFICATION_FOR_TOPIC2_V1_REVIEW.md](OS1_TOPIC1_ERROR_TAXONOMY_CLARIFICATION_FOR_TOPIC2_V1_REVIEW.md) |
| T2 | [OS1_CUSTODY_AUTHORITY_LIFECYCLE_AND_VALIDATOR_REGISTRY_V1.md](OS1_CUSTODY_AUTHORITY_LIFECYCLE_AND_VALIDATOR_REGISTRY_V1.md) |
| T2R | [OS1_CUSTODY_AUTHORITY_LIFECYCLE_AND_VALIDATOR_REGISTRY_V1_REVIEW.md](OS1_CUSTODY_AUTHORITY_LIFECYCLE_AND_VALIDATOR_REGISTRY_V1_REVIEW.md) |
| T3 draft | [OS1_ENUMERATION_AND_MAP_PRODUCTION_V1.md](OS1_ENUMERATION_AND_MAP_PRODUCTION_V1.md) |
| BR | [OS1_TOPIC3_BLOCKER_REVIEW_U3_001_U3_002.md](OS1_TOPIC3_BLOCKER_REVIEW_U3_001_U3_002.md) |

R G-006 and its preserved category-B requirement establish the historical
five-option enumerate syntax. RR section 4 treats its syntax as closed, and
T1's reconciliation-gap discussion preserves that closure. T1's MAP helper
fields, A1 sections 5.3/8, A1R section 3.1, and T2 section 5.3 and
V02-018–021/028 nevertheless require current-helper acquisition without
supplying its operands to enumerate. BR confirms this as U3-001.

T1's MAP schema contains no entries_sha256; its binding schema does.
T2 section 15 V02-030 requires equality with an unavailable declared value
even for bind-map, which has no binding input. T2 sections 5.6 and 9 prohibit
using the future binding output there. BR confirms this as U3-002.

Only the interface/operand seam and that entries-digest predicate are amended.
Enumeration mechanics, MAP production details, candidate inspection, lifecycle
transitions, publication mechanics, terminal behavior, and test registries
remain with their existing owners.

## 3. Part A — exact enumerate interface

### 3.1 Narrow additive amendment to G-006

The preserved five-option syntax was approved historical interface authority.
It did not already contain the missing operands. This is an explicit narrow
additive amendment to that syntax, retaining all five names, meanings, and
their relative order, and adding exactly five mandatory options.

The sole canonical command surface is:

```text
phoenix-os1-metadata-helper enumerate \
  --executable-authorization FILE \
  --preflight FILE \
  --path-map FILE \
  --root ROOT \
  --implementation-review-root ROOT \
  --custody-record-root ROOT \
  --private-output-root ROOT \
  --helper-source FILE \
  --helper-executable FILE \
  --map-output FILE
```

Every option must occur exactly once in exactly this order and take exactly
one operand. No optional form, old five-option fallback, alias, default,
alternate spelling, positional operand, config, environment fallback, or
additional option is accepted. A command or option/arity/order defect is
INVALID_INVOCATION. FILE and ROOT are metavariables, not literal arguments.

The command name identifies the interface; it does not authorize PATH lookup,
argv[0] inference, or process-image discovery as a source of helper identity.
Current identity is measured exclusively from the two explicit raw files.

### 3.2 Option roles and acquisition

In the table, review means IMPLEMENTATION_REVIEW_ROOT, custody means
CUSTODY_RECORD_ROOT, private means PRIVATE_OUTPUT_ROOT, and candidate means
CANDIDATE_ROOT. Every pathname is a caller-supplied locator, never durable
authority. No option supplies an authority verdict merely by naming a path.

| Option | Semantic role / assigned root | Root or child | Acquisition/open/read behavior | Evidence class |
|---|---|---|---|---|
| --executable-authorization | executable authorization / review | immediate child | descriptor-relative no-follow evidence open; complete bounded T1 parse, canonical/self-digest validation and correlations | serialized OS1_EXECUTABLE_AUTHORIZATION_V1 input |
| --preflight | frozen four-root authority / custody | immediate child | same serialized-evidence procedure | serialized OS1_FOUR_ROOT_PREFLIGHT_V1 input |
| --path-map | preflight/root correlation / custody | immediate child | same serialized-evidence procedure; serialized locators never acquire roots | serialized OS1_PATH_MAP_V1 input |
| --root | current candidate root / candidate | root | T2 section 4 no-follow chain acquisition and identity validation; no child access during this gate | current root observation, not serialized input |
| --implementation-review-root | current raw/evidence input root / review | root | T2 section 4 chain acquisition and identity validation | current root observation |
| --custody-record-root | current preflight/path-map input root / custody | root | T2 section 4 chain acquisition and identity validation | current root observation |
| --private-output-root | current MAP publication root / private | root | T2 section 4 chain acquisition and identity validation | current root observation |
| --helper-source | current complete helper source / review | immediate child | T2 section 5.3 raw no-follow open, stat, kind, complete stable byte acquisition and SHA-256 | raw current executable-authority input; not serialized evidence |
| --helper-executable | current complete helper executable / review | immediate child | same raw acquisition/hash procedure | raw current executable-authority input; not serialized evidence |
| --map-output | prospective MAP / private | immediate child output | grammar, authorized parent, and absence checks; later Topic-3 construction/publication only | output locator; no input bytes are read from it |

T1's only serialized candidate-root pathname remains the exact --root operand
encoded as candidate_root_path_b64u after identity equality. The added root
and raw-file options create no serialized fields. Existing path-map locators
remain informational predecessor data. An alternate explicit root path may
validate only if its complete current T2 identity matches frozen authority;
serialized locator byte equality is not a new root-authority requirement.

### 3.3 Exact pathname constraints

All ROOT and FILE operands must satisfy T2 section 4.1 absolute filesystem-byte
grammar and its 4096-byte bound. A FILE operand must consist exactly of its
assigned explicit ROOT operand followed by one slash and one basename; for
ROOT `/`, append the basename directly to `/`. The basename must satisfy
T1's decoded basename grammar: 1–255 bytes, no NUL or slash, not `.` or `..`.
No nesting, normalization, realpath conversion, symlink-parent traversal, or
inferred-root acquisition is allowed. Failure of this lexical child/root
relationship is PATH_GRAMMAR_INVALID, before filesystem access.

The lexical relationship binds each operand to the supplied root descriptor;
it does not authenticate that root. After extracting that one basename,
all file operations use the retained assigned-root descriptor. Do not open
the complete FILE pathname independently or infer any root from its parent.
The explicit --implementation-review-root is required even when authorization
or raw file paths have that lexical parent. The explicit custody/private
root operands are likewise required even when their children expose a parent.

Authorization and both raw helper basenames must be immediate children of
the explicit review root. Preflight and path map must be immediate children
of the explicit custody root. MAP output must be an immediate child of the
explicit private root. Serialized inputs must be regular files; no-follow
open/kind enforcement uses T2 section 4.3 and the evidence registry. Raw files
must be regular under T2 section 5.3. Final output must be absent; an existing
symlink is occupancy, not permission to follow or replace it.

### 3.4 Canonical acquisition and input order

After invocation validation, validate all path operands and child/root
relationships in canonical option order before filesystem access.
Acquire and retain the four root chains in this exact order, matching their
root-option order in the signature:

1. CANDIDATE_ROOT from --root;
2. IMPLEMENTATION_REVIEW_ROOT from --implementation-review-root;
3. CUSTODY_RECORD_ROOT from --custody-record-root;
4. PRIVATE_OUTPUT_ROOT from --private-output-root.

Use T2 sections 4.2/4.3 and V02-003/004 unchanged, including chain descriptors,
projections, immediate acquisition rechecks, pairwise separation and nonnesting.
The first failing root stops the command; no later root is attempted. These
root observations do not enumerate or open candidate children.

Then process the three serialized inputs completely in signature order:
authorization, preflight, path map. Apply T2 V02-005–014 and T1 parse
precedence to each input before the next. Apply V02-015–017: predecessor
IDs/provenance and frozen root components
must agree, and each fresh complete root component must equal the corresponding
preflight/path-map component. Authorization has no root component; its
placement is checked against the review-root identity established by the
validated preflight/path-map chain, not an invented authorization field.

At V02-018–021 acquire helper source first, then helper executable, using
T2 section 5.3 exactly, including its before/after protected-field checks.
At V02-028 compare the measured source SHA-256 to authorization.helper_source_sha256,
then the measured executable SHA-256 to authorization.helper_executable_sha256.
Both comparisons must pass. Authorization hashes are expected values, not a
substitute for the current measurements. Retain raw/input/root descriptors
through the applicable T2 rechecks; V02-029 follows V02-028 in registry order.

No review object, review-identities object, build-command file, custody source,
custody executable, or Git repository observation is added to enumerate.
Custody provenance and repository fields are correlated from supplied evidence.
T2 review-only/repository-only rows remain inapplicable. No filename, file bytes,
or root comes from serialized locator discovery, digest search, CWD, environment,
PATH, compile-time paths, or process introspection.

Before any candidate-child enumeration, the complete applicable input/root/
current-helper authority gate and authorized MAP-parent/absence preconditions
must pass. Topic 3 retains the task of integrating enumeration, its local
rechecks and output availability with final V02-033–040, timestamp V02-041,
construction, V02-042, and publication. This paragraph neither relocates those
numbered rows nor approves the existing draft's local algorithm.

The amendment supplies all required command operands and their acquisition
order. It does not purport to finish Topic 3's downstream algorithm.

## 4. Part B — corrected V02-030

### 4.1 One conditional rule, one existing row

V02-030 retains its ID, numeric position, observational mode, and private-root
assignment. It has the following single normative meaning:

> For an explicitly supplied, canonical, digest-valid MAP, compute SHA-256 of
> its exact T1 canonical entries-array substring, from `[` through `]`, without
> LF. If and only if the command also explicitly supplies a binding input,
> compare that computed digest with that binding's entries_sha256 after the
> binding has passed V02-014. Inequality is EVIDENCE_DIGEST_MISMATCH. If no
> binding input is supplied, retain the computed digest without comparison.

The retained observation is called `computed_entries_sha256` in this
clarification. This name denotes in-memory derived data, not a new serialized
field, evidence object, authority declaration, or self-comparison.
With a supplied binding, successful comparison additionally establishes
binding-entries-digest equality for the supplied pair. Without binding, the
observation is computation only and must not be called verified against a
declared entries digest.

This explicitly replaces the impossible unconditional “equals its declared
value” predicate. It does not pretend the old wording already meant this.
The branch is determined exclusively by the fixed command signature's inputs,
not by finding a binding, inspecting an output path, or choosing to omit an
otherwise mandatory argument.

### 4.2 Exact registry replacement and dependencies

The following replaces only the V02-030 entries in T2 section 15's registry
and dependency table:

| ID | Operation/object | Prerequisite | Exact predicate/observation | Error | Mode | Root | Later dependency |
|---|---|---|---|---|---|---|---|
| V02-030 | MAP entries digest derivation and supplied-binding check | V02-014 MAP; additionally V02-014 binding if that command supplies binding | compute exact T1 entries digest; compare to supplied binding.entries_sha256 only when binding is an input; retain computed digest | EVIDENCE_DIGEST_MISMATCH only for unequal supplied binding.entries_sha256; no comparison error in MAP-only case | O | private | V02-031 input correlations; bind-map construction after V02-041 and output validation at V02-042 |

| Row | Required prior observations | Observation produced | Commands/modes | Failure enum |
|---|---|---|---|---|
| V02-030 | V02-014 MAP, plus V02-014 binding when explicitly supplied | computed_entries_sha256; also equality to supplied binding.entries_sha256 when binding supplied | exact seven commands below, P/O when MAP supplied | EVIDENCE_DIGEST_MISMATCH for supplied-binding inequality only |

The additional binding prerequisite is available before row 030 because
V02-005–014 process every serialized input before later input correlations.
V02-015/029 continue their existing earlier input-chain checks; they do not
need the row-030 observation. Row 030 requires no constructed output and no
future row. V02-031 continues to depend on V02-030 for applicable lifecycle
inputs; its existing general correlation rules do not duplicate this equality
comparison under EVIDENCE_CORRELATION_MISMATCH.

For this one relationship, V02-030 is the exclusive supplied-input comparison
owner. General digest/correlation wording in V02-014/015/029/031 must not
execute the binding.entries_sha256 versus recomputed MAP-array comparison
earlier or assign it another error. Those rows retain all other schema,
self/reference-digest and correlation obligations. In particular binding's
own self-digest is still validated at V02-014; that is a different predicate
from whether its declared entries digest matches this supplied MAP.

This is a narrow ordering qualification for the corrected row, not a new row,
schema, digest domain, or public error. The registry still contains exactly
42 rows; a row may produce a derived observation without an evidence-rejection
predicate on its MAP-only branch. T2 section 15's general description of rows
as predicates is qualified only to that extent.

### 4.3 Closed command applicability

| Command | Inputs relevant to row 030 | Required behavior |
|---|---|---|
| bind-map | MAP; no binding input | compute and retain; no declared-value comparison |
| freeze-map-review | MAP and binding | compute and compare supplied binding.entries_sha256 |
| authorize-run | MAP and binding | compute and compare supplied binding.entries_sha256 |
| reserve-run | MAP and binding | compute and compare supplied binding.entries_sha256 |
| validate-binding | MAP and binding | compute and compare supplied binding.entries_sha256 |
| validate-freeze-review | MAP and binding | compute and compare supplied binding.entries_sha256 |
| validate-run-reservation | MAP and binding | compute and compare supplied binding.entries_sha256 |

All other nine T2 commands skip V02-030 because their signatures supply no
MAP. Enumerate also skips it: its MAP is a future output, not a supplied
input. No future Topic-4/5 command surface is specified by this applicability
table. Later integration must preserve the explicit-input versus output
boundary and the corrected rule where applicable.

### 4.4 bind-map and later validation

bind-map follows T2 section 9. It validates the supplied MAP and its chain,
then computes and retains the entries digest at row 030. It cannot open,
construct, or consume its binding output there. After the remaining input
rechecks, authorized output parent, absence checks, and successful V02-041
timestamp, it constructs binding with entries_sha256 equal to the retained
computed_entries_sha256, plus all other unchanged T1 binding fields.
V02-042 validates that completed output, including this exact copied digest;
an impossible internally constructed mismatch uses its existing
INTERNAL_INVARIANT_FAILED. Publication remains after V02-042.

Later commands explicitly supplying binding validate its canonical shape and
self-digest as input, then use corrected V02-030 for its entries digest
equality with the supplied MAP. The expected value is precisely the supplied
binding.entries_sha256, not MAP.object_sha256, the complete MAP digest, an
output under construction, or a second copy of the computed digest.
Full pair/root/chain authority still requires every other applicable row;
digest equality alone is not freeze authority or proof of enumeration.

No T1 schema changes. MAP still has exactly its 17 declared top-level fields;
map_entry_v1 still has record_id, basename_b64u, kind. Only
OS1_MAP_BINDING_V1 contains a serialized entries_sha256. Its digest domain,
complete MAP/binding digest domains, self-digest projections, and forward
MAP-to-binding graph remain unchanged.

## 5. Part C — compatibility table

| Surface | Before clarification | After clarification | Classification |
|---|---|---|---|
| G-006 | preserved approved five-option syntax, treated as closed | exactly five required operands added; historical approval retained as provenance | NARROWLY AMENDED |
| enumerate command surface | five named options insufficient for current gate | one ten-option ordered form in section 3.1 | NARROWLY AMENDED |
| current-helper raw measurement | required, but operands unavailable to enumerate | explicit source/executable inputs use unchanged T2 acquisition and authorized digest comparison | CLARIFIED |
| four-root acquisition | four-role algorithm fixed; enumerate operand sources/order incomplete | explicit candidate/review/custody/private roots in exact signature order | CLARIFIED |
| Topic-1 MAP schema | 17 closed fields; no entries_sha256 | same | UNCHANGED |
| entries_sha256 ownership | serialized only in binding | same; computed observation is not a schema field | UNCHANGED |
| V02-030 meaning | unconditional comparison to unavailable declared value | compute from supplied MAP; compare only to explicit supplied binding field | NARROWLY AMENDED |
| V02-030 dependencies | V02-014 MAP only | MAP plus V02-014 binding when supplied | NARROWLY AMENDED |
| V02-030 applicability | P/O when MAP supplied | same seven T2 commands; branch determined by binding input presence | CLARIFIED |
| binding construction order | after inputs, rechecks, output availability, timestamp | same; uses retained computed digest | UNCHANGED |
| V02-041 | timestamp before construction | same | UNCHANGED |
| V02-042 | completed-output validation only after construction | same, including binding's copied entries digest | UNCHANGED |
| validator row count | 42 | 42, same IDs and numeric ordering | UNCHANGED |
| public error vocabulary count | 59 | 59 | UNCHANGED |

Canonical bytes, root identities/projections, privacy rules, all other digest
domains, stderr/exit grammar, and candidate-access boundaries are unchanged.

## 6. Part D — error compatibility

The existing 59-member OS1_METADATA_HELPER_ERRORS_V2 vocabulary is sufficient.
No sixtieth error is needed or defined. T1 semantic partitions and A1's
open/stat/kind/read boundary govern every newly explicit operand operation.

| Failure | Sole existing result |
|---|---|
| missing/extra/duplicate/reordered option or wrong arity/command | INVALID_INVOCATION |
| malformed ROOT/FILE path, oversized path/basename, nested child, or FILE not the lexical immediate child of its assigned explicit root | PATH_GRAMMAR_INVALID |
| required root-chain directory open/stat/kind/projection acquisition failure | ROOT_CHAIN_INVALID |
| acquired complete root differs from frozen authorized component | ROOT_AUTHORITY_MISMATCH |
| prohibited root aliasing/nesting | ROOT_SEPARATION_FAILED |
| supplied serialized evidence no-follow open failure | EVIDENCE_OPEN_FAILED |
| supplied serialized evidence observed nonregular | EVIDENCE_NOT_REGULAR_FILE |
| supplied serialized evidence exceeds T1 byte bound | EVIDENCE_SIZE_LIMIT_EXCEEDED |
| supplied serialized evidence complete read failure | EVIDENCE_READ_FAILED |
| supplied serialized UTF-8/BOM, JSON, schema discriminator, closed-schema, or canonical-byte defect | respectively EVIDENCE_UTF8_INVALID, EVIDENCE_JSON_INVALID, EVIDENCE_SCHEMA_UNSUPPORTED, EVIDENCE_SCHEMA_INVALID, EVIDENCE_CANONICAL_BYTES_INVALID |
| supplied serialized self/reference digest mismatch | EVIDENCE_DIGEST_MISMATCH |
| cross-object IDs/provenance/root components disagree | EVIDENCE_CORRELATION_MISMATCH |
| correlated supplied evidence lacks required authority | EVIDENCE_UNAUTHORIZED |
| current raw helper source/executable no-follow open fails | EXECUTABLE_AUTHORITY_INPUT_OPEN_FAILED |
| raw helper descriptor-stat syscall fails | OBJECT_STAT_FAILED |
| raw helper stat succeeds but kind is nonregular | EXECUTABLE_AUTHORITY_INPUT_OPEN_FAILED |
| raw helper byte bound/read/required rewind/reread/stable complete-digest acquisition fails | EXECUTABLE_AUTHORITY_INPUT_READ_FAILED |
| measured helper source or executable digest differs from authorization | EXECUTABLE_IDENTITY_MISMATCH |
| final preproduction retained non-root input descriptor-stat fails | OBJECT_STAT_FAILED |
| later bound device/inode/kind discontinuity | OBJECT_CHANGED_OR_REPLACED |
| later same-object protected state differs | RACE_OR_MUTATION_DETECTED |
| successfully acquired MAP output parent lacks assigned authorized identity | OUTPUT_PARENT_UNAUTHORIZED |
| required MAP final basename already occupied before rename | OUTPUT_ALREADY_EXISTS |
| V02-030 computed digest unequal to supplied binding.entries_sha256 | EVIDENCE_DIGEST_MISMATCH |
| V02-030 with MAP only | no evidence comparison failure; produces computed digest only |
| completed binding internally constructed with wrong entries digest | INTERNAL_INVARIANT_FAILED at V02-042 |

Root-specific failures exclude ordinary object-stat errors. Input parsing
retains T1 precedence. At raw acquisition, open precedes stat, successful-kind
check precedes byte acquisition, and completed measurement precedes comparison;
later replacement/mutation retains T2's existing distinct results. The input
and root orders in section 3.4 select the first failing observation.

V02-030 operates on canonical bytes already acquired at V02-014; it performs
no filesystem read, allocation of a new authority object, or byte discovery.
Hashing those retained bounded bytes is deterministic computation. No new
ordinary I/O failure domain exists at that row. An impossible internal
implementation state remains the existing internal-invariant domain, never
a substitute for an ordinary acquisition failure or an absent operand.

Existing timestamp, final input rechecks, and stage-specific output publication
errors are unchanged. A producer's temp/final is not a supplied evidence input.
Invocation/path failures still exit 64; other enum failures exit 70; success
exits 0. Public output remains empty on success, empty stdout on failure,
and exactly OS1_METADATA_HELPER_ERROR:<ENUM> followed by LF on stderr.

## 7. Part E — exact authority effect

Upon independent approval, this document has precedence only as follows:

1. Section 3 narrowly amends R's G-006 five-option signature and its repeated
   preserved enumerate-interface requirement. RR section 4's G-006 closure,
   RR section 8's no-redesign-needed statement, and T1's preservation of G-006
   are qualified solely by the five newly mandatory operands and exact form
   here. Their historical approval is not denied or rewritten.
2. Section 3 supplies the previously missing enumerate-specific operand and
   root-order integration for T2 sections 3–5 and V02-003/005/018–021/028/029.
   It changes none of custody's sixteen command signatures or root algorithms.
3. Section 4 replaces T2 section 15's V02-030 predicate and dependency entry,
   narrows its comparison to commands supplying binding, and names the actual
   computed observation. Its row applicability remains MAP-input based.
   Generic row descriptions and overlapping digest/correlation language are
   qualified only as section 4.2 states. V02-031 retains its dependency on
   row 030 and all other correlations. T2 section 9's recomputation and later
   binding construction now have an explicit compatible registry observation.
4. Prior broad sufficiency/forward-compatibility review conclusions must be
   read with these two specific clarified interfaces. No other approval,
   design area, schema, error meaning, lifecycle rule, or registry row is
   replaced wholesale.

All T1 schemas, byte grammar, digest domains, privacy, path nonauthority,
RECORD limits, and T1/A1 error membership remain untouched. All T2 root
identities, 42 row IDs, timestamp/construction boundary, output validation,
binding/freeze/run/reservation lifecycle, and publication rules remain except
the exact entries-comparison qualifications stated above. No signature, MAC,
hidden state, registry service, filename convention, or evidence object is added.

After this clarification is independently approved, Topic 3 may rely on the
complete ten-option operand contract and corrected MAP-to-binding entries
digest rule to finish its own authoring. This authoring pass does not authorize
that edit now and does not approve Topic 3. The Checkpoint's remaining topic
and final authority-chain review gates continue to apply.

## 8. Author self-audit

| Audit | Result |
|---|---|
| accidental T1 schema or digest-domain change | none; no field added or relocated |
| accidental 60th error | none; existing 59 retained |
| accidental 43rd validator row | none; V02-030 retained in place |
| hidden/discovered helper operand | none; two mandatory FILE operands |
| inferred root paths | none; four explicit root operands, no parent/serialized-path acquisition |
| ambiguous option/root order | none; one ten-option order and one four-root order |
| future binding consumed as input | prohibited; bind-map's row 030 is computation only |
| digest compared with itself | prohibited; comparison requires explicit binding input |
| construction before V02-041 | prohibited; raw/entries observations do not construct output |
| completed-output validation before V02-042 | prohibited; earlier validation is supplied-input validation only |
| candidate-content access | none added; gate observes root directories only |
| broader redesign | none; changes restricted to operands and entries-digest row |
| newly introduced error-order ambiguity | none; entries equality exclusively row 030; other input errors retain upstream order |

Remaining implementation-critical decisions within this clarification: **0**.
Remaining ambiguity identified by author self-audit: **none**. This is an
author claim for independent review, not a review verdict or implementation
authorization. Topic 3's remaining local design is not audited for approval.

## 9. Part F — blocker closure test against every review question

This is a separate author-side adversarial check against BR's questions,
not an independent approving review. “After” means after this proposed
clarification receives independent approval; current draft status is unchanged.

### 9.1 U3-001 A–H

| BR question | Closure test |
|---|---|
| A: exact durable enumerate surface | Before: preserved five-option form. After: section 3.1's sole ten-option form supplies every operand. |
| B: historical form normative or only evidence | It was preserved approved syntax. Section 3.1 explicitly amends it; no claim that it was never normative. |
| C: current source/executable measurement obligations | T1 MAP, A1 5.3/8, A1R 3.1 and T2 raw/current-helper obligations remain; section 3.4 requires both complete measurements and comparisons. |
| D: exact raw operands | --helper-source and --helper-executable are explicit immediate children of --implementation-review-root. |
| E: existing object/root/basename/process source | None was sufficient before. After: mandatory caller operands supply roots/files; no inference or discovery is used. |
| F: Topic 2 canonical receipt mechanism | T2 previously lacked an enumerate signature; section 3 now supplies it while reusing T2 acquisition. |
| G: complete invocation without adding/clarifying operands | Impossible before; exactly five added required operands and exact root/child rules now close the interface. |
| H: interpretation without upstream clarification | None before; this is the explicit narrow cross-topic clarification, not a silent local workaround. |

A caller can name all four roots and both raw files independently; no file is
looked up by digest, process path, or conventional name. The preserved --root
continues to mean candidate root. Output and evidence opens are confined to
their explicit assigned roots, and current hashes are compared to authorization.
No missing operand prevents the Topic-3 author from specifying its remaining
enumeration-to-publication algorithm.

### 9.2 U3-002 A–H

| BR question | Closure test |
|---|---|
| A: approved MAP fields | The exact 17 T1 fields remain: schema, map_id, preflight_id, path_map_id, authorization_id, executable_review_id, helper_source_sha256, helper_executable_sha256, custody_source_sha256, custody_executable_sha256, candidate_root_path_b64u, candidate_root_identity, private_output_root_identity, enumerated_at, entry_count, entries, object_sha256. |
| B: is entries_sha256 in MAP | No; neither top-level nor in map_entry_v1. |
| C: schemas containing entries_sha256 | Only OS1_MAP_BINDING_V1; the computed observation is not serialized. |
| D: row requirement and dependency | Section 4 replaces the unconditional comparison: MAP computation, conditional explicit-binding comparison; V02-014 MAP plus V02-014 binding when supplied. |
| E: unambiguous declared referent at execution | Binding-input branch uses exactly supplied binding.entries_sha256; MAP-only branch requires no declared referent. |
| F: MAP/binding/recompute/other interpretations | One deterministic conditional rule: binding comparison when input exists, computation only otherwise. Never a MAP field or other operand. |
| G: can binding output exist at row 030 | No. bind-map retains a computed digest, constructs after V02-041, validates completed output at V02-042. |
| H: satisfying T1/T2 without changes | The old unconditional predicate could not. This explicit row/dependency amendment now satisfies unchanged T1 and retained T2 construction/order. |

Adversarial bind-map case: there is no binding input, so row 030 produces only
computed_entries_sha256; nothing reads or validates a future object. Later
construction copies it and V02-042 checks the copy. Adversarial supplied-binding
case: a canonical/self-digest-valid binding with wrong entries_sha256 fails at
V02-030 with EVIDENCE_DIGEST_MISMATCH. These cases have explicit existing
operands, disjoint branches and no circular authority dependency.

### 9.3 Scope and consequences

BR's ownership conclusion is preserved: U3-001 required a narrow cross-topic
interface amendment, and U3-002 required T2 registry clarification compatible
with T1. Neither was locally resolvable solely inside Topic 3. This proposal
supplies those authority decisions without a new schema or public error.
It does not turn BR's NO for Topic-3 approval or implementation into YES.

The author-side closure result below means the proposed text is complete for
review. It does not mean that either amendment is already approved authority.

U3-001 CLOSED BY THIS CLARIFICATION: YES
U3-002 CLOSED BY THIS CLARIFICATION: YES
AUTHORITY CLARIFICATION COMPLETE: YES
READY FOR FRESH INDEPENDENT CLARIFICATION REVIEW: YES
TOPIC 3 MAY BE EDITED TO COMPLETION: NO
TOPIC 4 MAY BEGIN: NO
CORRECTIVE IMPLEMENTATION AUTHORIZED: NO
