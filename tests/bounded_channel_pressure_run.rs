use std::{fs, ops::Range};

use phoenix::channel_pressure::{
    decode_bounded_channel_pressure_run, BoundedChannelPressureError, ChannelPressureRunBounds,
};
use phoenix::controller::{decode_bounded_controller_record, ControllerRecordBounds};

const BASELINE_PATH: &str = "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline";
const PRECEDING_CONTROLLER_RANGE: Range<usize> = 0x14783..0x1478c;
const CHANNEL_PRESSURE_RUN_RANGE: Range<usize> = 0x1478c..0x147ce;
const FOLLOWING_NOTE_RANGE: Range<usize> = 0x147ce..0x147d6;

const EXPECTED_DELTAS: [u32; 32] = [
    288, 8, 9, 11, 13, 15, 7, 11, 11, 12, 15, 8, 10, 10, 11, 14, 12, 6, 13, 16, 9, 11, 11, 8, 27,
    42, 11, 11, 9, 13, 14, 10,
];
const EXPECTED_VALUES: [u8; 32] = [
    1, 5, 9, 15, 20, 27, 32, 37, 42, 46, 50, 53, 57, 60, 62, 66, 68, 70, 71, 72, 75, 76, 77, 78,
    79, 77, 72, 65, 51, 31, 10, 0,
];

fn baseline() -> Vec<u8> {
    fs::read(BASELINE_PATH).expect("authentic untouched baseline should be available")
}

#[test]
fn decodes_all_authentic_track_9_channel_pressure_entries() {
    let bytes = baseline();
    let run = decode_bounded_channel_pressure_run(
        &bytes,
        ChannelPressureRunBounds {
            run_range: CHANNEL_PRESSURE_RUN_RANGE,
        },
    )
    .unwrap();

    assert_eq!(run.run_range, 0x1478c..0x147ce);
    assert_eq!(run.entry_tag.value, 0xd0);
    assert_eq!(run.entry_tag.offset, 0x1478e);
    assert_eq!(run.entries.len(), 32);

    for (index, entry) in run.entries.iter().enumerate() {
        assert_eq!(entry.timing_delta.value, EXPECTED_DELTAS[index]);
        assert_eq!(entry.pressure_value.value, EXPECTED_VALUES[index]);
        assert_eq!(entry.timing_delta.range.start, entry.entry_range.start);
        if index == 0 {
            assert_eq!(
                entry.timing_delta.range.end + 1,
                entry.pressure_value.offset
            );
        } else {
            assert_eq!(entry.timing_delta.range.end, entry.pressure_value.offset);
        }
        assert_eq!(entry.pressure_value.offset + 1, entry.entry_range.end);
        if index > 0 {
            assert_eq!(
                run.entries[index - 1].entry_range.end,
                entry.entry_range.start
            );
        }
    }

    assert_eq!(run.entries[0].entry_range, 0x1478c..0x14790);
    assert_eq!(run.entries[0].timing_delta.bytes, &[0x82, 0x20]);
    assert_eq!(run.entries[0].timing_delta.value, 288);
    assert_eq!(run.entries[0].pressure_value.value, 1);

    let last = run.entries.last().unwrap();
    assert_eq!(last.timing_delta.bytes, &[0x0a]);
    assert_eq!(last.timing_delta.value, 10);
    assert_eq!(last.pressure_value.value, 0);
    assert_eq!(last.entry_range.end, 0x147ce);
}

#[test]
fn authentic_neighbors_confirm_exact_run_adjacency() {
    let bytes = baseline();
    let preceding = decode_bounded_controller_record(
        &bytes,
        ControllerRecordBounds {
            record_range: PRECEDING_CONTROLLER_RANGE,
        },
    )
    .unwrap();
    assert_eq!(preceding.record_range.end, 0x1478c);
    assert_eq!(preceding.controller_number.value, 1);

    let run = decode_bounded_channel_pressure_run(
        &bytes,
        ChannelPressureRunBounds {
            run_range: CHANNEL_PRESSURE_RUN_RANGE,
        },
    )
    .unwrap();
    assert_eq!(run.run_range.start, preceding.record_range.end);
    assert_eq!(run.run_range.end, FOLLOWING_NOTE_RANGE.start);
    assert_eq!(
        &bytes[FOLLOWING_NOTE_RANGE],
        &[0x83, 0x56, 0x90, 0x6c, 0x33, 0x34, 0x8b, 0x31]
    );
}

#[test]
fn oversized_authentic_bound_does_not_scan_for_following_note_marker() {
    let bytes = baseline();
    let result = decode_bounded_channel_pressure_run(
        &bytes,
        ChannelPressureRunBounds {
            run_range: 0x1478c..0x147d6,
        },
    );

    assert_eq!(
        result,
        Err(
            BoundedChannelPressureError::MissingContinuationPressureValue {
                entry_index: 35,
                offset: 0x147d6,
            }
        )
    );
}
