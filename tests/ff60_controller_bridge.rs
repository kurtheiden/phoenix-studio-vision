use phoenix::mixed_event::{
    walk_bounded_mixed_events, MixedEventBounds, MixedEventItem, MixedEventKind,
    MixedEventTimingBasis, MixedEventWalkError,
};

fn bridge(leading: u8, successor: u8, status: u8) -> Vec<u8> {
    vec![
        leading, 0xff, 0x60, 7, 0x57, 0x7f, 0, 0x21, 0x42, 0x83, 0x4c, successor, status, 6, 8,
    ]
}
fn walk(
    b: &[u8],
    previous: u32,
) -> Result<phoenix::mixed_event::MixedEventWalk<'_>, MixedEventWalkError> {
    walk_bounded_mixed_events(
        b,
        MixedEventBounds {
            event_range: 0..b.len(),
        },
        MixedEventTimingBasis {
            previous_event_position: previous,
        },
    )
}

#[test]
fn bridge_retains_exact_ranges_and_both_intervals_with_nonzero_origin() {
    let b = [
        0xaa, 0xbb, 0x81, 2, 0xff, 0x60, 7, 0x57, 0x7f, 0, 0x21, 0x42, 0x83, 0x4c, 0x81, 3, 0xb0,
        6, 8, 0xcc,
    ];
    let w = walk_bounded_mixed_events(
        &b,
        MixedEventBounds { event_range: 2..19 },
        MixedEventTimingBasis {
            previous_event_position: 10,
        },
    )
    .unwrap();
    assert_eq!(w.consumed_range, 2..19);
    assert_eq!(w.logical_event_count(), 1); // context is a separate raw object
    let MixedEventItem::Event(e) = &w.items[0] else {
        panic!()
    };
    assert_eq!(e.position, 271); // 10 + 130 + 131
    let MixedEventKind::MidiController(c) = &e.event else {
        panic!()
    };
    assert_eq!(c.representation_range, 2..19);
    assert_eq!(c.timing_delta.range, 14..16);
    assert_eq!(c.timing_delta.bytes, [0x81, 3]);
    assert_eq!(c.status.offset, 16);
    assert_eq!(c.controller_number.offset, 17);
    assert_eq!(c.controller_value.offset, 18);
    let ctx = c.context.as_ref().unwrap();
    assert_eq!(ctx.leading_timing.range, 2..4);
    assert_eq!(ctx.leading_timing.bytes, [0x81, 2]);
    assert_eq!(ctx.context.range, 4..14);
    assert_eq!(ctx.context.tag_range, 4..6);
    assert_eq!(ctx.context.payload_length.offset, 6);
    assert_eq!(ctx.context.payload.range, 7..14);
    assert_eq!(ctx.context.payload.bytes, &b[7..14]);
}

#[test]
fn explicit_controller_establishes_compact_state_then_notes_reenter_normally() {
    for status in 0xb0..=0xbf {
        let mut b = bridge(2, 3, status);
        b.extend([5, 6, 12, 7, 0x90, 42, 120, 94, 87, 2, 43, 110, 64, 12]);
        let w = walk(&b, 10).unwrap();
        let positions: Vec<_> = w
            .items
            .iter()
            .map(|item| {
                let MixedEventItem::Event(e) = item else {
                    panic!()
                };
                e.position
            })
            .collect();
        assert_eq!(positions, [15, 20, 27, 29]);
        let MixedEventItem::Event(e) = &w.items[1] else {
            panic!()
        };
        let MixedEventKind::MidiController(c) = &e.event else {
            panic!()
        };
        assert!(c.context.is_none());
        assert!(!c.explicit_status);
        assert_eq!(c.status.value, status);
        assert_eq!(c.status.offset, 12);
        assert_eq!(c.representation_range, 15..18);
        assert_eq!(c.channel, (status & 15) + 1);
        assert!(
            matches!(&w.items[2], MixedEventItem::Event(e) if matches!(e.event, MixedEventKind::Note(_)))
        );
    }
}

#[test]
fn both_zero_and_nonzero_timing_components_are_added_once() {
    for (leading, successor) in [(0, 0), (0, 11), (13, 0), (13, 17)] {
        let b = bridge(leading, successor, 0xb0);
        let w = walk(&b, 9).unwrap();
        let MixedEventItem::Event(e) = &w.items[0] else {
            panic!()
        };
        assert_eq!(e.position, 9 + u32::from(leading) + u32::from(successor));
    }
}

#[test]
fn unsupported_context_lengths_families_and_nonexplicit_successors_refuse() {
    for len in [0, 1, 6, 8, 9] {
        let mut b = vec![2, 0xff, 0x60, len];
        b.extend([0x57, 0x7f, 0].into_iter().cycle().take(usize::from(len)));
        b.extend([3, 0xb0, 6, 8]);
        assert!(walk(&b, 0).is_err(), "length {len}");
    }
    for at in 4..7 {
        let mut b = bridge(2, 3, 0xb0);
        b[at] ^= 1;
        assert!(walk(&b, 0).is_err(), "prefix byte {at}");
    }
    // Other established Note/context compositions retain their existing grammar;
    // this new bridge must not admit compact CC, Program, Bend or Pressure.
    for successor in [6, 0x80, 0xc0, 0xd0, 0xe0, 0xf0, 0xff] {
        let b = bridge(2, 3, successor);
        assert!(walk(&b, 0).is_err(), "successor {successor:x}");
    }
    let mut b = vec![0, 0xb0, 6, 1];
    let mut unsupported = bridge(2, 3, 6);
    unsupported.remove(13);
    b.extend(unsupported); // inherited CC state cannot cross the context
    assert!(walk(&b, 0).is_err());
}

#[test]
fn truncation_bad_data_and_later_errors_refuse_transactionally_without_scanning() {
    let b = bridge(2, 3, 0xb0);
    for end in 1..b.len() {
        assert!(walk(&b[..end], 0).is_err(), "end {end}");
    }
    for at in [13, 14] {
        let mut bad = b.clone();
        bad[at] = 128;
        assert!(walk(&bad, 0).is_err());
    }
    let mut bad = b.clone();
    bad.extend([0, 0xc0, 12]);
    bad.extend(&b);
    assert!(matches!(
        walk(&bad, 0),
        Err(MixedEventWalkError::UnsupportedStatus { cursor: 15, .. })
    ));
    let mut timing = b[..11].to_vec();
    timing.extend([0x81, 0x81, 0x81, 0x81, 0, 0xb0, 6, 8]);
    assert!(matches!(
        walk(&timing, 0),
        Err(MixedEventWalkError::TimingVlq { .. })
    ));
}

#[test]
fn checked_arithmetic_rejects_either_clock_overflow() {
    assert!(matches!(
        walk(&bridge(2, 0, 0xb0), u32::MAX - 1),
        Err(MixedEventWalkError::PositionOverflow { .. })
    ));
    assert!(matches!(
        walk(&bridge(1, 2, 0xb0), u32::MAX - 2),
        Err(MixedEventWalkError::PositionOverflow { .. })
    ));
}

#[test]
fn prior_notes_and_controllers_retain_values_ranges_and_clocks() {
    let prior = [0, 0x90, 60, 100, 64, 12, 5, 0xb0, 10, 30, 3, 10, 31];
    let before = walk(&prior, 0).unwrap();
    let mut after = prior.to_vec();
    after.extend(bridge(2, 3, 0xb0));
    let after = walk(&after, 0).unwrap();
    assert_eq!(before.items, after.items[..before.items.len()]);
}
