use super::*;
use crate::sequence_container::parse_project_166;

fn record(bytes: &mut Vec<u8>, kind: u8, payload: &[u8]) {
    bytes.push(kind);
    bytes.extend((payload.len() as u32).to_be_bytes());
    bytes.extend(payload);
}

pub(crate) fn synthetic(count: u8, name: &[u8]) -> Vec<u8> {
    assert!(name.len() < 18);
    let mut bytes = vec![0; 8];
    let mut payload = vec![0; 75 + 120 * usize::from(count) - 5];
    payload[0] = count;
    payload[16] = 1; // candidate +21
    payload[17] = 0x88;
    payload[18..18 + name.len()].copy_from_slice(name);
    payload[36..38].copy_from_slice(&[0xfe, 0xff]);
    record(&mut bytes, 1, &payload);
    record(&mut bytes, 7, &[31, 42, 53]); // opaque following content is not constrained
    bytes
}

#[test]
fn positive_geometry_accepts_defensive_minimum_and_unseen_counts() {
    for count in [2, 3, 25, 26, 255] {
        let bytes = synthetic(count, b"Unrelated");
        let observed = observe_project_120(&bytes).unwrap();
        assert_eq!(observed.root_header.range, 0..8);
        assert_eq!(observed.consumed_range, 0..bytes.len());
        let candidate = &observed.candidates[0];
        assert_eq!(candidate.count.value, count);
        assert_eq!(
            candidate.candidate_range.len(),
            75 + 120 * usize::from(count)
        );
        assert_eq!(candidate.observed_name.range, 31..40);
        assert_eq!(candidate.name_as_utf8(), Some("Unrelated"));
        assert_eq!(candidate.record_index, 0);
    }
}

#[test]
fn count_zero_and_one_are_defensive_refusals_not_track_semantics() {
    for count in [0, 1] {
        assert!(observe_project_120(&synthetic(count, b"Name")).is_err());
    }
}

#[test]
fn each_local_family_guard_is_required() {
    let original = synthetic(3, b"Name");
    for relative in (15..22).chain([22, 41, 42]) {
        let mut bytes = original.clone();
        bytes[8 + relative] ^= 1;
        assert!(observe_project_120(&bytes).is_err(), "guard +{relative}");
    }
    let mut alternate = original;
    alternate[8 + 22] = 0x80;
    assert!(observe_project_120(&alternate).is_ok());
}

#[test]
fn empty_observed_name_is_retained_without_interpretation() {
    let bytes = synthetic(3, b"");
    let result = observe_project_120(&bytes).unwrap();
    assert_eq!(result.candidates.len(), 1);
    assert_eq!(result.candidates[0].observed_name.range, 31..31);
    assert_eq!(result.candidates[0].terminator_offset, 31);
}

#[test]
fn latest_observed_and_latest_authorized_terminators_are_bounded() {
    for length in [16, 17] {
        let bytes = synthetic(3, &vec![b'X'; length]);
        let result = observe_project_120(&bytes).unwrap();
        assert_eq!(result.candidates[0].terminator_offset, 8 + 23 + length);
    }
}

#[test]
fn no_terminator_in_window_refuses_even_if_zero_exists_later() {
    let mut bytes = synthetic(3, b"Name");
    bytes[8 + 23..8 + 41].fill(b'X');
    bytes[8 + 43] = 0;
    assert!(observe_project_120(&bytes).is_err());
}

#[test]
fn opaque_nonzero_suffix_and_raw_non_utf8_are_preserved() {
    let mut bytes = synthetic(3, &[0xff, 0x80]);
    bytes[8 + 26..8 + 41].fill(0xaf);
    let result = observe_project_120(&bytes).unwrap();
    let candidate = &result.candidates[0];
    assert_eq!(candidate.name_as_utf8(), None);
    assert_eq!(candidate.observed_name.bytes, [0xff, 0x80]);
    assert_eq!(candidate.after_terminator.bytes, [0xaf; 15]);
}

#[test]
fn count_size_and_record_header_contradictions_refuse() {
    for offset in [8 + 5, 8 + 4] {
        let mut bytes = synthetic(3, b"Name");
        bytes[offset] += 1;
        assert!(observe_project_120(&bytes).is_err());
    }
}

#[test]
fn truncation_and_huge_record_lengths_refuse() {
    let original = synthetic(3, b"Name");
    for end in [0, 7, 9, 12, 40, original.len() - 1] {
        assert!(observe_project_120(&original[..end]).is_err());
    }
    let mut bytes = original;
    bytes[9..13].copy_from_slice(&u32::MAX.to_be_bytes());
    assert!(matches!(
        observe_project_120(&bytes),
        Err(ObservationError::Root(_))
    ));
}

#[test]
fn following_record_must_exist_and_be_immediately_type07() {
    let mut bytes = synthetic(3, b"Name");
    let end = 8 + 75 + 120 * 3;
    bytes[end] = 0x09;
    assert!(observe_project_120(&bytes).is_err());
    bytes.truncate(end);
    assert!(observe_project_120(&bytes).is_err());
}

#[test]
fn no_candidates_and_changed_candidate_type_are_not_positive_matches() {
    assert_eq!(observe_project_120(&[0; 8]), Err(ObservationError::NoMatch));
    let mut bytes = synthetic(3, b"Name");
    bytes[8] = 0x10;
    assert_eq!(observe_project_120(&bytes), Err(ObservationError::NoMatch));
}

#[test]
fn complete_root_framing_and_nonpadding_local_structure_are_required() {
    let mut bytes = synthetic(3, b"Name");
    bytes.extend([0x99, 0, 0]);
    assert!(matches!(
        observe_project_120(&bytes),
        Err(ObservationError::Root(_))
    ));
    let mut size_only = synthetic(3, b"Name");
    size_only[8 + 21] = 0;
    assert!(parse_project_166(&size_only).is_err());
    assert!(observe_project_120(&size_only).is_err());
}

#[test]
fn duplicate_and_empty_observations_preserve_distinct_record_ranges() {
    let mut bytes = synthetic(3, b"Same");
    bytes.extend(&synthetic(4, b"Same")[8..]);
    bytes.extend(&synthetic(2, b"")[8..]);
    let observed = observe_project_120(&bytes).unwrap();
    assert_eq!(observed.candidates.len(), 3);
    assert_ne!(
        observed.candidates[0].candidate_range,
        observed.candidates[1].candidate_range
    );
    assert!(observed.candidates[2].observed_name.bytes.is_empty());
}

#[test]
fn malformed_sibling_invalidates_complete_observation() {
    let mut bytes = synthetic(3, b"Valid");
    let mut invalid = synthetic(4, b"Invalid");
    invalid[8 + 21] = 0;
    bytes.extend(&invalid[8..]);
    assert!(matches!(
        observe_project_120(&bytes),
        Err(ObservationError::MalformedCandidate { record_index: 2 })
    ));
}

#[test]
fn checked_local_ranges_reject_overflow_and_out_of_candidate_bounds() {
    assert_eq!(local_range(&(usize::MAX - 5..usize::MAX), 23..41), None);
    assert_eq!(local_range(&(8..40), 23..41), None);
    assert_eq!(local_range(&(8..100), Range { start: 41, end: 23 }), None);
    let bytes = synthetic(3, b"Name");
    let root = parse_root_record_stream(&bytes).unwrap();
    let mut invalid = root.records[0].clone();
    invalid.record_range = usize::MAX - 5..usize::MAX;
    assert!(observe_candidate(&bytes, &invalid, root.records.get(1), 0).is_none());
}

#[test]
fn discriminator_truth_table_never_prioritizes_an_ambiguous_layout() {
    assert_eq!(
        discriminate(true, true),
        Err(ObservationError::AmbiguousLayout)
    );
    assert_eq!(discriminate(false, false), Err(ObservationError::NoMatch));
    assert_eq!(discriminate(true, false), Ok(CandidateLayout::Observed120));
    assert_eq!(
        discriminate(false, true),
        Ok(CandidateLayout::Established166)
    );
}

// A genuine dual-valid synthetic context: the following07 is large enough to
// contain the existing166-derived Pascal name; all conductor/terminal guards
// pass. This demonstrates why geometry alone cannot prove disjointness.
pub(crate) fn ambiguous() -> Vec<u8> {
    let mut bytes = synthetic(2, b"Raw");
    bytes.truncate(8 + 75 + 120 * 2);
    let length_offset = 8 + 208 + 166 * 2 - 15;
    let following_start = bytes.len();
    let mut payload = vec![0; length_offset - following_start - 5];
    payload.extend([1, b'Z']);
    record(&mut bytes, 7, &payload);
    record(&mut bytes, 2, &[0; 22]);
    record(&mut bytes, 0x29, &[]);
    record(&mut bytes, 2, &[0; 21]);
    record(&mut bytes, 0x29, &[]);
    record(&mut bytes, 0, &[]);
    bytes
}

#[test]
fn genuinely_dual_valid_candidate_is_refused() {
    let bytes = ambiguous();
    assert!(parse_project_166(&bytes).is_ok());
    assert_eq!(
        observe_project_120(&bytes),
        Err(ObservationError::AmbiguousLayout)
    );
}

#[test]
fn unchanged166_and_mixed_layouts_are_independently_classified() {
    let semantic = crate::bounded_sequence::tests::synthetic(crate::bounded_sequence::tests::NOTE);
    assert_eq!(parse_project_166(&semantic).unwrap().sequences.len(), 1);
    assert_eq!(
        observe_project_120(&semantic),
        Err(ObservationError::NoMatch)
    );
    for reverse in [false, true] {
        let raw = synthetic(3, b"Raw");
        let mut mixed = if reverse {
            semantic.clone()
        } else {
            raw.clone()
        };
        mixed.extend(if reverse { &raw[8..] } else { &semantic[8..] });
        assert_eq!(
            observe_project_120(&mixed),
            Err(ObservationError::MixedLayouts)
        );
        assert!(parse_project_166(&mixed).is_err());
    }
}

#[test]
fn root_identity_and_literal_names_do_not_select_the_layout() {
    let mut bytes = synthetic(3, b"Other archive");
    bytes[..8].copy_from_slice(b"Whatever");
    assert!(observe_project_120(&bytes).is_ok());
}

#[test]
#[ignore = "requires explicitly authorized private SCHOOL PROJECTS evidence outside Git"]
fn authorized_school_evidence_remains_raw_and_not_exportable() {
    use crate::app_contract::{
        DiagnosticsLevel, InspectProjectRequest, Readiness, CONTRACT_VERSION,
    };
    use crate::app_service::AppService;
    use sha2::{Digest, Sha256};
    let path =
        std::env::var("PHOENIX_SCHOOL_OBSERVATION_SOURCE").expect("authorized evidence path");
    let bytes = std::fs::read(&path).unwrap();
    assert_eq!(bytes.len(), 343875);
    assert_eq!(
        format!("{:x}", Sha256::digest(&bytes)),
        "bfd4fa1208e2cd884ec51ccfb1131d5c02723d7c90acafc98bd89597e1a20331"
    );
    let observed = observe_project_120(&bytes).unwrap();
    assert_eq!(observed.candidates.len(), 15);
    assert_eq!(
        observed
            .candidates
            .iter()
            .filter(|c| c.observed_name.bytes.is_empty())
            .count(),
        3
    );
    let anchor = observed
        .candidates
        .iter()
        .find(|c| c.candidate_range.start == 263588)
        .unwrap();
    assert_eq!(anchor.observed_name.bytes, b"kickin'dance");
    assert_eq!(anchor.observed_name.range, 263611..263623);
    for candidate in &observed.candidates {
        assert_eq!(
            candidate.candidate_range.len(),
            75 + 120 * usize::from(candidate.count.value)
        );
        assert!(candidate.terminator_offset < candidate.candidate_range.start + 41);
    }
    let mut service = AppService::new();
    let response = service
        .inspect_project(InspectProjectRequest {
            contract_version: CONTRACT_VERSION,
            source_path: path,
            diagnostics_level: DiagnosticsLevel::Full,
        })
        .unwrap();
    assert!(response.sequences.is_empty());
    assert_eq!(response.project.overall_readiness, Readiness::Unknown);
    assert_eq!(response.project.profile_label, None);
    assert!(service.profile_evidence(&response.session_id).is_err());
    let diagnostics = service
        .get_diagnostics(&response.session_id, DiagnosticsLevel::Full)
        .unwrap();
    assert_eq!(diagnostics.recognized_profile, None);
    assert_eq!(diagnostics.compatibility_profile, None);
    let status = diagnostics.structural_status.unwrap();
    assert!(status.contains("InvalidSequenceNameBounds"));
    assert!(status.contains("15 120-layout structural candidates, including 3 empty"));
    println!("{}", observed.diagnostic_summary());
    println!(
        "Inspection: {} semantic sequences, {:?}; {}",
        response.sequences.len(),
        response.project.overall_readiness,
        status
    );
}
