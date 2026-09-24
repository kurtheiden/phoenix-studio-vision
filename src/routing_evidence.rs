//! Provisional, read-only observations related to Studio Vision routing.
//!
//! This module records bounded bytes and relationships observed in the
//! authenticated Descriptor166 project shape.  It deliberately does not
//! decode an Instrument reference, populate an authoritative MIDI channel, or
//! affect compatibility matching and export policy.

use crate::compatibility::ByteRange;
use crate::sequence_container::{parse_project_166, parse_root_record_stream, SequenceDescriptor};
use std::fmt;

const TYPE_10: u8 = 0x10;
const TYPE_2A: u8 = 0x2a;
const ASSIGNMENT_BACKSTEP: usize = 31;
const TYPE_10_ORDINAL_OFFSET: usize = 25;
const TYPE_10_DEVICE_OFFSET: usize = 26;
const TYPE_10_CHANNEL_OFFSET: usize = 27;
const TYPE_2A_IDENTIFIER_OFFSET: usize = 33;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RoutingEvidenceIssueKind {
    AssignmentCandidateOutOfBounds,
    Type10PayloadTooShort,
    Type2aPayloadTooShort,
    Type2aNameOutOfBounds,
    UnresolvedAssignment,
    AmbiguousAssignment,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoutingEvidenceIssue {
    pub kind: RoutingEvidenceIssueKind,
    pub detail: String,
    pub source_range: Option<ByteRange>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoutingEvidenceStatus {
    CompleteFraming,
    ContainsUnresolved,
    ContainsAmbiguity,
    ContainsMalformedObservations,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObservationMethod {
    FramedRootRecord,
    NameMinus31Candidate,
    ProvisionalOrdinalLookup,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObservationProvenance {
    pub source_range: ByteRange,
    pub method: ObservationMethod,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FramedRoutingRecord {
    pub record_type: u8,
    pub record_range: ByteRange,
    pub payload_range: ByteRange,
    pub payload: Vec<u8>,
    pub provenance: ObservationProvenance,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Type10RecordObservation {
    pub framed: FramedRoutingRecord,
    pub ordinal_field: Option<(u8, ObservationProvenance)>,
    pub device_identifier_field: Option<(u8, ObservationProvenance)>,
    pub channel_candidate_field: Option<(u8, ObservationProvenance)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Type2aRecordObservation {
    pub framed: FramedRoutingRecord,
    pub name_bytes: Option<Vec<u8>>,
    pub name_provenance: Option<ObservationProvenance>,
    pub identifier_field: Option<(u8, ObservationProvenance)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackAssignmentCandidate {
    pub structural_ordinal: u32,
    pub descriptor_ordinal: u32,
    pub descriptor_range: ByteRange,
    pub label_range: ByteRange,
    pub label_bytes: Vec<u8>,
    pub candidate: Option<(u8, ObservationProvenance)>,
    pub provenance: ObservationProvenance,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProvisionalRelationshipStatus {
    ProvisionalCorrelation,
    Unresolved(String),
    Ambiguous(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProvisionalRoutingRelationship {
    pub assignment: TrackAssignmentCandidate,
    pub type10_record_index: Option<usize>,
    pub type2a_record_indices: Vec<usize>,
    pub proposed_device_identifier: Option<u8>,
    pub proposed_zero_based_channel: Option<u8>,
    pub status: ProvisionalRelationshipStatus,
    pub provenance: ObservationProvenance,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoutingEvidence {
    pub source_byte_size: u64,
    pub assignments: Vec<TrackAssignmentCandidate>,
    pub type10_records: Vec<Type10RecordObservation>,
    pub type2a_records: Vec<Type2aRecordObservation>,
    pub relationships: Vec<ProvisionalRoutingRelationship>,
    pub issues: Vec<RoutingEvidenceIssue>,
    pub status: RoutingEvidenceStatus,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RoutingEvidenceError {
    ProjectStructure(String),
    RootFraming(String),
}

impl fmt::Display for RoutingEvidenceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProjectStructure(detail) => {
                write!(
                    formatter,
                    "routing evidence project parsing failed: {detail}"
                )
            }
            Self::RootFraming(detail) => {
                write!(formatter, "routing evidence root framing failed: {detail}")
            }
        }
    }
}

impl std::error::Error for RoutingEvidenceError {}

/// Collects bounded routing observations without making them authoritative.
pub fn collect_routing_evidence(bytes: &[u8]) -> Result<RoutingEvidence, RoutingEvidenceError> {
    let project = parse_project_166(bytes)
        .map_err(|error| RoutingEvidenceError::ProjectStructure(format!("{error:?}")))?;
    let root = parse_root_record_stream(bytes)
        .map_err(|error| RoutingEvidenceError::RootFraming(error.to_string()))?;
    let mut issues = Vec::new();
    let type10_records = root
        .records
        .iter()
        .filter(|record| record.record_type.value == TYPE_10)
        .enumerate()
        .map(|(ordinal, record)| type10_observation(record, ordinal, &mut issues))
        .collect::<Vec<_>>();
    let type2a_records = root
        .records
        .iter()
        .filter(|record| record.record_type.value == TYPE_2A)
        .enumerate()
        .map(|(ordinal, record)| type2a_observation(record, ordinal, &mut issues))
        .collect::<Vec<_>>();

    let mut assignments = Vec::new();
    for (structural_ordinal, sequence) in project.sequences.iter().enumerate() {
        for descriptor in sequence.track_descriptors() {
            assignments.push(assignment_candidate(
                structural_ordinal,
                sequence.sequence_range.clone(),
                descriptor,
                bytes,
                bytes.len(),
                &mut issues,
            ));
        }
    }

    let relationships = assignments
        .iter()
        .map(|assignment| relationship(assignment, &type10_records, &type2a_records, &mut issues))
        .collect::<Vec<_>>();
    let status = if issues.iter().any(|issue| {
        matches!(
            issue.kind,
            RoutingEvidenceIssueKind::Type10PayloadTooShort
                | RoutingEvidenceIssueKind::Type2aPayloadTooShort
                | RoutingEvidenceIssueKind::Type2aNameOutOfBounds
                | RoutingEvidenceIssueKind::AssignmentCandidateOutOfBounds
        )
    }) {
        RoutingEvidenceStatus::ContainsMalformedObservations
    } else if relationships.iter().any(|relationship| {
        matches!(
            relationship.status,
            ProvisionalRelationshipStatus::Ambiguous(_)
        )
    }) {
        RoutingEvidenceStatus::ContainsAmbiguity
    } else if relationships.iter().any(|relationship| {
        matches!(
            relationship.status,
            ProvisionalRelationshipStatus::Unresolved(_)
        )
    }) {
        RoutingEvidenceStatus::ContainsUnresolved
    } else {
        RoutingEvidenceStatus::CompleteFraming
    };

    Ok(RoutingEvidence {
        source_byte_size: bytes.len() as u64,
        assignments,
        type10_records,
        type2a_records,
        relationships,
        issues,
        status,
    })
}

fn framed(record: &crate::sequence_container::FramedRecord<'_>) -> FramedRoutingRecord {
    let record_range = ByteRange::new(
        record.record_range.start as u64,
        record.record_range.end as u64,
    )
    .expect("parser supplied a valid record range");
    let payload_range = ByteRange::new(
        record.payload.range.start as u64,
        record.payload.range.end as u64,
    )
    .expect("parser supplied a valid payload range");
    FramedRoutingRecord {
        record_type: record.record_type.value,
        record_range,
        payload_range,
        payload: record.payload.bytes.to_vec(),
        provenance: ObservationProvenance {
            source_range: record_range,
            method: ObservationMethod::FramedRootRecord,
        },
    }
}

fn field(
    payload_start: usize,
    offset: usize,
    payload: &[u8],
    method: ObservationMethod,
) -> Option<(u8, ObservationProvenance)> {
    let absolute = payload_start.checked_add(offset)?;
    let range = ByteRange::new(absolute as u64, (absolute + 1) as u64).ok()?;
    Some((
        payload.get(offset).copied()?,
        ObservationProvenance {
            source_range: range,
            method,
        },
    ))
}

fn type10_observation(
    record: &crate::sequence_container::FramedRecord<'_>,
    ordinal: usize,
    issues: &mut Vec<RoutingEvidenceIssue>,
) -> Type10RecordObservation {
    let framed = framed(record);
    let payload = record.payload.bytes;
    if payload.len() <= TYPE_10_CHANNEL_OFFSET {
        issues.push(RoutingEvidenceIssue {
            kind: RoutingEvidenceIssueKind::Type10PayloadTooShort,
            detail: format!(
                "type-0x10 record {ordinal} has {} payload bytes",
                payload.len()
            ),
            source_range: Some(framed.record_range),
        });
    }
    Type10RecordObservation {
        framed,
        ordinal_field: field(
            record.payload.range.start,
            TYPE_10_ORDINAL_OFFSET,
            payload,
            ObservationMethod::FramedRootRecord,
        ),
        device_identifier_field: field(
            record.payload.range.start,
            TYPE_10_DEVICE_OFFSET,
            payload,
            ObservationMethod::FramedRootRecord,
        ),
        channel_candidate_field: field(
            record.payload.range.start,
            TYPE_10_CHANNEL_OFFSET,
            payload,
            ObservationMethod::FramedRootRecord,
        ),
    }
}

fn type2a_observation(
    record: &crate::sequence_container::FramedRecord<'_>,
    ordinal: usize,
    issues: &mut Vec<RoutingEvidenceIssue>,
) -> Type2aRecordObservation {
    let framed = framed(record);
    let payload = record.payload.bytes;
    let (name_bytes, name_provenance) = match payload.first().copied() {
        Some(length) => {
            let end = 1usize.saturating_add(length as usize);
            if end > payload.len() {
                issues.push(RoutingEvidenceIssue {
                    kind: RoutingEvidenceIssueKind::Type2aNameOutOfBounds,
                    detail: format!("type-0x2a record {ordinal} name exceeds its payload"),
                    source_range: Some(framed.record_range),
                });
                (None, None)
            } else {
                let range = ByteRange::new(
                    record.payload.range.start as u64 + 1,
                    record.payload.range.start as u64 + end as u64,
                )
                .expect("bounded type-0x2a name range");
                (
                    Some(payload[1..end].to_vec()),
                    Some(ObservationProvenance {
                        source_range: range,
                        method: ObservationMethod::FramedRootRecord,
                    }),
                )
            }
        }
        None => {
            issues.push(RoutingEvidenceIssue {
                kind: RoutingEvidenceIssueKind::Type2aPayloadTooShort,
                detail: format!("type-0x2a record {ordinal} has an empty payload"),
                source_range: Some(framed.record_range),
            });
            (None, None)
        }
    };
    if payload.len() <= TYPE_2A_IDENTIFIER_OFFSET {
        issues.push(RoutingEvidenceIssue {
            kind: RoutingEvidenceIssueKind::Type2aPayloadTooShort,
            detail: format!("type-0x2a record {ordinal} has no bounded identifier candidate"),
            source_range: Some(framed.record_range),
        });
    }
    Type2aRecordObservation {
        framed,
        name_bytes,
        name_provenance,
        identifier_field: field(
            record.payload.range.start,
            TYPE_2A_IDENTIFIER_OFFSET,
            payload,
            ObservationMethod::FramedRootRecord,
        ),
    }
}

fn assignment_candidate(
    structural_ordinal: usize,
    sequence_range: std::ops::Range<usize>,
    descriptor: &SequenceDescriptor<'_>,
    bytes: &[u8],
    source_len: usize,
    issues: &mut Vec<RoutingEvidenceIssue>,
) -> TrackAssignmentCandidate {
    let label_range = ByteRange::new(
        descriptor
            .label
            .as_ref()
            .map_or(0, |label| label.range.start) as u64,
        descriptor.label.as_ref().map_or(0, |label| label.range.end) as u64,
    )
    .expect("descriptor label range is ordered");
    let provenance = ObservationProvenance {
        source_range: label_range,
        method: ObservationMethod::NameMinus31Candidate,
    };
    let candidate = descriptor.label.as_ref().and_then(|label| {
        let Some(start) = label.range.start.checked_sub(ASSIGNMENT_BACKSTEP) else {
            issues.push(RoutingEvidenceIssue {
                kind: RoutingEvidenceIssueKind::AssignmentCandidateOutOfBounds,
                detail: format!(
                    "descriptor {} name-minus-31 candidate underflows the source",
                    descriptor.ordinal
                ),
                source_range: Some(label_range),
            });
            return None;
        };
        let end = start.checked_add(1)?;
        if start < sequence_range.start || end > sequence_range.end || end > source_len {
            issues.push(RoutingEvidenceIssue {
                kind: RoutingEvidenceIssueKind::AssignmentCandidateOutOfBounds,
                detail: format!(
                    "descriptor {} name-minus-31 candidate is outside its sequence",
                    descriptor.ordinal
                ),
                source_range: Some(label_range),
            });
            return None;
        }
        let range = ByteRange::new(start as u64, end as u64).expect("candidate range is ordered");
        Some((
            bytes[start],
            ObservationProvenance {
                source_range: range,
                method: ObservationMethod::NameMinus31Candidate,
            },
        ))
    });
    TrackAssignmentCandidate {
        structural_ordinal: structural_ordinal as u32,
        descriptor_ordinal: descriptor.ordinal as u32,
        descriptor_range: ByteRange::new(
            descriptor.range.start as u64,
            descriptor.range.end as u64,
        )
        .expect("descriptor range is ordered"),
        label_range,
        label_bytes: descriptor
            .label
            .as_ref()
            .map_or_else(Vec::new, |label| label.bytes.to_vec()),
        candidate,
        provenance,
    }
}

fn relationship(
    assignment: &TrackAssignmentCandidate,
    type10_records: &[Type10RecordObservation],
    type2a_records: &[Type2aRecordObservation],
    issues: &mut Vec<RoutingEvidenceIssue>,
) -> ProvisionalRoutingRelationship {
    let Some((value, candidate_provenance)) = assignment.candidate else {
        return ProvisionalRoutingRelationship {
            assignment: assignment.clone(),
            type10_record_index: None,
            type2a_record_indices: Vec::new(),
            proposed_device_identifier: None,
            proposed_zero_based_channel: None,
            status: ProvisionalRelationshipStatus::Unresolved(
                "assignment candidate is not bounded".into(),
            ),
            provenance: candidate_provenance_or_label(assignment),
        };
    };
    let matches = type10_records
        .iter()
        .enumerate()
        .filter(|(_, record)| record.ordinal_field.map(|field| field.0) == Some(value))
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    if matches.len() != 1 {
        let status = if matches.is_empty() {
            ProvisionalRelationshipStatus::Unresolved(
                "no unique type-0x10 ordinal candidate".into(),
            )
        } else {
            ProvisionalRelationshipStatus::Ambiguous("multiple type-0x10 ordinal candidates".into())
        };
        issues.push(RoutingEvidenceIssue {
            kind: if matches.is_empty() {
                RoutingEvidenceIssueKind::UnresolvedAssignment
            } else {
                RoutingEvidenceIssueKind::AmbiguousAssignment
            },
            detail: format!(
                "assignment value {value} has {} type-0x10 matches",
                matches.len()
            ),
            source_range: Some(candidate_provenance.source_range),
        });
        return ProvisionalRoutingRelationship {
            assignment: assignment.clone(),
            type10_record_index: matches.first().copied(),
            type2a_record_indices: Vec::new(),
            proposed_device_identifier: None,
            proposed_zero_based_channel: None,
            status,
            provenance: candidate_provenance,
        };
    }
    let type10_index = matches[0];
    let record = &type10_records[type10_index];
    let device = record.device_identifier_field.map(|field| field.0);
    let channel = record
        .channel_candidate_field
        .map(|field| field.0)
        .filter(|channel| *channel <= 15);
    let device_matches = device
        .map(|identifier| {
            type2a_records
                .iter()
                .enumerate()
                .filter(|(_, record)| {
                    record.identifier_field.map(|field| field.0) == Some(identifier)
                })
                .map(|(index, _)| index)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let status = if device_matches.len() > 1 {
        ProvisionalRelationshipStatus::Ambiguous("multiple type-0x2a identifier candidates".into())
    } else if device_matches.is_empty() {
        ProvisionalRelationshipStatus::Unresolved("no type-0x2a identifier candidate".into())
    } else if device.is_none() || channel.is_none() {
        ProvisionalRelationshipStatus::Unresolved(
            "type-0x10 fields are incomplete or channel candidate is out of range".into(),
        )
    } else {
        ProvisionalRelationshipStatus::ProvisionalCorrelation
    };
    if matches!(
        status,
        ProvisionalRelationshipStatus::Ambiguous(_) | ProvisionalRelationshipStatus::Unresolved(_)
    ) {
        issues.push(RoutingEvidenceIssue {
            kind: if matches!(status, ProvisionalRelationshipStatus::Ambiguous(_)) {
                RoutingEvidenceIssueKind::AmbiguousAssignment
            } else {
                RoutingEvidenceIssueKind::UnresolvedAssignment
            },
            detail: "provisional relationship is not uniquely resolved".into(),
            source_range: Some(candidate_provenance.source_range),
        });
    }
    ProvisionalRoutingRelationship {
        assignment: assignment.clone(),
        type10_record_index: Some(type10_index),
        type2a_record_indices: device_matches,
        proposed_device_identifier: device,
        proposed_zero_based_channel: channel,
        status,
        provenance: ObservationProvenance {
            source_range: type10_records[type10_index].framed.record_range,
            method: ObservationMethod::ProvisionalOrdinalLookup,
        },
    }
}

fn candidate_provenance_or_label(assignment: &TrackAssignmentCandidate) -> ObservationProvenance {
    assignment.provenance
}
