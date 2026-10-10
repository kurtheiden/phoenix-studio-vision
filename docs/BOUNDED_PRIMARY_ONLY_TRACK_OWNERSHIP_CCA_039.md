# Bounded primary-only Track ownership — CCA-039

2026-10-10. This change adds semantic ownership evidence for bounded primary-only ordinary Tracks in the 120 layout. It does not grant event, routing, conductor or MIDI export authority.

## Evidence and provenance

The original COMIC BOOK is 452,284 bytes, SHA-256 `22a9e2589098b0d475f6e288eeebc9303bed644aa8ce09f9971a0060c9160486`, at `/Users/kurtheiden/Documents/Phoenix Research/Authentic Studio Vision Projects/COMIC BOOK`.

The Archive version is independently identified as 406,294 bytes, SHA-256 `40217bf534e7ac7f01ccab65d5d6f3fc873f08c134095410da2cbf5cef88ac70`, at `/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/MY OLD MUSIC/Archive folder/COMIC BOOK`. Both source hashes were rechecked through the authentic test and preservation snapshot.

[CCA-038](SWEEPER_NATIVE_TRACK_OWNERSHIP_CCA_038.md) records the owner's native Studio Vision Pro observation of original `sweeper / multi`: Instrument `Multi: All`, 46 events, initial Sound Canvas SysEx at `1·1·0`, Seashore Patch PC122 at `1·1·70`, Controllers and later Patches through `57·1·0`. The owner reported no edit or save. Codex used the owner's transcription; screenshots were not independently inspected. This remains the provenance of the native witness. The CCA-038 document is unchanged.

The witness independently corroborates primary `[70358,70919)` as the original multi Track's event record. CCA-038's existing decoder probe consumed the suffix as 15 Patches and 30 Controllers, consistent with 46 events including the native-observed SysEx. Its whole-body SysEx decode still refused. No new decoder or native observation is added here.

Surviving CCA-036/037/038 reports were consulted. CCA-036 measured 42 zero-prefix primaries without adjacent secondaries and 742 nonzero-prefix primaries with adjacent secondaries across original/Archive COMIC BOOK, SCHOOL and Prologue. This is an observed applicability guard, not a decoded pointer or universal format specification. Prior paired ownership and EXP039 exact never-used ZERO-STATE behavior remain the basis of the ordinal bridge.

## Implementation scope

`src/semantic_layout120.rs` retains the existing paired-only `associate_project_120` entry point and adds `associate_semantic_project_120` for broader diagnostic association. Both share the existing observer, framing, prelude, sparse metadata, descriptor/state and overlap logic. `AppService` switches only its supplementary semantic diagnostics call to the broader entry point.

Authority, verified recovery and experimental export continue calling the paired-only entry point. Their code and acceptance predicates are unchanged. This avoids making a newly associated record silently disappear from an export consumer that only knows paired Tracks.

A new `PrimaryOnlyBinding` retains the ordinary descriptor, local primary ordinal, actual primary record, source-located zero field, declared count and count range, bounded body range and raw tail. It has no secondary or event-bound proof. Actual paired Tracks retain `OrdinaryBinding` and `Source120Pair` unchanged. No `TrackRecordPair` or secondary is fabricated for primary-only data.

The association keeps separate paired, primary-only and no-data collections. Merge them by `slot.ordinal` when presenting source Track order; the original descriptor and local primary ordinal are retained. In a neighborhood containing a primary-only unit, the existing paired binding's local ordinal remains its position among all consuming primary units, including the primary-only unit. Previously admitted all-paired neighborhoods retain their exact old ordinals and bounds-probe outcomes.

## Structural preconditions

The extension accepts at most one ordinary primary-only unit in an otherwise admitted neighborhood:

- Existing positive 120-layout observation, supported type07 prelude, sparse metadata, label bounds, special labels and nonoverlapping neighborhood checks must pass.
- Every consuming descriptor gets exactly one primary in source order. Exact ordinary EXP039 state `00 00 04 00 00 04 00 00` consumes none and remains visible. No blank-name, empty-event or Instrument-count shortcut is added.
- Meter and Tempo remain real adjacent primary/secondary pairs. They cannot use the primary-only extension.
- Every present adjacent type29 belongs to its immediately preceding type02. Existing paired zero-prefix records remain paired; zero does not require discarding their secondaries.
- For an absent ordinary secondary, the next framed record must be type02 or the proper type00 terminal, with exact adjacency. The primary must contain at least 21 payload bytes: 14-byte prefix plus seven-byte tail, allowing an empty body.
- The primary payload starts `00 01`; opaque payload bytes `+10..+14` are all zero. Its tail envelope starts FF and ends FF2F00. The three intermediate bytes remain uninterpreted. These are bounded envelope checks, not new event or terminal interpretation.
- Exact consuming-slot/primary cardinality is required before issuing any association. The existing `Cardinality { slots, pairs }` refusal field name remains for compatibility; in broader diagnostics its `pairs` count denotes consuming primary units, paired or primary-only.
- More than one primary-only ordinary unit refuses after cardinality checking. This is a conservative implementation scope, not a proven maximum of the format.
- Foreign/interleaved/orphan records, missing/extra primaries, skipped descriptors, swallowed siblings and ambiguous ownership remain transactional refusals. No scans, retries, borrowed records or partial sequence association are issued.

Sequence names, file hashes, particular offsets, native display names, event signatures and desired musical output are validation assertions only, never production acceptance predicates.

## Positive and negative validation

Original Sweeper candidate 5, root record 210, metadata `[33024,36219)`, neighborhood `[33024,105985)` now yields diagnostic association for 26 consuming primary units. Descriptor ordinal 9 `multi` binds to primary `[70358,70919)`, declared count 46 at `[70365,70369)`, zero field `[70373,70377)`, body `[70377,70912)` and tail `[70912,70919)`. It is explicitly primary-only, with no invented secondary. Descriptor 10 `pipe` retains the following paired primary `[70919,75079)` and secondary `[75079,75340)`.

Archive Sweeper candidate 5, root record 188, metadata `[32842,35917)`, neighborhood `[32842,105545)` has 25 consuming primaries. The same rule binds descriptor 9 `multi` to `[70050,70611)`, count 46, body `[70069,70604)`. That body is byte-identical to the original; CCA-038 also established whole-primary byte equality. The Archive binding is a structural correspondence supported by the original witness, not a separate native confirmation or a claim that both complete projects are identical.

Eight new default tests cover a name-independent structural positive, source ordinals/ranges, export refusal, real zero-prefix pairs versus unexpected orphan/duplicate secondaries, nonzero absent-secondary field, malformed header/prefix/tail, truncation, exact shortage/surplus, multiple interruptions, ZERO-STATE versus near-state, never-used ordering, special exclusions, foreign boundaries/swallowed siblings, and supplementary service diagnostics without readiness or export capabilities. Existing paired/refusal/no-data fixtures are compared between both entry points.

A ninth, explicitly enabled private-evidence test verifies both authentic source hashes and Sweeper associations, exact body equality, all previously accepted paired associations within both COMIC BOOK versions unchanged, both Sweeper #2 shortages, and verified/experimental refusal. Private project bytes are not added to Git. This test is ignored in the ordinary portable suite and was run separately with explicitly supplied source paths.

A zero field with a valid real adjacent pair is preserved because existing paired fixtures establish that behavior. The tested “unexpected secondary” is an orphan or additional secondary outside that valid pair. Treating every zero/present combination as contradictory would break existing paired behavior and exceed the evidence. Nonzero with absent secondary cannot enter the extension.

## Sweeper #2 remains refused

Original candidate 18 `sweeper #2`: 28 descriptors, exact no-data slots 25–27, therefore 25 consuming slots but 24 primaries. Diagnostic association returns `Cardinality { slots: 25, pairs: 24 }`.

Archive candidate 19: 25 consuming slots but 23 primaries. Diagnostic association returns `Cardinality { slots: 25, pairs: 23 }`; nineteen unpaired primaries would also exceed this extension's scope. Count shortage is reported first. No new skip, invented primary or reassignment repairs either shortage.

Both verified and experimental paths retain their previous paired-only `IncompleteNeighborhood` refusals for Sweeper and Sweeper #2. CCA-035's nine ordered Sweeper #2 CC99/98/6 tuples remain local evidence with their original limited interpretation; this change does not decode or consolidate NRPN.

## Export non-authorization and remaining work

Only semantic association and supplementary diagnostics change. No MIDI event decoder, writer, serializer, Instrument routing, SysEx/Patch/Controller/NRPN handling, conductor/terminal interpretation, selection policy or verified/experimental eligibility is changed. The synthetic firewall tests and authentic comparison explicitly verify that primary-only semantic association does not create export proof or capabilities.

The full original multi body still begins with unsupported F0/0B source serialization, including an additional zero that is absent from the native displayed SysEx message. The zero's role is unresolved. Complete SysEx preservation, per-Patch/device/channel routing and translation, other incomplete Patch/event bodies, exact event accounting and whole-sequence conductor/terminal authority remain independent blockers. Notes, durations, attack/release velocities, Controllers, Bend and source ordering elsewhere must remain intact. Sweeper is not export-ready; no silent omission is introduced.

Recommended next action: one focused read-only validation of the multi Track's initial F0/0B envelope and additional zero against existing source SysEx/routing evidence and the native transcription, to determine a defensible byte-preserving source-to-MIDI rule before implementing a decoder extension. Do not revisit established ownership or Sweeper #2 count shortages in that action.

## Validation and preservation

`cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` and `git diff --check` passed. Full suite: 640 passed, zero failed, 11 ignored. The new private authentic test passed separately (one test). Logs are in `/tmp/cca039-{fmt,clippy,tests,authentic}.txt`.

A before/after probe using the surviving pre-change driver and a newly linked identical driver compared original/Archive authority/recovery results, full inspection rows and target experimental preview outcomes: all identical except the intended semantic diagnostic additions. Existing verified reference and experimental export regression tests also pass. No authentic or reference MIDI was created or overwritten by this task.

Only `src/semantic_layout120.rs`, its test file, the one diagnostics call in `src/app_service.rs`, and this new document were changed. All pre-existing owner documents, CCA-038, Xcode user-state files, sources, reference exports and listening copies remain protected. HEAD/main and the clean index remain unchanged; no staging, commit or push occurred. Final preservation results are recorded in `/tmp/phoenix-primary-only-ownership-cca-039.txt`.
