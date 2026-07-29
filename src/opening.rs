use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub(crate) const CANDIDATE_START: usize = 0x0e;
pub(crate) const CANDIDATE_SPACING: usize = 0x2d;
pub(crate) const CANDIDATE_COUNT: usize = 11;
pub(crate) const CANDIDATE_END: usize = CANDIDATE_START + CANDIDATE_SPACING * CANDIDATE_COUNT;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct PrintableSequence {
    pub(crate) offset: usize,
    pub(crate) bytes: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct CandidateEntry {
    pub(crate) offset: usize,
    pub(crate) bytes: Vec<u8>,
    pub(crate) printable_sequences: Vec<PrintableSequence>,
}

/// Reads only the documented candidate opening range.
///
/// Fixed windows preserve the observed bytes for research comparison. They are
/// intentionally not modeled as device, instrument, OMS, or decoded records.
pub(crate) fn inspect_opening(path: &Path) -> io::Result<Option<Vec<CandidateEntry>>> {
    let mut file = File::open(path).map_err(|error| {
        io::Error::new(
            error.kind(),
            format!("cannot open '{}': {error}", path.display()),
        )
    })?;
    let mut opening = vec![0_u8; CANDIDATE_END];
    let mut bytes_read = 0;

    while bytes_read < opening.len() {
        let count = file.read(&mut opening[bytes_read..]).map_err(|error| {
            io::Error::new(
                error.kind(),
                format!("cannot read '{}': {error}", path.display()),
            )
        })?;
        if count == 0 {
            return Ok(None);
        }
        bytes_read += count;
    }

    let entries = (0..CANDIDATE_COUNT)
        .map(|index| {
            let offset = CANDIDATE_START + index * CANDIDATE_SPACING;
            let bytes = opening[offset..offset + CANDIDATE_SPACING].to_vec();
            let printable_sequences = printable_sequences(offset, &bytes);
            CandidateEntry {
                offset,
                bytes,
                printable_sequences,
            }
        })
        .collect();

    Ok(Some(entries))
}

fn printable_sequences(entry_offset: usize, bytes: &[u8]) -> Vec<PrintableSequence> {
    let mut sequences = Vec::new();
    let mut start = None;

    for (index, byte) in bytes.iter().copied().enumerate() {
        if (b' '..=b'~').contains(&byte) {
            start.get_or_insert(index);
        } else if let Some(sequence_start) = start.take() {
            sequences.push(PrintableSequence {
                offset: entry_offset + sequence_start,
                bytes: bytes[sequence_start..index].to_vec(),
            });
        }
    }
    if let Some(sequence_start) = start {
        sequences.push(PrintableSequence {
            offset: entry_offset + sequence_start,
            bytes: bytes[sequence_start..].to_vec(),
        });
    }

    sequences
}

#[cfg(test)]
mod tests {
    use super::{
        inspect_opening, CANDIDATE_COUNT, CANDIDATE_END, CANDIDATE_SPACING, CANDIDATE_START,
    };
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temporary_path(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after Unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "phoenix-opening-test-{}-{unique}-{name}",
            std::process::id()
        ))
    }

    #[test]
    fn extracts_documented_opening_pattern_fixture_without_changing_bytes() {
        // These names and offsets reproduce published research observations;
        // no authentic project bytes are committed as a test fixture.
        let names: [&[u8]; CANDIDATE_COUNT] = [
            b"IAC Bus #1",
            b"JD-800s #1",
            b"JD-990s #1",
            b"Juno-106#1",
            b"entry four",
            b"entry five",
            b"entry six",
            b"Quicktime Music",
            b"entry eight",
            b"entry nine",
            b"Studio Patches pgm chg",
        ];
        let mut fixture = vec![0xa5; CANDIDATE_END + 8];
        for (index, name) in names.iter().enumerate() {
            let offset = CANDIDATE_START + index * CANDIDATE_SPACING;
            fixture[offset..offset + CANDIDATE_SPACING].fill(0);
            fixture[offset..offset + name.len()].copy_from_slice(name);
            fixture[offset + CANDIDATE_SPACING - 1] = index as u8;
        }
        let path = temporary_path("documented-pattern");
        fs::write(&path, &fixture).expect("fixture should be written");

        let entries = inspect_opening(&path)
            .expect("inspection should succeed")
            .expect("candidate range should be complete");

        assert_eq!(entries.len(), CANDIDATE_COUNT);
        for (index, entry) in entries.iter().enumerate() {
            let expected_offset = CANDIDATE_START + index * CANDIDATE_SPACING;
            assert_eq!(entry.offset, expected_offset);
            assert_eq!(
                entry.bytes,
                fixture[expected_offset..expected_offset + CANDIDATE_SPACING]
            );
            assert_eq!(entry.printable_sequences[0].offset, expected_offset);
            assert_eq!(entry.printable_sequences[0].bytes, names[index]);
        }
        assert_eq!(
            fs::read(&path).expect("fixture should remain readable"),
            fixture
        );

        fs::remove_file(path).expect("fixture should be removed");
    }

    #[test]
    fn returns_none_for_every_truncated_length_without_panicking() {
        let path = temporary_path("truncated");
        for length in [0, 1, CANDIDATE_START, CANDIDATE_END - 1] {
            fs::write(&path, vec![0_u8; length]).expect("fixture should be written");
            assert_eq!(
                inspect_opening(&path).expect("inspection should succeed"),
                None
            );
        }

        fs::remove_file(path).expect("fixture should be removed");
    }
}
