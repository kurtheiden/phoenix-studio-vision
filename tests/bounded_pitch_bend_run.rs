use std::{fs, ops::Range};

use phoenix::controller::{decode_bounded_controller_record, ControllerRecordBounds};
use phoenix::pitch_bend::{
    decode_bounded_pitch_bend_run, BoundedPitchBendError, PitchBendRunBounds,
};
use phoenix::track7::decode_7bit_be_vlq;

const BASELINE: &str = "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline";

const RUNS: [(Range<usize>, usize); 9] = [
    (0x1541f..0x15439, 8),
    (0x15440..0x15453, 6),
    (0x154bb..0x15524, 34),
    (0x1552b..0x1554a, 10),
    (0x1555e..0x1556f, 5),
    (0x15576..0x15589, 6),
    (0x155b7..0x155dc, 12),
    (0x1588f..0x158a3, 6),
    (0x158aa..0x158d8, 15),
];

const EXPECTED_DELTAS: [u32; 102] = [
    1361, 9, 7, 5, 8, 7, 10, 10, 12, 6, 6, 6, 12, 6, 22, 7, 11, 11, 6, 7, 6, 10, 10, 11, 10, 10, 7,
    6, 1586, 3, 7, 7, 9, 11, 6, 4, 13, 9, 8, 11, 11, 10, 6, 7, 10, 11, 6, 694, 7, 6, 6, 6, 36, 6,
    5, 6, 8, 9, 435, 6, 6, 6, 9, 1, 19, 11, 7, 6, 6, 11, 7, 12, 10, 7, 7, 6, 9, 8, 11, 11, 6, 387,
    5, 13, 7, 9, 11, 1, 44, 10, 10, 11, 6, 12, 10, 10, 21, 5, 11, 11, 11, 6,
];

const EXPECTED_RAW_VALUES: [u16; 102] = [
    8127, 7228, 5268, 3501, 2280, 1124, 289, 0, 321, 1895, 3887, 5493, 7260, 8192, 8159, 7677,
    6585, 5461, 4722, 4369, 4272, 4304, 4529, 5075, 6135, 7260, 7934, 8192, 8159, 8095, 8031, 7967,
    7870, 7710, 7517, 7388, 7292, 7099, 6842, 6457, 5878, 5493, 5525, 5943, 6521, 7485, 8192, 6810,
    4497, 2023, 224, 0, 160, 1574, 3694, 5589, 7388, 8192, 7710, 5878, 4015, 3212, 2602, 2152,
    2537, 4112, 6649, 8159, 8192, 7163, 5429, 3758, 2184, 1349, 1156, 1285, 1638, 2762, 4272, 6585,
    8192, 7003, 4497, 2184, 931, 417, 96, 0, 481, 1542, 2537, 3308, 4015, 4529, 5011, 5750, 6810,
    7549, 7742, 7999, 8159, 8192,
];

fn baseline() -> Vec<u8> {
    fs::read(BASELINE).expect("authentic untouched baseline should be available")
}

#[test]
fn decodes_all_nine_authentic_runs_and_all_102_entries() {
    let bytes = baseline();
    let mut observed_index = 0;

    for (expected_range, expected_count) in RUNS {
        let run = decode_bounded_pitch_bend_run(
            &bytes,
            PitchBendRunBounds {
                run_range: expected_range.clone(),
            },
        )
        .unwrap();
        assert_eq!(run.run_range, expected_range);
        assert_eq!(run.entry_tag.value, 0xe0);
        assert_eq!(run.entries.len(), expected_count);

        for (entry_index, entry) in run.entries.iter().enumerate() {
            let expected_raw = EXPECTED_RAW_VALUES[observed_index];
            assert_eq!(entry.timing_delta.value, EXPECTED_DELTAS[observed_index]);
            assert_eq!(entry.pitch_lsb.value, (expected_raw & 0x7f) as u8);
            assert_eq!(entry.pitch_msb.value, (expected_raw >> 7) as u8);
            assert_eq!(entry.raw_value(), expected_raw);
            assert_eq!(entry.timing_delta.range.start, entry.entry_range.start);
            if entry_index == 0 {
                assert_eq!(entry.timing_delta.range.end + 1, entry.pitch_lsb.offset);
            } else {
                assert_eq!(entry.timing_delta.range.end, entry.pitch_lsb.offset);
                assert_eq!(
                    run.entries[entry_index - 1].entry_range.end,
                    entry.entry_range.start
                );
            }
            assert_eq!(entry.pitch_lsb.offset + 1, entry.pitch_msb.offset);
            assert_eq!(entry.pitch_msb.offset + 1, entry.entry_range.end);
            observed_index += 1;
        }
        assert_eq!(
            run.entries.last().unwrap().entry_range.end,
            run.run_range.end
        );
    }

    assert_eq!(observed_index, 102);
}

#[test]
fn authentic_representative_runs_preserve_detailed_fields() {
    let bytes = baseline();
    let run1 = decode_bounded_pitch_bend_run(
        &bytes,
        PitchBendRunBounds {
            run_range: 0x1541f..0x15439,
        },
    )
    .unwrap();
    assert_eq!(run1.entry_tag.offset, 0x15421);
    assert_eq!(run1.entries[0].timing_delta.bytes, &[0x8a, 0x51]);
    assert_eq!(run1.entries[0].timing_delta.value, 1361);
    assert_eq!(run1.entries[0].pitch_lsb.value, 63);
    assert_eq!(run1.entries[0].pitch_msb.value, 63);
    assert_eq!(run1.entries[0].raw_value(), 8127);
    assert_eq!(run1.entries.last().unwrap().raw_value(), 0);

    let run3 = decode_bounded_pitch_bend_run(
        &bytes,
        PitchBendRunBounds {
            run_range: 0x154bb..0x15524,
        },
    )
    .unwrap();
    assert!(run3
        .entries
        .iter()
        .any(|entry| entry.timing_delta.bytes.len() == 1));
    assert!(run3
        .entries
        .iter()
        .any(|entry| entry.timing_delta.bytes.len() == 2));
    assert!(run3
        .entries
        .iter()
        .any(|entry| entry.timing_delta.value == 1586));
    assert!(run3.entries.iter().any(|entry| {
        entry.pitch_lsb.value == 0 && entry.pitch_msb.value == 0x40 && entry.raw_value() == 8192
    }));

    let run9 = decode_bounded_pitch_bend_run(
        &bytes,
        PitchBendRunBounds {
            run_range: 0x158aa..0x158d8,
        },
    )
    .unwrap();
    assert_eq!(run9.entries[0].raw_value(), 0);
    assert_eq!(run9.entries.last().unwrap().raw_value(), 8192);
    assert_eq!(run9.entries.last().unwrap().entry_range.end, 0x158d8);
}

#[test]
fn authentic_neighbors_confirm_all_run_ends() {
    let bytes = baseline();
    for note_start in [
        0x15439, 0x15453, 0x15524, 0x1554a, 0x1556f, 0x15589, 0x155dc, 0x158a3,
    ] {
        let timing = decode_7bit_be_vlq(&bytes, note_start, bytes.len()).unwrap();
        assert_eq!(bytes[note_start + timing.bytes_consumed], 0x90);
    }

    let following_controller = decode_bounded_controller_record(
        &bytes,
        ControllerRecordBounds {
            record_range: 0x158d8..0x158e2,
        },
    )
    .unwrap();
    assert_eq!(following_controller.record_range.start, RUNS[8].0.end);
    assert_eq!(following_controller.controller_number.value, 1);
    assert_eq!(following_controller.controller_value.value, 7);
}

#[test]
fn oversized_authentic_bounds_fail_without_transition_scanning() {
    let bytes = baseline();
    assert_eq!(
        decode_bounded_pitch_bend_run(
            &bytes,
            PitchBendRunBounds {
                run_range: 0x1541f..0x15440,
            },
        ),
        Err(BoundedPitchBendError::MissingContinuationLsb {
            entry_index: 10,
            offset: 0x15440,
        })
    );
    assert_eq!(
        decode_bounded_pitch_bend_run(
            &bytes,
            PitchBendRunBounds {
                run_range: 0x158aa..0x158e1,
            },
        ),
        Err(BoundedPitchBendError::MissingContinuationMsb {
            entry_index: 17,
            offset: 0x158e1,
        })
    );
}
