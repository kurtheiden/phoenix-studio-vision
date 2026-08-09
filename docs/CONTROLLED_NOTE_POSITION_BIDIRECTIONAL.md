# Objective

This report analyzes Experiment 020, which moved the established C1 drum note
one position unit later, from `25·4·469` to `25·4·470`. It is the
bidirectional counterpart to Experiment 019's one-unit-earlier edit and tests
exact opposite predictions for two adjacent timing candidates. The work is
read-only evidence gathering and does not claim complete event framing or MIDI
delta-time semantics.

# Experimental provenance

Experiment 007 is the known-good baseline. Experiment 020 was created from a
fresh native Finder duplicate of that baseline. In Studio Vision's List
Window, the user moved only the fourth event from `25·4·469` to `25·4·470`,
retaining pitch C1, attack velocity 127, release velocity 92, duration 442, and
note count. The user saved on quit, reopened and verified the project as
functional, quit without saving, and Finder-copied it through `Unix` into the
research directory.

Directory inspection positively identified the uncompressed project as
`/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 020 - Single MIDI Note Position Up One/newest STUFF baseline EXP20`.
It is 211,468 bytes and has SHA-256
`35434b73dcaec131140dad7cc8a8ccd5efdebcaf9a54706dcf0b3c97caf7fae2`.
Finder Type is `MID2`, Creator is `MIDA`, and the observed attributes are
`com.apple.FinderInfo` (32 bytes), `org.BasiliskII.FinderInfo` (16 bytes), and
`org.BasiliskII.ExtendedFinderInfo` (16 bytes). No other attribute or resource
fork was observed.

# Preregistered results

All three preregistered predictions succeed exactly:

| Candidate | Baseline | Experiment 020 prediction | Experiment 020 observation | Result |
|---|---|---|---|---|
| five properties, `0x00031c1f–0x00031c23` | `24 7f 5c 83 3a` | unchanged | `24 7f 5c 83 3a` | succeeds |
| preceding VLQ, `0x00031c1d–0x00031c1e` | `81 65` = 229 | `81 66` = 230 | `81 66` = 230 | succeeds |
| following VLQ, `0x00031c24–0x00031c25` | `83 60` = 480 | `83 5f` = 479 | `83 5f` = 479 | succeeds |

Pitch, attack velocity, release velocity, and duration remain byte-for-byte
stable while both adjacent candidates respond in the exact opposite direction
from Experiment 019.

# Bidirectional timing table

| Displayed position | Artifact | Preceding VLQ | Five-byte properties | Following VLQ | Sum |
|---|---|---:|---|---:|---:|
| `25·4·468` | Experiment 019 | `81 64` = 228 | `24 7f 5c 83 3a` | `83 61` = 481 | 709 |
| `25·4·469` | Experiment 007 | `81 65` = 229 | `24 7f 5c 83 3a` | `83 60` = 480 | 709 |
| `25·4·470` | Experiment 020 | `81 66` = 230 | `24 7f 5c 83 3a` | `83 5f` = 479 | 709 |

For each one-unit increase in displayed position, the preceding candidate
increases by exactly one and the following candidate decreases by exactly one.
Their sum remains exactly 709 in all three experiments. From Experiment 019 to
020, the preceding value changes 228 to 230 while the following value changes
481 to 479.

# Musical-property controls

Experiment 020 retains the confirmed property structure:

- pitch: `0x24`, C1;
- attack velocity: `0x7f`, decimal 127;
- release velocity: `0x5c`, decimal 92;
- duration: `83 3a`, VLQ 442.

The 81 established pitch anchors were reconstructed from the bidirectional
pitch controls. All 81 remain `0x24`; there are zero exceptions.

# Control filtering

Experiments 009, 010, and 013–018 retain baseline values at both timing
candidates during unrelated pitch, attack-velocity, release-velocity, and
duration edits. Experiments 005, 006, 008, 011, 012, and 019 were also used to
identify previously variable save output.

Relative to the complete prior control set through Experiment 019, Experiment
020 introduces no newly variable offset: the two timing offsets first varied
in Experiment 019. Relative to the unrelated musical-property controls, the
only two control-stable position-correlated offsets are again:

- `0x00031c1e`, baseline `65`, Experiment 019 `64`, Experiment 020 `66`;
- `0x00031c25`, baseline `60`, Experiment 019 `61`, Experiment 020 `5f`.

A whole-file search finds no other byte with the same exact bidirectional
`-1, baseline, +1` or `+1, baseline, -1` relationship. No other unexplained
control-stable Experiment-020 difference remains.

Experiment 019 and 020 have equal sizes and differ at 1,174 same-position
bytes in 396 runs. Their first difference is `0x00000fdb`, last difference
`0x0003380a`, maximum unequal run three bytes, common prefix 4,059 bytes, and
common suffix 513 bytes. Those broad differences are save-run variation; only
the two candidates exhibit the preregistered bidirectional relationship.

# Delta-style timing analysis

The evidence strongly supports compensating delta-style timing intervals:

- moving the middle event earlier shortens the candidate leading into it and
  lengthens the candidate following it;
- moving the event later produces the exact opposite changes;
- each interval changes one-for-one with displayed position;
- their sum remains constant;
- the controlled event's musical-property bytes remain unchanged.

This response is difficult to explain as two unrelated run-varying values. It
also weighs against storage of the displayed tick component as one isolated
absolute field. The evidence does not prove these are Standard MIDI File delta
times, does not yet establish the internal time unit, and does not alone prove
which event owns each field.

# Local structural ordering

The most evidence-supported local sequences are:

| Artifact | Local sequence |
|---|---|
| Experiment 019, earlier | `81 3b 81 64 24 7f 5c 83 3a 83 61 26 7f 56 81 75 81 70 24` |
| Experiment 007, baseline | `81 3b 81 65 24 7f 5c 83 3a 83 60 26 7f 56 81 75 81 70 24` |
| Experiment 020, later | `81 3b 81 66 24 7f 5c 83 3a 83 5f 26 7f 56 81 75 81 70 24` |

The provisionally supported order is:

1. stable preceding value `81 3b` = 187;
2. timing candidate leading into the controlled note;
3. controlled note properties: pitch, attack velocity, release velocity,
   duration VLQ;
4. compensating timing candidate;
5. a plausible next property sequence `26 7f 56 81 75`;
6. another stable VLQ `81 70`, followed by the next confirmed pitch anchor.

The unchanged `81 3b` occupies `0x00031c1b–0x00031c1c`, immediately before the
preceding timing candidate, and remains value 187 across all controls. The
repeated layout suggests it may be the variable-width duration ending a prior
property structure (`26 7f 36 81 3b`), but that field has not been
independently controlled and its semantics remain provisional.

# Neighboring-event analysis

The following candidate `83 60` occupies `0x00031c24–0x00031c25`. It is
immediately followed at `0x00031c26` by `26 7f 56 81 75`, a compact sequence
structurally analogous to the confirmed order:

- possible pitch `26`;
- possible attack velocity `7f`;
- possible release velocity `56`;
- possible duration VLQ `81 75` = 245.

This places the following timing candidate immediately before a plausible next
event property structure. The next confirmed `0x24` pitch anchor is at
`0x00031c2d`, preceded by another stable VLQ `81 70`.

The structural ordering and equal-and-opposite controlled response support
interpreting `83 60` as an interval leading into the immediately following
property structure more strongly than as part of the controlled note's five
properties. Ownership is still provisional because the next `26` structure
has not itself been independently manipulated.

Experiment 021 subsequently changed only that following structure's pitch from
D1 to C#1. Under a 56-byte event-stream shift, the sequence changed exactly
`26 7f 56 81 75 -> 25 7f 56 81 75`, while preceding `83 60` and the fourth
event remained stable. This independently identifies the sequence following
`83 60` as the fifth List Window event. See
`CONTROLLED_NEXT_EVENT_PITCH_CHANGE.md`.

# Whole-file comparison

Experiments 007 and 020 both contain 211,468 bytes. They differ at 1,761
same-position bytes in 633 disjoint runs. The first difference is
`0x0000001e`, the last is `0x00033a06`, the maximum unequal run is 36 bytes,
the common prefix is 30 bytes, and the common suffix is five bytes.

`Track 7` remains at `0x0002f6ca`, `Ode to Clarke` remains at
`0x0002f753`, and the established label cadence remains aligned. Same-position
comparison is used without inferring insertions or deletions.

The 1,761 differences classify as:

- position/timing-correlated candidates: two previously identified offsets,
  `0x00031c1e` and `0x00031c25`;
- recurring or previously variable save-output positions: the other 1,759;
- newly variable offsets relative to all prior artifacts: zero;
- Experiment-020-only unresolved control-stable offsets: zero.

# Evidence supported

- All three Experiment 020 preregistered predictions succeed exactly.
- The confirmed property sequence remains `24 7f 5c 83 3a`, and all 81 pitch
  anchors remain `0x24`.
- The preceding candidate forms 228, 229, 230 across earlier, baseline, and
  later positions.
- The following candidate forms 481, 480, 479 across the same positions.
- Both candidates change one-for-one in opposite directions and maintain
  constant sum 709.
- No other whole-file offset exhibits the same control-stable bidirectional
  relationship.
- The following candidate sits immediately before a plausible next event
  property sequence.
- The evidence strongly supports bidirectionally position-correlated,
  compensating delta-style timing intervals.
- The evidence does not yet establish MIDI delta-time semantics, exact field
  ownership, or complete event boundaries.

# Unknowns

- The exact ownership of each timing interval remains provisional.
- The relationship between internal values 229/480 and Studio Vision's
  displayed position remains unknown.
- Stable VLQ 187 has not been independently manipulated; its apparent role as
  a prior duration remains provisional.
- The next `26 7f 56 81 75` property sequence has not been independently
  controlled.
- Absolute-time reconstruction, internal time units, channel/status encoding,
  track-event framing, and broader event ordering remain unknown.
- No valid Standard MIDI File has yet been recovered.

# Single recommended next step

Experiment 021 independently identified the fifth property structure; see
`CONTROLLED_NEXT_EVENT_PITCH_CHANGE.md`. The next controlled experiment should
move only that fifth event from `26·1·469` to `26·1·470`, testing whether its
leading `83 60` interval increases to `83 61` and the following `81 70`
interval decreases to `81 6f` while both identified property structures remain
stable. Experiment 022 subsequently confirmed all of those predictions and
preserved the interval sum of 720; see
`CONTROLLED_NEXT_EVENT_POSITION_CHANGE.md`. The same compensating timing rule
therefore applies to two consecutive controlled events.
