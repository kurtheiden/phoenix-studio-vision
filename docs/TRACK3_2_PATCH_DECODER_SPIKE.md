# Objective

Implement a diagnostic-only decoder for the known first Patch representation
in `Ode to Clarke` / `Track 3 #2`, bounded by explicit caller-supplied evidence
offsets. The spike validates the confirmed fields across Experiments 007 and
023–027 without implementing discovery, a general Patch grammar, or MIDI
emission.

# Evidence boundary

The public library function is
`patch::decode_known_track3_2_patch(bytes, start, end)`. The caller must provide
the exact start of the known absolute-position VLQ and an exclusive upper
bound. The function never scans or resynchronizes. Controlled validation uses
`start = 0x31886`; production logic contains no experiment-specific semantic
values or file paths.

The decoder is library/test-only. No CLI command was added because exposing a
file-level Patch command would imply unsupported discovery and generality.

# Supported representation

The decoder supports only the experimentally confirmed Track 3 #2 first Patch
layout:

- two-byte 7-bit big-endian absolute-position VLQ;
- evidence-stable local layout leading to a one-byte name length;
- exactly that many ASCII name bytes;
- evidence-stable local context leading to the direct PC byte;
- a bounded two-byte VLQ-shaped field whose semantics remain unresolved;
- stable local transition context ending at expected Note status `0x90`.

The local name-bearing payload length is reported diagnostically and validated
as `name length + 15`. Its broader ownership is not generalized.

# Input contract

The decoder requires:

- the complete byte slice;
- explicit known Patch start offset;
- explicit exclusive evidence bound.

It rejects invalid or out-of-range bounds, malformed or wrong-width position
VLQs, truncation, inconsistent local payload length, non-ASCII names,
unexpected evidence-stable layout bytes, malformed/wrong-width unresolved
interval-shaped bytes, and a missing `0x90` first-note transition. There is no
search-ahead, fallback, recovery, or hidden resynchronization.

# Decoded fields

`PatchDiagnostic` reports:

- absolute position, offset, and encoded width;
- conservative local payload length and offset;
- name length and offset;
- decoded ASCII name and payload offset;
- direct Program Change value and offset;
- first-note status and transition offset.

No semantics are assigned to unresolved metadata or the compound
Patch-to-first-note timing relationship.

# Variable-length name handling

The decoder reads the one-byte name length and advances by exactly that many
bytes before validating the following known context. Program Change and Note
transition offsets are derived from this cursor, not hardcoded. Experiment 027
therefore reports `Phoenix`, PC offset `0x318a0`, and Note status offset
`0x318af`; the 12-character controls report `0x318a5` and `0x318b4`.

# Controlled-file validation

Six read-only integration tests decode the external controlled artifacts:

| Artifact | Position | Name | PC |
|---|---:|---|---:|
| Experiment 007 | 530 | `Ming Dynasty` | 23 |
| Experiment 023 | 530 | `Ming Dynasty` | 24 |
| Experiment 024 | 530 | `Ming Dynasty` | 100 |
| Experiment 025 | 531 | `Ming Dynasty` | 23 |
| Experiment 026 | 530 | `Phoenix Test` | 23 |
| Experiment 027 | 530 | `Phoenix` | 23 |

The values are derived from artifact bytes. Expected values and file paths
exist only in integration tests, not decoder logic.

# First-note transition validation

Every controlled state reaches and validates status `0x90`:

| Artifact | Status offset | Value |
|---|---:|---:|
| Experiments 007, 023, 024, 025, 026 | `0x318b4` | `0x90` |
| Experiment 027 | `0x318af` | `0x90` |

Experiment 027's `-5` transition relocation is derived through the variable
name length. A focused negative test verifies rejection of a missing or altered
transition.

# Error handling

`PatchError` provides structured failures for bounds, absolute-position VLQ,
position width, truncation, unexpected known context, inconsistent payload
length, non-ASCII names, unresolved interval-shaped field encoding/width, and
the first-note transition. Focused unit tests cover baseline/equal/short-name
layouts, PC 24, PC 100, position 531, truncated and non-ASCII names, malformed
position VLQ, and missing Note transition.

# What remains intentionally unsupported

- whole-file Patch discovery or heuristic scanning;
- a general Patch event grammar or general track parser;
- exact event-type discriminator semantics;
- semantic ownership of the compound Patch-to-first-note interval;
- unknown metadata fields or complete Patch-event ownership;
- other Patch events, projects, Studio Vision versions, or event types;
- MIDI emission.

The reused VLQ helper is the existing bounded `track7::decode_7bit_be_vlq`.
No existing helper or Track 7 behavior was refactored.

# Evidence supported

- Explicit bounds are sufficient to decode the confirmed known representation.
- Absolute position, variable-length ASCII name, and direct PC decode correctly
  across all six controlled saves.
- Program and Note transition offsets follow the variable-length name rather
  than a baseline constant.
- All malformed/truncated inputs fail without recovery.
- Existing production/general parser and CLI behavior are unchanged.

# Unknowns

Complete Patch-event semantic boundaries, exact event-type discriminator,
unresolved metadata meanings, compound interval ownership, automatic track or
event selection, and generality to another Patch event remain unknown.

# Single recommended next step

Validate the same explicitly bounded Patch representation on one independently
identified real Patch event in another track. This is the smallest
highest-information test of whether the confirmed layout generalizes before
considering any discovery or broader Patch abstraction.

The subsequent independent validation selected authentic `Ode to Clarke` /
`Track 3` / `JV-1080`, whose MIDI export and note stream identify a `Wavox`, PC
29 Patch region at `0x31300`. Core position, length-prefixed ASCII name, direct
PC, post-PC VLQ shape, and Note transition relationships recur, but pre-name,
post-name, payload-length, and pre-Note context differ. The unchanged decoder
correctly fails at `0x31305`; no generalization is yet justified. See
`INDEPENDENT_PATCH_EVENT_VALIDATION.md`.
