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

### Observations

- Filename: `Install OMS 2.3.8`
- Source: Local research copy at
  `/Users/kurtheiden/Documents/Phoenix Research/Install OMS 2.3.8`
- Filesystem object: Regular file.
- File size and data-fork size: 2,485,279 bytes.
- Data-fork SHA-256:
  `34d3f759284941ca4ecc91fb33902087140accbcd37c4677f94cd58ee68f826d`
- Finder Type: `APPL` (`41 50 50 4c`).
- Finder Creator: `VIS3` (`56 49 53 33`).
- The Finder information is stored in a 32-byte `com.apple.FinderInfo`
  extended attribute.
- The first four data-fork bytes are ASCII `SVCT` (`53 56 43 54`). Generic
  `file` reports the data fork only as `data`.
- A `com.apple.ResourceFork` extended attribute is present and is 295,885
  bytes long.
- Resource-fork SHA-256:
  `a66e9c96711e24fe6a27b5981efcaed66eea22a1f734ed85ac2c5a9991c94188`
- `DeRez` parsed the resource fork and reported two `vers` resources naming
  `Installer VISE 5.5.1` and `InstallerVISE 5.5.1`, with a copyright string for
  MindVision Software covering 1991–1998.
- The parsed resource map includes classic Mac application resources such as
  `CODE`, `BNDL`, `FREF`, `SIZE`, `MENU`, `DLOG`, `DITL`, icon resources, and
  an owner resource of type `VIS3`. It also includes a resource of type `XOMS`
  named `OMSVISE`.
- Printable resource-fork strings include `OMS Folder`, `OMS Preferences`,
  `OMS Current State`, `Install OMS 2.3.8`, and an instruction to click the
  Install button to install OMS 2.3.8.
- `bsdtar` reports `Unrecognized archive format` when asked to list the data
  fork.

### Tentative format identification

- The observed Finder metadata, version resources, and resource map are
  consistent with an Installer VISE application. The internal organization of
  the data fork remains unexamined.
- This is not identified as a ZIP archive or disk image. The observation does
  not establish the internal format of the `SVCT` data or prove that the
  installer payload is self-extracting.

### Safely observable structure

- The resource fork can be listed non-destructively with `DeRez`.
- No safely listable top-level payload structure was identified in the data
  fork with the currently available generic archive tool.
- No contained OMS applications, utilities, drivers, documentation, or
  examples were enumerated. The printable names above are evidence of strings
  in the resource fork, not proof of contained files or their paths.

### Preservation-container observations

- The artifact is preserved on the current filesystem as a data fork plus
  Finder-information and resource-fork extended attributes.
- The inspected data-fork header is not a MacBinary or BinHex header and does
  not begin with the standard `SIT!` or `StuffIt` signatures.
- No AppleDouble container was used to access the observed Finder information
  or resource fork; both are directly available as extended attributes.
- No adjacent `._Install OMS 2.3.8` AppleDouble file is present at the source
  location.
- These header observations do not rule out nested or embedded structures in
  the unexamined data.

### Unknowns and unexamined metadata

- The internal structure and compression, if any, of the `SVCT` data fork are
  unknown and unexamined.
- The installer was not executed, so its install choices, destinations,
  contained-file inventory, and runtime behavior are unknown.
- No appropriate non-executing integrity checker for the Installer VISE data
  was available. Successful `DeRez` parsing establishes only that the resource
  map was readable; it is not a complete installer integrity test.
- The semantic roles of resources other than the directly named and typed
  observations above are unexamined.
- Notes: The installer was inspected in place and was not renamed, modified,
  executed, installed, extracted, recompressed, or added to the repository.

## Studio Vision application

### Observations

- Filename: `StudioVision.app`
- Source: Local research copy at
  `/Users/kurtheiden/Documents/Phoenix Research/Studio Vision Pro 4.5 ƒ/StudioVision.app`
- Filesystem object: Regular file, despite the `.app` suffix; it is not a
  directory bundle.
- File size and data-fork size: 3,159,081 bytes.
- Data-fork SHA-256:
  `24b58a168086a65776f715a7c92c1295818d6b42ebbd406a7a9c583bd1c0eb4b`
- Finder Type: `APPL` (`41 50 50 4c`).
- Finder Creator: `MIDA` (`4d 49 44 41`).
- The Finder information is stored in a 32-byte `com.apple.FinderInfo`
  extended attribute.
- A `com.apple.ResourceFork` extended attribute is present and is 2,112,471
  bytes long.
- Resource-fork SHA-256:
  `718df2f317713a51101fe64cf113ab03b479103e053dc49ac42ec00a4ce4676e`

#### Version and executable observations

- The `vers` resource reports version `4.5.1` and the string
  `4.5.1, Copyright © Opcode Systems, Inc. 1985-99`.
- `file` identifies the data fork as a PowerPC PEF executable.
- The data fork begins `Joy!peffpwpc` in ASCII.
- The `cfrg` resource contains the architecture code `pwpc` and the name
  `StudioVision PPC`.
- No `CODE` resource is present in the resource map.

#### Resource types

`DeRez` parsed the resource fork and reported 65 distinct resource types:

- `ABSz`, `ALRT`, `BNDL`, `CDEF`, `CNTL`, `CURS`, `Cmd#`, `Cpic`, `DITL`,
  `DLGX`, `DLOG`, `ETYP`, `ErrS`, `FNAM`, `FOND`, `FREF`, `FTBL`, `Fnt#`,
  `GRAM`, `Gpic`, `ICN#`, `KCHR`, `LDEF`, `LMAP`, `MDEF`, `MENU`, `MIDA`,
  `Mcmd`, `MdSw`, `NFNT`, `OpHk`, `PICT`, `PMAP`, `SICN`, `SIZE`, `STR `,
  `STR#`, `SVSM`, `TMPL`, `WDEF`, `WIND`, `actb`, `bmap`, `cctb`, `cfrg`,
  `cicn`, `clut`, `dctb`, `iSNP`, `icl4`, `icl8`, `icm#`, `icm4`, `icm8`,
  `ics#`, `ics4`, `ics8`, `ictb`, `mctb`, `odcs`, `ppat`, `snd `, `sysz`,
  `vers`, and `wctb`.

#### Directly observed printable strings

The following are representative literal strings observed in the data fork or
resource fork. They are recorded without interpretation:

- Studio Vision and Vision: `About Studio Vision Pro`, `Studio Vision`,
  `Vision`, `Vision Setup`, and `StudioVision PPC`.
- OMS: `Initializing OMS`, `Error starting OMS`, `OMS missing`,
  `Can't Use OMS`, and `AV OMS Driver`.
- MIDI: `MIDI Devices`, `MIDIKeys Window`, `MIDI File`, `Importing MIDI`,
  `Export MIDI File...`, and `Start MIDI Time Code`.
- Devices: `Input Device`, `Output Device Popup`, `Device Dialog`,
  `Device Sync Options`, and `Checking Device Control Network`.
- Instruments: `Instruments`, `Instrument Window`, `Set Instrument`,
  `Audio Instruments`, and `All Instruments In Use`.
- Tracks: `Tracks`, `Groove Tracks`, `Song Track`, `Track Setup`, and
  `Detailed Track Overview`.
- Sequences: `Sequences Window`, `New Sequence`, `Clear Sequences`,
  `Capture Sequence to Sequence`, and `Duplicate Sequence`.
- Audio: `Audio`, `Audio System`, `Audio Recording Preferences`,
  `Audio Routings`, `Audio-to-MIDI`, and `MIDI-to-Audio`.
- Synchronization: `Initializing synchronization`, `Send Sync`,
  `Sync Options`, and `Sync has been set to "Internal"`.
- SMPTE: `SMPTE Setup`, `Absolute SMPTE`, `Display Relative SMPTE`, and
  `Insert SMPTE Marker`.
- MTC: `Send MTC` and `MTC`.
- Tempo: `Change Tempo`, `Tempo Track`, `Enter New Tempo:`, and
  `Average tempo:`.
- Markers: `Insert Bar Marker`, `Insert SMPTE Marker`, `MarkerPanel`, and
  `StripDrawingAreaMarkers`.
- Regions: `Allow crossfade regions to move about splice point as needed.` and
  `New events contain data inside fade region`.

#### Bundle information

- The artifact is not a modern application bundle at the inspected path. No
  bundle directory structure or `Info.plist` is present.

### Unknowns and unexamined metadata

- The semantic roles and relationships of the resource types and strings are
  unexamined.
- The printable-string observations are representative, not a complete string
  inventory.
- No file-format structures were inferred from executable bytes, resources,
  or terminology.
- Metadata other than the directly reported Finder information, forks,
  version, resource types, architecture identifiers, and strings is
  unexamined.
- Notes: The application was inspected in place and was not executed,
  modified, renamed, installed, or copied.

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
