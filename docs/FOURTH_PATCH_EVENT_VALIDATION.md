# Objective

Independently identify authentic `Ode to Clarke` / `Track 2` / `JV-1080` / PC
37 and validate the existing bounded representation decoder without changing
its contract or production code.

# Source identities

The untouched project is `/Users/kurtheiden/Documents/Phoenix
Research/Controlled Save Experiments/Experiment 007 - Untouched
Baseline/newest STUFF baseline`, 211,468 bytes, SHA-256
`e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`.

The MIDI source is `Ode to Clarke Multi All`, 12,141 bytes, SHA-256
`4f63b34ef92204d4bc5eeb78dbbe7b94d005c1f9ceb57ea0f9809533ad590f29`,
SMF format 1, division 480.

# Independent MIDI ground truth

Export-local MIDI track 2 names `Track 2` and instrument `JV-1080`. At tick 0
on channel 2 it exports CC0=81, CC32=1, then PC 37. It contains 211 notes. The
first ten independently re-read rows are:

| Tick | Pitch | Attack | Release | Duration |
|---:|---:|---:|---:|---:|
| 1,920 | 44 | 90 | 42 | 139 |
| 2,400 | 56 | 126 | 85 | 63 |
| 2,640 | 44 | 120 | 76 | 88 |
| 3,600 | 42 | 88 | 52 | 186 |
| 3,840 | 44 | 120 | 90 | 117 |
| 4,320 | 56 | 114 | 77 | 78 |
| 4,560 | 44 | 123 | 100 | 88 |
| 5,760 | 44 | 114 | 73 | 84 |
| 6,240 | 56 | 110 | 72 | 73 |
| 6,480 | 44 | 108 | 51 | 114 |

PC tick 0 is musical position `1·1·0`; the first note at tick 1,920 is
`2·1·0`.

# Independent Note-region correlation

The first ten exported note property/timing structures form one unique project
hit beginning at `0x2fb75`. It includes all pitches, attacks, releases,
durations, and nine note-to-note timing fields without search-ahead or
resynchronization. This establishes first Note status `0x2fb74` independently
of the Patch decoder. A complete-chain comparison is recorded below after the
bounded location is fixed.

# Patch-region bounds

Working backward from the independently identified Note status gives:

`00 | ff 7c | 19 | 00 01 25 f8 a5 | 0b | Stereoww Bs |
02 33 38 04 ff 51 01 | 25 | 8f 00 | 90`

The defensible decoder bounds are:

- position start: `0x2fb55`;
- Note status: `0x2fb74`;
- exclusive end: `0x2fb75`.

# Preregistered expected result

The following expectations were written before invoking
`decode_bounded_patch_representation`:

- position: 0;
- Patch name: `Stereoww Bs` (11 ASCII bytes);
- PC: 37 at `0x2fb71`;
- payload length: 25;
- pre-name context: `00 01 25 f8 a5`;
- post-name context: `02 33 38 04 ff 51 01`;
- bank evidence: exported CC0=81 and CC32=1 correspond to tail `ff 51 01`;
- post-PC timing: `8f 00` = 1,920;
- pre-Note context: empty;
- status: `90` at `0x2fb74`.

The expected post-PC value equals first-note tick 1,920 minus Patch tick 0.

# Existing decoder result

`decode_bounded_patch_representation` was invoked unchanged with
`position_start = 0x2fb55` and `note_status_end = 0x2fb75`. It returned:

- position `00` = 0, range `0x2fb55–0x2fb55`;
- marker `ff 7c`, range `0x2fb56–0x2fb57`;
- payload length 25 at `0x2fb58`, payload `0x2fb59–0x2fb71`;
- pre-name `00 01 25 f8 a5`;
- name length 11 and `Stereoww Bs` at `0x2fb5f–0x2fb69`;
- post-name `02 33 38 04 ff 51 01` at `0x2fb6a–0x2fb70`;
- PC 37 at `0x2fb71`;
- post-PC `8f 00` = 1,920 at `0x2fb72–0x2fb73`;
- empty pre-Note context;
- Note status `90` at `0x2fb74`.

Result: **PASS unchanged**. No production decoder modification preceded or
followed the call.

# Semantic cross-check

Every independently preregistered value is a **MATCH**: position, name, PC,
payload, opaque contexts, post-PC timing, and status offset. The complete
binary note walk from `0x2fb75` contains 211/211 pitches, 211/211 attacks,
211/211 durations, and 210/210 note-to-note timings. Release velocities match
207/211; four SMF zero-velocity note-offs differ from stored project release
values, analogous to the already observed Track 1 export distinction. The
chain stops at `0x300d8` before the known repeated post-chain form.

# Four-event comparison

| Feature | Track 1 | Track 2 | Track 3 | Track 3 #2 |
|---|---|---|---|---|
| position | `00` = 0 | `00` = 0 | `83 60` = 480 | `84 12` = 530 |
| payload | 25 | 25 | 19 | 27 |
| pre-name | 5 bytes | 5 bytes | 5 bytes | 5 bytes |
| name | `Empty Patch` (11) | `Stereoww Bs` (11) | `Wavox` (5) | `Ming Dynasty` (12) |
| post-name width | 7 | 7 | 7 | 8 |
| PC | 61 | 37 | 29 | 23 |
| post-PC | 9,720 | 1,920 | 9,123 | 8,908 |
| pre-Note width | 0 | 0 | 0 | 12 |
| transition | `90` | `90` | `90` | `90` |

Track 2 matches the shared contract unchanged (**A: YES**), resembles Track 3
framing particularly closely (**B: YES**), and introduces different opaque
values but no new framing variant (**C: NO**).

# JV-1080 comparison

Track 2 and Track 3 both use `JV-1080` metadata and show the compact layout
with a one-byte payload length, five-byte pre-name context, seven-byte
post-name context, direct final-payload PC, complete-interval post-PC VLQ, and
no pre-Note context. Both post-name contexts begin `02 33`, then differ (`38`
versus `30`) before `04` and the bank-correlated tail. Track 2 uses channel 2,
bank 81/1, name `Stereoww Bs`, PC 37, and Patch tick 0; Track 3 uses channel 1,
bank 81/2, name `Wavox`, PC 29, and Patch tick 480. Shared device metadata may
explain some framing, but two samples cannot assign the opaque identifier
bytes to the device specifically.

# Bank-select evidence

Track 2 independently supplies a second exact correlation: CC0=81 / CC32=1
versus opaque tail `ff 51 01`. Track 3 has CC0=81 / CC32=2 versus
`ff 51 02`; Track 1 and Track 3 #2 export no bank select and contain
`ff ff ff`. Bank-field evidence is therefore **STRONGER** after Track 2, but
still correlational rather than controlled proof. The decoder correctly keeps
these bytes opaque.

# Timing evidence

Patch tick 0, first-note tick 1,920, and raw post-PC `8f 00` independently
predict a complete interval value of 1,920. The decoded component is 1,920 and
pre-Note context is empty, so Track 2 matches Track 1 and Track 3: the post-PC
VLQ equals the complete Patch-to-first-note interval. Track 3 #2 remains the
only compound sample.

# Shared-contract assessment

Track 2 validates the existing shared contract with no new framing rule. A
read-only regression test now uses the independently established bounds and
asserts semantic values plus opaque provenance. No discovery, contract
broadening, bank interpretation, or production-code change was added.

# Evidence supported

- MIDI metadata independently identifies Track 2 / JV-1080 / channel 2.
- CC0=81, CC32=1, PC 37 occur at tick 0.
- Ten initial notes uniquely locate the project chain; the full chain matches
  211 pitches/attacks/durations and 210 timings.
- `Stereoww Bs` is structurally established as the length-prefixed Patch name.
- The shared decoder passes unchanged at `0x2fb55..0x2fb75`.
- Four authentic events now validate the bounded semantic core.

# Unknowns

Opaque context semantics, exact bank serialization semantics, discovery,
complete event/container ownership, and generality remain unknown.

# Single recommended next step

Run one controlled bank-select-only experiment on a known compact JV-1080
Patch event, leaving position, name, PC, and notes unchanged. Four authentic
events are sufficient for the bounded core; isolating the `ff 51 NN` tail now
has higher information value than another ordinary Patch validation.

That experiment is complete. Experiment 028 changed only Track 2 CC32 from 1
to 2 and confirmed `0x2fb6e..0x2fb70: ff 51 01 -> ff 51 02`; all other bounded
Patch bytes and the complete Note chain remained stable. See
`CONTROLLED_TRACK2_BANK_LSB_CHANGE.md`.
