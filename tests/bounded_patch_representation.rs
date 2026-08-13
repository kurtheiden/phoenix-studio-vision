use phoenix::patch::{
    decode_bounded_patch_representation, decode_known_track3_2_patch, BoundedPatchRepresentation,
    PatchRepresentationBounds,
};
use std::fs;
use std::ops::Range;

const BASELINE: &str = "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline";

fn decode(path: &str, start: usize, status_offset: usize) -> BoundedPatchRepresentation<'static> {
    let bytes = fs::read(path).unwrap_or_else(|error| panic!("cannot read '{path}': {error}"));
    let bytes = Box::leak(bytes.into_boxed_slice());
    decode_bounded_patch_representation(
        bytes,
        PatchRepresentationBounds {
            position_start: start,
            note_status_end: status_offset + 1,
        },
    )
    .unwrap()
}

struct ExpectedCommon<'a> {
    position: u32,
    position_range: Range<usize>,
    payload_length: u8,
    name: &'a str,
    program: u8,
    pc_offset: usize,
    timing: u32,
    status_offset: usize,
}

fn assert_common(result: &BoundedPatchRepresentation<'_>, expected: ExpectedCommon<'_>) {
    assert_eq!(result.position.value, expected.position);
    assert_eq!(result.position.range, expected.position_range);
    assert_eq!(result.payload_length.value, expected.payload_length);
    assert_eq!(result.pre_name_context.bytes.len(), 5);
    assert_eq!(result.name.text, expected.name);
    assert_eq!(result.name.bytes, expected.name.as_bytes());
    assert_eq!(result.program_change.value, expected.program);
    assert_eq!(result.program_change.offset, expected.pc_offset);
    assert_eq!(result.post_pc_timing_component.value, expected.timing);
    assert_eq!(result.note_status.value, 0x90);
    assert_eq!(result.note_status.offset, expected.status_offset);
}

#[test]
fn decodes_four_authentic_representations_with_provenance() {
    let track1 = decode(BASELINE, 0x2f833, 0x2f852);
    assert_common(
        &track1,
        ExpectedCommon {
            position: 0,
            position_range: 0x2f833..0x2f834,
            payload_length: 25,
            name: "Empty Patch",
            program: 61,
            pc_offset: 0x2f84f,
            timing: 9720,
            status_offset: 0x2f852,
        },
    );
    assert_eq!(
        track1.pre_name_context.bytes,
        &[0x00, 0x00, 0x3d, 0x08, 0x1d]
    );
    assert_eq!(
        track1.post_name_context.bytes,
        &[0x02, 0x33, 0x30, 0x04, 0xff, 0xff, 0xff]
    );
    assert!(track1.pre_note_context.bytes.is_empty());

    let track3 = decode(BASELINE, 0x31300, 0x3131a);
    assert_common(
        &track3,
        ExpectedCommon {
            position: 480,
            position_range: 0x31300..0x31302,
            payload_length: 19,
            name: "Wavox",
            program: 29,
            pc_offset: 0x31317,
            timing: 9123,
            status_offset: 0x3131a,
        },
    );
    assert_eq!(
        track3.post_name_context.bytes,
        &[0x02, 0x33, 0x30, 0x04, 0xff, 0x51, 0x02]
    );
    assert!(track3.pre_note_context.bytes.is_empty());

    let track3_2 = decode(BASELINE, 0x31886, 0x318b4);
    assert_common(
        &track3_2,
        ExpectedCommon {
            position: 530,
            position_range: 0x31886..0x31888,
            payload_length: 27,
            name: "Ming Dynasty",
            program: 23,
            pc_offset: 0x318a5,
            timing: 8908,
            status_offset: 0x318b4,
        },
    );
    assert_eq!(
        track3_2.pre_note_context.bytes,
        &[0xff, 0x60, 0x07, 0x57, 0x7f, 0x00, 0x6c, 0x6c, 0xa3, 0x4a, 0x81, 0x25]
    );
    assert_eq!(track3_2.pre_note_context.range, 0x318a8..0x318b4);

    let track2 = decode(BASELINE, 0x2fb55, 0x2fb74);
    assert_common(
        &track2,
        ExpectedCommon {
            position: 0,
            position_range: 0x2fb55..0x2fb56,
            payload_length: 25,
            name: "Stereoww Bs",
            program: 37,
            pc_offset: 0x2fb71,
            timing: 1920,
            status_offset: 0x2fb74,
        },
    );
    assert_eq!(
        track2.pre_name_context.bytes,
        &[0x00, 0x01, 0x25, 0xf8, 0xa5]
    );
    assert_eq!(
        track2.post_name_context.bytes,
        &[0x02, 0x33, 0x38, 0x04, 0xff, 0x51, 0x01]
    );
    assert_eq!(track2.post_name_context.range, 0x2fb6a..0x2fb71);
    assert!(track2.pre_note_context.bytes.is_empty());
}

#[test]
fn decodes_all_controlled_track3_2_states() {
    struct Case<'a> {
        path: &'a str,
        position: u32,
        name: &'a str,
        pc: u8,
        pc_offset: usize,
        timing: u32,
        status_offset: usize,
        payload_length: u8,
    }
    let cases = [
        Case { path: BASELINE, position: 530, name: "Ming Dynasty", pc: 23, pc_offset: 0x318a5, timing: 8908, status_offset: 0x318b4, payload_length: 27 },
        Case { path: "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 023 - Track 3-2 Patch Change/newest STUFF baseline EXP23", position: 530, name: "Ming Dynasty", pc: 24, pc_offset: 0x318a5, timing: 8908, status_offset: 0x318b4, payload_length: 27 },
        Case { path: "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 024 - Track 3-2 Program Change 100/newest STUFF baseline EXP24", position: 530, name: "Ming Dynasty", pc: 100, pc_offset: 0x318a5, timing: 8908, status_offset: 0x318b4, payload_length: 27 },
        Case { path: "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 025 - Track 3-2 Patch Position Plus One/newest STUFF baseline EXP25", position: 531, name: "Ming Dynasty", pc: 23, pc_offset: 0x318a5, timing: 8907, status_offset: 0x318b4, payload_length: 27 },
        Case { path: "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 026 - Track 3-2 Patch Name Change/newest STUFF baseline EXP26", position: 530, name: "Phoenix Test", pc: 23, pc_offset: 0x318a5, timing: 8908, status_offset: 0x318b4, payload_length: 27 },
        Case { path: "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 027 - Track 3-2 Short Patch Name/newest STUFF baseline EXP27", position: 530, name: "Phoenix", pc: 23, pc_offset: 0x318a0, timing: 8908, status_offset: 0x318af, payload_length: 22 },
    ];

    for case in cases {
        let result = decode(case.path, 0x31886, case.status_offset);
        assert_common(
            &result,
            ExpectedCommon {
                position: case.position,
                position_range: 0x31886..0x31888,
                payload_length: case.payload_length,
                name: case.name,
                program: case.pc,
                pc_offset: case.pc_offset,
                timing: case.timing,
                status_offset: case.status_offset,
            },
        );
        assert_eq!(result.pre_note_context.bytes.len(), 12);
    }
}

#[test]
fn strict_track3_2_decoder_still_rejects_other_authentic_layouts() {
    let bytes = fs::read(BASELINE).unwrap();
    assert!(decode_known_track3_2_patch(&bytes, 0x2f833, 0x2f853).is_err());
    assert!(decode_known_track3_2_patch(&bytes, 0x31300, 0x3131b).is_err());
}
