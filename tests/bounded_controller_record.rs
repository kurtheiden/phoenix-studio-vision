use phoenix::controller::{decode_bounded_controller_record, ControllerRecordBounds};
use std::fs;
use std::ops::Range;

const BASELINE: &str = "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline";

struct Expected<'a> {
    range: Range<usize>,
    timing: u32,
    timing_width: usize,
    context: &'a [u8],
    number: u8,
    value: u8,
}

fn assert_authentic(bytes: &[u8], expected: Expected<'_>) {
    let record = decode_bounded_controller_record(
        bytes,
        ControllerRecordBounds {
            record_range: expected.range.clone(),
        },
    )
    .unwrap();

    let timing_end = expected.range.start + expected.timing_width;
    assert_eq!(record.record_range, expected.range);
    assert_eq!(record.timing_delta.value, expected.timing);
    assert_eq!(
        record.timing_delta.range,
        record.record_range.start..timing_end
    );
    assert_eq!(record.timing_delta.bytes.len(), expected.timing_width);
    assert_eq!(record.event_tag_range, timing_end..timing_end + 2);
    assert_eq!(record.payload_length.value, 5);
    assert_eq!(record.payload_length.offset, timing_end + 2);
    assert_eq!(
        record.payload_range,
        timing_end + 3..record.record_range.end
    );
    assert_eq!(record.context.bytes, expected.context);
    assert_eq!(record.context.range, timing_end + 3..timing_end + 6);
    assert_eq!(record.controller_number.value, expected.number);
    assert_eq!(record.controller_number.offset, timing_end + 6);
    assert_eq!(record.controller_value.value, expected.value);
    assert_eq!(record.controller_value.offset, timing_end + 7);
}

#[test]
fn decodes_independent_track_contexts_without_changing_shared_fields() {
    let bytes = fs::read(BASELINE).unwrap();
    for expected in [
        Expected {
            range: 0x10a6d..0x10a77,
            timing: 480,
            timing_width: 2,
            context: &[0x00, 0x23, 0x00],
            number: 7,
            value: 127,
        },
        Expected {
            range: 0x1123a..0x11243,
            timing: 28,
            timing_width: 1,
            context: &[0x00, 0x05, 0x00],
            number: 7,
            value: 127,
        },
        Expected {
            range: 0x11eac..0x11eb6,
            timing: 130,
            timing_width: 2,
            context: &[0x00, 0x02, 0x00],
            number: 7,
            value: 127,
        },
    ] {
        assert_authentic(&bytes, expected);
    }
}

#[test]
fn decodes_track9_zero_single_and_multi_byte_timing() {
    let bytes = fs::read(BASELINE).unwrap();
    for expected in [
        Expected {
            range: 0x143c8..0x143d1,
            timing: 0,
            timing_width: 1,
            context: &[0x00, 0x1f, 0x00],
            number: 7,
            value: 127,
        },
        Expected {
            range: 0x14401..0x1440a,
            timing: 8,
            timing_width: 1,
            context: &[0x00, 0x1f, 0x00],
            number: 7,
            value: 122,
        },
        Expected {
            range: 0x1452f..0x14539,
            timing: 305,
            timing_width: 2,
            context: &[0x00, 0x1f, 0x00],
            number: 7,
            value: 86,
        },
    ] {
        assert_authentic(&bytes, expected);
    }
}

#[test]
fn the_same_decoder_handles_authentic_cc1_and_cc7() {
    let bytes = fs::read(BASELINE).unwrap();
    assert_authentic(
        &bytes,
        Expected {
            range: 0x14548..0x14552,
            timing: 187,
            timing_width: 2,
            context: &[0x00, 0x1f, 0x00],
            number: 7,
            value: 76,
        },
    );
    assert_authentic(
        &bytes,
        Expected {
            range: 0x14552..0x1455b,
            timing: 1,
            timing_width: 1,
            context: &[0x00, 0x1f, 0x00],
            number: 7,
            value: 86,
        },
    );
    assert_authentic(
        &bytes,
        Expected {
            range: 0x14571..0x1457b,
            timing: 1254,
            timing_width: 2,
            context: &[0x00, 0x1f, 0x00],
            number: 1,
            value: 1,
        },
    );
    assert_authentic(
        &bytes,
        Expected {
            range: 0x15213..0x1521d,
            timing: 2030,
            timing_width: 2,
            context: &[0x00, 0x01, 0x00],
            number: 1,
            value: 1,
        },
    );
}

#[test]
fn patch_bank_bytes_are_not_an_ordinary_controller_record() {
    let bytes = fs::read(BASELINE).unwrap();
    let result = decode_bounded_controller_record(
        &bytes,
        ControllerRecordBounds {
            record_range: 0x2fb6e..0x2fb71,
        },
    );
    assert!(result.is_err());
}
