# Objective

Identify the authentic `Ode to Clarke` / `Track 1` / `Juno-106` Patch event
and compare it structurally with the established Track 3 / JV-1080 and Track 3
#2 / JD-800 events. This is read-only research; no parser or Studio Vision
artifact was changed.

# Source identities

The authentic project is `/Users/kurtheiden/Documents/Phoenix
Research/Controlled Save Experiments/Experiment 007 - Untouched
Baseline/newest STUFF baseline`, 211,468 bytes, SHA-256
`e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`.
FinderInfo is type `MID2`, creator `MIDA`; the sibling StuffIt archive was not
used.

The primary MIDI evidence is `Ode to Clarke Multi All`, 12,141 bytes,
SHA-256 `4f63b34ef92204d4bc5eeb78dbbe7b94d005c1f9ceb57ea0f9809533ad590f29`,
SMF format 1 with 10 tracks and division 480.

# Independent Track 1 identification

The export-local MIDI track index 1 independently names `Track 1` and
instrument `Juno-106`. It has channel 1 PC 61 at tick 0 followed by 91 notes.
The first eight-note property/timing signature has two project hits because
that musical phrase repeats later within one continuous track. Only the first
hit, at `0x2f853`, follows a coherent tick-zero Patch structure and begins the
strict 91-note walk; the later `0x2f961` hit is an internal phrase repetition.

# Track 1 MIDI evidence

The first ten exported notes, independently re-read, are:

| Start | Pitch | Attack | Release | Duration |
|---:|---:|---:|---:|---:|
| 9,720 | 73 | 81 | 80 | 61 |
| 10,080 | 72 | 68 | 56 | 75 |
| 10,560 | 68 | 96 | 82 | 61 |
| 11,040 | 63 | 81 | 56 | 88 |
| 11,280 | 65 | 87 | 80 | 88 |
| 12,960 | 68 | 96 | 60 | 77 |
| 13,200 | 63 | 114 | 70 | 55 |
| 17,280 | 73 | 88 | 49 | 101 |
| 21,120 | 73 | 90 | 55 | 99 |
| 21,600 | 72 | 96 | 67 | 93 |

The first note is D-flat5 at `6·1·120`. MIDI channel events before it consist
only of PC 61 at tick 0; no bank-select controllers are exported.

# Track 1 note-region correlation

A strict walk from first-note properties at `0x2f853` accounts for all 91
exported notes and stops at `0x2fa73`, immediately before the repeated
post-chain form `ff fb 89 7f ff 2f 00 29 ...`. Without search-ahead or
resynchronization it matches:

- pitch: 91/91;
- attack velocity: 91/91;
- duration: 91/91;
- note-to-note timing: 90/90;
- release velocity: 89/91.

For notes 77 and 86, the SMF uses note-on velocity zero as note-off while the
project property byte is `0x40`. This export-level release representation does
not weaken the location: every pitch, attack, duration, and timing field still
matches across the complete chain.

# Track 1 Patch-region identification

Working backward from status `0x90` at `0x2f852` identifies:

`00 | ff 7c | 19 | 00 00 3d 08 1d | 0b | Empty Patch |
02 33 30 04 ff ff ff | 3d | cb 78 | 90`

The Patch begins at `0x2f833`; one-byte VLQ `00` is absolute position 0.
Payload length 25 at `0x2f836` counts exactly `0x2f837–0x2f84f`, through the
direct PC byte. Post-PC `cb 78` decodes to 9,720, exactly the interval from the
Patch at tick 0 to the first note at tick 9,720.

# Track 1 Patch name

**Established: `Empty Patch`.** Name length `0b` at `0x2f83c` is immediately
followed by 11 ASCII bytes at `0x2f83d–0x2f847`. The string lies inside the
independently located Patch-to-note structure, has the same length-prefixed
relationship as the two established events, and is not assigned from
readability alone.

# Three-event structural map

| Field | Track 1 / Juno-106 | Track 3 / JV-1080 | Track 3 #2 / JD-800 |
|---|---|---|---|
| position | `0x2f833`: `00` = 0 | `0x31300–01`: `83 60` = 480 | `0x31886–87`: `84 12` = 530 |
| marker | `0x2f834–35`: `ff 7c` | `0x31302–03`: `ff 7c` | `0x31888–89`: `ff 7c` |
| payload length | `0x2f836`: `19` = 25 | `0x31304`: `13` = 19 | `0x3188a`: `1b` = 27 |
| pre-name context | `0x2f837–3b`, 5: `00 00 3d 08 1d` | `0x31305–09`, 5: `00 01 9d f9 1d` | `0x3188b–8f`, 5: `00 00 17 00 17` |
| name length | `0x2f83c`: 11 | `0x3130a`: 5 | `0x31890`: 12 |
| name | `0x2f83d–47`: `Empty Patch` | `0x3130b–0f`: `Wavox` | `0x31891–9c`: `Ming Dynasty` |
| post-name context | `0x2f848–4e`, 7: `02 33 30 04 ff ff ff` | `0x31310–16`, 7: `02 33 30 04 ff 51 02` | `0x3189d–a4`, 8: `03 49 33 38 04 ff ff ff` |
| direct PC | `0x2f84f`: `3d` = 61 | `0x31317`: `1d` = 29 | `0x318a5`: `17` = 23 |
| post-PC VLQ | `0x2f850–51`: `cb 78` = 9,720 | `0x31318–19`: `c7 23` = 9,123 | `0x318a6–a7`: `c5 4c` = 8,908 |
| pre-Note context | none | none | `0x318a8–b3`, 12 bytes; ends `81 25` = 165 |
| Note status | `0x2f852`: `90` | `0x3131a`: `90` | `0x318b4`: `90` |

# Invariant versus variable fields

| Feature | Classification | Evidence |
|---|---|---|
| absolute-position encoding role | INVARIANT | 7-bit big-endian VLQ in all three |
| absolute-position width/value | VARIABLE | one byte for zero; two bytes for 480/530 |
| `ff 7c` | INVARIANT | immediately follows each position VLQ |
| payload-length presence/span | INVARIANT | one byte; counts following payload through PC inclusive |
| payload-length relationship | VARIABLE | name + 14 for Track 1/3; name + 15 for Track 3 #2 |
| pre-name context width | INVARIANT | five bytes |
| pre-name context values | VARIABLE | all three differ; semantics unresolved |
| name length | INVARIANT role/width, VARIABLE value | one byte, exact payload length |
| ASCII name | INVARIANT role, VARIABLE length/value | exact bytes follow length |
| post-name context width | VARIABLE | seven bytes for Track 1/3; eight for Track 3 #2 |
| post-name context bytes | PARTIAL | prefixes group Track 1/3; bank tail differs |
| direct PC | INVARIANT role/width, VARIABLE value | byte equals exported/displayed PC |
| post-PC VLQ encoding | INVARIANT | two-byte 7-bit big-endian VLQ |
| post-PC VLQ meaning | PARTIAL | full interval in Track 1/3; first component in Track 3 #2 |
| pre-Note context width/bytes | VARIABLE | absent for Track 1/3; 12 bytes for Track 3 #2 |
| transition to `0x90` | INVARIANT | all enter the proven Note representation |

# Device and instrument correlations

Track 1 and Track 3 share channel 1 and the seven-byte post-name prefix form
`02 33 30 04 ...` despite different Juno-106/JV-1080 metadata. Track 3 #2 is
channel 15/JD-800 and instead uses eight bytes beginning `03 49 33 38 04` plus
12 extra pre-Note bytes. This is a real grouping, but channel and device change
together in only one sample; neither cause is established.

The five pre-name bytes differ even between the two channel-1 events, so their
values are not simply MIDI channel. No documented OMS identity, track-local
identifier, or sequence-local identifier maps safely onto them. The leading
post-name prefixes are plausible patcher/device metadata with **low-to-medium**
confidence; their exact semantics remain unresolved.

# Bank and Program Change evidence

The Track 3 export emits, at tick 480, CC0=81, CC32=2, then PC 29. Its project
post-name tail is `ff 51 02`, where `51` and `02` equal those two bank values.
Track 1 and Track 3 #2 export no bank controllers before their PCs and both
store `ff ff ff` in the corresponding tail. Across three states this is
**strong correlational evidence** that the final two bytes encode bank-select
values with `ff` as absent, but it is not controlled proof that export and
project serialization have identical semantics.

The direct PC byte independently equals 61, 29, and 23. PC is separate from
the bank-tail bytes in all three events.

# Payload-length analysis

| Event | Payload length | Name length | Difference | Accounted structure after length through PC |
|---|---:|---:|---:|---|
| Track 1 | 25 | 11 | 14 | 5 pre-name + 1 name-length + 11 name + 7 post-name + 1 PC |
| Track 3 | 19 | 5 | 14 | 5 + 1 + 5 + 7 + 1 |
| Track 3 #2 | 27 | 12 | 15 | 5 + 1 + 12 + 8 + 1 |

**Span/ownership understood: YES for the local payload.** The byte counts the
complete following name-bearing payload through PC, exclusive of the post-PC
VLQ. Its conceptual ownership within a larger Patch/container record remains
PARTIAL.

# Post-PC timing analysis

| Event | Patch | First note | Difference | Post-PC field | Result |
|---|---:|---:|---:|---|---|
| Track 1 | 0 | 9,720 | 9,720 | `cb 78` = 9,720 | direct interval |
| Track 3 | 480 | 9,603 | 9,123 | `c7 23` = 9,123 | direct interval |
| Track 3 #2 | 530 | 9,603 | 9,073 | `c5 4c` = 8,908 | needs later `81 25` = 165; sum 9,073 |

All three use the same VLQ units and encoding. The post-PC field is the whole
Patch-to-first-note interval when no pre-Note context follows. Track 3 #2's
extra context contains the second component. This supports a timing-component
role, but the reason for compound representation and ownership of its 12-byte
context remain unresolved.

# Existing decoder result

The unchanged decoder was invoked with defensible start `0x2f833` and exclusive
bound `0x2f853`, including Note status. It failed immediately and usefully:

`UnexpectedPositionWidth { offset: 0x2f833, bytes: 1 }`

The Track 3 #2-specific decoder requires a two-byte position; no alternate
start, recovery, relaxed check, or code change was attempted.

# Common semantic core assessment

- **A. Common absolute-position encoding: YES.** Width varies minimally.
- **B. Common `ff 7c` marker: YES.** Exact placement is invariant.
- **C. Common one-byte name length: YES.** Exact payload count in all three.
- **D. Common variable-length ASCII Patch name: YES.** Three names/lengths.
- **E. Common direct Program Change: YES.** Matches independent MIDI evidence.
- **F. Common post-PC VLQ field: YES.** Encoding/units recur; whole-versus-
  component semantics are PARTIAL.
- **G. Common Note transition: YES.** Every structure reaches `0x90`.
- **H. Common fixed local Patch layout: NO.** Position width, post-name width,
  bank values, and pre-Note context vary.
- **I. Common semantic Patch core despite variable framing: YES.** Three
  independent authentic events support it, without proving universality.

# Implications for decoder design

Potential shared bounded invariants are explicit bounds, a VLQ position,
`ff 7c`, one-byte payload length spanning through PC, five-byte opaque pre-name
context, one-byte ASCII-name length, variable name, direct final payload PC,
two-byte post-PC VLQ, and eventual transition to `0x90`.

Track 3 #2-specific constraints are two-byte position width, payload equal to
name length + 15, exact pre/post-name constants, exact 12-byte pre-Note
context, and fixed placement of PC/status. The post-name bank tail is
potentially bank/patcher-specific. Meanings of pre-name context, leading
post-name identifier bytes, compound timing context, and event-type semantics
remain unresolved. A future shared design should preserve opaque context and
validate measured lengths/relationships, not silently discard checks.

# Evidence supported

- Track 1 is independently identified from 91 notes and MIDI metadata.
- `Empty Patch`, position 0, direct PC 61, and first-note transition are
  structurally established.
- A common semantic Patch core recurs across Juno-106, JV-1080, and JD-800.
- The local payload length has an exact common span through PC.
- Bank-tail bytes correlate with exported bank controllers in all three cases.
- Fixed local framing does not generalize across all three.
- Existing decoder and parser code remain unchanged.

# Unknowns

Pre-name field semantics, post-name identifier semantics, whether bank-tail
correlation is exact serialization, why JD-800 uses extra timing context,
complete Patch/container boundaries, event-type discrimination, and behavior
in other files/versions remain unknown.

# Single recommended next step

Design—but do not yet implement—a bounded representation-oriented decoder
contract for the common semantic fields that preserves and returns opaque
pre-name, post-name, and pre-Note context. It should retain explicit caller
bounds and length/transition validation while separating Track 3 #2-specific
constants from the three-event invariants.

That design is now complete in `BOUNDED_PATCH_DECODER_DESIGN.md`. It specifies
explicit position start and post-`90` exclusive bounds, payload-derived PC
placement, borrowed opaque contexts with absolute ranges, variable-width VLQs,
deterministic errors, a predefined test matrix, and a future strict-wrapper
relationship for the existing Track 3 #2 decoder. No implementation was made.
