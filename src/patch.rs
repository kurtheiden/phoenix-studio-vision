//! Bounded diagnostics for the known Track 3 #2 first Patch representation.
//!
//! This module does not discover Patch events or define a general Studio
//! Vision event grammar. Callers must supply the exact evidence-backed start
//! and end offsets for the known representation.

use crate::track7::{decode_7bit_be_vlq, VlqError};
use std::fmt;
use std::ops::Range;

const PREFIX_AFTER_POSITION: [u8; 2] = [0xff, 0x7c];
const PREFIX_BEFORE_NAME_LENGTH: [u8; 5] = [0x00, 0x00, 0x17, 0x00, 0x17];
const CONTEXT_AFTER_NAME: [u8; 8] = [0x03, b'I', b'3', b'8', 0x04, 0xff, 0xff, 0xff];
const CONTEXT_BEFORE_NOTE_STATUS: [u8; 12] = [
    0xff, 0x60, 0x07, 0x57, 0x7f, 0x00, 0x6c, 0x6c, 0xa3, 0x4a, 0x81, 0x25,
];
const EXPECTED_NOTE_STATUS: u8 = 0x90;
const COMMON_MARKER: [u8; 2] = [0xff, 0x7c];
const PRE_NAME_CONTEXT_BYTES: usize = 5;
const MINIMUM_PAYLOAD_LENGTH: u8 = 7;

/// Caller-known boundaries for one evidence-backed Patch representation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PatchRepresentationBounds {
    pub position_start: usize,
    /// Exclusive boundary immediately after the expected `0x90` Note status.
    pub note_status_end: usize,
}

/// Hard caller boundary for one Patch core ending at its declared payload end.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PatchCoreBounds {
    pub position_start: usize,
    pub end: usize,
}

/// Borrowed bytes paired with their absolute source range.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocatedBytes<'a> {
    pub bytes: &'a [u8],
    pub range: Range<usize>,
}

/// A bounded VLQ paired with its raw bytes and absolute source range.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocatedVlq<'a> {
    pub value: u32,
    pub bytes: &'a [u8],
    pub range: Range<usize>,
}

/// An ASCII field paired with its raw bytes and absolute source range.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocatedAscii<'a> {
    pub text: &'a str,
    pub bytes: &'a [u8],
    pub range: Range<usize>,
}

/// One byte paired with its absolute source offset.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LocatedByte {
    pub value: u8,
    pub offset: usize,
}

/// Common semantic fields plus byte-exact opaque context and provenance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundedPatchRepresentation<'a> {
    pub representation_range: Range<usize>,
    pub position: LocatedVlq<'a>,
    pub marker_range: Range<usize>,
    pub payload_length: LocatedByte,
    pub payload_range: Range<usize>,
    pub pre_name_context: LocatedBytes<'a>,
    pub name_length: LocatedByte,
    pub name: LocatedAscii<'a>,
    pub post_name_context: LocatedBytes<'a>,
    pub program_change: LocatedByte,
    pub post_pc_timing_component: LocatedVlq<'a>,
    pub pre_note_context: LocatedBytes<'a>,
    pub note_status: LocatedByte,
}

/// The length-delimited Patch fields through the direct Program Change byte.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundedPatchCore<'a> {
    pub representation_range: Range<usize>,
    pub position: LocatedVlq<'a>,
    pub marker_range: Range<usize>,
    pub payload_length: LocatedByte,
    pub payload_range: Range<usize>,
    pub pre_name_context: LocatedBytes<'a>,
    pub name_length: LocatedByte,
    pub name: LocatedAscii<'a>,
    pub post_name_context: LocatedBytes<'a>,
    pub program_change: LocatedByte,
}

/// Deterministic failures from the shared bounded representation decoder.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BoundedPatchError {
    InvalidBounds {
        start: usize,
        end: usize,
        size: usize,
    },
    PositionVlq(VlqError),
    MissingMarker {
        offset: usize,
        observed: Vec<u8>,
    },
    PayloadLengthOverflow {
        offset: usize,
        length: u8,
    },
    PayloadExceedsBoundary {
        payload_end: usize,
        status_offset: usize,
    },
    PayloadTooShort {
        offset: usize,
        length: u8,
        minimum: u8,
    },
    NameLengthExceedsPayload {
        offset: usize,
        length: u8,
        pc_offset: usize,
    },
    InvalidAsciiName {
        range: Range<usize>,
    },
    MissingProgramChange {
        payload_range: Range<usize>,
    },
    PostPcVlq(VlqError),
    PostPcVlqCrossesStatus {
        range: Range<usize>,
        status_offset: usize,
    },
    MissingNoteStatus {
        offset: usize,
        expected: u8,
        observed: Option<u8>,
    },
}

impl fmt::Display for BoundedPatchError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBounds { start, end, size } => write!(
                formatter,
                "invalid bounded Patch range 0x{start:08x}..0x{end:08x} for {size} bytes"
            ),
            Self::PositionVlq(error) => write!(formatter, "Patch position: {error}"),
            Self::MissingMarker { offset, observed } => write!(
                formatter,
                "missing ff 7c Patch marker at 0x{offset:08x}; observed {observed:02x?}"
            ),
            Self::PayloadLengthOverflow { offset, length } => write!(
                formatter,
                "Patch payload length {length} at 0x{offset:08x} overflows its source offset"
            ),
            Self::PayloadExceedsBoundary {
                payload_end,
                status_offset,
            } => write!(
                formatter,
                "Patch payload ends at 0x{payload_end:08x}, beyond Note status at 0x{status_offset:08x}"
            ),
            Self::PayloadTooShort {
                offset,
                length,
                minimum,
            } => write!(
                formatter,
                "Patch payload length at 0x{offset:08x} is {length}; minimum is {minimum}"
            ),
            Self::NameLengthExceedsPayload {
                offset,
                length,
                pc_offset,
            } => write!(
                formatter,
                "Patch name length {length} at 0x{offset:08x} crosses PC at 0x{pc_offset:08x}"
            ),
            Self::InvalidAsciiName { range } => write!(
                formatter,
                "Patch name at 0x{:08x}..0x{:08x} is not ASCII",
                range.start, range.end
            ),
            Self::MissingProgramChange { payload_range } => write!(
                formatter,
                "Patch payload 0x{:08x}..0x{:08x} has no Program Change byte",
                payload_range.start, payload_range.end
            ),
            Self::PostPcVlq(error) => write!(formatter, "post-PC timing component: {error}"),
            Self::PostPcVlqCrossesStatus {
                range,
                status_offset,
            } => write!(
                formatter,
                "post-PC timing component 0x{:08x}..0x{:08x} crosses Note status at 0x{status_offset:08x}",
                range.start, range.end
            ),
            Self::MissingNoteStatus {
                offset,
                expected,
                observed,
            } => write!(
                formatter,
                "expected Note status {expected:02x} at 0x{offset:08x}; observed {observed:02x?}"
            ),
        }
    }
}

impl std::error::Error for BoundedPatchError {}

/// Decodes one caller-located Patch representation without scanning or recovery.
pub fn decode_bounded_patch_representation<'a>(
    bytes: &'a [u8],
    bounds: PatchRepresentationBounds,
) -> Result<BoundedPatchRepresentation<'a>, BoundedPatchError> {
    let start = bounds.position_start;
    let end = bounds.note_status_end;
    if start >= end || end > bytes.len() {
        return Err(BoundedPatchError::InvalidBounds {
            start,
            end,
            size: bytes.len(),
        });
    }

    let status_offset = end - 1;
    let observed_status = bytes.get(status_offset).copied();
    if observed_status != Some(EXPECTED_NOTE_STATUS) {
        return Err(BoundedPatchError::MissingNoteStatus {
            offset: status_offset,
            expected: EXPECTED_NOTE_STATUS,
            observed: observed_status,
        });
    }

    let core = decode_bounded_patch_core(
        bytes,
        PatchCoreBounds {
            position_start: start,
            end: status_offset,
        },
    )?;
    let post_pc_start = core.representation_range.end;
    let post_pc = decode_7bit_be_vlq(bytes, post_pc_start, status_offset)
        .map_err(BoundedPatchError::PostPcVlq)?;
    let post_pc_end = post_pc_start.checked_add(post_pc.bytes_consumed).ok_or(
        BoundedPatchError::PostPcVlqCrossesStatus {
            range: post_pc_start..usize::MAX,
            status_offset,
        },
    )?;
    let post_pc_range = post_pc_start..post_pc_end;
    if post_pc_end > status_offset {
        return Err(BoundedPatchError::PostPcVlqCrossesStatus {
            range: post_pc_range,
            status_offset,
        });
    }
    let post_pc_bytes =
        bytes
            .get(post_pc_range.clone())
            .ok_or(BoundedPatchError::PostPcVlqCrossesStatus {
                range: post_pc_range.clone(),
                status_offset,
            })?;
    let pre_note_range = post_pc_end..status_offset;

    Ok(BoundedPatchRepresentation {
        representation_range: start..end,
        position: core.position,
        marker_range: core.marker_range,
        payload_length: core.payload_length,
        payload_range: core.payload_range,
        pre_name_context: core.pre_name_context,
        name_length: core.name_length,
        name: core.name,
        post_name_context: core.post_name_context,
        program_change: core.program_change,
        post_pc_timing_component: LocatedVlq {
            value: post_pc.value,
            bytes: post_pc_bytes,
            range: post_pc_range,
        },
        pre_note_context: LocatedBytes {
            bytes: &bytes[pre_note_range.clone()],
            range: pre_note_range,
        },
        note_status: LocatedByte {
            value: EXPECTED_NOTE_STATUS,
            offset: status_offset,
        },
    })
}

/// Decodes one current-cursor Patch core through its declared payload end.
pub fn decode_bounded_patch_core<'a>(
    bytes: &'a [u8],
    bounds: PatchCoreBounds,
) -> Result<BoundedPatchCore<'a>, BoundedPatchError> {
    let start = bounds.position_start;
    let end = bounds.end;
    if start >= end || end > bytes.len() {
        return Err(BoundedPatchError::InvalidBounds {
            start,
            end,
            size: bytes.len(),
        });
    }

    let position = decode_7bit_be_vlq(bytes, start, end).map_err(BoundedPatchError::PositionVlq)?;
    let position_end =
        start
            .checked_add(position.bytes_consumed)
            .ok_or(BoundedPatchError::InvalidBounds {
                start,
                end,
                size: bytes.len(),
            })?;
    let position_range = start..position_end;
    let position_bytes =
        bytes
            .get(position_range.clone())
            .ok_or(BoundedPatchError::PositionVlq(VlqError::Truncated {
                offset: start,
            }))?;

    let marker_start = position_end;
    let marker_end =
        marker_start
            .checked_add(COMMON_MARKER.len())
            .ok_or(BoundedPatchError::InvalidBounds {
                start,
                end,
                size: bytes.len(),
            })?;
    let marker = bytes
        .get(marker_start..marker_end.min(end))
        .unwrap_or_default();
    if marker_end > end || marker != COMMON_MARKER {
        return Err(BoundedPatchError::MissingMarker {
            offset: marker_start,
            observed: marker.to_vec(),
        });
    }

    let payload_length_offset = marker_end;
    let Some(payload_length) = bytes
        .get(payload_length_offset)
        .copied()
        .filter(|_| payload_length_offset < end)
    else {
        return Err(BoundedPatchError::PayloadExceedsBoundary {
            payload_end: payload_length_offset.saturating_add(1),
            status_offset: end,
        });
    };
    if payload_length < MINIMUM_PAYLOAD_LENGTH {
        return Err(BoundedPatchError::PayloadTooShort {
            offset: payload_length_offset,
            length: payload_length,
            minimum: MINIMUM_PAYLOAD_LENGTH,
        });
    }

    let payload_start =
        payload_length_offset
            .checked_add(1)
            .ok_or(BoundedPatchError::PayloadLengthOverflow {
                offset: payload_length_offset,
                length: payload_length,
            })?;
    let payload_end = payload_start
        .checked_add(usize::from(payload_length))
        .ok_or(BoundedPatchError::PayloadLengthOverflow {
            offset: payload_length_offset,
            length: payload_length,
        })?;
    if payload_end > end {
        return Err(BoundedPatchError::PayloadExceedsBoundary {
            payload_end,
            status_offset: end,
        });
    }
    let payload_range = payload_start..payload_end;
    let program_change_offset =
        payload_end
            .checked_sub(1)
            .ok_or(BoundedPatchError::MissingProgramChange {
                payload_range: payload_range.clone(),
            })?;
    let pre_name_end = payload_start.checked_add(PRE_NAME_CONTEXT_BYTES).ok_or(
        BoundedPatchError::PayloadLengthOverflow {
            offset: payload_length_offset,
            length: payload_length,
        },
    )?;
    let pre_name_range = payload_start..pre_name_end;
    let pre_name_bytes =
        bytes
            .get(pre_name_range.clone())
            .ok_or(BoundedPatchError::PayloadExceedsBoundary {
                payload_end,
                status_offset: end,
            })?;

    let name_length_offset = pre_name_end;
    let Some(name_length) = bytes
        .get(name_length_offset)
        .copied()
        .filter(|_| name_length_offset < program_change_offset)
    else {
        return Err(BoundedPatchError::NameLengthExceedsPayload {
            offset: name_length_offset,
            length: 0,
            pc_offset: program_change_offset,
        });
    };
    let name_start =
        name_length_offset
            .checked_add(1)
            .ok_or(BoundedPatchError::NameLengthExceedsPayload {
                offset: name_length_offset,
                length: name_length,
                pc_offset: program_change_offset,
            })?;
    let name_end = name_start.checked_add(usize::from(name_length)).ok_or(
        BoundedPatchError::NameLengthExceedsPayload {
            offset: name_length_offset,
            length: name_length,
            pc_offset: program_change_offset,
        },
    )?;
    if name_end > program_change_offset {
        return Err(BoundedPatchError::NameLengthExceedsPayload {
            offset: name_length_offset,
            length: name_length,
            pc_offset: program_change_offset,
        });
    }
    let name_range = name_start..name_end;
    let name_bytes =
        bytes
            .get(name_range.clone())
            .ok_or(BoundedPatchError::NameLengthExceedsPayload {
                offset: name_length_offset,
                length: name_length,
                pc_offset: program_change_offset,
            })?;
    if !name_bytes.is_ascii() {
        return Err(BoundedPatchError::InvalidAsciiName { range: name_range });
    }
    let name =
        std::str::from_utf8(name_bytes).map_err(|_| BoundedPatchError::InvalidAsciiName {
            range: name_range.clone(),
        })?;
    let post_name_range = name_end..program_change_offset;
    let post_name_bytes = &bytes[post_name_range.clone()];
    let program_change = bytes.get(program_change_offset).copied().ok_or(
        BoundedPatchError::MissingProgramChange {
            payload_range: payload_range.clone(),
        },
    )?;

    Ok(BoundedPatchCore {
        representation_range: start..payload_end,
        position: LocatedVlq {
            value: position.value,
            bytes: position_bytes,
            range: position_range,
        },
        marker_range: marker_start..marker_end,
        payload_length: LocatedByte {
            value: payload_length,
            offset: payload_length_offset,
        },
        payload_range,
        pre_name_context: LocatedBytes {
            bytes: pre_name_bytes,
            range: pre_name_range,
        },
        name_length: LocatedByte {
            value: name_length,
            offset: name_length_offset,
        },
        name: LocatedAscii {
            text: name,
            bytes: name_bytes,
            range: name_start..name_end,
        },
        post_name_context: LocatedBytes {
            bytes: post_name_bytes,
            range: post_name_range,
        },
        program_change: LocatedByte {
            value: program_change,
            offset: program_change_offset,
        },
    })
}

/// Confirmed fields and source offsets from one bounded diagnostic decode.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PatchDiagnostic {
    pub position: u32,
    pub position_offset: usize,
    pub position_bytes: usize,
    pub local_payload_length: u8,
    pub local_payload_length_offset: usize,
    pub name_length: u8,
    pub name_length_offset: usize,
    pub name: String,
    pub name_offset: usize,
    pub program_change: u8,
    pub program_change_offset: usize,
    pub first_note_status: u8,
    pub first_note_status_offset: usize,
}

/// Safe failures from the explicitly bounded known-representation decoder.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PatchError {
    InvalidBounds {
        start: usize,
        end: usize,
        size: usize,
    },
    Position(VlqError),
    UnresolvedInterval(VlqError),
    UnexpectedPositionWidth {
        offset: usize,
        bytes: usize,
    },
    UnexpectedUnresolvedIntervalWidth {
        offset: usize,
        bytes: usize,
    },
    Truncated {
        offset: usize,
        needed: usize,
        end: usize,
    },
    UnexpectedBytes {
        offset: usize,
        expected: Vec<u8>,
        observed: Vec<u8>,
    },
    InvalidLocalPayloadLength {
        offset: usize,
        expected: u8,
        observed: u8,
    },
    NonAsciiName {
        offset: usize,
    },
    MissingFirstNoteTransition {
        offset: usize,
        observed: Option<u8>,
    },
}

impl fmt::Display for PatchError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBounds { start, end, size } => write!(
                formatter,
                "invalid Patch diagnostic bounds 0x{start:08x}..0x{end:08x} for {size} bytes"
            ),
            Self::Position(error) => write!(formatter, "absolute Patch position: {error}"),
            Self::UnresolvedInterval(error) => {
                write!(formatter, "unresolved post-PC interval-shaped field: {error}")
            }
            Self::UnexpectedPositionWidth { offset, bytes } => write!(
                formatter,
                "absolute Patch position at 0x{offset:08x} uses {bytes} bytes; expected 2"
            ),
            Self::UnexpectedUnresolvedIntervalWidth { offset, bytes } => write!(
                formatter,
                "unresolved post-PC interval-shaped field at 0x{offset:08x} uses {bytes} bytes; expected 2"
            ),
            Self::Truncated { offset, needed, end } => write!(
                formatter,
                "Patch diagnostic needs {needed} bytes at 0x{offset:08x} before 0x{end:08x}"
            ),
            Self::UnexpectedBytes { offset, expected, observed } => write!(
                formatter,
                "unexpected Patch context at 0x{offset:08x}: expected {expected:02x?}, observed {observed:02x?}"
            ),
            Self::InvalidLocalPayloadLength { offset, expected, observed } => write!(
                formatter,
                "local Patch payload length at 0x{offset:08x} is {observed}; expected {expected}"
            ),
            Self::NonAsciiName { offset } => {
                write!(formatter, "Patch name at 0x{offset:08x} is not ASCII")
            }
            Self::MissingFirstNoteTransition { offset, observed } => write!(
                formatter,
                "missing expected 0x90 first-note transition at 0x{offset:08x}; observed {observed:02x?}"
            ),
        }
    }
}

impl std::error::Error for PatchError {}

/// Decodes only the explicitly located, evidence-backed Track 3 #2 Patch.
pub fn decode_known_track3_2_patch(
    bytes: &[u8],
    start: usize,
    end: usize,
) -> Result<PatchDiagnostic, PatchError> {
    if start >= end || end > bytes.len() {
        return Err(PatchError::InvalidBounds {
            start,
            end,
            size: bytes.len(),
        });
    }

    let position = decode_7bit_be_vlq(bytes, start, end).map_err(PatchError::Position)?;
    if position.bytes_consumed != 2 {
        return Err(PatchError::UnexpectedPositionWidth {
            offset: start,
            bytes: position.bytes_consumed,
        });
    }

    let prefix_offset = checked_add(start, position.bytes_consumed, end)?;
    expect_bytes(bytes, prefix_offset, end, &PREFIX_AFTER_POSITION)?;
    let local_payload_length_offset = checked_add(prefix_offset, 2, end)?;
    let local_payload_length = read_byte(bytes, local_payload_length_offset, end)?;
    let before_name_offset = checked_add(local_payload_length_offset, 1, end)?;
    expect_bytes(bytes, before_name_offset, end, &PREFIX_BEFORE_NAME_LENGTH)?;

    let name_length_offset = checked_add(before_name_offset, PREFIX_BEFORE_NAME_LENGTH.len(), end)?;
    let name_length = read_byte(bytes, name_length_offset, end)?;
    let expected_payload_length =
        name_length
            .checked_add(15)
            .ok_or(PatchError::InvalidLocalPayloadLength {
                offset: local_payload_length_offset,
                expected: u8::MAX,
                observed: local_payload_length,
            })?;
    if local_payload_length != expected_payload_length {
        return Err(PatchError::InvalidLocalPayloadLength {
            offset: local_payload_length_offset,
            expected: expected_payload_length,
            observed: local_payload_length,
        });
    }

    let name_offset = checked_add(name_length_offset, 1, end)?;
    let name_end = checked_add(name_offset, usize::from(name_length), end)?;
    let name_bytes = read_range(bytes, name_offset, name_end, end)?;
    if !name_bytes.is_ascii() {
        return Err(PatchError::NonAsciiName {
            offset: name_offset,
        });
    }
    let name = String::from_utf8(name_bytes.to_vec()).map_err(|_| PatchError::NonAsciiName {
        offset: name_offset,
    })?;

    expect_bytes(bytes, name_end, end, &CONTEXT_AFTER_NAME)?;
    let program_change_offset = checked_add(name_end, CONTEXT_AFTER_NAME.len(), end)?;
    let program_change = read_byte(bytes, program_change_offset, end)?;
    let after_program_offset = checked_add(program_change_offset, 1, end)?;
    let unresolved_interval = decode_7bit_be_vlq(bytes, after_program_offset, end)
        .map_err(PatchError::UnresolvedInterval)?;
    if unresolved_interval.bytes_consumed != 2 {
        return Err(PatchError::UnexpectedUnresolvedIntervalWidth {
            offset: after_program_offset,
            bytes: unresolved_interval.bytes_consumed,
        });
    }
    let before_note_status_offset = checked_add(
        after_program_offset,
        unresolved_interval.bytes_consumed,
        end,
    )?;
    expect_bytes(
        bytes,
        before_note_status_offset,
        end,
        &CONTEXT_BEFORE_NOTE_STATUS,
    )?;

    let first_note_status_offset = checked_add(
        before_note_status_offset,
        CONTEXT_BEFORE_NOTE_STATUS.len(),
        end,
    )?;
    let first_note_status = bytes.get(first_note_status_offset).copied();
    if first_note_status_offset >= end || first_note_status != Some(EXPECTED_NOTE_STATUS) {
        return Err(PatchError::MissingFirstNoteTransition {
            offset: first_note_status_offset,
            observed: first_note_status.filter(|_| first_note_status_offset < end),
        });
    }

    Ok(PatchDiagnostic {
        position: position.value,
        position_offset: start,
        position_bytes: position.bytes_consumed,
        local_payload_length,
        local_payload_length_offset,
        name_length,
        name_length_offset,
        name,
        name_offset,
        program_change,
        program_change_offset,
        first_note_status: EXPECTED_NOTE_STATUS,
        first_note_status_offset,
    })
}

fn checked_add(offset: usize, amount: usize, end: usize) -> Result<usize, PatchError> {
    let next = offset.checked_add(amount).ok_or(PatchError::Truncated {
        offset,
        needed: amount,
        end,
    })?;
    if next > end {
        return Err(PatchError::Truncated {
            offset,
            needed: amount,
            end,
        });
    }
    Ok(next)
}

fn read_byte(bytes: &[u8], offset: usize, end: usize) -> Result<u8, PatchError> {
    if offset >= end {
        return Err(PatchError::Truncated {
            offset,
            needed: 1,
            end,
        });
    }
    bytes.get(offset).copied().ok_or(PatchError::Truncated {
        offset,
        needed: 1,
        end,
    })
}

fn read_range(
    bytes: &[u8],
    start: usize,
    range_end: usize,
    end: usize,
) -> Result<&[u8], PatchError> {
    if range_end > end {
        return Err(PatchError::Truncated {
            offset: start,
            needed: range_end.saturating_sub(start),
            end,
        });
    }
    bytes.get(start..range_end).ok_or(PatchError::Truncated {
        offset: start,
        needed: range_end.saturating_sub(start),
        end,
    })
}

fn expect_bytes(
    bytes: &[u8],
    offset: usize,
    end: usize,
    expected: &[u8],
) -> Result<(), PatchError> {
    let observed_end = checked_add(offset, expected.len(), end)?;
    let observed = read_range(bytes, offset, observed_end, end)?;
    if observed != expected {
        return Err(PatchError::UnexpectedBytes {
            offset,
            expected: expected.to_vec(),
            observed: observed.to_vec(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        decode_bounded_patch_representation, decode_known_track3_2_patch, BoundedPatchError,
        PatchError, PatchRepresentationBounds,
    };
    use crate::track7::VlqError;

    fn fixture(name: &[u8], position: [u8; 2], program: u8) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(position);
        bytes.extend([0xff, 0x7c, name.len() as u8 + 15]);
        bytes.extend([0x00, 0x00, 0x17, 0x00, 0x17, name.len() as u8]);
        bytes.extend(name);
        bytes.extend([0x03, b'I', b'3', b'8', 0x04, 0xff, 0xff, 0xff, program]);
        bytes.extend([
            0xc5, 0x4c, 0xff, 0x60, 0x07, 0x57, 0x7f, 0x00, 0x6c, 0x6c, 0xa3, 0x4a, 0x81, 0x25,
            0x90,
        ]);
        bytes
    }

    fn common_fixture(
        position: &[u8],
        name: &[u8],
        post_name: &[u8],
        program: u8,
        timing: &[u8],
        pre_note: &[u8],
    ) -> Vec<u8> {
        let payload_length = 5 + 1 + name.len() + post_name.len() + 1;
        let mut bytes = Vec::new();
        bytes.extend(position);
        bytes.extend([0xff, 0x7c, payload_length as u8]);
        bytes.extend([1, 2, 3, 4, 5, name.len() as u8]);
        bytes.extend(name);
        bytes.extend(post_name);
        bytes.push(program);
        bytes.extend(timing);
        bytes.extend(pre_note);
        bytes.push(0x90);
        bytes
    }

    fn common_decode(
        bytes: &[u8],
    ) -> Result<super::BoundedPatchRepresentation<'_>, BoundedPatchError> {
        decode_bounded_patch_representation(
            bytes,
            PatchRepresentationBounds {
                position_start: 0,
                note_status_end: bytes.len(),
            },
        )
    }

    #[test]
    fn shared_decoder_preserves_variable_fields_and_opaque_provenance() {
        let bytes = common_fixture(&[0x00], b"", &[0xaa, 0xbb], 0xfe, &[0x01], &[0x90, 0xcc]);
        let result = common_decode(&bytes).unwrap();
        assert_eq!(
            (result.position.value, result.position.bytes),
            (0, &[0x00][..])
        );
        assert_eq!(result.position.range, 0..1);
        assert_eq!(result.pre_name_context.bytes, &[1, 2, 3, 4, 5]);
        assert_eq!(result.name.text, "");
        assert_eq!(result.post_name_context.bytes, &[0xaa, 0xbb]);
        assert_eq!(result.program_change.value, 0xfe);
        assert_eq!(result.post_pc_timing_component.value, 1);
        assert_eq!(result.pre_note_context.bytes, &[0x90, 0xcc]);
        assert_eq!(result.note_status.offset, bytes.len() - 1);
    }

    #[test]
    fn shared_decoder_rejects_invalid_bounds_and_position_vlqs() {
        let bytes = common_fixture(&[0x00], b"A", &[], 1, &[0x01], &[]);
        assert!(matches!(
            decode_bounded_patch_representation(
                &bytes,
                PatchRepresentationBounds {
                    position_start: 0,
                    note_status_end: bytes.len() + 1
                }
            ),
            Err(BoundedPatchError::InvalidBounds { .. })
        ));

        assert_eq!(
            common_decode(&[0x81, 0x90]),
            Err(BoundedPatchError::PositionVlq(VlqError::Truncated {
                offset: 0
            }))
        );
        assert_eq!(
            common_decode(&[0x81, 0x81, 0x81, 0x81, 0x00, 0x90]),
            Err(BoundedPatchError::PositionVlq(VlqError::TooLong {
                offset: 0,
                maximum: 4
            }))
        );
    }

    #[test]
    fn shared_decoder_rejects_marker_and_payload_failures() {
        let mut wrong_marker = common_fixture(&[0x00], b"A", &[], 1, &[0x01], &[]);
        wrong_marker[1] = 0xfe;
        assert!(matches!(
            common_decode(&wrong_marker),
            Err(BoundedPatchError::MissingMarker { offset: 1, .. })
        ));
        assert!(matches!(
            common_decode(&[0x00, 0xff, 0x90]),
            Err(BoundedPatchError::MissingMarker { .. })
        ));

        let mut too_short = common_fixture(&[0x00], b"A", &[], 1, &[0x01], &[]);
        too_short[3] = 6;
        assert!(matches!(
            common_decode(&too_short),
            Err(BoundedPatchError::PayloadTooShort { length: 6, .. })
        ));

        let mut beyond = common_fixture(&[0x00], b"A", &[], 1, &[0x01], &[]);
        beyond[3] = 100;
        assert!(matches!(
            common_decode(&beyond),
            Err(BoundedPatchError::PayloadExceedsBoundary { .. })
        ));
    }

    #[test]
    fn shared_decoder_rejects_name_and_program_failures() {
        let mut crossing = common_fixture(&[0x00], b"A", &[], 1, &[0x01], &[]);
        crossing[9] = 2;
        assert!(matches!(
            common_decode(&crossing),
            Err(BoundedPatchError::NameLengthExceedsPayload { .. })
        ));

        let mut non_ascii = common_fixture(&[0x00], b"A", &[], 1, &[0x01], &[]);
        non_ascii[10] = 0xff;
        assert!(matches!(
            common_decode(&non_ascii),
            Err(BoundedPatchError::InvalidAsciiName { .. })
        ));

        let no_pc = [0x00, 0xff, 0x7c, 0x06, 1, 2, 3, 4, 5, 0, 0x01, 0x90];
        assert!(matches!(
            common_decode(&no_pc),
            Err(BoundedPatchError::PayloadTooShort { .. })
        ));
    }

    #[test]
    fn shared_decoder_rejects_post_pc_and_status_failures() {
        let mut truncated_timing = common_fixture(&[0x00], b"A", &[], 1, &[0x81], &[]);
        assert_eq!(
            common_decode(&truncated_timing),
            Err(BoundedPatchError::PostPcVlq(VlqError::Truncated {
                offset: 12
            }))
        );

        truncated_timing.insert(truncated_timing.len() - 1, 0x81);
        truncated_timing.insert(truncated_timing.len() - 1, 0x81);
        truncated_timing.insert(truncated_timing.len() - 1, 0x81);
        assert!(matches!(
            common_decode(&truncated_timing),
            Err(BoundedPatchError::PostPcVlq(VlqError::TooLong { .. }))
        ));

        let mut wrong_status = common_fixture(&[0x00], b"A", &[], 1, &[0x01], &[]);
        *wrong_status.last_mut().unwrap() = 0x91;
        assert!(matches!(
            common_decode(&wrong_status),
            Err(BoundedPatchError::MissingNoteStatus {
                observed: Some(0x91),
                ..
            })
        ));
        assert!(matches!(
            decode_bounded_patch_representation(
                &[],
                PatchRepresentationBounds {
                    position_start: 0,
                    note_status_end: 0
                }
            ),
            Err(BoundedPatchError::InvalidBounds { .. })
        ));
    }

    #[test]
    fn decodes_baseline_and_variable_length_layouts() {
        for (name, position, program) in [
            (&b"Ming Dynasty"[..], 530, 23),
            (&b"Phoenix Test"[..], 530, 23),
            (&b"Phoenix"[..], 530, 23),
        ] {
            let bytes = fixture(name, [0x84, 0x12], program);
            let result = decode_known_track3_2_patch(&bytes, 0, bytes.len()).unwrap();
            assert_eq!(result.position, position);
            assert_eq!(result.name, String::from_utf8_lossy(name));
            assert_eq!(result.program_change, program);
            assert_eq!(result.first_note_status, 0x90);
            assert_eq!(result.first_note_status_offset, 34 + name.len());
        }
    }

    #[test]
    fn decodes_controlled_position_and_program_values() {
        for (position_bytes, position, program) in [
            ([0x84, 0x12], 530, 24),
            ([0x84, 0x12], 530, 100),
            ([0x84, 0x13], 531, 23),
        ] {
            let bytes = fixture(b"Ming Dynasty", position_bytes, program);
            let result = decode_known_track3_2_patch(&bytes, 0, bytes.len()).unwrap();
            assert_eq!(
                (result.position, result.program_change),
                (position, program)
            );
        }
    }

    #[test]
    fn rejects_truncated_name_and_non_ascii_name() {
        let mut truncated = fixture(b"Phoenix", [0x84, 0x12], 23);
        truncated.truncate(17);
        assert!(matches!(
            decode_known_track3_2_patch(&truncated, 0, truncated.len()),
            Err(PatchError::Truncated { .. })
        ));

        let mut non_ascii = fixture(b"Phoenix", [0x84, 0x12], 23);
        non_ascii[11] = 0xff;
        assert!(matches!(
            decode_known_track3_2_patch(&non_ascii, 0, non_ascii.len()),
            Err(PatchError::NonAsciiName { .. })
        ));
    }

    #[test]
    fn rejects_malformed_position_vlq() {
        let mut bytes = fixture(b"Phoenix", [0x84, 0x12], 23);
        bytes[0..5].copy_from_slice(&[0x81, 0x81, 0x81, 0x81, 0x00]);
        assert!(matches!(
            decode_known_track3_2_patch(&bytes, 0, bytes.len()),
            Err(PatchError::Position(_))
        ));
    }

    #[test]
    fn rejects_missing_note_transition() {
        let mut bytes = fixture(b"Phoenix", [0x84, 0x12], 23);
        let last = bytes.len() - 1;
        bytes[last] = 0x91;
        assert_eq!(
            decode_known_track3_2_patch(&bytes, 0, bytes.len()),
            Err(PatchError::MissingFirstNoteTransition {
                offset: last,
                observed: Some(0x91)
            })
        );
    }
}
