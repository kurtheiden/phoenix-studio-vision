# Phoenix SV and Phoenix SV Pro — product strategy

Date: 2026-10-15

Status: Owner-approved direction; documentation only

Baseline: `61986cb`

## Phoenix SV

Phoenix SV is the free, open-source Studio Vision preservation and MIDI
conversion application. The existing native macOS UI remains the intended
interface for version 1.0. Prioritize accurate recovery of intact sequences
from authentic Studio Vision projects, preserving MIDI notes, controllers,
timing, program changes, SysEx, and other supported events as faithfully as
source evidence allows. Clearly identify incomplete or unsupported recovery.
Do not intentionally reserve better decoding fidelity for a commercial edition.

Functional UI improvements are permitted when necessary; cosmetic redesign is
not a current priority.

## Phoenix SV Pro

Phoenix SV Pro is a proposed future advanced studio migration product using
the same core decoding engine as Phoenix SV. Potential capabilities include
historical OMS configuration interpretation, instrument mapping, MIDI routing
reconstruction, modern DAW migration assistance, and external audio dependency
handling. These are possibilities, not committed features or implementation
milestones.

The Pro feature set, licensing, commercial model, and UI remain undecided.
The owner prefers exploring a music-studio-inspired interface rather than
committing to the earlier 3D video converter-inspired concept. No Pro
implementation or UI design work is authorized yet. Do not create a second
product or duplicate the decoder as part of this decision.

## Recovery principle

Sweeper, newest STUFF, SCHOOL PROJECTS, and independent third-party specimens
are evidence for learning the Studio Vision format, not individual products
to repair. Implement decoding rules only with defensible structural evidence
and appropriate validation. Avoid specimen-specific fixes and assumptions.

Successful recovery must reflect actual source content. Never invent events
or silently describe partial recovery as complete. Commercial migration
features must not become a prerequisite for accurate supported MIDI decoding
in Phoenix SV.

## Roadmap and release clarification

The long-term ambition is to recover all valid Studio Vision sequences.
That ambition is not a present compatibility claim or a requirement to claim
universal support at version 1.0. Initial 1.0 compatibility claims must stay
within documented, evidence-supported and appropriately validated recovery
scope, with unsupported cases and partial results stated explicitly.

This decision preserves existing technical milestones, validation gates, and
release-scope decisions in the [roadmap](ROADMAP.md),
[decision log](DECISIONS.md), and
[open-source release plan](OPEN_SOURCE_RELEASE_PLAN.md). The earlier provisional
`0.1.0-alpha` plan remains a separate release stage; this decision neither
relabels it as 1.0 nor declares any release gate passed. Existing deferrals
remain in place; listing a potential Pro capability does not authorize it or
remove an existing open-source responsibility, including safe audio reference
reporting.

The approved direction clarifies the earlier open-source/commercial boundary:
decoding fidelity belongs to the shared open core, while potential Pro value
lies in advanced migration capabilities. No application behavior, repository
name, application name, executable, bundle identifier, or existing asset is
changed by these product labels.
