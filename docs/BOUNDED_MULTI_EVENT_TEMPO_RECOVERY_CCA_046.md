# Bounded multi-event Tempo recovery — CCA-046

The new caller-bounded Tempo decoder preserves complete counted event bodies and
serializes their exact Tempo timing and values into an SMF conductor chunk. It
does **not** authorize Sweeper export or broaden conductor-envelope acceptance.

## Evidence and provenance

CCA-045 correlated SCHOOL PROJECTS `minuet.mid` with the existing MIDI reference:
all 152 ordered Tempo ticks/MPQN values, 35 separate meter/key events and independent
musical Note fingerprints agree. Exact native export-session history remains
unconfirmed; this is sufficient cross-correlated evidence for the observed grammar,
not a claim of native complete-export equivalence.

Authentic fixtures are outside Git:

| Artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| Original COMIC BOOK | 452284 | `22a9e2589098b0d475f6e288eeebc9303bed644aa8ce09f9971a0060c9160486` |
| SCHOOL PROJECTS | 343875 | `bfd4fa1208e2cd884ec51ccfb1131d5c02723d7c90acafc98bd89597e1a20331` |
| Standalone minuet | 30536 | `f95c8e75dbce342fdbfbce9083d0d91fcbf55f9dae585c3bc129bce5a9bc9a4a` |
| Existing minuet.mid reference | 20993 | `641c29017f64f5c9fc285c5a5f9433db09c8953a03e63341d5b20c91827e49b8` |

Original Sweeper Tempo primary `[36314,36548)`, body `[36333,36541)`, declares
26 events. SCHOOL Minuet primary `[197491,198734)`, body `[197510,198727)`, declares
152. Standalone Minuet primary `[4013,5255)`, body `[4032,5248)`, also declares 152.
All ranges are absolute and half-open. Archive COMIC BOOK is not substituted.

## Implementation and contract

`tempo::decode_bounded_tempo_map` takes exact caller-established event-only bounds
and a declared count. The grammar is a bounded four-byte-maximum 7-bit big-endian
VLQ delta followed by `FF 51 03` and three unsigned big-endian MPQN bytes.
It reuses `track7::decode_7bit_be_vlq`, retains every event's source range, raw
located delta bytes and located MPQN bytes, and computes absolute ticks using
checked `u32` addition from zero. Identity480 is the supported timebase: no
scaling, offset, BPM round trip or synthetic initial Tempo.

A successful map consumes the entire supplied body exactly once, with the exact
count. Equal-position and duplicate events remain in source order. The immutable
map has no public constructor from arbitrary schedules. Caller-supplied bounds
are a prerequisite, not sequence ownership or a new export proof.

`smf::serialize_tempo_map_conductor` uses existing name, Tempo, meter/key, VLQ,
EOT and chunk serializers. It emits each Tempo at its checked difference from
the preceding absolute tick; assembly must use `map.ppqn()` (480). The supplied
supported initial meter/key remains at zero. If the first Tempo is at zero, legacy
single-Tempo name/Tempo/key/meter ordering is preserved byte-for-byte. If it starts
later, no Tempo at zero is invented. EOT is at the last Tempo position.

This API does not represent general meter/key maps. The Minuet test compares all
152 Tempo events and preserves its supported initial 3/4 meter; it does not claim
to reproduce the reference's other 35 conductor objects or complete MIDI file.
Existing initial-conductor APIs and all production/experimental export callers
remain unchanged. No Note, Controller, Patch, Bend, SysEx, routing or ownership
implementation changed.

## Validation and refusal

Seven default tests cover explicit independent wire fixtures, raw provenance,
zero deltas/duplicates, nonzero first position without synthesized Tempo, exact
single-Tempo meter/key byte compatibility, wrong counts/bounds, extra bytes/events,
unknown tags/lengths, every truncation, malformed VLQs, zero MPQN and cumulative
position overflow. SMF deltas above the existing four-byte limit remain refused.
The old initial decoder's raw zero-MPQN behavior is unchanged; the new recovery
map refuses zero under the existing valid-SMF Tempo contract.

Two explicitly enabled authentic tests validate all 152 Minuet events against an
independent parser of the hashed existing MIDI reference, and all 26 Sweeper
events against the independently recorded source inventory. Source accounting
asserts consecutive ranges and exact body end. Minuet starts at 1440 and ends
at 152640. Standalone starts at zero and ends at 151200; only the first timing
prefix differs, and the remaining source event bytes are identical. No historical
cause or universal +1440 correction is inferred.

Sweeper's first Tempo is 625000 MPQN (96 BPM) at zero; the closing map starts
changing at tick 138240 and ends at tick 144001 with 937500 MPQN (64 BPM). All
26 ticks/values and strict monotonic slowdown are asserted. Authority/recovery
refuse Sweeper; AppService exposes no bounded capability and experimental preview
still refuses.

Run the authentic tests without mutable `/tmp` fixtures:

```sh
PHOENIX_COMIC_ORIGINAL_SOURCE='/Users/kurtheiden/Documents/Phoenix Research/Authentic Studio Vision Projects/COMIC BOOK' \
PHOENIX_SCHOOL_OBSERVATION_SOURCE='/Users/kurtheiden/Documents/Phoenix Research/Authentic Studio Vision Projects/SCHOOL PROJECTS' \
PHOENIX_MINUET_STANDALONE_SOURCE='/Users/kurtheiden/Documents/Phoenix Research/Authentic Studio Vision Projects/minuet' \
PHOENIX_MINUET_REFERENCE_MIDI='/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/MIDI Exports from SVP/minuet.mid' \
cargo test --test bounded_tempo_map -- --ignored
```

Formatting and all-target/all-feature Clippy passed. Full suite: 647 passed,
zero failed, 13 ignored. Authentic map tests separately: two passed, zero failed.
Existing single-Tempo, conductor meter/key, verified MIDI and experimental-export
regressions passed. `git diff --check` passed.

## Remaining limitations

Complete accounting here means **the supplied Tempo body**, not an entire source
primary/secondary pair, conductor or project. Prefix/tail/secondary bytes remain
outside the decoder and require separate bounded coverage before integration into
a whole conductor proof. No arbitrary header or suffix waiver is introduced.
Sweeper's context-to-Controller/terminal-context, SysEx framing and preservation,
mixed Patch/CC routing, audio classification, terminal coverage and export-authority
blockers remain. Sweeper #2's ownership shortages are unaffected. No complete
Sweeper export, native reference or source modification was created.

Smallest next implementation: a separately justified conductor-envelope accounting
adapter for the already decoded Sweeper Tempo body and supported initial meter,
retaining opaque source material and refusing unexplained content. It must not
weaken export authorization or solve unrelated event families in the same change.
