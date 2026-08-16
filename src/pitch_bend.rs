//! Exact-bounded decoding for the observed stateful Pitch Bend run.
//!
//! This module requires caller-known run bounds and an `e0` entry. It does not
//! discover runs, parse arbitrary continuations, infer MIDI channel, or walk a
//! mixed event stream.

use std::ops::Range;

use crate::patch::{LocatedByte, LocatedVlq};
use crate::track7::{decode_7bit_be_vlq, VlqError};

pub const PITCH_BEND_ENTRY_TAG: u8 = 0xe0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PitchBendRunBounds {
    pub run_range: Range<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PitchBendEntry<'a> {
    pub entry_range: Range<usize>,
    pub timing_delta: LocatedVlq<'a>,
    pub pitch_lsb: LocatedByte,
    pub pitch_msb: LocatedByte,
}

impl PitchBendEntry<'_> {
    pub fn raw_value(&self) -> u16 {
        u16::from(self.pitch_lsb.value) + (u16::from(self.pitch_msb.value) << 7)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PitchBendRun<'a> {
    pub run_range: Range<usize>,
    pub entry_tag: LocatedByte,
    pub entries: Vec<PitchBendEntry<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoundedPitchBendError {
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
    MissingEntryLsb {
        offset: usize,
    },
    MissingEntryMsb {
        offset: usize,
    },
    ContinuationTiming {
        entry_index: usize,
        cursor: usize,
        source: VlqError,
    },
    MissingContinuationLsb {
        entry_index: usize,
        offset: usize,
    },
    MissingContinuationMsb {
        entry_index: usize,
        offset: usize,
    },
    OffsetOverflow {
        offset: usize,
    },
}

pub fn decode_bounded_pitch_bend_run(
    bytes: &[u8],
    bounds: PitchBendRunBounds,
) -> Result<PitchBendRun<'_>, BoundedPitchBendError> {
    let start = bounds.run_range.start;
    let end = bounds.run_range.end;
    if start >= end || end > bytes.len() {
        return Err(BoundedPitchBendError::InvalidBounds {
            start,
            end,
            size: bytes.len(),
        });
    }

    let decoded_timing =
        decode_7bit_be_vlq(bytes, start, end).map_err(BoundedPitchBendError::EntryTiming)?;
    let timing_end = start
        .checked_add(decoded_timing.bytes_consumed)
        .ok_or(BoundedPitchBendError::OffsetOverflow { offset: start })?;
    let timing_delta = LocatedVlq {
        value: decoded_timing.value,
        bytes: &bytes[start..timing_end],
        range: start..timing_end,
    };

    if timing_end == end {
        return Err(BoundedPitchBendError::MissingEntryTag { offset: timing_end });
    }
    let observed_tag = bytes[timing_end];
    if observed_tag != PITCH_BEND_ENTRY_TAG {
        return Err(BoundedPitchBendError::WrongEntryTag {
            offset: timing_end,
            observed: observed_tag,
            expected: PITCH_BEND_ENTRY_TAG,
        });
    }
    let entry_tag = LocatedByte {
        value: observed_tag,
        offset: timing_end,
    };

    let lsb_offset = timing_end
        .checked_add(1)
        .ok_or(BoundedPitchBendError::OffsetOverflow { offset: timing_end })?;
    if lsb_offset == end {
        return Err(BoundedPitchBendError::MissingEntryLsb { offset: lsb_offset });
    }
    let msb_offset = lsb_offset
        .checked_add(1)
        .ok_or(BoundedPitchBendError::OffsetOverflow { offset: lsb_offset })?;
    if msb_offset == end {
        return Err(BoundedPitchBendError::MissingEntryMsb { offset: msb_offset });
    }
    let first_entry_end = msb_offset
        .checked_add(1)
        .ok_or(BoundedPitchBendError::OffsetOverflow { offset: msb_offset })?;

    let mut entries = vec![PitchBendEntry {
        entry_range: start..first_entry_end,
        timing_delta,
        pitch_lsb: LocatedByte {
            value: bytes[lsb_offset],
            offset: lsb_offset,
        },
        pitch_msb: LocatedByte {
            value: bytes[msb_offset],
            offset: msb_offset,
        },
    }];
    let mut cursor = first_entry_end;

    while cursor < end {
        let entry_index = entries.len();
        let decoded_timing = decode_7bit_be_vlq(bytes, cursor, end).map_err(|source| {
            BoundedPitchBendError::ContinuationTiming {
                entry_index,
                cursor,
                source,
            }
        })?;
        let timing_end = cursor
            .checked_add(decoded_timing.bytes_consumed)
            .ok_or(BoundedPitchBendError::OffsetOverflow { offset: cursor })?;
        let timing_delta = LocatedVlq {
            value: decoded_timing.value,
            bytes: &bytes[cursor..timing_end],
            range: cursor..timing_end,
        };

        if timing_end == end {
            return Err(BoundedPitchBendError::MissingContinuationLsb {
                entry_index,
                offset: timing_end,
            });
        }
        let msb_offset = timing_end
            .checked_add(1)
            .ok_or(BoundedPitchBendError::OffsetOverflow { offset: timing_end })?;
        if msb_offset == end {
            return Err(BoundedPitchBendError::MissingContinuationMsb {
                entry_index,
                offset: msb_offset,
            });
        }
        let entry_end = msb_offset
            .checked_add(1)
            .ok_or(BoundedPitchBendError::OffsetOverflow { offset: msb_offset })?;
        entries.push(PitchBendEntry {
            entry_range: cursor..entry_end,
            timing_delta,
            pitch_lsb: LocatedByte {
                value: bytes[timing_end],
                offset: timing_end,
            },
            pitch_msb: LocatedByte {
                value: bytes[msb_offset],
                offset: msb_offset,
            },
        });
        cursor = entry_end;
    }

    Ok(PitchBendRun {
        run_range: start..end,
        entry_tag,
        entries,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decode(bytes: &[u8]) -> Result<PitchBendRun<'_>, BoundedPitchBendError> {
        decode_bounded_pitch_bend_run(
            bytes,
            PitchBendRunBounds {
                run_range: 0..bytes.len(),
            },
        )
    }

    #[test]
    fn accepts_single_and_multiple_entries_and_derives_raw_values() {
        let single = decode(&[0x00, 0xe0, 0x00, 0x40]).unwrap();
        assert_eq!(single.entries.len(), 1);
        assert_eq!(single.entries[0].raw_value(), 8192);
        assert_eq!(single.entries[0].entry_range, 0..4);

        let multiple = decode(&[0x81, 0x00, 0xe0, 0x3f, 0x3f, 0x01, 0xff, 0xfe]).unwrap();
        assert_eq!(multiple.entries.len(), 2);
        assert_eq!(multiple.entries[0].timing_delta.value, 128);
        assert_eq!(multiple.entries[0].raw_value(), 8127);
        assert_eq!(multiple.entries[1].timing_delta.value, 1);
        assert_eq!(multiple.entries[1].pitch_lsb.value, 0xff);
        assert_eq!(multiple.entries[1].pitch_msb.value, 0xfe);
        assert_eq!(multiple.entries[1].raw_value(), 0x7fff);
    }

    #[test]
    fn rejects_invalid_ranges() {
        let bytes = [0x00, 0xe0, 0x00, 0x40];
        for range in [
            Range { start: 0, end: 0 },
            Range { start: 2, end: 1 },
            Range { start: 0, end: 5 },
        ] {
            assert!(matches!(
                decode_bounded_pitch_bend_run(&bytes, PitchBendRunBounds { run_range: range }),
                Err(BoundedPitchBendError::InvalidBounds { .. })
            ));
        }
    }

    #[test]
    fn rejects_malformed_entry() {
        assert!(matches!(
            decode(&[0x80]),
            Err(BoundedPitchBendError::EntryTiming(VlqError::Truncated {
                offset: 0
            }))
        ));
        assert!(matches!(
            decode(&[0x81, 0x80, 0x80, 0x80, 0x00]),
            Err(BoundedPitchBendError::EntryTiming(VlqError::TooLong {
                offset: 0,
                maximum: 4,
            }))
        ));
        assert_eq!(
            decode(&[0x00]),
            Err(BoundedPitchBendError::MissingEntryTag { offset: 1 })
        );
        assert_eq!(
            decode(&[0x00, 0xdf, 0x00, 0x40]),
            Err(BoundedPitchBendError::WrongEntryTag {
                offset: 1,
                observed: 0xdf,
                expected: 0xe0,
            })
        );
        assert_eq!(
            decode(&[0x00, 0xe0]),
            Err(BoundedPitchBendError::MissingEntryLsb { offset: 2 })
        );
        assert_eq!(
            decode(&[0x00, 0xe0, 0x00]),
            Err(BoundedPitchBendError::MissingEntryMsb { offset: 3 })
        );
    }

    #[test]
    fn rejects_malformed_continuation() {
        assert!(matches!(
            decode(&[0x00, 0xe0, 0x00, 0x40, 0x80]),
            Err(BoundedPitchBendError::ContinuationTiming {
                entry_index: 1,
                cursor: 4,
                source: VlqError::Truncated { offset: 4 },
            })
        ));
        assert!(matches!(
            decode(&[0x00, 0xe0, 0x00, 0x40, 0x81, 0x80, 0x80, 0x80, 0x00]),
            Err(BoundedPitchBendError::ContinuationTiming {
                entry_index: 1,
                cursor: 4,
                source: VlqError::TooLong {
                    offset: 4,
                    maximum: 4,
                },
            })
        ));
        assert_eq!(
            decode(&[0x00, 0xe0, 0x00, 0x40, 0x01]),
            Err(BoundedPitchBendError::MissingContinuationLsb {
                entry_index: 1,
                offset: 5,
            })
        );
        assert_eq!(
            decode(&[0x00, 0xe0, 0x00, 0x40, 0x01, 0x00]),
            Err(BoundedPitchBendError::MissingContinuationMsb {
                entry_index: 1,
                offset: 6,
            })
        );
    }

    #[test]
    fn continuation_form_is_not_an_independently_decodable_run() {
        assert_eq!(
            decode(&[0x08, 0x3f, 0x3f]),
            Err(BoundedPitchBendError::WrongEntryTag {
                offset: 1,
                observed: 0x3f,
                expected: 0xe0,
            })
        );
    }
}
