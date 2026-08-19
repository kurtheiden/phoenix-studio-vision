# Objective

Design the narrowest non-throwaway Phase D path from the authenticated
Experiment 007 `Ode to Clarke` sequence to one complete SMF Format 1 file with
one conductor track and all nine musical tracks. This is a design for one
provenance-locked sequence, not arbitrary Studio Vision export.

The primary design answer is **YES**: current sequence framing, exact-range
derivation, mixed-event walking, decoded-event adaptation, and SMF primitives
cover every musical event in all nine Ode tracks. No new binary-format
interpretation is required.

# Scope

Phase D includes the established 166-byte sequence profile, initial Tempo and
Meter, four evidence-classified Patch events, and 1,308 Notes. It uses the
authenticated nine-row channel manifest as proof policy. It does not add a
CLI, general channel discovery, new event families, mid-sequence maps, legacy
text conversion, or full-project/multi-sequence export.

# Provenance

The source is Experiment 007's untouched baseline, independently reverified as
211,468 bytes with SHA-256
`e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`.
The reference `Ode to Clarke Multi All` is independently reverified as 12,141
bytes with SHA-256
`4f63b34ef92204d4bc5eeb78dbbe7b94d005c1f9ceb57ea0f9809533ad590f29`.
Both were read only.

The structurally parsed sequence is `0x02ef6f..0x03202c`; its derived name is
the 13 raw ASCII bytes `Ode to Clarke` at `0x02f753..0x02f760`. Phase D must
validate all of these facts before consulting any track row.

# Reference MIDI inventory

An independent running-status-aware parse consumes the reference exactly. It
is Format 1, division 480, with ten tracks. Positive-velocity Note On is
counted as a Note start; explicit `8n` and `9n` velocity-zero are both counted
as Note endings. CC totals shown here are the Patch-associated Bank Selects.

| SMF | Track Name | Ch. | Notes | CC | PC | Pressure | Bend | Bank Select | First channel tick | Last channel tick | EOT tick |
|---:|---|---:|---:|---:|---:|---:|---:|---|---:|---:|---:|
| 0 | `Ode to Clarke` | — | 0 | 0 | 0 | 0 | 0 | — | — | — | 94,080 |
| 1 | `Track 1` | 1 | 91 | 0 | 1 | 0 | 0 | — | 0 | 80,692 | 94,080 |
| 2 | `Track 2` | 2 | 211 | 2 | 1 | 0 | 0 | tick 0: CC0=81, CC32=1 | 0 | 87,246 | 94,080 |
| 3 | `sys100loops` | 10 | 322 | 0 | 0 | 0 | 0 | — | 1,920 | 87,906 | 94,080 |
| 4 | `Track 4` | 10 | 179 | 0 | 0 | 0 | 0 | — | 1,920 | 89,856 | 94,080 |
| 5 | `Track 5` | 10 | 134 | 0 | 0 | 0 | 0 | — | 2,640 | 88,136 | 94,080 |
| 6 | `Track 3` | 1 | 84 | 2 | 1 | 0 | 0 | tick 480: CC0=81, CC32=2 | 480 | 83,904 | 94,080 |
| 7 | `Track 6` | 10 | 60 | 0 | 0 | 0 | 0 | — | 59,520 | 88,429 | 94,080 |
| 8 | `Track 3 #2` | 15 | 84 | 0 | 1 | 0 | 0 | — | 530 | 83,904 | 94,080 |
| 9 | `Track 7` | 10 | 143 | 0 | 0 | 0 | 0 | — | 47,280 | 91,078 | 94,080 |

Other events are metadata, not unsupported channel messages. Conductor track
0 has Track Name, SMPTE Offset `ff 54 05 60 00 00 00 00`, Set Tempo, Time
Signature, and EOT. Each musical track has Instrument Name, Track Name, and
EOT. There is no SysEx, Poly Pressure, ordinary Controller, Channel Pressure,
Pitch Bend, later Tempo, or later Time Signature in this reference.

# SVP track inventory

All ranges are structurally reached through `parse_project_166`, equal-count
ordinal descriptor/pair bindings, and the validated exact-end rule. `Final
position` is the final decoded source event start, not the latest Note end.

| Ref. SMF | Desc. ordinal / range | Raw descriptor name | Pair | Primary range | Exact event range | Ch. | Logical events / families | Final position |
|---:|---|---|---:|---|---|---:|---|---:|
| 1 | 2 / `0x02f18b..0x02f231` | `Track 1` | 0 | `0x02f820..0x02fa7a` | `0x02f833..0x02fa73` | 1 | 92: Patch 1, Note 91 | 80,640 |
| 2 | 3 / `0x02f231..0x02f2d7` | `Track 2` | 1 | `0x02fb42..0x0300df` | `0x02fb55..0x0300d8` | 2 | 212: Patch 1, Note 211 | 87,120 |
| 3 | 4 / `0x02f2d7..0x02f37d` | `sys100loops` | 2 | `0x0301b7..0x03097d` | `0x0301ca..0x030976` | 10 | 322: Note 322 | 87,840 |
| 4 | 5 / `0x02f37d..0x02f423` | `Track 4` | 3 | `0x030a17..0x030e9f` | `0x030a2a..0x030e98` | 10 | 179: Note 179 | 89,520 |
| 5 | 6 / `0x02f423..0x02f4c9` | `Track 5` | 4 | `0x030f31..0x03125b` | `0x030f44..0x031254` | 10 | 134: Note 134 | 88,080 |
| 6 | 7 / `0x02f4c9..0x02f56f` | `Track 3` | 5 | `0x0312ed..0x03156b` | `0x031300..0x031564` | 1 | 85: Patch 1, Note 84 | 80,386 |
| 7 | 8 / `0x02f56f..0x02f615` | `Track 6` | 6 | `0x03165b..0x031805` | `0x03166e..0x0317fe` | 10 | 60: Note 60 | 88,336 |
| 8 | 9 / `0x02f615..0x02f6bb` | `Track 3 #2` | 7 | `0x031873..0x031b05` | `0x031886..0x031afe` | 15 | 85: Patch 1, Note 84 | 80,386 |
| 9 | 10 / `0x02f6bb..0x02f761` | `Track 7` | 8 | `0x031bf5..0x031fa3` | `0x031c08..0x031f9c` | 10 | 143: Note 143 | 90,960 |

# Coverage matrix

`POLICY` means an upstream evidence classification is required; it does not
mean the adapter guesses opaque bytes.

| Track | Note | Patch | Controller | Pressure | Bend | Other mixed family |
|---|---|---|---|---|---|---|
| `Track 1` | SUPPORTED | POLICY: confirmed Program only | NOT PRESENT | NOT PRESENT | NOT PRESENT | NOT PRESENT |
| `Track 2` | SUPPORTED | POLICY: confirmed CC0/CC32/Program | NOT PRESENT | NOT PRESENT | NOT PRESENT | NOT PRESENT |
| `sys100loops` | SUPPORTED | NOT PRESENT | NOT PRESENT | NOT PRESENT | NOT PRESENT | NOT PRESENT |
| `Track 4` | SUPPORTED | NOT PRESENT | NOT PRESENT | NOT PRESENT | NOT PRESENT | NOT PRESENT |
| `Track 5` | SUPPORTED | NOT PRESENT | NOT PRESENT | NOT PRESENT | NOT PRESENT | NOT PRESENT |
| `Track 3` | SUPPORTED | POLICY: confirmed CC0/CC32/Program | NOT PRESENT | NOT PRESENT | NOT PRESENT | NOT PRESENT |
| `Track 6` | SUPPORTED | NOT PRESENT | NOT PRESENT | NOT PRESENT | NOT PRESENT | NOT PRESENT |
| `Track 3 #2` | SUPPORTED | POLICY: confirmed Program only | NOT PRESENT | NOT PRESENT | NOT PRESENT | NOT PRESENT |
| `Track 7` | SUPPORTED | NOT PRESENT | NOT PRESENT | NOT PRESENT | NOT PRESENT | NOT PRESENT |

Every logical event is covered; no skip policy is needed.

# Exact-walk readiness

All nine current `walk_bounded_mixed_events` calls return **PASS**, consume
their supplied exact range completely, and produce the inventories above.
There is no first failing offset or walker error. Phase D tests should repeat
these assertions against the locked authentic fixture rather than encode the
observed inventories without walking.

# Channel manifest

The complete immutable proof input is:

```text
project_sha256 = e5a70056...7e5132
sequence_range = 0x02ef6f..0x03202c
sequence_name_bytes = "Ode to Clarke"
sequence_name_range = 0x02f753..0x02f760

rows = (descriptor ordinal, descriptor range, pair ordinal,
        primary range, exact event range, human MIDI channel)
       (2,  0x02f18b..0x02f231, 0, 0x02f820..0x02fa7a, 0x02f833..0x02fa73, 1)
       (3,  0x02f231..0x02f2d7, 1, 0x02fb42..0x0300df, 0x02fb55..0x0300d8, 2)
       (4,  0x02f2d7..0x02f37d, 2, 0x0301b7..0x03097d, 0x0301ca..0x030976, 10)
       (5,  0x02f37d..0x02f423, 3, 0x030a17..0x030e9f, 0x030a2a..0x030e98, 10)
       (6,  0x02f423..0x02f4c9, 4, 0x030f31..0x03125b, 0x030f44..0x031254, 10)
       (7,  0x02f4c9..0x02f56f, 5, 0x0312ed..0x03156b, 0x031300..0x031564, 1)
       (8,  0x02f56f..0x02f615, 6, 0x03165b..0x031805, 0x03166e..0x0317fe, 10)
       (9,  0x02f615..0x02f6bb, 7, 0x031873..0x031b05, 0x031886..0x031afe, 15)
       (10, 0x02f6bb..0x02f761, 8, 0x031bf5..0x031fa3, 0x031c08..0x031f9c, 10)
```

Validation must be all-or-nothing: exact project and sequence identity; exactly
nine rows; unique, ordered descriptor and pair ordinals; exact descriptor,
primary, and derived event ranges; established ordinal binding; valid channel;
and no extra or missing event-bearing binding. A descriptor label corroborates
identity and supplies output text but is never the sole manifest key. Any
mismatch fails before sequence bytes are returned; there is no fallback
channel.

# Manifest architecture

Choose **C: a caller-supplied manifest structure**, with the concrete Ode value
stored only in authenticated proof integration.

- A test-only ad hoc manifest is smallest locally but encourages another
  one-track implementation and gives no reusable validation seam.
- A production module containing an Ode constant would isolate the data but
  still pollute shipping format logic with target-specific knowledge.
- A generic immutable manifest/validator input lets reusable sequence assembly
  consume explicit policy. The Ode constant, reference digest, and Patch
  classifications remain proof fixture/policy and can disappear when parsed
  routing becomes available without changing the assembler.

Neither `sequence_container`, `mixed_event`, `midi_export`, nor `smf` should
contain Ode hashes, ranges, labels, or channels.

# Track ordering

Use validated Studio Vision descriptor order, which equals pair order and the
authenticated reference musical-track order for all nine rows. Emit conductor
Track 0, then rows in ascending descriptor ordinal 2 through 10. Reject a
manifest whose rows do not reproduce the established ordinal bindings. Do not
sort by label: names repeat (`Track 3` / `Track 3 #2`) and are not structural
identity.

# Track naming

All nine raw descriptor labels are already reliable and ASCII/UTF-8:

| Descriptor order | Raw bytes / output name | Conversion |
|---:|---|---|
| 2 | `Track 1` | unchanged ASCII |
| 3 | `Track 2` | unchanged ASCII |
| 4 | `sys100loops` | unchanged ASCII |
| 5 | `Track 4` | unchanged ASCII |
| 6 | `Track 5` | unchanged ASCII |
| 7 | `Track 3` | unchanged ASCII |
| 8 | `Track 6` | unchanged ASCII |
| 9 | `Track 3 #2` | unchanged ASCII |
| 10 | `Track 7` | unchanged ASCII |

No MacRoman conversion is required for Ode. General legacy conversion remains
deferred rather than weakening byte-preserving parsing.

# Notes

| Track | Notes | Earliest start | Latest end | Zero duration | Overlapping same pitch | Ref. velocity-zero endings |
|---|---:|---:|---:|---:|---:|---:|
| `Track 1` | 91 | 9,720 | 80,692 | 0 | 0 | 2 |
| `Track 2` | 211 | 1,920 | 87,246 | 0 | 0 | 4 |
| `sys100loops` | 322 | 1,920 | 87,906 | 0 | 0 | 4 |
| `Track 4` | 179 | 1,920 | 89,856 | 0 | 0 | 1 |
| `Track 5` | 134 | 2,640 | 88,136 | 0 | 0 | 1 |
| `Track 3` | 84 | 9,603 | 83,904 | 0 | 0 | 2 |
| `Track 6` | 60 | 59,520 | 88,429 | 0 | 0 | 1 |
| `Track 3 #2` | 84 | 9,603 | 83,904 | 0 | 0 | 2 |
| `Track 7` | 143 | 47,280 | 91,078 | 0 | 0 | 0 |

Read-only normalized comparison establishes **1,308/1,308 one-for-one musical
matches** for pitch, start, duration/end, and attack velocity. All 1,291
reference explicit `8n` release velocities match the SVP values. Seventeen
reference endings use `9n` velocity zero and therefore carry no comparable
release velocity; Phoenix preserves the decoded release value in explicit
`8n`, an expected representation difference. Phase D must repeat the strict
per-track comparison and fail on its first unmatched Note. No quantization,
pair repair, or omission is permitted.

# Patch coverage

Four bounded Patch representations occur:

| Track | Tick | Ch. | Name / opaque tail | Classification | Required emission |
|---|---:|---:|---|---|---|
| `Track 1` | 0 | 1 | `Empty Patch`; `02 33 30 04 ff ff ff` | A: confirmed Program only | PC 61 |
| `Track 2` | 0 | 2 | `Stereoww Bs`; `02 33 38 04 ff 51 01` | B: confirmed banked form | CC0=81, CC32=1, PC 37 |
| `Track 3` | 480 | 1 | `Wavox`; `02 33 30 04 ff 51 02` | B: confirmed banked form | CC0=81, CC32=2, PC 29 |
| `Track 3 #2` | 530 | 15 | `Ming Dynasty`; `03 49 33 38 04 ff ff ff` | A: confirmed Program only | PC 23 |

The four direct Program fields and positions are established independently;
the Track 2/3 bank values have controlled/evidence-backed classification.
Phase D does **not** interpret `ff ff ff` as a universal sentinel. It classifies
only the direct Program translation for Track 1 and Track 3 #2 and preserves
their remaining opaque context in the proof report. Patch/device names and
other context are C: safely non-emitting metadata. The authenticated reference
contains exactly the emissions above, so no required channel state is omitted.

# Controller coverage

No ordinary Controller record occurs in any Ode event range. The reference's
four Control Changes are precisely the two established Patch bank pairs.
Existing `ConfirmedBankSelect` adaptation is sufficient. General Controller
support remains implemented but is not exercised by Phase D.

# Channel Pressure coverage

No Channel Pressure occurs in the nine exact walks or the reference. Existing
adaptation needs no change and is not exercised. The bounded Bells evidence is
not broadened into this target.

# Pitch Bend coverage

No Pitch Bend occurs in the nine exact walks or the reference. Existing exact
LSB/MSB adaptation needs no change and is not exercised. Source-unresolved
`ANALOG.MID #2` is unrelated to and outside Phase D.

# Tempo and Meter

Reuse Phase C conductor adaptation unchanged: name `Ode to Clarke`, Tempo
500,000 MPQN, source Meter `4,2,8,8` adapted to SMF `4,2,24,8`, PPQN 480, and
automatic EOT. The reference has exactly one Tempo and one Time Signature,
both at tick zero; it has no later Tempo/Meter change. Initial-only support is
therefore musically complete for Ode.

# Unsupported reference events

| Reference-only behavior | Classification | Phase D treatment |
|---|---|---|
| SMPTE Offset at conductor tick 0 | OPTIONAL synchronization/export metadata | omit and report expected difference |
| Instrument Name at each musical tick 0 | OPTIONAL metadata | omit; Patch/device metadata remains reported |
| EOT at tick 94,080 on every track | HISTORICAL EXPORT/sequence-duration policy | use minimum EOT policy; expected difference |
| Running status / historical same-tick order | HISTORICAL encoding | normalize; retain Phoenix explicit-status ordering |

Track Name, Tempo, Meter, channel messages, and EOT framing are supported.
There is no required unsupported musical event and no UNKNOWN reference event.

# End of Track policy

Studio Vision ends all ten reference tracks at tick 94,080, after their latest
channel messages. No currently parsed field establishes that tick as a general
sequence duration. Phoenix should keep its existing minimum standards-valid
policy: conductor EOT at zero and each musical EOT at its latest emitted event,
including generated Note Off, with delta zero. The differing trailing silence
is an **EXPECTED POLICY DIFFERENCE** because all musical events and ends are
preserved. Do not invent padding solely to imitate bytes. If later product
requirements establish sequence-duration semantics, add them as separate
policy.

# Transactional sequence export

Implement one pure sequence operation that validates, adapts, and serializes
all rows transactionally:

1. validate project/sequence manifest identity and complete row coverage;
2. derive and validate every exact event range;
3. walk and flatten every track without omission;
4. apply the row's explicit channel and Patch classifications;
5. adapt conductor and all tracks;
6. assemble Format 1 only after all nine results exist;
7. independently compare the complete in-memory file in proof integration.

Any manifest, walk, unsupported-data, adaptation, serialization, or comparison
failure returns an error and no successful/partial MIDI byte result. Empty or
extra bindings are not silently dropped. Ode has nine nonempty supported rows.

# Normalized comparison

The independent Phase D parser/comparator should reuse the test-side approach,
not serializer internals. Validate Format 1, division 480, ten chunks, exact
chunk consumption, final EOT, names, and one channel per musical track. Pair
Notes FIFO per `(track, channel, pitch)` as in Phase C, compare every Note
musically, and compare explicit release velocity where the reference carries
it. Compare bank/Program tuples at exact ticks. Ordinary Controller, Pressure,
and Bend inventories must be empty on both sides. Compare initial Tempo/Meter.

Classifications are:

- **EXACT MATCH:** track order/names/channels, PPQN, Tempo/Meter, all Patch
  emissions, all Note pitches/starts/ends/attacks, 1,291 explicit releases;
- **MUSICAL EQUIVALENCE:** 17 velocity-zero historical endings versus
  Phoenix's release-preserving explicit Note Offs;
- **EXPECTED POLICY DIFFERENCE:** SMPTE Offset, Instrument Name, EOT padding,
  running status, and potentially same-tick byte order;
- **UNSUPPORTED/UNKNOWN:** must remain empty, otherwise fail Phase D.

# Output artifact

Only after the complete in-memory comparison passes may an explicit research
action write:

`/Users/kurtheiden/Documents/Phoenix Research/Phoenix MIDI Proofs/Ode to Clarke - Phoenix Multitrack Proof.mid`

It must be Format 1, PPQN 480, exactly ten tracks (one conductor plus nine
musical), descriptor-order deterministic, and re-open to bytes identical to
the in-memory result with independent exact-EOF validation. Normal `cargo test`
must not create or rewrite it. This design task creates no artifact.

# Manual DAW validation

After automated success, the user should open the proof in Logic Pro and
verify normal import; ten total SMF tracks / nine musical tracks; sensible
names and descriptor-order layout; complete glitch-free playback; no hanging
or obviously misplaced Notes; sensible Patch/Program behavior; and sensible
Tempo/Meter. This remains pending until user-observed and must not be inferred
from Phase C.

# Production/proof boundary

Production-quality reusable code should own validated multitrack input/result
types, transactional sequence assembly, conductor/track adaptation orchestration,
Format 1 assembly, aggregate reporting, and context-rich conversion errors.
Generic normalized SMF parsing/comparison may remain test support unless it has
a product consumer.

Proof policy owns the Ode project/reference hashes, structural rows,
authenticated channel overrides, Patch classifications, expected reference
inventory, and artifact path. Generic parsing and MIDI modules must remain
free of those constants. Later parsed routing can supply the same channel
assignment seam and remove the proof manifest without redesigning assembly.

# Export report

On successful Ode export report:

- sequence name/structural identity and authenticated proof policy identity;
- 9 musical tracks and 10 total SMF tracks;
- per-track structural identity, name, channel/provenance, and counts;
- totals: 1,308 Notes, 1,308 generated Note Offs, 0 ordinary Controllers,
  2 Bank Select MSB, 2 Bank Select LSB, 4 Program Changes, 0 Channel Pressure,
  0 Pitch Bend, 1 Tempo, and 1 Meter;
- opaque/non-emitting Patch and instrument metadata;
- warnings and unsupported structures (both must contain no lost musical
  content for this strict proof).

On failure retain a diagnostic report of validation progress but return no
successful MIDI bytes. The report structure should aggregate naturally toward
future multi-sequence workflow wording without defining CLI/UI behavior.

# Implementation decomposition

1. **D1 — reusable sequence assembly (IMPLEMENTED):** introduce narrow owned/borrowed
   multitrack inputs and transactional assembly over existing Phase A/B APIs;
   test ordering, aggregate reports, complete-failure behavior, and ten-track
   synthetic Format 1 construction.
2. **D2 — authenticated Ode integration (IMPLEMENTED):** define the caller-supplied manifest
   contract and proof-local nine-row value/Patch classifications; structurally
   validate, walk, flatten, and adapt all rows in one pass.
3. **D3 — independent comparison (IMPLEMENTED):** parse both complete SMFs
   independently and assert the full inventory, 1,308 Note equivalences, and
   explicit policy differences transactionally.
4. **D4 — proof artifact:** behind an explicit research action only, write the
   named file after D3 passes, re-open it, hash it, and validate exact EOF.
5. **D5 — user-owned Logic Pro validation:** record the multitrack layout and
   playback result; do not infer it from automation.

This avoids nine separate one-track integrations and keeps authentic policy
outside reusable assembly.

# Implementation gate

| Gate | Result | Basis |
|---|---|---|
| A. all nine tracks structurally identifiable | **YES** | exact ordinal descriptor/pair/range map |
| B. all nine exact event ranges walkable | **YES** | nine complete current-walker PASS results |
| C. channel manifest complete | **YES** | nine single-channel provenance-locked rows |
| D. all Note data exportable | **YES** | 1,308/1,308 strict normalized matches |
| E. all Controller data exportable | **YES** | no ordinary Controller; two supported bank pairs |
| F. all Channel Pressure data exportable | **YES** | not present |
| G. all Pitch Bend data exportable | **YES** | not present |
| H. all required Patch data exportable | **YES** | four direct PCs; two established bank pairs |
| I. all required Tempo/Meter data exportable | **YES** | one initial pair; no later changes |
| J. track names/order understood | **YES** | descriptor=pair=reference order; all ASCII |
| K. unsupported required musical events absent | **YES** | complete reference family inventory |
| L. full Ode Phase D implementation-ready | **YES** | no remaining design blocker |

# Unknowns

- General Studio Vision channel routing remains unknown; the manifest is not
  promoted to format knowledge.
- The universal semantics of Patch `ff ff ff`, Instrument Name reconstruction,
  SMPTE Offset source, and sequence duration tick 94,080 remain unknown and
  are not required for musical equivalence here.
- General MacRoman, later Tempo/Meter maps, arbitrary tracks/profiles, and
  other event families remain outside Phase D.

# Single recommended next step

Implement D4's explicit proof write and independent disk re-open, using the
already-compared in-memory result and leaving normal tests write-free.
