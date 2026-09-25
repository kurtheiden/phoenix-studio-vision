# Experiment 035 - Track 1 Mute On to Off

Analysis date: 2026-09-25. Status: MIDI prediction passed exactly; credible
mute-state candidate; reusable authoritative inclusion rule not yet approved.
Analysis baseline: `main`, `991f66b667ddf26644d8dfa888ba48dbddc5a3a0`.
The analysis changed no parser/export implementation and performed no new
experiment or export. This record is the documentation-only checkpoint.

## Provenance and intervention

Owner-reported procedure: CONTROL was duplicated from returned/saved Experiment
033 CONTROL. Immediately before the fresh CONTROL MIDI export, Ode to Clarke
had Track 1 and Track 3 Mute ON, all other visible musical tracks Mute OFF,
and Multitrack checked. Only Track 1 Mute was changed ON -> OFF. EDIT was saved,
closed, reopened, visually verified with Track 1 OFF and Track 3 ON, then
exported with Multitrack checked. No intentional musical-event, routing,
Instrument, Patch, timing, solo, or other track-state edit was made.
The owner reports contemporaneous screenshots for both exports. This analysis
uses that supplied provenance; it does not claim independent screenshot review.

The requested nested directory does not exist. The actual directory is one
component containing a literal colon, and was not renamed:

`/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 035 - Track 1 Mute On to Off:Returned from MacOS9/`

All four identities below were measured directly from the data forks.

| Artifact | Bytes | SHA-256 |
| --- | ---: | --- |
| EXP35 CTRL | 211468 | `b1c82a7085f98f054f7320e2800aba523447c3c987c1aa983865518b7a7dd368` |
| EXP35 CTRL MID | 10514 | `9979ed6d5fc58edb85c3c03e5e43b4c7015a353af6075ece8ba94ac49cbf5059` |
| EXP35 EDIT | 211205 | `7b6d507a745441727312f93d36e891221cc06a051467c5f9d9855529c033fb94` |
| EXP35 EDIT MID | 11368 | `1568e33978c022e5a4a2165a9c782038f3807aa073d16a40f135b4ad73ba32d9` |

Both project and MIDI pairs are distinct. EXP35 CTRL equals returned EXP33 CTRL
byte-for-byte. The fresh EXP35 CTRL MID equals returned EXP33 CTRL MID and the
historical Ode to Clarke Multitrack MIDI byte-for-byte. Fresh export provenance
comes from the owner, not from those identical bytes.

## MIDI outcome

Both SMFs have valid header/chunk boundaries, Format 1, PPQN 480, parsed events
with valid running status/data bytes, and exactly one terminal End-of-Track
per chunk. Header track counts agree with the actual chunks.

| Property | CONTROL | EDIT |
| --- | --- | --- |
| Total MTrk chunks | 8 | 9 |
| Conductor name | Ode to Clarke | Ode to Clarke |
| Musical tracks | 7 | 8 |
| Track 1 | absent | present |
| Track 3 | absent | absent |

CONTROL musical order: Track 2, sys100loops, Track 4, Track 5, Track 6,
Track 3 #2, Track 7.

EDIT musical order: Track 1, Track 2, sys100loops, Track 4, Track 5, Track 6,
Track 3 #2, Track 7. Track 3 #2 is distinct from Track 3.

The conductor and all seven retained musical-track payloads are byte-identical
between CONTROL and EDIT. Their payload sizes in that order are 47, 1940, 2876,
1693, 1206, 602, 760, and 1312 bytes. Newly included Track 1 has an 846-byte
payload, byte-identical to Track 1 in the authenticated 12141-byte Ode to Clarke
Multi All reference (SHA-256
`4f63b34ef92204d4bc5eeb78dbbe7b94d005c1f9ceb57ea0f9809533ad590f29`).
Track 1 contains channel-1 PC61, 93 Note On and 89 explicit Note Off messages;
91 positive-velocity Note Ons and two velocity-zero Note On endings. Exact
payload equality preserves timing, data, metadata, and event order as well.
The 854-byte file increase is exactly this payload plus its eight-byte header.

All 168 aligned project primary records have changes only within record-relative
`+0x10..+0x13`; all musical event data beginning at primary-relative `+0x13`
is unchanged, including Track 1. All 168 secondary records are unchanged.
Thus Track 1's existing source events are preserved, and the added MIDI agrees
exactly with its authenticated source-reference performance.

**Primary inclusion prediction: PASSED EXACTLY.** Given the supplied controlled
UI provenance, changing only Track 1 Mute changed inclusion as predicted under
these conditions. No MIDI discrepancy was observed. This is not a statement
about every export mode, solo/Instrument state, or Studio Vision version.

## Exact raw comparison and structural alignment

Offsets and ranges below are hexadecimal and half-open unless stated otherwise.
The exact raw comparison was independently rechecked. There are
**178682 unequal overlapping absolute positions in
12029 contiguous runs**, plus **263 CONTROL-only trailing positions** at
`0x033905..0x033a0c`. Counting absent positions as differences gives 178945
positions. Keeping the absent-byte tail separate gives 12030 entries; the
last unequal-byte run touches that tail, so merging both categories gives
12029 contiguous difference runs overall. The regeneration recipe below
reproduces the individual offsets, values, and surrounding context.
This raw comparison must not be mistaken for 178682 independent semantic edits.

Both complete root walks contain 527 records in identical type order; root
header `002d009300ac00a6` is unchanged. Only record 97 (type `0x1b`) changes
length: CONTROL `0x000fc1..0x0019ee` (2605 bytes), EDIT
`0x000fc1..0x0018e7` (2342 bytes). Every later record relocates by `-0x107`
(263 bytes); their lengths remain unchanged. Name records are unchanged.
The length change exactly accounts for the whole-file shrinkage.

Ordinal-aligned comparison, retaining separate absolute offsets in each file,
has **2351 unequal overlapping record-relative positions
in 584 runs**, plus 263 unmatched CONTROL bytes in record 97's positional tail.
That tail is a comparison convention, not a claim that only the record's end
was deleted. Internal content in the variable-length record was rewritten.

Changed records: twelve type `0x2a`, one type `0x1b`, one type `0x2f`, eighteen
type `0x01`, and 168 type `0x02`. All 85 type `0x10` records, 27 type `0x09`
preludes, 168 type `0x29` secondaries, and 18 terminal records are unchanged.

## Target-associated candidate versus save changes

Ode's type `0x01` record is ordinal 467, CONTROL `0x02ef6f..0x02f742`,
EDIT `0x02ee68..0x02f63b`. Its descriptor count remains 11. The existing
208-byte-preamble/166-byte-descriptor layout and nine ordinal track/pair
associations remain aligned.

The candidate run is one byte at sequence-record-relative `+0x204`:

| | CONTROL | EDIT |
| --- | --- | --- |
| Absolute candidate offset | `0x02f173` | `0x02f06c` |
| Value | `88` | `80` |
| Track 1 descriptor start | `0x02f18b` | `0x02f084` |
| Track 1 label start | `0x02f19a` | `0x02f093` |
| Context start | `0x02f167` | `0x02f060` |

CONTROL context:

```text
00 00 00 00 00 00 2d 00 01 51 80 00 88 00 04 00 00 04 01 00 03 ff ff ff ff ff ff ff
```

EDIT context:

```text
00 00 00 00 00 00 2d 00 01 51 80 00 80 00 04 00 00 04 01 00 03 ff ff ff ff ff ff ff
```

The candidate is **label-minus-39 / target-descriptor-start-minus-24**, or
**previous descriptor slice +0x8e** under the current parser boundaries.
It is not Track 1's own descriptor +0x8e. The observed transition clears bit
`0x08`; a one-byte observation is established, but a complete flags-field
width and meanings of its other bits are not. Do not shift descriptor boundaries
or assign semantic ownership solely from the containing slice.

Target association rests on the controlled Track 1 transition, the other eight
unchanged track-associated candidate values, cross-track UI agreement, and
unchanged event/routing evidence, not merely proximity to a name.

Separate categories of remaining changes:

- **Repeated save-associated metadata:** all 168 primary-record changes are
  confined to `+0x10..+0x13`; repeated descriptor-relative `+0x4e..+0x51`
  and `+0x5a..+0x5d` rewrites, and sequence-relative `+0x48..+0x4b` rewrites,
  have corresponding variation in the Experiment 002 no-edit save, Experiment
  006 Save Copy As, and returned 033/034 saves. This supports save-churn
  classification, not a decoded pointer format or permission to discard bytes.
- **Global/ancillary metadata, exact semantics unresolved:** twelve type `0x2a`
  records change `+0x22..+0x25` (`901530 -> 800f00`); the first also changes
  `+0x16..+0x19` (`6cc724 -> 5cc104`). Type `0x1b` changes length and embedded
  path/name material, including old bundle/Unix and MacOS9HD/OMS references.
  No intentional routing edit is inferred from this serialization change.
- **Other unexplained non-event changes:** type `0x2f` record 108 changes
  relative `+0x0a` (`73 -> 80`) and `+0x0e` (`25 -> 26`). Ode's preamble also
  changes `+0x20..+0x23`, `+0x24..+0x27`, `+0x34`, and `+0x36..+0x3c`.
  The recipe below reproduces their exact values; they are not established mute fields.

## Prior evidence and cross-track test

Identities and exact candidate offsets/values were independently rechecked for
Experiment 007, returned 033/034 CONTROL/EDIT, and both 035 projects, including
all baseline and 035 EDIT sequence candidates. The recipe below reproduces
this inventory; prior artifact identities are also recorded in the linked
Experiment 033/034 records.

| Ode track | 007, 033 C/E, 034 C/E, 035 C | 035 E | Known 035 C -> E mute |
| --- | --- | --- | --- |
| Track 1 | `88` | `80` | ON -> OFF |
| Track 2 | `80` | `80` | OFF -> OFF |
| sys100loops | `80` | `80` | OFF -> OFF |
| Track 4 | `80` | `80` | OFF -> OFF |
| Track 5 | `80` | `80` | OFF -> OFF |
| Track 3 | `88` | `88` | ON -> ON |
| Track 6 | `80` | `80` | OFF -> OFF |
| Track 3 #2 | `80` | `80` | OFF -> OFF |
| Track 7 | `80` | `80` | OFF -> OFF |

Earlier file values are measured; this table does not invent earlier UI states.
The position is stable relative to each label despite 035's relocation. Track 2's
assignment byte at label-minus-31 / previous slice +0x96 changes `05 -> 06`
in 033 and `05 -> 03` in 034, while its mute candidate stays `80`. In 035,
Track 1's assignment byte remains `03` and Track 2's remains `05`. The mute
candidate is eight bytes before the assignment byte, not a reinterpretation
of the assignment field.

Generalization is corroborative rather than independently causal: Track 3
supplies a second known muted row; seven other Ode rows supply known unmuted
rows. Bells' authenticated nonempty omissions Track 2 and Track 7 both carry
`88` at the same track-relative location. No second track has yet undergone
a controlled mute transition in this evidence. The baseline also has `00`
for Sequence I descriptor 11, so interpreting every value as a Boolean or
masking arbitrary flags is not supported.

## Implementation decision and smallest next experiment

**NO to an authoritative reusable mute/inclusion decoder now. YES to a credible,
structurally located mute-state candidate and this experiment's causal MIDI
result.** A read-only evidence collector could report the raw byte and observed
association, but must not promote it to export policy under this decision.

The unresolved step is independent transfer of the state interpretation to
another track, distinct from a Track 1-specific/dependent representation.
The observed `88/80` association does not establish other flag combinations,
or make mute OFF sufficient for inclusion when other playback/export controls
apply. A structural Descriptor166 match alone cannot establish those controls.
Unknown values (including `00`), unsupported layouts, ambiguous ordinal binding,
or unreadable candidate bounds must remain unknown, with no inferred inclusion
or readiness upgrade. Do not use a global absolute offset or blindly mask `0x08`.

This cannot replace authenticated exact-profile policy. Bells' two nonempty
omissions correlate with `88`, but empty-row omissions are a different policy.
Moreover, the authenticated Ode profile explicitly includes Track 1 and Track 3
from a baseline whose candidate bytes are `88`. Its historical nine-track
reference export procedure remains unestablished. Applying a global `88 means
omit` rule would change that profile's authenticated output contract.

The baseline's twelve sequences outside the six exact Ready profiles are
xForm, Situation, Sequence D, Sequence E, mission impossibl, happyone,
Sequence I, newsong, Renaissance, Get on up & Dance, Jurrasic Park, and Sequence R.
The six with `88` candidates are xForm, Sequence E, happyone, Sequence I,
Get on up & Dance, and Jurrasic Park (the actual serialized spelling).
They are potential future inclusion-investigation targets, not readiness gains;
Sequence I additionally contains unsupported `00`. The other six have only
`80` among their track candidates. Current generic AppService assessment still
requires authoritative routing and safe track binding, so this finding alone
promotes none of the twelve to Ready.

**Single next action for owner review:** preregister one Track 3-only Mute
ON -> OFF replication from a duplicate of saved EXP35 EDIT, with Track 1
remaining OFF. Use EXP35 EDIT and its MIDI as the existing control if the same
verified environment/export conditions persist. Record the reopened Track 3
state and fresh edited Multitrack export. Predict Track 3's label-minus-39
byte `88 -> 80`, unchanged assignment byte, unchanged Track 1 and other
candidate values/events, and exactly Track 3 added to the eight existing musical
tracks. This is the smallest second-track intervention; no rerun of Experiment
035 or new no-edit control is required solely to reproduce established evidence.
A matching result would strengthen a rule limited to verified `80/88` states
and this export mode; it would not validate unknown flags or override profiles.
No follow-up experiment is authorized or executed by this analysis.

## Related research

- [Experiment 033](CONTROLLED_TRACK2_INSTRUMENT_ASSIGNMENT_CHANGE.md)
- [Experiment 034](CONTROLLED_TRACK2_INSTRUMENT_ASSIGNMENT_JUNO106.md)
- [Ode structure](ODE_TO_CLARKE_CHANNEL_CORRELATION.md)
- [Sequence parser](ROOT_SEQUENCE_CONTAINER_PARSER_IMPLEMENTATION.md)
- [Exact-profile policy](AUTHENTICATED_COMPATIBILITY_PROFILE_IMPLEMENTATION.md)

## Evidence retention and regeneration

Only this evidence record is versioned for Experiment 035. The generated raw
(4987627 bytes), aligned (166123 bytes), and cross-project (21114 bytes) annexes
were reviewed and removed from the repository worktree before checkpointing.
They repeated reproducible data and added no independent provenance; the raw
dump also repeated substantial authentic payload content. This follows the
repository's preference for small evidence-based changes and its restriction
on submitting non-redistributable sample files. Authentic external artifacts
were neither modified nor deleted. Hashes, structural anchors, byte contexts,
comparison totals, and interpretation boundaries remain here.

To regenerate the comparison/inventory, save the following standard-library
Python 3 recipe outside the repository, then run it with stdout redirected to
a local report (for example `python3 /tmp/exp035_regenerate.py > /tmp/exp035_report.txt`).
It reads external data forks only. Verify the four printed identities against
the table above before interpreting output; verify reference MIDI and prior
project identities against the linked records. Ranges are half-open; aligned
records use zero-based ordinals. A tail reports positional excess, not a
semantically localized deletion. `candidate` rows are raw observations, not
a mute decoder. The recipe reproduces byte evidence; the MIDI comparisons
were separately structurally parsed and checked as described above.

```python
from pathlib import Path
from hashlib import sha256

root = Path("/Users/kurtheiden/Documents/Phoenix Research")
collection = root / "Controlled Save Experiments"
folder = collection / "Experiment 035 - Track 1 Mute On to Off:Returned from MacOS9"
a = (folder / "EXP35 CTRL").read_bytes()
b = (folder / "EXP35 EDIT").read_bytes()
for name in ("EXP35 CTRL", "EXP35 CTRL MID", "EXP35 EDIT", "EXP35 EDIT MID"):
    data = (folder / name).read_bytes()
    print("identity", name, len(data), sha256(data).hexdigest())

def records(data):
    result, start = [], 8
    while start < len(data):
        assert start + 5 <= len(data)
        end = start + 5 + int.from_bytes(data[start + 1:start + 5], "big")
        assert end <= len(data)
        result.append((start, end, data[start]))
        start = end
    assert start == len(data)
    return result

def differences(left, right, left_base=0, right_base=0, label="raw"):
    runs = []
    for i, (x, y) in enumerate(zip(left, right)):
        if x != y:
            if runs and runs[-1][1] == i:
                runs[-1][1] += 1
            else:
                runs.append([i, i + 1])
    for start, end in runs:
        print(label, "run", hex(left_base + start), hex(left_base + end),
              hex(right_base + start), hex(right_base + end),
              left[start:end].hex(), right[start:end].hex(),
              "context", left[max(0, start - 12):end + 12].hex(),
              right[max(0, start - 12):end + 12].hex())
    if len(left) != len(right):
        start = min(len(left), len(right))
        print(label, "tail", hex(left_base + start), hex(left_base + len(left)),
              hex(right_base + start), hex(right_base + len(right)),
              left[start:].hex(), right[start:].hex())
    return sum(end - start for start, end in runs), len(runs)

print("raw totals", differences(a, b))
ra, rb = records(a), records(b)
assert len(ra) == len(rb) == 527
assert a[:8] == b[:8]
totals = [0, 0]
for i, ((start, end, kind), (other, stop, other_kind)) in enumerate(zip(ra, rb)):
    assert kind == other_kind
    label = f"record={i} type={kind:02x} bases={start:x},{other:x}"
    counts = differences(a[start:end], b[other:stop], start, other, label)
    totals = [x + y for x, y in zip(totals, counts)]
print("aligned totals", totals)

projects = [collection / "Experiment 007 - Untouched Baseline/newest STUFF baseline"]
for number in (33, 34):
    parent, = collection.glob(f"Experiment {number:03d} - *")
    projects += [parent / "Returned from MacOS9" / f"EXP{number} {role}"
                 for role in ("CTRL", "EDIT")]
projects += [folder / f"EXP35 {role}" for role in ("CTRL", "EDIT")]
for path in projects:
    data = path.read_bytes()
    print("project", path, len(data), sha256(data).hexdigest())
    for start, end, kind in records(data):
        if kind != 1:
            continue
        count = data[start + 5]
        name_at = start + 208 + count * 166 - 15
        assert count >= 2 and name_at < len(data)
        name = data[name_at + 1:name_at + 1 + data[name_at]].decode("mac_roman")
        for ordinal in range(2, count):
            descriptor = start + 208 + ordinal * 166
            assert descriptor + 166 <= len(data)
            label = data[descriptor + 15:descriptor + 166].split(b"\0")[0]
            print("candidate", name, ordinal, label.decode("mac_roman"),
                  hex(descriptor - 24), hex(data[descriptor - 24]),
                  "assignment", hex(descriptor - 16), hex(data[descriptor - 16]))
```

## Checkpoint review and validation

Independent review re-read all four artifacts, checked their sizes/hashes,
parsed both SMFs, compared retained and reference payloads, re-walked root
records, and verified candidate anchors, relocation, and comparison totals.
Every cross-project annex value/offset and identity was checked against its
source before removing the redundant generated files. The original analysis
also reconstructed EDIT exactly from CONTROL and the complete raw differences.

The review retains the causal result only for the supplied controlled conditions;
`88`, previous-slice `+0x8e`, and their bit interpretation are not universal or
authoritative. No exact-profile policy is removed and no Partial status is
promoted. The proposed Track 3 replication remains a future owner-review item;
Experiment 036 was not begun.

Checkpoint gates passed: `cargo fmt --check`,
`cargo clippy --all-targets --all-features -- -D warnings`, `cargo test`, and
`git diff --check`. Local-link, final-newline, trailing-whitespace, and
conflict-marker checks passed. The embedded regeneration recipe was executed
and reproduced all four identities and both comparison totals. No separate
repository documentation-check command was found. Pre-existing unrelated
worktree file contents were preserved and verified by SHA-256.
