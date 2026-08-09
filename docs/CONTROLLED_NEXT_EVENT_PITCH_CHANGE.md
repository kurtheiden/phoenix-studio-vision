# Objective

This report analyzes Experiment 021, a controlled pitch-only edit of the fifth
Studio Vision List Window event from D1 to C#1. It tests whether the confirmed
property encoding for the fourth event generalizes to the immediately
following event. The work is read-only evidence gathering and does not claim a
complete event parser or framing model.

# Experimental provenance

Experiment 007 is the known-good baseline. Before Experiment 021, Studio
Vision's List Window independently showed the fifth event at position
`26·1·469`, pitch D1, attack velocity 127, release velocity 86, and duration
245. Experiment 021 was created from a fresh native Finder duplicate of the
baseline. The user changed only that fifth event's pitch from D1 to C#1, saved
on quit, reopened and verified the project as functional, quit without saving,
and Finder-copied it through `Unix` into the research directory.

Directory inspection positively identified the uncompressed project as
`/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 021 - Next Event Pitch Down One/newest STUFF baseline EXP21`.
It is 211,524 bytes and has SHA-256
`5aa2c305d502afafcd56071740170e64d496f89948313e1f9c03b93e015a7da2`.
Finder Type is `MID2`, Creator is `MIDA`, and the observed attributes are
`com.apple.FinderInfo` (32 bytes), `org.BasiliskII.FinderInfo` (16 bytes), and
`org.BasiliskII.ExtendedFinderInfo` (16 bytes). No other attribute or resource
fork was observed.

Experiment 021 is 56 bytes larger than Experiment 007. Labels through
`Ode to Clarke` retain their baseline offsets, while the event stream containing
the controlled structures appears 56 bytes later. This layout change is
reported explicitly rather than treating fixed same-position bytes as aligned.

# Preregistered result

The baseline fifth-event candidate `26 7f 56 81 75` occurs at `0x00031c26`.
In Experiment 021, its structurally corresponding sequence occurs at
`0x00031c5e` and is exactly:

`25 7f 56 81 75`

The complete preregistered property prediction succeeds:

- pitch changes exactly `0x26 -> 0x25`, D1 to C#1 under the established
  mapping;
- attack velocity remains `0x7f`, decimal 127;
- release velocity remains `0x56`, decimal 86;
- duration remains `81 75`, mechanically decoding as VLQ 245.

At the old fixed offset `0x00031c26`, Experiment 021 does not contain this
structure because of the 56-byte serialization shift. The controlled result is
therefore a structure-aligned success, not a same-absolute-offset comparison.

# Fifth-event binary location

Experiment 021 locations are:

| Field | Offset(s) | Bytes |
|---|---|---|
| preceding timing candidate | `0x00031c5c–0x00031c5d` | `83 60` |
| pitch | `0x00031c5e` | `25` |
| attack velocity | `0x00031c5f` | `7f` |
| release velocity | `0x00031c60` | `56` |
| duration | `0x00031c61–0x00031c62` | `81 75` |

The pitch is two bytes after the start of the preceding VLQ. The fourth-event
pitch is at `0x00031c57`, so the fifth-event pitch remains seven bytes later,
the same relative distance as baseline offsets `0x00031c1f` and
`0x00031c26`.

The local sequences are:

- baseline: `81 65 | 24 7f 5c 83 3a | 83 60 | 26 7f 56 81 75`;
- Experiment 021: `81 65 | 24 7f 5c 83 3a | 83 60 | 25 7f 56 81 75`.

# Preceding timing control

The VLQ immediately preceding the fifth properties remains exactly `83 60`,
mechanically decoding as 480. The preregistered timing-stability prediction
succeeds. A fifth-event pitch-only edit therefore does not alter the adjacent
position-correlated candidate.

Current evidence best describes `83 60` as a timing interval immediately
preceding the fifth property structure: it directly precedes the independently
identified fifth event, and Experiments 019/020 varied it compensatingly when
the fourth event moved. Exact ownership and MIDI delta-time semantics remain
provisional because changing fifth-event pitch does not itself test timing
ownership.

# Fourth-event control

The established fourth-event properties remain exactly
`24 7f 5c 83 3a` at Experiment 021 offsets
`0x00031c57–0x00031c5b`. The preceding timing candidate also remains
`81 65` = 229. Changing the fifth event's pitch does not affect any confirmed
fourth-event musical-property byte.

# Consecutive property structures

| Event | SVP position | SVP pitch | SVP attack | SVP release | SVP duration | Binary properties |
|---|---|---|---:|---:|---:|---|
| fourth, baseline/control | `25·4·469` | C1 | 127 | 92 | 442 | `24 7f 5c 83 3a` |
| fifth, baseline | `26·1·469` | D1 | 127 | 86 | 245 | `26 7f 56 81 75` |
| fifth, Experiment 021 | `26·1·469` | C#1 | 127 | 86 | 245 | `25 7f 56 81 75` |

Strong controlled evidence now supports the fifth-event pitch byte: it changes
exactly and alone within the property sequence. The fifth event's attack
velocity, release velocity, and duration remain stable during that pitch edit
and match the independently displayed numeric values, but those three fields
have not yet been independently manipulated on the fifth event. Their support
is structural and numeric, not separate controlled manipulation.

# Additional candidate events

Starting at baseline `0x00031c24`, a bounded mechanical scan can provisionally
segment the fifth event and seven following note-like structures using a VLQ,
three single-byte property candidates, and a variable-width VLQ. Representative
sequences include:

- `83 60 | 26 7f 56 81 75`;
- `81 70 | 24 7f 60 6b`;
- `83 60 | 26 79 7d 47`;
- `81 70 | 26 7f 3e 81 56`.

Only the fifth-event row has documented Studio Vision values available in the
provided evidence, and it matches all four displayed properties mechanically.
No displayed values for the seven subsequent rows are available in the
research artifacts inspected here, so zero structures beyond the fifth can be
matched to List Window values without invention.

Consequently, Experiment 021 adds one independently documented consecutive
note structure beyond the previously controlled fourth event. The additional
seven structures remain provisional binary segmentations, not verified SVP
row mappings.

# Control filtering

At baseline alignment, the fifth pitch byte is `0x26` in Experiments 009, 010,
and 013–020; those unrelated edits leave this note unchanged. Experiment 021's
structurally aligned byte is exactly `0x25`.

After applying the observed +56 event-stream alignment, Experiment 007 and 021
differ at 149 tail positions in 50 runs. Of those, 148 positions had already
varied in an unrelated controlled artifact. The only control-stable aligned
change is:

- baseline `0x00031c26: 26` -> Experiment 021 `0x00031c5e: 25`.

No other new control-stable musical-data offset remains unexplained. The
56-byte size/layout expansion is a separate unresolved serialization change
and is not attributed to the pitch edit's field representation.

# Whole-file comparison

Experiment 007 is 211,468 bytes; Experiment 021 is 211,524 bytes, a difference
of 56 bytes. A raw same-position comparison over the baseline length reports
16,078 unequal bytes in 1,889 runs, first difference `0x0000001e`, last
`0x00033a0b`, maximum unequal run 246 bytes, common prefix 30 bytes, and common
suffix five bytes. These counts are dominated by the layout shift and should
not be interpreted as 16,078 independent field changes.

The known labels remain aligned: `Track 7` still occurs at `0x0002f6ca`, and
`Ode to Clarke` at `0x0002f753`. A baseline 32-byte chunk at `0x0002f740`
remains at the same offset, while chunks by baseline `0x0002f788` and the event
stream occur 56 bytes later. The precise boundary and meaning of the added
serialization are unresolved.

Evidence classification is:

- recurring or previously variable aligned event-tail differences: 148;
- fifth-event pitch candidate: one exact structure-aligned byte;
- 56-byte serialization/layout expansion: unresolved and kept separate;
- other new control-stable musical-data differences: zero.

# Generalization evidence

Experiment 021 demonstrates that the property encoding established for the
fourth event generalizes to the immediately following fifth event:

- the fifth pitch changes exactly `26 -> 25` for D1 to C#1;
- attack `7f`, release `56`, and duration `81 75` remain stable;
- preceding timing candidate `83 60` remains stable;
- the fifth baseline values independently match Studio Vision's displayed
  pitch, attack velocity, release velocity, and duration.

The pitch field has direct controlled support on both consecutive notes. The
other fifth-event properties have independent UI correspondence and stability
during the pitch edit, but not yet property-specific edits on that event. This
is sufficient to demonstrate structural generalization beyond one note without
claiming that every note or track uses identical framing.

# Evidence supported

- Experiment 021's fifth property sequence is exactly `25 7f 56 81 75`.
- The complete `26 7f 56 81 75 -> 25 7f 56 81 75` prediction succeeds under
  the observed +56 structural alignment.
- Preceding timing candidate `83 60` remains unchanged.
- Fourth-event properties `24 7f 5c 83 3a` remain unchanged.
- Only the fifth pitch byte survives aligned control filtering; no other new
  control-stable musical-data offset remains unexplained.
- One additional consecutive structure—the fifth event—matches documented SVP
  List Window values; zero later structures can be matched because their
  displayed rows are not documented in the available evidence.
- The property encoding demonstrably generalizes from the fourth event to the
  immediately following fifth event.
- The file's 56-byte expansion remains unresolved and prevents naive
  same-absolute-offset comparison.

# Unknowns

- The cause and exact boundary of the 56-byte serialization expansion are
  unknown.
- Timing-field ownership and whether `83 60` is an SMF-like delta remain
  provisional.
- Fifth-event attack velocity, release velocity, and duration have not been
  independently edited, despite exact numeric correspondence.
- Displayed SVP values for later provisional structures are unavailable.
- Complete event boundaries, channel/status encoding, track framing, and
  absolute-time reconstruction remain unknown.
- No valid Standard MIDI File has yet been recovered.

# Single recommended next step

From a fresh native Finder duplicate of Experiment 007, move only the fifth
event from `26·1·469` to `26·1·470`, retaining D1 pitch, attack velocity 127,
release velocity 86, duration 245, and note count. Preregister the fourth-event
timing and properties as stable; test whether the fifth-event leading interval
`83 60` becomes `83 61` and whether the following interval `81 70` becomes
`81 6f`. This is the highest-information next experiment because it directly
tests timing ownership between two now independently identified consecutive
property structures and extends the delta-style chain.
