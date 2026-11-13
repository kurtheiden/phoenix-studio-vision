//! Caller-bounded counted Meter/Key bodies, with exact source provenance.
//! No discovery, record-envelope proof, defaults, serialization or export authority.

use crate::{
    patch::{LocatedByte, LocatedBytes, LocatedVlq},
    track7::{decode_7bit_be_vlq, VlqError},
};
use std::{fmt, ops::Range};

/// Event-only bounds and count established independently by the caller.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MeterKeyMapBounds {
    pub event_range: Range<usize>,
    pub declared_count: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MeterPayload {
    pub numerator: LocatedByte,
    pub denominator_exponent: LocatedByte,
    /// Raw SV value, not MIDI clocks. Adapt separately with existing policy.
    pub third_payload: LocatedByte,
    pub fourth_payload: LocatedByte,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct KeyPayload {
    /// Retained unsigned source byte; interpretation is signed -7..7.
    pub accidentals_byte: LocatedByte,
    pub mode: LocatedByte,
}
impl KeyPayload {
    pub fn accidentals(&self) -> i8 {
        self.accidentals_byte.value as i8
    }
    pub fn minor(&self) -> bool {
        self.mode.value == 1
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum MeterKeyKind {
    Meter(MeterPayload),
    Key(KeyPayload),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MeterKeyMapEvent<'a> {
    /// Complete original representation, including timing, tag, length and payload.
    pub source: LocatedBytes<'a>,
    pub delta: LocatedVlq<'a>,
    pub absolute_tick: u32,
    pub ff_tag: LocatedByte,
    pub event_tag: LocatedByte,
    pub payload_length: LocatedByte,
    pub kind: MeterKeyKind,
}

/// Exact consumption of a supplied body, not proof of conductor applicability.
/// Private construction and immutable access prevent replacing the decoded map.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct BoundedMeterKeyMap<'a> {
    event_range: Range<usize>,
    declared_count: u32,
    events: Vec<MeterKeyMapEvent<'a>>,
}
impl<'a> BoundedMeterKeyMap<'a> {
    pub fn event_range(&self) -> Range<usize> {
        self.event_range.clone()
    }
    pub fn declared_count(&self) -> u32 {
        self.declared_count
    }
    pub fn events(&self) -> &[MeterKeyMapEvent<'a>] {
        &self.events
    }
    /// Existing identity480 timing domain, not a decoded source PPQN field.
    pub fn ppqn(&self) -> u16 {
        480
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum MeterKeyMapError {
    Bounds,
    Count,
    Timing(VlqError),
    Truncated {
        offset: usize,
    },
    UnsupportedEvent {
        offset: usize,
    },
    PayloadLength {
        offset: usize,
        expected: u8,
        observed: u8,
    },
    InvalidKey {
        offset: usize,
    },
    PositionOverflow {
        offset: usize,
    },
    Unconsumed {
        offset: usize,
    },
}
impl fmt::Display for MeterKeyMapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bounded Meter/Key map refused: {self:?}")
    }
}
impl std::error::Error for MeterKeyMapError {}

/// Decode VLQ delta + FF58/04/nn/dd/xx/yy or FF59/02/sf/mi.
/// Accumulate from zero, retaining a nonzero first delta and every duplicate in
/// source order. Any failure refuses the entire body; no partial walk or scan.
pub(crate) fn decode_bounded_meter_key_map(
    bytes: &[u8],
    bounds: MeterKeyMapBounds,
) -> Result<BoundedMeterKeyMap<'_>, MeterKeyMapError> {
    let range = bounds.event_range;
    if range.start >= range.end || range.end > bytes.len() {
        return Err(MeterKeyMapError::Bounds);
    }
    let count = usize::try_from(bounds.declared_count).map_err(|_| MeterKeyMapError::Count)?;
    // Six bytes is the shortest possible Key event. No allocation from a
    // contradictory or unbounded caller count; count-zero is not a Meter default.
    if count == 0 || count > range.len() / 6 {
        return Err(MeterKeyMapError::Count);
    }
    let mut cursor = range.start;
    let mut tick = 0u32;
    let mut events = Vec::new();
    for _ in 0..count {
        let start = cursor;
        let timing =
            decode_7bit_be_vlq(bytes, start, range.end).map_err(MeterKeyMapError::Timing)?;
        let tag = start
            .checked_add(timing.bytes_consumed)
            .ok_or(MeterKeyMapError::Bounds)?;
        let payload = tag.checked_add(3).ok_or(MeterKeyMapError::Bounds)?;
        if payload > range.end {
            return Err(MeterKeyMapError::Truncated { offset: tag });
        }
        if bytes[tag] != 0xff {
            return Err(MeterKeyMapError::UnsupportedEvent { offset: tag });
        }
        let expected = match bytes[tag + 1] {
            0x58 => 4,
            0x59 => 2,
            _ => return Err(MeterKeyMapError::UnsupportedEvent { offset: tag }),
        };
        if bytes[tag + 2] != expected {
            return Err(MeterKeyMapError::PayloadLength {
                offset: tag + 2,
                expected,
                observed: bytes[tag + 2],
            });
        }
        let end = payload
            .checked_add(usize::from(expected))
            .ok_or(MeterKeyMapError::Bounds)?;
        if end > range.end {
            return Err(MeterKeyMapError::Truncated { offset: payload });
        }
        let located = |offset| LocatedByte {
            value: bytes[offset],
            offset,
        };
        let kind = if expected == 4 {
            // As with the initial decoder, raw Meter values are preserved;
            // musical/MIDI validity and clocks adaptation are separate policy.
            MeterKeyKind::Meter(MeterPayload {
                numerator: located(payload),
                denominator_exponent: located(payload + 1),
                third_payload: located(payload + 2),
                fourth_payload: located(payload + 3),
            })
        } else {
            let key = KeyPayload {
                accidentals_byte: located(payload),
                mode: located(payload + 1),
            };
            if !(-7..=7).contains(&key.accidentals()) || key.mode.value > 1 {
                return Err(MeterKeyMapError::InvalidKey { offset: payload });
            }
            MeterKeyKind::Key(key)
        };
        tick = tick
            .checked_add(timing.value)
            .ok_or(MeterKeyMapError::PositionOverflow { offset: start })?;
        events.push(MeterKeyMapEvent {
            source: LocatedBytes {
                bytes: &bytes[start..end],
                range: start..end,
            },
            delta: LocatedVlq {
                value: timing.value,
                bytes: &bytes[start..tag],
                range: start..tag,
            },
            absolute_tick: tick,
            ff_tag: located(tag),
            event_tag: located(tag + 1),
            payload_length: located(tag + 2),
            kind,
        });
        cursor = end;
    }
    if cursor != range.end {
        return Err(MeterKeyMapError::Unconsumed { offset: cursor });
    }
    Ok(BoundedMeterKeyMap {
        event_range: range,
        declared_count: bounds.declared_count,
        events,
    })
}

#[cfg(test)]
#[path = "meter_key_map_tests.rs"]
mod tests;
