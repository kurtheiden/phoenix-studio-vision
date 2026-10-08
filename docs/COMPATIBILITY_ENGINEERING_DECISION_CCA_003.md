# Compatibility engineering decision — CCA-003

## Baseline and scope

Decision/consolidation date: 2026-10-08. Baseline HEAD
`710d6fdc417177b371cd80fdaabe4399e12c462f`, branch `main`; initial index empty.
This document is the only authorized repository change. No parser, adapter,
export authorization, tests, specimens, releases or owner files were changed.
No experiment, MIDI export, staging, commit or push occurred.

Initial owner state, preserved verbatim:

```text
 M docs/DECISIONS.md
 M docs/ROADMAP.md
?? docs/CONTROLLED_TRACK3_2_MIDI_CHANNEL_CHANGE.md
?? docs/OS1_TOPIC3_BLOCKER_REVIEW_U3_001_U3_002.md
?? macos/PhoenixApp/PhoenixApp.xcodeproj/project.xcworkspace/xcuserdata/kurtheiden.xcuserdatad/UserInterfaceState.xcuserstate
?? macos/PhoenixApp/PhoenixApp.xcodeproj/xcuserdata/kurtheiden.xcuserdatad/xcschemes/xcschememanagement.plist
```

Frozen locations: `/tmp/phoenix-gsc-rc1-final`,
`/tmp/phoenix-gsc-rc2-final`, and
`/Users/kurtheiden/Documents/Phoenix Research/GameSoundCon 2026/Phoenix 0.1.0 GSC RC2`.
Recorded release/master, repository, owner, authentic specimen and historical
report fingerprints were checked using the existing preservation inventory
`/tmp/phoenix-cca-001-preservation.json`. No release was rebuilt or launched.
The original Prologue and normalized comparison retain their identities in the
[post-PM-R13 checkpoint](PROLOGUE_POST_PM_R13_RECOVERY_RESEARCH_CHECKPOINT.md).

The product objective is general recovery of intact supported Studio Vision
projects without requiring user-side legacy conversion. Unsupported serialization
is not evidence of damage. Complete, faithful MIDI recovery remains mandatory;
no partial-export policy is proposed.

## Verified recovery and limits

- Eight SCHOOL PROJECTS Sequences have verified native-app MIDI outputs and
  complete mechanical readback, including frozen RC2 owner validation. See
  [batch validation](NATIVE_UI_BATCH_RECOVERY_VALIDATION.md) and
  [listening evidence](SCHOOL_PROJECTS_EXPERIMENTAL_LISTENING.md). Comparison to
  Phoenix reference artifacts is not universal equivalence to native exports.
- Ode to Clarke and Bells for her have authenticated newest-STUFF recovery
  evidence. Ode has normalized native-MIDI comparison and persisted/listened
  proof. Bells uses a specific authenticated output policy with two nonempty
  omissions; that is not universal mute/inclusion decoding. See
  [Ode comparison](ODE_MULTITRACK_COMPARISON.md) and
  [profile implementation](AUTHENTICATED_COMPATIBILITY_PROFILE_IMPLEMENTATION.md).
- No Prologue or COMIC BOOK candidate is currently established as completely
  recoverable. Structural parsing, ordinary Track authority and observational
  blocker removal are distinct from production readiness and verified output.
- The 25-file [authentic survey](AUTHENTIC_PROJECT_SURVEY.md) is an inventory,
  not a current decoder sweep or 25 independent recovery tests. Alternate saves
  and Sequence/Segment pairs are not independent projects. Flying high's native
  versus SMF identity remains unresolved in its
  [blind checkpoint](OS1_FLYING_HIGH_BLIND_OBSERVATION_CHECKPOINT.md).
- No listed format refusal establishes confirmed damage. Declared-range failures
  can be parser-profile mismatch as well as genuine truncation; classification
  needs independently established format and bounds.

## Closed decisions: do not reopen without new evidence

**PM-R15 — retain/park:** Frame 1, Frames 7-8 and Frame 11 remain blocked at
conductor Envelope. Their initial meter/tempo event values are identifiable,
but padded primary payload+1 values `0C`, `04`, `03` have unproven conductor
applicability independence. Full historical naming is unnecessary; positive
independence is necessary. Experiment 026's `01 -> 02` co-variation does not
prove all other variants. The current complete-recovery contract forbids a
partial listening MIDI under this uncertainty.

**CCA-002 — defer:** COMIC BOOK ordinals 1 (pick up the pace), 3 (unnamed),
7 (nitemare b4 xmas), 8 (sampler xperimnt) pass recorded ordinary authority,
but all eight Meter/Tempo secondaries append `FF FF FF`. The short envelopes
are 32/31 bytes versus accepted 29/28. Existing acceptance of that footer in
an expanded 40/39-byte manifest envelope does not prove its independence in
this short form. No acceptance widening was justified. Later Patch/order/
terminal/readback gates have not been certified for those four candidates.

Neither decision authorized implementation. Both investigations are parked;
no repeated tempo edit, suffix audit or new experiment is selected here.

## Root-cause families

1. **Ownership:** legitimate absent-pair Tracks versus missing/unmatched pairs.
   SCHOOL experimental/ultra change have 15 consuming slots versus 14/13 pairs;
   eleven Prologue serialized Frame records remain cardinality-blocked after
   observational metadata handling; twelve COMIC candidates have interrupted
   neighborhoods. These are related risk domains, not one demonstrated grammar.
2. **Conductor completeness/applicability:** primary-header variants, short
   secondary suffixes and broader/absent/multiple Meter compositions are separate
   unresolved subclasses. Matching primitives cannot replace complete authority.
3. **Owned musical event/context forms:** ff40, f0, ff20/ff02, triple contexts
   and Patch/context transitions need exact musical effects, timing and counting.
4. **Routing/Instrument/Patch/bank:** bounded same-source rules and authenticated
   profiles do not establish universal channel, bank or inclusion semantics.
5. **Order/terminal coverage:** minuet source-order ambiguity and kickin'dance
   paired-empty terminal grammar remain protected; established non-event terminal
   diagnostics do not authorize ignoring potentially musical structures.
6. **Proven but unimplemented forms:** sparse bounded metadata association and
   explicit canonical C0 Program entries have positive evidence within a narrow
   120-layout scope. These can be engineered without resolving unrelated unknowns.

## Engineering readiness matrix

“READY” means the stated bounded rule can be implemented under existing safety
contracts in a separately authorized task. It does not mean a Sequence becomes
Ready. Counts below distinguish logical Sequences from serialized candidate units;
no counts are summed into a corpus compatibility rate.

| Candidate | Evidence/impact unit | Rule proven? Musical interpretation/regression evidence | Current implementation | Readiness |
| --- | --- | --- | --- | --- |
| Explicit canonical C0 Program entry in independently owned 120 single-Instrument streams | Three first-group named Frame records; nine entries in three initial bodies, 27 in the wider 18-pair observation. Independent logical-object category/count not fully established | Yes within PM-R13 scope: normalized FF7C bridge preserves Program, timing and Notes. Adds known Program restoration, not a new bank/channel interpretation. Authentic bridge plus controlled FF7C evidence; negative production tests currently refuse C0 | Missing from mixed walker/handoff | **READY** for profile-scoped support |
| Sparse additional type07 association in evidenced 120 grammar | 64 records across14 serialized Frame records; not14 proven independent logical Sequences | Yes for Track association: independent inventory, nonzero+85..88 mask, physical/ascending sparse order, prospective visible #90 vs #49 test. Payload semantics unused. Event ownership remains independent; A14/B50/C0 are occurrence classifications | Observational only; production pair-run still refuses interruptions | **READY** for bounded association only, retaining cardinality/authority refusals |
| Prologue conductor primary+1 variants | Three serialized Frame records | Applicability independence missing; timing/playback risk | Refused | **NEEDS ONE SPECIFIC FACT**; parked by PM-R15 |
| COMIC short secondary FF footer | Four candidates | Short-form footer applicability independence missing; expanded form is not equivalent | Short form refused; expanded form supported elsewhere | **NEEDS ONE SPECIFIC FACT**; defer per CCA-002 |
| Absent/unmatched Track-pair ownership | SCHOOL2 named Sequences; Prologue11 serialized records; COMIC12 candidates | Shared rule not proved; different possible omissions/references/structures and musical ownership implications | Existing exact zero-state handling only | **NOT READY** |
| Broader conductor maps, unknown event/context and Patch/routing variants | Multiple SCHOOL/COMIC paths; no certified new complete count | Material timing/state implications unresolved; existing controls/references useful but not a complete rule for each variant | Narrow forms supported | **NOT READY** |
| Longer adjacent type07 preludes, 27/35-byte counted field | SCHOOL3 serialized Sequences; correspondence advances for minuet only | Implemented bounded proof; does not fix other cardinality/events/conductor/order gates | Present in semantic_layout120::supported_prelude and durable bridge record | **ALREADY IMPLEMENTED** |
| Bounded compact/padded initial conductors and secondary+19 provenance | Eight successful SCHOOL outputs provide regression evidence | Established exact envelopes; no arbitrary suffix/header rule | Present | **ALREADY IMPLEMENTED** |
| Non-event terminal diagnostics, established context-Note variants and zero-state Tracks | Successful SCHOOL handoffs/listening/readback | Established only within documented bounds | Present | **ALREADY IMPLEMENTED** |
| Expanded40/39-byte conductor manifest with exact FF footer | Established separate manifest grammar | Exact whole-envelope proof, not short-form permission | Present | **ALREADY IMPLEMENTED** |

See the [120 semantic bridge](BOUNDED_120_LAYOUT_SEMANTIC_BRIDGE.md),
[recovery builder](BOUNDED_120_RECOVERY_BUILDER.md),
[initial conductor variants](BOUNDED_INITIAL_CONDUCTOR_VARIANTS.md),
[bounded manifest](BOUNDED_SEQUENCE_MANIFEST_IMPLEMENTATION.md) and
[terminal diagnostic](NON_EVENT_TERMINAL_DIAGNOSTIC.md).
The older SCHOOL sweep's longer-prelude recommendation is superseded by current
implementation. Do not implement it again or claim minuet is now recoverable.

## Selected next engineering action

**Prepare and, only under a separate implementation request, implement the
narrow explicit C0 Program-entry rule for independently owned, bounded
120-layout single-Instrument event streams.** This is the sole selected action.
The sparse metadata rule is also implementation-ready at its bounded association
layer, but is not bundled into this action.

Why select C0: PM-R13 supplies a decisive same-project serialization bridge,
complete consumption/count evidence and independent channel/routing checks.
It is a reusable source representation of already-understood musical content,
not a familiar-status guess, project-name exception or unknown-byte acceptance.
It is a smaller engineering boundary than association changes, with stronger
independent musical validation. CCA-001 prioritized a possible near-complete
conductor opportunity; CCA-002 eliminated that immediate route. An evidenced
missing decoder form now outranks another speculative blocker investigation.

### Exact boundary

- Require independently established applicable 120 profile, event-body ownership,
  bounds and single-Instrument same-source routing. Do not broaden Descriptor166
  or Multi handling through the shared walker by accident.
- At the current event cursor use the existing bounded timing VLQ, explicit `C0`,
  exactly one following Program byte in `0..127`, checked accumulated time,
  exact source provenance and one logical event.
- C0 is a canonical project tag: output channel comes from proven Track routing,
  not its literal wire-MIDI channel nibble. Low Strings routes to channel3;
  Piano l Hand routes to channel1. This distinction is required by the evidence.
- Preserve the existing Program-only event semantics; invent no bank select,
  Patch name or Instrument selection. Do not fabricate an FF7C source wrapper.
- Reset continuation state. Permit only independently supported explicit next
  entries or exact body end. No Program running status, no C1..CF generalization.
- Retain complete event-count, pair/secondary, routing, order, conductor,
  terminal, strict adaptation, readback and fresh export-authorization gates.
  Malformed/high-bit/missing data, ambiguous routing or unowned bytes still refuse.
- Select by evidenced structural applicability, never name, offset, hash or file.

Likely affected modules in a future task: `src/mixed_event.rs` for bounded item/
continuation handling; `src/authority120.rs` for applicability/count/routing proof;
`src/recovery120.rs` for Program-only handoff; existing MIDI adapter/readback paths
only as needed to carry the already-supported Program event. Internal source DTO
plumbing must preserve provenance and exhaustiveness. No JSON/C ABI/Swift capability,
profile omission, Ready policy or public partial-export change is required.
Implementation details are not prescribed where safer internal design is possible.

### Required regression and authentic checks

- Positive synthetic owned120 streams: Program0/127, multiple timed entries,
  channel other than1, explicit90/E0 successors, exact body-end Program and
  counted consumption. Source-order ties must keep existing ambiguity policy.
- Negative cases: missing/high-bit Program, truncated/overflow timing, unowned
  range, unknown channel/Instrument, C1..CF, data-only Program continuation,
  count deficits, trailing bytes and unsupported context/Multi/166 applicability.
- Preserve generic C0-refusal tests outside the new proven scope, particularly
  `tests/bounded_mixed_event_walker.rs`; add narrow scoped tests rather than
  deleting blanket negative coverage. Exercise authority/recovery rejection
  and no capability promotion when conductor remains refused.
- Reuse PM-R13 original/normalized correspondence: Frame1 body70 events,
  Frame4 body24, Frame6 body180; nine direct Programs map to the same normalized
  FF7C values with all other body bytes/timing unchanged. Wider18 ordinary pairs
  reconcile with27 Programs, established contexts and route graph.
- Directly validated owned-body checks may use existing observational association
  evidence; production end-to-end Prologue remains blocked until separate sparse
  association support exists. Do not add a hidden production metadata bypass to
  make the authentic C0 check run.
- In a future implementation validation, preserve eight SCHOOL output baselines,
  Ode/Bells normalized proofs and independently checked MIDI event ordering. No
  frozen release artifact is regenerated. No such checks or exports ran here.

Invalidation conditions: any bridge contradiction, changed timing/Note bytes,
required bank/context dependency, ambiguous channel/ownership, newly admitted
unsupported continuation or broader-profile regression invalidates the proposed
scope and must retain/refine refusal. A disappearing C0 error cannot authorize
recovery through remaining conductor/cardinality uncertainty.

### Benefit and authorization limit

Demonstrated capability gap: three inspected original bodies and wider18-pair
observation contain the explicit Program representation, currently unsupported.
Demonstrated new complete exports: **zero**. Those Frames still have parked
conductor blockers; the metadata interruption is also unimplemented in production.
Future matching intact projects may benefit, but no unseen-corpus count is claimed.
No new SheepShaver experiment is necessary for this bounded rule. CCA-003 selects
and specifies the next engineering work; it does not execute it.

## Supportability: one narrow recommendation

Current UI distinguishes Ready/recovered workflow from Partially supported/Not
currently exportable and preserves diagnostic stage/refusal details. It does not
provide a proved damaged-file classifier; unsupported layout, ambiguous recovery
and actual truncation must not be conflated. Existing raw declared-range errors
alone are insufficient for a damage claim under an unrecognized grammar.

Recommend one future presentation-only clarification: for a recognized candidate
refused by a format/authority gate, show its existing bounded refusal category with
“Phoenix does not yet support this structure; this result does not establish file
damage.” Use existing readiness reasons/diagnostics; do not classify unexplained
failures as damaged or alter export eligibility. Successful recovery remains a
completed transaction, not parser success. No new support system, payload upload,
telemetry or diagnostic schema is designed here.

## Deferred questions and evidence

Keep parked: Prologue conductor header applicability; COMIC short-footer
applicability; arbitrary missing-pair cardinality reconciliation; separate
serialized Prologue object02/05 role; unknown ff40/ff20/ff02/f0 and triple contexts;
unknown routing/Patch/bank variants; unproven source order/terminal forms. No new
experiments, broad intake or repeated forensic chain is selected.

Unresolved questions remain precise: can a particular header/footer alter applicable
conductor context; which independently established slots own missing/unmatched pairs;
what exact timing/state/count does each unsupported owned event contribute? No
speculative meanings are supplied. The sparse list answers metadata association,
not missing event ownership; saved mute is historical state, not deletion authority.

Temporary evidence references (material conclusions are preserved above):

- `/tmp/phoenix-consolidated-compatibility-cca-001.txt`
- `/tmp/phoenix-comic-book-conductor-suffix-cca-002.txt`
- `/tmp/phoenix-prologue-conductor-decision-pm-r15.txt`
- `/tmp/phoenix-prologue-conductor-envelope-pm-r14.txt`
- `/tmp/phoenix-prologue-program-change-pm-r13.txt`
- `/tmp/phoenix-prologue-sparse-type07-ordering-pm-r12a.txt`
- `/tmp/phoenix-comic-book-blocker-decomposition-cb-r01.txt`
- `/tmp/phoenix-school-practical-export-blocker-sweep-4.txt`
- `/tmp/phoenix-longer-type07-prelude-review.txt`

Durable evidence: [post-PM-R13 checkpoint](PROLOGUE_POST_PM_R13_RECOVERY_RESEARCH_CHECKPOINT.md),
[120 authority](BOUNDED_120_EXPORT_AUTHORITY_PROOF.md),
[AppService integration](BOUNDED_120_APPSERVICE_INTEGRATION.md),
[MIDI export policy](MIDI_EXPORT_DESIGN.md), and records linked above.
Consolidation preserves decisions without making the temporary observations
production authorities. No runtime dependence on temporary reports is proposed.
