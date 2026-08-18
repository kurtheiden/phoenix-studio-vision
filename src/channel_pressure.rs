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
pub struct DecodedChannelPressureEntry<'a> {
    pub entry: ChannelPressureEntry<'a>,
    pub entry_tag: Option<LocatedByte>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChannelPressureEntryError {
    Timing(VlqError),
    MissingEntryTag { offset: usize },
    WrongEntryTag { offset: usize, observed: u8 },
    MissingValue { offset: usize },
    OffsetOverflow { offset: usize },
}

/// Decodes one exact Channel Pressure entry or continuation at the current
/// cursor. It never discovers a run boundary.
pub fn decode_channel_pressure_entry_at(
    bytes: &[u8],
    cursor: usize,
    end: usize,
    explicit_status: bool,
) -> Result<(DecodedChannelPressureEntry<'_>, usize), ChannelPressureEntryError> {
    let decoded_timing =
        decode_7bit_be_vlq(bytes, cursor, end).map_err(ChannelPressureEntryError::Timing)?;
    let timing_end = cursor
        .checked_add(decoded_timing.bytes_consumed)
        .ok_or(ChannelPressureEntryError::OffsetOverflow { offset: cursor })?;
    let timing_delta = LocatedVlq {
        value: decoded_timing.value,
        bytes: &bytes[cursor..timing_end],
        range: cursor..timing_end,
    };
    let (entry_tag, value_offset) = if explicit_status {
        let Some(observed) = bytes.get(timing_end).copied().filter(|_| timing_end < end) else {
            return Err(ChannelPressureEntryError::MissingEntryTag { offset: timing_end });
        };
        if observed != CHANNEL_PRESSURE_ENTRY_TAG {
            return Err(ChannelPressureEntryError::WrongEntryTag {
                offset: timing_end,
                observed,
            });
        }
        (
            Some(LocatedByte {
                value: observed,
                offset: timing_end,
            }),
            timing_end
                .checked_add(1)
                .ok_or(ChannelPressureEntryError::OffsetOverflow { offset: timing_end })?,
        )
    } else {
        (None, timing_end)
    };
    let Some(value) = bytes
        .get(value_offset)
        .copied()
        .filter(|_| value_offset < end)
    else {
        return Err(ChannelPressureEntryError::MissingValue {
            offset: value_offset,
        });
    };
    let next = value_offset
        .checked_add(1)
        .ok_or(ChannelPressureEntryError::OffsetOverflow {
            offset: value_offset,
        })?;
    Ok((
        DecodedChannelPressureEntry {
            entry: ChannelPressureEntry {
                entry_range: cursor..next,
                timing_delta,
                pressure_value: LocatedByte {
                    value,
                    offset: value_offset,
                },
            },
            entry_tag,
        },
        next,
    ))
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

    let (first, mut cursor) =
        decode_channel_pressure_entry_at(bytes, start, end, true).map_err(|error| match error {
            ChannelPressureEntryError::Timing(source) => {
                BoundedChannelPressureError::EntryTiming(source)
            }
            ChannelPressureEntryError::MissingEntryTag { offset } => {
                BoundedChannelPressureError::MissingEntryTag { offset }
            }
            ChannelPressureEntryError::WrongEntryTag { offset, observed } => {
                BoundedChannelPressureError::WrongEntryTag {
                    offset,
                    observed,
                    expected: CHANNEL_PRESSURE_ENTRY_TAG,
                }
            }
            ChannelPressureEntryError::MissingValue { offset } => {
                BoundedChannelPressureError::MissingFirstPressureValue { offset }
            }
            ChannelPressureEntryError::OffsetOverflow { offset } => {
                BoundedChannelPressureError::OffsetOverflow { offset }
            }
        })?;
    let entry_tag = first.entry_tag.expect("explicit entry has a validated tag");
    let mut entries = vec![first.entry];

    while cursor < end {
        let entry_index = entries.len();
        let (entry, next) = decode_channel_pressure_entry_at(bytes, cursor, end, false).map_err(
            |error| match error {
                ChannelPressureEntryError::Timing(source) => {
                    BoundedChannelPressureError::ContinuationTiming {
                        entry_index,
                        cursor,
                        source,
                    }
                }
                ChannelPressureEntryError::MissingValue { offset } => {
                    BoundedChannelPressureError::MissingContinuationPressureValue {
                        entry_index,
                        offset,
                    }
                }
                ChannelPressureEntryError::OffsetOverflow { offset } => {
                    BoundedChannelPressureError::OffsetOverflow { offset }
                }
                ChannelPressureEntryError::MissingEntryTag { offset }
                | ChannelPressureEntryError::WrongEntryTag { offset, .. } => {
                    BoundedChannelPressureError::OffsetOverflow { offset }
                }
            },
        )?;
        entries.push(entry.entry);
        cursor = next;
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
