# Prologue Studio Vision "Key" field replication checkpoint — 2026-09-18

## Evidence boundary and paired experiments

This documentation-only checkpoint follows `e0fade8dcc4e1faa5e2c14acb6aff3b38898a41b` (`Document Prologue Key I-J causal result`). The earlier controlled edit was Sequences -> Frames 9-10, Studio Vision "Key" assignment I -> J: G1 object-relative `+70` changed `0x22 -> 0x26`; corresponding G2 `+70` remained `0x22`. Its primary preregistered result was H1. See `docs/OS1_PROLOGUE_KEY_I_J_CAUSAL_CHECKPOINT_2026_09_18.md` and the separate durable I/J research record.

An intended Frame 1 F -> G replication stopped **before edit acceptance**. Studio Vision warned exactly, “This key is already used by . Are you sure you want to use it for this also?” The owner selected Cancel, made no F -> G save, and did not classify that abort as an R1–R6 experimental outcome. The screenshot independently showed Frames 2-4 displaying G, while the warning's assignment-owner name appeared blank. This is behavioral evidence of reuse detection, not proof that uniqueness is mandatory or that the blank name identifies Frames 2-4. The F/G abort record remains separate.

The replacement experiment began from preserved `MacOS9HD:Desktop Folder:Original Files:Prologue Master`. Two independent same-volume Finder duplicates, `FJ-C-IN` and `FJ-E-IN`, remained on MacOS9HD. In separate fresh Studio Vision Pro 4.5 sessions, the responsive control was saved without an intentional project edit; the responsive experimental arm changed only the intended **Sequences -> Frame 1 F -> J** assignment and displayed J after acceptance. Both arms used File -> Save a Copy As... on MacOS9HD to create authoritative `FJ-C-OUT` and `FJ-E-OUT`. The owner copied the completed results to Unix/Shared only after Studio Vision closed.

## Preserved identities and transfer limits

The successful outside-repository research record is `/Users/kurtheiden/Documents/Phoenix Research/Prologue-Key-F-J-Guest-Native-2026-09-18/OBSERVATION.md`. Its complete exact byte ledger is `EXACT_DIFF_LEDGER.txt`, SHA-256 `2a5eb65117bcaf31d88d57f3077e87834a74d7370516bb5f54e9706cda2f0c6c`.

| Arm | Host-visible transfer copy in `/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/FJ Key Results/` | Logical bytes | SHA-256 |
|---|---|---:|---|
| No-edit control | `FJ-C-OUT` | 58,119 | `8e8e2ff1a16d3bb83e42fed333b94cbb844688faaea1c13579acc215ae49c5d0` |
| F -> J experiment | `FJ-E-OUT` | 58,119 | `ba818cdbf72400481f49c2af9c60d8c0e7b0baa63f14e2c6d07d1bc801ee25fe` |

The transfer is class **A for data-fork comparison**. The host copies were not independently hash-matched to their authoritative MacOS9HD outputs; native save and copy provenance is owner-reported. No host-visible resource fork was found. This does not prove preservation of all HFS metadata or forks.

## Exact F/J result and replication

Both data forks have **297 root records** from byte 8 to EOF `0xe307`, with matching record boundaries and unchanged geometry. They differ at **69 bytes in 35 contiguous runs**: 21 two-byte, eight one-byte, five three-byte, and one four-byte runs. The exact ledger lists every changed byte and structural location. G1 Frame 1 contains 23 differences, G2 Frame 1 contains four, and 42 lie outside those two objects. Repeated two-byte changes across named objects and other save-time differences have unknown meanings; **do not attribute all 69 bytes to F -> J**.

The frozen pre-experiment prediction was G1 Frame 1 `+70` `0x03 -> 0x26`, with G2 Frame 1 `+70` remaining `0x03`. The observed result matches exactly:

| Saved Frame 1 object | Range in both outputs, inclusive | Absolute `+70` | Control -> experiment |
|---|---|---:|---|
| G1 | `0x09d8..0x0dc6` | `0x0a1e` | `0x03 -> 0x26` |
| G2 | `0x77d0..0x7bbe` | `0x7816` | `0x03 -> 0x03` |

**Primary preregistered outcome: R1.** Together, the I/J and F/J controlled edits establish for this Prologue specimen that two different directly edited Sequences, starting at different assignment bytes, each changed G1 object-relative `+70`; corresponding G2 `+70` stayed unchanged; and J appeared as `0x26` at G1 `+70` in both. The J value was **preregistered from I/J**, not independently discovered by F/J. G1 `+70` is strongly supported as a replicated causal serialized field for the displayed Studio Vision "Key" assignment, and G1 is strongly associated with the directly edited Sequences-side representation. Complete G1/G2 Sequence/Segment semantics remain unresolved.

## UI function, unknowns, and release priority

In separate owner-observed UI behavior, holding Shift while entering J made Studio Vision display **`↑J`**. With `FJ-C-OUT` open and no Sequence editor open, pressing computer-keyboard D later made playback **appear to start on the transport**; the owner made no intentional edit and did not save after this exploration. Alongside the duplicate-assignment warning, these observations establish the displayed "Key" field's **computer-keyboard keystroke assignment** UI function and ability to represent at least a Shift-modified keystroke. The D observation does not yet establish exact runtime trigger semantics or a specific shortcut for opening Sequences.

F/J tested only an **unmodified** assignment. Modifier serialization, meanings of other save-time differences, mapping across all assignments, generality across files/versions, and complete group semantics remain unknown. F, I, and J are not musical keys in this context. No generalized production decoder or Phoenix parser, readiness, profile, or export change is warranted now.

Key-field experimentation is **paused for Phoenix 0.1**. The next work returns to the release-critical OS1 cross-project/blind-validation and recovery path; the 0.1 release goals do not require a complete shortcut-field decoder. This checkpoint changes only this new research document. It does not modify ROADMAP.md, DECISIONS.md, protected local files, source/result artifacts, or existing research records.
