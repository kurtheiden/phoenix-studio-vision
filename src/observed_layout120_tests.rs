use super::*;
use crate::sequence_container::parse_project_166;

fn record(bytes: &mut Vec<u8>, kind: u8, payload: &[u8]) {
    bytes.push(kind);
    bytes.extend((payload.len() as u32).to_be_bytes());
    bytes.extend(payload);
}

pub(crate) fn synthetic(count: u8, name: &[u8]) -> Vec<u8> {
    synthetic_marker(count, name, [0xfe, 0xff])
}

pub(crate) fn synthetic_marker(count: u8, name: &[u8], marker: [u8; 2]) -> Vec<u8> {
    assert!(name.len() < 18);
    let mut bytes = vec![0; 8];
    let mut payload = vec![0; 75 + 120 * usize::from(count) - 5];
    payload[0] = count;
    payload[16] = 1; // candidate +21
    payload[17] = 0x88;
    payload[18..18 + name.len()].copy_from_slice(name);
    payload[36..38].copy_from_slice(&marker);
    record(&mut bytes, 1, &payload);
    record(&mut bytes, 7, &[31, 42, 53]); // opaque following content is not constrained
    bytes
}

#[test]
fn positive_geometry_accepts_defensive_minimum_and_unseen_counts() {
    for marker in [[0xfe, 0xff], [0xff, 0xff]] {
        for count in [2, 3, 25, 26, 255] {
            let bytes = synthetic_marker(count, b"Unrelated", marker);
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
}

#[test]
fn count_zero_and_one_are_defensive_refusals_not_track_semantics() {
    for marker in [[0xfe, 0xff], [0xff, 0xff]] {
        for count in [0, 1] {
            assert!(observe_project_120(&synthetic_marker(count, b"Name", marker)).is_err());
        }
    }
}

#[test]
fn each_local_family_guard_is_required() {
    for marker in [[0xfe, 0xff], [0xff, 0xff]] {
        let original = synthetic_marker(3, b"Name", marker);
        for relative in (15..22).chain([22, 41, 42]) {
            let mut bytes = original.clone();
            if relative == 41 {
                bytes[8 + relative] = 0xfd;
            } else {
                bytes[8 + relative] ^= 1;
            }
            assert!(observe_project_120(&bytes).is_err(), "guard +{relative}");
        }
        let mut alternate = original;
        alternate[8 + 22] = 0x80;
        assert!(observe_project_120(&alternate).is_ok());
    }
}

#[test]
fn empty_observed_name_is_retained_without_interpretation() {
    for marker in [[0xfe, 0xff], [0xff, 0xff]] {
        let bytes = synthetic_marker(3, b"", marker);
        let result = observe_project_120(&bytes).unwrap();
        assert_eq!(result.candidates.len(), 1);
        assert_eq!(result.candidates[0].observed_name.range, 31..31);
        assert_eq!(result.candidates[0].terminator_offset, 31);
    }
}

#[test]
fn latest_observed_and_latest_authorized_terminators_are_bounded() {
    for marker in [[0xfe, 0xff], [0xff, 0xff]] {
        for length in [16, 17] {
            let bytes = synthetic_marker(3, &vec![b'X'; length], marker);
            let result = observe_project_120(&bytes).unwrap();
            assert_eq!(result.candidates[0].terminator_offset, 8 + 23 + length);
        }
    }
}

#[test]
fn no_terminator_in_window_refuses_even_if_zero_exists_later() {
    for marker in [[0xfe, 0xff], [0xff, 0xff]] {
        let mut bytes = synthetic_marker(3, b"Name", marker);
        bytes[8 + 23..8 + 41].fill(b'X');
        bytes[8 + 43] = 0;
        assert!(observe_project_120(&bytes).is_err());
    }
}

#[test]
fn opaque_nonzero_suffix_and_raw_non_utf8_are_preserved() {
    for marker in [[0xfe, 0xff], [0xff, 0xff]] {
        let mut bytes = synthetic_marker(3, &[0xff, 0x80], marker);
        bytes[8 + 26..8 + 41].fill(0xaf);
        let result = observe_project_120(&bytes).unwrap();
        let candidate = &result.candidates[0];
        assert_eq!(candidate.name_as_utf8(), None);
        assert_eq!(candidate.observed_name.bytes, [0xff, 0x80]);
        assert_eq!(candidate.after_terminator.bytes, [0xaf; 15]);
    }
}

#[test]
fn count_size_and_record_header_contradictions_refuse() {
    for marker in [[0xfe, 0xff], [0xff, 0xff]] {
        for offset in [8 + 5, 8 + 4] {
            let mut bytes = synthetic_marker(3, b"Name", marker);
            bytes[offset] += 1;
            assert!(observe_project_120(&bytes).is_err());
        }
    }
}

#[test]
fn truncation_and_huge_record_lengths_refuse() {
    for marker in [[0xfe, 0xff], [0xff, 0xff]] {
        let original = synthetic_marker(3, b"Name", marker);
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
}

#[test]
fn following_record_must_exist_and_be_immediately_type07() {
    for marker in [[0xfe, 0xff], [0xff, 0xff]] {
        let mut bytes = synthetic_marker(3, b"Name", marker);
        let end = 8 + 75 + 120 * 3;
        bytes[end] = 0x09;
        assert!(observe_project_120(&bytes).is_err());
        bytes.truncate(end);
        assert!(observe_project_120(&bytes).is_err());
    }
}

#[test]
fn no_candidates_and_changed_candidate_type_are_not_positive_matches() {
    for marker in [[0xfe, 0xff], [0xff, 0xff]] {
        assert_eq!(observe_project_120(&[0; 8]), Err(ObservationError::NoMatch));
        let mut bytes = synthetic_marker(3, b"Name", marker);
        bytes[8] = 0x10;
        assert_eq!(observe_project_120(&bytes), Err(ObservationError::NoMatch));
    }
}

#[test]
fn complete_root_framing_and_nonpadding_local_structure_are_required() {
    for marker in [[0xfe, 0xff], [0xff, 0xff]] {
        let mut bytes = synthetic_marker(3, b"Name", marker);
        bytes.extend([0x99, 0, 0]);
        assert!(matches!(
            observe_project_120(&bytes),
            Err(ObservationError::Root(_))
        ));
        let mut size_only = synthetic_marker(3, b"Name", marker);
        size_only[8 + 21] = 0;
        assert!(parse_project_166(&size_only).is_err());
        assert!(observe_project_120(&size_only).is_err());
    }
}

#[test]
fn duplicate_and_empty_observations_preserve_distinct_record_ranges() {
    for marker in [[0xfe, 0xff], [0xff, 0xff]] {
        let mut bytes = synthetic_marker(3, b"Same", marker);
        bytes.extend(&synthetic_marker(4, b"Same", marker)[8..]);
        bytes.extend(&synthetic_marker(2, b"", marker)[8..]);
        let observed = observe_project_120(&bytes).unwrap();
        assert_eq!(observed.candidates.len(), 3);
        assert_ne!(
            observed.candidates[0].candidate_range,
            observed.candidates[1].candidate_range
        );
        assert!(observed.candidates[2].observed_name.bytes.is_empty());
    }
}

#[test]
fn malformed_sibling_invalidates_complete_observation() {
    for marker in [[0xfe, 0xff], [0xff, 0xff]] {
        let mut bytes = synthetic_marker(3, b"Valid", marker);
        let mut invalid = synthetic_marker(4, b"Invalid", marker);
        invalid[8 + 21] = 0;
        bytes.extend(&invalid[8..]);
        assert!(matches!(
            observe_project_120(&bytes),
            Err(ObservationError::MalformedCandidate { record_index: 2 })
        ));
    }
}

#[test]
fn checked_local_ranges_reject_overflow_and_out_of_candidate_bounds() {
    for marker in [[0xfe, 0xff], [0xff, 0xff]] {
        assert_eq!(local_range(&(usize::MAX - 5..usize::MAX), 23..41), None);
        assert_eq!(local_range(&(8..40), 23..41), None);
        assert_eq!(local_range(&(8..100), Range { start: 41, end: 23 }), None);
        let bytes = synthetic_marker(3, b"Name", marker);
        let root = parse_root_record_stream(&bytes).unwrap();
        let mut invalid = root.records[0].clone();
        invalid.record_range = usize::MAX - 5..usize::MAX;
        assert!(observe_candidate(&bytes, &invalid, root.records.get(1), 0).is_none());
    }
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
    ambiguous_marker([0xfe, 0xff])
}

pub(crate) fn ambiguous_marker(marker: [u8; 2]) -> Vec<u8> {
    let mut bytes = synthetic_marker(2, b"Raw", marker);
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
    for marker in [[0xfe, 0xff], [0xff, 0xff]] {
        let bytes = ambiguous_marker(marker);
        assert!(parse_project_166(&bytes).is_ok());
        assert_eq!(
            observe_project_120(&bytes),
            Err(ObservationError::AmbiguousLayout)
        );
    }
}

#[test]
fn unchanged166_and_mixed_layouts_are_independently_classified() {
    for marker in [[0xfe, 0xff], [0xff, 0xff]] {
        let semantic =
            crate::bounded_sequence::tests::synthetic(crate::bounded_sequence::tests::NOTE);
        assert_eq!(parse_project_166(&semantic).unwrap().sequences.len(), 1);
        assert_eq!(
            observe_project_120(&semantic),
            Err(ObservationError::NoMatch)
        );
        for reverse in [false, true] {
            let raw = synthetic_marker(3, b"Raw", marker);
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
}

#[test]
fn root_identity_and_literal_names_do_not_select_the_layout() {
    for marker in [[0xfe, 0xff], [0xff, 0xff]] {
        let mut bytes = synthetic_marker(3, b"Other archive", marker);
        bytes[..8].copy_from_slice(b"Whatever");
        assert!(observe_project_120(&bytes).is_ok());
    }
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
    assert!(observed
        .candidates
        .iter()
        .all(|c| c.marker_form == Observed120MarkerForm::FeFf
            && c.observed_marker.bytes == [0xfe, 0xff]
            && c.observed_marker.range
                == (c.candidate_range.start + 41..c.candidate_range.start + 43)));
    assert!(observed
        .diagnostic_summary()
        .contains("Observed marker forms: fe ff 15, ff ff 0."));
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

#[test]
fn exact_marker_forms_preserve_raw_provenance_and_name_behavior() {
    for (marker, form) in [
        ([0xfe, 0xff], Observed120MarkerForm::FeFf),
        ([0xff, 0xff], Observed120MarkerForm::FfFf),
    ] {
        let bytes = synthetic_marker(4, b"Different", marker);
        let project = observe_project_120(&bytes).unwrap();
        let candidate = &project.candidates[0];
        assert_eq!(candidate.marker_form, form);
        assert_eq!(candidate.observed_marker.bytes, marker);
        assert_eq!(candidate.observed_marker.range, 49..51);
        assert_eq!(candidate.observed_name.bytes, b"Different");
        assert_eq!(candidate.observed_name.range, 31..40);
        assert_eq!(candidate.terminator_offset, 40);
    }
}

#[test]
fn unsupported_nearby_markers_and_wrong_lengths_refuse() {
    for marker in [
        [0xfd, 0xff],
        [0xfe, 0xfe],
        [0xff, 0xfe],
        [0xff, 0],
        [0, 0xff],
    ] {
        assert_eq!(Observed120MarkerForm::from_bytes(&marker), None);
        assert!(observe_project_120(&synthetic_marker(3, b"Unrelated", marker)).is_err());
        let mut sibling = synthetic(3, b"Valid");
        sibling.extend(&synthetic_marker(3, b"Invalid", marker)[8..]);
        assert!(observe_project_120(&sibling).is_err());
    }
    for invalid in [&[][..], &[0xfe][..], &[0xfe, 0xff, 0][..]] {
        assert_eq!(Observed120MarkerForm::from_bytes(invalid), None);
    }
}

#[test]
fn supported_marker_forms_coexist_without_layout_mixing() {
    for markers in [
        vec![[0xfe, 0xff], [0xff, 0xff]],
        vec![[0xff, 0xff], [0xfe, 0xff]],
        vec![[0xfe, 0xff], [0xff, 0xff], [0xfe, 0xff]],
        vec![[0xff, 0xff], [0xff, 0xff]],
    ] {
        let mut bytes = vec![0; 8];
        for marker in &markers {
            bytes.extend(&synthetic_marker(3, b"", *marker)[8..]);
        }
        let project = observe_project_120(&bytes).unwrap();
        assert_eq!(project.candidates.len(), markers.len());
        for (candidate, marker) in project.candidates.iter().zip(&markers) {
            assert_eq!(candidate.observed_marker.bytes, *marker);
            assert!(candidate.observed_name.bytes.is_empty());
        }
        let fe = markers.iter().filter(|m| **m == [0xfe, 0xff]).count();
        let ff = markers.len() - fe;
        assert!(project
            .diagnostic_summary()
            .contains(&format!("Observed marker forms: fe ff {fe}, ff ff {ff}.")));
        assert!(project
            .candidates
            .windows(2)
            .all(|pair| pair[0].candidate_range.end < pair[1].candidate_range.start));
    }
}

#[test]
#[ignore = "requires explicitly authorized private Prologue structural evidence outside Git"]
fn authorized_prologue_two_marker_evidence_remains_structural_only() {
    use crate::app_contract::{
        DiagnosticsLevel, InspectProjectRequest, Readiness, CONTRACT_VERSION,
    };
    use crate::app_service::AppService;
    use sha2::{Digest, Sha256};
    assert!(
        std::env::var_os("PHOENIX_PROLOGUE_RESEARCH_AUTH_FILE").is_none(),
        "unrelated research decoder must be disabled"
    );
    let path = std::env::var("PHOENIX_PROLOGUE_OBSERVATION_SOURCE")
        .expect("explicitly authorized specimen path");
    let bytes = std::fs::read(&path).unwrap();
    assert_eq!(bytes.len(), 33057);
    assert_eq!(
        format!("{:x}", Sha256::digest(&bytes)),
        "b3a3efd8179d3ffe2a5026b3a40eab74da3ce6e674bb06162a176df07389b733"
    );
    let project = observe_project_120(&bytes).unwrap();
    assert_eq!(project.candidates.len(), 15);
    assert_eq!(
        project
            .candidates
            .iter()
            .filter(|c| c.marker_form == Observed120MarkerForm::FeFf)
            .count(),
        8
    );
    assert_eq!(
        project
            .candidates
            .iter()
            .filter(|c| c.marker_form == Observed120MarkerForm::FfFf)
            .count(),
        7
    );
    assert!(project
        .candidates
        .iter()
        .all(|c| !c.observed_name.bytes.is_empty()));
    for c in &project.candidates {
        assert_eq!(
            c.observed_marker.range,
            c.candidate_range.start + 41..c.candidate_range.start + 43
        );
        assert_eq!(
            Observed120MarkerForm::from_bytes(c.observed_marker.bytes),
            Some(c.marker_form)
        );
        assert_eq!(c.observed_name.range.start, c.candidate_range.start + 23);
        assert!(c.terminator_offset < c.candidate_range.start + 41);
    }
    let mut service = AppService::new();
    let inspection = service
        .inspect_project(InspectProjectRequest {
            contract_version: CONTRACT_VERSION,
            source_path: path,
            diagnostics_level: DiagnosticsLevel::Full,
        })
        .unwrap();
    assert!(inspection.sequences.is_empty());
    assert_eq!(inspection.project.sequence_count, 0);
    assert_eq!(inspection.project.overall_readiness, Readiness::Unknown);
    assert_eq!(inspection.project.profile_label, None);
    assert!(inspection.research_observation.is_none());
    assert!(service.profile_evidence(&inspection.session_id).is_err());
    let diagnostics = service
        .get_diagnostics(&inspection.session_id, DiagnosticsLevel::Full)
        .unwrap();
    assert_eq!(diagnostics.recognized_profile, None);
    assert_eq!(diagnostics.compatibility_profile, None);
    assert_eq!(diagnostics.export_report, None);
    let status = diagnostics.structural_status.unwrap();
    assert!(status.contains("InvalidSequenceNameBounds"));
    assert!(status.contains("Observed marker forms: fe ff 8, ff ff 7."));
    println!(
        "{}; semantic sequences: 0; readiness: Unknown; profile evidence/export authority absent.",
        project.diagnostic_summary()
    );
}
