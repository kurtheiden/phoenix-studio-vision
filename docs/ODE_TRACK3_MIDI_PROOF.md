# Objective

Produce the first authentic Phoenix-generated MIDI from one exactly bounded,
validated Studio Vision track without broadening parser claims or implementing
a general export workflow.

# Source provenance

The read-only source is Experiment 007's untouched `newest STUFF baseline`:
211,468 bytes, SHA-256
`e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`.
The proof verifies the digest before parsing.

# Reference MIDI provenance

The authenticated Studio Vision reference is `Ode to Clarke Multi All`:
12,141 bytes, SHA-256
`4f63b34ef92204d4bc5eeb78dbbe7b94d005c1f9ceb57ea0f9809533ad590f29`.
An independent test-side parser selects SMF track 6 only after parsing the
complete file.

# Structural sequence location

**OBSERVED:** `parse_project_166` consumes the authentic project and produces
exactly one parsed sequence whose derived Pascal-name bytes are `Ode to
Clarke`. Its range is `0x02ef6f..0x03202c`; its name-byte range is
`0x02f753..0x02f760`. The proof locator selects among structurally parsed
sequences and does not begin at a hard-coded project offset.

# Structural Track 3 location

**OBSERVED:** descriptor ordinal 7 is `0x02f4c9..0x02f56f`, has label `Track
3`, and the equal-count ordinal association binds it to pair ordinal 5. That
pair's primary is `0x0312ed..0x03156b`. The exact event end is derived by
validating and excluding the established seven-byte primary tail; the result
is `0x031300..0x031564`.

# Manifest validation

The proof-only manifest requires simultaneous equality of project digest,
sequence range, sequence name bytes and range, descriptor ordinal and range,
pair ordinal, primary range, exact event range, and human MIDI channel 1. Any
mismatch is a typed proof failure. The row is integration-test policy, not
Studio Vision parser knowledge; it is absent from `midi_export` and `smf`.

# Meter/Tempo

The existing bounded decoders derive initial Tempo 500,000 MPQN and Meter
source values `4, 2, 8, 8`. Phase B's established Meter policy produces the
SMF Time Signature payload `4, 2, 24, 8`. Both match the authenticated
reference conductor metadata at tick zero. Timing is explicit `Identity480`.

# Mixed-event walk

The existing walker consumes `0x031300..0x031564` exactly. It returns one
coupled Patch-to-first-Note item followed by 83 Notes: 85 logical source events
in total, comprising one Patch and 84 Notes. No unsupported family is present
or omitted.

# Patch classification

The proof validates the already established Track 3 evidence before supplying
the adapter classification: Patch position 480, name `Wavox`, exact
post-name context `02 33 30 04 ff 51 02`, Program 29, and first Note position
9,603. The evidence-backed translation is CC0=81, CC32=2, Program=29 at tick
480. The generic adapter does not interpret these opaque bytes.

# Adapter result

The transaction uses channel 1 with `AuthenticatedOverride`, `Identity480`,
and `StrictKnownOnly`. It produces 84 Note Ons, 84 generated explicit Note
Offs, one CC0, one CC32, and one Program Change, with no warning or partial
result. Source order and decoded Note properties are preserved.

# Generated SMF

The existing serializer produces an in-memory Format 1 file at PPQN 480 with
two tracks. Track 0 contains sequence name, initial Tempo, initial Meter, and
EOT. Track 1 contains Track Name `Track 3`, the adapted channel messages, and
EOT. A general serializer helper now supports a tick-zero musical Track Name;
it contains no Studio Vision knowledge.

# Independent parser validation

The test-side parser independently validates MThd/MTrk sizes, MIDI VLQs,
running status in the historical reference, legal channel data, exactly one
final EOT per track, and exact EOF consumption. It extracts names, Tempo,
Meter, channel messages, banks/program, and paired Notes without using Phoenix
serializer internals.

# Reference comparison

Channel 1, CC0=81, CC32=2, Program=29, PPQN 480, initial Tempo, and initial
Meter are exact normalized matches. Same-tick serialization order is Phoenix's
documented deterministic policy; byte identity and historical ordering are not
required.

# Note equivalence

**EXACT MATCH:** all 84 Phoenix Notes match the 84 reference Notes one-for-one
for channel, pitch, start, end/duration, and attack velocity. For the 82
reference endings encoded as explicit `8n`, release velocity also matches
exactly.

**EXPECTED POLICY DIFFERENCE:** Studio Vision encodes two endings as `9n`
velocity zero, which carries no release velocity. Their musical end ticks
match. Phoenix retains the decoded SVP release velocities and emits explicit
`8n` Note Offs rather than discarding information to imitate historical bytes.

# Policy differences

The proof permits normalized musical equivalence rather than byte identity.
Phoenix always writes explicit statuses and follows its deliberate same-tick
priority. The two zero-velocity reference endings are the only release-form
difference found; there is no supported musical-content difference.

# Proof artifact

After the in-memory comparison passed, the research-only action wrote:

`/Users/kurtheiden/Documents/Phoenix Research/Phoenix MIDI Proofs/Ode to Clarke - Track 3 - Phoenix Proof.mid`

The artifact is 856 bytes with SHA-256
`6b6553566eeee1e5e47ffe24b3ed4d0fdc7fed933d7f40811778ac6bb4108317`.
It is Format 1, PPQN 480, two tracks. A disk re-open matched the in-memory bytes
exactly and independently parsed through EOF. Normal tests do not rewrite it;
the write is gated by an explicit research-test environment variable.

# Manual DAW validation status

**PASS / USER-OBSERVED, 2026-08-19.** The Phoenix proof opened successfully in
Logic Pro 12, which displayed the musical track as `Track 3`. Playback
completed without audible glitches or obvious timing errors, hanging notes,
misplaced notes, or other playback problems. Changing instrument patches
during playback did not interrupt correct playback. Logic showing only Track
3 is the expected result of this deliberately bounded one-track proof, not a
missing-track failure.

# Evidence supported

- The existing bounded project/container/walker APIs can drive a real export.
- Phase A and Phase B compose into a standards-valid authentic two-track SMF.
- Identity 480 timing, Track 3's authenticated channel override, classified
  Patch translation, Tempo/Meter policy, and all 84 Notes agree with Studio
  Vision's export.

# Unknowns

- General parser-derived channel routing remains unknown.
- This proof does not establish any other Ode track, a complete sequence
  export, arbitrary Patch translation, or a user-facing file-writing workflow.
- Full-sequence and multitrack behavior remain outside this proof despite the
  successful one-track DAW result.

# Phase C gate

**PASS / PHASE C COMPLETE.** Every required structural, provenance,
adaptation, serialization, independent parsing, supported musical-comparison,
and manual DAW gate passes. This remains a one-track proof, not a full-sequence
export.

# Single recommended next step

Design Phase D's strict Ode multitrack export while retaining exact structural
identity checks and provenance-locked channel policy.
