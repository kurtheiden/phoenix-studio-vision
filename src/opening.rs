use crate::comparison::{compare_bytes, ByteComparison, ByteDifference};

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

/// A lossless snapshot of one candidate range in a comparison.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CandidateRangeSnapshot {
    pub ordinal: usize,
    pub start: usize,
    pub end: usize,
    pub bytes: Vec<u8>,
}

/// Structured differences between a corresponding pair of candidate ranges.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CandidateRangeDifference {
    pub left: CandidateRangeSnapshot,
    pub right: CandidateRangeSnapshot,
    pub bytes: ByteComparison,
}

/// Summary counts for an opening-region comparison.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OpeningRegionComparisonSummary {
    pub candidate_range_count: usize,
    pub identical_range_count: usize,
    pub differing_range_count: usize,
    pub unchanged_byte_count: usize,
    pub changed_byte_count: usize,
    pub inserted_byte_count: usize,
    pub removed_byte_count: usize,
    pub printable_difference_count: usize,
}

impl OpeningRegionComparisonSummary {
    pub fn difference_count(&self) -> usize {
        self.changed_byte_count + self.inserted_byte_count + self.removed_byte_count
    }
}

/// A lossless, non-semantic comparison of two candidate opening regions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpeningRegionComparison {
    pub identical_ranges: Vec<CandidateRangeSnapshot>,
    pub differing_ranges: Vec<CandidateRangeDifference>,
    pub byte_differences: Vec<ByteDifference>,
    pub printable_byte_differences: Vec<ByteDifference>,
    pub summary: OpeningRegionComparisonSummary,
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

/// Compares corresponding candidate ranges without interpreting their contents.
pub fn compare_opening_regions(
    left: &CandidateOpeningRegion,
    right: &CandidateOpeningRegion,
) -> OpeningRegionComparison {
    let mut identical_ranges = Vec::new();
    let mut differing_ranges = Vec::new();
    let mut byte_differences = Vec::new();
    let mut printable_byte_differences = Vec::new();
    let mut summary = OpeningRegionComparisonSummary {
        candidate_range_count: left.ranges.len().min(right.ranges.len()),
        ..OpeningRegionComparisonSummary::default()
    };

    for (left_range, right_range) in left.ranges.iter().zip(&right.ranges) {
        let left_snapshot = CandidateRangeSnapshot::from(left_range);
        let right_snapshot = CandidateRangeSnapshot::from(right_range);
        let bytes = compare_bytes(
            left_range.start,
            &left_range.bytes,
            right_range.start,
            &right_range.bytes,
        );
        summary.unchanged_byte_count += bytes.summary.unchanged_byte_count;
        summary.changed_byte_count += bytes.summary.changed_byte_count;
        summary.inserted_byte_count += bytes.summary.inserted_byte_count;
        summary.removed_byte_count += bytes.summary.removed_byte_count;
        summary.printable_difference_count += bytes.summary.printable_difference_count;

        if bytes.differences.is_empty() {
            summary.identical_range_count += 1;
            identical_ranges.push(left_snapshot);
        } else {
            summary.differing_range_count += 1;
            byte_differences.extend(bytes.differences.iter().cloned());
            printable_byte_differences.extend(bytes.printable_differences.iter().cloned());
            differing_ranges.push(CandidateRangeDifference {
                left: left_snapshot,
                right: right_snapshot,
                bytes,
            });
        }
    }

    OpeningRegionComparison {
        identical_ranges,
        differing_ranges,
        byte_differences,
        printable_byte_differences,
        summary,
    }
}

impl From<&CandidateRange> for CandidateRangeSnapshot {
    fn from(range: &CandidateRange) -> Self {
        Self {
            ordinal: range.ordinal,
            start: range.start,
            end: range.end,
            bytes: range.bytes.clone(),
        }
    }
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
        compare_opening_regions, parse_opening_region, CANDIDATE_COUNT, CANDIDATE_END,
        CANDIDATE_SPACING, CANDIDATE_START,
    };
    use crate::comparison::ByteDifference;

    fn fixture() -> Vec<u8> {
        let mut bytes = vec![0; CANDIDATE_END];
        bytes[CANDIDATE_START..CANDIDATE_START + 4].copy_from_slice(b"Test");
        bytes
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

    #[test]
    fn compares_identical_regions() {
        let left = parse_opening_region(&fixture()).expect("left region should parse");
        let right = parse_opening_region(&fixture()).expect("right region should parse");

        let comparison = compare_opening_regions(&left, &right);

        assert_eq!(comparison.identical_ranges.len(), CANDIDATE_COUNT);
        assert!(comparison.differing_ranges.is_empty());
        assert!(comparison.byte_differences.is_empty());
        assert!(comparison.printable_byte_differences.is_empty());
        assert_eq!(comparison.summary.candidate_range_count, CANDIDATE_COUNT);
        assert_eq!(comparison.summary.identical_range_count, CANDIDATE_COUNT);
        assert_eq!(comparison.summary.differing_range_count, 0);
        assert_eq!(
            comparison.summary.unchanged_byte_count,
            CANDIDATE_SPACING * CANDIDATE_COUNT
        );
        assert_eq!(comparison.summary.difference_count(), 0);
    }

    #[test]
    fn compares_changed_regions_with_absolute_offsets_and_bytes() {
        let left_bytes = fixture();
        let mut right_bytes = fixture();
        right_bytes[CANDIDATE_START + 1] = b'a';
        right_bytes[CANDIDATE_START + CANDIDATE_SPACING + 2] = 0xff;
        let left = parse_opening_region(&left_bytes).expect("left region should parse");
        let right = parse_opening_region(&right_bytes).expect("right region should parse");

        let comparison = compare_opening_regions(&left, &right);

        assert_eq!(
            comparison.summary.identical_range_count,
            CANDIDATE_COUNT - 2
        );
        assert_eq!(comparison.summary.differing_range_count, 2);
        assert_eq!(comparison.summary.changed_byte_count, 2);
        assert_eq!(comparison.summary.inserted_byte_count, 0);
        assert_eq!(comparison.summary.removed_byte_count, 0);
        assert_eq!(
            comparison.byte_differences,
            vec![
                ByteDifference::Changed {
                    left_offset: CANDIDATE_START + 1,
                    right_offset: CANDIDATE_START + 1,
                    left_byte: b'e',
                    right_byte: b'a',
                },
                ByteDifference::Changed {
                    left_offset: CANDIDATE_START + CANDIDATE_SPACING + 2,
                    right_offset: CANDIDATE_START + CANDIDATE_SPACING + 2,
                    left_byte: 0,
                    right_byte: 0xff,
                },
            ]
        );
    }

    #[test]
    fn isolates_printable_byte_differences() {
        let left_bytes = fixture();
        let mut right_bytes = fixture();
        right_bytes[CANDIDATE_START] = b'B';
        right_bytes[CANDIDATE_START + 10] = 1;
        let left = parse_opening_region(&left_bytes).expect("left region should parse");
        let right = parse_opening_region(&right_bytes).expect("right region should parse");

        let comparison = compare_opening_regions(&left, &right);

        assert_eq!(comparison.byte_differences.len(), 2);
        assert_eq!(
            comparison.printable_byte_differences,
            vec![ByteDifference::Changed {
                left_offset: CANDIDATE_START,
                right_offset: CANDIDATE_START,
                left_byte: b'T',
                right_byte: b'B',
            }]
        );
        assert_eq!(comparison.summary.printable_difference_count, 1);
    }

    #[test]
    fn opening_comparison_reports_inserted_and_removed_bytes() {
        let left = parse_opening_region(&fixture()).expect("left region should parse");
        let mut right = parse_opening_region(&fixture()).expect("right region should parse");
        right.ranges[0].bytes.insert(1, 0xfe);

        let inserted = compare_opening_regions(&left, &right);
        assert_eq!(
            inserted.byte_differences,
            vec![ByteDifference::Inserted {
                right_offset: CANDIDATE_START + 1,
                byte: 0xfe,
            }]
        );
        assert_eq!(inserted.summary.inserted_byte_count, 1);

        let removed = compare_opening_regions(&right, &left);
        assert_eq!(
            removed.byte_differences,
            vec![ByteDifference::Removed {
                left_offset: CANDIDATE_START + 1,
                byte: 0xfe,
            }]
        );
        assert_eq!(removed.summary.removed_byte_count, 1);
    }

    #[test]
    fn truncated_inputs_do_not_produce_comparable_regions() {
        let complete = parse_opening_region(&fixture());
        let truncated = parse_opening_region(&fixture()[..CANDIDATE_END - 1]);

        assert!(complete.is_some());
        assert!(truncated.is_none());
    }
}
