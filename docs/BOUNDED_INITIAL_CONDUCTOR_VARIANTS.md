# Bounded initial conductor variants

The practical SCHOOL PROJECTS sweep found four otherwise-complete Sequences
whose current first gate refusal was conductor coverage. The existing helper
required compact Key+Meter and Tempo primaries with one exact header form and
an `ff` secondary context byte. It refused three already-decodable padded
Meter-only/Tempo bundles and one compact header/context variant.

## Accepted structure and completeness

`bounded_conductor::decode_compact_conductor` now accepts two complete layouts
within the same initial-conductor envelope family:

- Compact: a 35-byte Meter primary contains exactly an initial six-byte Key
  event and eight-byte Meter event; a 28-byte Tempo primary contains exactly
  one seven-byte initial Tempo event. Counts are two and one respectively.
  The Meter primary's bounded header variant may be `00 01` or `00 03`;
  Tempo remains `00 01`.
- Padded: both primaries are exactly 38 bytes. Meter contains exactly one
  eight-byte initial Meter event, Tempo exactly one seven-byte initial Tempo
  event, with count one and header `00 01`. Both have the established exact
  seven-byte terminal marker followed by zero-filled padding: nine bytes for
  Meter and ten for Tempo. There is no Key event in this form.

The first event starts at primary payload +14; the compact Meter follows its
Key at +20. Every accepted event has zero initial timing. The exact payload
extent selects the layout; the decoder never searches for a recognizable
event, resynchronizes, or accepts an event prefix of a larger map. The existing
Tempo/Meter decoders and historical conductor adapter remain authoritative.
The compact Key tag, length, timing, signed accidentals and mode remain checked.
The returned `key()` is now optional: absence stays absent, including in SMF.

All four framed records must be contiguous, exact supplied root-record bounds
with alternating type02/type29 tags. The caller must still establish that they
are the selected Sequence's complete conductor pairs, with no extra prelude.
Primary counts, fixed zero fields, terminal markers and padding must agree.
Each secondary has one exact bounded entry: its own length, event count,
event-region length, fixed framing and final event tag/value copy are checked.
The single secondary byte at payload +19, between the fixed `2f` marker and
four fixed zero bytes, is retained as opaque context rather than required to
be `ff`. Its fixed one-byte extent cannot hide an additional conductor event
under this envelope rule; lengths/counts/value coverage do not depend on it.

`envelope_context()` retains absolute source ranges and bytes for primary
header variants, four-byte primary opaque context, the one-byte secondary
context and any validated zero padding. Existing record/event provenance and
the four-byte `opaque_context()` accessor remain available. The meaning of
these non-event fields is not established and none becomes a MIDI event.

## Refusal boundaries

Additional Tempo, Meter or Key changes, changed counts, contradictory secondary
copies, malformed/truncated events, nonzero initial timing, unknown event tags,
nonzero padding, extra payload bytes, unsupported headers, mixed compact/padded
layouts, reordered/overlapping/inexact record bounds and broader conductor maps
remain refused. In particular, the 38-byte two-Meter lookalike is not treated
as a one-Meter padded form. No arbitrary unknown suffix is admitted.

This change does not alter ordinary event decoding, routing, Patch translation,
mute inclusion, source ordering, sequence-terminal coverage, Descriptor166
manifest policy, AppService readiness or release export authorization.

## Authentic verification

Read-only source: 343,875 bytes, SHA-256
`bfd4fa1208e2cd884ec51ccfb1131d5c02723d7c90acafc98bd89597e1a20331`.
Ranges below are decimal, absolute and half-open. Initial conductor events all
occur at tick zero; no additional conductor event exists in the accepted region.

| Target | Meter primary / secondary | Tempo primary / secondary | Decoded conductor | Result |
|---|---|---|---|---|
| #2 NOTHING.MID | 64034..64074 / 64074..64108 | 64108..64141 / 64141..64174 | 1,000,000 MPQN (60 BPM), 4/4, Key 0 accidentals/major | Conductor PASS; new terminal refusal |
| #9 unnamed | 248917..248960 / 248960..248994 | 248994..249037 / 249037..249070 | 731,707 MPQN (~82 BPM), 7/8, no Key event | Full conversion and mechanical verification PASS |
| #12 Tuba thing | 280266..280309 / 280309..280343 | 280343..280386 / 280386..280419 | 545,454 MPQN (~110 BPM), 4/4, no Key event | Full conversion and mechanical verification PASS |
| #13 unnamed, owner UI Sequence M | 281507..281550 / 281550..281584 | 281584..281627 / 281627..281660 | 545,454 MPQN (~110 BPM), 4/4, no Key event | Conductor PASS; new terminal refusal |

All four ordinary musical paths remain complete: 3,542 logical events, 3,114
Notes, 15 Controllers, seven Patches, 406 Pitch Bends. The legitimate no-event-data
Track in Sequence M is retained without an invented channel or event. Tuba
thing's saved-muted musical Track remains included; mute is not serialized as
permission to omit music.

The two newly exposed sequence-terminal refusals are reported without fixes:

- NOTHING.MID: payload 85485..85509 is
  `0000fffffffffffffffffffffffffffffffffffffffeffff`; leading zeros fall outside
  the existing all-ff / single-internal-fe predicate.
- Sequence M: payload 281665..281671 is `fffffffffffe`; the `fe` is final rather
  than followed by `ff`, so the current predicate refuses.

No semantics or musical-loss conclusion is assigned to these terminal bytes.
Neither blocked target received a partial listening MIDI.

Fresh, collision-safe listening artifacts were created outside the repository:

- `SCHOOL - Unnamed Sequence 09.mid`: 1,098 bytes, 113 Note attacks and 113
  generated releases; SHA-256
  `a4421055e4f6d6915e7b04e196b87590f3689735a768a0d25ca9b62feca00fc2`.
- `SCHOOL - Tuba thing.mid`: 787 bytes, 75 Note attacks and 75 generated
  releases; SHA-256
  `4085abaa837942e426a6c95895b414ce77345488ba2e2fcb4d73d4470f97bcc5`.

Both reside in the owner's Experimental MIDI Recovery/SCHOOL PROJECTS directory.
Independent SMF readback checks structure, 480 PPQN, every channel message's
tick/status/data against the owned source handoff, source order, generated
releases, Track names/EOT and initial conductor metadata. Their bytes also
match the previously validated simple-listening artifacts. No new owner
listening result is claimed.

DANCING.MID and Nothing FINAL remain fully mechanically verified, with identical
MIDI bytes to the sweep baseline. The broader/multiple conductor cases #7 and
#15 still refuse `Envelope`; no additional recovery investigation was performed.

Twelve focused conductor tests pass, including the six existing tests and new
coverage for both variants, retained optional Key/provenance, bounded opaque
context, every truncation, structural mutations, ambiguous bounds, additional
event-bearing material, nonzero padding and the two-Meter lookalike. Full
`cargo test`, `cargo fmt --check`, all-target/all-feature Clippy with warnings
denied, and `git diff --check` pass. The authentic source and unrelated
owner/Xcode files remain unchanged.
