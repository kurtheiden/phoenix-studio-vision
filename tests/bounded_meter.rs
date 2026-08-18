use std::{fs, ops::Range};

use phoenix::meter::{decode_bounded_initial_meter, InitialMeterBounds, InitialMeterEvent};

const BASELINE: &str = "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline";
const CONTROLLED_7_8: &str = "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 030 - Change initial Meter from 4-4 to 7-8/newest STUFF meter 7-8 EXP30";

struct Expected<'a> {
    path: &'a str,
    range: Range<usize>,
    bytes: [u8; 8],
    denominator: u64,
}

fn assert_authentic(expected: Expected<'_>) -> InitialMeterEvent {
    let bytes = fs::read(expected.path).expect("authentic Meter fixture should be available");
    assert_eq!(&bytes[expected.range.clone()], expected.bytes);

    let event = decode_bounded_initial_meter(
        &bytes,
        InitialMeterBounds {
            event_range: expected.range.clone(),
        },
    )
    .unwrap();

    assert_eq!(event.event_range, expected.range);
    let located = [
        event.initial_position_byte,
        event.ff_tag,
        event.meter_tag,
        event.payload_length,
        event.numerator,
        event.denominator_exponent,
        event.third_payload,
        event.fourth_payload,
    ];
    for (index, field) in located.into_iter().enumerate() {
        assert_eq!(field.value, expected.bytes[index]);
        assert_eq!(field.offset, event.event_range.start + index);
    }
    assert_eq!(event.numerator.value, expected.bytes[4]);
    assert_eq!(event.denominator_exponent.value, expected.bytes[5]);
    assert_eq!(event.denominator(), Some(expected.denominator));
    assert_eq!(event.third_payload.value, expected.bytes[6]);
    assert_eq!(event.fourth_payload.value, expected.bytes[7]);
    assert_eq!(event.fourth_payload.offset + 1, event.event_range.end);
    event
}

#[test]
fn decodes_natural_bells_4_4_with_complete_provenance() {
    assert_authentic(Expected {
        path: BASELINE,
        range: 0x0000eb80..0x0000eb88,
        bytes: [0x00, 0xff, 0x58, 0x04, 0x04, 0x02, 0x08, 0x08],
        denominator: 4,
    });
}

#[test]
fn decodes_natural_sequence_k_6_8_with_complete_provenance() {
    assert_authentic(Expected {
        path: BASELINE,
        range: 0x000258df..0x000258e7,
        bytes: [0x00, 0xff, 0x58, 0x04, 0x06, 0x03, 0x06, 0x08],
        denominator: 8,
    });
}

#[test]
fn decodes_controlled_bells_7_8_with_complete_provenance() {
    assert_authentic(Expected {
        path: CONTROLLED_7_8,
        range: 0x0000eb80..0x0000eb88,
        bytes: [0x00, 0xff, 0x58, 0x04, 0x07, 0x03, 0x06, 0x08],
        denominator: 8,
    });
}

#[test]
fn decodes_natural_mission_impossibl_10_8_with_complete_provenance() {
    assert_authentic(Expected {
        path: BASELINE,
        range: 0x0001c864..0x0001c86c,
        bytes: [0x00, 0xff, 0x58, 0x04, 0x0a, 0x03, 0x06, 0x08],
        denominator: 8,
    });
}
