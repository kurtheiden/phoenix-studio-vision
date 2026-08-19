# Objective

Phase D4 persists the exact in-memory `Ode to Clarke` multitrack bytes that
passed D3, then independently re-opens and validates the disk artifact. It is
an explicit research action, not general file-writing or CLI behavior.

# Preconditions

The action reuses the single D2/D3 path. It validates both authentic hashes,
regenerates the complete sequence once, independently parses it, and requires
the full normalized comparison before any write.

# Source provenance

Experiment 007's untouched 211,468-byte source passed SHA-256
`e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`.

# Reference provenance

The 12,141-byte `Ode to Clarke Multi All` reference passed SHA-256
`4f63b34ef92204d4bc5eeb78dbbe7b94d005c1f9ceb57ea0f9809533ad590f29`.

# D3 revalidation

Immediately before persistence, D3 passed again for all ten tracks, 1,308
Notes, 1,291 comparable releases, 17 reference velocity-zero endings, four
Patch translations, conductor metadata, track identity/order/channels, and
the absence of unexpected musical families.

# Output path

The sole artifact is:

`/Users/kurtheiden/Documents/Phoenix Research/Phoenix MIDI Proofs/Ode to Clarke - Phoenix Multitrack Proof.mid`

# Atomic write behavior

The ignored explicit research test creates the output directory if needed. If
the final file exists, it requires byte identity and never overwrites a
difference. Otherwise it uses a process-specific temporary file in the same
directory, writes and syncs all validated bytes, closes it, and renames it to
the final path. On write failure it removes only its own temporary file.

# Proof size and SHA-256

- Size: 12,184 bytes
- SHA-256: `14d855f9d6c8e609365ac8d45335ca1e6c36fd9ede8299d01fba9d5d0f4a72eb`

# Independent disk re-open

The final file was read back byte-for-byte equal to the exact buffer that had
passed D3. The independent parser accepted `MThd`, all bounded `MTrk` chunks,
VLQs, channel/meta messages, final EOTs, and exact EOF.

# Disk normalized comparison

The independently parsed disk bytes were compared again against the
authenticated Studio Vision reference. The complete D3 comparison passed from
disk, including every Note and comparable release velocity.

# SMF structure

- Format: 1
- PPQN: 480
- Total tracks: 10
- Musical tracks: 9
- Conductor: `Ode to Clarke`, 500,000 MPQN, `4,2,24,8`

# Musical inventory

- Notes: 1,308
- Explicit Phoenix Note Offs: 1,308
- Program Changes: 4
- Bank Select MSB / LSB: 2 / 2
- Ordinary Controllers: 0
- Channel Pressure: 0
- Pitch Bend: 0

# Expected policy differences

The established D3 differences remain: 17 historical velocity-zero endings,
reference-only SMPTE Offset and Instrument Name metadata, reference EOT padding
to tick 94,080, raw/running-status encoding, and musically equivalent same-tick
ordering. D4 does not transform or imitate them.

# Repository boundary

The MIDI remains outside the repository. Normal `cargo test` ignores the
explicit writer, does not require the proof to exist, and does not rewrite it.
No production parser, adapter, serializer, CLI, or Cargo dependency changed.

# Manual DAW validation status

**PASS / USER-OBSERVED — Logic Pro 12 (2026-08-19).** The user opened this
exact proof and reported, “It looks good it sounds good!” Logic displayed all
nine expected musical tracks in order: `Track 1`, `Track 2`, `sys100loops`,
`Track 4`, `Track 5`, `Track 3`, `Track 6`, `Track 3 #2`, and `Track 7`.
Visual inspection looked correct and playback sounded correct, with no
playback problem reported.

Logic's Event List/filter showed nine top-level events corresponding to the
nine MIDI regions/tracks. This is not an independent Logic count of 1,308
Notes. The Note count and exact musical equivalence remain automated D3/D4
findings.

# D4 gate

**PASS.** The exact D3 buffer was persisted only at the approved path, matched
on re-read, parsed independently, and passed the complete disk comparison.

# D5 gate

**PASS / USER-OBSERVED.** Logic Pro 12 opened the proof with all nine expected
musical tracks; it looked and sounded correct during playback, with no problem
reported. The complete bounded Ode multitrack proof cycle is **COMPLETE**.

This result is limited to the authenticated `Ode to Clarke` sequence, current
supported event families, and authenticated channel/Patch policy. It does not
establish general channel derivation, arbitrary project conversion,
unsupported-family conversion, original synthesizer/audio recreation, or a
user-facing Phoenix export workflow.

# Single recommended next step

Preserve this completed bounded proof as the baseline; choose the next broader
export target only in a separate scoped task.
