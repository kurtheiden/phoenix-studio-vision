//! Independently constructed schedules; no external fixtures.
use super::*;
use crate::{
    meter_key_map::{decode_bounded_meter_key_map, MeterKeyMapBounds},
    tempo::{decode_bounded_tempo_map, TempoMapBounds},
};
const M: [u8; 8] = [0, 255, 88, 4, 5, 3, 8, 8];
const K: [u8; 6] = [0, 255, 89, 2, 253, 1];
fn meter(b: &[u8], count: u32) -> BoundedMeterKeyMap<'_> {
    decode_bounded_meter_key_map(
        b,
        MeterKeyMapBounds {
            event_range: 0..b.len(),
            declared_count: count,
        },
    )
    .unwrap()
}
fn tempo(b: &[u8], count: u32) -> BoundedTempoMap<'_> {
    decode_bounded_tempo_map(
        b,
        TempoMapBounds {
            event_range: 0..b.len(),
            declared_count: count,
        },
    )
    .unwrap()
}
fn tb(events: &[(u32, u32)]) -> Vec<u8> {
    events
        .iter()
        .flat_map(|(d, v)| {
            let mut b = encode_midi_vlq(*d).unwrap();
            b.extend(serialize_set_tempo(*v).unwrap());
            b
        })
        .collect()
}
fn wire(plan: &ConductorMapPlan<'_>) -> (Vec<u8>, crate::readback120::Events) {
    let chunk = serialize(b"map", plan).unwrap();
    let midi = serialize_format1(480, &[chunk]).unwrap();
    let expected = crate::readback120::conductor_map_expected(b"map", plan).unwrap();
    crate::readback120::verify(&midi, 480, std::slice::from_ref(&expected)).unwrap();
    (midi, expected)
}
#[test]
fn duplicates_simultaneous_key_meter_and_late_meter_keep_all_objects() {
    let mut m = M.to_vec();
    m.extend([73]);
    m.extend(&K[1..]);
    m.extend(M);
    m.extend(M);
    let mk = meter(&m, 4);
    let t = tb(&[(0, 524288)]);
    let tm = tempo(&t, 1);
    let (_, e) = wire(&prepare(&tm, &mk).unwrap());
    assert_eq!(
        e,
        vec![
            (0, 3, b"map".to_vec()),
            (0, 81, vec![8, 0, 0]),
            (0, 88, vec![5, 3, 24, 8]),
            (73, 89, vec![253, 1]),
            (73, 88, vec![5, 3, 24, 8]),
            (73, 88, vec![5, 3, 24, 8]),
            (73, 47, vec![])
        ]
    );
}
#[test]
fn nonzero_first_tempo_and_meter_are_never_corrected_or_synthesized() {
    let mut m = vec![29];
    m.extend(&M[1..]);
    let mk = meter(&m, 1);
    let t = tb(&[(421, 524288)]);
    let tm = tempo(&t, 1);
    let (_, e) = wire(&prepare(&tm, &mk).unwrap());
    assert_eq!(
        e.iter().map(|e| (e.0, e.1)).collect::<Vec<_>>(),
        [(0, 3), (29, 88), (421, 81), (421, 47)]
    );
}
#[test]
fn initial_legacy_serialization_is_byte_identical() {
    for key in [false, true] {
        let mut m = vec![];
        if key {
            m.extend(K);
        }
        m.extend(M);
        let mk = meter(&m, if key { 2 } else { 1 });
        let t = tb(&[(0, 524288), (137, 655360)]);
        let tm = tempo(&t, 2);
        let plan = prepare(&tm, &mk).unwrap();
        let signature = TimeSignature::new(
            MidiDataByte::new(5).unwrap(),
            MidiDataByte::new(3).unwrap(),
            MidiDataByte::new(24).unwrap(),
            MidiDataByte::new(8).unwrap(),
        )
        .unwrap();
        assert_eq!(
            serialize(b"map", &plan).unwrap(),
            serialize_tempo_map_conductor(b"map", &tm, signature, key.then_some((-3, true)))
                .unwrap()
        );
        wire(&plan);
    }
}
#[test]
fn unresolved_cross_map_ties_and_unsupported_initial_order_refuse() {
    let mut m = M.to_vec();
    m.extend([73]);
    m.extend(&M[1..]);
    let mk = meter(&m, 2);
    let t = tb(&[(0, 524288), (73, 655360)]);
    let tm = tempo(&t, 2);
    assert!(matches!(
        prepare(&tm, &mk),
        Err(MapError::AmbiguousTie { tick: 73 })
    ));
    let duplicated = [M, M].concat();
    let mk = meter(&duplicated, 2);
    assert!(matches!(
        prepare(&tm, &mk),
        Err(MapError::AmbiguousTie { tick: 0 })
    ));
    let t = tb(&[(0, 524288), (0, 524289)]);
    let tm = tempo(&t, 2);
    let mk = meter(&M, 1);
    assert!(matches!(
        prepare(&tm, &mk),
        Err(MapError::AmbiguousTie { tick: 0 })
    ));
}
#[test]
fn reference_must_correlate_every_value_tick_count_and_within_map_order() {
    let mut m = M.to_vec();
    m.extend([73]);
    m.extend(&K[1..]);
    m.extend(M);
    let mk = meter(&m, 3);
    let t = tb(&[(0, 524288), (73, 655360)]);
    let tm = tempo(&t, 2);
    let r = vec![
        (0, 81, vec![8, 0, 0]),
        (0, 88, vec![5, 3, 24, 8]),
        (73, 81, vec![10, 0, 0]),
        (73, 89, vec![253, 1]),
        (73, 88, vec![5, 3, 24, 8]),
    ];
    wire(&correlate_reference(&tm, &mk, &r).unwrap());
    for i in 0..r.len() {
        let mut bad = r.clone();
        bad.remove(i);
        assert!(correlate_reference(&tm, &mk, &bad).is_err());
        let mut bad = r.clone();
        bad[i].0 += 1;
        assert!(correlate_reference(&tm, &mk, &bad).is_err());
        let mut bad = r.clone();
        bad[i].2[0] ^= 1;
        assert!(correlate_reference(&tm, &mk, &bad).is_err());
    }
    let mut bad = r.clone();
    bad.push(r[4].clone());
    assert!(correlate_reference(&tm, &mk, &bad).is_err());
    let mut bad = r.clone();
    bad.swap(3, 4);
    assert!(correlate_reference(&tm, &mk, &bad).is_err());
    let mut bad = r;
    bad.swap(0, 1);
    assert!(correlate_reference(&tm, &mk, &bad).is_err());
}
#[test]
fn unsupported_adaptation_never_falls_back() {
    let t = tb(&[(0, 524288)]);
    let tm = tempo(&t, 1);
    for (index, value) in [(4, 0), (4, 128), (5, 128), (6, 7), (7, 128)] {
        let mut m = M;
        m[index] = value;
        let mk = meter(&m, 1);
        assert!(matches!(prepare(&tm, &mk), Err(MapError::UnsupportedMeter)));
    }
    let m = [0, 255, 88, 4, 6, 3, 6, 8];
    let mk = meter(&m, 1);
    let (_, e) = wire(&prepare(&tm, &mk).unwrap());
    assert_eq!(e[2].2, [6, 3, 12, 8]);
}
#[test]
fn independent_readback_refuses_missing_extra_altered_and_reordered_objects() {
    let mut m = M.to_vec();
    m.extend([73]);
    m.extend(&K[1..]);
    m.extend(M);
    let mk = meter(&m, 3);
    let t = tb(&[(0, 524288)]);
    let tm = tempo(&t, 1);
    let (midi, e) = wire(&prepare(&tm, &mk).unwrap());
    for i in 1..e.len() - 1 {
        let mut bad = e.clone();
        bad.remove(i);
        assert!(crate::readback120::verify(&midi, 480, &[bad]).is_err());
        let mut bad = e.clone();
        bad.insert(i, e[i].clone());
        assert!(crate::readback120::verify(&midi, 480, &[bad]).is_err());
        let mut bad = e.clone();
        bad[i].2[0] ^= 1;
        assert!(crate::readback120::verify(&midi, 480, &[bad]).is_err());
        let mut bad = e.clone();
        bad[i].0 += 1;
        assert!(crate::readback120::verify(&midi, 480, &[bad]).is_err());
    }
    let mut bad = e;
    bad.swap(3, 4);
    assert!(crate::readback120::verify(&midi, 480, &[bad]).is_err());
}
#[test]
fn checked_arithmetic_and_smf_delta_limits_are_defensive() {
    let mut m = M.to_vec();
    m.extend(encode_midi_vlq(MAX_MIDI_VLQ).unwrap());
    m.extend(&M[1..]);
    m.extend([1]);
    m.extend(&M[1..]);
    let mk = meter(&m, 3);
    let t = tb(&[(0, 524288)]);
    let tm = tempo(&t, 1);
    let mut plan = prepare(&tm, &mk).unwrap();
    wire(&plan);
    // Private-plan fault injection: no external constructor can forge this order.
    plan.order = vec![Origin::MeterKey(2)];
    assert!(matches!(
        serialize(b"map", &plan),
        Err(MapError::Smf(SmfSerializeError::MidiVlqOverflow { .. }))
    ));
    // First isolate the nonmonotonic subtraction using a representable first tick.
    let m = [M.to_vec(), vec![1], M[1..].to_vec()].concat();
    let mk = meter(&m, 2);
    let mut plan = prepare(&tm, &mk).unwrap();
    plan.order = vec![Origin::MeterKey(1), Origin::Tempo(0)];
    assert!(matches!(
        serialize(b"map", &plan),
        Err(MapError::Arithmetic)
    ));
    let overflowing = tb(&[(MAX_MIDI_VLQ, 524288); 17]);
    assert!(matches!(
        decode_bounded_tempo_map(
            &overflowing,
            TempoMapBounds {
                event_range: 0..overflowing.len(),
                declared_count: 17
            }
        ),
        Err(crate::tempo::TempoMapError::PositionOverflow { .. })
    ));
}
#[test]
fn isolated_meter_key_schedule_has_no_invented_tempo() {
    let mut bytes = vec![29];
    bytes.extend(&K[1..]);
    bytes.extend([73]);
    bytes.extend(&M[1..]);
    bytes.extend(M);
    let mk = meter(&bytes, 3);
    let (_, events) = wire(&prepare_meter_key(&mk).unwrap());
    assert_eq!(
        events,
        vec![
            (0, 3, b"map".to_vec()),
            (29, 89, vec![253, 1]),
            (102, 88, vec![5, 3, 24, 8]),
            (102, 88, vec![5, 3, 24, 8]),
            (102, 47, vec![]),
        ]
    );
}
