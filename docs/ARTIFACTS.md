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

### Observations

- Filename: `Studio Vision Setup`
- Source: Local research copy at
  `/Users/kurtheiden/Documents/Phoenix Research/Studio Vision Pro 4.5 ƒ/Studio Vision Setup`
- Filesystem object: Regular file.
- File size and data-fork size: 34,039 bytes.
- Data-fork SHA-256:
  `be78001952e5f8eb78ed94ccb0efe8ac87f7131bf0b5c9a63ff1d01aa1fe52e5`
- Finder Type: `MIDS` (`4d 49 44 53`).
- Finder Creator: `MIDA` (`4d 49 44 41`).
- Three extended attributes are present:
  `com.apple.FinderInfo` (32 bytes), `com.apple.ResourceFork` (286 bytes), and
  `com.apple.quarantine` (57 bytes).
- A resource fork is present and is 286 bytes long.
- Resource-fork SHA-256:
  `40dc81615e0c31b2e4fe653a3c1a41e50530dcf64a4b2428ff8ae6b5770863b6`
- `file` identifies the resource fork as an Apple HFS/HFS+ resource fork.
- `DeRez` parses the resource fork but reports no resources. Consequently, no
  `vers` resource or resource type was observed.
- Generic `file` identification reports the data fork only as `data`. No
  executable format or architecture identifier was observed.

#### Directly observed printable strings

The following are representative literal strings observed in the data fork.
They are recorded without interpretation:

- Setup: `Save As Setup`, `Page Setup...`, `Hardware Setup...`,
  `OMS Studio Setup...`, and `OMS MIDI Setup...`.
- OMS: `OMS Studio Setup...` and `OMS MIDI Setup...`.
- MIDI devices: `MIDIKeys Window`, `Enable Input Devices`,
  `Show Console Remote Devices`, `D-550`, and `K2000`.
- Synchronization: `Sync to Internal Clock`, `Sync to External Beat Clock`,
  `Sync to MIDI Time Code`, `Sync to MTC/Machine Control`, `Send Sync`, and
  `Toggle Sync Mode`.
- Instruments: `Set Instrument...`, `New Instrument`, `Instruments Window`,
  `Select Unused Instruments`, and `Make Instruments from Studio Setup...`.
- Preferences: `Preferences...` and `Audio Preferences...`.
- Audio: `Audio Instruments Window`, `Mix Audio...`, `Import Audio...`,
  `Lock Audio to Tape`, and `Audio Event Height Automatic`.
- A case-insensitive scan of printable strings found no matches for `port`,
  `routing`, or `configuration`.

### Unknowns and unexamined metadata

- The data-fork format and internal organization are unknown and unexamined.
- The semantic roles and relationships of the observed strings are
  unexamined.
- The printable-string observations are representative, not a complete string
  inventory.
- The absence of printable matches for `port`, `routing`, and `configuration`
  does not establish that corresponding data or concepts are absent.
- The meaning of the empty resource fork and metadata other than the directly
  reported Finder information and extended-attribute names and sizes is
  unexamined.
- Notes: The artifact was inspected in place and was not executed, modified,
  installed, renamed, or copied.

## OMS applications

### Earlier search and installer-extraction limitation

- During the earlier search, no separately extracted OMS application artifacts
  were located in the documented Phoenix research directories.
- The only OMS-named artifact located during that search was the already
  inventoried `Install OMS 2.3.8` regular file.
- The installer identity was reconfirmed as:
  - Data-fork size: 2,485,279 bytes.
  - SHA-256:
    `34d3f759284941ca4ecc91fb33902087140accbcd37c4677f94cd58ee68f826d`.
  - Finder Type: `APPL`.
  - Finder Creator: `VIS3`.
  - Data-fork header: `SVCT`.
- The installed read-only tools `bsdtar`, `xar`, `hdiutil`, and `unzip` did
  not recognize the payload as a supported archive or disk image.
- The installed copies of The Unarchiver and StuffIt Expander did not provide
  a demonstrated command-line listing method or explicit VISE or `SVCT`
  support.
- No installer extraction directory was created. Nothing was extracted from or
  executed from the installer.

- The internal `SVCT` installer container structure remains unknown.
- The identities and contents of any applications embedded in the installer
  remain unknown.
- It remains unknown whether the installer contains standalone OMS
  applications.
- A safe non-executing method for extracting the `SVCT` installer payload has
  not been identified.
- The absence of separately extracted OMS applications at that time was not
  evidence that OMS applications did not exist.

### Extraction provenance for the installed OMS collection

- Source archive: `Opcode_OMS.sit`.
- Source path:
  `/Users/kurtheiden/Documents/Phoenix Research/Opcode_OMS.sit`.
- Archive size: 4,502,486 bytes.
- Archive SHA-256:
  `120f52b5e3ecaf99b32b981cba95853ebca115e31f80bce60d3e4865a0d9737c`.
- Finder Type: `SIT5`.
- Finder Creator: `SIT!`.
- The archive was created inside the working SheepShaver environment.
- A destination-controlled attempt using StuffIt Expander failed with
  `StuffItManagerError 21` and created no extracted files.
- Extraction succeeded using The Unarchiver through its documented AppleScript
  interface.
- Destination:
  `/Users/kurtheiden/Documents/Phoenix Research/Opcode_OMS_extracted`.
- Exact successful extraction command:

```text
osascript -e 'tell application "The Unarchiver" to unarchive {"/Users/kurtheiden/Documents/Phoenix Research/Opcode_OMS.sit"} to "/Users/kurtheiden/Documents/Phoenix Research/Opcode_OMS_extracted" deleting Original false creating Folder Never wait Until Finished true'
```

- The original archive hash remained unchanged after extraction.
- No extracted application, extension, driver, installer, or utility was
  executed.
- Extracted files retained FinderInfo attributes, and classic Macintosh files
  retained resource forks where present.

### Observed installed OMS hierarchy

The primary working source is
`/Users/kurtheiden/Documents/Phoenix Research/Opcode_OMS_extracted/Opcode_OMS`.
The following hierarchy was directly observed:

```text
Opcode_OMS/
├── Control Panels/
├── OMS Preferred Device
├── Install OMS 2.3.8
├── OMS_2.3_Mac.pdf
├── Extensions/
│   ├── Open Music System
│   └── USB OMSMIDIDriver
└── OMS Applications/
    ├── Studio Patches Editor
    ├── Icon\r
    ├── OMS 2.3.8 Read Me
    ├── OMS Setup
    └── OMS Tech Pubs/
        ├── OMS_NameMgr.pdf
        ├── OMS_MTP.pdf
        ├── OMS_Pbay.pdf
        └── XTC Read Me
```

No purpose is attributed to these entries beyond their filenames, locations,
and directly observed metadata.

### Installed `OMS Setup`

#### Observations

- Filename: `OMS Setup`.
- Source path:
  `/Users/kurtheiden/Documents/Phoenix Research/Opcode_OMS_extracted/Opcode_OMS/OMS Applications/OMS Setup`.
- Filesystem object: Regular file.
- Data-fork size: 259,828 bytes.
- Data-fork SHA-256:
  `1e2a0be4c4f8cb8cd34994b86bc94f1972211789e6903a43035388787ce52998`.
- Finder Type: `APPL`.
- Finder Creator: `OmsS`.
- `com.apple.FinderInfo` is present.
- Resource-fork size: 430,252 bytes.
- Resource-fork SHA-256:
  `7b1efbb36b2383c5662f4d6af2523f5f7ae13070c8f4c3ce75446a982e3783d0`.

#### Version and executable evidence

- Two `vers` resources report `2.3.8`; one contains
  `Open Music System 2.3.8`, and the other contains
  `©1990-1999 Opcode Systems, Inc.`.
- `file` identifies the data fork as a PowerPC PEF executable.
- The data fork begins `Joy!peffpwpc` in ASCII.
- The `cfrg` resource contains `pwpc` and `OMSSetupPPC`.
- A `CODE` resource type is present. No architecture conclusion is drawn from
  that resource-type observation.

#### Complete observed resource-type list

`DeRez` reported 40 distinct resource types:

- `ALRT`, `BNDL`, `CNTL`, `CODE`, `CURS`, `DATA`, `DITL`, `DLOG`, `Estr`,
  `FOND`, `FONT`, `FREF`, `Fnt#`, `ICN#`, `ICON`, `LDEF`, `MBAR`, `MENU`,
  `MMA `, `OmsS`, `PICT`, `PROC`, `SICN`, `SIZE`, `STR `, `STR#`, `TEXT`,
  `WIND`, `bmap`, `cfrg`, `cicn`, `clut`, `hmnu`, `icl4`, `icl8`, `ics#`,
  `ics4`, `ics8`, `snd `, and `vers`.

#### Representative literal strings

The following literal strings were directly observed and are recorded without
interpretation:

- OMS: `About OMS Setup`, `OMS MIDI Setup`, `OMS Setup version `,
  `OMS Timing`, and `Open Music System 2.3.8`.
- Studios: `Create a New Studio Setup`, `My Studio Setup`, `Test Studio`,
  `Studio Setup Editing`, and `Studio setup errs`.
- MIDI devices: `MIDI Device Info`, `Identifying MIDI devices...`,
  `Auto-Detect Devices`, `New Device`, and `Device list`.
- Ports: `Add Device Per Port`, `Different In/Out Ports`, `Port disabled`,
  `Ports:`, and `Sort by Port/Name`.
- Interfaces: `Interfaces`, `MIDI Cards & Interfaces`,
  `MIDI Cards and Interfaces`, and `The MIDI interface `.
- Drivers: `Drivers`, `Edit Driver Device`, and `NoSyncRoutDrvr`.
- Synchronization: `Check this if the device receives MIDI Time Code.`,
  `Check this if the device sends MIDI Time Code`, `OMS Timing`, and
  `To route sync to or from ^1, choose `.
- Preferences: `Preferences`.
- Setup files and documents: `Open Current Studio Setup`,
  `Save studio setup as:`, `Studio Setup Doc Help`, and
  `Customizing the Studio Setup Document`.
- Patches: `New Patch`, `New Patcher`, `Clear Patches`, and `Paste Patches`.

#### Comparison evidence

- The archive-extracted copy exactly matches the `Opcode System Files` loose
  copy in data-fork hash, resource-fork size and hash, and FinderInfo bytes.
- The other `Opcode` loose copy has the same data fork but a resource fork 54
  bytes smaller, a different resource-fork hash, and different Finder flag
  bytes.
- The cause and semantic significance of that difference are unknown.

#### Conclusion limited to observed evidence

- The extracted object has Finder Type `APPL`, Finder Creator `OmsS`, version
  resources reporting `2.3.8`, and directly observed PowerPC PEF evidence.
- These observations do not establish undocumented application behavior,
  resource semantics, file-format structures, or relationships to Studio
  Vision project bytes.

#### Unknowns and unexamined structures

- Resource contents beyond the directly reported version, type, name, and
  printable-string observations remain unexamined.
- The internal organization of the data fork and resource fork is unexamined.
- The semantic roles and relationships of the observed resources and strings
  are unknown.
- The representative strings are not a complete string inventory.
- No extracted artifact was executed during this inventory.

## Documentation

### Directory observations

- Source directory:
  `/Users/kurtheiden/Documents/Phoenix Research/Studio Vision Pro 4.5 ƒ/Documentation`
- The directory contains five PDF documents and one non-document `Icon\r`
  entry.
- Each PDF begins with a PDF 1.2 signature.

### `Vision opPLUGS Manual.pdf`

- Filesystem object: Regular file.
- Size: 228,178 bytes.
- SHA-256:
  `ff9baddb4176fc31645289f95e91d574e5cad2253af34565bb0ec8d7f8c4fa4c`
- Filesystem creation date: 1998-11-13 09:06:09 +0000.
- Filesystem modification date: 1998-11-13 09:06:09 +0000.
- Format: PDF 1.2; `file` reports 14 pages.
- Embedded title: `Vision opPLUGS Manual`.
- Embedded author: `Gregory Simpson`.
- Embedded creator and producer: `FrameMaker 5.5` and
  `Acrobat Distiller 3.01 for Power Macintosh`.
- Embedded creation date: 1998-08-20 12:54:57; no time-zone offset is present
  in the field.
- Embedded modification date: 1998-08-20 12:58:27; no time-zone offset is
  present in the field.
- Directly observed outline titles include `opPLUGS Overview`,
  `Credits, Colophon, & Notices`, `opSQUASH`, `opRING`, `opREZN8`, `opPLATE`,
  `opPANNER`, `opFLANGE`, `opECHO`, `opCYCLE`, `opCOMP`, `opCHORUS`,
  `opBOXDLY`, and `opALIGN`.
- No requested-topic determination beyond the literal outline titles above was
  made.

### `MIDI Reference Manual.pdf`

- Filesystem object: Regular file.
- Size: 6,062,074 bytes.
- SHA-256:
  `9f99fad6a358848398ce3513e06e6ef49140458683bacda7e282f061a2992239`
- Filesystem creation date: 1999-07-30 10:56:13 +0100.
- Filesystem modification date: 1999-07-30 10:56:13 +0100.
- Format: PDF 1.2.
- Embedded title: `Reference Book`.
- Embedded author: `ahill`.
- Embedded creator and producer: `FrameMaker 5.5` and
  `Acrobat Distiller 3.02 for Power Macintosh`.
- Embedded creation date: 1999-07-30 17:48:34; no time-zone offset is present
  in the field.
- Directly observed chapter or section titles include
  `CHAPTER 2: About MIDI`, `CHAPTER 3: Vision Basics`,
  `The Open Music System (OMS)`, `Tracks`, `Instruments`, `Sequences`,
  `CHAPTER 18: Synchronization`, `CHAPTER 20: Templates`,
  `CHAPTER 22: Custom Instruments`, `OMS MIDI Setup`,
  `Export as MIDI File`, and `Import Audio Using QuickTime`.
- Those literal titles directly contain references to MIDI, Vision, OMS,
  instruments, synchronization, templates, export, import, and audio.

### `QuickStart - Vis 4.5.pdf`

- Filesystem object: Regular file.
- Size: 362,458 bytes.
- SHA-256:
  `1b550919ba2af7e889f3aff1638859ab8db298ac15bce222c7e9af713716da99`
- Filesystem creation date: 1999-07-30 04:41:52 +0100.
- Filesystem modification date: 1999-07-30 04:41:52 +0100.
- Format: PDF 1.2; `file` reports 2 pages.
- Embedded title: `QuickStart - Vis 4.1`.
- Embedded author: `ahill`.
- Embedded creator and producer: `QuarkXPress\250: LaserWriter 8 8.6.5` and
  `Acrobat Distiller 3.02 for Power Macintosh`.
- Embedded creation date: 1999-07-30 11:41:52; no time-zone offset is present
  in the field.
- No table of contents, section headings, or requested-topic references were
  directly observable with the available byte-string inspection.

### `Audio Reference Manual.pdf`

- Filesystem object: Regular file.
- Size: 7,220,565 bytes.
- SHA-256:
  `75a333f99b846cde6db3c5a0e3d408085c94851402f6c5139c9dac5e3717fdc9`
- Filesystem creation date: 1999-07-30 11:04:41 +0100.
- Filesystem modification date: 1999-07-30 11:04:41 +0100.
- Format: PDF 1.2.
- Embedded title: `AudioRef`.
- Embedded author: `ahill`.
- Embedded creator and producer: `FrameMaker 5.5` and
  `Acrobat Distiller 3.02 for Power Macintosh`.
- Embedded creation date: 1999-07-30 17:57:13; no time-zone offset is present
  in the field.
- Directly observed chapter or section titles include
  `Introduction to Digital Audio`, `CHAPTER 3: Acadia: Hardware Setup`,
  `CHAPTER 6: Acadia: Audio Instruments & Routings Window`,
  `Saving Instrument Setups`, `Audio Preferences`,
  `CHAPTER 20: Import/Export Cornucopia`,
  `CHAPTER 29: Synchronizing Audio Playback`, `Creating Custom Audio Templates`,
  `Importing MIDI from a QuickTime Movie`, and
  `Using Pro Tools Project with Studio Vision`.
- Those literal titles directly contain references to Studio Vision, a
  project, MIDI, instruments, synchronization, audio, routing, import/export,
  preferences, and templates.
- No outline title containing `OMS` was observed by the targeted title scan.

### `What's New in 4.5.pdf`

- Filesystem object: Regular file.
- Size: 2,480,419 bytes.
- SHA-256:
  `537013a927c8b889cb42eb10c8564d047a9369f39792333457193aff18dca411`
- Filesystem creation date: 1999-07-30 07:50:36 +0100.
- Filesystem modification date: 1999-07-30 07:50:36 +0100.
- Format: PDF 1.2.
- Embedded title: `4.5.book`.
- Embedded author: `ahill`.
- Embedded creator and producer: `FrameMaker 5.5` and
  `Acrobat Distiller 3.02 for Power Macintosh`.
- Embedded creation date: 1999-07-30 14:38:28; no time-zone offset is present
  in the field.
- Directly observed chapter or section titles include
  `CHAPTER 2: Automation`, `MIDI Controller Data`, `CHAPTER 3: ReWire`,
  `ReWire and Synchronization`, `CHAPTER 4: File Management Changes`,
  `CHAPTER 5: Strip Silence and Slice Audio`, `Audio Sends`,
  `Build Console From MIDI Instruments In Use`, and
  `Instrument, Device, Channel Number and Controller`.
- Those literal titles directly contain references to MIDI, devices,
  instruments, synchronization, and audio.

### Non-document directory entry: `Icon\r`

- Filesystem object: Regular file.
- Data-fork size: 0 bytes.
- Data-fork SHA-256:
  `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.
- Filesystem creation date: 1999-07-07 11:36:56 +0100.
- Filesystem modification date: 1999-07-07 11:36:56 +0100.
- Finder Type: `icon`; Finder Creator: `MACS`.
- A 2,670-byte resource fork is present with SHA-256
  `dd5fa527661fd0ffb0d67f98c670ff66aa7c4a16da4255d96cba2c939d88471c`.
- No document format is attributed to this entry.

### Unknowns and unexamined metadata

- Poppler PDF metadata, text, and rendering tools were unavailable, and the
  installed system PDFKit toolchain could not be used. No pages were rendered.
- PDF fields and outline titles not exposed as direct printable byte strings
  remain unexamined.
- Page counts for PDFs where `file` did not report a count are unexamined.
- The absence of an observed outline title or topic term does not establish
  that the document does not contain that topic.
- The manuals were not summarized or interpreted.

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
