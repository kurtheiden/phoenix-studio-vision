# Objective

Determine how a bounded Studio Vision performance-event walker can classify
the byte immediately after a timing VLQ, continue or exit active family state,
and stop at the exact track-event end without scanning, backtracking, or
musical-plausibility tests.

# Scope

The primary artifact is the untouched Experiment 007 `newest STUFF baseline`,
SHA-256
`e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`.
Navigation uses the implemented 166-byte sequence/container profile and the
validated event bounds:

```text
event_start = primary.payload.start + 14
event_end   = primary.payload.end - 7
```

The detailed state-exit corpus is Bells for her Track 9
`0x0143c8..0x014957` and Track 14 `0x014e26..0x015ed4`. Their independent SMF
correlations establish event family, value, timing, and order before this
current-cursor rule is tested.

# Transition corpus

The two tracks contain 785 independently accounted Studio Vision List events:
184 in Track 9 and 601 in Track 14. Treating each event's successor, or exact
`event_end` for the last event, as one transition gives 785 analyzed
transitions. Counts are:

| Current -> next | Track 9 | Track 14 | Total |
|---|---:|---:|---:|
| Note -> Note | 20 | 212 | 232 |
| Note -> Controller | 10 | 7 | 17 |
| Note -> Channel Pressure | 0 | 0 | 0 observed |
| Note -> Pitch Bend | 0 | 8 | 8 |
| Controller -> Controller | 109 | 264 | 373 |
| Controller -> Note | 9 | 6 | 15 |
| Controller -> Channel Pressure | 1 | 0 | 1 |
| Controller -> Pitch Bend | 0 | 1 | 1 |
| Controller -> Patch | 1 | 0 | 1 |
| Patch -> Note | 1 | 0 | 1 |
| Channel Pressure -> Channel Pressure | 31 | 0 | 31 |
| Channel Pressure -> Note | 1 | 0 | 1 |
| Channel Pressure -> Controller | 0 | 0 | 0 observed |
| Pitch Bend -> Pitch Bend | 0 | 93 | 93 |
| Pitch Bend -> Note | 0 | 8 | 8 |
| Pitch Bend -> Controller | 0 | 1 | 1 |
| Note -> event_end | 1 | 0 | 1 |
| Controller -> event_end | 0 | 1 | 1 |

The requested Note-to-Pressure and Pressure-to-Controller classes do not occur
in the authenticated population and are not invented. Their target entry
forms are nevertheless observed after other families.

Representative exact transitions include:

- Track 9 Controller `0x0143c8..0x0143d1` -> Patch at `0x0143d1`, timing
  `8f 00`, then `ff 7c`;
- Track 9 Controller -> extended Note at `0x01450e..0x014521`, timing `2b`,
  then `ff 60 07`, seven context bytes, final timing `81 29`, and `90`;
- Track 9 Controller `0x014783..0x01478c` -> Pressure entry
  `0x01478c..0x014790`, timing `82 20`, then `d0`;
- Track 9 final Pressure `0x0147cc..0x0147ce` -> Note
  `0x0147ce..0x0147d6`, timing `83 56`, then `90`;
- Track 14 Controller `0x015416..0x01541f` -> Bend entry
  `0x01541f..0x015424`, timing `8a 51`, then `e0`;
- Track 14 Bend `0x015436..0x015439` -> Note
  `0x015439..0x015440`, timing `31`, then `90`;
- Track 14 Note `0x015439..0x015440` -> Bend re-entry
  `0x015440..0x015444`, timing `0c`, then `e0`;
- Track 14 final Bend `0x0158d5..0x0158d8` -> Controller
  `0x0158d8..0x0158e2`, timing `83 5b`, then `ff 41 05`;
- Track 9 final Note `0x014950..0x014957` -> exact event end;
- Track 14 final Controller `0x015ecb..0x015ed4` -> exact event end.

Every population count and representative boundary agrees with the prior
event-order, value, and SMF correlations. The byte classifier is the tested
hypothesis, not the source of family identity.

# Post-VLQ byte classes

The 783 internal transitions divide without overlap:

| First byte after next timing VLQ | Count | Observed role |
|---|---:|---|
| `00..7f` | 356 | continuation of active Note, Pressure, or Bend state |
| `ff` | 393 | tagged Controller/Patch or length-framed Note-entry context |
| `80..ef` explicit status | 34 | `90`, `d0`, or `e0` family entry/re-entry |

Two further transitions stop at `event_end` before decoding another VLQ.
There are no first-payload high-bit collisions in 356 state continuations and
no observed untagged new-family entry.

# Stateful continuation model

The evidence supports a running-status-like structural analogy, but not literal
MIDI running status or stored MIDI channel semantics:

- **Note: SUPPORTED ANALOGY.** `90` establishes or re-enters Note state;
  consecutive Notes omit it. A high-bit branch exits the state.
- **Channel Pressure: SUPPORTED ANALOGY.** `d0` establishes state, 31
  continuations omit it, and explicit `90` exits it.
- **Pitch Bend: SUPPORTED ANALOGY.** `e0` establishes state in nine runs, 93
  continuations omit it, and explicit `90` or tagged `ff 41 05` exits it.

The observed project status low nibble is always zero. Exported channels differ
(`db` for Pressure and `ee` for Bend), so that zero is not a literal exported
MIDI channel. Its broader meaning remains unresolved.

# Note continuation and exits

After a Note continuation timing VLQ, the representation is:

```text
pitch | attack | release | duration VLQ
```

There are three direct property bytes, followed by a one-to-four-byte duration
VLQ; the total after-timing width is therefore variable, not always five bytes.
Across 232 authenticated Note continuations, the first property byte is always
below `80` (observed maximum `6a`). Timing widths are 15 one-byte, 216 two-byte,
and one three-byte VLQ. Pitch/velocity correlations establish seven-bit data
values in the natural population. Duration may legitimately contain high-bit
VLQ continuation bytes, but those occur only after the first three property
positions and cannot collide with the first-byte dispatch decision.

All 25 internal Note exits use a high-bit branch: 17 tagged Controllers and
eight explicit `e0` Bend entries. No Note-to-Pressure or internal Note-to-Patch
example is present. A terminal Note stops at exact `event_end` without reading
the seven-byte track tail. Note internal termination is **YES for the supported
family set and authenticated state rule**.

# Channel Pressure continuation and exits

The Track 9 run `0x01478c..0x0147ce` contains one explicit `d0` entry and all
31 possible continuation transitions in this population. Every continuation
uses a one-byte timing VLQ followed by one pressure byte. Values span 0..79;
none has its high bit set.

The final continuation ends at `0x0147ce`. The next timing `83 56` is followed
by explicit `90`, outside the continuation value domain, and independently
correlates to the following Note. The run end is therefore mechanically
derivable from active Pressure state plus the post-VLQ byte class. The existing
caller's run-bound dependency is **RESOLVED BY WALKER** for this supported
grammar.

# Pitch Bend continuation and exits

The nine authenticated runs contain 93 continuations. Each is:

```text
timing VLQ | LSB | MSB
```

Ninety-one continuation timings are one byte and two are two bytes. All 186
value bytes are below `80` (observed maximum `7e`). Each of the nine exits is a
high-bit branch after the next timing VLQ: eight `90` Note entries and one
`ff 41 05` Controller. All agree with the independently correlated run bounds.
Pitch Bend run ends are therefore mechanically derivable, and the existing
caller's run-bound dependency is **RESOLVED BY WALKER** for this supported
grammar.

# Tagged families

Established `ff` branches reachable from mixed performance state are:

- `ff 41 05`: ordinary Controller, exact width determined by the fixed payload
  length;
- `ff 7c | length | payload`: Patch entry, observed after Controller in Track
  9;
- `ff 60 | one-byte length | payload | final timing VLQ | 90`: a bounded
  context branch into Note, observed in Patch extended transitions and at the
  Track 9 Controller-to-Note transition.

`ff 60` is not counted as an independent List event or performance family. Its
semantic purpose remains unknown. In every active Note/Pressure/Bend state,
`ff` is outside the first continuation-data-byte domain and therefore
unambiguously exits continuation state. Unknown `ff` tags must be rejected, not
searched past or guessed.

# Explicit status families

Only these explicit status-bearing entries occur in the authenticated Track 9
and Track 14 regions:

- `90`: Note entry/re-entry;
- `d0`: Channel Pressure entry;
- `e0`: Pitch Bend entry/re-entry.

No other byte in `80..ef` appears at the first post-VLQ dispatch position.
These are observed project discriminators, not literal channel-bearing export
statuses.

# Candidate rule tests

- **Candidate A — CONFIRMED for the authenticated population.** Under active
  Note/Pressure/Bend state, first post-VLQ byte `< 80` continues that family;
  byte `>= 80` exits it.
- **Candidate B — CONFIRMED.** The active family supplies the exact continuation
  grammar: Note has three properties plus duration VLQ, Pressure one data byte,
  Bend two data bytes.
- **Candidate C — CONFIRMED.** `ff` selects a strict tagged/context branch;
  `80..ef` selects an explicit observed status branch; `00..7f` selects active
  state continuation.
- **Candidate D — REFUTED for the supported corpus.** No lookahead beyond the
  current representation, run-bound table, backtracking, or musical
  plausibility is needed. Active family state itself remains required.

This is an evidence-bounded grammar, not a claim about unknown families,
arbitrary status bytes, other Studio Vision versions, or malformed values.

# Cursor classification matrix

| Active state | First post-VLQ class | Interpretation | Required bytes / next cursor | Collision status |
|---|---|---|---|---|
| None | `00..7f` | unsupported | none; stop with error | no family identity |
| Note | `00..7f` | Note continuation | 3 properties + bounded duration VLQ | no observed first-byte collision |
| Pressure | `00..7f` | Pressure continuation | 1 data byte | no observed collision |
| Bend | `00..7f` | Bend continuation | 2 data bytes | no observed collision |
| any/None | `90` | explicit Note entry | status + Note properties; enter Note | observed exact |
| any/None | `d0` | explicit Pressure entry | status + 1 value; enter Pressure | observed exact |
| any/None | `e0` | explicit Bend entry | status + 2 values; enter Bend | observed exact |
| any/None | `ff 41 05` | Controller | exact fixed remainder; clear compact state | observed exact |
| any/None | `ff 7c` | Patch | declared Patch/transition grammar | observed exact |
| any/None | `ff 60` | context-mediated Note entry | declared context, final VLQ, `90`, properties | observed exact; not a standalone family |
| any | other high-bit/tag | unsupported | stop at current cursor | unknown form |
| any | cursor equals `event_end` | terminate track events | consume nothing | exact outer contract |

# Track-end behavior

The walker compares `cursor` with `event_end` before attempting a timing VLQ.
Terminal Note chains in Ode and Track 9, the Controller-ending Track 14, and
all zero-count tracks agree. Empty tracks begin with `cursor == event_end`.
The following `ff aa bb cc ff 2f 00` structure is validated by the container
layer and is never dispatched as an event family.

# Bells Track 9 full-walk validation

A conceptual cursor walk parses all 184 independently established List events:
31 Notes, one Patch, 120 Controllers, and 32 Channel Pressure events. It begins
at `0x0143c8`, uses the established Patch-to-first-Note grammar, handles both
direct and `ff 60` context-mediated Note entry, derives the Pressure entry and
all 31 continuations, exits on `90`, and stops exactly at `0x014957`.

The resulting counts and known ranges match the provenance-controlled SMF and
prior correlation. It requires no supplied Pressure run bound, scanning,
backtracking, or property plausibility. Track 9 full-walk status: **YES for the
supported family set**.

# Bells Track 14 full-walk validation

The same rule parses all 601 independently established events: 227 Notes, 272
Controllers, and 102 Pitch Bends. It derives all nine `e0` entries, all 93
continuations, eight Bend-to-Note exits, and the final Bend-to-Controller exit,
then stops at `0x015ed4` after the last Controller.

All nine former caller-supplied Pitch Bend run boundaries emerge at the exact
independently correlated offsets. Track 14 full-walk status: **YES for the
supported family set**.

# Decoder implications

The bounded Channel Pressure and Pitch Bend run decoders remain valid exact-
bound components. A future mixed walker can derive a run end transactionally
and supply that exact range, or a later implementation may factor shared entry
and continuation parsing without weakening provenance or no-scanning behavior.
No refactor is authorized by this research task.

General internal Note-chain termination becomes **YES for the supported family
set** because the first post-VLQ high-bit branch cannot be consumed as a Note
continuation. Unknown branches remain deterministic errors.

# Mixed-walker implementation gate

**YES.** Evidence supports designing a first caller-bounded mixed-event walker
for the established 166-byte profile with:

- exact container-supplied `event_start..event_end`;
- Note continuations and explicit `90` entries;
- ordinary `ff 41 05` Controllers;
- established direct/one-`ff 60` Patch-to-Note transitions;
- Channel Pressure `d0` entry and one-byte continuations;
- Pitch Bend `e0` entry and two-byte continuations;
- strict high-bit state exit and exact stop at `event_end`.

Explicit exclusions are unknown `ff` tags/status bytes, SysEx, Poly Pressure,
other event families, repeated/other Patch context forms, the older 120-byte
profile, malformed out-of-domain data, discovery/scanning, and MIDI emission.

After that bounded walker exists, the highest-value converter target is MIDI
writer/export integration for the already covered families. The two known
SysEx events remain lower-volume unsupported evidence and should not broaden
the first walker.

# Evidence supported

- 785 independently grounded event-to-next/termination transitions;
- 356 collision-free data continuations across three active families;
- 393 strict `ff` branches and 34 explicit-status entries;
- mechanically reproduced 184-event Track 9 and 601-event Track 14 walks;
- dynamic derivation of all Pressure and Bend run ends;
- exact track termination before the seven-byte tail.

# Unknowns

- unobserved transition pairs, including Note-to-Pressure and
  Pressure-to-Controller;
- other explicit statuses, `ff` tags, families, context forms, and versions;
- whether project low-nibble zero has any meaning beyond observed framing;
- malformed/high-bit property policy;
- SysEx storage and broader export integration.

These unknowns delimit support; they do not block a strict walker that rejects
anything outside the established grammar.

# Experiment decision

**NO CONTROLLED EXPERIMENT REQUIRED.** The natural population repeatedly
separates continuation data from high-bit exits across Note, Pressure, and Bend
and reproduces both complete mixed tracks. A parameter experiment would test
generality rather than resolve the bounded implementation gate.

# Single recommended next step

Design the first exact-bounded mixed-event walker for only the supported family
set and rejection behavior documented here. Do not implement it or begin SysEx
work in the research checkpoint.
