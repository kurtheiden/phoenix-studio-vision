# OS1 Untouched Reserve Intake Protocol

## Status and authority

This protocol governs metadata-only intake and neutral selection when OS1 uses
previously untouched reserve artifacts. It is subordinate to
[`OS1_BLIND_VALIDATION_PROTOCOL.md`](OS1_BLIND_VALIDATION_PROTOCOL.md). If the
documents conflict, the blind-validation protocol controls.

Reserve intake ends after independent review of a frozen selection. It does
not authorize Phoenix inspection, Studio Vision access, reference inspection,
candidate copying, or a provenance lock.

## Objective

The intake must identify at least two genuinely distinct logical-project
families that independently have adequate provenance and permission,
affirmative later-reference availability, FULLY BLIND or REFERENCE-BLIND
status, and evidence-bounded DEVELOPMENT-NAIVE status. Selection must not
favor likely Phoenix success. A later conservative Unsupported result remains
useful OS1 evidence.

## Roles and concealment boundary

Use three recorded roles:

1. the owner/provenance authority supplies bounded history, relationships,
   rights, and reference-availability attestations;
2. the metadata intake operator performs only preregistered metadata and
   identity-hash operations;
3. an independent reviewer verifies the firewall, inventory, grouping,
   classifications, representative choices, and complete selection order
   before any provenance lock.

One person may perform multiple roles only when the record explains why the
required independence and concealment remain intact. Reference MIDI, prior
exports, Studio Vision listings/screenshots, content-bearing notes, and
associated audio remain outside the intake workspace and analyst access.
Record the quarantine boundary and future reveal authority.

## Candidate-only enumeration boundary

Before receiving or enumerating a reserve root, require one of:

- owner attestation that the authorized root contains only prospective
  candidate project artifacts and no reference MIDI, exports, screenshots,
  notes, associated audio, or other content-revealing material; or
- a separately prepared candidate-only directory or list established without
  opening or inspecting candidate contents.

Do not recursively wander through unrelated archive material. Begin with
nonrecursive enumeration and recurse only into owner-declared candidate
containers. Do not copy authentic projects merely for intake convenience;
copying requires a separately approved preservation procedure.

## Command and wrapper firewall

Before the reserve path is supplied, freeze and independently review the exact
commands or helper implementation used for intake. The allowlist must emit
only bounded fields and must provide:

- nonrecursive, no-follow enumeration;
- `lstat`-equivalent kind/stat behavior without alias or symlink resolution;
- exactly Finder Type and Creator from the first eight FinderInfo bytes, never
  the full attribute;
- boolean-only FinderInfo, resource-fork, and other approved xattr probes;
- nonverbose complete-data-fork SHA-256;
- neutral artifact IDs in visible command output wherever possible;
- private output destinations; and
- no shell tracing, byte output, previews, or verbose content-bearing errors.

A reviewed helper is preferred when shell composition would be brittle. It may
retrieve metadata internally only as necessary to validate and emit the
allowlisted result; it must discard and never log unrelated FinderInfo bytes,
xattr names/values, or resource-fork size/content. Render Type/Creator safely
so control bytes cannot enter logs. The implementation review must finish
before it receives a reserve path.

Reject unresolved symlinks, aliases, packages, archives, disk images, and
directories as UNCERTAIN rather than following or opening them.

## Permitted observations

For each ordinary regular-file artifact, record only:

- a neutral inventory ID that encodes no project information;
- basename;
- the minimum containing-folder basename necessary for provenance or family
  relationships;
- filesystem kind, data-fork byte size, and available birth, modification,
  metadata-change, and access timestamps;
- FinderInfo availability and exactly classic Finder Type/Creator;
- resource-fork presence as PRESENT/ABSENT/ERROR only when a reviewed
  boolean-only probe exists;
- `OTHER_XATTRS_PRESENT` as YES/NO only when it can be obtained without
  exposing names or values;
- SHA-256 of the complete regular-file data fork;
- bounded owner provenance, relationship, artifact-status, prior-exposure,
  later-reference, and permission attestations; and
- repository-history matches based on already permitted identifiers.

Do not enumerate or record arbitrary xattr names. Never record arbitrary xattr
values, Finder comments, tags, quarantine values, Spotlight attributes,
thumbnails, custom icons, or resource-fork size/content. If a bounded probe
cannot be guaranteed, omit that field.

Finder Type/Creator may support identity and provenance. It must never be used
to predict Phoenix support or alter candidate ranking.

## Content firewall through selection review

Before completion of stages A–K below, prohibit:

- `strings`, printable-string scans, embedded-name extraction, or text
  indexing;
- hex dumps, byte previews, sampling, or direct byte display;
- entropy, histograms, fuzzy/chunk/rolling hashes, signatures, similarity, or
  other content features;
- magic, libmagic, MIME sniffing, Spotlight content inference, Quick Look, or
  content-aware identification;
- Phoenix CLI, Core, native inspection, parsing, identification, diagnostics,
  readiness, profile, or export operations;
- sequence, track, event, channel, Patch, Controller, Tempo/Meter,
  routing/inclusion, OMS/path, device, embedded-filename, or audio-reference
  inspection;
- resource-fork reading, sizing, hashing, preview, or decoding;
- structural comparison, binary diff, `cmp`, or byte-region comparison beyond
  equality inferred from complete-data-fork SHA-256 plus size;
- MIDI/reference correlation, prior export inspection, or candidate-specific
  listening;
- opening a project or reference in Studio Vision or another DAW;
- copying candidate data into source, tests, fixtures, or parser workspaces;
  and
- predicting or ranking likely Phoenix identification, support, readiness, or
  export success.

Do not use a tool whose normal output can expose candidate content. Ambiguity
produces UNCERTAIN and exclusion, never content inspection.

## SHA-256 boundary

Complete-data-fork SHA-256 is permitted solely for artifact identity,
exact-data-fork duplicate detection, development-history correlation, and
deterministic neutral ranking after family representative selection.

Hash through a nonverbose sequential read and emit digest plus neutral ID,
keeping ID-to-path mapping only in the private inventory. Do not display bytes,
derive other signatures/features, compare internal regions, or infer structure
from a digest. Equal size and digest establish an exact data-fork duplicate;
they do not establish equality of uninspected forks or metadata. Different
hashes establish only data-fork nonidentity, never logical-project
independence.

Record kind, size, and timestamps before hashing, then recheck size and digest.
A permitted read may change access time; record that separately and do not
automatically call an access-time-only change source mutation. Any unexpected
data-fork, metadata, xattr, or resource-fork change requires independent review
even when the data-fork digest remains unchanged.

Hash order is a neutral selection key, not content evidence.

## Frozen owner questionnaire and attestation

Before selection, freeze answers supplied to the best of the owner's
knowledge for each artifact and all known copies, variants, and aliases. Ask
only about:

- approximate era and broad employer/client/project context;
- likely family relationships and original/copy/backup/Save As/revision/
  recovered/corrected/derivative/unknown status;
- whether the project may broadly involve MIDI, audio, or samplers;
- whether the owner previously opened, listened to, or exported it in ordinary
  pre-Phoenix use;
- private-validation and separate public-distribution permission;
- affirmative later-reference availability and reveal method; and
- every known form of Phoenix-related handling.

The exposure attestation must explicitly cover committed and uncommitted
Phoenix work; deleted or temporary research; external/private notes; prior
Codex/ChatGPT work; collaborators, contractors, and other machines/accounts
under owner control; Phoenix CLI/Core/native-app execution; strings, hex,
entropy, magic/content identification, structural/binary inspection,
MIDI/reference comparison, readiness/profile work, parser/event/Patch/
Controller analysis, and test/fixture use; and whether any variant entered a
previously exposed Phoenix research collection.

Do not ask for sequence/track/event/instrument/routing details or expected
Phoenix/reference results. Do not ask the owner to refresh memory by opening a
project, reference, note, export, or media. Stop elaboration that crosses this
boundary and record unavoidable prior knowledge honestly.

## Blindness classification

- **FULLY BLIND:** no expected content or result is known before the
  Phoenix-only freeze; permitted metadata and generic authenticity provenance
  may be known.
- **REFERENCE-BLIND:** identity or broad history may be known, including
  filename recognition, era/context, broad audio/sampler recollection, or
  ordinary prior owner listening/export, while expected structure, reference
  details, Phoenix behavior, readiness, profile match, export result, and
  comparison outcome remain concealed.
- **PARTIALLY BLIND:** candidate-specific expected musical, structural, or
  reference facts are known before the Phoenix-only freeze.
- **NON-BLIND:** Phoenix/parser/readiness/export results, result-revealing
  reference knowledge, or prior Phoenix content-derived development is known.

Only FULLY BLIND and REFERENCE-BLIND families qualify. Familiarity is recorded
as a limitation, never treated as forgotten.

## Development-exposure determination

Search current repository text, allowed untracked repository text, all
reachable branches/tags/stashes/commits, renamed/deleted documentation, commit
messages, and explicitly supplied private Phoenix records using filename,
complete SHA-256, permitted paths, and owner-supplied aliases. Inspect only
repository/history context, never candidate content. Generic filename matches
require hash/provenance corroboration and remain uncertain otherwise.

Classify:

- **DEVELOPMENT-EXPOSED:** evidence shows Phoenix/content-derived analysis,
  execution, correlation, fixture/profile/readiness use, or equivalent work.
- **DEVELOPMENT-NAIVE — no evidence of prior Phoenix content-derived use after
  the specified repository/history searches and owner attestation.** This is
  an evidence-bounded conclusion, not absolute proof.
- **UNCERTAIN:** searches, attestation, aliases, generic-name collisions,
  variants, or provenance do not support a safe conclusion.

A metadata-only filename/path/size/hash record is not disqualifying. A
content-exposed variant conservatively contaminates its family unless
independent provenance establishes a genuinely different logical project.
Conflicting or incomplete evidence means UNCERTAIN and exclusion.

## Affirmative reference availability

A qualifying family must have affirmative reference availability before
selection without opening the artifact. The owner must credibly confirm that,
after the Phoenix-only freeze, appropriate reference evidence can be produced
or revealed under the governing protocol. Ordinarily this means either:

- the project can later be opened in an authorized Studio Vision environment
  and a fresh reference MIDI exported after freeze; or
- an already-existing qualifying reference can later be revealed while
  remaining uninspected during intake and Phoenix-only observation.

Do not create or inspect the reference during intake. UNKNOWN, unavailable, or
noncredible reference availability excludes the family from the qualifying
cohort.

## Logical-project grouping

Group before eligibility ranking:

- exact data-fork duplicates count once;
- known copies, archive extractions, backups, autosaves, Save As files,
  revisions, recovered/corrected files, and derivatives remain one family
  unless independent historical provenance affirmatively establishes distinct
  logical projects;
- different hashes, sizes, timestamps, folders, or filenames alone do not
  establish independence;
- similar names do not prove sameness when independent high-level provenance
  establishes different projects; and
- unresolved relationships are UNKNOWN and excluded rather than guessed.

Record every artifact-to-family mapping, split/merge rationale, confidence,
duplicate relationship, and representative choice.

## Deterministic family representative

Choose exactly one representative per eligible family before hash ranking.
Apply this total precedence:

1. owner-attested original artifact;
2. documented bit-preserving or source-preserving copy with retained relevant
   forks and metadata;
3. documented ordinary copy from the earliest independently recorded
   acquisition or provenance record;
4. recovered or corrected copy;
5. unknown-status copy.

Within a tier, use only objective preregistered provenance fields. An
independently documented acquisition/provenance date or order may be used.
Mutable filesystem timestamps are not historical proof. If candidates remain
tied, choose the lowercase complete-data-fork SHA-256 in ascending
lexicographic order. If they have equal data-fork hashes but still differ in
uninspected fork/metadata state, apply the same documented provenance order;
if no objective order resolves them, classify the family UNCERTAIN and exclude
it.

Never use musical/content expectations, predicted Phoenix support, or a
preferred hash outcome. Record the winning tier and tie-break fields so the
independent reviewer can recompute the choice. Any remaining discretion means
UNCERTAIN and exclusion.

## Eligible cohort and neutral selection

The eligible cohort contains only representatives from genuinely distinct,
adequately provenanced families with private permission, safe read access,
affirmative reference availability, FULLY BLIND or REFERENCE-BLIND status, and
the evidence-bounded DEVELOPMENT-NAIVE classification.

Freeze and separately hash the complete private inventory before selection.
Sort every eligible representative by lowercase complete-data-fork SHA-256 in
ascending lexicographic order. Candidate #1 is rank 1 and Candidate #2 is rank
2. Freeze the entire remaining eligible order as fallback ranks. The ordering
is independent of enumeration order and may not be altered using size, Finder
Type/Creator, history suggesting simplicity, expected content, similarity to
known fixtures, or perceived success probability.

If fewer than two qualifying families remain, stop without selecting a
singleton qualifying set. Select Candidates #1 and #2 together before Phoenix
sees either. Successful export is not required.

## Replacement and contamination after selection

Replacement is permitted only for independently demonstrated pre-analysis
eligibility or integrity invalidation of the affected candidate/family. A
dated independent review must promote the next still-eligible family from the
original frozen order. Never rerank. Phoenix success, failure, Unsupported
status, surprise, or musical outcome can never justify replacement.

If contamination is discovered after selection but before a run, preserve the
audit, exclude the affected family, reassess analyst exposure, and promote
only after independent review. If discovered after a Phoenix run, preserve the
run and classify it as nonqualifying when appropriate; do not erase, relabel,
or silently rerun it. Leave unaffected frozen candidates intact, reassess
cross-candidate analyst contamination, and promote from the original order
only because of eligibility invalidation, never because of the result.

Contamination excludes the affected conservative family without automatically
contaminating unrelated families whose contents and expectations remain
concealed.

## Operational privacy

Future intake execution must use restrictive permissions such as `umask 077`
or equivalent and private non-cloud-synced temporary storage. Keep artifacts,
inventories, mappings, provenance, references, and outputs outside the
repository and public logs. Avoid shell tracing, shell-history retention of
private paths, unnecessary terminal/transcript output, and world-readable
temporary names. Prefer neutral IDs in visible output. Bound private log
retention; cleanup or destruction requires separate authorization so audit
evidence is not silently lost.

Private validation permission is not public-distribution permission.
Authentic projects, reference MIDI, music, audio, sensitive filenames/paths,
private provenance, inventories, and generated outputs remain private unless
separately authorized. Open-source regression fixtures must be synthetic or
minimized non-authentic data unless a specific authentic artifact is separately
authorized for publication.

## Automatic and incidental exposure

Use reasonable operator controls to avoid Finder/Quick Look previews,
Spotlight/content indexing, cloud synchronization, backup agents,
antivirus/EDR output, IDE/media-browser previews, shell globbing,
aliases/functions, command substitution, symlink following, verbose errors,
terminal scrollback, logs/audit logs, crash reports, temporary files, pipes,
caches, swap, clipboard, and accidental reference filename/path association.
Record controls that are actually available; do not claim control over OS or
security behavior the operator cannot guarantee.

Background reading that reveals nothing to the analyst, causes no external
disclosure/upload, and causes no mutation is not automatically blindness
contamination. Analyst-visible content, candidate-specific expectation
leakage, external disclosure/upload, or source/metadata mutation is an incident
requiring containment, exposure accounting, and classification review.

## Mandatory execution order

### A. Verify repository and protocol state

Record checkpoint/divergence/status, protocol version/hash, and allowed
preexisting changes. Read the governing and intake protocols completely. Stop
on mismatch.

### B. Establish and freeze the intake boundary

Freeze roles, private storage, quarantine, candidate-only root attestation,
owner questionnaire, exact command/wrapper allowlist, permitted fields,
incidental-exposure controls, and prohibitions. Independently approve the
implementation before receiving a reserve path.

### C. Enumerate permitted metadata only

Use nonrecursive/no-follow behavior, assign neutral IDs immediately, and emit
only allowlisted fields. Do not resolve unsupported filesystem objects.

### D. Identity-hash artifacts

Record pre-read metadata, compute only complete-data-fork SHA-256, recheck
size/digest, and document access-time effects. Group exact data-fork duplicates
provisionally without inferring independence from different hashes.

### E. Collect and freeze owner provenance

Complete the bounded questionnaire, affirmative reference attestation,
permissions, and exposure attestation without asking for content details.

### F. Check development history

Search the specified repository/history/private-record scope using permitted
identifiers. Classify matches as metadata-only or content-derived without
opening candidates.

### G. Group logical families and select representatives

Apply the conservative family rules and deterministic total representative
precedence. Record all decisions; unresolved discretion means exclusion.

### H. Classify eligibility

Assign blindness and evidence-bounded development-history classes. Confirm
provenance, permissions, safe access, family independence, and affirmative
reference availability. Exclude every nonqualifying or uncertain family.

### I. Freeze inventory and entire eligible order

Write the complete private inventory outside the repository, including the
firewall attestation, metadata, hashes, provenance, grouping, representative
calculations, classifications, exclusions, ordered eligible cohort, fallback
order, and quarantine. Compute and separately record its SHA-256.

### J. Select Candidates #1 and #2 together

Apply the frozen ascending-hash ordering. Assign stable neutral candidate IDs.
Stop if fewer than two qualifying families exist.

### K. Independently review selection

Recompute firewall compliance, representatives, eligibility, complete order,
inventory hash, candidate selection, distinctness, permissions, affirmative
reference availability, and quarantine. Resolve issues only through permitted
metadata/provenance; otherwise exclude and recompute strictly from the frozen
rules. Sign and date approval. No Phoenix/reference access occurs.

### L. Begin Candidate #1 provenance lock in a separate task

Only after approval, start a separately authorized provenance-lock task under
the governing protocol. Reverify repository and source integrity. Stage L does
not belong to reserve intake and grants no authority to run Phoenix.

No stage A–K may inspect candidate content.

## Stop and containment conditions

Stop the affected intake or candidate on:

- any strings, hex, preview, entropy, magic, content-derived metadata,
  structural/parser/Phoenix output, or reference exposure;
- a candidate opened in Studio Vision/DAW before the Phoenix-only freeze;
- unresolved alias/symlink/archive/package handling;
- development-history, provenance, relationship, representative, permission,
  reference-availability, or independence ambiguity;
- fewer than two qualifying families;
- unexpected source, metadata, xattr, or fork mutation;
- unauthorized copy, repository placement, disclosure, upload, publication,
  or reference creation;
- checkpoint/protocol/frozen-record mismatch or loss;
- private-path/content leakage through previews, indexing output, logs, shell
  behavior, temporary state, caches, crash reports, clipboard, or associated
  reference names; or
- any attempt at predicted- or observed-result-driven substitution.

Record what was exposed, to whom, by what mechanism, and which family was
affected without reproducing more content. Preserve unrelated quarantine. Do
not silently retry. Independent review decides whether unaffected families may
continue. Source mutation or unauthorized disclosure is a safety incident,
not merely a blindness-classification change.

## Private intake record

Keep candidate-specific inventory, paths, filenames where sensitive, hashes,
owner provenance, permissions, classifications, reference plans, selection,
and audit evidence in a private external record. Repository documentation may
describe this protocol and abstract outcomes only.
