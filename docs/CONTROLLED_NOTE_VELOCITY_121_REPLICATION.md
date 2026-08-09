# Objective

This report analyzes Experiment 016, an independent replication of the
velocity-121 edit on the established C1 drum note. It tests whether Experiment
012's anomalous `24 2d` isolated pair and `24 7f` dense pair reproduce, or
whether the pitch-adjacent direct-value candidate takes the preregistered value
`0x79`. The analysis is read-only and does not claim a complete MIDI event
record.

# Experimental provenance

Experiment 016 was created from a fresh native Finder duplicate of the
verified-working Experiment 007 baseline. In Studio Vision's List Window, the
user changed only the same C1 note's velocity from 127 to 121, saved on quit,
reopened and verified the project as functional, quit without saving, and
Finder-copied it through `Unix` into the research directory.

Directory inspection positively identified the uncompressed project as
`/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 016 - Velocity 121 Replication/newest STUFF baseline EXP16`.
It is 211,468 bytes and has SHA-256
`4698392d3361fc82f19156c91d43caf6ef1eb7e704c6616f34bf2b3e25966beb`.
Finder Type is `MID2` and Creator is `MIDA`. Its extended attributes are
`com.apple.FinderInfo` (32 bytes), `org.BasiliskII.FinderInfo` (16 bytes), and
`org.BasiliskII.ExtendedFinderInfo` (16 bytes). No other attribute or resource
fork was observed.

# Preregistered results

The decisive locations were inspected before the broad comparison:

| Location | Experiment 012 | Experiment 016 | Replication result |
|---|---|---|---|
| `0x0002f76f–0x0002f770` | `24 2d` | `24 0f` | Experiment 012 state did not replicate |
| `0x00031c1f–0x00031c20` | `24 7f` | `24 79` | Experiment 012 value did not replicate; direct prediction succeeded exactly |

Both pitch bytes retain the preregistered `0x24`. Experiment 016 returns to
baseline `0f` at the isolated position and contains the exact decimal-121
value `0x79` immediately after the first dense pitch anchor.

# Pitch-anchor verification

The 81 anchors were reconstructed from positions where Experiment 010 is
`0x23`, Experiment 007 is `0x24`, and Experiment 009 is `0x25`. Experiment 016
contains `0x24` at all 81 positions.

| Experiment 016 value | Count |
|---|---:|
| `0x24` | 81 |
| `0x23` | 0 |
| `0x25` | 0 |
| other | 0 |

The established pitch-bearing representation remains stable without
exception.

# Independent velocity-121 comparison

Experiments 012 and 016 both contain 211,468 bytes. They differ at 1,181
same-position bytes in 418 disjoint runs. The first difference is
`0x00000fdb`, the last is `0x0003380a`, the maximum unequal run is 13 bytes,
the common prefix is 4,059 bytes, and the common suffix is 513 bytes. Their
known labels and broad structures remain aligned.

Relative to Experiment 007, the two velocity-121 artifacts share the same
non-baseline value at 638 positions. None remains baseline-valued in all rename
and pitch controls, so none is isolated as a control-stable reproduced
velocity-121 signature. Fifty-eight positions differ from baseline only in
Experiment 012 when Experiment 016 is baseline-valued; 14 show the opposite
relationship. Other differences are recurring or differently valued
save-output variation.

Neither decisive Experiment-012 value reproduces: `0x0002f770` is `0f` rather
than `2d`, and `0x00031c20` is direct `79` rather than `7f`.

# Direct velocity series

| Velocity | Experiment | `0x00031c20` |
|---:|---|---:|
| 127 | 007 | `7f` |
| 126 | 013 | `7e` |
| 125 | 014 | `7d` |
| 124 | 015 | `7c` |
| 121 | 012 | `7f` |
| 121 | 016 | `79` |

Experiment 016 satisfies the independently preregistered exact prediction
`0x79`. Experiment 012's `0x7f` result therefore does not replicate. The fixed
pitch-adjacent candidate now equals the known velocity at five independently
tested values: 127, 126, 125, 124, and 121. The gap at untested values 123 and
122 does not weaken those exact observations, but behavior there is unknown.

This is strong evidence for a direct numeric velocity representation at
`0x00031c20` in the clean List Window series. Numeric identity and adjacency do
not alone establish complete event ownership or framing.

# Direct velocity-121 search

Experiment 016 introduces two `0x79` bytes relative to Experiment 007. The
first, `0x0002d5ee`, belongs to a recurring serialized numeric sequence already
present in other saved artifacts. Only `0x00031c20` is baseline-different while
remaining baseline-valued in the rename and pitch controls.

Within the dense pitch region, `0x00031c20` is the only byte that differs from
Experiment 007. It immediately follows confirmed pitch byte `0x00031c1f`,
forming exact pair `24 79`, with the local surrounding bytes stable.
Experiment 012 has no control-stable introduced `0x79` candidate anywhere.

# Experiment 012 non-reproduced differences

The user's suspicion that graphical velocity interaction affected Experiment
012 is a hypothesis, not an established cause. The byte evidence supports only
a narrower statement: Experiment 012 contains non-reproduced differences
consistent with an uncontrolled additional edit or run-specific structural
variation.

Of the 58 positions that differ from baseline only in Experiment 012 when
Experiment 016 is baseline-valued, 48 cluster within
`0x0002ef8c–0x0002f01f`. Two more occur at `0x0002f20c–0x0002f20d`, and one is
the isolated candidate at `0x0002f770`; seven are isolated earlier in the
file. The main cluster comprises compact unequal runs up to 13 bytes. Its
localization makes it relevant as non-reproduced structure, but no semantic or
UI-action attribution is supported.

# Compact-region comparison

At `0x0002f6f2–0x0002f6fd`, the observed patterns are:

| Artifact | Twelve bytes |
|---|---|
| 007 and 012 | `ff ff 80 00 00 14 00 c8 00 c8 00 00` |
| 013 | `fe ff 00 97 00 74 02 8b 03 0b 00 23` |
| 014 | `fe ff 00 97 00 74 01 d5 01 dd 00 15` |
| 015 and 016 | `00 ff 00 97 00 74 01 d5 01 dd 00 15` |

Experiment 016 exactly matches Experiment 015, follows the later clean series
rather than Experiment 012, and differs from baseline. No semantic or ordered
velocity interpretation is assigned to this unresolved region.

# Control filtering

Experiments 007 and 016 have equal 211,468-byte data forks. They differ at
1,761 same-position bytes in 632 runs. The first difference is `0x0000001e`,
the last is `0x00033a06`, the maximum unequal run is 36 bytes, the common
prefix is 30 bytes, and the common suffix is five bytes. `Track 7` remains at
`0x0002f6ca`, `Ode to Clarke` at `0x0002f753`, and the established label
cadence remains aligned.

All 1,761 differing offsets had previously varied in Experiments 005, 006, or
008–015; Experiment 016 introduces no wholly new differing offset. The
classification is:

- recurring or previously variable offsets: all 1,761 baseline differences;
- direct velocity-121 candidate: `0x00031c20`, exact `7f -> 79`;
- exact non-baseline values shared with Experiment 012: 638, all already
  variable in a rename or pitch control;
- control-stable same-velocity changes shared specifically with Experiment
  012: zero;
- Experiment-016-only offsets relative to every prior artifact: zero;
- unresolved evidence: differing values within the recurring population and
  the compact-region pattern.

# Evidence supported

- Experiment 016 contains `24 0f` at the isolated pair; Experiment 012's
  `24 2d` did not replicate.
- Experiment 016 contains exactly `24 79` at the dense pair; the preregistered
  direct-value prediction succeeded and Experiment 012's `24 7f` did not
  replicate.
- All 81 established pitch-bearing positions remain `0x24`.
- `0x00031c20` is the only control-stable introduced `0x79` in Experiment 016.
- The fixed candidate now equals velocity at five independently tested values:
  127, 126, 125, 124, and 121.
- No control-stable non-baseline change is shared by the two independent
  velocity-121 artifacts.
- Experiment 012 contains 58 non-reproduced baseline differences, including a
  48-position cluster, consistent with an uncontrolled additional edit or
  run-specific structural variation. Their cause is not established.
- Experiment 012 should now be treated as anomalous and non-reproduced for both
  decisive candidates.
- The evidence strongly supports a direct pitch-adjacent numeric velocity
  representation but not a complete MIDI event record.

# Unknowns

- The cause of Experiment 012's non-reproduced values is unknown; graphical UI
  contamination is not established.
- The semantic relationship, if any, of isolated byte `0x0002f770` to the
  pitch/velocity candidate remains unknown.
- The compact-region patterns and broader save-output variation remain
  unresolved.
- Values 123 and 122 have not been tested in the direct sequence.
- Event ownership, timing, duration, channel, note-on/note-off semantics, and
  complete framing have not been established.
- No valid Standard MIDI File has yet been recovered.

# Single recommended next step

From a fresh native Finder duplicate of Experiment 007, make one controlled
duration-only edit to the same C1, velocity-127 note in the List Window, using
an exact recorded before/after duration while leaving pitch, velocity, timing,
and note count unchanged. Preregister `24 7f` at the established pair and all
81 pitch anchors as stable, then search for compact, control-stable numeric
changes near that pair. This advances from confirmed pitch and velocity values
toward the remaining structure needed for event framing.
