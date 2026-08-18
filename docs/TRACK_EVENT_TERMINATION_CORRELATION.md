# Objective

Determine the exact performance-event boundary inside a structurally supplied
track-primary container and identify the narrowest supported event-family
transition grammar. This is read-only correlation against the authenticated
Experiment 007 baseline; it does not design or implement a mixed-event walker.

# Scope and provenance

The artifact is `Experiment 007 - Untouched Baseline/newest STUFF baseline`,
211,468 bytes, SHA-256
`e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`.
Tracks were reached through the implemented root-record and 166-byte sequence
profile: sequence descriptor order selects a track pair, and the pair's
type-`0x02` primary record supplies its exact payload. No global event-signature
search is part of the navigation evidence.

# Track corpus

Ranges are half-open. `start` is primary payload `+14`. `end` is the newly
established performance-event exclusive end, primary payload end `-7`.

| Sequence / track | Primary record | Primary payload | `start..end` | Independent family evidence |
|---|---|---|---|---|
| Bells for her / Track 3 | `0x010a3a..0x0110cf` | `0x010a3f..0x0110cf` | `0x010a4d..0x0110c8` | authenticated ordinary Controller population and performance data |
| Bells for her / Track 4 | `0x011208..0x011931` | `0x01120d..0x011931` | `0x01121b..0x01192a` | authenticated ordinary Controller population and performance data |
| Bells for her / Track 6 | `0x011e99..0x0123e4` | `0x011e9e..0x0123e4` | `0x011eac..0x0123dd` | authenticated ordinary Controller population and performance data |
| Bells for her / Track 9 | `0x0143b5..0x01495e` | `0x0143ba..0x01495e` | `0x0143c8..0x014957` | Controller, Patch, Note, and 32-event Channel Pressure run |
| Bells for her / Track 14 | `0x014e13..0x015edb` | `0x014e18..0x015edb` | `0x014e26..0x015ed4` | Note, Controller, and nine Pitch Bend runs |
| Ode to Clarke / Track 1 | `0x02f820..0x02fa7a` | `0x02f825..0x02fa7a` | `0x02f833..0x02fa73` | bounded Patch followed by 91 correlated Notes |
| Ode to Clarke / Track 2 | `0x02fb42..0x0300df` | `0x02fb47..0x0300df` | `0x02fb55..0x0300d8` | bounded Patch followed by 211 correlated Notes |
| Ode to Clarke / Track 3 | `0x0312ed..0x03156b` | `0x0312f2..0x03156b` | `0x031300..0x031564` | bounded Patch followed by 84 mechanically bounded Notes |
| Ode to Clarke / Track 3 #2 | `0x031873..0x031b05` | `0x031878..0x031b05` | `0x031886..0x031afe` | bounded Patch followed by 84 export-correlated Notes |
| Ode to Clarke / Track 7 | `0x031bf5..0x031fa3` | `0x031bfa..0x031fa3` | `0x031c08..0x031f9c` | 143-note authenticated chain |
| Sequence K / Track 1 | `0x02597b..0x025a6c` | `0x025980..0x025a6c` | `0x02598e..0x025a65` | present as the one performance track in the controlled full export |
| Sequence K / Track 2 | `0x025b36..0x025b50` | `0x025b3b..0x025b50` | empty `0x025b49..0x025b49` | descriptor/pair exists but no exported performance track |

Each secondary type-`0x29` record begins exactly at its primary record end.
That adjacency supplies the primary containing end, but no secondary payload is
interpreted as performance-event metadata here.

The focused corpus contains 12 tracks. A supplemental structural census covered
all 132 track primary records in all 18 authenticated sequences.

# Primary container anatomy

Every corpus track has a 14-byte prefix before the candidate event start:

```text
00 01 | four-byte big-endian candidate count | 00 00 00 00 | four opaque bytes
```

The four-byte field at payload `+2..+6` is zero for all 15 empty candidates and
often resembles a displayed or exported event count. It is not established as
an exact List Window count: Track 3 #2 stores 86 for 85 displayed Patch-plus-Note
events, and Bells Track 9 stores 186 for 184 independently reconciled List
events. It therefore cannot delimit bytes or terminate a mixed-event walk.
The remaining header words are variable and have no established length, end,
or pointer semantics.

# Empty-track evidence

Sequence K Track 2 stores count zero and, after payload `+14`, contains exactly:

```text
ff ff ff 7f ff 2f 00
```

It has no bytes between the candidate event start and this seven-byte form and
is absent from the provenance-controlled Sequence K SMF as a performance
track. Fourteen additional zero-count track primaries have the identical
layout. This independently separates the seven bytes from performance-event
data: they remain present when the performance region is empty.

# Track-tail comparison

All 132 authenticated track primaries end in the exact seven-byte grammar:

```text
ff aa bb cc ff 2f 00
```

The first and final three bytes are invariant; `aa bb cc` vary. There are 108
complete tail values in the population. The empty value is
`ff ff ff 7f ff 2f 00` and occurs 16 times, including all 15 zero-count tracks.
The middle bytes are not decoded.

Boundary role is **PROVEN for the authenticated 166-byte profile**: the form is
at the exact declared payload end in 132/132 tracks; empty tracks contain only
the form after the common header; and independently bounded Note chains and the
last Track 14 Controller end exactly where it begins. The constant bytes and
placement are a **PROVEN repeated invariant**. The middle three bytes are a
**VARIABLE opaque field**. Whether `ff 2f 00` intentionally mirrors an SMF End
of Track meta-event and the semantic name of the complete seven-byte structure
remain **UNKNOWN**.

Consequently the exact performance-event range for this supported profile is:

```text
primary.payload.start + 14 .. primary.payload.end - 7
```

Both subtractions/additions must be checked, and the seven tail bytes must be
validated before using this stronger bound. This conclusion must not be
generalized to the unsupported 120-byte descriptor profile without evidence.

# Track 9 termination

The primary payload is `0x0143ba..0x01495e`; its event region is therefore
`0x0143c8..0x014957`. Bytes `0x014957..0x01495e` are
`ff f6 fd 6b ff 2f 00`.

This byte-exact result corrects an earlier durable annotation of the correlated
span as ending at `0x014956`. The byte at `0x014956` is `2f`, the fifth and
final property byte of the last Note structure beginning at `0x014952`; the
tail begins at `0x014957`. The earlier external Controller report is retained
unchanged as historical evidence.

The header count is 186 while the independently reconciled Studio Vision List
population is 184. That mismatch remains evidence against using the header
field as a termination count; it does not weaken the tail-derived byte bound.

# Track 14 termination

The primary payload is `0x014e18..0x015edb`; the event region is
`0x014e26..0x015ed4`. The final ordinary Controller is
`0x015ecb..0x015ed4`, followed immediately by
`ff f6 ea 53 ff 2f 00`.

The ninth and final Pitch Bend run ends much earlier at `0x0158d8` and exits to
an explicit Controller. Thus Pitch Bend run termination is distinct from track
termination. The primary tail supplies the eventual track end but does not
supply the nine internal Pitch Bend run boundaries.

# Patch-to-Note transitions

The four established Ode Patch starts are `0x02f833`, `0x02fb55`, `0x031300`,
and `0x031886`. Their first Note statuses are respectively `0x02f852`,
`0x02fb74`, `0x03131a`, and `0x0318b4`. Experiment 031 resolves the prior
variable-context ambiguity for this established corpus.

## Experiment 031 controlled result

Experiment 031 moved only Ode to Clarke Track 3 #2's first Note from `6·1·3`
to `6·1·4`. The primary remained `0x031873..0x031b05`, with unchanged length,
payload, event bounds, Patch framing, and first-Note status offset.

The preregistered final-component model was confirmed:

- final timing `0x0318b2..0x0318b4`: `81 25 -> 81 26`, VLQ 165 -> 166;
- post-PC timing `0x0318a6..0x0318a8`: unchanged `c5 4c` = 8,908;
- following Note timing `0x0318ba..0x0318bc`: `81 63 -> 81 62`, VLQ
  227 -> 226;
- first Note status remains `0x0318b4 = 90`.

Thus the Patch-to-first-Note interval rises from 8,908 + 165 = 9,073 to
8,908 + 166 = 9,074, exactly +1 Studio Vision position unit. The following
interval's independent -1 compensation agrees with moving the first Note one
unit later while leaving the next Note fixed.

The range `0x0318a8..0x0318b2` is not 12 opaque context bytes. It is the
10-byte length-framed structure:

```text
ff 60 | 07 | 57 7f 00 6c 6c a3 4a
```

followed separately by final timing VLQ `81 25`/`81 26`. Experiment 031 leaves
the complete `ff 60` record byte-identical. Its semantic purpose and the
meaning of its seven payload bytes remain unknown.

## Established direct and extended forms

The bounded corpus supports:

```text
position VLQ
ff 7c | payload_length | payload ending in direct Program Change
post-PC timing VLQ
[optional ff 60 | one-byte context_length | context payload | final timing VLQ]
90
```

Ode Tracks 1, 2, and 3 use the direct form: their post-PC timing VLQ is
followed immediately by `90` and owns the complete Patch-to-first-Note
interval. Ode Track 3 #2 and Bells Track 9 use the extended form: `ff 60 07`
explicitly bounds seven payload bytes, followed by a final timing VLQ and
`90`. In the established extended form the interval is the sum of the post-PC
and final timing values. Bells Track 9 independently satisfies that arithmetic
with 117,890 + 192 = 118,082.

A cursor can validate either branch and derive the first Note status without
searching. Patch-to-first-Note transition is therefore **YES for the
established bounded grammar**. This does not establish arbitrary `ff 60`
semantics, other context tags, repeated optional records, other Patch layouts,
or other Studio Vision versions.

# Note-chain termination

Ode Tracks 1, 2, 3, 3 #2, and 7 end their established Note chains exactly at
the newly established track-event end. This replaces the older
pitch/plausibility-only stopping condition for those terminal chains.

Once Note state is established, an ordinary continuation has a timing VLQ and
five property bytes, so its next cursor is locally derivable. A Note has no
collision-resistant family tag on every continuation, however, and evidence
does not establish how a Note run hands off to another family before the track
tail. General Note-chain termination is therefore **PARTIAL**: exact for a
Note chain known to occupy the remainder of the event region, unresolved for
an internal family transition.

# Controller transitions

An ordinary Controller record is self-delimiting once reached:

```text
timing VLQ | ff 41 | 05 | three context bytes | controller | value
```

The timing width plus eight fixed bytes determines the exact next cursor.
`ff 41 05` supplies collision-resistant current-cursor family identification
after the timing VLQ. Accordingly:

- Controller record length: **YES**;
- Controller family identification at the current cursor: **YES**;
- transition from Controller to another explicitly tagged/status family:
  **PARTIAL** because the next family's own rules still govern dispatch;
- track termination after Controller: **YES** under the validated seven-byte
  tail rule.

# Channel Pressure run boundaries

The proven Track 9 run `0x01478c..0x0147ce` has one explicit `d0` entry and 31
stateful timing/value continuations. The next event has explicit Note status
after its timing. This establishes the observed exit but not an intrinsic run
count, run length, subcontainer, or universal continuation/end discriminator.
The decoder still correctly requires entry state and caller-supplied run
bounds. Channel Pressure run bounds are **PARTIAL**; the missing fact is a
general current-cursor rule that distinguishes a continuation from the next
family without already knowing the run end.

# Pitch Bend run boundaries

Track 14 has nine correlated runs. Eight exit to explicit Note status and the
ninth exits to `ff 41 05` Controller structure. No count, length, or enclosing
subcontainer was found for any run. Pitch Bend run bounds are **PARTIAL**; the
missing fact is the same kind of proven state-exit rule for its timing/two-value
continuations.

# Family discriminator matrix

| Family | Known discriminator at a current event | Required context/state | Exact next cursor once identified | Ambiguity |
|---|---|---|---|---|
| Note | explicit entry `90`; no tag on ordinary continuations | Note state and timing-VLQ ownership | YES for a continuation | entry is strong; internal exit remains ambiguous |
| Patch | timing VLQ then `ff 7c`, plus payload length | direct or validated optional `ff 60` branch | YES for established transition grammar | unsupported optional forms must be rejected |
| Controller | timing VLQ then `ff 41 05` | track timing state | YES | collision-resistant at the current cursor |
| Channel Pressure | timing VLQ then entry `d0`; continuation omits status | active Pressure state | YES per bounded entry/continuation | run exit is not generally derived |
| Pitch Bend | timing VLQ then entry `e0`; continuation omits status | active Bend state | YES per bounded entry/continuation | run exit is not generally derived |

# Boundary-level distinction

- **A. Track-primary containing end: YES.** The type-`0x02` declared length
  supplies the exact record and payload end.
- **B. Exact performance-event-region end: YES for the authenticated 166-byte
  profile.** It is the validated payload end minus the seven-byte terminal
  form.
- **C. Individual family/run end: PARTIAL.** Controller records, terminal Note
  chains, and the established Patch-to-first-Note transition are bounded;
  internal Note and stateful Pressure/Bend exits are not general.

# Candidate transition grammar

Evidence now supports the outer cursor contract:

```text
validate primary header and seven-byte terminal form
event_start = payload.start + 14
event_end   = payload.end - 7
walk only within event_start..event_end
```

It also supports exact advancement through the established direct or extended
Patch transition, an identified Controller, and a Note/Pressure/Bend item while
the corresponding state is valid. It does not yet support a total
current-cursor dispatcher: stateful continuation bytes do not have a proven
universal exit rule into a new tagged/status-bearing family. Additional
family-transition evidence, not another container layer, is required.

# Highest-value blocker

The single highest-value blocker is current-cursor state-exit classification
after a timing VLQ. Evidence must distinguish established Note, Channel
Pressure, and Pitch Bend continuations from entry into a new tagged or
status-bearing family. This shared rule would unlock more mixed-event walking
than further Patch parameter work.

# Evidence supported

- exact performance-event start and end for every authenticated 166-profile
  track primary;
- an empty event region for Sequence K Track 2 and 14 other zero-count tracks;
- exact Controller next-cursor behavior and track termination;
- exact terminal ends for five established Ode Note chains;
- deterministic direct and extended Patch-to-first-Note navigation;
- Experiment 031's exact `81 25 -> 81 26` first-Note timing ownership and
  independent `81 63 -> 81 62` following-interval compensation;
- local entry tags for Patch, Controller, Pressure, and Bend;
- distinct track termination versus Pressure/Bend run termination.

# Unknowns

- semantics of the three variable bytes in the seven-byte terminal form;
- semantics and inclusions of the four-byte candidate count;
- internal Note-to-other-family handoff;
- general Channel Pressure and Pitch Bend state-exit rules;
- a total mixed-event current-cursor classifier;
- semantic purpose of `ff 60`, support for other context tags, or repeated
  optional context records;
- applicability of the terminal grammar to the older 120-byte profile.

# Experiment decision

**NO FURTHER PATCH EXPERIMENT NEEDED.** Experiment 031 confirmed the
preregistered final-component model and resolves Patch-to-first-Note navigation
for the established corpus. Further Patch parameter edits do not outrank
read-only state-exit correlation.

# Implementation gate

- exact track-primary containing bounds: **YES**;
- exact performance-event-region end: **YES** for the supported profile;
- Patch-to-Note transition: **YES for the established bounded grammar**;
- Note-chain termination: **PARTIAL** generally, exact for terminal chains;
- Controller next-cursor decoding: **YES**;
- Channel Pressure run bounds: **PARTIAL**;
- Pitch Bend run bounds: **PARTIAL**;
- mixed-event current-cursor family classification: **PARTIAL**;
- first bounded mixed-event walker design-ready: **PARTIAL**.

# Single recommended next step

Perform read-only current-cursor state-exit correlation after timing VLQs across
established Note, Channel Pressure, and Pitch Bend transitions before designing
a mixed-event walker.
