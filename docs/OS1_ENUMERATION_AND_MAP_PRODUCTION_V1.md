# OS1 Enumeration and MAP Production V1

## 1. Status, provenance, and authority boundary

**AUTHORING COMPLETE — INDEPENDENT APPROVAL REQUIRED — NOT APPROVED AUTHORITY**

This is targeted re-specification Topic 3, authored from durable repository
documents. It does not reconstruct a lost `/tmp` design. No historical command,
algorithm, test count, or remembered MAP behavior supplies a new rule here.

The 2026-09-10 authoring gate verified `HEAD`, `main`, and `origin/main` at
`cb25005545bf5f8c464e2edc3930174efb40547c`, subject
`Preserve Topic 3 authority clarification`, divergence `0/0`, and no staged
paths. All eleven authority documents below match their committed HEAD bytes.
The starting draft was 776 lines, SHA-256
`587564ea6d9659e6420c4adc207d9f68311eda36f9c44e02538cded67bbcb18b`.
The unchanged blocker review was 314 lines, SHA-256
`d7aeb9277550a20c30255e41fa62718d924706dad1e9d54152b62d40975f3639`.

U3-001 and U3-002 are closed by the approved clarification C, not by a local
reinterpretation. Sections 4, 5, 13, 16, and 18 integrate its exact authority
effect. The old five-option invocation and unconditional entries-digest
comparison are superseded and are not alternative implementation paths.

**NORMATIVE TOPIC-3 REQUIREMENT** marks decisions within Topic 3's delegated
scope. **UPSTREAM** marks incorporated closed authority. Both are requirements
of this completed design; Topic-3 requirements become approved authority only
after fresh independent approval. Unless labeled **RATIONALE** or **LATER
TOPIC**, algorithm steps, tables, prohibitions, and error mappings are normative.
Audit/status sections report author conclusions, not independent approval.

Corrective implementation, reserve access, authentic candidate access, Studio
Vision access, reference MIDI access, and provenance lock remain unauthorized.
No Topic-4 algorithm, Topic-5 terminal procedure, or Topic-6 fixture is defined.

## 2. Durable authority references

All eleven documents were read in full. References T1, A1, T2, and C mean the
following approved designs as qualified by their approving reviews:

| Reference | Repository document |
|---|---|
| Checkpoint | [OS1_TARGETED_RESPEC_RESUME_CHECKPOINT.md](OS1_TARGETED_RESPEC_RESUME_CHECKPOINT.md) |
| Reconciliation | [OS1_DESIGN_AUTHORITY_RECONCILIATION.md](OS1_DESIGN_AUTHORITY_RECONCILIATION.md) |
| Reconciliation review | [OS1_DESIGN_AUTHORITY_RECONCILIATION_REVIEW.md](OS1_DESIGN_AUTHORITY_RECONCILIATION_REVIEW.md) |
| T1 | [OS1_CANONICAL_EVIDENCE_SCHEMAS_AND_ERRORS_V1.md](OS1_CANONICAL_EVIDENCE_SCHEMAS_AND_ERRORS_V1.md) |
| T1 review | [OS1_CANONICAL_EVIDENCE_SCHEMAS_AND_ERRORS_V1_REVIEW.md](OS1_CANONICAL_EVIDENCE_SCHEMAS_AND_ERRORS_V1_REVIEW.md) |
| A1 | [OS1_TOPIC1_ERROR_TAXONOMY_CLARIFICATION_FOR_TOPIC2_V1.md](OS1_TOPIC1_ERROR_TAXONOMY_CLARIFICATION_FOR_TOPIC2_V1.md) |
| A1 review | [OS1_TOPIC1_ERROR_TAXONOMY_CLARIFICATION_FOR_TOPIC2_V1_REVIEW.md](OS1_TOPIC1_ERROR_TAXONOMY_CLARIFICATION_FOR_TOPIC2_V1_REVIEW.md) |
| T2 | [OS1_CUSTODY_AUTHORITY_LIFECYCLE_AND_VALIDATOR_REGISTRY_V1.md](OS1_CUSTODY_AUTHORITY_LIFECYCLE_AND_VALIDATOR_REGISTRY_V1.md) |
| T2 review | [OS1_CUSTODY_AUTHORITY_LIFECYCLE_AND_VALIDATOR_REGISTRY_V1_REVIEW.md](OS1_CUSTODY_AUTHORITY_LIFECYCLE_AND_VALIDATOR_REGISTRY_V1_REVIEW.md) |
| C | [OS1_TOPIC3_AUTHORITY_CLARIFICATION_U3_001_U3_002_V1.md](OS1_TOPIC3_AUTHORITY_CLARIFICATION_U3_001_U3_002_V1.md) |
| C review | [OS1_TOPIC3_AUTHORITY_CLARIFICATION_U3_001_U3_002_V1_REVIEW.md](OS1_TOPIC3_AUTHORITY_CLARIFICATION_U3_001_U3_002_V1_REVIEW.md) |

T1 controls every schema, field, bound, canonical byte, self/reference digest,
privacy rule, and public output grammar. A1 supplies the approved additive
four-error amendment: exactly 59 errors. T2 controls root identities,
custody lifecycle, 42 registry rows, timestamp acquisition, and publication.
C controls only its five added enumerate operands and their acquisition, and
the corrected V02-030 predicate/dependencies/comparison ownership. C section 7
states its narrow precedence over earlier interface and registry wording.
All remaining T1/A1/T2 authority is unchanged. The checkpoint establishes
durable approval despite prospective status wording
retained in the design documents themselves. Its original baseline paragraph
is historical checkpoint context, not a claim that current HEAD is Topic 1.

The existing [blocker review](OS1_TOPIC3_BLOCKER_REVIEW_U3_001_U3_002.md) was
read as review evidence only. It does not override C's later approved closure.
This design neither edits nor supersedes the eleven documents. Its local
requirements must receive independent review before becoming authority.

## 3. Scope and exact content boundary

**NORMATIVE TOPIC-3 REQUIREMENT:** Enumeration means collecting the complete set of
immediate directory-entry names under an authorized retained CANDIDATE_ROOT,
observing their no-follow kinds, and checking the stability of those
observations. Inclusion in this inventory is not selection for intake,
confirmation of project format, or a successful inspection outcome.

MAP production means constructing exactly one `OS1_METADATA_MAP_V2` from that
inventory and the validated upstream chain, then publishing it beneath
PRIVATE_OUTPUT_ROOT using T2 publication mechanics.

`OS1_PATH_MAP_V1` is a distinct, already-produced T2 input. Enumerate does not
produce, replace, or combine it with MAP. The phrase PATH MAP must not obscure
this distinction. No binding is produced by enumerate.

Topic 3 owns entry collection, inclusion, ordering, record-ID assignment,
ephemeral enumeration observations, local race checks, MAP construction, and
MAP producer integration with the upstream gate and publication procedure.

Topic 4 owns opening candidate files, FinderInfo/alias decisions, resource-fork
presence probes, whole-data-fork SHA-256, inspection outcomes, and RECORD.
Topic 5 owns terminal completion and operational-event publication. Topic 6
owns deterministic fixtures and assertions. T2 retains binding/freeze/run/
reservation creation and observational validation.

The Topic-3 boundary is before acquisition of any descriptor to a candidate
child, including a directory child or a regular file. Candidate data-fork
bytes cannot affect enumeration, ordering, inclusion, identifiers, or MAP.
Reading a prefix, magic value, alias payload, or zero-byte-file verification
would breach that boundary just as a complete content read would.

## 4. Canonical invocation and approved U3 integration

### 4.1 Sole command surface — U3-001 integrated

**UPSTREAM — C section 3:** The canonical command is exactly:

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

Every option occurs exactly once, in this order, with exactly one operand.
FILE and ROOT are metavariables. There are no aliases, defaults, optional
operands, positional additions, `--option=value` forms, old-form fallbacks,
configuration inputs, or environment fallbacks. Missing, duplicate, extra,
reordered, wrong-command, or wrong-arity arguments are INVALID_INVOCATION.
The original five options retain their meanings and relative order; exactly
five mandatory options have been added by C. Topic 3 adds none.

### 4.2 Explicit inputs, assignments, and path grammar

| Option | Assigned root / role | Acquisition and use |
|---|---|---|
| --executable-authorization | IMPLEMENTATION_REVIEW_ROOT | supplied OS1_EXECUTABLE_AUTHORIZATION_V1; bounded canonical evidence acquisition |
| --preflight | CUSTODY_RECORD_ROOT | supplied OS1_FOUR_ROOT_PREFLIGHT_V1; bounded canonical evidence acquisition |
| --path-map | CUSTODY_RECORD_ROOT | supplied OS1_PATH_MAP_V1; bounded canonical evidence acquisition |
| --root | CANDIDATE_ROOT | explicit root chain; inventory only after the complete gate |
| --implementation-review-root | IMPLEMENTATION_REVIEW_ROOT | explicit root chain for authorization and both raw helper files |
| --custody-record-root | CUSTODY_RECORD_ROOT | explicit root chain for preflight/path map |
| --private-output-root | PRIVATE_OUTPUT_ROOT | explicit root chain and retained MAP publication parent |
| --helper-source | IMPLEMENTATION_REVIEW_ROOT | explicit raw regular file; complete stable acquisition and SHA-256 |
| --helper-executable | IMPLEMENTATION_REVIEW_ROOT | explicit raw regular file; complete stable acquisition and SHA-256 |
| --map-output | PRIVATE_OUTPUT_ROOT | absent immediate-child final output; no input read |

Validate every ROOT and FILE as absolute filesystem bytes of length 1..4096,
with no NUL, empty component, dot or dot-dot component, and no trailing slash
except `/`. Each FILE must be exactly its assigned explicit ROOT bytes plus
`/` plus one basename; when ROOT is `/`, append the basename directly.
The basename is 1..255 bytes, without NUL or slash, and is neither `.` nor
`..`. Nested children and mismatched lexical parents fail PATH_GRAMMAR_INVALID.
Check operands and their child/root relationships in canonical option order,
before any filesystem access; parsed later root operands are already available
for earlier FILE lexical checks. No normalization, realpath, Unicode conversion,
inferred-parent acquisition, or symlink-parent traversal is permitted.

After extracting each basename, operate only relative to its retained assigned
root descriptor. Do not independently open the complete FILE pathname.
All paths are locators. Even a valid lexical child relationship grants no
root or evidence authority; complete fresh identity equality is still required.
Serialized path-map locators do not supply any root or raw-file operand.

### 4.3 Exact acquisition and measurement order

Acquire and retain roots in canonical root-option order:

1. CANDIDATE_ROOT;
2. IMPLEMENTATION_REVIEW_ROOT;
3. CUSTODY_RECORD_ROOT;
4. PRIVATE_OUTPUT_ROOT.

Use T2 section 4 including chain projections and immediate acquisition
rechecks. Stop on the first failure without acquiring a later root. After all
four acquisitions, apply pairwise terminal separation and nonnesting.
Candidate-root directory acquisition is allowed here; no child enumeration,
child stat, or child open occurs during this gate.

Process serialized authorization, then preflight, then path map. Complete
V02-005..014 for each before the next. They are bounded by T1's 1,048,576-byte
standalone-object limit including LF. Retain descriptors and exact validated
bytes. V02-015..017 then correlate the chain and compare all four fresh roots
with frozen preflight/path-map components. Authorization has no root component:
its review-root placement is authenticated through that validated chain.

Acquire helper source completely before helper executable at V02-018..021,
using T2 section 5.3: descriptor-relative no-follow open; descriptor stat;
regular-kind requirement; representable complete byte count; complete read and
SHA-256; post-acquisition stat. Retain exactly device, inode, kind, permission
mode, size, mtime, and ctime before/after; compare identity before protected
state. Atime, birthtime, and other stat fields are ignored. This procedure
requires one complete raw-file acquisition per operand, not candidate hash
passes or an A/B build comparison. Incomplete/unstable acquisition never
supplies an accepted digest. Keep both descriptors for later T2 rechecks.

At V02-028 compare measured source SHA-256 with
`authorization.helper_source_sha256`, then measured executable SHA-256 with
`authorization.helper_executable_sha256`. Both must pass. Expected digest
strings are not current measurements. V02-029 follows those comparisons.

No review object, review-identities object, build-command file, custody source,
custody executable, or Git observation is added. Correlate custody provenance
and repository values from the supplied chain. Never discover files via PATH,
argv[0], CWD, environment, process-image introspection, configuration,
compile-time paths, conventional filenames, scans, or digest searches.

### 4.4 Corrected V02-030 — U3-002 integrated

**UPSTREAM — C section 4:** V02-030 retains its ID, numeric position,
observational mode O, and PRIVATE_OUTPUT_ROOT assignment. For a supplied
canonical digest-valid MAP, compute SHA-256 over its exact T1 canonical
entries-array substring from `[` through `]`, excluding LF. Retain
`computed_entries_sha256` only in memory.

If and only if the command explicitly supplies a binding input, after that
binding has passed V02-014 compare the computed digest with that binding's
`entries_sha256`. Inequality is EVIDENCE_DIGEST_MISMATCH. With MAP alone,
compute and retain without comparison or a declared-value verification claim.
The branch follows the command signature; no output or discovered file can
supply the missing side of a comparison.

V02-030 exclusively owns this supplied binding/entries comparison.
V02-014/015/029/031 retain every other digest/correlation obligation but must
not perform this comparison early or return a competing correlation error.
Binding self-digest and complete MAP reference-digest checks remain distinct.

Enumerate has no MAP input and skips V02-030. Its completed MAP is checked at
V02-042. Section 18 gives the exact separate custody handoff applicability.
There is no new validator row, evidence object, MAP field, or public error.
The registry remains exactly 42 rows; MAP remains exactly 17 top-level fields
and each map_entry_v1 exactly three fields. Only binding serializes
entries_sha256. U3-001 and U3-002 are fully integrated closed authority seams.

## 5. Required pre-enumeration gate and registry ordering

The following order incorporates T2 and C. Section 4 supplies every operand
and its order. Local collection and availability checks are Topic-3 producer
requirements; they add no validator row.

| Ordered phase | Required work | Candidate boundary |
|---|---|---|
| V02-001..002 | exact invocation then all path grammar, before filesystem access | no access |
| V02-003..004 | acquire explicit roots in canonical argument order, retain chains, apply separation | candidate root directory metadata may be acquired here; no child enumeration |
| V02-005..014 | each serialized input completely, in argument order; authorization, preflight, path map dependency chain | no children |
| V02-015..017 | input correlations; skip 016 because no verdict/state-bearing input is supplied; complete fresh-root equality | no children |
| V02-018..021 | explicit current helper raw acquisition, regular kind, stable complete bytes | no children |
| V02-022..027 | skip review-only separation/A-B and repository-only acquisition | no observation |
| V02-028..029 | current helper equality and complete supplied chain/root/provenance correlations | no children |
| V02-030..032 | no input MAP, binding, freeze, run, or reservation; skip their predicates | no observation |
| pre-enumeration availability gate | authorized output parent and absent final, following successful applicable authority checks | no children until this gate succeeds |
| Topic-3 local collection | sections 6..11 | only immediate names and no-follow stat |
| V02-033..036 | final root/input continuity after the section-11 preconstruction candidate recheck | no child content |
| V02-037, 040 | final output-parent and absence validation; reservation rows do not apply | no child content |
| V02-041 | single T2 section 5.5 timestamp | collection complete; no MAP bytes yet |
| construction then V02-042 | complete output, ID, self-digest, and output predicates | no child content |
| T2 section 14 | ordered MAP publication | no child content |

The early output-availability gate is a normative Topic-3 local precondition,
not an early execution of numbered V02-037/040. The numbered final rows still
execute in their approved order after collection. It neither acquires time
nor evaluates a nonexistent MAP. No registry row is renumbered or added.

All overlapping predecessor IDs and root components must agree; preflight
authorization/review/commit/custody values must agree with authorization;
path-map preflight/authorization IDs and four components must agree with
preflight. Only fields present in T1 may be compared or copied. In particular,
authorization does not contain a root-identity field. Its placement is checked
against the preflight's review-root component, not an invented field.

Reading a serialized candidate-root locator is not candidate filesystem
access. Root acquisition necessarily precedes its comparison with parsed
authority under V02-003/017. This must not be described as deferring all root
access until after V02-040; it is child enumeration that the gate defers.

### 5.1 Exact input correlations and recheck subjects

Let A be authorization, F preflight, and P path map, each already canonical
and digest-valid. At V02-015 check these equalities in this order, following
the later input's T1 field order; stop at the first disagreement:

1. F.authorization_id = A.authorization_id;
   F.executable_review_id = A.review_id;
   F.repository_commit = A.repository_commit;
   F.custody_source_sha256 = A.custody_source_sha256;
   F.custody_executable_sha256 = A.custody_executable_sha256.
2. P.preflight_id = F.preflight_id;
   P.authorization_id = F.authorization_id;
   P.candidate_root_identity = F.candidate_root_identity;
   P.private_output_root_identity = F.private_output_root_identity;
   P.custody_record_root_identity = F.custody_record_root_identity;
   P.implementation_review_root_identity = F.implementation_review_root_identity.

Every root component comparison includes all six T1 fields. V02-017 compares
fresh candidate, review, custody, then private root components with their
corresponding F components in acquisition order. V02-029 reaffirms the same
supplied-chain correlations and assigned-root associations after current-helper
measurement; it adds no field or exact-locator equality. Do not compare any
P path locator with an explicit current root's pathname. Timestamps are typed
predecessor facts; there is no extra monotonicity or age threshold.

No input supplies an independent verdict or RESERVED state, so V02-016 is
skipped. No review/identities/binding/freeze/run/reservation input is supplied,
so V02-031 and V02-032 are skipped. Referenced review digests in A are checked
for T1 grammar and A's self-digest coverage; they are not compared to an
unsupplied review file. This is C's explicit-input trust boundary.

For T3's retained serialized-input observations, record device, inode, kind,
mode, size, mtime, and ctime at initial descriptor stat, retaining only those
fields for continuity. At initial evidence acquisition, descriptor-stat syscall
failure is OBJECT_STAT_FAILED; successful nonregular observation remains
EVIDENCE_NOT_REGULAR_FILE. Complete bounded acquisition and parse failures
retain their T1 evidence errors. The retained input bytes are the values used
for construction; final checks do not silently replace them with new bytes.
These local continuity observations add no serialized evidence fields.

At V02-033 stat every retained root node in root acquisition order, each chain
from `/` to terminal; failure is ROOT_CHAIN_INVALID. At V02-034 stat non-root
inputs in order A, F, P, helper source, helper executable; failure is
OBJECT_STAT_FAILED. After those observations succeed, V02-035 compares
device/inode/kind in that same roots-then-inputs order. V02-036 then compares
root modes and non-root mode/size/mtime/ctime in that order. Identity change
is OBJECT_CHANGED_OR_REPLACED; protected change is RACE_OR_MUTATION_DETECTED.
Thus an identity discontinuity is never reported as a same-object mutation.
Candidate-child observations are separate section-11 checks; no child
descriptor is added to the registry's retained-input list.

### 5.2 Output availability before enumeration and before construction

After V02-029, re-stat the retained private-root chain, apply T2 continuity,
and compare the resulting complete component with F.private_output_root_identity.
Use only that terminal descriptor as the output parent. Acquisition/stat
failure is ROOT_CHAIN_INVALID; a later identity/protected change retains
T2's object/race result; successful output-parent authority inequality is
OUTPUT_PARENT_UNAUTHORIZED. Then check only the supplied final basename via
descriptor-relative no-follow fstatat. Success means occupied regardless of
kind and fails OUTPUT_ALREADY_EXISTS; ENOENT establishes absence. Any other
stat failure is OBJECT_STAT_FAILED for this non-root final-name observation,
not a parent-stat failure. Do not open the occupant or search another name.

The early gate grants only permission for the bounded local inventory.
V02-033..036 and final V02-037/040 remain required after inventory checks and
before time/construction. At V02-037 compare the freshly rechecked retained
private parent with frozen authority; at V02-040 repeat the exact absence
observation above. Final availability is not a reservation: exclusive rename
is still required, and a collision returned by rename is FINAL_RENAME_FAILED.

## 6. Root acquisition and traversal model

**UPSTREAM:** Apply T2 sections 4.1..4.3 exactly: absolute filesystem bytes,
4096-byte bound, no empty/dot/dot-dot components, component-by-component
directory-only no-follow opens from `/`, descriptor stat and chain projection,
and all chain descriptors retained through final verification. Initial root
open/stat/kind failures are `ROOT_CHAIN_INVALID`; successful frozen-identity
inequality is `ROOT_AUTHORITY_MISMATCH`. Separation uses T2 terminal identity
and nonnesting checks, not textual path-prefix tests.

Root protected fields remain device, inode, DIRECTORY kind, and mode. This
design does not add timestamps or names to `root_identity_v1` or its projection.
Local directory-inventory observations below are separate ephemeral state.

The unchanged T2 chain projection is exactly ASCII, with LF after every line:

```text
OS1_ROOT_CHAIN_PROJECTION_V1\n
role=<ROLE>\n
count=<canonical decimal node count>\n
node=<index>,device=<u64s>,inode=<u64s>,kind=DIRECTORY,mode=<octal4>\n
```

Angle-bracket tokens denote T1 canonical values, not literal brackets.
Repeat the node line for every retained node from `/` (index 0) through the
terminal root. Hash those exact bytes for chain_sha256. The role participates
in the digest; pathname components do not. The terminal stat supplies device,
inode, DIRECTORY kind, and four-octal-digit mode. Re-stat each acquired chain
immediately in root-to-terminal order before acquiring the next root.
Root `/` is lexically valid but still subject to four-root separation; lexical
acceptance never exempts a terminal from T2's nonnesting check.

**NORMATIVE TOPIC-3 REQUIREMENT:** No recursion. Read only immediate directory entries
from the retained candidate directory. Never traverse a returned child, even
if its stat kind is DIRECTORY. Symlink targets are never read or followed.
Root chain acquisition may cross devices because T2 imposes no same-device
restriction. An immediate mounted directory is represented as DIRECTORY and
never entered. A stable different-device child is not itself a failure.

For each name pass, duplicate the retained terminal directory descriptor and
give the duplicate to fdopendir. Reset that stream to its beginning with
rewinddir before its first readdir. Execute passes sequentially; no other
directory stream may use the shared directory offset concurrently. Before
every readdir set errno to zero: a null result with zero errno is end-of-stream;
a null result with nonzero errno is ENUMERATION_FAILED. Copy each returned
name's exact bytes before the next call. Close the stream after the pass and
before comparing its inventory; a close failure is ENUMERATION_FAILED.
Duplicate/stream setup failure is ENUMERATION_FAILED. The original descriptor
remains retained for root identity and child stat operations. No
high-level URL, display-name, Finder, recursive walker, or metadata enrichment
API may substitute for raw directory-entry names and descriptor-relative stat.
The complete command-specific gate is sections 4 and 5.

## 7. Entry eligibility and raw-name collection

**NORMATIVE TOPIC-3 REQUIREMENT:** Include every immediate entry except exact byte names
`.` and `..`. Do not select by extension, case, hidden state, size, project
format, inferred family, Finder metadata, or physical-object uniqueness.

| Observed object | MAP treatment |
|---|---|
| regular file, including zero bytes | include REGULAR_FILE; no content verification |
| directory, including mount point | include DIRECTORY; no traversal |
| symbolic link | include SYMLINK; no readlink or target access |
| FIFO or socket | include FIFO or SOCKET; never open/connect |
| character/block device | include CHARACTER_DEVICE or BLOCK_DEVICE; never open |
| another no-follow kind | include OTHER; never inspect |
| Finder alias stored as regular file | include REGULAR_FILE; alias detection belongs to Topic 4 |
| hard link | include each distinct name independently |
| dot-prefixed/hidden name | include; do not query hidden flags |
| invalid UTF-8 or arbitrary non-NUL/non-slash bytes | include exact raw name bytes within T1 basename bounds |
| changing/disappearing entry | fatal under section 11; no omission or partial MAP |

Classify only the successful no-follow stat's file-type bits (`st_mode &
S_IFMT`): S_IFREG -> REGULAR_FILE, S_IFDIR -> DIRECTORY, S_IFLNK -> SYMLINK,
S_IFIFO -> FIFO, S_IFSOCK -> SOCKET, S_IFCHR -> CHARACTER_DEVICE, S_IFBLK ->
BLOCK_DEVICE; every other bit pattern -> OTHER. Permission mode means only
`st_mode & 07777`, separate from kind. Do not classify from d_type, extension,
size, or content. A successful unfamiliar kind is represented as OTHER;
inability to stat an object is fatal and never fabricated as OTHER.

Collect raw names before entry-stat operations. Ignore directory-record inode
and kind hints: they are not binding observations. Each non-dot name must fit
T1's decoded basename bound of 1..255 bytes. Malformed directory records or
unrepresentable names fail `ENUMERATION_FAILED`; never truncate, replace
invalid UTF-8, or emit a partial inventory. Seeing a 4097th non-dot entry fails
`ENTRY_LIMIT_EXCEEDED`, without dropping entries or choosing a subset.

A stable filesystem namespace cannot contain two equal immediate-child byte
names. If an iteration returns a duplicate byte name, fail
`ENUMERATION_FAILED`; do not deduplicate a potentially unstable observation.
On each returned record, validate its name, reject duplicates, then check the
count bound. An iteration syscall failure precedes any subsequent record work.
All record-acquisition defects use the same enumeration error, so directory
order cannot choose between differently named per-entry I/O errors at this
phase. Later entry observations run in sorted order.

## 8. Deterministic order and record IDs

**NORMATIVE TOPIC-3 REQUIREMENT:** Sort the raw decoded basename byte sequences by
unsigned lexicographic order. Compare corresponding bytes numerically in
0..255; at the first difference the smaller byte sorts first. If one sequence
is an exact prefix, the shorter sequence sorts first. This is the total order.

Do not compare base64url encodings, decoded Unicode strings, display names,
case-folded strings, or locale collation keys. Perform no Unicode normalization
or case conversion. Invalid UTF-8 has exactly the same byte comparison rule.
Different raw names remain different regardless of filesystem display rules.

**UPSTREAM:** Entry at zero-based index i receives `record_id` equal to `R-`
plus decimal i+1 left-padded with ASCII zeroes to exactly six digits. Start at
`R-000001`; the last permitted value is `R-004096`. No gaps, reuse, truncation,
wraparound, or object-based IDs. Zero entries yield `entry_count=0` and `[]`.
An empty MAP is a complete nonempty canonical JSON object, not a zero-byte
file. T1's zero-byte rule belongs to a later empty RECORD only.

## 9. Path privacy and metadata allowlist

**UPSTREAM:** MAP serializes exact explicit `--root` bytes only as
`candidate_root_path_b64u`, after root identity equality. T1 expressly permits
this private absolute-root locator. Claiming that MAP contains no absolute
path-derived information would contradict T1. No raw absolute path string,
joined candidate path, output path, or additional relative path is serialized.

Entry basenames appear only as unpadded canonical base64url of exact bytes.
There are no JSON escapes in these ASCII fields. Decode then re-encode must
reproduce the exact value. Base64url is encoding, not encryption or redaction;
MAP is private evidence beneath PRIVATE_OUTPUT_ROOT, not repository content.

Path bytes participate in object integrity but never independently authorize
the root or object. Operational lookup uses retained root descriptors and raw
immediate-child bytes. Durable MAP authority is its complete bytes, approved
correlations, root identities, and later binding/freeze chain.

**NORMATIVE TOPIC-3 REQUIREMENT:** A no-follow `fstatat` observation of each sorted name
may retain only device, inode, kind, permission mode, size, modification time,
and metadata-change time for local identity/mutation comparisons. The exact
seconds/nanoseconds components are compared without formatted timestamps.
Only kind and encoded basename reach `map_entry_v1`; record ID is derived.
The additional stat fields are ephemeral and are discarded after the command.

Directory inventory stability additionally retains terminal candidate-directory
mtime and ctime in memory. They do not enter the T2 root identity or digest.
Access time is ignored so directory reads do not create a self-induced race.
Birth time, owner/group, link count, flags, ACLs, allocation/block data, and
all other returned stat fields are ignored, neither interpreted nor retained.

No size or timestamp from a candidate entry is serialized into MAP. RECORD
metadata belongs to Topic 4. No public diagnostic may contain a name, path,
kind-specific detail, OS message, dynamic ID, or hash.

## 10. Finder/xattr/resource/data firewall

**NORMATIVE TOPIC-3 REQUIREMENT:** Enumerate must not list xattr names, query xattr
counts, read FinderInfo, detect resource-fork presence, query resource-fork
size, read resource-fork content, or read arbitrary attributes. This includes
presence-only and size-only calls: none is necessary for MAP fields.

Candidate child opens are forbidden, including metadata-only opens, symlink
opens, directory-child opens, resource-fork pseudo-path opens, and opens used
only to confirm zero size. No candidate child descriptor may exist.
No read, pread, mmap, seek/read sequence, hash, copy, clone, preview, decoder,
Quick Look, Spotlight, Finder, or external helper may obtain candidate content.
No symlink target text is read.

Permitted candidate-domain operations are root-chain directory acquisition,
terminal directory enumeration, retained-root stat, and no-follow immediate
child `fstatat(..., AT_SYMLINK_NOFOLLOW)` for the allowlisted observations.
Directory entries are namespace observations, not file-data reads.

Raw source/executable reads belong exclusively to separately authorized review
root operands. Evidence reads belong exclusively to supplied authority files.
Output reads belong exclusively to the current publication's temp/final.
Root separation must precede child enumeration. It proves directory-domain
separation, not absence of hard links across domains. No broader object-alias
proof is claimed; raw inputs require the upstream authorized-byte checks.

Later review can prove the firewall from the absence of any candidate-child
open path and from syscall target confinement. A runtime claim to have obeyed
the firewall is not serialized as an invented MAP field. Topic 4 alone owns
the candidate data-fork SHA-256 procedure; this design does not define it.

## 11. Local identity, race, and mutation decisions

**NORMATIVE TOPIC-3 REQUIREMENT:** The first successful no-follow stat of each sorted
entry binds an ephemeral tuple (device, inode, kind) and protected state
(mode, size, mtime, ctime). Do not compare it with directory-record hints.
No first descriptor/path binding exists because no child descriptor is opened.
Accordingly `OBJECT_IDENTITY_MISMATCH` is not used for candidate entries here.

Immediately before raw-name collection, descriptor-stat the retained candidate
directory and capture its local mtime/ctime observation in section 9. Compare
device/inode/kind and mode with its acquired root snapshot using T2's later
identity/protected-state rules before accepting that local baseline.
After collecting and sorting names,
stat all names in order. Before construction and again in T2 section 14 step 6,
perform a fresh complete raw-name pass, apply the same validation/count/order
rules, and require byte-for-byte equality with the original sorted names.
Then stat every original name again in order and compare with its first stat.
Finally descriptor-stat the retained candidate directory, check identity and
mode against the acquired root snapshot, then compare its local mtime/ctime
with the initial values. All entry stat calls use the retained candidate-root
descriptor, exact raw basename bytes, and AT_SYMLINK_NOFOLLOW.
Do not retry or restart enumeration to obtain a passing snapshot.

| Observation/failure | Sole local result and precedence |
|---|---|
| stream setup/iteration failure, malformed or duplicate returned name | ENUMERATION_FAILED at that collection operation |
| more than 4096 names in any pass | ENTRY_LIMIT_EXCEEDED at collection; no subsequent stat |
| complete re-collected name set differs | RACE_OR_MUTATION_DETECTED before entry restats |
| entry first stat or later stat syscall fails, including ENOENT | OBJECT_STAT_FAILED; do not infer an identity from errno |
| later successful stat differs in device/inode/kind | OBJECT_CHANGED_OR_REPLACED before protected-state comparison |
| identity equal but mode/size/mtime/ctime differs | RACE_OR_MUTATION_DETECTED |
| root-chain initial or prepublication restat fails | ROOT_CHAIN_INVALID under T2, not OBJECT_STAT_FAILED |
| stat of the retained candidate root for local directory mtime/ctime fails | ROOT_CHAIN_INVALID; the target is still a root observation |
| root successful initial identity differs from frozen authority | ROOT_AUTHORITY_MISMATCH |
| later root identity discontinuity | OBJECT_CHANGED_OR_REPLACED under T2 |
| root identity stable but protected mode differs | RACE_OR_MUTATION_DETECTED under T2 |
| local directory mtime/ctime differs after preceding checks pass | RACE_OR_MUTATION_DETECTED; not a changed root projection |

T1 mandatory partition rule 6 and its final approving review explicitly assign
candidate stat syscall failures to OBJECT_STAT_FAILED. This controls the
broader descriptive ENUMERATION_FAILED table phrase "entry stat failed";
this design does not allocate a stat syscall failure to both members.

A later missing name found by complete namespace comparison is a changed
inventory; a missing name encountered by an attempted stat is a stat failure.
These are different ordered observations, not discretionary error choices.
Within each entry, stat failure precedes identity comparison, which precedes
protected-state comparison. Entries are checked in section 8 order.

The final candidate namespace check occurs immediately before the remaining
pre-rename root/input/output checks. No content check is inserted after rename
into T2's fixed final-file/root/reopen sequence. Root final checks retain their
specialized FINAL_ROOT_VERIFICATION_FAILED mapping.

**RATIONALE / LIMIT OF PROOF:** This is observation-based mutation detection under T2's honest,
access-controlled threat model. It is not an atomic filesystem snapshot or a
claim to detect an ABA change restored between observations. Mutations after
the last candidate observation, including during rename, cannot be proved
absent by MAP. MAP records an enumerated inventory; it does not lease or freeze
candidate content. No lock service, filesystem freeze, or hostile-kernel
protection is introduced. Later candidate acquisition belongs to Topic 4.

## 12. Hard links and handoff identity

**NORMATIVE TOPIC-3 REQUIREMENT:** MAP identity is per directory entry within this MAP.
Two different names pointing to the same device/inode produce two entries and
two record IDs. Do not deduplicate, inspect link count, select a representative,
or hash content for equivalence. Each name has independent local rechecks.

MAP carries no candidate device/inode snapshot. It cannot prove that a later
regular file is the same historical inode as at enumeration. Later Topic 4
must establish its own authorized no-follow acquisition and race checks and
maintain T1's exact MAP-index-to-RECORD-line correspondence. This handoff
requirement supplies no Topic-4 syscall algorithm or new serialized identity.

## 13. MAP construction after V02-041

**UPSTREAM:** Finish every applicable input/root/local recheck, output-parent
check, and final-absence check before one T2 section 5.5 CLOCK_REALTIME sample.
Use its exact representable seconds/nanoseconds as `enumerated_at`. No clock
retry, normalization, or alternate clock. Failure is
`TIMESTAMP_ACQUISITION_FAILED`. Name sorting and ephemeral observations may
precede the clock; canonical MAP construction and output projections must not.

The sample is exactly clock_gettime(CLOCK_REALTIME), using POSIX epoch
1970-01-01 00:00:00 UTC, signed i64s seconds and integral nanoseconds in
0..999999999. Do not round, rescale, format civil time, or consult timezone.
Equal or backward values are accepted as observed, without chronology claims.
Clock syscall, conversion, or range failure stops before construction.

**NORMATIVE TOPIC-3 REQUIREMENT:** After that sample:

1. Assign record IDs to sorted names and encode names as T1 base64url.
2. Construct exactly T1's declared MAP fields in declared order. Copy IDs and
   helper/custody digests from the fully validated chain and current-helper
   comparisons. Copy current candidate and private-output root components
   only after their equality with preflight/path-map authority.
3. Encode only the exact explicit candidate-root operand as its informational
   path field. Do not replace it with a path-map locator or resolved path.
4. Let M_ID be the canonical object projection omitting only `map_id` and
   `object_sha256`, retaining all other MAP fields in order and final LF.
   Derive map_id as the first 32 lowercase hex digits of
   SHA-256(ASCII("OS1_TOPIC3_MAP_ID_V1\n") || M_ID). This is a new Topic-3 ID
   generation decision, not an amendment of T2's eight primary-ID rules.
   M_ID is never published and is not another evidence schema.
5. Insert map_id in its declared slot. Compute T1 self-digest over the
   canonical object excluding only object_sha256, including LF; insert it.
6. Emit the complete canonical MAP including its final LF. Compute the
   complete-object map_sha256 separately when needed for validation/handoff.
   It is not the embedded self-digest and is not an additional MAP field.
7. Derive `computed_entries_sha256` over the exact `[` through `]` array
   substring without LF and retain it only in memory. It is not an extra
   input, serialized MAP member, public output, or V02-030 execution within
   enumerate. A later bind-map independently recomputes it from its explicitly
   supplied MAP under C's corrected row; no hidden in-memory handoff is used.
8. Evaluate V02-042 against the finished object, including copied fields,
   cardinality, kinds, IDs, ordering, locators, projections and digests.

T1 limits remain 4096 entries and 8,388,608 complete MAP bytes including LF.
Given 4096 entries, 255-byte names, bounded roots and T1 fields, the maximum
constructed MAP remains below that ceiling. Entry-bound failure is detected
during collection; an impossible oversized output from otherwise conforming
typed fields is a producer invariant failure, not supplied-evidence I/O.
Zero entries still have a timestamp, map_id, self-digest, and complete envelope.

### 13.1 Closed field population in canonical order

This is a use table for T1's unchanged schema, not an extension. A, F, and P
are the already validated inputs in section 5.1. All copied fields keep their
T1 types, bounds, and byte grammar; nothing is taken from an output pathname.

| Position / field | Exact source |
|---|---|
| 1 schema | literal OS1_METADATA_MAP_V2 |
| 2 map_id | section 13 step 4 projection derivation |
| 3 preflight_id | F.preflight_id |
| 4 path_map_id | P.path_map_id |
| 5 authorization_id | A.authorization_id |
| 6 executable_review_id | A.review_id |
| 7 helper_source_sha256 | measured complete helper-source digest, already equal to A.helper_source_sha256 |
| 8 helper_executable_sha256 | measured complete helper-executable digest, already equal to A.helper_executable_sha256 |
| 9 custody_source_sha256 | F.custody_source_sha256, already equal to A's corresponding field |
| 10 custody_executable_sha256 | F.custody_executable_sha256, already equal to A's corresponding field |
| 11 candidate_root_path_b64u | canonical unpadded base64url of exact --root operand bytes |
| 12 candidate_root_identity | current fully rechecked CANDIDATE_ROOT component equal to F and P |
| 13 private_output_root_identity | current fully rechecked publication-parent component equal to F and P |
| 14 enumerated_at | the single V02-041 timestamp |
| 15 entry_count | number of original sorted names, 0..4096 |
| 16 entries | exactly that many objects: record_id, basename_b64u, kind, in that key order |
| 17 object_sha256 | T1 self-excluding canonical-object projection including LF |

The current root components use only role, terminal_device, terminal_inode,
terminal_kind, terminal_mode, chain_sha256 in T1 order. They carry no candidate
child inode, local directory timestamp, or new chain field. Equality permits
copying the corresponding validated frozen components byte-for-byte because
all current fields have been verified equal; it does not bypass measurement.

Each entry uses its sorted index's derived ID, exact original raw name encoded
as base64url, and first successful no-follow kind confirmed by all required
preconstruction rechecks. There are no optional fields, nulls, error entries,
alias markers, candidate sizes/times, or candidate content hashes in MAP.

Emit ASCII-compatible UTF-8 JSON directly in T1 declared key order, with no
BOM, escapes, whitespace, or CR, followed by exactly one LF. Encode integers,
decimal strings, timestamps, root fields, and lower-case hexadecimal digests
by T1 grammar. Array order is section 8 order and must never be resorted by a
serializer. The primary-ID projection omits exactly map_id and object_sha256;
the self-digest projection omits only object_sha256; the complete-object
digest includes all 17 fields and LF; the entries digest includes only the
bracketed array, without LF. For no entries that last domain is exactly `[]`.

V02-042 verifies the complete typed field set, each field source/equality,
primitive bounds, root roles/projections, entry_count, strictly increasing raw
names, ID/index correspondence, kind values, byte envelope, map_id derivation,
self-digest, and retained complete-object/entries digest derivations. A
violation in internally constructed output is INTERNAL_INVARIANT_FAILED;
never reclassify it as malformed supplied evidence or publish a partial object.

## 14. Output, temporary, and publication boundary

**UPSTREAM INVOCATION:** MAP uses T2 section 14 as required for Topic-3
producer integration. Its final is one explicit immediate child of the
authorized PRIVATE_OUTPUT_ROOT. No nested parent, final symlink, overwrite,
search, alternate-name selection, repair, or cleanup is allowed.

After V02-042, execute these stages in T2 section 14 order:

1. Revalidate output grammar, authorized retained parent, and final absence
   by section 5.2. No alternate root or basename is selected.
2. Derive exactly `.OS1-TEMP-` plus object_sha256. Exclusively create that
   immediate child, no-follow and mode 0600, through the retained private
   parent. Any occupant, regardless of kind, is TEMP_CREATE_FAILED and is not
   opened or altered. No random name, suffix, or retry is allowed.
   Retain the newly created descriptor and immediately descriptor-stat it to
   capture the device/inode needed by step 9. This non-root object-stat syscall
   failure is OBJECT_STAT_FAILED; it is not a creation-syscall or evidence
   failure. Do not acquire identity from a directory-entry hint or a different
   pathname. The created descriptor must support the required write and reread.
3. Write the already constructed canonical MAP bytes. Failed or short writes
   are TEMP_WRITE_FAILED; do not publish a prefix or serialize new field values.
4. File-fsync; failure is TEMP_FILE_SYNC_FAILED.
5. Rewind the retained temporary descriptor and reread completely through EOF.
   Rewind/read failure is TEMP_REREAD_FAILED; successfully reread bytes must
   equal the canonical buffer exactly and pass the complete schema, count,
   and digest predicates, or fail TEMP_CONTENT_MISMATCH.
6. Repeat section 11's full candidate namespace/object/directory checks, then
   section 5's root/input observations, identity/state comparisons, authorized
   output-parent check, and final absence. Use their precise pre-rename errors.
7. Exclusively rename the temp to the supplied final basename within the same
   retained parent; any attempted rename failure, including EEXIST, is
   FINAL_RENAME_FAILED. Never overwrite an occupant.
8. Directory-fsync the retained publication parent;
   failure is FINAL_DIRECTORY_SYNC_FAILED.
9. Verify the final basename no-follow identifies the renamed regular file
   with mode 0600, exact canonical byte count, and the created temp's device/
   inode continuity. Any failure is FINAL_FILE_VERIFICATION_FAILED.
10. Perform T2's dedicated final root-chain/terminal/parent/authorized-identity
    verification with retained chains. Any failure in this stage is
    FINAL_ROOT_VERIFICATION_FAILED, including stat or continuity failure.
11. Reopen the explicit final basename no-follow through the retained parent;
    validate exact complete bytes, schema, and digest. Any reopen/read/
    validation failure here is FINAL_REOPEN_FAILED. Only then return success.

Steps 1 and 6 are the already-required publication checks, not additional
registry rows. No clock sample, construction, candidate-child open, new field,
or post-rename candidate-inventory pass is introduced. Keep root/input and
publication descriptors through their last required check; closing a directory
stream is handled by section 6 and cannot replace an earlier failure.

The relevant T2 operation order and specialized failure mapping are unchanged.
Before rename residue is nonauthoritative. After rename a failure leaves named
observational residue; no overwrite or resumed publication is allowed. Success
requires every final stage. No terminal RECORD completion or operational event
is created by enumerate.

T2 bind-map receives the explicit successfully published MAP and full supplied
chain. It creates its own binding in a separate custody invocation. Enumerate
does not create binding, grant independent freeze approval, authorize a run,
reserve use, or begin inspect.

## 15. Ordered public errors and invocation precedence

The vocabulary remains exactly the 59 approved members of
OS1_METADATA_HELPER_ERRORS_V2. No new public error is
required by the Topic-3 decisions. U3-001/002 are closed by C and introduce
no new runtime failure domain or enum amendment.

The following sequence is fixed; sections 4 and 5 define all operand orders:

1. INVALID_INVOCATION, then PATH_GRAMMAR_INVALID, before filesystem access.
2. T2 root acquisition/separation, then each input's acquisition and T1 parse
   precedence: open, regular kind, bound, complete read, UTF-8/BOM, JSON,
   schema version, closed schema, canonical bytes, self/reference digest.
3. V02-015 input correlation, skip inapplicable verdict/state row, then
   V02-017 root equality.
4. Raw helper open/stat/kind/read, stable identity and mutation checks, then
   EXECUTABLE_IDENTITY_MISMATCH on unequal hashes, with exact mappings below.
5. Remaining V02-029 input correlations, then local early output gate.
6. Section 7 name collection, section 8 sorting, and section 11 stat/rechecks.
7. V02-033..036 root/non-root stat, identity, and protected-state checks;
   then V02-037/040 parent and final absence; then V02-041 timestamp, with
   TIMESTAMP_ACQUISITION_FAILED on syscall
   or representability failure.
8. Constructed-output invariants at V02-042; then publication below.

Within serialized evidence use the exact corresponding EVIDENCE_* members in
T1; no raw source, candidate, or producer output operation uses those errors.
OBJECT_STAT_FAILED excludes root/output-parent operations. Output-parent
acquisition failure is ROOT_CHAIN_INVALID; successfully acquired unauthorized
parent is OUTPUT_PARENT_UNAUTHORIZED. Observed final occupancy before rename
is OUTPUT_ALREADY_EXISTS regardless of occupant kind; attempted exclusive
rename failure, including collision, is FINAL_RENAME_FAILED.

| Input/local operation or result | Sole public error |
|---|---|
| malformed option/command/arity/order | INVALID_INVOCATION |
| malformed path or lexical child/root relationship | PATH_GRAMMAR_INVALID |
| required root/parent chain acquisition or stat fails | ROOT_CHAIN_INVALID |
| acquired root differs from frozen component | ROOT_AUTHORITY_MISMATCH |
| prohibited root aliasing or nesting | ROOT_SEPARATION_FAILED |
| supplied serialized input no-follow open fails | EVIDENCE_OPEN_FAILED |
| serialized input descriptor-stat syscall fails | OBJECT_STAT_FAILED |
| serialized input successfully observed nonregular | EVIDENCE_NOT_REGULAR_FILE |
| supplied serialized input exceeds T1 bound | EVIDENCE_SIZE_LIMIT_EXCEEDED |
| supplied serialized input complete read fails | EVIDENCE_READ_FAILED |
| supplied input invalid UTF-8 or BOM | EVIDENCE_UTF8_INVALID |
| supplied input invalid JSON syntax | EVIDENCE_JSON_INVALID |
| unsupported supplied schema discriminator | EVIDENCE_SCHEMA_UNSUPPORTED |
| supplied closed field/type/value/cardinality defect | EVIDENCE_SCHEMA_INVALID |
| supplied canonical key-order/lexical/envelope defect | EVIDENCE_CANONICAL_BYTES_INVALID |
| supplied self/reference digest mismatch | EVIDENCE_DIGEST_MISMATCH |
| supplied-chain ID/provenance/root component disagreement | EVIDENCE_CORRELATION_MISMATCH |
| raw helper no-follow open fails | EXECUTABLE_AUTHORITY_INPUT_OPEN_FAILED |
| raw helper descriptor-stat syscall fails | OBJECT_STAT_FAILED |
| raw helper successful stat reports nonregular kind | EXECUTABLE_AUTHORITY_INPUT_OPEN_FAILED |
| raw helper complete byte count outside 0..9007199254740991, or complete read/required rewind/reread/digest-byte acquisition fails | EXECUTABLE_AUTHORITY_INPUT_READ_FAILED |
| measured source/executable digest unequal to authorization | EXECUTABLE_IDENTITY_MISMATCH |
| later bound device/inode/kind discontinuity | OBJECT_CHANGED_OR_REPLACED |
| later same-object protected-state change | RACE_OR_MUTATION_DETECTED |
| directory stream setup/read/close, malformed name or duplicate name | ENUMERATION_FAILED |
| a name pass exceeds 4096 entries | ENTRY_LIMIT_EXCEEDED |
| complete repeated name inventory differs | RACE_OR_MUTATION_DETECTED |
| no-follow candidate entry stat fails, including ENOENT | OBJECT_STAT_FAILED |
| retained candidate-directory local stat fails | ROOT_CHAIN_INVALID |
| same candidate-directory identity/mode but local mtime/ctime differs | RACE_OR_MUTATION_DETECTED |
| successfully observed output parent lacks assigned authority | OUTPUT_PARENT_UNAUTHORIZED |
| final-name no-follow stat fails other than ENOENT | OBJECT_STAT_FAILED |
| final name successfully observed occupied before rename | OUTPUT_ALREADY_EXISTS |
| required realtime timestamp syscall/conversion/range fails | TIMESTAMP_ACQUISITION_FAILED |
| constructed output violates a completed-output invariant | INTERNAL_INVARIANT_FAILED at V02-042 |

The table is an operation map, not a replacement stage order. Section 16 and
the per-input/per-entry suborders determine which failure is reached first.
The supplied-binding entries mismatch in a later custody command is exclusively
EVIDENCE_DIGEST_MISMATCH at V02-030 under section 18; enumerate does not reach
that input predicate. EVIDENCE_UNAUTHORIZED remains in the closed vocabulary
but no enumerate input has a verdict/state predicate; do not invent one.

| Publication stage | First failing operation/result |
|---|---|
| exclusive temp creation, including deterministic-temp collision | TEMP_CREATE_FAILED |
| initial created-temp descriptor stat for retained identity | OBJECT_STAT_FAILED |
| complete write, including short write | TEMP_WRITE_FAILED |
| file fsync | TEMP_FILE_SYNC_FAILED |
| complete reread I/O | TEMP_REREAD_FAILED |
| successful reread with byte/schema/digest/count mismatch | TEMP_CONTENT_MISMATCH |
| pre-rename input/root/local checks | their T2 or section 11 exact errors; before attempted rename |
| exclusive rename | FINAL_RENAME_FAILED |
| directory fsync | FINAL_DIRECTORY_SYNC_FAILED |
| final basename/file/kind/mode/size/identity verification | FINAL_FILE_VERIFICATION_FAILED |
| dedicated final root/chain/terminal/parent verification | FINAL_ROOT_VERIFICATION_FAILED |
| final no-follow reopen and all validation within it | FINAL_REOPEN_FAILED |

Stop at the first failed operation. Never run a later check to replace the
first result, and never publish a partial MAP. Reservation, candidate-open/
read/seek, FinderInfo, resource-fork, two-pass hash, RECORD, completion, event,
and repository-acquisition errors are inapplicable to this enumerate design.

Public success is exit 0 and empty stdout/stderr. Failure has empty stdout,
exactly `OS1_METADATA_HELPER_ERROR:<ENUM>\n` on stderr, and exit 64 for
invocation/path grammar or 70 otherwise. Host/signal termination is outside
this public grammar. Ordinary I/O never becomes INTERNAL_INVARIANT_FAILED.

Invocation defects all produce INVALID_INVOCATION before path checks. Path
checks run in canonical option order. Root, serialized input, raw input,
correlation, local inventory, final recheck, timestamp, and publication stages
stop at their first failed operation. No automatic retry, rescan after failure,
alternate operand, or fallback error is permitted.

## 16. Complete invocation-to-publication algorithm

The following is the sole normative path. Failure at any step terminates the
invocation under section 15; a later step cannot replace the reported result.
No candidate-child descriptor or content/probe operation occurs at any step.

1. Validate the exact ten-option invocation (V02-001), then all operand path
   bytes and lexical child/root relationships (V02-002) in section 4 order.
2. Acquire candidate, review, custody, private roots and their immediate
   acquisition rechecks, then separation (V02-003/004). Retain every chain.
3. Acquire and completely validate authorization, preflight, path map, one
   input through V02-005..014 before the next; retain descriptors and bytes.
4. Apply section 5.1's V02-015 correlations; skip V02-016; compare complete
   fresh/frozen root components at V02-017 in root acquisition order.
5. Acquire helper source then executable through V02-018..021, including
   complete stable measurements. Skip V02-022..027. At V02-028 compare source
   then executable with authorization; complete V02-029. Skip V02-030..032.
6. Perform section 5.2's early authorized-parent/absence gate. Only after
   success establish a candidate directory stream for the inventory.
7. Capture local candidate-directory baseline, collect raw names, reject
   invalid/duplicate names and overflow, and sort using sections 6..8.
8. No-follow stat every sorted name and retain exactly section 9's ephemeral
   fields. Classify all kinds using section 7; never open a child.
9. Perform the complete section-11 preconstruction recheck: collect/sort a
   second name pass, compare to original names, restat each original name in
   order, compare identity then state per entry, and finally recheck the
   candidate-directory identity/mode/local times. No retry is permitted.
10. Execute V02-033, 034, 035, 036 in numeric order using section 5.1's exact
    subjects and comparison order. Then V02-037, skip V02-038/039, and execute
    V02-040. Retain every root and input descriptor through publication.
11. Acquire one V02-041 timestamp. Only now construct entries, all MAP fields,
    map_id, canonical self-digest and complete bytes, and in-memory derived
    digests as specified in section 13.
12. Execute V02-042 against the completed output. No output predicate occurs
    at an earlier input row and no timestamp is reacquired.
13. Execute all section-14 publication stages. Its pre-rename recheck first
    repeats the complete section-11 candidate inventory/object/directory check,
    then the retained root/input stat, identity, protected-state, authorized
    parent, and final-absence checks in section 5 order. These are publication
    rechecks required by T2 section 14 step 6, not a second numbered registry
    pass, another construction, or another timestamp.
14. Return success only after exclusive rename, directory sync, final-file,
    final-root, and final-reopen validation all succeed. Stop before binding,
    freeze review, run authorization, reservation, inspection, or RECORD.

The section-14 pre-rename candidate recheck does not reconstruct the MAP when
observations differ; it fails. Successful rechecks attest only the specified
observations. Post-rename stages retain T2's fixed scope and specialization.

## 17. Crash and interruption table

This table specifies the designed producer. This authoring task performs
none of these runtime operations.

| Interruption point | Residue/authority | Retry behavior under T2 |
|---|---|---|
| before candidate-root access | no new MAP/temp | fresh invocation, complete gate |
| during enumeration | memory only; no partial MAP | fresh gate and complete enumeration |
| after entry collection | memory only; no authority | no saved inventory or resumed suffix |
| after MAP construction | memory only; no authority | fresh gate, enumeration, timestamp and object |
| after temp creation/during write | deterministic nonauthoritative temp | never delete/open as evidence/repair; fresh new-object rule |
| after temp fsync/before rename | nonauthoritative temp, even if canonical | same no-cleanup rule; occupied newly derived temp fails |
| after final rename | named observational residue; no producer success | occupied explicit final unavailable; never overwrite/resume |
| after directory sync | named residue until all final checks succeed | same final-name block; explicitly supplied observational validation only |
| after final file/root/reopen validation | successfully published MAP, no binding or completion | existing final blocks another producer; separate explicit custody handoff |

Fresh invocation never recovers a previous timestamp, discovers residue, or
selects another output name. It revalidates, re-enumerates, captures one new
timestamp, constructs a new object, and attempts only that object's temp name.
An identical timestamp/object may reproduce a collision; progress is not
guaranteed. No cleanup exception or manual deletion authority is introduced.

## 18. Observational validation and separate custody handoff

**UPSTREAM:** T2 validators operate only on explicitly supplied evidence and
roots. They do not enumerate candidate children to validate MAP and do not
create another validator command or a forty-third registry row. The MAP output
of enumerate is not automatically passed to another command or binding agent.

### 18.1 MAP input predicates

Applicable existing predicates establish T1 canonical MAP bytes and bounds,
self-digest, complete-object digest correlation, exact entry_count, ID/index
correspondence, legal kinds and canonical private locators, assigned output
root, and complete supplied-chain correlations. A syntactically valid pathname
or digest alone does not satisfy these obligations.

**NORMATIVE TOPIC-3 REQUIREMENT:** Supplied MAP entries must have strictly
increasing decoded raw basenames according to section 8. Validate this as an
array-value constraint at V02-012; duplicate or out-of-order decoded names
fail EVIDENCE_SCHEMA_INVALID before canonical-byte/digest checks. This is
Topic 3's delegated entry-order rule, not a new schema field or registry row.
T1 key-order, whitespace, escape, and numeric-spelling defects remain
EVIDENCE_CANONICAL_BYTES_INVALID at V02-013. Do not confuse entry-array value
order with object-key lexical order.

### 18.2 Corrected row applicability and digest ownership

C's exact V02-030 command set is:

| Command | Supplied MAP / binding | Required V02-030 action |
|---|---|---|
| bind-map | MAP only | compute and retain computed_entries_sha256; no comparison |
| freeze-map-review | MAP and binding | compute and compare to supplied binding.entries_sha256 |
| authorize-run | MAP and binding | compute and compare to supplied binding.entries_sha256 |
| reserve-run | MAP and binding | compute and compare to supplied binding.entries_sha256 |
| validate-binding | MAP and binding | compute and compare to supplied binding.entries_sha256 |
| validate-freeze-review | MAP and binding | compute and compare to supplied binding.entries_sha256 |
| validate-run-reservation | MAP and binding | compute and compare to supplied binding.entries_sha256 |

All other nine T2 commands skip row 030 because they supply no MAP. Enumerate
also skips it. This table defines no future Topic-4/5 command surface.
Row 030 consumes retained V02-014-valid MAP bytes and, when supplied, a
V02-014-valid binding. It performs no filesystem read or output construction.
The digest covers the exact entries-array byte substring, brackets included,
LF excluded; empty entries hash `[]`. Only an unequal explicitly supplied
binding field produces EVIDENCE_DIGEST_MISMATCH at this row.

No MAP entries_sha256 member, self-comparison, complete-MAP-digest comparison,
future binding operand, or hidden state is allowed. V02-030 alone owns this
entries comparison; V02-014/015/029/031 retain all other predicates as qualified
by C section 4.2. The row keeps its numeric order, mode O, and private root.

### 18.3 MAP-to-binding boundary

The separate caller-supplied handoff uses the unchanged T2 signature:

```text
phoenix-os1-custody bind-map \
  --executable-authorization FILE --preflight FILE --path-map FILE \
  --map FILE --implementation-review-root ROOT \
  --custody-record-root ROOT --private-output-root ROOT --binding-output FILE
```

T2 owns execution of this command. It acquires review, custody, then private
roots, opens only the explicit chain and MAP under their assigned retained
roots, and applies all applicable input validations. It does not reacquire or
enumerate a candidate root from a serialized locator. At V02-030 it computes
and retains the entries digest without comparison: binding is an output here.
After final rechecks, output-parent/absence checks, and V02-041's timestamp,
it constructs the T1 binding with entries_sha256 equal to that retained value,
map_sha256 equal to the complete canonical MAP digest including LF and its
self-digest, and all other T1 binding fields copied/derived under T2 section 9.
V02-042 validates the completed binding, including its copied entries digest;
an impossible internally constructed mismatch is INTERNAL_INVARIANT_FAILED.
T2 publication follows. Enumerate neither performs nor resumes this handoff.

A later command supplying MAP and binding checks the binding's own canonical
bytes/self-digest, then the row-030 entries relationship at its prescribed
position. The full supplied pair/root/chain must still pass every other
applicable row. Independent freeze authority remains binding_sha256 in the
freeze review; no new pair authority is introduced.

**RATIONALE / LIMIT OF PROOF:** Validators establish byte-level conformity to
MAP ordering, not historical enumeration completeness, candidate inode
continuity since production, content nonaccess, correct historical syscalls,
or absence of every race. T2 section 9 denies treating binding as proof of
correct enumeration. Source review and Topic-6 conformance work address
producer behavior; an invented MAP attestation field cannot supply that proof.
Temporary residue never substitutes for supplied evidence. A same-root
alternate final basename can validate only when explicitly supplied and all
T2 checks pass. Observational validation creates no ID, binding, reservation,
use, completion, or other lifecycle transition.

## 19. Reconciliation gap disposition

These are design-completeness dispositions, conditional on independent
approval of this Topic-3 document, not claims that Topic 3 is already approved
or durably committed. Upstream closed authority remains unchanged.

| Gap | Topic-3 disposition | Exact closure and remaining owner |
|---|---|---|
| G-006 | CLOSED through approved C and integrated by Topic 3 | C explicitly amends the preserved five-option interface to ten mandatory options. Section 4 supplies every operand, role, grammar, root order, and current measurement; no interface decision remains. |
| G-007 | CLOSED BY TOPIC 3 | Sections 4..16 specify invocation, input acquisition/correlation, helper gate, parent/absence gate, enumeration/access firewall, local and registry race checks, construction, publication, and error order. |
| G-008 | CLOSED BY T1 plus TOPIC 3, consuming T2 and C | T1's unchanged schema/canonicalization is fully populated and validated in sections 8..18; exact root/path meanings, deterministic ID, digest domains, publication, and separate binding handoff are specified. |
| G-014 | PARTIALLY CLOSED | T2 placement/identity authority remains closed; Topic 3 closes MAP producer enforcement under PRIVATE_OUTPUT_ROOT. Topic 4 retains RECORD producer enforcement; Topic 5 retains completion/event producer enforcement under CUSTODY_RECORD_ROOT. |
| G-028 | PARTIALLY CLOSED | T1/A1 retain exactly 59 errors; T2/C retain exactly 42 registry rows. Topic 3 closes enumerate applicability, ordered mappings and corrected MAP/binding consumption. Topic 6 retains deterministic fixtures/assertions and full conformance-registry definitions; Topics 4/5 retain their own algorithm-specific integration and failure mappings. |

No relevant Topic-3 gap requires new authority clarification. G-001 remains
closed by T1; G-002..005 and G-009..013 remain closed by T2 as narrowly
qualified by C. G-015/016 non-agency and copy rules are preserved. G-017..027
inspection/RECORD/terminal portions are not claimed closed by MAP production.
In particular G-023's RECORD fatal/nonfatal matrix belongs to Topic 4:
enumerate creates no rejection RECORD line. Topic-6 tests are not authored here.

## 20. Determinism and threat-model boundaries

**NORMATIVE TOPIC-3 REQUIREMENT:** Given identical validated predecessor typed
fields, explicit candidate-root bytes, current root components, stable raw
name/kind inventory, and one identical captured timestamp, produce identical
MAP bytes, map_id, self-digest, complete-object digest, and entries digest.
No locale, timezone, Unicode normalization, case folding, base64 collation,
dictionary iteration, display name, environment, random value, or selected
clock retry may influence those values. Different supplied root locator bytes
may produce different MAP bytes despite equal root authority; T1 intentionally
includes that informational locator in object integrity.

Directory iteration order cannot affect a successful stable MAP. All entry
stat and restat comparisons run in raw-name order. Errors follow the actual
ordered observations: stream errors or a 4097th entry can terminate a name
pass before later observations exist. Under concurrent mutation or multiple
stream defects, this design does not promise the same error across different
observed syscall histories. No extra scan is performed to rank unobserved
failures. Stable inputs and the same syscall results select the same first
error; first-failure semantics remain unchanged.

**RATIONALE / LIMIT OF PROOF:** The threat model is T2's honest,
access-controlled custody. Root identities, explicit supplied bytes, repeated
observations, and digest/correlation checks detect substitution and mutation
at the defined observations. They do not authenticate against a malicious
kernel or administrator, a Byzantine filesystem, coordinated rewriting of all
authority artifacts, or ABA mutations restored between observations.
Raw helper files measure the explicitly supplied files, not the running process
image. Independent review and controlled execution remain procedural trust
requirements. Directory separation does not prove absence of cross-domain
hard links. MAP is neither a filesystem snapshot nor a content lease, and
contains no historical child inode or candidate content digest. No signature,
MAC, secret, service, registry, database, lock, or hidden authority is added.

**LATER TOPIC:** Topic 4 must establish its own authorized candidate acquisition
and inspect/RECORD rules while preserving MAP index correspondence and the
approved privacy boundary. Topic 5 owns completion, terminal validity and
operational events. Topic 6 owns deterministic fixtures, fault injections,
expected bytes/errors/transitions, and conformance mappings. No later-topic
procedure, implementation permission, or candidate-access permission follows
from this document's authoring-complete status.

## 21. Author self-audit and implementation-critical decisions

This is an author self-audit, not fresh independent approval. T1, A1, T2,
C, their approving reviews, reconciliation/review, checkpoint, and unchanged
blocker review were checked as the authority/review set stated in section 2.

| Audit / decision area | Result and controlling source |
|---|---|
| CLI syntax, operands, path grammar and root order | complete; section 4 exactly consumes C section 3; no additional inputs |
| Current-helper measurements | complete; source then executable use T2 5.3 and compare at V02-028; copied digests alone cannot pass |
| Upstream authority inputs and correlations | complete; section 5 names each equality, assigned root, skipped row and retained subject |
| Scope, directory streams and candidate kinds | complete; sections 3, 6, 7 fix immediate children, stream lifecycle, no-follow type-bit classification |
| Raw ordering, invalid UTF-8, IDs, hard links, limit, empty MAP | complete; sections 7, 8, 12; no normalization, deduplication, subset or truncation |
| MAP path representation and metadata privacy | T1 unchanged; section 9 permits only existing base64url fields and ephemeral local observations |
| Candidate access firewall | complete; section 10 prohibits child descriptors, data-fork reads, FinderInfo, xattrs, resource probes and symlink target reads |
| Race/mutation and first-error ordering | complete; sections 5, 11, 15, 16 fix observations, comparisons and limits of proof |
| MAP fields, current roots, ID and digest construction | complete; section 13 populates exactly 17 fields and three-field entries from named sources |
| Timestamp/construction/output-validation boundary | preserved; V02-041 then construction then V02-042; no future output consumed by an input row |
| MAP publication and crash behavior | complete; sections 14, 17 integrate T2 temp/write/fsync/reread/recheck/exclusive-rename/directory-sync/final checks |
| U3-001 | fully integrated, CLOSED; exact ten-option interface and current measurements |
| U3-002 | fully integrated, CLOSED; sections 4.4 and 18 preserve conditional comparison ownership, 42 rows, and MAP-only computation |
| T1/A1 error vocabulary | exactly 59 unchanged; raw/stat/root/timestamp/publication domains retain their disjoint meanings |
| Observational validator non-agency | preserved; supplied instances only; no discovered binding or historical-enumeration proof |
| Reconciliation assignments | G-006/007/008 design-closed; G-014/028 explicitly partial with named later owners |
| Threat model and determinism | preserved and bounded in section 20; no hidden trust or new candidate-content access |
| Later-topic boundary | no Topic-4/5/6 procedure or test fixture is specified; no implementation authorization |

Author findings after self-audit:

- BLOCKER: **0**.
- IMPORTANT: **0**.
- MINOR: **0**.
- Unresolved Topic-3 implementation-critical decisions: **0**.
- New authority clarification required: **NO**.

The completed design is ready for fresh independent approval review. That
review must assess the local rules as well as C integration; earlier approval
of T1, A1, T2, or C does not approve this document. Repository formatting/build
checks are authoring hygiene, not proof of an unimplemented OS1 algorithm.
No ordinary test-suite execution or authentic/reference fixture access is
part of this authoring task. Only this design document is edited; the blocker
review and durable checkpoint remain unchanged.

TOPIC 3 AUTHORING COMPLETE: YES
U3-001 FULLY INTEGRATED: YES
U3-002 FULLY INTEGRATED: YES
UNRESOLVED IMPLEMENTATION-CRITICAL DECISIONS: 0
TOPIC 3 READY FOR INDEPENDENT APPROVAL REVIEW: YES
TOPIC 4 MAY BEGIN: NO
CORRECTIVE IMPLEMENTATION AUTHORIZED: NO
