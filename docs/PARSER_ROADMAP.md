# Phoenix Parser Roadmap

## Scope and decision standard

This roadmap is based only on the existing Phoenix research documentation
listed for Issue #5. It recommends a parser component only when the recorded
evidence supplies a bounded input, repeatable observations, and an output that
can be described without assigning undocumented semantics.

The evidence supports one parser target now: the repeated-name candidate area
near the beginning of a project data fork. Even this target must initially be
parsed as an experimental, observational structure. The evidence does not yet
support a device table, instrument table, track table, tempo map, audio-file
reference table, OMS structure, Name Setup, or patch-name parser.

## Priority 1: candidate opening-region parser

### Objective

Read the currently documented candidate opening region, preserve its exact
bytes, and report its candidate ranges as defined by the current research.
Each result should contain only its ordinal, absolute byte range, raw bytes,
and a conservative printable-byte view starting at the candidate offset. The
API and output should identify these as candidate entries, not devices,
instruments, OMS objects, or decoded records. Future evidence may refine the
documented region; the current measurements guide the implementation without
establishing permanent properties of the Studio Vision format.

### Evidence supporting implementation

- `PROJECT_FILE_STRUCTURE.md` records eleven printable-name starts in the
  repetitive opening area of `newest-stuff-001`, each separated by `0x2d`.
- `DEVICE_TABLE_RESEARCH.md` identifies this opening repeated-name area as the
  strongest current candidate for a first parser target and bounds the known
  observations without claiming decoded fields.
- `CANDIDATE_OPENING_COMPARISON.md` applies the same eleven complete,
  non-overlapping ranges to three authentic projects: `SMF files`, `dominion
  with samples`, and `analog only`. A printable sequence begins at every one of
  the first ten candidate offsets in all three files.
- The compared projects include both observed Finder Types (`MID2` and `MIDS`),
  differ substantially in size, and contain differing names. This makes the
  target less dependent on a single filename, device name, project size, or
  Finder Type.
- `AUTHENTIC_PROJECT_SURVEY.md` records recurring device-like vocabulary across
  the 25-project collection, while also documenting enough variation to rule
  out using any particular literal name as a signature.
- `CONTROLLED_SAVE_EXPERIMENT_001.md` and
  `CONTROLLED_SAVE_EXPERIMENTS_SUMMARY.md` show that the full 495-byte candidate
  region is present in the original and all four saved results. All four saved
  results are mutually identical in this region, even though they represent a
  no-edit save and three different edits.

### Remaining unknowns

- Whether the ranges are records, and whether `0x2d` is an actual record width.
- Whether the candidate region always starts and ends at the documented
  offsets in other projects.
- The string encoding and termination or padding rule. Some observed printable
  runs extend into bytes that may belong to other values.
- The meanings of all bytes following the initial printable sequence.
- Whether the entries represent devices, OMS-visible endpoints, a cached
  snapshot, a Name Setup, or another kind of object.
- Whether the apparent final slot is populated; two of the three opening
  comparisons contain no printable sequence at `0x0001d0`.
- Whether `MID2` and `MIDS` require different structural treatment.
- Whether the common changes produced by saving are deterministic; the current
  series contains only one no-edit save.

### Implementation risk

**Medium.** The byte window and candidate offsets are directly documented, so
raw extraction is straightforward. The risk lies in turning an inspector-defined
window into an accidental format claim. The component must reject truncated
input, retain raw bytes, avoid name-based signatures, and keep all field and
object terminology explicitly provisional.

### Expected user value

This provides Phoenix's first repeatable structured view across authentic
projects. It lets users compare the opening project state without relying on a
whole-file string dump, and it creates stable fixtures for the next controlled
experiments. Its immediate value is diagnostic rather than full project
reconstruction.

### Dependencies on other parser components

None. It requires only bounded access to project data-fork bytes. Finder
metadata can be reported separately but must not be used as a content
signature or as proof of a format variant.

### Recommended implementation order

1. Define an experimental result type that retains the complete raw candidate
   region and each candidate range's absolute offsets and bytes.
2. Implement bounds checking for the currently documented candidate opening
   region and return an explicit unsupported/truncated result when it is
   unavailable.
3. Split the region at the currently documented candidate starts, without
   assigning field meanings.
4. Add a conservative printable-byte view that never changes or discards the
   raw representation.
5. Validate byte-for-byte output against the three documented opening
   comparison fixtures and the controlled-save family.
6. Exercise additional documented authentic samples before treating the
   candidate offsets as broadly applicable, including both Finder Type groups.

## Priority 2: opening-entry name decoder, gated on new validation

### Objective

Promote the printable prefix of each opening candidate range from a diagnostic
view to a decoded name value, with a documented termination, maximum length,
and lossless representation.

This is the next component in priority order, but it is **not ready for
implementation**. It becomes eligible only after controlled device-name saves
and broader fixture checks establish the string boundary and encoding.

### Evidence supporting implementation

- The first ten aligned candidate offsets in all three projects examined by
  `CANDIDATE_OPENING_COMPARISON.md` begin with printable sequences.
- The sequences include recurring hardware and software endpoint names such as
  `JD-990`, `QuickTime Music`, and `Sound Canvas` families.
- `OMS_TECHNICAL_FINDINGS.md` documents that device names from the current OMS
  Studio Setup appear in the Names Window and that Vision saves the current
  Name Setup with a sequence. This supports investigating names in saved
  projects, but does not identify their binary representation.
- `DEVICE_TABLE_RESEARCH.md` records consistent starting alignment while also
  preserving variations in capitalization, suffixes, and apparent overrun.

### Remaining unknowns

- The name terminator or length representation and the handling of unused
  bytes.
- Whether apparent suffixes such as `e`, `ge`, `#1`, `H`, `/2`, or
  `w/Vintage` are part of a name, adjacent data, or accidental printability.
- The supported character encoding, maximum name length, truncation behavior,
  and empty-name representation.
- Whether names are copied from OMS, stored by Studio Vision, or cached from
  another source.
- Whether slot order or count is stable across project variants.

### Implementation risk

**High until gated evidence is obtained; Medium afterward.** A printable scan
is not a string decoder. Implementing the component now would embed guesses
about boundaries and semantics that the comparisons explicitly leave open.

### Expected user value

Once validated, named opening entries would be the first human-readable project
objects Phoenix can expose reliably. They would improve inspection, comparison,
and later mapping to OMS or instrument concepts.

### Dependencies on other parser components

Depends on the candidate opening-region parser and its lossless raw-entry
model. It does not depend on an OMS, instrument, patch, or track parser.

### Recommended implementation order

1. First complete the Priority 1 component.
2. Obtain repeated, one-variable controlled saves that rename one opening
   device, including shorter, longer, empty if permitted, and maximum-length
   names.
3. Verify the inferred rule across multiple `MID2` and `MIDS` authentic
   projects.
4. Add the decoded name as an optional view while retaining the raw bytes.
5. Defer semantic classification until separate evidence distinguishes device,
   OMS, Name Setup, and patch-related roles.

## Components not currently supported

The following are intentionally excluded from the implementation roadmap until
new evidence establishes repeatable structures:

- **Track-name or track-record parser.** The controlled track-name save proves
  that `Track 7 TEST` is stored literally and isolates five appended ASCII
  bytes against the other saved outputs, but it does not identify the
  containing field, corresponding original occurrence, record boundary, or
  track structure. Repeated `Meter Track`, `Tempo Track`, and `Track` strings
  and nearby `0x78` spacing are observations, not a grammar.
- **Instrument-assignment or instrument-table parser.** The controlled
  instrument edit produces output-specific bytes, but the summary does not
  identify which bytes encode the assignment. Instrument-like names are spread
  widely through projects and are not classified reliably.
- **Tempo parser.** The 120-to-130 BPM experiment identifies several
  output-specific byte ranges but no encoding, field, or containing structure.
- **Audio-reference parser.** `AUDIO_REFERENCE_FINDINGS.md` provides strong
  semantic evidence that literal `mercwork` bytes correspond to a missing
  audio prompt, and the authentic survey records other filename candidates.
  However, the three occurrences have different surrounding bytes and no
  reference boundary, path representation, extension rule, or record layout
  has been established.
- **OMS, Name Setup, device-mode, bank, patch-name, note-name, or control-name
  parser.** The manuals establish application concepts and saved behavior, and
  the projects contain related literal strings. Neither the manuals nor the
  artifact inventory establishes their project-file encodings.
- **Finder Type dispatch.** `MID2` and `MIDS` are observed metadata values, but
  their meanings and relationship to data-fork layout are unknown.
- **General section or chunk parser.** Entropy changes, repeated binary
  patterns, long exact matches, and broad save rewrites do not establish
  section boundaries, counts, pointers, or a file-level grammar.

These exclusions are not lower-priority implementation proposals. They are
research gates intended to prevent speculative parser work.

## Recommended next implementation milestone

Implement an **experimental, lossless candidate opening-region parser** that
returns the raw ranges and conservative printable views from the currently
documented candidate opening region, backed by fixtures from the three-project
opening comparison and the controlled-save family. Do not label the results as
devices and do not decode the remaining bytes. The implementation may use the
existing documented measurements as internal constants while the research
supports them, but its design should allow later evidence to refine the region.

This is the best use of development effort now because it is the only bounded
candidate repeated across multiple authentic projects and both observed Finder
Type groups. It converts the strongest existing measurements into a testable
parser seam while preserving every byte needed to correct later
interpretations. It also directly enables the highest-value next research:
controlled device-name changes can be compared at stable candidate-entry
boundaries. By contrast, implementing track, instrument, tempo, audio, or OMS
parsers now would require assumptions that the current documentation expressly
does not support.
