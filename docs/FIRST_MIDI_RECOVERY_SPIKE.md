# Objective

This research spike asks whether known-good Standard MIDI File (SMF) exports named `Ode to Clarke` can be correlated with the authentic Studio Vision project `newest STUFF`. It does not implement a Studio Vision parser, alter any source artifact, add CLI behavior, or generate recovered MIDI.

The investigation distinguishes exact bytes, partial matches, transformed-pattern candidates, likely coincidental matches, and unknowns. It does not assume that Studio Vision stores SMF bytes directly.

# Files and provenance

The three exports were resolved in the external Studio Vision MIDI Exports collection. The exact basename `newest STUFF` was not present in the curated external Authentic Studio Vision Projects directory at inspection time. A search of the external research tree found one exact-basename project at `Opcode/MY MUSIC/newest STUFF`; its hash agrees with the `newest STUFF` artifact documented by prior Phoenix controlled-save research. That uniquely resolved artifact was used, but its collection-location discrepancy remains explicit.

| Evidence label | Path-neutral location | Data-fork size | SHA-256 |
|---|---|---:|---|
| authentic project | external research tree, `Opcode/MY MUSIC/newest STUFF` | 203,422 | `7f97918efd8c8756058b72b4cada4a39a1f0f94655aa2982d4e03cfa8a876114` |
| format-0 export | external Studio Vision MIDI Exports collection, `Project 001/Ode to Clarke` | 8,644 | `eb37711a81eee7d78877bfe2ca67712ac2b98067cbec9e23f9f8e739380bf5a6` |
| format-1 export | external Studio Vision MIDI Exports collection, `Project 001/Ode to Clarke Multitrack` | 10,514 | `9979ed6d5fc58edb85c3c03e5e43b4c7015a353af6075ece8ba94ac49cbf5059` |
| format-1 export | external Studio Vision MIDI Exports collection, `Project 001/Ode to Clarke Multi All` | 12,141 | `4f63b34ef92204d4bc5eeb78dbbe7b94d005c1f9ceb57ea0f9809533ad590f29` |

All analysis was read-only. A temporary Python analysis script and JSON output were written under `/tmp`; neither is part of the repository.

# Reference SMF event inventory

The exports were parsed only through SMF chunk and event framing. Running status, variable-length delta values, channel messages, tempo (`FF 51`), track-name (`FF 03`), and end-of-track (`FF 2F`) events were recognized to obtain the requested reference measurements. No musical interpretation was made.

All three files declare division 480 and end exactly at the final declared `MTrk` boundary. Chunk offsets and payload offsets are zero-based absolute data-fork offsets. SHA-256 covers the declared payload only.

## Chunk boundaries, hashes, and event measurements

Status counts include logically active status bytes after applying running status. Channel numbers are reported as 1–16. Delta ranges include every parsed event's delta; the complete in-memory inventory retained each event's delta, channel, note, and velocity during the search.

| Export | SMF format | Chunk | Chunk / payload offset | Declared length | Payload SHA-256 | Events | Status-byte counts | Note on / off | Channels | Note range | Velocity range | Delta range |
|---|---:|---:|---|---:|---|---:|---|---:|---|---|---|---|
| `Ode to Clarke` | 0 | 0 | 14 / 22 | 8,622 | `a31dc0da71a43423584222763575598de8affd3b1c4e23b880439e2adc19345f` | 2,275 | `81:207 89:831 8E:82 91:215 99:845 9E:86 B1:2 C1:1 CE:1 FF:5` | 1,146 / 1,120 | 2, 10, 15 | 36–85 | 0–127 | 0–3,002 |
| `Ode to Clarke Multitrack` | 1 | 0 | 14 / 22 | 47 | `f0c5182df76bd38e97f518d8ea25cbf193cc0281442dd01e1940b8a5b5b6f847` | 5 | `FF:5` | 0 / 0 | — | — | — | 0–94,080 |
| same | 1 | 1 | 69 / 77 | 1,940 | `703af92a1ea2c5bec5d5bbaaf7ea328c8d3dec0ccc6d4b9553227f175e5ead07` | 428 | `81:207 91:215 B1:2 C1:1 FF:3` | 215 / 207 | 2 | 37–61 | 0–127 | 0–6,834 |
| same | 1 | 2 | 2,017 / 2,025 | 2,876 | `d9b69d9a2f68e8b97ba7ff935b866fe20aaa366c5c373b756d0640e925985b6e` | 647 | `89:318 99:326 FF:3` | 326 / 318 | 10 | 66–70 | 0–120 | 0–6,174 |
| same | 1 | 3 | 4,901 / 4,909 | 1,693 | `bbe699f260728631620808a706f5764e2a0d69bcca7f8efaa2ceecbd0c3078ef` | 361 | `89:178 99:180 FF:3` | 180 / 178 | 10 | 72 | 0–127 | 0–4,224 |
| same | 1 | 4 | 6,602 / 6,610 | 1,206 | `30fb000ba9d9a1659526b6be0bb4bbf06657d5476a2870cc53d72c070e481714` | 271 | `89:133 99:135 FF:3` | 135 / 133 | 10 | 37–39 | 0–127 | 0–5,944 |
| same | 1 | 5 | 7,816 / 7,824 | 602 | `9f5213ff5eb967d36fcd1233c41533676bf295800647612b981ec2daa593faf6` | 123 | `89:59 99:61 FF:3` | 61 / 59 | 10 | 72 | 0–127 | 0–59,520 |
| same | 1 | 6 | 8,426 / 8,434 | 760 | `dad23dadad85a3e45ffdb38bf010448ffb27cfb8d0d6cdd09e241a29bd70d57a` | 172 | `8E:82 9E:86 CE:1 FF:3` | 86 / 82 | 15 | 73–85 | 0–127 | 0–10,176 |
| same | 1 | 7 | 9,194 / 9,202 | 1,312 | `ac3b4c0b213099cef87660e1945f51eb5037e0bbbd2d77b6c04587f165f39e1d` | 289 | `89:143 99:143 FF:3` | 143 / 143 | 10 | 36–38 | 11–127 | 0–47,280 |
| `Ode to Clarke Multi All` | 1 | 0 | 14 / 22 | 47 | `f0c5182df76bd38e97f518d8ea25cbf193cc0281442dd01e1940b8a5b5b6f847` | 5 | `FF:5` | 0 / 0 | — | — | — | 0–94,080 |
| same | 1 | 1 | 69 / 77 | 846 | `4bd80106efed0ad4d7257d5fa60c1c214770a9be05527649eafbcc17f2f189dd` | 186 | `80:89 90:93 C0:1 FF:3` | 93 / 89 | 1 | 63–75 | 0–127 | 0–13,388 |
| same | 1 | 2 | 923 / 931 | 1,940 | `703af92a1ea2c5bec5d5bbaaf7ea328c8d3dec0ccc6d4b9553227f175e5ead07` | 428 | `81:207 91:215 B1:2 C1:1 FF:3` | 215 / 207 | 2 | 37–61 | 0–127 | 0–6,834 |
| same | 1 | 3 | 2,871 / 2,879 | 2,876 | `d9b69d9a2f68e8b97ba7ff935b866fe20aaa366c5c373b756d0640e925985b6e` | 647 | `89:318 99:326 FF:3` | 326 / 318 | 10 | 66–70 | 0–120 | 0–6,174 |
| same | 1 | 4 | 5,755 / 5,763 | 1,693 | `bbe699f260728631620808a706f5764e2a0d69bcca7f8efaa2ceecbd0c3078ef` | 361 | `89:178 99:180 FF:3` | 180 / 178 | 10 | 72 | 0–127 | 0–4,224 |
| same | 1 | 5 | 7,456 / 7,464 | 1,206 | `30fb000ba9d9a1659526b6be0bb4bbf06657d5476a2870cc53d72c070e481714` | 271 | `89:133 99:135 FF:3` | 135 / 133 | 10 | 37–39 | 0–127 | 0–5,944 |
| same | 1 | 6 | 8,670 / 8,678 | 765 | `e53433613bfbac3770c0621f1570d43958fcced7e5364657401d98f48003b4bb` | 174 | `80:82 90:86 B0:2 C0:1 FF:3` | 86 / 82 | 1 | 73–85 | 0–127 | 0–10,176 |
| same | 1 | 7 | 9,443 / 9,451 | 602 | `9f5213ff5eb967d36fcd1233c41533676bf295800647612b981ec2daa593faf6` | 123 | `89:59 99:61 FF:3` | 61 / 59 | 10 | 72 | 0–127 | 0–59,520 |
| same | 1 | 8 | 10,053 / 10,061 | 760 | `dad23dadad85a3e45ffdb38bf010448ffb27cfb8d0d6cdd09e241a29bd70d57a` | 172 | `8E:82 9E:86 CE:1 FF:3` | 86 / 82 | 15 | 73–85 | 0–127 | 0–10,176 |
| same | 1 | 9 | 10,821 / 10,829 | 1,312 | `ac3b4c0b213099cef87660e1945f51eb5037e0bbbd2d77b6c04587f165f39e1d` | 289 | `89:143 99:143 FF:3` | 143 / 143 | 10 | 36–38 | 11–127 | 0–47,280 |

The two format-1 files share eight byte-identical complete payloads: Multitrack chunks 0–7 correspond by hash to Multi All chunks 0, 2, 3, 4, 5, 7, 8, and 9. This is only payload identity between exports; it does not label project tracks or user actions.

## Meta-event inventory

- Each file has one tempo meta-event at absolute file offset 56 (format 0) or 48 (format 1), delta 0, value 500,000 microseconds per quarter note.
- Every chunk has one track-name event and one end-of-track event.
- Format-0 track name: `Ode to Clarke`. Its end-of-track occurs at file offset 8,639, delta 3,002, absolute time 94,080.
- Both format-1 chunk 0 payloads are identical. They contain `Ode to Clarke`; their end-of-track event has delta and absolute time 94,080.
- Multitrack names by chunk are `Ode to Clarke`, `Track 2`, `sys100loops`, `Track 4`, `Track 5`, `Track 6`, `Track 3 #2`, and `Track 7`.
- Multi All names add `Track 1` and `Track 3` to that set. Each format-1 data chunk's end-of-track absolute time is 94,080; its event delta is included in the range table.

# Exact-match search results

## Exact complete payloads

No one of the 19 measured export payload occurrences was found as an exact byte sequence in the 203,422-byte project. Because eight payload values repeat between the two format-1 exports, this represents 11 distinct payload hashes, all absent from the project data fork.

## Exact event sequences

- No raw sequence spanning one or more complete parsed events and at least eight bytes was found in the project.
- No sequence of two or more normalized MIDI messages, with running status expanded and deltas omitted, was found.
- No sequence of two or more consecutive note events was found under any tested fixed representation: `status,note,velocity`; `channel,note,velocity`; zero-padded four-byte `status,note,velocity,00`; or four-byte big-endian 16-bit `note,velocity`.
- `MThd` and `MTrk` embedding was not used as an assumption. Complete payload and event-sequence absence is stronger evidence for this narrow test than isolated identifier searches.

These negative results show that direct embedding of these exported SMF payloads or their ordinary contiguous event encoding is unlikely in this project data fork. They do not show that MIDI-like project data is absent.

## Exact strings

`Ode to Clarke` occurs exactly once in the project, at `0x0002680d` (157,709). Related exact strings occur nearby:

| String | Offset |
|---|---:|
| `Meter Track` | `0x0002686f` (157,807) |
| `Tempo Track` | `0x000268e7` (157,927) |
| `Track 1` | `0x0002695f` (158,047) |
| `Track 2` | `0x000269d7` (158,167) |
| `sys100loops` | `0x00026a4f` (158,287) |
| `Track 4` | `0x00026ac7` (158,407) |
| `Track 5` | `0x00026b3f` (158,527) |
| `Track 3` | `0x00026bb7` (158,647) |
| `Track 6` | `0x00026c2f` (158,767) |
| `Track 3 #2` | `0x00026ca7` (158,887) |
| `Track 7` | `0x00026d1f` (159,007) |

Starting with `Meter Track`, these names begin exactly 120 (`0x78`) bytes apart. Further `Track` names continue at the same cadence. The string correlation is exact; the containing bytes are not thereby established as MIDI event records.

# Transformed-pattern search results

The search considered 1,053 distinct exported `status,note,velocity` triplets and 938 distinct `channel,note,velocity` triplets. Results were:

| Tested representation | Distinct reference patterns with at least one project occurrence | Classification |
|---|---:|---|
| `status,note,velocity` | 32 | partial, mostly coincidental candidates |
| reversed `velocity,note,status` | 53 | partial, mostly coincidental candidates |
| `status,note,velocity,00` | 0 | evidence not found |
| `00,status,note,velocity` | 1 | isolated; coincidental candidate |
| `channel,note,velocity` | 43 | partial, mostly coincidental candidates |
| `note,velocity,channel` | 13 | partial, mostly coincidental candidates |
| 16-bit big-endian note and velocity | 12 of 780 | isolated/common-value candidates |
| 16-bit little-endian note and velocity | 12 of 780 | isolated/common-value candidates |

None of these isolated matches extended to two consecutive reference events. They are scattered across the file and include common low-valued bytes. They do not support identifying an event region.

One raw triplet, `90 52 68`, occurs three times at `0x0002649f`, `0x0002656e`, and `0x000265e1`, within 878 bytes before the unique project-name string. It corresponds to one exported note-event triplet, but no adjacent exported event matched. This is a bounded partial-match candidate, not evidence of a MIDI event representation.

Fixed-width searches for the 572 distinct positive exported delta values produced many matches in both byte orders: 497 values as 16-bit big-endian, 470 as 16-bit little-endian, 399 as 32-bit big-endian, and 212 as 32-bit little-endian. Zero-rich binary data and small values make those single-value matches non-discriminating. No timing-field layout was established.

# Candidate Studio Vision regions

## Strong metadata-correlated candidate

The bounded range `0x00026800–0x00026e2f` (1,584 bytes) contains the unique `Ode to Clarke` string and the regular 120-byte sequence of `Meter Track`, `Tempo Track`, and export-related track names through `Track 8`. The requested exported names `Track 2`, `sys100loops`, `Track 4`, `Track 5`, `Track 6`, `Track 3 #2`, and `Track 7` are all present.

This region is strongly correlated with the named project at the metadata/record-layout level. It is not claimed to contain note events. Directly observed repeated features include names, padding bytes, and repeated local byte patterns; their field meanings remain unknown.

## Weak nearby partial-match candidate

The range `0x0002649f–0x000265e3` bounds the three occurrences of exact triplet `90 52 68`. Its proximity to the name region makes it worth retaining for later controlled comparison, but the absence of any two-event continuation means it is presently a weak candidate and may be coincidental.

## Non-candidate scattered matches

Other isolated three- and four-byte transformed matches occur throughout the project. Without repeatable grouping, locality, or controlled-save correlation, they are classified as coincidental candidates rather than bounded MIDI regions.

# Evidence that was not found

- No exact complete `MTrk` payload in the project.
- No raw contiguous exported event sequence of at least eight bytes.
- No two-message normalized sequence after removing deltas and expanding running status.
- No two-note grouping under the tested 3-byte or 4-byte fixed-width forms.
- No zero-padded `status,note,velocity,00` occurrence at all.
- No repeatable timing representation that separates plausible fields from common binary values.
- No evidence sufficient to generate a recovered MIDI file without guessing.

# Unknowns

- The curated authentic-project directory did not contain the requested basename; the exact reason the matching artifact resides elsewhere in the external research tree is unknown.
- The project serialization for event time, duration, note, velocity, channel, sequence ownership, and record boundaries remains unknown.
- Whether status bytes are stored, derived from another field, packed into flags, or absent from project note records is unknown.
- Whether timing uses ticks, absolute positions, duration/end values, scaling, fixed point, or another unit is unknown.
- Whether the weak `90 52 68` cluster is related to `Ode to Clarke` is unknown.
- The meanings of repeated fields surrounding the 120-byte name records are unknown.
- Export processing may add, reorder, normalize, or transform events; this spike did not infer those actions.
- A match against data-fork bytes does not address unavailable, encoded, compressed, or resource-fork representations.

# Conclusions

Useful progress was achieved in two forms. First, direct SMF byte embedding is unlikely: no complete payload, raw event sequence, normalized two-message sequence, or tested two-note fixed grouping was found. Second, a 1,584-byte project region was isolated with strong exact textual correlation and a regular 120-byte record cadence. That region supports the presence of `Ode to Clarke`-related project metadata but does not yet identify note-event storage.

The precise blocker is the missing mapping between the repeated 120-byte track-like records and the project's separate event-bearing data: specifically, no evidenced pointer/offset, record boundary, or encoding for time, note, velocity, and channel has yet been established.

# Single recommended next step

Perform a controlled one-note edit/save experiment on a copy of `newest STUFF`, changing exactly one known `Ode to Clarke` note while leaving track names and other settings unchanged, then compare the original and saved project byte-for-byte to isolate changed regions and test them against that note's known channel, note, velocity, start time, and duration. This is the narrowest experiment capable of establishing an event-record mapping without guessing.
