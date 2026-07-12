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

