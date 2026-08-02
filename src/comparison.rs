/// One lossless byte-level difference between two byte sequences.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ByteDifference {
    Changed {
        left_offset: usize,
        right_offset: usize,
        left_byte: u8,
        right_byte: u8,
    },
    Inserted {
        right_offset: usize,
        byte: u8,
    },
    Removed {
        left_offset: usize,
        byte: u8,
    },
}

impl ByteDifference {
    /// Reports whether either byte represented by this difference is printable ASCII.
    pub fn involves_printable_byte(&self) -> bool {
        match self {
            Self::Changed {
                left_byte,
                right_byte,
                ..
            } => is_printable(*left_byte) || is_printable(*right_byte),
            Self::Inserted { byte, .. } | Self::Removed { byte, .. } => is_printable(*byte),
        }
    }
}

/// Summary counts for a byte comparison.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ByteComparisonSummary {
    pub unchanged_byte_count: usize,
    pub changed_byte_count: usize,
    pub inserted_byte_count: usize,
    pub removed_byte_count: usize,
    pub printable_difference_count: usize,
}

impl ByteComparisonSummary {
    pub fn difference_count(&self) -> usize {
        self.changed_byte_count + self.inserted_byte_count + self.removed_byte_count
    }
}

/// A lossless comparison of two byte sequences.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ByteComparison {
    pub differences: Vec<ByteDifference>,
    pub printable_differences: Vec<ByteDifference>,
    pub summary: ByteComparisonSummary,
}

/// Compares byte sequences without assigning meaning to any byte.
///
/// Offsets are absolute and are calculated independently from the supplied
/// starting offset for each sequence.
pub fn compare_bytes(
    left_start: usize,
    left: &[u8],
    right_start: usize,
    right: &[u8],
) -> ByteComparison {
    let costs = edit_costs(left, right);
    let mut differences = Vec::new();
    let mut unchanged_byte_count = 0;
    let (mut left_index, mut right_index) = (0, 0);

    while left_index < left.len() || right_index < right.len() {
        if left_index < left.len()
            && right_index < right.len()
            && left[left_index] == right[right_index]
            && costs[left_index][right_index] == costs[left_index + 1][right_index + 1]
        {
            unchanged_byte_count += 1;
            left_index += 1;
            right_index += 1;
        } else if left_index < left.len()
            && right_index < right.len()
            && costs[left_index][right_index] == costs[left_index + 1][right_index + 1] + 1
        {
            differences.push(ByteDifference::Changed {
                left_offset: left_start + left_index,
                right_offset: right_start + right_index,
                left_byte: left[left_index],
                right_byte: right[right_index],
            });
            left_index += 1;
            right_index += 1;
        } else if left_index < left.len()
            && costs[left_index][right_index] == costs[left_index + 1][right_index] + 1
        {
            differences.push(ByteDifference::Removed {
                left_offset: left_start + left_index,
                byte: left[left_index],
            });
            left_index += 1;
        } else {
            differences.push(ByteDifference::Inserted {
                right_offset: right_start + right_index,
                byte: right[right_index],
            });
            right_index += 1;
        }
    }

    let printable_differences = differences
        .iter()
        .filter(|difference| difference.involves_printable_byte())
        .cloned()
        .collect::<Vec<_>>();
    let mut summary = ByteComparisonSummary {
        unchanged_byte_count,
        printable_difference_count: printable_differences.len(),
        ..ByteComparisonSummary::default()
    };
    for difference in &differences {
        match difference {
            ByteDifference::Changed { .. } => summary.changed_byte_count += 1,
            ByteDifference::Inserted { .. } => summary.inserted_byte_count += 1,
            ByteDifference::Removed { .. } => summary.removed_byte_count += 1,
        }
    }

    ByteComparison {
        differences,
        printable_differences,
        summary,
    }
}

fn edit_costs(left: &[u8], right: &[u8]) -> Vec<Vec<usize>> {
    let mut costs = vec![vec![0; right.len() + 1]; left.len() + 1];
    for left_index in (0..=left.len()).rev() {
        for right_index in (0..=right.len()).rev() {
            costs[left_index][right_index] = match (left.get(left_index), right.get(right_index)) {
                (None, None) => 0,
                (Some(_), None) => left.len() - left_index,
                (None, Some(_)) => right.len() - right_index,
                (Some(left_byte), Some(right_byte)) if left_byte == right_byte => {
                    costs[left_index + 1][right_index + 1]
                }
                (Some(_), Some(_)) => {
                    1 + costs[left_index + 1][right_index + 1]
                        .min(costs[left_index + 1][right_index])
                        .min(costs[left_index][right_index + 1])
                }
            };
        }
    }
    costs
}

fn is_printable(byte: u8) -> bool {
    (b' '..=b'~').contains(&byte)
}

#[cfg(test)]
mod tests {
    use super::{compare_bytes, ByteDifference};

    #[test]
    fn reports_insertions_and_removals_losslessly() {
        let inserted = compare_bytes(10, b"ac", 20, b"abc");
        assert_eq!(
            inserted.differences,
            vec![ByteDifference::Inserted {
                right_offset: 21,
                byte: b'b',
            }]
        );
        assert_eq!(inserted.summary.inserted_byte_count, 1);

        let removed = compare_bytes(20, b"abc", 10, b"ac");
        assert_eq!(
            removed.differences,
            vec![ByteDifference::Removed {
                left_offset: 21,
                byte: b'b',
            }]
        );
        assert_eq!(removed.summary.removed_byte_count, 1);
    }
}
