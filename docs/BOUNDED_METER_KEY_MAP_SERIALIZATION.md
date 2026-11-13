# Map-preserving Meter/Key serialization

This internal serialization/readback handoff uses bounded Tempo and Meter/Key
maps. It changes no recovery admission or export eligibility.

`ConductorMapPlan` borrows immutable maps and privately retains each ordered
object's map/index identity. Original source and timing provenance stay in the
maps. No unrelated event schedule can replace the plan's order.

- `prepare` merges by absolute tick while preserving each map's source order.
  At zero, only one Tempo followed by one Meter, optionally preceded by one Key,
  resolves a cross-map tie. Other cross-map ties refuse.
- `correlate_reference` accepts ALL and ONLY independently parsed reference
  Tempo/Meter/Key tuples. The caller must authenticate correspondence. Every
  count, tick, adapted value and within-map order must match. Correlation supplies
  order only for those maps and preserves the initial ordering restriction.
- `prepare_meter_key` permits isolated Meter/Key serialization without inventing
  Tempo or claiming a complete conductor.
- `serialize` retains every object and duplicate, recomputes checked SMF deltas
  from absolute ticks and puts EOT at the latest emitted event. No offsets,
  scaling, default events, deduplication or source repair are supplied.

Every Meter uses the existing `KnownHistoricalOnly` adaptation policy. Errors
and warnings refuse; unsupported fields never fall back. Checked arithmetic,
four-byte SMF deltas and chunk-length checks remain mandatory.

`readback120::conductor_map_expected` independently derives expected ordered
meta tuples from source maps and the validated order, without copying serialized
bytes. Existing strict readback checks values, counts, ticks, order and EOT.

## Reproducible synthetic coverage

Public tests build all input in memory. For example, an isolated three-flat minor
Key at tick 29 is followed by two synthetic 5/8 Meters at tick 102. Expected
adapted Meter payloads are explicitly `[5, 3, 24, 8]`, and EOT is at 102.
The correlation tests use separately specified expected tuples and reject
missing, extra, changed and reordered tuples. Additional cases cover ambiguous
ties, legacy initial byte compatibility, late changes, unsupported adaptation,
overflow and independent readback mutations.

These tests validate synthetic protocol behavior, not authentic-project
regressions. Run the standard checks from the repository root:

```sh
cargo fmt --check
cargo test --offline --locked
cargo clippy --offline --locked --all-targets --all-features -- -D warnings
git diff --check
```

Production integration remains separate. It requires owned exact envelopes,
complete non-event accounting, independently justified tie order and existing
track, timing, source-integrity, readback, preview, consent and publication gates.
No additional recoverable sequence is claimed.
