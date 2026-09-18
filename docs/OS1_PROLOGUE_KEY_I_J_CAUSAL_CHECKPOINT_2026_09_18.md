# Prologue Studio Vision "Key" I -> J causal checkpoint — 2026-09-18

## Scope and provenance

This is a documentation-only checkpoint after committed baseline `fdd049f43638ffaa3afd3a9d8ed134f3aebfa45d` (`Document failed Prologue Key control preparation`). The earlier failed preparation at `/Users/kurtheiden/Documents/Phoenix Research/Prologue-Key-I-J-m2m75ogo` remains separate and untouched. The successful guest-native record and complete byte ledger are in `/Users/kurtheiden/Documents/Phoenix Research/Prologue-Key-I-J-Guest-Native-2026-09-18/`: `OBSERVATION.md` and `EXACT_DIFF_LEDGER.txt`.

The owner preserved a currently responsive source at `MacOS9HD:Desktop Folder:Original Files:Prologue Master`. Two independent same-volume Finder duplicates of that source, `IJ-C-IN` and `IJ-E-IN`, remained on MacOS9HD. In separate fresh Studio Vision Pro 4.5 sessions, the control opened responsively and received **no intentional project edit**, while the experimental input opened responsively and received exactly one intentional project edit: **Sequences -> Frames 9-10, Studio Vision "Key" assignment I -> J**. The owner observed I before and J after the edit. Both arms used **File -> Save a Copy As...** on MacOS9HD; the owner declined to save changes back to `IJ-E-IN` when quitting. The authoritative guest-native results, `MacOS9HD:Desktop Folder:Original Files:IJ-C-OUT` and `IJ-E-OUT`, remain there per owner report.

After Studio Vision closed, the owner copied the completed results into Unix/Shared `IJ Key Results`. The host-visible transfer copies were verified before byte analysis:

| Arm | Host transfer copy | Exact data-fork bytes | SHA-256 |
|---|---|---:|---|
| No-edit control | `/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/IJ Key Results/IJ-C-OUT` | 58,119 | `214d5adcc358f03483ba0ccf70045ee7cc6d06ce9b1ad7e1ee9e352186d6b7c0` |
| I -> J experiment | `/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/IJ Key Results/IJ-E-OUT` | 58,119 | `ecd582af918135951d3d8d634ad3e3a6369adc74d63b633593309b9def430ecd` |

The transfer is class **A for data-fork comparison**, not proof that every classic HFS metadata field or fork survived. The host copies were not independently hash-matched to their authoritative MacOS9HD sources; the copy provenance is owner-reported. No host-visible resource fork was found. The successful record preserves full host metadata and these limits. The exact ledger's SHA-256 is `bbbc049445bef98fa4580ae4fa30c8620b22d971c7738efb5f028d9c18913abc`.

## Exact paired difference and preregistered outcome

The two 58,119-byte data forks differ at **907 bytes in 361 contiguous runs**: 269 three-byte, 88 one-byte, two two-byte, and two four-byte runs. Both files walk from offset 8 through **297 root records**, with matching record boundaries and the same EOF framing. `EXACT_DIFF_LEDGER.txt` records **every changed byte and run**, with root/object-relative locations.

| Saved object | Range, inclusive, in both files | `+70` absolute offset | Control | Experiment |
|---|---|---:|---:|---:|
| G1 Frames 9-10 | `0x4a87..0x4e75` | `0x4acd` | `0x22` | `0x26` |
| G2 Frames 9-10 | `0xb721..0xbb0f` | `0xb767` | `0x22` | `0x22` |

**Primary preregistered outcome: H1.** Only G1 Frames 9-10 object-relative `+70` changes. This supports association of G1 with the directly edited Sequences-side representation. The paired saves also contain widespread patterned differences: 269 three-byte runs share numeric increase `0x675d0`, 84 one-byte runs are `dd -> e4`, and two four-byte changes occur in G1 named objects. G1 Frames 9-10 has 32 changed bytes including `+70`; G2 Frames 9-10 has 27 changed bytes with `+70` unchanged; 848 differences are outside those two objects. These other bytes remain fully enumerated in the ledger. Their spread means the complete save is **not a one-byte patch**. The repeated patterns and preserved root geometry allow the targeted `+70` response to be recognized, but do not justify attributing all 907 changes to the assignment edit.

## Interpretation and limits

- **Established:** the exact paired byte differences; G1 Frames 9-10 `+70` changes `0x22 -> 0x26`; G2 Frames 9-10 `+70` remains `0x22`; for this controlled specimen, the I -> J intervention changes G1 `+70`.
- **Strongly supported:** G1 `+70` is a causal serialized field for this specimen's displayed Studio Vision "Key" assignment; observed I/J bytes agree with classic Mac virtual-key values; G1 is associated with the directly edited Sequences-side representation.
- **Plausible:** repeated numeric shifts reflect save-time relocation or state; the two four-byte changes reflect selection state.
- **Unknown:** meanings of other save-time fields; full G1/G2 semantics; whether G2 specifically represents Segments; whether this encoding generalizes across other Studio Vision files or versions.

The I and J values are **Studio Vision "Key" assignments**, likely keyboard assignments or shortcuts; they are not musical keys. The `0x22` / `0x26` correspondence strengthens that interpretation, but does not establish a general decoder. G2's unchanged `+70` does not prove absence of every possible Segment-side propagation, nor identify the two groups conclusively as Sequence and Segment. The full role of the G1 four-byte change and the repeated save-time fields remains unknown.

No Phoenix production parser, readiness/profile, or export change is warranted from this single controlled specimen. The next research step is a second controlled Studio Vision "Key" assignment experiment on a **different Sequence and/or a different letter transition**, with a fresh no-edit paired control, to test whether object-relative `+70` and the virtual-key correspondence replicate independently. That experiment has not begun.

This checkpoint adds only this research document. It does not alter production code, tests, research result artifacts, the failed preparation, or the four unrelated local files.
