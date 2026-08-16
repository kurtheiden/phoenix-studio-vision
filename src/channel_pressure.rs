use std::ops::Range;

use crate::patch::{LocatedByte, LocatedVlq};
use crate::track7::{decode_7bit_be_vlq, VlqError};

pub const CHANNEL_PRESSURE_ENTRY_TAG: u8 = 0xd0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelPressureRunBounds {
    pub run_range: Range<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelPressureEntry<'a> {
    pub entry_range: Range<usize>,
    pub timing_delta: LocatedVlq<'a>,
    pub pressure_value: LocatedByte,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChannelPressureRun<'a> {
    pub run_range: Range<usize>,
    pub entry_tag: LocatedByte,
    pub entries: Vec<ChannelPressureEntry<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoundedChannelPressureError {
    InvalidBounds {
        start: usize,
        end: usize,
        size: usize,
    },
    EntryTiming(VlqError),
    MissingEntryTag {
        offset: usize,
    },
    WrongEntryTag {
        offset: usize,
        observed: u8,
        expected: u8,
    },
    MissingFirstPressureValue {
        offset: usize,
    },
    ContinuationTiming {
        entry_index: usize,
        cursor: usize,
        source: VlqError,
    },
    MissingContinuationPressureValue {
        entry_index: usize,
        offset: usize,
    },
    OffsetOverflow {
        offset: usize,
    },
}

pub fn decode_bounded_channel_pressure_run(
    bytes: &[u8],
    bounds: ChannelPressureRunBounds,
) -> Result<ChannelPressureRun<'_>, BoundedChannelPressureError> {
    let start = bounds.run_range.start;
    let end = bounds.run_range.end;

    if start >= end || end > bytes.len() {
        return Err(BoundedChannelPressureError::InvalidBounds {
            start,
            end,
            size: bytes.len(),
        });
    }

    let first_timing =
        decode_7bit_be_vlq(bytes, start, end).map_err(BoundedChannelPressureError::EntryTiming)?;
    let first_timing_end = start
        .checked_add(first_timing.bytes_consumed)
        .ok_or(BoundedChannelPressureError::OffsetOverflow { offset: start })?;
    let first_timing = LocatedVlq {
        value: first_timing.value,
        bytes: &bytes[start..first_timing_end],
        range: start..first_timing_end,
    };

    if first_timing_end == end {
        return Err(BoundedChannelPressureError::MissingEntryTag {
            offset: first_timing_end,
        });
    }

    let observed_tag = bytes[first_timing_end];
    if observed_tag != CHANNEL_PRESSURE_ENTRY_TAG {
        return Err(BoundedChannelPressureError::WrongEntryTag {
            offset: first_timing_end,
            observed: observed_tag,
            expected: CHANNEL_PRESSURE_ENTRY_TAG,
        });
    }
    let entry_tag = LocatedByte {
        value: observed_tag,
        offset: first_timing_end,
    };

    let first_value_offset =
        first_timing_end
            .checked_add(1)
            .ok_or(BoundedChannelPressureError::OffsetOverflow {
                offset: first_timing_end,
            })?;
    if first_value_offset == end {
        return Err(BoundedChannelPressureError::MissingFirstPressureValue {
            offset: first_value_offset,
        });
    }
    let first_entry_end =
        first_value_offset
            .checked_add(1)
            .ok_or(BoundedChannelPressureError::OffsetOverflow {
                offset: first_value_offset,
            })?;

    let mut entries = vec![ChannelPressureEntry {
        entry_range: start..first_entry_end,
        timing_delta: first_timing,
        pressure_value: LocatedByte {
            value: bytes[first_value_offset],
            offset: first_value_offset,
        },
    }];
    let mut cursor = first_entry_end;

    while cursor < end {
        let entry_index = entries.len();
        let decoded_timing = decode_7bit_be_vlq(bytes, cursor, end).map_err(|source| {
            BoundedChannelPressureError::ContinuationTiming {
                entry_index,
                cursor,
                source,
            }
        })?;
        let timing_end = cursor
            .checked_add(decoded_timing.bytes_consumed)
            .ok_or(BoundedChannelPressureError::OffsetOverflow { offset: cursor })?;
        let timing = LocatedVlq {
            value: decoded_timing.value,
            bytes: &bytes[cursor..timing_end],
            range: cursor..timing_end,
        };

        if timing_end == end {
            return Err(
                BoundedChannelPressureError::MissingContinuationPressureValue {
                    entry_index,
                    offset: timing_end,
                },
            );
        }
        let entry_end = timing_end
            .checked_add(1)
            .ok_or(BoundedChannelPressureError::OffsetOverflow { offset: timing_end })?;
        entries.push(ChannelPressureEntry {
            entry_range: cursor..entry_end,
            timing_delta: timing,
            pressure_value: LocatedByte {
                value: bytes[timing_end],
                offset: timing_end,
            },
        });
        cursor = entry_end;
    }

    Ok(ChannelPressureRun {
        run_range: start..end,
        entry_tag,
        entries,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decode(bytes: &[u8]) -> Result<ChannelPressureRun<'_>, BoundedChannelPressureError> {
        decode_bounded_channel_pressure_run(
            bytes,
            ChannelPressureRunBounds {
                run_range: 0..bytes.len(),
            },
        )
    }

    #[test]
    fn accepts_single_and_multiple_entries_without_value_range_restrictions() {
        let single = decode(&[0x00, 0xd0, 0xff]).unwrap();
        assert_eq!(single.entries.len(), 1);
        assert_eq!(single.entries[0].timing_delta.value, 0);
        assert_eq!(single.entries[0].pressure_value.value, 0xff);
        assert_eq!(single.entries[0].entry_range, 0..3);

        let multiple = decode(&[0x81, 0x00, 0xd0, 0x80, 0x01, 0xfe]).unwrap();
        assert_eq!(multiple.entries.len(), 2);
        assert_eq!(multiple.entries[0].timing_delta.value, 128);
        assert_eq!(multiple.entries[0].pressure_value.value, 0x80);
        assert_eq!(multiple.entries[1].timing_delta.value, 1);
        assert_eq!(multiple.entries[1].pressure_value.value, 0xfe);
        assert_eq!(multiple.entries[1].entry_range, 4..6);
    }

    #[test]
    fn rejects_invalid_ranges() {
        let bytes = [0x00, 0xd0, 0x01];
        for range in [
            Range { start: 0, end: 0 },
            Range { start: 2, end: 1 },
            Range { start: 0, end: 4 },
        ] {
            assert!(matches!(
                decode_bounded_channel_pressure_run(
                    &bytes,
                    ChannelPressureRunBounds { run_range: range }
                ),
                Err(BoundedChannelPressureError::InvalidBounds { .. })
            ));
        }
    }

    #[test]
    fn rejects_malformed_entry() {
        assert!(matches!(
            decode(&[0x80]),
            Err(BoundedChannelPressureError::EntryTiming(
                VlqError::Truncated { offset: 0 }
            ))
        ));
        assert!(matches!(
            decode(&[0x81, 0x80, 0x80, 0x80, 0x00]),
            Err(BoundedChannelPressureError::EntryTiming(
                VlqError::TooLong {
                    offset: 0,
                    maximum: 4,
                }
            ))
        ));
        assert_eq!(
            decode(&[0x00]),
            Err(BoundedChannelPressureError::MissingEntryTag { offset: 1 })
        );
        assert_eq!(
            decode(&[0x00, 0xcf, 0x01]),
            Err(BoundedChannelPressureError::WrongEntryTag {
                offset: 1,
                observed: 0xcf,
                expected: 0xd0,
            })
        );
        assert_eq!(
            decode(&[0x00, 0xd0]),
            Err(BoundedChannelPressureError::MissingFirstPressureValue { offset: 2 })
        );
    }

    #[test]
    fn rejects_malformed_continuation() {
        assert!(matches!(
            decode(&[0x00, 0xd0, 0x01, 0x80]),
            Err(BoundedChannelPressureError::ContinuationTiming {
                entry_index: 1,
                cursor: 3,
                source: VlqError::Truncated { offset: 3 },
            })
        ));
        assert!(matches!(
            decode(&[0x00, 0xd0, 0x01, 0x81, 0x80, 0x80, 0x80, 0x00]),
            Err(BoundedChannelPressureError::ContinuationTiming {
                entry_index: 1,
                cursor: 3,
                source: VlqError::TooLong {
                    offset: 3,
                    maximum: 4,
                },
            })
        ));
        assert_eq!(
            decode(&[0x00, 0xd0, 0x01, 0x01]),
            Err(
                BoundedChannelPressureError::MissingContinuationPressureValue {
                    entry_index: 1,
                    offset: 4,
                }
            )
        );
    }

    #[test]
    fn continuation_form_is_not_an_independently_decodable_run() {
        assert_eq!(
            decode(&[0x08, 0x05]),
            Err(BoundedChannelPressureError::WrongEntryTag {
                offset: 1,
                observed: 0x05,
                expected: 0xd0,
            })
        );
    }
}
