# Status and provenance

This is targeted re-specification Topic 1 of 6 following
`OS1_DESIGN_AUTHORITY_RECONCILIATION.md` and its independent review. It defines
canonical serialized evidence shapes and the closed public error vocabulary.
It is a new design, not a recovered copy of any lost `/tmp` artifact.

Every normative rule is marked with one of:

- **DURABLE PRIOR AUTHORITY** — copied from committed OS1 repository authority.
- **HISTORICAL APPROVED REQUIREMENT** — explicitly preserved in the committed
  reconciliation as a fact from the lost approved design chain.
- **NEW RECONCILIATION SPECIFICATION** — chosen here to replace missing detail.

Historical SHA-256 values for lost files do not authenticate this document.
Current uncommitted helper code is implementation-state evidence only and does
not choose any rule below.

This document requires independent review. It does not authorize
implementation, reserve access, candidate intake, Studio Vision access,
reference MIDI access or creation, or provenance lock.

# Scope

Topic 1 owns only:

- the common canonical JSON grammar;
- closed serialized schemas, field order, types, bounds, and digest domains;
- the RECORD JSONL stream grammar;
- schema-level privacy and path/basename representation;
- the complete `OS1_METADATA_HELPER_ERRORS_V2` vocabulary and public line
  grammar; and
- the boundary that leaves algorithms, lifecycle, publication state machines,
  and conformance tests to Topics 2–6.

Topic 1 does not decide who creates evidence, when it is created, which
filesystem checks authorize it, how reservations transition, the enumerate or
inspect operation order, IP01–IP11 behavior, or test implementations.

# Reconciliation gaps addressed

The independent review found 20 implementation-critical open gaps. Fifteen
belong wholly or partly to Topic 1:

| Gap | Topic-1 concern | Affected surface | Topic-1 result | Deferred portion |
|---|---|---|---|---|
| G-001 | Closed schemas/canonical bytes | all evidence, including review identities | fully closed | none |
| G-002 | Public error vocabulary only | custody CLI | partial | commands, sequencing, lifecycle, mappings: Topic 2 |
| G-003 | Review artifact schema | executable review | partial | creator/validation/publication: Topic 2 |
| G-004 | Authorization artifact schema | executable authorization | partial | verification/lifecycle: Topic 2 |
| G-005 | Preflight/path-map schemas | four roots/path map | partial | creation/identity algorithm: Topic 2 |
| G-007 | No Topic-1 schema/error decision | enumerate algorithm | outside Topic 1 | validation/access/race order: Topic 3 |
| G-008 | MAP schema/canonicalization | MAP | partial | production/validation/publication: Topic 3 |
| G-009 | Binding schema/canonicalization | binding | partial | creation/authority/validation: Topic 2 |
| G-010 | Freeze-review schema | freeze review | partial | procedure/validation: Topic 2 |
| G-011 | Run-authorization schema | run authorization | partial | issue/verify/lifecycle: Topic 2 |
| G-012 | Reservation schema | consumption reservation | partial | exclusivity/one-use lifecycle: Topic 2 |
| G-013 | Root-identity component and canonical digest slot | root-bound evidence | partial | chain projection/check/race algorithm: Topic 2 |
| G-014 | Root-role placement and identity fields | all authority evidence | partial | placement enforcement and identity validation: Topics 2–5 |
| G-017 | Closed RECORD line schemas | RECORD | fully closed | none |
| G-019 | RECORD fields/privacy shape | RECORD | fully closed | none |
| G-022 | No Topic-1 schema/error decision | RA01–RA19 | outside Topic 1 | complete candidate-access algorithm: Topic 4 |
| G-023 | Fatal vocabulary and no-RECORD error rule | RECORD/error surface | partial | exact failure-point matrix/precedence: Topic 4 |
| G-026 | Completion/event schemas | completion/event | partial | creation and transitions: Topic 5 |
| G-027 | E09 fields and operation-class vocabulary | event/completion | partial | IP01–IP11 ordering/failures: Topic 5 |
| G-028 | Error enum and stderr grammar | public errors | partial | validator rows/tests: Topics 2 and 6 |

G-007 and G-022 remain wholly outside Topic 1 because their open substance is
algorithmic. G-013, G-014, G-023, and G-028 are partially addressed but remain
open for their explicitly delegated algorithm, enforcement, registry, or test
portions. Previously closed G-006, G-015, G-016, G-018, G-020, G-021, G-024,
and G-025 are preserved and not reopened.

Topic-1 accounting: 3 gaps fully closed, 15 partially addressed and delegated,
and 2 of the 20 open gaps outside scope.

# Common canonical grammar

## Byte envelope

- **HISTORICAL APPROVED REQUIREMENT:** Evidence uses strict closed schemas,
  canonical UTF-8, no BOM, exact key order, and no insignificant whitespace.
- **NEW RECONCILIATION SPECIFICATION:** A standalone JSON evidence object is
  exactly one JSON object encoded as UTF-8 followed by one LF byte (`0x0a`).
  No CR is permitted. Its canonical object bytes include that final LF.
- **NEW RECONCILIATION SPECIFICATION:** A RECORD line follows the same envelope.
  The RECORD stream is the concatenation of its line bytes. The empty RECORD
  has zero bytes, not a newline.
- **NEW RECONCILIATION SPECIFICATION:** Parsers reject invalid UTF-8, a BOM,
  leading/trailing bytes other than the one required LF, CR, tabs, pretty
  printing, or any whitespace outside JSON string values.

## JSON value grammar

- **NEW RECONCILIATION SPECIFICATION:** Top-level values are objects. Objects
  and arrays may occur only where a schema below declares them. `null` and JSON
  fractional/exponent numbers are prohibited everywhere.
- **NEW RECONCILIATION SPECIFICATION:** Every integer is a JSON decimal integer
  in `0..9007199254740991`, with `0` represented only as `0`, no leading zero,
  no plus sign, and no negative sign. Signed filesystem seconds use a decimal
  string type defined below, not a JSON number.
- **NEW RECONCILIATION SPECIFICATION:** Booleans are lowercase JSON `true` or
  `false`. A schema either requires a boolean or does not permit one.
- **NEW RECONCILIATION SPECIFICATION:** All schema strings are restricted to
  printable ASCII `0x20..0x7e` and to the narrower field grammar stated below.
  Therefore canonical serialization never emits JSON escapes. A parser rejects
  `\u` escapes, backslash escapes, raw non-ASCII, and control characters even
  if a general JSON parser would accept them.
- **NEW RECONCILIATION SPECIFICATION:** Arrays preserve specified order and
  have exact cardinality or bounds. Object keys appear once, in exactly the
  declared order. Duplicate, unknown, reordered, or missing keys are fatal.
- **NEW RECONCILIATION SPECIFICATION:** Producers must serialize directly in
  declared order. Re-parsing and generic dictionary reserialization is not a
  canonicalization algorithm.

## Primitive types

| Type | Exact grammar and bound |
|---|---|
| `sha256` | 64 lowercase ASCII hex characters |
| `id32` | 32 lowercase ASCII hex characters; generation/meaning is deferred to Topic 2 |
| `schema` | exact literal named by the containing schema |
| `git_oid_sha1` | exactly 40 lowercase ASCII hex characters; the repository's current Git SHA-1 object-name format, not a SHA-256 claim |
| `enum` | one exact ASCII literal listed for the field |
| `u64s` | decimal ASCII string for `0..18446744073709551615`, no leading zero except `0` |
| `i64s` | decimal ASCII string for `-9223372036854775808..9223372036854775807`, no plus or leading zero, and no `-0` |
| `octal4` | exactly four ASCII octal digits, such as `0700` |
| `b64u` | RFC 4648 base64url alphabet `A-Z a-z 0-9 - _`, without padding; empty bytes are the empty string |
| `path_b64u` | `b64u` encoding of an absolute pathname's exact filesystem bytes, decoded length `1..4096`; informational only |
| `basename_b64u` | `b64u` encoding of one nonempty immediate-child basename, decoded length `1..255`, containing no NUL or `/`, and not `.` or `..`; informational locator only |
| `token64` | `1..64` characters from `A-Z`, `0-9`, `_`, `-` |
| `tool_version` | `1..64` characters from `A-Z a-z 0-9 . _ + -` |

- **NEW RECONCILIATION SPECIFICATION:** `git_oid_sha1` records the repository's
  actual Git object name exactly as 40 lowercase hex characters, for example
  `c8261425fb0090dfc663f96cb364edbd4c05973e`. The type name states the Git
  object format and makes no SHA-256 security claim. All three
  `repository_commit` fields use this type; no equivalent repository identity
  field uses `sha256`. Their objects remain far below the unchanged 1,048,576-
  byte standalone-object ceiling.
- **NEW RECONCILIATION SPECIFICATION:** Base64url decoding must be canonical:
  decode then unpadded re-encode must reproduce the exact string.
- **HISTORICAL APPROVED REQUIREMENT:** Serialized pathnames and basenames are
  not durable authority.
- **NEW RECONCILIATION SPECIFICATION:** `path_b64u` and `basename_b64u` are
  permitted only in fields explicitly marked informational. They locate the
  explicitly supplied filesystem instance; identity/correlation fields remain
  authoritative.

## Reusable closed components

These components are nested values, not independent evidence artifacts.

### `timestamp_v1`

Exact key order: `seconds`, `nanoseconds`.

| Key | Type | Meaning |
|---|---|---|
| `seconds` | `i64s` | whole seconds in the supplying syscall's epoch |
| `nanoseconds` | integer `0..999999999` | subsecond component |

### `root_identity_v1`

Exact key order: `role`, `terminal_device`, `terminal_inode`, `terminal_kind`,
`terminal_mode`, `chain_sha256`.

| Key | Type | Meaning/authority |
|---|---|---|
| `role` | enum `CANDIDATE_ROOT`, `PRIVATE_OUTPUT_ROOT`, `CUSTODY_RECORD_ROOT`, `IMPLEMENTATION_REVIEW_ROOT` | authoritative logical role |
| `terminal_device` | `u64s` | authoritative frozen/current correlation input |
| `terminal_inode` | `u64s` | authoritative frozen/current correlation input |
| `terminal_kind` | exact `DIRECTORY` | authoritative no-follow terminal kind |
| `terminal_mode` | `octal4` | authoritative permission snapshot |
| `chain_sha256` | `sha256` | authoritative digest of the Topic-2-defined no-follow root-chain projection |

- **NEW RECONCILIATION SPECIFICATION:** Topic 2 must define the
  `chain_sha256` projection and acquisition/validation algorithm without
  changing this component.

### `file_digest_v1`

Exact key order: `byte_count`, `sha256`.

| Key | Type | Meaning |
|---|---|---|
| `byte_count` | integer `0..9007199254740991` | complete regular-file byte count |
| `sha256` | `sha256` | SHA-256 of exactly those complete bytes |

### Common object bounds

- **NEW RECONCILIATION SPECIFICATION:** Unless narrower below, each standalone
  evidence object is at most 1,048,576 bytes including LF.
- **NEW RECONCILIATION SPECIFICATION:** MAP may be at most 8,388,608 bytes
  including LF. Each string's decoded/character bound is its primitive bound;
  no unconstrained string exists.
- **NEW RECONCILIATION SPECIFICATION:** Exceeding any field, collection, or
  object bound is malformed evidence, not truncation permission.

# Evidence-object inventory

| Object | Classification before this design | Topic-1 result |
|---|---|---|
| Executable review | schema partially preserved: interface and role only | closed schema defined |
| Executable review identities | schema missing for explicit `--review-identities FILE` input | closed schema defined |
| Executable authorization | schema partially preserved: interface and role only | closed schema defined |
| Four-root/custody preflight | schema partially preserved: four identities/correlations named | closed schema defined |
| Path map | schema partially preserved: explicit supplied evidence and path non-authority | closed schema defined |
| MAP | schema partially preserved: provenance fields, identities, IDs, and path meanings | closed schema defined |
| Binding | schema partially preserved: existence, pair authority, and root continuity | closed schema defined |
| Freeze review | schema partially preserved: role and `binding_sha256` authority | closed schema defined |
| Run authorization | schema partially preserved: existence/root continuity | closed schema defined |
| Consumption reservation | schema partially preserved: existence/copy non-creation/one-use concept | closed schema defined |
| Inspected RECORD line | schema partially preserved: class, allowlist, bounds | closed schema defined |
| Expected-rejection RECORD line | schema partially preserved: class and four-code count | closed schema defined |
| RECORD stream | schema partially preserved: JSONL/order/cardinality/bounds | closed stream grammar defined |
| Completion | schema partially preserved: identifier, sole creator, terminal authority | closed schema defined |
| Operational event | schema partially preserved: identifier and nonauthority | closed schema defined |
| Root identity | not a standalone evidence object | reusable closed component defined |
| Filesystem temp residue | not a serialized evidence object; nonauthoritative bytes in an unpublished temp | no schema added |
| Public stderr | not evidence | closed line grammar and enum defined |

Fourteen top-level serialized evidence schemas are defined: ten non-RECORD
authority artifacts, two RECORD line variants, completion, and operational
event. The RECORD stream is a fifteenth serialized form but not a standalone
JSON-object schema.

# Closed evidence schemas

## Table conventions

All keys are required and appear in table order. `Digest: yes` means the value
participates in the object's self-digest projection. `Authority` states whether
the serialized value contributes to validation authority or is informational.
Lifecycle topics decide when authoritative values are trusted; schema presence
alone grants no authority.

Every object below except RECORD lines has `object_sha256` as its final key.
That field is excluded from its own digest projection; every preceding key and
value is included. RECORD lines have no self-digest.

## Executable review — `OS1_EXECUTABLE_REVIEW_V1`

**NEW RECONCILIATION SPECIFICATION:** Exact keys:

| Key | Type/bound | Semantic source | Digest | Authority/privacy |
|---|---|---|---|---|
| `schema` | exact `OS1_EXECUTABLE_REVIEW_V1` | schema | yes | authority |
| `review_id` | `id32` | Topic 2 | yes | correlation authority |
| `repository_commit` | `git_oid_sha1` | reviewed Git commit object name | yes | authority; object identity, not a SHA-256 claim |
| `implementation_review_root` | `root_identity_v1` | reviewed root | yes | authority; no path |
| `helper_source` | `file_digest_v1` | complete source bytes | yes | authority |
| `custody_source` | `file_digest_v1` | complete source bytes | yes | authority |
| `helper_build_command` | `file_digest_v1` | complete command-file bytes | yes | authority |
| `custody_build_command` | `file_digest_v1` | complete command-file bytes | yes | authority |
| `helper_executable_sha256` | `sha256` | identical approved A/B executable bytes | yes | authority |
| `custody_executable_sha256` | `sha256` | identical approved A/B executable bytes | yes | authority |
| `reviewed_at` | `timestamp_v1` | Topic 2 review event | yes | authority |
| `object_sha256` | `sha256` | self-digest | no | authority |

The A/B equality algorithm and review decision are Topic 2. Paths are not
serialized.

## Executable review identities — `OS1_EXECUTABLE_REVIEW_IDENTITIES_V1`

**NEW RECONCILIATION SPECIFICATION:** This is the closed schema for the
historically preserved `verify-executables --review-identities FILE` input. It
is an identity projection of an executable review, not a second review verdict.
Exact keys:

| Key | Type/bound | Semantic source | Digest | Authority/privacy |
|---|---|---|---|---|
| `schema` | exact `OS1_EXECUTABLE_REVIEW_IDENTITIES_V1` | schema | yes | authority |
| `review_id` | `id32` | executable review | yes | correlation authority |
| `review_sha256` | `sha256` | complete canonical executable-review bytes | yes | authority |
| `repository_commit` | `git_oid_sha1` | executable review | yes | authority; Git object identity |
| `implementation_review_root` | `root_identity_v1` | executable review | yes | authority; no path |
| `helper_source_sha256` | `sha256` | executable review `helper_source.sha256` | yes | authority |
| `custody_source_sha256` | `sha256` | executable review `custody_source.sha256` | yes | authority |
| `helper_build_command_sha256` | `sha256` | executable review `helper_build_command.sha256` | yes | authority |
| `custody_build_command_sha256` | `sha256` | executable review `custody_build_command.sha256` | yes | authority |
| `helper_executable_sha256` | `sha256` | executable review | yes | authority |
| `custody_executable_sha256` | `sha256` | executable review | yes | authority |
| `reviewed_at` | `timestamp_v1` | executable review | yes | authority |
| `object_sha256` | `sha256` | self-digest | no | authority |

The projection's creation, review comparison, and authorization use are Topic
2. Topic 1 specifies only its bytes and correlations.

## Executable authorization — `OS1_EXECUTABLE_AUTHORIZATION_V1`

| Key | Type/bound | Semantic source | Digest | Authority/privacy |
|---|---|---|---|---|
| `schema` | exact `OS1_EXECUTABLE_AUTHORIZATION_V1` | schema | yes | authority |
| `authorization_id` | `id32` | Topic 2 | yes | correlation authority |
| `review_id` | `id32` | executable review | yes | authority |
| `review_sha256` | `sha256` | review identities; digest of complete canonical review bytes | yes | authority |
| `review_identities_sha256` | `sha256` | complete canonical executable-review-identities bytes | yes | authority |
| `repository_commit` | `git_oid_sha1` | executable review identities | yes | authority; Git object identity |
| `helper_source_sha256` | `sha256` | executable review | yes | authority |
| `helper_executable_sha256` | `sha256` | verified runtime | yes | authority |
| `custody_source_sha256` | `sha256` | executable review | yes | authority |
| `custody_executable_sha256` | `sha256` | verified runtime | yes | authority |
| `authorized_at` | `timestamp_v1` | Topic 2 | yes | authority |
| `object_sha256` | `sha256` | self-digest | no | authority |

## Four-root/custody preflight — `OS1_FOUR_ROOT_PREFLIGHT_V1`

| Key | Type/bound | Semantic source | Digest | Authority/privacy |
|---|---|---|---|---|
| `schema` | exact `OS1_FOUR_ROOT_PREFLIGHT_V1` | schema | yes | authority |
| `preflight_id` | `id32` | Topic 2 | yes | correlation authority |
| `authorization_id` | `id32` | validated executable authorization | yes | authority |
| `executable_review_id` | `id32` | authorization | yes | authority |
| `repository_commit` | `git_oid_sha1` | authorization/current repository | yes | authority; Git object identity |
| `custody_source_sha256` | `sha256` | authorization | yes | authority |
| `custody_executable_sha256` | `sha256` | current custody executable | yes | authority |
| `candidate_root_identity` | `root_identity_v1` with role `CANDIDATE_ROOT` | Topic 2 preflight | yes | authority |
| `private_output_root_identity` | component with role `PRIVATE_OUTPUT_ROOT` | Topic 2 preflight | yes | authority |
| `custody_record_root_identity` | component with role `CUSTODY_RECORD_ROOT` | Topic 2 preflight | yes | authority |
| `implementation_review_root_identity` | component with role `IMPLEMENTATION_REVIEW_ROOT` | executable review/current check | yes | authority |
| `preflighted_at` | `timestamp_v1` | Topic 2 | yes | authority |
| `object_sha256` | `sha256` | self-digest | no | authority |

## Path map — `OS1_PATH_MAP_V1`

| Key | Type/bound | Semantic source | Digest | Authority/privacy |
|---|---|---|---|---|
| `schema` | exact `OS1_PATH_MAP_V1` | schema | yes | authority |
| `path_map_id` | `id32` | Topic 2 | yes | correlation authority |
| `preflight_id` | `id32` | preflight | yes | authority |
| `authorization_id` | `id32` | preflight | yes | authority |
| `candidate_root_path_b64u` | `path_b64u` | explicit operator input | yes | informational locator only; private |
| `candidate_root_identity` | candidate `root_identity_v1` | preflight/current correlation | yes | authority |
| `private_output_root_path_b64u` | `path_b64u` | explicit operator input | yes | informational locator only; private |
| `private_output_root_identity` | private-output component | preflight/current correlation | yes | authority |
| `custody_record_root_path_b64u` | `path_b64u` | explicit operator input | yes | informational locator only; private |
| `custody_record_root_identity` | custody component | preflight/current correlation | yes | authority |
| `implementation_review_root_path_b64u` | `path_b64u` | explicit operator input | yes | informational locator only; private |
| `implementation_review_root_identity` | review-root component | preflight/current correlation | yes | authority |
| `mapped_at` | `timestamp_v1` | Topic 2 | yes | authority |
| `object_sha256` | `sha256` | self-digest | no | authority |

Paths participate in byte identity/correlation so alteration is detectable, but
they never establish filesystem authority. Topic 2 defines the explicit-input
and identity-validation rules without hidden lookup.

## Binding — `OS1_MAP_BINDING_V1`

| Key | Type/bound | Semantic source | Digest | Authority/privacy |
|---|---|---|---|---|
| `schema` | exact `OS1_MAP_BINDING_V1` | schema | yes | authority |
| `binding_id` | `id32` | Topic 2 | yes | correlation authority |
| `map_id` | `id32` | validated MAP | yes | authority |
| `map_sha256` | `sha256` | complete canonical MAP bytes | yes | authority |
| `preflight_id` | `id32` | MAP/preflight | yes | authority |
| `path_map_id` | `id32` | MAP/path map | yes | authority |
| `authorization_id` | `id32` | MAP/authorization | yes | authority |
| `candidate_root_identity` | candidate component | MAP | yes | authority |
| `private_output_root_identity` | private-output component | MAP | yes | authority |
| `entry_count` | integer `0..4096` | MAP | yes | authority |
| `entries_sha256` | `sha256` | canonical MAP `entries` array bytes defined below | yes | authority |
| `bound_at` | `timestamp_v1` | Topic 2 | yes | authority |
| `object_sha256` | `sha256` | self-digest | no | authority |

## Freeze review — `OS1_FREEZE_REVIEW_V1`

| Key | Type/bound | Semantic source | Digest | Authority/privacy |
|---|---|---|---|---|
| `schema` | exact `OS1_FREEZE_REVIEW_V1` | schema | yes | authority |
| `freeze_review_id` | `id32` | Topic 2 | yes | correlation authority |
| `binding_id` | `id32` | validated binding | yes | authority |
| `binding_sha256` | `sha256` | complete canonical binding bytes | yes | sole external MAP/binding-pair authority |
| `map_id` | `id32` | binding | yes | correlation authority |
| `candidate_root_identity` | candidate component | binding | yes | authority |
| `reviewer_role` | exact `INDEPENDENT_REVIEWER` | Topic 2 review | yes | authority without personal identity |
| `verdict` | enum `APPROVED`, `REJECTED` | Topic 2 review | yes | authority |
| `reviewed_at` | `timestamp_v1` | Topic 2 | yes | authority |
| `object_sha256` | `sha256` | self-digest | no | authority |

- **HISTORICAL APPROVED REQUIREMENT:** `binding_sha256` is the sole external
  MAP/binding pair authority.

## Run authorization — `OS1_RUN_AUTHORIZATION_V1`

| Key | Type/bound | Semantic source | Digest | Authority/privacy |
|---|---|---|---|---|
| `schema` | exact `OS1_RUN_AUTHORIZATION_V1` | schema | yes | authority |
| `run_id` | `id32` | Topic 2 | yes | correlation authority |
| `freeze_review_id` | `id32` | approved freeze review | yes | authority |
| `freeze_review_sha256` | `sha256` | complete canonical freeze-review bytes | yes | authority |
| `binding_id` | `id32` | freeze review | yes | authority |
| `binding_sha256` | `sha256` | freeze review | yes | authority |
| `map_id` | `id32` | binding | yes | authority |
| `candidate_root_identity` | candidate component | binding/freeze review | yes | authority |
| `authorization_id` | `id32` | executable authorization | yes | authority |
| `authorized_at` | `timestamp_v1` | Topic 2 | yes | authority |
| `object_sha256` | `sha256` | self-digest | no | authority |

## Consumption reservation — `OS1_CONSUMPTION_RESERVATION_V1`

| Key | Type/bound | Semantic source | Digest | Authority/privacy |
|---|---|---|---|---|
| `schema` | exact `OS1_CONSUMPTION_RESERVATION_V1` | schema | yes | authority |
| `reservation_id` | `id32` | Topic 2 | yes | correlation authority |
| `run_id` | `id32` | validated run authorization | yes | authority |
| `run_authorization_sha256` | `sha256` | complete canonical run authorization bytes | yes | authority |
| `binding_id` | `id32` | run authorization | yes | authority |
| `map_id` | `id32` | run authorization | yes | authority |
| `use_ordinal` | exact integer `1` | one-use protocol | yes | authority |
| `state` | exact `RESERVED` | reservation at publication | yes | authority |
| `reserved_at` | `timestamp_v1` | Topic 2 | yes | authority |
| `object_sha256` | `sha256` | self-digest | no | authority |

- **HISTORICAL APPROVED REQUIREMENT:** Copying reservation/run bytes creates no
  new run ID, use count, freeze-review authority, or completion.
- **NEW RECONCILIATION SPECIFICATION:** This schema represents the reservation
  evidence object's serialized state as `RESERVED`. Topic 1 makes no rule about
  when it is created or authoritative, whether or how consumption is
  represented outside this object, exclusivity, one-use enforcement,
  transition ordering, filesystem lifecycle, or validator procedure. All such
  decisions belong to Topic 2. Topic 2 consumes this closed schema but is not
  required by Topic 1 to use either mutable or immutable lifecycle mechanics.

# MAP schema

## `OS1_METADATA_MAP_V2`

**HISTORICAL APPROVED REQUIREMENT:** MAP provenance includes preflight ID,
executable authorization/review IDs, authorized root identities copied from
validated preflight/path map, candidate root path from explicit `--root` bytes
only after identity match, current candidate-root identity, and current
MAP-output-parent identity equal to authorized `PRIVATE_OUTPUT_ROOT`.

**NEW RECONCILIATION SPECIFICATION:** Exact top-level keys:

| Key | Type/bound | Semantic source | Digest | Authority/privacy |
|---|---|---|---|---|
| `schema` | exact `OS1_METADATA_MAP_V2` | schema | yes | authority |
| `map_id` | `id32` | Topic 3 | yes | correlation authority |
| `preflight_id` | `id32` | validated preflight | yes | authority |
| `path_map_id` | `id32` | validated path map | yes | authority |
| `authorization_id` | `id32` | validated executable authorization | yes | authority |
| `executable_review_id` | `id32` | authorization | yes | authority |
| `helper_source_sha256` | `sha256` | authorization/current helper | yes | authority |
| `helper_executable_sha256` | `sha256` | authorization/current helper | yes | authority |
| `custody_source_sha256` | `sha256` | preflight/authorization | yes | authority |
| `custody_executable_sha256` | `sha256` | preflight/authorization | yes | authority |
| `candidate_root_path_b64u` | `path_b64u` | exact explicit `--root` bytes | yes | informational locator only; private |
| `candidate_root_identity` | candidate component | current no-follow identity equal to authorization | yes | authority |
| `private_output_root_identity` | private-output component | current no-follow output-parent identity | yes | authority |
| `enumerated_at` | `timestamp_v1` | Topic 3 | yes | authority |
| `entry_count` | integer `0..4096` | number of entries | yes | authority |
| `entries` | array of `map_entry_v1`, exactly `entry_count` | Topic 3 enumeration | yes | authority; private names |
| `object_sha256` | `sha256` | self-digest | no | authority |

### `map_entry_v1`

Exact key order: `record_id`, `basename_b64u`, `kind`.

| Key | Type/bound | Semantic source | Authority/privacy |
|---|---|---|---|
| `record_id` | exact `R-` plus six decimal digits; ordinal `000001..004096` | array index + 1 | authority |
| `basename_b64u` | primitive above | immediate child name bytes | informational locator, private, not durable authority |
| `kind` | enum `REGULAR_FILE`, `DIRECTORY`, `SYMLINK`, `FIFO`, `SOCKET`, `CHARACTER_DEVICE`, `BLOCK_DEVICE`, `OTHER` | no-follow enumeration observation | authority for later correlation |

- **NEW RECONCILIATION SPECIFICATION:** Canonical MAP bytes preserve the entry
  array order supplied by the Topic-3 enumeration algorithm; generic JSON
  canonicalization never sorts the array. Topic 3 must define the deterministic
  entry ordering and how enumeration safely obtains and rechecks it.
- **NEW RECONCILIATION SPECIFICATION:** The canonical `entries` array bytes for
  `entries_sha256` start with `[` and end with `]`, use the same grammar, and
  are exactly the substring serialized as the MAP's `entries` value.
- **HISTORICAL APPROVED REQUIREMENT:** Serialized paths and basenames are not
  durable authority. The MAP is under `PRIVATE_OUTPUT_ROOT`.

# RECORD schemas and stream grammar

## Common rules

- **HISTORICAL APPROVED REQUIREMENT:** RECORD is UTF-8 JSONL, no BOM, exact key
  order, no whitespace, strict closed schema, MAP index `i` to RECORD line `i`,
  IDs `R-000001..R-004096`, at most 4096 lines, and zero MAP entries produce a
  zero-byte RECORD.
- **HISTORICAL APPROVED REQUIREMENT:** The lost approved design recorded a
  4096-byte terminal outer-line cap, an exact maximum valid success line of
  1631 bytes including LF, and maximum total RECORD size of 6,680,576 bytes.
  Those figures remain historical provenance for the lost schema; they are not
  claimed as the exact maxima of this newly specified line schema.
- **NEW RECONCILIATION SPECIFICATION:** The corrected V1
  `INSPECTED_REGULAR_FILE` schema has an exact maximum of **1124 bytes including
  LF**. Reproducible derivation at maximum values: 21 quoted keys plus colons
  contribute 338 bytes; their 21 canonical values contribute 763 bytes (each
  maximum `timestamp_v1` is 58 bytes); opening/closing braces plus 20 commas
  contribute 22 bytes; final LF contributes 1 byte; `338+763+22+1=1124`.
- **NEW RECONCILIATION SPECIFICATION:** The exact maximum canonical
  `EXPECTED_REJECTION` line is 626 bytes including LF, attained by
  `observed_kind=CHARACTER_DEVICE` with
  `rejection_code=UNSUPPORTED_FILE_KIND`. The 4096-byte historical outer cap
  remains an initial fail-closed read ceiling, but schema validation applies
  the narrower 1124/626 exact maxima and permits no padding, unknown field, or
  whitespace.
- **NEW RECONCILIATION SPECIFICATION:** The exact maximum complete RECORD is
  **4,603,904 bytes**, derived as 4096 lines times the larger 1124-byte line
  maximum. The historical 6,680,576-byte value is a looser ceiling belonging
  to the lost schema and is not a valid-size or allocation requirement for
  this new schema.
- **HISTORICAL APPROVED REQUIREMENT:** A complete RECORD is the exact ordered
  concatenation of all lines. A failed unpublished temp is nonauthoritative
  residue. There is no separate RECORD incident artifact.
- **NEW RECONCILIATION SPECIFICATION:** A nonempty complete RECORD contains
  exactly the MAP's `entry_count` lines. Every line's `record_id`, `map_id`, and
  `map_sha256` must correlate with that MAP. No blank line or trailing byte
  beyond the last line's LF is permitted.

## Inspected regular file — `INSPECTED_REGULAR_FILE`

Exact key order; all keys required:

| Key | Type/bound | Semantic source | Authority/privacy |
|---|---|---|---|
| `schema` | exact `OS1_METADATA_RECORD_LINE_V1` | schema | authority |
| `line_class` | exact `INSPECTED_REGULAR_FILE` | outcome | authority |
| `record_id` | MAP record ID | MAP index | authority |
| `map_id` | `id32` | MAP | authority |
| `map_sha256` | `sha256` | complete canonical MAP bytes | authority |
| `binding_id` | `id32` | validated binding | authority |
| `run_id` | `id32` | validated run authorization | authority |
| `reservation_id` | `id32` | validated reservation | authority |
| `kind` | exact `REGULAR_FILE` | stable no-follow/descriptor observation | authority |
| `size_bytes` | integer `0..9007199254740991` | stable `st_size` | allowlisted authority |
| `birth_time` | `timestamp_v1` | stable stat | allowlisted authority |
| `modification_time` | `timestamp_v1` | stable stat | allowlisted authority |
| `metadata_change_time` | `timestamp_v1` | stable stat | allowlisted authority |
| `access_time` | `timestamp_v1` | pre-data-read stable stat | allowlisted observation |
| `finder_info_state` | enum `ABSENT`, `PRESENT` | descriptor probe | authority |
| `finder_type_hex` | empty string if absent; otherwise exactly 8 lowercase hex | FinderInfo bytes 0..3 | allowlisted metadata |
| `finder_creator_hex` | empty string if absent; otherwise exactly 8 lowercase hex | FinderInfo bytes 4..7 | allowlisted metadata |
| `resource_fork_presence` | enum `ABSENT`, `PRESENT` | presence-only descriptor query | allowlisted metadata |
| `data_fork_sha256` | `sha256` | two equal complete passes | authority |
| `helper_source_sha256` | `sha256` | validated helper | authority |
| `helper_executable_sha256` | `sha256` | current validated helper executable | authority |

- **NEW RECONCILIATION SPECIFICATION:** `finder_info_state=ABSENT` requires both
  hex fields empty; `PRESENT` requires both eight-hex fields nonempty. Probe
  errors are fatal and never serialized as a state.
- **HISTORICAL APPROVED REQUIREMENT:** Runtime device/inode/kind may bind the
  object operationally but device and inode are not serialized in RECORD.

## Expected rejection — `EXPECTED_REJECTION`

Exact key order; all keys required:

| Key | Type/bound | Semantic source | Authority/privacy |
|---|---|---|---|
| `schema` | exact `OS1_METADATA_RECORD_LINE_V1` | schema | authority |
| `line_class` | exact `EXPECTED_REJECTION` | outcome | authority |
| `record_id` | MAP record ID | MAP index | authority |
| `map_id` | `id32` | MAP | authority |
| `map_sha256` | `sha256` | complete canonical MAP bytes | authority |
| `binding_id` | `id32` | validated binding | authority |
| `run_id` | `id32` | validated run authorization | authority |
| `reservation_id` | `id32` | validated reservation | authority |
| `observed_kind` | same kind enum as MAP | stable observation | authority |
| `rejection_code` | one of four literals below | stable expected outcome | authority |
| `helper_source_sha256` | `sha256` | validated helper | authority |
| `helper_executable_sha256` | `sha256` | current validated helper | authority |

**NEW RECONCILIATION SPECIFICATION:** The four closed rejection literals are:

1. `SYMLINK_NOFOLLOW`
2. `DIRECTORY_NOT_CANDIDATE`
3. `UNSUPPORTED_FILE_KIND`
4. `FINDER_ALIAS_FILE`

The first requires `observed_kind=SYMLINK`; the second requires `DIRECTORY`;
the third requires `FIFO`, `SOCKET`, `CHARACTER_DEVICE`, `BLOCK_DEVICE`, or
`OTHER`; the fourth requires `REGULAR_FILE` and Finder Type bytes `616c6973`
(`alis`). No other pairing is canonical.

- **HISTORICAL APPROVED REQUIREMENT:** Stable nonregular and alias outcomes are
  nonfatal expected rejections. I/O, race, mutation, malformed evidence, and
  probe failures are fatal and are not RECORD lines.
- **HISTORICAL APPROVED REQUIREMENT:** After alias determination, candidate data
  is not read. FinderInfo uses descriptor `fgetxattr`; `ENOATTR` means absent;
  length is 8..4096; only the first eight bytes are retained. Resource-fork
  handling is presence only and never serializes size or content.

# Completion schema

## `OS1_RECORD_COMPLETION_V1`

**HISTORICAL APPROVED REQUIREMENT:** `metadata-helper inspect` is the sole
completion creator, and byte-valid completion is terminal durable authority.

| Key | Type/bound | Semantic source | Digest | Authority/privacy |
|---|---|---|---|---|
| `schema` | exact `OS1_RECORD_COMPLETION_V1` | schema | yes | authority |
| `completion_id` | `id32` | Topic 5 | yes | correlation authority |
| `run_id` | `id32` | E09 run authorization | yes | authority |
| `run_authorization_sha256` | `sha256` | complete canonical E09 run-authorization bytes | yes | content-bound authority |
| `reservation_id` | `id32` | E09 reservation | yes | authority |
| `reservation_sha256` | `sha256` | complete canonical E09 consumption-reservation bytes | yes | content-bound authority |
| `map_id` | `id32` | E09 MAP | yes | authority |
| `map_sha256` | `sha256` | complete canonical MAP bytes | yes | authority |
| `binding_id` | `id32` | E09 binding | yes | authority |
| `binding_sha256` | `sha256` | complete canonical binding bytes | yes | authority |
| `record_sha256` | `sha256` | complete RECORD stream bytes | yes | terminal evidence authority |
| `record_byte_count` | integer `0..4603904` | complete RECORD | yes | authority |
| `record_line_count` | integer `0..4096` | complete RECORD | yes | authority |
| `candidate_root_identity` | candidate component | E09/correlated chain | yes | authority |
| `private_output_root_identity` | private-output component | correlated chain | yes | authority |
| `custody_record_root_identity` | custody component | correlated chain | yes | authority |
| `helper_source_sha256` | `sha256` | validated helper | yes | authority |
| `helper_executable_sha256` | `sha256` | validated helper | yes | authority |
| `completed_at` | `timestamp_v1` | Topic 5 | yes | authority |
| `object_sha256` | `sha256` | self-digest | no | authority |

Topic 5 defines creation preconditions, IP sequencing, and failure behavior.
Schema presence alone is not completion authority.

# Operational-event schema

## `OS1_OPERATIONAL_EVENT_V1`

**HISTORICAL APPROVED REQUIREMENT:** Operational events are nonauthoritative
operational history. IP11 suppresses the operational event.

| Key | Type/bound | Semantic source | Digest | Authority/privacy |
|---|---|---|---|---|
| `schema` | exact `OS1_OPERATIONAL_EVENT_V1` | schema | yes | nonauthority |
| `event_id` | `id32` | Topic 5 | yes | correlation only |
| `run_id` | `id32` | run authorization | yes | correlation only |
| `reservation_id` | `id32` | reservation | yes | correlation only |
| `operation_class` | enum `IP01` through `IP10` | Topic 5 operation | yes | history only |
| `outcome` | enum `SUCCEEDED`, `FAILED` | observed operation | yes | history only |
| `error` | exact `NONE` for success; otherwise one V2 error literal | failure | yes | history only |
| `observed_at` | `timestamp_v1` | event observation | yes | history only |
| `object_sha256` | `sha256` | self-digest | no | integrity, not authority |

**NEW RECONCILIATION SPECIFICATION:** `SUCCEEDED` requires `error=NONE`;
`FAILED` requires a V2 error other than `NONE`. The schema deliberately cannot
represent IP11. Topic 5 decides when events are attempted, whether a failed
event write affects operational success, and the exact operation-to-error map.

# Digest/canonical-byte rules

- **NEW RECONCILIATION SPECIFICATION:** SHA-256 always means FIPS 180-4 SHA-256
  over an exact byte sequence and renders as 64 lowercase hexadecimal digits.
  No pathname, metadata, xattr, resource fork, encoding wrapper, or separator
  is implicitly added.
- **NEW RECONCILIATION SPECIFICATION:** “Complete canonical object bytes” means
  the common canonical object envelope including its final LF and its validated
  `object_sha256` field.
- **NEW RECONCILIATION SPECIFICATION:** An object's self-digest projection is
  a canonical object with the final `object_sha256` member omitted and all
  preceding members unchanged, followed by LF. SHA-256 of that projection is
  the required `object_sha256`. There is no placeholder, empty digest, or
  recursive hashing.
- **NEW RECONCILIATION SPECIFICATION:** `map_sha256`, `binding_sha256`,
  `review_sha256`, `review_identities_sha256`, `freeze_review_sha256`,
  `run_authorization_sha256`, and `reservation_sha256` cover the complete
  canonical bytes of the named standalone object, including its self-digest
  and LF.
- **NEW RECONCILIATION SPECIFICATION:** `record_sha256` covers the exact complete
  RECORD stream, including every line LF; for the empty RECORD it is SHA-256 of
  zero bytes.
- **NEW RECONCILIATION SPECIFICATION:** File digests cover exactly the complete
  regular-file data bytes and no LF or metadata.
- **NEW RECONCILIATION SPECIFICATION:** A parser validates grammar/schema and
  self-digest before an object may supply correlation values. Digest validity
  proves byte integrity only; later topics define authority.

The corrected digest dependency graph is:

```text
source/build/executable bytes -> executable review
executable review -> executable review identities
executable review identities -> executable authorization -> preflight -> path map
preflight + path map + executable authorization -> MAP -> binding
binding -> freeze review -> run authorization -> consumption reservation
MAP + binding + run authorization + consumption reservation + RECORD -> completion
run/reservation identifiers -> operational event
```

**NEW RECONCILIATION SPECIFICATION:** Every arrow involving a serialized
evidence object is a forward reference to that object's complete canonical
bytes or to fields copied from an already existing predecessor. Each
`object_sha256` uses only the self-excluding projection already defined. No
predecessor includes a digest of a successor, so the graph is acyclic. Topic 5
can bind all four E09 sources by complete canonical bytes without changing the
completion schema.

# OS1_METADATA_HELPER_ERRORS_V2

## Taxonomy rules

- **HISTORICAL APPROVED REQUIREMENT:** The closed taxonomy identity is
  `OS1_METADATA_HELPER_ERRORS_V2`.
- **NEW RECONCILIATION SPECIFICATION:** The enum has exactly the 55 literals in
  the table below. Expected candidate rejections are not errors and never use
  this enum.
- **NEW RECONCILIATION SPECIFICATION:** Every enum outcome is fatal to the
  current command and forbids a valid terminal completion for that invocation.
  Evidence final before invocation remains untouched. A failure after a rename
  may leave named but nonterminal bytes as governed by Topics 2–5; this taxonomy
  neither deletes them nor promotes them to durable authority. Topic 5 may
  define nonauthoritative operational-event attempts without converting them
  into final authoritative evidence.
- **NEW RECONCILIATION SPECIFICATION:** “May raise” allocates vocabulary only;
  Topics 2–5 define exact failure-point mapping and precedence.

### Mandatory semantic partition and precedence

**NEW RECONCILIATION SPECIFICATION:** Topics 2–5 must map each fatal failure to
exactly one literal by the following partitions. They may not choose ad hoc:

1. A specialized literal for the semantic operation and target always excludes
   a generic literal. `INTERNAL_INVARIANT_FAILED` is used only for a violated
   program invariant not representable by any other member; it never wraps an
   ordinary OS, parse, authorization, or publication failure.
2. `EVIDENCE_*` applies only while acquiring or validating an explicitly
   supplied serialized evidence input. It never classifies the producer's own
   temporary, final output, candidate data fork, completion, or operational
   event.
3. Within supplied-evidence parsing, precedence is UTF-8/BOM, JSON syntax,
   unsupported schema discriminator, schema field/type/value/cardinality,
   canonical lexical bytes, digest, cross-object correlation, then authority.
   `EVIDENCE_SCHEMA_INVALID` excludes ordering, whitespace, escaping, and
   numeric-spelling defects; those are
   `EVIDENCE_CANONICAL_BYTES_INVALID`. `EVIDENCE_UNAUTHORIZED` applies only to
   a structurally valid, canonical, digest-valid, correlated object that lacks
   a required verdict/state/authority.
4. Root errors are partitioned by subject: inability to acquire a required
   no-follow root chain/terminal/parent observation, a non-directory terminal,
   or malformed chain structure is `ROOT_CHAIN_INVALID`; unequal current versus
   authorized identity after successful acquisition is
   `ROOT_AUTHORITY_MISMATCH`; prohibited overlap is `ROOT_SEPARATION_FAILED`;
   and a successfully acquired output parent that lacks the authorized role or
   identity is `OUTPUT_PARENT_UNAUTHORIZED`. `OBJECT_STAT_FAILED` excludes
   root-chain and output-parent operations.
5. `OBJECT_IDENTITY_MISMATCH` is a comparison failure present at the first
   relevant binding check. `OBJECT_CHANGED_OR_REPLACED` is a later device,
   inode, or kind discontinuity for that bound object.
   `RACE_OR_MUTATION_DETECTED` excludes identity discontinuity and means the
   same bound object retained device/inode/kind but another protected stat,
   metadata, or digest observation changed.
6. Candidate open, data-fork read, and descriptor-rewind failures use,
   respectively, `CANDIDATE_OPEN_FAILED`, `CANDIDATE_READ_FAILED`, and
   `CANDIDATE_SEEK_FAILED`. Candidate stat failure uses `OBJECT_STAT_FAILED`:
   that literal is intentionally shared by ordinary object-stat operations,
   including candidate stat, because no more-specific candidate-stat literal
   exists. Other specialized candidate operations use their exact
   `CANDIDATE_*` literal when one exists. FinderInfo operations use
   `FINDERINFO_*`, resource-fork presence operations use
   `RESOURCE_FORK_PROBE_FAILED`, and two-pass length/digest disagreement uses
   `HASH_LENGTH_MISMATCH` or `HASH_PASSES_DIFFER`. A specialized literal wins
   over a generic literal only when it covers that exact semantic operation;
   unrelated `CANDIDATE_*` literals do not bar `OBJECT_STAT_FAILED` for
   candidate stat. Generic evidence/object I/O literals otherwise do not apply
   inside those specialized domains.
7. Generic `TEMP_*` and `FINAL_*` publication literals apply only to MAP,
   binding, RECORD, and other non-completion authoritative objects. They exclude
   completion and operational-event publication. Completion uses only
   `COMPLETION_*`; operational-event creation/publication uses only
   `OPERATIONAL_EVENT_FAILED`.
   `OUTPUT_ALREADY_EXISTS` applies when required absence is observed before a
   rename; an `EEXIST` returned by the attempted exclusive rename uses
   `FINAL_RENAME_FAILED` or `COMPLETION_RENAME_FAILED` according to target.
8. A temporary reread syscall/open/read failure is `TEMP_REREAD_FAILED`; a
   successful reread whose bytes/schema/digest/count differ is
   `TEMP_CONTENT_MISMATCH`. A final no-follow reopen or any validation performed
   as part of that reopen is `FINAL_REOPEN_FAILED`, not `EVIDENCE_*` or
   `EVIDENCE_DIGEST_MISMATCH`.
9. For non-completion output, `FINAL_FILE_VERIFICATION_FAILED` covers final
   basename/file/size checks, while `FINAL_ROOT_VERIFICATION_FAILED` covers only
   root/chain/terminal/parent checks. For completion, both file/byte/size and
   custody-root final checks are exclusively
   `COMPLETION_FINAL_VERIFICATION_FAILED`; completion rename and directory sync
   retain their distinct specialized literals.
10. `RECORD_INCOMPLETE` applies only after individually valid canonical RECORD
    lines fail required count/order/coverage. Malformed individual lines use
    the applicable evidence error when supplied for validation, or
    `TEMP_CONTENT_MISMATCH`/`FINAL_REOPEN_FAILED` in those production stages.

This is a semantic partition, not the later operation-to-error matrix. Later
topics still identify each concrete failure point and precedence between
multiple failures, but every selected literal must satisfy exactly one domain
above.

| Literal | Semantic class | Final evidence from failing command | May raise |
|---|---|---|---|
| `INVALID_INVOCATION` | malformed arguments/arity/options | none | T2,T3,T4,T5 |
| `PATH_GRAMMAR_INVALID` | nonabsolute, NUL, empty, or disallowed path form | none | T2,T3,T4,T5 |
| `EVIDENCE_OPEN_FAILED` | supplied evidence cannot be opened no-follow | none | T2,T3,T4,T5 |
| `EVIDENCE_NOT_REGULAR_FILE` | supplied evidence is not a regular file | none | T2,T3,T4,T5 |
| `EVIDENCE_SIZE_LIMIT_EXCEEDED` | evidence/object/line bound exceeded | none | T2,T3,T4,T5 |
| `EVIDENCE_READ_FAILED` | complete evidence read failed | none | T2,T3,T4,T5 |
| `EVIDENCE_UTF8_INVALID` | invalid UTF-8 or BOM | none | T2,T3,T4,T5 |
| `EVIDENCE_JSON_INVALID` | JSON grammar invalid | none | T2,T3,T4,T5 |
| `EVIDENCE_SCHEMA_UNSUPPORTED` | schema literal/version unsupported | none | T2,T3,T4,T5 |
| `EVIDENCE_SCHEMA_INVALID` | closed field/type/value/cardinality constraint failed; excludes byte-order/lexical defects | none | T2,T3,T4,T5 |
| `EVIDENCE_CANONICAL_BYTES_INVALID` | otherwise schema-shaped bytes violate order/whitespace/escape/numeric canonical form | none | T2,T3,T4,T5 |
| `EVIDENCE_DIGEST_MISMATCH` | self/reference digest mismatch | none | T2,T3,T4,T5 |
| `EVIDENCE_CORRELATION_MISMATCH` | IDs/counts/digests disagree across evidence | none | T2,T3,T4,T5 |
| `EVIDENCE_UNAUTHORIZED` | evidence lacks required authority/verdict/state | none | T2,T3,T4,T5 |
| `EXECUTABLE_IDENTITY_MISMATCH` | source/runtime/review executable mismatch | none | T2,T3,T4 |
| `ROOT_AUTHORITY_MISMATCH` | current root differs from authorized identity | none | T2,T3,T4,T5 |
| `ROOT_CHAIN_INVALID` | no-follow chain/parent/terminal validation failed | none | T2,T3,T4,T5 |
| `ROOT_SEPARATION_FAILED` | required root separation failed | none | T2,T3,T4,T5 |
| `OUTPUT_PARENT_UNAUTHORIZED` | output parent is not authorized root | none | T2,T3,T4,T5 |
| `OUTPUT_ALREADY_EXISTS` | required-absent final output exists | none | T2,T3,T4,T5 |
| `RESERVATION_UNAVAILABLE` | required reservation cannot be established | none | T2,T4 |
| `RESERVATION_ALREADY_CONSUMED` | run/reservation already has terminal use | none | T2,T4,T5 |
| `ENUMERATION_FAILED` | immediate-child enumeration or entry stat failed | none | T3 |
| `ENTRY_LIMIT_EXCEEDED` | enumeration exceeds 4096 MAP entries | none | T3 |
| `OBJECT_STAT_FAILED` | required object/path stat failed | none | T3,T4,T5 |
| `OBJECT_IDENTITY_MISMATCH` | descriptor/path or bound identity disagrees | none | T3,T4,T5 |
| `OBJECT_CHANGED_OR_REPLACED` | bound object's device/inode/kind changes after initial binding | none | T3,T4,T5 |
| `RACE_OR_MUTATION_DETECTED` | same bound identity retains device/inode/kind but another protected observation changes | none | T3,T4,T5 |
| `CANDIDATE_OPEN_FAILED` | authorized candidate open failed | none | T4 |
| `CANDIDATE_READ_FAILED` | candidate data-fork read failed | none | T4 |
| `CANDIDATE_SEEK_FAILED` | descriptor rewind failed | none | T4 |
| `FINDERINFO_PROBE_FAILED` | FinderInfo query/read failed | none | T4 |
| `FINDERINFO_MALFORMED` | FinderInfo present outside 8..4096 bytes | none | T4 |
| `RESOURCE_FORK_PROBE_FAILED` | resource-fork presence query failed | none | T4 |
| `HASH_LENGTH_MISMATCH` | bytes hashed differ from authorized expected size | none | T4 |
| `HASH_PASSES_DIFFER` | two complete data-fork digests differ | none | T4 |
| `TEMP_CREATE_FAILED` | non-completion/non-event opaque temporary creation failed | none | T2,T3,T4,T5 |
| `TEMP_WRITE_FAILED` | non-completion/non-event complete temporary write failed | none | T2,T3,T4,T5 |
| `TEMP_FILE_SYNC_FAILED` | non-completion/non-event temporary file fsync failed | none | T2,T3,T4,T5 |
| `TEMP_REREAD_FAILED` | non-completion/non-event required temporary reread I/O failed | none | T2,T3,T4,T5 |
| `TEMP_CONTENT_MISMATCH` | non-completion/non-event successful reread has wrong bytes/schema/digest/count | none | T2,T3,T4,T5 |
| `FINAL_RENAME_FAILED` | non-completion/non-event exclusive final rename failed other than collision | none | T2,T3,T4,T5 |
| `FINAL_DIRECTORY_SYNC_FAILED` | non-completion/non-event containing-directory fsync failed | none | T2,T3,T4,T5 |
| `FINAL_FILE_VERIFICATION_FAILED` | non-completion final basename/file/size check failed outside reopen | none | T2,T3,T4,T5 |
| `FINAL_ROOT_VERIFICATION_FAILED` | non-completion final root/chain/parent check failed | none | T2,T3,T4,T5 |
| `FINAL_REOPEN_FAILED` | non-completion required final no-follow reopen or its validation failed | none | T2,T3,T4,T5 |
| `RECORD_INCOMPLETE` | RECORD line count/order/coverage incomplete | none | T4,T5 |
| `COMPLETION_CREATE_FAILED` | completion temporary creation failed | none | T5 |
| `COMPLETION_WRITE_FAILED` | completion bytes write failed | none | T5 |
| `COMPLETION_SYNC_FAILED` | completion temporary-file fsync failed | none | T5 |
| `COMPLETION_RENAME_FAILED` | exclusive completion rename failed | none | T5 |
| `COMPLETION_DIRECTORY_SYNC_FAILED` | custody-root directory fsync failed after completion rename | none | T5 |
| `COMPLETION_FINAL_VERIFICATION_FAILED` | final completion bytes/size/root check failed | none | T5 |
| `OPERATIONAL_EVENT_FAILED` | nonauthoritative event creation/publication failed | none | T5 |
| `INTERNAL_INVARIANT_FAILED` | unreachable/internal invariant or uncategorized safe failure | none | T2,T3,T4,T5 |

There are exactly 55 enum members. “Final evidence” in the table means valid
terminal completion: no error outcome may publish one. A failure after an
earlier rename may leave named MAP, binding, RECORD, event, or invalid
completion bytes; Topic 5 determines that operation state and later validators
determine validity. Such bytes are not silently deleted or called authoritative
by this taxonomy. Publication-related terms identify failure
classes only. Topics 2–5 decide which class applies to each operation and what
already-published non-completion bytes remain as observational residue under
the historical terminal-authority model.

# Public stderr grammar

- **HISTORICAL APPROVED REQUIREMENT:** The public grammar is
  `OS1_METADATA_HELPER_ERROR:<ENUM>\n`.
- **NEW RECONCILIATION SPECIFICATION:** On failure, an OS1 metadata-helper or
  custody command writes exactly the ASCII bytes
  `OS1_METADATA_HELPER_ERROR:`, one of the 55 literals, and LF to stderr. It
  writes zero stdout bytes and no other stderr bytes.
- **NEW RECONCILIATION SPECIFICATION:** On success it writes zero stdout and
  zero stderr bytes. Exit status is `0` for success, `64` for
  `INVALID_INVOCATION` or `PATH_GRAMMAR_INVALID`, and `70` for every other V2
  error. Signal/host termination is outside the public command contract.
- **NEW RECONCILIATION SPECIFICATION:** Paths, basenames, OS error strings,
  candidate bytes, xattr data, IDs, hashes, and dynamic detail never appear on
  public stdout/stderr.

# Schema-level privacy boundary

- **DURABLE PRIOR AUTHORITY:** Only bounded provenance metadata and complete
  data-fork SHA-256 are permitted; candidate content inference, arbitrary xattr
  enumeration/values, reference data, and content-bearing output are forbidden.
- **DURABLE PRIOR AUTHORITY:** FinderInfo output is only Type/Creator from its
  first eight bytes. Resource-fork handling is presence-only.
- **HISTORICAL APPROVED REQUIREMENT:** RECORD allowlists four stat times, kind,
  `st_size`, FinderInfo Type/Creator only, resource-fork presence only,
  whole-data-fork SHA-256, and helper source/runtime executable digests.
- **NEW RECONCILIATION SPECIFICATION:** The schemas form an allowlist. Any datum
  without an exact field above is forbidden from serialization.

The explicit schema-level denylist is:

- candidate data-fork bytes, excerpts, strings, magic, MIME/UTI, entropy,
  histograms, rolling/chunk/fuzzy hashes, or structural deductions;
- resource-fork size, bytes, digest, preview, structure, or decoded value;
- FinderInfo bytes after byte 7;
- arbitrary xattr names, values, counts, sizes, digests, or inferred meaning;
- Finder comments, tags, quarantine values, Spotlight data, thumbnails, icons,
  previews, sequence/track/event/channel/Patch/Controller/Tempo/Meter data,
  routing, OMS/device data, embedded filenames, and audio references;
- candidate device/inode values in RECORD lines; file modes, link targets,
  owner/group IDs, ACLs, flags, block counts, or physical locations;
- raw path or basename strings; only bounded base64url byte locators in MAP and
  path-map are permitted;
- personal names, usernames, hostnames, volume names, cloud locations, owner
  questionnaire text, permissions narrative, or provenance narrative;
- arbitrary diagnostics, OS messages, stack traces, crash content, or dynamic
  public error detail; and
- null placeholders, extension maps, unknown fields, or vendor fields.

Root device/inode and root path locators are private operational evidence,
allowed only in their exact schemas. Path locator bytes participate in object
integrity but remain informational and never authorize an object. Candidate
runtime device/inode/kind may be held ephemerally for Topic-4 binding; only
allowlisted `kind` is serialized.

# Cross-topic boundary table

| Topic-1 decision | Fixed here | Topic 2 remains | Topic 3 remains | Topic 4 remains | Topic 5 remains | Topic 6 remains |
|---|---|---|---|---|---|---|
| Canonical JSON/primitive grammar | bytes, order, types, bounds | authority validation | producer integration | parser integration | terminal parser integration | byte vectors |
| Self/reference digests | exact domains | trust/correlation order | MAP production checks | RECORD correlations | completion/event checks | digest vectors |
| Executable review/identities/authorization schemas | closed fields and complete-byte links | creators, A/B checks, identity projection, verdict/authorization lifecycle | consume authorization | consume authorization | final correlation | tests |
| Preflight/path-map schemas | closed fields/path nonauthority | root-chain projection, creation, validation | consume and recheck | consume and recheck | terminal recheck | tests |
| MAP schema | exact object/entry shape | binding relationship | enumeration, timestamps, publication, validation order | consume MAP | E09/final checks | tests |
| Binding/freeze/run/reservation schemas | closed shapes | creation, authority, one-use lifecycle, validator rows | MAP handoff | pre-candidate gate | terminal transitions | tests |
| RECORD line/stream schemas | exact bytes/fields/codes/bounds | run authority inputs | no RECORD behavior | RA01–RA19, outcome selection, construction, publication | terminal use | tests |
| Completion schema | exact fields and complete-byte digests for all four E09 sources | authority inputs | none | provide RECORD result | sole creation, E09 validation, IP sequence, durable validity | tests |
| Operational event schema | exact fields/nonauthority | authority inputs | none | none | triggers, suppression, failure effect | tests |
| Error enum/stderr | closed literals/classes/line/exit | exact T2 mappings/precedence | exact T3 mappings/precedence | exact T4 mappings/precedence | exact T5 mappings/precedence | coverage tests |
| Privacy allow/deny schema | serializable boundary | custody handling | enumeration access algorithm | candidate probe/read algorithm | event redaction | nonexposure tests |

No table cell marked “remains” is specified by implication. Later topics may
choose exact algorithms within these schemas but may not add fields, enum
literals, alternate canonical encodings, or hidden-state dependencies without
amending and independently reviewing Topic 1.

# Determinism requirements

- **NEW RECONCILIATION SPECIFICATION:** Two conforming producers given the same
  typed field values must produce byte-identical output.
- **NEW RECONCILIATION SPECIFICATION:** Two conforming parsers must agree on
  acceptance, decoded typed values, self-digest projection, and referenced
  digest domain for every byte string.
- **NEW RECONCILIATION SPECIFICATION:** No locale, timezone formatting, Unicode
  normalization, dictionary iteration order, platform JSON formatting,
  filesystem display name, environment variable, clock formatting, or pathname
  decoding may influence canonical bytes.
- **NEW RECONCILIATION SPECIFICATION:** Timestamps serialize numeric syscall
  components, never formatted dates. Filesystem byte strings use base64url and
  are never lossily converted to Unicode.
- **NEW RECONCILIATION SPECIFICATION:** Schema validation precedes correlation;
  correlation precedes later authority use. Unknown versions fail closed.
- **NEW RECONCILIATION SPECIFICATION:** No object requires a database, daemon,
  network lookup, global registry, signature, MAC, secret, or hidden path
  lookup. All cross-object inputs are explicitly supplied under later command
  contracts.

# Unresolved decisions

Topic 1 has no unresolved implementation-critical schema, canonical-byte,
digest-domain, schema-privacy, public enum, stderr, or exit-status decision.

The following are deliberately unresolved here and owned by later topics:

- Topic 2: ID generation; root-chain digest projection; custody commands;
  executable checks; creators, authority, lifecycle, correlations, reservation
  one-use mechanics, and validator rows.
- Topic 3: enumerate validation/access/race algorithm, MAP construction timing,
  and MAP/binding publication handoff.
- Topic 4: inspect pre-candidate gate, RA01–RA19, candidate I/O/race mechanics,
  exact outcome-to-enum mapping, and RECORD publication execution.
- Topic 5: operational success, event triggers, completion creation, IP01–IP11,
  failure precedence, fsync/rename/final/reopen checks, and durable validation.
- Topic 6: every conformance test, fixture, fault injection, expected byte
  vector, error assertion, and registry mapping.

These deferrals do not permit later topics to change Topic-1 schemas or invent
new public errors silently.

## Forward-compatibility audit

**NEW RECONCILIATION SPECIFICATION:** The 14-object inventory contains every
explicit serialized evidence input/output identified by surviving authority,
including `--review-identities FILE`. All authority-required fields needed to
bind complete canonical predecessor bytes are present. Canonical grammar and
digest domains are closed; the V2 vocabulary remains exactly 55 members; and
the privacy allowlist cannot be broadened by implication.

Topics 2–6 can define lifecycle/state transitions, validator algorithms,
enumeration ordering, candidate-read algorithms, publication ordering,
operation-to-error mappings, and conformance tests without adding a schema or
field, changing canonical bytes/digest domains, adding an error literal, or
broadening candidate metadata. If later review discovers otherwise, it must
amend and independently review Topic 1 rather than silently extending it.

# Implementation authority status

Self-audit against the independent findings:

- **I1 — missing review-identities schema: CLOSED.** The fourteenth top-level
  schema and its authorization digest link are defined.
- **I2 — repository identity typing: CLOSED.** Every `repository_commit` uses
  exact 40-hex `git_oid_sha1`.
- **I3 — E09 content binding: CLOSED.** Completion has complete-canonical-byte
  digests for MAP, binding, run authorization, and reservation; the graph is
  explicitly acyclic.
- **I4 — reservation lifecycle leakage: CLOSED.** Topic 1 defines only the
  serialized `RESERVED` object and defers all lifecycle mechanisms to Topic 2.
- **I5 — RECORD size contradiction: CLOSED.** Historical 1631/6,680,576 values
  remain labeled provenance; the new schema's derived normative maxima are
  1124 bytes per success line and 4,603,904 bytes per complete RECORD.
- **I6 — error semantic overlap: CLOSED.** The 55-member vocabulary now has
  mandatory nonoverlapping semantic domains and specialization precedence
  without a later-topic failure-point matrix.

The complete audit found no new rule represented as historical, no
contradiction with surviving authority, no ambiguous key order or digest
domain, no digest cycle, no schema-level privacy regression, no durable
authority assigned to a pathname, and no hidden state requirement. MAP ordering
remains Topic 3. Current helper V1 names and behavior were not adopted as
authority. Lifecycle, enumerate, inspect, publication, and test algorithms are
explicitly deferred, with no later-topic algorithm added here.

Final 20-gap disposition: **3 FULLY CLOSED BY TOPIC 1; 15 PARTIALLY CLOSED —
LATER TOPIC REQUIRED; 2 OUTSIDE TOPIC 1; 0 NOT ACTUALLY ADDRESSED.**

**TOPIC 1 CORRECTION COMPLETE: YES**

**TOPIC 1 DESIGN COMPLETE: YES**

**UNRESOLVED TOPIC 1 IMPLEMENTATION-CRITICAL DECISIONS: 0**

**CORRECTIVE IMPLEMENTATION AUTHORIZED: NO**

**NEXT REQUIRED ACTION:**
**INDEPENDENT REVIEW OF OS1 TARGETED RE-SPECIFICATION TOPIC 1**
