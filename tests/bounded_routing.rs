//! Synthetic boundary fixtures are generated below. External files used by
//! the separate evidence tests are authentic controlled saves, never fabricated.
use phoenix::bounded_routing::{
    decode_bounded_routing, AssociationForm, BoundedRoutingResolution, ContextKind, RoutingRefusal,
};
use phoenix::sequence_container::parse_project_166;

const BASELINE: &str = "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline";
const NOTE: &[u8] = &[0, 0x90, 60, 64, 32, 1];

fn record(bytes: &mut Vec<u8>, tag: u8, payload: &[u8]) {
    bytes.push(tag);
    bytes.extend((payload.len() as u32).to_be_bytes());
    bytes.extend(payload);
}

fn synthetic(count: usize, events: &[u8], f: bool) -> Vec<u8> {
    let mut b = vec![0; 8];
    for i in 0..count {
        let mut p = vec![0; 36];
        p[25] = i as u8;
        p[26] = 7;
        p[27] = 15;
        record(&mut b, 0x10, &p);
    }
    let mut device = vec![0; 40];
    device[0] = 4;
    device[1..5].copy_from_slice(b"Test");
    device[33] = 7;
    record(&mut b, 0x2a, &device);
    let start = b.len();
    let mut p = vec![0; 3 * 166 + 172];
    p[0] = 3;
    record(&mut b, 1, &p);
    let label = start + 208 + 2 * 166 + 15;
    b[label..label + 5].copy_from_slice(b"Track");
    b[label - 33] = 1;
    b[label - 31] = 1;
    b[label - 30..label].fill(if f { 0xff } else { 0 });
    let mut name = vec![0; 11];
    name.extend([4, b'T', b'e', b's', b't']);
    record(&mut b, 7, &name);
    record(&mut b, 2, &[0; 22]);
    record(&mut b, 0x29, &[]);
    record(&mut b, 2, &[0; 21]);
    record(&mut b, 0x29, &[]);
    let mut track = vec![0; 14];
    track.extend(events);
    track.extend([0xff, 0, 0, 0, 0xff, 0x2f, 0]);
    record(&mut b, 2, &track);
    record(&mut b, 0x29, &[]);
    record(&mut b, 0, &[]);
    b
}
fn result(b: &[u8]) -> Result<BoundedRoutingResolution, RoutingRefusal> {
    decode_bounded_routing(b).unwrap().remove(0).result
}
fn label(b: &[u8]) -> usize {
    parse_project_166(b).unwrap().sequences[0].descriptors[2].label_start
}
fn payload(b: &[u8], tag: u8, n: usize) -> usize {
    phoenix::sequence_container::parse_root_record_stream(b)
        .unwrap()
        .records
        .iter()
        .filter(|r| r.record_type.value == tag)
        .nth(n)
        .unwrap()
        .payload
        .range
        .start
}

#[test]
fn synthetic_f_z_accept_and_preserve_provenance() {
    for f in [true, false] {
        let b = synthetic(3, NOTE, f);
        let r = result(&b).unwrap();
        assert_eq!(
            r.association_form,
            if f {
                AssociationForm::F
            } else {
                AssociationForm::Z
            }
        );
        assert_eq!(r.first_slot, [0, 1]);
        assert_eq!(r.instrument_candidate, 1);
        assert_eq!(
            (
                r.h1_position,
                r.h2_position,
                r.selected_25,
                r.selected_26,
                r.selected_27,
                r.midi_channel
            ),
            (1, 1, 1, 7, 15, 16)
        );
        assert!(r.table_wide_position_agreement);
        assert_eq!(r.table_count, 3);
        assert_eq!(r.type10_payload_range.start, payload(&b, 0x10, 1));
        assert_eq!(
            r.type10_record_range.start + 5,
            r.type10_payload_range.start
        );
        assert_eq!(&b[r.association_range.clone()][..3], &[1, 0, 1]);
        assert_eq!(&b[r.label_range.clone()], b"Track");
        assert_eq!(r.label_bytes, b"Track");
        assert_eq!(r.event_range, r.consumed_range);
        assert_eq!(r.device_name, b"Test");
        assert_eq!(r.type2a_record_range.start + 5, payload(&b, 0x2a, 0));
        assert_eq!(r.pair_ordinal, 0);
        assert!(r.validated_contexts.is_empty());
        let rows = decode_bounded_routing(&b).unwrap();
        assert_eq!(
            (rows[0].structural_ordinal, rows[0].descriptor_ordinal),
            (0, 2)
        );
    }
}

#[test]
fn synthetic_association_refusals() {
    for (back, value) in [(33, 0), (33, 2), (32, 1), (32, 0x70), (30, 0), (29, 2)] {
        let mut b = synthetic(3, NOTE, true);
        let l = label(&b);
        b[l - back] = value;
        assert_eq!(result(&b), Err(RoutingRefusal::UnsupportedAssociation));
    }
    for i in [0, 3, 255] {
        let mut b = synthetic(3, NOTE, false);
        let l = label(&b);
        b[l - 31] = i;
        assert_eq!(result(&b), Err(RoutingRefusal::UnsupportedNumericScope));
    }
    assert_eq!(
        result(&synthetic(100, NOTE, true)),
        Err(RoutingRefusal::UnsupportedNumericScope)
    );
    // Synthetic seven-Instrument shape from the reported counterexample.
    let mut b = synthetic(12, NOTE, false);
    let l = label(&b);
    b[l - 33] = 7;
    for n in 0..7 {
        b[l - 31 + 2 * n] = 4 + n as u8;
    }
    assert_eq!(result(&b), Err(RoutingRefusal::UnsupportedAssociation));
    // Even with prefix 01, residual and multiple associations are refused.
    b[l - 33] = 1;
    assert_eq!(result(&b), Err(RoutingRefusal::UnsupportedAssociation));
}

#[test]
fn synthetic_table_device_channel_refusals() {
    for (record_index, offset, value, expected) in [
        (1, 25, 2, RoutingRefusal::Type10AgreementFailure),
        (0, 25, 1, RoutingRefusal::AmbiguousType10Lookup),
        (2, 25, 9, RoutingRefusal::Type10AgreementFailure),
        (1, 27, 16, RoutingRefusal::InvalidChannel),
        (1, 26, 8, RoutingRefusal::MissingDevice),
    ] {
        let mut b = synthetic(3, NOTE, true);
        let p = payload(&b, 0x10, record_index);
        b[p + offset] = value;
        assert_eq!(result(&b), Err(expected));
    }
    let mut b = synthetic(3, NOTE, true);
    let p = payload(&b, 0x10, 1);
    let q = payload(&b, 0x10, 2);
    b[p + 25] = 2;
    b[q + 25] = 1;
    assert_eq!(result(&b), Err(RoutingRefusal::Type10AgreementFailure));
    let mut b = synthetic(3, NOTE, true);
    let p = payload(&b, 0x10, 1);
    b[p - 4..p].copy_from_slice(&35u32.to_be_bytes());
    b.remove(p + 35);
    assert_eq!(result(&b), Err(RoutingRefusal::MalformedType10Table));
    let mut b = synthetic(3, NOTE, true);
    let p = payload(&b, 0x10, 1);
    b[p - 4..p].copy_from_slice(&u32::MAX.to_be_bytes());
    assert!(decode_bounded_routing(&b).is_err());
    let mut b = synthetic(3, NOTE, true);
    let p = payload(&b, 0x2a, 0);
    let copy = b[p - 5..p + 40].to_vec();
    b.splice(p + 40..p + 40, copy);
    assert_eq!(result(&b), Err(RoutingRefusal::AmbiguousDevice));
    for n in [0, 33, 255] {
        let mut b = synthetic(3, NOTE, true);
        let p = payload(&b, 0x2a, 0);
        b[p] = n;
        assert_eq!(result(&b), Err(RoutingRefusal::MalformedDeviceTable));
    }
}

#[test]
fn synthetic_events_ownership_and_context_refusals() {
    assert_eq!(
        result(&synthetic(3, &[], true)),
        Err(RoutingRefusal::EmptyEventRegion)
    );
    assert!(matches!(
        result(&synthetic(3, &[0, 0xff, 0x99], true)),
        Err(RoutingRefusal::IncompleteEventWalk(_))
    ));
    let mut b = synthetic(3, NOTE, true);
    let l = label(&b);
    b[l] = 0;
    assert_eq!(result(&b), Err(RoutingRefusal::BlankOrUnboundedLabel));
    let mut b = synthetic(3, NOTE, true);
    let p = payload(&b, 0x29, 2);
    b.drain(p - 5..p);
    let p = payload(&b, 2, 2);
    let end = p + 14 + NOTE.len() + 7;
    b.drain(p - 5..end);
    assert_eq!(result(&b), Err(RoutingRefusal::AmbiguousOwnership));
    let cc = vec![0, 0xff, 0x41, 5, 0, 1, 0, 7, 64];
    let patch = vec![0, 0xff, 0x7c, 7, 0, 0, 0, 0, 0, 0, 25];
    let mut ff60 = vec![0, 0xff, 0x60, 7, 0x57, 0x7f, 0, 0, 0, 0, 0];
    ff60.extend(NOTE);
    for (event, kind, offset) in [
        (cc, ContextKind::Controller, 5),
        (patch, ContextKind::Patch, 4),
        (ff60, ContextKind::Ff60, 4),
    ] {
        let b = synthetic(3, &event, true);
        let r = result(&b).unwrap();
        assert_eq!(r.validated_contexts.len(), 1);
        assert_eq!(r.validated_contexts[0].kind, kind);
        assert_eq!(
            &b[r.validated_contexts[0].range.clone()],
            r.validated_contexts[0].bytes
        );
        let mut changed = event;
        changed[offset] = 2;
        assert!(
            matches!(result(&synthetic(3,&changed,true)),Err(RoutingRefusal::ConflictingContext{kind:k,..}) if k==kind)
        );
    }
}

#[test]
fn controlled_exp33_exp34_target_relationships() {
    for (experiment, file, i, device, channel) in [
        (
            "Experiment 033 - Track 2 Instrument JV-1080-2 to JV-1080-3",
            "EXP33 CTRL",
            5,
            12,
            2,
        ),
        (
            "Experiment 033 - Track 2 Instrument JV-1080-2 to JV-1080-3",
            "EXP33 EDIT",
            6,
            12,
            3,
        ),
        (
            "Experiment 034 - Track 2 Instrument JV-1080-2 to Juno-106",
            "EXP34 CTRL",
            5,
            12,
            2,
        ),
        (
            "Experiment 034 - Track 2 Instrument JV-1080-2 to Juno-106",
            "EXP34 EDIT",
            3,
            11,
            1,
        ),
    ] {
        let path=format!("/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/{experiment}/Returned from MacOS9/{file}");
        let b = std::fs::read(path).expect("authentic controlled save required");
        let rows = decode_bounded_routing(&b).unwrap();
        let r = rows
            .iter()
            .find(|r| r.structural_ordinal == 14 && r.descriptor_ordinal == 3)
            .unwrap()
            .result
            .as_ref()
            .unwrap();
        assert_eq!(r.label_bytes, b"Track 2");
        assert_eq!(r.instrument_candidate, i);
        assert_eq!((r.h1_position, r.h2_position), (i as usize, i as usize));
        assert_eq!(
            (r.selected_26, r.selected_27, r.midi_channel),
            (device, channel - 1, channel)
        );
        println!("{file}: candidate {i}, device {device}, channel {channel}");
    }
}

#[test]
fn authenticated_corpus_predicate_results() {
    let b = std::fs::read(BASELINE).expect("authenticated baseline required");
    let p = parse_project_166(&b).unwrap();
    let rows = decode_bounded_routing(&b).unwrap();
    let expected = [
        "Situation",
        "Sequence D",
        "Sequence E",
        "mission impossibl",
        "Renaissance",
        "Get on up & Dance",
        "Jurrasic Park",
        "Sequence R",
    ];
    let expected_counts = [
        (11, 0, 2),
        (12, 2, 0),
        (6, 0, 0),
        (5, 1, 0),
        (6, 2, 0),
        (3, 0, 0),
        (10, 0, 0),
        (7, 2, 2),
        (0, 0, 11),
        (0, 1, 4),
        (1, 1, 0),
        (4, 2, 0),
        (15, 1, 0),
        (5, 2, 0),
        (9, 0, 0),
        (3, 0, 0),
        (1, 0, 0),
        (2, 0, 0),
    ];
    assert_eq!(p.sequences.len(), expected_counts.len());
    let mut matched_expected = 0;
    let mut forms = [0usize; 2];
    for (s, seq) in p.sequences.iter().enumerate() {
        let name = seq.sequence_name.as_utf8().unwrap();
        let tracks: Vec<_> = rows.iter().filter(|r| r.structural_ordinal == s).collect();
        let mut accepted = 0;
        let mut empty = 0;
        let mut refused = 0;
        for row in tracks {
            match &row.result {
                Ok(r) => {
                    accepted += 1;
                    forms[usize::from(r.association_form == AssociationForm::Z)] += 1;
                    println!(
                        "{name}|d{}|{:?}|i{}|ch{}",
                        row.descriptor_ordinal,
                        r.association_form,
                        r.instrument_candidate,
                        r.midi_channel
                    );
                }
                Err(RoutingRefusal::EmptyEventRegion) => empty += 1,
                Err(e) => {
                    refused += 1;
                    println!("{name}|d{}|REFUSED {e:?}", row.descriptor_ordinal);
                }
            }
        }
        println!("SEQUENCE {name}: accepted={accepted} empty={empty} refused={refused}");
        assert_eq!((accepted, empty, refused), expected_counts[s], "{name}");
        if expected.contains(&name) {
            matched_expected += 1;
            assert!(accepted > 0);
            assert_eq!(refused, 0, "{name}");
        }
        if ["xForm", "happyone"].contains(&name) {
            assert!(rows.iter().any(|r| r.structural_ordinal == s
                && matches!(r.result, Err(RoutingRefusal::IncompleteEventWalk(_)))));
        }
        if name == "Sequence I" {
            assert!(rows
                .iter()
                .filter(|r| r.structural_ordinal == s)
                .all(|r| r.result == Err(RoutingRefusal::AmbiguousOwnership)));
        }
        if name == "newsong" {
            assert!(rows.iter().any(|r| r.structural_ordinal == s
                && r.result == Err(RoutingRefusal::UnsupportedAssociation)));
        }
    }
    assert_eq!(matched_expected, 8);
    assert_eq!(forms, [83, 17]);
    println!("Authenticated F={} Z={}", forms[0], forms[1]);
}

#[test]
fn authentic_seven_instrument_and_audio_counterexamples_are_refused() {
    let path = "/Users/kurtheiden/Documents/Phoenix Research/Recovered Audio Projects/Project 001/repaired project/chris stuff with audio";
    let b = std::fs::read(path).expect("authentic recovered project required");
    let p = parse_project_166(&b).unwrap();
    let rows = decode_bounded_routing(&b).unwrap();
    let mut seven = 0;
    let mut audio = 0;
    for row in &rows {
        let descriptor = &p.sequences[row.structural_ordinal].descriptors[row.descriptor_ordinal];
        let l = descriptor.label_start;
        if b[l - 33] == 7 && b[l - 32..l - 18] == [0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9, 0, 10] {
            seven += 1;
            // The public decoder refuses even earlier: this authentic project
            // has unresolved descriptor/pair ownership. Synthetic tests above
            // isolate the seven-slot association rejection itself.
            assert_eq!(row.result, Err(RoutingRefusal::AmbiguousOwnership));
            println!(
                "Authentic seven-Instrument: sequence {} d{} at {:#x} refused",
                row.structural_ordinal,
                row.descriptor_ordinal,
                l - 33
            );
        }
        if b[l - 32] == 0x70 {
            audio += 1;
            assert!(row.result.is_err());
            println!(
                "Authentic audio: sequence {} d{}: {:?}",
                row.structural_ordinal, row.descriptor_ordinal, row.result
            );
        }
    }
    assert!(seven > 0 && audio > 0);
}

#[test]
fn bounded_success_leaves_existing_readiness_and_profile_evidence_unchanged() {
    use phoenix::app_contract::{
        DiagnosticsLevel, InspectProjectRequest, Readiness, CONTRACT_VERSION,
    };
    use phoenix::app_service::AppService;
    let mut service = AppService::new();
    let before = service
        .inspect_project(InspectProjectRequest {
            contract_version: CONTRACT_VERSION,
            source_path: BASELINE.into(),
            diagnostics_level: DiagnosticsLevel::Full,
        })
        .unwrap();
    let profile_before = service.profile_evidence(&before.session_id).unwrap();
    let provisional_before = service
        .routing_channel_resolutions(&before.session_id)
        .unwrap();
    let rows = decode_bounded_routing(&std::fs::read(BASELINE).unwrap()).unwrap();
    assert_eq!(rows.iter().filter(|r| r.result.is_ok()).count(), 100);
    assert_eq!(
        before
            .sequences
            .iter()
            .filter(|s| s.readiness == Readiness::Ready && s.export_capability.is_some())
            .count(),
        6
    );
    assert_eq!(
        before
            .sequences
            .iter()
            .filter(|s| s.export_capability.is_none())
            .count(),
        12
    );
    assert_eq!(service.get_inspection(&before.session_id).unwrap(), before);
    assert_eq!(
        service.profile_evidence(&before.session_id).unwrap(),
        profile_before
    );
    assert_eq!(
        service
            .routing_channel_resolutions(&before.session_id)
            .unwrap(),
        provisional_before
    );
    assert!(profile_before
        .sequences
        .iter()
        .flat_map(|s| &s.tracks)
        .all(|t| t.observed_channel.is_none()));
}

#[test]
fn synthetic_numeric_endpoints_and_unmeasurable_bounds() {
    let mut b = synthetic(99, NOTE, false);
    let l = label(&b);
    b[l - 31] = 98;
    let p = payload(&b, 0x10, 98);
    b[p + 27] = 0;
    let r = result(&b).unwrap();
    assert_eq!(
        (r.table_count, r.instrument_candidate, r.midi_channel),
        (99, 98, 1)
    );
    for count in [0, 1] {
        assert_eq!(
            result(&synthetic(count, NOTE, true)),
            Err(RoutingRefusal::UnsupportedNumericScope)
        );
    }
    let mut b = synthetic(3, NOTE, true);
    let p = payload(&b, 0x2a, 0);
    b[p - 4..p].copy_from_slice(&33u32.to_be_bytes());
    b.drain(p + 33..p + 40);
    assert_eq!(result(&b), Err(RoutingRefusal::MalformedDeviceTable));
    let mut b = synthetic(3, NOTE, true);
    let l = label(&b);
    b[l..l + 5].fill(b' ');
    assert_eq!(result(&b), Err(RoutingRefusal::BlankOrUnboundedLabel));
    let mut b = synthetic(3, NOTE, true);
    let p = payload(&b, 2, 2);
    b[p + 14 + NOTE.len()] = 0;
    assert!(matches!(
        result(&b),
        Err(RoutingRefusal::InvalidEventBounds(_))
    ));
}
