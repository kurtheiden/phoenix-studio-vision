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
fn service_diagnostics_cannot_create_sequence_or_export_authority() {
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
    assert!(result.sequences.is_empty());
    assert_eq!(result.project.overall_readiness, Readiness::Unknown);
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
    assert_eq!(accepted, 8);
    assert_eq!(associated, 103);
    assert_eq!(matched, 102);
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
