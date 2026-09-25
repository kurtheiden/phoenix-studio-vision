# Experiment 036 - Track 3 Mute On to Off

Status: **PREREGISTERED; NOT EXECUTED**. Date: 2026-09-25.
Baseline: `main`, `f48744400ffa4e7c50f7614474d1815dc41ec146`.
This task inspected existing control/reference artifacts read-only. No new
experiment, new-artifact analysis, implementation, or commit was performed.

## Purpose and reused control

Replicate Experiment 035 on a second track: change only Ode to Clarke / Track 3
Mute ON -> OFF. Track 3 is not Track 3 #2. Test both the provisional saved-state
candidate and the causal MIDI inclusion response under the same conditions.

Reuse these existing files in the actual single directory component containing
a literal colon:

`/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 035 - Track 1 Mute On to Off:Returned from MacOS9/`

| Role | Filename | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| Source/control project | EXP35 EDIT | 211205 | `7b6d507a745441727312f93d36e891221cc06a051467c5f9d9855529c033fb94` |
| Control MIDI | EXP35 EDIT MID | 11368 | `1568e33978c022e5a4a2165a9c782038f3807aa073d16a40f135b4ad73ba32d9` |

Both identities were reverified directly. The owner-established saved/reopened
state is Track 1 OFF, Track 3 ON, all other musical tracks OFF. The control MIDI
was exported contemporaneously with Multitrack checked; read-only inspection
confirms Format 1, PPQN 480, conductor plus eight musical tracks, Track 1 present,
and Track 3 absent. Existing screenshots/provenance remain part of Experiment 035.

**Reuse is valid. Only EXP36 EDIT and EXP36 EDIT MID are required.** No concrete
uncontrolled variable requiring another control export has been identified.
Preserve the same Studio Vision/SheepShaver/OMS environment and export settings.
If these have changed, the opened source state differs, or a routing/remapping
prompt requires intervention, stop and record that specific discrepancy before
editing. Do not automatically create a new control merely for symmetry.

## Locked source-byte prediction

The read-only root walk gives 527 records. Ode's type-0x01 record is ordinal 467,
`0x02ee68..0x02f63b`, with 11 descriptors under the established 208-byte-preamble /
166-byte-descriptor layout. Track 3 is descriptor ordinal 7 (musical pair ordinal
5); its descriptor slice is `0x02f3c2..0x02f468` and label starts at `0x02f3d1`.

| Observation / prediction | Source absolute offset | Source -> predicted EDIT |
| --- | --- | --- |
| Track 3 mute candidate | `0x02f3aa` | `88 -> 80` |
| Track 3 assignment candidate | `0x02f3b2` | `04 -> 04` |
| Track 1 mute candidate | `0x02f06c` | `80 -> 80` |

Derivation: `0x02f3d1 - 39 = 0x02f3aa`, also Track 3 descriptor start minus 24,
or previous descriptor slice (ordinal 6, starting `0x02f31c`) plus `0x8e`.
The assignment candidate is previous-slice `+0x96`, eight bytes later.
These are structural anchors, not proof that the containing slice owns the field.

Source context beginning `0x02f39e`:

```text
00 00 00 00 00 00 2e 00 01 59 00 00 88 00 04 00 00 04 01 00 04 ff ff ff ff ff ff ff
```

Predict clearing only bit `0x08` in this candidate byte. The other eight Ode
mute candidates remain `80`; all assignment candidates and musical event data
remain unchanged. Save-generated metadata may change. The new EDIT's absolute
offset is `0x02f3aa` only if the layout does not relocate: otherwise locate the
same bounded sequence/track and compare label-minus-39. Do not compare the
same absolute offset blindly or subtract unexplained differences as save noise.

## Locked MIDI prediction

CONTROL musical order: Track 1, Track 2, sys100loops, Track 4, Track 5, Track 6,
Track 3 #2, Track 7. A conductor named Ode to Clarke precedes these eight tracks.

Predict EDIT Format 1, PPQN 480, **10 total MTrk chunks: conductor plus nine
musical tracks**, in this order:

Track 1, Track 2, sys100loops, Track 4, Track 5, **Track 3**, Track 6,
Track 3 #2, Track 7.

Track 1 remains present. The conductor and all eight previously included
musical-track payloads remain byte-identical to EXP35 EDIT MID. Newly included
Track 3 must exactly match its authenticated reference MTrk payload: 765 bytes,
SHA-256 `e53433613bfbac3770c0621f1570d43958fcced7e5364657401d98f48003b4bb`.

Reference: `/Users/kurtheiden/Documents/Phoenix Research/Studio Vision MIDI Exports/Project 001/Ode to Clarke Multi All`,
12141 bytes, SHA-256
`4f63b34ef92204d4bc5eeb78dbbe7b94d005c1f9ceb57ea0f9809533ad590f29`.
The reference identity and Track 3 payload were reverified read-only.

## Minimum owner procedure in SheepShaver

1. In Finder, duplicate the saved EXP35 EDIT project represented by the verified
   returned control above, preserving its Mac file metadata. Name the duplicate
   **EXP36 EDIT** and open that duplicate in Studio Vision. Preserve EXP35 EDIT;
   do not start from EXP35 CTRL, EXP33, or an unsaved source duplicate. If the
   target filename already exists, stop rather than overwrite it.
2. Select **Ode to Clarke**. Verify Track 1 Mute OFF, Track 3 Mute ON, and the
   other musical tracks Mute OFF. Preserve sequence selection thereafter.
3. Click only the track-level **M** button on the row named **Track 3** once,
   changing ON -> OFF. Do not click Track 3 #2 or an Instrument-level control.
4. Use the normal Save command on EXP36 EDIT. Close, reopen EXP36 EDIT, and
   visually verify Track 3 OFF, Track 1 OFF, and all other musical tracks OFF.
   Capture the reopened Tracks state for provenance.
5. Export the complete Ode to Clarke sequence as **EXP36 EDIT MID**, keeping
   **Multitrack checked** and all other export options unchanged. Capture the
   export dialog contemporaneously. Do not export a selected-track subset.
6. Close without further saves. Return the two files and screenshots through
   the established metadata-preserving transfer workflow. Proposed destination:
   `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 036 - Track 3 Mute On to Off/Returned from MacOS9/`.
   Preserve any extension Studio Vision creates and record actual returned paths.
   **Stop there for read-only analysis; do not make further edits or exports.**

Keep Track 1 and all other mute settings, solo/enable, events, timing, Instrument
assignment, MIDI channel/routing, Patch/controllers, sequence selection, export
mode/options, and all other project settings fixed. Do not repair OMS, accept
remapping, edit Instrument definitions, or compensate for unexpected behavior.
If a required state cannot be retained, stop and preserve the discrepancy.
If saved Track 3 does not reopen OFF, retain the result and stop before export.
The owner need not inspect bytes or use Terminal.

## Classification and boundaries

Classify MIDI inclusion and candidate behavior separately. Exact MIDI success
requires only Track 3 added with the specified payload and all nine retained
chunks unchanged. Candidate support requires the structurally aligned `88 -> 80`
transition with unaffected track/assignment/event controls. An interpretable
contrary result fails the relevant prediction; a state/environment mismatch or
unsafe structural alignment makes that comparison inconclusive. Preserve all
results; do not silently repair or waive a prediction.

A matching second-track result strengthens a bounded `80/88` saved-state
interpretation under these conditions. It does not establish all flag values,
a universal `88 means omit` rule, exact-profile replacement, general routing,
or automatic Partial-to-Ready promotion. Implementation remains a separate
owner-reviewed decision. No experiment is performed by this preregistration.

Related: [Experiment 035 evidence and control provenance](CONTROLLED_TRACK1_MUTE_ON_TO_OFF.md)
and [Ode structural research](ODE_TO_CLARKE_CHANNEL_CORRELATION.md).

## Post-experiment findings — 2026-09-25

**Completed: MIDI prediction PASS; source-byte prediction PASS after structural
relocation.** Everything above this heading is the original preregistration,
preserved verbatim. Its historical “NOT EXECUTED” status is not the current
outcome. The owner reports changing only Track 3 Mute ON -> OFF, saving,
closing/reopening, verifying Track 1 and Track 3 OFF, then exporting with
Multitrack checked. These UI facts are owner provenance, distinct from the
following directly measured file evidence. No new experiment was performed by
this analysis.

### Actual paths and identities

The actual returned directory has a literal colon in a single component:

`/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 036 - Track 3 Mute On to Off:Returned from MacOS9/`

The source/control pair remains at the Experiment 035 directory recorded above.
No files or paths were renamed or normalized.

| Artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| EXP35 EDIT | 211205 | `7b6d507a745441727312f93d36e891221cc06a051467c5f9d9855529c033fb94` |
| EXP35 EDIT MID | 11368 | `1568e33978c022e5a4a2165a9c782038f3807aa073d16a40f135b4ad73ba32d9` |
| EXP36 EDIT | 209903 | `f90ad9087b674eb17c0f8764ae479df5e98d1dbc111af1067109c43e04baccd1` |
| EXP36 EDIT MID | 12141 | `4f63b34ef92204d4bc5eeb78dbbe7b94d005c1f9ceb57ea0f9809533ad590f29` |

Both new artifacts differ from their respective control artifacts.

### Exact MIDI result

EXP36 EDIT MID is valid Format 1, PPQN 480, ten MTrk chunks: conductor plus
nine musical tracks. Bound chunk lengths, event parsing/running status, and
terminal End-of-Track events were checked. Musical order is exactly:

Track 1, Track 2, sys100loops, Track 4, Track 5, Track 3, Track 6,
Track 3 #2, Track 7.

The conductor and all eight previously included musical-track payloads are
byte-identical to EXP35 EDIT MID. Track 3 is newly present; Track 1 remains
present. Track 3's 765-byte payload matches the preregistered reference hash
`e53433613bfbac3770c0621f1570d43958fcced7e5364657401d98f48003b4bb`.
The entire MIDI is byte-identical to authenticated Ode to Clarke Multi All.
The increase is exactly 773 bytes (765-byte payload plus eight-byte chunk header).
**MIDI prediction: PASS, with no discrepancy or waived gate.**

### Byte prediction, relocation, and other changes

Both projects have 527 root records in the same type order. Only record 97,
type `0x1b`, changes length: CONTROL `0x000fc1..0x0018e7` (2342 bytes), EDIT
`0x000fc1..0x0013d1` (1040 bytes). It shortens by 1302 bytes (`0x516`), and all
later records relocate by `-0x516` with unchanged lengths. The eight-byte root
header is unchanged. Raw same-offset comparison has 181161 unequal overlapping
positions in 11052 runs, plus 1302 CONTROL-only tail positions. These raw counts
include relocation effects; they are not a count of independent semantic edits.
Ordinal-aligned record comparison has 1034 unequal overlapping positions in
452 runs, plus the 1302-byte positional excess within record 97. This excess
is a comparison convention, not a claim of a deletion localized to the tail.

Ode's record 467 moves from `0x02ee68..0x02f63b` to `0x02e952..0x02f125`.
Track 3 remains descriptor ordinal 7 / musical pair ordinal 5; its label moves
from `0x02f3d1` to `0x02eebb`. Label-minus-39 gives:

| Field | CONTROL offset/value | EDIT offset/value |
| --- | --- | --- |
| Track 3 mute candidate | `0x02f3aa: 88` | `0x02ee94: 80` |
| Track 3 assignment candidate | `0x02f3b2: 04` | `0x02ee9c: 04` |
| Track 1 mute candidate | `0x02f06c: 80` | `0x02eb56: 80` |

The candidate is still previous descriptor slice `+0x8e`, not the current
Track 3 slice `+0x8e`; assignment is previous-slice `+0x96`. The local contexts
begin at CONTROL `0x02f39e` / EDIT `0x02ee88`:

```text
CONTROL 00 00 00 00 00 00 2e 00 01 59 00 00 88 00 04 00 00 04 01 00 04 ff ff ff ff ff ff ff
EDIT    00 00 00 00 00 00 2e 00 01 59 00 00 80 00 04 00 00 04 01 00 04 ff ff ff ff ff ff ff
```

The other eight Ode candidate values stay 80. All assignment candidates across
all sequence descriptor rows are unchanged. All 85 type-0x10 records are
byte-identical after alignment. All 168 primary records change only within
record-relative `+0x10..+0x13`; event bytes from `+0x13` onward are unchanged.
All 168 secondary records, 27 preludes, 18 name records, and 18 terminal records
are unchanged. Therefore musical-event stability and assignment-candidate
stability pass independently of the MIDI comparison.

Other differences affect twelve type-0x2a records, type-0x1b ancillary data,
one type-0x2f record, repeated sequence/descriptor metadata, and primary prefix
metadata. The repeated metadata positions also changed in Experiment 035 and
prior no-edit saves. Their exact semantics are not decoded, and the ancillary
rewrites must not be asserted to be mute fields or intentional routing edits.
**Source-byte prediction: PASS at its preregistered structural position.**

### Relationship to Experiment 035 and scope

Experiment 035 changed Ode Track 1's candidate 88 -> 80 while only Track 1 was
newly included; Experiment 036 repeats that response on Track 3 with Track 1
remaining unmuted/included. Both added payloads exactly match the authenticated
reference, and retained payloads and source musical events remain unchanged.
This supplies the requested second-track causal replication; the earlier
“second-track replication still required” limitation is now satisfied.

The measured transition clears bit 08 in one byte, with other bits fixed at 80.
Only whole-byte values 80/88 have been causally tested. No arbitrary-flags rule,
historical export procedure, Instrument-level mute rule, exact-profile policy
replacement, or automatic readiness promotion follows. The separate
[structural-scope decision](MUTE_INCLUSION_STRUCTURAL_SCOPE.md) inventories all
133 baseline descriptor slots and recommends only a narrow saved-state subset,
with inclusion/export authority remaining separately gated.

The analysis made no implementation change or commit. This evidence checkpoint
preserves the original preregistration above; authentic artifacts and unrelated
worktree changes are preserved.
