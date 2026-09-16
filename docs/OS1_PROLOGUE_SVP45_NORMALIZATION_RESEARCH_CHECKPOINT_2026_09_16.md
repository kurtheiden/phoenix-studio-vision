# Prologue Master: SVP 4.5 normalization research checkpoint — 2026-09-16

## Status and evidence boundary

This documentation-only checkpoint follows `1104a2ff4eab8aba1b68f0fedf9a308c4a1d5965` (`Document aborted Prologue save-diff experiment`). The [post-freeze research checkpoint](OS1_PROLOGUE_MASTER_POST_FREEZE_RESEARCH_CHECKPOINT.md) remains the authority for the frozen Phoenix observation and earlier structural research. This checkpoint records a Studio Vision Pro 4.5 no-intentional-edit save and subsequent **read-only** host byte analysis. Guest behavior, save actions, and classic AppleScript results are owner-reported; host sizes, hashes, metadata, and byte structures were inspected directly. No I -> J edit occurred.

The observations below distinguish data forks from metadata. A copy in SheepShaver Shared does not establish every HFS property of a separate guest-native save. Neither the research hypothesis below nor a structural resemblance authorizes Phoenix format support.

## 1. Provenance and save context

| Role | Artifact and location | Data fork | Identity |
|---|---|---|---|
| Preserved pre-save source | `Prologue Master` at `/Users/kurtheiden/Documents/Phoenix Research/Prologue Master Metadata Test/Prologue Master` | 33,057 bytes; SHA-256 `b3a3efd8179d3ffe2a5026b3a40eab74da3ce6e674bb06162a176df07389b733` | Historical Finder identity recovered from FINDER.DAT: Type `MID2`, Creator `MIDI` |
| Preserved guest-native save | `Prologue SVP Save` on native Mac OS 9 startup disk `Unix` | Not host-compared here | Owner used SVP 4.5 File -> Save a Copy As...; classic AppleScript reported Type `MID2`, Creator `MIDA`. Preserve untouched. |
| Host-accessible research save | `Prologue SVP Transfer` at `/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/Prologue SVP Transfer` | 58,119 bytes; SHA-256 `0618cd5087987d7f694ca7532e1cc8a01b4adf7e3043ec648783d71e7e5ad714` | Separately written directly by SVP 4.5 with Save a Copy As... into Shared; host-visible Type `MID2`, Creator `MIDA` |

The two saves are **separately written artifacts**, not a transfer of one saved file to the other location. The owner made no intentional project or musical edit before either save. The original project remained open in Studio Vision during the initial report. The preserved guest-native save was not touched for this analysis. The host Shared specimen cannot establish its complete metadata or fork state, nor independently prove semantic identity of every internal property.

### Corrected behavior

Both `Prologue Master-fail` and the known-working `Prologue Master` initially showed project-window controls that appeared nonresponsive. Studio Vision itself was responsive enough to quit normally; an application-wide freeze or crash was **not established**. The known-working original subsequently became fully responsive without a file change: disclosure triangles operated and Sequences could be selected and opened. A post-open settling/delay is plausible, but its cause is unknown. Earlier shorthand that Studio Vision “froze” must not be read as an established crash or application-wide freeze.

## 2. Save-time data-fork rewrite and root framing

The saved transfer is **25,062 bytes larger** than the preserved original. Its data fork and SHA-256 differ. Differences begin near the start and continue across the old file's range; substantial original byte runs also survive at shifted offsets. This is a substantial re-serialization, not a small patch at original offsets. No intentional edit was made, but complete semantic equivalence has not been independently proved.

Both data forks support an exact checked root walk from offset 8 using `type:u8 | payload_length:u32 big-endian | payload`:

| Observation | Original | SVP transfer |
|---|---|---|
| Eight-byte header | `00 1b 00 03 00 46 00 78` | `00 2d 00 03 00 ac 00 a6` |
| Root records | 224 | 297 |
| Exact EOF | `0x8121` (33,057) | `0xe307` (58,119) |
| Type `0x01` | 15 | 14 |
| Type `0x02` | 46 | 82 |
| Type `0x29` | 45 | 82 |

These are root-record counts, not independently established semantic-object counts.

## 3. Two named groups and changed geometry

The original's 15 type-`0x01` records comprise seven named frame records, one unmatched record named `Prologue`, and another seven named frame records. Each original named record has total byte size `75 + 120*n`, where `n` is its byte at record start `+5`.

The SVP transfer has **14** type-`0x01` records in two ordered seven-name groups. Each has total size `177 + 166*n` (payload `172 + 166*n`). Both groups have the same ordered names and count bytes:

| Ordinal in each group | Name | Count `n` | Group 1 saved range | Group 2 saved range |
|---:|---|---:|---|---|
| 1 | Frame 1 | 5 | `0x09d8..0x0dc7` | `0x77d0..0x7bbf` |
| 2 | Frames 2-4 | 7 | `0x13b6..0x18f1` | `0x80ff..0x863a` |
| 3 | Frames 5-6 | 8 | `0x1f0c..0x24ed` | `0x8c55..0x9236` |
| 4 | Frames 7-8 | 11 | `0x3c1f..0x43f2` | `0xa968..0xb13b` |
| 5 | Frames 9-10 | 5 | `0x4a87..0x4e76` | `0xb721..0xbb10` |
| 6 | Frame 11 | 10 | `0x5387..0x5ab4` | `0xc021..0xc74e` |
| 7 | Frame 12 | 9 | `0x6c9d..0x7324` | `0xd888..0xdf0f` |

Every saved candidate satisfies Phoenix's existing Descriptor166 count and name-boundary arithmetic. All **14 are partial structural matches; zero are full matches** under the current container acceptance sequence. The cross-version ordinal mapping of these 14 named objects is **strongly supported**, not a proven complete semantic equivalence: the ordered names, count sequence, and object-relative group markers persist. Group 1 retains `+41 = fe` and `+43..+46 = 00 00 00 00`; group 2 retains `+41 = ff` and `+43..+46 = 00 1f 5e 98`. Which group represents visible Sequences versus Segments remains unresolved.

### Unmatched original Prologue record

The original unmatched type-`0x01` record is `0x6004..0x61b7`. Its neighborhood contained seven `ff 31 03` patterns with values `0000..0006` and a type-`0x05` boundary before the second seven-name group. The saved fork has 14 type-`0x01` records, no exact `Prologue` text, and none of those seven exact patterns. A type-`0x05` boundary remains between saved groups at `0x77cb..0x77d0`. The fate of the unmatched material is **UNKNOWN**: absence of the literal name and patterns is not proof that its content was deleted. The owner's contextual observation is that “Prologue” likely refers to the game's prologue content; the word supplies no technical-role evidence.

## 4. Saved object neighborhoods and cross-group pairs

All 14 saved named objects have this complete root-record family order, from their type-`0x01` record through their associated type-`0x00` terminal:

```text
type 0x01 -> consecutive type 0x07 records
          -> alternating type 0x02 / type 0x29 pairs
          -> type 0x00 terminal
```

The first type-`0x07` record contains the object name. Subsequent type-`0x07` records contain short text/comments or short nonprinting data. For each same-named group-1/group-2 pair, the complete ordered type-`0x07` payload sequences are byte-identical. The `0x07` count varies by object and does not equal the type-`0x01` count byte. Meter Track, Tempo Track, and musical track labels appear in 166-byte-spaced positions of the type-`0x01` layout, rather than in these short following records.

All **82** saved type-`0x02` records are immediately followed by type `0x29`: **45 pairs** in group 1 and **37 pairs** in group 2. For Frame 1, Frames 7-8, Frame 11, and Frame 12, group 1 has two distinctive leading pairs that group 2 lacks. Excluding those optional leading pairs aligns **37** cross-group pairs by name and order. For all 37, corresponding `0x02` payload lengths match and corresponding `0x29` payloads are byte-for-byte identical. Corresponding `0x02` payloads differ almost exclusively at payload offsets 12 and 13; one pair also differs at offset 11. Ordered cross-group pair correspondence is **strongly supported**. The pair contents, including the optional two leading pairs, have not been decoded into meter, tempo, track, or reference semantics.

The pair count is not uniformly `n`, `n-2`, or the number of `0x07` records:

| Name | `n` | `0x07` records per group | Pairs group 1 / group 2 |
|---|---:|---:|---:|
| Frame 1 | 5 | 4 | 5 / 3 |
| Frames 2-4 | 7 | 5 | 4 / 4 |
| Frames 5-6 | 8 | 6 | 5 / 5 |
| Frames 7-8 | 11 | 6 | 10 / 8 |
| Frames 9-10 | 5 | 4 | 3 / 3 |
| Frame 11 | 10 | 7 | 9 / 7 |
| Frame 12 | 9 | 7 | 9 / 7 |

The old type-`0x01` form already carried ordered labels, and following type-`0x07` records already carried short text. The save changed stride and name placement and expanded local pair neighborhoods, especially group 2 (which had no local `0x02/0x29` pairs in the original). It is not established that all newly present pairs represent newly created musical content or that old information was simply externalized.

## 5. Frames 9-10: strongest visible/byte correlation

The owner observed `Sequences -> Frames 9-10` and `Segments -> Frames 9-10` with the same apparent material. The UI showed meter 4/4, tempo 102.00, Seq Len 10, Display Phrases, tracks Low Strings / Middle Strings / High Strings, and comments `#97` / `#56-24` / `#52+24`. These are UI observations. No unique saved byte fields for the numeric UI properties or Display Phrases have been established.

Both saved Frames 9-10 type-`0x01` records have count 5. Five 166-byte-spaced descriptor label positions contain:

| Position | Label | Group 1 offset (`0x4a87..0x4e76`) | Group 2 offset (`0xb721..0xbb10`) |
|---:|---|---|---|
| 1 | Meter Track | `0x4b61` | `0xb7fb` |
| 2 | Tempo Track | `0x4c07` | `0xb8a1` |
| 3 | Low Strings | `0x4cad` | `0xb947` |
| 4 | Middle Strings | `0x4d53` | `0xb9ed` |
| 5 | High Strings | `0x4df9` | `0xba93` |

The following type-`0x07` name records are `0x4e76..0x4e92` and `0xbb10..0xbb2c`. Three further type-`0x07` records in each neighborhood carry `#97`, `#56-24`, and `#52+24`, respectively. Their string offsets are `0x4e97`, `0x4e9f`, `0x4eaa` (group 1) and `0xbb31`, `0xbb39`, `0xbb44` (group 2).

Each neighborhood then has exactly three `0x02/0x29` pairs. The three corresponding primary payload lengths are **367, 407, 327** bytes in both groups. Their corresponding secondaries are byte-identical; each primary differs only at payload offsets 12 and 13. Positional association of these three pairs with the three musical tracks is **strongly supported**, not established by a decoded descriptor-to-pair binding.

The old Frames 9-10 records and saved counterparts also preserve the object-relative Key-correlated byte:

| Group | Original record; `+70` byte | Saved record; `+70` byte |
|---|---|---|
| 1 | `0x3c4c..0x3eef`; `0x3c92 = 0x22` | `0x4a87..0x4e76`; `0x4acd = 0x22` |
| 2 | `0x73f2..0x7695`; `0x7438 = 0x22` | `0xb721..0xbb10`; `0xb767 = 0x22` |

Continuity of that relative location and value across the rewrite is **strongly supported**. The field remains *Key-correlated*; its full semantics and the encoding of `J` are unresolved. No I -> J experiment was performed.

## 6. Current Descriptor166 limitation and research hypothesis

Phoenix's existing container sequence requires a type-`0x02` meter primary immediately after the validated type-`0x07` name, apart from optional type-`0x09` preludes. In **every one of the 14** saved neighborhoods, the next record is another type `0x07`. This is their first common failure under the existing complete Descriptor166 container acceptance sequence. The immediate-meter assumption is therefore too narrow to describe this authentic SVP 4.5-written specimen. The arithmetic match is not a complete parser match, and simply skipping extra `0x07` records would not validate meter/tempo semantics.

**Research hypothesis, not an adopted parser rule:** for this specimen, a candidate structural-discovery frame is a type-`0x01` candidate, validated type-`0x07` name, bounded run of further type-`0x07` records, nonempty alternating type-`0x02/0x29` pair run, and type-`0x00` terminal. Such discovery would preserve root-record and pair ordinals without assigning meter, tempo, track, or export semantics. This hypothesis comes from **one SVP-written specimen**; it is not implemented, is insufficient for generalized parsing or Ready/export, and does not justify weakening any profile or hash gate.

## 7. Assessment, next target, and preservation boundary

The relationship between the older 120-byte and SVP-written 166-byte representations for the 14 named objects is **STRONGLY SUPPORTED** as related Studio Vision serializations: both use exact root framing, retain object ordering/count sequence and several relative fields, while changing stride and surrounding record neighborhoods. A complete shared object model is **not established**. Group category assignment, descriptor-to-pair binding, unmatched Prologue fate, and numeric UI field mapping remain unresolved.

A controlled Frames 9-10 Key `I -> J` edit would add unique causal evidence about the Key-correlated field and new encoding. It is deferred while read-only mapping remains available. **Next recommended target, not performed here:** map descriptor slots against `0x02/0x29` pair metadata, particularly the 37 aligned cross-group pairs and the distinctive optional two leading group-1 pairs.

No Phoenix production parser, code, test, profile/hash/readiness gate, or research artifact was changed. No generalized support or export readiness is claimed. No generated MIDI derivative was inspected or used. The original source remained unchanged; research specimens remain private. This checkpoint is the sole new repository file from this task and is left untracked for review.
