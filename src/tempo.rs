//! Exact-bounded initial Tempo and counted Tempo-map decoding.
//!
//! The map grammar is supported by CCA-045 source/SMF correlations. Callers
//! supply established event-only bounds and count; no discovery, envelope
//! qualification or complete-export authority is supplied by this module.

use std::fmt;
use std::ops::Range;

use crate::patch::LocatedByte;

const REPRESENTATION_LENGTH: usize = 7;
const INITIAL_POSITION_BYTE: u8 = 0x00;
const FF_TAG: u8 = 0xff;
const TEMPO_TAG: u8 = 0x51;
const PAYLOAD_LENGTH: u8 = 0x03;

/// Exact caller-known bounds for one initial Tempo representation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InitialTempoBounds {
    pub event_range: Range<usize>,
}

/// One bounded initial Tempo representation with byte-exact provenance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InitialTempoEvent {
    pub event_range: Range<usize>,
    pub initial_position_byte: LocatedByte,
    pub ff_tag: LocatedByte,
    pub tempo_tag: LocatedByte,
    pub payload_length: LocatedByte,
    pub mpqn_byte_0: LocatedByte,
    pub mpqn_byte_1: LocatedByte,
    pub mpqn_byte_2: LocatedByte,
}

impl InitialTempoEvent {
    /// Derives the stored unsigned 24-bit big-endian MPQN value.
    pub fn mpqn(&self) -> u32 {
        (u32::from(self.mpqn_byte_0.value) << 16)
            | (u32::from(self.mpqn_byte_1.value) << 8)
            | u32::from(self.mpqn_byte_2.value)
    }

    /// Derives BPM when the stored MPQN value is nonzero.
    pub fn bpm(&self) -> Option<f64> {
        let mpqn = self.mpqn();
        (mpqn != 0).then(|| 60_000_000.0 / f64::from(mpqn))
    }
}

/// Deterministic failures from decoding one bounded initial Tempo form.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BoundedInitialTempoError {
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
    WrongTempoTag {
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

impl fmt::Display for BoundedInitialTempoError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBounds { start, end, size } => write!(
                formatter,
                "invalid bounded initial Tempo range 0x{start:08x}..0x{end:08x} for {size} bytes"
            ),
            Self::WrongRepresentationLength {
                start,
                end,
                observed,
                expected,
            } => write!(
                formatter,
                "bounded initial Tempo range 0x{start:08x}..0x{end:08x} has {observed} bytes; expected {expected}"
            ),
            Self::UnsupportedInitialPositionByte {
                offset,
                observed,
                expected,
            } => write!(
                formatter,
                "unsupported initial Tempo position byte {observed:02x} at 0x{offset:08x}; expected {expected:02x}"
            ),
            Self::WrongFfTag {
                offset,
                observed,
                expected,
            } => write!(
                formatter,
                "wrong initial Tempo ff tag {observed:02x} at 0x{offset:08x}; expected {expected:02x}"
            ),
            Self::WrongTempoTag {
                offset,
                observed,
                expected,
            } => write!(
                formatter,
                "wrong initial Tempo tag {observed:02x} at 0x{offset:08x}; expected {expected:02x}"
            ),
            Self::WrongPayloadLength {
                offset,
                observed,
                expected,
            } => write!(
                formatter,
                "wrong initial Tempo payload length {observed} at 0x{offset:08x}; expected {expected}"
            ),
        }
    }
}

impl std::error::Error for BoundedInitialTempoError {}

/// Decodes one exact caller-bounded initial Tempo representation without scanning.
pub fn decode_bounded_initial_tempo(
    bytes: &[u8],
    bounds: InitialTempoBounds,
) -> Result<InitialTempoEvent, BoundedInitialTempoError> {
    let start = bounds.event_range.start;
    let end = bounds.event_range.end;
    if start >= end || end > bytes.len() {
        return Err(BoundedInitialTempoError::InvalidBounds {
            start,
            end,
            size: bytes.len(),
        });
    }

    let observed_length = end - start;
    if observed_length != REPRESENTATION_LENGTH {
        return Err(BoundedInitialTempoError::WrongRepresentationLength {
            start,
            end,
            observed: observed_length,
            expected: REPRESENTATION_LENGTH,
        });
    }

    let initial_position_byte = located_byte(bytes, start);
    if initial_position_byte.value != INITIAL_POSITION_BYTE {
        return Err(BoundedInitialTempoError::UnsupportedInitialPositionByte {
            offset: initial_position_byte.offset,
            observed: initial_position_byte.value,
            expected: INITIAL_POSITION_BYTE,
        });
    }

    let ff_tag = located_byte(bytes, start + 1);
    if ff_tag.value != FF_TAG {
        return Err(BoundedInitialTempoError::WrongFfTag {
            offset: ff_tag.offset,
            observed: ff_tag.value,
            expected: FF_TAG,
        });
    }

    let tempo_tag = located_byte(bytes, start + 2);
    if tempo_tag.value != TEMPO_TAG {
        return Err(BoundedInitialTempoError::WrongTempoTag {
            offset: tempo_tag.offset,
            observed: tempo_tag.value,
            expected: TEMPO_TAG,
        });
    }

    let payload_length = located_byte(bytes, start + 3);
    if payload_length.value != PAYLOAD_LENGTH {
        return Err(BoundedInitialTempoError::WrongPayloadLength {
            offset: payload_length.offset,
            observed: payload_length.value,
            expected: PAYLOAD_LENGTH,
        });
    }

    Ok(InitialTempoEvent {
        event_range: bounds.event_range,
        initial_position_byte,
        ff_tag,
        tempo_tag,
        payload_length,
        mpqn_byte_0: located_byte(bytes, start + 4),
        mpqn_byte_1: located_byte(bytes, start + 5),
        mpqn_byte_2: located_byte(bytes, start + 6),
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
        decode_bounded_initial_tempo, BoundedInitialTempoError, InitialTempoBounds,
        InitialTempoEvent,
    };

    fn decode(bytes: &[u8]) -> Result<InitialTempoEvent, BoundedInitialTempoError> {
        decode_bounded_initial_tempo(
            bytes,
            InitialTempoBounds {
                event_range: 0..bytes.len(),
            },
        )
    }

    #[test]
    fn derives_complete_unsigned_24_bit_range_and_bpm() {
        let zero = decode(&[0x00, 0xff, 0x51, 0x03, 0x00, 0x00, 0x00]).unwrap();
        assert_eq!(zero.mpqn(), 0);
        assert_eq!(zero.bpm(), None);

        let one = decode(&[0x00, 0xff, 0x51, 0x03, 0x00, 0x00, 0x01]).unwrap();
        assert_eq!(one.mpqn(), 1);

        let maximum = decode(&[0x00, 0xff, 0x51, 0x03, 0xff, 0xff, 0xff]).unwrap();
        assert_eq!(maximum.mpqn(), 16_777_215);

        let exact = decode(&[0x00, 0xff, 0x51, 0x03, 0x07, 0xa1, 0x20]).unwrap();
        assert_eq!(exact.mpqn(), 500_000);
        assert_eq!(exact.bpm(), Some(120.0));
    }

    #[test]
    fn preserves_absolute_offsets_from_a_nonzero_slice_start() {
        let bytes = [0xaa, 0xbb, 0x00, 0xff, 0x51, 0x03, 0x09, 0x10, 0x8b, 0xcc];
        let event =
            decode_bounded_initial_tempo(&bytes, InitialTempoBounds { event_range: 2..9 }).unwrap();

        assert_eq!(event.event_range, 2..9);
        assert_eq!(event.initial_position_byte.offset, 2);
        assert_eq!(event.ff_tag.offset, 3);
        assert_eq!(event.tempo_tag.offset, 4);
        assert_eq!(event.payload_length.offset, 5);
        assert_eq!(event.mpqn_byte_0.offset, 6);
        assert_eq!(event.mpqn_byte_1.offset, 7);
        assert_eq!(event.mpqn_byte_2.offset, 8);
        assert_eq!(event.mpqn(), 594_059);
    }

    #[test]
    fn rejects_invalid_and_wrong_length_bounds_before_indexing() {
        let bytes = [0x00, 0xff, 0x51, 0x03, 0x07, 0xa1, 0x20, 0x00, 0x00];
        for range in [0..0, std::ops::Range { start: 4, end: 3 }, 0..10] {
            assert!(matches!(
                decode_bounded_initial_tempo(&bytes, InitialTempoBounds { event_range: range }),
                Err(BoundedInitialTempoError::InvalidBounds { .. })
            ));
        }

        for range in [0..1, 0..2, 0..3, 0..4, 0..5, 0..6, 0..8, 0..9] {
            let observed = range.end - range.start;
            assert_eq!(
                decode_bounded_initial_tempo(
                    &bytes,
                    InitialTempoBounds {
                        event_range: range.clone()
                    }
                ),
                Err(BoundedInitialTempoError::WrongRepresentationLength {
                    start: range.start,
                    end: range.end,
                    observed,
                    expected: 7,
                })
            );
        }
    }

    #[test]
    fn rejects_each_wrong_structural_byte_in_order() {
        assert_eq!(
            decode(&[0x01, 0xff, 0x51, 0x03, 0x07, 0xa1, 0x20]),
            Err(BoundedInitialTempoError::UnsupportedInitialPositionByte {
                offset: 0,
                observed: 0x01,
                expected: 0x00,
            })
        );
        assert_eq!(
            decode(&[0x00, 0xfe, 0x51, 0x03, 0x07, 0xa1, 0x20]),
            Err(BoundedInitialTempoError::WrongFfTag {
                offset: 1,
                observed: 0xfe,
                expected: 0xff,
            })
        );
        assert_eq!(
            decode(&[0x00, 0xff, 0x50, 0x03, 0x07, 0xa1, 0x20]),
            Err(BoundedInitialTempoError::WrongTempoTag {
                offset: 2,
                observed: 0x50,
                expected: 0x51,
            })
        );
        assert_eq!(
            decode(&[0x00, 0xff, 0x51, 0x04, 0x07, 0xa1, 0x20]),
            Err(BoundedInitialTempoError::WrongPayloadLength {
                offset: 3,
                observed: 0x04,
                expected: 0x03,
            })
        );
    }

    #[test]
    fn rejects_supplied_range_without_scanning_for_later_valid_tempo() {
        let bytes = [
            0x00, 0xfe, 0x51, 0x03, 0x07, 0xa1, 0x20, 0x00, 0xff, 0x51, 0x03, 0x09, 0x10, 0x8b,
        ];
        assert_eq!(
            decode_bounded_initial_tempo(&bytes, InitialTempoBounds { event_range: 0..7 }),
            Err(BoundedInitialTempoError::WrongFfTag {
                offset: 1,
                observed: 0xfe,
                expected: 0xff,
            })
        );
    }
}

/// Caller-established event-only bounds, excluding primary prefix/tail/secondary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TempoMapBounds {
    pub event_range: Range<usize>,
    pub declared_count: u32,
}

/// One source Tempo object, retaining timing/value bytes and absolute provenance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TempoMapEvent<'a> {
    pub source_range: Range<usize>,
    pub delta: crate::patch::LocatedVlq<'a>,
    pub absolute_tick: u32,
    pub mpqn_bytes: [LocatedByte; 3],
}
impl TempoMapEvent<'_> {
    pub fn mpqn(&self) -> u32 {
        (u32::from(self.mpqn_bytes[0].value) << 16)
            | (u32::from(self.mpqn_bytes[1].value) << 8)
            | u32::from(self.mpqn_bytes[2].value)
    }
}

/// Complete consumption of the supplied map body, not a conductor/export proof.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundedTempoMap<'a> {
    event_range: Range<usize>,
    events: Vec<TempoMapEvent<'a>>,
}
impl<'a> BoundedTempoMap<'a> {
    pub fn event_range(&self) -> Range<usize> {
        self.event_range.clone()
    }
    pub fn events(&self) -> &[TempoMapEvent<'a>] {
        &self.events
    }
    /// CCA-045's identity conversion; no scaling or inferred offset.
    pub fn ppqn(&self) -> u16 {
        480
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TempoMapError {
    Bounds,
    Count,
    Timing(crate::track7::VlqError),
    Truncated { offset: usize },
    UnsupportedEvent { offset: usize },
    InvalidTempo { offset: usize },
    PositionOverflow { offset: usize },
    Unconsumed { offset: usize },
}
impl fmt::Display for TempoMapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bounded Tempo map refused: {self:?}")
    }
}
impl std::error::Error for TempoMapError {}

/// Decode exactly the counted VLQ-delta / FF51 / 03 / MPQN[3] grammar.
/// Equal-position events are retained in source order, never deduplicated.
/// The enclosing record and its non-event bytes remain the caller's responsibility.
pub fn decode_bounded_tempo_map(
    bytes: &[u8],
    bounds: TempoMapBounds,
) -> Result<BoundedTempoMap<'_>, TempoMapError> {
    let range = bounds.event_range;
    if range.start >= range.end || range.end > bytes.len() {
        return Err(TempoMapError::Bounds);
    }
    let count = usize::try_from(bounds.declared_count).map_err(|_| TempoMapError::Count)?;
    if count == 0 || count > (range.end - range.start) / 7 {
        return Err(TempoMapError::Count);
    }
    let mut cursor = range.start;
    let mut tick = 0u32;
    let mut events = Vec::new();
    for _ in 0..count {
        let start = cursor;
        let timing = crate::track7::decode_7bit_be_vlq(bytes, cursor, range.end)
            .map_err(TempoMapError::Timing)?;
        let timing_end = cursor
            .checked_add(timing.bytes_consumed)
            .ok_or(TempoMapError::Bounds)?;
        let end = timing_end.checked_add(6).ok_or(TempoMapError::Bounds)?;
        if end > range.end {
            return Err(TempoMapError::Truncated { offset: timing_end });
        }
        if bytes[timing_end..timing_end + 3] != [0xff, 0x51, 0x03] {
            return Err(TempoMapError::UnsupportedEvent { offset: timing_end });
        }
        tick = tick
            .checked_add(timing.value)
            .ok_or(TempoMapError::PositionOverflow { offset: start })?;
        let event = TempoMapEvent {
            source_range: start..end,
            delta: crate::patch::LocatedVlq {
                value: timing.value,
                bytes: &bytes[start..timing_end],
                range: start..timing_end,
            },
            absolute_tick: tick,
            mpqn_bytes: [
                located_byte(bytes, timing_end + 3),
                located_byte(bytes, timing_end + 4),
                located_byte(bytes, timing_end + 5),
            ],
        };
        // Raw zero remains supported by the old initial decoder, but cannot be
        // represented as a valid recovered SMF Tempo under the existing contract.
        if event.mpqn() == 0 {
            return Err(TempoMapError::InvalidTempo {
                offset: timing_end + 3,
            });
        }
        events.push(event);
        cursor = end;
    }
    if cursor != range.end {
        return Err(TempoMapError::Unconsumed { offset: cursor });
    }
    Ok(BoundedTempoMap {
        event_range: range,
        events,
    })
}
