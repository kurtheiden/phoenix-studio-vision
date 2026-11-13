//! Independently constructed protocol cases; no external fixtures.
use super::*;

const METER: [u8; 8] = [0, 0xff, 0x58, 4, 5, 3, 8, 8];
const KEY: [u8; 6] = [0, 0xff, 0x59, 2, 253, 1];

fn decode(bytes: &[u8], count: u32) -> Result<BoundedMeterKeyMap<'_>, MeterKeyMapError> {
    decode_bounded_meter_key_map(
        bytes,
        MeterKeyMapBounds {
            event_range: 0..bytes.len(),
            declared_count: count,
        },
    )
}

fn assert_provenance(bytes: &[u8], map: &BoundedMeterKeyMap<'_>) {
    let mut cursor = map.event_range().start;
    let mut tick = 0u32;
    for event in map.events() {
        assert_eq!(event.source.range.start, cursor);
        assert_eq!(event.source.bytes, &bytes[event.source.range.clone()]);
        assert_eq!(
            event.source.bytes.as_ptr(),
            bytes[event.source.range.clone()].as_ptr()
        );
        assert_eq!(event.delta.range.start, cursor);
        assert_eq!(event.delta.bytes, &bytes[event.delta.range.clone()]);
        assert_eq!(event.delta.range.end, event.ff_tag.offset);
        tick = tick.checked_add(event.delta.value).unwrap();
        assert_eq!(event.absolute_tick, tick);
        let mut fields = vec![event.ff_tag, event.event_tag, event.payload_length];
        match &event.kind {
            MeterKeyKind::Meter(m) => fields.extend([
                m.numerator,
                m.denominator_exponent,
                m.third_payload,
                m.fourth_payload,
            ]),
            MeterKeyKind::Key(k) => fields.extend([k.accidentals_byte, k.mode]),
        }
        for (i, field) in fields.iter().enumerate() {
            assert_eq!(field.offset, event.delta.range.end + i);
            assert_eq!(field.value, bytes[field.offset]);
        }
        assert_eq!(fields.last().unwrap().offset + 1, event.source.range.end);
        cursor = event.source.range.end;
    }
    assert_eq!(cursor, map.event_range().end);
    assert_eq!(map.events().len() as u32, map.declared_count());
    assert_eq!(
        map.events()
            .iter()
            .flat_map(|e| e.source.bytes.iter().copied())
            .collect::<Vec<_>>(),
        bytes[map.event_range()]
    );
}

#[test]
fn duplicates_and_simultaneous_meter_key_objects_keep_source_order() {
    let mut body = METER.to_vec();
    body.extend([0x82, 0x25]);
    body.extend(&METER[1..]);
    body.extend(KEY);
    body.extend(METER);
    body.extend([0x82, 0x25]);
    body.extend(&METER[1..]);
    let mut source = vec![0xa5; 11];
    source.extend(&body);
    source.extend([0x5a; 7]);
    let map = decode_bounded_meter_key_map(
        &source,
        MeterKeyMapBounds {
            event_range: 11..11 + body.len(),
            declared_count: 5,
        },
    )
    .unwrap();
    assert_eq!(map.ppqn(), 480);
    assert_eq!(
        map.events()
            .iter()
            .map(|e| e.absolute_tick)
            .collect::<Vec<_>>(),
        [0, 293, 293, 293, 586]
    );
    assert_eq!(
        map.events()
            .iter()
            .map(|e| e.event_tag.value)
            .collect::<Vec<_>>(),
        [0x58, 0x58, 0x59, 0x58, 0x58]
    );
    assert_eq!(map.events()[1].delta.bytes, [0x82, 0x25]);
    assert_provenance(&source, &map);
}

#[test]
fn nonzero_first_delta_and_raw_vlq_widths_are_not_normalized() {
    let mut bytes = vec![0x83, 0x09];
    bytes.extend(&METER[1..]);
    bytes.extend([0x80, 0]);
    bytes.extend(&KEY[1..]);
    let map = decode(&bytes, 2).unwrap();
    assert_eq!(
        map.events()
            .iter()
            .map(|e| e.absolute_tick)
            .collect::<Vec<_>>(),
        [393, 393]
    );
    assert_eq!(map.events()[0].delta.bytes, [0x83, 0x09]);
    assert_eq!(map.events()[1].delta.bytes, [0x80, 0]);
    assert_provenance(&bytes, &map);
}

#[test]
fn meter_storage_preservation_and_existing_adaptation_remain_separate() {
    use crate::midi_export::{adapt_meter_values, MeterPolicy};
    for (payload, expected) in [
        ([5, 3, 8, 8], [5, 3, 24, 8]),
        ([6, 3, 6, 8], [6, 3, 12, 8]),
        ([7, 3, 6, 8], [7, 3, 12, 8]),
    ] {
        let bytes = [vec![0, 255, 88, 4], payload.to_vec()].concat();
        let map = decode(&bytes, 1).unwrap();
        let MeterKeyKind::Meter(m) = &map.events()[0].kind else {
            panic!("Meter")
        };
        let (signature, warnings) = adapt_meter_values(
            m.numerator.value,
            m.denominator_exponent.value,
            m.third_payload.value,
            m.fourth_payload.value,
            MeterPolicy::KnownHistoricalOnly,
        )
        .unwrap();
        assert!(warnings.is_empty());
        assert_eq!(
            signature,
            crate::smf::TimeSignature::new(
                crate::smf::MidiDataByte::new(expected[0]).unwrap(),
                crate::smf::MidiDataByte::new(expected[1]).unwrap(),
                crate::smf::MidiDataByte::new(expected[2]).unwrap(),
                crate::smf::MidiDataByte::new(expected[3]).unwrap()
            )
            .unwrap()
        );
        let initial = crate::meter::decode_bounded_initial_meter(
            &bytes,
            crate::meter::InitialMeterBounds { event_range: 0..8 },
        )
        .unwrap();
        assert_eq!(m.third_payload, initial.third_payload);
    }
    let bytes = [0, 255, 88, 4, 0, 255, 165, 200];
    let map = decode(&bytes, 1).unwrap();
    let MeterKeyKind::Meter(m) = &map.events()[0].kind else {
        panic!("Meter")
    };
    assert_eq!(
        [
            m.numerator.value,
            m.denominator_exponent.value,
            m.third_payload.value,
            m.fourth_payload.value
        ],
        [0, 255, 165, 200]
    );
    assert!(adapt_meter_values(0, 255, 165, 200, MeterPolicy::KnownHistoricalOnly).is_err());
    assert_provenance(&bytes, &map);
}

#[test]
fn key_signed_values_modes_and_original_bytes_are_retained() {
    for sf in [-7i8, -1, 0, 1, 7] {
        for mode in [0, 1] {
            let bytes = [0, 255, 89, 2, sf as u8, mode];
            let map = decode(&bytes, 1).unwrap();
            let MeterKeyKind::Key(k) = &map.events()[0].kind else {
                panic!("Key")
            };
            assert_eq!(k.accidentals(), sf);
            assert_eq!(k.minor(), mode == 1);
            assert_eq!(k.accidentals_byte.value, sf as u8);
            assert_provenance(&bytes, &map);
        }
    }
    for (sf, mode) in [(8, 0), (248, 0), (128, 0), (1, 2)] {
        assert!(matches!(
            decode(&[0, 255, 89, 2, sf, mode], 1),
            Err(MeterKeyMapError::InvalidKey { .. })
        ));
    }
}

#[test]
fn bounds_counts_and_hidden_or_trailing_objects_refuse_transactionally() {
    for range in [
        0..0,
        Range { start: 2, end: 1 },
        0..9,
        usize::MAX..usize::MAX,
    ] {
        assert_eq!(
            decode_bounded_meter_key_map(
                &METER,
                MeterKeyMapBounds {
                    event_range: range,
                    declared_count: 1
                }
            ),
            Err(MeterKeyMapError::Bounds)
        );
    }
    for count in [0, 2, u32::MAX] {
        assert!(decode(&METER, count).is_err());
    }
    let two = [METER.to_vec(), KEY.to_vec()].concat();
    assert!(matches!(
        decode(&two, 1),
        Err(MeterKeyMapError::Unconsumed { offset: 8 })
    ));
    assert!(decode(&two, 3).is_err());
    for suffix in [vec![0], vec![255], vec![0, 255, 47, 0], METER.to_vec()] {
        let bytes = [METER.to_vec(), suffix].concat();
        assert!(matches!(
            decode(&bytes, 1),
            Err(MeterKeyMapError::Unconsumed { .. })
        ));
    }
    assert_eq!(decode(&two, 2).unwrap().events().len(), 2);
}

#[test]
fn every_truncation_and_malformed_length_stays_within_the_caller_bound() {
    let bytes = [
        METER.to_vec(),
        KEY.to_vec(),
        vec![0x82, 0x25],
        METER[1..].to_vec(),
    ]
    .concat();
    for end in 0..bytes.len() {
        assert!(
            decode_bounded_meter_key_map(
                &bytes,
                MeterKeyMapBounds {
                    event_range: 0..end,
                    declared_count: 3
                }
            )
            .is_err(),
            "end {end}"
        );
    }
    for event in [METER.to_vec(), KEY.to_vec()] {
        for end in 0..event.len() {
            assert!(decode(&event[..end], 1).is_err());
        }
        for length in [0, 1, 3, 5, 127, 255] {
            if length == event[3] {
                continue;
            }
            let mut bad = event.clone();
            bad[3] = length;
            assert!(matches!(
                decode(&bad, 1),
                Err(MeterKeyMapError::PayloadLength { .. })
            ));
        }
    }
}

#[test]
fn unknown_tags_never_scan_or_restart_at_later_valid_events() {
    for tag in [0x51, 0x2f, 0x60, 0x7c, 0] {
        let mut bytes = METER.to_vec();
        bytes.extend([0, 255, tag, 4, 5, 3, 8, 8]);
        bytes.extend(METER);
        assert_eq!(
            decode(&bytes, 3),
            Err(MeterKeyMapError::UnsupportedEvent { offset: 9 })
        );
    }
    let mut bytes = METER.to_vec();
    bytes[1] = 0xfe;
    bytes.extend(METER);
    assert_eq!(
        decode(&bytes, 2),
        Err(MeterKeyMapError::UnsupportedEvent { offset: 1 })
    );
}

#[test]
fn malformed_vlqs_and_cumulative_tick_overflow_refuse() {
    let mut bytes = vec![0x80; 4];
    bytes.extend(&KEY[1..]);
    assert!(matches!(
        decode(&bytes, 1),
        Err(MeterKeyMapError::Timing(VlqError::TooLong { .. }))
    ));
    // A continuation VLQ at the supplied end must not borrow the following Key.
    let mut bytes = METER.to_vec();
    bytes.extend(KEY);
    bytes.extend([0x80; 6]);
    assert!(matches!(
        decode(&bytes, 3),
        Err(MeterKeyMapError::Timing(_))
    ));
    let truncated = [METER.to_vec(), METER.to_vec(), METER.to_vec(), vec![0x80]].concat();
    assert_eq!(
        decode(&truncated, 4),
        Err(MeterKeyMapError::Timing(VlqError::Truncated { offset: 24 }))
    );
    let mut bytes = vec![];
    for _ in 0..17 {
        bytes.extend([255, 255, 255, 127]);
        bytes.extend(&KEY[1..]);
    }
    assert_eq!(
        decode(&bytes, 17),
        Err(MeterKeyMapError::PositionOverflow { offset: 16 * 9 })
    );
    let map = decode(&bytes[..16 * 9], 16).unwrap();
    assert_eq!(map.events().last().unwrap().absolute_tick, u32::MAX - 15);
}
