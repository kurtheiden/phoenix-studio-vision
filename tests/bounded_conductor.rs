use phoenix::bounded_conductor::decode_compact_conductor;
use std::ops::Range;
fn fixture(mpqn: [u8; 3], key: [u8; 2]) -> (Vec<u8>, [Range<usize>; 4]) {
    let meter = [0, 255, 88, 4, 3, 2, 8, 8];
    let tempo = [0, 255, 81, 3, mpqn[0], mpqn[1], mpqn[2]];
    let mut b = vec![0; 8];
    let mut ranges = Vec::new();
    for (count, events, last) in [
        (
            2u32,
            [vec![0, 255, 89, 2, key[0], key[1]], meter.to_vec()].concat(),
            meter.to_vec(),
        ),
        (1, tempo.to_vec(), tempo.to_vec()),
    ] {
        let mut p = vec![0, 1];
        p.extend(count.to_be_bytes());
        p.extend([0; 4]);
        p.extend([9, 8, 7, 6]);
        p.extend(&events);
        p.extend([135, 255, 255, 127, 255, 47, 0]);
        let mut value = vec![last[2]];
        value.extend(&last[4..]);
        let mut s = vec![0, 1, 0, 0, 0, (18 + value.len()) as u8];
        s.extend(count.to_be_bytes());
        s.extend(((events.len() + 4) as u32).to_be_bytes());
        s.extend([0, 255, 255, 255, 47, 255, 0, 0, 0, 0]);
        s.extend(value);
        for (ty, v) in [(2, p), (41, s)] {
            let start = b.len();
            b.push(ty);
            b.extend((v.len() as u32).to_be_bytes());
            b.extend(v);
            ranges.push(start..b.len());
        }
    }
    (b, ranges.try_into().unwrap())
}
#[test]
fn compact_values_ranges_and_key_preserved() {
    for (tempo, key) in [([7, 161, 32], [253, 1]), ([8, 0, 1], [2, 0])] {
        let (b, r) = fixture(tempo, key);
        let c = decode_compact_conductor(&b, r.clone()).unwrap();
        let a = c.adapt(b"Synthetic").unwrap();
        assert_eq!(
            a.tempo_mpqn,
            u32::from_be_bytes([0, tempo[0], tempo[1], tempo[2]])
        );
        assert_eq!(
            phoenix::smf::serialize_time_signature(a.time_signature),
            vec![255, 88, 4, 3, 2, 24, 8]
        );
        assert_eq!(c.key().accidentals, key[0] as i8);
        assert_eq!(c.key().minor, key[1] == 1);
        assert_eq!(c.record_ranges(), &r);
        assert_eq!(c.meter().event_range, r[0].start + 25..r[0].start + 33);
        assert_eq!(c.key().source_range, r[0].start + 19..r[0].start + 25);
        assert_eq!(c.opaque_context(), &[vec![9, 8, 7, 6], vec![9, 8, 7, 6]]);
    }
}
#[test]
fn structural_mutations_and_all_truncations_refuse() {
    let (b, r) = fixture([7, 161, 32], [0, 0]);
    for record in 0..4 {
        for off in r[record].clone() {
            let local = off - r[record].start;
            // Only primary opaque identity and event values are variable, not framing.
            if record % 2 == 0 && (15..19).contains(&local) {
                continue;
            }
            if record == 0 && (23..=24).contains(&local) {
                continue;
            }
            let mut bad = b.clone();
            bad[off] ^= 128;
            assert!(
                decode_compact_conductor(&bad, r.clone()).is_err(),
                "record{record} offset{local}"
            );
        }
    }
    for n in 0..b.len() {
        assert!(decode_compact_conductor(&b[..n], r.clone()).is_err());
    }
}
#[test]
fn invalid_values_and_unsupported_events_refuse() {
    for (tempo, key) in [
        ([0, 0, 0], [0, 0]),
        ([7, 161, 32], [8, 0]),
        ([7, 161, 32], [0, 2]),
    ] {
        let (b, r) = fixture(tempo, key);
        assert!(decode_compact_conductor(&b, r).is_err());
    }
    let (b, r) = fixture([7, 161, 32], [0, 0]);
    for (record, local, value) in [
        (0, 19, 1),
        (0, 21, 81),
        (0, 25, 1),
        (0, 29, 0),
        (0, 31, 1),
        (2, 19, 1),
    ] {
        let mut bad = b.clone();
        bad[r[record].start + local] = value;
        assert!(decode_compact_conductor(&bad, r.clone()).is_err());
    }
    let mut wrong = r.clone();
    wrong.swap(0, 2);
    assert!(decode_compact_conductor(&b, wrong).is_err());
}
#[test]
fn key_serialization_is_initial_only_and_old_output_unchanged() {
    let (b, r) = fixture([7, 161, 32], [253, 1]);
    let c = decode_compact_conductor(&b, r).unwrap();
    let a = c.adapt(b"S").unwrap();
    let old =
        phoenix::smf::serialize_conductor_track(b"S", a.tempo_mpqn, a.time_signature).unwrap();
    let same = phoenix::smf::serialize_conductor_track_with_key(
        b"S",
        a.tempo_mpqn,
        a.time_signature,
        None,
    )
    .unwrap();
    assert_eq!(old.as_bytes(), same.as_bytes());
    let track = phoenix::smf::serialize_conductor_track_with_key(
        b"S",
        a.tempo_mpqn,
        a.time_signature,
        Some((-3, true)),
    )
    .unwrap();
    assert_eq!(
        &track.as_bytes()[8..],
        &[
            0, 255, 3, 1, b'S', 0, 255, 81, 3, 7, 161, 32, 0, 255, 89, 2, 253, 1, 0, 255, 88, 4, 3,
            2, 24, 8, 0, 255, 47, 0
        ]
    );
    assert!(phoenix::smf::serialize_conductor_track_with_key(
        b"S",
        a.tempo_mpqn,
        a.time_signature,
        Some((8, false))
    )
    .is_err());
}

#[test]
fn additional_event_with_updated_framing_still_refuses() {
    let (b, r) = fixture([7, 161, 32], [0, 0]);
    let extra = [0, 255, 81, 3, 8, 0, 1];
    let insertion = r[2].end - 7;
    let mut bad = b.clone();
    bad.splice(insertion..insertion, extra);
    bad[r[2].start + 1..r[2].start + 5].copy_from_slice(&35u32.to_be_bytes());
    bad[r[2].start + 7..r[2].start + 11].copy_from_slice(&2u32.to_be_bytes());
    let mut ranges = r.clone();
    ranges[2].end += extra.len();
    ranges[3] = r[3].start + extra.len()..r[3].end + extra.len();
    bad[ranges[3].start + 11..ranges[3].start + 15].copy_from_slice(&2u32.to_be_bytes());
    bad[ranges[3].start + 15..ranges[3].start + 19].copy_from_slice(&18u32.to_be_bytes());
    assert!(decode_compact_conductor(&bad, ranges).is_err());
}

#[test]
fn opaque_context_is_retained_without_semantic_effect() {
    let (mut b, r) = fixture([7, 161, 32], [0, 0]);
    let original = decode_compact_conductor(&b, r.clone()).unwrap();
    for record in [0, 2] {
        b[r[record].start + 15..r[record].start + 19].fill(255);
    }
    let changed = decode_compact_conductor(&b, r).unwrap();
    assert_eq!(original.adapt(b"S").unwrap(), changed.adapt(b"S").unwrap());
    assert_eq!(changed.opaque_context(), &[vec![255; 4], vec![255; 4]]);
}
