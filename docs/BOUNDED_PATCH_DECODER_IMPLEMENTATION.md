# Objective

Implement the previously designed `decode_bounded_patch_representation` as a
library-level, caller-bounded decoder for the common semantic Patch core. The
implementation adds no discovery, general Patch grammar, CLI, MIDI emission,
or Studio Vision artifact mutation.

# Design contract implemented

The implementation follows `BOUNDED_PATCH_DECODER_DESIGN.md` without a
contract contradiction. It decodes, in order:

`position VLQ | ff 7c | payload length | payload | post-PC VLQ |
opaque pre-Note bytes | 90`

The payload is deterministically divided into five opaque pre-name bytes, one
name-length byte, exactly that many ASCII name bytes, variable opaque
post-name bytes, and the final payload byte as direct PC.

# API

`src/patch.rs` now exposes:

- `PatchRepresentationBounds`;
- `LocatedBytes`, `LocatedVlq`, `LocatedAscii`, and `LocatedByte`;
- `BoundedPatchRepresentation`;
- `BoundedPatchError`;
- `decode_bounded_patch_representation(bytes, bounds)`.

Semantic values are paired with absolute offsets/ranges. Variable and unknown
regions borrow the input slice rather than copying it.

# Bounds and no-scan behavior

The caller supplies the exact position start and `note_status_end`, an
exclusive boundary immediately after the expected `0x90`. Bounds must be
non-empty and within the byte slice. The decoder never scans, searches,
retries, skips, or resynchronizes. It validates `0x90` only at `end - 1`; an
earlier `0x90` remains ordinary opaque context.

# Payload framing

The byte after `ff 7c` is read as payload length. Checked arithmetic derives
payload start/end, and PC is located as `payload_end - 1`. Payloads shorter
than seven bytes or extending beyond the terminal status boundary fail.
Internal name ranges are checked against the derived PC offset. No
`name + 14` or `name + 15` parsing rule is used.

# Position handling

The existing bounded 7-bit big-endian VLQ helper is reused. One through four
bytes are accepted and returned with raw bytes/range and decoded `u32`. This
decodes authentic `00`, `83 60`, and `84 12`; malformed, unterminated, and
longer encodings fail.

# Name handling

Exactly five opaque pre-name bytes precede a one-byte name length. Exactly the
declared number of bytes are borrowed, must be ASCII, and are returned as both
raw bytes and `&str` with an absolute range. Zero length is accepted
structurally. No terminator, padding, or fixed width is assumed.

# Program Change handling

PC is always the final payload byte and is returned with its absolute offset.
The structural decoder accepts the full `u8` range and does not impose MIDI
semantic validation.

# Opaque context

Pre-name, post-name, and pre-Note bytes are returned byte-exactly as borrowed
slices with absolute ranges. Track 3's bank-correlated `ff 51 02` remains
opaque. Tests cover empty and non-empty pre-Note context and an embedded
earlier `0x90`.

# Post-PC timing component

A bounded one-to-four-byte VLQ is decoded immediately at payload end and
returned neutrally as `post_pc_timing_component`, with raw bytes/range. The API
does not claim it is the complete Patch-to-first-note interval.

# Note-transition handling

The final byte at the exact caller-defined boundary must be `0x90`. Bytes
between the post-PC VLQ end and that offset are preserved as opaque pre-Note
context. There is no status search.

# Authentic validation

| Event | Position | Name | PC / offset | Post-PC | Pre-Note | Status |
|---|---:|---|---|---:|---:|---|
| Track 1 / Juno-106 | 0 | `Empty Patch` | 61 / `0x2f84f` | 9,720 | 0 | `0x2f852` |
| Track 3 / JV-1080 | 480 | `Wavox` | 29 / `0x31317` | 9,123 | 0 | `0x3131a` |
| Track 3 #2 / JD-800 | 530 | `Ming Dynasty` | 23 / `0x318a5` | 8,908 | 12 | `0x318b4` |

Tests also assert raw/range provenance for all opaque regions, including Track
3's `ff 51 02` and Track 3 #2's exact 12 pre-Note bytes.

# Controlled validation

Read-only tests derive all required semantic states:

| Experiment | Position | Name | PC | PC offset | Status offset |
|---|---:|---|---:|---:|---:|
| 007 | 530 | `Ming Dynasty` | 23 | `0x318a5` | `0x318b4` |
| 023 | 530 | `Ming Dynasty` | 24 | `0x318a5` | `0x318b4` |
| 024 | 530 | `Ming Dynasty` | 100 | `0x318a5` | `0x318b4` |
| 025 | 531 | `Ming Dynasty` | 23 | `0x318a5` | `0x318b4` |
| 026 | 530 | `Phoenix Test` | 23 | `0x318a5` | `0x318b4` |
| 027 | 530 | `Phoenix` | 23 | `0x318a0` | `0x318af` |

Experiment 025 also derives post-PC 8,907. Experiment 027 derives payload 22
and both relocated offsets from framing rather than production constants.

# Malformed-input validation

Focused synthetic tests cover invalid/out-of-range bounds, truncated and
over-four-byte position VLQs, wrong/truncated marker, payload beyond the
boundary, payload below the seven-byte safe minimum (including no room for
PC), name crossing PC, non-ASCII name, truncated/over-four-byte post-PC VLQ,
wrong/missing terminal status, and an earlier embedded `0x90` that remains
opaque. All errors are deterministic and preserve useful offsets/ranges.

# Strict Track 3 #2 compatibility

The existing decoder remains independent for now. Converting it immediately
would require translating the new error model back into every existing
Track-3-#2-specific error and proving rejection-order compatibility. That
adds risk without improving the shared decoder result. Existing tests remain
unchanged, all six controlled files still pass, and an explicit test confirms
the strict API still rejects authentic Track 1 and Track 3 layouts.

A later focused refactor may make it a wrapper only with dedicated equivalence
tests for both accepted values and failure ordering.

# What remains unsupported

- Patch discovery or whole-file scanning;
- general Studio Vision Patch/event/track parsing;
- complete surrounding record/container ownership;
- bank-field interpretation;
- opaque context semantics;
- complete post-PC timing semantics;
- other projects, versions, or event types;
- MIDI emission or CLI exposure.

# Evidence supported

- **A. Shared bounded decoder implemented: YES.**
- **B. Track 1 authentic event decodes: YES.**
- **C. Track 3 authentic event decodes: YES.**
- **D. Track 3 #2 authentic event decodes: YES.**
- **E. Experiments 007 and 023–027 all decode: YES.**
- **F. Variable position width handled: YES.**
- **G. Variable post-name width handled: YES.**
- **H. Variable pre-Note context handled: YES.**
- **I. Earlier embedded `0x90` safely ignored: YES.**
- **J. Existing Track 3 #2 strict behavior preserved: YES.**
- **K. Patch discovery implemented: NO.**
- **L. General Studio Vision Patch parser implemented: NO.**

# Unknowns

Discovery, complete Patch boundaries, event-type semantics, bank serialization,
opaque-field meanings, compound timing grammar, authentic empty/non-ASCII
names, and generality beyond the validated artifacts remain unknown.

# Single recommended next step

Validate one additional naturally occurring Patch representation using
independently established caller bounds, preferably Track 2 / JV-1080 / PC 37.
This tests the shared contract on another channel/device context without
weakening the explicit-bound requirement or introducing discovery.

That validation is complete: authentic Track 2 / JV-1080 / `Stereoww Bs` / PC
37 decodes unchanged at `0x2fb55..0x2fb75`. Its 211-note chain independently
establishes the bounds, and its `ff 51 01` tail matches exported CC0=81/CC32=1.
See `FOURTH_PATCH_EVENT_VALIDATION.md`.

Experiment 028 independently changes only Track 2's final opaque post-name
byte from `01` to `02`. The shared decoder accepts it unchanged and preserves
`02 33 38 04 ff 51 02` exactly. A read-only regression test records this
controlled state; bank interpretation remains outside the decoder.
