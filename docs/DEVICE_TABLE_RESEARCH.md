# Studio Vision Device and Instrument Table Research

This document evaluates the existing Phoenix evidence for the first binary
structure that might be recognized consistently across authentic Studio Vision
projects. It does not decode a structure and does not establish field meanings.

The evidence supports one narrow conclusion: a repeated-name area near the
beginning of multiple projects is the strongest current candidate for a first
parser target. The evidence does not yet support calling the later instrument
names a decoded instrument table or treating every nearby repeated byte pattern
as part of the same structure.

The terms **direct observation**, **documented behavior**, and **hypothesis**
are used deliberately:

- A direct observation is a measured byte offset, exact byte sequence, literal
  printable string, file measurement, or cross-file equality already recorded
  in Phoenix research.
- Documented behavior is a statement supported by the reviewed OMS
  documentation.
- A hypothesis is a candidate interpretation that requires new evidence.

## 1. Evidence Summary

### Direct observations

- [`PROJECT_FILE_STRUCTURE.md`](PROJECT_FILE_STRUCTURE.md) records 11 printable
  name starts in `newest-stuff-001` at `0x00000e`, `0x00003b`, `0x000068`,
  `0x000095`, `0x0000c2`, `0x0000ef`, `0x00011c`, `0x000149`, `0x000176`,
  `0x0001a3`, and `0x0001d0`. Every consecutive start is exactly `0x2d`
  bytes after the preceding start.
- The names at the first four of those offsets are `IAC Bus #1`,
  `JD-800s #1`, `JD-990s #1`, and `Juno-106#1`. `Quicktime Music` occurs at
  `0x000149`, and `Studio Patches pgm chg` occurs at `0x0001d0`.
- The same survey bounds this repeated opening observation to starts through
  `0x0001d0` and describes the opening through `0x0001e5` as repetitive. It
  does not identify an exact enclosing structure.
- [`PROJECT_FILE_COMPARISON.md`](PROJECT_FILE_COMPARISON.md) records device-like
  names in two other authentic projects, including `JD-800`, `JD-990
  w/Vintage`, `JD-800H`, `IAC Bus #1`, `Quicktime Music` or `QuickTime Music`,
  `General MIDI`, `Roland`, and `Studio Patches`.
- The comparison's Sample B has SHA-256
  `e2394569e865dd89c16138c5733eb2b86255eff1ef914ca5f54c6b8761743dd9`.
  That digest identifies it as project-006 in
  [`AUTHENTIC_PROJECT_SURVEY.md`](AUTHENTIC_PROJECT_SURVEY.md), whose original
  filename in the current collection is `New Dance Tracks`.
- The two compared projects have 279 distinct printable strings in common.
  The documented shared examples include `General MIDI`, `Factory Names`,
  `JD-800`, `Roland`, `Studio Patches`, `Meter Track`, `Tempo Track`,
  `Track 1`, and `Stereoww Bs`.
- The 25-project survey records the literal substring `JD-990`, `Meter Track`,
  and `Tempo Track` in every project. It records `Studio Patches` in 22 of 25
  projects and the exact substring `OMS` in the same number.
- Finder metadata divides the surveyed projects into 19 `MID2`/`MIDA` samples
  and six `MIDS`/`MIDA` samples. Both groups contain recurring device and
  instrument names. The Type-code meanings are not established.

### Documented behavior

- [`OMS_TECHNICAL_FINDINGS.md`](OMS_TECHNICAL_FINDINGS.md) records that the OMS
  Name Manager documentation describes device names from the current OMS
  Studio Setup appearing in a Names Window.
- The same documentation defines a Name Setup as a mapping between MIDI
  devices in the current Studio Setup and selected Patch Name documents.
- The documentation states that Vision saves the current Name Setup with a
  sequence. It also distinguishes device names, Patch Name documents, device
  modes, banks, patches, note names, control names, and General MIDI
  equivalents.
- The documentation does not describe the binary representation used in a
  Studio Vision project and does not use the phrase `Studio Patches` in the
  reviewed Name Manager text.
- [`ARTIFACTS.md`](ARTIFACTS.md) records Studio Vision preference-resource
  strings such as `MIDI Devices`, `MIDI Instruments`, `Instruments`,
  `Set Instrument...`, `Patch Names`, `Edit Patch`, `Find Patch...`, and
  `Copy Patches`. The installed OMS Setup contains interface strings including
  `MIDI Device Info`, `Auto-Detect Devices`, `New Device`, and `Device list`.
  These terminology matches do not establish shared storage layouts.

### Bounded hypothesis

The regularly spaced opening names are a candidate table or array of
name-bearing entries associated with devices or other OMS-visible endpoints.
This is a hypothesis, not a decoded structure. The exact start, end, entry
width, entry count, field encoding, and relationship to OMS remain unknown.

## 2. Device Name Inventory

The following inventory includes representative names already recorded in the
project surveys. “Device” here means only that a string resembles a hardware,
software endpoint, or OMS-visible device name in the available context.

| Observed name or family | Projects or source | Direct evidence |
| --- | --- | --- |
| `JD-990` family | project-001 through project-025; `newest-stuff-001` | The literal substring `JD-990` occurs in all 25 currently surveyed projects. Variants include `JD-990s #1`, `JD-990 w/Vintage`, and `Roland JD-990`. In `newest-stuff-001`, `JD-990s #1` starts at `0x000068`. In comparison Sample B/project-006, `JD-990 w/Vintage` occurs at `0x00003b` and `0x00127b`. |
| `JD-800` family | Most surveyed projects; `newest-stuff-001`; both comparison samples | Variants include `JD-800`, `JD-800H`, `JD-800s #1`, and `JD-800x`. `JD-800s #1` starts at `0x00003b` in `newest-stuff-001`; `JD-800` occurs at `0x0012b0` and `0x001322` in comparison Sample A and at `0x0011c1` and `0x001233` in Sample B/project-006. |
| `JV-1080` family | Repeated across the 25-project inventory; `newest-stuff-001` | Observed variants include `JV-1080`, `JV-10806/Vintage`, `Roland JV-1080`, and strings with `#1`. Existing research does not record a byte offset for this family in the current 25-project survey. |
| `JV-880` family | Repeated across the 25-project inventory; `newest-stuff-001` | Observed variants include `JV-880`, `JV-880/2`, `JV-880 Vintage`, and `Roland JV-880`. Existing research does not establish whether suffixes identify devices, modes, documents, or another distinction. |
| `Sound Canvas` family | Repeated across many surveyed projects | Observed variants include `Sound Canvas` and longer printable runs such as `Sound Canvassice`. The longer runs are recorded as literal bytes, not normalized field values. |
| `IAC Bus #1` | Many surveyed projects; `newest-stuff-001`; comparison Sample A | Starts at `0x00000e` in `newest-stuff-001`. Comparison Sample A records occurrences at `0x0011f6` and `0x001268`, plus `IAC Bus #1>` at `0x00000e`. |
| `Quicktime Music` / `QuickTime Music` | Multiple surveyed projects and both comparison samples | `Quicktime Music` starts at `0x000149` in `newest-stuff-001`. The comparison records capitalization differences between samples. |
| `Juno-106#1` | `newest-stuff-001` | Starts at `0x000095`, the fourth documented `0x2d`-spaced name start. |
| `Emulator III` and `SampleCell #2` | project-015 | Both occur as printable strings; `SampleCell #2` occurs repeatedly. No offsets were recorded in the existing project survey. |

### Direct comparison limits

- The inventory contains substrings and representative variants, not a set of
  canonical device identities.
- A suffix such as `#1`, `H`, `x`, `/2`, or `w/Vintage` is not decoded.
- A name can occur more than once in one file and in more than one apparent
  context. Repetition does not establish references, copies, or ownership.
- `IAC Bus #1` and QuickTime Music are software-visible names rather than
  physical synthesizer model names. The candidate area may therefore be
  broader than a hardware-device list.

## 3. Instrument Name Inventory

The existing evidence contains many patch-like, instrument-like, and track-like
names. The present research cannot determine which category applies to every
string.

### Recurring names across projects

| Literal name or family | Representative samples | Direct observation |
| --- | --- | --- |
| `Polysynth` | project-001, project-002, project-004, project-006, project-014, project-017, project-025 | The same literal name occurs in multiple projects with both `MID2` and `MIDS` Finder Types. |
| `Warm Pad` | project-001, project-002, project-004, project-017 | The same capitalization is recorded in several projects; lower-case `warm pad` also occurs in project-004 and project-017. |
| `Piano 3` | project-002, project-004, project-008, project-025 | The literal name recurs among many other piano-related strings. |
| `Multi Bass` | project-002, project-006, project-014 | The literal name occurs in three surveyed projects. |
| `Filter Bass` | project-004, project-017 | The literal name occurs in both projects, which also share OMS Applications and Studio 5 Patches strings. That co-occurrence does not establish a relationship. |
| `ORBit Pad` | project-015, project-017 | The literal name occurs in two projects with different Finder Types. |
| `Killer Pad` | project-002, project-004, project-020 | The literal name occurs in three projects. |
| `Synth Bass` family | Multiple projects | Variants include `Synth Bass 1`, `Synth Bass 2`, `Synth Bass1`, and project-specific lower-case phrases. These are not normalized as equivalent values. |
| `Strings` family | Multiple projects | Variants include `Strings`, `Strings Arco`, `Syn. Strings1`, `Syn. Strings2`, `Velo Syn Strings`, `JP-8 Strings`, and lower-case `strings`. |

### Repeated names with known offsets in `newest-stuff-001`

- `Sharp Bass` starts at `0x005111` and `0x0100c8`.
- `Saw Mass` starts at `0x01f070` and `0x0201a7`.
- `Empty Patch` starts at `0x0057ac` and `0x0263b9`.
- Other recorded examples include `Velo-Crunch` at `0x0031d4`, `Dist Gtr 1`
  at `0x005cde`, `SquareLead 1` at `0x00acaf`, `ORBit Pad` at `0x00c056`,
  `JP-8 Pad` at `0x00e8d3`, `Pick Bass` at `0x0115e8`, `D-50 Stack` at
  `0x013c08`, `Multi Bass` at `0x021026`, and `Bark Bass` at `0x022068`.

### Interpretation limit

OMS documentation establishes that patch names, device names, modes, banks,
note names, and control names are distinct concepts. The printable scan does
not establish which observed project strings are instrument names, patch names,
track names, Name Setup content, or cached display text. For that reason, this
document uses “instrument name” as an inventory heading, not as a decoded field
classification.

## 4. Repeated Binary Patterns

### Opening `0x2d`-spaced name starts

**Direct observation:** In `newest-stuff-001`, 11 printable names start at:

```text
0x00000e  0x00003b  0x000068  0x000095  0x0000c2  0x0000ef
0x00011c  0x000149  0x000176  0x0001a3  0x0001d0
```

Each difference is `0x2d`. The first four literal names and two later names are
documented in the Evidence Summary. The bytes preceding, following, and between
the strings were not decoded.

**Hypothesis:** These starts may identify fixed-width name-bearing entries.
The observation alone does not establish that `0x2d` is an entry size: a name
could appear at the same relative offset inside larger entries, adjacent
structures could coincide with this spacing, or the final name could belong to
a different object.

### `0x29`-spaced repeated byte neighborhoods

**Direct observation:** The two-project comparison records 85 occurrences in
each file of each of these exact 16-byte sequences:

```text
24 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
10 00 00 00 24 00 00 00 00 00 00 00 00 00 00 00
00 24 00 00 00 00 00 00 00 00 00 00 00 00 00 00
```

Representative starts for the first sequence are `0x0001fb`, `0x000224`, and
`0x00024d` in comparison Sample A, and `0x0001ce`, `0x0001f7`, and
`0x000220` in Sample B/project-006. Consecutive starts differ by `0x29`.
The Sample B starts shown are `0x2d` earlier than their Sample A counterparts.

**Direct observation:** These three 16-byte sequences overlap descriptions of
the same nearby bytes. Their equal counts are not three independent entry
counts.

**Unknown:** No existing evidence establishes that these `0x29`-spaced bytes
are device entries, instrument entries, track entries, unused slots, or any
other semantic unit.

### Repeated track-associated patterns

**Direct observation:** In `newest-stuff-001`, `Meter Track`, `Tempo Track`,
and following `Track` strings have starts separated by `0x78` in the first
documented group beginning at `0x002875`. Comparable groups recur later.

Two exact 16-byte sequences recur near multiple such groups:

```text
ff ff 80 00 00 14 00 c8 00 c8 00 00 00 00 80 00  (106 occurrences)
ff ff ff ff ff ff ff ff ff ff ff ff ff ff 54 72  (93 occurrences)
```

Their documented early starts include `0x00289d` and `0x002957`, respectively.
These patterns demonstrate repeatable binary neighborhoods around track-like
names, not an instrument record format.

### Pattern-separation rule

The `0x2d`, `0x29`, and `0x78` observations must remain separate until exact
byte comparisons show a relationship. Treating them as one record grammar
would exceed the available evidence.

## 5. Cross-project Comparisons

### Shared name vocabulary

- `JD-990` is present as a literal substring in project-001 through
  project-025. It also occurs in `newest-stuff-001`; project-006 is the Sample B
  file in the earlier comparison.
- `JD-800`, `JV-1080`, `JV-880`, Sound Canvas-related names, General MIDI,
  Roland, Factory Names, and Studio Patches recur across multiple projects.
- The two earlier comparison samples share 279 distinct printable strings,
  including both device-related and track- or instrument-related names.
- The 25-project survey spans two Finder Type groups, a size range from 10,107
  to 881,556 bytes, and an entropy range from 3.716483 to 6.579512 bits/byte.
  Recurring device vocabulary is therefore not limited to one observed file
  size or Finder Type.

### Exact cross-file binary evidence

- The two comparison samples have the same counts for the three documented
  `0x29`-spaced 16-byte sequences.
- Numerous exact early matches in those files are displaced by `0x2d`,
  including documented 40-byte matches. This is an offset relationship, not a
  decoded insertion or field.
- Their first two 4 KiB windows have similar entropy and zero-byte shares, and
  both contain device, bundle, Factory Names, and General MIDI strings.
- Later exact matching regions and shared instrument-like strings demonstrate
  recurring content but do not locate a universal instrument table.

### Differences that constrain a first parser

- The string at `0x00000e` is `IAC Bus #1>` in comparison Sample A and
  `JD-800H` in Sample B/project-006. A parser cannot rely on one fixed device
  name as a signature.
- `Quicktime Music` and `QuickTime Music` differ in capitalization between the
  compared samples.
- Device-family strings include suffix variations and printable bytes that may
  extend beyond a human-readable name.
- Some projects contain `OMS`, bundle, and Factory Names strings while
  project-007, project-016, and project-019 do not contain the exact `OMS` or
  bundle substrings in the existing printable scan.
- The six `MIDS` projects and 19 `MID2` projects must not be assumed to share
  identical boundaries merely because their name vocabularies overlap.

## 6. Candidate Structure Boundaries

### Highest-confidence observational boundary

For `newest-stuff-001`, the only currently bounded candidate is the opening
repeated-name area:

- first known printable-name start: `0x00000e`;
- last known `0x2d`-spaced printable-name start: `0x0001d0`;
- observational range previously described as repetitive: file start through
  `0x0001e5`.

This range is suitable for targeted comparison because all reported offsets
are direct measurements. It is not an established table boundary. The 14 bytes
before the first name, the 21 bytes after the last recorded start, and the
actual length of the last string are not header, trailer, or padding fields
unless future evidence demonstrates those roles.

### Related but separate candidate range

In the two-project comparison, the first reported `0x29`-spaced pattern starts
at `0x0001fb` in Sample A and `0x0001ce` in Sample B/project-006. Each file has
85 total occurrences, and the documented early examples are equally spaced.
The existing document does not state an exact enclosing boundary, so this
research does not supply one.

### Instrument boundary status

No candidate instrument-table boundary meets the same confidence level.
Instrument-like names occur from `0x0031d4` through at least `0x0263b9` in
`newest-stuff-001`, interspersed with repeated track labels and
punctuation-heavy runs. Some exact names recur far apart. Those observations
are consistent with several possible organizations and establish none of them.

## 7. Explicit Unknowns

- Whether the opening `0x2d`-spaced names are stored in a table, array, list,
  cache, snapshot, or another organization is unknown.
- The bytes before, after, and between every observed name remain undecoded.
- No count, length, type, status, index, pointer, offset, checksum, padding, or
  terminator field has been identified.
- It is unknown whether names are fixed-width, variable-width, truncated,
  null-terminated, Pascal-style, or represented in more than one way.
- It is unknown whether `0x2d`, `0x29`, or `0x78` is a record size. No
  relationship among those spacings is established.
- It is unknown whether device names in a project are copied from an OMS Studio
  Setup, retained from a Name Setup, entered in Studio Vision, or cached from a
  Patch Name document.
- It is unknown whether `Studio Patches`, `Factory Names`, and General MIDI
  strings are document names, paths, subscriptions, display labels, or another
  kind of data.
- It is unknown which visible sound names are Studio Vision instruments, OMS
  patches, track labels, device patches, note names, or unused cached strings.
- The relationship between repeated track-associated binary patterns and
  nearby instrument-like names is unknown.
- No controlled save experiment has isolated a device rename, instrument
  rename, Name Setup change, patch subscription, bank change, or device-mode
  change.
- No existing evidence establishes one universal boundary across `MID2` and
  `MIDS` projects.
- The observed strings do not establish that every authentic project contains
  the candidate opening structure, even when related names occur elsewhere.

## 8. Recommendations for the first parser implementation

No parser should be implemented until the candidate is validated with
additional read-only comparisons or controlled-save fixtures. When that
evidence is available, the first implementation should remain deliberately
observational:

1. Use at least one `MID2` and one `MIDS` project, plus multiple projects of
   each observed metadata group, as fixtures. Identify fixtures by SHA-256 and
   never modify the authentic copies.
2. Record exact bytes for the complete opening candidate area in each fixture.
   Verify whether repeated starts, not merely familiar names, occur at stable
   relative positions.
3. Test the candidate using projects whose first visible names differ. Do not
   use `JD-990`, `JD-800`, `IAC Bus #1`, Finder Type, or any filename as a
   content signature.
4. Before assigning fields, obtain controlled saves that change one device
   name at a time while holding the rest of the project and OMS environment
   constant. Repeat the experiment for insertion, deletion, and a maximum-
   length name if the original software permits those operations.
5. Initially expose only raw candidate entries: file offset, exact bytes, and
   conservatively extracted printable bytes. Label the output experimental and
   avoid returning semantic device or instrument objects.
6. Reject or report ambiguity when spacing, boundaries, or printable-byte
   behavior differs from the validated fixtures. Do not silently force an
   unknown file into the candidate model.
7. Keep the `0x2d` opening-name candidate, the `0x29` repeated patterns, and the
   `0x78` track-associated patterns as separate investigations until exact
   evidence connects them.
8. Defer instrument-table parsing. First establish a bounded device/name-entry
   representation; then use controlled changes and cross-project exact-byte
   comparisons to determine whether instrument-like names form a distinct
   structure.

The smallest defensible first milestone is therefore a read-only experimental
inspector for a validated opening name-entry candidate, not a general Studio
Vision device or instrument parser.

### Experimental inspector interface

The opt-in research interface is:

```text
phoenix --inspect-candidate-opening <file>
```

It reports the currently documented candidate opening region described in this
document, including raw offsets, exact bytes, and printable ASCII sequences.
Its output is labeled experimental and does not assign semantic meanings or
establish structure boundaries. The default Phoenix inspection mode is
unchanged when this option is not used.
