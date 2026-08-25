use std::fs;

use phoenix::mixed_event::{
    walk_bounded_mixed_events, MixedEventBounds, MixedEventItem, MixedEventKind,
    MixedEventTimingBasis, MixedEventWalkError,
};
use phoenix::sequence_container::parse_project_166;

const BASELINE: &str = "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline";

fn walk(bytes: &[u8]) -> Result<phoenix::mixed_event::MixedEventWalk<'_>, MixedEventWalkError> {
    walk_bounded_mixed_events(
        bytes,
        MixedEventBounds {
            event_range: 0..bytes.len(),
        },
        MixedEventTimingBasis::default(),
    )
}

fn family_counts(result: &phoenix::mixed_event::MixedEventWalk<'_>) -> [usize; 5] {
    let mut counts = [0; 5];
    for item in &result.items {
        match item {
            MixedEventItem::Patch(_) => counts[1] += 1,
            MixedEventItem::PatchToNote(_) => {
                counts[0] += 1;
                counts[1] += 1;
            }
            MixedEventItem::Event(event) => match &event.event {
                MixedEventKind::Note(_)
                | MixedEventKind::ContextMediatedNote(_)
                | MixedEventKind::DoubleContextMediatedNote(_) => counts[0] += 1,
                MixedEventKind::Controller(_) => counts[2] += 1,
                MixedEventKind::ChannelPressure { .. } => counts[3] += 1,
                MixedEventKind::PitchBend { .. } => counts[4] += 1,
            },
        }
    }
    counts
}

#[test]
fn empty_range_and_nonzero_provenance_are_exact() {
    let bytes = [0xaa, 0xbb, 0x00, 0x90, 0x3c, 0x40, 0x20, 0x01];
    let empty = walk_bounded_mixed_events(
        &bytes,
        MixedEventBounds { event_range: 2..2 },
        MixedEventTimingBasis::default(),
    )
    .unwrap();
    assert!(empty.items.is_empty());
    assert_eq!(empty.consumed_range, 2..2);

    let decoded = walk_bounded_mixed_events(
        &bytes,
        MixedEventBounds { event_range: 2..8 },
        MixedEventTimingBasis {
            previous_event_position: 10,
        },
    )
    .unwrap();
    assert_eq!(decoded.consumed_range, 2..8);
    let MixedEventItem::Event(event) = &decoded.items[0] else {
        panic!("expected Note")
    };
    assert_eq!(event.position, 10);
    let MixedEventKind::Note(note) = &event.event else {
        panic!("expected Note")
    };
    assert_eq!(note.representation_range, 2..8);
    assert_eq!(note.timing.range, 2..3);
    assert_eq!(note.status.unwrap().offset, 3);
    assert_eq!(note.pitch.offset, 4);
    assert_eq!(note.duration.range, 7..8);
}

#[test]
fn note_continuation_and_exact_end_are_stateful() {
    let bytes = [
        0x00, 0x90, 0x3c, 0x40, 0x20, 0x01, // entry
        0x02, 0x3d, 0x41, 0x21, 0x81, 0x00, // continuation
    ];
    let result = walk(&bytes).unwrap();
    assert_eq!(result.logical_event_count(), 2);
    assert_eq!(result.consumed_range, 0..bytes.len());
    let MixedEventItem::Event(second) = &result.items[1] else {
        panic!("expected Note")
    };
    assert_eq!(second.position, 2);
    let MixedEventKind::Note(note) = &second.event else {
        panic!("expected Note")
    };
    assert!(note.status.is_none());
    assert_eq!(note.duration.value, 128);
}

#[test]
fn controller_transitions_replace_compact_state() {
    let note = [0x00, 0x90, 0x3c, 0x40, 0x20, 0x01];
    let controller = [0x01, 0xff, 0x41, 0x05, 0x00, 0x1f, 0x00, 0x07, 0x40];
    let bend = [0x02, 0xe0, 0x00, 0x40];
    let pressure = [0x03, 0xd0, 0x22];
    let explicit_note = [0x04, 0x90, 0x3e, 0x41, 0x21, 0x01];
    let mut bytes = Vec::new();
    bytes.extend(note);
    bytes.extend(controller);
    bytes.extend(bend);
    bytes.extend(controller);
    bytes.extend(pressure);
    bytes.extend(explicit_note);
    let result = walk(&bytes).unwrap();
    assert_eq!(family_counts(&result), [2, 0, 2, 1, 1]);
    assert_eq!(result.consumed_range.end, bytes.len());
}

#[test]
fn pressure_and_bend_continuations_exit_on_high_bit_branches() {
    let bytes = [
        0x00, 0xd0, 0x01, // Pressure entry
        0x01, 0x02, // Pressure continuation
        0x02, 0x90, 0x3c, 0x40, 0x20, 0x01, // Note
        0x03, 0xe0, 0x00, 0x40, // Bend entry
        0x04, 0x01, 0x3f, // Bend continuation
        0x05, 0xff, 0x41, 0x05, 0x00, 0x1f, 0x00, 0x01, 0x7f, // Controller
    ];
    let result = walk(&bytes).unwrap();
    assert_eq!(family_counts(&result), [1, 0, 1, 2, 2]);
}

#[test]
fn explicit_cross_family_entries_cover_note_pressure_and_bend() {
    for bytes in [
        vec![0x00, 0x90, 0x3c, 0x40, 0x20, 0x01, 0x01, 0xe0, 0x00, 0x40],
        vec![0x00, 0xd0, 0x01, 0x01, 0x90, 0x3c, 0x40, 0x20, 0x01],
        vec![0x00, 0xe0, 0x00, 0x40, 0x01, 0x90, 0x3c, 0x40, 0x20, 0x01],
    ] {
        let result = walk(&bytes).unwrap();
        assert_eq!(result.logical_event_count(), 2);
        assert_eq!(result.consumed_range.end, bytes.len());
    }
}

#[test]
fn controller_can_enter_note_explicitly() {
    let bytes = [
        0x00, 0xff, 0x41, 0x05, 0x00, 0x1f, 0x00, 0x07, 0x40, 0x01, 0x90, 0x3c, 0x40, 0x20, 0x01,
    ];
    let result = walk(&bytes).unwrap();
    assert_eq!(family_counts(&result), [1, 0, 1, 0, 0]);
}

#[test]
fn one_ff60_context_mediates_note_entry() {
    let bytes = [
        0x02, 0xff, 0x60, 0x02, 0x11, 0x22, 0x03, 0x90, 0x3c, 0x40, 0x20, 0x01,
    ];
    let result = walk(&bytes).unwrap();
    assert_eq!(result.logical_event_count(), 1);
    let MixedEventItem::Event(event) = &result.items[0] else {
        panic!("expected mediated Note")
    };
    assert_eq!(event.position, 5);
    let MixedEventKind::ContextMediatedNote(note) = &event.event else {
        panic!("expected mediated Note")
    };
    assert_eq!(note.context.payload.bytes, &[0x11, 0x22]);
    assert_eq!(note.leading_timing.value, 2);
    assert_eq!(note.final_timing.value, 3);
    assert_eq!(note.representation_range, 0..bytes.len());
}

struct AuthenticDoubleContextCase {
    range: std::ops::Range<usize>,
    transition_end: usize,
    previous_position: u32,
    position: u32,
    timings: [u32; 3],
    timing_ranges: [std::ops::Range<usize>; 3],
    first_range: std::ops::Range<usize>,
    first_payload: &'static [u8],
    second_range: std::ops::Range<usize>,
    second_payload: &'static [u8],
    note_range: std::ops::Range<usize>,
    note: [u8; 4],
}

fn assert_authentic_double_context(case: AuthenticDoubleContextCase) {
    let bytes = fs::read(BASELINE).unwrap();
    let result = walk_bounded_mixed_events(
        &bytes,
        MixedEventBounds {
            event_range: case.range.clone(),
        },
        MixedEventTimingBasis {
            previous_event_position: case.previous_position,
        },
    )
    .unwrap();
    assert_eq!(result.consumed_range, case.range);
    assert_eq!(result.logical_event_count(), 2);

    let MixedEventItem::Event(first) = &result.items[0] else {
        panic!("expected double-context Note")
    };
    assert_eq!(first.position, case.position);
    let MixedEventKind::DoubleContextMediatedNote(entry) = &first.event else {
        panic!("expected double-context Note")
    };
    assert_eq!(
        entry.representation_range,
        case.range.start..case.transition_end
    );
    assert_eq!(entry.leading_timing.value, case.timings[0]);
    assert_eq!(entry.leading_timing.range, case.timing_ranges[0]);
    assert_eq!(entry.first_context.range, case.first_range);
    assert_eq!(entry.first_context.payload_length.value, 6);
    assert_eq!(entry.first_context.payload.bytes, case.first_payload);
    assert_eq!(entry.inter_context_timing.value, case.timings[1]);
    assert_eq!(entry.inter_context_timing.range, case.timing_ranges[1]);
    assert_eq!(entry.second_context.range, case.second_range);
    assert_eq!(entry.second_context.payload_length.value, 7);
    assert_eq!(entry.second_context.payload.bytes, case.second_payload);
    assert_eq!(entry.final_timing.value, case.timings[2]);
    assert_eq!(entry.final_timing.range, case.timing_ranges[2]);
    assert_eq!(entry.note.representation_range, case.note_range);
    assert!(entry.note.status.is_some());
    assert_eq!(
        [
            entry.note.pitch.value,
            entry.note.attack_velocity.value,
            entry.note.release_velocity.value,
            entry.note.duration.value as u8,
        ],
        case.note
    );

    let MixedEventItem::Event(next) = &result.items[1] else {
        panic!("expected compact Note continuation")
    };
    let MixedEventKind::Note(next_note) = &next.event else {
        panic!("expected compact Note continuation")
    };
    assert!(next_note.status.is_none());
    assert!(next.position > first.position);
}

#[test]
fn authentic_bells_track_6_double_context_notes_are_exact() {
    for case in [
        AuthenticDoubleContextCase {
            range: 0x121ba..0x121dc,
            transition_end: 0x121d6,
            previous_position: 103_455,
            position: 104_036,
            timings: [538, 39, 4],
            timing_ranges: [0x121ba..0x121bc, 0x121c5..0x121c6, 0x121d0..0x121d1],
            first_range: 0x121bc..0x121c5,
            first_payload: &[0x57, 0x7f, 0x00, 0x7e, 0x7c, 0x27],
            second_range: 0x121c6..0x121d0,
            second_payload: &[0x57, 0x7f, 0x00, 0x7e, 0x44, 0x8a, 0x6f],
            note_range: 0x121d1..0x121d6,
            note: [74, 123, 126, 49],
        },
        AuthenticDoubleContextCase {
            range: 0x122c7..0x122e8,
            transition_end: 0x122e2,
            previous_position: 111_634,
            position: 111_720,
            timings: [3, 26, 57],
            timing_ranges: [0x122c7..0x122c8, 0x122d1..0x122d2, 0x122dc..0x122dd],
            first_range: 0x122c8..0x122d1,
            first_payload: &[0x57, 0x7f, 0x00, 0x7e, 0x7c, 0x1a],
            second_range: 0x122d2..0x122dc,
            second_payload: &[0x57, 0x7f, 0x00, 0x7c, 0x3e, 0x89, 0x53],
            note_range: 0x122dd..0x122e2,
            note: [70, 118, 123, 58],
        },
        AuthenticDoubleContextCase {
            range: 0x12310..0x12331,
            transition_end: 0x1232b,
            previous_position: 113_536,
            position: 113_631,
            timings: [12, 13, 70],
            timing_ranges: [0x12310..0x12311, 0x1231a..0x1231b, 0x12325..0x12326],
            first_range: 0x12311..0x1231a,
            first_payload: &[0x57, 0x7f, 0x00, 0x7f, 0x7f, 0x0d],
            second_range: 0x1231b..0x12325,
            second_payload: &[0x57, 0x7f, 0x00, 0x7e, 0x42, 0x8b, 0x16],
            note_range: 0x12326..0x1232b,
            note: [70, 120, 106, 64],
        },
    ] {
        assert_authentic_double_context(case);
    }
}

fn synthetic_double_context(first_length: u8, second_length: u8) -> Vec<u8> {
    let mut bytes = vec![0x02, 0xff, 0x60, first_length];
    bytes.extend(std::iter::repeat(0x11).take(usize::from(first_length)));
    bytes.extend([0x03, 0xff, 0x60, second_length]);
    bytes.extend(std::iter::repeat(0x22).take(usize::from(second_length)));
    bytes.extend([0x04, 0x90, 0x3c, 0x40, 0x20, 0x01]);
    bytes
}

#[test]
fn double_context_requires_exact_six_then_seven_lengths() {
    for (first, second, expected, observed) in [(5, 7, 6, 5), (6, 6, 7, 6)] {
        assert!(matches!(
            walk(&synthetic_double_context(first, second)),
            Err(MixedEventWalkError::ContextLengthMismatch {
                expected: actual_expected,
                observed: actual_observed,
                ..
            }) if actual_expected == expected && actual_observed == observed
        ));
    }
}

#[test]
fn double_context_rejects_malformed_current_tags_and_timings_without_scanning() {
    let valid = synthetic_double_context(6, 7);

    for bytes in [
        valid[..11].to_vec(),
        valid[..12].to_vec(),
        {
            let mut value = valid.clone();
            value[11] = 0xfe;
            value.extend([0x00, 0xff, 0x60, 0x07, 0, 0, 0, 0, 0, 0, 0]);
            value
        },
        {
            let mut value = valid.clone();
            value[12] = 0x61;
            value.extend([0x00, 0xff, 0x60, 0x07, 0, 0, 0, 0, 0, 0, 0]);
            value
        },
    ] {
        assert!(walk(&bytes).is_err());
    }

    for timing_range in [0..1, 10..11, 21..22] {
        let mut bytes = valid.clone();
        bytes.splice(timing_range, [0x81, 0x81, 0x81, 0x81]);
        assert!(matches!(
            walk(&bytes),
            Err(MixedEventWalkError::TimingVlq { .. })
        ));
    }
}

#[test]
fn double_context_requires_immediate_direct_note_and_exact_bounds() {
    let valid = synthetic_double_context(6, 7);
    for end in 1..valid.len() {
        assert!(walk(&valid[..end]).is_err(), "unexpected success at {end}");
    }

    for replacement in [
        vec![
            0x00, 0xff, 0x60, 0x07, 0, 0, 0, 0, 0, 0, 0, 0x00, 0x90, 0x3c, 0x40, 0x20, 1,
        ],
        vec![0x00, 0xff, 0x41, 0x05, 0, 0, 0, 7, 1],
        vec![0x00, 0xd0, 0x01],
        vec![0x00, 0x3c, 0x40, 0x20, 1],
        vec![
            0x00, 0x91, 0x3c, 0x40, 0x20, 1, 0x00, 0x90, 0x3c, 0x40, 0x20, 1,
        ],
        vec![0x00, 0x90, 0x3c],
    ] {
        let mut bytes = valid[..21].to_vec();
        bytes.extend(replacement);
        assert!(walk(&bytes).is_err());
    }

    assert!(matches!(
        walk_bounded_mixed_events(
            &valid,
            MixedEventBounds {
                event_range: 0..valid.len(),
            },
            MixedEventTimingBasis {
                previous_event_position: u32::MAX,
            },
        ),
        Err(MixedEventWalkError::PositionOverflow { .. })
    ));
}

#[test]
fn authentic_direct_patch_to_note_is_coupled() {
    let bytes = fs::read(BASELINE).unwrap();
    let result = walk_bounded_mixed_events(
        &bytes,
        MixedEventBounds {
            event_range: 0x2f833..0x2f857,
        },
        MixedEventTimingBasis::default(),
    )
    .unwrap();
    assert_eq!(result.logical_event_count(), 2);
    let MixedEventItem::PatchToNote(transition) = &result.items[0] else {
        panic!("expected Patch-to-Note")
    };
    assert!(transition.context.is_none());
    assert!(transition.final_timing.is_none());
    assert_eq!(transition.patch_position, 0);
    assert_eq!(transition.first_note_position, 9720);
    assert_eq!(transition.first_note.status.unwrap().offset, 0x2f852);
}

fn assert_bells_patch_controller_note(
    event_range: std::ops::Range<usize>,
    patch_range: std::ops::Range<usize>,
    controller_range: std::ops::Range<usize>,
    note_range: std::ops::Range<usize>,
    positions: [u32; 3],
    context: [u8; 3],
    expected_logical_events: usize,
) {
    let bytes = fs::read(BASELINE).unwrap();
    let result = walk_bounded_mixed_events(
        &bytes,
        MixedEventBounds {
            event_range: event_range.clone(),
        },
        MixedEventTimingBasis::default(),
    )
    .unwrap();
    assert_eq!(result.consumed_range, event_range);
    assert_eq!(result.logical_event_count(), expected_logical_events);

    let MixedEventItem::Patch(patch) = &result.items[0] else {
        panic!("expected standalone Patch")
    };
    assert_eq!(patch.representation_range, patch_range);
    assert_eq!(patch.position.value, positions[0]);

    let MixedEventItem::Event(controller) = &result.items[1] else {
        panic!("expected Controller")
    };
    assert_eq!(controller.position, positions[1]);
    let MixedEventKind::Controller(controller) = &controller.event else {
        panic!("expected Controller")
    };
    assert_eq!(controller.record_range, controller_range);
    assert_eq!(controller.context.bytes, context);
    assert_eq!(controller.controller_number.value, 7);
    assert_eq!(controller.controller_value.value, 127);

    let MixedEventItem::Event(note) = &result.items[2] else {
        panic!("expected Note")
    };
    assert_eq!(note.position, positions[2]);
    let first_note_position = note.position;
    let MixedEventKind::Note(note) = &note.event else {
        panic!("expected Note")
    };
    assert_eq!(note.representation_range, note_range);
    assert!(note.status.is_some());
    assert_eq!(
        patch.representation_range.end,
        controller.record_range.start
    );
    assert_eq!(controller.record_range.end, note.representation_range.start);

    let MixedEventItem::Event(next) = &result.items[3] else {
        panic!("expected Note continuation")
    };
    let MixedEventKind::Note(next_note) = &next.event else {
        panic!("expected Note continuation")
    };
    assert!(next_note.status.is_none());
    assert!(next.position > first_note_position);
}

#[test]
fn authentic_bells_track_3_patch_controller_note_consumes_exactly() {
    assert_bells_patch_controller_note(
        0x10a4d..0x110c8,
        0x10a4d..0x10a6d,
        0x10a6d..0x10a77,
        0x10a77..0x10a80,
        [480, 960, 71_040],
        [0x00, 0x23, 0x00],
        275,
    );
}

#[test]
fn authentic_bells_track_4_patch_controller_note_consumes_exactly() {
    assert_bells_patch_controller_note(
        0x1121b..0x1192a,
        0x1121b..0x1123a,
        0x1123a..0x11243,
        0x11243..0x1124b,
        [180, 208, 71_278],
        [0x00, 0x05, 0x00],
        296,
    );
}

#[test]
fn authentic_bells_exact_consumption_is_fourteen_of_fourteen() {
    let bytes = fs::read(BASELINE).unwrap();
    let project = parse_project_166(&bytes).unwrap();
    let bells = project
        .sequences
        .iter()
        .find(|sequence| sequence.sequence_name.as_utf8() == Some("Bells for her"))
        .unwrap();
    assert_eq!(bells.track_pairs.len(), 14);

    let mut consumed = Vec::new();
    let mut rejected = Vec::new();
    for (index, pair) in bells.track_pairs.iter().enumerate() {
        let bounds = pair.validated_event_bounds().unwrap();
        match walk_bounded_mixed_events(
            &bytes,
            MixedEventBounds {
                event_range: bounds.event_range.clone(),
            },
            MixedEventTimingBasis::default(),
        ) {
            Ok(walk) if walk.consumed_range == bounds.event_range => consumed.push(index + 1),
            Ok(_) | Err(_) => rejected.push(index + 1),
        }
    }
    assert_eq!(consumed.len(), 14);
    assert!(rejected.is_empty());
}

#[test]
fn authentic_bells_track_6_consumes_structurally_but_retains_timing_mismatch() {
    let bytes = fs::read(BASELINE).unwrap();
    let result = walk_bounded_mixed_events(
        &bytes,
        MixedEventBounds {
            event_range: 0x11eac..0x123dd,
        },
        MixedEventTimingBasis::default(),
    )
    .unwrap();
    assert_eq!(result.consumed_range, 0x11eac..0x123dd);
    assert_eq!(result.logical_event_count(), 184);
    assert_eq!(family_counts(&result), [182, 1, 1, 0, 0]);

    let mut residual_single = 0;
    let mut double = 0;
    let mut single_lengths = Vec::new();
    for item in &result.items {
        match item {
            MixedEventItem::PatchToNote(transition) => {
                assert_eq!(transition.patch_position, 160);
                assert_eq!(transition.first_note_position, 71_773);
            }
            MixedEventItem::Event(event) => match &event.event {
                MixedEventKind::ContextMediatedNote(note) => {
                    single_lengths.push(note.context.payload_length.value);
                    if note.representation_range.start >= 0x121ba {
                        residual_single += 1;
                    }
                }
                MixedEventKind::DoubleContextMediatedNote(_) => double += 1,
                _ => {}
            },
            MixedEventItem::Patch(_) => {}
        }
    }
    assert_eq!(residual_single, 6);
    assert_eq!(double, 3);
    assert!(single_lengths.contains(&6));
    assert!(single_lengths.contains(&7));

    let last_position = result
        .items
        .iter()
        .rev()
        .find_map(|item| match item {
            MixedEventItem::Event(event) => Some(event.position),
            _ => None,
        })
        .unwrap();
    assert_eq!(last_position, 118_572);
    assert_eq!(last_position + 130, 118_702);
}

fn synthetic_patch_controller_note() -> Vec<u8> {
    vec![
        0x00, 0xff, 0x7c, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x01, 0xff, 0x41, 0x05,
        0x00, 0x01, 0x00, 0x07, 0x7f, 0x02, 0x90, 0x3c, 0x40, 0x20, 0x01,
    ]
}

#[test]
fn strict_patch_controller_note_rejects_malformed_current_controller() {
    let mut wrong_tag = synthetic_patch_controller_note();
    wrong_tag[13] = 0x42;
    wrong_tag.extend([0x00, 0xff, 0x41, 0x05, 0, 0, 0, 7, 127]);
    assert!(matches!(
        walk(&wrong_tag),
        Err(MixedEventWalkError::PatchContextMismatch {
            cursor: 0,
            offset: 12,
            observed: Some(0xff),
        })
    ));

    let mut wrong_length = synthetic_patch_controller_note();
    wrong_length[14] = 0x04;
    assert!(matches!(
        walk(&wrong_length),
        Err(MixedEventWalkError::MalformedController { cursor: 11, .. })
    ));

    for end in 12..20 {
        let bytes = synthetic_patch_controller_note();
        assert!(walk(&bytes[..end]).is_err(), "unexpected success at {end}");
    }

    let mut overlong_timing = synthetic_patch_controller_note();
    overlong_timing.splice(11..12, [0x81, 0x81, 0x81, 0x81]);
    assert!(matches!(
        walk(&overlong_timing),
        Err(MixedEventWalkError::TimingVlq { .. })
    ));
}

#[test]
fn strict_patch_controller_note_requires_one_direct_explicit_note() {
    let mut non_note = synthetic_patch_controller_note();
    non_note[21] = 0xd0;
    non_note.extend([0x00, 0x90, 0x3c, 0x40, 0x20, 0x01]);
    assert!(matches!(
        walk(&non_note),
        Err(MixedEventWalkError::MalformedNote { cursor: 20, .. })
    ));

    let mut ff60 = synthetic_patch_controller_note();
    ff60.splice(
        20..,
        [
            0x00, 0xff, 0x60, 0x01, 0x11, 0x00, 0x90, 0x3c, 0x40, 0x20, 0x01,
        ],
    );
    assert!(matches!(
        walk(&ff60),
        Err(MixedEventWalkError::MalformedNote { cursor: 20, .. })
    ));

    let mut second_controller = synthetic_patch_controller_note();
    second_controller.splice(
        20..,
        [
            0x00, 0xff, 0x41, 0x05, 0, 0, 0, 7, 1, 0, 0x90, 0x3c, 0x40, 0x20, 1,
        ],
    );
    assert!(matches!(
        walk(&second_controller),
        Err(MixedEventWalkError::MalformedNote { cursor: 20, .. })
    ));

    let mut malformed_note = synthetic_patch_controller_note();
    malformed_note[21] = 0x91;
    malformed_note.extend([0x00, 0x90, 0x3c, 0x40, 0x20, 0x01]);
    assert!(matches!(
        walk(&malformed_note),
        Err(MixedEventWalkError::MalformedNote { cursor: 20, .. })
    ));

    let bytes = synthetic_patch_controller_note();
    for end in 20..bytes.len() {
        assert!(walk(&bytes[..end]).is_err(), "unexpected success at {end}");
    }
}

#[test]
fn rejects_current_cursor_without_scanning_for_later_valid_event() {
    let bytes = [
        0x00, 0xff, 0x55, 0x00, // unsupported current tag
        0x00, 0x90, 0x3c, 0x40, 0x20, 0x01, // valid-looking later Note
    ];
    assert_eq!(
        walk(&bytes),
        Err(MixedEventWalkError::UnsupportedFfTag {
            cursor: 0,
            offset: 2,
            observed: Some(0x55),
        })
    );
}

#[test]
fn rejects_unestablished_repeated_ff60_without_recovery() {
    let bytes = [
        0x00, 0xff, 0x60, 0x01, 0x11, 0x01, 0xff, 0x60, 0x01, 0x22, 0x01, 0x90, 0x3c, 0x40, 0x20,
        0x01,
    ];
    assert!(matches!(
        walk(&bytes),
        Err(MixedEventWalkError::ContextLengthMismatch {
            cursor: 0,
            offset: 3,
            expected: 6,
            observed: 1,
        })
    ));
}

#[test]
fn rejects_unsupported_state_and_malformed_bounds_transactionally() {
    assert!(matches!(
        walk(&[0x00, 0x3c]),
        Err(MixedEventWalkError::DataWithoutActiveState { cursor: 0, .. })
    ));
    assert!(matches!(
        walk(&[0x00, 0xa0]),
        Err(MixedEventWalkError::UnsupportedStatus { cursor: 0, .. })
    ));
    assert!(matches!(
        walk(&[0x81]),
        Err(MixedEventWalkError::TimingVlq { cursor: 0, .. })
    ));
    assert!(matches!(
        walk(&[0x00, 0x90, 0x3c, 0x40, 0x20, 0x81]),
        Err(MixedEventWalkError::MalformedNote { cursor: 0, .. })
    ));
    assert!(matches!(
        walk(&[0x00, 0x90, 0x3c, 0x40, 0x20, 0x81, 0x81, 0x81, 0x81, 0x00]),
        Err(MixedEventWalkError::MalformedNote { cursor: 0, .. })
    ));
    assert!(matches!(
        walk(&[0x00, 0xe0, 0x01]),
        Err(MixedEventWalkError::MalformedPitchBend { cursor: 0, .. })
    ));
    let bytes = [0; 2];
    assert!(matches!(
        walk_bounded_mixed_events(
            &bytes,
            MixedEventBounds {
                event_range: std::ops::Range { start: 2, end: 1 },
            },
            MixedEventTimingBasis::default(),
        ),
        Err(MixedEventWalkError::InvalidEventBounds { .. })
    ));
}

#[test]
fn rejects_position_overflow_without_partial_success() {
    assert!(matches!(
        walk_bounded_mixed_events(
            &[0x01, 0x90, 0x3c, 0x40, 0x20, 0x01],
            MixedEventBounds { event_range: 0..6 },
            MixedEventTimingBasis {
                previous_event_position: u32::MAX,
            },
        ),
        Err(MixedEventWalkError::PositionOverflow { cursor: 0, .. })
    ));
}

#[test]
fn authentic_track_9_walks_all_events_and_stops_exactly() {
    let bytes = fs::read(BASELINE).unwrap();
    let result = walk_bounded_mixed_events(
        &bytes,
        MixedEventBounds {
            event_range: 0x143c8..0x14957,
        },
        MixedEventTimingBasis::default(),
    )
    .unwrap();
    assert_eq!(result.logical_event_count(), 184);
    assert_eq!(family_counts(&result), [31, 1, 120, 32, 0]);
    assert_eq!(result.consumed_range, 0x143c8..0x14957);
    assert_eq!(result.items.last().unwrap().logical_event_count(), 1);
    let pressure_entries = result
        .items
        .iter()
        .filter_map(|item| match item {
            MixedEventItem::Event(event) => match &event.event {
                MixedEventKind::ChannelPressure { entry_tag, .. } => Some(entry_tag.is_some()),
                _ => None,
            },
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(pressure_entries.len(), 32);
    assert_eq!(pressure_entries.iter().filter(|entry| **entry).count(), 1);
    assert!(result.items.iter().any(|item| matches!(
        item,
        MixedEventItem::PatchToNote(transition) if transition.context.is_some()
    )));
    assert!(result.items.iter().any(|item| matches!(
        item,
        MixedEventItem::Event(event)
            if matches!(event.event, MixedEventKind::ContextMediatedNote(_))
    )));
}

#[test]
fn authentic_track_14_walks_all_events_and_stops_exactly() {
    let bytes = fs::read(BASELINE).unwrap();
    let result = walk_bounded_mixed_events(
        &bytes,
        MixedEventBounds {
            event_range: 0x14e26..0x15ed4,
        },
        MixedEventTimingBasis::default(),
    )
    .unwrap();
    assert_eq!(result.logical_event_count(), 601);
    assert_eq!(family_counts(&result), [227, 0, 272, 0, 102]);
    assert_eq!(result.consumed_range, 0x14e26..0x15ed4);
    let bend_entries = result
        .items
        .iter()
        .filter_map(|item| match item {
            MixedEventItem::Event(event) => match &event.event {
                MixedEventKind::PitchBend { entry_tag, .. } => Some(entry_tag.is_some()),
                _ => None,
            },
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(bend_entries.len(), 102);
    assert_eq!(bend_entries.iter().filter(|entry| **entry).count(), 9);
    let MixedEventItem::Event(last) = result.items.last().unwrap() else {
        panic!("expected final Controller")
    };
    assert!(matches!(last.event, MixedEventKind::Controller(_)));
}
