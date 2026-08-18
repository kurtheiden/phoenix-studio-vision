use std::{fs, ops::Range};

use phoenix::tempo::{decode_bounded_initial_tempo, InitialTempoBounds, InitialTempoEvent};

const BASELINE: &str = "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline";
const CONTROLLED_120: &str = "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 002 - Save As with no edits/newest STUFF no edits";
const CONTROLLED_130: &str = "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 004 - Change one Tempo value/newest STUFF tempo to 130";

struct Expected<'a> {
    path: &'a str,
    range: Range<usize>,
    bytes: [u8; 7],
    mpqn: u32,
}

fn assert_authentic(expected: Expected<'_>) -> InitialTempoEvent {
    let bytes = fs::read(expected.path).expect("authentic Tempo fixture should be available");
    assert_eq!(&bytes[expected.range.clone()], expected.bytes);

    let event = decode_bounded_initial_tempo(
        &bytes,
        InitialTempoBounds {
            event_range: expected.range.clone(),
        },
    )
    .unwrap();

    assert_eq!(event.event_range, expected.range);
    let located = [
        event.initial_position_byte,
        event.ff_tag,
        event.tempo_tag,
        event.payload_length,
        event.mpqn_byte_0,
        event.mpqn_byte_1,
        event.mpqn_byte_2,
    ];
    for (index, field) in located.into_iter().enumerate() {
        assert_eq!(field.value, expected.bytes[index]);
        assert_eq!(field.offset, event.event_range.start + index);
    }
    assert_eq!(event.mpqn(), expected.mpqn);
    assert_eq!(event.mpqn_byte_2.offset + 1, event.event_range.end);
    event
}

#[test]
fn decodes_natural_bells_initial_tempo_with_complete_provenance() {
    let event = assert_authentic(Expected {
        path: BASELINE,
        range: 0x0000ebd8..0x0000ebdf,
        bytes: [0x00, 0xff, 0x51, 0x03, 0x09, 0x10, 0x8b],
        mpqn: 594_059,
    });
    let bpm = event.bpm().unwrap();
    assert!((bpm - 101.000_069_016_713_82).abs() < 1e-12);
}

#[test]
fn decodes_controlled_120_bpm_initial_tempo_with_complete_provenance() {
    let event = assert_authentic(Expected {
        path: CONTROLLED_120,
        range: 0x0002f7dc..0x0002f7e3,
        bytes: [0x00, 0xff, 0x51, 0x03, 0x07, 0xa1, 0x20],
        mpqn: 500_000,
    });
    assert_eq!(event.bpm(), Some(120.0));
}

#[test]
fn decodes_controlled_130_bpm_initial_tempo_with_complete_provenance() {
    let event = assert_authentic(Expected {
        path: CONTROLLED_130,
        range: 0x0002f7dc..0x0002f7e3,
        bytes: [0x00, 0xff, 0x51, 0x03, 0x07, 0x0a, 0xe2],
        mpqn: 461_538,
    });
    let bpm = event.bpm().unwrap();
    assert!((bpm - 130.000_130_000_13).abs() < 1e-12);
}
