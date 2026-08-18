use std::fs;

use phoenix::mixed_event::{
    walk_bounded_mixed_events, MixedEventBounds, MixedEventItem, MixedEventKind,
    MixedEventTimingBasis, MixedEventWalkError,
};

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
            MixedEventItem::PatchToNote(_) => {
                counts[0] += 1;
                counts[1] += 1;
            }
            MixedEventItem::Event(event) => match &event.event {
                MixedEventKind::Note(_) | MixedEventKind::ContextMediatedNote(_) => counts[0] += 1,
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
fn rejects_repeated_ff60_without_recovery() {
    let bytes = [
        0x00, 0xff, 0x60, 0x01, 0x11, 0x01, 0xff, 0x60, 0x01, 0x22, 0x01, 0x90, 0x3c, 0x40, 0x20,
        0x01,
    ];
    assert!(matches!(
        walk(&bytes),
        Err(MixedEventWalkError::PatchContextMismatch {
            cursor: 0,
            offset: 6,
            observed: Some(0xff),
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
