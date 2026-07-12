# Phoenix

Phoenix is an open-source recovery toolkit for rescuing music projects created
with legacy digital audio workstations.

The initial focus is Opcode Studio Vision and Studio Vision Pro. Phoenix is
currently an exploratory, format-neutral inspection tool: recognition and
recovery of Studio Vision files are not implemented yet.

The first milestone is a read-only inspection command that reports basic file
metadata, a SHA-256 digest, and a hexadecimal preview of the first 256 bytes.

## Requirements

- A stable Rust toolchain

## Usage

```sh
cargo run --release -- /full/path/to/sample-file
```

Phoenix accepts files regardless of their name or extension. It never writes to
the input file.

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
