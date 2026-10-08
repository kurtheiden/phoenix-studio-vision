use phoenix::mixed_event::{
    walk_bounded_mixed_events, MixedEventBounds, MixedEventItem, MixedEventKind,
    MixedEventTimingBasis,
};
use phoenix::sequence_container::parse_project_166;
use sha2::{Digest, Sha256};
const SOURCE: &str = "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline";
fn walk(
    b: &[u8],
) -> Result<phoenix::mixed_event::MixedEventWalk<'_>, phoenix::mixed_event::MixedEventWalkError> {
    walk_bounded_mixed_events(
        b,
        MixedEventBounds {
            event_range: 0..b.len(),
        },
        Default::default(),
    )
}
const SYSEX: [u8; 10] = [0, 0xf0, 7, 0, 0x41, 0x32, 0, 12, 93, 0xf7];
#[test]
fn authentic_happyone_all_sysex_payloads_ticks_ranges_and_source_counts() {
    let b = std::fs::read(SOURCE).unwrap();
    assert_eq!(
        format!("{:x}", Sha256::digest(&b)),
        "e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132"
    );
    let p = parse_project_166(&b).unwrap();
    for (pair, range, expected, notes, patches, contexts, declared, last) in [
        (
            5,
            0x21236..0x21714,
            EXPECTED_6.as_slice(),
            138,
            1,
            1,
            166,
            62160,
        ),
        (
            6,
            0x217df..0x22375,
            EXPECTED_7.as_slice(),
            519,
            0,
            0,
            521,
            64320,
        ),
    ] {
        let t = &p.sequences[7].track_pairs[pair];
        assert_eq!(t.validated_event_bounds().unwrap().event_range, range);
        let w = walk_bounded_mixed_events(
            &b,
            MixedEventBounds {
                event_range: range.clone(),
            },
            Default::default(),
        )
        .unwrap();
        assert_eq!(w.consumed_range, range);
        let mut observed = vec![];
        let mut note_count = 0;
        let mut patch_count = 0;
        let mut context_count = 0;
        for item in &w.items {
            match item {
                MixedEventItem::PatchToNote(n) => {
                    note_count += 1;
                    patch_count += 1;
                    context_count += usize::from(n.initial_context.is_some());
                }
                MixedEventItem::Patch(_) => patch_count += 1,
                MixedEventItem::Event(e) => match &e.event {
                    MixedEventKind::Note(_) => note_count += 1,
                    MixedEventKind::SysEx(s) => {
                        assert_eq!(s.status.value, 0xf0);
                        assert_eq!(s.payload_length.value, 7);
                        assert_eq!(s.payload_length.offset, s.status.offset + 1);
                        assert_eq!(
                            s.payload.range,
                            s.status.offset + 2..s.representation_range.end
                        );
                        assert_eq!(s.payload.bytes, &b[s.payload.range.clone()]);
                        assert_eq!(s.timing_delta.bytes, &b[s.timing_delta.range.clone()]);
                        observed.push((
                            s.representation_range.start,
                            s.representation_range.end,
                            s.status.offset,
                            s.timing_delta.value,
                            e.position,
                            s.payload.bytes.try_into().unwrap(),
                        ));
                    }
                    _ => panic!("unexpected family"),
                },
            }
        }
        assert_eq!(observed, expected);
        assert_eq!(
            (note_count, patch_count, context_count),
            (notes, patches, contexts)
        );
        assert_eq!(w.logical_event_count() + contexts, declared);
        let count_offset = t.primary.record_range.start + 7;
        assert_eq!(
            u32::from_be_bytes(b[count_offset..count_offset + 4].try_into().unwrap()) as usize,
            declared
        );
        let MixedEventItem::Event(e) = w.items.last().unwrap() else {
            panic!()
        };
        assert_eq!(e.position, last);
        assert_eq!(
            phoenix::bounded_sequence::build_bounded_sequence_manifest(&b, 7, 480)
                .unwrap_err()
                .stage,
            "conductor prelude"
        );
    }
}
type Row = (usize, usize, usize, u32, u32, [u8; 7]);
const EXPECTED_6: [Row; 26] = [
    (
        0x21476,
        0x21481,
        0x21478,
        540,
        33900,
        [0x00, 0x41, 0x32, 0x00, 0x0c, 0x5d, 0xf7],
    ),
    (
        0x21481,
        0x2148c,
        0x21483,
        357,
        34257,
        [0x00, 0x41, 0x32, 0x00, 0x0c, 0x5c, 0xf7],
    ),
    (
        0x214a0,
        0x214ab,
        0x214a2,
        307,
        35587,
        [0x00, 0x41, 0x32, 0x00, 0x0c, 0x5d, 0xf7],
    ),
    (
        0x214ab,
        0x214b5,
        0x214ac,
        104,
        35691,
        [0x00, 0x41, 0x32, 0x00, 0x0c, 0x5c, 0xf7],
    ),
    (
        0x214ef,
        0x214f9,
        0x214f0,
        62,
        39182,
        [0x00, 0x41, 0x32, 0x00, 0x0c, 0x5d, 0xf7],
    ),
    (
        0x21501,
        0x2150b,
        0x21502,
        1,
        39361,
        [0x00, 0x41, 0x32, 0x00, 0x0c, 0x5c, 0xf7],
    ),
    (
        0x2150b,
        0x21516,
        0x2150d,
        790,
        40151,
        [0x00, 0x41, 0x32, 0x00, 0x0c, 0x5d, 0xf7],
    ),
    (
        0x2152a,
        0x21535,
        0x2152c,
        720,
        41760,
        [0x00, 0x41, 0x32, 0x00, 0x0c, 0x5c, 0xf7],
    ),
    (
        0x21535,
        0x2153f,
        0x21536,
        87,
        41847,
        [0x00, 0x41, 0x32, 0x00, 0x0c, 0x5d, 0xf7],
    ),
    (
        0x2156f,
        0x21579,
        0x21570,
        99,
        45699,
        [0x00, 0x41, 0x32, 0x00, 0x0c, 0x5c, 0xf7],
    ),
    (
        0x2158d,
        0x21598,
        0x2158f,
        566,
        47366,
        [0x00, 0x41, 0x32, 0x00, 0x0c, 0x5d, 0xf7],
    ),
    (
        0x215a0,
        0x215ab,
        0x215a2,
        377,
        47897,
        [0x00, 0x41, 0x32, 0x00, 0x0c, 0x5c, 0xf7],
    ),
    (
        0x215ab,
        0x215b5,
        0x215ac,
        40,
        47937,
        [0x00, 0x41, 0x32, 0x00, 0x0c, 0x5d, 0xf7],
    ),
    (
        0x215c8,
        0x215d3,
        0x215ca,
        653,
        49373,
        [0x00, 0x41, 0x32, 0x00, 0x0c, 0x5c, 0xf7],
    ),
    (
        0x215da,
        0x215e5,
        0x215dc,
        322,
        49762,
        [0x00, 0x41, 0x32, 0x00, 0x0c, 0x5d, 0xf7],
    ),
    (
        0x215f2,
        0x215fd,
        0x215f4,
        150,
        50430,
        [0x00, 0x41, 0x32, 0x00, 0x0c, 0x5c, 0xf7],
    ),
    (
        0x21605,
        0x21610,
        0x21607,
        277,
        50917,
        [0x00, 0x41, 0x32, 0x00, 0x0c, 0x5d, 0xf7],
    ),
    (
        0x21610,
        0x2161b,
        0x21612,
        357,
        51274,
        [0x00, 0x41, 0x32, 0x00, 0x0c, 0x5c, 0xf7],
    ),
    (
        0x21648,
        0x21652,
        0x21649,
        67,
        53827,
        [0x00, 0x41, 0x32, 0x00, 0x0c, 0x5d, 0xf7],
    ),
    (
        0x21667,
        0x21672,
        0x21669,
        164,
        55364,
        [0x00, 0x41, 0x32, 0x00, 0x0c, 0x5c, 0xf7],
    ),
    (
        0x21679,
        0x21683,
        0x2167a,
        79,
        55759,
        [0x00, 0x41, 0x32, 0x00, 0x0c, 0x5d, 0xf7],
    ),
    (
        0x21691,
        0x2169b,
        0x21692,
        52,
        56452,
        [0x00, 0x41, 0x32, 0x00, 0x0c, 0x5c, 0xf7],
    ),
    (
        0x2169b,
        0x216a5,
        0x2169c,
        38,
        56490,
        [0x00, 0x41, 0x32, 0x00, 0x0c, 0x5d, 0xf7],
    ),
    (
        0x216b4,
        0x216be,
        0x216b5,
        12,
        57852,
        [0x00, 0x41, 0x32, 0x00, 0x0c, 0x5c, 0xf7],
    ),
    (
        0x216de,
        0x216e8,
        0x216df,
        93,
        59613,
        [0x00, 0x41, 0x32, 0x00, 0x0c, 0x5d, 0xf7],
    ),
    (
        0x21702,
        0x2170c,
        0x21703,
        12,
        61812,
        [0x00, 0x41, 0x32, 0x00, 0x0c, 0x5c, 0xf7],
    ),
];
const EXPECTED_7: [Row; 2] = [
    (
        0x219db,
        0x219e5,
        0x219dc,
        92,
        12092,
        [0x00, 0x41, 0x32, 0x00, 0x0b, 0x1e, 0xf7],
    ),
    (
        0x219ec,
        0x219f6,
        0x219ed,
        6,
        12246,
        [0x00, 0x41, 0x32, 0x00, 0x0b, 0x1f, 0xf7],
    ),
];

#[test]
fn sysex_keeps_source_note_state_but_requires_explicit_reentry() {
    let mut b = vec![3, 0x90, 60, 100, 40, 2];
    b.extend(SYSEX);
    b.extend([0, 0x90, 61, 101, 41, 2, 0, 62, 102, 42, 3]);
    let w = walk(&b).unwrap();
    assert_eq!(w.logical_event_count(), 4);
    let MixedEventItem::Event(e) = &w.items[1] else {
        panic!()
    };
    assert_eq!(e.position, 3);
    assert!(matches!(e.event, MixedEventKind::SysEx(_)));
    let MixedEventItem::Event(e) = &w.items[3] else {
        panic!()
    };
    assert!(matches!(&e.event,MixedEventKind::Note(n) if n.status.is_none() && n.pitch.value==62));
    // Implicit immediate successor is refused even when Note state preceded SysEx.
    let mut bad = vec![3, 0x90, 60, 100, 40, 2];
    bad.extend(SYSEX);
    bad.extend([0, 61, 101, 41, 2]);
    assert!(walk(&bad).is_err());
}
#[test]
fn sysex_refuses_all_truncations_lengths_high_bits_terminators_and_successors() {
    for n in 0..SYSEX.len() {
        if n == 0 {
            continue;
        }
        assert!(walk(&SYSEX[..n]).is_err(), "truncation {n}");
    }
    for length in [0, 6, 8, 0x87, 0xff] {
        let mut b = SYSEX;
        b[2] = length;
        assert!(walk(&b).is_err());
    }
    for i in 3..9 {
        let mut b = SYSEX;
        b[i] = 0xf7;
        assert!(walk(&b).is_err());
    }
    let mut b = SYSEX;
    b[9] = 0;
    assert!(walk(&b).is_err());
    for status in [0x01, 0xb0, 0xd0, 0xe0, 0xff, 0xf7] {
        let mut b = SYSEX.to_vec();
        b.extend([0, status, 0, 0, 0]);
        assert!(walk(&b).is_err());
    }
    let mut b = SYSEX.to_vec();
    b.extend([0]);
    assert!(walk(&b).is_err());
    assert!(walk_bounded_mixed_events(
        &SYSEX,
        MixedEventBounds {
            event_range: 0..SYSEX.len()
        },
        MixedEventTimingBasis {
            previous_event_position: u32::MAX
        }
    )
    .is_ok());
    let mut b = SYSEX;
    b[0] = 1;
    assert!(walk_bounded_mixed_events(
        &b,
        MixedEventBounds {
            event_range: 0..b.len()
        },
        MixedEventTimingBasis {
            previous_event_position: u32::MAX
        }
    )
    .is_err());
}
