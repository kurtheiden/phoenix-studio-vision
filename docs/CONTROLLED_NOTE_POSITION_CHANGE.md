# Objective

This report analyzes Experiment 019, a controlled position-only edit of the
established C1 drum note from `25·4·469` to `25·4·468`. The confirmed
five-byte musical-property structure is used as a fixed control while nearby
changes are tested for timing relationships. The work is read-only evidence
gathering and does not claim complete event framing.

# Experimental provenance

Experiment 007 is the known-good baseline. Experiment 019 was created from a
fresh native Finder duplicate of that baseline. In Studio Vision's List
Window, the user moved only the fourth event from position `25·4·469` to
`25·4·468`, retaining pitch C1, attack velocity 127, release velocity 92,
duration 442, and note count. The user saved on quit, reopened and verified the
project as functional, quit without saving, and Finder-copied it through
`Unix` into the research directory.

Directory inspection positively identified the uncompressed project as
`/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 019 - Single MIDI Note Position Down One/newest STUFF baseline EXP19`.
It is 211,468 bytes and has SHA-256
`1fc50f59416f64271004f174f5b68566672301ce072ad6e863f6fad90a76b217`.
Finder Type is `MID2`, Creator is `MIDA`, and the observed attributes are
`com.apple.FinderInfo` (32 bytes), `org.BasiliskII.FinderInfo` (16 bytes), and
`org.BasiliskII.ExtendedFinderInfo` (16 bytes). No other attribute or resource
fork was observed.

# Five-byte property control

Before broad comparison, `0x00031c1f–0x00031c23` was inspected. Experiment 019
contains exactly:

`24 7f 5c 83 3a`

The preregistered five-byte stability prediction succeeds. Pitch remains C1
`0x24`, attack velocity remains 127 `0x7f`, release velocity remains 92
`0x5c`, and duration remains `83 3a`, mechanically decoding as 442 under the
established 7-bit big-endian VLQ calculation.

All 81 established pitch anchors also remain `0x24`; there are zero `0x23`,
`0x25`, or other exceptions.

# Preceding-byte result

The four immediately preceding bytes change from baseline `81 3b 81 65` to:

`81 3b 81 64`

Treated as two standard 7-bit big-endian VLQs:

| Field | Baseline | Experiment 019 | Mechanical values |
|---|---|---|---:|
| first | `81 3b` | `81 3b` | 187 to 187 |
| second | `81 65` | `81 64` | 229 to 228 |

Only the second preceding VLQ changes, decreasing by exactly one. This is an
observation about an adjacent numeric field; its timing meaning is assessed
with the compensating change below rather than assumed from proximity alone.

# Local neighborhood analysis

Within `0x00031bdf–0x00031ca0`, five bytes differ. Three at
`0x00031c05–0x00031c07` are previously variable save output. Two are stable in
every clean property control and occur around the confirmed structure:

| Field | Offset(s) | Relative to pitch | Baseline | Experiment 019 | Mechanical change |
|---|---|---:|---|---|---|
| preceding VLQ | `0x00031c1d–0x00031c1e` | `-2..-1` | `81 65` | `81 64` | 229 to 228 |
| following VLQ | `0x00031c24–0x00031c25` | `+5..+6` | `83 60` | `83 61` | 480 to 481 |

The intervening confirmed properties at relative `+0..+4` remain byte-for-byte
stable. The two candidate values change by equal and opposite amounts around
the edited note structure.

# Direct position-value search

The displayed component values mechanically encode as:

- 469 decimal: fixed-width `0x01d5`; VLQ `83 55`;
- 468 decimal: fixed-width `0x01d4`; VLQ `83 54`.

No literal 16-bit or 32-bit big- or little-endian `469 -> 468` transition was
found. No `83 55 -> 83 54` VLQ transition was found. The file therefore does
not expose the changed displayed component directly in any tested standard
integer form.

Two control-stable compact values do track the edit mechanically by one:
`81 65 -> 81 64` (229 to 228) and `83 60 -> 83 61` (480 to 481). They are
transformed or contextual timing candidates, not direct representations of the
displayed value 469.

# Timing-model analysis

The evidence favors a delta-style relationship over literal displayed-component
or simple absolute-value storage:

- moving the edited note one unit earlier decreases the preceding adjacent VLQ
  by one;
- a following adjacent VLQ increases by one;
- the note's confirmed five property bytes remain stable;
- the net change across the two candidate intervals is zero.

This is the expected qualitative compensation if one interval into the edited
event shortens while an interval onward to a later event lengthens. The
comparison does not establish whether these are deltas from the immediately
preceding and to the immediately following note, another pair of internal
timing intervals, or values in the displayed tick unit. The individual values
229 and 480 do not directly equal displayed components 25, 4, or 469.

Absolute timing cannot be excluded merely because the direct displayed value
is absent: Studio Vision may transform position into another origin or unit.
Nevertheless, the paired equal-and-opposite response is substantially more
consistent with delta-style timing than with one isolated absolute field.

# Dense-anchor neighborhood analysis

The same 12-byte bounded neighborhoods around all 80 dense pitch anchors were
compared. Two neighborhoods change:

- the first anchor at `0x00031c1f` includes both timing candidates, relative
  `-1` (`65 -> 64`) and `+6` (`60 -> 61`);
- the second anchor at `0x00031c2d` includes the following candidate at
  relative `-8` (`60 -> 61`).

The remaining 78 neighborhoods are byte-identical. The five-byte property
structure following the first anchor is unchanged. Thus more than one
neighboring anchor neighborhood responds, but through the same following VLQ,
not through a second independent changed field.

No recurring timing-correlated relative offset is established from one edit.
The overlap does show that the compensating candidate lies between the edited
note properties and the next pitch-bearing structure.

# Control filtering

Experiments 009, 010, and 013–018 retain baseline values at both timing
candidates through controlled pitch, attack-velocity, duration, and
release-velocity edits. Experiments 005, 006, 008, 011, and 012 were also used
to subtract known save-output variation.

Of the 1,762 baseline/Experiment-019 unequal offsets, 1,760 had already varied
in a prior artifact. Only two offsets are new relative to the complete control
set:

- `0x00031c1e: 65 -> 64`, low-order byte of VLQ 229 to 228;
- `0x00031c25: 60 -> 61`, low-order byte of VLQ 480 to 481.

Both have exact position-edit-correlated numeric relationships. There are no
Experiment-019-only unresolved offsets after separating these candidates.

# Whole-file comparison

Experiments 007 and 019 both contain 211,468 bytes. They differ at 1,762
same-position bytes in 633 disjoint runs. The first difference is
`0x0000001e`, the last is `0x00033a06`, the maximum unequal run is 36 bytes,
the common prefix is 30 bytes, and the common suffix is five bytes.

`Track 7` remains at `0x0002f6ca`, `Ode to Clarke` remains at
`0x0002f753`, and the established labels and cadence remain aligned.
Same-position comparison is used without inferring insertions or deletions.

Classification is:

- recurring or previously variable save-output positions: 1,760;
- timing/position-correlated candidates: two bytes in two adjacent VLQs;
- Experiment-019-only unresolved positions: zero.

# Neighboring-event observations

The dense region contains repeated pitch-bearing structures at variable
distances. The confirmed note begins at pitch anchor `0x00031c1f`; the next
pitch anchor is at `0x00031c2d`, 14 bytes later. The compensating
`83 60 -> 83 61` field lies between them at `0x00031c24–0x00031c25`.

Several anchors are followed by bytes consistent with the confirmed order of
pitch, attack velocity, release velocity, and a variable-width duration, but
the differing duration widths and intervening values prevent complete
segmentation without parser assumptions. For the controlled note, the field
order is independently established; for neighboring notes it remains
provisional.

The edited note's preceding candidate decreases while a value between its
property structure and the next anchor increases. This is evidence potentially
consistent with compensating adjacent-event deltas. It does not yet prove
which event owns either field or establish full record boundaries.

Experiment 020 subsequently moved the same note one unit later. The property
bytes remained stable, while the preceding VLQ became `81 66` = 230 and the
following VLQ became `83 5f` = 479. Together with Experiment 019 and baseline,
the candidates form exact bidirectional series 228/229/230 and 481/480/479
with constant sum 709. See `CONTROLLED_NOTE_POSITION_BIDIRECTIONAL.md`.

# Evidence supported

- Experiment 019 retains exact property sequence `24 7f 5c 83 3a`.
- All 81 pitch anchors remain `0x24`.
- The preceding four bytes become `81 3b 81 64`; only the second VLQ changes,
  229 to 228.
- A following VLQ changes `83 60 -> 83 61`, 480 to 481.
- These are the only two new offsets after complete control subtraction.
- No literal fixed-width or VLQ representation directly changes 469 to 468.
- The paired `-1` and `+1` changes around an unchanged note-property structure
  strongly favor a compensating delta-style timing relationship.
- Two neighboring dense-anchor windows respond because the following candidate
  lies between the first and second pitch anchors.
- Complete event framing and ownership of the timing values remain
  unestablished.

# Unknowns

- Whether the two changing VLQs are immediate event deltas, other timing
  intervals, or values in another internal unit remains unknown.
- Why the values are 229 and 480 relative to displayed position `25·4·469`
  remains unknown.
- The role of unchanged preceding VLQ 187 remains unknown.
- The start and end of each variable-length note structure are not established.
- Channel, status, track ownership, event ordering, and conversion to absolute
  musical time remain unknown.
- No valid Standard MIDI File has yet been recovered.

# Single recommended next step

Experiment 020 completed the one-unit-later test and satisfied both opposite
predictions; see `CONTROLLED_NOTE_POSITION_BIDIRECTIONAL.md`. The next
controlled experiment should move only the immediately following fifth List
Window event by one unit, testing ownership of the intervening `83 60` field
and the next `81 70` interval while the fourth event remains stable.
