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
    assert_eq!(patch.patch.representation_range, patch_range);
    assert_eq!(patch.patch.position.value, positions[0]);
    assert_eq!(patch.position, positions[0]);

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
        patch.patch.representation_range.end,
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
fn authentic_bells_track_6_consumes_and_uses_retained_controller_timing_basis() {
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
    let mut controller_position = None;
    for item in &result.items {
        match item {
            MixedEventItem::PatchToNote(transition) => {
                assert_eq!(transition.patch.position.value, 160);
                assert_eq!(transition.patch_position, 290);
                assert_eq!(transition.first_note_position, 71_903);
            }
            MixedEventItem::Event(event) => match &event.event {
                MixedEventKind::ContextMediatedNote(note) => {
                    single_lengths.push(note.context.payload_length.value);
                    if note.representation_range.start >= 0x121ba {
                        residual_single += 1;
                    }
                }
                MixedEventKind::DoubleContextMediatedNote(_) => double += 1,
                MixedEventKind::Controller(controller) => {
                    assert_eq!(controller.controller_number.value, 7);
                    assert_eq!(controller.controller_value.value, 127);
                    controller_position = Some(event.position);
                }
                _ => {}
            },
            MixedEventItem::Patch(_) => {}
        }
    }
    assert_eq!(residual_single, 6);
    assert_eq!(double, 3);
    assert_eq!(controller_position, Some(130));
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
    assert_eq!(last_position, 118_702);
}

fn synthetic_patch_controller_note() -> Vec<u8> {
    vec![
        0x00, 0xff, 0x7c, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x01, 0xff, 0x41, 0x05,
        0x00, 0x01, 0x00, 0x07, 0x7f, 0x02, 0x90, 0x3c, 0x40, 0x20, 0x01,
    ]
}

fn synthetic_patch_note(raw_patch_component: u8) -> Vec<u8> {
    vec![
        raw_patch_component,
        0xff,
        0x7c,
        0x07,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x19,
        0x02,
        0x90,
        0x3c,
        0x40,
        0x20,
        0x01,
    ]
}

fn synthetic_controller() -> [u8; 9] {
    [0x0a, 0xff, 0x41, 0x05, 0x00, 0x00, 0x00, 0x07, 0x7f]
}

#[test]
fn nonzero_timing_basis_accumulates_the_raw_patch_component() {
    let bytes = synthetic_patch_note(5);
    let result = walk_bounded_mixed_events(
        &bytes,
        MixedEventBounds {
            event_range: 0..bytes.len(),
        },
        MixedEventTimingBasis {
            previous_event_position: 100,
        },
    )
    .unwrap();
    let MixedEventItem::PatchToNote(transition) = &result.items[0] else {
        panic!("expected coupled Patch")
    };
    assert_eq!(transition.patch.position.value, 5);
    assert_eq!(transition.patch_position, 105);
    assert_eq!(transition.first_note_position, 107);
}

#[test]
fn controller_to_patch_note_retains_the_controller_position() {
    let mut bytes = synthetic_controller().to_vec();
    bytes.extend(synthetic_patch_note(5));
    let result = walk(&bytes).unwrap();
    let MixedEventItem::Event(controller) = &result.items[0] else {
        panic!("expected Controller")
    };
    assert_eq!(controller.position, 10);
    let MixedEventItem::PatchToNote(transition) = &result.items[1] else {
        panic!("expected coupled Patch")
    };
    assert_eq!(transition.patch.position.value, 5);
    assert_eq!(transition.patch_position, 15);
    assert_eq!(transition.first_note_position, 17);
}

#[test]
fn controller_to_patch_controller_note_accumulates_each_position() {
    let mut patch = synthetic_patch_controller_note();
    patch[0] = 5;
    let mut bytes = synthetic_controller().to_vec();
    bytes.extend(patch);
    let result = walk(&bytes).unwrap();
    let MixedEventItem::Patch(patch) = &result.items[1] else {
        panic!("expected positioned Patch")
    };
    assert_eq!(patch.patch.position.value, 5);
    assert_eq!(patch.position, 15);
    let MixedEventItem::Event(controller) = &result.items[2] else {
        panic!("expected Patch-following Controller")
    };
    assert_eq!(controller.position, 16);
    let MixedEventItem::Event(note) = &result.items[3] else {
        panic!("expected Patch-following Note")
    };
    assert_eq!(note.position, 18);
}

#[test]
fn patch_position_overflow_fails_transactionally() {
    let bytes = synthetic_patch_note(1);
    assert!(matches!(
        walk_bounded_mixed_events(
            &bytes,
            MixedEventBounds {
                event_range: 0..bytes.len(),
            },
            MixedEventTimingBasis {
                previous_event_position: u32::MAX,
            },
        ),
        Err(MixedEventWalkError::PositionOverflow {
            cursor: 0,
            left: u32::MAX,
            right: 1,
        })
    ));

    let direct = synthetic_patch_note(5);
    assert!(matches!(
        walk_bounded_mixed_events(
            &direct,
            MixedEventBounds {
                event_range: 0..direct.len(),
            },
            MixedEventTimingBasis {
                previous_event_position: u32::MAX - 5,
            },
        ),
        Err(MixedEventWalkError::PositionOverflow {
            cursor: 0,
            left: u32::MAX,
            right: 2,
        })
    ));

    let mut controller = synthetic_patch_controller_note();
    controller[0] = 5;
    assert!(matches!(
        walk_bounded_mixed_events(
            &controller,
            MixedEventBounds {
                event_range: 0..controller.len(),
            },
            MixedEventTimingBasis {
                previous_event_position: u32::MAX - 5,
            },
        ),
        Err(MixedEventWalkError::PositionOverflow {
            cursor: 11,
            left: u32::MAX,
            right: 1,
        })
    ));
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

fn patch_controller_chain(count: usize, context_length: Option<u8>) -> Vec<u8> {
    let base = synthetic_patch_controller_note();
    let mut bytes = base[..11].to_vec();
    bytes[0] = 3;
    for _ in 0..count {
        bytes.extend(synthetic_controller()); // delta 10, context 00 00 00
    }
    if let Some(length) = context_length {
        bytes.extend([4, 0xff, 0x60, length]);
        bytes.extend(vec![0x55; usize::from(length)]);
    }
    bytes.extend([2, 0x90, 60, 64, 32, 1]);
    bytes
}

#[test]
fn bounded_controller_chains_preserve_timing_provenance_and_note_state() {
    for (count, context) in [
        (1, None),
        (2, None),
        (3, None),
        (1, Some(7)),
        (1, Some(8)),
        (2, Some(7)),
        (2, Some(8)),
    ] {
        let chain = patch_controller_chain(count, context);
        let mut bytes = vec![0xaa; 5];
        bytes.extend(&chain);
        // Compact Note continuation proves the terminating Note established state.
        bytes.extend([1, 61, 65, 33, 2]);
        let w = walk_bounded_mixed_events(
            &bytes,
            MixedEventBounds {
                event_range: 5..bytes.len(),
            },
            MixedEventTimingBasis {
                previous_event_position: 100,
            },
        )
        .unwrap();
        let MixedEventItem::Patch(p) = &w.items[0] else {
            panic!("Patch")
        };
        assert_eq!(p.position, 103);
        assert_eq!(p.patch.representation_range, 5..16);
        for (i, item) in w.items[1..=count].iter().enumerate() {
            let MixedEventItem::Event(e) = item else {
                panic!("Controller")
            };
            let MixedEventKind::Controller(c) = &e.event else {
                panic!("Controller")
            };
            assert_eq!(e.position, 113 + 10 * i as u32);
            assert_eq!(c.record_range, 16 + 9 * i..25 + 9 * i);
            assert_eq!(c.context.bytes, &[0, 0, 0]);
            assert_eq!(
                (c.controller_number.value, c.controller_value.value),
                (7, 127)
            );
        }
        let MixedEventItem::Event(n) = &w.items[count + 1] else {
            panic!("Note")
        };
        let expected = 103 + 10 * count as u32 + 2 + if context.is_some() { 4 } else { 0 };
        assert_eq!(n.position, expected);
        match (&n.event, context) {
            (MixedEventKind::Note(n), None) => {
                assert_eq!(n.representation_range, 16 + 9 * count..5 + chain.len())
            }
            (MixedEventKind::ContextMediatedNote(n), Some(len)) => {
                assert_eq!(n.representation_range, 16 + 9 * count..5 + chain.len());
                assert_eq!(n.context.payload.bytes, vec![0x55; usize::from(len)]);
                assert_eq!((n.leading_timing.value, n.final_timing.value), (4, 2));
            }
            _ => panic!("unexpected Note form"),
        }
        let MixedEventItem::Event(last) = w.items.last().unwrap() else {
            panic!("continuation")
        };
        assert_eq!(last.position, expected + 1);
    }
}

#[test]
fn bounded_chain_rejects_unsupported_successors_without_scanning() {
    for status in [0xf0, 0xd0, 0xe0, 0x91, 0x40] {
        let mut b = synthetic_patch_controller_note();
        b[21] = status;
        b.extend([0, 0x90, 60, 64, 32, 1]);
        assert!(walk(&b).is_err());
    }
    assert!(walk(&patch_controller_chain(4, None)).is_err());
    assert!(walk(&patch_controller_chain(3, Some(7))).is_err());
    for len in [0, 1, 6, 9] {
        assert!(walk(&patch_controller_chain(1, Some(len))).is_err());
    }
    for tag in [0x41, 0x7c, 0x60] {
        let mut b = patch_controller_chain(1, Some(7));
        let note_status = b.len() - 5;
        b[note_status] = 0xff;
        b[note_status + 1] = tag;
        b.extend([0, 0x90, 60, 64, 32, 1]);
        assert!(walk(&b).is_err());
    }
    for b in [
        patch_controller_chain(2, None),
        patch_controller_chain(2, Some(8)),
    ] {
        for end in 12..b.len() {
            assert!(
                walk(&b[..end]).is_err(),
                "accepted truncated chain at {end}"
            );
        }
    }
    let mut malformed = patch_controller_chain(2, None);
    malformed[23] = 4; // second Controller's length must be five
    assert!(walk(&malformed).is_err());
    let b = patch_controller_chain(2, Some(8));
    assert!(walk_bounded_mixed_events(
        &b,
        MixedEventBounds {
            event_range: 0..b.len()
        },
        MixedEventTimingBasis {
            previous_event_position: u32::MAX - 4
        }
    )
    .is_err());
}

#[test]
fn terminal_patch_requires_exact_complete_core_and_preserves_position() {
    let b = synthetic_patch_note(3);
    let core = &b[..11];
    let w = walk_bounded_mixed_events(
        &b,
        MixedEventBounds { event_range: 0..11 },
        MixedEventTimingBasis {
            previous_event_position: 100,
        },
    )
    .unwrap();
    let MixedEventItem::Patch(p) = &w.items[0] else {
        panic!("terminal Patch")
    };
    assert_eq!(p.position, 103);
    assert_eq!(p.patch.representation_range, 0..11);
    assert_eq!(p.patch.program_change.value, 25);
    assert_eq!(w.consumed_range, 0..11);
    for end in 1..11 {
        assert!(walk(&core[..end]).is_err());
    }
    for suffix in [
        vec![0],
        vec![0, 0xf0, 0],
        vec![0xff, 0xff, 0xff, 0x7f, 0xff, 0x2f, 0],
    ] {
        let mut x = core.to_vec();
        x.extend(suffix);
        assert!(walk(&x).is_err());
    }
}

#[test]
fn authentic_composition_extensions_complete_only_the_audited_tracks() {
    let bytes = fs::read(BASELINE).unwrap();
    let project = parse_project_166(&bytes).unwrap();
    let audited = [
        (0, 5, 135, 96151),
        (0, 10, 72, 63360),
        (0, 11, 123, 140154),
        (0, 12, 73, 63363),
        (8, 0, 106, 32400),
        (12, 1, 255, 101286),
        (12, 2, 255, 101292),
        (0, 0, 65, 99818),
        (9, 1, 1, 0),
        (12, 9, 28, 69120),
    ];
    for (si, ti, count, position) in audited {
        let range = project.sequences[si].track_pairs[ti]
            .validated_event_bounds()
            .unwrap()
            .event_range;
        let w = walk_bounded_mixed_events(
            &bytes,
            MixedEventBounds {
                event_range: range.clone(),
            },
            Default::default(),
        )
        .unwrap();
        assert_eq!(w.consumed_range, range);
        assert_eq!(w.logical_event_count(), count);
        let last = match w.items.last().unwrap() {
            MixedEventItem::Event(e) => e.position,
            MixedEventItem::Patch(p) => p.position,
            MixedEventItem::PatchToNote(p) => p.first_note_position,
        };
        assert_eq!(last, position);
    }
    let mut failures = Vec::new();
    for (si, s) in project.sequences.iter().enumerate() {
        for (ti, t) in s.track_pairs.iter().enumerate() {
            let r = t.validated_event_bounds().unwrap().event_range;
            if walk_bounded_mixed_events(
                &bytes,
                MixedEventBounds { event_range: r },
                Default::default(),
            )
            .is_err()
            {
                failures.push((si, ti));
            }
        }
    }
    assert_eq!(failures, [(0, 1), (0, 8), (7, 5), (7, 6)]);
    assert!(matches!(
        project.sequences[8].track_associations,
        phoenix::sequence_container::TrackAssociations::Unresolved {
            descriptor_count: 11,
            pair_count: 10
        }
    ));
}

fn initial_context_patch_note(length: u8) -> Vec<u8> {
    let mut b = vec![0, 0xff, 0x60, length];
    b.extend(vec![0x55; usize::from(length)]);
    b.extend(synthetic_patch_note(3));
    b
}

#[test]
fn initial_zero_context_preserves_direct_patch_note_and_continuation() {
    for length in [7, 8] {
        let original = initial_context_patch_note(length);
        let mut bytes = vec![0xaa; 5];
        bytes.extend(&original);
        bytes.extend([1, 61, 65, 33, 2]);
        let w = walk_bounded_mixed_events(
            &bytes,
            MixedEventBounds {
                event_range: 5..bytes.len(),
            },
            MixedEventTimingBasis {
                previous_event_position: 100,
            },
        )
        .unwrap();
        let MixedEventItem::PatchToNote(p) = &w.items[0] else {
            panic!("composition")
        };
        let c = p.initial_context.as_ref().unwrap();
        let patch_start = 9 + usize::from(length);
        assert_eq!(p.representation_range, 5..5 + original.len());
        assert_eq!(c.leading_timing.range, 5..6);
        assert_eq!(c.leading_timing.value, 0);
        assert_eq!(c.context.range, 6..patch_start);
        assert_eq!(c.context.payload.bytes, vec![0x55; usize::from(length)]);
        assert_eq!(p.patch.representation_range, patch_start..patch_start + 13);
        assert_eq!(p.patch.position.range, patch_start..patch_start + 1);
        assert_eq!(p.patch.program_change.value, 25);
        assert_eq!(
            p.first_note.representation_range,
            patch_start + 12..patch_start + 17
        );
        assert_eq!((p.patch_position, p.first_note_position), (103, 105));
        assert!(p.context.is_none());
        let MixedEventItem::Event(e) = &w.items[1] else {
            panic!("Note continuation")
        };
        assert_eq!(e.position, 106);
        assert!(
            matches!(&e.event,MixedEventKind::Note(n) if n.status.is_none() && n.pitch.value==61)
        );
    }
}

#[test]
fn initial_context_patch_rejects_every_unsupported_boundary() {
    let base = initial_context_patch_note(7);
    for length in [0, 1, 6, 9, 255] {
        assert!(walk(&initial_context_patch_note(length)).is_err());
    }
    let mut b = base.clone();
    b[0] = 1;
    assert!(walk(&b).is_err());
    for end in 1..base.len() {
        assert!(walk(&base[..end]).is_err(), "truncation {end}");
    }
    // A valid-looking later composition cannot resynchronize a bad current one.
    for (offset, value) in [
        (12, 0xf0),
        (13, 0x41),
        (14, 255),
        (14, 0),
        (23, 0x91),
        (23, 0xf0),
        (23, 0x40),
    ] {
        let mut b = base.clone();
        b[offset] = value;
        b.extend([0, 0x90, 60, 64, 32, 1]);
        assert!(walk(&b).is_err(), "mutation at {offset}");
    }
    let context = base[..11].to_vec();
    for successor in [
        synthetic_patch_controller_note(),
        synthetic_controller().to_vec(),
        vec![0, 0xf0, 1],
    ] {
        let mut b = context.clone();
        b.extend(successor);
        assert!(walk(&b).is_err());
    }
    let mut b = context.clone();
    b.extend(&base);
    assert!(walk(&b).is_err());
    let core = synthetic_patch_note(3)[..11].to_vec();
    let mut b = context.clone();
    b.extend(&core);
    assert!(walk(&b).is_err());
    let mut b = context;
    b.extend(&core);
    b.extend(&core);
    assert!(walk(&b).is_err());
    // A context after the Patch is not the approved direct Note form.
    let mut b = base.clone();
    b.splice(22..23, [0, 0xff, 0x60, 7, 0, 0, 0, 0, 0, 0, 0, 0]);
    assert!(walk(&b).is_err());
    // Neither initial position nor None state alone is enough: both are required.
    let mut b = vec![0, 0x90, 60, 64, 32, 1];
    b.extend(&base);
    assert!(walk(&b).is_err());
    let mut b = synthetic_controller().to_vec();
    b.extend(&base);
    assert!(walk(&b).is_err());
    assert!(walk_bounded_mixed_events(
        &base,
        MixedEventBounds {
            event_range: 0..base.len()
        },
        MixedEventTimingBasis {
            previous_event_position: u32::MAX - 2
        }
    )
    .is_err());
    assert!(walk_bounded_mixed_events(
        &base,
        MixedEventBounds {
            event_range: 0..usize::MAX
        },
        Default::default()
    )
    .is_err());
}

#[test]
fn terminal_two_patch_cores_preserve_timing_and_provenance() {
    let mut b = vec![0, 0x90, 60, 64, 32, 1];
    b.extend(&synthetic_patch_note(3)[..11]);
    b.extend(&synthetic_patch_note(7)[..11]);
    let w = walk_bounded_mixed_events(
        &b,
        MixedEventBounds {
            event_range: 0..b.len(),
        },
        MixedEventTimingBasis {
            previous_event_position: 100,
        },
    )
    .unwrap();
    for (item, range, position) in [(&w.items[1], 6..17, 103), (&w.items[2], 17..28, 110)] {
        let MixedEventItem::Patch(p) = item else {
            panic!("standalone Patch")
        };
        assert_eq!(p.position, position);
        assert_eq!(p.patch.representation_range, range);
        assert_eq!(p.patch.program_change.value, 25);
        assert_eq!(p.patch.pre_name_context.bytes, [0, 0, 0, 0, 0]);
    }
    assert_eq!(w.logical_event_count(), 3);
    assert_eq!(w.consumed_range, 0..b.len());
}

#[test]
fn terminal_pair_rejects_truncation_extra_bytes_and_recovery() {
    let core = synthetic_patch_note(3)[..11].to_vec();
    let mut pair = core.clone();
    pair.extend(&core);
    for end in 1..pair.len() {
        if end != 11 {
            assert!(walk(&pair[..end]).is_err());
        }
    }
    for tail in [
        vec![0],
        vec![0, 0xf0, 0],
        core.clone(),
        vec![0, 0x90, 60, 64, 32, 1],
        vec![0xff, 0xff, 0xfc, 0x1f, 0xff, 0x2f, 0],
    ] {
        let mut b = pair.clone();
        b.extend(tail);
        assert!(walk(&b).is_err());
    }
    for offset in [3, 14] {
        let mut b = pair.clone();
        b[offset] = 255;
        b.extend(&pair);
        assert!(walk(&b).is_err());
    }
    for offset in [1, 12] {
        let mut b = pair.clone();
        b[offset] = 0xf0;
        b.extend(&pair);
        assert!(walk(&b).is_err());
    }
    assert!(walk_bounded_mixed_events(
        &pair,
        MixedEventBounds {
            event_range: 0..pair.len() - 1
        },
        Default::default()
    )
    .is_err());
    assert!(walk_bounded_mixed_events(
        &pair,
        MixedEventBounds {
            event_range: 0..pair.len()
        },
        MixedEventTimingBasis {
            previous_event_position: u32::MAX - 4
        }
    )
    .is_err());
    // Actual data beyond the validated event end must remain untouched.
    let mut b = pair.clone();
    b.extend([0xff, 0xff, 0xfc, 0x1f, 0xff, 0x2f, 0]);
    assert_eq!(
        walk_bounded_mixed_events(
            &b,
            MixedEventBounds {
                event_range: 0..pair.len()
            },
            Default::default()
        )
        .unwrap()
        .consumed_range,
        0..pair.len()
    );
}

#[test]
fn authentic_remaining_forms_have_exact_successes_and_failure_frontiers() {
    let b = fs::read(BASELINE).unwrap();
    let p = parse_project_166(&b).unwrap();
    for (si, ti, count, position) in [(9, 0, 2, 480), (11, 4, 20, 33824), (17, 0, 38, 15117)] {
        let range = p.sequences[si].track_pairs[ti]
            .validated_event_bounds()
            .unwrap()
            .event_range;
        let w = walk_bounded_mixed_events(
            &b,
            MixedEventBounds {
                event_range: range.clone(),
            },
            Default::default(),
        )
        .unwrap();
        assert_eq!(w.consumed_range, range);
        assert_eq!(w.logical_event_count(), count);
        let last = match w.items.last().unwrap() {
            MixedEventItem::Patch(x) => x.position,
            MixedEventItem::Event(x) => x.position,
            _ => panic!("unexpected final item"),
        };
        assert_eq!(last, position);
    }
    let mut complete = 0;
    let mut failures = Vec::new();
    let mut per_sequence = Vec::new();
    for (si, s) in p.sequences.iter().enumerate() {
        let mut n = 0;
        for (ti, t) in s.track_pairs.iter().enumerate() {
            let range = t.validated_event_bounds().unwrap().event_range;
            match walk_bounded_mixed_events(
                &b,
                MixedEventBounds { event_range: range },
                Default::default(),
            ) {
                Ok(_) => {
                    complete += 1;
                    n += 1
                }
                Err(e) => failures.push((si, ti, e)),
            }
        }
        per_sequence.push(n);
    }
    assert_eq!(complete, 128);
    assert_eq!(
        (per_sequence[9], per_sequence[11], per_sequence[17]),
        (5, 6, 2)
    );
    assert_eq!(
        failures,
        vec![
            (
                0,
                1,
                MixedEventWalkError::UnsupportedStatus {
                    cursor: 0x7a25,
                    offset: 0x7a27,
                    observed: 0xf0
                }
            ),
            (
                0,
                8,
                MixedEventWalkError::PatchContextMismatch {
                    cursor: 0xc490,
                    offset: 0xc49c,
                    observed: Some(0xff)
                }
            ),
            (
                7,
                5,
                MixedEventWalkError::UnsupportedStatus {
                    cursor: 0x21476,
                    offset: 0x21478,
                    observed: 0xf0
                }
            ),
            (
                7,
                6,
                MixedEventWalkError::UnsupportedStatus {
                    cursor: 0x219db,
                    offset: 0x219dc,
                    observed: 0xf0
                }
            ),
        ]
    );
}
