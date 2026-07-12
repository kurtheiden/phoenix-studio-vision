use sha2::{Digest, Sha256};
use std::env;
use std::error::Error;
use std::fmt::Write as _;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

const PREVIEW_LIMIT: usize = 256;
const READ_BUFFER_SIZE: usize = 64 * 1024;

fn main() {
    if let Err(error) = run(env::args_os().skip(1)) {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run<I>(mut args: I) -> Result<(), Box<dyn Error>>
where
    I: Iterator<Item = std::ffi::OsString>,
{
    let path = PathBuf::from(
        args.next()
            .ok_or("missing file path\nusage: phoenix <FILE>")?,
    );
    if args.next().is_some() {
        return Err("expected exactly one file path\nusage: phoenix <FILE>".into());
    }

    let inspection = inspect(&path)?;
    println!("Filename: {}", inspection.filename);
    println!("Full path: {}", inspection.full_path.display());
    println!("Size: {} bytes", inspection.size);
    println!("SHA-256: {}", inspection.sha256);
    println!("Preview (first {} bytes):", inspection.preview.len());
    print!("{}", format_hex_dump(&inspection.preview));
    Ok(())
}

struct Inspection {
    filename: String,
    full_path: PathBuf,
    size: u64,
    sha256: String,
    preview: Vec<u8>,
}

fn inspect(path: &Path) -> Result<Inspection, Box<dyn Error>> {
    let mut file = File::open(path).map_err(|error| {
        io::Error::new(
            error.kind(),
            format!("cannot open '{}': {error}", path.display()),
        )
    })?;
    let metadata = file.metadata().map_err(|error| {
        io::Error::new(
            error.kind(),
            format!("cannot read metadata for '{}': {error}", path.display()),
        )
    })?;

    if !metadata.is_file() {
        return Err(format!("'{}' is not a regular file", path.display()).into());
    }
    if metadata.len() == 0 {
        return Err(format!("'{}' is empty", path.display()).into());
    }

    let full_path = path.canonicalize().map_err(|error| {
        io::Error::new(
            error.kind(),
            format!("cannot resolve full path for '{}': {error}", path.display()),
        )
    })?;
    let filename = full_path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .ok_or_else(|| format!("cannot determine filename for '{}'", path.display()))?;

    let mut hasher = Sha256::new();
    let mut preview = Vec::with_capacity(PREVIEW_LIMIT);
    let mut buffer = [0_u8; READ_BUFFER_SIZE];

    loop {
        let count = file.read(&mut buffer).map_err(|error| {
            io::Error::new(
                error.kind(),
                format!("cannot read '{}': {error}", path.display()),
            )
        })?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
        let remaining = PREVIEW_LIMIT.saturating_sub(preview.len());
        preview.extend_from_slice(&buffer[..count.min(remaining)]);
    }

    Ok(Inspection {
        filename,
        full_path,
        size: metadata.len(),
        sha256: format!("{:x}", hasher.finalize()),
        preview,
    })
}

fn format_hex_dump(bytes: &[u8]) -> String {
    let mut output = String::new();
    for (offset, chunk) in bytes.chunks(16).enumerate() {
        let _ = write!(output, "{:08x}  ", offset * 16);
        for index in 0..16 {
            if let Some(byte) = chunk.get(index) {
                let _ = write!(output, "{byte:02x} ");
            } else {
                output.push_str("   ");
            }
            if index == 7 {
                output.push(' ');
            }
        }
        output.push_str(" |");
        for byte in chunk {
            output.push(if byte.is_ascii_graphic() || *byte == b' ' {
                char::from(*byte)
            } else {
                '.'
            });
        }
        output.push_str("|\n");
    }
    output
}

#[cfg(test)]
mod tests {
    use super::format_hex_dump;

    #[test]
    fn hex_dump_formats_hex_ascii_and_padding() {
        let dump = format_hex_dump(b"Phoenix\0\xff");
        assert_eq!(
            dump,
            "00000000  50 68 6f 65 6e 69 78 00  ff                       |Phoenix..|\n"
        );
    }

    #[test]
    fn hex_dump_uses_sixteen_byte_rows() {
        let bytes: Vec<u8> = (0..17).collect();
        let dump = format_hex_dump(&bytes);
        assert_eq!(dump.lines().count(), 2);
        assert!(dump.contains("00000010  10"));
    }
}
