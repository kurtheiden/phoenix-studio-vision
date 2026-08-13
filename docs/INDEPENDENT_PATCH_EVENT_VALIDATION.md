# Objective

Select and test one authentic, naturally occurring Patch event outside `Ode to
Clarke` / `Track 3 #2` against the existing bounded diagnostic decoder. No
Studio Vision artifact or parser code was changed.

# Validation rule

Candidate selection used Studio Vision-generated MIDI evidence before testing
the project bytes. Identification required multiple independent anchors. The
existing decoder was then invoked unchanged with an explicit position-field
start and exclusive upper bound. A structural mismatch was retained as a
failure rather than bypassed or generalized away.

# Source identities

The authentic project is `/Users/kurtheiden/Documents/Phoenix
Research/Controlled Save Experiments/Experiment 007 - Untouched
Baseline/newest STUFF baseline`, 211,468 bytes, SHA-256
`e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`.
FinderInfo identifies type `MID2` and creator `MIDA`. The sibling 89,525-byte
StuffIt archive is not the inspected project.

The Studio Vision MIDI sources are:

| Export | Size | SHA-256 | SMF layout |
|---|---:|---|---|
| `Ode to Clarke` | 8,644 | `eb37711a81eee7d78877bfe2ca67712ac2b98067cbec9e23f9f8e739380bf5a6` | format 0, 1 track, division 480 |
| `Ode to Clarke Multi All` | 12,141 | `4f63b34ef92204d4bc5eeb78dbbe7b94d005c1f9ceb57ea0f9809533ad590f29` | format 1, 10 tracks, division 480 |
| `Ode to Clarke Multitrack` | 10,514 | `9979ed6d5fc58edb85c3c03e5e43b4c7015a353af6075ece8ba94ac49cbf5059` | format 1, 8 tracks, division 480 |

# Program Change inventory

The `Multi All` export supplies the clearest per-track inventory. MIDI indices
below are zero-based chunk indices; they are not assumed to be Studio Vision
List Window numbers.

| MIDI index | Track name | Instrument meta | Channel | Tick / position | PC | Following note context |
|---:|---|---|---:|---|---:|---|
| 1 | `Track 1` | `Juno-106` | 1 | 0 / `1·1·0` | 61 | 91 notes; first D-flat5/81 at 9,720 (`6·1·120`) |
| 2 | `Track 2` | `JV-1080` | 2 | 0 / `1·1·0` | 37 | 211 notes; first G-sharp2/90 at 1,920 (`2·1·0`) |
| 6 | `Track 3` | `JV-1080` | 1 | 480 / `1·2·0` | 29 | 84 notes; first C-sharp5/78 at 9,603 (`6·1·3`) |
| 8 | `Track 3 #2` | `JD-800` | 15 | 530 / `1·2·50` | 23 | known source event; 84 notes; first C-sharp5/100 at 9,603 |

The format-0 export combines PC 37 at tick 0 and PC 23 at tick 530. The
eight-track `Multitrack` export contains PC 37 and PC 23 but omits `Track 1`
and `Track 3`. This export difference is why MIDI chunk numbering is treated
only as export-local evidence.

# Candidate selection

1. **Selected: `Track 3` / JV-1080 / PC 29.** Its exported 84-note stream has
   an exact, already distinguished project correlate containing `Wavox`.
2. **`Track 2` / JV-1080 / PC 37.** It has an early PC and 211 following notes,
   but no comparably established Patch-name/event-region identity.
3. **`Track 1` / Juno-106 / PC 61.** It offers a different device and 91 notes,
   but its relevant project region and Patch name are not independently
   established.

`Track 3` provides the strongest validation target because selection does not
depend on making the existing decoder succeed.

# Selected independent Patch event

- Sequence: `Ode to Clarke`
- Studio Vision/export track name: `Track 3`
- Export-local MIDI track index: 6
- Instrument metadata: `JV-1080`
- MIDI channel: 1
- Patch position: tick 480, `1·2·0`
- Program Change: 29
- Project Patch name: `Wavox`
- First note: tick 9,603, C-sharp5, attack 78, release 127, duration 461

This is a natural event in the untouched authentic project. It is a different
track and device context from Track 3 #2 / JD-800 and is not a controlled
derivative of it.

# Independent evidence

The MIDI export names `Track 3`, identifies `JV-1080`, and contains PC 29 at
tick 480. Its first four notes are:

| Tick | Pitch | Attack | Release | Duration |
|---:|---:|---:|---:|---:|
| 9,603 | 85 | 78 | 127 | 461 |
| 10,086 | 84 | 76 | 81 | 443 |
| 10,524 | 80 | 92 | 51 | 480 |
| 10,987 | 75 | 90 | 60 | 301 |

The project structures beginning at `0x3131c`, `0x31321`, `0x31328`, and
`0x3132f` encode those four complete note-property rows, with intervening
start-to-start VLQs 483, 438, and 463. The first-note start is independently
linked to Patch tick 480 by `9603 - 480 = 9123`.

# Binary location and anchors

The candidate Patch position begins at `0x31300`; the first Note status is at
`0x3131b`, and first-note properties begin at `0x3131c`. The compact local
layout is:

`83 60 | ff 7c | 13 | 00 01 9d f9 1d | 05 | 57 61 76 6f 78 |
02 33 30 04 ff 51 02 | 1d | c7 23 | 90`

Independent anchors are the absolute time 480 (`83 60`), unique local ASCII
`Wavox`, direct PC 29 (`1d`), the 9,123-unit Patch-to-note interval (`c7 23`),
first Note status `90`, and four complete exported note rows. Together these
identify the region much more strongly than a string match alone.

# Comparison with Track 3 #2 representation

| Relationship | Candidate observation | Assessment |
|---|---|---|
| absolute-position VLQ | `0x31300–0x31301`: `83 60` = 480 | MATCH |
| `ff 7c` after position | `0x31302–0x31303` | MATCH |
| local payload-length relationship | `0x31304`: 19; name length 5, difference 14 rather than 15 | DIFFERENT |
| stable pre-name context | `00 01 9d f9 1d`, not `00 00 17 00 17` | DIFFERENT |
| one-byte name length | `0x3130a`: `05` | MATCH |
| variable ASCII name | `0x3130b–0x3130f`: `Wavox` | MATCH |
| stable post-name context | `02 33 30 04 ff 51 02`, not Track 3 #2's context | DIFFERENT |
| direct Program Change | `0x31318`: `1d` = 29, matching export | MATCH |
| two-byte post-PC VLQ shape | `0x31319–0x3131a`: `c7 23` = 9,123 | MATCH |
| stable context before Note | Track 3 has no Track 3 #2 12-byte intervening context | DIFFERENT |
| transition to Note status | `0x3131b`: `90` | MATCH |

# Existing decoder result

The unchanged `decode_known_track3_2_patch` was invoked with start `0x31300`
and exclusive bound `0x3131c`, including the expected status byte at
`0x3131b`. It failed, as expected from the comparison:

`UnexpectedBytes { offset: 0x31305, expected: [00, 00, 17, 00, 17], observed:
[00, 01, 9d, f9, 1d] }`

No alternate bound, recovery, byte skipping, or code change was attempted.
Because decoding failed, there is no decoder-produced semantic result for this
candidate.

# Semantic cross-check

Manual structural decoding, kept separate from the failed decoder result,
agrees with the MIDI export for position 480, PC 29, and first-note timing and
properties. The literal `Wavox` supplies project-internal Patch-name evidence;
the MIDI export does not contain that name. `JV-1080` comes independently from
the MIDI instrument-name meta event.

# Generalization assessment

- **A. Same position representation: YES.** Both use a two-byte 7-bit
  big-endian absolute VLQ followed by `ff 7c`.
- **B. Same name-length representation: YES.** One byte gives the exact ASCII
  payload length.
- **C. Same ASCII Patch-name representation: YES.** `Wavox` immediately follows
  its length byte.
- **D. Same direct Program Change representation: YES.** Project byte 29 equals
  exported Program Change 29.
- **E. Same post-PC structural pattern: PARTIAL.** A two-byte VLQ follows PC,
  but Track 3 then reaches Note status immediately rather than through the
  Track 3 #2 context.
- **F. Same transition into Note data: PARTIAL.** Both reach `0x90` and the
  proven Note representation, but the intervening layout differs.
- **G. Existing bounded decoder succeeds unchanged: NO.** It rejects the first
  differing pre-name context at `0x31305`.
- **H. Evidence of generalization beyond Track 3 #2: PARTIAL.** Core semantic
  fields recur in a second authentic event, while device/local framing does
  not match the Track 3 #2-specific decoder contract.

# Evidence supported

- Track 3 contains a naturally occurring Patch-like representation at
  `0x31300–0x3131a`, followed by Note status at `0x3131b`.
- Position, length-prefixed ASCII name, direct PC, post-PC VLQ shape, and Note
  transition recur outside Track 3 #2.
- MIDI PC, timing, device/track metadata, and four complete notes independently
  corroborate the binary identification.
- Track 3 #2-specific local context does not generalize unchanged.
- The existing decoder correctly refuses this different representation.

# Unknowns

The meanings of Track 3's differing pre/post-name bytes, whether its payload
length spans exactly the same conceptual object, why device contexts differ,
event-type semantics, complete Patch boundaries, and generality to any third
event remain unknown. `Wavox` is project-binary evidence, not MIDI text.

# Single recommended next step

Investigate the exact Track 3 versus Track 3 #2 local-context differences
before changing the decoder. Specifically, map which differing bytes are
device/patcher metadata and which are structural by comparing this candidate
with one more independently identified Patch event; retain explicit bounds and
do not relax context validation merely to accept both.
