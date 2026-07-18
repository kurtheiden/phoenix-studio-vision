mod identification;
mod inspection;

use identification::{identify, read_finder_metadata, Confidence};
use inspection::{format_hex_dump, inspect};
use std::env;
use std::error::Error;
use std::path::PathBuf;

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
