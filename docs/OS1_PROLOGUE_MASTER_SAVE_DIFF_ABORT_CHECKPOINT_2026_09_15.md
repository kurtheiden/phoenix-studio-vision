# Prologue Master save/diff preparation and aborted control checkpoint

## Status and evidence boundaries — 2026-09-15

This documentation-only stopping point follows
`e8494cf942c9641c0f98e63f10b320862f569f38`
(`Document Prologue Master post-freeze research`). The
[post-freeze research checkpoint](OS1_PROLOGUE_MASTER_POST_FREEZE_RESEARCH_CHECKPOINT.md)
remains the authority for the frozen Phoenix observation and earlier structural
research. Nothing here rewrites that observation or authorizes format support.

Host byte and metadata results below came from read-only inspection. Guest
opening behavior, classic AppleScript results, and guest copy provenance are
owner-reported observations. The intended Key edit was never performed. No
control or experimental save occurred. No valid control save exists, and no
experimental save is currently authorized.

No Phoenix run, code change, or generated MIDI derivative inspection was part
of this investigation. This checkpoint changes no Prologue artifact.

## 1. Experiment design: designed, never performed

The proposed first experiment targeted exactly one visible field:

| Property | Selection |
|---|---|
| Category | Sequences |
| Object | Frames 9-10 |
| Field | Key |
| Intended edit | I -> J |
| Control save | Required |
| Predicted encoding of J | Unresolved |

The original paired Key-correlated candidate bytes are:

| Serialized group | Object | Original data-fork offset | Before byte |
|---|---:|---|---|
| First seven-name group | 5 | `0x3c92` | `0x22` |
| Second seven-name group | 13 | `0x7438` | `0x22` |

Both locations are object-relative `+70` decimal (`+0x46`). The proposed edit
was intended to distinguish whether one paired record, both records, or another
structure responds to a Sequence-category Key change. Neither category-to-group
assignment nor independent category editing was assumed established.

A newer Studio Vision Pro build might normalize, reorder, or migrate the older
document on save. An untouched control Save As and an independently derived
one-field experimental Save As were therefore required to separate intentional
change from ordinary save churn. Original offsets would require structural
alignment if saving changed the layout. No replacement byte for J was assumed.

## 2. Preparation

The disposable experiment area was created at:

```text
/Users/kurtheiden/Documents/Phoenix Research/Prologue-Save-Diff-io8oeuh0
```

Both inputs were independently copied directly from the preserved
metadata-restored research source:

```text
/Users/kurtheiden/Documents/Phoenix Research/Prologue Master Metadata Test/Prologue Master
```

Verified source identity:

- Data fork: **33,057 bytes**.
- SHA-256: `b3a3efd8179d3ffe2a5026b3a40eab74da3ce6e674bb06162a176df07389b733`.
- Finder Type: `MID2`; Creator: `MIDI`.
- No host-visible resource fork.

Paths relative to the disposable experiment area:

| Role | Path |
|---|---|
| Control input | `CONTROL INPUT/Prologue Master` |
| Experiment input | `EXPERIMENT INPUT/Prologue Master` |
| Planned control output | `SAVE OUTPUTS/Prologue CONTROL` |
| Planned experiment output | `SAVE OUTPUTS/Prologue EXPERIM` |
| Preparation identity record | `PREPARATION.json` |

The temporary preparation script at
`/private/tmp/phoenix_prepare_prologue.py` used
`/usr/bin/ditto --rsrc --extattr` separately from the source to each input.
Neither input was derived from the other. The manifest records identities and
planned output paths; the script, rather than the manifest, records the copy
command. Both output filenames have 16 characters and were reserved as new
paths, not created as files.

Preparation verified matching size, hash, FinderInfo, all listed extended
attributes, and resource-fork state. Subsequent read-only host comparison also
confirmed exact data-fork byte equality. Source and inputs matched in relevant
file metadata, apart from filesystem identity, metadata-change times, paths,
and containing-directory context. These host checks did not establish
behavioral equivalence after guest transfer.

No control or experimental output was saved; the host `SAVE OUTPUTS` directory
was observed empty.

## 3. Control attempt and abort

The owner transferred the prepared control into the SheepShaver Mac OS 9
environment and opened it in Studio Vision. After the normal Undefined
Channels dialog was cancelled, Studio Vision became nonresponsive. The owner
retried and reported that this prepared control repeatedly froze Studio Vision.

**No save occurred.** The intended I -> J experiment was halted before
execution. This behavior is not evidence of project corruption.

## 4. Known-working reference: corrected account

The earlier metadata-restored guest reference successfully opened in Studio
Vision, exposed the authenticated seven Sequences, seven same-named Segments,
and empty Templates structure, remained usable, and allowed Studio Vision to
be quit normally. It must not be described as a reference that crashed Studio
Vision.

The owner previously reported an apparent initial freeze before a SheepShaver
restart, followed by stable opening. That history leaves runtime state relevant;
it does not negate the successful, usable reference observation.

## 5. Guest Finder metadata

Read-only classic AppleScript inspection reported:

| Guest specimen | Guest path | Type | Creator |
|---|---|---|---|
| Known-working reference | `Unix:Desktop Folder:Prologue Master` | `MID2` | `MIDI` |
| Known-failing specimen | `Unix:Desktop Folder:Prologue Master-fail` | `MID2` | `MIDI` |

`Unix` is the name of the native Mac OS 9 startup disk; it does not identify
the Shared/extfs filesystem. Loss of basic Type/Creator metadata does not
explain the observed working/failing distinction. Other guest-native metadata
was not established by these AppleScript results.

MPW was not installed in the guest, so the proposed MPW binary comparison was
not performed.

## 6. Round-trip data-fork test

The owner preserved the known-working guest reference and made a duplicate
inside Mac OS 9 named `Prologue Master copy`. The working duplicate and failing
specimen were placed in SheepShaver's Shared directory for read-only host
comparison. The duplicate is a working-reference round-trip specimen, not an
independently authenticated successful Studio Vision opening of that duplicate.

Exact compared host paths:

```text
A — working round-trip specimen
/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/Prologue Master copy

B — failing round-trip specimen
/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/Prologue Master-fail

C — preserved metadata-restored host source
/Users/kurtheiden/Documents/Phoenix Research/Prologue Master Metadata Test/Prologue Master
```

All three were exactly **33,057 bytes**, with SHA-256:

```text
b3a3efd8179d3ffe2a5026b3a40eab74da3ce6e674bb06162a176df07389b733
```

Exact comparisons A/B, A/C, and B/C each found **zero differing data-fork
bytes**, zero unpaired trailing bytes, and no first differing offset. Both
round-trip specimens retain the original project data fork.

Ordinary data-fork transfer corruption is **not established**. There is no
data-fork difference in these specimens to explain their reported working/failing
provenance. This does not reconstruct every intermediate guest state.

## 7. Host metadata after the round trip

A and B both report Type `MID2` / Creator `MIDI`. Their identical 32-byte
`com.apple.FinderInfo` value is:

```text
4D4944324D494449010000000000000000000000000000000000000000000000
```

Both also have identical emulator-specific attributes:

```text
org.BasiliskII.FinderInfo
4D4944324D4944490100FFFFFFFF0000

org.BasiliskII.ExtendedFinderInfo
00000000000000000000000000000000
```

C's FinderInfo is:

```text
4D4944324D494449000000000000000000000000000000000000000000000000
```

At zero-based FinderInfo byte 8, C has `00` and both A/B have `01`. This does
**not** distinguish A from B. C has neither emulator-specific attribute.

Other observations:

| Property | A: working round trip | B: failing round trip | C: source |
|---|---|---|---|
| Quarantine | Absent | Present | Same as B |
| Permissions | `0644` | `0777` | `0777` |
| Modification/birth time | 2026-09-15 15:17:09 | 1994-05-08 22:55:24 | Same as B |
| Metadata-change time | 2026-09-15 15:29:24 | Same as A | 2026-09-13 19:55:52 |
| Owner/group | kurtheiden:staff | Same | Same |
| File kind | Regular file | Same | Same |
| Flags / ACL entries | None shown | None shown | None shown |

Times are as reported by host `stat`. Paths, filenames, inode identities, and
access timestamps also differed. A/B were on the external volume; C was on the
internal host filesystem. B/C had this identical quarantine value:

```text
0083;6aa569a0;Mail;CDC63CBE-F775-4EC0-9DDF-27118D5EE4A6
```

No resource-fork attribute or readable named resource fork was found for A/B/C;
no corresponding AppleDouble or checked `.rsrc` companion was found. This is a
host-visible result, not proof of guest fork state before transfer.

Quarantine, permissions, timestamps, paths, and filenames are not established
explanations for Studio Vision behavior. Shared-directory transfer can change
metadata representation. These results must not be treated as proof of exact
guest HFS metadata during the opening attempts.

## 8. Current interpretation

Established:

- The working-reference duplicate and failing round-trip specimen retain
  identical original project data forks.
- Both guest specimens reported `MID2` / `MIDI` through classic AppleScript.
- No observed ordinary data-fork corruption explains the behavioral difference.
- The save/diff experiment remains unperformed.
- No control save is valid; no experimental save is authorized yet.

Unresolved:

- Guest-native metadata beyond Type/Creator, including fork state at test time.
- Studio Vision / SheepShaver runtime state.
- Why the prepared control froze while the earlier reference worked.
- The original two-seven-record-group assignment and unmatched Prologue record
  role; this investigation did not resolve either structural question.

## 9. Next resume step: proposed, not performed

The next proposed diagnostic is one controlled behavioral retest after a clean
SheepShaver restart. Deeper HFS forensics is not automatically necessary.

On resuming with authorization for that test:

1. Preserve all current files and leave `Prologue Master copy` unused.
2. Open `Unix:Desktop Folder:Prologue Master-fail` in Studio Vision.
3. Cancel Undefined Channels.
4. Do not resize or save.
5. Click the Sequences disclosure triangle once.
6. Record whether Studio Vision responds.

This test has **not** been performed after the latest clean restart, according
to the owner's stopping-point update. This checkpoint does not perform it or
authorize an experimental save.

If the formerly failing byte-identical specimen now works, runtime/emulator
state becomes a stronger explanation, not a proven cause. If it still fails
while the known-working reference succeeds under comparable fresh conditions,
guest-native metadata differences become worth deeper investigation. That
conditional comparison is not a claim that such matched fresh tests already
occurred.

## 10. Repository and preservation boundary

Durable pre-checkpoint baseline:

```text
e8494cf942c9641c0f98e63f10b320862f569f38
Document Prologue Master post-freeze research
```

Before this document was created, HEAD, main, and origin/main matched that
baseline and staging was empty. Protected unrelated local work was:

```text
 M docs/DECISIONS.md
 M docs/ROADMAP.md
?? docs/CONTROLLED_TRACK3_2_MIDI_CHANNEL_CHANGE.md
?? docs/OS1_TOPIC3_BLOCKER_REVIEW_U3_001_U3_002.md
```

Those four files remain outside this checkpoint and must not be modified,
staged, reverted, deleted, or absorbed. Their content hashes matched the
preparation manifest during today's read-only repository verification.

The Steve-supplied original, blind intake, frozen observation, preserved
metadata-restored source, guest reference/specimens, and prepared host inputs
remain outside this documentation change. No generated MIDI derivative was
inspected. This stopping point creates only this new document; it stages,
commits, and pushes nothing.
