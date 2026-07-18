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
