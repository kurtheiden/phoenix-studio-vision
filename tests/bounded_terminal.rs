use phoenix::bounded_terminal::*;
use phoenix::smf::*;
use std::ops::Range;

fn record(b: &mut Vec<u8>, kind: u8, payload: &[u8]) -> Range<usize> {
    let start = b.len();
    b.push(kind);
    b.extend((payload.len() as u32).to_be_bytes());
    b.extend(payload);
    start..b.len()
}
fn fixture(terminal: &[u8]) -> (Vec<u8>, Range<usize>, Vec<Range<usize>>) {
    let mut b = vec![0; 8];
    let start = b.len();
    record(&mut b, 1, &[]);
    record(&mut b, 7, &[]);
    let mut ranges = Vec::new();
    for _ in 0..3 {
        ranges.push(record(&mut b, 2, &[0; 20]));
        ranges.push(record(&mut b, 0x29, &[]));
    }
    record(&mut b, 0, terminal);
    let end = b.len();
    (b, start..end, ranges)
}
#[test]
fn isolated_external_terminal_material_retains_diagnostic_and_source() {
    let (b, seq, ranges) = fixture(&[255, 255, 254, 255, 255]);
    let d = assess_terminal_coverage(&b, seq.clone(), &ranges)
        .unwrap()
        .unwrap();
    assert_eq!(d.record_range.end, seq.end);
    assert_eq!(d.payload_range, seq.end - 5..seq.end);
    assert_eq!(d.source_bytes, vec![255, 255, 254, 255, 255]);
    assert_eq!(
        d.message(),
        "Uninterpreted terminal material remains outside recovered MIDI event data."
    );
}
#[test]
fn all_ff_and_diagnostic_do_not_change_midi_or_counts() {
    let events = vec![ScheduledEvent {
        absolute_tick: 0,
        stable_ordinal: 0,
        message: ChannelMessage::ControlChange {
            channel: MidiChannel::new(2).unwrap(),
            controller: MidiDataByte::new(1).unwrap(),
            value: MidiDataByte::new(70).unwrap(),
        },
    }];
    let mut outputs = Vec::new();
    for t in [
        &[255, 255, 255, 255, 255][..],
        &[255, 255, 254, 255, 255][..],
    ] {
        let (b, seq, ranges) = fixture(t);
        let diagnostic = assess_terminal_coverage(&b, seq, &ranges).unwrap();
        assert_eq!(diagnostic.is_some(), t.contains(&254));
        outputs.push(
            serialize_named_musical_track_with_ordering(
                b"synthetic",
                &events,
                MusicalTrackOrdering::SourceOrder,
            )
            .unwrap(),
        );
        assert_eq!(events.len(), 1);
    }
    assert_eq!(outputs[0], outputs[1]);
}
#[test]
fn incomplete_overlapping_or_ambiguous_coverage_refuses() {
    for mutation in 0..4 {
        let (b, seq, mut ranges) = fixture(&[255, 254, 255]);
        match mutation {
            0 => {
                ranges.pop();
            }
            1 => {
                ranges[1].start = ranges[0].start;
            }
            2 => {
                ranges.swap(0, 2);
            }
            3 => {
                ranges.push(ranges[0].clone());
            }
            _ => unreachable!(),
        }
        assert_eq!(
            assess_terminal_coverage(&b, seq, &ranges),
            Err(TerminalRefusal::IncompleteCoverage)
        );
    }
}
#[test]
fn interrupting_or_unrecognized_event_record_refuses() {
    for kind in [9, 2, 0x40] {
        let (mut b, seq, ranges) = fixture(&[255, 254, 255]);
        b[ranges[3].start] = kind;
        assert_eq!(
            assess_terminal_coverage(&b, seq, &ranges),
            Err(TerminalRefusal::Neighborhood)
        );
    }
}
#[test]
fn possibly_musical_and_unevidenced_terminal_content_refuses() {
    for t in [
        &[0, 0x90, 60, 100][..],
        &[0, 255, 0x40, 1][..],
        &[255, 254, 254, 255][..],
        &[254, 255][..],
        &[][..],
    ] {
        let (b, seq, ranges) = fixture(t);
        assert_eq!(
            assess_terminal_coverage(&b, seq, &ranges),
            Err(TerminalRefusal::UnsupportedMaterial)
        );
    }
}
#[test]
fn framing_and_event_region_as_terminal_refuse() {
    let (b, seq, ranges) = fixture(&[255, 254, 255]);
    assert_eq!(
        assess_terminal_coverage(&b, seq.start..ranges[3].end, &ranges),
        Err(TerminalRefusal::Neighborhood)
    );
    let mut bad = b;
    bad.pop();
    assert_eq!(
        assess_terminal_coverage(&bad, seq, &ranges),
        Err(TerminalRefusal::Framing)
    );
}

#[test]
fn prefixed_and_final_fe_variants_retain_exact_non_event_provenance() {
    for payload in [
        vec![0, 0, 255, 254, 255],
        vec![0, 0, 255, 255, 255, 254, 255, 255],
        vec![255, 254],
        vec![255, 255, 255, 254],
    ] {
        let (b, seq, ranges) = fixture(&payload);
        let d = assess_terminal_coverage(&b, seq.clone(), &ranges)
            .unwrap()
            .unwrap();
        assert_eq!(
            d.classification(),
            "UNINTERPRETED NON-EVENT TERMINAL MATERIAL"
        );
        assert_eq!(d.source_bytes, payload);
        assert_eq!(d.payload_range, seq.end - payload.len()..seq.end);
        assert_eq!(d.record_range, d.payload_range.start - 5..seq.end);
        assert_eq!(&b[d.payload_range.clone()], d.source_bytes);
        assert_eq!(ranges.last().unwrap().end, d.record_range.start);
    }
}

#[test]
fn new_terminal_variants_require_complete_non_overlapping_pair_coverage() {
    for payload in [&[0, 0, 255, 254, 255][..], &[255, 255, 254][..]] {
        let (b, seq, ranges) = fixture(payload);
        let terminal_start = ranges.last().unwrap().end;
        for mutation in 0..5 {
            let mut bad = ranges.clone();
            match mutation {
                0 => {
                    bad.pop();
                }
                1 => {
                    bad[0].end = terminal_start + 5 + payload.len();
                }
                2 => {
                    bad.push(terminal_start + 5..seq.end);
                }
                3 => {
                    bad.swap(0, 2);
                }
                4 => {
                    bad[1] = bad[0].clone();
                }
                _ => unreachable!(),
            }
            assert_eq!(
                assess_terminal_coverage(&b, seq.clone(), &bad),
                Err(TerminalRefusal::IncompleteCoverage)
            );
        }
    }
}

#[test]
fn arbitrary_prefixes_suffixes_and_multiple_deviations_remain_refused() {
    for payload in [
        vec![],
        vec![0, 255, 254, 255],
        vec![0, 0, 0, 255, 254, 255],
        vec![0, 0, 255, 254],
        vec![0, 0, 255, 255],
        vec![0, 0, 254, 255],
        vec![254],
        vec![254, 255],
        vec![255, 254, 254],
        vec![0, 0, 255, 254, 255, 254, 255],
        vec![255, 254, 255, 0],
        vec![255, 254, 255, 1],
        vec![255, 253, 255],
    ] {
        let (b, seq, ranges) = fixture(&payload);
        assert_eq!(
            assess_terminal_coverage(&b, seq, &ranges),
            Err(TerminalRefusal::UnsupportedMaterial),
            "{payload:?}"
        );
    }
}

#[test]
fn supported_event_material_inside_terminal_is_never_diagnostic() {
    let events = [
        vec![0, 0x90, 60, 100, 64, 120], // Explicit Note.
        vec![0, 0xb2, 7, 100],           // Source-channel Controller.
        vec![0, 0xe0, 0, 64],
        vec![0, 0xd0, 64],
        vec![0, 255, 0x51, 3, 7, 161, 32],
        vec![0, 255, 0x58, 4, 4, 2, 8, 8],
        vec![0, 255, 0x59, 2, 0, 0],
        vec![0, 255, 0x7c, 1, 0],
        vec![0, 255, 0x60, 7],
        vec![0, 255, 0x40, 1, 0],
    ];
    for event in events {
        for original in [vec![255, 254], vec![0, 0, 255, 254, 255]] {
            // Try both placement before the family and inside its ff body.
            for position in [0, original.len() - 1] {
                let mut payload = original.clone();
                payload.splice(position..position, event.iter().copied());
                let (b, seq, ranges) = fixture(&payload);
                assert_eq!(
                    assess_terminal_coverage(&b, seq, &ranges),
                    Err(TerminalRefusal::UnsupportedMaterial)
                );
            }
        }
    }
}

#[test]
fn new_forms_refuse_bad_framing_placement_and_unrecovered_extra_pairs() {
    for payload in [&[0, 0, 255, 254, 255][..], &[255, 255, 254][..]] {
        let (b, seq, ranges) = fixture(payload);
        let terminal_start = ranges.last().unwrap().end;
        for n in terminal_start..b.len() {
            assert!(assess_terminal_coverage(&b[..n], seq.clone(), &ranges).is_err());
        }
        for kind in [2, 0x29, 7, 1] {
            let mut bad = b.clone();
            bad[terminal_start] = kind;
            assert_eq!(
                assess_terminal_coverage(&bad, seq.clone(), &ranges),
                Err(TerminalRefusal::Neighborhood)
            );
        }
        let mut bad = b.clone();
        bad[terminal_start + 4] += 1;
        assert_eq!(
            assess_terminal_coverage(&bad, seq.clone(), &ranges),
            Err(TerminalRefusal::Framing)
        );
        let mut extra = Vec::new();
        record(&mut extra, 2, &[0; 20]);
        record(&mut extra, 0x29, &[]);
        let mut bad = b.clone();
        bad.splice(terminal_start..terminal_start, extra.iter().copied());
        assert_eq!(
            assess_terminal_coverage(&bad, seq.start..seq.end + extra.len(), &ranges),
            Err(TerminalRefusal::IncompleteCoverage)
        );
        assert!(assess_terminal_coverage(&b, ranges[0].start..seq.end, &ranges).is_err());
        let mut bad = b;
        bad.extend([0x90, 60, 100]);
        assert_eq!(
            assess_terminal_coverage(&bad, seq, &ranges),
            Err(TerminalRefusal::Framing)
        );
    }
}

#[test]
fn ordinary_track_zero_terminal_remains_refused_independently() {
    use phoenix::sequence_container::{
        parse_root_record_stream, TrackEventBoundsError, TrackRecordPair,
    };
    let mut bytes = vec![0; 8];
    let mut primary = vec![0; 14];
    primary.extend([0, 0x90, 60, 100, 64, 120]);
    primary.extend([0; 7]);
    record(&mut bytes, 2, &primary);
    record(&mut bytes, 0x29, &[]);
    let root = parse_root_record_stream(&bytes).unwrap();
    let p = root.records[0].clone();
    let start = p.payload.range.start + 14;
    let pair = TrackRecordPair {
        pair_ordinal: 0,
        candidate_event_start: start,
        event_containing_range: start..p.payload.range.end,
        primary: p,
        secondary: root.records[1].clone(),
    };
    assert!(matches!(
        pair.validated_event_bounds(),
        Err(TrackEventBoundsError::InvalidTerminalGrammar { .. })
    ));
}
