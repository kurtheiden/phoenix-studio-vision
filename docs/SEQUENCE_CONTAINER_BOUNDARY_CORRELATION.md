# Objective

Determine the narrowest evidence-supported hierarchy for navigating the
untouched Experiment 007 Studio Vision project from sequences to sequence-level
Meter/Tempo structures, track descriptors, and bounded track-primary records
without globally scanning for event signatures.

# Scope

This is read-only correlation and design. It does not implement discovery,
promote unverified numeric values to pointers, interpret mixed event streams,
or change any decoder. The source is:

`/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline`

The data fork is 211,468 bytes with SHA-256
`e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`.
All 18 active sequence identities come from existing Studio Vision UI ground
truth, not printable-name inference alone.

# Authentic sequence inventory

The table uses these structural terms:

- `start`: beginning of the repeated 208-byte sequence preamble;
- `descriptors`: start of the 166-byte descriptor array;
- `name`: one-byte Pascal-length field immediately before the sequence name;
- `end`: exclusive end of the terminal type-`0x00` record and, for sequences
  1–17, the next sequence's `start`;
- `tracks`: descriptor count minus the two Meter/Tempo descriptors.

Primary ranges are exact bounded-decoder inputs. `M2` and `T2` are correlated
secondary value-bearing ranges inside their newly bounded type-`0x29` records;
their broader semantic purpose remains unknown.

| Sequence | Start / next start | Descriptors / name | Tracks | Meter / M2 | Tempo / T2 | Independently known performance evidence |
|---|---|---|---:|---|---|---|
| `xForm` | `0x006abc..0x00dfff` | `0x006b8c` / `0x007537` | 13 | `0x0075c2..0x0075ca` / `0x0075ff..0x007604` | `0x00761a..0x007621` / `0x007657..0x00765b` | none bounded by identity |
| `Bells for her` | `0x00dfff..0x01603b` | `0x00e0cf` / `0x00eb20` | 14 | `0x00eb80..0x00eb88` / `0x00ebbd..0x00ebc2` | `0x00ebd8..0x00ebdf` / `0x00ec15..0x00ec19` | Tracks 3/4/6 Controllers; Track 9 mixed span; Track 14 Controllers/Pitch Bend |
| `Situation` | `0x01603b..0x01780e` | `0x01610b` / `0x01662c` | 6 | `0x016673..0x01667b` / `0x0166b0..0x0166b5` | `0x0166cb..0x0166d2` / `0x016708..0x01670c` | none bounded by identity |
| `Sequence D` | `0x01780e..0x01900f` | `0x0178de` / `0x017dff` | 6 | `0x017e50..0x017e58` / `0x017e8d..0x017e92` | `0x017ea8..0x017eaf` / `0x017ee5..0x017ee9` | none bounded by identity |
| `Sequence E` | `0x01900f..0x01b0db` | `0x0190df` / `0x01974c` | 8 | `0x0197aa..0x0197b2` / `0x0197e7..0x0197ec` | `0x019802..0x019809` / `0x01983f..0x019843` | none bounded by identity |
| `Girl-U-Want` | `0x01b0db..0x01bf6d` | `0x01b1ab` / `0x01b4da` | 3 | `0x01b516..0x01b51e` / `0x01b553..0x01b558` | `0x01b56e..0x01b575` / `0x01b5ab..0x01b5af` | none bounded by identity |
| `mission impossibl` | `0x01bf6d..0x01e71a` | `0x01c03d` / `0x01c7f6` | 10 | `0x01c864..0x01c86c` / `0x01c8a1..0x01c8a6` | `0x01c8bc..0x01c8c3` / `0x01c8f9..0x01c8fd` | project-only 10/8 Meter evidence |
| `happyone` | `0x01e71a..0x0227a6` | `0x01e7ea` / `0x01f049` | 11 | `0x01f093..0x01f09b` / `0x01f0d0..0x01f0d5` | `0x01f0eb..0x01f0f2` / `0x01f128..0x01f12c` | none bounded by identity |
| `Sequence I` | `0x0227a6..0x0246c2` | `0x022876` / `0x0230d5` | 11 | `0x023119..0x023121` / `0x023156..0x02315b` | `0x023171..0x023178` / `0x0231ae..0x0231b2` | one blank descriptor; only ten track record pairs |
| `newsong` | `0x0246c2..0x025568` | `0x024792` / `0x024c0d` | 5 | `0x024c3d..0x024c45` / `0x024c71..0x024c76` | `0x024c8c..0x024c93` / `0x024cbf..0x024cc3` | shorter primary record variants |
| `Sequence K` | `0x025568..0x025b81` | `0x025638` / `0x0258c1` | 2 | `0x0258df..0x0258e7` / `0x02591c..0x025921` | `0x025937..0x02593e` / `0x025974..0x025978` | natural 6/8 export; Track 1 performance, Track 2 not exported |
| `Renaissance` | `0x025b81..0x026b67` | `0x025c51` / `0x026172` | 6 | `0x026191..0x026199` / `0x0261ce..0x0261d3` | `0x0261e9..0x0261f0` / `0x026226..0x02622a` | none bounded by identity |
| `Get on up & Dance` | `0x026b67..0x02d60b` | `0x026c37` / `0x0277d4` | 16 | `0x0277f9..0x027801` / `0x027836..0x02783b` | `0x027851..0x027858` / `0x02788e..0x027892` | none bounded by identity |
| `Jurrasic Park` | `0x02d60b..0x02ef6f` | `0x02d6db` / `0x02dca2` | 7 | `0x02dcd0..0x02dcd8` / `0x02dd0d..0x02dd12` | `0x02dd28..0x02dd2f` / `0x02dd65..0x02dd69` | none bounded by identity |
| `Ode to Clarke` | `0x02ef6f..0x03202c` | `0x02f03f` / `0x02f752` | 9 | `0x02f784..0x02f78c` / `0x02f7c1..0x02f7c6` | `0x02f7dc..0x02f7e3` / `0x02f819..0x02f81d` | Tracks 1/2/3/3 #2 Patches and Notes; Track 7 Note chain |
| `Over the Top` | `0x03202c..0x032a9d` | `0x0320fc` / `0x03242b` | 3 | `0x03244b..0x032453` / `0x032488..0x03248d` | `0x0324a3..0x0324aa` / `0x0324e0..0x0324e4` | none bounded by identity |
| `Sequence Q` | `0x032a9d..0x03329a` | `0x032b6d` / `0x032d50` | 1 | `0x032d6e..0x032d76` / `0x032dab..0x032db0` | `0x032dc6..0x032dcd` / `0x032e03..0x032e07` | none bounded by identity |
| `Sequence R` | `0x03329a..0x033a07` | `0x03336a` / `0x0335f3` | 2 | `0x033611..0x033619` / `0x03364e..0x033653` | `0x033669..0x033670` / `0x0336a6..0x0336aa` | terminal sequence; project-level type-`0x05` zero-length record follows to EOF |

No gap is assigned to a sequence merely because of file order. The sequence
range above is established by the current sequence's preamble and its terminal
typed record; for sequences 1–17, that exclusive end independently equals the
next preamble start.

# Repeated sequence-level layout

Within this artifact, the following are **PROVEN invariants across all 18
authenticated sequences**:

1. A 208-byte preamble begins the sequence structure.
2. Preamble byte `+5` equals the total descriptor count.
3. The descriptor array begins at preamble `+208`; descriptor starts advance by
   exactly 166 bytes.
4. Descriptor 0 is `Meter Track`, descriptor 1 is `Tempo Track`, and remaining
   descriptors are track descriptors. Each descriptor label begins at `+15`.
5. The Pascal sequence-name length byte is at
   `descriptor_start + descriptor_count * 166 - 15`; its following bytes equal
   the authenticated sequence name.
6. Immediately after the sequence name is a stream of
   `type:u8 | length:u32 big-endian | payload[length]` records.
7. Zero or more type-`0x09` records precede four records in order:
   Meter primary `0x02`, Meter secondary `0x29`, Tempo primary `0x02`, Tempo
   secondary `0x29`.
8. In both primary records, the bounded event begins 14 bytes into the payload.
9. Track structures then occur as type-`0x02` primary / type-`0x29` secondary
   pairs, followed by one type-`0x00` terminal record.
10. The terminal record end equals the next 208-byte preamble start for all 17
    adjacent pairs.

**Strong repeated patterns** are that descriptor order is track-primary-pair
order and that type-`0x02` track records own the performance-bearing primary
data. This is independently confirmed by Bells Tracks 9/14 and Ode Tracks
1/2/3/3 #2/7. It is not yet universal because `Sequence I` has eleven track
descriptors, including one blank label, but only ten primary/secondary pairs.

**Variable fields** include descriptor count, descriptor labels, sequence-name
length, number and lengths of pre-Meter type-`0x09` records, every track primary
and secondary length, terminal-record length, Meter/Tempo primary lengths in
`newsong`, event payloads, and footer widths.

**Unknowns** include most preamble and descriptor fields, the semantic names
of record types `0x02`, `0x09`, `0x29`, and `0x00`, inactive/blank descriptor policy,
and the purpose of most primary/secondary payload bytes.

# Sequence boundary evidence

Sequence ends are explicitly derivable once a sequence preamble is known:
checked big-endian record lengths walk from the Pascal name end to the terminal
type-`0x00` record. All 17 terminal ends land exactly at the next sequence
preamble, and the final terminal ends at `0x033a07`, followed by a separate
type-`0x05`, zero-length project record through EOF.

The first preamble at `0x006abc` is also the exact end of a type-`0x2f` record
at `0x006aab` whose big-endian length is 12. Project-root correlation later in
this document establishes that both are consecutive members of the top-level
record stream beginning at `0x00000008`. No direct two-, three-, or four-byte
big- or little-endian absolute encoding of `0x006abc` is needed.

Sequence boundaries are therefore **YES** at project level for the established
form: checked top-level lengths reach the first type-`0x01` preamble, and the
validated sequence-local grammar derives all following boundaries.

# Track ownership evidence

Preamble `+5` supplies total descriptors; subtracting Meter and Tempo gives
track descriptor count. Track descriptors are local to the sequence preamble,
ordered, and preserve user-visible names, including custom names such as
`bass`, `kick drum`, `sys100loops`, and duplicate-numbered tracks. An empty
label is present in `Sequence I`, demonstrating that descriptor existence does
not require a printable track name.

Track primary/secondary pairs follow Meter/Tempo in descriptor order. Three
independent sequence checks are especially strong:

- `Bells for her`: 14 descriptors and 14 pairs. Track 9 maps to primary record
  `0x0143b5..0x01495e`, containing the independently established mixed-event
  span `0x0143c8..0x014956`. Track 14 maps to
  `0x014e13..0x015edb`, containing its Controllers and all nine Pitch Bend
  runs.
- `Ode to Clarke`: nine descriptors and nine pairs. Track 1
  `0x02f820..0x02fa7a`, Track 2 `0x02fb42..0x0300df`, Track 3
  `0x0312ed..0x03156b`, Track 3 #2 `0x031873..0x031b05`, and Track 7
  `0x031bf5..0x031fa3` contain the independently established Patch/Note
  evidence. The five old anonymous Note-chain candidates are thereby assigned
  by structure to Track 2, `sys100loops`, Track 4, Track 5, and Track 6.
- `Sequence K`: two descriptors and two pairs. Track 1 primary is
  `0x02597b..0x025a6c`; Track 2 is the short primary
  `0x025b36..0x025b50`. Only Track 1 appears in the provenance-controlled SMF,
  showing that a descriptor/pair may exist without exported performance
  events.

`Sequence I` is the exception: eleven track descriptors but ten pairs. The
blank descriptor is the leading candidate for omission/inactive state, but its
exact flag and pair-skipping rule are unproven. Track ownership is therefore
**PARTIAL** for a general parser, despite exact order for all independently
identified nonblank examples.

# Track event-region evidence

Each type-`0x02` track record has an exact container range derived from its
big-endian length. Every independently identified performance representation
begins 14 bytes after the primary payload start, or equivalently 19 bytes after
the type byte. This includes Ode Patch starts `0x02f833`, `0x02fb55`,
`0x031300`, and `0x031886`, plus Bells Track 9 at `0x0143c8` and Ode Track 7
at `0x031c08` before its first Note properties.

The exclusive inner event-region end is not a fixed subtraction from the
primary record end. Bells Track 9's established events end eight bytes before
its primary end; Ode Track 7's established Note chain ends seven bytes before
its primary end. The tail contains repeated footer-like bytes but no proven
length or terminator grammar. Track 14 has an exact primary bound containing
its known families, but no one exact whole-event-region end or internal run
table.

Thus track primary containers are exactly bounded, while performance-event
regions inside them are **PARTIAL**: their starts are strongly repeated and
their containing upper bounds are exact, but their exclusive event ends and
family-run subdivisions are not generally derived.

# Candidate offset/length fields

The following numeric fields have repeated structural support:

- preamble `+5`: one-byte descriptor count, 18/18 agreement;
- descriptor stride: 166 bytes, all declared descriptors;
- sequence preamble width: 208 bytes, all 18;
- record `+1..+5`: unsigned 32-bit big-endian payload length, with checked
  record ends landing through every sequence stream;
- primary event offset: payload `+14` for all Meter/Tempo primaries and all
  independently identified track primaries.

Previous exhaustive surveys of 120-byte/older descriptor records found many
arithmetically in-range values but no systematic pointer target. Fresh tests
around descriptor, name, and Meter anchors found no common two-, three-, or
four-byte big- or little-endian field equal to the next absolute sequence
start, descriptor-span length, or relative distance to the next sequence.
Those bytes remain opaque rather than being called pointers.

# Exact-bound supply chain

| Decoder | Required caller knowledge | Owning structural layer | Current supply status |
|---|---|---|---|
| initial Meter | exact eight-byte range | sequence primary type-`0x02` record, payload `+14` | derivable from the checked root and sequence walk |
| initial Tempo | exact seven-byte range | sequence primary type-`0x02` record, payload `+14` | derivable from the checked root and sequence walk |
| Controller | exact record range and timing state | track primary plus current-cursor family classification | length derivable only after mixed walker reaches a tagged Controller |
| Channel Pressure | exact run range and active entry state | event-family run container/state | no run-bound field established |
| Pitch Bend | exact run range and active entry state | event-family run container/state | no run-bound field established |
| Patch | exact position start and boundary after first Note status | track/event transition layer | track primary start is known; terminal Patch/Note handoff is not container-derived |
| Note | caller-asserted Note-chain bound and timing basis | track/event-family region | containing primary is known; type discrimination and exact chain end remain partial |

# Minimum viable container parser

The narrow evidence-supported core should accept the complete project bytes,
preserve the eight-byte root header, walk checked top-level records from offset
eight, and return raw-preserving structures:

```text
SequenceCollection
  sequences: [Sequence]

Sequence
  preamble_range
  raw_preamble
  descriptor_count
  descriptor_ranges and raw descriptors
  descriptor labels
  sequence_name with source range
  prelude_records
  meter_primary_record and exact event_range
  meter_secondary_record and correlated value_range
  tempo_primary_record and exact event_range
  tempo_secondary_record and correlated value_range
  track descriptor / primary / secondary associations where unambiguous
  terminal_record

Track
  descriptor_range and label
  primary_record_range and payload_range, if associated
  secondary_record_range, if associated
  candidate event start at primary payload + 14
  raw unresolved tail
```

Every addition and multiplication must be checked; every declared count and
length must remain within input; fixed tags should be validated at their
record-relative locations; unknown bytes should be borrowed with absolute
ranges. The parser must walk only from the current top-level cursor and must
not search for names, `ff 58`, `ff 51`, `ff 41`, or event signatures.

This bounded core is designable now. The project-root entry-point correlation
below additionally establishes how a checked whole-file walk reaches the first
sequence without a content-signature search.

# Automatic decoder-invocation readiness

- A. initial Meter: **YES** — the root and sequence walks supply its exact
  primary-record-relative bound without scanning.
- B. initial Tempo: **YES** — the same chain supplies its exact primary bound.
- C. Controller: **PARTIAL** — exact track primary bounds exist, but the mixed
  walker cannot always reach/classify each Controller from the event start.
- D. Channel Pressure: **NO** — no structural run-end field supplies its exact
  stateful bound.
- E. Pitch Bend: **NO** — no structural run-end field supplies its exact
  stateful bound.
- F. Patch: **PARTIAL** — primary event start is derivable, but the exact
  Patch-to-first-Note terminal boundary is not.
- G. Note: **PARTIAL** — containing track primary and repeated start are known,
  but collision-resistant mixed classification and exact chain ends are not.

# What container discovery does not solve

Container discovery supplies sequence identity, descriptor ownership,
Meter/Tempo primary bounds, exact track primary containing bounds, and a safe
upper bound for track-local work. It does not identify every byte inside a
track primary, derive a universal event footer, split stateful Pressure/Bend
runs, resolve Patch-to-first-Note ownership/timing, or make untagged Note forms
collision-free. Mixed-event interpretation remains a separate layer.

# Highest-value blocker

The project-root entry-point blocker is resolved by the checked root record
walk documented below. The highest-value remaining structural blocker is the
**exact inner performance-event end and family transition grammar within a
track-primary record**. Its resolution would let more existing bounded event
decoders receive exact ranges rather than only an exact containing upper bound.

# Evidence supported

- All 18 authenticated sequences were structurally compared.
- Forward sequence boundaries and next-sequence starts are exact from the
  checked project-root record walk.
- Meter and Tempo primary record-relative bounds are exact.
- Descriptor count, width, order, and local names are repeated across all 18.
- Track primary/secondary record bounds are exact and track order is strongly
  correlated across multiple independently identified sequences.
- Inner performance-event region bounds and stateful run bounds remain partial.
- No candidate numeric field is promoted to a pointer.
- A narrow whole-file sequence/container parser is **implementation-ready**:
  consume the eight-byte root header, walk checked top-level records, and
  validate each type-`0x01` sequence preamble with the established local
  sequence grammar.

# Unknowns

Unknowns include the meanings of the four root-header words, most
preamble/descriptor fields, the `Sequence I` inactive/blank descriptor rule,
semantic record-type names, exact event footer grammar, general event-region
ends, family-run bounds, mixed Note discrimination, Patch/Note handoff, and
generality beyond the structurally checked project population.

# Single recommended next step

Implement the narrow read-only project-record/sequence-container parser that
starts after the eight-byte root header, validates every checked top-level
record boundary, and admits type-`0x01` records as sequences only after the
established preamble, descriptor, name, and terminal invariants validate. No
controlled experiment is indicated. The exact API, deterministic errors,
166-byte supported profile, Sequence I mismatch policy, and explicit older
120-byte-form exclusion are defined in
`ROOT_SEQUENCE_CONTAINER_PARSER_DESIGN.md`.

# Project-root entry-point correlation

## Forward chain

Experiment 007 begins with an exact eight-byte root header:

```text
0x00000000..0x00000008  00 2d 00 93 00 ac 00 a6
```

The header's four big-endian 16-bit values are observed as `0x002d`, `0x0093`,
`0x00ac`, and `0x00a6`; their semantics are not assigned. At `0x00000008`, a
top-level stream begins with the same checked grammar used later in sequence
records:

```text
type:u8 | payload_length:u32 big-endian | payload[payload_length]
```

Walking that grammar from `0x00000008`, validating header availability,
checked addition, and `record_end <= file_length` at every step, consumes 527
consecutive records and lands exactly at EOF `0x00033a0c`. It does not stop at
an ambiguous frontier. The first 109 records end at `0x00006abc`; records
104--109 around the transition are:

| Index | Range | Type | Payload length |
|---:|---|---:|---:|
| 104 | `0x000069e6..0x00006a21` | `0x2d` | 54 |
| 105 | `0x00006a21..0x00006a9e` | `0x2e` | 120 |
| 106 | `0x00006a9e..0x00006aa4` | `0x25` | 1 |
| 107 | `0x00006aa4..0x00006aab` | `0x3b` | 2 |
| 108 | `0x00006aab..0x00006abc` | `0x2f` | 12 |
| 109 | `0x00006abc..0x00007527` | `0x01` | 2,662 |

The type-`0x01` record beginning at `0x00006abc` is the first authenticated
sequence preamble. All 18 authenticated preambles are top-level type-`0x01`
record starts, and no earlier top-level type-`0x01` record occurs. Recognition
therefore uses the structural record type plus the already established
sequence-local invariants, not a sequence name, descriptor label, Meter/Tempo
signature, or hard-coded target offset.

The eight-byte root-header boundary is cross-validated rather than inferred
from one successful alignment. The same offset-eight checked record walk ends
exactly at EOF in the untouched older authentic sample (495 records), both
Experiment 001 project states (600 and 527 records), and Experiments 002--030
(527 records each, including the two size-changing states). The older authentic
sample's final header word is `0x0078`, matching its independently surveyed
120-byte descriptor width; Experiment 007's is `0x00a6`, matching its proven
166-byte descriptor width. The other header words remain opaque.

## Backward chain and type-`0x2f` record

The record immediately before the first sequence is exactly:

```text
record range   0x00006aab..0x00006abc
type           2f
length bytes   00 00 00 0c (unsigned big-endian 12)
payload range  0x00006ab0..0x00006abc
payload        00 83 00 02 02 66 01 30 00 24 00 00
```

Its preceding boundary is independently supplied by the type-`0x3b` record
`0x00006aa4..0x00006aab` (length 2, payload `00 05`), which is preceded by the
type-`0x25` record `0x00006a9e..0x00006aa4` (length 1, payload `03`) and the
type-`0x2e` record `0x00006a21..0x00006a9e` (length 120). Continuing backward
by the same already aligned top-level record table reaches `0x00000008`.

Exactly one type-`0x2f` record occurs in each checked project. In the older
authentic sample and both structurally different Experiment 001 states it also
has a 12-byte payload and ends exactly at the first type-`0x01` sequence
preamble. Its payload values vary, so no payload field is promoted to an
offset, count, or semantic navigation field. It is a recurring terminal record
of the pre-sequence portion of the top-level stream, but no stronger semantic
name is assigned.

## Candidate parent and joining result

No separate length-delimited parent enclosing the 18-sequence collection is
needed or evidenced. The project-level parent is the top-level record stream
that begins after the fixed root header and is bounded by EOF. Sequence
structures are a contiguous run of top-level records: each begins with
type-`0x01`, continues through its validated local records, and ends with
type-`0x00`. The next record is another type-`0x01` for sequences 1--17.

The forward and backward chains therefore join **YES** at every checked record
boundary, including `0x00006abc`. No candidate absolute pointer, relative
displacement, or reverse-fitted number is required.

## Project-tail cross-check

The final sequence begins with top-level type-`0x01` at `0x0003329a`. Its
terminal type-`0x00` record is `0x000339fa..0x00033a07`. The following
top-level type-`0x05` record has zero payload and occupies
`0x00033a07..0x00033a0c`, landing exactly at EOF. This independently confirms
that sequence records and the final project record share the same top-level
grammar; it does not establish a sequence count field or semantic name for
type `0x05`.

## Implementation gate and remaining limits

A first read-only whole-project sequence/container parser is **IMPLEMENTED**
within a deliberately narrow scope:

1. require the eight-byte root header to be present and preserve its four raw
   big-endian words without interpreting them;
2. start the top-level record cursor at offset `0x08`;
3. validate and consume every record using its unsigned big-endian length;
4. accept a type-`0x01` record as a sequence start only when the established
   208-byte preamble, descriptor count/stride, Pascal name, Meter/Tempo order,
   track-pair, and terminal invariants validate;
5. require the top-level walk to consume the input exactly, including the final
   zero-length type-`0x05` record for the established form.

This does not interpret the opaque root words, assign meanings to pre-sequence
record types, prove an encoded sequence count, resolve inactive descriptor
handling, or solve inner mixed-event bounds. No controlled experiment is
required for the root entry point. The next blocker is exact track-local event
termination and transition structure, not project-root navigation.
