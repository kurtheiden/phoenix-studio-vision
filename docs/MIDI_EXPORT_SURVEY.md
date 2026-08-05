# Scope and method

This read-only survey covers every file in the external Studio Vision MIDI Exports collection. Relative paths and original filenames are preserved. Source artifacts were not modified. No parser code was implemented, no MIDI events were comprehensively decoded, and no commit was made.

Evidence was gathered with direct filesystem and byte-level tools: `find`, `stat`, `shasum -a 256`, `file`, `xxd`, `rg -aob`, `xattr`, named-fork reads, and `strings -a -t d`. Standard MIDI File (SMF) status required an `MThd` identifier at data-fork offset 0 and direct header/chunk-boundary measurements consistent with the file. Chunk signatures and big-endian declared lengths were measured without interpreting track event streams.

The complete heuristic printable-ASCII inventory, including every reported decimal byte offset and unfiltered value of four or more printable characters, is preserved in [MIDI_EXPORT_SURVEY_STRINGS.txt](MIDI_EXPORT_SURVEY_STRINGS.txt). Trailing ASCII spaces are rendered visibly as one `\x20` marker per byte. Short binary sequences can coincidentally appear printable; their inclusion is not an interpretation.

Filename-based grouping is used only to select cautious high-level comparisons. In particular, `Multi All` is treated solely as an original filename. It is not evidence of a distinct export mode.

# Direct observations

Eight files occur in the collection: seven SMFs under `Project 001/` and one root `.DS_Store`. No file was identified as an original Studio Vision project. The `.DS_Store` is inventoried separately as a non-SMF collection artifact and was not decoded.

## Complete file inventory

| Relative path | Size | SHA-256 | Data fork | Finder Type / Creator | Extended-attribute names | Resource fork |
|---|---:|---|---|---|---|---|
| `.DS_Store` | 6,148 | `18b11a26c918560db54f51b3fb229d00cfba4bd1b5b81926cfae98fbae546f56` | not SMF | eight space bytes in the Type/Creator positions | `com.apple.FinderInfo` | absent |
| `Project 001/ANALOG.MID #2` | 38,874 | `8115784c95850f55fc9addc711536e1a72e115d3ca1b539d879c995adc736f8e` | SMF | `Midi` / `MIDA` | `com.apple.FinderInfo`; `org.BasiliskII.ExtendedFinderInfo`; `org.BasiliskII.FinderInfo` | absent |
| `Project 001/ANALOG.MID #2 Multitrack` | 33,704 | `bda7a60314c4e1552acc105599d26739f44190365ec3054a093547d98c4bb59e` | SMF | `Midi` / `MIDA` | same three Finder-related names | absent |
| `Project 001/BATTL2GS.MID` | 19,848 | `de902b71506481ac481504ec8fa6b9b4782391d28a7ee185f759839817526c88` | SMF | `Midi` / `MIDA` | same three Finder-related names | absent |
| `Project 001/BATTL2GS.MID Multitrack` | 20,817 | `8c7deba333c9f97d5347f93a6b04429c28cb1d3e6ab6377aa4328c7ecbe0e2b1` | SMF | `Midi` / `MIDA` | same three Finder-related names | absent |
| `Project 001/Ode to Clarke` | 8,644 | `eb37711a81eee7d78877bfe2ca67712ac2b98067cbec9e23f9f8e739380bf5a6` | SMF | `Midi` / `MIDA` | same three Finder-related names | absent |
| `Project 001/Ode to Clarke Multi All` | 12,141 | `4f63b34ef92204d4bc5eeb78dbbe7b94d005c1f9ceb57ea0f9809533ad590f29` | SMF | `Midi` / `MIDA` | same three Finder-related names | absent |
| `Project 001/Ode to Clarke Multitrack` | 10,514 | `9979ed6d5fc58edb85c3c03e5e43b4c7015a353af6075ece8ba94ac49cbf5059` | SMF | `Midi` / `MIDA` | same three Finder-related names | absent |

For all seven SMFs, the 32-byte `com.apple.FinderInfo` begins with hexadecimal `4D 69 64 69 4D 49 44 41`, directly rendering as Type `Midi` and Creator `MIDA`. The corresponding `org.BasiliskII.FinderInfo` values have the same first eight bytes but differ later. Those later bytes were not decoded. No file has a readable resource fork, so resource-fork size and SHA-256 are not applicable.

## Printable ASCII and directly observable meta-event text

The exhaustive `strings` measurements contain 1,475 printable runs: `.DS_Store` 6; `ANALOG.MID #2` 112; `ANALOG.MID #2 Multitrack` 885; `BATTL2GS.MID` 36; `BATTL2GS.MID Multitrack` 178; `Ode to Clarke` 115; `Ode to Clarke Multi All` 79; and `Ode to Clarke Multitrack` 64. See the linked appendix for every value and offset.

The following readable values were also directly adjacent to MIDI meta-event marker bytes and declared text lengths. They are therefore recorded as obvious meta-event text, without inferring purpose beyond the observable event kind.

- `ANALOG.MID #2`: track-name text `ANALOG.MID #2` at offset 26; copyright text `(C)1994 Roland Corporation` at offset 80.
- `ANALOG.MID #2 Multitrack`: track-name text includes `ANALOG.MID #2` (26), `ANALOG.MID #8` (115), `ANALOG.MID #9` (192), `ANALOG.MID #10` (1,731), `ANALOG.MID #5` (2,513), `ANALOG.MID #4` (5,533), `ANALOG.MID #2` (6,253), `ANALOG.MID` (9,331), `ANALOG.MID #6` (15,128), `ANALOG.MID #3` (22,355), `ANALOG.MID #7` (22,929), `Track 1` (29,024), `Track 2` (29,672), and `Track 3` (33,205). Repeated `Sound Canvas` text occurs after track-chunk starts, and copyright text `(C)1994 Roland Corporation` occurs at offset 132.
- `BATTL2GS.MID`: track-name text `BATTL2GS.MID` at offset 25; readable channel-label text runs `M Cha 1 ` through `M Cha16 ` occur from offsets 58 through 278.
- `BATTL2GS.MID Multitrack`: track-name text includes `BATTL2GS.MID` (25), `piano hi` (88), `piano lo` (1,863), `Partials` (3,584), `violin  ` (3,666), `viola   ` (5,900), `cello   ` (7,163), `Basses  ` (7,943), `CB      ` (8,688), `hit,chime,hit` (9,433), `fl,hit,fl` (9,989), `Perc,Orch` (11,295), `ob      ` (15,447), `cl      ` (16,457), `tpt     ` (17,555), `Brass   ` (18,222), `tuba    ` (19,287), and `Timp,Chime, Timp` (19,719). Repeated `Sound Canvas` and channel-label text is also directly observable.
- `Ode to Clarke`: track-name text `Ode to Clarke` at offset 26.
- `Ode to Clarke Multi All`: track-name text includes `Ode to Clarke` (26), `Track 1` (93), `Track 2` (946), `sys100loops` (2,892), `Track 4` (5,776), `Track 5` (7,477), `Track 3` (8,693), `Track 6` (9,464), `Track 3 #2` (10,075), and `Track 7` (10,844). Nearby readable text includes `Juno-106`, `JV-1080`, `S-760`, and `JD-800`.
- `Ode to Clarke Multitrack`: track-name text includes `Ode to Clarke` (26), `Track 2` (92), `sys100loops` (2,038), `Track 4` (4,922), `Track 5` (6,623), `Track 6` (7,837), `Track 3 #2` (8,448), and `Track 7` (9,217). Nearby readable text includes `JV-1080`, `S-760`, and `JD-800`.

The nearby device-like strings are not labeled as a distinct meta-event kind here and are not interpreted as instruments, routing, or export settings.

# Standard MIDI File measurements

All offsets below are zero-based data-fork offsets. Every SMF has header identifier `MThd` at offset 0, declared header length 6, and timing division hexadecimal `0x01E0` (decimal 480). Each first track chunk begins at offset 14. In every file the actual count of boundary-consistent `MTrk` chunks equals the declared track count, and the final declared track boundary equals the data-fork size. Therefore no trailing or non-SMF bytes were measured in any of the seven SMFs.

| Relative filename | Format | Declared tracks | Actual track chunks | Track chunk declared lengths, in file order | Exact final boundary |
|---|---:|---:|---:|---|---|
| `Project 001/ANALOG.MID #2` | 0 | 1 | 1 | 38,852 | yes, 38,874 |
| `Project 001/ANALOG.MID #2 Multitrack` | 1 | 14 | 14 | 65, 69, 1,531, 774, 3,012, 712, 3,070, 5,789, 7,219, 566, 6,087, 640, 3,525, 519 | yes, 33,704 |
| `Project 001/BATTL2GS.MID` | 0 | 1 | 1 | 19,826 | yes, 19,848 |
| `Project 001/BATTL2GS.MID Multitrack` | 1 | 18 | 18 | 38, 1,767, 1,713, 74, 2,226, 1,255, 772, 737, 737, 548, 1,298, 4,144, 1,002, 1,090, 659, 1,057, 424, 1,118 | yes, 20,817 |
| `Project 001/Ode to Clarke` | 0 | 1 | 1 | 8,622 | yes, 8,644 |
| `Project 001/Ode to Clarke Multi All` | 1 | 10 | 10 | 47, 846, 1,940, 2,876, 1,693, 1,206, 765, 602, 760, 1,312 | yes, 12,141 |
| `Project 001/Ode to Clarke Multitrack` | 1 | 8 | 8 | 47, 1,940, 2,876, 1,693, 1,206, 602, 760, 1,312 | yes, 10,514 |

The `.DS_Store` begins with hexadecimal `00 00 00 01 42 75 64 31`, not `MThd`, and is not classified as an SMF. Its internal contents were not decoded.

# Comparisons among apparent related exports

These groups are based only on shared filename stems and readable text. “Appear related” does not establish common provenance or the actions that produced differences.

| Filename-based group | High-level comparison |
|---|---|
| `ANALOG.MID #2` | The unsuffixed-name file is format 0, 1 track, division 480, one 38,852-byte track chunk, total 38,874 bytes. The `Multitrack`-named file is format 1, 14 tracks, division 480, the 14-length layout listed above, total 33,704 bytes. Format, track count, chunk layout, and size differ; division is equal. |
| `BATTL2GS.MID` | The unsuffixed-name file is format 0, 1 track, division 480, one 19,826-byte track chunk, total 19,848 bytes. The `Multitrack`-named file is format 1, 18 tracks, division 480, the 18-length layout listed above, total 20,817 bytes. Format, track count, chunk layout, and size differ; division is equal. |
| `Ode to Clarke` | The unsuffixed-name file is format 0, 1 track, division 480, one 8,622-byte track chunk, total 8,644 bytes. The `Multi All`-named file is format 1, 10 tracks, division 480, total 12,141 bytes. The `Multitrack`-named file is format 1, 8 tracks, division 480, total 10,514 bytes. The latter two have six identical declared chunk lengths in the same relative sequence after omission of the `Multi All` file's 846- and 765-byte chunks, but no event comparison was performed. Across the group, format, track count, chunk layout, and size differ; division is equal. |

The literal filename `Ode to Clarke Multi All` and its measured ten-track layout do not establish a distinct Studio Vision export mode.

# Unknowns

- Whether filename-grouped files were exported from the same project state.
- Whether filename suffixes were assigned by Studio Vision, another application, or a person.
- Whether `Multitrack`, `.MID`, `#2`, or `Multi All` has any stable workflow meaning.
- Whether readable device-like and channel-like strings describe active settings, labels, copied metadata, or something else.
- The semantic contents of the MIDI event streams; they were not comprehensively decoded.
- Whether coincidental `MTrk` byte sequences occur inside track data. Actual-track counts here use declared-boundary traversal corroborated by observed chunk locations, not a claim that arbitrary signature searching alone proves structure.
- The meaning of non-Type/Creator FinderInfo bytes and of the BasiliskII extended metadata.
- The provenance and significance of `.DS_Store`.
- Whether an original Studio Vision project exists elsewhere but was not present in this collection.

# Hypotheses requiring further evidence

- Files sharing a filename stem may represent exports of related musical material. Establishing that requires provenance or event-level comparison.
- Format-0 and format-1 files within a filename group may reflect different export choices. Establishing the choices or user actions requires controlled Studio Vision experiments.
- The two format-1 `Ode to Clarke` files may share six track chunks at a high structural level because their declared lengths align after two chunks are omitted from one layout. Byte identity and musical equivalence were not tested.
- Finder Type `Midi` and Creator `MIDA` may reflect the creating application or later metadata restoration. The observed codes alone do not establish which.

# Recommended next investigations

1. Preserve fork-aware, read-only copies and the hashes reported here before further handling.
2. Acquire provenance for filenames and export steps, especially the literal `Multi All` name.
3. Compare related exports at the next evidence level: exact chunk hashes and then event inventories, without inferring user intent.
4. Create controlled Studio Vision exports with one option changed at a time and compare only after documenting the application version and environment.
5. Inventory any separately located original Studio Vision project without decoding it before authorizing binary comparison with these exports.
6. Retain the exhaustive strings appendix as heuristic evidence; do not treat arbitrary printable runs as fields or meta-events without surrounding-byte confirmation.
