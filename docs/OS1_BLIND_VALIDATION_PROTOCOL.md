# OS1 Blind Cross-Project Validation Protocol

## Status and purpose

This protocol is preregistered for OS1 Gate 1 (generalization beyond `newest
STUFF`) and Gate 5 (blind validation). It was established before candidate #1
was selected or internally inspected. No candidate is selected by this
document. The central question is whether Phoenix behaves conservatively and
correctly on authentic Studio Vision projects not used to develop its parser,
decoder, or authenticated profiles. Export success is additional evidence, not
a requirement for positive validation.

Record evidence separately for identification, container/project parsing,
sequence enumeration, track/event structure, readiness conservatism, natural
export behavior, failure robustness, and source/destination safety.

## Blindness and candidate set

Classify each candidate truthfully:

- **FULLY BLIND** — no expected content or result is known before freeze.
- **REFERENCE-BLIND** — identity/history may be known, but expected structure
  and export results remain concealed.
- **PARTIALLY BLIND** — some expected content is known; useful research
  evidence, but not Gate 5 evidence.
- **NON-BLIND** — expectations or results are known before Phoenix analysis.

Inescapable prior knowledge is recorded as a limitation, never treated as
forgotten. Only FULLY BLIND and REFERENCE-BLIND candidates count toward Gate 5.

Eligibility is decided from external provenance, permission, authenticity
belief, development-history separation, safe read access, and eventual
reference availability. Selection may consider known era/version, size,
sequence count, device history, audio linkage, and other high-level diversity,
but never internal contents. The minimum is **two genuinely distinct logical
projects**; a preferred inexpensive set is **three to five**. Exact duplicates,
recovered copies, autosaves, Save As variants, and archive extractions do not
automatically count as distinct projects; equal hashes are recorded as
duplicates.

## Validation phases

### A — Candidate eligibility

Record external selection rationale, rights, development-history separation,
and blindness class. Stop if the candidate is a development fixture.

### B — Provenance lock

Before Phoenix access, assign a validation ID and record filename, controlled
location, byte size, SHA-256, lock timestamp, provenance description,
original/copy/recovered/archive status, observable Finder metadata, prior-
knowledge limitations, and permission status. Do not modify the artifact after
lock; uncertain provenance invalidates the run.

### C — Concealment/reference quarantine

Keep reference MIDI, Studio Vision listings/screenshots, event lists, device
notes, and forensic notes outside the analyst's access. Record the quarantine
boundary and reveal authority. A pre-freeze leak invalidates the blind run.

### D — Phoenix-only observation

Use the current release checkpoint and normal public/native workflow. Record
identification confidence/evidence, parse result, diagnostics, every reported
sequence name/identity, readiness, limitations/errors, export eligibility, and
profile match. Export only a sequence that naturally reaches Ready; record
output name/path, size, SHA-256, and result. Never force Unsupported material
through internal APIs or add decoding to enrich the report.

### E — Freeze Phoenix result

Before reference reveal, create a timestamped immutable record containing the
candidate hash, Phoenix commit/version, all observations and diagnostics,
outputs/hashes, predictions/classifications, and uncertainty. From the start
of Phase D through classification, no code, profile, parser, fixture,
expected-hash, opaque-byte interpretation, or candidate copy may change. An
operational failure is classified, not silently restarted.

### F — Reveal reference evidence

Only after the freeze may an independent operator reveal Studio Vision, an
export, screenshots, event listings, device setup, or trusted notes. Label each
item as **ON-DISK FACT**, **STUDIO VISION UI OBSERVATION**, **STUDIO VISION
EXPORT OBSERVATION**, **PHOENIX OBSERVATION**, **INFERENCE**, or **UNKNOWN**.
Studio Vision MIDI establishes musical/export behavior, not every internal
binary meaning.

### G — Compare and classify

Compare only what available evidence supports, bound diagnostics, and record
discrepancies by family/track/sequence and safety impact. No tuning is
permitted during the run.

### H — Gate decision

Aggregate candidate distinctness, blindness, provenance, generalized and
non-generalized structures, Unsupported outcomes, false Ready outcomes,
safety outcomes, and research debt into the Gate 1/Gate 5 decision.

## Integrity and reference rules

Record source content size and SHA-256 before analysis, after analysis, and
after any export attempt. A changed data-fork hash or size is a safety failure;
ordinary Finder/filesystem metadata observations are recorded separately.
Phoenix must not write the source. Overwrite, partial/corrupt output, or unsafe
destination behavior is a safety failure.

Before comparing a Studio Vision MIDI export, record its filename/location,
size, SHA-256, export method if known, sequence identity, whether it predates
Phoenix analysis, and any filtering/editing. Parse it independently and use
normalized musical/event reconciliation where appropriate; byte identity is
not assumed.

External audio does not automatically fail OS1. MIDI recovery must not be
described as recovering external media or resurrecting the whole project.
Absent audio-reference discovery remains a documented limitation.

## Discrepancy classes and gate criteria

- **Class 0 — Exact/expected agreement:** passes.
- **Class 1 — Benign representation difference:** within an established
  normalization boundary; passes with explicit documentation.
- **Class 2 — Conservative Unsupported:** safe refusal despite later evidence
  of content; passes the safety objective and creates research debt.
- **Class 3 — Identification/parsing generalization gap:** requires
  investigation, without implying unsafe behavior.
- **Class 4 — False Ready, silent loss, guessed semantics, or materially wrong
  reconstruction:** presumptively release-blocking.
- **Class 5 — Safety/integrity defect:** release-blocking.
- **Class 6 — Reference uncertainty:** unresolved; not a parser or gate claim.

Gate 1 passes only with at least two genuinely distinct projects showing that
exercised common container/sequence behavior is not dependent on newest-STUFF
offsets or identities, unsupported structures remain bounded, and no false
Ready occurs. A new structure may create research debt without invalidating
independently generalized behavior.

Gate 5 passes only with at least two genuinely distinct FULLY BLIND or
REFERENCE-BLIND projects having provenance lock, concealed expectations,
pre-reveal Phoenix freeze, documented post-reveal comparison, no source
mutation, silent loss, guessed Ready, or unsafe output. Export from both is not
required; accurate Unsupported may count positively.

A false Ready is zero-tolerance: record the failed blind outcome before any
corrective profile/parser/decoder work, and treat it as presumptively blocking.
Unsupported is not failure merely because Studio Vision can play/export the
material when Phoenix accurately refuses without mutation or unsafe output.

## Stop conditions and research transition

Stop or invalidate a run on source-hash change, uncertain provenance,
pre-freeze reference leak, development-fixture discovery, ordinary crash or
panic, false Ready, source mutation, overwrite/partial output, or a need to
change production code before classification. A clean bounded parser refusal
or conservative Unsupported result is a valid classified outcome.

After a candidate is locked, observed, frozen, revealed, and classified, new
research may be proposed. Prioritize work that prevents unsafe Ready behavior,
generalizes common structures, improves conservative classification, or
unlocks multiple projects over one-off exportability.

## Validation record template

Use these sections for each run:

1. **IDENTITY** — validation ID, artifact, location, project relationship.
2. **PROVENANCE** — size/hash/time/source type/rights/Finder metadata.
3. **BLINDNESS CLASSIFICATION** — class, known history, limitations.
4. **PRE-ANALYSIS HASHES**.
5. **PHOENIX CHECKPOINT** — commit/version/build/environment.
6. **PHOENIX-ONLY OBSERVATIONS** — file, project, diagnostics, sequences.
7. **READINESS RESULTS** — state, reason, export eligibility.
8. **EXPORT RESULTS** — only natural Ready outputs and hashes.
9. **FREEZE DECLARATION** — timestamp, immutable record/hash, no tuning.
10. **REFERENCE EVIDENCE** — source, rank, kind, provenance.
11. **POST-REVEAL COMPARISON**.
12. **DISCREPANCIES** — class, severity, bounded evidence.
13. **SOURCE-INTEGRITY CHECK** — before/after hashes and sizes.
14. **GATE EVIDENCE** — Gate 1/5 contribution.
15. **RESEARCH DEBT CREATED**.
16. **FINAL CLASSIFICATION**.

## Aggregate OS1 decision

The aggregate record must state candidate count, genuinely distinct logical-
project count, blindness/provenance per candidate, structures generalized and
not generalized, Unsupported outcomes, false Ready and safety outcomes,
research debt, Gate 1 verdict, Gate 5 verdict, whether OS2 may proceed, and
whether targeted reverse engineering must intervene.

## Candidate-selection boundary

Candidate selection is a separate subsequent task. It may use only filenames,
provenance, hashes, high-level historical information, and filesystem metadata.
It must not use internal Studio Vision structure, Phoenix parsing results,
reference MIDI, or expected track/event/channel/Patch/controller information.
Candidate #1 is not selected or inspected by this protocol.

When previously untouched reserve artifacts are used, candidate intake and
selection must first follow
[`OS1_RESERVE_INTAKE_PROTOCOL.md`](OS1_RESERVE_INTAKE_PROTOCOL.md) as the
required preselection firewall. This document remains governing; if the two
protocols conflict, this blind-validation protocol controls.
