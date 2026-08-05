//! Evidence-first comparison of complete Standard MIDI File track chunks.
//!
//! This module treats every declared `MTrk` payload as an opaque byte sequence.
//! It does not decode MIDI events or assign musical meaning to chunk order,
//! content, or equality.

use sha2::{Digest, Sha256};
use std::fmt;

const HEADER_IDENTIFIER: &[u8; 4] = b"MThd";
const TRACK_IDENTIFIER: &[u8; 4] = b"MTrk";
const CHUNK_HEADER_LENGTH: usize = 8;
const REQUIRED_HEADER_PAYLOAD_LENGTH: u32 = 6;

/// Identifies which supplied file produced a comparison error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComparisonFile {
    Left,
    Right,
}

/// A directly observed problem that prevents complete `MTrk` measurement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SmfTrackChunkError {
    MissingHeaderIdentifier {
        observed: Option<[u8; 4]>,
    },
    IncompleteHeader {
        available_length: usize,
    },
    DeclaredHeaderTooShort {
        declared_length: u32,
    },
    HeaderBoundaryBeyondData {
        declared_length: u32,
        data_length: usize,
    },
    IncompleteChunkHeader {
        offset: usize,
        available_length: usize,
    },
    UnexpectedChunkIdentifier {
        chunk_index: usize,
        offset: usize,
        observed: [u8; 4],
    },
    ChunkPayloadBeyondData {
        chunk_index: usize,
        payload_offset: usize,
        declared_length: u32,
        available_length: usize,
    },
    DeclaredTrackCountMismatch {
        declared_track_count: u16,
        observed_chunk_count: usize,
    },
}

impl fmt::Display for SmfTrackChunkError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "incomplete SMF track-chunk evidence: {self:?}")
    }
}

impl std::error::Error for SmfTrackChunkError {}

/// An error associated with one side of a two-file comparison.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SmfTrackChunkComparisonError {
    pub file: ComparisonFile,
    pub observation: SmfTrackChunkError,
}

impl fmt::Display for SmfTrackChunkComparisonError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "cannot compare {:?} file: {}",
            self.file, self.observation
        )
    }
}

impl std::error::Error for SmfTrackChunkComparisonError {}

/// Measurements for one complete, declared `MTrk` payload.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SmfTrackChunkFingerprint {
    /// Zero-based chunk order within this file.
    pub chunk_index: usize,
    pub declared_chunk_length: u32,
    /// SHA-256 of the payload only; the eight-byte `MTrk` header is excluded.
    pub sha256: String,
    /// Whether any complete payload in the other file is byte-for-byte equal.
    pub identical_chunk_exists_in_comparison_file: bool,
}

/// Structured measurements for one side of a comparison.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SmfFileTrackChunks {
    pub chunks: Vec<SmfTrackChunkFingerprint>,
}

/// Opaque track-chunk comparison results for both supplied files.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SmfTrackChunkComparison {
    pub left: SmfFileTrackChunks,
    pub right: SmfFileTrackChunks,
}

#[derive(Debug)]
struct ObservedChunk<'a> {
    chunk_index: usize,
    declared_chunk_length: u32,
    payload: &'a [u8],
}

/// Compares complete `MTrk` payloads without decoding MIDI events.
///
/// Equality means only that an identical opaque payload exists somewhere in
/// the other file. It does not establish matching tracks, musical parts, or
/// user actions.
pub fn compare_smf_track_chunks(
    left: &[u8],
    right: &[u8],
) -> Result<SmfTrackChunkComparison, SmfTrackChunkComparisonError> {
    let left_chunks = observe_chunks(left).map_err(|observation| SmfTrackChunkComparisonError {
        file: ComparisonFile::Left,
        observation,
    })?;
    let right_chunks =
        observe_chunks(right).map_err(|observation| SmfTrackChunkComparisonError {
            file: ComparisonFile::Right,
            observation,
        })?;

    Ok(SmfTrackChunkComparison {
        left: fingerprint_chunks(&left_chunks, &right_chunks),
        right: fingerprint_chunks(&right_chunks, &left_chunks),
    })
}

fn fingerprint_chunks(
    chunks: &[ObservedChunk<'_>],
    comparison_chunks: &[ObservedChunk<'_>],
) -> SmfFileTrackChunks {
    let chunks = chunks
        .iter()
        .map(|chunk| SmfTrackChunkFingerprint {
            chunk_index: chunk.chunk_index,
            declared_chunk_length: chunk.declared_chunk_length,
            sha256: format!("{:x}", Sha256::digest(chunk.payload)),
            identical_chunk_exists_in_comparison_file: comparison_chunks
                .iter()
                .any(|candidate| candidate.payload == chunk.payload),
        })
        .collect();
    SmfFileTrackChunks { chunks }
}

fn observe_chunks(data: &[u8]) -> Result<Vec<ObservedChunk<'_>>, SmfTrackChunkError> {
    let observed_header = data.get(..4).map(|bytes| {
        let mut observed = [0; 4];
        observed.copy_from_slice(bytes);
        observed
    });
    if observed_header.as_ref() != Some(HEADER_IDENTIFIER) {
        return Err(SmfTrackChunkError::MissingHeaderIdentifier {
            observed: observed_header,
        });
    }

    if data.len() < CHUNK_HEADER_LENGTH {
        return Err(SmfTrackChunkError::IncompleteHeader {
            available_length: data.len(),
        });
    }

    let declared_header_length = read_u32(data, 4).expect("complete SMF chunk header");
    if declared_header_length < REQUIRED_HEADER_PAYLOAD_LENGTH {
        return Err(SmfTrackChunkError::DeclaredHeaderTooShort {
            declared_length: declared_header_length,
        });
    }

    let header_boundary = CHUNK_HEADER_LENGTH
        .checked_add(declared_header_length as usize)
        .filter(|boundary| *boundary <= data.len())
        .ok_or(SmfTrackChunkError::HeaderBoundaryBeyondData {
            declared_length: declared_header_length,
            data_length: data.len(),
        })?;
    let declared_track_count =
        read_u16(data, 10).ok_or(SmfTrackChunkError::HeaderBoundaryBeyondData {
            declared_length: declared_header_length,
            data_length: data.len(),
        })?;

    let mut chunks = Vec::new();
    let mut offset = header_boundary;
    while offset < data.len() {
        let available_length = data.len() - offset;
        if available_length < CHUNK_HEADER_LENGTH {
            return Err(SmfTrackChunkError::IncompleteChunkHeader {
                offset,
                available_length,
            });
        }

        let mut observed = [0; 4];
        observed.copy_from_slice(&data[offset..offset + 4]);
        if &observed != TRACK_IDENTIFIER {
            return Err(SmfTrackChunkError::UnexpectedChunkIdentifier {
                chunk_index: chunks.len(),
                offset,
                observed,
            });
        }

        let declared_chunk_length = read_u32(data, offset + 4).expect("complete chunk header");
        let payload_offset = offset + CHUNK_HEADER_LENGTH;
        let payload_boundary = payload_offset
            .checked_add(declared_chunk_length as usize)
            .filter(|boundary| *boundary <= data.len())
            .ok_or(SmfTrackChunkError::ChunkPayloadBeyondData {
                chunk_index: chunks.len(),
                payload_offset,
                declared_length: declared_chunk_length,
                available_length: data.len() - payload_offset,
            })?;
        chunks.push(ObservedChunk {
            chunk_index: chunks.len(),
            declared_chunk_length,
            payload: &data[payload_offset..payload_boundary],
        });
        offset = payload_boundary;
    }

    if chunks.len() != usize::from(declared_track_count) {
        return Err(SmfTrackChunkError::DeclaredTrackCountMismatch {
            declared_track_count,
            observed_chunk_count: chunks.len(),
        });
    }
    Ok(chunks)
}

fn read_u16(data: &[u8], offset: usize) -> Option<u16> {
    let bytes: [u8; 2] = data.get(offset..offset + 2)?.try_into().ok()?;
    Some(u16::from_be_bytes(bytes))
}

fn read_u32(data: &[u8], offset: usize) -> Option<u32> {
    let bytes: [u8; 4] = data.get(offset..offset + 4)?.try_into().ok()?;
    Some(u32::from_be_bytes(bytes))
}

#[cfg(test)]
mod tests {
    use super::{
        compare_smf_track_chunks, ComparisonFile, SmfTrackChunkComparisonError, SmfTrackChunkError,
    };

    fn smf(declared_track_count: u16, chunks: &[&[u8]]) -> Vec<u8> {
        let mut data = Vec::from(&b"MThd\0\0\0\x06\0\x01"[..]);
        data.extend_from_slice(&declared_track_count.to_be_bytes());
        data.extend_from_slice(&480_u16.to_be_bytes());
        for payload in chunks {
            data.extend_from_slice(b"MTrk");
            data.extend_from_slice(&(payload.len() as u32).to_be_bytes());
            data.extend_from_slice(payload);
        }
        data
    }

    #[test]
    fn reports_declared_lengths_hashes_and_cross_file_equality() {
        let left = smf(2, &[b"abc", b"left"]);
        let right = smf(2, &[b"right", b"abc"]);

        let comparison = compare_smf_track_chunks(&left, &right).unwrap();
        assert_eq!(comparison.left.chunks[0].chunk_index, 0);
        assert_eq!(comparison.left.chunks[0].declared_chunk_length, 3);
        assert_eq!(
            comparison.left.chunks[0].sha256,
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert!(comparison.left.chunks[0].identical_chunk_exists_in_comparison_file);
        assert!(!comparison.left.chunks[1].identical_chunk_exists_in_comparison_file);
        assert!(!comparison.right.chunks[0].identical_chunk_exists_in_comparison_file);
        assert!(comparison.right.chunks[1].identical_chunk_exists_in_comparison_file);
    }

    #[test]
    fn duplicate_payloads_each_report_that_an_identical_payload_exists() {
        let left = smf(2, &[b"same", b"same"]);
        let right = smf(1, &[b"same"]);

        let comparison = compare_smf_track_chunks(&left, &right).unwrap();
        assert!(comparison
            .left
            .chunks
            .iter()
            .all(|chunk| chunk.identical_chunk_exists_in_comparison_file));
        assert!(comparison.right.chunks[0].identical_chunk_exists_in_comparison_file);
    }

    #[test]
    fn fingerprints_empty_payloads_without_decoding_events() {
        let file = smf(1, &[b""]);
        let comparison = compare_smf_track_chunks(&file, &file).unwrap();

        assert_eq!(comparison.left.chunks[0].declared_chunk_length, 0);
        assert_eq!(
            comparison.left.chunks[0].sha256,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn accepts_and_skips_observed_extra_header_bytes() {
        let mut file = smf(1, &[b"payload"]);
        file[7] = 8;
        file.splice(14..14, [0xaa, 0xbb]);

        let comparison = compare_smf_track_chunks(&file, &file).unwrap();
        assert_eq!(comparison.left.chunks[0].declared_chunk_length, 7);
    }

    #[test]
    fn identifies_the_file_with_invalid_header_evidence() {
        let valid = smf(0, &[]);
        let error = compare_smf_track_chunks(&valid, b"not an smf").unwrap_err();

        assert_eq!(
            error,
            SmfTrackChunkComparisonError {
                file: ComparisonFile::Right,
                observation: SmfTrackChunkError::MissingHeaderIdentifier {
                    observed: Some(*b"not "),
                },
            }
        );
    }

    #[test]
    fn rejects_declared_track_count_that_differs_from_observed_chunks() {
        let invalid = smf(2, &[b"one"]);
        let error = compare_smf_track_chunks(&invalid, &smf(0, &[])).unwrap_err();

        assert_eq!(error.file, ComparisonFile::Left);
        assert_eq!(
            error.observation,
            SmfTrackChunkError::DeclaredTrackCountMismatch {
                declared_track_count: 2,
                observed_chunk_count: 1,
            }
        );
    }

    #[test]
    fn rejects_payload_that_crosses_the_data_boundary() {
        let mut invalid = smf(1, &[b"abc"]);
        invalid[18..22].copy_from_slice(&4_u32.to_be_bytes());
        let error = compare_smf_track_chunks(&invalid, &invalid).unwrap_err();

        assert_eq!(
            error.observation,
            SmfTrackChunkError::ChunkPayloadBeyondData {
                chunk_index: 0,
                payload_offset: 22,
                declared_length: 4,
                available_length: 3,
            }
        );
    }

    #[test]
    fn rejects_non_track_chunk_and_trailing_bytes_as_direct_observations() {
        let mut non_track = smf(1, &[b"abc"]);
        non_track[14..18].copy_from_slice(b"JUNK");
        let non_track_error = compare_smf_track_chunks(&non_track, &non_track).unwrap_err();
        assert_eq!(
            non_track_error.observation,
            SmfTrackChunkError::UnexpectedChunkIdentifier {
                chunk_index: 0,
                offset: 14,
                observed: *b"JUNK",
            }
        );

        let mut trailing = smf(1, &[b"abc"]);
        trailing.extend_from_slice(&[1, 2, 3]);
        let trailing_error = compare_smf_track_chunks(&trailing, &trailing).unwrap_err();
        assert_eq!(
            trailing_error.observation,
            SmfTrackChunkError::IncompleteChunkHeader {
                offset: 25,
                available_length: 3,
            }
        );
    }

    #[test]
    fn rejects_header_lengths_that_cannot_contain_required_measurements() {
        let mut short_header = smf(0, &[]);
        short_header[4..8].copy_from_slice(&5_u32.to_be_bytes());
        let error = compare_smf_track_chunks(&short_header, &short_header).unwrap_err();
        assert_eq!(
            error.observation,
            SmfTrackChunkError::DeclaredHeaderTooShort { declared_length: 5 }
        );
    }

    #[test]
    fn rejects_truncated_header_without_inventing_a_declared_length() {
        let error = compare_smf_track_chunks(b"MThd\0", &smf(0, &[])).unwrap_err();
        assert_eq!(
            error.observation,
            SmfTrackChunkError::IncompleteHeader {
                available_length: 5,
            }
        );
    }
}
