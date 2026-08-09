# Objective

This report analyzes Experiment 011, a verified-working controlled Studio
Vision edit in which the velocity of the same C1 drum note used in Experiments
009 and 010 changed from 127 to 124. Pitch remained C1; timing, duration, and
note count were unchanged. The work is read-only evidence gathering. It does
not modify an artifact, implement parser logic, or assign event semantics.

# Experimental provenance

Experiment 007 is the untouched verified-working native-save baseline derived
from the authentic `newest STUFF` project. Experiment 009 changed the selected
note's pitch from C1 to C#1, and Experiment 010 changed the same note from C1
to B0. Those experiments established 81 bidirectionally responsive
pitch-bearing positions.

Experiment 011 was created from a fresh native Finder duplicate of the same
known-good baseline. The user retained the note at C1 and changed only velocity
from 127 to 124. Studio Vision saved the edit when the user quit and selected
Save; the result reopened and functioned normally. It was then quit without
saving and Finder-copied through SheepShaver's `Unix` shared volume into the
research folder. The Experiment 007 StuffIt archive was excluded.

# Artifact inventory

All four directories were inspected before selecting the uncompressed project
files. Experiment 007 contained both the project and a StuffIt archive; each
other directory contained one project. Experiment 011's reported basename was
confirmed exactly.

| Artifact | Exact path | Basename | Data-fork size | Data-fork SHA-256 |
|---|---|---|---:|---|
| Experiment 007 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline` | `newest STUFF baseline` | 211,468 (`0x33a0c`) | `e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132` |
| Experiment 009 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 009 - Single MIDI Note Pitch Change/newest STUFF baseline copy` | `newest STUFF baseline copy` | 211,468 (`0x33a0c`) | `2b08f822c65bf21b1eeda8f509e6b9c162414f210bb2fbf037650a3541c8ce87` |
| Experiment 010 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 010 - Single MIDI Note Down One Semitone/newest STUFF baseline EXP10` | `newest STUFF baseline EXP10` | 211,468 (`0x33a0c`) | `146d4a9da37aed1c1d6085ead4af9c57ad6d51e4e021001ae442f5d1fc9d2a42` |
| Experiment 011 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 011 - Single MIDI Note Velocity Change/newest STUFF baseline EXP11` | `newest STUFF baseline EXP11` | 211,468 (`0x33a0c`) | `8f9a18f629a58a4eede289b181fae6ba1b61ef2361d28a2eb17cc0248747ccd6` |

All four have Finder Type `MID2`, Finder Creator `MIDA`, a 32-byte
`com.apple.FinderInfo`, a 16-byte `org.BasiliskII.FinderInfo`, and a 16-byte
`org.BasiliskII.ExtendedFinderInfo`. No other extended attributes or resource
forks were observed. Their `com.apple.FinderInfo` values and zero-filled
extended FinderInfo values are identical. Experiment 007 has BasiliskII
FinderInfo `4d4944324d4944410100007b01080000`; Experiments 009, 010, and
011 have `4d4944324d4944410100ffffffff0000`.

The relevant labels are aligned identically in all four files:

| Label | Absolute offset |
|---|---:|
| `Meter Track` | `0x0002f04e` |
| `Tempo Track` | `0x0002f0f4` |
| `Track 1` | `0x0002f19a` |
| `Track 2` | `0x0002f240` |
| `sys100loops` | `0x0002f2e6` |
| `Track 4` | `0x0002f38c` |
| `Track 5` | `0x0002f432` |
| `Track 3` | `0x0002f4d8` |
| `Track 6` | `0x0002f57e` |
| `Track 3 #2` | `0x0002f624` |
| `Track 7` | `0x0002f6ca` |
| `Ode to Clarke` | `0x0002f753` |

The 166-byte (`0xa6`) label cadence is unchanged.

# Whole-file comparisons

All compared data forks have identical sizes and aligned broad structure.
Same-position measurements are:

| Comparison | Unequal bytes | Disjoint runs | First difference | Last difference | Maximum run | Common prefix | Common suffix |
|---|---:|---:|---:|---:|---:|---:|---:|
| Experiment 007 vs 011 | 1,820 | 651 | `0x0000001e` | `0x00033a06` | 36 bytes | 30 bytes | 5 bytes |
| Experiment 009 vs 011 | 1,110 | 488 | `0x00000fdc` | `0x0003380a` | 3 bytes | 4,060 bytes | 513 bytes |
| Experiment 010 vs 011 | 1,254 | 481 | `0x00000fdc` | `0x0003380a` | 5 bytes | 4,060 bytes | 513 bytes |

File sizes, relevant labels, and the observed cadence remain aligned. These
are direct same-position comparisons; no insertion or deletion is inferred.

# Pitch-anchor stability

The established pitch set was reconstructed mechanically as the 81 positions
where Experiment 010 is `0x23`, Experiment 007 is `0x24`, Experiment 009 is
`0x25`, and Experiments 005, 006, and 008 retain `0x24`. Experiment 011 retains
`0x24` at every one of those positions.

| Experiment 011 value at the 81 pitch positions | Count |
|---|---:|
| `0x24` | 81 |
| `0x23` | 0 |
| `0x25` | 0 |
| any other value | 0 |

Thus the preregistered pitch-stability prediction succeeds without exception.
The pitch-bearing representation follows pitch in the bidirectional pitch
experiments and remains baseline-valued when only velocity changes.

# Direct velocity-value search

The direct MIDI-data-byte hypothesis predicts hexadecimal `7f` to `7c` for
decimal velocity 127 to 124. No `7f` to `7c` change occurs anywhere in the
Experiment 007/011 same-position comparison. More generally, no single byte in
the whole-file diff decreases by three.

Mechanical searches for two- and four-byte unsigned integers, at every byte
alignment and in both big- and little-endian order, also found no field that
decreases by exactly three while remaining byte-identical in Experiments 008,
009, and 010. These negative results do not show that velocity is absent; they
show that the tested direct and simple numeric encodings were not observed.

Only one Experiment 011 difference is absent from the complete prior-variation
union of Experiments 005, 006, 008, 009, and 010: at `0x0002f770`, baseline
`0f` becomes `2d`. This position is stable at `0f` in every non-velocity
control. Its difference is `+0x1e` (decimal +30), not the known velocity
difference of decimal -3. It is therefore strong experiment-correlated and
positionally paired evidence, but its numeric encoding remains unresolved.

# Dense pitch-region analysis

The 80 dense pitch-bearing anchors occupy `0x00031c1f–0x00031f98`. Every one
remains `0x24` in Experiment 011. A comparison of the complete wider range
`0x00031c0f–0x00031fa8`, providing at least a 16-byte exterior margin around
the bounded pitch region, finds zero Experiment 007/011 differences.
Consequently, every bounded local window around every dense pitch anchor is
unchanged.

Specifically:

- all 70 `81 70 24` forms remain `81 70 24`;
- all four `83 60 24` forms remain `83 60 24`;
- the other six pitch-bearing positions remain `24`; and
- no recurring relative offset from any dense pitch byte contains a
  velocity-only change.

This is strong negative evidence against a velocity field located within the
inspected dense-region neighborhood, including a direct `7f` to `7c` value or
another simple -3 change. It does not exclude velocity storage elsewhere or a
representation whose bytes do not change under the tested comparison.

The isolated pitch anchor at `0x0002f76f` also remains `24`. The immediately
following byte at `0x0002f770` is the sole fully control-filtered Experiment
011 difference and changes from `0f` to `2d`.

# Pitch/velocity positional relationships

The only strong velocity-correlated candidate is one byte after the isolated
pitch-bearing position:

| Candidate | Nearest pitch anchor | Signed distance | Baseline | Experiment 011 | Controls 008/009/010 |
|---:|---:|---:|---:|---:|---|
| `0x0002f770` | `0x0002f76f` | `+1` byte, after | `0f` | `2d` | `0f` |

Experiments 005 and 006 also retain `0f` at the candidate. The compact local
bytes from `0x0002f761` through `0x0002f77f` are otherwise stable between
Experiments 007 and 011 except at `0x0002f777`, discussed below. Around the
anchor, the observed pairs are:

| Experiment | Bytes at `0x0002f76f–0x0002f770` | Intentional edit |
|---|---|---|
| 007 | `24 0f` | baseline C1, velocity 127 |
| 009 | `25 0f` | C#1, velocity unchanged |
| 010 | `23 0f` | B0, velocity unchanged |
| 011 | `24 2d` | C1, velocity 124 |

This orthogonal response is important: the first byte follows pitch in both
directions and remains fixed during the velocity edit, while the immediately
following byte remains fixed during both pitch edits and changes only during
the velocity edit. It strongly supports a local pitch/velocity pairing at
these two positions. It does not establish that `0f` and `2d` are direct
velocity values, explain their transformation, or prove a complete event
record.

At `0x0002f777`, Experiment 007 is `01`, while Experiments 009, 010, and 011
are `02`; Experiment 008 remains `01`. This position is part of a broad
recurring `01` to `02` save-output population and is not classified as a
velocity candidate. Three following run-varying bytes at
`0x0002f781–0x0002f783` also differ among saved outputs and do not provide a
stable positional relationship. No analogous `+1` velocity-correlated byte is
observed beside any of the 80 dense pitch anchors.

# Save-run controls

Relative to Experiment 007, Experiment 011 has 1,820 unequal positions. Using
Experiments 005, 006, 008, 009, and 010 as controls yields:

| Class | Count | Observation |
|---|---:|---|
| exactly reproduced Experiment 008 value | 191 | common save-on-quit output |
| other previously variable locations | 1,628 | offset varied in at least one prior control, with another observed value pattern |
| established pitch positions | 0 changed; 81 stable | all retain baseline `24` as predicted |
| velocity-correlated candidate | 1 | `0x0002f770`, stable in every non-velocity control |
| new unresolved differences | 0 | no other position survives the complete control union |

Two positions that are stable in Experiments 008, 009, and 010 but change in
Experiment 011 were already variable in Experiment 005 or 006; the complete
control union therefore correctly places them among previously variable
locations. The classification does not imply that all 1,819 recurring
positions have one cause.

# Evidence supported

- All four verified-working artifacts have equal data-fork sizes, aligned
  broad structure, and identical relevant label locations.
- All 81 established pitch-bearing positions remain exactly `0x24` when pitch
  remains C1 and velocity changes.
- The complete dense pitch-bearing region plus exterior margins is unchanged;
  no dense anchor has a nearby velocity-correlated byte in the inspected
  windows.
- No direct `7f` to `7c`, single-byte -3, or control-stable two- or four-byte
  integer -3 change was observed.
- `0x0002f770` is the only Experiment 011 difference absent from all five prior
  controls. It changes `0f` to `2d` immediately after the isolated pitch anchor
  at `0x0002f76f`.
- The two-byte local pair responds orthogonally across experiments:
  `24 0f` baseline, `25 0f` pitch-up, `23 0f` pitch-down, and `24 2d`
  velocity-edited.
- This provides strong velocity-correlated and pitch/velocity positional
  evidence, but only weak evidence about the velocity's numeric encoding.
- The other 1,819 Experiment 011 differences occur at previously variable
  positions and remain classified as save-output or run-specific variation.

# Unknowns

- The transformation, scale, or encoding relating velocity 127/124 to observed
  bytes `0f`/`2d` is unknown.
- It is unknown whether `0x0002f76f–0x0002f770` is a direct field pair, a
  mapping entry, cached state, or another representation.
- The reason the 80 dense pitch-bearing positions have no nearby velocity
  response is unknown.
- No complete event boundary, timing, duration, channel, track ownership, or
  note-on/note-off semantics have been established.
- The mechanism and deterministic extent of recurring save-output differences
  remain unknown.
- One velocity edit does not establish that `0x0002f770` responds monotonically
  or reproducibly to other velocity values.
- No valid Standard MIDI File has yet been recovered.

# Single recommended next step

From a fresh native Finder duplicate of Experiment 007, change only the same
C1 drum note's velocity from 127 to 125, retaining pitch, timing, duration, and
note count. After the verified-working save-on-quit procedure, test the
preregistered predictions that all 81 pitch anchors remain `0x24` and inspect
only `0x0002f770` first. A reproducible, ordered response distinct from both
baseline `0f` and Experiment 011 `2d` would begin to constrain the velocity
encoding while preserving the isolated pitch/velocity positional anchor.
