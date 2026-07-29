# Authentic Studio Vision Project Survey

This document records a read-only survey of the external Authentic Studio
Vision Projects collection. The collection contained 25 project files and one
`README.md` when surveyed on 2026-07-29. The README describes the collection
and is not counted as a project. No file in the collection was modified.

This is a catalog of filesystem facts and printable strings, not a parse of the
project format. A string's presence does not establish its purpose, and a
string's absence does not establish that the corresponding feature is absent.

## Method

- Sample identifiers follow the filename order shown below and remain tied to
  each file's SHA-256 digest. Future surveys should retain these assignments.
- Sizes and SHA-256 digests cover each complete data fork.
- Finder Type and Creator codes are the first eight bytes of the observed
  `com.apple.FinderInfo` extended attribute.
- Resource-fork presence was tested by enumerating extended attributes and
  checking for `com.apple.ResourceFork`.
- Printable strings are maximal runs of bytes from `0x20` through `0x7e`, with
  a minimum length of four bytes. Counts include coincidentally printable
  binary runs as well as human-readable text.
- Shannon entropy is measured over the complete data fork and reported in bits
  per byte. It describes byte distribution only.
- String categories below contain representative literal observations. Case,
  spelling, and punctuation are preserved where useful; obvious nontextual
  prefixes or suffixes are omitted from the displayed excerpts.
- A colon-delimited string is called path-like only as a description of its
  appearance. No filesystem reference structure is inferred.

## Sample registry and measurements

All 25 files are regular files without filename extensions. Every file has
`com.apple.FinderInfo` and `com.apple.quarantine`; no other extended attribute
was observed. No file currently has a `com.apple.ResourceFork` attribute. That
absence on these copies does not establish whether the originals had resource
forks before transfer.

| ID | Original filename | Size (bytes) | SHA-256 | Type / Creator | Printable strings | Entropy |
| --- | --- | ---: | --- | --- | ---: | ---: |
| project-001 | `7th level` | 53,190 | `ae9da4a8d181508aeb96f70bd1f13cae4f3085e1556df7a771ccca8b57cb06d3` | `MIDS` / `MIDA` | 2,958 | 5.886087 |
| project-002 | `COMIC BOOK` | 452,284 | `22a9e2589098b0d475f6e288eeebc9303bed644aa8ce09f9971a0060c9160486` | `MIDS` / `MIDA` | 15,409 | 6.547505 |
| project-003 | `DIANA` | 20,224 | `22550cd1fd2984e36c83db208150f57709af4ddefe90ac7c224951fa110dfc7f` | `MID2` / `MIDA` | 940 | 5.264849 |
| project-004 | `F2EQ4Z~T` | 187,360 | `d10854658659c1f2df7e1677eff37e81d8447dd152b66c561fdf701d66668c50` | `MIDS` / `MIDA` | 10,409 | 6.579512 |
| project-005 | `FH87SJ~D` | 114,574 | `9c9ece53da391f4b0d46f2fd07bbee978c898415ce7e7e6b58832f62d2c83cea` | `MID2` / `MIDA` | 6,832 | 5.939708 |
| project-006 | `New Dance Tracks` | 62,238 | `e2394569e865dd89c16138c5733eb2b86255eff1ef914ca5f54c6b8761743dd9` | `MID2` / `MIDA` | 3,010 | 6.044894 |
| project-007 | `Nothin compares` | 159,519 | `1a7fc9bb64711a8c87257123495972f0e6eeca6dccc70b340a126af2b9150e06` | `MID2` / `MIDA` | 11,741 | 5.673634 |
| project-008 | `SCHOOL PROJECTS` | 343,875 | `bfd4fa1208e2cd884ec51ccfb1131d5c02723d7c90acafc98bd89597e1a20331` | `MID2` / `MIDA` | 24,239 | 6.004170 |
| project-009 | `SMF files` | 881,556 | `058b8398c58089891e66c38696f35766df9babf0d1aef43f1086f90e3d319c71` | `MID2` / `MIDA` | 40,782 | 5.844710 |
| project-010 | `_2DC8B~C` | 84,944 | `14be8753c86482b5e51f1a213a00aa884674425676926e1c7d50866f60f51f31` | `MID2` / `MIDA` | 4,347 | 6.048368 |
| project-011 | `analog only` | 10,107 | `f3c945c764280bae7bee96c2faab716b61a0743f6949df034d231a52257ab101` | `MID2` / `MIDA` | 237 | 4.098292 |
| project-012 | `chris stuff` | 118,069 | `b15974c03840f2823aa9a21187e9536be28c203b4c28084886484ffe5891437c` | `MIDS` / `MIDA` | 6,523 | 5.855495 |
| project-013 | `cool pad progressions!` | 10,554 | `0c7e2e2ed39a74894cf74829c0b9c2e3403bcb2742ccf81acabfb559b3a16b97` | `MID2` / `MIDA` | 140 | 3.716483 |
| project-014 | `dominion` | 78,555 | `5a8888610926bf5e3313152c3915ec2ab95628aaf9f9ddae1dcf5c3d06079d1d` | `MIDS` / `MIDA` | 3,958 | 6.068521 |
| project-015 | `dominion with samples` | 134,204 | `4ec77addb3b39dfb5791d853cbb6225df9da30a95431633137e02a6811a8b34b` | `MIDS` / `MIDA` | 6,097 | 6.066688 |
| project-016 | `drum corps rythym` | 11,454 | `c203083e06c9bb6323eea3cbfa963936bcf9f6cda4bb88a5c7e201769915d500` | `MID2` / `MIDA` | 538 | 5.222834 |
| project-017 | `far away` | 124,777 | `52956a8eac862a66ffc9efbf880d25cfbfd03e5b80f250e4c2c4f89587daad94` | `MID2` / `MIDA` | 7,847 | 6.439500 |
| project-018 | `g-nome music` | 39,259 | `3a443fe4e924c318c0c8001a147ef430273a2567537be9ab652f9127b0e123b8` | `MID2` / `MIDA` | 2,607 | 5.356638 |
| project-019 | `minuet` | 30,536 | `f95c8e75dbce342fdbfbce9083d0d91fcbf55f9dae585c3bc129bce5a9bc9a4a` | `MID2` / `MIDA` | 1,035 | 5.715941 |
| project-020 | `movielike stuff` | 17,532 | `38605e20c032770b27c4ac2d0b5b3ae46450065896946c04f04a729f50471f2c` | `MID2` / `MIDA` | 518 | 4.802519 |
| project-021 | `rock&roll` | 17,087 | `28c4cd17f864374ef2cdffdf294d230ea5ac3a5d82a0e1986f6aa7a7f25407fd` | `MID2` / `MIDA` | 655 | 5.240044 |
| project-022 | `sampler xperiments` | 11,874 | `2436a6f13b4a463b38a717e24c76531c4adf9cb185f34bef4e2e80de2a357741` | `MID2` / `MIDA` | 307 | 4.604081 |
| project-023 | `school 2` | 68,474 | `c8fde14e95d2380a0da84c94b00bf9c08b304bcdb9e88da819130f8f580de7d0` | `MID2` / `MIDA` | 4,102 | 5.914965 |
| project-024 | `something new` | 17,718 | `452f7036ff7814b2631603f46ee2be559d3dfb1c41acfc56221df6b2add33761` | `MID2` / `MIDA` | 728 | 5.316794 |
| project-025 | `things i'll miss` | 16,297 | `5798103e4ef609dcd1b83c1dea245c00e7f2253047d5fdd31cbd8bc654dc9cda` | `MID2` / `MIDA` | 561 | 5.103845 |

## Per-project printable-string observations

“None observed” means that the stated scan found no clearly relevant printable
string; it is not evidence that the underlying project lacks that information.
Instrument examples are representative rather than exhaustive. Generic
`Audio-1` through `Audio-16` and `Audio/Video` strings are reported as setup
labels, not as proof that a project contains or references recorded audio.

### project-001 — `7th level`

- Devices: `JD-800`, `JD-990`, `JV-1080`, `JV-880`, `Sound Canvas`.
- Instruments: `Alto Sax`, `Drums`, `Hip Bass`, `Organ 3`, `Warm Pad`.
- OMS: `IAC Bus #1`; path-like `System Drive:System Folder:OMS Folder:Factory Names:General MIDI`.
- Bundle: `KURT SV3.0 Bundle 12-16-95`.
- Audio: generic `Audio-1`--`Audio-16`; `7th level Audio 1` occurs three times.
- Literal external-audio filename candidate: `7th level Audio 1`.
- Volumes and other paths: `System Drive`, `Hard Disk 2149`; path-like
  `Audio/Video:Opcode:Galaxy Plus Editors:KURT SV3.0 Bundle 12-16-95`.
- Unusual observation: `Hard Disk 2149:@` occurs near a later
  `7th level Audio 1` occurrence.

### project-002 — `COMIC BOOK`

- Devices: `JD-800`, `JD-990 w/Vintage`, `JV-1080`, `JV-880`, `Sound Canvas`.
- Instruments: `BritelowBass`, `Brass 1`, `Strings Arco`, `Sweep Pad`,
  `Velo Syn Strings`.
- OMS: `IAC Bus #1`; path-like `System Drive:System Folder:OMS Folder:Factory Names:General MIDI`.
- Bundle: `KURT SV3.0 Bundle 12-16-95`.
- Audio: generic `Audio-1`--`Audio-16`; `sweeper audio take 3 normw/verb`;
  repeated `analog take` names; `take 1`, `take 2`, and `take 2.5`.
- Literal external-audio filename candidates: `analog take 1.norm`,
  `analog take 2`, `analog take 2.5.norm`, `analog take 3.norm`, and
  `sweeper audio take 3 normw/verb`.
- Volumes and other paths: `System Drive`, `Barracuda 2GB`; the standard
  `Audio/Video:Opcode:Galaxy Plus Editors:...` path-like string.
- Unusual observation: at 452,284 bytes this is the second-largest sample; it
  also contains `THE DECISION MIX`, `THE DECISION MIX2`, `Opcode VisionMIX`,
  and `Opcode VisionMIX2`.

### project-003 — `DIANA`

- Devices: `JD-800`, `JD-990`, `JV-1080`, `JV-880`, `Sound Canvas`.
- Instruments: `Bass&Lead`.
- OMS: path-like `System Drive:System Folder:OMS Folder:Factory Names:General MIDI`.
- Bundle: `KURT SV3.0 Bundle 12-16-95`.
- Audio: generic `Audio/Video` setup string only; no project-specific audio
  filename observed.
- Literal external-audio filename candidates: none observed.
- Volumes and other paths: `System Drive`; the standard `Audio/Video:Opcode:Galaxy Plus Editors:...` path-like string.
- Unusual observation: several device strings have the literal suffix
  `528MIDA`, including `JD-990 w/Vintage528MIDA`.

### project-004 — `F2EQ4Z~T`

- Devices: `JD-800`, `JD-990 w/Vintage`, `JV-1080`, `JV-880`, `Sound Canvas`.
- Instruments: `Filter Bass`, `Piano 3`, `Syn. Strings1`, `Tenor Sax`,
  `wah guitar`.
- OMS: `IAC Bus #1`, `OMS Applications`, `OmsP`, `OmsPoSPE`; path-like
  `System Drive:Opcode:OMS Applications:Studio 5 Patches`.
- Bundle: `KURT SV3.0 Bundle 12-16-95`.
- Audio: generic `Audio-1`--`Audio-16` and `Audio/Video` strings only.
- Literal external-audio filename candidates: none observed.
- Volumes and other paths: `System Drive`; the standard `Audio/Video:Opcode:Galaxy Plus Editors:...` path-like string.
- Unusual observation: at 6.579512 bits/byte, it has the highest measured
  entropy in this collection.

### project-005 — `FH87SJ~D`

- Devices: `JD-800`, `Roland JD-990`, `Roland JV-880`, `JV-1080`,
  `Sound Canvas`.
- Instruments: `12 Strings`, `C-Thumpbass1`, `MIDIed Grand`, `S-10Choir`,
  `Toms`.
- OMS: `IAC Bus #1`; path-like `FWB 8 GB:System Folder:OMS Folder:Factory Names:Roland JD-990` and corresponding `Roland JV-880` string.
- Bundle: `KURT SV3.0 Bundle 12-16-95`.
- Audio: generic `Audio/Video` setup string only; no project-specific audio
  filename observed.
- Literal external-audio filename candidates: none observed.
- Volumes and other paths: `FWB 8 GB`, `System Drive`; the standard
  `Audio/Video:Opcode:Galaxy Plus Editors:...` path-like string.
- Unusual observation: the longest printable run is 1,082 bytes, substantially
  longer than its human-readable names and visibly dominated by repeated
  punctuation and letters.

### project-006 — `New Dance Tracks`

- Devices: `JD-800`, `JD-990 w/Vintage`, `JV-1080`, `JV-880`, `Sound Canvas`.
- Instruments: `64voicePiano`, `Big Pad Swp`, `Brass Vel.Fall`, `Multi Bass`,
  `XXLargeSynth08`.
- OMS: `IAC Bus #1`; path-like `System Drive:System Folder:OMS Folder:Factory Names:General MIDI`.
- Bundle: `KURT SV3.0 Bundle 12-16-95`.
- Audio: generic `Audio/Video` setup string only; no project-specific audio
  filename observed.
- Literal external-audio filename candidates: none observed.
- Volumes and other paths: `System Drive`; the standard `Audio/Video:Opcode:Galaxy Plus Editors:...` path-like string.
- Unusual observation: instrument-like names include `XX-LG Synth21`,
  `XXlg Synth24`, and `xxl-synth13` with differing capitalization.

### project-007 — `Nothin compares`

- Devices: `JD-990`.
- Instruments: `C1-Lead Vocal`, `C2-Bass`, `C3-Acou. Piano`, `C4-Strings`,
  `C5-Guitar`.
- OMS: no clearly OMS-related string observed.
- Bundle: no bundle-related string observed.
- Audio: track-like strings include `C1-Lead Vocal` through
  `C10-Drums & Per`; no project-specific audio filename observed.
- Literal external-audio filename candidates: none observed.
- Volumes and paths: `Macintosh HD` occurs within the printable run
  `Macintosh HDn AV`; no clearly path-like string observed.
- Unusual observation: `Strings 1 sample` and the vocal/percussion track-like
  names distinguish its visible vocabulary from the shared setup-name groups.

### project-008 — `SCHOOL PROJECTS`

- Devices: `JD-800`, `JD-990 w/Vintage`, `JV-1080`, `JV-880`, `Sound Canvas`.
- Instruments: `Baritone Sax`, `Bassoon`, `Brass ff 2`, `Pan Flute`,
  `Slap Bass 2`.
- OMS: `IAC Bus #1`; path-like `System Drive:System Folder:OMS Folder:Factory Names:General MIDI`.
- Bundle: `KURT SV3.0 Bundle 12-16-95`.
- Audio: generic `Audio/Video` and vocal/percussion track-like strings; no
  project-specific audio filename observed.
- Literal external-audio filename candidates: none observed.
- Volumes and other paths: `System Drive`; the standard `Audio/Video:Opcode:Galaxy Plus Editors:...` path-like string.
- Unusual observation: 24,239 printable runs are the second-highest count in
  the collection.

### project-009 — `SMF files`

- Devices: `JD-800`, `JD-990`, `JV-1080`, `JV-880`, `Sound Canvas`.
- Instruments: `Acoustic Bass`, `Chorused Piano`, `Distortion Guitar`,
  `Synth Bass 1`, `Vibraphone / Pad`.
- OMS: `IAC Bus #1`; path-like `Macintosh HD:System Folder:OMS Folder:Factory Names:General MIDI`.
- Bundle: `KURT SV3.0 Bundle 12-16-95`.
- Audio: no clearly audio-related setup or project-specific filename string
  observed.
- Literal external-audio filename candidates: none observed.
- Volumes and other paths: `Macintosh HD`; path-like
  `Macintosh HD:Opcode:KURT SV3.0 Bundle 12-16-95`.
- Unusual observation: it is the largest sample (881,556 bytes), has the most
  printable runs (40,782), and has an 11,120-byte coincidentally printable run.

### project-010 — `_2DC8B~C`

- Devices: `JD-800`, `JD-990`, `JV-1080`, `JV-880`, `Sound Canvas`.
- Instruments: `unaamable piano`.
- OMS: `IAC Bus #1`; path-like `Macintosh HD:System Folder:OMS Folder:Factory Names:General MIDI`.
- Bundle: `KURT SV3.0 Bundle 12-16-95`.
- Audio: no clearly audio-related setup or project-specific filename string
  observed.
- Literal external-audio filename candidates: none observed.
- Volumes and other paths: `Macintosh HD`; path-like
  `Macintosh HD:Opcode:KURT SV3.0 Bundle 12-16-95`.
- Unusual observation: the original filename resembles an 8.3-style generated
  name; its provenance is not established by this survey.

### project-011 — `analog only`

- Devices: `JD-800`, `JD-990 w/Vintage`, `JV-1080`, `JV-880`, `Sound Canvas`.
- Instruments: `J-60 bass` and `j-60 bass`.
- OMS: `IAC Bus #1`; path-like `System Drive:System Folder:OMS Folder:Factory Names:General MIDI`.
- Bundle: `KURT SV3.0 Bundle 12-16-95`.
- Audio: generic `Audio/Video` setup string only; no project-specific audio
  filename observed.
- Literal external-audio filename candidates: none observed.
- Volumes and other paths: `System Drive`; the standard `Audio/Video:Opcode:Galaxy Plus Editors:...` path-like string.
- Unusual observation: at 10,107 bytes, it is the smallest sample.

### project-012 — `chris stuff`

- Devices: `JD-800`, `JD-990`, `JV-1080`, `JV-880`, `Sound Canvas`.
- Instruments: `Deep Bass`, `Sharp Bass`.
- OMS: `IAC Bus #1`; path-like `Macintosh HD:System Folder:OMS Folder:Factory Names:General MIDI`.
- Bundle: `KURT SV3.0 Bundle 12-16-95`.
- Audio: generic `Audio-1s`--`Audio-9s` and `Audio-10`--`Audio-16`;
  `chris stuff Audio 1` occurs repeatedly.
- Literal external-audio filename candidate: `chris stuff Audio 1`.
- Volumes and other paths: `Macintosh HD`, `Hard Disk 2149`, `HILLARY`;
  path-like `Hard Disk 2149:HILLARY:Opcode:KURT SV3.0 Bundle 12-16-95`.
- Unusual observation: both `chris stuff Audio 1` and
  `chris stuff Audio 1@` occur.

### project-013 — `cool pad progressions!`

- Devices: `JD-800`, `Roland JD-990`, `Roland JV-880`, `JV-1080`,
  `Sound Canvas`.
- Instruments: no project-specific instrument name was clearly distinguished
  from the observed device/setup names.
- OMS: `IAC Bus #1`; path-like `FWB 8 GB:System Folder:OMS Folder:Factory Names:Roland JD-990` and corresponding `Roland JV-880` string.
- Bundle: `KURT SV3.0 Bundle 12-16-95`.
- Audio: generic `Audio/Video` setup string only; no project-specific audio
  filename observed.
- Literal external-audio filename candidates: none observed.
- Volumes and other paths: `FWB 8 GB`, `System Drive`; the standard
  `Audio/Video:Opcode:Galaxy Plus Editors:...` path-like string.
- Unusual observation: it has the fewest printable runs (140) and the lowest
  entropy (3.716483 bits/byte) in the collection.

### project-014 — `dominion`

- Devices: `JD-800`, `JD-990`, `JV-1080`, `JV-880`, `Sound Canvas`.
- Instruments: `Bark Bass`, `JP-8 Strings`, `Multi Bass`, `Polysynth`,
  `Synth Bass1`.
- OMS: `IAC Bus #1`; path-like `Macintosh HD:System Folder:OMS Folder:Factory Names:General MIDI`.
- Bundle: `KURT SV3.0 Bundle 12-16-95`.
- Audio: generic `Audio-1q`--`Audio-9q` and `Audio-10`--`Audio-16`;
  `dominion Audio 1` and `dominion Audio 2`.
- Literal external-audio filename candidates: `dominion Audio 1` and
  `dominion Audio 2`.
- Volumes and other paths: `Macintosh HD`, `Hard Disk 2149`, `Hillary`;
  path-like `Hard Disk 2149:Music:Opcode:Studio Vision Pro:KURT SV3.0 Bundle 12-16-95`.
- Unusual observation: `Studio Vision Pro` occurs three times in the printable
  scan, including within the path-like string.

### project-015 — `dominion with samples`

- Devices: `Emulator III`, `JD-800`, `JD-990 w/Vintage`, `JV-1080`,
  `SampleCell #2`.
- Instruments: `Big Apple Bass`, `ORBit Pad`, `PILE DRIVER`, `Techno Bass`,
  `Wiry Sync Bass`.
- OMS: path-like `Backup 2GB:System Folder:OMS Folder:Factory Names:General MIDI`.
- Bundle: `KURT SV3.0 Bundle 12-16-95`.
- Audio: generic `Audio-1`--`Audio-16`; `SampleCell #2` occurs repeatedly;
  `mercwork` occurs twice.
- Literal external-audio filename candidate: `mercwork` is a literal name near
  later storage-related strings, but this survey cannot establish its role.
- Volumes and other paths: `Backup 2GB`, `Hillary`; the standard
  `Audio/Video:Opcode:Galaxy Plus Editors:...` path-like string.
- Unusual observation: the printable run `mercworkrive49Pro` occurs after
  `mercwork` and `Hillary:`; it is recorded without interpretation.

### project-016 — `drum corps rythym`

- Devices: `JD-990`, `Sound Canvas`.
- Instruments: `Jarre sound`.
- OMS: no clearly OMS-related string observed.
- Bundle: no bundle-related string observed.
- Audio: no clearly audio-related string observed.
- Literal external-audio filename candidates: none observed.
- Volumes and paths: `Macintosh HD` occurs within the printable run
  `Macintosh HDn AV`; no clearly path-like string observed.
- Unusual observation: the filename preserves the original spelling
  `rythym`; it has 538 printable runs in 11,454 bytes.

### project-017 — `far away`

- Devices: `JD-800`, `JD-990 w/Vintage`, `JV-1080`, `JV-880`, `Sound Canvas`.
- Instruments: `64voicePiano`, `Filter Bass`, `ORBit Pad`, `Slap Bass 3`,
  `Warm Pad`.
- OMS: `IAC Bus #1`, `OMS Applications`, `OmsP`, `OmsPoSPE`; path-like
  `System Drive:Opcode:OMS Applications:Studio 5 Patches`.
- Bundle: `KURT SV3.0 Bundle 12-16-95`.
- Audio: generic `Audio/Video` setup string only; no project-specific audio
  filename observed.
- Literal external-audio filename candidates: none observed.
- Volumes and other paths: `System Drive`; path-like
  `System Drive:System Folder:OMS Folder:Factory Names:General MIDI` and the
  standard `Audio/Video:Opcode:Galaxy Plus Editors:...` string.
- Unusual observation: `OMS Applications`, `OmsP`, and `OmsPoSPE` all occur.

### project-018 — `g-nome music`

- Devices: `JD-800`, `Roland JD-990`, `Roland JV-880`, `JV-1080`,
  `Sound Canvas`.
- Instruments: no project-specific instrument name was clearly distinguished
  from the observed device/setup names.
- OMS: `IAC Bus #1`; path-like `FWB 8 GB:System Folder:OMS Folder:Factory Names:Roland JD-990` and corresponding `Roland JV-880` string.
- Bundle: `KURT SV3.0 Bundle 12-16-95`.
- Audio: generic `Audio/Video` setup string only; no project-specific audio
  filename observed.
- Literal external-audio filename candidates: none observed.
- Volumes and other paths: `FWB 8 GB`, `System Drive`; the standard
  `Audio/Video:Opcode:Galaxy Plus Editors:...` path-like string.
- Unusual observation: the longest printable run is 325 bytes and is visibly
  dominated by repeating `OO` and punctuation sequences.

### project-019 — `minuet`

- Devices: `JD-990`, `Sound Canvas`.
- Instruments: `fretless bass`, `loud bass`, `piano`.
- OMS: no clearly OMS-related string observed.
- Bundle: no bundle-related string observed.
- Audio: no clearly audio-related string observed.
- Literal external-audio filename candidates: none observed.
- Volumes and paths: `Macintosh HD` occurs within the printable run
  `Macintosh HDn AV`; no clearly path-like string observed.
- Unusual observation: `Kurt's SV Prog&notnames` is the longest printable run,
  at 23 bytes.

### project-020 — `movielike stuff`

- Devices: `JD-800`, `Roland JD-990`, `Roland JV-1080`, `Roland JV-880`,
  `Sound Canvas`.
- Instruments: `Killer Pad`, `Strings`.
- OMS: `OMS Applications`, `OmsP`, `OmsPoSPE`; path-like
  `System Drive:Opcode:OMS Applications:Studio 5 Patches`.
- Bundle: `KURT SV3.0 Bundle 12-16-95`.
- Audio: generic `Audio/Video` setup string only; no project-specific audio
  filename observed.
- Literal external-audio filename candidates: none observed.
- Volumes and other paths: `System Drive`; several path-like `System Drive:System Folder:OMS Folder:Factory Names:...` strings and the standard `Audio/Video:Opcode:Galaxy Plus Editors:...` string.
- Unusual observation: separate path-like strings name `Roland JD-990`,
  `Roland JV-1080`, and `Roland JV-880`.

### project-021 — `rock&roll`

- Devices: `JD-800`, `JD-990`, `JV-1080`, `JV-880`, `Sound Canvas`.
- Instruments: `Piano bass 1`, `Piano bass 2`, `Piano lead 1`,
  `Piano lead 2`.
- OMS: `IAC Bus #1`; path-like `Macintosh HD:System Folder:OMS Folder:Factory Names:General MIDI`.
- Bundle: `KURT SV3.0 Bundle 12-16-95`.
- Audio: no clearly audio-related string observed.
- Literal external-audio filename candidates: none observed.
- Volumes and other paths: `Macintosh HD`; path-like
  `Macintosh HD:Opcode:KURT SV3.0 Bundle 12-16-95`.
- Unusual observation: four numbered piano/bass/lead names occur as a related
  set.

### project-022 — `sampler xperiments`

- Devices: `JD-800`, `JD-990`, `JV-1080`, `JV-880`, `Sound Canvas`.
- Instruments: no project-specific instrument name was clearly distinguished
  from the observed device/setup names.
- OMS: `IAC Bus #1`; path-like `Macintosh HD:System Folder:OMS Folder:Factory Names:General MIDI`.
- Bundle: `KURT SV3.0 Bundle 12-16-95`.
- Audio: no clearly audio-related string observed.
- Literal external-audio filename candidates: none observed.
- Volumes and other paths: `Macintosh HD`; path-like
  `Macintosh HD:Opcode:KURT SV3.0 Bundle 12-16-95`.
- Unusual observation: despite the original filename, the printable scan does
  not contain a project-specific sample or audio filename.

### project-023 — `school 2`

- Devices: `JD-800`, `Roland JD-990`, `Roland JV-880`, `JV-1080`,
  `Sound Canvas`.
- Instruments: `bass & snare`, `brass falls`, `c-a23 loud bass`,
  `i17 warm pad`, `panning drums`.
- OMS: `IAC Bus #1`; path-like `FWB 8 GB:System Folder:OMS Folder:Factory Names:Roland JD-990` and corresponding `Roland JV-880` string.
- Bundle: `KURT SV3.0 Bundle 12-16-95`.
- Audio: generic `Audio/Video` setup string only; no project-specific audio
  filename observed.
- Literal external-audio filename candidates: none observed.
- Volumes and other paths: `FWB 8 GB`, `System Drive`; the standard
  `Audio/Video:Opcode:Galaxy Plus Editors:...` path-like string.
- Unusual observation: lower-case project strings include `piano`, `strings`,
  and `panning drums`.

### project-024 — `something new`

- Devices: `JD-800`, `JD-990 w/Vintage`, `JV-1080`, `JV-880`, `Sound Canvas`.
- Instruments: `Techno Bass`.
- OMS: `IAC Bus #1`, `OMS Applications`, `OmsP`, `OmsPoSPE`; path-like
  `System Drive:Opcode:OMS Applications:Studio 5 Patches`.
- Bundle: `KURT SV3.0 Bundle 12-16-95`.
- Audio: generic `Audio/Video` setup string only; no project-specific audio
  filename observed.
- Literal external-audio filename candidates: none observed.
- Volumes and other paths: `System Drive`; path-like
  `System Drive:System Folder:OMS Folder:Factory Names:General MIDI` and the
  standard `Audio/Video:Opcode:Galaxy Plus Editors:...` string.
- Unusual observation: both the standard OMS Folder path and the OMS
  Applications/Studio 5 Patches path occur.

### project-025 — `things i'll miss`

- Devices: `JD-800`, `JD-990 w/Vintage`, `JV-1080`, `JV-880`, `Sound Canvas`.
- Instruments: `Organ 1`, `Piano 3`, `Polysynth`, `Synth Bass 2`.
- OMS: `IAC Bus #1`, `OMS Applications`, `OmsP`, `OmsPoSPE`; path-like
  `System Drive:Opcode:OMS Applications:Studio 5 Patches`.
- Bundle: `KURT SV3.0 Bundle 12-16-95`.
- Audio: generic `Audio/Video` setup string only; no project-specific audio
  filename observed.
- Literal external-audio filename candidates: none observed.
- Volumes and other paths: `System Drive`; path-like
  `System Drive:System Folder:OMS Folder:Factory Names:General MIDI` and the
  standard `Audio/Video:Opcode:Galaxy Plus Editors:...` string.
- Unusual observation: the visible setup vocabulary includes both
  `Studio 5 Patches` and `Studio Patches`.

## Comparison

### Observations common across all projects

- Every data fork is nonempty, and every SHA-256 digest is unique within this
  collection.
- Every file has Finder Creator `MIDA`, a Finder Type beginning with `MID`, a
  32-byte `com.apple.FinderInfo` attribute, and the same printable quarantine
  value, `0081;00000000;;`.
- No inspected copy currently exposes a `com.apple.ResourceFork` attribute.
- Every data fork contains the literal substrings `JD-990`, `Meter Track`, and
  `Tempo Track` in the minimum-four-byte printable scan.
- Every file contains both human-readable strings and long or short printable
  runs that are not clearly natural-language text.

These shared observations do not establish file signatures, required fields,
or binary structures.

### Observations present only in some projects

- Finder Type is `MID2` in 19 samples and `MIDS` in project-001, project-002,
  project-004, project-012, project-014, and project-015.
- The exact substring `OMS` occurs in 22 samples. It was not observed in
  project-007, project-016, or project-019.
- `KURT SV3.0 Bundle 12-16-95` occurs in the same 22-sample subset and was not
  observed in project-007, project-016, or project-019.
- `OMS Applications` and `Studio 5 Patches` occur in project-004, project-017,
  project-020, project-024, and project-025.
- `FWB 8 GB` path-like strings occur in project-005, project-013, project-018,
  and project-023. `Macintosh HD`, `System Drive`, `Hard Disk 2149`,
  `Barracuda 2GB`, and `Backup 2GB` are other observed volume-like names.
- `Studio Vision Pro` was directly observed in project-014. This survey does
  not establish which application version wrote any other sample.
- Printable-string counts range from 140 (project-013) to 40,782
  (project-009); measured entropy ranges from 3.716483 (project-013) to
  6.579512 bits/byte (project-004).

### Digital-audio references and MIDI-only candidates

Project-001, project-002, project-012, and project-014 contain project-specific
audio-name or take-name strings that distinguish them from most other surveyed
samples. These observations make them strong candidates for future
investigation of digital-audio references, but do not by themselves establish
the meaning or purpose of those strings. Project-002 has the most extensive
set of such observations: multiple repeated `analog take` names, a
`sweeper audio` name, mix names, and a `Barracuda 2GB` volume-like string.

Project-015 contains generic audio-slot strings, repeated `SampleCell #2`, the
literal `mercwork`, and storage-related strings. These are relevant to sampling
or audio research, but they do not by themselves establish that `mercwork` is
an external audio file or that the project references recorded audio.
Project-004 has `MIDS` Finder Type and generic audio slots but no observed
project-specific audio filename. The meaning of `MIDS` is not inferred here.

The remaining 19 samples are MIDI-only candidates for the limited purpose of
future comparison because their printable scans contain device, instrument,
track, or MIDI-oriented names but no project-specific external-audio filename.
This is not proof that they are MIDI-only. In particular, generic `Audio/Video`
or vocal track names are insufficient to establish a digital-audio reference,
and unprintable data could contain relevant information.

### Candidates for future parser development

These choices are based only on measured contrasts and directly visible
strings, not on inferred layouts:

- project-011 and project-013 are compact `MID2`/`MIDA` samples with low
  printable-string counts and low entropy, making them bounded candidates for
  early format-neutral comparison.
- project-001 and project-014 combine `MIDS`/`MIDA` metadata with repeated,
  project-specific audio-name strings.
- project-002 is the strongest candidate for audio-reference investigation
  because it contains many repeated literal take names, mix names, and a
  volume-like name.
- project-014 and project-015 share `dominion` in their original filenames but
  differ in size and visible device/audio vocabulary. Their actual historical
  relationship remains unknown, so any future comparison must first establish
  provenance or treat them simply as two independent samples.
- project-009 is the largest sample and has the most printable runs; it is a
  useful stress case, but its scale makes it less bounded than the small
  candidates.
- The six `MIDS`/`MIDA` samples and the 19 `MID2`/`MIDA` samples provide two
  directly observed metadata groups for behavior comparisons without assigning
  undocumented meanings to the Type codes.

### Explicit unknowns requiring additional investigation

- The Studio Vision or Studio Vision Pro version that created or last saved
  each project is unknown, except that project-014 literally contains the text
  `Studio Vision Pro`; that text alone does not prove authorship or version.
- Whether any project still opens in its original application has not been
  tested.
- The meanings, locations, and relationships of all binary data are unknown.
  This survey identifies no headers, records, fields, sections, offsets, or
  parser boundaries.
- The meanings of Finder Type `MID2` and `MIDS` are not established here.
- The observed Finder Type split between `MID2` and `MIDS` may prove
  significant, but this survey does not establish what those values represent
  or whether they correspond to project type, application version, or any
  other distinction.
- The original presence or absence of resource forks and other metadata before
  the files were copied is unknown.
- It is unknown which human-readable strings are active project data, embedded
  setup data, cached names, unused names, or coincidentally printable bytes.
- The existence and location of external audio files named by the observed
  strings have not been verified.
- No content-based test established that any sample is MIDI-only. The
  MIDI-only labels above are candidates defined by absence of project-specific
  audio filenames in this printable-string survey.
- The apparent volume names and colon-delimited strings have not been tested
  against a classic Mac OS filesystem or original Studio Vision behavior.
- The provenance and original names of `F2EQ4Z~T`, `FH87SJ~D`, and `_2DC8B~C`
  are unknown.
- Entropy and printable-string counts do not establish compression,
  encryption, corruption, complexity, or semantic content.
