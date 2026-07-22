# OMS Technical Publication Findings

This document records terminology and statements from the PDF publications in
`Phoenix Research/Opcode_OMS_extracted/Opcode_OMS/OMS Applications/OMS Tech Pubs`.
It is a documentation survey, not a description of any binary format.

The directory also contains `XTC Read Me`, which is not a PDF and was not read
for this survey.

## Method and evidence boundaries

- Three PDFs were read without modification.
- Extracted text was checked against rendered first-page previews for titles
  and visible version wording.
- Page references below use the publication's printed page numbers, which
  correspond to the PDF page numbers in these files.
- **Documentation** summarizes or briefly quotes statements in the named
  publication.
- **Phoenix observation** connects literal terminology to existing Phoenix
  research notes without asserting a relationship between file structures.
- **Future hypothesis** is a question that would require new evidence. It is
  not a conclusion.
- No binary structure, record, field, identifier mapping, or storage encoding
  is inferred from these publications.

## `OMS_NameMgr.pdf`

### Document identity

- Title: *Using the OMS Name Manager*.
- Length: 35 pages.
- No publication version or publication date was visible in the extracted
  document text or rendered title page.
- PDF metadata records creation `1996-04-19 13:57:02` and modification
  `1996-05-06 19:44:36`; these are PDF metadata values, not claimed
  publication dates.
- PDF metadata identifies FrameMaker 5.0 as creator and Acrobat Distiller 2.0
  for Power Macintosh as producer.

### Important terminology and definitions from the documentation

- **Name Manager:** an OMS patch-name management tool that organizes names for
  patches in MIDI devices and supplies those names to Vision and other
  OMS 2.0-compatible applications (p. 2).
- **Names Window:** the OMS-compatible application window used to configure
  the Name Manager (pp. 2, 5-14).
- **Patch:** a collection of device parameters that produces a sound when
  recalled (p. 2).
- **Bank:** a collection of patches in a MIDI device; the document distinguishes
  bank selection from the 128 program changes defined by MIDI (p. 3).
- **Mode:** a device state affecting how MIDI program changes are interpreted;
  examples in the document are Patch, Effect, and Performance modes (p. 3).
- **Patch Name document:** a document storing and providing patch names and
  device modes for one or more devices (p. 3).
- **Current Patch Name document:** a Patch Name document that OMS treats as
  corresponding to patches currently loaded in a device (p. 4).
- **Patch Name Provider:** an application able to create a Patch Name document
  readable by the OMS Name Manager (p. 4).
- **Name Setup:** a mapping between each MIDI device in the current OMS Studio
  Setup document and a selected Patch Name document (p. 4).
- **Subscription:** the selection of a Patch Name document for a MIDI device
  (pp. 6-7).
- **Typed-In Patch Name document:** a Patch Name document created within the
  Name Manager rather than supplied by an external librarian (pp. 7, 16).
- **Device Mode:** the mode selected for a device or MIDI channel in the Names
  Window (pp. 6, 9-11).
- **GM Equivalent:** a General MIDI equivalent assignable to a patch in the
  Patch Name Editor (p. 17).
- **Note Names** and **Control Names:** names associated with notes and MIDI
  controls and stored in a Patch Name document (pp. 18, 24-25).

### Documented concepts relevant to the OMS environment

#### Studio Setup and MIDI devices

- The Names Window's Device column reflects device names in the current OMS
  Studio Setup document and changes when the current Studio Setup changes
  (p. 6).
- A device retained in a Name Setup but absent from the current Studio Setup is
  displayed in italics (p. 6).
- Device mode may be selected per MIDI channel when channels are shown
  separately (pp. 9-11).

#### Patch names, Factory Names, and device naming

- The publication lists three Patch Name document sources: bundles from Galaxy
  or another compatible librarian, documents typed in through the Name Manager,
  and factory documents supplied by Opcode (p. 7).
- The `Use Factory/General MIDI Names` command selects a General MIDI document
  from the `Factory Names` folder, or a device-specific factory document when
  one is available (p. 12).
- A missing subscribed Patch Name document is displayed in italics; a current
  document is marked separately (p. 8).
- The Patch Name Editor presents the MIDI program representation as
  `bank0/bank32/patch number (0-127)` (p. 16).
- The editor can associate patch names, General MIDI equivalents, note names,
  and control names with entries (pp. 17-18, 24-25).
- The document discusses copying and pasting name lists as lines containing a
  patch number, a tab, and a patch name (p. 20).
- The phrase `Studio Patches` was not observed in this publication's extracted
  text.

#### File storage and sequence organization

- The publication distinguishes typed-in documents, Galaxy bundles, and
  bundles from other OMS-compatible librarians as kinds or sources of Patch
  Name documents (pp. 3, 7).
- On Windows, the documented Open dialog filters include `.ONM` for typed-in
  name documents and `.BND` for Galaxy bundles (p. 6). No Macintosh filename
  extension is specified there.
- Save and Save As allow a typed-in Patch Name document to be named and stored
  at a selected location (pp. 19-20).
- Note and control names can be added to a saved Patch Name document
  (pp. 20, 25).
- The publication states that the Names Window information is saved with a
  sequence through Save or Save As (p. 6).
- It also states that Vision saves Name Setups in sequence files and describes
  choices shown when opening a sequence with a different Name Setup
  (pp. 33-35).
- One choice makes the opened file's Name Setup current; another keeps the
  current setup and then offers choices for retaining or changing the opened
  document's assignments (pp. 34-35).

#### Routing and synchronization

- The publication describes channel-specific device modes and MIDI program,
  bank-select, note-name, and control-name presentation. It does not provide a
  general MIDI cable-routing model.
- No synchronization or timecode procedure was identified in this publication.

### Brief quoted documentation

> “When you save a sequence file in Vision, the current Name Setup is saved along with it.” (p. 4)

This quote establishes only the documented inclusion statement. It does not
identify how, where, or in what encoding the Name Setup is stored.

### Phoenix observations

- [`PROJECT_FILE_STRUCTURE.md`](PROJECT_FILE_STRUCTURE.md) records the literal
  strings `General MIDI` and
  `Macintosh HD:System Folder:OMS Folder:Factory Names:General MIDI` in the
  inspected project data fork.
- That survey also records device-like names near the beginning of the file,
  including `IAC Bus #1`, `JD-800s #1`, `JD-990s #1`, and `Juno-106#1`.
- [`ARTIFACTS.md`](ARTIFACTS.md) records `Patch Names`, `Edit Patch`,
  `Find Patch...`, and `Copy Patches` in the Studio Vision preferences resource
  fork, and patch-related strings in the installed OMS Setup application.
- These are terminology overlaps only. They do not show that any observed
  project bytes encode a Name Setup, subscription, device mode, bank, or Patch
  Name document.

### Future hypotheses requiring new evidence

- Controlled Vision saves made before and after a Name Setup change could be
  compared to ask whether any changed byte ranges correlate with the documented
  saved Name Setup.
- Controlled changes to one device name, subscription, mode, patch name, note
  name, or control name could test whether corresponding literal strings or
  repeatable byte differences appear in a saved sequence.
- A comparison with the referenced `Factory Names` file could test whether the
  project retains a path, a document name, copied names, or some other value.
- These are proposed experiments, not claims about the current sample.

## `OMS_MTP.pdf`

### Document identity

- Title: *Using OMS with a MIDI Time Piece*.
- Length: 10 pages.
- No publication version or publication date was visible.
- The title page requires MIDI Time Piece DA version 1.2 or later and MTP MIDI
  Manager Driver 1.0 or later. These are software requirements, not a version
  identifier for the publication.
- PDF metadata records creation `1997-03-05 16:07:56`; this is a PDF metadata
  value, not claimed as a publication date.
- PDF metadata identifies FrameMaker 5.1 as creator and Acrobat Distiller 2.0
  for Power Macintosh as producer.

### Important terminology from the documentation

- **MIDI Time Piece (MTP):** identified as Mark of the Unicorn's MIDI
  interface (p. 1).
- **MIDI Time Piece OMS driver:** the driver whose presence in the OMS folder,
  together with connected MTP hardware, results in MTP icons in a Studio Setup
  document (p. 1).
- **Port configuration dialog:** a dialog opened from an MTP icon and used for
  settings including the interface speed (p. 2).
- **MIDI source:** an input selection in an OMS-compatible application; the
  publication distinguishes selecting the MTP itself from selecting attached
  devices by name (p. 2).
- **Network configuration**, **cable routing grid**, **fast mode**,
  **1-8 mode**, and **9-16 mode** are recurring terms in the described MTP
  configurations (pp. 3-10).
- The publication does not present a formal glossary.

### Documented concepts relevant to the OMS environment

#### Studio Setup, devices, ports, and naming

- Connected MTP hardware is shown as one or more icons in the Studio Setup
  document when the OMS driver is present (p. 1).
- A connected MTP may occasionally appear as a `Standard Interface`; the
  publication directs the user to create a new Studio Setup after power-cycling
  the MTP in that case (p. 2).
- Devices need be entered only for MTP ports to which devices are connected
  (pp. 3, 6-7, 9-10).
- A single MTP connected through two Macintosh serial ports appears as two
  Studio Setup icons named `MIDI Time Piece-Modem` and
  `MIDI Time Piece-Printer` in the publication (p. 6).
- The selected serial-port representation determines whether an attached
  device communicates through the Macintosh Modem or Printer port in that
  example (p. 6).
- The publication warns against defining multiple devices as occupying the
  same port (p. 10).

#### MIDI routing and synchronization

- Selecting the MTP itself as an application's MIDI source supplies timecode
  from the MTP and MIDI beat clock from attached devices, but not other MIDI
  data from those devices (p. 2).
- The publication directs users to select attached devices by name for
  non-synchronization MIDI input (p. 2).
- It states that the OMS MTP driver exposes fast mode and cable-routing
  capabilities to OMS applications (p. 3).
- The Studio Setup connections and the MTP DA cable-routing grid are described
  as needing to agree (pp. 3-10).
- Configurations cover one or two MTP units, one or both Macintosh serial
  ports, and MTP cable ranges 1-8 and 9-16 (pp. 3-10).

#### Patch names, Factory Names, and storage

- No discussion of Patch Name documents, Factory Names, Studio Patches, or
  project-file storage was identified.
- The publication discusses closing a Studio Setup and declining to save
  changes during a recovery procedure (p. 2), but does not describe the
  Studio Setup file's storage format.

### Brief quoted documentation

> “Select devices by name” for non-synchronization MIDI sources. (p. 2)

The surrounding documentation distinguishes these selections from choosing the
MTP itself. No encoding or saved-project representation is described.

### Phoenix observations

- [`PROJECT_FILE_STRUCTURE.md`](PROJECT_FILE_STRUCTURE.md) records the literal
  string `Modem/Printer ` in the authentic project sample.
- [`ARTIFACTS.md`](ARTIFACTS.md) records `MIDI Time Piece`, `MIDI Time Code`,
  serial-port terminology, and Studio Setup terminology in installed OMS
  artifacts.
- These overlaps do not establish that the project sample stores an MTP
  configuration, routing grid, port selection, or synchronization setting.

### Future hypotheses requiring new evidence

- Controlled saves with one selected device, port assignment, or
  synchronization source changed could test whether any project-file bytes
  change repeatably.
- A controlled comparison between Studio Setups using one versus two MTP icons
  could test whether icon names or port labels appear in Studio Setup files.
- These are proposed comparisons only; the publication supplies no binary
  format evidence.

## `OMS_Pbay.pdf`

### Document identity

- Title: *Using OMS with a MIDI Patchbay*.
- Length: 13 pages.
- No publication version or publication date was visible in the extracted
  document text or rendered title page.
- PDF metadata records creation `1996-04-19 15:50:37` and modification
  `1996-05-06 19:45:11`; these are PDF metadata values, not claimed
  publication dates.
- PDF metadata identifies FrameMaker 5.0 as creator and Acrobat Distiller 2.0
  for Power Macintosh as producer.

### Important terminology and definitions from the documentation

- **MIDI patcher:** hardware also called a patchbay, switcher, or thru box in
  the publication (p. 2).
- **Patcher Info:** the dialog containing manufacturer, model, port count, and
  receive-channel settings for a patcher (p. 4).
- **Patcher Icon:** the patcher representation in a Studio Setup document
  (p. 7).
- **Patcher Name:** the name assigned through Patcher Info (p. 7).
- **Patcher Port:** the numbered patchbay port associated with a connected MIDI
  device in the Studio Setup document (pp. 8-9).
- **Patcher Default Program:** a patcher program used when the patcher is not
  routing data from a particular device to the computer (pp. 7-8).
- **Patcher Program:** a patcher program associated with two-way communication
  between one MIDI device and the computer (pp. 8-10).
- **Standard connection:** paired MIDI inputs and outputs connected through
  corresponding patcher ports (p. 12).
- **Non-standard connection:** the publication's term for connections that
  depart from that paired-port arrangement (pp. 12-13).
- The publication does not present a separate formal glossary; these terms are
  defined in its explanatory text.

### Documented concepts relevant to the OMS environment

#### Studio Setup, devices, ports, and naming

- A patcher is added to the current Studio Setup through `Studio>New Patcher`,
  after which manufacturer, model, port count, and receive channels are
  selected (pp. 3-4).
- The Studio Setup can contain multiple patcher icons (p. 5).
- One physical patcher shared between two serial ports is represented by two
  patcher icons in the documented example (pp. 5-6).
- The publication requires those two representations to have unique names and
  uses `MSB 16/20 Modem` and `MSB 16/20 Printer` as examples (p. 6).
- Patcher port numbers in the Studio Setup are set to match physical device
  connections (p. 9).
- Different MIDI input and output ports for one device require two device icons
  in the documented arrangement (p. 9).

#### MIDI routing and synchronization

- Patcher programs are described as routing MIDI between the computer and
  selected devices (pp. 8-10).
- A default program connects computer output to all connected MIDI devices and
  computer input to a controller keyboard in the documented example (p. 8).
- The `Send program changes to` selection chooses the computer and patcher port
  used for program changes when more than one patcher representation is present
  (pp. 5-6).
- OMS Setup uses program-change values 0-127 for patcher program numbers
  (p. 10).
- OMS Preferences contains a delay after a patcher program change, measured in
  milliseconds, before subsequent MIDI transfer (p. 11).
- `Studio>Different In/Out Ports` is used for the publication's non-standard
  configuration example (p. 13).
- No MIDI Time Code, beat-clock, or other synchronization procedure was
  identified in this publication.

#### Patch names, Studio Patches, Factory Names, and storage

- The word `patcher` and the term `patcher program` refer to patchbay routing in
  this publication; they are not presented as Patch Name documents.
- No discussion of Patch Name documents, Studio Patches, Factory Names, or
  Vision project organization was identified.
- The publication instructs the user to save the Studio Setup after defining
  patchers (p. 5), but does not describe the file's storage format.

### Brief quoted documentation

> “you must give each patcher a unique name” (p. 6)

This statement concerns the two representations in the documented shared-
patcher example. It does not identify how names are stored.

### Phoenix observations

- [`ARTIFACTS.md`](ARTIFACTS.md) records the literal strings `New Patcher`,
  `Different In/Out Ports`, `PortInUse`, `PatchBay`, and patcher-related terms
  in installed OMS artifacts.
- [`PROJECT_FILE_STRUCTURE.md`](PROJECT_FILE_STRUCTURE.md) records device-like
  strings and `Modem/Printer ` in the authentic project sample, but does not
  identify any patcher name or routing structure.
- The shared vocabulary is observational only and does not establish a binary
  relationship among a Studio Setup, a Vision sequence, and the inspected OMS
  artifacts.

### Future hypotheses requiring new evidence

- Controlled Studio Setup saves could vary one patcher name, physical port,
  default program, or per-device program at a time and compare resulting bytes.
- Controlled Vision sequence saves could test whether any patcher-related
  settings are retained with the sequence or only referenced through the
  current Studio Setup.
- These questions are not answered by the publication and do not imply a
  record layout.

## Cross-publication terminology map

| Concept | Name Manager | MIDI Time Piece | MIDI Patchbay |
| --- | --- | --- | --- |
| Studio Setup document | Supplies devices used by a Name Setup | Contains MTP representations and device connections | Contains patcher representations, ports, and connected devices |
| Device naming | Device names appear in the Names Window | Attached devices are selected by name; dual-port MTP names are shown | Dual-port representations require unique names |
| Ports | MIDI channels and device modes are discussed; physical ports are not a focus | Macintosh serial ports and MTP cable ranges are central | Physical patcher ports and computer serial ports are central |
| Patch names | Patch Name documents, subscriptions, modes, Factory Names, note names, and control names | Not identified | `Patcher program` is routing terminology, not documented as a Patch Name document |
| MIDI routing | No general cable-routing model identified | MTP DA cable-routing grids and Studio Setup connections | Patcher programs, default programs, and port assignments |
| Synchronization | No synchronization procedure identified | Timecode and MIDI beat clock source selection | No synchronization procedure identified |
| Saved data | Name Setup information is documented as saved with Vision sequences | No project storage statement identified | Studio Setup is saved, but no file format is described |

This table compares documented terminology only. It is not a proposed data
model for Studio Vision or OMS files.

## Overall findings and limits

### Documentation-supported findings

- The Name Manager publication explicitly documents that a Vision sequence can
  carry Name Setup information.
- It defines a Name Setup as associations between devices in the current OMS
  Studio Setup and Patch Name documents.
- The three publications consistently treat the Studio Setup document as the
  representation of configured interfaces, devices, patchers, ports, and
  connections relevant to their respective workflows.
- Names, ports, channels, modes, subscriptions, program numbers, and routing
  selections are distinct terms in the publications.

### What these publications do not establish

- They do not disclose a Studio Vision sequence-file grammar.
- They do not identify offsets, byte order, record boundaries, lengths,
  identifiers, checksums, compression, or resource semantics.
- They do not establish that any particular printable string in the authentic
  Phoenix sample belongs to a saved Name Setup or another specific structure.
- They do not establish whether Studio Setup or Patch Name documents are
  embedded, copied, referenced by path, referenced by name, or represented in
  another way beyond the user-visible statements summarized above.
