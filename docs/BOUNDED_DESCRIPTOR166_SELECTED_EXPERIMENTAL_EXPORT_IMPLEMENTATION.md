# Descriptor166 selected-Track experimental Note export — CCA-031

Baseline: `7196fc48d40327961bc458c8f79131186b727d55`, main.
This additive Rust service capability does not confer verified export authority.

## API and source binding

`AppService::experimental_tracks166` returns source-derived `TrackIdentity166`
rows. Each contains SHA-256, Sequence ordinal/range, descriptor ordinal/range,
pair ordinal and primary/secondary ranges. Display names are not identity.
`preview_experimental_selected166(session, sequence, selection)` requires a
nonempty explicit selection, rejects duplicate/unknown/mismatched identities,
and orders the output by structural source order. The immutable experimental
plan retains the exact selection. Existing `export_experimental_sequence` requires
explicit acknowledgment, rereads/hash-checks the source and rebuilds that selection
before using the unchanged collision-safe publisher. No automatic selection,
verified fallback, Ready mutation, JSON/C ABI or GUI operation is added.

## Local proof and exclusions

The existing Descriptor166 parser must establish exhaustive equal-count ordinal
Track/pair ownership for the whole object. Included Tracks must have exactly
consumed Note-only bodies, declared-count agreement, valid labels, existing
same-source Single routing/context proof and source-order serialization proof.
Selected empty Tracks, Patch/context, Controllers, SysEx, Pressure/Bend, malformed
or ambiguous events and unresolved routing refuse. No individual event is skipped.

The existing `initial_only` conductor helper is shared crate-internally without
changing its predicates: both initial primary geometry and complete matching
secondary copies must pass. Initial tempo/meter are decoded/adapted by existing
handlers; no default values or full conductor applicability are invented.
Unresolved preludes, map/synchronization and terminal semantics remain explicit
limits of a conditional local timeline, not new verified acceptance rules.

Every ordinary Track appears either in the selected identity list or excluded
ledger. Exclusions carry source identities, names, saved state, direct-Note count
lower bounds (or UNKNOWN) and explicit reasons. `omitted_known_notes` is likewise
a direct-Note lower bound for this scope, not complete missing musical content.
Unknown semantic event count stays UNKNOWN. All framed Sequence bytes have a
contiguous range/type/hash ledger. Unsupported excluded music is never described
as harmless. Original saved ON state may be explicitly selected and reported;
it does not grant verified inclusion or change the source.

## Serialization and limitations

Reuses existing Note decoder, `from_note`, strict MIDI adapter, Identity480 timing,
Note provenance ordering, SMF writer and readback. Output title says SELECTED
TRACKS / EXPERIMENTAL / INCOMPLETE / NOT VERIFIED; filename, response and mandatory
embedded/companion report remain explicitly experimental. EOT closes rendered
releases only. Instrument/OMS timbre, historical inclusion and whole-sequence
completeness are not reproduced. Publication remains synchronous, FailIfExists
only, non-atomic report-first/MIDI-second, with disclosed report-only failure.

Existing 120 experimental outputs retain their report shape and identical MIDI
bytes: the optional Descriptor166 scope is absent, not serialized as null.
Verified builders, profiles, guards and service authorization are unchanged.

## Authentic observations and tests

Authenticated Experiment007 newest STUFF baseline: 211468 bytes, SHA-256
`e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`.
Surviving CCA-008 evidence and CCA-010/012/017 implementation records supply local
ownership/timing/routing/Note proofs. Fourteen missing older temporary reports
are not asserted as reverified; no experiment was repeated.

- Situation: descriptor ordinals 4/5/6/7, 367 Notes, four musical Tracks; two
  excluded Tracks. Selected saved mute states now decode OFF with implemented04.
- Sequence D: ordinals 4/5, 176 Notes, two musical Tracks; four excluded Tracks.
- happyone: ordinals 2/3/4, 893 Notes, three musical Tracks, all saved ON explicitly
  selected; eight excluded Tracks. The SysEx/Patch-bearing rows are excluded whole.

Actual production service outputs under unique `/tmp/phoenix-cca031-export-*`
were independently parsed and compared with separately collected source-Note
proofs: all 2872 Note On/Off messages match tick, duration, pitch, attack/release,
channel, Track and source order. Embedded/companion reports, region hashes and
selected/excluded identities agree. All three normal verified exports still refuse.
No native-reference equivalence, DAW audition or additional complete recovery claimed.

Six focused tests cover exact Notes, explicit source-bound selection, malformed
and unsupported events, count mismatch, unsafe initial timing/contradictory copies,
unknown routes, ownership failure, stale source, missing opt-in, collision refusal,
source-order ambiguity, full authentic exclusion coverage and unchanged Ready
population. A three-output golden SHA regression proves current120 bytes unchanged.
Existing complete-export and publication fault regressions remain intact.
Full validation results are recorded in `/tmp/phoenix-descriptor166-experimental-export-cca-031.txt`.
