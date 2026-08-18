//! Bounded, evidence-backed diagnostics for one observed Track 7 event chain.
//!
//! This module intentionally models only the local sequence supported by the
//! controlled experiments: a 7-bit big-endian VLQ timing interval followed by
//! pitch, attack velocity, release velocity, and a duration VLQ. It does not
//! identify arbitrary bytes, decode channels/status, or emit MIDI.

use std::{fmt, ops::Range};

use crate::patch::{LocatedByte, LocatedVlq};

/// Maximum number of bytes accepted for one observed 7-bit big-endian VLQ.
pub const MAX_VLQ_BYTES: usize = 4;

/// A decoded VLQ and its source extent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DecodedVlq {
    pub value: u32,
    pub bytes_consumed: usize,
}

/// Safe failures from a bounded VLQ decode.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VlqError {
    OffsetOutsideRange { offset: usize, end: usize },
    Truncated { offset: usize },
    TooLong { offset: usize, maximum: usize },
}

impl fmt::Display for VlqError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OffsetOutsideRange { offset, end } => {
                write!(
                    formatter,
                    "VLQ offset 0x{offset:08x} is outside end 0x{end:08x}"
                )
            }
            Self::Truncated { offset } => {
                write!(formatter, "truncated VLQ at 0x{offset:08x}")
            }
            Self::TooLong { offset, maximum } => write!(
                formatter,
                "VLQ at 0x{offset:08x} exceeds maximum length {maximum}"
            ),
        }
    }
}

impl std::error::Error for VlqError {}

/// Decodes one bounded 7-bit big-endian VLQ.
pub fn decode_7bit_be_vlq(bytes: &[u8], offset: usize, end: usize) -> Result<DecodedVlq, VlqError> {
    let bounded_end = end.min(bytes.len());
    if offset >= bounded_end {
        return Err(VlqError::OffsetOutsideRange {
            offset,
            end: bounded_end,
        });
    }

    let mut value = 0_u32;
    for consumed in 0..MAX_VLQ_BYTES {
        let Some(byte) = bytes.get(offset + consumed).copied() else {
            return Err(VlqError::Truncated { offset });
        };
        if offset + consumed >= bounded_end {
            return Err(VlqError::Truncated { offset });
        }
        value = (value << 7) | u32::from(byte & 0x7f);
        if byte & 0x80 == 0 {
            return Ok(DecodedVlq {
                value,
                bytes_consumed: consumed + 1,
            });
        }
    }

    Err(VlqError::TooLong {
        offset,
        maximum: MAX_VLQ_BYTES,
    })
}

/// One bounded, diagnostic event structure. Terminology is deliberately
/// evidence-aware: `timing_interval` and `accumulated_interval` are not
/// asserted to be MIDI delta-times or Studio Vision absolute positions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiagnosticEvent {
    pub timing_offset: usize,
    pub timing_interval: u32,
    pub timing_bytes: usize,
    pub property_offset: usize,
    pub pitch: u8,
    pub attack_velocity: u8,
    pub release_velocity: u8,
    pub duration: u32,
    pub duration_offset: usize,
    pub duration_bytes: usize,
    pub accumulated_interval: u32,
}

/// Failure while decoding one bounded event.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventError {
    Timing(VlqError),
    MissingPropertyBytes { offset: usize, end: usize },
    Duration(VlqError),
}

/// One exact Note representation at a caller-supplied cursor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundedNoteEvent<'a> {
    pub representation_range: Range<usize>,
    pub timing: LocatedVlq<'a>,
    pub status: Option<LocatedByte>,
    pub pitch: LocatedByte,
    pub attack_velocity: LocatedByte,
    pub release_velocity: LocatedByte,
    pub duration: LocatedVlq<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundedNoteBody<'a> {
    pub representation_range: Range<usize>,
    pub status: Option<LocatedByte>,
    pub pitch: LocatedByte,
    pub attack_velocity: LocatedByte,
    pub release_velocity: LocatedByte,
    pub duration: LocatedVlq<'a>,
}

/// Strict failures from one current-cursor Note decode.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BoundedNoteError {
    Timing(VlqError),
    MissingStatus { offset: usize },
    WrongStatus { offset: usize, observed: u8 },
    MissingPropertyBytes { offset: usize, end: usize },
    HighBitProperty { offset: usize, observed: u8 },
    Duration(VlqError),
    OffsetOverflow { offset: usize },
}

/// Decodes one Note at `cursor`, bounded by `end`, without discovering either.
///
/// `explicit_status` selects whether the established `90` entry byte is
/// required. The returned cursor is the exact end of the duration VLQ.
pub fn decode_note_at(
    bytes: &[u8],
    cursor: usize,
    end: usize,
    explicit_status: bool,
) -> Result<(BoundedNoteEvent<'_>, usize), BoundedNoteError> {
    let decoded_timing =
        decode_7bit_be_vlq(bytes, cursor, end).map_err(BoundedNoteError::Timing)?;
    let timing_end = cursor
        .checked_add(decoded_timing.bytes_consumed)
        .ok_or(BoundedNoteError::OffsetOverflow { offset: cursor })?;
    let timing = LocatedVlq {
        value: decoded_timing.value,
        bytes: &bytes[cursor..timing_end],
        range: cursor..timing_end,
    };

    let (body, next) = decode_note_body_at(bytes, timing_end, end, explicit_status)?;
    Ok((
        BoundedNoteEvent {
            representation_range: cursor..next,
            timing,
            status: body.status,
            pitch: body.pitch,
            attack_velocity: body.attack_velocity,
            release_velocity: body.release_velocity,
            duration: body.duration,
        },
        next,
    ))
}

/// Decodes the status/properties/duration portion of one Note after timing has
/// already been established by an enclosing transition grammar.
pub fn decode_note_body_at(
    bytes: &[u8],
    cursor: usize,
    end: usize,
    explicit_status: bool,
) -> Result<(BoundedNoteBody<'_>, usize), BoundedNoteError> {
    let (status, property_start) = if explicit_status {
        let Some(observed) = bytes.get(cursor).copied().filter(|_| cursor < end) else {
            return Err(BoundedNoteError::MissingStatus { offset: cursor });
        };
        if observed != 0x90 {
            return Err(BoundedNoteError::WrongStatus {
                offset: cursor,
                observed,
            });
        }
        (
            Some(LocatedByte {
                value: observed,
                offset: cursor,
            }),
            cursor
                .checked_add(1)
                .ok_or(BoundedNoteError::OffsetOverflow { offset: cursor })?,
        )
    } else {
        (None, cursor)
    };

    let property_end = property_start
        .checked_add(3)
        .ok_or(BoundedNoteError::OffsetOverflow {
            offset: property_start,
        })?;
    let Some(properties) = bytes.get(property_start..property_end) else {
        return Err(BoundedNoteError::MissingPropertyBytes {
            offset: property_start,
            end: end.min(bytes.len()),
        });
    };
    if property_end > end {
        return Err(BoundedNoteError::MissingPropertyBytes {
            offset: property_start,
            end,
        });
    }
    for (index, observed) in properties.iter().copied().enumerate() {
        if observed >= 0x80 {
            return Err(BoundedNoteError::HighBitProperty {
                offset: property_start + index,
                observed,
            });
        }
    }

    let decoded_duration =
        decode_7bit_be_vlq(bytes, property_end, end).map_err(BoundedNoteError::Duration)?;
    let next = property_end
        .checked_add(decoded_duration.bytes_consumed)
        .ok_or(BoundedNoteError::OffsetOverflow {
            offset: property_end,
        })?;
    let duration = LocatedVlq {
        value: decoded_duration.value,
        bytes: &bytes[property_end..next],
        range: property_end..next,
    };

    Ok((
        BoundedNoteBody {
            representation_range: cursor..next,
            status,
            pitch: LocatedByte {
                value: properties[0],
                offset: property_start,
            },
            attack_velocity: LocatedByte {
                value: properties[1],
                offset: property_start + 1,
            },
            release_velocity: LocatedByte {
                value: properties[2],
                offset: property_start + 2,
            },
            duration,
        },
        next,
    ))
}

impl fmt::Display for EventError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Timing(error) => write!(formatter, "timing field: {error}"),
            Self::MissingPropertyBytes { offset, end } => write!(
                formatter,
                "missing property bytes at 0x{offset:08x} before 0x{end:08x}"
            ),
            Self::Duration(error) => write!(formatter, "duration field: {error}"),
        }
    }
}

impl std::error::Error for EventError {}

/// A parse failure tied to the chain cursor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChainError {
    pub offset: usize,
    pub source: EventError,
}

impl fmt::Display for ChainError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "event-chain parse failed at 0x{:08x}: {}",
            self.offset, self.source
        )
    }
}

impl std::error::Error for ChainError {}

/// Decodes one event without searching beyond the supplied range.
pub fn decode_event(
    bytes: &[u8],
    cursor: usize,
    end: usize,
) -> Result<(DiagnosticEvent, usize), EventError> {
    let timing = decode_7bit_be_vlq(bytes, cursor, end).map_err(EventError::Timing)?;
    let property_offset =
        cursor
            .checked_add(timing.bytes_consumed)
            .ok_or(EventError::MissingPropertyBytes {
                offset: cursor,
                end: end.min(bytes.len()),
            })?;
    let property_end = property_offset
        .checked_add(3)
        .ok_or(EventError::MissingPropertyBytes {
            offset: property_offset,
            end: end.min(bytes.len()),
        })?;
    let Some(properties) = bytes.get(property_offset..property_end) else {
        return Err(EventError::MissingPropertyBytes {
            offset: property_offset,
            end: end.min(bytes.len()),
        });
    };
    if property_end > end.min(bytes.len()) {
        return Err(EventError::MissingPropertyBytes {
            offset: property_offset,
            end: end.min(bytes.len()),
        });
    }

    let duration_offset = property_end;
    let duration = decode_7bit_be_vlq(bytes, duration_offset, end).map_err(EventError::Duration)?;
    let next = duration_offset
        .checked_add(duration.bytes_consumed)
        .ok_or(EventError::Duration(VlqError::Truncated {
            offset: duration_offset,
        }))?;
    Ok((
        DiagnosticEvent {
            timing_offset: cursor,
            timing_interval: timing.value,
            timing_bytes: timing.bytes_consumed,
            property_offset,
            pitch: properties[0],
            attack_velocity: properties[1],
            release_velocity: properties[2],
            duration: duration.value,
            duration_offset,
            duration_bytes: duration.bytes_consumed,
            accumulated_interval: 0,
        },
        next,
    ))
}

/// Walks only the explicitly supplied event-chain range.
pub fn walk_chain(
    bytes: &[u8],
    start: usize,
    end: usize,
) -> Result<Vec<DiagnosticEvent>, ChainError> {
    let bounded_end = end.min(bytes.len());
    let mut cursor = start;
    let mut accumulated = 0_u32;
    let mut events = Vec::new();
    while cursor < bounded_end {
        let (mut event, next) =
            decode_event(bytes, cursor, bounded_end).map_err(|source| ChainError {
                offset: cursor,
                source,
            })?;
        accumulated = accumulated.saturating_add(event.timing_interval);
        event.accumulated_interval = accumulated;
        events.push(event);
        cursor = next;
    }
    Ok(events)
}

#[cfg(test)]
mod tests {
    use super::{decode_7bit_be_vlq, decode_event, walk_chain, EventError, VlqError};

    fn bytes(hex: &[u8]) -> Vec<u8> {
        hex.to_vec()
    }

    #[test]
    fn decodes_documented_vlq_values() {
        for (input, value) in [
            (&[0x81, 0x65][..], 229),
            (&[0x83, 0x60][..], 480),
            (&[0x81, 0x70][..], 240),
            (&[0x83, 0x3a][..], 442),
            (&[0x81, 0x75][..], 245),
            (&[0x6b][..], 107),
        ] {
            assert_eq!(
                decode_7bit_be_vlq(input, 0, input.len()).unwrap().value,
                value
            );
        }
    }

    #[test]
    fn rejects_truncation_bounds_and_excessive_length() {
        assert_eq!(
            decode_7bit_be_vlq(&[0x81], 0, 1),
            Err(VlqError::Truncated { offset: 0 })
        );
        assert_eq!(
            decode_7bit_be_vlq(&[0x81, 0x65], 1, 1),
            Err(VlqError::OffsetOutsideRange { offset: 1, end: 1 })
        );
        assert_eq!(
            decode_7bit_be_vlq(&[0x81, 0x81, 0x81, 0x81, 0x00], 0, 5),
            Err(VlqError::TooLong {
                offset: 0,
                maximum: 4
            })
        );
    }

    #[test]
    fn decodes_one_event_and_reports_offsets() {
        let input = bytes(&[0x81, 0x65, 0x24, 0x7f, 0x5c, 0x83, 0x3a]);
        let (event, next) = decode_event(&input, 0, input.len()).unwrap();
        assert_eq!(event.timing_offset, 0);
        assert_eq!(event.timing_interval, 229);
        assert_eq!(event.property_offset, 2);
        assert_eq!(
            (event.pitch, event.attack_velocity, event.release_velocity),
            (0x24, 127, 92)
        );
        assert_eq!(event.duration, 442);
        assert_eq!(event.duration_offset, 5);
        assert_eq!(event.duration_bytes, 2);
        assert_eq!(next, input.len());
    }

    #[test]
    fn rejects_missing_properties_and_duration() {
        assert!(matches!(
            decode_event(&[0x81, 0x65, 0x24, 0x7f], 0, 4),
            Err(EventError::MissingPropertyBytes { .. })
        ));
        assert!(matches!(
            decode_event(&[0x81, 0x65, 0x24, 0x7f, 0x5c, 0x83], 0, 6),
            Err(EventError::Duration(VlqError::Truncated { .. }))
        ));
    }

    #[test]
    fn walks_baseline_fixture_and_provisional_third_structure() {
        let input = bytes(&[
            0x81, 0x65, 0x24, 0x7f, 0x5c, 0x83, 0x3a, 0x83, 0x60, 0x26, 0x7f, 0x56, 0x81, 0x75,
            0x81, 0x70, 0x24, 0x7f, 0x60, 0x6b,
        ]);
        let events = walk_chain(&input, 0, input.len()).unwrap();
        assert_eq!(events.len(), 3);
        assert_eq!(
            (events[0].timing_interval, events[0].accumulated_interval),
            (229, 229)
        );
        assert_eq!(
            (
                events[0].pitch,
                events[0].attack_velocity,
                events[0].release_velocity,
                events[0].duration
            ),
            (0x24, 127, 92, 442)
        );
        assert_eq!(
            (events[1].timing_interval, events[1].accumulated_interval),
            (480, 709)
        );
        assert_eq!(
            (
                events[1].pitch,
                events[1].attack_velocity,
                events[1].release_velocity,
                events[1].duration
            ),
            (0x26, 127, 86, 245)
        );
        assert_eq!(
            (events[2].timing_interval, events[2].accumulated_interval),
            (240, 949)
        );
        assert_eq!(
            (
                events[2].pitch,
                events[2].attack_velocity,
                events[2].release_velocity,
                events[2].duration
            ),
            (0x24, 127, 96, 107)
        );
    }

    #[test]
    fn decodes_position_control_fixtures() {
        for (leading_low, following, expected_leading, expected_following) in [
            (0x64, [0x83, 0x61], 228, 481),
            (0x65, [0x83, 0x60], 229, 480),
            (0x66, [0x83, 0x5f], 230, 479),
        ] {
            let mut fixture = vec![0x81, leading_low, 0x24, 0x7f, 0x5c, 0x83, 0x3a];
            fixture.extend(following);
            let first = decode_event(&fixture, 0, 7).unwrap().0;
            assert_eq!(first.timing_interval, expected_leading);
            let second = decode_7bit_be_vlq(&fixture, 7, fixture.len()).unwrap();
            assert_eq!(second.value, expected_following);
        }
        let baseline = [0x83, 0x60, 0x26, 0x7f, 0x56, 0x81, 0x75, 0x81, 0x70];
        let edited = [0x83, 0x61, 0x26, 0x7f, 0x56, 0x81, 0x75, 0x81, 0x6f];
        assert_eq!(
            decode_event(&baseline, 0, 7).unwrap().0.timing_interval,
            480
        );
        assert_eq!(decode_7bit_be_vlq(&baseline, 7, 9).unwrap().value, 240);
        assert_eq!(decode_event(&edited, 0, 7).unwrap().0.timing_interval, 481);
        assert_eq!(decode_7bit_be_vlq(&edited, 7, 9).unwrap().value, 239);
    }
}
