# Objective

This report analyzes Experiment 009, a verified-working controlled Studio
Vision edit in which the user changed one drum note in `Track 7` of
`Ode to Clarke` from C1 to C#1 while leaving timing, duration, velocity, and
note count unchanged. It is a read-only evidence comparison against the
verified-working untouched Experiment 007 baseline. It does not modify an
artifact, implement parser logic, or assign binary-field semantics.

# Experimental provenance

Experiment 007 is the untouched baseline derived from the authentic
`newest STUFF` project. Its native Mac OS 9 save opened and functioned normally
in Studio Vision, and it remained usable after a Finder-copy round trip through
SheepShaver's `Unix` shared volume.

Experiment 009 was derived from a native Finder duplicate of the same
known-good baseline. The user changed exactly one drum-note pitch in `Track 7`
from C1 to C#1. The project was saved by quitting Studio Vision and choosing
Save, reopened and verified to function normally, quit without saving, and
Finder-copied through `Unix` into the research directory. This provenance is
distinct from `Save As...` and from saving directly to `Unix`.

Experiment 007's StuffIt archive was excluded. Experiment 008, the
verified-working rename-only native control, and the earlier edited Experiments
005 and 006 were used only to identify recurring save-output variation.

# Artifact inventory

Both directories were inspected before selecting the files. Experiment 007
contained an uncompressed project and a StuffIt archive; only the uncompressed
project was used. Experiment 009 contained one file, whose observed basename
is retained below.

| Artifact | Exact path | Basename | Data-fork size | Data-fork SHA-256 |
|---|---|---|---:|---|
| Experiment 007 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline` | `newest STUFF baseline` | 211,468 (`0x33a0c`) | `e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132` |
| Experiment 009 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 009 - Single MIDI Note Pitch Change/newest STUFF baseline copy` | `newest STUFF baseline copy` | 211,468 (`0x33a0c`) | `2b08f822c65bf21b1eeda8f509e6b9c162414f210bb2fbf037650a3541c8ce87` |

Both artifacts have Finder Type `MID2`, Finder Creator `MIDA`, a 32-byte
`com.apple.FinderInfo`, a 16-byte `org.BasiliskII.FinderInfo`, and a 16-byte
`org.BasiliskII.ExtendedFinderInfo`. Neither has a resource fork, and no other
extended attributes were observed. Their `com.apple.FinderInfo` values and
zero-filled extended FinderInfo values are identical. Their BasiliskII
FinderInfo values differ: Experiment 007 has
`4d4944324d4944410100007b01080000`, while Experiment 009 has
`4d4944324d4944410100ffffffff0000`.

Relevant printable labels are aligned at identical offsets:

| Label | Experiment 007 | Experiment 009 |
|---|---:|---:|
| `Meter Track` | `0x0002f04e` | `0x0002f04e` |
| `Tempo Track` | `0x0002f0f4` | `0x0002f0f4` |
| `Track 1` | `0x0002f19a` | `0x0002f19a` |
| `Track 2` | `0x0002f240` | `0x0002f240` |
| `sys100loops` | `0x0002f2e6` | `0x0002f2e6` |
| `Track 4` | `0x0002f38c` | `0x0002f38c` |
| `Track 5` | `0x0002f432` | `0x0002f432` |
| `Track 3` | `0x0002f4d8` | `0x0002f4d8` |
| `Track 6` | `0x0002f57e` | `0x0002f57e` |
| `Track 3 #2` | `0x0002f624` | `0x0002f624` |
| `Track 7` | `0x0002f6ca` | `0x0002f6ca` |
| `Ode to Clarke` | `0x0002f753` | `0x0002f753` |

Successive track-label offsets retain the established 166-byte (`0xa6`)
cadence.

# Whole-file comparison

The data forks have identical sizes but are not byte-identical. At the same
absolute offsets, 1,885 of 211,468 bytes differ, approximately 0.89%, in 733
disjoint runs. The first difference is `0x0000001e`, the last is
`0x00033a06`, and the maximum unequal-run length is 36 bytes. The common prefix
is 30 bytes and the common suffix is five bytes.

The size, relevant label locations, observed cadence, and `Ode to Clarke`
location remain aligned. The differences therefore occur within a shared broad
serialization layout. No insertion or deletion is inferred.

Comparison with prior controls divides the 1,885 positions as follows:

- 1,112 positions have the same changed value in Experiment 008, showing
  directly reproduced variation in the two verified-working native
  save-on-quit outputs. Their presence in a pitch edit demonstrates that they
  are not rename-specific.
- Another 103 positions occur at offsets that vary in at least one of
  Experiments 005, 006, or 008, but Experiment 009 does not reproduce the
  Experiment 008 value. These are recurring locations with run-varying values.
- 670 changed positions were baseline-valued in all three prior controls and
  are newly observed in Experiment 009. Of these, 81 form the pitch-correlated
  `24` to `25` population described below. The remaining 589 are unresolved;
  prominent patterns include 194 `01` to `02` changes and 169 `ff` to `00`
  changes, plus structured initialization-like runs. Their form does not
  isolate them as musical-edit consequences.

# Track 7 aligned-structure comparison

`Track 7` begins at `0x0002f6ca` in both files. The preceding label cadence and
the established label position at relative `0x0f` support an aligned 166-byte
comparison structure from `0x0002f6bb` through `0x0002f760`, inclusive. This is
a cadence-supported comparison boundary, not a proven semantic record.

All eight differences in that structure are:

| Absolute offset | Relative offset | Experiment 007 | Experiment 009 | Control evidence |
|---:|---:|---:|---:|---|
| `0x0002f709` | `0x4e` | `c4` | `db` | recurring run-varying position; 005 also has `db` |
| `0x0002f70a` | `0x4f` | `b2` | `03` | recurring position; distinct prior edited values |
| `0x0002f70b` | `0x50` | `60` | `98` | recurring position; distinct prior edited values |
| `0x0002f715` | `0x5a` | `c4` | `db` | recurring run-varying position; 005 also has `db` |
| `0x0002f716` | `0x5b` | `b2` | `8b` | recurring position; distinct prior edited values |
| `0x0002f717` | `0x5c` | `a0` | `94` | recurring position; distinct prior edited values |
| `0x0002f73c` | `0x81` | `00` | `ff` | unique to Experiment 009; unresolved |
| `0x0002f73d` | `0x82` | `00` | `ff` | unique to Experiment 009; unresolved |

No byte in this aligned structure changes by one, and no direct C1/C#1
note-number hypothesis is observed there. The first six offsets are the same
run-varying locations seen in the rename-only comparison. The final two are
new but have no demonstrated numeric relationship to the pitch edit.

# Pitch-correlated candidate search

The strongest finding is 81 positions where Experiment 007 contains `24`,
Experiment 009 contains `25`, and Experiments 005, 006, and 008 all retain
`24`. These are hexadecimal bytes: `0x24` is decimal 36 and `0x25` is decimal
37. Under the common MIDI naming convention that calls MIDI note 36 C1, the
transition is exactly the hypothesized C1 to C#1 one-semitone increase. Note
names vary between software conventions; under a convention that calls MIDI
note 24 C1, the direct-byte hypothesis would instead be `18` to `19`, and no
such changed position was observed. No assumption that Studio Vision stores
standard MIDI note numbers directly is required for the correlation.

Eighty of the 81 positions form a dense candidate region:

| Observation | Result |
|---|---|
| bounded range | `0x00031c1f–0x00031f98` |
| span | 890 bytes |
| `24` to `25` positions | 80 |
| baseline `24` bytes in `0x00031c00–0x00031f9f` | 81 |
| baseline `24` bytes unchanged | one, at `0x00031f0e` |
| common immediate prefix | 70 of 80 are preceded by `81 70`; four by `83 60` |
| gaps between candidate positions | 5–27 bytes; most commonly 6, 13, 14, 12, and 20 |
| human-readable labels | none observed |

The region contains repeated local binary forms, including many instances of
`81 70 24` becoming `81 70 25`, amid byte-diverse data and short incidental
printable runs. The 80 single-byte candidates are otherwise isolated changes;
the only three additional differences in the wider
`0x00031c00–0x00031f9f` window are the recurring run-varying bytes at
`0x00031c05–0x00031c07`.

The remaining unique `24` to `25` change is at `0x0002f76f`, nine bytes beyond
the cadence-supported Track 7 structure and 28 bytes after the `Ode to Clarke`
label begins. It lies in a compact, non-printable region. The same local window
also changes `01` to `02` at `0x0002f777`; that second transition is unique to
Experiment 009 but is a common global save-output pattern and is not isolated
as pitch-related.

One additional whole-file `24` to `25` change occurs at `0x00006ab9`, but
Experiment 008 independently has `25` there. It is therefore classified as
prior save-run variation, not pitch-specific evidence. No other direct
36-to-37 transition survives the control subtraction.

The concentration, exact one-unit relationship, stable alignment, absence from
all three controls, and repeated local forms make the 81-position population
strong pitch-correlated evidence. The evidence does not establish whether any
one byte is the edited note, whether the 80-byte population represents events,
dependent serialized values, a drum mapping, or another consequence of the
edit.

# Comparison against rename-only control

Experiment 008 changed only the printable track label intentionally, while
Experiment 009 retained `Track 7` and changed musical content. The
Experiment-007/008 comparison found 1,208 unequal bytes; Experiment 009 exactly
reproduces Experiment 008's changed value at 1,112 positions. This large shared
population is strong negative evidence against treating those positions as
rename or pitch fields and supports a repeatable save-on-quit serialization
component.

Within the Track 7 aligned structure, both controls vary at relative
`0x4e–0x50` and `0x5a–0x5c`, but their values differ. The rename-only edit also
changes the printable byte at relative `0x15`; Experiment 009 correctly retains
`37` (`7`) there. Experiment 008 has none of the 81 control-filtered
`24` to `25` changes.

The comparison classes used here are:

- **Directly reproduced save-output variation:** 1,112 positions where
  Experiments 008 and 009 share a non-baseline value.
- **Recurring, run-specific locations:** 103 additional Experiment 009
  differences at offsets already variable in 005, 006, or 008.
- **Pitch-correlated candidates:** the 81 control-filtered `24` to `25`
  positions.
- **Unresolved new differences:** the other 589 positions absent from all
  three prior control comparisons.

These are empirical classifications. They do not assign field meaning or
prove that all members of a class have one cause.

# Candidate event-bearing regions

The earlier SMF recovery spike found no direct embedded SMF payload or
multi-event representation. It isolated the `Ode to Clarke` label region as
metadata-correlated and retained a weak `90 52 68` cluster in the original
serialization. The later track-structure surveys established repeated label
cadence and candidate numeric windows but did not establish a link to events.

Experiment 009 supplies two new bounded candidates in the saved-file
serialization:

1. **Strong pitch-correlated binary region,
   `0x00031c1f–0x00031f98`.** Eighty control-filtered bytes reproduce the exact
   hypothesized decimal 36-to-37 change, 70 in a repeated `81 70 XX` local
   form. The region has no recognizable metadata label. This is strong
   evidence that the region responds systematically to the pitch edit. It is
   not yet sufficient to call the bytes MIDI events because one intentional
   note edit produced 80 changed occurrences and no time, duration, velocity,
   channel, or event framing has been established.
2. **Compact post-structure candidate around `0x0002f76f`.** One additional
   control-filtered `24` to `25` byte lies immediately beyond the aligned
   Track 7 structure in a non-printable window. Its proximity to
   `Ode to Clarke` and exact numeric relationship make it relevant, but the
   single occurrence and nearby global-pattern `01` to `02` change leave its
   role unresolved.

The changed Track 7 label structure itself is not a pitch candidate: none of
its eight differences has a one-unit relationship or direct note-number
pattern, and six occupy known run-varying locations. The prior weak original
`90 52 68` region does not emerge as a uniquely supported pitch region in this
saved-file comparison.

# Evidence supported

- Experiments 007 and 009 are verified-working, same-size files with aligned
  broad structure and unchanged relevant labels.
- The comparison contains a control-filtered population of 81 exact
  `0x24` to `0x25` changes, numerically decimal 36 to 37.
- Eighty of those changes are concentrated within 890 bytes, and 70 share the
  immediate prefix `81 70`.
- Under the explicitly stated MIDI naming convention where C1 is note 36, the
  transition exactly matches C1 to C#1. Under the C1-is-24 convention, the
  corresponding direct-byte change was not found.
- Experiments 005, 006, and 008 retain the baseline `0x24` value at all 81
  pitch-correlated positions.
- Experiment 009 reproduces 1,112 Experiment 008 non-baseline values, providing
  strong control evidence for common save-on-quit output variation unrelated
  to the distinct intentional edits.
- The 81-position population is strong pitch-correlated evidence, but it does
  not by itself establish Studio Vision event encoding or identify a Standard
  MIDI File payload.

# Unknowns

- It is unknown why one intentional note edit correlates with 80 changes in the
  dense candidate region plus one nearby metadata-region change.
- It is unknown whether the candidate bytes are direct pitch values, repeated
  dependent values, a mapping, cached data, or another representation.
- No event boundary, time, duration, velocity, channel, track ownership, or
  note-on/note-off relationship has been established.
- The meaning of the isolated `0x0002f76f` candidate is unknown.
- The causes and meanings of the 589 other newly observed changes are unknown.
- Repeated save-on-quit variation is observed, but its mechanism and complete
  deterministic rules remain unknown.
- The label cadence supports structural alignment but not semantic record
  boundaries.
- No valid Standard MIDI File has yet been recovered.

# Single recommended next step

Repeat the same verified-working native save-on-quit procedure from a fresh
Finder duplicate of Experiment 007, changing the same drum note from C1 to D1
with timing, duration, velocity, and note count unchanged. Test whether the 81
identified positions change from baseline `0x24` to `0x26`, while the 1,112
reproduced save-output positions remain in their control pattern. A matching
two-semitone response would sharply strengthen the pitch-field hypothesis and
distinguish direct pitch-like values from a coincidental one-step population.
