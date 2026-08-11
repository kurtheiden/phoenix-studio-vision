# Objective

Reconcile Track 3 #2 / JD-800 with the complete 85-event Studio Vision List
Window and inspect the local binary region immediately before its proven
84-note chain for the first Patch event. The authentic Experiment 007 project
was read only. No MIDI was emitted and no parser code was changed.

# Corrected complete List Window evidence

The two human-supplied screenshots show `Ode to Clarke: Track 3 #2`, instrument
`JD-800`, and `85 Events`. Their overlap reconciles to one Patch event followed
by 84 notes:

- event 1: `1·2·50`, Patch, `Ming Dynasty`, `PC 23`;
- event 2 / note 1: `6·1·3`, C#5, duration `0 + 461`, attack 100,
  release 127;
- event 85 / note 84: `42·4·226`, Eb4, duration `7 + 158`, attack 127,
  release 41.

List 1 contains event 1 through note 51. List 2 overlaps notes 33–51 and
continues through note 84. The external CSV preserves all 85 transcribed rows
and screenshot provenance. Every reported value used below is visibly readable;
no screenshot value was filled from project bytes.

# Previous assumption and correction

The prior report was told that the 17-note sample beginning `18·4·241` was
the beginning of the List Window. That was wrong. The complete screenshots
establish that it is note indices 33–49, or complete event indices 34–50.

The earlier preregistered comparison remains valid because its 17 notes are a
real consecutive screenshot sequence and were compared against one consecutive
binary sequence without resynchronization: 68/68 properties and 16/16
note-to-note intervals. What changes is its provenance, numbering, and choice
between two duplicate 17-note hits—not the measured matches.

The new first note supplies the discriminator that was previously missing.
The region at `0x3131b` begins with C#5 attack 78 and contains `Wavox`; it is
not Track 3 #2. The region at `0x318b5` begins with C#5 attack 100, contains
`Ming Dynasty`, and matches the complete list. The prior report's selection of
the first region from its nearby 85 value was therefore mistaken.

# 85-event reconciliation

Studio Vision observationally reports exactly:

`1 Patch + 84 Notes = 85 Events`.

The earlier apparent discrepancy between 85 displayed events and 84 bounded
note structures is fully explained at the UI level; it was never a missing
note. Binary representation of the Patch is investigated below and remains
less complete than the note representation.

# First-note binary boundary

The positively identified uncompressed baseline is `newest STUFF baseline`,
211,468 bytes, SHA-256
`e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`.

The corrected local boundaries are:

- recurring marker: `0x31882`, bytes `2c c4 b2`;
- status/type-like byte immediately before note 1: `0x318b4`, byte `90`;
- note 1 property structure: `0x318b5`, `55 64 7f 83 4d`;
- note 2 timing: `0x318ba`, `83 63` = 483;
- final note properties: `0x31af9`, `4b 7f 29 9b 3e`;
- cursor after final note: `0x31afe`;
- post-chain bytes: `ff fb 8b 7d ff 2f 00 29 00 00 00 eb 00 06 00 00 ...`.

The mechanically bounded note chain contains exactly 84 consecutive property
structures. Note 1 has no established note-format timing VLQ immediately
before its properties because `90` occupies that position and is also seen
before Track 7's first note properties.

# Patch-to-note timing prediction

Using `(measure - 1) × 4 × 480 + (beat - 1) × 480 + units`:

- Patch `1·2·50`: `0 × 1920 + 1 × 480 + 50 = 530`;
- first note `6·1·3`: `5 × 1920 + 0 × 480 + 3 = 9603`;
- predicted start-to-start interval: `9603 - 530 = 9073`;
- 7-bit big-endian VLQ prediction: `c6 71`.

This prediction was calculated before testing the local bytes.

# Pre-note binary context

The local region from marker through the first note is:

`2c c4 b2 5c 84 12 ff 7c 1b 00 00 17 00 17 0c`

`4d 69 6e 67 20 44 79 6e 61 73 74 79 03 49 33 38 04`

`ff ff ff 17 c5 4c ff 60 07 57 7f 00 6c 6c a3 4a 81 25`

`90 | 55 64 7f 83 4d ...`

The predicted contiguous VLQ `c6 71` is absent from this bounded region. Two
local byte pairs decode separately as `c5 4c` = 8908 and `81 25` = 165; their
sum happens to equal 9073. No established structure authorizes adding those
fields, so this is recorded as a coincidence/candidate relationship, not a
timing match. Patch-to-note timing is **UNRESOLVED**, not contradicted: the
Patch structure and timing-field ownership are not yet decoded.

# Patch event representation

The Patch event is locally and credibly represented in some form before the
note chain:

- the exact UI patch name `Ming Dynasty` occurs at `0x31891`;
- `ff ff ff 17` occurs at `0x318a2`;
- the note stream begins with a distinct `90` byte at `0x318b4`;
- the known Studio Vision `Ode to Clarke Multi All` export contains one Program
  Change for `Track 3 #2` at absolute time 530, channel 15, data value 23,
  followed by 84 positive-velocity note-ons whose first is C#5/100 at 9603.

Together these identify the mixed Patch-plus-note region and strongly associate
the local `0x17` at `0x318a5` with the displayed/exported program value 23.
They do not yet establish a complete Patch record grammar or exact start/end
field ownership. Patch representation is therefore **PARTIAL**, not a parser-
ready identification.

# Program-number analysis

`PC 23` cannot be interpreted from UI wording alone as one-based or zero-based.
However, the independent existing Studio Vision SMF export stores Program
Change data byte decimal 23 for this track, and the local candidate sequence
ends `ff ff ff 17`, where `0x17` is decimal 23. This supports a direct stored
value of 23 in this artifact. It does not prove that all Studio Vision displays
or devices use the same numbering convention, nor does it prove the three
leading `ff` bytes' meaning.

Other `0x17` bytes occur at `0x3188d` and `0x3188f` in the same header. Their
mere value is not enough to call them program fields; `0x318a5` is the stronger
candidate because of the unique `ff ff ff 17` relationship immediately after
the patch/name metadata.

# Patch-name/reference analysis

`Ming Dynasty` occurs once in the baseline, directly inside this bounded
pre-note region at `0x31891`. Existing project-structure documentation had
recorded the name in an earlier authentic serialization, but had not assigned
its structure. OMS documentation establishes that patch names may be supplied
through device-specific Patch Name documents; it does not specify project-file
encoding.

The local literal proves direct storage of the name in this region. The nearby
bytes `03 49 33 38 04` (`I38` within them) and other numeric values remain
uninterpreted; no patch-table index or reference is established. The Tracks
window's JD-800 / Ming Dynasty observation is UI-consistent context, not a
binary reference proof.

# Event-type discriminator

The first note properties in both identified Track 3 #2 and Track 7 regions
are immediately preceded by `90`, after which later notes use the established
timing/property form without repeating it. This is strong evidence that `90`
is a note-stream status/type-like discriminator in these regions.

Track 3 #2 uniquely has the Patch-specific pre-note material including
`ff ff ff 17`; Track 7 has no analogous material before its first note.
`ff` is therefore a plausible Patch/status discriminator, but its exact width
and ownership are not isolated. No claims are extended to other event types.

# Complete 84-note validation

All screenshot notes were compared strictly against the chain beginning at
`0x318b5`. There was no search-ahead, skip, or resynchronization.

| Comparison | Exact matches |
|---|---:|
| Pitch | 84/84 |
| Attack velocity | 84/84 |
| Release velocity | 84/84 |
| Duration | 84/84 |
| Complete note rows | 84/84 |
| Individual properties | 336/336 |
| Note-to-note timing | 83/83 |

The Patch-to-first-note interval is excluded from the 83 note-only timing
comparisons. No screenshot field was uncertain or excluded.

# Event-count field reassessment

The four-byte value eight bytes before the corrected Track 3 #2 marker is
`00 00 00 56` = 86, while the complete UI list has 85 total events. The value
85 at the analogous location belongs to the separate `Wavox` region selected
in error by the prior report.

Track 7 still has 143 at marker minus eight and 143 displayed notes. But the
corrected Track 3 #2 comparison is 86 versus 85 displayed mixed events. Thus
the hypothesis that this field exactly represents total List Window events is
**CONTRADICTED** by one of the two independently identified tracks. The field
may count a framing item, another internal object, or something else; no new
meaning is assigned.

# Comparison with Track 7

| Relationship | Track 3 #2 / JD-800 | Track 7 / JV-1080-10 |
|---|---|---|
| UI events | 85 = 1 Patch + 84 notes | 143 notes |
| Marker-minus-eight value | 86 | 143 |
| Marker | `0x31882`: `2c c4 b2` | `0x31c04`: `2c c4 b2` |
| Pre-note content | literal patch name, program candidate, other fields | short unresolved prefix |
| First note | `90` then properties at `0x318b5` | `90` then properties at `0x31c0c` |
| Later notes | timing + properties | timing + properties |
| Post-chain | `ff fb 8b 7d ff 2f 00 29 ...` | `ff fa b9 2f ff 2f 00 29 ...` |

The Patch explains why Track 3 #2 has substantial pre-note content, but it does
not resolve possible length/reference fields or complete event-stream header
grammar. Repeated framing remains strongly supported at the broad byte-
relationship level; exact count semantics do not.

# Engineering conclusions

- **A. YES:** observationally, one Patch plus 84 notes exactly explains 85.
- **B. PARTIAL:** name, program candidate, export correlation, and local mixed-
  event placement are identified, but complete Patch field framing is not.
- **C. PARTIAL:** the predicted interval is known, but no established single
  local field equals it; the cross-type timing rule is unresolved.
- **D. NO:** corrected Track 3 #2 has 86 in the analogous field versus 85 UI
  events, contradicting exact total-event-count semantics.
- **E. PARTIAL:** all 84 notes are mechanically accounted for and the Patch is
  localized/partially identified, but a complete 85-event decoder is not yet
  justified.
- **F. YES:** the note model matches 336/336 properties and 83/83 note timing
  intervals; the earlier 68/68 and 16/16 subset remains valid.

# Evidence supported

- The corrected Track 3 #2 note chain is `0x318b5–0x31afe`.
- The former 17-note validation covers note indices 33–49.
- All 84 screenshot notes match all four binary properties and all 83
  note-to-note timing intervals.
- `Ming Dynasty` is stored literally in the local pre-note region.
- Local/export evidence supports decimal 23 as a program-value candidate.
- `90` distinguishes the beginning of the note stream; a complete Patch type
  record is not yet decoded.
- The analogous 86/85 mismatch rejects an exact total-event-count reading.

# Unknowns

Exact Patch record start/end, timing ownership, whether `ff` is the Patch type,
the meanings of the other pre-note bytes, why the marker-minus-eight field is
86, length/reference fields, and generality beyond these two tracks remain
unknown.

# Single recommended next step

Experiment 023 subsequently changed only displayed `PC 23` to `PC 24`. The
preregistered candidate changed exactly `0x318a5: 17 -> 18`; `Ming Dynasty`,
the position candidates, first-note boundary, and complete 84-note chain stayed
byte-identical. Three bytes at `0x31883–0x31885` also changed and remain
unexplained header/dependent/save candidates. The direct Program Change field
is now identified, while complete Patch framing and timing remain unresolved.
See `CONTROLLED_TRACK3_2_PATCH_CHANGE.md`.

Perform a second controlled Program Change experiment using a deliberately
non-adjacent value while leaving name, position, and notes unchanged. This
tests direct field behavior beyond a one-step correlation and separates the
program byte from the unexplained three-byte header change.
