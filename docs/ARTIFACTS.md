# Artifact Inventory

This document catalogs original Opcode artifacts used as evidence during
reverse engineering. Entries should record direct observations and provenance
without inferring undocumented file formats or structures.

## Studio Vision Pro 4.5 installer

### Archive-level information

- Filename: `Studio Vision Pro 4.5 ƒ.zip`
- Source: Local research copy at
  `/Users/kurtheiden/Documents/Phoenix Research/Studio Vision Pro 4.5 ƒ.zip`
- Size: 13,630,338 bytes
- SHA-256: `1ed3b0de5a7a0ae84a608e2572a4bbbfc30b80f6b0c01721a411880388c4a16a`
- Archive format: ZIP; entries use stored and deflated compression methods.
- Entry count: 90 total central-directory entries: 84 file entries and 6
  directory entries.
- Expanded size reported by the ZIP directory: 31,267,853 bytes.
- Compressed entry data reported by the ZIP directory: 13,607,782 bytes.
- Integrity: `unzip -t` reported no errors in the compressed data.
- Finder Type: Unknown; not examined for the archive itself.
- Finder Creator: Unknown; not examined for the archive itself.
- Resource information: Unknown; not examined for the archive itself.
- Printable strings: Unknown; not examined for the archive itself.

### Archive structure

- Top-level archive paths: `Studio Vision Pro 4.5 ƒ/` and `__MACOSX/`.
- The distribution directory contains these immediate entries:
  `StudioVision`, `Documentation/`, `Information/`, `.DS_Store`, `Icon\r`,
  `Studio Vision Setup`, `Vision Grooves`, `VstPlugins/`,
  `Welcome to Studio Vision 4.5`, `ASIO Drivers/`,
  `Vision Effects Templates`, `Studio Vision Help`,
  `SV Audio-to-MIDI™ Templates`, `Optimized Acadias/`, and `Acadia`.
- The ZIP contains 45 paths below `__MACOSX/`, all of which have an
  AppleDouble-style `._` basename.
- The other 45 entries form the distribution tree: 39 file entries and 6
  explicit directory entries.
- No entry has the ZIP general-purpose UTF-8 filename flag set. The stored name
  bytes for the two non-ASCII names decode as UTF-8 to
  `Studio Vision Pro 4.5 ƒ` and `SV Audio-to-MIDI™ Templates`; tools that honor
  the absent flag may display those names differently.

### Significant contained entries

The categories below are based only on paths and filenames in the ZIP listing.
They do not establish classic Mac file kinds. Sizes are uncompressed sizes of
the listed primary ZIP entry data streams.

#### Application

- `StudioVision` — 3,159,081 bytes.

#### Setup utility

- `Studio Vision Setup` — 34,039 bytes.

#### Manuals

- `Documentation/Vision opPLUGS Manual.pdf` — 228,178 bytes.
- `Documentation/MIDI Reference Manual.pdf` — 6,062,074 bytes.
- `Documentation/QuickStart - Vis 4.5.pdf` — 362,458 bytes.
- `Documentation/Audio Reference Manual.pdf` — 7,220,565 bytes.
- `Documentation/What's New in 4.5.pdf` — 2,480,419 bytes.
- `Information/Authorizing Studio Vision.pdf` — 49,443 bytes.

#### Templates

- `Vision Effects Templates` — 26,902 bytes.
- `SV Audio-to-MIDI™ Templates` — 1,834 bytes.

#### Plug-ins

- `VstPlugins/` contains `opALIGN`, `opBOXDLY`, `opCHORUS`, `opCOMP`,
  `opCYCLE`, `opECHO`, `opFLANGE`, `opPANNER`, `opPLATE`, `opREZN8`,
  `opRING`, `opSQUASH`, and `PSP84`.
- Each listed plug-in has a zero-byte primary ZIP entry and a corresponding
  `__MACOSX` AppleDouble entry.
- `VstPlugins/VST Read Me` has a 588-byte primary ZIP entry.

#### Drivers

- `ASIO Drivers/DirectIO Driver` and
  `ASIO Drivers/Sound Manager Driver` each have a zero-byte primary ZIP entry
  and a corresponding `__MACOSX` AppleDouble entry.

#### Help and information

- `Studio Vision Help` has a zero-byte primary ZIP entry and a corresponding
  1,029,851-byte `__MACOSX` AppleDouble entry.
- `Welcome to Studio Vision 4.5` — 2,890 bytes.
- `Information/Tips for Studio Vision 4.5` — 8,201 bytes.
- `Information/Last Minute Notes` — 1,142 bytes.
- `Information/Welcome to Studio Vision 4.5` — 2,890 bytes.

#### Examples and other named content

- `Vision Grooves` — 300,483 bytes.
- `Acadia` — 261,795 bytes.
- `Optimized Acadias/` is an explicit directory entry; no child paths are
  listed beneath it.

### Unknown or unexamined contained-file metadata

- Finder types and creators for contained files are unknown and unexamined.
- Contained-file SHA-256 digests are unknown and unexamined.
- Printable strings in contained files are unknown and unexamined.
- The `__MACOSX` entries are present and their paths and ZIP entry sizes were
  observed, but their payload structures and contents were not examined.
  Their presence alone does not establish that they contain usable resource
  forks.
- Resource-fork contents and other resource information for contained files
  are unknown and unexamined.
- Notes: The original archive was inspected in place and was not added to or
  extracted into the repository.

## OMS 2.3.8 installer

- Filename: Unknown
- Source: Unknown
- Size: Unknown
- SHA-256: Unknown
- Finder Type: Unknown
- Finder Creator: Unknown
- Resource information: Unknown
- Printable strings: Unknown
- Notes: Unknown

## Studio Vision application

- Filename: Unknown
- Source: Unknown
- Size: Unknown
- SHA-256: Unknown
- Finder Type: Unknown
- Finder Creator: Unknown
- Resource information: Unknown
- Printable strings: Unknown
- Notes: Unknown

## Studio Vision Setup

- Filename: Unknown
- Source: Unknown
- Size: Unknown
- SHA-256: Unknown
- Finder Type: Unknown
- Finder Creator: Unknown
- Resource information: Unknown
- Printable strings: Unknown
- Notes: Unknown

## OMS applications

- Filename: Unknown
- Source: Unknown
- Size: Unknown
- SHA-256: Unknown
- Finder Type: Unknown
- Finder Creator: Unknown
- Resource information: Unknown
- Printable strings: Unknown
- Notes: Unknown

## Manuals

- Filename: Unknown
- Source: Unknown
- Size: Unknown
- SHA-256: Unknown
- Finder Type: Unknown
- Finder Creator: Unknown
- Resource information: Unknown
- Printable strings: Unknown
- Notes: Unknown

## Templates

- Filename: Unknown
- Source: Unknown
- Size: Unknown
- SHA-256: Unknown
- Finder Type: Unknown
- Finder Creator: Unknown
- Resource information: Unknown
- Printable strings: Unknown
- Notes: Unknown

## Plug-ins

- Filename: Unknown
- Source: Unknown
- Size: Unknown
- SHA-256: Unknown
- Finder Type: Unknown
- Finder Creator: Unknown
- Resource information: Unknown
- Printable strings: Unknown
- Notes: Unknown

## Example projects

- Filename: Unknown
- Source: Unknown
- Size: Unknown
- SHA-256: Unknown
- Finder Type: Unknown
- Finder Creator: Unknown
- Resource information: Unknown
- Printable strings: Unknown
- Notes: Unknown
