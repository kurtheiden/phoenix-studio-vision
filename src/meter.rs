//! Exact-bounded decoding for the observed Studio Vision initial Meter form.
//!
//! This module does not discover Meter structures, parse a Meter map, parse
//! secondary copies, interpret position, or apply historical SMF export policy.

use std::fmt;
use std::ops::Range;

use crate::patch::LocatedByte;

const REPRESENTATION_LENGTH: usize = 8;
const INITIAL_POSITION_BYTE: u8 = 0x00;
const FF_TAG: u8 = 0xff;
const METER_TAG: u8 = 0x58;
const PAYLOAD_LENGTH: u8 = 0x04;

/// Exact caller-known bounds for one initial Meter representation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InitialMeterBounds {
    pub event_range: Range<usize>,
}

/// One bounded initial Meter representation with byte-exact provenance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InitialMeterEvent {
    pub event_range: Range<usize>,
    pub initial_position_byte: LocatedByte,
    pub ff_tag: LocatedByte,
    pub meter_tag: LocatedByte,
    pub payload_length: LocatedByte,
    pub numerator: LocatedByte,
    pub denominator_exponent: LocatedByte,
    pub third_payload: LocatedByte,
    pub fourth_payload: LocatedByte,
}

impl InitialMeterEvent {
    /// Derives `2^dd` when the result fits in `u64`.
    pub fn denominator(&self) -> Option<u64> {
        1_u64.checked_shl(u32::from(self.denominator_exponent.value))
    }
}

/// Deterministic failures from decoding one bounded initial Meter form.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BoundedInitialMeterError {
    InvalidBounds {
        start: usize,
        end: usize,
        size: usize,
    },
    WrongRepresentationLength {
        start: usize,
        end: usize,
        observed: usize,
        expected: usize,
    },
    UnsupportedInitialPositionByte {
        offset: usize,
        observed: u8,
        expected: u8,
    },
    WrongFfTag {
        offset: usize,
        observed: u8,
        expected: u8,
    },
    WrongMeterTag {
        offset: usize,
        observed: u8,
        expected: u8,
    },
    WrongPayloadLength {
        offset: usize,
        observed: u8,
        expected: u8,
    },
}

impl fmt::Display for BoundedInitialMeterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBounds { start, end, size } => write!(
                formatter,
                "invalid bounded initial Meter range 0x{start:08x}..0x{end:08x} for {size} bytes"
            ),
            Self::WrongRepresentationLength {
                start,
                end,
                observed,
                expected,
            } => write!(
                formatter,
                "bounded initial Meter range 0x{start:08x}..0x{end:08x} has {observed} bytes; expected {expected}"
            ),
            Self::UnsupportedInitialPositionByte {
                offset,
                observed,
                expected,
            } => write!(
                formatter,
                "unsupported initial Meter position byte {observed:02x} at 0x{offset:08x}; expected {expected:02x}"
            ),
            Self::WrongFfTag {
                offset,
                observed,
                expected,
            } => write!(
                formatter,
                "wrong initial Meter ff tag {observed:02x} at 0x{offset:08x}; expected {expected:02x}"
            ),
            Self::WrongMeterTag {
                offset,
                observed,
                expected,
            } => write!(
                formatter,
                "wrong initial Meter tag {observed:02x} at 0x{offset:08x}; expected {expected:02x}"
            ),
            Self::WrongPayloadLength {
                offset,
                observed,
                expected,
            } => write!(
                formatter,
                "wrong initial Meter payload length {observed} at 0x{offset:08x}; expected {expected}"
            ),
        }
    }
}

impl std::error::Error for BoundedInitialMeterError {}

/// Decodes one exact caller-bounded initial Meter representation without scanning.
pub fn decode_bounded_initial_meter(
    bytes: &[u8],
    bounds: InitialMeterBounds,
) -> Result<InitialMeterEvent, BoundedInitialMeterError> {
    let start = bounds.event_range.start;
    let end = bounds.event_range.end;
    if start >= end || end > bytes.len() {
        return Err(BoundedInitialMeterError::InvalidBounds {
            start,
            end,
            size: bytes.len(),
        });
    }

    let observed_length = end - start;
    if observed_length != REPRESENTATION_LENGTH {
        return Err(BoundedInitialMeterError::WrongRepresentationLength {
            start,
            end,
            observed: observed_length,
            expected: REPRESENTATION_LENGTH,
        });
    }

    let initial_position_byte = located_byte(bytes, start);
    if initial_position_byte.value != INITIAL_POSITION_BYTE {
        return Err(BoundedInitialMeterError::UnsupportedInitialPositionByte {
            offset: initial_position_byte.offset,
            observed: initial_position_byte.value,
            expected: INITIAL_POSITION_BYTE,
        });
    }

    let ff_tag = located_byte(bytes, start + 1);
    if ff_tag.value != FF_TAG {
        return Err(BoundedInitialMeterError::WrongFfTag {
            offset: ff_tag.offset,
            observed: ff_tag.value,
            expected: FF_TAG,
        });
    }

    let meter_tag = located_byte(bytes, start + 2);
    if meter_tag.value != METER_TAG {
        return Err(BoundedInitialMeterError::WrongMeterTag {
            offset: meter_tag.offset,
            observed: meter_tag.value,
            expected: METER_TAG,
        });
    }

    let payload_length = located_byte(bytes, start + 3);
    if payload_length.value != PAYLOAD_LENGTH {
        return Err(BoundedInitialMeterError::WrongPayloadLength {
            offset: payload_length.offset,
            observed: payload_length.value,
            expected: PAYLOAD_LENGTH,
        });
    }

    Ok(InitialMeterEvent {
        event_range: bounds.event_range,
        initial_position_byte,
        ff_tag,
        meter_tag,
        payload_length,
        numerator: located_byte(bytes, start + 4),
        denominator_exponent: located_byte(bytes, start + 5),
        third_payload: located_byte(bytes, start + 6),
        fourth_payload: located_byte(bytes, start + 7),
    })
}

fn located_byte(bytes: &[u8], offset: usize) -> LocatedByte {
    LocatedByte {
        value: bytes[offset],
        offset,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        decode_bounded_initial_meter, BoundedInitialMeterError, InitialMeterBounds,
        InitialMeterEvent,
    };

    fn decode(bytes: &[u8]) -> Result<InitialMeterEvent, BoundedInitialMeterError> {
        decode_bounded_initial_meter(
            bytes,
            InitialMeterBounds {
                event_range: 0..bytes.len(),
            },
        )
    }

    #[test]
    fn preserves_payload_bytes_and_derives_denominator_safely() {
        let event = decode(&[0x00, 0xff, 0x58, 0x04, 0x07, 0x03, 0xa5, 0x37]).unwrap();
        assert_eq!(event.numerator.value, 0x07);
        assert_eq!(event.denominator_exponent.value, 0x03);
        assert_eq!(event.denominator(), Some(8));
        assert_eq!(event.third_payload.value, 0xa5);
        assert_eq!(event.fourth_payload.value, 0x37);

        let high = decode(&[0x00, 0xff, 0x58, 0x04, 0x01, 0xff, 0x00, 0x00]).unwrap();
        assert_eq!(high.denominator_exponent.value, 0xff);
        assert_eq!(high.denominator(), None);
    }

    #[test]
    fn preserves_absolute_offsets_from_a_nonzero_event_start() {
        let bytes = [
            0xaa, 0xbb, 0x00, 0xff, 0x58, 0x04, 0x06, 0x03, 0x06, 0x08, 0xcc,
        ];
        let event = decode_bounded_initial_meter(&bytes, InitialMeterBounds { event_range: 2..10 })
            .unwrap();

        assert_eq!(event.event_range, 2..10);
        let fields = [
            event.initial_position_byte,
            event.ff_tag,
            event.meter_tag,
            event.payload_length,
            event.numerator,
            event.denominator_exponent,
            event.third_payload,
            event.fourth_payload,
        ];
        for (index, field) in fields.into_iter().enumerate() {
            assert_eq!(field.offset, index + 2);
        }
        assert_eq!(event.fourth_payload.offset + 1, event.event_range.end);
    }

    #[test]
    fn rejects_invalid_and_wrong_length_bounds_before_indexing() {
        let bytes = [0x00, 0xff, 0x58, 0x04, 0x04, 0x02, 0x08, 0x08, 0xaa];
        for range in [0..0, std::ops::Range { start: 5, end: 4 }, 0..10] {
            assert_eq!(
                decode_bounded_initial_meter(
                    &bytes,
                    InitialMeterBounds {
                        event_range: range.clone(),
                    },
                ),
                Err(BoundedInitialMeterError::InvalidBounds {
                    start: range.start,
                    end: range.end,
                    size: bytes.len(),
                })
            );
        }

        for range in [0..1, 0..4, 0..7, 0..9] {
            let observed = range.end - range.start;
            assert_eq!(
                decode_bounded_initial_meter(
                    &bytes,
                    InitialMeterBounds {
                        event_range: range.clone(),
                    },
                ),
                Err(BoundedInitialMeterError::WrongRepresentationLength {
                    start: range.start,
                    end: range.end,
                    observed,
                    expected: 8,
                })
            );
        }
    }

    #[test]
    fn rejects_each_wrong_structural_byte_in_order() {
        assert_eq!(
            decode(&[0x01, 0xff, 0x58, 0x04, 0x04, 0x02, 0x08, 0x08]),
            Err(BoundedInitialMeterError::UnsupportedInitialPositionByte {
                offset: 0,
                observed: 0x01,
                expected: 0x00,
            })
        );
        assert_eq!(
            decode(&[0x00, 0xfe, 0x58, 0x04, 0x04, 0x02, 0x08, 0x08]),
            Err(BoundedInitialMeterError::WrongFfTag {
                offset: 1,
                observed: 0xfe,
                expected: 0xff,
            })
        );
        assert_eq!(
            decode(&[0x00, 0xff, 0x57, 0x04, 0x04, 0x02, 0x08, 0x08]),
            Err(BoundedInitialMeterError::WrongMeterTag {
                offset: 2,
                observed: 0x57,
                expected: 0x58,
            })
        );
        assert_eq!(
            decode(&[0x00, 0xff, 0x58, 0x03, 0x04, 0x02, 0x08, 0x08]),
            Err(BoundedInitialMeterError::WrongPayloadLength {
                offset: 3,
                observed: 0x03,
                expected: 0x04,
            })
        );
    }

    #[test]
    fn rejects_supplied_range_without_scanning_for_later_valid_meter() {
        let bytes = [
            0x00, 0xfe, 0x58, 0x04, 0x04, 0x02, 0x08, 0x08, 0x00, 0xff, 0x58, 0x04, 0x06, 0x03,
            0x06, 0x08,
        ];
        assert_eq!(
            decode_bounded_initial_meter(&bytes, InitialMeterBounds { event_range: 0..8 }),
            Err(BoundedInitialMeterError::WrongFfTag {
                offset: 1,
                observed: 0xfe,
                expected: 0xff,
            })
        );
    }
}
