# Bounded experimental Note export (CCA-027)

## Separate capability

`AppService::preview_experimental_sequence(session_id, sequence_id)` returns an
immutable `ExperimentalPlan` with a public limitations report and private source
selection/MIDI. `export_experimental_sequence(request, &plan, true)` requires
explicit acknowledgment of incomplete rendering and inclusion of all proved
ordinary Tracks. False acknowledgment refuses before publication. The response
is `ExperimentalExportResponse`, never `ExportSequenceResponse`, Ready, Validated
or a verified recovery capability. Already verified objects are refused here.

This first version is a callable Rust service API. No Swift UI, JSON transport or
C ABI operation is added. Existing verified interfaces and readiness are unchanged.
CCA-026 and CCA-019 supply the design scope, not a new conductor interpretation.

## Eligibility

The existing 120-layout association and `authority120` independently establish
object identity, exhaustive ordinary Track ownership, complete bounded musical
walking/count reconciliation and routing. All included items must be direct
Notes. Patches, context-mediated forms, Controllers, Pressure, Bend, SysEx and
other unsupported forms refuse; no partial body scanning or event omission.
Single-channel routes must be proved. Existing legitimate empty Tracks retain
boundaries without inventing a channel. All proved ordinary Tracks are selected.

Initial Meter/Tempo require the existing padded 38-byte primary profile: header
00 01, count one, known zero/context geometry, exact initial-event position,
established tail and padding. Existing initial event decoders and strict MIDI
adapters validate values; no default tempo is used. Compact/key-bearing,
unfamiliar header, invalid timing/value and ambiguous ordering forms refuse.

Unknown secondary applicability, complete conductor map/synchronization and
terminal duration remain unresolved. Output is a **conditional local timeline**
using observed initial values, not proof of original playback equivalence. These
unknowns are not accepted as harmless by the verified decoder. All selected
Sequence framed regions have contiguous range/hash/type coverage in the report.
Known included Notes are counted; unknown semantic event count is `null`/UNKNOWN,
not zero. Saved mute is reported Unknown where this 120 proof does not establish
it. Explicit inclusion does not modify the source or discard muted music.

## Preservation and publication

Existing Note decoding, parsed channel assignment, Identity480 timing, MIDI
adapter, source-order Note provenance and SMF writer/readback are reused.
Every Note onset, duration, pitch, attack/release velocity and ordering is retained.
The writer only adds descriptive text to the existing conductor serialization.

Output filename and in-file Sequence name say EXPERIMENTAL / INCOMPLETE / NOT
VERIFIED. Full limitations JSON is embedded as a tick-zero SMF Text event and
published as a mandatory `.limitations.json` companion. The result carries the
same report. Labels can be stripped by another application; no label permanence
or native-reference fidelity is promised.

Preview and export reread and hash source against the inspection session. Export
rebuilds and compares the immutable plan before publication. The existing
no-overwrite temporary-file/write/flush/sync/hard-link/cleanup transaction is
shared; verified publication still uses its original MIDI suffix and behavior.
Experimental export supports only FailIfExists; GenerateUniqueName refuses to
avoid ambiguous companion names. Limitations publish first. If MIDI publication
fails, a report may remain and its path is explicitly returned in the error;
no published file is deleted or overwritten. Publication warnings remain visible.
Operations remain synchronous with cancellation unsupported, exactly as the
existing service reports. No new cancellation promise is made.

## Authentic validation

The source is the previously researched 452,284-byte COMIC BOOK:
`/Users/kurtheiden/Documents/Phoenix Research/Authentic Studio Vision Projects/COMIC BOOK`,
SHA-256 `22a9e2589098b0d475f6e288eeebc9303bed644aa8ce09f9971a0060c9160486`.
It is **different** from CCA-023's verified 406,294-byte Archive source.

- pick up the pace: 64 Notes, channel 5, 521739 MPQN.
- nitemare b4 xmas: 167 Notes, channel 2, 600000 MPQN.
- sampler xperimnt: 239 Notes, channel 10, 631578 MPQN; five ordinary Tracks,
  including one independently paired empty Track.

Production experimental service exports to new unique `/tmp` directories were
independently parsed. All 940 expanded Note messages across these three outputs
match the source proofs in tick, channel, pitch, velocity and source order.
Track names/boundaries, observed tempo/meter, embedded/companion reports and EOT
consumption agree. Verified export still refuses each object. No native-reference
equivalence or DAW audition is claimed. No source or native reference was edited.

## Regression and limitations

Five portable unit tests and four integration tests cover exact Notes, complete
region consumption, paired empty Tracks, malformed/truncated Notes, unsupported
Program entry, ambiguity, unknown routing/ownership, invalid initial timing/tempo,
missing opt-in, stale source, selection mismatch, invalid filenames, report/MIDI
collisions, truthful unknown counts and unchanged readiness/verified refusal.
All existing authenticated recovery tests remain intact.

Validation: formatting, Clippy with all targets, full tests **626 passed, 10
ignored, zero failures**, and diff checks pass. Early synthetic fixture/expectation
failures and an empty-Track handoff error were corrected and revalidated; no
acceptance guard was broadened to make them pass.

This is Note-only 120 service support, not a generic partial-event exporter or
complete-recovery expansion. It excludes unresolved ownership and unsupported
musical content; no source-hash/name-specific eligibility exceptions. Integration
tests require existing mounted research artifacts. No new emulator experiment,
staging, commit or push occurred.
