use phoenix::mixed_event::{
    walk_bounded_mixed_events, MixedEventBounds, MixedEventItem, MixedEventKind,
    MixedEventTimingBasis, MixedEventWalkError,
};

#[path = "support/reference_smf.rs"]
mod reference_smf;

fn bridge() -> Vec<u8> {
    vec![
        4, 255, 96, 7, 35, 96, 0, 114, 108, 131, 76, 1, 255, 65, 5, 0, 1, 0, 1, 58,
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
fn ff60_ff41_retains_both_intervals_and_complete_provenance() {
    let mut b = vec![0xaa, 0xbb];
    // Nonzero origin/range and multi-byte intervals avoid a sample-only rule.
    let mut event = bridge();
    event.splice(11..12, [0x81, 3]);
    event.splice(0..1, [0x81, 2]);
    b.extend(event);
    b.push(0xcc);
    let w = walk_bounded_mixed_events(
        &b,
        MixedEventBounds { event_range: 2..24 },
        MixedEventTimingBasis {
            previous_event_position: 10,
        },
    )
    .unwrap();
    assert_eq!(w.consumed_range, 2..24);
    assert_eq!(w.logical_event_count(), 1);
    let MixedEventItem::Event(e) = &w.items[0] else {
        panic!()
    };
    assert_eq!(e.position, 271); // 10 + 130 + 131, each exactly once.
    let MixedEventKind::ContextMediatedController(c) = &e.event else {
        panic!()
    };
    assert_eq!(c.representation_range, 2..24);
    assert_eq!(c.context.leading_timing.range, 2..4);
    assert_eq!(c.context.leading_timing.bytes, [0x81, 2]);
    assert_eq!(c.context.context.range, 4..14);
    assert_eq!(c.context.context.tag_range, 4..6);
    assert_eq!(c.context.context.payload_length.offset, 6);
    assert_eq!(c.context.context.payload.range, 7..14);
    assert_eq!(c.context.context.payload.bytes, &b[7..14]);
    assert_eq!(c.controller.record_range, 14..24);
    assert_eq!(c.controller.timing_delta.range, 14..16);
    assert_eq!(c.controller.timing_delta.bytes, [0x81, 3]);
    assert_eq!(c.controller.event_tag_range, 16..18);
    assert_eq!(c.controller.context.range, 19..22);
    assert_eq!(c.controller.context.bytes, [0, 1, 0]);
    assert_eq!(
        (
            c.controller.controller_number.offset,
            c.controller.controller_number.value
        ),
        (22, 1)
    );
    assert_eq!(
        (
            c.controller.controller_value.offset,
            c.controller.controller_value.value
        ),
        (23, 58)
    );
    // One logical Controller plus one explicitly retained context, not two MIDI events.
    assert_eq!(w.logical_event_count() + 1, 2);
}

#[test]
fn ff60_ff41_preserves_none_continuation_and_explicit_successors() {
    for prefix in [
        vec![],
        vec![0, 0x90, 60, 100, 64, 1],
        vec![0, 0xe0, 0, 64],
        vec![0, 0xb3, 7, 80],
    ] {
        let mut b = prefix;
        b.extend(bridge());
        assert!(walk(&b, 0).is_ok());
        let mut compact = b.clone();
        compact.extend([0, 60, 100, 64, 1]);
        assert!(matches!(
            walk(&compact, 0),
            Err(MixedEventWalkError::DataWithoutActiveState { .. })
        ));
        b.extend([2, 0x90, 61, 99, 64, 3, 4, 62, 98, 64, 2]);
        let w = walk(&b, 0).unwrap();
        assert!(
            matches!(&w.items[w.items.len()-1], MixedEventItem::Event(e) if matches!(e.event, MixedEventKind::Note(_)))
        );
    }
    for number in [0, 127] {
        let mut b = bridge();
        b[7..11].copy_from_slice(&[1, 2, 3, 4]); // Opaque tail is not specimen dispatch.
        b[18] = number;
        b[19] = 127 - number;
        assert!(walk(&b, 0).is_ok());
    }
}

#[test]
fn ff60_ff41_refuses_unsupported_neighbors_truncation_and_overflow() {
    let b = bridge();
    for cut in 1..b.len() {
        assert!(walk(&b[..cut], 0).is_err(), "cut {cut}");
    }
    for (at, value) in [
        (3, 6),
        (3, 8),
        (4, 0x57),
        (5, 0x7f),
        (6, 1),
        (13, 0x60),
        (14, 4),
        (18, 128),
        (19, 128),
    ] {
        let mut bad = b.clone();
        bad[at] = value;
        assert!(walk(&bad, 0).is_err(), "mutation {at}/{value}");
    }
    for suffix in [vec![0], vec![0x81], vec![0, 60, 100, 64, 1]] {
        let mut bad = b.clone();
        bad.extend(suffix);
        assert!(walk(&bad, 0).is_err());
    }
    let mut consecutive = b[..11].to_vec();
    consecutive.extend(&b);
    assert!(walk(&consecutive, 0).is_err());
    let mut overlong = b.clone();
    overlong.splice(11..12, [0x81, 0x81, 0x81, 0x81, 0]);
    assert!(walk(&overlong, 0).is_err());
    for previous in [u32::MAX, u32::MAX - 4] {
        assert!(matches!(
            walk(&b, previous),
            Err(MixedEventWalkError::PositionOverflow { .. })
        ));
    }
}

#[test]
fn ff60_ff41_does_not_expand_descriptor166_routing() {
    use phoenix::bounded_routing::{
        validate_bounded_routing_contexts, ContextKind, RoutingRefusal,
    };
    let b = bridge();
    let w = walk(&b, 128704).unwrap();
    assert_eq!(
        validate_bounded_routing_contexts(&w, 1),
        Err(RoutingRefusal::ConflictingContext {
            kind: ContextKind::Ff60,
            range: 4..11,
            bytes: b[4..11].to_vec(),
        })
    );
}

#[test]
fn authentic_xform_track9_walk_and_native_events_when_available() {
    use phoenix::{bounded_routing, bounded_sequence, sequence_container::parse_project_166};
    use sha2::{Digest, Sha256};
    let path = "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline";
    let Ok(b) = std::fs::read(path) else { return };
    let hash = format!("{:x}", Sha256::digest(&b));
    assert_eq!(
        hash,
        "e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132"
    );
    let project = parse_project_166(&b).unwrap();
    let bounds = project.sequences[0]
        .validated_track_event_bounds(8)
        .unwrap()
        .event_range;
    assert_eq!(bounds, 0xb168..0xcf38);
    let w = walk_bounded_mixed_events(
        &b,
        MixedEventBounds {
            event_range: bounds.clone(),
        },
        Default::default(),
    )
    .unwrap();
    assert_eq!(w.consumed_range, bounds);
    let (mut notes, mut cc, mut bends, mut contexts) = (0, 0, 0, 0);
    let mut expected = Vec::new();
    for item in &w.items {
        let MixedEventItem::Event(e) = item else {
            panic!("unexpected Patch")
        };
        let (status, data) = match &e.event {
            MixedEventKind::Note(n) => {
                notes += 1;
                (0x9e, vec![n.pitch.value, n.attack_velocity.value])
            }
            MixedEventKind::ContextMediatedNote(n) => {
                notes += 1;
                contexts += 1;
                (0x9e, vec![n.note.pitch.value, n.note.attack_velocity.value])
            }
            MixedEventKind::Controller(c) => {
                cc += 1;
                (
                    0xbe,
                    vec![c.controller_number.value, c.controller_value.value],
                )
            }
            MixedEventKind::ContextMediatedController(c) => {
                cc += 1;
                contexts += 1;
                assert_eq!(e.position, 128709);
                assert_eq!(c.representation_range, 0xc490..0xc4a4);
                (
                    0xbe,
                    vec![
                        c.controller.controller_number.value,
                        c.controller.controller_value.value,
                    ],
                )
            }
            MixedEventKind::PitchBend { entry, .. } => {
                bends += 1;
                (0xee, vec![entry.pitch_lsb.value, entry.pitch_msb.value])
            }
            _ => panic!("unexpected event: {:?}", e.event),
        };
        expected.push((e.position, status, data));
    }
    assert_eq!((notes, cc, bends, contexts), (159, 444, 806, 4));
    let raw = u32::from_be_bytes(b[0xb15c..0xb160].try_into().unwrap()) as usize;
    assert_eq!(w.logical_event_count() + contexts, raw);
    assert_eq!(raw, 1413);
    assert!(bounded_routing::decode_bounded_routing(&b)
        .unwrap()
        .iter()
        .any(|r| r.structural_ordinal == 0
            && r.descriptor_ordinal == 10
            && matches!(
                r.result,
                Err(bounded_routing::RoutingRefusal::ConflictingContext { .. })
            )));
    assert_eq!(
        bounded_sequence::build_bounded_sequence_manifest(&b, 0, 480)
            .unwrap_err()
            .stage,
        "conductor prelude"
    );
    let reference = "/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/MIDI Exports from SVP/xForm Multitrack";
    if let Ok(midi) = std::fs::read(reference) {
        assert_eq!(
            format!("{:x}", Sha256::digest(&midi)),
            "872272ce43c389474fdb408416ad2406fa9a63de5e1d8bb94e0f6bbced7a1241"
        );
        let tracks = reference_smf::midi_tracks(&midi);
        let native: Vec<_> = tracks[9]
            .iter()
            .filter(|(_, s, d)| *s == 0xbe || *s == 0xee || (*s == 0x9e && d[1] != 0))
            .cloned()
            .collect();
        assert_eq!(expected, native); // Complete joint order, not source ordinal inference.
    }
    assert_eq!(
        format!("{:x}", Sha256::digest(std::fs::read(path).unwrap())),
        hash
    );
}
