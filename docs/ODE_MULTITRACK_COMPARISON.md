# Objective

Phase D3 independently compares Phoenix's complete in-memory `Ode to Clarke`
Format 1 result with the authenticated Studio Vision multitrack export. It is
validation-only and writes no MIDI file.

# Provenance

The 211,468-byte Experiment 007 source passes SHA-256
`e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`.
The 12,141-byte reference passes SHA-256
`4f63b34ef92204d4bc5eeb78dbbe7b94d005c1f9ceb57ea0f9809533ad590f29`.
Both gates precede comparison.

# Independent parser

Test-side code parses `MThd`, bounded `MTrk`, four-byte VLQs, running status,
channel messages, arbitrary meta events, SysEx framing, and exactly one final
EOT. It accumulates absolute ticks and requires exact EOF. It does not use the
Phoenix serializer to decode either result.

# Phoenix structure

D3 reuses D2's manifest validation, exact walks, flattening, Patch policy, and
single D1 assembly call. The independently parsed result is Format 1, PPQN 480,
with one conductor plus nine musical tracks.

# Reference structure

The independently parsed reference is also Format 1, PPQN 480, ten tracks,
with exact EOF and one final EOT per track.

# Conductor comparison

Both conductors have Track Name `Ode to Clarke`, Set Tempo 500,000 MPQN at
tick 0, and Time Signature `4,2,24,8` at tick 0, with no later Tempo or Meter.
The reference-only SMPTE Offset `ff 54 05 60 00 00 00 00` is optional metadata
and an expected policy difference.

# Track identity/order

Tracks 1–9 match in order: `Track 1`, `Track 2`, `sys100loops`, `Track 4`,
`Track 5`, `Track 3`, `Track 6`, `Track 3 #2`, `Track 7`. Their sole channels
match: 1, 2, 10, 10, 10, 1, 10, 15, 10. Reference-only Instrument Name events
are optional metadata.

# Note normalization

Positive-velocity `9n` starts Notes; `8n` and velocity-zero `9n` end them.
FIFO pairing by `(track, channel, pitch)` fails on unmatched endings or starts.
Normalization retains channel, pitch, start/end, attack, and optional release.

# Per-track Note comparison

Every Note matches one-for-one for channel, pitch, start, end, and attack:

| Track | Phoenix | Reference |
|---|---:|---:|
| Track 1 | 91 | 91 |
| Track 2 | 211 | 211 |
| sys100loops | 322 | 322 |
| Track 4 | 179 | 179 |
| Track 5 | 134 | 134 |
| Track 3 | 84 | 84 |
| Track 6 | 60 | 60 |
| Track 3 #2 | 84 | 84 |
| Track 7 | 143 | 143 |
| **Total** | **1,308** | **1,308** |

# Release velocity comparison

All 1,291 explicit reference `8n` releases match Phoenix exactly. Seventeen
reference endings use velocity-zero `9n` and have no release value; Phoenix
preserves the decoded release explicitly. This is an expected representation
difference with equivalent musical ends.

# Patch comparison

Normalized CC/Program inventories match exactly: Track 1 Program 61 at tick 0;
Track 2 CC0=81, CC32=1, Program 37 at tick 0; Track 3 CC0=81, CC32=2, Program
29 at tick 480; Track 3 #2 Program 23 at tick 530.

# Other channel families

After excluding those bank pairs, both results contain zero ordinary
Controllers, zero Channel Pressure, and zero Pitch Bend.

# Unsupported musical events

Neither result contains Poly Pressure, SysEx, another channel family, or other
unsupported required musical data. Such data would fail comparison.

# Event timing

All supported musical events match at exact absolute ticks. Raw deltas and
running-status encoding are intentionally not compared.

# Same-tick ordering

No ordering difference changes normalized state. Phoenix retains its documented
deterministic ordering rather than copying historical byte order.

# EOT policy

Studio Vision pads every track to tick 94,080. Phoenix uses tick 0 for conductor
EOT and the latest emitted event for each musical EOT. Since musical content
matches first, this is an expected policy difference.

# Inventory summary

| Family | Phoenix | Reference |
|---|---:|---:|
| SMF tracks | 10 | 10 |
| Notes | 1,308 | 1,308 |
| Explicit releases | 1,308 | 1,291 |
| Velocity-zero endings | 0 | 17 |
| Ordinary Controllers | 0 | 0 |
| Bank Select MSB / LSB | 2 / 2 | 2 / 2 |
| Program Changes | 4 | 4 |
| Channel Pressure | 0 | 0 |
| Pitch Bend | 0 | 0 |
| Tempo / Meter | 1 / 1 | 1 / 1 |

# Exact matches

Structure, PPQN, conductor musical state, track identity/order/channel, Notes,
comparable releases, Patch state, and supported-family inventories match.

# Musical equivalences

The 17 velocity-zero reference endings have identical Note ends while Phoenix
retains the source release velocities.

# Expected policy differences

Only the established release representation, reference SMPTE Offset and
Instrument Name metadata, tick-94,080 EOT padding, running-status/raw encoding,
and musically equivalent same-tick ordering remain.

# Unsupported/unknown

None occurs in the compared musical content. This does not establish general
channel derivation or export beyond the locked Ode profile.

# D3 gate

**PASS.** Both provenance gates and independent parses pass; all ten tracks,
1,308 Notes, 1,291 comparable releases, four Patch translations, and supported
inventories match. No permanent proof artifact was written.

# Single recommended next step

Implement D4: explicitly write the compared in-memory bytes to the one approved
proof path and independently re-open them.
