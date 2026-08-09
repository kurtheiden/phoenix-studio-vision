# Objective

This report analyzes Experiment 022, which moved the fifth Studio Vision List
Window event one position unit later, from `26·1·469` to `26·1·470`. It tests
preregistered equal-and-opposite changes in the timing fields immediately
before and after the already identified fifth-event properties. This is
read-only evidence gathering; it does not claim complete event framing or
Standard MIDI File delta-time semantics.

# Experimental provenance

Experiment 007 is the verified-working baseline. Experiment 022 was made from
a fresh native Finder duplicate. In Studio Vision's List Window, the user
changed only the fifth event's position, retained pitch D1, attack velocity
127, release velocity 86, duration 245, and note count, saved on quit, reopened
and verified the project as functional, quit without saving, and Finder-copied
it through `Unix` into the research directory.

Directory inspection positively identified the uncompressed project as
`/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 022 - Next Event Position Up One/newest STUFF baseline EXP22`.
It is 211,468 bytes and has SHA-256
`fab15f1ab97a1e2d19d2ffc2e9bf85e204ddb146aa7c27946eb6b97db6841295`.
Finder Type is `MID2`, Creator is `MIDA`, and the observed attributes are
`com.apple.FinderInfo` (32 bytes), `org.BasiliskII.FinderInfo` (16 bytes), and
`org.BasiliskII.ExtendedFinderInfo` (16 bytes). No other attribute or resource
fork was observed.

# Preregistered results

The fifth structure was located by its properties and neighboring timing
fields, not by assuming an offset. It remains at the baseline location in
Experiment 022. All three preregistered predictions succeed exactly:

| Candidate | Baseline | Experiment 022 prediction | Observation | Result |
|---|---|---|---|---|
| fifth properties, `0x00031c26–0x00031c2a` | `26 7f 56 81 75` | unchanged | `26 7f 56 81 75` | succeeds |
| leading timing, `0x00031c24–0x00031c25` | `83 60` = 480 | `83 61` = 481 | `83 61` = 481 | succeeds |
| following timing, `0x00031c2b–0x00031c2c` | `81 70` = 240 | `81 6f` = 239 | `81 6f` = 239 | succeeds |

The exact Experiment 022 sequence is:

`83 61 | 26 7f 56 81 75 | 81 6f`

# Fourth-event controls

The fourth-event property sequence at `0x00031c1f–0x00031c23` remains exactly
`24 7f 5c 83 3a`. Its immediately preceding interval at
`0x00031c1d–0x00031c1e` also remains `81 65`, VLQ 229. Thus the fifth-event
position edit does not change the fourth event's confirmed pitch, attack
velocity, release velocity, or duration, nor the interval immediately before
the fourth event.

The 81 pitch-bearing positions reconstructed from the bidirectional pitch
controls all remain `0x24`; there are zero `0x23`, `0x25`, or other values.

# Fifth-event timing table

| Displayed position | Artifact | Leading interval | Fifth-event properties | Following interval | Sum |
|---|---|---:|---|---:|---:|
| `26·1·469` | Experiment 007 | `83 60` = 480 | `26 7f 56 81 75` | `81 70` = 240 | 720 |
| `26·1·470` | Experiment 022 | `83 61` = 481 | `26 7f 56 81 75` | `81 6f` = 239 | 720 |

Moving the event one unit later increases the leading value by one and
decreases the following value by one. Their sum remains 720.

# Comparison with prior timing experiment

Experiments 019 and 020 moved the fourth event one unit earlier and later. Its
leading interval formed 228, 229, 230; its following interval formed 481, 480,
479; and the sum remained 709. Experiment 022 applies the same structural rule
to the next event: moving it later increases the immediately preceding field
and decreases the immediately following field while its properties remain
stable.

This is controlled evidence that the compensating timing behavior generalizes
from the fourth event to the fifth. Only the later direction has been tested
on the fifth event, so its response is not itself bidirectional yet.

# Timing ownership analysis

The observations strongly support interpreting the two values as intervals
between adjacent event-property structures. The on-disk order also makes it
convenient to parse each timing VLQ as a prefix to the property structure that
follows it (Model A). Describing the values as inter-event intervals without
assigning ownership (Model C) is the most conservative format-level statement.

Model B, in which each timing VLQ belongs exclusively to the preceding event,
is less consistent with the repeated `[timing][properties]` ordering, but the
controlled changes alone do not establish semantic ownership. The evidence
does not establish that these are SMF delta-times, their time unit, or whether
the format itself defines ownership rather than merely sequence.

# Next-event candidate

Immediately after Experiment 022's following timing field `81 6f` is the
unchanged property-like sequence at `0x00031c2d–0x00031c30`:

`24 7f 60 6b`

It is mechanically consistent with the established property order: a
pitch-like byte `24`, attack-like byte `7f`, release-like byte `60`, and
one-byte duration VLQ `6b` = 107. The timing field `81 70` in baseline and
`81 6f` in Experiment 022 sits immediately before it. No documented Studio
Vision row values for this sixth candidate were available in the inspected
evidence, so this remains a provisional structure rather than a verified event
mapping.

# Structural-alignment handling

Experiment 022 and Experiment 007 are both 211,468 bytes. Their `Track 7`
labels, including the relevant occurrence at `0x0002f6ca`, and `Ode to Clarke`
at `0x0002f753` remain aligned. The fourth event, fifth event, and following
candidate remain at baseline offsets. No serialization expansion or event
region shift occurred in Experiment 022.

Experiment 021's separate 56-byte expansion was therefore not used as a fixed
offset template. Its fifth event was previously aligned structurally. Here,
the same sequence and neighbors demonstrate that ordinary same-position
comparison is valid in the controlled event region.

# Control filtering

Experiment 007 and Experiment 022 differ at 1,762 same-position bytes. Of
these, 1,761 offsets had varied in at least one prior same-layout controlled
artifact; `0x00031c2c` is the only newly variable absolute offset relative to
that prior set. The other timing response, `0x00031c25`, had already varied in
Experiments 019 and 020 because it is also the field following the fourth
event.

Against unrelated pitch, attack-velocity, release-velocity, and duration
controls, both timing offsets are stable and respond only to controlled
position changes. Classification is therefore:

- fifth-event timing candidates: `0x00031c25`, `60 -> 61`, and
  `0x00031c2c`, `70 -> 6f`;
- recurring or previously variable save-output positions: the other 1,760;
- serialization/layout variation: none in Experiment 022;
- other new control-stable musical-data differences: zero;
- Experiment-022-only unresolved musical-data differences: zero.

# Whole-file comparison

Both files are 211,468 bytes, for a size delta of zero. They differ at 1,762
same-position bytes in 633 disjoint runs. The first difference is
`0x0000001e`, the last is `0x00033a06`, the maximum unequal run is 36 bytes,
the common prefix is 30 bytes, and the common suffix is five bytes.

Because labels and the event stream remain aligned and there is no size
change, these statistics are not distorted by an Experiment-021-like layout
shift. The broad difference population is consistent with recurring save-run
variation; control filtering isolates the two preregistered musical-data
responses.

# Consecutive event-chain evidence

The local baseline and Experiment 022 sequences are:

- baseline:
  `81 65 | 24 7f 5c 83 3a | 83 60 | 26 7f 56 81 75 | 81 70 | 24 7f 60 6b`;
- Experiment 022:
  `81 65 | 24 7f 5c 83 3a | 83 61 | 26 7f 56 81 75 | 81 6f | 24 7f 60 6b`.

Current evidence supports a provisional local chain:

`[timing interval] [event properties] [timing interval] [event properties] ...`

with properties ordered as `[pitch] [attack velocity] [release velocity]
[duration VLQ]`. The fourth-event properties are independently controlled
field by field. The fifth-event pitch is independently controlled, and its
other displayed values match and remain stable. The timing behavior is
bidirectionally controlled around the fourth event and one-directionally
controlled around the fifth. The sixth property-like structure, exact event
boundaries, timing ownership, status/channel representation, and broader track
framing remain provisional.

# Evidence supported

- Experiment 022 contains `83 61 | 26 7f 56 81 75 | 81 6f` at the expected
  structural location.
- All three preregistered predictions succeed exactly.
- The fourth event and its leading `81 65` interval remain unchanged.
- All 81 established pitch anchors remain `0x24`.
- Fifth-event leading and following intervals change +1 and -1 and preserve
  constant sum 720.
- The compensating timing rule generalizes from the fourth event to the fifth.
- The changed following interval sits immediately before an unchanged next
  property-like structure.
- No structural shift and no other new control-stable musical-data difference
  were observed.

# Unknowns

- Exact ownership of timing fields remains provisional; interval behavior is
  better supported than an ownership label.
- Whether the timing values are SMF delta-times and their internal unit are
  unknown.
- The sixth and later candidate structures lack documented List Window values.
- Complete event boundaries, channel/status representation, track framing,
  absolute-time reconstruction, and non-note event encoding remain unknown.
- No valid Standard MIDI File has yet been recovered.

# Single recommended next step

Implement a bounded, diagnostic-only parser spike for the known Track 7 event
region. It should identify consecutive timing/property structures, decode
pitch, attack velocity, release velocity, and duration, and accumulate the
timing intervals into provisional event positions while emitting offsets and
alignment diagnostics. It should not produce MIDI output yet. This has higher
information value than another one-unit edit because the same timing/property
rule now reproduces across two consecutive controlled events.
