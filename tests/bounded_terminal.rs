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
        &[255, 254][..],
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
