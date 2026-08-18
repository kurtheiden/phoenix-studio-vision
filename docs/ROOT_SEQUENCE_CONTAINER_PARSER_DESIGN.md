# Objective

Design the first read-only whole-project parser that begins at byte zero,
walks the checked Studio Vision root-record stream, and exposes validated
sequence, Meter/Tempo, descriptor, and track-container structure without
content-signature scanning. This document defines an implementation contract;
it does not implement the parser or mixed-event interpretation.

# Evidence boundary

The primary evidence is the untouched Experiment 007 project, whose 211,468
bytes and 527 top-level records have been checked from offset `0x08` through
exact EOF. Its 18 authenticated sequences validate the 208-byte preamble,
166-byte descriptor, derived Pascal-name, required Meter/Tempo record order,
track-pair, and terminal structure documented in
`SEQUENCE_CONTAINER_BOUNDARY_CORRELATION.md`.

The offset-eight root-record grammar is independently repeated in the older
authentic sample and controlled project states through Experiment 030. That
does not make their sequence layouts identical. In particular, the older
authentic sample uses 120-byte descriptors. The first semantic sequence parser
is deliberately limited to the proven 166-byte descriptor form.

# Supported project form

The first implementation has one explicit profile, conceptually
`SequenceContainerProfile::Descriptor166`:

- eight opaque root-header bytes;
- top-level records beginning at offset 8 with one-byte type and unsigned
  big-endian 32-bit payload length;
- sequences represented by the established 208-byte preamble and 166-byte
  descriptor layout;
- sequence-local records framed by the same generic record grammar;
- the established required Meter/Tempo ordering and type-`0x00` terminal;
- exact input consumption.

The profile is selected explicitly by the parsing API or fixed by a clearly
named 166-form entry point. The parser must not choose it by interpreting the
fourth root-header word as a descriptor width. The correlation is strong but
is not yet an established universal selector.

The older 120-byte descriptor form is **unsupported for semantic sequence
classification** in the first implementation. Its root header and generic
top-level records remain structurally parseable by the generic root walker. A
whole-project sequence parse under the 166 profile rejects its first malformed
type-`0x01` candidate deterministically; it does not scan onward, substitute a
120-byte stride, or return a misleading empty sequence list.

# Parser layers

The minimum useful separation is:

1. `RootHeader`: preserves bytes `0..8` and offers raw word conveniences.
2. `FramedRecord`: losslessly frames any valid generic record at an exact
   cursor, independent of record type.
3. `RootRecordStream`: walks `FramedRecord` values from offset 8 to EOF.
4. `SequenceContainer`: validates a type-`0x01` candidate plus the following
   top-level records using only derived local structure.
5. `SequenceDescriptor`: preserves each fixed 166-byte descriptor and its
   bounded label observation.
6. Sequence navigation: supplies Meter/Tempo decoder bounds and track
   primary/secondary containing records.

Meter, Tempo, and performance-event semantic decoders remain separate callers
of the ranges produced here.

# Root header

Conceptual representation:

```text
RootHeader<'a>
  range: 0..8
  raw: LocatedBytes<'a>
  raw_words: [LocatedBeU16<'a>; 4] (derived convenience)
```

`raw` is authoritative. Each optional word convenience preserves its two-byte
range and borrowed bytes as well as the decoded big-endian `u16`; word fields
are named only by ordinal (`word_0` through `word_3`). The parser must not
require Experiment 007's values. Input shorter than eight bytes returns
`TruncatedRootHeader` before any record access.

This follows the repository's `LocatedByte`, `LocatedBytes`, and absolute
half-open `Range<usize>` conventions. A small `LocatedBeU16` type is acceptable
if it keeps raw bytes, range, and derived value together; it must not trigger a
refactor of existing decoders.

# Generic record framing

One reusable borrowed representation is sufficient for both top-level and
sequence-local views:

```text
FramedRecord<'a>
  record_range: Range<usize>
  record_type: LocatedByte
  length_bytes: LocatedBytes<'a>       // exact four bytes
  payload_length: u32                  // derived unsigned BE value
  payload: LocatedBytes<'a>
```

Framing one record at an exact cursor proceeds in this order:

1. require five bytes for the header;
2. preserve the type and four length bytes;
3. decode the length as unsigned big-endian `u32`;
4. use checked conversion/addition to derive payload start and end;
5. require the payload end not to exceed the containing input bound;
6. return the exact record and next cursor.

A zero-length payload is valid and has an empty `payload.range` at the record
end. Unknown type bytes are valid opaque records. Fewer than five bytes return
`TruncatedRecordHeader`; arithmetic failure returns `RecordLengthOverflow`;
and an in-range decoded integer whose payload crosses the containing bound
returns `PayloadBeyondInput`. No fallback cursor is attempted.

# Top-level walk

The top-level walker starts at constant root boundary `8`, not at a discovered
signature. It repeatedly frames exactly one record at the current cursor and
advances only to the declared record end. It succeeds only when the cursor
equals `bytes.len()` exactly.

The smallest inspectable public result should retain every generic record:

```text
ParsedProject<'a>
  profile: SequenceContainerProfile
  root_header: RootHeader<'a>
  records: Vec<FramedRecord<'a>>
  sequences: Vec<SequenceContainer<'a>>
```

Retaining the records makes the 527-record regression directly testable and
preserves unknown project structures. Sequence containers may hold cloned
borrowed record views or record indices plus absolute ranges; they must not
require self-referential Rust structures. The generic root walker can be a
separate API from the profile-specific classifier so opaque inspection remains
available for unsupported semantic layouts.

Trailing bytes that cannot form a record are `TruncatedRecordHeader`, not
ignored padding. A defensive final assertion that the last next-cursor equals
input length yields `TopLevelDidNotConsumeInput` if internal iteration ever
stops early. Exact EOF after any valid record, including a zero-length record,
is success.

# Sequence recognition

A top-level type `0x01` is a candidate, not sufficient identification. Under
the supported profile, classification must validate all of the following at
the candidate's exact offset:

1. the candidate and following bytes contain a 208-byte sequence preamble;
2. preamble byte `+5` supplies the descriptor count;
3. the count is at least two and checked multiplication by 166 plus the
   descriptor-array start remains in input;
4. descriptors begin at preamble `+208`, have exact 166-byte ranges, and have
   safely bounded label observations at descriptor `+15`;
5. descriptor ordinals 0 and 1 are reserved structurally for Meter and Tempo;
   their literal label text is preserved but is not required as a signature;
6. the Pascal sequence-name length byte is derived exactly as
   `descriptor_start + descriptor_count * 166 - 15`;
7. that byte and its declared name bytes lie within the following type-`0x07`
   record, and the name ends exactly at that record's end in the established
   form;
8. zero or more type-`0x09` records follow;
9. the next four records are `0x02`, `0x29`, `0x02`, `0x29` in Meter primary,
   Meter secondary, Tempo primary, Tempo secondary order;
10. each primary payload is long enough for its payload-`+14` bounded event;
11. zero or more complete `0x02`/`0x29` track-record pairs follow;
12. a type-`0x00` terminal follows the complete pairs;
13. the next top-level cursor is either another validated type-`0x01`
    candidate or a later opaque project-level record; for Experiment 007 the
    post-sequence form is the final zero-length type-`0x05` record.

The validation is ordinal and arithmetic. It does not search for names,
labels, Meter/Tempo bytes, or events. Literal Meter/Tempo decoder tags are
validated later by their existing bounded decoders, not duplicated here.

For the supported form, failure of any type-`0x01` candidate is a whole-project
classification error (`MalformedSequenceCandidate`) carrying its exact record
index, candidate range, and a structured cause. Downgrading it to an opaque
record would hide corruption or an unsupported version; scanning for a later
candidate is forbidden. Generic root framing may still be requested
separately and can succeed without semantic classification.

# Descriptor representation

```text
SequenceDescriptor<'a>
  ordinal: usize
  range: Range<usize>                  // exactly 166 bytes
  raw: LocatedBytes<'a>
  label_start: usize                   // range.start + 15
  label: Option<LocatedBytes<'a>>      // bytes before bounded NUL
  label_terminator: Option<LocatedByte>
```

All opaque fields remain in `raw`; no numeric values are exposed as offsets,
flags, devices, or active state. Label observation begins only at the derived
`+15` location and looks for a NUL only within the descriptor range. This is a
bounded field operation, not a project-wide string search. If evidence does
not justify requiring a terminator for every future value, absence is
preserved as `None`; it is not used to shift any boundary. Blank labels are
represented as an empty byte range followed by the terminator.

The first two descriptor ordinals may be exposed as `meter_descriptor` and
`tempo_descriptor` because their ordinal roles are established by the
following required record layout. Remaining descriptors are retained as track
descriptor candidates. No label text is required for any role.

# Sequence name

```text
SequenceName<'a>
  length: LocatedByte
  bytes: LocatedBytes<'a>
```

Its location is derived from the descriptor arithmetic, never searched. The
declared length must remain inside the type-`0x07` record that contains it and
must end at that record's established boundary. Violations return
`InvalidSequenceNameBounds` with the length offset, declared length, containing
range, and derived end.

Raw bytes and range are authoritative. No UTF-8 requirement is imposed.
Callers may request an optional `std::str::from_utf8` view for ASCII/UTF-8 data
or an explicitly lossy diagnostic rendering, but parse success and sequence
identity never depend on modern text decoding. A future legacy Mac encoding
policy is separate.

# Sequence-local records

Sequence-local records are views of the same top-level `FramedRecord` values;
their header and length grammar is genuinely identical. The parser should
reuse that representation rather than introduce speculative semantic enums.

`SequenceContainer` groups record indices/ranges from its type-`0x01`
candidate through its terminal type-`0x00`. It may provide named structural
fields for the required ordinal roles (`name_record`, prelude records, Meter
primary/secondary, Tempo primary/secondary, track pairs, terminal), while
preserving each underlying record's original type and opaque payload.

# Meter and Tempo bound supply

After required record-order validation, the sequence representation stores or
returns these derived ranges:

- Meter: `meter_primary.payload.range.start + 14 .. + 22`;
- Tempo: `tempo_primary.payload.range.start + 14 .. + 21`.

All additions are checked and must remain within their respective primary
payloads. Methods such as `initial_meter_range()` and `initial_tempo_range()`
keep derivation visible and avoid duplicating bytes. They return exact
half-open ranges suitable for `decode_bounded_initial_meter` and
`decode_bounded_initial_tempo`.

The container parser does not validate `00 ff 58 04` or `00 ff 51 03`, decode
values, parse secondary copies, or infer map/position semantics. The existing
bounded decoders remain authoritative for event grammar.

# Track association

```text
TrackRecordPair<'a>
  pair_ordinal: usize
  primary: FramedRecord<'a>            // type 0x02
  secondary: FramedRecord<'a>          // type 0x29

TrackBinding
  descriptor_ordinal: Option<usize>
  pair_ordinal: Option<usize>
  status: EstablishedOrdinal | Unresolved
```

When the number of track descriptors equals the number of complete record
pairs, the established form may expose ordinal one-to-one bindings. If counts
differ, the parser still accepts the sequence and preserves both collections,
but emits no guessed ordinal bindings: every descriptor and pair is available
through unresolved bindings/indices. This represents Sequence I's eleven track
descriptors and ten pairs without deleting the blank descriptor, inventing an
inactive flag, or rejecting the sequence.

The API can therefore represent a descriptor without a proven pair, a pair
without proven descriptor identity, and the established equal-count case.
Mismatch metadata should include both counts for diagnostics.

# Track-primary containing bounds

Each `TrackRecordPair.primary` exposes its exact record and payload ranges from
generic framing. A checked convenience returns:

```text
candidate_event_start = primary.payload.range.start + 14
event_containing_range = candidate_event_start..primary.payload.range.end
exact_event_range = None
```

The containing range is explicitly named as an upper-bounded container, not an
event stream. The parser must not subtract a guessed footer, identify a first
event, or claim the payload end is the exact performance-event end.

# Error model

Conceptual deterministic errors include:

- `TruncatedRootHeader { available, expected: 8 }`;
- `TruncatedRecordHeader { offset, available, expected: 5, context }`;
- `RecordLengthOverflow { offset, payload_length }`;
- `PayloadBeyondInput { record_offset, payload_range, input_len, context }`;
- `TopLevelDidNotConsumeInput { cursor, input_len }`;
- `MalformedSequenceCandidate { record_index, candidate_range, cause }`;
- `DescriptorCountTooSmall { offset, observed, minimum: 2 }`;
- `DescriptorArithmeticOverflow { candidate_offset, count, stride: 166 }`;
- `DescriptorBeyondInput { ordinal, range, input_len }`;
- `InvalidSequenceNameBounds { length_offset, declared, containing_range,
  derived_end }`;
- `MissingRequiredSequenceRecord { expected_role, record_index }`;
- `WrongRequiredSequenceRecordType { expected_role, offset, observed,
  expected }`;
- `PrimaryPayloadTooShort { role, payload_range, required }`;
- `MalformedTrackPair { record_index, observed_types }`;
- `MalformedSequenceTerminal { record_index, offset, observed_type }`.

Root-header and generic-record failures invalidate the whole root parse.
Within the supported 166 profile, a malformed type-`0x01` candidate invalidates
the whole semantic project parse because skipping or downgrading it would be a
form of recovery. A descriptor/pair count mismatch alone is not an error; it is
preserved unresolved. Unknown non-`0x01` top-level records remain opaque.

The older descriptor form is outside the profile and therefore fails semantic
candidate validation deterministically. Documentation and diagnostics should
identify the supported profile rather than claim file corruption.

# Provenance

Every conclusion retains an absolute half-open range into the original input:

- root header and each raw word;
- complete generic record, type offset, length-byte range, and payload range;
- 208-byte sequence preamble;
- every 166-byte descriptor and bounded label observation;
- sequence-name length byte and raw name;
- all grouped sequence-local records;
- exact Meter and Tempo decoder ranges;
- every track primary/secondary record and payload;
- candidate event start and containing upper-bound range.

Borrowed `&[u8]` slices avoid copying and preserve exact bytes. Derived values
never replace their raw source bytes.

# Validation policy

## MUST validate

- at least eight input bytes;
- root record parsing begins exactly at offset eight;
- every record header, checked length, payload, and next cursor;
- exact top-level EOF consumption;
- every type-`0x01` candidate under the selected 166 profile;
- 208-byte preamble availability, descriptor count arithmetic, 166-byte
  descriptor ranges, derived name bounds, required local record order, primary
  payload capacity, complete track pairs, and terminal record;
- Meter/Tempo returned ranges remain inside their owning primary payloads.

## MAY preserve without interpretation

- all eight root bytes and four raw words;
- unknown top-level and sequence-local record types/payloads outside required
  positions;
- descriptor numeric fields and arbitrary label/name bytes;
- secondary Meter/Tempo payloads;
- descriptor/pair count mismatch and all unassociated structures;
- track-primary bytes after the candidate event start.

## MUST NOT require

- 18 sequences, 527 records, or any authentic offset;
- Experiment 007 root-header values;
- any sequence, Meter/Tempo descriptor, or track label text;
- printable or UTF-8 names;
- Meter/Tempo/event signatures during navigation;
- one record pair per track descriptor;
- the final project record type for generic root framing;
- a universal sequence count, pointer, or parent-payload interpretation.

For the supported semantic profile, the established final zero-length
type-`0x05` is an authentic regression assertion, not a universal generic
record-framing rule. The first implementation should preserve and report it
without assigning meaning; exact EOF consumption remains mandatory.

# Authentic fixtures

## Experiment 007 untouched baseline

Tests use the fixed authenticated external artifact and never scan:

- root header range `0x00000000..0x00000008` and all bytes preserved;
- 527 records from offset eight to EOF `0x00033a0c`;
- 18 validated sequences;
- first candidate derived as record 109 at `0x00006abc`;
- final type-`0x05`, zero-length record `0x00033a07..0x00033a0c`;
- Bells Meter `0x0000eb80..0x0000eb88` and Tempo
  `0x0000ebd8..0x0000ebdf`;
- Bells Track 9 primary `0x000143b5..0x0001495e` and Track 14 primary
  `0x00014e13..0x00015edb` as exact containing records;
- Ode to Clarke sequence `0x0002ef6f..0x0003202c`, its Meter/Tempo ranges,
  and representative known track-primary ranges;
- Sequence I retains 11 track descriptors, 10 pairs, and unresolved
  associations without rejection or speculative repair.

The counts and offsets are regression expectations after structural parsing,
not production locators or required constants.

## Older authentic `samples/newest STUFF`

The generic root-record test asserts its different raw header is preserved and
its offset-eight walk consumes 495 records to exact EOF. The 166-profile
sequence classifier must reject its first type-`0x01` candidate
deterministically as unsupported/malformed for that profile. No test derives a
120-byte stride from root word 3. Supporting that form requires a separate
evidence/design checkpoint and profile.

# Synthetic tests

Focused future tests cover:

- arbitrary eight-byte root header preserved losslessly;
- valid zero-length and unknown-type records;
- exact EOF after a record;
- root lengths `0..7`;
- record-header suffixes of length `0..4`;
- declared payload beyond input;
- checked overflow via a factored boundary helper accepting synthetic cursor
  and length values, where allocation-sized input cannot exercise it;
- valid records followed by malformed trailing bytes;
- type-`0x01` candidate failing each sequence validation stage;
- descriptor count below two, multiplication/range failures, and truncated
  descriptors;
- invalid Pascal-name length/end;
- wrong/missing required `0x02/0x29` ordering and malformed terminal;
- Sequence-I-like 11-descriptor/10-pair acceptance with unresolved bindings;
- equal-count ordinal binding;
- arbitrary opaque labels, blank labels, and non-UTF-8 sequence names;
- Meter/Tempo range arithmetic and track containing-range naming.

# No-scanning guarantee

A mandatory regression constructs valid framing up to the current cursor,
places a malformed record or malformed type-`0x01` sequence candidate there,
and places a fully valid-looking sequence later in the bytes. The parse must
return the error at the current cursor with its absolute provenance. It must
not search for the later type `0x01`, sequence name, descriptor label,
Meter/Tempo pattern, or any event tag.

# Relationship to existing decoders

This parser immediately unlocks automatic exact range supply for bounded
initial Meter and Tempo decoders, exact sequence identity/provenance, and exact
track-primary containing records. It does not invoke the Meter/Tempo decoders
implicitly unless a higher orchestration layer requests that composition.

It does not solve Controller discovery in mixed streams, Channel Pressure or
Pitch Bend run ends, Patch/Note handoff, exact Note-chain ends, mixed-event
classification, or the exact general performance-event exclusive end.

# Explicit exclusions

No sequence-name search, printable-label search, Meter/Tempo/event scan,
hard-coded authentic offset, mixed-event parser, event-family dispatcher,
secondary Meter/Tempo decoder, MIDI export policy, active-track inference,
project mutation, or generalized Studio Vision object model belongs in this
checkpoint.

# Implementation scope

## SUPPORTED

- raw eight-byte root header;
- generic checked records from offset eight to exact EOF;
- unknown opaque record types;
- 166-byte descriptor sequence classification and grouping;
- descriptor/name raw preservation;
- Meter/Tempo exact bound supply;
- track pair preservation, equal-count ordinal association, mismatch-safe
  unresolved representation;
- track-primary exact containing bounds and candidate payload-`+14` start.

## UNSUPPORTED

- semantic sequence classification for the older 120-byte descriptor form;
- automatic descriptor-width/profile selection;
- malformed-record recovery or scanning;
- active/inactive descriptor inference;
- exact inner performance-event ends and mixed-event parsing;
- general Meter/Tempo maps and secondary-copy semantics.

## PRESERVED OPAQUELY

- root words;
- unknown record types and payloads;
- descriptor fields and non-text bytes;
- secondary Meter/Tempo records;
- unresolved descriptor/pair associations;
- track-primary content outside established navigation fields.

# Remaining unknowns

Root-word meanings, a universal descriptor-width selector, the older sequence
profile, record-type semantic names, Sequence I's omitted-pair rule,
descriptor numeric fields, legacy text encoding, secondary record semantics,
track event tails, family-run bounds, and mixed-event transitions remain
unknown.

# Implementation gate

The deliberately narrow root-record/sequence-container parser is
**IMPLEMENTED** for the explicit 166-byte descriptor profile. Root and record
framing are exact; sequence validation is deterministic; Sequence I is
preserved without guessed repair; the older form remains explicitly
unsupported semantically; errors and provenance are retained; and no scanning
is performed. Implementation and test details are recorded in
`ROOT_SEQUENCE_CONTAINER_PARSER_IMPLEMENTATION.md`.
