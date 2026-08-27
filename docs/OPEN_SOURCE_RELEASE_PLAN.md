# Phoenix Open-Source Release Plan

## Purpose

Phoenix is preparing its first public open-source release, provisionally
labelled **0.1.0-alpha**. Phoenix 0.1 is an experimental recovery tool for
Opcode Studio Vision projects: it identifies and inspects candidates and
conservatively recovers MIDI only for documented authenticated compatibility
profiles. This is a release-scope boundary, not a claim of universal Studio
Vision compatibility.

## Current authenticated baseline

Two supported profiles currently exist: **Ode to Clarke** and **Bells for
her**. Both are provenance-locked to the same `newest STUFF` source project.
Their proofs establish exact supported recovery paths, not cross-project
generalization.

Bells has a complete 14-row structural manifest, ten musical output tracks,
authenticated omissions, fixed authenticated channels, and all three
established Patch output forms. Its generated MIDI has passed full normalized
generated/reference reconciliation with zero unexplained musical discrepancies
within the defined release-velocity boundary. See the authenticated profile,
known-facts, and MIDI reconciliation documentation for detailed evidence.

## Five release gates

### Gate 1 — Generalization beyond newest STUFF

Exercised common parser/decoder behavior must not be accidentally specific to
`newest STUFF`. Universal Studio Vision compatibility is not required.

**OS0 status: SIGNIFICANT GAP.**

### Gate 2 — Safe unsupported behavior

When evidence is insufficient, Phoenix must remain non-ready rather than guess,
silently discard questionable material, or produce unsafe MIDI.

**OS0 status: PASS NOW.** Residual stress, diagnostic, and unsupported-case
hardening may proceed without changing this gate result.

### Gate 3 — Nontechnical usability

A normal Mac user must be able to complete the basic supported workflow without
Rust, Cargo, or Terminal knowledge. The native UI already implements open,
inspect, readiness/limitations, sequence selection, destination selection,
export, and result/failure feedback. Distribution is not yet complete.

**OS0 status: SMALL GAP.**

### Gate 4 — Distribution/package readiness

Phoenix must have documented installation/use, version identity, release
packaging, and an explicit signing/notarization posture suitable for outside
users.

**OS0 status: SIGNIFICANT GAP.**

### Gate 5 — Blind validation

Phoenix must be tested against projects not used to develop the parser/profile
behavior. A safe, accurate Unsupported result is useful validation; a blind
project need not become exportable.

**OS0 status: UNKNOWN / NOT YET EXECUTED.**

Untouched private reserve material may enter OS1 only through the metadata-only
preselection firewall in
[`OS1_RESERVE_INTAKE_PROTOCOL.md`](OS1_RESERVE_INTAKE_PROTOCOL.md). Private
validation permission does not authorize publication of projects, references,
music, provenance, or validation outputs.

## Phoenix Open-Source 0.1 MUST

- inspect local Studio Vision candidates without modifying source files;
- enumerate sequences and expose conservative readiness and limitations;
- export only supported authenticated compatibility-profile sequences;
- preserve destination and no-overwrite safety;
- provide clear structured success/failure behavior;
- document supported and unsupported authenticated scope;
- document the MIDI-only recovery boundary and external-audio limitation;
- publish source, license, build, test, and use instructions;
- document privacy implications of diagnostics and sample submission.

0.1 does not require all 18 `newest STUFF` sequences, arbitrary project
conversion, universal routing/inclusion or Patch decoding, audio recovery,
OMS resurrection, DAW-specific workflows, batch conversion, or drag-and-drop.

## External audio boundary

External Studio Vision audio recovery is not required for 0.1. Phoenix must not
imply that MIDI recovery also recovers externally stored audio. If reference
discovery is not implemented, user-facing documentation must say that external
media remains the user’s responsibility. The reserved `AudioReferenceSummary`
contract seam remains intact; reference reporting is future open-source work.

## Open-source and future commercial boundary

The open project should remain a genuinely useful preservation/recovery engine.
Open responsibilities include identification, inspection, sequence enumeration,
evidence-based readiness, supported MIDI recovery, the basic native UI, Core/
CLI infrastructure, diagnostics, recovery correctness, and future safe audio
reference reporting.

Future commercial layers may add polished workflow/UI, batch and project
management, advanced reports, intelligent instrument remapping, OMS Studio
Setup interpretation, legacy hardware mapping, DAW integrations, and richer
automation. Recovery correctness and provenance policy remain in the open Core;
commercial functionality must layer on top and never be required for basic
recovery. This document makes no licensing or business-model decision.

## OS1 — Cross-project and blind validation

**Purpose:** address Gates 1 and 5. Select permission-cleared authentic
projects not used to develop current parser/profile behavior. Establish
immutable provenance first, conceal expected identities and outcomes, run
Phoenix, then compare with Studio Vision reference information. Test
identification, inspection, safe refusal, and any supported behavior.

**Acceptance direction:** at least two genuinely distinct blind projects yield
conservative, accurate outcomes with no source mutation, silent musical loss,
guessed Ready state, or unsafe output. Unsupported is a successful outcome when
accurate.

**Non-goals:** forcing export, supporting every newest STUFF sequence, broad
speculative reverse engineering, or universal routing/inclusion decoding.

**Relative size:** M.

## OS2 — Public usability and release hardening

**Purpose:** address residual Gate 2 work, Gate 3, and Gate 4 documentation
prerequisites. Add an unsupported/malformed matrix, bounded stress/resource
tests, privacy and diagnostic wording, README workflow/support guidance,
accessibility review, version/About surface, explicit external-audio messaging,
and a tested release build path.

**Non-goals:** audio relinking, OMS/DAW integration, batch workflow, arbitrary
new grammar, or mandatory drag-and-drop.

**Relative size:** M.

## OS3 — 0.1-alpha distribution

**Purpose:** address Gate 4. Freeze release identity and architecture, produce a
release build, decide signing/notarization posture, package a downloadable
artifact, define GitHub Releases/checksums/release notes, and document clean-Mac
installation and use.

**Acceptance direction:** a supported Mac can obtain and launch Phoenix,
inspect a candidate, understand readiness, and safely export an authenticated
supported sequence without Rust/Terminal knowledge.

**Relative size:** M/L.

## Critical path and claims

Cross-project generalization and blind validation are the current dominant
uncertainties. OS1 precedes broad new reverse engineering; OS2 and packaging
preparation may proceed in parallel. All 18 newest STUFF sequences are not a
0.1 prerequisite. Unknown routing, inclusion, 120-byte structures, and extra
event families should be prioritized only from OS1 evidence.

If all gates pass, Phoenix may claim that it is an experimental open-source
tool that identifies and inspects Studio Vision projects and conservatively
recovers MIDI for documented authenticated compatibility profiles. It may claim
source preservation and conservative unsupported behavior where tested.

Phoenix 0.1 must not claim arbitrary or universal Studio Vision conversion,
general routing/inclusion decoding, universal Patch semantics, audio recovery,
DAW resurrection, support for all 18 newest STUFF sequences, or byte-identical
MIDI where the target is normalized musical reconciliation.
