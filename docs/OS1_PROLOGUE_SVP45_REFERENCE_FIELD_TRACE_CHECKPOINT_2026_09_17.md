# Prologue SVP 4.5 reference-field trace checkpoint — 2026-09-17

## Status and evidence boundary

This documentation-only checkpoint follows committed authority `1db26694e5aa094fa4f869083e402edc8b8405ec` (`Document Prologue SVP 4.5 normalization research`). It records the final **bounded, read-only** search for a descriptor-to-pair reference relationship in the same preserved original and Studio Vision Pro 4.5 save. The [normalization checkpoint](OS1_PROLOGUE_SVP45_NORMALIZATION_RESEARCH_CHECKPOINT_2026_09_16.md) remains the authority for the save provenance, corrected Studio Vision behavior, root framing, and earlier old-to-new mapping.

| Artifact | Host path | Verified data-fork SHA-256 |
|---|---|---|
| Preserved old `Prologue Master` | `/Users/kurtheiden/Documents/Phoenix Research/Prologue Master Metadata Test/Prologue Master` | `b3a3efd8179d3ffe2a5026b3a40eab74da3ce6e674bb06162a176df07389b733` |
| Separately SVP-written `Prologue SVP Transfer` | `/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/Prologue SVP Transfer` | `0618cd5087987d7f694ca7532e1cc8a01b4adf7e3043ec648783d71e7e5ad714` |

Both data forks remained unchanged. No Phoenix production inspection was run. The preserved guest-native `Prologue SVP Save` was not touched; host Shared metadata is not a complete statement about its guest HFS state.

## 1. Bounded linkage question

Prior read-only analysis established **110** count-derived saved 166-byte descriptor slots, **55 per seven-object group**, and **37** strongly aligned cross-group type-`0x02/0x29` pairs. Frames 9-10's three musical descriptors and three pairs are **strongly supported** as positionally associated through ordering and cross-group byte evidence, but no decoded descriptor-to-pair key had been demonstrated. Saved descriptor-relative bytes `+79..80`, `+87..88`, and the apparent 46-byte old-to-new expansion were selected for this final bounded trace. This checkpoint reports the tests' negative and positive evidence; it does not infer a reference from numerical proximity.

## 2. Two candidate fields

| Field | Direct observations | Result |
|---|---|---|
| Descriptor `+79..80` | All **74 labeled musical slots** have nonzero values, distinct across the saved data fork. All **eight empty-label slots** are zero. Meter and Tempo slots are nonzero, with their values conserved between same-name groups; corresponding musical values differ between groups. | Class-correlated varying field. Target and semantics **UNKNOWN**. |
| Descriptor `+87..88` | All Meter, Tempo, and empty-label slots are zero. **64 of 74** musical slots are nonzero; **10** are zero. The nonzero values are distinct. This field lies within the new 46-byte region. | SVP-added class-correlated field. Target and semantics **UNKNOWN**. |

Neither field produced a repeatable absolute-offset, relative-offset, ordinal/index, or simple arithmetic target. Field distribution is evidence of structure, not evidence of what a field references.

## 3. Apparent 46-byte descriptor expansion

Across the mapped old/new descriptor views, old slot offsets `+81..+120` align approximately with saved offsets `+127..+166`. The intervening **saved `+81..+126` inclusive** is a 46-byte region; saved `+87..88` is at region bytes 6–7. This is a coherent observed alignment, **not** a complete historical format-migration specification. The final count-derived slot in each object crosses into the following name-record area, so those views must not be described as entirely inside the type-`0x01` record.

| Slot class | Inserted region |
|---|---|
| 14 Meter slots | All zero |
| 14 Tempo slots | All zero |
| 8 empty-label slots | All zero |
| 74 labeled musical slots | Nonzero repeated structure |

Among musical slots, saved `+85..88` contains `29 fa xx xx` in **64** slots and is zero in **10**. Recurring fixed bytes across all 74 musical slots include:

| Saved slot-relative offset | Byte |
|---:|---|
| `+98` | `01` |
| `+100` | `01` |
| `+104` | `78` |
| `+108` | `78` |
| `+112` | `78` |
| `+113` | `08` |
| `+115` | `01` |
| `+117` | `01` |
| `+126` | `80` |

Within the inserted region, matching group descriptors differ only at `+87..88` in the 32 corresponding slot pairs where that field is nonzero. The region's **purpose is UNKNOWN**. In particular, `0x78` must not be interpreted as a pair length merely because some pair payloads also have length 120.

## 4. Target-search scope and result

The bounded search tested both big-endian and little-endian readings; direct absolute offsets; addition and subtraction from file, containing type-`0x01`/neighborhood, descriptor start/end, and pair-run bases; root and payload starts; descriptor starts; type-`0x01`, `0x07`, `0x02`, `0x29`, `0x00`, and `0x05` record positions; ordinals and indices; pair offsets and payload lengths; primary identifier bytes; and simple fixed arithmetic relationships. Exact representations were tested before simple transformations. The search required recurrence and uniqueness across independent objects.

**No candidate met that standard.** Sparse isolated boundary matches were chance-compatible, not established references. The inserted `29 fa xx xx` values did not equal the 82 primary four-byte values at payload `+10..13`. Common template values such as 120 generated non-identifying length coincidences. No descriptor field uniquely selected its following pair by these tests.

Negative evidence rejects the tested direct BE/LE absolute-offset and relative-offset interpretations, direct ordinal/index interpretation, direct equality with pair primary identifiers, and a universal fixed arithmetic delta. It also rejects the assumptions that every object's first two local pairs are meter/tempo or that the tested descriptor fields predict the presence of local meter/tempo pairs. It does not prove that no indirect reference exists anywhere in the format.

## 5. Frames 9-10 targeted trace

The table gives descriptor two-byte values as raw hexadecimal, proposed paired root-record **starts**, primary payload length, and primary payload bytes `+11..13`. A proposed pair is an ordered correlation, not a decoded reference.

| Group / label | Descriptor `+79..80` | Descriptor `+87..88` | Proposed `0x02 / 0x29` starts | Primary payload length | Primary `+11..13` |
|---|---|---|---|---:|---|
| G1 Low Strings | `23f4` | `23c8` | `4eb0 / 5024` | 367 | `fa2404` |
| G1 Middle Strings | `23f8` | `2420` | `5048 / 51e4` | 407 | `fa240c` |
| G1 High Strings | `2400` | `241c` | `5208 / 5354` | 327 | `fa2414` |
| G2 Low Strings | `1efc` | `252c` | `bb4a / bcbe` | 367 | `fa1f0c` |
| G2 Middle Strings | `1f00` | `1ef4` | `bce2 / be7e` | 407 | `fa1f14` |
| G2 High Strings | `1f08` | `1ef0` | `bea2 / bfee` | 327 | `fa1f1c` |

The six slots have the common musical-slot inserted template. None of the tested descriptor fields uniquely selects pair 1, 2, or 3 through a tested reference key. Frames 9-10 descriptor-to-pair association remains **STRONGLY SUPPORTED** by ordinal and cross-group evidence, not established by a decoded reference.

## 6. Meter/tempo contrast and old-to-new field status

Earlier bounded-byte evidence identified two distinctive optional leading group-1 pairs in **Frame 1, Frames 7-8, Frame 11, and Frame 12**: their primary payloads contain bounded `FF 58 04` meter-form data followed by bounded `FF 51 03` tempo-form data. Corresponding original group-1 neighborhoods contain the same bounded event forms. This establishes the identity of those **payload forms**, without explaining why these local pairs occur only in those four saved group-1 neighborhoods.

All Meter and Tempo descriptors have zero-filled inserted regions and zero `+87..88`, including objects both with and without the optional local pairs. Their `+79..80` values are conserved between matching groups. **These descriptor fields do not predict optional local meter/tempo pair presence.**

The `+79..80` field position existed in the old 120-byte descriptor, but saved values changed; its role remains unresolved. Saved `+87..88` lies in the new region; old `+87..88` aligns elsewhere after the expansion. Saved `+87..88` is therefore newly introduced in this representation. No old-to-new descriptor/pair reference target was established.

## 7. Planned stop condition and next evidence source

The bounded read-only reference-field search found **NO repeatable relationship that materially improves descriptor-to-pair binding**.

**READ-ONLY REFERENCE-FIELD BRANCH: CLOSED.** Do not broaden this search speculatively. Reopen it only if future **independent evidence** supplies a specific, testable new relationship.

The next planned evidence source is a controlled Studio Vision Pro 4.5 edit of **Sequences -> Frames 9-10 -> Key: I -> J**. Its causal purpose is to determine which serialized Key-correlated field or fields change and establish the encoding of the newly displayed Key value. Because SVP 4.5 substantially reserializes the old source on save, that experiment requires an **independent no-edit control save** and a separately derived experimental save. Neither save nor the edit is performed by this checkpoint; prior preservation and control requirements remain in force.

## 8. Production and repository boundary

No Phoenix production parser, test, profile/hash/readiness gate, or research artifact was changed. No generalized support, Ready, or export claim is made. No generated MIDI derivative was inspected or used. This checkpoint creates only this documentation file, leaves it untracked for review, and stages, commits, and pushes nothing. The four protected pre-existing local files remain outside this checkpoint.
