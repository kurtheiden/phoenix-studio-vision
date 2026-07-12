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
    println!("Printable strings (minimum length 4):");
    for string in &inspection.discovery.strings {
        println!(
            "0x{:08x}  length={}  {}",
            string.offset,
            string.value.len(),
            string.value
        );
    }
    println!("Printable string summary:");
    println!("Total strings: {}", inspection.discovery.strings.len());
    if let Some(longest) = inspection.discovery.longest_string() {
        println!(
            "Longest string: offset=0x{:08x} length={} {}",
            longest.offset,
            longest.value.len(),
            longest.value
        );
    } else {
        println!("Longest string: none");
    }
    println!(
        "Bytes in reported printable strings: {} of {} ({:.2}%)",
        inspection.discovery.printable_bytes,
        inspection.size,
        inspection.discovery.printable_percentage()
    );
    println!(
        "Shannon entropy: {:.6} bits per byte",
        inspection.discovery.entropy
    );
    Ok(())
}

struct Inspection {
    filename: String,
    full_path: PathBuf,
    size: u64,
    sha256: String,
    preview: Vec<u8>,
    discovery: DiscoveryAnalysis,
}

#[derive(Debug, PartialEq, Eq)]
struct PrintableString {
    offset: u64,
    value: String,
}

#[derive(Debug)]
struct DiscoveryAnalysis {
    strings: Vec<PrintableString>,
    printable_bytes: u64,
    total_bytes: u64,
    entropy: f64,
}

impl DiscoveryAnalysis {
    fn longest_string(&self) -> Option<&PrintableString> {
        let mut longest: Option<&PrintableString> = None;
        for string in &self.strings {
            if longest
                .map(|current| string.value.len() > current.value.len())
                .unwrap_or(true)
            {
                longest = Some(string);
            }
        }
        longest
    }

    fn printable_percentage(&self) -> f64 {
        if self.total_bytes == 0 {
            0.0
        } else {
            self.printable_bytes as f64 * 100.0 / self.total_bytes as f64
        }
    }
}

struct DiscoveryScanner {
    strings: Vec<PrintableString>,
    active_string: Vec<u8>,
    active_offset: u64,
    byte_counts: [u64; 256],
    total_bytes: u64,
}

impl DiscoveryScanner {
    fn new() -> Self {
        Self {
            strings: Vec::new(),
            active_string: Vec::new(),
            active_offset: 0,
            byte_counts: [0; 256],
            total_bytes: 0,
        }
    }

    fn update(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.byte_counts[usize::from(byte)] += 1;
            if (b' '..=b'~').contains(&byte) {
                if self.active_string.is_empty() {
                    self.active_offset = self.total_bytes;
                }
                self.active_string.push(byte);
            } else {
                self.finish_active_string();
            }
            self.total_bytes += 1;
        }
    }

    fn finish(mut self) -> DiscoveryAnalysis {
        self.finish_active_string();
        let printable_bytes = self
            .strings
            .iter()
            .map(|string| string.value.len() as u64)
            .sum();
        let entropy = shannon_entropy(&self.byte_counts, self.total_bytes);

        DiscoveryAnalysis {
            strings: self.strings,
            printable_bytes,
            total_bytes: self.total_bytes,
            entropy,
        }
    }

    fn finish_active_string(&mut self) {
        if self.active_string.len() >= 4 {
            let value = String::from_utf8(std::mem::take(&mut self.active_string))
                .expect("printable ASCII is valid UTF-8");
            self.strings.push(PrintableString {
                offset: self.active_offset,
                value,
            });
        } else {
            self.active_string.clear();
        }
    }
}

fn shannon_entropy(byte_counts: &[u64; 256], total_bytes: u64) -> f64 {
    if total_bytes == 0 {
        return 0.0;
    }

    byte_counts
        .iter()
        .filter(|&&count| count != 0)
        .map(|&count| {
            let probability = count as f64 / total_bytes as f64;
            -probability * probability.log2()
        })
        .sum()
}

#[cfg(test)]
fn analyze_bytes(bytes: &[u8]) -> DiscoveryAnalysis {
    let mut scanner = DiscoveryScanner::new();
    scanner.update(bytes);
    scanner.finish()
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
    let mut discovery = DiscoveryScanner::new();
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
        discovery.update(&buffer[..count]);
        let remaining = PREVIEW_LIMIT.saturating_sub(preview.len());
        preview.extend_from_slice(&buffer[..count.min(remaining)]);
    }

    Ok(Inspection {
        filename,
        full_path,
        size: metadata.len(),
        sha256: format!("{:x}", hasher.finalize()),
        preview,
        discovery: discovery.finish(),
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
    use super::{analyze_bytes, format_hex_dump, DiscoveryScanner, PrintableString};

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

    #[test]
    fn printable_strings_require_four_characters() {
        let analysis = analyze_bytes(b"abc\0abcd\0~~~~");
        assert_eq!(
            analysis.strings,
            vec![
                PrintableString {
                    offset: 4,
                    value: "abcd".into(),
                },
                PrintableString {
                    offset: 9,
                    value: "~~~~".into(),
                },
            ]
        );
    }

    #[test]
    fn printable_string_offsets_cover_adjacent_regions_in_file_order() {
        let analysis = analyze_bytes(b"\0First\x1fSecond\x7fLast\0");
        assert_eq!(
            analysis.strings,
            vec![
                PrintableString {
                    offset: 1,
                    value: "First".into(),
                },
                PrintableString {
                    offset: 7,
                    value: "Second".into(),
                },
                PrintableString {
                    offset: 14,
                    value: "Last".into(),
                },
            ]
        );
    }

    #[test]
    fn printable_strings_can_span_scanner_updates() {
        let mut scanner = DiscoveryScanner::new();
        scanner.update(b"\0Pho");
        scanner.update(b"enix\0");
        let analysis = scanner.finish();
        assert_eq!(
            analysis.strings,
            vec![PrintableString {
                offset: 1,
                value: "Phoenix".into(),
            }]
        );
    }

    #[test]
    fn reports_no_printable_strings_when_runs_are_too_short() {
        let analysis = analyze_bytes(b"\0ab\xffxyz\n");
        assert!(analysis.strings.is_empty());
        assert!(analysis.longest_string().is_none());
        assert_eq!(analysis.printable_bytes, 0);
        assert_eq!(analysis.printable_percentage(), 0.0);
    }

    #[test]
    fn longest_string_is_the_first_longest_in_file_order() {
        let analysis = analyze_bytes(b"four\0longest\0another\0longest");
        let longest = analysis.longest_string().expect("a longest string");
        assert_eq!(longest.offset, 5);
        assert_eq!(longest.value, "longest");
    }

    #[test]
    fn printable_percentage_counts_only_reported_strings() {
        let analysis = analyze_bytes(b"abcd\0xy\0");
        assert_eq!(analysis.printable_bytes, 4);
        assert!((analysis.printable_percentage() - 50.0).abs() < f64::EPSILON);
    }

    #[test]
    fn entropy_matches_simple_known_distributions() {
        let one_value = analyze_bytes(&[0; 8]);
        assert!(one_value.entropy.abs() < f64::EPSILON);

        let two_equal_values = analyze_bytes(&[0, 1, 0, 1]);
        assert!((two_equal_values.entropy - 1.0).abs() < f64::EPSILON);

        let four_equal_values = analyze_bytes(&[0, 1, 2, 3]);
        assert!((four_equal_values.entropy - 2.0).abs() < f64::EPSILON);
    }
}
