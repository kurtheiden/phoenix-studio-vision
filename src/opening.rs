pub const CANDIDATE_START: usize = 0x0e;
pub const CANDIDATE_SPACING: usize = 0x2d;
pub const CANDIDATE_COUNT: usize = 11;
pub const CANDIDATE_END: usize = CANDIDATE_START + CANDIDATE_SPACING * CANDIDATE_COUNT;

#[derive(Debug, PartialEq, Eq)]
/// A lossless printable-byte observation within a candidate range.
pub struct PrintableSequence {
    pub offset: usize,
    pub bytes: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
/// One currently documented fixed-width candidate range.
///
/// `start` is inclusive and `end` is exclusive. The ordinal and boundaries are
/// observational research aids, not permanent format properties.
pub struct CandidateRange {
    pub ordinal: usize,
    pub start: usize,
    pub end: usize,
    pub bytes: Vec<u8>,
    pub printable_sequences: Vec<PrintableSequence>,
}

#[derive(Debug, PartialEq, Eq)]
/// The complete currently documented candidate opening region.
///
/// `start` is inclusive and `end` is exclusive. The raw region bytes and the
/// bytes of every candidate range are retained without decoding.
pub struct CandidateOpeningRegion {
    pub start: usize,
    pub end: usize,
    pub bytes: Vec<u8>,
    pub ranges: Vec<CandidateRange>,
}

/// Parses the currently documented candidate opening region.
///
/// Fixed windows preserve the observed bytes for research comparison. They are
/// intentionally not modeled as device, instrument, OMS, or decoded records.
pub fn parse_opening_region(bytes: &[u8]) -> Option<CandidateOpeningRegion> {
    if bytes.len() < CANDIDATE_END {
        return None;
    }

    let ranges = (0..CANDIDATE_COUNT)
        .map(|index| {
            let start = CANDIDATE_START + index * CANDIDATE_SPACING;
            let end = start + CANDIDATE_SPACING;
            let range_bytes = bytes[start..end].to_vec();
            let printable_sequences = printable_sequences(start, &range_bytes);
            CandidateRange {
                ordinal: index + 1,
                start,
                end,
                bytes: range_bytes,
                printable_sequences,
            }
        })
        .collect();

    Some(CandidateOpeningRegion {
        start: CANDIDATE_START,
        end: CANDIDATE_END,
        bytes: bytes[CANDIDATE_START..CANDIDATE_END].to_vec(),
        ranges,
    })
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
        parse_opening_region, CANDIDATE_COUNT, CANDIDATE_END, CANDIDATE_SPACING, CANDIDATE_START,
    };

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
        let region = parse_opening_region(&fixture).expect("candidate range should be complete");

        assert_eq!(region.start, CANDIDATE_START);
        assert_eq!(region.end, CANDIDATE_END);
        assert_eq!(region.bytes, fixture[CANDIDATE_START..CANDIDATE_END]);
        assert_eq!(region.ranges.len(), CANDIDATE_COUNT);
        for (index, range) in region.ranges.iter().enumerate() {
            let expected_offset = CANDIDATE_START + index * CANDIDATE_SPACING;
            assert_eq!(range.ordinal, index + 1);
            assert_eq!(range.start, expected_offset);
            assert_eq!(range.end, expected_offset + CANDIDATE_SPACING);
            assert_eq!(
                range.bytes,
                fixture[expected_offset..expected_offset + CANDIDATE_SPACING]
            );
            assert_eq!(range.printable_sequences[0].offset, expected_offset);
            assert_eq!(range.printable_sequences[0].bytes, names[index]);
        }
    }

    #[test]
    fn returns_none_for_every_truncated_length_without_panicking() {
        for length in [0, 1, CANDIDATE_START, CANDIDATE_END - 1] {
            assert_eq!(parse_opening_region(&vec![0_u8; length]), None);
        }
    }
}
