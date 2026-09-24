use phoenix::app_contract::{DiagnosticsLevel, InspectProjectRequest, Readiness, CONTRACT_VERSION};
use phoenix::app_service::AppService;
use phoenix::routing_evidence::{
    collect_routing_evidence, ProvisionalRelationshipStatus, RoutingEvidenceStatus,
};
use phoenix::sequence_container::parse_root_record_stream;
use std::fs;
use std::path::Path;

const BASELINE: &str = "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline";
const EXP33_CTRL: &str = "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 033 - Track 2 Instrument JV-1080-2 to JV-1080-3/Returned from MacOS9/EXP33 CTRL";
const EXP33_EDIT: &str = "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 033 - Track 2 Instrument JV-1080-2 to JV-1080-3/Returned from MacOS9/EXP33 EDIT";
const EXP34_CTRL: &str = "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 034 - Track 2 Instrument JV-1080-2 to Juno-106/Returned from MacOS9/EXP34 CTRL";
const EXP34_EDIT: &str = "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 034 - Track 2 Instrument JV-1080-2 to Juno-106/Returned from MacOS9/EXP34 EDIT";

fn evidence(path: &str) -> phoenix::routing_evidence::RoutingEvidence {
    let bytes = fs::read(path).expect("authenticated routing fixture");
    collect_routing_evidence(&bytes).expect("bounded routing evidence")
}

fn collect_bytes(bytes: &[u8]) -> phoenix::routing_evidence::RoutingEvidence {
    collect_routing_evidence(bytes).expect("bounded routing evidence")
}

fn replace_byte(bytes: &[u8], range: phoenix::compatibility::ByteRange, value: u8) -> Vec<u8> {
    let mut mutated = bytes.to_vec();
    let start = usize::try_from(range.start()).expect("fixture range fits usize");
    let end = usize::try_from(range.end_exclusive()).expect("fixture range fits usize");
    assert_eq!(end, start + 1);
    assert!(end <= mutated.len());
    mutated[start] = value;
    mutated
}

fn shorten_record(bytes: &[u8], record_type: u8, ordinal: usize, payload_len: usize) -> Vec<u8> {
    let root = parse_root_record_stream(bytes).expect("authenticated root framing");
    let record = root
        .records
        .iter()
        .filter(|record| record.record_type.value == record_type)
        .nth(ordinal)
        .expect("target record");
    assert_eq!(record.record_type.value, record_type);
    assert!(payload_len < record.payload.bytes.len());
    let mut rebuilt = bytes[..8].to_vec();
    for current in &root.records {
        if current.record_range == record.record_range {
            rebuilt.push(record_type);
            rebuilt.extend_from_slice(&(payload_len as u32).to_be_bytes());
            rebuilt.extend_from_slice(&current.payload.bytes[..payload_len]);
        } else {
            rebuilt.extend_from_slice(&bytes[current.record_range.clone()]);
        }
    }
    collect_bytes(&rebuilt);
    rebuilt
}

fn record_for_mutation(
    bytes: &[u8],
    record_type: u8,
    ordinal: usize,
    required_payload_len: usize,
) -> (usize, usize) {
    let root = parse_root_record_stream(bytes).expect("authenticated root framing");
    let record = root
        .records
        .iter()
        .filter(|record| record.record_type.value == record_type)
        .nth(ordinal)
        .expect("target record");
    assert_eq!(record.record_type.value, record_type);
    assert!(record.payload.bytes.len() >= required_payload_len);
    assert!(record.record_range.end <= bytes.len());
    assert!(record.payload.range.end <= bytes.len());
    (record.record_range.start, record.payload.range.start)
}

fn track_2_ode_value(
    path: &str,
) -> (
    u8,
    phoenix::routing_evidence::ProvisionalRoutingRelationship,
) {
    let collected = evidence(path);
    let assignment = collected
        .assignments
        .iter()
        .find(|assignment| {
            assignment.structural_ordinal == 14
                && assignment.label_bytes == b"Track 2"
                && assignment.candidate.is_some()
        })
        .expect("Ode Track 2 assignment candidate");
    let relationship = collected
        .relationships
        .iter()
        .find(|relationship| {
            relationship.assignment.structural_ordinal == 14
                && relationship.assignment.descriptor_ordinal == assignment.descriptor_ordinal
        })
        .expect("Ode Track 2 provisional relationship")
        .clone();
    (assignment.candidate.expect("candidate").0, relationship)
}

#[test]
fn controlled_assignment_candidates_and_bounded_records_are_reproduced() {
    for path in [BASELINE, EXP33_CTRL, EXP33_EDIT, EXP34_CTRL, EXP34_EDIT] {
        assert!(
            Path::new(path).is_file(),
            "missing authenticated fixture: {path}"
        );
        let collected = evidence(path);
        assert_eq!(collected.source_byte_size, 211_468);
        assert_eq!(collected.type10_records.len(), 85);
        assert_eq!(collected.type2a_records.len(), 12);
        assert_eq!(collected.status, RoutingEvidenceStatus::CompleteFraming);
        assert!(collected.issues.is_empty());
        assert!(collected
            .type10_records
            .iter()
            .all(|record| record.framed.payload.len() == 36));
        assert!(collected
            .type2a_records
            .iter()
            .all(|record| record.framed.payload.len() == 40));
    }

    let (baseline, _) = track_2_ode_value(BASELINE);
    let (control33, _) = track_2_ode_value(EXP33_CTRL);
    let (edit33, relationship33) = track_2_ode_value(EXP33_EDIT);
    let (control34, _) = track_2_ode_value(EXP34_CTRL);
    let (edit34, relationship34) = track_2_ode_value(EXP34_EDIT);
    assert_eq!(
        (baseline, control33, edit33, control34, edit34),
        (5, 5, 6, 5, 3)
    );

    for (relationship, expected_index, expected_device, expected_channel) in
        [(relationship33, 6, 0x0c, 2), (relationship34, 3, 0x0b, 0)]
    {
        assert_eq!(relationship.type10_record_index, Some(expected_index));
        assert_eq!(relationship.type2a_record_indices.len(), 1);
        assert_eq!(
            relationship.proposed_device_identifier,
            Some(expected_device)
        );
        assert_eq!(
            relationship.proposed_zero_based_channel,
            Some(expected_channel)
        );
        assert_eq!(
            relationship.status,
            ProvisionalRelationshipStatus::ProvisionalCorrelation
        );
        assert_eq!(relationship.assignment.label_bytes, b"Track 2");
        assert_eq!(
            relationship
                .assignment
                .candidate
                .expect("candidate")
                .1
                .source_range
                .start(),
            0x02f221
        );
        assert_eq!(
            relationship.provenance.source_range.start(),
            match expected_index {
                6 => 0x31a,
                3 => 0x29f,
                _ => unreachable!(),
            }
        );
    }
}

#[test]
fn repeated_candidates_remain_repeated_observations() {
    let collected = evidence(BASELINE);
    let values = collected
        .assignments
        .iter()
        .filter_map(|assignment| assignment.candidate.map(|candidate| candidate.0))
        .collect::<Vec<_>>();
    assert!(values.len() >= 9);
    assert!(values
        .iter()
        .enumerate()
        .any(|(index, value)| values[index + 1..].contains(value)));
    assert!(collected.relationships.iter().all(|relationship| {
        matches!(
            relationship.status,
            ProvisionalRelationshipStatus::ProvisionalCorrelation
                | ProvisionalRelationshipStatus::Unresolved(_)
                | ProvisionalRelationshipStatus::Ambiguous(_)
        )
    }));
}

#[test]
fn malformed_and_ambiguous_observations_fail_closed() {
    let bytes = fs::read(BASELINE).expect("baseline");

    let mut malformed = bytes.clone();
    let (record_start, _) = record_for_mutation(&malformed, 0x10, 5, 36);
    let payload_length = u32::from_be_bytes(
        malformed[record_start + 1..record_start + 5]
            .try_into()
            .unwrap(),
    );
    assert_eq!(payload_length, 36);
    malformed[record_start + 1..record_start + 5].copy_from_slice(&u32::MAX.to_be_bytes());
    assert!(collect_routing_evidence(&malformed).is_err());

    let mut ambiguous = bytes.clone();
    let (_, duplicate_payload_start) = record_for_mutation(&ambiguous, 0x10, 0, 26);
    ambiguous[duplicate_payload_start + 25] = 5;
    let collected = collect_routing_evidence(&ambiguous).expect("bounded ambiguity");
    assert_eq!(collected.status, RoutingEvidenceStatus::ContainsAmbiguity);
    assert!(collected.relationships.iter().any(|relationship| {
        matches!(
            relationship.status,
            ProvisionalRelationshipStatus::Ambiguous(_)
        )
    }));
}

#[test]
fn short_but_framed_routing_records_are_reported_without_guessing() {
    let bytes = fs::read(BASELINE).expect("baseline");
    let short_type10 = shorten_record(&bytes, 0x10, 5, 27);
    let type10 = collect_bytes(&short_type10);
    assert_eq!(
        type10.status,
        RoutingEvidenceStatus::ContainsMalformedObservations
    );
    assert!(type10.issues.iter().any(|issue| matches!(
        issue.kind,
        phoenix::routing_evidence::RoutingEvidenceIssueKind::Type10PayloadTooShort
    )));

    let short_type2a = shorten_record(&bytes, 0x2a, 0, 1);
    let type2a = collect_bytes(&short_type2a);
    assert_eq!(
        type2a.status,
        RoutingEvidenceStatus::ContainsMalformedObservations
    );
    assert!(type2a.issues.iter().any(|issue| matches!(
        issue.kind,
        phoenix::routing_evidence::RoutingEvidenceIssueKind::Type2aPayloadTooShort
    )));
}

#[test]
fn missing_matches_duplicate_identifiers_and_bad_channels_remain_unresolved() {
    let source = fs::read(BASELINE).expect("baseline");
    let baseline = collect_bytes(&source);
    let target = baseline
        .relationships
        .iter()
        .find(|relationship| {
            relationship.assignment.structural_ordinal == 14
                && relationship.assignment.label_bytes == b"Track 2"
        })
        .expect("Ode Track 2 relationship");
    let candidate_range = target
        .assignment
        .candidate
        .expect("candidate")
        .1
        .source_range;
    let missing_type10 = replace_byte(&source, candidate_range, 0xff);
    let missing_type10_evidence = collect_bytes(&missing_type10);
    assert!(missing_type10_evidence
        .relationships
        .iter()
        .any(|relationship| {
            relationship.assignment.structural_ordinal == 14
                && relationship.assignment.label_bytes == b"Track 2"
                && matches!(
                    relationship.status,
                    ProvisionalRelationshipStatus::Unresolved(_)
                )
        }));

    let selected_type10 = baseline
        .type10_records
        .iter()
        .find(|record| {
            record.framed.record_type == 0x10
                && record.ordinal_field.map(|field| field.0) == Some(5)
        })
        .expect("target type-0x10 record");
    let device_field = selected_type10
        .device_identifier_field
        .expect("device candidate")
        .1
        .source_range;
    let missing_type2a = replace_byte(&source, device_field, 0xfe);
    let missing_type2a_evidence = collect_bytes(&missing_type2a);
    assert!(missing_type2a_evidence
        .relationships
        .iter()
        .any(|relationship| {
            relationship.assignment.structural_ordinal == 14
                && relationship.assignment.label_bytes == b"Track 2"
                && matches!(
                    relationship.status,
                    ProvisionalRelationshipStatus::Unresolved(_)
                )
        }));

    let duplicate_source = baseline
        .type2a_records
        .iter()
        .find(|record| {
            record.framed.record_type == 0x2a
                && record.identifier_field.map(|field| field.0) != Some(0x0c)
        })
        .expect("second type-0x2a identifier");
    let duplicate_field = duplicate_source
        .identifier_field
        .expect("identifier candidate")
        .1
        .source_range;
    let duplicate_type2a = replace_byte(&source, duplicate_field, 0x0c);
    let duplicate_evidence = collect_bytes(&duplicate_type2a);
    assert!(duplicate_evidence.relationships.iter().any(|relationship| {
        relationship.assignment.structural_ordinal == 14
            && relationship.assignment.label_bytes == b"Track 2"
            && matches!(
                relationship.status,
                ProvisionalRelationshipStatus::Ambiguous(_)
            )
    }));

    let channel_field = selected_type10
        .channel_candidate_field
        .expect("channel candidate")
        .1
        .source_range;
    let out_of_range = replace_byte(&source, channel_field, 0xff);
    let out_of_range_evidence = collect_bytes(&out_of_range);
    let target_relationship = out_of_range_evidence
        .relationships
        .iter()
        .find(|relationship| {
            relationship.assignment.structural_ordinal == 14
                && relationship.assignment.label_bytes == b"Track 2"
        })
        .expect("Track 2 relationship");
    assert_eq!(target_relationship.proposed_zero_based_channel, None);
    assert!(matches!(
        target_relationship.status,
        ProvisionalRelationshipStatus::Unresolved(_)
    ));
}

#[test]
fn collector_reparses_each_supplied_source_and_cannot_mix_project_inputs() {
    let source = fs::read(BASELINE).expect("baseline");
    let baseline = collect_bytes(&source);
    let target = baseline
        .relationships
        .iter()
        .find(|relationship| {
            relationship.assignment.structural_ordinal == 14
                && relationship.assignment.label_bytes == b"Track 2"
        })
        .expect("Ode Track 2 relationship");
    let candidate_range = target
        .assignment
        .candidate
        .expect("candidate")
        .1
        .source_range;
    let changed = replace_byte(&source, candidate_range, 6);
    let reparsed = collect_bytes(&changed);
    let changed_target = reparsed
        .relationships
        .iter()
        .find(|relationship| {
            relationship.assignment.structural_ordinal == 14
                && relationship.assignment.label_bytes == b"Track 2"
        })
        .expect("reparsed Ode Track 2 relationship");
    assert_eq!(changed_target.assignment.candidate.expect("candidate").0, 6);
    assert_eq!(changed_target.type10_record_index, Some(6));
}

#[test]
fn app_service_exposes_evidence_without_changing_readiness_or_channels() {
    let mut service = AppService::new();
    let response = service
        .inspect_project(InspectProjectRequest {
            contract_version: CONTRACT_VERSION,
            source_path: BASELINE.into(),
            diagnostics_level: DiagnosticsLevel::Full,
        })
        .expect("baseline inspection");
    let before = response.sequences.clone();
    let routing = service
        .routing_evidence(&response.session_id)
        .expect("routing evidence");
    assert_eq!(routing.type10_records.len(), 85);
    assert_eq!(routing.type2a_records.len(), 12);
    assert_eq!(
        before
            .iter()
            .filter(|sequence| sequence.readiness == Readiness::Ready)
            .count(),
        5
    );
    assert_eq!(
        before
            .iter()
            .filter(|sequence| sequence.readiness != Readiness::Ready)
            .count(),
        13
    );
    let profile_evidence = service
        .profile_evidence(&response.session_id)
        .expect("profile evidence");
    assert!(profile_evidence
        .sequences
        .iter()
        .flat_map(|sequence| sequence.tracks.iter())
        .all(|track| track.observed_channel.is_none()));
    assert_eq!(
        service.routing_evidence(&response.session_id).unwrap(),
        routing
    );
}
