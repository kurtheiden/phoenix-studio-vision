use phoenix::patch::decode_known_track3_2_patch;
use std::fs;
use std::path::Path;

const START: usize = 0x31886;

struct Expected<'a> {
    path: &'a str,
    position: u32,
    name: &'a str,
    program_change: u8,
    program_change_offset: usize,
    first_note_status_offset: usize,
}

fn assert_controlled_state(expected: Expected<'_>) {
    let path = Path::new(expected.path);
    let bytes = fs::read(path).unwrap_or_else(|error| {
        panic!(
            "cannot read controlled Patch artifact '{}': {error}",
            path.display()
        )
    });
    let diagnostic = decode_known_track3_2_patch(&bytes, START, bytes.len()).unwrap();

    assert_eq!(diagnostic.position, expected.position);
    assert_eq!(diagnostic.name, expected.name);
    assert_eq!(diagnostic.program_change, expected.program_change);
    assert_eq!(
        diagnostic.program_change_offset,
        expected.program_change_offset
    );
    assert_eq!(diagnostic.first_note_status, 0x90);
    assert_eq!(
        diagnostic.first_note_status_offset,
        expected.first_note_status_offset
    );
}

#[test]
fn decodes_experiment_007_baseline() {
    assert_controlled_state(Expected {
        path: "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline",
        position: 530,
        name: "Ming Dynasty",
        program_change: 23,
        program_change_offset: 0x318a5,
        first_note_status_offset: 0x318b4,
    });
}

#[test]
fn decodes_experiment_023_pc_24() {
    assert_controlled_state(Expected {
        path: "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 023 - Track 3-2 Patch Change/newest STUFF baseline EXP23",
        position: 530,
        name: "Ming Dynasty",
        program_change: 24,
        program_change_offset: 0x318a5,
        first_note_status_offset: 0x318b4,
    });
}

#[test]
fn decodes_experiment_024_pc_100() {
    assert_controlled_state(Expected {
        path: "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 024 - Track 3-2 Program Change 100/newest STUFF baseline EXP24",
        position: 530,
        name: "Ming Dynasty",
        program_change: 100,
        program_change_offset: 0x318a5,
        first_note_status_offset: 0x318b4,
    });
}

#[test]
fn decodes_experiment_025_position_531() {
    assert_controlled_state(Expected {
        path: "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 025 - Track 3-2 Patch Position Plus One/newest STUFF baseline EXP25",
        position: 531,
        name: "Ming Dynasty",
        program_change: 23,
        program_change_offset: 0x318a5,
        first_note_status_offset: 0x318b4,
    });
}

#[test]
fn decodes_experiment_026_equal_length_name() {
    assert_controlled_state(Expected {
        path: "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 026 - Track 3-2 Patch Name Change/newest STUFF baseline EXP26",
        position: 530,
        name: "Phoenix Test",
        program_change: 23,
        program_change_offset: 0x318a5,
        first_note_status_offset: 0x318b4,
    });
}

#[test]
fn decodes_experiment_027_variable_length_name_and_relocation() {
    assert_controlled_state(Expected {
        path: "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 027 - Track 3-2 Short Patch Name/newest STUFF baseline EXP27",
        position: 530,
        name: "Phoenix",
        program_change: 23,
        program_change_offset: 0x318a0,
        first_note_status_offset: 0x318af,
    });
}
