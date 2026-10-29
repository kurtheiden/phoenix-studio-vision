# Padded initial conductor reuse in partial120

Baseline: `87818457013999b89ed95658fd06247d6db105d5`. Implemented 2026-10-29.

## Evidence and implementation boundary

`experimental_export::decode_padded_initial_conductor` extracts the existing
CCA-027 predicate and exact initial Meter/Tempo decoders. Both the older
experimental Note-export builder and `experimental120::build` call this helper.
It proves event-only primary bounds, not complete conductor authority.

Both source-owned primaries must have exactly 38 payload bytes, header `00 01`,
count one and zero bytes at +6..+10. The Meter event is +14..+22 and Tempo is
+14..+21. Each is followed immediately by `87 FF FF 7F FF 2F 00`, then only
zero padding (nine Meter bytes, ten Tempo bytes). The exact initial decoders
check framing and zero initial position. Opaque +10..+14 bytes remain provenance.
Only the validated seven-byte Tempo event reaches the existing Tempo-map decoder.

The previous unpadded/multi-event partial branch is unchanged, including its
existing Meter-header scope, initial-zero requirement and ordering checks.
Removing all ten Tempo padding bytes produces that already-supported unpadded
initial form; removing only part of the padding refuses. There is no scan,
default Tempo, inferred offset, generalized Meter map or additional header rule.

[Experiments 004](TEMPO_EVENT_REPRESENTATION.md) and
[030](METER_EVENT_REPRESENTATION.md) support the bounded initial fields.
[CCA-045/046](BOUNDED_MULTI_EVENT_TEMPO_RECOVERY_CCA_046.md) support counted
Tempo-map timing; they do not authorize subtracting an offset or dropping
Meter/Key changes. [CCA-002](COMPATIBILITY_ENGINEERING_DECISION_CCA_003.md)
still defers complete acceptance of the short secondary `FF FF FF` footer.
[CCA-027](BOUNDED_EXPERIMENTAL_NOTE_EXPORT_IMPLEMENTATION.md) separately permits
conditional initial-conductor interpretation for explicitly incomplete output.
No new experiment or forensic investigation was performed.

`bounded_conductor::check_pair`, complete recovery, authority, semantic association,
Meter/Tempo decoders and timing normalization are unchanged. Unknown secondary
applicability is not declared harmless. Existing track proof/routing, prefix stop,
Patch/SysEx refusal, placeholders, playable-Note gate, readback, source/consent
binding and exclusive publication remain authoritative.

## Authentic five-project comparison

The same five approved inventory files retain their recorded hashes. Recognition
remains 4/5 and semantic enumeration 3/5, with 36 enumerated sequences. Totals for
the two unenumerated files remain unknown. These are previously exposed cases,
not blind validation or proof of independent musical lineages.

| Inventory ID | Complete before/after | Native partial before/after | Native unavailable before/after |
|---|---|---|---|
| project-001 | 0 / 0 | 0 / 0 | Unknown / Unknown |
| project-002 | 0 / 0 | 1 / 12 | 19 / 8 |
| project-003 | 0 / 0 | 0 / 0 | Unknown / Unknown |
| project-008 | 8 / 8 | 0 / 1 | 7 / 6 |
| project-011 | 1 / 1 | 0 / 0 | 0 / 0 |
| Enumerated total | **9 / 9** | **1 / 13** | **26 / 14** |

The 12 newly exposed native partials were actually published through the
production Swift protocol actor, C ABI and JSON route. MIDI and limitations
sidecars match the service-generated artifacts byte-for-byte. Independent SMF
readback in the authentic regression confirms event counts.

Zero-based candidate ordinals below are serialized sequence identities, not
independently distinct compositions. All new rows contain zero Program Changes.

| ID / ordinal | Notes | Controllers | Prior older Note API |
|---|---:|---:|---|
| project-002 / 0 | 64 | 0 | Supported |
| project-002 / 1 | 71 | 0 | Unavailable |
| project-002 / 2 | 225 | 0 | Unavailable |
| project-002 / 3 | 1,577 | 1 | Unavailable |
| project-002 / 4 | 140 | 0 | Unavailable |
| project-002 / 6 | 167 | 0 | Supported |
| project-002 / 7 | 239 | 0 | Supported |
| project-002 / 8 | 473 | 40 | Unavailable |
| project-002 / 10 | 676 | 0 | Unavailable |
| project-002 / 14 | 614 | 0 | Unavailable |
| project-002 / 17 | 2,896 | 2,810 | Unavailable |
| project-008 / 10 | 1,389 | 0 | Unavailable |
| New native-partial total | **8,531** | **2,851** | **12 sequences** |

New native output also contains 3,994 Pitch Bends. The extra 98 Notes beyond the
audit's 8,433 whole-track candidates come from an already-supported bounded
prefix in project-002/17, not a new decoder rule. Three rows (470 Notes) already
worked through the older experimental API; their musical ticks/status/data and
track order are identical. The genuinely additional scope beyond both previous
APIs is nine sequences, 8,061 Notes, 2,851 Controllers and zero Programs.

All native partials together now contain 12,452 Notes, 8,721 Controllers and
15 Programs, versus 3,921 / 5,870 / 15 before. Controller counts follow the
existing output convention, including Bank Select. The nine complete outputs
remain 16,812 Notes / 132 Controllers / 48 Programs. No new complete export is
claimed. Existing older-API outputs and reports also remain byte-identical.

## Remaining refusals and native disclosures

Fourteen enumerated sequences remain unavailable through native complete/partial
recovery: nine Meter-envelope cases, one padded Tempo case outside the shared
header profile, and four ownership/cardinality cases. Absent/multiple Meter,
mixed Meter/Key maps and alternate short-envelope sentinels remain unsupported.
The excluded Meter-header02 padded case has no playable Notes in its paired
Tracks; the existing playable-Note requirement remains independent.

Qualified partial sequences retain independent track restrictions:

- Unsupported single-Instrument Patches: project-002/1, /2, /17 and project-008/10.
- Unresolved primary-only material: project-002/1, /3, /4, /8, /10, /14 and /17.
- Routing-table refusal: project-002/4; ambiguous source order: project-002/8.
- Unsupported SysEx successors: project-002/10; refused contexts/suffixes: /17.
- Zero-tail event-bound refusal on a paired Track: project-008/10.

The native review and sidecar continue to disclose unresolved conductor
header/secondary/index applicability, unknown omissions, mute uncertainty and
terminal duration. Zero unresolved raw track objects is not a complete-conductor
claim. Unsupported Tracks stay labelled placeholders; raw object counts are not
MIDI-event counts. No unknown object is skipped to resume later music. Readiness
and complete-export eligibility remain unchanged; partial output still requires
the explicit incomplete-review action and consent. Exclude-muted remains refused
for this 120-layout partial contract.

## Regression and preservation

Focused tests cover shared exact bounds, canonical versus short-footer forms,
opaque provenance, every primary truncation, counts/timing/terminal/padding
mutations, unsupported headers, hidden Tempo/Meter/Key events, absent/multiple
Meter, compact conductor controls, unchanged multi-event Tempo behavior and
native preview/consent/source-change/collision safety. An opt-in authentic test
pins all five source hashes and checks all 13 native JSON publications/counts.
The existing authentic Sweeper regression now selects its protected sequence
explicitly rather than assuming it is the first qualifying partial sequence.

Strict all-target/all-feature Clippy, formatting, native Xcode build, C/Swift
bridge checks, Swift model decoding and 131 workflow checks pass. The full Rust
suite passes: 834 passed, zero failed, 27 ignored. Three separately selected
authentic checks pass: new native JSON publications, protected Sweeper publication
and the 25-output comparison. Production Swift/C/JSON publication also passes for
all 13 partials. Private artifact hashes are retained in the task validation report.

All 25 protected MIDI outputs, all nine audited complete exports and the existing
Sweeper partial MIDI/report/sidecar remain byte-identical. All five audit sources
and excluded Experiment 007 retain data hashes, checked metadata and full xattr
snapshots. Pre-existing modified/untracked files and private artifacts are
preserved. No SheepShaver, project repair, staging, commit or push occurred.
