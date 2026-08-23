//! Read-only framing and classification for the established Studio Vision
//! root-record stream and 166-byte-descriptor sequence form.
//!
//! This module does not scan, support the older 120-byte sequence form, parse
//! mixed events, or infer exact performance-event ends.

use std::fmt;
use std::ops::Range;

use crate::patch::{LocatedByte, LocatedBytes};

const ROOT_HEADER_LENGTH: usize = 8;
const RECORD_HEADER_LENGTH: usize = 5;
const SEQUENCE_TYPE: u8 = 0x01;
const NAME_RECORD_TYPE: u8 = 0x07;
const PRELUDE_TYPE: u8 = 0x09;
const PRIMARY_TYPE: u8 = 0x02;
const SECONDARY_TYPE: u8 = 0x29;
const TERMINAL_TYPE: u8 = 0x00;
const SEQUENCE_PREAMBLE_LENGTH: usize = 208;
const DESCRIPTOR_STRIDE: usize = 166;
const DESCRIPTOR_LABEL_OFFSET: usize = 15;
const NAME_OFFSET_BACKSTEP: usize = 15;
const PRIMARY_EVENT_OFFSET: usize = 14;
const METER_REPRESENTATION_LENGTH: usize = 8;
const TEMPO_REPRESENTATION_LENGTH: usize = 7;

/// One raw big-endian root-header word with absolute provenance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocatedBeU16<'a> {
    pub value: u16,
    pub bytes: &'a [u8],
    pub range: Range<usize>,
}

/// The opaque eight-byte project root header.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RootHeader<'a> {
    pub range: Range<usize>,
    pub raw: LocatedBytes<'a>,
    pub raw_words: [LocatedBeU16<'a>; 4],
}

/// One checked `type | u32-BE length | payload` record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FramedRecord<'a> {
    pub record_range: Range<usize>,
    pub record_type: LocatedByte,
    pub length_bytes: LocatedBytes<'a>,
    pub payload_length: u32,
    pub payload: LocatedBytes<'a>,
}

/// A complete generic root walk. No semantic sequence profile is implied.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RootRecordStream<'a> {
    pub root_header: RootHeader<'a>,
    pub records: Vec<FramedRecord<'a>>,
    pub consumed_range: Range<usize>,
}

/// Deterministic errors from generic root framing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RootRecordError {
    TruncatedRootHeader {
        available: usize,
        expected: usize,
    },
    TruncatedRecordHeader {
        offset: usize,
        available: usize,
        expected: usize,
    },
    RecordLengthOverflow {
        offset: usize,
        payload_length: u32,
    },
    PayloadBeyondInput {
        record_offset: usize,
        payload_start: usize,
        payload_end: usize,
        input_len: usize,
    },
    TopLevelDidNotConsumeInput {
        cursor: usize,
        input_len: usize,
    },
}

impl fmt::Display for RootRecordError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TruncatedRootHeader {
                available,
                expected,
            } => write!(
                formatter,
                "root header has {available} bytes; expected {expected}"
            ),
            Self::TruncatedRecordHeader {
                offset,
                available,
                expected,
            } => write!(
                formatter,
                "record header at 0x{offset:08x} has {available} bytes; expected {expected}"
            ),
            Self::RecordLengthOverflow {
                offset,
                payload_length,
            } => write!(
                formatter,
                "record at 0x{offset:08x} has overflowing payload length {payload_length}"
            ),
            Self::PayloadBeyondInput {
                record_offset,
                payload_start,
                payload_end,
                input_len,
            } => write!(
                formatter,
                "record at 0x{record_offset:08x} declares payload 0x{payload_start:08x}..0x{payload_end:08x} beyond {input_len} bytes"
            ),
            Self::TopLevelDidNotConsumeInput { cursor, input_len } => write!(
                formatter,
                "top-level walk stopped at 0x{cursor:08x} before input end 0x{input_len:08x}"
            ),
        }
    }
}

impl std::error::Error for RootRecordError {}

/// The only semantic sequence layout supported by this module.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SequenceContainerProfile {
    Descriptor166,
}

/// One raw 166-byte sequence descriptor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SequenceDescriptor<'a> {
    pub ordinal: usize,
    pub range: Range<usize>,
    pub raw: LocatedBytes<'a>,
    pub label_start: usize,
    pub label: Option<LocatedBytes<'a>>,
    pub label_terminator: Option<LocatedByte>,
}

/// A structurally derived Pascal sequence name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SequenceName<'a> {
    pub length: LocatedByte,
    pub bytes: LocatedBytes<'a>,
}

impl SequenceName<'_> {
    /// Returns a UTF-8 view when the preserved legacy bytes happen to be UTF-8.
    pub fn as_utf8(&self) -> Option<&str> {
        std::str::from_utf8(self.bytes.bytes).ok()
    }
}

/// One structurally paired track primary and secondary record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackRecordPair<'a> {
    pub pair_ordinal: usize,
    pub primary: FramedRecord<'a>,
    pub secondary: FramedRecord<'a>,
    pub candidate_event_start: usize,
    /// A containing upper bound, not an exact performance-event range.
    pub event_containing_range: Range<usize>,
}

/// Exact half-open bounds for a Descriptor166 track payload.
///
/// The event range excludes the validated seven-byte terminal structure. This
/// helper is intentionally scoped to the established Descriptor166 grammar;
/// it is not a format-wide Studio Vision terminator rule.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackEventBounds {
    pub primary_range: Range<usize>,
    pub payload_range: Range<usize>,
    pub event_range: Range<usize>,
    pub tail_range: Range<usize>,
}

/// Failures while deriving exact Descriptor166 track-event bounds.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrackEventBoundsError {
    PairNotFound {
        pair_ordinal: usize,
        pair_count: usize,
    },
    MalformedBounds {
        detail: &'static str,
    },
    PayloadTooShort {
        payload_range: Range<usize>,
        required: usize,
    },
    ArithmeticOverflow {
        detail: &'static str,
    },
    EventStartAfterEnd {
        event_start: usize,
        event_end: usize,
    },
    InvalidTerminalGrammar {
        tail_range: Range<usize>,
        observed: [u8; 7],
    },
}

impl fmt::Display for TrackEventBoundsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for TrackEventBoundsError {}

/// Evidence status for descriptor-to-record-pair association.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrackAssociations {
    Ordinal(Vec<TrackBinding>),
    Unresolved {
        descriptor_count: usize,
        pair_count: usize,
    },
}

/// One equal-count ordinal descriptor/pair binding.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackBinding {
    pub descriptor_ordinal: usize,
    pub pair_ordinal: usize,
}

/// One validated sequence container in the established 166-byte form.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SequenceContainer<'a> {
    pub sequence_range: Range<usize>,
    pub candidate_record_index: usize,
    pub preamble: LocatedBytes<'a>,
    pub descriptor_count: LocatedByte,
    pub descriptors: Vec<SequenceDescriptor<'a>>,
    pub sequence_name: SequenceName<'a>,
    pub name_record: FramedRecord<'a>,
    pub prelude_records: Vec<FramedRecord<'a>>,
    pub meter_primary: FramedRecord<'a>,
    pub meter_secondary: FramedRecord<'a>,
    pub tempo_primary: FramedRecord<'a>,
    pub tempo_secondary: FramedRecord<'a>,
    pub initial_meter_range: Range<usize>,
    pub initial_tempo_range: Range<usize>,
    pub track_pairs: Vec<TrackRecordPair<'a>>,
    pub track_associations: TrackAssociations,
    pub terminal_record: FramedRecord<'a>,
}

impl SequenceContainer<'_> {
    pub fn track_descriptors(&self) -> &[SequenceDescriptor<'_>] {
        &self.descriptors[2..]
    }

    /// Derives exact event bounds for one ordinal-associated Descriptor166
    /// track pair after validating its terminal seven-byte grammar.
    pub fn validated_track_event_bounds(
        &self,
        pair_ordinal: usize,
    ) -> Result<TrackEventBounds, TrackEventBoundsError> {
        let pair =
            self.track_pairs
                .get(pair_ordinal)
                .ok_or(TrackEventBoundsError::PairNotFound {
                    pair_ordinal,
                    pair_count: self.track_pairs.len(),
                })?;
        pair.validated_event_bounds()
    }
}

impl TrackRecordPair<'_> {
    /// Derives exact event bounds from already parsed Descriptor166 facts.
    ///
    /// The payload prefix and seven-byte suffix are validated in place; no
    /// forward or backward heuristic search is performed.
    pub fn validated_event_bounds(&self) -> Result<TrackEventBounds, TrackEventBoundsError> {
        let primary_range = self.primary.record_range.clone();
        let payload_range = self.primary.payload.range.clone();
        if payload_range.start < primary_range.start
            || payload_range.end > primary_range.end
            || payload_range.start > payload_range.end
            || self.event_containing_range.start != self.candidate_event_start
            || self.event_containing_range.end != payload_range.end
        {
            return Err(TrackEventBoundsError::MalformedBounds {
                detail: "parsed payload and containing bounds are inconsistent",
            });
        }

        let expected_start = payload_range
            .start
            .checked_add(PRIMARY_EVENT_OFFSET)
            .ok_or(TrackEventBoundsError::ArithmeticOverflow {
                detail: "event start",
            })?;
        if expected_start != self.candidate_event_start {
            return Err(TrackEventBoundsError::MalformedBounds {
                detail: "candidate event start does not match the parsed payload prefix",
            });
        }

        let payload_length = payload_range.end - payload_range.start;
        if payload_length < 7 {
            return Err(TrackEventBoundsError::PayloadTooShort {
                payload_range,
                required: 7,
            });
        }
        let event_end =
            payload_range
                .end
                .checked_sub(7)
                .ok_or(TrackEventBoundsError::ArithmeticOverflow {
                    detail: "terminal tail",
                })?;
        if self.candidate_event_start > event_end {
            return Err(TrackEventBoundsError::EventStartAfterEnd {
                event_start: self.candidate_event_start,
                event_end,
            });
        }
        let tail_range = event_end..payload_range.end;
        let tail = self
            .primary
            .payload
            .bytes
            .get(tail_range.start - payload_range.start..tail_range.end - payload_range.start)
            .ok_or(TrackEventBoundsError::MalformedBounds {
                detail: "terminal tail is outside the parsed payload",
            })?;
        let observed: [u8; 7] =
            tail.try_into()
                .map_err(|_| TrackEventBoundsError::MalformedBounds {
                    detail: "terminal tail is not seven bytes",
                })?;
        if observed[0] != 0xff || observed[4] != 0xff || observed[5] != 0x2f || observed[6] != 0 {
            return Err(TrackEventBoundsError::InvalidTerminalGrammar {
                tail_range,
                observed,
            });
        }
        Ok(TrackEventBounds {
            primary_range,
            payload_range,
            event_range: self.candidate_event_start..event_end,
            tail_range,
        })
    }
}

/// A semantically classified project using only the explicit 166-byte profile.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParsedProject166<'a> {
    pub profile: SequenceContainerProfile,
    pub root_header: RootHeader<'a>,
    pub records: Vec<FramedRecord<'a>>,
    pub sequences: Vec<SequenceContainer<'a>>,
    pub consumed_range: Range<usize>,
}

/// Roles whose ordinal record types are established.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RequiredRecordRole {
    Name,
    MeterPrimary,
    MeterSecondary,
    TempoPrimary,
    TempoSecondary,
}

/// Roles whose primary payload capacity is established.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrimaryRole {
    Meter,
    Tempo,
    Track,
}

/// Exact sequence-candidate validation failures.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SequenceValidationError {
    PreambleBeyondInput {
        range: Range<usize>,
        input_len: usize,
    },
    DescriptorCountTooSmall {
        offset: usize,
        observed: u8,
        minimum: u8,
    },
    DescriptorArithmeticOverflow {
        candidate_offset: usize,
        count: usize,
        stride: usize,
    },
    DescriptorBeyondInput {
        ordinal: usize,
        range: Range<usize>,
        input_len: usize,
    },
    MissingRequiredSequenceRecord {
        role: RequiredRecordRole,
        record_index: usize,
    },
    WrongRequiredSequenceRecordType {
        role: RequiredRecordRole,
        offset: usize,
        observed: u8,
        expected: u8,
    },
    InvalidSequenceNameBounds {
        length_offset: usize,
        declared: u8,
        containing_range: Range<usize>,
        derived_end: usize,
    },
    PrimaryPayloadTooShort {
        role: PrimaryRole,
        payload_range: Range<usize>,
        required: usize,
    },
    MalformedTrackPair {
        record_index: usize,
        primary_offset: usize,
        secondary_type: Option<u8>,
    },
    MalformedSequenceTerminal {
        record_index: usize,
        offset: Option<usize>,
        observed_type: Option<u8>,
    },
}

impl fmt::Display for SequenceValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for SequenceValidationError {}

/// Deterministic failures from the 166-byte semantic project parser.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Project166Error {
    Root(RootRecordError),
    MalformedSequenceCandidate {
        record_index: usize,
        candidate_range: Range<usize>,
        cause: SequenceValidationError,
    },
}

impl fmt::Display for Project166Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Root(error) => write!(formatter, "root record stream: {error}"),
            Self::MalformedSequenceCandidate {
                record_index,
                candidate_range,
                cause,
            } => write!(
                formatter,
                "malformed 166-profile sequence candidate record {record_index} at 0x{:08x}..0x{:08x}: {cause}",
                candidate_range.start, candidate_range.end
            ),
        }
    }
}

impl std::error::Error for Project166Error {}

impl From<RootRecordError> for Project166Error {
    fn from(value: RootRecordError) -> Self {
        Self::Root(value)
    }
}

/// Frames one generic record at the exact supplied cursor without scanning.
pub fn frame_record_at(
    bytes: &[u8],
    cursor: usize,
) -> Result<(FramedRecord<'_>, usize), RootRecordError> {
    let available = bytes.len().saturating_sub(cursor);
    if available < RECORD_HEADER_LENGTH {
        return Err(RootRecordError::TruncatedRecordHeader {
            offset: cursor,
            available,
            expected: RECORD_HEADER_LENGTH,
        });
    }

    let length_start = cursor + 1;
    let payload_start = cursor + RECORD_HEADER_LENGTH;
    let length_bytes = &bytes[length_start..payload_start];
    let payload_length = u32::from_be_bytes(length_bytes.try_into().expect("four length bytes"));
    let payload_length_usize =
        usize::try_from(payload_length).map_err(|_| RootRecordError::RecordLengthOverflow {
            offset: cursor,
            payload_length,
        })?;
    let payload_end = checked_payload_end(payload_start, payload_length_usize).ok_or(
        RootRecordError::RecordLengthOverflow {
            offset: cursor,
            payload_length,
        },
    )?;
    if payload_end > bytes.len() {
        return Err(RootRecordError::PayloadBeyondInput {
            record_offset: cursor,
            payload_start,
            payload_end,
            input_len: bytes.len(),
        });
    }

    Ok((
        FramedRecord {
            record_range: cursor..payload_end,
            record_type: LocatedByte {
                value: bytes[cursor],
                offset: cursor,
            },
            length_bytes: located_bytes(bytes, length_start..payload_start),
            payload_length,
            payload: located_bytes(bytes, payload_start..payload_end),
        },
        payload_end,
    ))
}

/// Parses only the generic root header and record stream.
pub fn parse_root_record_stream(bytes: &[u8]) -> Result<RootRecordStream<'_>, RootRecordError> {
    let root_header = parse_root_header(bytes)?;
    let mut records = Vec::new();
    let mut cursor = ROOT_HEADER_LENGTH;
    while cursor < bytes.len() {
        let (record, next) = frame_record_at(bytes, cursor)?;
        records.push(record);
        cursor = next;
    }
    if cursor != bytes.len() {
        return Err(RootRecordError::TopLevelDidNotConsumeInput {
            cursor,
            input_len: bytes.len(),
        });
    }
    Ok(RootRecordStream {
        root_header,
        records,
        consumed_range: 0..cursor,
    })
}

/// Parses the explicit 208-byte-preamble/166-byte-descriptor sequence profile.
pub fn parse_project_166(bytes: &[u8]) -> Result<ParsedProject166<'_>, Project166Error> {
    let root = parse_root_record_stream(bytes)?;
    let mut sequences = Vec::new();
    for (record_index, record) in root.records.iter().enumerate() {
        if record.record_type.value != SEQUENCE_TYPE {
            continue;
        }
        let sequence =
            parse_sequence_candidate(bytes, &root.records, record_index).map_err(|cause| {
                Project166Error::MalformedSequenceCandidate {
                    record_index,
                    candidate_range: record.record_range.clone(),
                    cause,
                }
            })?;
        sequences.push(sequence);
    }

    Ok(ParsedProject166 {
        profile: SequenceContainerProfile::Descriptor166,
        root_header: root.root_header,
        records: root.records,
        sequences,
        consumed_range: root.consumed_range,
    })
}

fn parse_root_header(bytes: &[u8]) -> Result<RootHeader<'_>, RootRecordError> {
    if bytes.len() < ROOT_HEADER_LENGTH {
        return Err(RootRecordError::TruncatedRootHeader {
            available: bytes.len(),
            expected: ROOT_HEADER_LENGTH,
        });
    }
    let raw_words = std::array::from_fn(|ordinal| {
        let start = ordinal * 2;
        let range = start..start + 2;
        LocatedBeU16 {
            value: u16::from_be_bytes(bytes[range.clone()].try_into().expect("two root bytes")),
            bytes: &bytes[range.clone()],
            range,
        }
    });
    Ok(RootHeader {
        range: 0..ROOT_HEADER_LENGTH,
        raw: located_bytes(bytes, 0..ROOT_HEADER_LENGTH),
        raw_words,
    })
}

fn parse_sequence_candidate<'a>(
    bytes: &'a [u8],
    records: &[FramedRecord<'a>],
    candidate_index: usize,
) -> Result<SequenceContainer<'a>, SequenceValidationError> {
    let candidate = &records[candidate_index];
    let sequence_start = candidate.record_range.start;
    let preamble_end = sequence_start.checked_add(SEQUENCE_PREAMBLE_LENGTH).ok_or(
        SequenceValidationError::DescriptorArithmeticOverflow {
            candidate_offset: sequence_start,
            count: 0,
            stride: DESCRIPTOR_STRIDE,
        },
    )?;
    if preamble_end > bytes.len() {
        return Err(SequenceValidationError::PreambleBeyondInput {
            range: sequence_start..preamble_end,
            input_len: bytes.len(),
        });
    }
    let descriptor_count_offset = sequence_start + 5;
    let descriptor_count = LocatedByte {
        value: bytes[descriptor_count_offset],
        offset: descriptor_count_offset,
    };
    if descriptor_count.value < 2 {
        return Err(SequenceValidationError::DescriptorCountTooSmall {
            offset: descriptor_count.offset,
            observed: descriptor_count.value,
            minimum: 2,
        });
    }

    let count = usize::from(descriptor_count.value);
    let descriptor_bytes = count.checked_mul(DESCRIPTOR_STRIDE).ok_or(
        SequenceValidationError::DescriptorArithmeticOverflow {
            candidate_offset: sequence_start,
            count,
            stride: DESCRIPTOR_STRIDE,
        },
    )?;
    let descriptors_start = preamble_end;
    let descriptors_end = descriptors_start.checked_add(descriptor_bytes).ok_or(
        SequenceValidationError::DescriptorArithmeticOverflow {
            candidate_offset: sequence_start,
            count,
            stride: DESCRIPTOR_STRIDE,
        },
    )?;
    if descriptors_end > bytes.len() {
        return Err(SequenceValidationError::DescriptorBeyondInput {
            ordinal: count - 1,
            range: descriptors_end - DESCRIPTOR_STRIDE..descriptors_end,
            input_len: bytes.len(),
        });
    }

    let descriptors = (0..count)
        .map(|ordinal| {
            let start = descriptors_start + ordinal * DESCRIPTOR_STRIDE;
            descriptor(bytes, ordinal, start..start + DESCRIPTOR_STRIDE)
        })
        .collect();

    let name_offset = descriptors_end.checked_sub(NAME_OFFSET_BACKSTEP).ok_or(
        SequenceValidationError::DescriptorArithmeticOverflow {
            candidate_offset: sequence_start,
            count,
            stride: DESCRIPTOR_STRIDE,
        },
    )?;
    let name_record = required_record(
        records,
        candidate_index + 1,
        RequiredRecordRole::Name,
        NAME_RECORD_TYPE,
    )?;
    if name_offset >= bytes.len() {
        return Err(SequenceValidationError::InvalidSequenceNameBounds {
            length_offset: name_offset,
            declared: 0,
            containing_range: name_record.record_range.clone(),
            derived_end: name_offset,
        });
    }
    let name_length = LocatedByte {
        value: bytes[name_offset],
        offset: name_offset,
    };
    let name_start = name_offset + 1;
    let name_end = name_start
        .checked_add(usize::from(name_length.value))
        .ok_or(SequenceValidationError::InvalidSequenceNameBounds {
            length_offset: name_offset,
            declared: name_length.value,
            containing_range: name_record.record_range.clone(),
            derived_end: usize::MAX,
        })?;
    if name_offset < name_record.payload.range.start
        || name_end != name_record.record_range.end
        || name_end > bytes.len()
    {
        return Err(SequenceValidationError::InvalidSequenceNameBounds {
            length_offset: name_offset,
            declared: name_length.value,
            containing_range: name_record.record_range.clone(),
            derived_end: name_end,
        });
    }
    let sequence_name = SequenceName {
        length: name_length,
        bytes: located_bytes(bytes, name_start..name_end),
    };

    let mut cursor = candidate_index + 2;
    let mut prelude_records = Vec::new();
    while let Some(record) = records.get(cursor) {
        if record.record_type.value != PRELUDE_TYPE {
            break;
        }
        prelude_records.push(record.clone());
        cursor += 1;
    }

    let meter_primary = required_record(
        records,
        cursor,
        RequiredRecordRole::MeterPrimary,
        PRIMARY_TYPE,
    )?;
    cursor += 1;
    let meter_secondary = required_record(
        records,
        cursor,
        RequiredRecordRole::MeterSecondary,
        SECONDARY_TYPE,
    )?;
    cursor += 1;
    let tempo_primary = required_record(
        records,
        cursor,
        RequiredRecordRole::TempoPrimary,
        PRIMARY_TYPE,
    )?;
    cursor += 1;
    let tempo_secondary = required_record(
        records,
        cursor,
        RequiredRecordRole::TempoSecondary,
        SECONDARY_TYPE,
    )?;
    cursor += 1;

    let initial_meter_range = primary_range(
        &meter_primary,
        PrimaryRole::Meter,
        METER_REPRESENTATION_LENGTH,
    )?;
    let initial_tempo_range = primary_range(
        &tempo_primary,
        PrimaryRole::Tempo,
        TEMPO_REPRESENTATION_LENGTH,
    )?;

    let mut track_pairs = Vec::new();
    loop {
        let Some(record) = records.get(cursor) else {
            return Err(SequenceValidationError::MalformedSequenceTerminal {
                record_index: cursor,
                offset: None,
                observed_type: None,
            });
        };
        if record.record_type.value == TERMINAL_TYPE {
            break;
        }
        if record.record_type.value != PRIMARY_TYPE {
            return Err(SequenceValidationError::MalformedSequenceTerminal {
                record_index: cursor,
                offset: Some(record.record_type.offset),
                observed_type: Some(record.record_type.value),
            });
        }
        let secondary = records.get(cursor + 1);
        if secondary.map(|value| value.record_type.value) != Some(SECONDARY_TYPE) {
            return Err(SequenceValidationError::MalformedTrackPair {
                record_index: cursor,
                primary_offset: record.record_type.offset,
                secondary_type: secondary.map(|value| value.record_type.value),
            });
        }
        let containing = primary_range(record, PrimaryRole::Track, 0)?;
        let candidate_event_start = containing.start;
        track_pairs.push(TrackRecordPair {
            pair_ordinal: track_pairs.len(),
            primary: record.clone(),
            secondary: secondary.expect("validated secondary").clone(),
            candidate_event_start,
            event_containing_range: containing,
        });
        cursor += 2;
    }
    let terminal_record = records[cursor].clone();
    let track_descriptor_count = count - 2;
    let track_associations = if track_descriptor_count == track_pairs.len() {
        TrackAssociations::Ordinal(
            (0..track_descriptor_count)
                .map(|ordinal| TrackBinding {
                    descriptor_ordinal: ordinal + 2,
                    pair_ordinal: ordinal,
                })
                .collect(),
        )
    } else {
        TrackAssociations::Unresolved {
            descriptor_count: track_descriptor_count,
            pair_count: track_pairs.len(),
        }
    };

    Ok(SequenceContainer {
        sequence_range: sequence_start..terminal_record.record_range.end,
        candidate_record_index: candidate_index,
        preamble: located_bytes(bytes, sequence_start..preamble_end),
        descriptor_count,
        descriptors,
        sequence_name,
        name_record,
        prelude_records,
        meter_primary,
        meter_secondary,
        tempo_primary,
        tempo_secondary,
        initial_meter_range,
        initial_tempo_range,
        track_pairs,
        track_associations,
        terminal_record,
    })
}

fn required_record<'a>(
    records: &[FramedRecord<'a>],
    record_index: usize,
    role: RequiredRecordRole,
    expected: u8,
) -> Result<FramedRecord<'a>, SequenceValidationError> {
    let record = records
        .get(record_index)
        .ok_or(SequenceValidationError::MissingRequiredSequenceRecord { role, record_index })?;
    if record.record_type.value != expected {
        return Err(SequenceValidationError::WrongRequiredSequenceRecordType {
            role,
            offset: record.record_type.offset,
            observed: record.record_type.value,
            expected,
        });
    }
    Ok(record.clone())
}

fn primary_range(
    record: &FramedRecord<'_>,
    role: PrimaryRole,
    representation_length: usize,
) -> Result<Range<usize>, SequenceValidationError> {
    let start = record
        .payload
        .range
        .start
        .checked_add(PRIMARY_EVENT_OFFSET)
        .ok_or_else(|| SequenceValidationError::PrimaryPayloadTooShort {
            role,
            payload_range: record.payload.range.clone(),
            required: PRIMARY_EVENT_OFFSET + representation_length,
        })?;
    let end = start.checked_add(representation_length).ok_or_else(|| {
        SequenceValidationError::PrimaryPayloadTooShort {
            role,
            payload_range: record.payload.range.clone(),
            required: PRIMARY_EVENT_OFFSET + representation_length,
        }
    })?;
    if end > record.payload.range.end {
        return Err(SequenceValidationError::PrimaryPayloadTooShort {
            role,
            payload_range: record.payload.range.clone(),
            required: PRIMARY_EVENT_OFFSET + representation_length,
        });
    }
    let containing_end = if representation_length == 0 {
        record.payload.range.end
    } else {
        end
    };
    Ok(start..containing_end)
}

fn descriptor<'a>(bytes: &'a [u8], ordinal: usize, range: Range<usize>) -> SequenceDescriptor<'a> {
    let label_start = range.start + DESCRIPTOR_LABEL_OFFSET;
    let terminator_offset = bytes[label_start..range.end]
        .iter()
        .position(|value| *value == 0)
        .map(|relative| label_start + relative);
    let (label, label_terminator) = match terminator_offset {
        Some(offset) => (
            Some(located_bytes(bytes, label_start..offset)),
            Some(LocatedByte {
                value: bytes[offset],
                offset,
            }),
        ),
        None => (None, None),
    };
    SequenceDescriptor {
        ordinal,
        range: range.clone(),
        raw: located_bytes(bytes, range),
        label_start,
        label,
        label_terminator,
    }
}

fn checked_payload_end(payload_start: usize, payload_length: usize) -> Option<usize> {
    payload_start.checked_add(payload_length)
}

fn located_bytes(bytes: &[u8], range: Range<usize>) -> LocatedBytes<'_> {
    LocatedBytes {
        bytes: &bytes[range.clone()],
        range,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn push_record(bytes: &mut Vec<u8>, record_type: u8, payload: &[u8]) {
        bytes.push(record_type);
        bytes.extend_from_slice(&(payload.len() as u32).to_be_bytes());
        bytes.extend_from_slice(payload);
    }

    fn synthetic_project(track_descriptors: usize, track_pairs: usize, name: &[u8]) -> Vec<u8> {
        synthetic_project_with_track_payload(track_descriptors, track_pairs, name, &[0; 14])
    }

    fn synthetic_project_with_track_payload(
        track_descriptors: usize,
        track_pairs: usize,
        name: &[u8],
        track_payload: &[u8],
    ) -> Vec<u8> {
        let count = track_descriptors + 2;
        let sequence_start = ROOT_HEADER_LENGTH;
        let type_one_payload_length = count * DESCRIPTOR_STRIDE + 172;
        let mut bytes = vec![0xa5; ROOT_HEADER_LENGTH];
        let mut type_one_payload = vec![0; type_one_payload_length];
        type_one_payload[0] = count as u8;
        push_record(&mut bytes, SEQUENCE_TYPE, &type_one_payload);

        let descriptor_start = sequence_start + SEQUENCE_PREAMBLE_LENGTH;
        for ordinal in 0..count {
            let label_start = descriptor_start + ordinal * DESCRIPTOR_STRIDE + 15;
            if ordinal == count - 1 {
                bytes[label_start] = 0;
            } else {
                bytes[label_start..label_start + 2].copy_from_slice(&[0xa5, 0]);
            }
        }

        let mut name_payload = vec![0; 11];
        name_payload.push(name.len() as u8);
        name_payload.extend_from_slice(name);
        push_record(&mut bytes, NAME_RECORD_TYPE, &name_payload);
        push_record(&mut bytes, PRIMARY_TYPE, &[0; 22]);
        push_record(&mut bytes, SECONDARY_TYPE, &[]);
        push_record(&mut bytes, PRIMARY_TYPE, &[0; 21]);
        push_record(&mut bytes, SECONDARY_TYPE, &[]);
        for _ in 0..track_pairs {
            push_record(&mut bytes, PRIMARY_TYPE, track_payload);
            push_record(&mut bytes, SECONDARY_TYPE, &[]);
        }
        push_record(&mut bytes, TERMINAL_TYPE, &[]);
        bytes
    }

    #[test]
    fn generic_root_preserves_arbitrary_header_unknown_and_zero_length_records() {
        let mut bytes = (0_u8..8).collect::<Vec<_>>();
        push_record(&mut bytes, 0xfe, &[1, 2, 3]);
        push_record(&mut bytes, 0x05, &[]);
        let parsed = parse_root_record_stream(&bytes).unwrap();
        assert_eq!(parsed.root_header.raw.bytes, &(0_u8..8).collect::<Vec<_>>());
        assert_eq!(parsed.root_header.raw_words[0].value, 0x0001);
        assert_eq!(parsed.records.len(), 2);
        assert_eq!(parsed.records[0].record_type.value, 0xfe);
        assert_eq!(parsed.records[1].payload.range.start, bytes.len());
        assert_eq!(parsed.records[1].payload.range.end, bytes.len());
        assert_eq!(parsed.consumed_range, 0..bytes.len());
    }

    #[test]
    fn rejects_every_truncated_root_and_record_header_suffix() {
        for length in 0..8 {
            assert!(matches!(
                parse_root_record_stream(&vec![0; length]),
                Err(RootRecordError::TruncatedRootHeader { available, .. }) if available == length
            ));
        }
        assert_eq!(
            parse_root_record_stream(&[0; 8]).unwrap().consumed_range,
            0..8
        );
        for suffix in 1..5 {
            let mut bytes = vec![0; 8 + suffix];
            if suffix > 0 {
                bytes[8] = 0x33;
            }
            assert!(matches!(
                parse_root_record_stream(&bytes),
                Err(RootRecordError::TruncatedRecordHeader { offset: 8, available, .. }) if available == suffix
            ));
        }
    }

    #[test]
    fn rejects_declared_payload_beyond_input_and_checks_overflow_helper() {
        let mut bytes = vec![0; 8];
        bytes.extend_from_slice(&[0x44, 0, 0, 0, 2, 0xaa]);
        assert!(matches!(
            parse_root_record_stream(&bytes),
            Err(RootRecordError::PayloadBeyondInput {
                record_offset: 8,
                ..
            })
        ));
        assert_eq!(checked_payload_end(usize::MAX, 1), None);
    }

    #[test]
    fn parses_equal_and_mismatched_track_associations_and_non_utf8_name() {
        let equal_bytes = synthetic_project(2, 2, &[0xff]);
        let equal = parse_project_166(&equal_bytes).unwrap();
        assert_eq!(equal.sequences.len(), 1);
        assert_eq!(equal.sequences[0].sequence_name.as_utf8(), None);
        assert!(matches!(
            &equal.sequences[0].track_associations,
            TrackAssociations::Ordinal(bindings) if bindings.len() == 2
        ));
        assert_eq!(
            equal.sequences[0].track_pairs[0]
                .event_containing_range
                .len(),
            0
        );

        let mismatch_bytes = synthetic_project(11, 10, b"Sequence I like");
        let mismatch = parse_project_166(&mismatch_bytes).unwrap();
        assert_eq!(mismatch.sequences[0].track_descriptors().len(), 11);
        assert_eq!(mismatch.sequences[0].track_pairs.len(), 10);
        assert!(matches!(
            mismatch.sequences[0].track_associations,
            TrackAssociations::Unresolved {
                descriptor_count: 11,
                pair_count: 10
            }
        ));
    }

    #[test]
    fn rejects_malformed_candidate_without_scanning_for_later_bytes() {
        let mut malformed = synthetic_project(0, 0, b"bad");
        malformed[13] = 1;
        let later = synthetic_project(1, 1, b"valid later");
        malformed.extend_from_slice(&later[8..]);
        assert!(matches!(
            parse_project_166(&malformed),
            Err(Project166Error::MalformedSequenceCandidate {
                record_index: 0,
                cause: SequenceValidationError::DescriptorCountTooSmall { observed: 1, .. },
                ..
            })
        ));
    }

    #[test]
    fn derives_meter_tempo_and_track_containing_ranges() {
        let bytes = synthetic_project(1, 1, b"one");
        let project = parse_project_166(&bytes).unwrap();
        let sequence = &project.sequences[0];
        assert_eq!(sequence.initial_meter_range.len(), 8);
        assert_eq!(sequence.initial_tempo_range.len(), 7);
        let track = &sequence.track_pairs[0];
        assert_eq!(
            track.candidate_event_start,
            track.primary.payload.range.start + 14
        );
        assert_eq!(
            track.event_containing_range.end,
            track.primary.payload.range.end
        );
    }

    #[test]
    fn preserves_arbitrary_and_blank_descriptor_labels() {
        let bytes = synthetic_project(1, 1, b"labels");
        let project = parse_project_166(&bytes).unwrap();
        let descriptors = &project.sequences[0].descriptors;
        assert_eq!(descriptors[0].label.as_ref().unwrap().bytes, &[0xa5]);
        assert_eq!(
            descriptors.last().unwrap().label.as_ref().unwrap().bytes,
            &[] as &[u8]
        );
        assert_eq!(descriptors.last().unwrap().label_start, 563);
    }

    #[test]
    fn reports_descriptor_and_name_failures_at_the_candidate() {
        let mut too_few_bytes = synthetic_project(1, 1, b"small");
        too_few_bytes[13] = u8::MAX;
        assert!(matches!(
            parse_project_166(&too_few_bytes),
            Err(Project166Error::MalformedSequenceCandidate {
                cause: SequenceValidationError::DescriptorBeyondInput { .. },
                ..
            })
        ));

        let mut bad_name = synthetic_project(1, 1, b"name");
        let name_offset = ROOT_HEADER_LENGTH + SEQUENCE_PREAMBLE_LENGTH + 3 * DESCRIPTOR_STRIDE
            - NAME_OFFSET_BACKSTEP;
        bad_name[name_offset] += 1;
        assert!(matches!(
            parse_project_166(&bad_name),
            Err(Project166Error::MalformedSequenceCandidate {
                cause: SequenceValidationError::InvalidSequenceNameBounds { .. },
                ..
            })
        ));
    }

    #[test]
    fn reports_required_order_short_primary_pair_and_terminal_failures() {
        let mut wrong_order = synthetic_project(1, 1, b"order");
        let meter_type_offset = parse_root_record_stream(&wrong_order).unwrap().records[2]
            .record_type
            .offset;
        wrong_order[meter_type_offset] = 0x44;
        assert!(matches!(
            parse_project_166(&wrong_order),
            Err(Project166Error::MalformedSequenceCandidate {
                cause: SequenceValidationError::WrongRequiredSequenceRecordType {
                    role: RequiredRecordRole::MeterPrimary,
                    ..
                },
                ..
            })
        ));

        let mut short_primary = synthetic_project(1, 1, b"short");
        let (length_start, payload_start, payload_end) = {
            let root = parse_root_record_stream(&short_primary).unwrap();
            let meter = &root.records[2];
            (
                meter.length_bytes.range.start,
                meter.payload.range.start,
                meter.payload.range.end,
            )
        };
        short_primary[length_start..length_start + 4].copy_from_slice(&13_u32.to_be_bytes());
        short_primary.drain(payload_start + 13..payload_end);
        assert!(matches!(
            parse_project_166(&short_primary),
            Err(Project166Error::MalformedSequenceCandidate {
                cause: SequenceValidationError::PrimaryPayloadTooShort {
                    role: PrimaryRole::Meter,
                    ..
                },
                ..
            })
        ));

        let mut broken_pair = synthetic_project(1, 1, b"pair");
        let secondary_offset = parse_root_record_stream(&broken_pair).unwrap().records[7]
            .record_type
            .offset;
        broken_pair[secondary_offset] = 0x44;
        assert!(matches!(
            parse_project_166(&broken_pair),
            Err(Project166Error::MalformedSequenceCandidate {
                cause: SequenceValidationError::MalformedTrackPair { .. },
                ..
            })
        ));

        let mut bad_terminal = synthetic_project(0, 0, b"terminal");
        let terminal_offset = parse_root_record_stream(&bad_terminal)
            .unwrap()
            .records
            .last()
            .unwrap()
            .record_type
            .offset;
        bad_terminal[terminal_offset] = 0x44;
        assert!(matches!(
            parse_project_166(&bad_terminal),
            Err(Project166Error::MalformedSequenceCandidate {
                cause: SequenceValidationError::MalformedSequenceTerminal { .. },
                ..
            })
        ));
    }

    #[test]
    fn reports_missing_required_record_without_recovery() {
        let mut bytes = synthetic_project(0, 0, b"12345678901234");
        let name_end = parse_root_record_stream(&bytes).unwrap().records[1]
            .record_range
            .end;
        bytes.truncate(name_end);
        assert!(matches!(
            parse_project_166(&bytes),
            Err(Project166Error::MalformedSequenceCandidate {
                cause: SequenceValidationError::MissingRequiredSequenceRecord {
                    role: RequiredRecordRole::MeterPrimary,
                    ..
                },
                ..
            })
        ));
    }

    #[test]
    fn validates_nonempty_and_empty_descriptor166_event_bounds() {
        let mut payload = vec![0x11; PRIMARY_EVENT_OFFSET];
        payload.extend_from_slice(&[0xa1, 0xa2, 0xa3]);
        payload.extend_from_slice(&[0xff, 0x01, 0x02, 0x03, 0xff, 0x2f, 0x00]);
        let bytes = synthetic_project_with_track_payload(1, 1, b"bounds", &payload);
        let parsed = parse_project_166(&bytes).unwrap();
        let bounds = parsed.sequences[0].validated_track_event_bounds(0).unwrap();
        assert_eq!(bounds.event_range.end - bounds.event_range.start, 3);
        assert_eq!(bounds.event_range.end, bounds.tail_range.start);
        assert_eq!(bounds.tail_range.end, bounds.payload_range.end);
        assert!(
            bounds.event_range.start
                >= parsed.sequences[0].track_pairs[0]
                    .event_containing_range
                    .start
        );
        assert!(
            bounds.event_range.end
                <= parsed.sequences[0].track_pairs[0]
                    .event_containing_range
                    .end
        );

        let mut empty_payload = vec![0x11; PRIMARY_EVENT_OFFSET];
        empty_payload.extend_from_slice(&[0xff, 0xfe, 0xfd, 0xfc, 0xff, 0x2f, 0x00]);
        let empty_bytes = synthetic_project_with_track_payload(1, 1, b"empty", &empty_payload);
        let empty = parse_project_166(&empty_bytes).unwrap();
        let empty_bounds = empty.sequences[0].validated_track_event_bounds(0).unwrap();
        assert_eq!(empty_bounds.event_range.start, empty_bounds.event_range.end);
    }

    #[test]
    fn accepts_opaque_middle_tail_bytes_and_rejects_fixed_grammar_bytes() {
        for middle in [[0, 0, 0], [1, 2, 3], [0xff, 0x00, 0x80]] {
            let mut payload = vec![0x11; PRIMARY_EVENT_OFFSET];
            payload.extend_from_slice(&[0xaa, 0xbb]);
            payload.push(0xff);
            payload.extend_from_slice(&middle);
            payload.extend_from_slice(&[0xff, 0x2f, 0x00]);
            let bytes = synthetic_project_with_track_payload(1, 1, b"opaque", &payload);
            let parsed = parse_project_166(&bytes).unwrap();
            assert!(parsed.sequences[0].validated_track_event_bounds(0).is_ok());
        }

        for index in [0_usize, 4, 5, 6] {
            let mut payload = vec![0x11; PRIMARY_EVENT_OFFSET];
            payload.extend_from_slice(&[0xff, 0, 0, 0, 0xff, 0x2f, 0]);
            payload[PRIMARY_EVENT_OFFSET + index] ^= 1;
            let bytes = synthetic_project_with_track_payload(1, 1, b"invalid", &payload);
            let parsed = parse_project_166(&bytes).unwrap();
            assert!(matches!(
                parsed.sequences[0].validated_track_event_bounds(0),
                Err(TrackEventBoundsError::InvalidTerminalGrammar { .. })
            ));
        }
    }

    #[test]
    fn exact_bounds_do_not_search_for_an_earlier_terminal_shape() {
        let mut payload = vec![0x11; PRIMARY_EVENT_OFFSET];
        payload.extend_from_slice(&[0xff, 0xaa, 0xbb, 0xcc, 0xff, 0x2f, 0x00]);
        payload.extend_from_slice(&[0x21, 0x22]);
        payload.extend_from_slice(&[0xff, 0x01, 0x02, 0x03, 0xff, 0x2f, 0x00]);
        let bytes = synthetic_project_with_track_payload(1, 1, b"terminal", &payload);
        let parsed = parse_project_166(&bytes).unwrap();
        let bounds = parsed.sequences[0].validated_track_event_bounds(0).unwrap();
        assert_eq!(bounds.event_range.end, bounds.tail_range.start);
        assert_eq!(
            bounds.event_range.end - bounds.event_range.start,
            9,
            "the earlier terminal-shaped bytes remain event-region bytes"
        );
    }

    #[test]
    fn typed_bounds_errors_cover_pair_and_end_failures() {
        let bytes = synthetic_project(1, 1, b"errors");
        let parsed = parse_project_166(&bytes).unwrap();
        assert!(matches!(
            parsed.sequences[0].validated_track_event_bounds(1),
            Err(TrackEventBoundsError::PairNotFound {
                pair_ordinal: 1,
                pair_count: 1
            })
        ));
        assert!(matches!(
            parsed.sequences[0].validated_track_event_bounds(0),
            Err(TrackEventBoundsError::EventStartAfterEnd { .. })
        ));

        let mut short_pair = parsed.sequences[0].track_pairs[0].clone();
        let payload_start = short_pair.primary.payload.range.start;
        let short_end = payload_start + 6;
        short_pair.primary.payload = located_bytes(&bytes, payload_start..short_end);
        short_pair.event_containing_range = short_pair.candidate_event_start..short_end;
        assert!(matches!(
            short_pair.validated_event_bounds(),
            Err(TrackEventBoundsError::PayloadTooShort { required: 7, .. })
        ));
    }
}
