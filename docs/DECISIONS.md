# Decision Log

## 2026-07-10: begin with a minimal Rust CLI

**Status:** Accepted

Phoenix begins as one Rust binary crate named `phoenix`. This keeps the first
milestone portable and testable without committing to a multi-crate, GUI, or
FFI architecture.

## 2026-07-10: remain format neutral

**Status:** Accepted

The first implementation reports only generic byte-level facts. It does not
recognize Studio Vision files, assign signatures, or infer undocumented format
details.

## 2026-07-10: read-only input handling

**Status:** Accepted

Phoenix opens input through Rust's read-only `File::open` API and exposes no
operation that writes to the input path.

## 2026-07-10: use the MIT License provisionally

**Status:** Accepted, review required

The repository initially uses the MIT License. Licensing must be reviewed
before commercial distribution or before accepting significant outside
contributions. This decision does not settle contributor agreements, patent
terms, or the licensing of research fixtures.

## 2026-07-10: use SHA-256 from a focused dependency

**Status:** Accepted

The CLI uses the RustCrypto `sha2` crate rather than maintaining a custom
cryptographic implementation. Dependency scope will remain deliberately small.

## 2026-07-12: keep discovery inspection streaming and format neutral

**Status:** Accepted

The Discovery Inspector updates its SHA-256 digest, byte-frequency histogram,
printable-string scanner, and 256-byte preview during the same buffered,
read-only pass. Printable strings use only bytes from `0x20` through `0x7e`,
have a minimum length of four bytes, and remain ordered by starting offset.
Strings may span read-buffer boundaries.

Shannon entropy is calculated directly from the complete file's 256-bin byte
frequency histogram, avoiding another dependency. The `Bytes in reported
printable strings` percentage counts only bytes in strings that meet the
four-byte reporting threshold. If multiple strings share the greatest length,
the first one in file order is reported as the longest.

These values are byte-level observations only. They do not identify file
formats or assign semantic meaning to strings or surrounding bytes. Repeated
block detection remains outside this decision.

## 2026-07-17: identify Studio Vision from explicit Finder metadata evidence

**Status:** Accepted

Phoenix keeps identification separate from both format-neutral inspection and
Studio Vision structural parsing. Identification does not consider filename
extensions or infer a content signature from the available authentic sample.

Identification reports three distinct layers:

- **Observation:** the literal Finder type and creator codes, or an explicit
  absent, unsupported, malformed, or read-error state.
- **Evidence:** the documented relevance assigned to those observations. The
  `MID2` and `MIDA` pair is strong observed Studio Vision evidence; a single
  matching code with the other unavailable is provisional evidence.
- **Conclusion:** one confidence value from Very High, High, Medium, Low, or
  Unknown. The initial implementation assigns only High, Low, and Unknown.

The `MID2` type plus `MIDA` creator produces High confidence. One matching code
with the other absent or unavailable produces Low confidence. Conflicting,
missing, unsupported, malformed, or unreadable metadata produces Unknown.
Confidence is not proof that a file is Studio Vision or structurally valid.

On macOS, Phoenix reads the first eight bytes of `com.apple.FinderInfo` through
a small read-only platform adapter. Unsupported platforms report that state
without failing. AppleDouble sidecars remain outside this decision.

## 2026-08-09: keep the first event parser explicitly bounded and diagnostic

**Status:** Accepted

The first parser spike is confined to a caller-supplied Track 7 byte range and
the experimentally supported sequence `[timing VLQ][pitch][attack][release][duration VLQ]`.
It reports source offsets and provisional accumulated interval units. It does
not scan whole files, infer unsupported framing or channels, classify the
provisional third structure as a confirmed note, or emit MIDI. A command-line
interface is deferred until an evidence-backed region-selection contract is
available.

## 2026-08-09: preserve strict sequential alignment for Track 7 validation

**Status:** Accepted

Full-list validation compares screenshot row N only with binary candidate N
after the experimentally established fourth-event anchor. The first three
visible rows remain an explicitly unaligned prefix, and a parser failure is
reported rather than repaired by skipping or resynchronizing. This preserves
the distinction between screenshot ground truth, parser mechanics, and
unresolved track framing.

## 2026-08-09: defer automatic Track 7 discovery after container survey

**Status:** Accepted

Comparative scanning found a family of bounded candidate chains with recurring
marker and post-context bytes, but no validated track identity or metadata
reference selects Track 7. Phoenix therefore keeps the forensic offset
explicit and does not add heuristic discovery until one neighboring candidate
is independently matched to Studio Vision ground truth.

## 2026-08-11: retain bounded diagnostics after Track 3 #2 generalization

**Status:** Accepted

Track 3 #2 independently validates the Track 7 property and timing model and
was initially assigned to the wrong one of two duplicate 17-note regions.
Complete ground truth corrects the selected chain to `0x318b5`, validates all
84 notes, and explains the UI count as one Patch plus 84 notes. Its analogous
marker-relative value is 86 rather than 85, so Phoenix withdraws the exact
85/143 event-count interpretation and still does not add a general parser.
Patch parsing remains deferred until its local fields and timing are isolated.

## 2026-08-11: identify the Patch program byte without adding Patch parsing

**Status:** Accepted

Experiment 023 confirms the preregistered direct Program Change field at
`0x318a5`: displayed `PC 23 -> PC 24` produces `17 -> 18`, while the Patch
name and complete note stream remain unchanged. Phoenix records the field but
does not implement Patch parsing because event start/type/timing framing remains
partial. A non-adjacent-value replication is required before broader field-
behavior claims.

## 2026-08-13: accept direct Patch Program Change storage, retain parser boundary

**Status:** Accepted

Experiment 024 independently confirms the non-adjacent preregistered change:
displayed `PC 23 -> PC 100` produces aligned project byte `17 -> 64` at
`0x318a5`. Together with Experiment 023's `PC 23 -> PC 24` / `17 -> 18`, the
three controlled states establish that this Patch event's project byte directly
equals the Vision-displayed PC number. Literal `Ming Dynasty` and all known
note performance data remain unchanged.

Phoenix still does not add Patch parsing. Exact event start/end ownership,
type discrimination, and timing representation remain insufficiently
established. The next controlled experiment should change only Patch position.

## 2026-08-09: keep Track 7 boundary claims conservative

**Status:** Accepted

Local bytes match the property fields of all 143 visible rows. Rows 2–143
also have established timing prefixes; row 1's prefix is not assigned because
backward VLQ boundaries are ambiguous. The post-row-143 bytes begin with a
syntactically consumable VLQ but an out-of-range property byte, so the bounded
note model stops without calling the bytes a footer, terminator, or complete
track framing.
