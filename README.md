# Phoenix

Phoenix is an open-source recovery toolkit for rescuing music projects created
with legacy digital audio workstations.

The initial focus is Opcode Studio Vision and Studio Vision Pro. Phoenix is
currently an exploratory inspection tool. It can report evidence-based Studio
Vision identification confidence, but recovery and structural parsing are not
implemented yet.

The read-only discovery inspector reports basic file metadata, a SHA-256
digest, a hexadecimal preview of the first 256 bytes, printable ASCII strings
found across the complete file, and summary statistics including Shannon
entropy. These are format-neutral observations; Phoenix does not yet recognize
Studio Vision from discovery output or parse Studio Vision file structures.

Phoenix also reads classic Mac Finder metadata when the platform makes it
available. The observed Finder type `MID2` together with creator `MIDA` produces
High confidence; either code alone with the other unavailable produces Low
confidence. Missing, unsupported, malformed, unreadable, or conflicting
metadata produces Unknown confidence. Filename extensions are displayed but
never used as identification evidence. Confidence is evidence-based and does
not claim that a file is definitively Studio Vision or structurally valid.

## Requirements

- A stable Rust toolchain

## Usage

```sh
cargo run --release -- /full/path/to/sample-file
```

Phoenix accepts files regardless of their name or extension. It never writes to
the input file. Printable-string results include the byte offset, byte length,
and literal value of every run of at least four bytes in the ASCII range `0x20`
through `0x7e`. The `Bytes in reported printable strings` percentage counts
only bytes in runs that meet this four-byte reporting threshold.

Finder metadata acquisition currently uses the read-only
`com.apple.FinderInfo` extended attribute on macOS. Other platforms report this
metadata as unsupported. AppleDouble `._` sidecars are not inspected.

## Development

```sh
cargo fmt --check
cargo test
```

See the [project charter](docs/PROJECT_CHARTER.md),
[roadmap](docs/ROADMAP.md), and [decision log](docs/DECISIONS.md) for current
scope and constraints.

## License

Phoenix is currently available under the MIT License. See [LICENSE](LICENSE).
