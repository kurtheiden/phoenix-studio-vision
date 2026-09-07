# OS1 Custody Authority, Lifecycle, and Validator Registry V1

# 1. Purpose and authority boundary

This document is targeted re-specification Topic 2 of 6. It closes custody
authority, lifecycle, root-bound validation, and the custody validator registry.
It is new design authority, not a reconstruction of a lost `/tmp` artifact.

Topic 1, `OS1_CANONICAL_EVIDENCE_SCHEMAS_AND_ERRORS_V1.md`, remains closed and
unchanged. Its schemas, field order and types, canonical bytes, digest domains,
privacy rules, pathname nonauthority, original 55-member error vocabulary,
stderr and exit grammar, RECORD line maximum of 1124 bytes including LF, and complete
RECORD maximum of 4,603,904 bytes are incorporated without modification.
The approved durable Topic-1 error-taxonomy amendment adds four members and
allocates existing `OBJECT_STAT_FAILED` to T2; the resulting vocabulary is
exactly 59 members. No other Topic-1 authority changes.

Topics 3 through 6 remain outside this design. This document defines only the
custody-facing inputs that those topics consume. It does not define MAP
enumeration or production, candidate access or RECORD production, completion
or operational-event publication, or deterministic conformance tests.

The threat model is honest, access-controlled custody. The rules detect
accidental substitution, incomplete, stale, mismatched or malformed evidence,
post-hoc tuning, mutation at defined observations, and unauthorized root or
path substitution. They do not claim protection against a malicious kernel,
malicious administrator, Byzantine filesystem, or coordinated hostile rewrite
of every authority artifact. No signature, MAC, secret, network trust, daemon,
database, global registry, or hidden state is introduced.

Corrective implementation remains unauthorized. Reserve access, candidate
selection or access, Studio Vision access, reference MIDI access or creation,
and provenance lock remain unauthorized.

Normative terms `must`, `must not`, `required`, and `exactly` are binding.

# 2. Authority objects consumed from Topic 1

This section inventories schemas; it does not redefine them.

| Topic-1 schema or form | Topic-2 role | Action | Relevant correlations/digests | Root assignment |
|---|---|---|---|---|
| `root_identity_v1` | Root authority component | create and validate | role, terminal device/inode/kind/mode, `chain_sha256` | the role named in the component |
| `file_digest_v1` | Complete source/build input identity | create and validate | byte count and SHA-256 | source input beneath `IMPLEMENTATION_REVIEW_ROOT` |
| `OS1_EXECUTABLE_REVIEW_V1` | Deterministic-build review | create, validate, correlate | review ID, repository commit, source/build digests, executable digests, review root | `IMPLEMENTATION_REVIEW_ROOT` |
| `OS1_EXECUTABLE_REVIEW_IDENTITIES_V1` | Explicit review projection | create, validate, correlate | review ID/SHA-256 and all projected identities | `IMPLEMENTATION_REVIEW_ROOT` |
| `OS1_EXECUTABLE_AUTHORIZATION_V1` | Current executable authority | create and validate | authorization/review IDs, review digests, source/runtime digests | `IMPLEMENTATION_REVIEW_ROOT` |
| `OS1_FOUR_ROOT_PREFLIGHT_V1` | Frozen four-root authority | create and validate | authorization/review IDs, repository commit, custody identities, four roots | `CUSTODY_RECORD_ROOT` |
| `OS1_PATH_MAP_V1` | Explicit locator-to-root correlation | create and validate | path-map/preflight/authorization IDs and four root identities | `CUSTODY_RECORD_ROOT` |
| `OS1_METADATA_MAP_V2` | Topic-3 enumeration result | validate and correlate only | MAP self/complete digest, IDs, executable/source digests, roots, entry count/digest input | `PRIVATE_OUTPUT_ROOT` |
| `OS1_MAP_BINDING_V1` | Custody binding of a MAP | create and validate | binding/MAP/preflight/path-map/authorization IDs, MAP SHA-256, roots, entry data | `PRIVATE_OUTPUT_ROOT` |
| `OS1_FREEZE_REVIEW_V1` | Independent MAP/binding verdict | create and validate | freeze/binding/MAP IDs, binding SHA-256, candidate root, verdict | `CUSTODY_RECORD_ROOT` |
| `OS1_RUN_AUTHORIZATION_V1` | Authority for one prospective run | create and validate | run/freeze/binding/MAP/authorization IDs and complete-object digests | `CUSTODY_RECORD_ROOT` |
| `OS1_CONSUMPTION_RESERVATION_V1` | Exclusive reservation of that run | create and validate | reservation/run/binding/MAP IDs, run-authorization SHA-256, ordinal/state | `CUSTODY_RECORD_ROOT` |
| RECORD line schemas and RECORD stream | Later Topic-4 output | no creation or validation in Topic 2 | only the already-fixed future correlations | `PRIVATE_OUTPUT_ROOT` |
| `OS1_RECORD_COMPLETION_V1` | Later Topic-5 terminal authority | no creation or validation in Topic 2 | no Topic-2 action | `CUSTODY_RECORD_ROOT` |
| `OS1_OPERATIONAL_EVENT_V1` | Later Topic-5 nonauthority | no creation or validation in Topic 2 | no Topic-2 action | `CUSTODY_RECORD_ROOT` |

The authoritative root assignment is a placement constraint, not a pathname
claim. A schema-valid object outside its required root is not authoritative.

## 2.1 Topic-2 ID rule

Topic 1 delegates `id32` generation to Topic 2. Each Topic-2 producer derives
its primary ID deterministically. Let `P` be the Topic-1 canonical JSON object
formed from the final typed fields, omitting only the object's primary ID and
`object_sha256`, retaining declared order, and followed by LF. This projection
is a digest input, not a serialized evidence schema and is never published.
The primary ID is the first 32 lowercase hexadecimal characters of:

```text
SHA-256(ASCII("OS1_TOPIC2_ID_V1\n") || ASCII(schema literal) || LF || P)
```

The primary IDs are `review_id`, `authorization_id`, `preflight_id`,
`path_map_id`, `binding_id`, `freeze_review_id`, `run_id`, and
`reservation_id`. All referenced predecessor IDs already exist before this
calculation. `reviewed_at`, `authorized_at`, `preflighted_at`, `mapped_at`,
`bound_at`, and `reserved_at` use the successful producing command's single
captured timestamp under section 5.5 and therefore distinguish otherwise repeated events. A
producer must reject a final-name collision; it must never replace the object
or choose another ID.

# 3. Root roles and evidence-location split

Only the four Topic-1 roles exist:

- `CANDIDATE_ROOT` contains candidate immediate children. Topic 2 may acquire
  and validate the root itself but must not enumerate children or open their
  data forks.
- `PRIVATE_OUTPUT_ROOT` contains a Topic-3 MAP, its Topic-2 binding, and later
  the Topic-4 RECORD. Topic 2 creates only bindings there.
- `CUSTODY_RECORD_ROOT` contains preflights, path maps, freeze reviews, run
  authorizations, consumption reservations, and later completion/events.
- `IMPLEMENTATION_REVIEW_ROOT` contains the explicitly supplied reviewed
  source files, build-command files, A/B executable inputs, executable review,
  review-identities projection, and executable authorization.

Every authority artifact is a regular file placed as one immediate child of
its assigned root. Its explicit output pathname must consist of an explicitly
supplied root path plus one nonempty basename; nested output directories,
`.`/`..`, symlink parents, and final symlinks are forbidden. Topic-2 final
files and temporaries use mode `0600`. Existing roots are never created,
renamed, chmodded, repaired, or discovered by these commands.

The four terminal roots must be directories acquired no-follow and must be
pairwise distinct by `(terminal_device, terminal_inode)`. In addition, no
root's terminal identity may equal any nonterminal chain element of another
root. This prevents nesting and aliasing of authority domains. A failure is
`ROOT_SEPARATION_FAILED`.

An output parent is valid only if its freshly acquired `root_identity_v1`
equals the authorized component for the assigned role and its terminal
descriptor is the parent used for all temporary/final operations. Otherwise
the result is `OUTPUT_PARENT_UNAUTHORIZED`. The final basename and absolute
path locate an object but are never durable authority.

Validation accepts only the explicitly supplied object. It never scans a root,
chooses a matching basename, follows a remembered path, or substitutes another
copy. A supplied same-root alternate immediate-child basename is allowed when
the current parent has the required identity and the object's canonical bytes,
digests, and complete correlations validate. No pathname-history conclusion
follows.

# 4. Root identity and root-chain authority algorithm

## 4.1 Path grammar

A root input is an absolute, nonempty filesystem byte string of at most 4096
bytes, begins with `/`, contains no NUL, has no empty component except the
leading separator, and contains neither `.` nor `..`. A trailing `/` is
allowed only for `/`. Violation is `PATH_GRAMMAR_INVALID`.

## 4.2 Acquisition

For each explicit root path, in command-line order:

1. Open `/` as a directory with no-follow semantics and retain its descriptor.
2. Snapshot its device, inode, kind, and permission mode from the descriptor.
3. For every path component, open that component relative to the retained
   predecessor descriptor with directory-only and no-follow semantics, then
   descriptor-stat it. A symlink, non-directory, failed component acquisition,
   malformed chain, or stat failure is `ROOT_CHAIN_INVALID`.
4. Retain every descriptor until the command's last root recheck. Record for
   each node its zero-based index, device, inode, exact `DIRECTORY` kind, and
   four-octal-digit permission mode.
5. Construct the chain projection exactly as ASCII:

```text
OS1_ROOT_CHAIN_PROJECTION_V1\n
role=<ROLE>\n
count=<canonical decimal node count>\n
node=<index>,device=<u64s>,inode=<u64s>,kind=DIRECTORY,mode=<octal4>\n
```

   The final `node` line repeats once per node from `/` through the terminal
   root. Angle-bracket forms denote their Topic-1 canonical lexical values and
   are not literal brackets. No pathname component or basename is included.
6. SHA-256 of exactly that projection is `chain_sha256`. The terminal node
   supplies the other `root_identity_v1` fields.
7. Re-stat every retained descriptor in root-to-terminal order. Device, inode,
   kind, and mode must equal step 4. Discontinuity of device/inode/kind is
   `OBJECT_CHANGED_OR_REPLACED`; a mode-only change is
   `RACE_OR_MUTATION_DETECTED`.

The role line makes equal filesystem chains under different logical roles
produce different chain digests. Root equality for separation ignores role and
uses filesystem identities as stated above.

## 4.3 Validation and rechecks

To validate an authorized root, acquire a fresh identity by 4.2 and compare
the complete component, including role and `chain_sha256`, to the supplied
authorized component. Inequality after successful acquisition is
`ROOT_AUTHORITY_MISMATCH`. Before any authority artifact is published, and
again after publication using the still-retained parent descriptor, re-stat
all retained nodes. A later identity discontinuity is
`OBJECT_CHANGED_OR_REPLACED`; another protected change is
`RACE_OR_MUTATION_DETECTED`.

For an explicitly supplied evidence file, open its parent root by this
algorithm, open the supplied immediate-child basename relative to that root
with no-follow semantics, require a regular file, and retain its descriptor
through complete read and final stat. Open failure is `EVIDENCE_OPEN_FAILED`;
nonregular kind is `EVIDENCE_NOT_REGULAR_FILE`; a later device/inode/kind
change is `OBJECT_CHANGED_OR_REPLACED`; another protected stat change is
`RACE_OR_MUTATION_DETECTED`.

Copying byte-valid evidence does not copy historical location authority. A copy
under the correct authorized root may validate observationally if all bytes and
correlations validate; validation creates no new authority, event, ID, use, or
completion. A copy outside the required root fails root/parent authority.

# 5. Custody command contract

There are exactly eight authority-producing Topic-2 commands and eight
read-only validators. There is no cleanup, deletion, recovery, or resume
command. Options occur exactly once in the listed order; there
are no optional arguments, aliases, environment fallbacks, config files, or
positional arguments. Any defect is `INVALID_INVOCATION`. Every file/root is
explicit. Success and failure output obey Topic 1.

## 5.1 Producing commands

```text
phoenix-os1-custody freeze-executable-review \
  --implementation-review-root ROOT --helper-source FILE --custody-source FILE \
  --helper-build-command FILE --custody-build-command FILE \
  --helper-executable-a FILE --helper-executable-b FILE \
  --custody-executable-a FILE --custody-executable-b FILE --review-output FILE

phoenix-os1-custody verify-executables \
  --implementation-review-root ROOT --review-identities FILE \
  --helper-executable FILE --custody-executable FILE --authorization-output FILE

phoenix-os1-custody preflight-roots \
  --implementation-review-root ROOT --candidate-root ROOT \
  --private-output-root ROOT --custody-record-root ROOT \
  --executable-authorization FILE --custody-source FILE \
  --custody-executable FILE --preflight-output FILE

phoenix-os1-custody create-path-map \
  --executable-authorization FILE --preflight FILE \
  --candidate-root ROOT --private-output-root ROOT \
  --custody-record-root ROOT --implementation-review-root ROOT \
  --path-map-output FILE

phoenix-os1-custody bind-map \
  --executable-authorization FILE --preflight FILE --path-map FILE \
  --map FILE --implementation-review-root ROOT \
  --custody-record-root ROOT --private-output-root ROOT --binding-output FILE

phoenix-os1-custody freeze-map-review \
  --preflight FILE --path-map FILE --map FILE --binding FILE \
  --candidate-root ROOT --private-output-root ROOT \
  --custody-record-root ROOT --review-verdict APPROVED|REJECTED \
  --freeze-review-output FILE

phoenix-os1-custody authorize-run \
  --executable-authorization FILE --preflight FILE --path-map FILE \
  --binding FILE --freeze-review FILE --map FILE \
  --implementation-review-root ROOT --helper-executable FILE \
  --custody-executable FILE --candidate-root ROOT \
  --private-output-root ROOT --custody-record-root ROOT \
  --run-authorization-output FILE

phoenix-os1-custody reserve-run \
  --run-authorization FILE --executable-authorization FILE \
  --preflight FILE --path-map FILE --map FILE --freeze-review FILE \
  --binding FILE --implementation-review-root ROOT \
  --helper-executable FILE --custody-executable FILE \
  --custody-record-root ROOT --candidate-root ROOT --private-output-root ROOT \
  --reservation-output FILE
```

The `FILE` inputs must be explicitly supplied immediate children of the root
required by section 3. The output must be an immediate child of its assigned
root. `freeze-executable-review` creates both its review at `review-output` and
the review-identities sibling named by appending the ASCII suffix
`.identities` to that output basename; the resulting basename must remain
within the Topic-1 `basename_b64u` bound. These two outputs are one publication
transaction as section 6 specifies.

`review-verdict` is command grammar, not serialized extension. `REJECTED`
creates a valid, non-authorizing freeze review. An unrecognized value is
`INVALID_INVOCATION`.

## 5.2 Observational commands

```text
phoenix-os1-custody validate-executable-review --implementation-review-root ROOT --review FILE
phoenix-os1-custody validate-review-identities --implementation-review-root ROOT --review FILE --review-identities FILE
phoenix-os1-custody validate-executable-authorization --implementation-review-root ROOT --review-identities FILE --executable-authorization FILE --helper-executable FILE --custody-executable FILE
phoenix-os1-custody validate-preflight --custody-record-root ROOT --executable-authorization FILE --preflight FILE --candidate-root ROOT --private-output-root ROOT --implementation-review-root ROOT
phoenix-os1-custody validate-path-map --custody-record-root ROOT --preflight FILE --path-map FILE --candidate-root ROOT --private-output-root ROOT --implementation-review-root ROOT
phoenix-os1-custody validate-binding --implementation-review-root ROOT --custody-record-root ROOT --private-output-root ROOT --executable-authorization FILE --preflight FILE --path-map FILE --map FILE --binding FILE
phoenix-os1-custody validate-freeze-review --custody-record-root ROOT --preflight FILE --path-map FILE --map FILE --binding FILE --freeze-review FILE --candidate-root ROOT --private-output-root ROOT
phoenix-os1-custody validate-run-reservation --custody-record-root ROOT --implementation-review-root ROOT --executable-authorization FILE --preflight FILE --path-map FILE --map FILE --binding FILE --freeze-review FILE --run-authorization FILE --reservation FILE --helper-executable FILE --custody-executable FILE --candidate-root ROOT --private-output-root ROOT
```

Each validator applies all registry rows relevant to its supplied objects in
row-number order, except that Topic-1 parse precedence always controls within
one input. It creates and changes nothing. A validator's success means only
that the explicitly supplied instances satisfy the registry at that instant.

All producers validate invocation, paths, input placement/bytes/digests,
correlations, authority, current executables and roots, output absence, then
construct and publish. More specific lifecycle sections define the exact
order. No command searches for an omitted predecessor.

`bind-map` and `validate-binding` acquire roots in argument order:
`IMPLEMENTATION_REVIEW_ROOT`, `CUSTODY_RECORD_ROOT`, then
`PRIVATE_OUTPUT_ROOT`. Authorization is opened no-follow relative to the
retained implementation-review descriptor; preflight and path map are opened
no-follow relative to the retained custody-record descriptor; MAP and binding
are opened no-follow relative to the retained private-output descriptor.
`bind-map` also uses that private-output descriptor for binding publication.
All three current root identities must equal the corresponding components in
the fully validated preflight/path-map/authorization/MAP chain before any
producer output check. Neither command uses a serialized path locator, current
directory, environment value, or inferred parent to acquire a root.

The other fourteen signatures were audited by assigned input/output root.
Each supplies every root needed for descriptor-relative opens: the review
commands need only the implementation-review root; preflight/path-map commands
supply all roles they touch; freeze review supplies custody/private/candidate;
and run/reservation commands supply all four. No additional root argument or
command is required.

## 5.3 Raw executable-authority input acquisition

Every T2 source, build-command, helper-executable, or custody-executable input
uses one closed acquisition sequence:

1. Open the explicit immediate-child basename relative to the retained
   `IMPLEMENTATION_REVIEW_ROOT` descriptor with no-follow semantics. Syscall
   failure is `EXECUTABLE_AUTHORITY_INPUT_OPEN_FAILED`.
2. Descriptor-stat it. Stat syscall failure is `OBJECT_STAT_FAILED`.
3. Require regular-file kind. A successful stat reporting any other kind is
   `EXECUTABLE_AUTHORITY_INPUT_OPEN_FAILED`.
4. Require its complete byte count to fit the Topic-1 `file_digest_v1` bound,
   then completely read, and rewind/reread whenever the owning procedure
   requires another complete pass. Bound, read, seek, reread, or required
   complete-digest-byte acquisition failure is
   `EXECUTABLE_AUTHORITY_INPUT_READ_FAILED`.
5. Snapshot exactly device ID, inode/file ID, object kind, permission mode,
   byte size, modification time, and metadata-change time before reading and
   re-stat those exact fields after acquisition. Access time, birth time, and
   every other stat field are ignored. Later device/inode/kind discontinuity
   is `OBJECT_CHANGED_OR_REPLACED`; another protected change is
   `RACE_OR_MUTATION_DETECTED`.
6. Only after complete stable bytes exist may digest or A/B/current-authority
   comparison begin. Unequal executable/source identity is
   `EXECUTABLE_IDENTITY_MISMATCH`.

These errors never use `EVIDENCE_*`, which remains limited to serialized
evidence. Ordinary I/O never uses `INTERNAL_INVARIANT_FAILED`.

## 5.4 Repository identity acquisition

The only supported repository representation is one normal, non-bare working
tree whose top-level directory is exactly the supplied
`IMPLEMENTATION_REVIEW_ROOT`. Its Git metadata is exactly the immediate child
directory `.git`. A `.git` symlink, non-directory, gitfile, or other indirection
is unsupported. Bare repositories, linked worktrees, submodule-style gitfiles,
alternate metadata directories, environment-selected repositories, and parent
directory discovery are unsupported. A valid Git repository outside this
narrow representation fails `REPOSITORY_IDENTITY_FAILED`.

Repository identity acquisition is exactly:

1. From the retained `IMPLEMENTATION_REVIEW_ROOT` descriptor, open the literal
   immediate child `.git` directory-only and no-follow and retain it. Open
   literal `HEAD` relative to `.git`, no-follow. Open literal directory `refs`
   relative to `.git`, then literal directory `heads` relative to `refs`, each
   directory-only and no-follow. Open literal `main` relative to `heads`,
   no-follow. Any of these open failures is
   `REPOSITORY_IDENTITY_FAILED`; no following step runs.
2. Descriptor-stat `.git`, `HEAD`, `refs`, `heads`, and `main` in that order.
   Any descriptor-stat syscall failure is exclusively `OBJECT_STAT_FAILED`.
   Record for directory objects `.git`, `refs`, and `heads` exactly device ID,
   inode/file ID, object kind, and mode. Record for file objects `HEAD` and
   `main` exactly device ID, inode/file ID, object kind, mode, byte size,
   modification time, and metadata-change time. Access time, birth time, owner,
   group, flags, ACLs, link count, block data, and every other stat field are
   ignored and never compared.
3. Require directory kind for `.git`, `refs`, and `heads`, and regular-file
   kind for `HEAD` and `main`. A successful stat reporting another kind is
   `REPOSITORY_IDENTITY_FAILED`. Before reading, require `HEAD` size exactly 21
   bytes and `main` size exactly 41 bytes; another size is
   `REPOSITORY_IDENTITY_FAILED`.
4. Read `HEAD` completely and require exactly the 21 ASCII bytes
   `ref: refs/heads/main\n`. Read `main` completely and require exactly 40
   lowercase ASCII hexadecimal characters followed by LF. Detached HEAD,
   another symbolic ref, CRLF, missing LF, extra whitespace/bytes, a BOM, and
   abbreviated or uppercase object names are unsupported. Open succeeded and
   stat succeeded, so bound, read, and representation/grammar failure here is
   exclusively `REPOSITORY_IDENTITY_FAILED`. No normalization, case conversion,
   whitespace trimming, refname expansion, or abbreviation is performed. The
   40 characters are the `git_oid_sha1` value.
5. Do not read `packed-refs`, reflogs, config, alternates, replacement refs,
   object databases, hooks, indexes, or worktree administration files. A
   missing loose `refs/heads/main`, including a repository whose main ref is
   available only through `packed-refs`, is unsupported. Reflogs are
   irrelevant. Object existence or reachability is not a separate authority
   claim; the exact loose-ref bytes are the acquired current identity.

Retain all five repository descriptors through the command's V02-034 re-stat,
V02-035 identity comparison, and V02-036 protected-state comparison. V02-034
descriptor-stat syscall failure is exclusively `OBJECT_STAT_FAILED`.
V02-035 compares device ID, inode/file ID, and kind; any discontinuity is
exclusively `OBJECT_CHANGED_OR_REPLACED`. Only if identity is unchanged,
V02-036 compares directory mode and file mode/size/modification time/
metadata-change time; any difference is exclusively
`RACE_OR_MUTATION_DETECTED`. Ignored fields, including access and birth time,
cannot cause rejection. These rechecks run after all input correlations and
immediately before output-parent/final-name checks. No repository recheck is
performed under `REPOSITORY_IDENTITY_FAILED`.

Thus repository precedence is exact: open, wrong kind, bound, read,
representation, resolution, or lexical failure is
`REPOSITORY_IDENTITY_FAILED`; initial or later descriptor-stat syscall failure
is `OBJECT_STAT_FAILED`; later device/inode/kind discontinuity is
`OBJECT_CHANGED_OR_REPLACED`; and later protected-field mutation is
`RACE_OR_MUTATION_DETECTED`. Only after a valid stable value exists does its
disagreement with validated predecessor authority become
`EVIDENCE_CORRELATION_MISMATCH`. No ambient `git` process, library search,
configuration, alias, hook, locale, environment, or current-directory behavior
participates.

This acquisition runs exactly in `freeze-executable-review`,
`verify-executables`, and `preflight-roots`. Other Topic-2 producers and all
observational validators consume and correlate already serialized repository
identity where their supplied schemas contain it; they do not independently
reacquire Git identity. Listing a repository error in the registry authorizes
no additional repository access.

## 5.5 Producer timestamp acquisition

Every Topic-2 producer acquires exactly one event timestamp after every
applicable authority/input/output-availability check succeeds and immediately
before constructing its output object. The sole source is one successful POSIX
`clock_gettime(CLOCK_REALTIME, &timespec)` observation. Its epoch is the POSIX
Realtime Clock epoch, 1970-01-01 00:00:00 UTC excluding leap-second encoding;
timezone and civil-time formatting never participate. Resolution is the
returned integral nanosecond component. The exact intermediate values are the
returned signed whole seconds and integral nanoseconds.

The returned seconds must be representable in Topic-1 `i64s`; nanoseconds must
be in `0..999999999`. The producer renders seconds directly as Topic-1 canonical
`i64s` and nanoseconds directly as the Topic-1 canonical JSON integer. It does
not round, truncate, rescale, normalize an out-of-range nanosecond component,
consult another clock, or synthesize a value. Acquisition syscall failure or
any conversion/representability defect is
`TIMESTAMP_ACQUISITION_FAILED`.

The executable-review command's review and review-identities projection share
the single `reviewed_at` observation because the projection copies the review
event; it performs no second timestamp acquisition. Every other producer emits
one object and uses its one observation. A backward or equal clock value is
accepted as observed and creates no ordering claim. There is no clock retry to
obtain a preferred or distinct ID.

A failed invocation never resumes construction or publication. A later fresh
invocation revalidates all inputs and output availability and acquires one new
clock observation; it does not preserve or recover the earlier attempted
object's timestamp. If the new observation and all other typed fields happen
to reproduce the same object digest, deterministic-temp collision rules apply.
Serialized predecessor timestamps and Topic-4 candidate stat timestamps remain
separate domains governed by Topic-1 parsing or later candidate-stat rules.

## 5.6 Normative producer execution sequence

Every authority-producing command executes this sequence. A phase is skipped
only when the command signature supplies no object of that class:

1. parse invocation and path bytes; acquire, retain, and validate every
   explicitly supplied root descriptor in command-argument order;
2. open and completely validate every serialized input in command-argument
   order;
3. acquire raw implementation inputs and the fixed repository representation
   when required;
4. perform all input digest, correlation, verdict, executable, and
   authorization checks;
5. perform the final root-chain, retained-descriptor identity, and protected-
   state rechecks for all inputs;
6. validate the output parent against the assigned authorized root;
7. classify reservation availability or verify every required final basename
   absent;
8. acquire the command's single producer timestamp under section 5.5;
9. construct the complete Topic-1 canonical output object bytes, or both
   review objects from the one shared review event;
10. derive every Topic-1 object ID and `object_sha256` from those complete
    bytes;
11. evaluate every predicate whose subject is a newly constructed output;
12. execute the section 14 publication state machine, review first and
    review-identities second for the two-object review command; and
13. complete the stage-specific final file, root, and reopen verification.

No output-object predicate executes before phase 9. Timestamp acquisition is
after every failure with input, authorization, recheck, parent, or absence
precedence and before any timestamp-dependent construction. Observational
validators execute only the applicable input-side registry rows; they never
execute phases 8 through 13, construct an object, or publish.

# 6. Executable-review lifecycle

`freeze-executable-review` performs:

1. Parse invocation and acquire `IMPLEMENTATION_REVIEW_ROOT` by section 4.
2. Require every source, command, and A/B executable input to be a distinct
   explicitly named immediate-child regular file of that root, opened
   descriptor-relative and no-follow. All eight named inputs must be distinct
   objects; aliasing is `EVIDENCE_CORRELATION_MISMATCH`.
3. Completely read and SHA-256 each source and build-command file and construct
   its `file_digest_v1`. Re-stat before and after reading.
4. Completely read each executable A and B, bounded by the standalone-object
   byte-count integer range, and hash it. The A/B bytes for each executable
   role must have equal byte count and digest. Inequality is
   `EXECUTABLE_IDENTITY_MISMATCH`.
5. The command does not execute builds. The two independently supplied A/B
   executable instances are the deterministic-build evidence. The build-command
   files and source files bind what the independent review approved; custody
   does not infer how they were produced.
6. Acquire the current repository `HEAD` object name by section 5.4. No other
   repository representation or resolver is permitted.
7. Complete every input correlation, then recheck every retained input and the
   root chain; validate both output parents and require both final basenames
   absent.
8. Capture `reviewed_at` once, construct the review and exact identities
   projection, derive both IDs/self-digests, and evaluate all constructed-
   output predicates.
9. Publish the review first and review identities second beneath the retained
    `IMPLEMENTATION_REVIEW_ROOT` using section 14. Review-identities publication
    is the pair's commit point because the preserved authorization interface
    consumes that explicit file. Until the identities directory sync and reopen
    succeed, neither object is usable for executable authorization. If review
    publication succeeds but identities publication fails, the review is named
    nonauthoritative residue and no usable identities input exists. Retry uses
    new absent output names; no command discovers, completes, or replaces the
    partial pair.

Review validation reopens the explicit review, validates canonical bytes and
self-digest, and checks its root. Review-identities validation additionally
requires exact projection equality and `review_sha256` over complete review
bytes. It does not recreate the projection. Stale source/build/executable facts
do not invalidate a historical review by themselves; authorization requires
fresh runtime checks in section 7.

# 7. Executable authorization lifecycle

`verify-executables` is the sole authorization creator:

1. Validate invocation and acquire `IMPLEMENTATION_REVIEW_ROOT`; require its
   complete identity equal
   the identities component.
2. Open the explicitly supplied review-identities as an immediate child of
   that retained root and validate its canonical bytes and self-digest. The
   preserved command signature supplies no review file, so this producer does
   not discover or reopen one. It consumes the review identity and complete
   review digest already closed into the identities object. The paired review
   was required to validate before identities publication; the separate
   `validate-review-identities` command can revalidate the pair explicitly.
3. Open the explicitly supplied helper and custody executables as distinct,
   regular, no-follow immediate children of `IMPLEMENTATION_REVIEW_ROOT`;
   completely hash and re-stat them.
4. Require current root, repository commit, source digest fields, review ID,
   and executable digests equal review-identities. An executable mismatch is
   `EXECUTABLE_IDENTITY_MISMATCH`; other copied-field disagreement is
   `EVIDENCE_CORRELATION_MISMATCH`.
5. Follow section 5.6: recheck inputs and roots, validate the output parent and
   absence, capture `authorized_at`, construct/ID/digest authorization, validate
   the constructed output, and publish it beneath the retained
   `IMPLEMENTATION_REVIEW_ROOT`.

A later validation must supply current
executables; replacement or digest mismatch is
`EXECUTABLE_IDENTITY_MISMATCH`. No ambient `PATH` executable is trusted.

# 8. Preflight and path-map lifecycle

## 8.1 Preflight

`preflight-roots` is the sole preflight creator:

1. Validate invocation and paths.
2. Acquire roots in fixed role order: `IMPLEMENTATION_REVIEW_ROOT`,
   `CANDIDATE_ROOT`, `PRIVATE_OUTPUT_ROOT`, `CUSTODY_RECORD_ROOT`.
   This is exactly their canonical command-argument order. If more than one
   root would fail, the first failure in this order is the sole observed
   `ROOT_CHAIN_INVALID`; no later root is attempted.
3. Apply pairwise and nesting separation.
4. Open authorization under the explicitly supplied implementation-review
   root; validate
   bytes/digest and require its custody source/executable digest fields.
5. Open and completely hash the explicitly supplied custody executable and its
   explicitly configured source file under `IMPLEMENTATION_REVIEW_ROOT`.
   Both filenames are explicit command inputs; neither is inferred from the
   running process or located by digest.

The exact preserved preflight schema requires current custody source and
executable provenance, so both named inputs are required. Any invocation
without them is invalid. Both are explicit immediate children of
`IMPLEMENTATION_REVIEW_ROOT`. Hashes must equal authorization; mismatch is
`EXECUTABLE_IDENTITY_MISMATCH`.
6. Require current repository commit equal authorization.
7. Follow section 5.6: recheck all roots and inputs, validate the output parent
   and absence, capture `preflighted_at`, construct/ID/digest and validate the
   preflight, then publish under `CUSTODY_RECORD_ROOT`.

The preflight is the custody-root bootstrap without an authority cycle. Its
freshly acquired `custody_record_root_identity` is embedded in the object, and
the retained descriptor for that exact root is the publication parent. No
earlier artifact claims custody-root authority; later artifacts must validate
against this preflight.

The custody executable may not infer its own pathname or trust process image
identity; it authenticates the explicit file bytes. Preflight grants no child
enumeration or candidate access.

## 8.2 Path map

`create-path-map` validates authorization, preflight, and all four freshly
acquired roots; requires all IDs, repository/custody fields, roles, and root
components to agree; captures the four exact explicit root path byte strings
only after identity validation; then follows section 5.6, including final
rechecks and output checks before `mapped_at`, construction/ID/digest,
constructed-output validation, and publication under `CUSTODY_RECORD_ROOT`.

The preflight must publish successfully before path-map construction begins.
A path map never authorizes through its path strings. A later alternate path to
the same complete root identity may validate while its serialized locator
remains only the original informational input.

# 9. MAP-binding custody contract

`bind-map` accepts but never produces a Topic-3 MAP. It performs:

1. Acquire `IMPLEMENTATION_REVIEW_ROOT`, `CUSTODY_RECORD_ROOT`, and
   `PRIVATE_OUTPUT_ROOT` in canonical argument order.
2. Open authorization under the implementation-review descriptor and
   preflight/path map under the custody-record descriptor; validate them in
   dependency order and require all three current roots equal their predecessor
   components.
3. Open the explicit MAP as an immediate-child regular file relative to the
   retained private-output descriptor;
   validate size, canonical bytes, schema and self-digest.
4. Require MAP authorization/review/preflight/path-map IDs, source/executable
   digests, candidate/private roots, and entry count to equal the supplied
   chain. Recompute `entries_sha256` over the Topic-1 array substring.
5. Do not interpret ordering as proof of correct enumeration; that is Topic 3.
6. Follow section 5.6: recheck MAP and roots, validate the binding parent and
   final absence, capture `bound_at`, construct/ID/digest and validate the
   binding using the complete MAP digest, copied identities, entry count, and
   entries digest, then publish it as a distinct immediate child under
   `PRIVATE_OUTPUT_ROOT`.

Binding validates the MAP's bytes and authority correlations, not how it was
enumerated. Wrong MAP bytes/digest are `EVIDENCE_DIGEST_MISMATCH`; cross-chain
disagreement is `EVIDENCE_CORRELATION_MISMATCH`.

# 10. Freeze-review lifecycle

`freeze-map-review` is operated by an `INDEPENDENT_REVIEWER` and is the sole
freeze-review creator. The role is procedural and carries no personal identity.

It validates preflight and path map under `CUSTODY_RECORD_ROOT`, MAP and binding
under `PRIVATE_OUTPUT_ROOT`, fresh candidate/private/custody roots, complete
MAP bytes, binding bytes, all correlations, and binding-to-MAP digest. It then
follows section 5.6: final rechecks and output checks precede capture of the
supplied verdict and `reviewed_at`; construction/ID/digest and output validation
precede publication under `CUSTODY_RECORD_ROOT`.

`binding_sha256` is the sole external MAP/binding-pair authority. The review
must not accept another MAP digest, pathname, remembered selection, registry,
or hidden lookup as supplementary pair authority. `APPROVED` may authorize the
next transition; `REJECTED` is valid durable evidence but causes later use to
fail `EVIDENCE_UNAUTHORIZED`. A malformed or mismatched review is fatal and is
not converted to `REJECTED`.

Revalidation authenticates only the supplied review and complete supplied
predecessor chain. It never changes a verdict or creates review authority.

# 11. Run-authorization lifecycle

`authorize-run` is the sole run-authorization creator. It validates, in order:

1. executable authorization and current explicit helper/custody executables;
2. preflight and path map;
3. fresh candidate, private-output, and custody roots;
4. binding and its explicitly supplied MAP;
5. freeze review, requiring `verdict=APPROVED`;
6. every ID, complete-object digest, executable identity, and root correlation.

The explicit MAP and current executable arguments prevent hidden lookup. The
executables must be explicit immediate children of
`IMPLEMENTATION_REVIEW_ROOT`; that root is taken from the validated path map
and freshly acquired. The MAP must be an immediate child of
`PRIVATE_OUTPUT_ROOT`.

After validation it follows section 5.6: final rechecks and output checks,
`authorized_at` acquisition, construction/ID/digest and output validation, then
publication under `CUSTODY_RECORD_ROOT`. One run authorization names exactly one freeze review,
binding, MAP, executable authorization, and candidate-root identity. It grants
no authority to another run or later unrelated evidence.

Any current executable mismatch is `EXECUTABLE_IDENTITY_MISMATCH`; a rejected
review is `EVIDENCE_UNAUTHORIZED`; stale or crossed IDs/digests are
`EVIDENCE_CORRELATION_MISMATCH`; changed roots use section 4 errors.

# 12. Consumption-reservation and one-use lifecycle

`reserve-run` is the sole reservation creator. It validates the explicit run
authorization and binding, their complete digest correlation, the current
explicit helper/custody executables, and all four roots. The remaining
explicitly supplied authority chain establishes
freshness without lookup. All predecessors are validated exactly as for run
validation. No candidate
child is accessed.

`use_ordinal=1` means a run has exactly one permitted reservation. The atomic
boundary is exclusive publication of the canonical
`OS1_RUN_CONSUMPTION.json` final basename under `CUSTODY_RECORD_ROOT`.
`reservation-output` must have exactly that basename. Because all runs in one
custody root share this basename, a custody root supports one active OS1 run;
separate authorized runs require separate preflighted custody roots. This
follows the preserved fixed filename without adding a hidden per-run registry.

Before creation, an existing final basename yields
`RESERVATION_ALREADY_CONSUMED` if it is a canonical reservation for the same
run, or `RESERVATION_UNAVAILABLE` otherwise. An exclusive-rename collision is
`FINAL_RENAME_FAILED` under Topic-1 precedence. Successful rename alone reserves
use operationally; success is returned only after file fsync, exclusive rename,
directory fsync, final root/file verification, and reopen validation all pass.
When the fixed final is absent, `reserve-run` completes the remaining section
5.6 phases: timestamp acquisition, reservation construction/ID/digest,
constructed-output validation, and publication.

The reservation's serialized state remains `RESERVED`. It is immutable. It is
not proof that inspect began or completed. Topic 5 completion is the only
terminal-use evidence, and is outside this design. A failed attempt before
rename consumes nothing; its invocation cannot resume, while a fresh invocation
may proceed only under section 14's new-object rule. A
failure after rename leaves the named reservation conservatively consumed:
retry for that run returns `RESERVATION_ALREADY_CONSUMED`, even if the producer
did not report success. This fails closed across crashes without hidden state.

A copied reservation creates no use. Validation of copied bytes is
observational. Only the exclusively published fixed-name instance under the
authorized custody root reserves the run. Deleting, moving, or overwriting it
is outside the protocol and does not restore authority; validators then fail.

# 13. Binding to freeze to run to reservation authority chain

```text
Topic-3 MAP (published, supplied)
  -> BOUND: canonical MAP + validated authorization/preflight/path-map
  -> FROZEN_APPROVED: supplied MAP/binding pair + independent APPROVED review
  -> RUN_AUTHORIZED: approved freeze + current executable/root chain
  -> USE_RESERVED: exclusive OS1_RUN_CONSUMPTION.json publication
```

Legal transitions require the immediately preceding durable artifact plus all
explicit predecessor inputs needed for complete validation. Terminal Topic-2
output is one valid reservation in `USE_RESERVED`. `FROZEN_REJECTED` is a
terminal review branch and cannot transition to run authorization.

Illegal transitions include binding without a valid MAP chain; freeze without
the exact binding/MAP pair; run authorization from a rejected or mismatched
freeze review; reservation without the exact run chain; a second reservation;
and use of copied bytes as a transition. No transition points to an earlier
state, mutates an existing artifact, or reaches RECORD/completion.

# 14. Publication and crash semantics

Every Topic-2 producer uses this algorithm for each output unless section 6's
two-file ordering applies:

1. Validate output grammar, parent/root authority, and required absence.
2. Derive the temporary basename as exact ASCII `.OS1-TEMP-` followed by the
   output object's 64-hex `object_sha256`. Create that immediate child using
   exclusive descriptor-relative creation, mode `0600`, and no-follow flags.
   There is no random suffix, retry counter, search, or fallback name. Any
   creation failure, including an existing deterministic temp, is
   `TEMP_CREATE_FAILED`.
3. Serialize canonical bytes once, write all bytes, and reject short/failed
   writes with `TEMP_WRITE_FAILED`.
4. File-fsync; failure is `TEMP_FILE_SYNC_FAILED`.
5. Rewind/reopen and completely reread. I/O failure is
   `TEMP_REREAD_FAILED`; byte/schema/digest mismatch is
   `TEMP_CONTENT_MISMATCH`.
6. Recheck predecessor descriptors, roots, output parent, and final absence.
7. Exclusively rename within the retained parent descriptor. A preobserved
   final is `OUTPUT_ALREADY_EXISTS` except reservation rules; any attempted
   rename failure, including collision, is `FINAL_RENAME_FAILED`.
8. Directory-fsync; failure is `FINAL_DIRECTORY_SYNC_FAILED`.
9. Verify the final basename is the renamed regular file with expected mode,
   byte count, device/inode continuity, and no-follow properties. Failure is
   `FINAL_FILE_VERIFICATION_FAILED`.
10. Recheck the root chain. Any acquisition, chain, terminal, parent, or
    authorized-identity failure in this dedicated post-publication root stage
    is `FINAL_ROOT_VERIFICATION_FAILED`, as required by Topic-1 publication
    specialization.
11. Reopen final no-follow and validate exact bytes/schema/digest. Any failure
    is `FINAL_REOPEN_FAILED`.

Before step 7, temporary residue is nonauthoritative. No OS1 command may
unlink, rename, truncate, overwrite, validate as evidence, or otherwise clean
it, and Topic 2 defines no cleanup authority or command. The failed invocation
is terminal and cannot resume. Reinvoking with the same typed object, hence the
same `object_sha256` and temporary basename, deterministically fails
`TEMP_CREATE_FAILED`; an unexpected file, directory, symlink, or other object
at that basename has the same result and is not opened or altered. An absent
temp needs no recovery action. No alternate name is searched or selected.

A later fresh producer invocation is not resumption. It revalidates every
input/root/final-absence predicate, acquires a new timestamp under section 5.5,
constructs a new object, and attempts only the temporary basename derived from
that new object's digest. It may proceed while unrelated old temp residues
exist because it never enumerates or opens them. If it derives an occupied temp
name, it fails `TEMP_CREATE_FAILED`. This is the complete recovery mechanism;
there is no removal operation, manual cleanup allowance, retry counter, or
hidden residue registry.

After step 7, named bytes remain observational residue until every later step
succeeds; they must never be overwritten. A crash after rename, including
before directory fsync, makes that explicit final basename unavailable and
every later invocation naming it fails closed. Reservation specialization in
section 12 still treats its fixed named final conservatively as consumed.

The executable-review pair is published review then identities. Identities are
usable only after their own step 11 succeeds, which mechanically implies the
producer completed review publication first. A partial review without
identities is residue and cannot be authorized. No publication rule here
applies to RECORD, completion, or operational events.

## 14.1 Complete crash/recovery classification

| Interruption point | Resulting state | Later action |
|---|---|---|
| before temp creation | no new file and no authority | fresh invocation may begin after full revalidation and a new timestamp |
| after temp creation or during write | nonauthoritative deterministic temp residue | failed invocation cannot resume; fresh invocation follows the new-object rule above |
| after temp file sync or before rename | nonauthoritative deterministic temp residue | same result; no cleanup exists or is permitted |
| after rename or before directory sync | named observational residue, no successful authority | final basename is permanently unavailable to OS1 producers; fail closed |
| after directory sync or during final file/root/reopen verification | named observational residue unless all remaining stages succeed | final basename is permanently unavailable; validators may only assess an explicitly supplied instance |
| between review and review-identities publication | completed review final plus absent, temp, or named identities residue; no usable pair | whole command cannot resume; a fresh invocation requires new absent review/identities final basenames and follows normal rules |
| around authorization, preflight, path-map, binding, freeze-review, or run-authorization publication | the applicable generic pre-rename temp or post-rename named state above | only a fresh invocation after full validation is possible; an occupied explicit final fails closed |
| around reservation publication before rename | no reserved use; possible nonauthoritative temp residue | fresh `reserve-run` may revalidate and create a newly timestamped reservation object if the fixed final is absent |
| around reservation publication after rename | fixed named reservation residue; use conservatively reserved | later `reserve-run` returns `RESERVATION_ALREADY_CONSUMED` for canonical same-run bytes or `RESERVATION_UNAVAILABLE` otherwise |

Successful authority exists only after the applicable last publication stage
succeeds. Temporary residue never becomes authority. Named post-rename residue
is never repaired, completed, replaced, or removed by Topic 2. Every recovery
decision uses only explicit inputs, the exact final name, and the one temp name
derived for the newly constructed object; it performs no discovery.

# 15. Validator registry

The durable reconciliation review assigns the historically preserved 42-row
validator registry to Topic 2; Topic 6 owns tests for these rows. The following
registry is normative and has exactly 42 closed rows. Rows are predicates, not
test cases. Every failure is fatal; none is an expected candidate rejection.

Legend: `C` means authority-creating only as part of a successful producer;
`O` means observational. A creating command also applies the predicate
observationally to inputs.

| ID | Operation/object | Prerequisite | Exact predicate | Error | Mode | Root | Later dependency |
|---|---|---|---|---|---|---|---|
| V02-001 | Invocation | raw argv | exact command and ordered option grammar | `INVALID_INVOCATION` | O | none | none |
| V02-002 | Path | parsed path option | section 4.1 byte grammar | `PATH_GRAMMAR_INVALID` | O | none | none |
| V02-003 | Root-chain acquisition | each explicit root path, in command-line order | section 4.2 no-follow acquisition/projection succeeds | `ROOT_CHAIN_INVALID` | O | role root | none |
| V02-004 | Root separation | all roots required by the command acquired | pairwise terminal and nonnesting constraints hold | `ROOT_SEPARATION_FAILED` | O | all applicable | none |
| V02-005 | Evidence acquisition | retained assigned-root descriptor + explicit evidence basename | descriptor-relative no-follow open succeeds | `EVIDENCE_OPEN_FAILED` | O | assigned | none |
| V02-006 | Evidence kind | opened serialized input | descriptor kind is regular file | `EVIDENCE_NOT_REGULAR_FILE` | O | assigned | none |
| V02-007 | Evidence bound | regular serialized input | Topic-1 object byte bound is not exceeded | `EVIDENCE_SIZE_LIMIT_EXCEEDED` | O | assigned | none |
| V02-008 | Evidence read | bounded serialized input | exact complete descriptor bytes are read | `EVIDENCE_READ_FAILED` | O | assigned | none |
| V02-009 | UTF-8/BOM | complete evidence bytes | valid UTF-8 and no BOM | `EVIDENCE_UTF8_INVALID` | O | assigned | none |
| V02-010 | JSON | UTF-8 evidence bytes | Topic-1 JSON grammar parses | `EVIDENCE_JSON_INVALID` | O | assigned | none |
| V02-011 | Schema version | parsed object | exact supported schema discriminator | `EVIDENCE_SCHEMA_UNSUPPORTED` | O | assigned | none |
| V02-012 | Closed schema | supported object | exact fields/types/values/cardinality | `EVIDENCE_SCHEMA_INVALID` | O | assigned | none |
| V02-013 | Canonical bytes | schema-shaped object | exact Topic-1 key/lexical/envelope bytes | `EVIDENCE_CANONICAL_BYTES_INVALID` | O | assigned | none |
| V02-014 | Self/reference digest | canonical object | every applicable Topic-1 digest recomputes equal | `EVIDENCE_DIGEST_MISMATCH` | O | assigned | none |
| V02-015 | Cross-object input chain | all serialized inputs digest-valid | all predecessor IDs/digests/counts/fields agree; no constructed output is consumed | `EVIDENCE_CORRELATION_MISMATCH` | O | assigned | none |
| V02-016 | Verdict/state | correlated supplied object | required `APPROVED`/`RESERVED` authority is present | `EVIDENCE_UNAUTHORIZED` | O | assigned | none |
| V02-017 | Root-authority comparison | acquired root + parsed authorized component, when present | complete fresh component equals the authorized component | `ROOT_AUTHORITY_MISMATCH` | O | role root | none |
| V02-018 | Raw authority input open | retained review-root descriptor + explicit source/build/executable basename, when required | descriptor-relative no-follow open syscall succeeds | `EXECUTABLE_AUTHORITY_INPUT_OPEN_FAILED` | C/O | review | T3/T4 current-helper use |
| V02-019 | Raw authority input stat | opened raw authority input | descriptor-stat syscall succeeds | `OBJECT_STAT_FAILED` | C/O | review | T3/T4 current-helper use |
| V02-020 | Raw authority input kind | successful descriptor stat | observed kind is regular file | `EXECUTABLE_AUTHORITY_INPUT_OPEN_FAILED` | C/O | review | T3/T4 current-helper use |
| V02-021 | Raw authority input bytes | regular raw authority input | representable bound and complete read/rewind/reread/digest-byte acquisition succeed | `EXECUTABLE_AUTHORITY_INPUT_READ_FAILED` | C/O | review | T3/T4 current-helper use |
| V02-022 | Review-input separation | all review raw inputs acquired, for `freeze-executable-review` only | source/build/executable filesystem identities are pairwise distinct | `EVIDENCE_CORRELATION_MISMATCH` | C | review | none |
| V02-023 | A/B executables | paired complete executable bytes, for `freeze-executable-review` only | A and B byte count/digest equal per role | `EXECUTABLE_IDENTITY_MISMATCH` | C | review | none |
| V02-024 | Repository opens | a section 5.4 producer requires current repository identity | the five fixed descriptor-relative no-follow opens succeed | `REPOSITORY_IDENTITY_FAILED` | C | review | none |
| V02-025 | Repository initial stats | five repository descriptors from V02-024 | descriptor-stat syscalls succeed in fixed order | `OBJECT_STAT_FAILED` | C | review | none |
| V02-026 | Repository representation | initial repository stats | kinds, sizes, complete bytes, exact HEAD/ref grammar yield one `git_oid_sha1` | `REPOSITORY_IDENTITY_FAILED` | C | review | none |
| V02-027 | Repository continuity | acquired OID + applicable predecessor | exact `repository_commit` equality | `EVIDENCE_CORRELATION_MISMATCH` | C | review | none |
| V02-028 | Current executables | complete runtime bytes + authorization chain, when required | explicit runtime hashes equal authorized hashes | `EXECUTABLE_IDENTITY_MISMATCH` | C/O | review | T3/T4 consume |
| V02-029 | Authority-chain correlations | applicable preflight, path map, authorization, MAP and roots | IDs, provenance, commit, roots, locators and MAP authority fields agree; paths only locate | `EVIDENCE_CORRELATION_MISMATCH` | C/O | custody/private | T3 consumes |
| V02-030 | MAP entries digest | canonical digest-valid MAP, when supplied | recomputed Topic-1 `entries` array digest equals its declared value | `EVIDENCE_DIGEST_MISMATCH` | O | private | T3 produces |
| V02-031 | Lifecycle input/output correlations | complete supplied lifecycle objects; for producers, predecessors only | review projection, binding, freeze, run, or reservation fields equal their applicable supplied authorities; producer output is excluded until V02-042 | `EVIDENCE_CORRELATION_MISMATCH` | C/O | assigned | none |
| V02-032 | Freeze verdict | valid supplied freeze review used downstream | verdict is `APPROVED` | `EVIDENCE_UNAUTHORIZED` | O | custody | none |
| V02-033 | Final root-chain recheck | every retained root after its last input use | descriptor-stat every retained chain node succeeds and its projection can be recomputed | `ROOT_CHAIN_INVALID` | C/O | role root | none |
| V02-034 | Final non-root descriptor stat | every retained non-root input descriptor after its last input use | descriptor-stat syscall succeeds | `OBJECT_STAT_FAILED` | C/O | assigned | none |
| V02-035 | Later object identity | initial and final observations from V02-003/006/019/025 and V02-033/034 | device/inode/kind remain unchanged | `OBJECT_CHANGED_OR_REPLACED` | C/O | assigned | none |
| V02-036 | Later protected state | identity-continuous objects from V02-035 | exactly the section 4, 5.3, or 5.4 protected fields remain equal | `RACE_OR_MUTATION_DETECTED` | C/O | assigned | none |
| V02-037 | Output parent | producer after all input/recheck rows | retained parent equals the artifact's assigned authorized root | `OUTPUT_PARENT_UNAUTHORIZED` | C | output root | none |
| V02-038 | Reservation already consumed | `reserve-run` final exists and is canonical for the same run | no same-run reservation is already named | `RESERVATION_ALREADY_CONSUMED` | C | custody | T4/T5 consume authority, not this producer predicate |
| V02-039 | Reservation unavailable | `reserve-run` final exists but is not canonical for the same run | no conflicting object/residue occupies the fixed final | `RESERVATION_UNAVAILABLE` | C | custody | T4 consumes authority, not this producer predicate |
| V02-040 | Output absence | non-reservation producer after V02-037; or `reserve-run` when fixed final is absent | every required final basename is absent | `OUTPUT_ALREADY_EXISTS` | C | output root | none |
| V02-041 | Producer timestamp | producer after every applicable V02-001..V02-040 row | one section 5.5 clock observation is acquired and representable | `TIMESTAMP_ACQUISITION_FAILED` | C | output root | T3/T5 analogous producers |
| V02-042 | Constructed output integrity | complete object bytes constructed after V02-041 | schema, copied fields, projection, ID, self-digest, and command-specific output correlations exactly satisfy Topic 1 | `INTERNAL_INVARIANT_FAILED` | C | output root | none |

All applicable rows execute once in numeric order. A row applies only under its
explicit command/mode condition; otherwise it is skipped without observation.
V02-005 through V02-014 process each serialized input completely in command-
argument order before the next input. V02-015 through V02-032 consume only
explicit inputs and observations already acquired. V02-033 through V02-036 are
the final preproduction rechecks. Reservation rows are mutually exclusive:
V02-038 applies only to a canonical same-run occupant, V02-039 only to another
occupant, and V02-040 to an absent final. Non-reservation producers execute
V02-040 directly. V02-041 then acquires time; only after it succeeds may the
producer construct bytes and execute V02-042. Publication follows V02-042.

The following dependency table is normative. `P` means all eight producers;
`O` means all eight observational validators; `ER` is
`freeze-executable-review`; `EA` is `verify-executables`; `PF` is
`preflight-roots`; and `RR` is the subset of producers requiring raw current
executables. “Applicable” means the exact command signature supplies the named
object; it never authorizes discovery.

| Row | Required prior observations | Observation produced | Commands/modes | Failure enum |
|---|---|---|---|---|
| V02-001 | raw argv | parsed command/options | P/O | `INVALID_INVOCATION` |
| V02-002 | V02-001 paths | valid path bytes | P/O | `PATH_GRAMMAR_INVALID` |
| V02-003 | V02-002 explicit roots in canonical signature order | retained root descriptors/projections | P/O | `ROOT_CHAIN_INVALID` |
| V02-004 | V02-003 applicable roots | separated roots | P/O when multiple roots | `ROOT_SEPARATION_FAILED` |
| V02-005 | V02-003 root + next evidence basename | opened evidence descriptor | P/O per evidence | `EVIDENCE_OPEN_FAILED` |
| V02-006 | V02-005 | regular evidence | P/O per evidence | `EVIDENCE_NOT_REGULAR_FILE` |
| V02-007 | V02-006 | bounded evidence | P/O per evidence | `EVIDENCE_SIZE_LIMIT_EXCEEDED` |
| V02-008 | V02-007 | complete bytes | P/O per evidence | `EVIDENCE_READ_FAILED` |
| V02-009 | V02-008 | UTF-8/BOM-valid bytes | P/O per evidence | `EVIDENCE_UTF8_INVALID` |
| V02-010 | V02-009 | parsed JSON | P/O per evidence | `EVIDENCE_JSON_INVALID` |
| V02-011 | V02-010 | supported discriminator | P/O per evidence | `EVIDENCE_SCHEMA_UNSUPPORTED` |
| V02-012 | V02-011 | schema-valid object | P/O per evidence | `EVIDENCE_SCHEMA_INVALID` |
| V02-013 | V02-012 | canonical object bytes | P/O per evidence | `EVIDENCE_CANONICAL_BYTES_INVALID` |
| V02-014 | V02-013 | digest-valid object | P/O per evidence | `EVIDENCE_DIGEST_MISMATCH` |
| V02-015 | all applicable V02-014 inputs | correlated predecessor inputs | P/O | `EVIDENCE_CORRELATION_MISMATCH` |
| V02-016 | V02-015 relevant supplied authority | authorized verdict/state | P/O when required | `EVIDENCE_UNAUTHORIZED` |
| V02-017 | V02-003 + authorized component from V02-012/014 | root-authority equality | P/O when component exists | `ROOT_AUTHORITY_MISMATCH` |
| V02-018 | V02-003 review root + explicit raw basename | opened raw descriptor | ER/EA/PF/RR and applicable O | `EXECUTABLE_AUTHORITY_INPUT_OPEN_FAILED` |
| V02-019 | V02-018 | initial raw stat | same as V02-018 | `OBJECT_STAT_FAILED` |
| V02-020 | V02-019 | regular raw object | same as V02-018 | `EXECUTABLE_AUTHORITY_INPUT_OPEN_FAILED` |
| V02-021 | V02-020 | complete stable raw bytes/digest | same as V02-018 | `EXECUTABLE_AUTHORITY_INPUT_READ_FAILED` |
| V02-022 | all ER V02-021 inputs | distinct review inputs | ER | `EVIDENCE_CORRELATION_MISMATCH` |
| V02-023 | V02-022 paired executable bytes | equal A/B identities | ER | `EXECUTABLE_IDENTITY_MISMATCH` |
| V02-024 | V02-003 review root | five repository descriptors | ER/EA/PF | `REPOSITORY_IDENTITY_FAILED` |
| V02-025 | V02-024 | repository initial stats | ER/EA/PF | `OBJECT_STAT_FAILED` |
| V02-026 | V02-025 | valid `git_oid_sha1` | ER/EA/PF | `REPOSITORY_IDENTITY_FAILED` |
| V02-027 | V02-026 + applicable V02-014 predecessor | repository continuity | EA/PF; ER has no predecessor comparison | `EVIDENCE_CORRELATION_MISMATCH` |
| V02-028 | V02-021 + V02-014 authorization | current executable equality | EA/PF/RR and applicable O | `EXECUTABLE_IDENTITY_MISMATCH` |
| V02-029 | V02-014/015/017 applicable chain | correlated root/preflight/path-map/MAP authority | P/O when supplied | `EVIDENCE_CORRELATION_MISMATCH` |
| V02-030 | V02-014 MAP | verified `entries_sha256` | P/O when MAP supplied | `EVIDENCE_DIGEST_MISMATCH` |
| V02-031 | V02-014/015/029/030 applicable lifecycle inputs | correlated supplied lifecycle objects | P/O when supplied | `EVIDENCE_CORRELATION_MISMATCH` |
| V02-032 | V02-031 freeze review | approved freeze | run/reservation producers and applicable O | `EVIDENCE_UNAUTHORIZED` |
| V02-033 | V02-003 roots after V02-032 | final root-chain observations | P/O | `ROOT_CHAIN_INVALID` |
| V02-034 | retained non-root descriptors after V02-032 | final object stats | P/O | `OBJECT_STAT_FAILED` |
| V02-035 | V02-003/006/019/025 initial stats + V02-033/034 | identity-continuous objects | P/O | `OBJECT_CHANGED_OR_REPLACED` |
| V02-036 | V02-035 + initial protected fields | mutation-free objects | P/O | `RACE_OR_MUTATION_DETECTED` |
| V02-037 | V02-017 + V02-033/036 | authorized output parent | P | `OUTPUT_PARENT_UNAUTHORIZED` |
| V02-038 | V02-037 + same-run fixed final | already-consumed classification | `reserve-run` producer only | `RESERVATION_ALREADY_CONSUMED` |
| V02-039 | V02-037 + other fixed-final occupant | unavailable classification | `reserve-run` producer only | `RESERVATION_UNAVAILABLE` |
| V02-040 | V02-037 + absent required final(s) | output availability | P; reservation only if absent | `OUTPUT_ALREADY_EXISTS` |
| V02-041 | all applicable V02-001..040 | one producer timestamp | P | `TIMESTAMP_ACQUISITION_FAILED` |
| V02-042 | V02-041 + canonical construction | validated output bytes/IDs/digests | P | `INTERNAL_INVARIANT_FAILED` |

This table proves every consumed observation is either an explicit command
input or was produced by a lower-numbered row. The only construction boundary
is between V02-041 and V02-042. Producer-output correlations are absent from
V02-015/V02-031 and occur only at V02-042; observational validators instead
validate already supplied objects through V02-005..V02-031. No validator
acquires a producer timestamp or invokes V02-042.

Every validator is non-agentic across all 42 rows: it performs zero create,
write, rename, delete, repair, discovery, or lifecycle transition. An attempted
agency path is an `INTERNAL_INVARIANT_FAILED` program invariant, not a
forty-third evidence predicate.

# 16. Error mapping

Topic-1 parse precedence applies first: UTF-8/BOM, JSON, schema discriminator,
closed schema, canonical lexical bytes, digest, correlation, then authority.
Invocation precedes path grammar; path grammar precedes filesystem access.

| Topic-2 semantic failure | Sole error |
|---|---|
| wrong command/options/arity/order | `INVALID_INVOCATION` |
| invalid explicit pathname bytes | `PATH_GRAMMAR_INVALID` |
| supplied evidence open/kind/size/read failure | corresponding `EVIDENCE_OPEN_FAILED`, `EVIDENCE_NOT_REGULAR_FILE`, `EVIDENCE_SIZE_LIMIT_EXCEEDED`, `EVIDENCE_READ_FAILED` |
| supplied evidence grammar through authority | V02-009 through V02-016 error in Topic-1 precedence |
| raw source/build/executable no-follow open syscall failure | `EXECUTABLE_AUTHORITY_INPUT_OPEN_FAILED` |
| raw source/build/executable descriptor-stat syscall failure | `OBJECT_STAT_FAILED` |
| successful raw-input stat reports nonregular kind | `EXECUTABLE_AUTHORITY_INPUT_OPEN_FAILED` |
| raw-input bound/read/rewind/reread/required digest-byte acquisition failure | `EXECUTABLE_AUTHORITY_INPUT_READ_FAILED` |
| current source/runtime executable mismatch, including A/B | `EXECUTABLE_IDENTITY_MISMATCH` |
| section 5.4 fixed repository open, wrong kind, bound/read, representation/resolution, or lexical failure | `REPOSITORY_IDENTITY_FAILED` |
| initial or later repository descriptor-stat syscall failure | `OBJECT_STAT_FAILED` |
| valid current repository identity disagrees with predecessor | `EVIDENCE_CORRELATION_MISMATCH` |
| section 5.5 `CLOCK_REALTIME` acquisition/conversion/range failure | `TIMESTAMP_ACQUISITION_FAILED` |
| root acquisition/shape failure | `ROOT_CHAIN_INVALID` |
| acquired root unequal to frozen authority | `ROOT_AUTHORITY_MISMATCH` |
| overlap/nesting | `ROOT_SEPARATION_FAILED` |
| output parent role/identity failure | `OUTPUT_PARENT_UNAUTHORIZED` |
| pre-rename final collision | `OUTPUT_ALREADY_EXISTS`, except reservation-specific rules |
| later device/inode/kind discontinuity | `OBJECT_CHANGED_OR_REPLACED` |
| later same-object protected change | `RACE_OR_MUTATION_DETECTED` |
| no reservation slot/conflicting fixed-name object | `RESERVATION_UNAVAILABLE` |
| same-run reservation already named | `RESERVATION_ALREADY_CONSUMED` |
| temp create/write/fsync/reread I/O/content failure | exact `TEMP_CREATE_FAILED`, `TEMP_WRITE_FAILED`, `TEMP_FILE_SYNC_FAILED`, `TEMP_REREAD_FAILED`, or `TEMP_CONTENT_MISMATCH` stage |
| attempted exclusive rename failure, including collision | `FINAL_RENAME_FAILED` |
| post-rename directory fsync | `FINAL_DIRECTORY_SYNC_FAILED` |
| final file check, root check, or reopen | exact `FINAL_FILE_VERIFICATION_FAILED`, `FINAL_ROOT_VERIFICATION_FAILED`, or `FINAL_REOPEN_FAILED` stage |
| impossible internal state not covered above | `INTERNAL_INVARIANT_FAILED` |

`EVIDENCE_*` never describes a producer's temp/final. Root-specific errors
exclude object stat errors. Publication stages do not overlap: the first failed
operation fixes the result, and no later operation runs. A collision observed
before rename differs from one returned by the rename exactly as Topic 1
requires. Reservation checks specialize the pre-rename collision only; an
attempted rename collision remains `FINAL_RENAME_FAILED`.

Ordered execution and disjoint semantic domains give one enum per classified
failure. The approved Topic-1 amendment closes U2-001 through U2-003 with the
exact mappings above. The raw-input open/stat/kind/read stages are mutually
exclusive; serialized inputs retain `EVIDENCE_*`; a valid but unequal
repository identity is correlation rather than acquisition failure; and a
serialized malformed timestamp remains an evidence schema/canonical error.

Every command step is invocation, path grammar, serialized-evidence
acquisition/parse, raw-authority input acquisition, repository/timestamp
acquisition, root/object/correlation/authority validation, reservation,
construction, publication, or internal invariant. Each category has an exact
member in this section or an exact stage-specific member in section 14. No
reachable Topic-2 failure lacks a public error and no ordinary I/O uses
`INTERNAL_INVARIANT_FAILED`.

**U2-001 CLOSED**

**U2-002 CLOSED**

**U2-003 CLOSED**

# 17. Determinism and privacy

Given the same typed fields and captured timestamp, producers emit identical
Topic-1 canonical bytes and IDs. Validators use fixed input order, Topic-1
parse precedence, registry order, root acquisition order, and publication
order. Locale, timezone formatting, Unicode normalization, directory iteration,
environment variables, current directory, `PATH`, display names, and generic
JSON serialization cannot affect authority.

Topic 2 serializes only fields already allowed by Topic 1: repository commit;
bounded root identities and private path-map locators; source/build byte counts
and SHA-256; executable SHA-256; schema IDs and complete-object SHA-256;
timestamps; reviewer role/verdict; root roles; entry count/digest copied from a
MAP; and fixed lifecycle state/ordinal. It adds no serialized field.

It never serializes candidate content, new candidate names, arbitrary xattrs,
resource-fork content or size, uncontrolled paths, OS diagnostics, personal or
host data, hidden environment state, or filesystem discovery results. Topic 2
does not enumerate the candidate root or open a candidate child.

# 18. Lifecycle/replay adversarial matrix

| Scenario | Required Topic-2 result |
|---|---|
| Correct complete chain | requested transition succeeds after publication/reopen |
| Wrong executable | `EXECUTABLE_IDENTITY_MISMATCH`; no output authority |
| Stale executable authorization | current executable/commit mismatch rejects; exact error by mismatching predicate |
| Wrong preflight | `EVIDENCE_CORRELATION_MISMATCH` |
| Wrong path map | `EVIDENCE_CORRELATION_MISMATCH` |
| Wrong MAP bytes | `EVIDENCE_DIGEST_MISMATCH`; wrong correlated MAP is `EVIDENCE_CORRELATION_MISMATCH` |
| Wrong binding | digest failure or `EVIDENCE_CORRELATION_MISMATCH` by precedence |
| Wrong/rejected freeze review | correlation error or `EVIDENCE_UNAUTHORIZED` for valid `REJECTED` |
| Wrong run authorization | digest/correlation error by precedence |
| Duplicate reservation | `RESERVATION_ALREADY_CONSUMED` for same run |
| Copied reservation bytes | observationally valid only under correct root/fixed final; creates no new use |
| Alternate basename under correct root | observational validation may pass for non-fixed-name evidence; no new authority |
| Correct bytes under wrong root | `ROOT_AUTHORITY_MISMATCH` or `OUTPUT_PARENT_UNAUTHORIZED` by operation |
| Root substitution | `ROOT_AUTHORITY_MISMATCH` |
| Root race | `OBJECT_CHANGED_OR_REPLACED` or `RACE_OR_MUTATION_DETECTED` by changed property |
| Publication collision before rename | `OUTPUT_ALREADY_EXISTS` or reservation-specialized error |
| Crash before final publication | temp is nonauthoritative and never removed; failed invocation cannot resume; a fresh invocation revalidates, reacquires time, and attempts only its newly derived temp |
| Crash after file fsync before directory fsync | before rename, the same new-object rule applies with no cleanup; after rename, named residue blocks every invocation using that final basename |
| Validator on copied byte-valid evidence | may report valid only if explicitly supplied under correct root; creates nothing |

# 19. Topic-2 gap closure table

| Gap | Disposition | Justification |
|---|---|---|
| G-002 | `CLOSED BY TOPIC 2` | Eight producing and eight observational commands are the complete command set; arguments, ordering, inputs, outputs, validation, publication, fail-closed no-cleanup recovery, and 59-member errors are exact. |
| G-003 | `CLOSED BY TOPIC 2` | Raw review inputs, A/B evidence, narrow loose-main repository resolution, realtime timestamp acquisition, review/projection creation, commit-point publication, and revalidation are exact. |
| G-004 | `CLOSED BY TOPIC 2` | Authorization creator, explicit current executables, review-identities consumption, correlations, placement, stale handling, and errors are exact. |
| G-005 | `CLOSED BY TOPIC 2` | Four-root acquisition/separation, custody provenance, exact repository/time acquisition, bootstrap, preflight/path-map creation, validation, publication, and errors are exact without candidate enumeration. |
| G-009 | `CLOSED BY TOPIC 2` | The custody binding creator validates an explicitly supplied Topic-3 MAP, constructs the closed Topic-1 binding, and publishes/validates it without defining MAP production. |
| G-010 | `CLOSED BY TOPIC 2` | Independent creator, sole `binding_sha256` pair authority, verdict, validation, publication, rejection, and revalidation are exact. |
| G-011 | `CLOSED BY TOPIC 2` | The exact explicit chain, approved-freeze rule, issuance, one-run scope, correlations, placement, stale rejection, and errors are defined. |
| G-012 | `CLOSED BY TOPIC 2` | Fixed-name exclusive reservation, ordinal/state, one-use, no-cleanup crash/fresh-invocation behavior, copy/replay, validation, and errors are exact without claiming completion. |
| G-013 | `CLOSED BY TOPIC 2` | Exact no-follow chain projection, acquisition, rechecks, alternate basename and errors are defined. |
| G-014 | `PARTIALLY CLOSED — later-topic portion explicitly identified` | All Topic-2 placement/identity enforcement is closed; MAP/RECORD/completion/event producer enforcement remains Topics 3–5. |
| G-028 | `PARTIALLY CLOSED — later-topic portion explicitly identified` | Topic 2 closes exactly 42 dependency-ordered normative validator rows and all Topic-2 mappings against the approved 59 members; deterministic fixtures/assertions remain exclusively Topic 6. |

No Topic-2-owned implementation-critical gap remains open. The only partial
classifications name production enforcement or conformance-test work already
assigned to later topics.

# 20. Forward-compatibility audit

Topics 3–6 can consume the object/lifecycle design using existing Topic-1
schemas, the approved 59-member vocabulary, and explicit files/roots. No new
serialized object or Topic-2 field, changed canonical grammar/digest domain,
additional public error, broader privacy access, or hidden authority state is
required.

The supported repository subset, realtime timestamp rule, and fail-closed
no-cleanup publication recovery are complete Topic-2 procedures. Later topics
that already have an allocated producer timestamp use the same section 5.5
clock/encoding rule; error applicability authorizes no new operation. Topics
3–5 need no cleanup command or residue lookup.

Topic 3 supplies a MAP to `bind-map`; it owns enumeration, ordering,
construction, and MAP publication. Topic 4 consumes the validated chain and
reservation; it owns candidate access and RECORD production. Topic 5 consumes
the same explicit chain for terminal publication. Topic 6 converts the 42 rows
and later-topic rules into deterministic fixtures and assertions.

# 21. Implementation-readiness audit

## 21.1 Eight-producer walkthrough

Each row below expands section 5.6; “publish” includes every section 14 stage
and final verification. Failure before rename follows the immutable-temp rule;
failure after rename leaves a blocking named residue.

| Producer | Roots/descriptors and validated inputs | Final preconstruction work | Constructed output and completion |
|---|---|---|---|
| `freeze-executable-review` | review root; eight raw inputs; fixed repository objects | A/B and repository checks; final rechecks; two parents/finals; one `reviewed_at` | review and identities projection, both IDs/digests/predicates; publish review then identities |
| `verify-executables` | review root; identities; two current executables; fixed repository objects | projection authority, executable and repository continuity; rechecks; parent/final; `authorized_at` | executable authorization bytes/ID/digest/predicates; publish |
| `preflight-roots` | roots in review, candidate, private, custody order; authorization; custody source/executable; fixed repository objects | separation, executable/repository/root correlations; rechecks; custody parent/final; `preflighted_at` | preflight bytes/ID/digest/predicates; publish |
| `create-path-map` | four roots; authorization and preflight | root/path/provenance correlations; rechecks; custody parent/final; `mapped_at` | path-map bytes/ID/digest/predicates; publish |
| `bind-map` | review, custody, private roots; authorization, preflight, path map, MAP | MAP digest/authority correlations; rechecks; private parent/final; `bound_at` | binding bytes/ID/digest/predicates; publish |
| `freeze-map-review` | candidate, private, custody roots; preflight, path map, MAP, binding | pair/root correlations and supplied verdict; rechecks; custody parent/final; `reviewed_at` | freeze-review bytes/ID/digest/predicates; publish |
| `authorize-run` | four roots; authorization, preflight, path map, MAP, binding, freeze review, two current executables | approved freeze and complete chain; rechecks; custody parent/final; `authorized_at` | run-authorization bytes/ID/digest/predicates; publish |
| `reserve-run` | four roots; run authorization and complete predecessor chain; two current executables | complete chain, one-use classification, rechecks, custody parent/fixed-final absence; reservation timestamp | reservation bytes/ID/digest/predicates; exclusive fixed-name publication |

Every root and evidence descriptor in this table is supplied by the command
signature; none is discovered. Repository acquisition occurs only in the first
three named producers. Construction follows timestamp acquisition in every
row, and object digest derivation follows complete construction.

## 21.2 Eight-observational-validator walkthrough

| Validator | Supplied roots/evidence | Terminal observation |
|---|---|---|
| `validate-executable-review` | review root and review | canonical review and root-bound validity |
| `validate-review-identities` | review root, review and identities | exact projection and pair validity |
| `validate-executable-authorization` | review root, identities, authorization and two current executables | authorization plus current-executable continuity |
| `validate-preflight` | custody, candidate, private and review roots; authorization and preflight | four-root/provenance/preflight validity |
| `validate-path-map` | custody, candidate, private and review roots; preflight and path map | root/path-map correlation validity |
| `validate-binding` | review, custody and private roots; authorization, preflight, path map, MAP and binding | complete binding/MAP authority validity |
| `validate-freeze-review` | custody, private and candidate roots; preflight, path map, MAP, binding and freeze review | pair/root/review validity; no verdict creation |
| `validate-run-reservation` | four roots; full supplied lifecycle chain, reservation and two current executables | complete run/reservation/current-authority validity |

Each observational command executes its applicable V02-001 through V02-036
rows, including the closing instantaneous root/object continuity checks. It
skips producer-only V02-037 through V02-042. It does not acquire a producer
timestamp, construct or publish authority, enumerate a root, select another
basename, or clean/modify any object.

- Unresolved Topic-2 implementation-critical decisions: **0**.
- Contradictions with Topic 1: **0**; no schema, field, bytes, digest, error,
  privacy rule, or RECORD limit changes.
- Contradictions with reconciliation: **0**; preserved historical requirements
  are retained only as recorded.
- Topic-3/4/5/6 leakage: **0**; only explicit consumer interfaces are defined.
- Hidden-state dependency: **0**; every authority input is explicit.
- Privacy broadening: **0**.
- Threat-model broadening: **0**.

The independent-review findings are closed: registry rows are dependency
ordered; deterministic temp residue has an exact no-removal/new-object rule;
repository identity uses one closed loose-main working-tree representation;
and producer timestamps use one exact realtime-clock acquisition.

Adversarial self-review found no authority or digest cycle: every digest points
to an existing predecessor, and primary IDs omit themselves. Lifecycle is
strictly forward and immutable. No validator searches. Copy/replay cannot
create a transition. Fixed-name exclusive reservation closes duplicate use
without inode/path history. Ordered error stages close ambiguity. Crash states
before and after rename are classified. Root paths locate but never authorize.
No later-topic algorithm or schema change is disguised as custody lifecycle.
The approved Topic-1 amendment closes U2-001 through U2-003, including the
raw-input open/stat/kind/read boundary and all cross-topic allocations.

Corrective implementation remains unauthorized until Topics 1–6 are approved,
durable, and the final authority-chain review explicitly authorizes it.

# 22. Independent-review checklist

- Verify Topic 2 stays within custody/lifecycle/validator scope.
- Compare every referenced object and field against closed Topic 1.
- Verify all eight producers and eight validators have explicit inputs only.
- Verify executable review/authorization and four-root bootstrap are closed.
- Verify root projection is deterministic, no-follow, root-bound, and not
  exact-path-bound.
- Verify binding/freeze/run/reservation transitions and illegal transitions.
- Verify fixed-name exclusive one-use and every crash/retry/copy case.
- Count exactly 42 validator rows and verify non-agency.
- Trace every command failure to exactly one of the approved 59 errors.
- Verify publication stages and residue authority classifications.
- Verify deterministic ID/digest behavior and absence of cycles.
- Verify privacy and honest/access-controlled threat model remain unchanged.
- Verify G-002/G-003/G-004/G-005/G-009/G-010/G-011/G-012/G-013 are closed and
  only the stated later-topic portions of G-014/G-028 remain.
- Verify Topics 3–6 need no new schema, field, error, digest, privacy access, or
  hidden state.
- Verify no implementation, reserve/candidate/reference access, or provenance
  activity is present.

**TOPIC 2 DESIGN COMPLETE: YES**

**TOPIC 2 INDEPENDENT REVIEW REQUIRED: YES**

**CORRECTIVE IMPLEMENTATION AUTHORIZED: NO**
