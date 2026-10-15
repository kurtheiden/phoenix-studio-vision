# CCA-060: length-6 FF60 evidence checkpoint

Recorded: 2026-10-15, baseline `02ce790`. Documentation only; no investigation
was repeated. Findings below are recovered contemporaneous records, not newly
reproduced validation or general parser authority.

## Original records

The original October 13 Codex transcript is:

`/Users/kurtheiden/.codex/sessions/2026/10/13/rollout-2026-10-13T14-29-16-01a13573-feaa-79a3-9f15-c5d43944e5a2.jsonl`

One-based JSONL line references:

- 501: corpus/control search and exploratory body counts.
- 508, 522, 533, 540: ANALOG correspondence, consecutive-context discrepancy,
  source ownership and corrected local comparisons.
- 540, 549, 556, 579: Square timing, accounting, ordering and duration checks.
- 608: original CCA-060 final report, 2026-10-13 14:29:07 UTC.
- 654: controlled-experiment feasibility decision, 14:35:19 UTC.

No standalone original report was located. The transcript contains the original
final report and supporting command outputs. The external transcript path is a
provenance locator, not a portable repository dependency.

## Isolated-bridge hypothesis and Square result

For an isolated `FF 60 06` context with payload prefix `57 7F 00`, followed
by a separate timing VLQ and explicit Controller, the proposed clock is
previous event tick + leading interval + successor interval, each once.
Opaque payload bytes are retained; their general semantics are not established.

At original COMIC BOOK byte46644, the record begins
`02 FF 60 06 57 7F 00 7C 7F 21 01 B0`.
Previous tick14272 +2 +1 =14275 matches native CC6=114; exclusion predicts14273.
The following Note37/attack124 starts14280.

Exploratory in-memory Square walking accounts for all6101 source objects:
519 Notes +5550 Controllers +32 contexts. Relative to CCA-059, this would add
455 Notes and4787 Controllers. This was not complete native fidelity:

- Nine zero-duration source Notes end one tick later in native MIDI; the report
  gives source byte47850, onset17879, native ending17880 as an example.
- Native reverses relative Note/Controller order at six shared-tick positions,
  although each family retains its own matching order.

Neither discrepancy authorizes altering source durations or order as part of
FF60 decoding. No production implementation or MIDI export resulted.

## Other COMIC BOOK evidence and counterexample

The source was452284 bytes, SHA-256
`22a9e2589098b0d475f6e288eeebc9303bed644aa8ce09f9971a0060c9160486`.
ANALOG.MID #2 sequence [199337,244167), descriptor9 ANALOG.MID #6,
primary [221483,228292), secondary [228292,228874), body [221502,228285),
was matched by168 Note pitch/attack and1864 Controller number/value sequences.
Both existing merged and multitrack references corroborated the comparisons;
filenames alone did not establish correspondence.

The isolated bridge at leading offset221719/tag221720 gives
30927 +28 +5 =30960 for CC99=1, followed by Note57/attack62 at30964.
EXCLUDE predicts30932. This corroborates isolated INCLUDE in different music,
but remains within the same project lineage.

Consecutive length-6 contexts at tags221680 and221691 contradict unconditional
interval addition: previous tick1385 +first interval29330 +successor5 =30720,
matching native CC6=62. Adding the second interval15360 instead predicts46080.
An exploratory model excluding that second interval matched all168 Note
timings/durations and1864 Controllers against both references. This is a local
hypothesis, not an established context-chain rule. The related The DECISION
passage and archived copies do not provide independent-project validation.

## Search, decisions and anti-rework boundary

CCA-060 searched74 distinct framed project data forks and50 controlled project
files. All66 length-6 explicit-Controller candidates,58 with nonzero successor
intervals, belonged to five COMIC BOOK versions. The controls had no qualifying
bridge. Related versions are not independent project lineages.

Original implementation decision: **NO-GO for a general parser rule**. The
missing evidence was an isolated length-6/prefix57 7F 00 bridge in an unrelated
authentic project, with positive leading and successor intervals and matched
preceding-event, successor-Controller and following-Note timings. Consecutive
contexts must remain refused absent their own evidence.

Original controlled-experiment decision: **NO-GO now**. Existing experiments
019/020/022 establish compensating Note intervals;031 separates length-7 context
framing from successor timing. Routing/mute, SCHOOL and Prologue experiments
do not establish an editing action that reliably creates or preserves the
required unrelated length-6 composition. No reproducible native action was
known; recommended user actions were zero.

CCA-057 Pipe evidence is source-bound. CCA-059 independently validates length7,
not length6. Neither removes the CCA-060 gap. Do not repeat the completed
corpus search, Square analysis or experiment-feasibility review without new
evidence. Do not treat isolated INCLUDE or the local consecutive-context model
as a general parser rule, introduce specimen dispatch into a general decoder,
or describe complete object accounting as exact native recovery.
