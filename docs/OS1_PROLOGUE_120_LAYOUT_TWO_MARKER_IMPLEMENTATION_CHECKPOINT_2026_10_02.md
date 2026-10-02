# Prologue: bounded two-marker structural observations — 2026-10-02

## Chronology and evidence boundary

SCHOOL PROJECTS supplied the original 120-layout framing and bounded name
observations. Its implementation was frozen at `cd1b8d6`. Prologue was then
tested prospectively, without tuning, and refused with
`MalformedCandidate { record_index: 170 }`. The [refusal checkpoint](OS1_PROLOGUE_120_LAYOUT_PROSPECTIVE_REFUSAL_CHECKPOINT_2026_10_02.md)
was committed and pushed at `5ed7662` before any refusal investigation.

Subsequent bounded investigation found that record 170 satisfied framing,
`75 + 120 × count`, and preceding local guards, but contained `ff ff` at
`[+41, +43)` where the frozen observer required `fe ff`. A seven-record cohort
review then verified every other existing guard, including the first NUL in
`[+23, +41)` and exclusion of 166-layout ambiguity. Eight earlier Prologue
candidates used `fe ff` and passed the frozen observer locally. None of these
15 observed name spans was empty. These are structural observations, not
semantic sequence identities or marker meanings.

Only after those reviews was the exact two-marker extension designed and
implemented. The source reports are temporary artifacts:

- `/tmp/phoenix-prologue-record170-bounded-investigation.txt`
- `/tmp/phoenix-prologue-ffff-cohort-review.txt`
- `/tmp/phoenix-120-layout-two-marker-design.txt`

The original prospective refusal remains valid and is not rewritten as a pass.
This later verification uses Prologue-informed structural evidence; it is not
an untouched prospective generalization test or independent recovery success.

## Implemented boundary

The only acceptance change is exact marker membership in `{fe ff, ff ff}` at
`[+41, +43)`. A crate-private `Observed120MarkerForm` distinguishes `FeFf` and
`FfFf`, with the actual located two-byte source slice retained. All other
framing/count/local guards, immediate type-07 relationship, checked bounds,
name-window behavior and 120/166 ambiguity/mixed-layout refusal remain intact.
Both marker forms may coexist as observations of the same bounded structural
profile. No meaning is assigned to either marker or their ordering.

Diagnostic summaries compute per-form counts from actual observations. No
semantic SequenceContainer, SequenceName, ownership, profile evidence, Ready
state, export capability or publication authority is created by this feature.
The primary 166-profile rejection remains in diagnostics.

## Explicit private implementation verification

Synthetic regressions passed before private evidence checks. Neither authentic
specimen is included in Git. Tests authenticate external size/hash only as
evidence assertions, never as production selection predicates.

| Specimen | Data bytes | SHA-256 | Structural observations |
|---|---:|---|---|
| SCHOOL PROJECTS | 343,875 | `bfd4fa1208e2cd884ec51ccfb1131d5c02723d7c90acafc98bd89597e1a20331` | 15 total; 15 `fe ff`; 0 `ff ff`; 3 empty spans |
| Prologue Master | 33,057 | `b3a3efd8179d3ffe2a5026b3a40eab74da3ce6e674bb06162a176df07389b733` | 15 total; 8 `fe ff`; 7 `ff ff`; 0 empty spans |

Both application-service inspections retain zero semantic sequences, Unknown
readiness and no semantic profile evidence/export authority. Private evidence
tests are ignored by default and run only with explicitly supplied authorized
paths. The unrelated historical Prologue research decoder is disabled.

No COMIC BOOK access, reference reveal, authentic-specimen MIDI export or
musical investigation was involved. Specimen data/observed metadata are
preserved. Marker semantics, object categories, ownership and musical recovery
remain unknown and outside this implementation.
