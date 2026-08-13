# Objective

Design, without implementing, a bounded representation-oriented decoder for
the semantic Patch core measured in three authentic `Ode to Clarke` events.
The design separates common structure from variable framing, preserves unknown
bytes and their provenance, and retains the existing Track 3 #2 decoder's
strict validation value.

# Evidence basis

The design is derived from these independently identified representations:

| Track / device | Position | Payload | Name | PC | Post-PC VLQ | Pre-Note bytes |
|---|---|---:|---|---:|---:|---:|
| Track 1 / Juno-106 | `00` = 0 | 25 | `Empty Patch` (11) | 61 | 9,720 | 0 |
| Track 3 / JV-1080 | `83 60` = 480 | 19 | `Wavox` (5) | 29 | 9,123 | 0 |
| Track 3 #2 / JD-800 | `84 12` = 530 | 27 | `Ming Dynasty` (12) | 23 | 8,908 | 12 |

All three place `ff 7c` after the position, use five opaque bytes before the
name length, make the local payload length span every following byte through
PC inclusive, and eventually transition to Note status `90`. Controlled
Experiments 023–027 independently establish Track 3 #2's position, variable
ASCII name, direct PC, and payload relocation behavior.

# Scope and non-goals

Proposed name: `decode_bounded_patch_representation`.

It would decode one caller-located member of the three-event representation
family. It would not discover events, scan a file, identify a track, parse a
complete Patch grammar, establish full event/container ownership, interpret
opaque context, decode other event types, emit MIDI, or claim universality
across Studio Vision files or versions.

Before calling it, the caller must already know:

- the exact absolute-position VLQ start;
- an exclusive boundary immediately after the expected first-Note `90` status;
- that the bounded bytes are an evidence-backed Patch-to-Note representation.

# Existing decoder audit

`decode_known_track3_2_patch` currently makes these checks:

| Current assumption or validation | Classification | Evidence and future treatment |
|---|---|---|
| caller `start < end <= bytes.len()` | COMMON INVARIANT | Required for every bounded read. |
| position is a bounded 7-bit big-endian VLQ | COMMON INVARIANT | Values 0, 480, 530 use the established encoding. |
| position consumes exactly two bytes | TRACK-3-#2-SPECIFIC | Track 1 position zero consumes one byte. |
| `ff 7c` immediately follows position | COMMON INVARIANT | Exact in all three. |
| next byte is local payload length | COMMON INVARIANT | All three; span through PC is measured. |
| payload length equals name length + 15 | TRACK-3-#2-SPECIFIC | Track 1/3 use name + 14; future parser must derive the payload end. |
| pre-name bytes equal `00 00 17 00 17` | TRACK-3-#2-SPECIFIC | Width five is common; values differ in all three and remain opaque. |
| name length is one byte | COMMON INVARIANT | Exact in all three and controlled Experiment 027. |
| exactly `name_length` bytes form the name | COMMON INVARIANT | Three names and controlled 12-to-7 change. |
| name must be ASCII | COMMON INVARIANT within evidence | All observed names are direct ASCII; invalid data should fail rather than be silently transformed. |
| post-name bytes equal Track 3 #2's eight-byte constant | TRACK-3-#2-SPECIFIC | Track 1/3 have seven bytes and varying bank-correlated tails. |
| PC follows that fixed constant | TRACK-3-#2-SPECIFIC placement method | Common rule is stronger: PC is the final payload byte. |
| PC is returned as an unconstrained `u8` | EVIDENCE-SUPPORTED BUT VARIABLE | Direct values 23/24/100/29/61; current code does not impose MIDI range. |
| a VLQ immediately follows PC | COMMON INVARIANT | Present in all three. |
| post-PC VLQ consumes exactly two bytes | EVIDENCE-SUPPORTED BUT VARIABLE | All three samples use two; no width-control experiment proves this is universal. |
| post-PC VLQ has interval semantics | UNRESOLVED / SHOULD NOT BE SEMANTICALLY VALIDATED | It is the whole interval twice and one component once. Return value and provenance only. |
| pre-Note bytes equal the Track 3 #2 12-byte constant | TRACK-3-#2-SPECIFIC | Track 1/3 have none; meanings of the 12 bytes are unresolved. |
| first Note status is `90` at the derived cursor | COMMON INVARIANT | Exact transition in all three and controlled files. |
| no recovery/resynchronization | COMMON INVARIANT | Necessary to keep the result evidence-bounded. |

# Common structural contract

The proposed decoder validates this ordered structure:

`position VLQ | ff 7c | payload_length | payload | post-PC VLQ |
opaque pre-Note context | 90`

Within `payload`:

`opaque pre-name[5] | name_length | ASCII name[name_length] |
opaque post-name[variable] | PC[1]`

The role and width of the two opaque regions differ: pre-name width five is
common evidence; post-name width is derived from the payload boundary and is
not fixed. Values are preserved without semantic validation.

# Caller input contract

Illustrative required input:

```rust
pub struct PatchRepresentationBounds {
    pub position_start: usize,
    /// Exclusive; bytes[note_status_end - 1] must be 0x90.
    pub note_status_end: usize,
}

pub fn decode_bounded_patch_representation<'a>(
    bytes: &'a [u8],
    bounds: PatchRepresentationBounds,
) -> Result<BoundedPatchRepresentation<'a>, BoundedPatchError>;
```

The bounds must satisfy `position_start < note_status_end <= bytes.len()`.
No expected semantic values are required. Optional constraints such as an
expected name, PC, position, or exact opaque bytes belong in calling
diagnostics or a strict wrapper, not the common parser. The decoder fails at
the first structural violation and never searches, skips, retries, or
resynchronizes.

# Payload framing

After position and marker, the decoder reads one unsigned payload-length byte.
Using checked arithmetic, it sets:

`payload_start = payload_length_offset + 1`

`payload_end = payload_start + payload_length`

`program_change_offset = payload_end - 1`

It rejects overflow, a payload end beyond `note_status_end - 1`, and any
payload too short to contain five pre-name bytes, one name-length byte, and one
PC byte. The minimum structurally parseable payload is therefore **7 bytes**:
five opaque pre-name bytes, zero-length name, and PC, with no post-name bytes.
No observed event is that small; this is the mathematical safe minimum and
does not assert that zero post-name context occurs in Studio Vision.

After the five bytes and name-length byte, `name_end` must be no later than
`program_change_offset`. Bytes from `name_end` to `program_change_offset` are
the opaque post-name context. Thus all lengths remain inside both the payload
and caller boundary, and PC is found without a hardcoded context width.

# Variable-width position

The position uses the existing bounded 7-bit big-endian VLQ mechanics. The
future decoder should accept one through four bytes, matching
`track7::MAX_VLQ_BYTES`, and return both `u32` value and exact source range.
This covers `00`, `83 60`, and `84 12` without a Patch-specific width rule.

Continuation beyond four bytes, unterminated input, or a read crossing the
caller boundary fails. The existing helper does not reject non-minimal but
syntactically terminated encodings; the shared Patch design should not add a
canonical-form rule without evidence. Raw range/bytes make any future
non-minimal observation visible.

# Patch-name representation

The name length is one byte and the payload must contain exactly that many
bytes before its final PC byte. Zero length is structurally safe and should be
accepted; no empty-name sample is yet established. Maximum length is bounded
by both the one-byte length and remaining payload space, not a fixed Patch-name
constant.

The first version should require ASCII and return both borrowed raw bytes and
a borrowed `&str`, plus absolute ranges for length and content. It must not
look for NUL, strip padding, replace invalid bytes, or assume a fixed width.
Non-ASCII data produces an explicit error; relaxing encoding would require new
evidence.

# Program Change representation

The direct PC is the final byte before `payload_end`, so its location is
derived solely from the measured payload span. It is returned with its absolute
offset as `u8`.

The common structural decoder should not reject values 128–255 in v1. MIDI
Program Change nominally permits 0–127, but controlled project evidence only
establishes direct stored values within that range; it does not establish how
malformed or non-MIDI project bytes should be classified. A strict consumer
may separately require `program_change <= 127` and report a semantic
validation error without changing structural decoding.

# Post-PC timing field

The decoder starts a bounded VLQ exactly at `payload_end`, returning its raw
range and decoded `u32` value under a neutral name such as
`post_pc_timing_component`. It must not call the value the complete
Patch-to-first-note interval.

Although all three samples use exactly two bytes, v1 should accept one through
four bytes using the compatible bounded VLQ helper. Variable-width position
demonstrates that field width follows encoded value elsewhere, while no
evidence establishes two bytes as a framing constant. The observed width
remains available diagnostically; the strict Track 3 #2 wrapper may continue
requiring two.

# Opaque context preservation

The result should borrow bytes rather than copy them and pair every slice with
an absolute `Range<usize>`. This supports byte-exact comparison, provenance,
and zero-copy diagnostics:

- `pre_name_context`: exactly five bytes;
- `post_name_context`: payload remainder between name and PC;
- `pre_note_context`: bytes after the post-PC VLQ and before final `90`.

None should be parsed, normalized, or discarded. Diagnostic code may inspect
candidate VLQs inside `pre_note_context`, including Track 3 #2's final
`81 25`, but heuristic VLQ scanning is explicitly outside the decoder.

# Note-transition boundary

`note_status_end` is not a loose search limit. It is a caller assertion that
the expected status occupies `note_status_end - 1`. The decoder verifies that
exact byte is `90`. After parsing the post-PC VLQ, all bytes from its end to
`note_status_end - 1` become opaque pre-Note context. It rejects a post-PC VLQ
that reaches or crosses the status offset.

This deterministically supports zero context for Track 1/3 and 12 bytes for
Track 3 #2. The decoder never scans forward for another `90`, so an embedded
`90` inside opaque context has no special meaning.

# Proposed Rust API

Illustrative only:

```rust
use std::ops::Range;

pub struct LocatedBytes<'a> {
    pub bytes: &'a [u8],
    pub range: Range<usize>,
}

pub struct LocatedVlq<'a> {
    pub value: u32,
    pub bytes: &'a [u8],
    pub range: Range<usize>,
}

pub struct LocatedAscii<'a> {
    pub text: &'a str,
    pub bytes: &'a [u8],
    pub range: Range<usize>,
}

pub struct LocatedByte {
    pub value: u8,
    pub offset: usize,
}

pub struct BoundedPatchRepresentation<'a> {
    pub representation_range: Range<usize>,
    pub position: LocatedVlq<'a>,
    pub marker_range: Range<usize>,
    pub payload_length: LocatedByte,
    pub payload_range: Range<usize>,
    pub pre_name_context: LocatedBytes<'a>,
    pub name_length: LocatedByte,
    pub name: LocatedAscii<'a>,
    pub post_name_context: LocatedBytes<'a>,
    pub program_change: LocatedByte,
    pub post_pc_timing_component: LocatedVlq<'a>,
    pub pre_note_context: LocatedBytes<'a>,
    pub note_status: LocatedByte,
}
```

The API intentionally has no generic event enum, instrument type, bank fields,
MIDI output, or discovery method.

# Proposed error model

```rust
pub enum BoundedPatchError {
    InvalidBounds { start: usize, end: usize, size: usize },
    PositionVlq(VlqError),
    MissingMarker { offset: usize, observed: Vec<u8> },
    PayloadLengthOverflow { offset: usize, length: u8 },
    PayloadExceedsBoundary { payload_end: usize, status_offset: usize },
    PayloadTooShort { offset: usize, length: u8, minimum: u8 },
    NameLengthExceedsPayload { offset: usize, length: u8, pc_offset: usize },
    InvalidAsciiName { range: Range<usize> },
    MissingProgramChange { payload_range: Range<usize> },
    PostPcVlq(VlqError),
    PostPcVlqCrossesStatus { range: Range<usize>, status_offset: usize },
    MissingNoteStatus { offset: usize, expected: u8, observed: Option<u8> },
}
```

`InvalidBounds`, the underlying `VlqError`, and broad bounded-read helpers may
be shared in implementation. Current `UnexpectedPositionWidth`,
`InvalidLocalPayloadLength` as `name + 15`, exact `UnexpectedBytes` contexts,
`UnexpectedUnresolvedIntervalWidth`, and the Track 3 #2-specific transition
path remain wrapper-level errors/checks. Error values should retain offsets
and observed bytes; no error triggers recovery.

# Compatibility with Track 3 #2 decoder

Recommendation: eventually make `decode_known_track3_2_patch` a thin strict
wrapper around the shared bounded decoder, after the shared implementation and
predeclared tests pass. The wrapper would additionally require:

- two-byte position and post-PC VLQ;
- payload length equal to name length + 15;
- exact Track 3 #2 pre-name, post-name, and 12-byte pre-Note contexts;
- the existing returned `PatchDiagnostic` shape and errors where compatibility
  matters.

This preserves Experiments 007 and 023–027 as strict regression evidence while
avoiding duplicate cursor/bounds mechanics. During implementation, both APIs
may coexist until equivalence tests prove the wrapper retains every current
acceptance and rejection behavior. The old decoder is not removed by this
design.

# Test plan

Tests are specified before implementation.

## Authentic positive cases

| Case | Required assertions |
|---|---|
| Track 1 / Juno-106 | position 0/range `0x2f833..34`; payload 25; name `Empty Patch`; PC 61 at `0x2f84f`; post-PC 9,720; empty pre-Note context; status `0x2f852` |
| Track 3 / JV-1080 | position 480/range `0x31300..02`; payload 19; name `Wavox`; PC 29 at `0x31317`; post-PC 9,123; empty pre-Note context; status `0x3131a`; preserve `ff 51 02` tail |
| Track 3 #2 / JD-800 | position 530/range `0x31886..88`; payload 27; name `Ming Dynasty`; PC 23 at `0x318a5`; post-PC 8,908; preserve 12 pre-Note bytes; status `0x318b4` |

## Controlled positive cases

| Case | Key assertions |
|---|---|
| Experiment 007 | position 530, `Ming Dynasty`, PC 23, established offsets/context |
| Experiment 023 | same position/name/context; PC 24 |
| Experiment 024 | same position/name/context; PC 100 |
| Experiment 025 | position 531; name/PC unchanged; post-PC component 8,907 |
| Experiment 026 | `Phoenix Test` length 12; position 530; PC 23 |
| Experiment 027 | payload 22; `Phoenix` length 7; PC/status ranges relocated by -5; opaque bytes preserved |

Each shared positive test also asserts marker, payload range, five-byte
pre-name context, raw post-name context, post-PC range, pre-Note range, and
exact final status boundary. Existing strict-wrapper tests must continue
passing unchanged.

## Malformed synthetic cases

- position offset outside bounds, truncated continuation, and more than four
  VLQ bytes;
- wrong or truncated `ff 7c` marker;
- payload arithmetic overflow or payload end beyond caller status boundary;
- payload length below seven;
- name length crossing the derived PC offset;
- non-ASCII name;
- payload with no byte available for PC;
- missing/truncated post-PC VLQ or a VLQ crossing the status offset;
- wrong/missing `90` exactly at `note_status_end - 1`;
- an earlier `90` inside opaque pre-Note context, proving no scan occurs.

# Bank-field decision

Choose **A: expose the last three post-name bytes only as opaque context** in
v1. Track 3's `ff 51 02` correlates strongly with exported CC0=81/CC32=2, and
the two no-bank cases contain `ff ff ff`, but this is not controlled proof of
field semantics. The common decoder should return the whole post-name region
and its range. A separate diagnostic may present the final three bytes as a
bank candidate without placing candidate semantics in the core API.

# Payload-length decision

Return payload length and payload range as **diagnostic framing metadata**, not
a musical semantic value. Its byte width and exact span through PC are strongly
established and essential to safe parsing. Its ownership inside the larger
event/container remains partial, so calling it a first-class Patch semantic or
record length would overclaim.

# Implementation readiness

- **A. Common semantic core sufficiently established: YES.** Three authentic
  events plus five controlled variants support a bounded implementation.
- **B. Patch discovery established: NO.** Callers must provide exact bounds.
- **C. Complete Patch-event boundary ownership established: PARTIAL.** The
  position-through-Note-transition representation is bounded; surrounding
  record/container ownership is not.
- **D. Bank-select semantics established: PARTIAL.** Strong three-state
  correlation, no controlled proof.
- **E. Post-PC timing semantics complete: PARTIAL.** Same encoding/units; full
  interval twice and a component once.
- **F. Opaque context can be preserved safely: YES.** Caller bounds and derived
  ranges make zero-copy preservation deterministic.
- **G. Existing strict decoder can remain a validation wrapper: YES.** Its
  extra checks are cleanly separable from common parsing.

# Evidence supported

- A deterministic decoder can locate semantic fields without hardcoded
  context widths or scanning.
- Payload length safely locates PC as its final byte.
- Caller-supplied status-end safely bounds optional pre-Note context.
- Variable-width position and opaque context support all three authentic
  samples.
- Provenance-rich borrowed ranges preserve every uninterpreted byte.
- Strict Track 3 #2 validation remains available above the common contract.

# Unknowns

Discovery, complete event/container ownership, event-type semantics, opaque
pre-name and post-name meanings, exact bank serialization, compound timing
grammar, non-ASCII/empty names in authentic projects, PC bytes above 127,
non-minimal VLQs, and generality beyond the three events/files/versions remain
unknown.

# Single recommended next step

Implement `decode_bounded_patch_representation` and the complete predefined
positive/malformed test matrix, then refactor the existing Track 3 #2 decoder
into a strict wrapper only after equivalence tests demonstrate unchanged
controlled-file behavior.
