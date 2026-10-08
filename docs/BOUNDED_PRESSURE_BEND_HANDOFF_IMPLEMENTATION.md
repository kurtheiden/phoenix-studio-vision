# Bounded Descriptor166 Pressure/Bend handoff (CCA-012)

Baseline: `8a51761d39c65e50aff79d63155649097a637854`, main.

## Rule and evidence

`bounded_sequence::build_bounded_sequence_manifest` now translates already
positioned `ChannelPressure` and `PitchBend` walker items through the existing
`DecodedExportEvent::from_channel_pressure` and `from_pitch_bend` constructors.
The existing logical ordinal, absolute position and source entry range are
retained. Pressure's direct value and Bend's direct LSB/MSB are preserved;
there is no center adjustment, approximation, high-bit masking or channel
inference from the canonical project entry tag.

Applicability remains the existing Descriptor166 whole-sequence proof:
exhaustive ordinal ownership, validated event bounds, complete nonempty walk,
independent bounded table/device/channel routing and context checks, saved Mute
OFF, complete initial conductor envelope, supported terminal, strict adaptation
and source-order safety checks. Output Dn/En uses the proven Track channel.
Explicit D0/E0 entries and existing validated compact continuations are handled
by the existing walker; this adds no source grammar or running-status rule.

Evidence: [Pressure decoder](BOUNDED_CHANNEL_PRESSURE_DECODER_IMPLEMENTATION.md),
[Bend decoder](BOUNDED_PITCH_BEND_DECODER_IMPLEMENTATION.md),
[mixed walker](BOUNDED_MIXED_EVENT_WALKER_IMPLEMENTATION.md),
[bounded routing](BOUNDED_SINGLE_INSTRUMENT_ROUTING_IMPLEMENTATION.md),
[MIDI adapter](MIDI_EXPORT_ADAPTER_IMPLEMENTATION.md) and
[source ordering](OPT_IN_SOURCE_ORDER_SERIALIZATION_IMPLEMENTATION.md).
The CCA-011 decision is `/tmp/phoenix-descriptor166-integration-cca-011.txt`.
Existing authenticated `export_handoff` already uses these constructors for
Bells for her. `tests/bells_generated_midi_reconciliation.rs` compares those
exported Pressure/Bend populations against the native reference including
channel, time and values. This closes the generic builder's two missing handoffs;
it does not extend that exact profile's inclusion policy to unrelated inputs.

## Exclusions and authority

Controller and explicit MIDI Controller translation refusals remain; the
Controller-context metadata policy is unchanged. Context-mediated Notes,
standalone Patches and unsupported Program/Patch variants remain outside this
generic builder. Existing Patch-to-Note classification remains required.
Malformed walks, unsupported successors, high-bit data, unresolved routing,
unknown/ON mute, ambiguity, conductor/terminal and ownership errors still refuse.
No events are skipped and no missing context is synthesized.

AppService exact-profile precedence, readiness, fresh source revalidation and
export authorization are unchanged. Builder success must still account for
every logical event and pass strict adaptation and source-order preflight.
There are no specimen-name/offset exceptions or partial exports.

## Tests

Five new tests in `tests/bounded_sequence.rs` cover:

- Mixed explicit/compact Pressure and Bend with Notes, exact values/ticks/source
  ordinals/ranges, complete bounds and independent SMF readback preserving
  same-tick order on channel 4.
- Routed channel endpoints 1 and 16.
- Missing/truncated entries and continuations, high-bit values and an unknown
  successor, with application refusal checks.
- ON mute, unsupported terminal and missing-pair ownership remaining refused.
- Immutable authenticated Bells/xForm event populations and unchanged independent
  first conductor-prelude refusals, without serializing or exporting these
  authentic candidates.

Existing Controller refusal, conductor/ownership mutations, authorization
firewall, exact-profile precedence and native Sequence R comparison remain.
The full suite retains existing authenticated export/reference regressions.
Standard required test-suite exports are isolated existing automated regression
operations; no new authentic MIDI export or product output was requested here.

## Authentic impact

The existing authenticated 211468-byte newest STUFF baseline has SHA-256
`e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`.
The read-only regression confirms Bells pair 8 contains 32 Pressure entries,
Bells pair 13 contains 102 Bend entries, and xForm pair 7 contains 44 Bend
entries. These are zero-based independently bounded pair ordinals.
Bells already exports through its exact authenticated policy. Its generic
builder and xForm still first refuse at conductor prelude; xForm also has
independent incomplete event and Patch/terminal limitations recorded in CCA-008.

Two generic event-family refusals are removed at the translation layer for
otherwise qualified inputs. Zero authentic first Sequence blockers are removed
by this change; zero additional complete recoveries are verified. No claim is
made that Pressure/Bend presence implies a complete recoverable Sequence.
Specimens, historical evidence and frozen release artifacts remain untouched.

## Validation

Validation passed: `cargo fmt --check`,
`cargo clippy --all-targets --all-features -- -D warnings`, `cargo test`
(607 passed, 10 ignored, zero failures) and `git diff --check`. The bounded
sequence suite passed 38 tests, including all five new regressions.
No stage, commit or push is part of this task.
