mod identification;
mod inspection;
mod opening;

use identification::{identify, read_finder_metadata, Confidence};
use inspection::{format_hex_dump, inspect};
use opening::inspect_opening;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

fn main() {
    if let Err(error) = run(env::args_os().skip(1)) {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run<I>(mut args: I) -> Result<(), Box<dyn Error>>
where
    I: Iterator<Item = OsString>,
{
    let first = args
        .next()
        .ok_or("missing file path\nusage: phoenix <FILE>")?;
    if first == "--inspect-candidate-opening" {
        let path = PathBuf::from(
            args.next()
                .ok_or("missing file path\nusage: phoenix --inspect-candidate-opening <FILE>")?,
        );
        if args.next().is_some() {
            return Err(
                "expected exactly one file path\nusage: phoenix --inspect-candidate-opening <FILE>"
                    .into(),
            );
        }
        return run_opening_inspection(&path);
    }

    let path = PathBuf::from(first);
    if args.next().is_some() {
        return Err("expected exactly one file path\nusage: phoenix <FILE>".into());
    }

    let inspection = inspect(&path)?;
    let identification = identify(read_finder_metadata(&inspection.full_path));

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

    println!("Studio Vision identification:");
    println!("Observation:");
    println!("  Finder type: {}", identification.metadata.file_type);
    println!("  Finder creator: {}", identification.metadata.creator);
    println!("Evidence:");
    for evidence in &identification.evidence {
        println!("  {evidence}");
    }
    println!("Conclusion: {} confidence", identification.confidence);
    if identification.confidence != Confidence::Unknown {
        println!("  This is evidence-based identification, not structural confirmation.");
    }
    Ok(())
}

fn run_opening_inspection(path: &Path) -> Result<(), Box<dyn Error>> {
    println!("EXPERIMENTAL opening name-entry candidate inspector");
    println!("Candidate structures only; no semantic interpretation is implied.");
    println!("Reports the currently documented candidate opening region described in docs/DEVICE_TABLE_RESEARCH.md.");

    let Some(entries) = inspect_opening(path)? else {
        println!("Candidate structure could not be inspected: the file does not contain the complete documented candidate opening region.");
        return Ok(());
    };

    for entry in &entries {
        println!("Candidate entry:");
        println!("  Raw file offset: 0x{:08x}", entry.offset);
        println!(
            "  Candidate byte range: 0x{:08x}--0x{:08x}",
            entry.offset,
            entry.offset + entry.bytes.len() - 1
        );
        println!("  Bytes (hex): {}", format_bytes(&entry.bytes));
        println!("  Printable ASCII sequences:");
        if entry.printable_sequences.is_empty() {
            println!("    none");
        } else {
            for sequence in &entry.printable_sequences {
                println!(
                    "    offset=0x{:08x} bytes={}",
                    sequence.offset,
                    String::from_utf8_lossy(&sequence.bytes)
                );
            }
        }
    }

    Ok(())
}

fn format_bytes(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 3 - 1);
    for (index, byte) in bytes.iter().enumerate() {
        if index != 0 {
            output.push(' ');
        }
        let _ = write!(output, "{byte:02x}");
    }
    output
}
