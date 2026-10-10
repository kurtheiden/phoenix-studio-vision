use super::*;
use crate::observed_layout120::tests::synthetic_marker;
use crate::sequence_container::parse_project_166;

fn record(bytes: &mut Vec<u8>, kind: u8, payload: &[u8]) {
    bytes.push(kind);
    bytes.extend((payload.len() as u32).to_be_bytes());
    bytes.extend(payload);
}

fn fixture(labels: &[&[u8]], pairs: usize, marker: [u8; 2]) -> Vec<u8> {
    let mut bytes = synthetic_marker(labels.len() as u8, b"Example", marker);
    bytes.truncate(8 + 75 + 120 * labels.len());
    for (i, label) in labels.iter().enumerate() {
        let at = 8 + 121 + 120 * i;
        bytes[at..at + label.len()].copy_from_slice(label);
        bytes[at - 39..at - 31].copy_from_slice(&[0x80, 0, 4, 0, 0, 4, 1, 0]);
    }
    record(&mut bytes, 7, &[0; 10]);
    for i in 0..pairs {
        let mut primary = vec![0; if i < 2 { 38 } else { 21 }];
        if i >= 2 {
            let tail = primary.len() - 7;
            primary[tail] = 0xff;
            primary[tail + 4..].copy_from_slice(&[0xff, 0x2f, 0]);
        }
        record(&mut bytes, 2, &primary);
        record(&mut bytes, 0x29, &[0; 23]);
    }
    record(&mut bytes, 0, &[0; 32]);
    bytes
}

fn ordinary_fixture(labels: &[&[u8]], pairs: usize) -> Vec<u8> {
    let mut all: Vec<&[u8]> = vec![b"Meter Track", b"Tempo Track"];
    all.extend_from_slice(labels);
    fixture(&all, pairs, [0xfe, 0xff])
}

#[test]
fn unique_bijection_retains_duplicate_empty_and_raw_labels_for_both_markers() {
    for marker in [[0xfe, 0xff], [0xff, 0xff]] {
        let bytes = fixture(
            &[
                b"Meter Track",
                b"Tempo Track",
                b"Same",
                b"Same",
                b"",
                &[0x80],
            ],
            6,
            marker,
        );
        let bridge = associate_project_120(&bytes).unwrap();
        let sequence = bridge.candidates[0].association.as_ref().unwrap();
        assert_eq!(sequence.source_name.bytes, b"Example");
        assert_eq!(sequence.candidate_record_index, 0);
        assert_eq!(sequence.leading_special[0].slot.ordinal, 0);
        assert_eq!(sequence.leading_special[1].slot.ordinal, 1);
        assert!(
            sequence.leading_special[0].pair.primary.record_range.end
                <= sequence.leading_special[1].pair.primary.record_range.start
        );
        assert_eq!(sequence.ordinary.len(), 4);
        for (i, binding) in sequence.ordinary.iter().enumerate() {
            assert_eq!(binding.slot.ordinal, i + 2);
            assert_eq!(binding.local_pair_ordinal, i + 2);
            assert_eq!(binding.slot.view_range.len(), 120);
            assert!(binding.existing_bounds_probe.is_ok());
            assert!(
                binding.pair.primary.record_range.end <= binding.pair.secondary.record_range.start
            );
        }
        assert!(sequence.ordinary[2].slot.label.bytes.is_empty());
        assert_eq!(sequence.ordinary[3].slot.label.bytes, [0x80]);
    }
}

#[test]
fn sparse_and_excess_cardinality_refuse_entire_mapping_without_shifting() {
    for pairs in [2, 3, 5] {
        let bytes = ordinary_fixture(&[b"One", b"Two"], pairs);
        let bridge = associate_project_120(&bytes).unwrap();
        assert_eq!(
            bridge.candidates[0].association.as_ref().unwrap_err(),
            &AssociationRefusal::Cardinality { slots: 4, pairs }
        );
    }
}

#[test]
fn minimum_has_only_separate_special_bindings() {
    let bytes = ordinary_fixture(&[], 2);
    let bridge = associate_project_120(&bytes).unwrap();
    assert!(bridge.candidates[0]
        .association
        .as_ref()
        .unwrap()
        .ordinary
        .is_empty());
}

#[test]
fn present_pair_with_unsupported_terminal_retains_association() {
    let mut bytes = ordinary_fixture(&[b"Track"], 3);
    let root = parse_root_record_stream(&bytes).unwrap();
    let range = root.records[6].payload.range.clone();
    bytes[range.end - 7..range.end].fill(0);
    let bridge = associate_project_120(&bytes).unwrap();
    let sequence = bridge.candidates[0].association.as_ref().unwrap();
    assert_eq!(sequence.ordinary.len(), 1);
    assert!(matches!(
        sequence.ordinary[0].existing_bounds_probe,
        Err(TrackEventBoundsError::InvalidTerminalGrammar { .. })
    ));
    assert!(bridge.diagnostic_summary().contains("refused 1"));
    assert!(parse_project_166(&bytes).is_err());
}

#[test]
fn label_cannot_borrow_next_record_and_special_pattern_is_required() {
    let mut bytes = ordinary_fixture(&[b"Track"], 3);
    let end = 8 + 75 + 120 * 3;
    bytes[8 + 121 + 240..end].fill(1);
    let bridge = associate_project_120(&bytes).unwrap();
    assert_eq!(
        bridge.candidates[0].association.as_ref().unwrap_err(),
        &AssociationRefusal::MissingLabel { ordinal: 2 }
    );
    let bytes = fixture(&[b"Other", b"Tempo Track", b"Track"], 3, [0xfe, 0xff]);
    assert_eq!(
        associate_project_120(&bytes).unwrap().candidates[0]
            .association
            .as_ref()
            .unwrap_err(),
        &AssociationRefusal::SpecialPositions
    );
}

#[test]
fn incomplete_runs_wrong_trailers_and_truncation_refuse() {
    let original = ordinary_fixture(&[b"Track"], 3);
    let root = parse_root_record_stream(&original).unwrap();
    for index in [2, 3, 8] {
        let mut bytes = original.clone();
        bytes[root.records[index].record_range.start] = 0x09;
        assert_eq!(
            associate_project_120(&bytes).unwrap().candidates[0]
                .association
                .as_ref()
                .unwrap_err(),
            &AssociationRefusal::IncompleteNeighborhood
        );
    }
    let mut bytes = original.clone();
    let following = root.records[1].record_range.clone();
    bytes[following.start + 1..following.start + 5].copy_from_slice(&11_u32.to_be_bytes());
    bytes.insert(following.end, 0);
    assert_eq!(
        associate_project_120(&bytes).unwrap().candidates[0]
            .association
            .as_ref()
            .unwrap_err(),
        &AssociationRefusal::UnsupportedTrailer
    );
    for end in [0, 8, original.len() - 1] {
        assert!(associate_project_120(&original[..end]).is_err());
    }
}

#[test]
fn sibling_neighborhoods_are_separate_and_invalid_child_cannot_be_swallowed() {
    let first = ordinary_fixture(&[b"Same"], 3);
    let second = ordinary_fixture(&[b"Same"], 3);
    let mut bytes = first.clone();
    bytes.extend_from_slice(&second[8..]);
    let bridge = associate_project_120(&bytes).unwrap();
    let a = bridge.candidates[0].association.as_ref().unwrap();
    let b = bridge.candidates[1].association.as_ref().unwrap();
    assert!(a.sequence_range.end <= b.sequence_range.start);
    let root = parse_root_record_stream(&first).unwrap();
    let mut bytes = first[..root.records.last().unwrap().record_range.start].to_vec();
    bytes.extend_from_slice(&second[8..]);
    let bridge = associate_project_120(&bytes).unwrap();
    assert_eq!(
        bridge.candidates[0].association.as_ref().unwrap_err(),
        &AssociationRefusal::IncompleteNeighborhood
    );
    assert!(bridge.candidates[1].association.is_ok());
}

#[test]
fn original_observer_guards_are_not_relaxed_by_semantic_bridge() {
    let original = ordinary_fixture(&[b"Track"], 3);
    for offset in [8 + 5, 8 + 21, 8 + 22, 8 + 41, 8 + 42] {
        let mut bytes = original.clone();
        bytes[offset] ^= 2;
        assert!(associate_project_120(&bytes).is_err());
    }
}

#[test]
fn service_association_alone_cannot_create_export_authority() {
    use crate::app_contract::{
        DiagnosticsLevel, InspectProjectRequest, Readiness, CONTRACT_VERSION,
    };
    use crate::app_service::AppService;
    let bytes = ordinary_fixture(&[b"Track"], 3);
    let path =
        std::env::temp_dir().join(format!("phoenix-source120-firewall-{}", std::process::id()));
    std::fs::write(&path, bytes).unwrap();
    let mut service = AppService::new();
    let result = service
        .inspect_project(InspectProjectRequest {
            contract_version: CONTRACT_VERSION,
            source_path: path.to_string_lossy().into_owned(),
            diagnostics_level: DiagnosticsLevel::Full,
        })
        .unwrap();
    std::fs::remove_file(path).unwrap();
    assert_eq!(result.sequences.len(), 1);
    assert!(result
        .sequences
        .iter()
        .all(|row| row.readiness != Readiness::Ready
            && row.bounded_export_capability.is_none()
            && row.export_capability.is_none()));
    assert!(service.profile_evidence(&result.session_id).is_err());
    let diagnostics = service
        .get_diagnostics(&result.session_id, DiagnosticsLevel::Full)
        .unwrap();
    assert!(diagnostics.export_report.is_none());
    assert!(diagnostics
        .technical_errors
        .iter()
        .any(|s| s.contains("1 sequence associations, 1 ordinary pair bindings")));
}

#[test]
#[ignore = "requires explicitly authorized private SCHOOL PROJECTS evidence outside Git"]
fn authorized_school_recovery_reach() {
    let path = std::env::var("PHOENIX_SCHOOL_OBSERVATION_SOURCE").unwrap();
    let bytes = std::fs::read(path).unwrap();
    use sha2::{Digest, Sha256};
    assert_eq!(bytes.len(), 343875);
    assert_eq!(
        format!("{:x}", Sha256::digest(&bytes)),
        "bfd4fa1208e2cd884ec51ccfb1131d5c02723d7c90acafc98bd89597e1a20331"
    );
    let bridge = associate_project_120(&bytes).unwrap();
    let mut accepted = 0;
    let mut associated = 0;
    let mut matched = 0;
    for (i, candidate) in bridge.candidates.iter().enumerate() {
        match &candidate.association {
            Ok(sequence) => {
                accepted += 1;
                associated += sequence.ordinary.len();
                matched += sequence
                    .ordinary
                    .iter()
                    .filter(|b| b.existing_bounds_probe.is_ok())
                    .count();
                eprintln!("ordinal {}: semantic association, {} ordinary bindings, {} existing-bound refusals", i + 1, sequence.ordinary.len(), sequence.ordinary.iter().filter(|b| b.existing_bounds_probe.is_err()).count());
                for b in &sequence.ordinary {
                    if let Err(error) = &b.existing_bounds_probe {
                        eprintln!(
                            "unit {} pair {}: {error:?}",
                            b.slot.ordinal, b.local_pair_ordinal
                        );
                    }
                }
            }
            Err(error) => eprintln!("ordinal {}: {error:?}", i + 1),
        }
    }
    eprintln!("{}", bridge.diagnostic_summary());
    assert_eq!(bridge.candidates.len(), 15);
    let h = bridge.candidates[7].association.as_ref().unwrap();
    assert_eq!(
        h.no_event_data
            .iter()
            .map(|t| t.slot.ordinal)
            .collect::<Vec<_>>(),
        vec![9, 10, 11, 12]
    );
    assert_eq!(
        h.ordinary
            .iter()
            .map(|b| (b.slot.ordinal, b.local_pair_ordinal))
            .collect::<Vec<_>>(),
        vec![
            (2, 2),
            (3, 3),
            (4, 4),
            (5, 5),
            (6, 6),
            (7, 7),
            (8, 8),
            (13, 9)
        ]
    );
    let m = bridge.candidates[12].association.as_ref().unwrap();
    assert!(m.ordinary.is_empty());
    assert_eq!(m.no_event_data.len(), 1);
    assert_eq!(m.no_event_data[0].slot.ordinal, 2);
    assert!(m.no_event_data[0].slot.label.bytes.is_empty());
    assert!(matches!(
        bridge.candidates[2].association,
        Err(AssociationRefusal::UnsupportedTrailer)
    ));
    assert!(matches!(
        bridge.candidates[3].association,
        Err(AssociationRefusal::UnsupportedTrailer)
    ));
    assert_eq!(accepted, 12);
    assert_eq!(associated, 129);
    assert_eq!(matched, 128);
}

#[test]
#[ignore = "requires explicitly authorized prospective Prologue check outside Git"]
fn prospective_prologue_first_candidate() {
    let path = std::env::var("PHOENIX_PROLOGUE_OBSERVATION_SOURCE").unwrap();
    let bytes = std::fs::read(path).unwrap();
    use sha2::{Digest, Sha256};
    assert_eq!(bytes.len(), 33057);
    assert_eq!(
        format!("{:x}", Sha256::digest(&bytes)),
        "b3a3efd8179d3ffe2a5026b3a40eab74da3ce6e674bb06162a176df07389b733"
    );
    let observations = observe_project_120(&bytes).unwrap();
    let root = parse_root_record_stream(&bytes).unwrap();
    let result = associate_candidate(&bytes, &root.records, &observations.candidates[0]);
    match result {
        Ok(sequence) => eprintln!("PROSPECTIVE FIRST CANDIDATE: accepted {} ordinary associations, {} matched existing-bound probes; no conductor/event/export authority", sequence.ordinary.len(), sequence.ordinary.iter().filter(|b| b.existing_bounds_probe.is_ok()).count()),
        Err(error) => eprintln!("PROSPECTIVE FIRST CANDIDATE REFUSED: {error:?}"),
    }
}

#[test]
#[ignore = "requires explicitly authorized private SCHOOL PROJECTS evidence outside Git"]
fn authorized_school_event_meter() {
    use crate::mixed_event::{
        walk_bounded_mixed_events, MixedEventBounds, MixedEventItem, MixedEventKind,
        MixedEventWalkError,
    };
    use crate::sequence_container::TrackRecordPair;
    use sha2::{Digest, Sha256};
    let bytes = std::fs::read(std::env::var("PHOENIX_SCHOOL_OBSERVATION_SOURCE").unwrap()).unwrap();
    assert_eq!(bytes.len(), 343875);
    assert_eq!(
        format!("{:x}", Sha256::digest(&bytes)),
        "bfd4fa1208e2cd884ec51ccfb1131d5c02723d7c90acafc98bd89597e1a20331"
    );
    let bridge = associate_project_120(&bytes).unwrap();
    let mut complete = 0;
    let mut excluded = 0;
    let mut families = [0usize; 5];
    let mut errors = std::collections::BTreeMap::new();
    let mut full_sequences = 0;
    for (ci, candidate) in bridge.candidates.iter().enumerate() {
        let Ok(s) = &candidate.association else {
            continue;
        };
        let mut done = 0;
        for b in &s.ordinary {
            let start = b.pair.primary.payload.range.start + 14;
            let pair = TrackRecordPair {
                pair_ordinal: b.local_pair_ordinal,
                primary: b.pair.primary.clone(),
                secondary: b.pair.secondary.clone(),
                candidate_event_start: start,
                event_containing_range: start..b.pair.primary.payload.range.end,
            };
            let range = match pair.validated_event_bounds() {
                Ok(r) => r.event_range,
                Err(e) => {
                    excluded += 1;
                    eprintln!(
                        "EXCLUDED candidate={} unit={} {e:?}",
                        ci + 1,
                        b.slot.ordinal
                    );
                    continue;
                }
            };
            match walk_bounded_mixed_events(
                &bytes,
                MixedEventBounds {
                    event_range: range.clone(),
                },
                Default::default(),
            ) {
                Ok(w) => {
                    complete += 1;
                    done += 1;
                    for item in &w.items {
                        match item {
                            MixedEventItem::Patch(_) => families[1] += 1,
                            MixedEventItem::PatchToNote(_) => {
                                families[0] += 1;
                                families[1] += 1;
                            }
                            MixedEventItem::Event(e) => {
                                families[match e.event {
                                    MixedEventKind::Note(_)
                                    | MixedEventKind::ContextMediatedNote(_)
                                    | MixedEventKind::DoubleContextMediatedNote(_) => 0,
                                    MixedEventKind::Controller(_)
                                    | MixedEventKind::MidiController(_) => 2,
                                    MixedEventKind::ChannelPressure { .. } => 3,
                                    MixedEventKind::PitchBend { .. } => 4,
                                    MixedEventKind::SysEx(_) => panic!("120 walker admitted SysEx"),
                                    MixedEventKind::Program(_) => {
                                        panic!("generic walker admitted scoped Program")
                                    }
                                }] += 1
                            }
                        }
                    }
                }
                Err(e) => {
                    let class = format!("{e:?}")
                        .split_whitespace()
                        .next()
                        .unwrap()
                        .to_string();
                    *errors.entry(class).or_insert(0usize) += 1;
                    eprintln!("REFUSAL candidate={} name={:?} unit={} pair={} track={:?} range={:?} {e:?}", ci+1, String::from_utf8_lossy(s.source_name.bytes), b.slot.ordinal,b.local_pair_ordinal,String::from_utf8_lossy(b.slot.label.bytes),range);
                    if let MixedEventWalkError::UnsupportedStatus { cursor, offset, .. } = e {
                        eprintln!(
                            "COHORT bytes={:02x?} preceding={:02x?}",
                            &bytes[cursor..(offset + 16).min(range.end)],
                            &bytes[cursor.saturating_sub(16).max(range.start)..cursor]
                        );
                        if let Ok(prefix) = walk_bounded_mixed_events(
                            &bytes,
                            MixedEventBounds {
                                event_range: range.start..cursor,
                            },
                            Default::default(),
                        ) {
                            eprintln!("PREVIOUS {:?}", prefix.items.last());
                        }
                    }
                }
            }
        }
        if done == s.ordinary.len() {
            full_sequences += 1;
        }
        eprintln!(
            "SEQUENCE candidate={} name={:?} complete={}/{}",
            ci + 1,
            String::from_utf8_lossy(s.source_name.bytes),
            done,
            s.ordinary.len()
        );
    }
    eprintln!("METER complete={complete}/102 excluded={excluded} errors={errors:?} families(Note,Patch,Controller,Pressure,Bend)={families:?} total={} full_sequences={full_sequences}/8",families.iter().sum::<usize>());
    assert_eq!(excluded, 1);
}

fn zero_state(bytes: &mut [u8], ordinary_ordinal: usize) {
    let label = 8 + 121 + 120 * (ordinary_ordinal + 2);
    bytes[label - 39..label - 31].copy_from_slice(&[0, 0, 4, 0, 0, 4, 0, 0]);
}

#[test]
fn no_data_tracks_are_retained_and_do_not_shift_consuming_pair_order() {
    let mut bytes = ordinary_fixture(&[b"First", b"Unused", b"Last"], 4);
    zero_state(&mut bytes, 1);
    let bridge = associate_project_120(&bytes).unwrap();
    let sequence = bridge.candidates[0].association.as_ref().unwrap();
    assert_eq!(sequence.ordinary.len(), 2);
    assert_eq!(sequence.no_event_data.len(), 1);
    assert_eq!(sequence.ordinary[0].slot.ordinal, 2);
    assert_eq!(sequence.ordinary[0].local_pair_ordinal, 2);
    assert_eq!(sequence.ordinary[1].slot.ordinal, 4);
    assert_eq!(sequence.ordinary[1].local_pair_ordinal, 3);
    let retained = &sequence.no_event_data[0];
    assert_eq!(retained.slot.ordinal, 3);
    assert_eq!(retained.slot.label.bytes, b"Unused");
    assert_eq!(retained.state_context.bytes, [0, 0, 4, 0, 0, 4, 0, 0]);
    assert_eq!(
        &bytes[retained.state_context.range.clone()],
        retained.state_context.bytes
    );
    assert_eq!(sequence.leading_special[0].slot.ordinal, 0);
    assert_eq!(sequence.leading_special[1].slot.ordinal, 1);
    assert!(parse_project_166(&bytes).is_err());
    assert!(bridge
        .diagnostic_details()
        .iter()
        .any(|d| d.contains("retained zero-event track")));
}

#[test]
fn consecutive_and_all_no_data_tracks_remain_in_the_model() {
    for remaining in [false, true] {
        let mut bytes = ordinary_fixture(
            &[b"", b"Unused", b"Remaining"],
            if remaining { 3 } else { 2 },
        );
        zero_state(&mut bytes, 0);
        zero_state(&mut bytes, 1);
        if !remaining {
            zero_state(&mut bytes, 2);
        }
        let bridge = associate_project_120(&bytes).unwrap();
        let sequence = bridge.candidates[0].association.as_ref().unwrap();
        assert_eq!(sequence.no_event_data.len(), if remaining { 2 } else { 3 });
        assert_eq!(sequence.ordinary.len(), usize::from(remaining));
        assert_eq!(sequence.no_event_data[0].slot.ordinal, 2);
        assert!(sequence.no_event_data[0].slot.label.bytes.is_empty());
        if remaining {
            assert_eq!(sequence.ordinary[0].local_pair_ordinal, 2);
        }
    }
}

#[test]
fn adjusted_cardinality_refuses_both_shortage_and_surplus() {
    for pairs in [2, 4, 5] {
        let mut bytes = ordinary_fixture(&[b"Unused", b"Present"], pairs);
        zero_state(&mut bytes, 0);
        assert_eq!(
            associate_project_120(&bytes).unwrap().candidates[0]
                .association
                .as_ref()
                .unwrap_err(),
            &AssociationRefusal::Cardinality { slots: 3, pairs }
        );
    }
}

#[test]
fn isolated_zero_byte_and_nearby_contexts_do_not_authorize_omission() {
    let original = ordinary_fixture(&[b"Unused"], 2);
    for changed in 1..8 {
        let mut bytes = original.clone();
        zero_state(&mut bytes, 0);
        bytes[8 + 121 + 240 - 39 + changed] ^= 1;
        assert!(matches!(
            associate_project_120(&bytes).unwrap().candidates[0].association,
            Err(AssociationRefusal::Cardinality { slots: 3, pairs: 2 })
        ));
    }
}

#[test]
fn zero_state_specials_still_require_their_own_pairs() {
    let mut bytes = ordinary_fixture(&[b"Unused"], 2);
    zero_state(&mut bytes, 0);
    for ordinal in 0..2 {
        let label = 8 + 121 + 120 * ordinal;
        bytes[label - 39..label - 31].copy_from_slice(&[0, 0, 4, 0, 0, 4, 0, 0]);
    }
    let bridge = associate_project_120(&bytes).unwrap();
    let sequence = bridge.candidates[0].association.as_ref().unwrap();
    assert_eq!(sequence.no_event_data.len(), 1);
    assert_eq!(sequence.leading_special[0].slot.label.bytes, b"Meter Track");
    assert_eq!(sequence.leading_special[1].slot.label.bytes, b"Tempo Track");
}

fn prelude_fixture(payload: &[u8], pairs: usize) -> Vec<u8> {
    let mut bytes = ordinary_fixture(&[b"One", b"Two"], pairs);
    let range = parse_root_record_stream(&bytes).unwrap().records[1]
        .record_range
        .clone();
    let mut replacement = Vec::new();
    record(&mut replacement, 7, payload);
    bytes.splice(range, replacement);
    bytes
}

fn longer_payload(count: usize) -> Vec<u8> {
    let mut payload = vec![0, count as u8];
    payload.extend(vec![b'X'; count]);
    payload.extend([0; 8]);
    payload
}

#[test]
fn bounded_longer_preludes_retain_opaque_provenance_and_order() {
    for payload in [vec![0xa5; 10], longer_payload(12), longer_payload(20)] {
        let bytes = prelude_fixture(&payload, 4);
        let bridge = associate_project_120(&bytes).unwrap();
        let sequence = bridge.candidates[0].association.as_ref().unwrap();
        assert_eq!(sequence.prelude.payload.bytes, payload);
        assert_eq!(sequence.prelude.record_range.len(), payload.len() + 5);
        assert_eq!(
            sequence.prelude.record_range,
            bridge.candidates[0].observation.following_record_range
        );
        assert_eq!(
            sequence.prelude.payload.range.start,
            sequence.prelude.record_range.start + 5
        );
        assert_eq!(&bytes[sequence.prelude.payload.range.clone()], payload);
        assert_eq!(
            sequence.leading_special[0].pair.primary.record_range.start,
            sequence.prelude.record_range.end
        );
        assert_eq!(sequence.ordinary.len(), 2);
        assert_eq!(sequence.source_name.bytes, b"Example");
        for (index, track) in sequence.ordinary.iter().enumerate() {
            assert_eq!(track.local_pair_ordinal, index + 2);
            assert_eq!(track.slot.ordinal, index + 2);
        }
    }
}

#[test]
fn longer_preludes_never_relax_exact_consuming_cardinality() {
    for count in [12, 20] {
        for pairs in [2, 3, 5] {
            let bytes = prelude_fixture(&longer_payload(count), pairs);
            let bridge = associate_project_120(&bytes).unwrap();
            assert_eq!(
                bridge.candidates[0].association.as_ref().unwrap_err(),
                &AssociationRefusal::Cardinality { slots: 4, pairs }
            );
        }
        let mut bytes = prelude_fixture(&longer_payload(count), 3);
        let label = 8 + 121 + 120 * 2;
        bytes[label - 39..label - 31].copy_from_slice(&[0, 0, 4, 0, 0, 4, 0, 0]);
        let bridge = associate_project_120(&bytes).unwrap();
        let sequence = bridge.candidates[0].association.as_ref().unwrap();
        assert_eq!(sequence.no_event_data.len(), 1);
        assert_eq!(sequence.ordinary[0].slot.ordinal, 3);
        assert_eq!(sequence.ordinary[0].local_pair_ordinal, 2);
    }
}

#[test]
fn longer_prelude_length_count_prefix_field_and_suffix_are_exact() {
    let mut invalid = vec![
        longer_payload(11),
        longer_payload(13),
        longer_payload(19),
        longer_payload(21),
    ];
    for count in [12, 20] {
        for (offset, value) in [
            (0, 1),
            (1, 11),
            (2, 0),
            (2, 0x1f),
            (2, 0x7f),
            (2, 0x80),
            (count + 2, 1),
            (count + 9, 1),
        ] {
            let mut payload = longer_payload(count);
            payload[offset] = value;
            invalid.push(payload);
        }
    }
    for payload in invalid {
        let bytes = prelude_fixture(&payload, 4);
        let bridge = associate_project_120(&bytes).unwrap();
        assert_eq!(
            bridge.candidates[0].association.as_ref().unwrap_err(),
            &AssociationRefusal::UnsupportedTrailer
        );
    }
}

#[test]
fn longer_preludes_require_complete_unambiguous_neighborhoods() {
    let bytes = prelude_fixture(&longer_payload(12), 4);
    let root = parse_root_record_stream(&bytes).unwrap();
    for index in [2, 3, root.records.len() - 1] {
        let mut malformed = bytes.clone();
        malformed[root.records[index].record_range.start] = 9;
        let bridge = associate_project_120(&malformed).unwrap();
        assert_eq!(
            bridge.candidates[0].association.as_ref().unwrap_err(),
            &AssociationRefusal::IncompleteNeighborhood
        );
    }
    let mut no_pairs = bytes[..root.records[2].record_range.start].to_vec();
    record(&mut no_pairs, 0, &[0; 32]);
    assert_eq!(
        associate_project_120(&no_pairs).unwrap().candidates[0]
            .association
            .as_ref()
            .unwrap_err(),
        &AssociationRefusal::Cardinality { slots: 4, pairs: 0 }
    );
    for end in [
        root.records[1].record_range.end - 1,
        root.records.last().unwrap().record_range.start,
        bytes.len() - 1,
    ] {
        assert!(
            associate_project_120(&bytes[..end]).is_err()
                || associate_project_120(&bytes[..end]).unwrap().candidates[0]
                    .association
                    .is_err()
        );
    }
    let mut swallowed = bytes[..root.records.last().unwrap().record_range.start].to_vec();
    swallowed.extend_from_slice(&bytes[8..]);
    let bridge = associate_project_120(&swallowed).unwrap();
    assert_eq!(
        bridge.candidates[0].association.as_ref().unwrap_err(),
        &AssociationRefusal::IncompleteNeighborhood
    );
    assert!(bridge.candidates[1].association.is_ok());
    let mut missing_adjacent = bytes.clone();
    missing_adjacent[root.records[1].record_range.start] = 9;
    assert!(associate_project_120(&missing_adjacent).is_err());
}

fn sparse_fixture(participating: &[usize], payloads: &[&[u8]], pairs: usize) -> Vec<u8> {
    let mut b = ordinary_fixture(&[b"First", b"Gap", b"Last"], pairs);
    for ordinal in 0..5 {
        let at = 8 + 106 + 120 * ordinal + 85;
        b[at..at + 4].fill(0);
        if participating.contains(&ordinal) {
            b[at..at + 4].copy_from_slice(&(ordinal as u32 + 1).to_be_bytes());
        }
    }
    let root = parse_root_record_stream(&b).unwrap();
    let at = root.records[1].record_range.end;
    let mut extra = Vec::new();
    for p in payloads {
        record(&mut extra, 7, p);
    }
    b.splice(at..at, extra);
    b
}

#[test]
fn sparse_type07_opaque_order_identity_and_exact_accounting() {
    let b = sparse_fixture(&[2, 4], &[&[0x80, 0xff], &[0, 1, 2]], 5);
    let dense = ordinary_fixture(&[b"First", b"Gap", b"Last"], 5);
    let bridge = associate_project_120(&b).unwrap();
    let s = bridge.candidates[0].association.as_ref().unwrap();
    let old = associate_project_120(&dense).unwrap();
    let old = old.candidates[0].association.as_ref().unwrap();
    assert_eq!(s.source_name.bytes, old.source_name.bytes);
    assert_eq!(
        s.metadata
            .iter()
            .map(|m| m.track_ordinal)
            .collect::<Vec<_>>(),
        [2, 4]
    );
    assert_eq!(s.metadata[0].record.payload.bytes, [0x80, 0xff]);
    assert_eq!(s.metadata[1].record.payload.bytes, [0, 1, 2]);
    assert_eq!(
        s.metadata[0].record.record_range.start,
        s.prelude.record_range.end
    );
    assert_eq!(
        s.metadata[0].record.record_range.end,
        s.metadata[1].record.record_range.start
    );
    assert_eq!(
        s.metadata[1].record.record_range.end,
        s.leading_special[0].pair.primary.record_range.start
    );
    assert_eq!(s.sequence_range.end, b.len());
    for (t, o) in s.ordinary.iter().zip(&old.ordinary) {
        assert_eq!(t.slot.ordinal, o.slot.ordinal);
        assert_eq!(t.slot.label.bytes, o.slot.label.bytes);
        assert_eq!(t.local_pair_ordinal, o.local_pair_ordinal);
        assert_eq!(t.pair.primary.payload.bytes, o.pair.primary.payload.bytes);
        assert_eq!(
            t.pair.secondary.payload.bytes,
            o.pair.secondary.payload.bytes
        );
    }
    assert!(old.metadata.is_empty());
    // Payload values and repeated names do not choose or reorder owners.
    let mut changed = b.clone();
    for m in &s.metadata {
        changed[m.record.payload.range.clone()].fill(0xff);
    }
    let again = associate_project_120(&changed).unwrap();
    assert_eq!(
        again.candidates[0]
            .association
            .as_ref()
            .unwrap()
            .metadata
            .iter()
            .map(|m| m.track_ordinal)
            .collect::<Vec<_>>(),
        [2, 4]
    );
}

#[test]
fn sparse_type07_conflicts_variants_and_cardinality_stay_refused() {
    for (mask, payloads) in [
        (vec![0, 2], vec![vec![1, 2], vec![3, 4]]),
        (vec![2, 4], vec![vec![1, 2]]),
        (vec![2], vec![vec![1, 2], vec![3, 4]]),
        (vec![], vec![vec![1, 2]]),
        (vec![2], vec![vec![]]),
        (vec![2], vec![vec![1; 4]]),
        (vec![2], vec![vec![1; 16]]),
    ] {
        let p: Vec<_> = payloads.iter().map(Vec::as_slice).collect();
        let b = sparse_fixture(&mask, &p, 5);
        assert!(matches!(
            associate_project_120(&b).unwrap().candidates[0].association,
            Err(AssociationRefusal::SparseMetadata)
        ));
    }
    let b = sparse_fixture(&[2, 4], &[&[1, 2], &[3, 4]], 4);
    assert!(matches!(
        associate_project_120(&b).unwrap().candidates[0].association,
        Err(AssociationRefusal::Cardinality { slots: 5, pairs: 4 })
    ));
    let mut b = sparse_fixture(&[2], &[&[1, 2]], 5);
    let root = parse_root_record_stream(&b).unwrap();
    // A metadata record after a musical pair is outside the bounded list.
    let range = root.records[2].record_range.clone();
    let at = root.records[4].record_range.end;
    let extra = b[range.clone()].to_vec();
    b.splice(at..at, extra);
    b.drain(range);
    assert!(matches!(
        associate_project_120(&b).unwrap().candidates[0].association,
        Err(AssociationRefusal::IncompleteNeighborhood)
    ));
}

#[test]
#[ignore = "requires explicitly supplied preserved Prologue original and normalized specimens"]
fn sparse_type07_authentic_cross_layout_and_remaining_guards_read_only() {
    use sha2::{Digest, Sha256};
    let b = std::fs::read(std::env::var("PHOENIX_CCA005_ORIGINAL").unwrap()).unwrap();
    let normalized = std::fs::read(std::env::var("PHOENIX_CCA005_NORMALIZED").unwrap()).unwrap();
    assert_eq!(
        format!("{:x}", Sha256::digest(&b)),
        "b3a3efd8179d3ffe2a5026b3a40eab74da3ce6e674bb06162a176df07389b733"
    );
    assert_eq!(
        format!("{:x}", Sha256::digest(&normalized)),
        "0618cd5087987d7f694ca7532e1cc8a01b4adf7e3043ec648783d71e7e5ad714"
    );
    let observed = observe_project_120(&b).unwrap();
    let root = parse_root_record_stream(&b).unwrap();
    let nroot = parse_root_record_stream(&normalized).unwrap();
    let bridge = associate_project_120(&b).unwrap();
    let proofs = crate::authority120::prove_project120(&b).unwrap();
    // PM-R12 cross-layout ranges, pinned by both source hashes; no payload semantics.
    let expected: &[(usize, usize, &[usize], &[usize])] = &[
        (0, 2520, &[2, 3, 4], &[3551, 3559, 3570]),
        (1, 5046, &[2, 3, 4, 6], &[6412, 6420, 6433, 6446]),
        (2, 7948, &[2, 3, 4, 6, 7], &[9480, 9488, 9499, 9510, 9518]),
        (
            3,
            15391,
            &[2, 5, 6, 7, 8],
            &[17421, 17428, 17436, 17444, 17452],
        ),
        (4, 19079, &[2, 3, 4], &[20114, 20122, 20133]),
        (
            5,
            21383,
            &[2, 3, 4, 6, 7, 8],
            &[23245, 23253, 23270, 23281, 23289, 23297],
        ),
        (
            6,
            27805,
            &[2, 3, 4, 5, 6, 7],
            &[29501, 29509, 29516, 29524, 29532, 29547],
        ),
        (8, 30672, &[2, 3, 4], &[31703, 31711, 31722]),
        (9, 33023, &[2, 3, 4, 6], &[34389, 34397, 34410, 34423]),
        (
            10,
            35925,
            &[2, 3, 4, 6, 7],
            &[37457, 37465, 37476, 37487, 37495],
        ),
        (
            11,
            43368,
            &[2, 5, 6, 7, 8],
            &[45398, 45405, 45413, 45421, 45429],
        ),
        (12, 46881, &[2, 3, 4], &[47916, 47924, 47935]),
        (
            13,
            49185,
            &[2, 3, 4, 6, 7, 8],
            &[51047, 51055, 51072, 51083, 51091, 51099],
        ),
        (
            14,
            55432,
            &[2, 3, 4, 5, 6, 7],
            &[57128, 57136, 57143, 57151, 57159, 57174],
        ),
    ];
    let mut total = 0;
    for &(ordinal, nstart, mask, record_starts) in expected {
        let c = &observed.candidates[ordinal];
        let mut cursor = c.record_index + 2;
        let metadata = associate_sparse_metadata(&b, &root.records, c, &mut cursor).unwrap();
        assert_eq!(
            metadata.iter().map(|m| m.track_ordinal).collect::<Vec<_>>(),
            mask
        );
        assert_eq!(metadata.len(), record_starts.len());
        for (m, &start) in metadata.iter().zip(record_starts) {
            let nr = nroot
                .records
                .iter()
                .find(|r| r.record_range.start == start)
                .unwrap();
            assert_eq!(nr.record_type.value, 7);
            assert_eq!(m.record.payload.bytes, nr.payload.bytes);
            assert_eq!(
                m.record.record_range.len(),
                5 + m.record.payload.bytes.len()
            );
        }
        // Normalized slots preserve the entire ordered mask and raw labels.
        for slot in 0..usize::from(c.count.value) {
            let original_label = c.candidate_range.start + 121 + 120 * slot;
            let normalized_label = nstart + 223 + 166 * slot;
            let old = b[original_label..c.candidate_range.end]
                .split(|v| *v == 0)
                .next()
                .unwrap();
            let new = normalized[normalized_label..]
                .split(|v| *v == 0)
                .next()
                .unwrap();
            assert_eq!(old, new);
            let at = nstart + 208 + 166 * slot + 85;
            assert_eq!(
                normalized[at..at + 4].iter().any(|v| *v != 0),
                mask.contains(&slot)
            );
        }
        total += metadata.len();
        if [0, 3, 5].contains(&ordinal) {
            let s = bridge.candidates[ordinal].association.as_ref().unwrap();
            assert_eq!(s.metadata.len(), metadata.len());
            assert!(proofs[ordinal].result.is_ok());
            let ranges = [
                s.leading_special[0].pair.primary.record_range.clone(),
                s.leading_special[0].pair.secondary.record_range.clone(),
                s.leading_special[1].pair.primary.record_range.clone(),
                s.leading_special[1].pair.secondary.record_range.clone(),
            ];
            assert_eq!(
                crate::bounded_conductor::decode_compact_conductor(&b, ranges).unwrap_err(),
                crate::bounded_conductor::ConductorRefusal::Envelope
            );
            eprintln!(
                "{}: sparse association and C0 authority pass; conductor Envelope remains",
                String::from_utf8_lossy(s.source_name.bytes)
            );
        } else {
            assert!(matches!(
                bridge.candidates[ordinal].association,
                Err(AssociationRefusal::Cardinality { .. })
            ));
            eprintln!(
                "{}: sparse metadata bounded; {:?}",
                String::from_utf8_lossy(c.observed_name.bytes),
                bridge.candidates[ordinal].association.as_ref().unwrap_err()
            );
        }
    }
    assert_eq!(total, 64);
    assert!(matches!(
        bridge.candidates[7].association,
        Err(AssociationRefusal::IncompleteNeighborhood)
    ));
    eprintln!("64 sparse records / 14 Frames matched; 3 conductor refusals, 11 cardinality refusals; separate Prologue unchanged; no MIDI generated");
}

#[test]
fn sparse_type07_label_overlap_and_missing_terminal_refuse() {
    let mut b = sparse_fixture(&[2], &[&[1, 2]], 5);
    let label = 8 + 121 + 240;
    b[label..label + 70].fill(b'x');
    assert!(matches!(
        associate_project_120(&b).unwrap().candidates[0].association,
        Err(AssociationRefusal::SparseMetadata)
    ));
    let mut b = sparse_fixture(&[2], &[&[1, 2]], 5);
    let root = parse_root_record_stream(&b).unwrap();
    b.truncate(root.records.last().unwrap().record_range.start);
    assert!(matches!(
        associate_project_120(&b).unwrap().candidates[0].association,
        Err(AssociationRefusal::IncompleteNeighborhood)
    ));
}

fn primary_only_fixture(labels: &[&[u8]], pairs: usize, primary_ordinal: usize) -> Vec<u8> {
    let mut bytes = ordinary_fixture(labels, pairs);
    let root = parse_root_record_stream(&bytes).unwrap();
    let primary = &root.records[2 + 2 * primary_ordinal];
    let secondary = root.records[3 + 2 * primary_ordinal].record_range.clone();
    let prefix = primary.payload.range.start;
    bytes[prefix..prefix + 2].copy_from_slice(&[0, 1]);
    bytes.drain(secondary);
    bytes
}

#[test]
fn primary_only_structural_binding_keeps_source_order_without_export_authority() {
    // Unrelated names and an empty body demonstrate that musical signatures are not guards.
    let bytes = primary_only_fixture(&[b"Unrelated", b"Following"], 4, 2);
    let bridge = associate_semantic_project_120(&bytes).unwrap();
    let sequence = bridge.candidates[0].association.as_ref().unwrap();
    let track = &sequence.primary_only[0];
    assert_eq!(track.slot.ordinal, 2);
    assert_eq!(track.slot.label.bytes, b"Unrelated");
    assert_eq!(track.local_primary_ordinal, 2);
    assert_eq!(track.declared_count, 0);
    assert!(track.event_range.is_empty());
    assert_eq!(track.zero_prefix_field.bytes, [0; 4]);
    assert_eq!(sequence.ordinary.len(), 1);
    assert_eq!(sequence.ordinary[0].slot.ordinal, 3);
    assert_eq!(sequence.ordinary[0].local_pair_ordinal, 3);
    assert_eq!(
        track.primary.record_range.end,
        sequence.ordinary[0].pair.primary.record_range.start
    );
    assert!(bridge
        .diagnostic_details()
        .iter()
        .any(|s| s.contains("primary-only, no secondary")));
    assert!(matches!(
        associate_project_120(&bytes).unwrap().candidates[0].association,
        Err(AssociationRefusal::IncompleteNeighborhood)
    ));
    assert!(crate::authority120::prove_project120(&bytes).unwrap()[0]
        .result
        .is_err());
    assert!(crate::recovery120::recover_project120(&bytes).unwrap()[0]
        .result
        .is_err());
    assert!(crate::experimental_export::build(&bytes, 0).is_err());
}

#[test]
fn primary_only_zero_prefix_preserves_real_pairs_but_refuses_unexpected_secondaries() {
    let bytes = ordinary_fixture(&[b"Track"], 3);
    assert_eq!(
        format!("{:?}", associate_semantic_project_120(&bytes).unwrap()),
        format!("{:?}", associate_project_120(&bytes).unwrap())
    );
    let mut bytes = primary_only_fixture(&[b"One", b"Two"], 4, 2);
    let root = parse_root_record_stream(&bytes).unwrap();
    let terminal = root.records.last().unwrap().record_range.start;
    let mut orphan = Vec::new();
    record(&mut orphan, 0x29, &[0; 23]);
    bytes.splice(terminal..terminal, orphan);
    assert!(
        associate_semantic_project_120(&bytes).unwrap().candidates[0]
            .association
            .is_err()
    );
    // An otherwise valid pair followed by another secondary cannot borrow that orphan.
    let mut bytes = ordinary_fixture(&[b"Track"], 3);
    let terminal = parse_root_record_stream(&bytes)
        .unwrap()
        .records
        .last()
        .unwrap()
        .record_range
        .start;
    let mut duplicate = Vec::new();
    record(&mut duplicate, 0x29, &[0; 23]);
    bytes.splice(terminal..terminal, duplicate);
    assert!(
        associate_semantic_project_120(&bytes).unwrap().candidates[0]
            .association
            .is_err()
    );
}

#[test]
fn primary_only_nonzero_prefix_malformed_header_tail_and_truncation_refuse() {
    let original = primary_only_fixture(&[b"Track"], 3, 2);
    let root = parse_root_record_stream(&original).unwrap();
    let range = root.records[6].payload.range.clone();
    for offset in [
        range.start,
        range.start + 1,
        range.start + 10,
        range.start + 11,
        range.start + 12,
        range.start + 13,
        range.end - 7,
        range.end - 3,
        range.end - 2,
        range.end - 1,
    ] {
        let mut bytes = original.clone();
        bytes[offset] ^= 1;
        assert!(
            associate_semantic_project_120(&bytes).unwrap().candidates[0]
                .association
                .is_err(),
            "offset {offset}"
        );
    }
    for size in [0, 1, 5, 10, 13, 14, 20] {
        let mut bytes = original.clone();
        let mut replacement = Vec::new();
        record(
            &mut replacement,
            2,
            &original[range.start..range.start + size],
        );
        bytes.splice(root.records[6].record_range.clone(), replacement);
        assert!(
            associate_semantic_project_120(&bytes).unwrap().candidates[0]
                .association
                .is_err(),
            "size {size}"
        );
    }
    for end in range.start..root.records[6].record_range.end {
        assert!(associate_semantic_project_120(&original[..end]).is_err());
    }
}

#[test]
fn primary_only_exact_cardinality_refuses_shortage_surplus_and_multiple_interruptions() {
    for (labels, pairs, expected) in [
        (&[b"One".as_slice(), b"Two".as_slice()][..], 3, 4),
        (&[b"One".as_slice()][..], 4, 3),
    ] {
        let bytes = primary_only_fixture(labels, pairs, 2);
        assert!(
            matches!(associate_semantic_project_120(&bytes).unwrap().candidates[0].association,
            Err(AssociationRefusal::Cardinality { slots, pairs: observed }) if slots == expected && observed == pairs)
        );
    }
    let mut bytes = primary_only_fixture(&[b"One", b"Two"], 4, 2);
    let root = parse_root_record_stream(&bytes).unwrap();
    let last_primary = root.records[7].payload.range.start;
    let last_secondary = root.records[8].record_range.clone();
    bytes[last_primary..last_primary + 2].copy_from_slice(&[0, 1]);
    bytes.drain(last_secondary);
    assert!(matches!(
        associate_semantic_project_120(&bytes).unwrap().candidates[0].association,
        Err(AssociationRefusal::IncompleteNeighborhood)
    ));
}

#[test]
fn primary_only_zero_state_is_distinct_and_specials_cannot_be_primary_only() {
    let mut bytes = primary_only_fixture(&[b"Never used", b"Owned", b"Following"], 4, 2);
    zero_state(&mut bytes, 0);
    let bridge = associate_semantic_project_120(&bytes).unwrap();
    let s = bridge.candidates[0].association.as_ref().unwrap();
    assert_eq!(s.no_event_data[0].slot.ordinal, 2);
    assert_eq!(s.primary_only[0].slot.ordinal, 3);
    assert_eq!(s.primary_only[0].local_primary_ordinal, 2);
    assert_eq!(s.ordinary[0].slot.ordinal, 4);
    // A near-zero state cannot silently skip a consuming descriptor.
    let label = 8 + 121 + 120 * 2;
    bytes[label - 39] = 1;
    assert!(matches!(
        associate_semantic_project_120(&bytes).unwrap().candidates[0].association,
        Err(AssociationRefusal::Cardinality { slots: 5, pairs: 4 })
    ));
    for ordinal in [0, 1] {
        let bytes = primary_only_fixture(&[b"Track"], 3, ordinal);
        assert!(
            associate_semantic_project_120(&bytes).unwrap().candidates[0]
                .association
                .is_err()
        );
    }
}

#[test]
fn primary_only_foreign_boundaries_and_swallowed_siblings_refuse() {
    let original = primary_only_fixture(&[b"Track"], 3, 2);
    let terminal = parse_root_record_stream(&original)
        .unwrap()
        .records
        .last()
        .unwrap()
        .record_range
        .start;
    for kind in [1, 5, 7, 9, 0x29] {
        let mut bytes = original.clone();
        bytes[terminal] = kind;
        match associate_semantic_project_120(&bytes) {
            Err(_) => {} // A malformed sibling candidate refuses at observation first.
            Ok(bridge) => assert!(bridge.candidates[0].association.is_err()),
        }
    }
    let mut bytes = original[..terminal].to_vec();
    bytes.extend_from_slice(&ordinary_fixture(&[b"Sibling"], 3)[8..]);
    let bridge = associate_semantic_project_120(&bytes).unwrap();
    assert!(bridge.candidates[0].association.is_err());
    assert!(bridge.candidates[1].association.is_ok());
}

#[test]
fn primary_only_diagnostics_do_not_publish_export_capabilities() {
    use crate::app_contract::*;
    use crate::app_service::AppService;
    let path = std::env::temp_dir().join(format!(
        "phoenix-primary-only-firewall-{}",
        std::process::id()
    ));
    std::fs::write(&path, primary_only_fixture(&[b"Track"], 3, 2)).unwrap();
    let mut service = AppService::new();
    let r = service
        .inspect_project(InspectProjectRequest {
            contract_version: CONTRACT_VERSION,
            source_path: path.to_string_lossy().into_owned(),
            diagnostics_level: DiagnosticsLevel::Full,
        })
        .unwrap();
    std::fs::remove_file(path).unwrap();
    let d = service
        .get_diagnostics(&r.session_id, DiagnosticsLevel::Full)
        .unwrap();
    assert!(d
        .technical_errors
        .iter()
        .any(|s| s.contains("primary-only, no secondary")));
    for row in &r.sequences {
        assert_ne!(row.readiness, Readiness::Ready);
        assert!(row.export_capability.is_none());
        assert!(row.bounded_export_capability.is_none());
        assert!(service
            .preview_experimental_sequence(&r.session_id, &row.sequence_id)
            .is_err());
    }
}

#[test]
#[ignore = "requires explicitly supplied authentic original and Archive COMIC BOOK outside Git"]
fn primary_only_authentic_sweeper_and_separate_shortages() {
    use sha2::{Digest, Sha256};
    let original = std::fs::read(std::env::var("PHOENIX_COMIC_ORIGINAL_SOURCE").unwrap()).unwrap();
    let archive = std::fs::read(std::env::var("PHOENIX_COMIC_ARCHIVE_SOURCE").unwrap()).unwrap();
    let cases = [
        (
            &original,
            452284,
            "22a9e2589098b0d475f6e288eeebc9303bed644aa8ce09f9971a0060c9160486",
            70358..70919,
            18,
            24,
        ),
        (
            &archive,
            406294,
            "40217bf534e7ac7f01ccab65d5d6f3fc873f08c134095410da2cbf5cef88ac70",
            70050..70611,
            19,
            23,
        ),
    ];
    for (source, size, hash, primary_range, second, primaries) in cases {
        assert_eq!(source.len(), size);
        assert_eq!(format!("{:x}", Sha256::digest(source)), hash);
        let bridge = associate_semantic_project_120(source).unwrap();
        let s = bridge.candidates[5].association.as_ref().unwrap();
        assert_eq!(s.source_name.bytes, b"sweeper");
        assert_eq!(
            s.sequence_range.start,
            if size == 452284 { 33024 } else { 32842 }
        );
        assert_eq!(s.primary_only.len(), 1);
        let multi = &s.primary_only[0];
        assert_eq!(multi.slot.ordinal, 9);
        assert_eq!(multi.local_primary_ordinal, 9);
        assert_eq!(multi.slot.label.bytes, b"multi");
        assert_eq!(multi.primary.record_range, primary_range);
        assert_eq!(multi.declared_count, 46);
        assert_eq!(
            multi.event_range,
            primary_range.start + 19..primary_range.end - 7
        );
        assert_eq!(multi.zero_prefix_field.bytes, [0; 4]);
        let pipe = s.ordinary.iter().find(|t| t.slot.ordinal == 10).unwrap();
        assert_eq!(pipe.slot.label.bytes, b"pipe");
        assert_eq!(pipe.local_pair_ordinal, 10);
        assert_eq!(pipe.pair.primary.record_range.start, primary_range.end);
        assert_eq!(
            bridge.candidates[second].observation.observed_name.bytes,
            b"sweeper #2"
        );
        assert!(matches!(bridge.candidates[second].association,
            Err(AssociationRefusal::Cardinality{slots:25,pairs}) if pairs==primaries));
        let paired = associate_project_120(source).unwrap();
        for (old, new) in paired.candidates.iter().zip(&bridge.candidates) {
            if old.association.is_ok() {
                assert_eq!(
                    format!("{:?}", old.association),
                    format!("{:?}", new.association)
                );
            }
        }
        assert!(crate::authority120::prove_project120(source).unwrap()[5]
            .result
            .is_err());
        assert!(crate::recovery120::recover_project120(source).unwrap()[5]
            .result
            .is_err());
        assert!(crate::experimental_export::build(source, 5).is_err());
    }
    assert_eq!(&original[70377..70912], &archive[70069..70604]);
}

#[test]
fn primary_only_extension_preserves_existing_paired_and_never_used_cases() {
    for marker in [[0xfe, 0xff], [0xff, 0xff]] {
        for pairs in [2, 3, 4, 5] {
            let bytes = fixture(
                &[b"Meter Track", b"Tempo Track", b"Same", b"Same"],
                pairs,
                marker,
            );
            assert_eq!(
                format!("{:?}", associate_project_120(&bytes).unwrap()),
                format!("{:?}", associate_semantic_project_120(&bytes).unwrap())
            );
        }
    }
    for unused in [0, 1, 2] {
        let mut bytes = ordinary_fixture(&[b"First", b"Unused", b"Last"], 4);
        zero_state(&mut bytes, unused);
        assert_eq!(
            format!("{:?}", associate_project_120(&bytes).unwrap()),
            format!("{:?}", associate_semantic_project_120(&bytes).unwrap())
        );
    }
    let original = ordinary_fixture(&[b"Track"], 3);
    let root = parse_root_record_stream(&original).unwrap();
    for index in [2, 3, 8] {
        let mut bytes = original.clone();
        bytes[root.records[index].record_range.start] = 9;
        assert_eq!(
            format!("{:?}", associate_project_120(&bytes).unwrap()),
            format!("{:?}", associate_semantic_project_120(&bytes).unwrap())
        );
    }
}
