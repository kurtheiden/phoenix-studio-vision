use crate::opening::{
    compare_opening_regions, CandidateOpeningRegion, OpeningRegionComparisonSummary,
};

/// Observations for one corresponding pair of candidate ranges.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CandidateRangeAnalysis {
    pub ordinal: usize,
    pub left_start: usize,
    pub left_end: usize,
    pub right_start: usize,
    pub right_end: usize,
    pub identical: bool,
    pub unchanged_byte_count: usize,
    pub changed_byte_count: usize,
    pub inserted_byte_count: usize,
    pub removed_byte_count: usize,
    pub printable_difference_count: usize,
}

impl CandidateRangeAnalysis {
    pub fn difference_count(&self) -> usize {
        self.changed_byte_count + self.inserted_byte_count + self.removed_byte_count
    }

    pub fn contains_printable_differences(&self) -> bool {
        self.printable_difference_count != 0
    }
}

/// Experimental, non-semantic analysis of two candidate opening regions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CandidateOpeningRegionAnalysis {
    pub candidate_ranges: Vec<CandidateRangeAnalysis>,
    pub summary: OpeningRegionComparisonSummary,
}

impl CandidateOpeningRegionAnalysis {
    pub fn changed_candidate_ranges(&self) -> impl Iterator<Item = &CandidateRangeAnalysis> {
        self.candidate_ranges
            .iter()
            .filter(|range| !range.identical)
    }

    pub fn identical_candidate_ranges(&self) -> impl Iterator<Item = &CandidateRangeAnalysis> {
        self.candidate_ranges.iter().filter(|range| range.identical)
    }

    pub fn candidate_ranges_with_printable_differences(
        &self,
    ) -> impl Iterator<Item = &CandidateRangeAnalysis> {
        self.candidate_ranges
            .iter()
            .filter(|range| range.contains_printable_differences())
    }
}

/// Analyzes which corresponding candidate ranges differ without interpreting bytes.
///
/// All byte-level observations are obtained from the reusable opening-region
/// comparison API. This function only organizes those observations by candidate
/// range for further research.
pub fn analyze_candidate_opening_regions(
    left: &CandidateOpeningRegion,
    right: &CandidateOpeningRegion,
) -> CandidateOpeningRegionAnalysis {
    let comparison = compare_opening_regions(left, right);
    let mut candidate_ranges = comparison
        .identical_ranges
        .iter()
        .map(|range| CandidateRangeAnalysis {
            ordinal: range.ordinal,
            left_start: range.start,
            left_end: range.end,
            right_start: range.start,
            right_end: range.end,
            identical: true,
            unchanged_byte_count: range.bytes.len(),
            changed_byte_count: 0,
            inserted_byte_count: 0,
            removed_byte_count: 0,
            printable_difference_count: 0,
        })
        .chain(
            comparison
                .differing_ranges
                .iter()
                .map(|range| CandidateRangeAnalysis {
                    ordinal: range.left.ordinal,
                    left_start: range.left.start,
                    left_end: range.left.end,
                    right_start: range.right.start,
                    right_end: range.right.end,
                    identical: false,
                    unchanged_byte_count: range.bytes.summary.unchanged_byte_count,
                    changed_byte_count: range.bytes.summary.changed_byte_count,
                    inserted_byte_count: range.bytes.summary.inserted_byte_count,
                    removed_byte_count: range.bytes.summary.removed_byte_count,
                    printable_difference_count: range.bytes.summary.printable_difference_count,
                }),
        )
        .collect::<Vec<_>>();
    candidate_ranges.sort_by_key(|range| range.ordinal);

    CandidateOpeningRegionAnalysis {
        candidate_ranges,
        summary: comparison.summary,
    }
}

#[cfg(test)]
mod tests {
    use super::analyze_candidate_opening_regions;
    use crate::opening::{
        parse_opening_region, CANDIDATE_COUNT, CANDIDATE_END, CANDIDATE_SPACING, CANDIDATE_START,
    };

    fn fixture() -> Vec<u8> {
        let mut bytes = vec![0; CANDIDATE_END];
        bytes[CANDIDATE_START..CANDIDATE_START + 4].copy_from_slice(b"Test");
        bytes
    }

    #[test]
    fn reports_every_identical_candidate_range_in_ordinal_order() {
        let left = parse_opening_region(&fixture()).expect("left region should parse");
        let right = parse_opening_region(&fixture()).expect("right region should parse");

        let analysis = analyze_candidate_opening_regions(&left, &right);

        assert_eq!(analysis.candidate_ranges.len(), CANDIDATE_COUNT);
        for (index, range) in analysis.candidate_ranges.iter().enumerate() {
            let start = CANDIDATE_START + index * CANDIDATE_SPACING;
            assert_eq!(range.ordinal, index + 1);
            assert_eq!(
                (range.left_start, range.left_end),
                (start, start + CANDIDATE_SPACING)
            );
            assert_eq!(
                (range.right_start, range.right_end),
                (start, start + CANDIDATE_SPACING)
            );
            assert!(range.identical);
            assert_eq!(range.unchanged_byte_count, CANDIDATE_SPACING);
            assert_eq!(range.difference_count(), 0);
            assert!(!range.contains_printable_differences());
        }
        assert_eq!(analysis.summary.identical_range_count, CANDIDATE_COUNT);
        assert_eq!(analysis.summary.differing_range_count, 0);
    }

    #[test]
    fn reports_changed_bytes_and_printable_differences_by_candidate_range() {
        let left_bytes = fixture();
        let mut right_bytes = fixture();
        right_bytes[CANDIDATE_START] = b'B';
        right_bytes[CANDIDATE_START + 8] = 0xff;
        right_bytes[CANDIDATE_START + 2 * CANDIDATE_SPACING + 3] = 1;
        let left = parse_opening_region(&left_bytes).expect("left region should parse");
        let right = parse_opening_region(&right_bytes).expect("right region should parse");

        let analysis = analyze_candidate_opening_regions(&left, &right);
        let first = &analysis.candidate_ranges[0];
        let third = &analysis.candidate_ranges[2];

        assert!(!first.identical);
        assert_eq!(first.changed_byte_count, 2);
        assert_eq!(first.difference_count(), 2);
        assert_eq!(first.printable_difference_count, 1);
        assert!(first.contains_printable_differences());
        assert!(!third.identical);
        assert_eq!(third.changed_byte_count, 1);
        assert!(!third.contains_printable_differences());
        assert!(analysis.candidate_ranges[1].identical);
        assert_eq!(
            analysis
                .changed_candidate_ranges()
                .map(|range| range.ordinal)
                .collect::<Vec<_>>(),
            vec![1, 3]
        );
        assert_eq!(
            analysis
                .identical_candidate_ranges()
                .map(|range| range.ordinal)
                .collect::<Vec<_>>(),
            (2..=CANDIDATE_COUNT)
                .filter(|ordinal| *ordinal != 3)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            analysis
                .candidate_ranges_with_printable_differences()
                .map(|range| range.ordinal)
                .collect::<Vec<_>>(),
            vec![1]
        );
        assert_eq!(analysis.summary.identical_range_count, CANDIDATE_COUNT - 2);
        assert_eq!(analysis.summary.differing_range_count, 2);
        assert_eq!(analysis.summary.changed_byte_count, 3);
        assert_eq!(analysis.summary.printable_difference_count, 1);
    }

    #[test]
    fn preserves_inserted_and_removed_byte_observations_from_comparison() {
        let left = parse_opening_region(&fixture()).expect("left region should parse");
        let mut right = parse_opening_region(&fixture()).expect("right region should parse");
        right.ranges[1].bytes.insert(2, b'X');

        let inserted = analyze_candidate_opening_regions(&left, &right);
        let inserted_range = &inserted.candidate_ranges[1];
        assert_eq!(inserted_range.inserted_byte_count, 1);
        assert_eq!(inserted_range.removed_byte_count, 0);
        assert_eq!(inserted_range.printable_difference_count, 1);

        let removed = analyze_candidate_opening_regions(&right, &left);
        let removed_range = &removed.candidate_ranges[1];
        assert_eq!(removed_range.inserted_byte_count, 0);
        assert_eq!(removed_range.removed_byte_count, 1);
        assert_eq!(removed_range.printable_difference_count, 1);
    }
}
