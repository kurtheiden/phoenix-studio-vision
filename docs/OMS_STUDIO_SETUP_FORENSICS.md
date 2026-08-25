# OMS Studio Setup and Vision Instrument Routing Forensics

## Scope and evidence vocabulary

This document records recovered OMS Studio Setup evidence relevant to Studio
Vision's displayed Instrument routing. It distinguishes:

- **Direct observation:** a value visible or mechanically measured in an
  identified artifact.
- **Bounded empirical correlation:** a displayed value and bytes bound in an
  identified instance, without claiming a general encoding rule.
- **Supported interpretation:** a conclusion supported by multiple direct
  observations but not proven as a universal ownership or serialization rule.
- **Hypothesis:** a testable possibility requiring new evidence.
- **Unknown:** a question the recovered evidence does not answer.

This investigation does not establish a complete OMS Studio Setup binary
grammar. In particular, it does not establish general record boundaries,
offsets, byte order, identifiers, checksums, version behavior, or project-file
storage.

## Sources and provenance

The authoritative recovery report for the exact measurements summarized here
is the external research artifact:

`/Users/kurtheiden/Documents/Phoenix Research/OMS Studio Setup Forensic Evidence 2026-08-25.md`

The configured SheepShaver disk was reidentified from
`/Users/kurtheiden/.sheepshaver_prefs`, not from a remembered image path. The
configured image is:

`/Volumes/Extreme Pro 2TB/Applications/SheepShaver/MacOS9`

It is 1,048,576,000 bytes with SHA-256
`9636374d777a26afe61554ffb1d02b18c012198b29dbdc826c4c93762c693491`.
Its HFS master directory block identifies volume `MacOS9HD`. SheepShaver was
not running. A copy at `/tmp/phoenix-macos9-forensic-copy.img` matched the
source in size, SHA-256, and byte-for-byte comparison before its HFS catalog
was inspected read-only. The original was not mounted or passed to the HFS
parser, and no OMS application was launched during recovery.

The evidence set also includes recovered historical OMS Studio Setup files,
the boot volume's `OMS Current State` and Recent Documents aliases, surviving
Studio Vision and OMS Setup screenshots, the Galaxy
`KURT SV3.0 Bundle 12-16-95`, and the
[`Experiment 032`](CONTROLLED_TRACK3_2_MIDI_CHANNEL_CHANGE.md)
preregistration and externally recorded abort outcome. Exact artifact
identities used below come only from the persistent recovery report; missing
temporary-log provenance is not reconstructed.

## Studio Vision Device and Channel observation

**Direct observation:** a surviving Studio Vision PPC screenshot shows
`Ode to Clarke` / `Track 3 #2` with Instrument `JD-800`. In the MIDI
Instruments window, the selected row displays:

`JD-800` -> Output Device `JD-800` -> Chan `15`

The same screenshot shows contextual help titled `Device and Channel`. It
states that this area selects the device and MIDI channel used by events
playing on the Instrument or Layer, and that the available devices and
channels are those defined in OMS Setup.

The screenshot is
`/Users/kurtheiden/Desktop/Screenshot 2026-08-24 at 7.35.55 PM.png`,
739,586 bytes, SHA-256
`650e3de5182e2193f062da9596adcfbdb3fdd6c2c1148e3d17b63e477add51c0`.
This establishes the displayed assignment and help wording. It does not
identify where or how Studio Vision serializes the assignment.

## Experiment 032 abort outcome

[`Experiment 032`](CONTROLLED_TRACK3_2_MIDI_CHANNEL_CHANGE.md) preregistered
one independent change from JD-800 channel 15 to channel 14 while preserving
the output device and every other routing value. That independent edit was not
available in the current environment, so the preregistered abort condition
triggered before save.

No controlled Experiment 032 project, save, or MIDI export exists. Its
byte-level and causal predictions were never tested. The experiment is aborted
in the current environment and deferred pending an independently controllable
variable or another evidence source. The abort is not evidence for or against
Studio Vision storing the channel in its project data.

The linked preregistration remains the experiment definition; this outcome
does not replace or restate its full protocol.

## Recovered OMS Studio Setup evidence

### Live boot-volume documents

Read-only traversal of the live HFS catalog found two genuine, non-alias files
with Finder Type and Creator `OmsS` / `OmsS`:

- `MacOS9HD:Desktop Folder:My Studio Setup`: data fork 2,400 bytes,
  SHA-256
  `99e7916b9115ce5c35b320a498fe1ba35ae80954a7aa7dab52f764b556648230`;
  empty resource fork. Recognizable entries include Studio 4-Modem/Printer,
  JD-800, other hardware devices, Studio Patches, and QuickTime Music.
- `MacOS9HD:System Folder:OMS Folder:Auto Setup 4/10/21, 0241`: data fork
  620 bytes, SHA-256
  `3ba84ee4f2713601ba5f805946123f1d2fe86880376103e45f2e037bf4be1040`;
  resource fork 2,335 bytes, SHA-256
  `835ead6e00d7da1b8180707475caec1a53835d4e868b836bc522b530cf3610e5`.
  Its recognizable entries are IAC Driver, Studio Patches pgm chg, IAC Bus
  #1, and QuickTime Music; its data fork contains no JD-800.

The inspection covered 2,923 catalog objects visible after the parser's
documented filtering of classic Desktop database entries: 2,592 files and 331
folders. No additional genuine `OmsS` / `OmsS` Studio Setup document was found
in that inspected live catalog. This is a bounded inventory of surviving
catalog contents, not a claim that no other historical setup ever existed.

### Historical/shared documents

The configured SheepShaver shared directory contains three distinguishable
historical candidates:

- `AUDIO:VIDEO Drive/Opcode/OMS Applications/Auto Setup 2:8:98, 2101`:
  2,672-byte data fork, SHA-256
  `c72ce7bcd0a03a8d9d549c376ad02a4e4fc4c6b2b27270521e7114d0795aefbb`.
- `AUDIO:VIDEO Drive/Opcode/OMS Applications/My Studio Setup`: 2,400-byte
  data fork with the same SHA-256 as the live-catalog `My Studio Setup`; these
  data forks are byte-identical.
- `Desktop Folder/Auto Setup 2:8:98, 2101 copy`: 2,672-byte data fork,
  SHA-256
  `5150bf0b9d78200837a2961c8bd73e0f7da572c7ea34c3406bb9535801095893`.
  Its equal size but different hash distinguishes it as a variant rather than
  a byte-identical copy of `Auto Setup 2/8/98, 2101`.

All three expose Finder Type and Creator `OmsS` / `OmsS`. Their host-exposed
resource forks are empty.

### Aliases and references

`OMS Setup` with Type `APPL` and Creator `OmsS` is the application, not a
Studio Setup document. The boot-volume Recent Documents entries named
`Auto Setup 4/10/21, 0241` and `My Studio Setup` are zero-data-fork aliases
whose `alis` resources resolve to the corresponding genuine in-volume files.
The Recent Documents entry `Auto Setup 2/8/98, 2101 copy` is also an alias,
but it targets the external classic path
`Unix:Desktop Folder:Auto Setup 2/8/98, 2101 copy`; it is not an additional
boot-volume Setup document.

## Closed boot-image catalog and current-setup evidence

**Direct observation:**
`MacOS9HD:System Folder:OMS Folder:OMS Preferences:OMS Current State` has
Finder Type `OMcs`, Creator `OmsI`, a zero-byte data fork, and a 903-byte
resource fork with SHA-256
`5c2171d65764779a08ccf4406cb531e605f75e99665d133980c70bec8439b1c7`.

Its `alis` resource ID 128 records Type/Creator `OmsS` / `OmsS` and directly
targets:

`MacOS9HD:System Folder:OMS Folder:Auto Setup 4/10/21, 0241`

A second `alis` resource targets the OMS Setup application. The current-setup
alias agrees with the genuine file in the same catalog. This directly
identifies `Auto Setup 4/10/21, 0241` as the current OMS Studio Setup in the
inspected emulated environment. Recovery inspected the closed image copy; it
did not activate OMS or make a setup current.

## JD-800 definition and receive-channel evidence

**Direct observations:** `My Studio Setup` and both Auto Setup 2/8/98 variants
contain recognizable JD-800 entries. Surviving OMS Setup screenshots show the
JD-800 as device/port 1 on `Studio 4-Modem/Printer` in `My Studio Setup` and on
`Studio 5-Modem/Printer` in `Auto Setup 2/8/98, 2101 copy`.

A surviving `MIDI Device Info` screenshot identifies Manufacturer Roland,
Model and Name JD-800, Device ID 17, with only Receive Channel 1 checked. It is
`/Users/kurtheiden/Desktop/Screenshot 2026-08-24 at 8.10.25 PM.png`,
62,989 bytes, SHA-256
`ff0ccc78e5888b9aca1fb5207abb36e077a24345021dbc81b1dda679c762dcc1`.

The corresponding JD-800 data-fork contexts begin at `0x100` in both Auto
Setup 2/8/98 variants and at `0x198` in `My Studio Setup`:

```text
00 00 00 11 80 00 04 00 06 4a 44 2d 38 30 30 31 ...
```

In these bounded instances, `00 11` correlates with displayed Device ID 17 and
`80 00` correlates with displayed Receive Channel 1. These observations do not
establish a general record start, offset, byte order, field rule, or OMS
grammar.

No inspected surviving Setup was verified as defining JD-800 Receive Channel
14 or 15. The current setup contains no JD-800. Both statements are bounded to
the identified surviving artifacts and do not establish historical
nonexistence or what another unavailable setup might have permitted.

## Controlled receive-channel correlation

The only reverified, auditable bounded correlation is:

| Observed bytes | Displayed value |
| --- | --- |
| `80 00` | Receive Channel 1 |

Five additional mappings were reported during earlier temporary research, but
their direct display-to-file provenance disappeared with those logs. They are
excluded from the established correlation set. Their byte patterns occurring
in inspected data is not enough to recover their displayed meanings, and no
apparent bitmap structure is inferred from the verified `80 00` instance.

## Galaxy bundle evidence boundary

The accessible artifact
`/Users/kurtheiden/Documents/Phoenix Research/KURT SV3.0 Bundle 12-16-95`
has Finder Type/Creator `oBnd` / `Glxy`, a 416,834-byte data fork, and SHA-256
`a29bf0194e417a9778d9d415b5b03a641e117bfa0213f921483383e9d43d2f18`.

Direct printable evidence includes Roland JD-800 and other device families,
Patches, Card Patches, `Ming Dynasty`, performances, banks, Multi, Name List,
patch names, and system data. This supports patch/device-library context. No
controlled comparison, decoded routing record, or direct routing-storage
statement was recovered. The examined evidence therefore does not currently
establish routing storage; it does not establish that the bundle contains no
routing-like data in unexamined structures.

## Supported interpretation

The combined evidence supports distinguishing:

1. OMS Studio Setup device/channel availability and configuration; and
2. Studio Vision's displayed logical Instrument Device/Channel assignment.

Historical JD-800 OMS definitions show Receive Channel 1, while Studio Vision
displays JD-800 Chan 15. The current OMS setup contains no JD-800, and no other
genuine Setup document in the inspected live boot-volume catalog explains
channel 15. This supports the distinction above. It does not establish where
Studio Vision stores Chan 15 or prove a universal ownership model between OMS
and Vision.

## Hypotheses requiring new evidence

- The Vision channel assignment may be serialized in the project.
- It may belong to an Instrument or Layer structure.
- It may reside in Studio Vision preferences or other environment data.
- It may be represented through a reference to another structure rather than
  as a direct channel value.
- Another historical OMS environment might expose the channel choices needed
  for a future independently controlled experiment.

These are possibilities for future evidence gathering, not findings from the
current artifacts.

## Explicit unknowns

- The serialized location and encoding of Vision Chan 15.
- The scope and ownership of that assignment: project, sequence, Instrument,
  Layer, preferences, environment, referenced structure, or another context.
- The relationship between any persisted OMS data and Studio Vision's
  assignment.
- The cause of the surviving OMS Receive Channel 1 versus Vision Chan 15
  mismatch.
- Whether missing, deleted, or off-volume historical OMS setups once
  contributed to the assignment or permitted channels 14 or 15.
- The complete OMS Studio Setup grammar and its version scope.
- The generality of the observed receive-channel byte representation.
- Direct provenance for the five excluded prior channel correlations.
- Whether unexamined Galaxy binary structures contain routing-like
  information.

## Documentation and implementation consequences

This investigation improves Phoenix's evidence model, but it does not justify
an OMS parser, Studio Setup migration engine, routing decoder, new
compatibility profile, readiness or export change, or a claim of general
Studio Vision routing recovery.

The next routing-recovery experiment must wait for an independently
controllable variable or another evidence source. No parser, Core, UI, OMS
reader, migration, or application-behavior work follows from this document
alone.
