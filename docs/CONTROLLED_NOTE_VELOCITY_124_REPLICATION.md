# Objective

This report analyzes Experiment 015, an independent velocity-124 replication
on the established C1 drum note. The experiment tests whether Experiment 011's
`24 2d` isolated pair and `24 7f` dense pair reproduce, or whether the dense
candidate extends the direct-value sequence established at velocities 127,
126, and 125. The analysis is read-only and does not claim a complete MIDI
event record.

# Experimental provenance

Experiment 015 was created from a fresh native Finder duplicate of the
verified-working Experiment 007 baseline. The user changed only the same C1
note's velocity from 127 to 124 in Studio Vision's List Window, saved on quit,
reopened and verified the project as functional, quit without saving, and
Finder-copied it through `Unix` into the research directory.

Directory inspection positively identified the uncompressed project as
`/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 015 - Velocity 124 Replication/newest STUFF baseline EXP15`.
It is 211,468 bytes and has SHA-256
`f6caecc0d1826edd2f322b28e2681b6455ad745a0f01a018d6348cbfc76aa09e`.
Finder Type is `MID2` and Creator is `MIDA`. The observed extended attributes
are `com.apple.FinderInfo` (32 bytes), `org.BasiliskII.FinderInfo` (16 bytes),
and `org.BasiliskII.ExtendedFinderInfo` (16 bytes). No other attribute or
resource fork was observed.

# Preregistered results

The decisive locations were inspected before the broad comparison:

| Location | Experiment 011 | Experiment 015 | Replication result |
|---|---|---|---|
| `0x0002f76f–0x0002f770` | `24 2d` | `24 0f` | Experiment 011 state did not replicate |
| `0x00031c1f–0x00031c20` | `24 7f` | `24 7c` | Experiment 011 value did not replicate; direct sequence extended exactly |

Both pitch bytes retain the predicted `0x24`. At the isolated candidate,
Experiment 015 takes baseline state `0f`, not the preregistered replication
value `2d`. At the dense candidate, it takes direct velocity value `7c`, not
Experiment 011's `7f`. This resolves the direct competition in favor of the
direct-value prediction for this independent save, while showing that neither
of Experiment 011's two candidate states reproduced.

# Pitch-anchor verification

The 81 established anchors were reconstructed from positions where Experiment
010 is `0x23`, Experiment 007 is `0x24`, and Experiment 009 is `0x25`.
Experiment 015 contains `0x24` at every position.

| Experiment 015 value | Count |
|---|---:|
| `0x24` | 81 |
| `0x23` | 0 |
| `0x25` | 0 |
| other | 0 |

The pitch-bearing representation remains stable without exception.

# Independent velocity-124 comparison

Experiments 011 and 015 both contain 211,468 bytes. They differ at 1,243
same-position bytes in 423 disjoint runs. The first difference is
`0x00000fdc`, the last is `0x0003380a`, the maximum unequal run is 13 bytes,
the common prefix is 4,060 bytes, and the common suffix is 513 bytes. Their
known labels and broad structure remain aligned.

Relative to Experiment 007, Experiments 011 and 015 share the same non-baseline
value at 589 positions. None of those 589 positions remains baseline-valued in
all rename and pitch controls, so none is isolated as a control-stable
velocity-124 signature. Sixty-six positions differ from baseline only in
Experiment 011 when the other replication is baseline-valued; 12 show the
opposite relationship. Other same-position differences comprise recurring or
differently valued save-output variation.

Most importantly, neither primary Experiment-011 candidate reproduces:
`0x0002f770` changes from Experiment 011's `2d` to `0f`, and
`0x00031c20` changes from `7f` to direct value `7c`. No other previously
reported Experiment-011 velocity-correlated change survives control filtering
as a replicated same-velocity change.

# Direct-value discontinuity

| Velocity | Experiment | `0x0002f770` | `0x00031c20` |
|---:|---|---:|---:|
| 127 | 007 | `0f` | `7f` |
| 126 | 013 | `0f` | `7e` |
| 125 | 014 | `0f` | `7d` |
| 124 | 011 | `2d` | `7f` |
| 124 | 015 | `0f` | `7c` |
| 121 | 012 | `2d` | `7f` |

Experiment 015 exactly extends the independently predicted fixed-position
direct sequence to `7c`. Experiment 011's return to `7f` is not reproduced and
should be treated as run-specific or dependent on an unresolved structural
difference rather than as reproducible velocity-124 behavior.

Experiment 016 subsequently repeated velocity 121 and produced direct `0x79`
with isolated state `0f`; neither Experiment 012 candidate reproduced. The
five tested direct values now match velocities 127, 126, 125, 124, and 121.
See `CONTROLLED_NOTE_VELOCITY_121_REPLICATION.md`.

The isolated `2d` state also fails to reproduce at velocity 124. Consequently,
the earlier boundary interpretation between 125 and 124 is weakened: the
tested value 124 now has conflicting `2d` and `0f` observations. The state is
velocity-correlated in Experiments 011 and 012 but is not determined by
velocity magnitude alone under the tested procedures.

# Direct velocity-124 search

Experiment 015 contains nine bytes equal to `0x7c` at positions that differ
from Experiment 007. Eight occur at previously variable or recurring
save-output positions. Only `0x00031c20` is baseline-different while remaining
baseline-valued in the rename-only and pitch-only controls.

Within the dense pitch region, `0x00031c20` is also the only byte that differs
from Experiment 007. It immediately follows the confirmed pitch anchor at
`0x00031c1f`, giving the exact pair `24 7c`; the surrounding local bytes remain
stable. Experiment 011 contains no equivalent control-stable introduced
`0x7c` anywhere. Numeric identity does not by itself prove event ownership,
but the preregistered exact value, stable controls, pitch adjacency, and
four-point sequence make this the strongest direct velocity candidate so far.

# Compact-region comparison

At `0x0002f6f2–0x0002f6fd`, the observed bytes are:

| Artifact | Twelve bytes |
|---|---|
| 007 and 011 | `ff ff 80 00 00 14 00 c8 00 c8 00 00` |
| 013 | `fe ff 00 97 00 74 02 8b 03 0b 00 23` |
| 014 | `fe ff 00 97 00 74 01 d5 01 dd 00 15` |
| 015 | `00 ff 00 97 00 74 01 d5 01 dd 00 15` |

Experiment 015 forms a fourth pattern. It matches Experiment 014 at ten of the
twelve bytes and differs in the first two-byte component (`00 ff` versus
`fe ff`). It does not reproduce Experiment 011's baseline pattern, and no
supported ordered relationship to velocity follows from these patterns. The
region remains unresolved.

# Control filtering

Experiment 007 and 015 have equal 211,468-byte data forks. They differ at
1,766 same-position bytes in 632 runs. The first difference is `0x0000001e`,
the last is `0x00033a06`, the maximum run is 36 bytes, the common prefix is 30
bytes, and the common suffix is five bytes. `Track 7` remains at
`0x0002f6ca`, `Ode to Clarke` at `0x0002f753`, and the established label
cadence remains aligned.

All 1,766 differing offsets had already varied in at least one of Experiments
005, 006, or 008–014; Experiment 015 introduces no wholly new differing
offset. This does not make every value recurring. The direct `0x7c` value at
`0x00031c20` is uniquely supported by its ordered series and controls. The
589 exact non-baseline values shared with Experiment 011 are recurring in at
least one non-velocity control, leaving zero control-stable same-velocity
changes shared by the two velocity-124 saves.

The evidence separates into:

- reproducible same-velocity candidate changes: none after control filtering;
- recurring or previously variable offsets: all 1,766 baseline differences;
- direct velocity candidate: `0x00031c20`, newly `7c` as predicted;
- Experiment-015-only offsets relative to all prior artifacts: zero;
- unresolved values: the compact-region fourth pattern and the broader
  save-output population.

# Evidence supported

- Experiment 015 contains `24 0f` at the isolated pair; Experiment 011's
  velocity-124 `24 2d` result did not replicate.
- Experiment 015 contains exactly `24 7c` at the dense pair; Experiment 011's
  `24 7f` result did not replicate.
- The fixed dense candidate now follows direct values `7f`, `7e`, `7d`, `7c`
  across independent velocity 127, 126, 125, and replicated 124 artifacts.
- `0x00031c20` is the only control-stable introduced `0x7c` in Experiment 015.
- All 81 established pitch-bearing positions remain `0x24`.
- No control-stable non-baseline change is shared by the two independent
  velocity-124 artifacts.
- The Experiment-011 discontinuity is not reproducible in Experiment 015 and
  should be treated as run-specific or structurally contingent.
- The old fixed-state boundary between velocities 125 and 124 is not
  reproducible and cannot be attributed to velocity alone.
- These results strengthen a direct pitch-adjacent numeric velocity candidate
  but do not establish a complete MIDI event record.

# Unknowns

- The structural or save-run reason Experiment 011 contains `2d` and `7f`
  instead of Experiment 015's `0f` and `7c` is unknown.
- Whether Experiment 012's velocity-121 `2d` and `7f` values will reproduce is
  unknown.
- The meaning of `0x0002f770` and its relationship, if any, to the direct
  candidate remain unknown.
- The compact-region patterns and broader save-output variation remain
  unresolved.
- Event ownership, timing, duration, channel, note-on/note-off semantics, and
  complete framing have not been established.
- No valid Standard MIDI File has yet been recovered.

# Single recommended next step

Experiment 016 completed the velocity-121 replication and produced direct
`0x79`; see `CONTROLLED_NOTE_VELOCITY_121_REPLICATION.md`. The next controlled
experiment should change only the same note's duration by an exact recorded
amount while retaining C1 pitch and velocity 127, to begin locating another
event-related numeric field around the stable pitch/velocity pair.
