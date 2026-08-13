//! Bounded diagnostics for the known Track 3 #2 first Patch representation.
//!
//! This module does not discover Patch events or define a general Studio
//! Vision event grammar. Callers must supply the exact evidence-backed start
//! and end offsets for the known representation.

use crate::track7::{decode_7bit_be_vlq, VlqError};
use std::fmt;

const PREFIX_AFTER_POSITION: [u8; 2] = [0xff, 0x7c];
const PREFIX_BEFORE_NAME_LENGTH: [u8; 5] = [0x00, 0x00, 0x17, 0x00, 0x17];
const CONTEXT_AFTER_NAME: [u8; 8] = [0x03, b'I', b'3', b'8', 0x04, 0xff, 0xff, 0xff];
const CONTEXT_BEFORE_NOTE_STATUS: [u8; 12] = [
    0xff, 0x60, 0x07, 0x57, 0x7f, 0x00, 0x6c, 0x6c, 0xa3, 0x4a, 0x81, 0x25,
];
const EXPECTED_NOTE_STATUS: u8 = 0x90;

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
    use super::{decode_known_track3_2_patch, PatchError};

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
