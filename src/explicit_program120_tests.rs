use super::*;

fn walk(b: &[u8]) -> Result<MixedEventWalk<'_>, MixedEventWalkError> {
    walk_bounded_120_single_events(
        b,
        MixedEventBounds {
            event_range: 0..b.len(),
        },
        Default::default(),
    )
}
#[test]
fn explicit_program120_values_timing_consumption_and_provenance() {
    let b = [0x81, 0x00, 0xc0, 0, 7, 0xc0, 127];
    let w = walk(&b).unwrap();
    assert_eq!(w.consumed_range, 0..7);
    assert_eq!(w.logical_event_count(), 2);
    for (item, position, range, value, offset) in [
        (&w.items[0], 128, 0..4, 0, 3),
        (&w.items[1], 135, 4..7, 127, 6),
    ] {
        let MixedEventItem::Event(e) = item else {
            panic!("event")
        };
        let MixedEventKind::Program(p) = &e.event else {
            panic!("program")
        };
        assert_eq!(e.position, position);
        assert_eq!(p.representation_range, range);
        assert_eq!(p.program, LocatedByte { value, offset });
        assert_eq!(p.status.value, 0xc0);
        assert_eq!(p.timing_delta.range.end, p.status.offset);
    }
    assert!(matches!(
        walk_bounded_mixed_events(
            &b,
            MixedEventBounds {
                event_range: 0..b.len()
            },
            Default::default()
        ),
        Err(MixedEventWalkError::UnsupportedStatus { observed: 0xc0, .. })
    ));
}
#[test]
fn explicit_program120_resets_continuation_and_preserves_explicit_successors() {
    let b = [
        0, 0x90, 60, 100, 64, 10, 5, 0xc0, 41, 2, 0x90, 61, 90, 64, 8, 3, 0xe0, 0, 64,
    ];
    let w = walk(&b).unwrap();
    assert_eq!(w.logical_event_count(), 4);
    assert_eq!(w.consumed_range, 0..b.len());
    let b = [0, 0xc0, 41, 0, 61, 90, 64, 8];
    assert!(matches!(
        walk(&b),
        Err(MixedEventWalkError::DataWithoutActiveState { .. })
    ));
    let b = [0, 0x90, 60, 100, 64, 10, 2, 61, 90, 64, 8];
    assert_eq!(
        walk(&b).unwrap(),
        walk_bounded_mixed_events(
            &b,
            MixedEventBounds {
                event_range: 0..b.len()
            },
            Default::default()
        )
        .unwrap()
    );
}
#[test]
fn explicit_program120_malformed_variants_refuse() {
    for b in [
        vec![0, 0xc0],
        vec![0, 0xc0, 128],
        vec![0x80],
        vec![0, 0xc0, 1, 0],
        vec![0, 0xc0, 1, 0, 2],
    ] {
        assert!(walk(&b).is_err(), "{b:?}");
    }
    for status in 0xc1..=0xcf {
        assert!(walk(&[0, status, 1]).is_err());
    }
    assert!(walk_bounded_120_single_events(
        &[1, 0xc0, 1],
        MixedEventBounds { event_range: 0..3 },
        MixedEventTimingBasis {
            previous_event_position: u32::MAX
        }
    )
    .is_err());
}
