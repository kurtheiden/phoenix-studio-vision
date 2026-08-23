//! Caller-bounded decoding for the observed ordinary Controller record.
//!
//! This module decodes one exact record range. It does not discover records,
//! parse a track, accumulate absolute time, or interpret the opaque context.

use crate::patch::{LocatedByte, LocatedBytes, LocatedVlq};
use crate::track7::{decode_7bit_be_vlq, VlqError};
use std::fmt;
use std::ops::Range;

const EVENT_TAG: [u8; 2] = [0xff, 0x41];
const PAYLOAD_LENGTH: u8 = 5;
const CONTEXT_LENGTH: usize = 3;

/// Exact caller-known bounds for one ordinary Controller record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ControllerRecordBounds {
    pub record_range: Range<usize>,
}

/// One ordinary Controller record with byte-exact source provenance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundedControllerRecord<'a> {
    pub record_range: Range<usize>,
    pub timing_delta: LocatedVlq<'a>,
    pub event_tag_range: Range<usize>,
    pub payload_length: LocatedByte,
    pub payload_range: Range<usize>,
    pub context: LocatedBytes<'a>,
    pub controller_number: LocatedByte,
    pub controller_value: LocatedByte,
}

/// Deterministic failures from decoding one exact-bounded Controller record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BoundedControllerError {
    InvalidBounds {
        start: usize,
        end: usize,
        size: usize,
    },
    TimingDelta(VlqError),
    MissingTag {
        offset: usize,
        observed: Vec<u8>,
    },
    MissingPayloadLength {
        offset: usize,
    },
    WrongPayloadLength {
        offset: usize,
        observed: u8,
        expected: u8,
    },
    TruncatedContext {
        offset: usize,
        available: usize,
    },
    MissingControllerNumber {
        offset: usize,
    },
    MissingControllerValue {
        offset: usize,
    },
    TrailingBytes {
        expected_end: usize,
        bound_end: usize,
    },
}

impl fmt::Display for BoundedControllerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBounds { start, end, size } => write!(
                formatter,
                "invalid bounded Controller range 0x{start:08x}..0x{end:08x} for {size} bytes"
            ),
            Self::TimingDelta(error) => write!(formatter, "Controller timing delta: {error}"),
            Self::MissingTag { offset, observed } => write!(
                formatter,
                "expected ff 41 Controller tag at 0x{offset:08x}; observed {observed:02x?}"
            ),
            Self::MissingPayloadLength { offset } => write!(
                formatter,
                "missing Controller payload length at 0x{offset:08x}"
            ),
            Self::WrongPayloadLength {
                offset,
                observed,
                expected,
            } => write!(
                formatter,
                "Controller payload length at 0x{offset:08x} is {observed}; expected {expected}"
            ),
            Self::TruncatedContext { offset, available } => write!(
                formatter,
                "Controller context at 0x{offset:08x} has {available} of {CONTEXT_LENGTH} bytes"
            ),
            Self::MissingControllerNumber { offset } => {
                write!(formatter, "missing Controller number at 0x{offset:08x}")
            }
            Self::MissingControllerValue { offset } => {
                write!(formatter, "missing Controller value at 0x{offset:08x}")
            }
            Self::TrailingBytes {
                expected_end,
                bound_end,
            } => write!(
                formatter,
                "Controller record ends at 0x{expected_end:08x}, before bound end 0x{bound_end:08x}"
            ),
        }
    }
}

impl std::error::Error for BoundedControllerError {}

/// Decodes one caller-located ordinary Controller record without scanning.
pub fn decode_bounded_controller_record(
    bytes: &[u8],
    bounds: ControllerRecordBounds,
) -> Result<BoundedControllerRecord<'_>, BoundedControllerError> {
    let start = bounds.record_range.start;
    let end = bounds.record_range.end;
    if start >= end || end > bytes.len() {
        return Err(BoundedControllerError::InvalidBounds {
            start,
            end,
            size: bytes.len(),
        });
    }

    let timing =
        decode_7bit_be_vlq(bytes, start, end).map_err(BoundedControllerError::TimingDelta)?;
    let timing_end = start + timing.bytes_consumed;
    let timing_range = start..timing_end;
    let timing_bytes = &bytes[timing_range.clone()];

    let tag_end = timing_end.saturating_add(EVENT_TAG.len()).min(end);
    let observed_tag = &bytes[timing_end..tag_end];
    if observed_tag != EVENT_TAG {
        return Err(BoundedControllerError::MissingTag {
            offset: timing_end,
            observed: observed_tag.to_vec(),
        });
    }
    let event_tag_range = timing_end..tag_end;

    let payload_length_offset = tag_end;
    let Some(payload_length) = bytes
        .get(payload_length_offset)
        .copied()
        .filter(|_| payload_length_offset < end)
    else {
        return Err(BoundedControllerError::MissingPayloadLength {
            offset: payload_length_offset,
        });
    };
    if payload_length != PAYLOAD_LENGTH {
        return Err(BoundedControllerError::WrongPayloadLength {
            offset: payload_length_offset,
            observed: payload_length,
            expected: PAYLOAD_LENGTH,
        });
    }

    let payload_start = payload_length_offset + 1;
    let available = end - payload_start;
    if available < CONTEXT_LENGTH {
        return Err(BoundedControllerError::TruncatedContext {
            offset: payload_start,
            available,
        });
    }
    let context_range = payload_start..payload_start + CONTEXT_LENGTH;
    let number_offset = context_range.end;
    if number_offset == end {
        return Err(BoundedControllerError::MissingControllerNumber {
            offset: number_offset,
        });
    }
    let value_offset = number_offset + 1;
    if value_offset == end {
        return Err(BoundedControllerError::MissingControllerValue {
            offset: value_offset,
        });
    }
    let expected_end = value_offset + 1;
    if expected_end != end {
        return Err(BoundedControllerError::TrailingBytes {
            expected_end,
            bound_end: end,
        });
    }
    let payload_range = payload_start..expected_end;

    Ok(BoundedControllerRecord {
        record_range: bounds.record_range,
        timing_delta: LocatedVlq {
            value: timing.value,
            bytes: timing_bytes,
            range: timing_range,
        },
        event_tag_range,
        payload_length: LocatedByte {
            value: payload_length,
            offset: payload_length_offset,
        },
        payload_range,
        context: LocatedBytes {
            bytes: &bytes[context_range.clone()],
            range: context_range,
        },
        controller_number: LocatedByte {
            value: bytes[number_offset],
            offset: number_offset,
        },
        controller_value: LocatedByte {
            value: bytes[value_offset],
            offset: value_offset,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::{decode_bounded_controller_record, BoundedControllerError, ControllerRecordBounds};
    use crate::track7::VlqError;

    fn decode(bytes: &[u8]) -> Result<super::BoundedControllerRecord<'_>, BoundedControllerError> {
        decode_bounded_controller_record(
            bytes,
            ControllerRecordBounds {
                record_range: 0..bytes.len(),
            },
        )
    }

    #[test]
    fn rejects_invalid_and_empty_bounds() {
        let bytes = [0x00, 0xff, 0x41, 0x05, 0, 0, 0, 7, 127];
        for range in [1..1, std::ops::Range { start: 4, end: 3 }, 0..10] {
            assert!(matches!(
                decode_bounded_controller_record(
                    &bytes,
                    ControllerRecordBounds {
                        record_range: range
                    }
                ),
                Err(BoundedControllerError::InvalidBounds { .. })
            ));
        }
    }

    #[test]
    fn rejects_truncated_and_overlong_timing() {
        assert_eq!(
            decode(&[0x81]),
            Err(BoundedControllerError::TimingDelta(VlqError::Truncated {
                offset: 0
            }))
        );
        assert_eq!(
            decode(&[0x81, 0x81, 0x81, 0x81, 0x00]),
            Err(BoundedControllerError::TimingDelta(VlqError::TooLong {
                offset: 0,
                maximum: 4
            }))
        );
    }

    #[test]
    fn rejects_truncated_and_wrong_tags() {
        for (bytes, observed) in [
            (&[0x00][..], vec![]),
            (&[0x00, 0xff][..], vec![0xff]),
            (&[0x00, 0xfe, 0x41][..], vec![0xfe, 0x41]),
            (&[0x00, 0xff, 0x42][..], vec![0xff, 0x42]),
        ] {
            assert_eq!(
                decode(bytes),
                Err(BoundedControllerError::MissingTag {
                    offset: 1,
                    observed
                })
            );
        }
    }

    #[test]
    fn rejects_missing_and_wrong_payload_length() {
        assert_eq!(
            decode(&[0x00, 0xff, 0x41]),
            Err(BoundedControllerError::MissingPayloadLength { offset: 3 })
        );
        assert_eq!(
            decode(&[0x00, 0xff, 0x41, 0x04]),
            Err(BoundedControllerError::WrongPayloadLength {
                offset: 3,
                observed: 4,
                expected: 5
            })
        );
    }

    #[test]
    fn rejects_each_payload_truncation_and_trailing_bytes() {
        let prefix = [0x00, 0xff, 0x41, 0x05];
        for context_bytes in 0..3 {
            let mut bytes = prefix.to_vec();
            bytes.extend(std::iter::repeat(0).take(context_bytes));
            assert_eq!(
                decode(&bytes),
                Err(BoundedControllerError::TruncatedContext {
                    offset: 4,
                    available: context_bytes
                })
            );
        }

        assert_eq!(
            decode(&[0x00, 0xff, 0x41, 0x05, 0, 0, 0]),
            Err(BoundedControllerError::MissingControllerNumber { offset: 7 })
        );
        assert_eq!(
            decode(&[0x00, 0xff, 0x41, 0x05, 0, 0, 0, 7]),
            Err(BoundedControllerError::MissingControllerValue { offset: 8 })
        );
        assert_eq!(
            decode(&[0x00, 0xff, 0x41, 0x05, 0, 0, 0, 7, 127, 0]),
            Err(BoundedControllerError::TrailingBytes {
                expected_end: 9,
                bound_end: 10
            })
        );
    }

    #[test]
    fn structurally_accepts_full_byte_range_for_number_and_value() {
        let decoded = decode(&[0x00, 0xff, 0x41, 0x05, 0, 0, 0, 0xff, 0xff]).unwrap();
        assert_eq!(decoded.controller_number.value, 0xff);
        assert_eq!(decoded.controller_value.value, 0xff);
    }
}
