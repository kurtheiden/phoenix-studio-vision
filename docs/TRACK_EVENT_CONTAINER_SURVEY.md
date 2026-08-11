# Objective

This read-only survey compares Track 7's validated event chain with nearby
record-like structures. It seeks repeated container relationships without
implementing discovery or treating plausible bytes as proven fields.

# Evidence boundary

The inspected artifact is the positively identified uncompressed Experiment
007 project, `newest STUFF baseline` (211,468 bytes). Candidate scans required
multiple consecutive mechanically decodable structures with pitch/velocity and
VLQ plausibility. They are discovery candidates, not identified tracks.

# Track 7 surrounding region

Track 7's visible property chain begins at `0x31c0c`; the established row-4
timing anchor is `0x31c1d`, and the validated row-4–143 walk returns at
`0x31f9c`. Immediately before the row-1 properties are:

`... 03 a9 00 01 00 00 00 8f 00 00 00 00 2c c4 b2 f0 82 f1 30 90 | 26 7f 7f 4c ...`

The `0x8f` value is numerically 143, but its semantics are not assigned. After
the final event (`81 70 | 24 7f 7f 76`) the bytes begin
`ff fa b9 2f ff 2f 00 29 00 00 00 69 00 06 ...`.

# Prior track-record evidence

The earlier 120-byte survey identifies named metadata records at
`0x26860–0x26e77`, including a `Track 7` label at relative record offset
`0x0f` in the record beginning `0x26d10`. Its arithmetic candidate readings
were explicitly non-proven and do not point to `0x31c0c` or `0x31f9c`.
No previously documented reference has a structurally established target in
the event chain.

# Candidate event chains

Five additional bounded candidates met the conservative multi-structure
threshold:

| Candidate | Range | Structures | Marker/header context |
|---:|---|---:|---|
| 1 | `0x2fb7a–0x300d8` | 210 | marker/header near `0x2fb51` |
| 2 | `0x301d8–0x30976` | 320 | marker/header near `0x301c6` |
| 3 | `0x30a32–0x30e98` | 178 | marker/header near `0x30a26` |
| 4 | `0x30f4c–0x31254` | 133 | marker/header near `0x30f40` |
| 5 | `0x31677–0x317fe` | 59 | marker/header near `0x3166a` |

The external CSV records exact contexts and stop information. Their identities
are unknown; no UI track adjacency is inferred.

# Candidate boundary comparison

All five candidates stop at the first conservative structural failure in their
sequential walk. Their post-contexts share the broad shape `ff fa ?? ?? ff 2f
00 29 00 00 00 ?? 00 06 ...`, analogous to Track 7's post-chain bytes. This is
repeated byte-level boundary evidence, not an SMF end marker interpretation.
Initial timing prefixes are ambiguous in each candidate because metadata and
status-like bytes precede the first mechanically clean structure.

# Pre-chain context

Each candidate is preceded nearby by `2c c4 b2` and a short variable prefix.
Track 7 has this marker at `0x31c04`; other marker offsets are `0x2fb51`,
`0x301c6`, `0x30a26`, `0x30f40`, and `0x3166a`. This recurring relationship is
stronger than the older unconstrained pointer coincidences, but the marker's
semantics remain unknown.

# Post-chain context

The repeated post-context is the clearest cross-candidate similarity. Exact
examples are listed in the CSV. No conclusion is drawn about padding, footer,
terminator, next record, or event type.

# Event-count analysis

Track 7's nearby `00 00 00 8f` equals 143 and coincides with the visible row
count. Other nearby records contain values such as `0x142`, `0xb3`, `0x86`,
`0x3c`, and `0xd4`; these are near their candidate structure lengths/counts
but do not form a validated common count convention. No credible event-count
field is established.

# Length analysis

Track 7's row-1-property to end-cursor span is `0x31f9c - 0x31c0c = 0x38f`
(911 bytes). The strict row-4–143 candidate span is `0x37f` (895 bytes).
Candidate spans are 0x55e, 0x79e, 0x466, 0x308, and 0x187 bytes. Nearby
header values are correlated but not yet a proven payload-length field.

# Reference analysis

The prior reference survey found only arithmetic in-range coincidences and no
structurally supported pointer to Track 7's chain start or end. No new absolute
or relative reference is established here.

# Track metadata association

The known `Track 7` label is in the earlier 120-byte metadata table, but no
validated link connects it to `0x31c0c`. Candidate chains therefore remain
unidentified despite the repeated marker and post-context.

In that metadata table, the label order around `Track 7` is `Track 3 #2`,
`Track 7`, then `Track 12`. This is UI/metadata ordering evidence only; it does
not establish binary adjacency or identify either neighboring event chain.

# Structural model candidates

**Strongly supported:** several record-like regions share a `2c c4 b2`
pre-context and an `ff fa ?? ?? ff 2f 00 29 ...` post-context; Track 7 is one
member of that broader family. **Plausible:** these bytes delimit or accompany
event-data records. **Unknown:** field widths, count/length semantics, record
ownership, track identity, and whether all candidate structures are note data.

# Can Track 7 be found without a hard-coded offset?

**PARTIAL.** Repeated local marker/post-context relationships can identify a
family of candidate records, but no validated metadata reference or Track 7
identity selects `0x31c0c` among them. Automatic discovery should not yet be
implemented.

# Evidence supported

- Five additional credible multi-structure candidates were found.
- Their boundaries and post-contexts resemble Track 7's at byte level.
- A recurring `2c c4 b2` marker occurs near candidate starts.
- The Track 7 `0x8f`/143 coincidence is interesting but not a proven count.
- Existing parser code remains unchanged.

# Unknowns

Track identity, marker meaning, count/length fields, initial timing ownership,
post-chain structure semantics, complete record framing, and links from labels
to event data remain unresolved.

# Single recommended next step

Obtain Studio Vision List Window ground truth for one strategically selected
neighboring candidate (prefer candidate 3, `0x30a32–0x30e98`) and compare it
under strict sequential alignment before implementing any discovery logic.

# Track 3 #2 ground-truth follow-up

Independent `Ode to Clarke` / `Track 3 #2` / `JD-800` ground truth later
tested all five candidates under fixed 17-row alignment. Each failed on all
four properties of row 1; none produced a complete property-row match or a
timing match. Their identities remain unknown.

A preregistered multi-event search then found exact 17-row hits at `0x313fa`
and `0x31994`, in two marker-framed regions omitted by this survey. The first
region is identified as Track 3 #2 by the independently expected count
relationship: `00 00 00 55` occurs eight bytes before its `2c c4 b2` marker,
analogous to Track 7's `00 00 00 8f` at the same relative position. The second
region has `00 00 00 56` and remains unidentified despite sharing the exact
17-row prefix.

The earlier heuristic missed Track 3 #2 because it required a long walk from a
mechanically clean timing/property start. Track 3 #2 has ambiguous initial
timing/header material and yields 84 consecutive note-property structures
against the nearby/UI count of 85. This follow-up strengthens the repeated
record-family and count-field evidence without identifying the original five
candidates or assigning them to any of the project's 18 Sequences.
