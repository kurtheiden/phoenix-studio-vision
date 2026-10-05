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
        assert_eq!(c.key().unwrap().accidentals, key[0] as i8);
        assert_eq!(c.key().unwrap().minor, key[1] == 1);
        assert_eq!(c.record_ranges(), &r);
        assert_eq!(c.meter().event_range, r[0].start + 25..r[0].start + 33);
        assert_eq!(
            c.key().unwrap().source_range,
            r[0].start + 19..r[0].start + 25
        );
        assert_eq!(c.opaque_context(), &[vec![9, 8, 7, 6], vec![9, 8, 7, 6]]);
    }
}
#[test]
fn structural_mutations_and_all_truncations_refuse() {
    let (b, r) = fixture([7, 161, 32], [0, 0]);
    for record in 0..4 {
        for off in r[record].clone() {
            let local = off - r[record].start;
            // Primary opaque identity, secondary bounded context and Key values vary.
            if record % 2 == 1 && local == 24 {
                continue;
            }
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

fn reframe(payloads: &[Vec<u8>; 4]) -> (Vec<u8>, [Range<usize>; 4]) {
    let mut bytes = vec![0; 8];
    let mut ranges = Vec::new();
    for (i, payload) in payloads.iter().enumerate() {
        let start = bytes.len();
        bytes.push(if i % 2 == 0 { 2 } else { 41 });
        bytes.extend((payload.len() as u32).to_be_bytes());
        bytes.extend(payload);
        ranges.push(start..bytes.len());
    }
    (bytes, ranges.try_into().unwrap())
}

fn payloads(bytes: &[u8], ranges: &[Range<usize>; 4]) -> [Vec<u8>; 4] {
    std::array::from_fn(|i| bytes[ranges[i].start + 5..ranges[i].end].to_vec())
}

fn variant_fixture(padded: bool) -> (Vec<u8>, [Range<usize>; 4]) {
    let (b, r) = fixture([9, 8, 7], [255, 1]);
    let mut p = payloads(&b, &r);
    if padded {
        p[0].drain(14..20); // Meter only; no Key event exists in this layout.
        p[0][2..6].copy_from_slice(&1u32.to_be_bytes());
        p[0].resize(38, 0);
        p[2].resize(38, 0);
        p[1][6..10].copy_from_slice(&1u32.to_be_bytes());
        p[1][10..14].copy_from_slice(&12u32.to_be_bytes());
        p[1][19] = 0;
        p[3][19] = 1;
    } else {
        p[0][1] = 3;
        p[3][19] = 28;
    }
    reframe(&p)
}

#[test]
fn compact_header_variant_and_secondary_context_preserve_all_events() {
    let (b, r) = variant_fixture(false);
    let c = decode_compact_conductor(&b, r.clone()).unwrap();
    assert_eq!(c.tempo().mpqn(), 0x090807);
    assert_eq!(c.key().unwrap().accidentals, -1);
    assert!(c.key().unwrap().minor);
    assert_eq!(c.meter().numerator.value, 3);
    assert_eq!(
        c.key().unwrap().source_range,
        r[0].start + 19..r[0].start + 25
    );
    assert!(c
        .envelope_context()
        .iter()
        .any(|x| { x.source_range == (r[0].start + 6..r[0].start + 7) && x.source_bytes == [3] }));
    assert!(c.envelope_context().iter().any(|x| {
        x.source_range == (r[3].start + 24..r[3].start + 25) && x.source_bytes == [28]
    }));
}

#[test]
fn padded_initial_only_form_preserves_ranges_and_does_not_invent_key() {
    let (b, r) = variant_fixture(true);
    let c = decode_compact_conductor(&b, r.clone()).unwrap();
    assert!(c.key().is_none());
    assert_eq!(c.tempo().mpqn(), 0x090807);
    assert_eq!(c.meter().event_range, r[0].start + 19..r[0].start + 27);
    assert_eq!(c.tempo().event_range, r[2].start + 19..r[2].start + 26);
    assert_eq!(c.record_ranges(), &r);
    for x in c.envelope_context() {
        assert_eq!(x.source_bytes, b[x.source_range.clone()]);
    }
    assert!(c
        .envelope_context()
        .iter()
        .any(|x| { x.source_range == (r[0].start + 34..r[0].end) && x.source_bytes == [0; 9] }));
    assert!(c
        .envelope_context()
        .iter()
        .any(|x| { x.source_range == (r[2].start + 33..r[2].end) && x.source_bytes == [0; 10] }));
    let a = c.adapt(b"Padded").unwrap();
    let optional_key = c.key().map(|k| (k.accidentals, k.minor));
    let output = phoenix::smf::serialize_conductor_track_with_key(
        b"Padded",
        a.tempo_mpqn,
        a.time_signature,
        optional_key,
    )
    .unwrap();
    let without_key =
        phoenix::smf::serialize_conductor_track(b"Padded", a.tempo_mpqn, a.time_signature).unwrap();
    assert_eq!(output.as_bytes(), without_key.as_bytes());
}

#[test]
fn opaque_secondary_byte_is_bounded_and_retained_not_interpreted() {
    for padded in [false, true] {
        let (b, r) = variant_fixture(padded);
        let original = decode_compact_conductor(&b, r.clone())
            .unwrap()
            .adapt(b"S")
            .unwrap();
        for value in [0, 1, 28, 0x51, 0x58, 0x59, 0xf0, 255] {
            let mut changed = b.clone();
            for record in [1, 3] {
                changed[r[record].start + 24] = value;
            }
            let c = decode_compact_conductor(&changed, r.clone()).unwrap();
            assert_eq!(c.adapt(b"S").unwrap(), original);
            for record in [1, 3] {
                assert!(c.envelope_context().iter().any(|x| {
                    x.source_range == (r[record].start + 24..r[record].start + 25)
                        && x.source_bytes == [value]
                }));
            }
        }
    }
}

#[test]
fn variant_structural_mutations_truncations_and_ambiguous_ranges_refuse() {
    for padded in [false, true] {
        let (b, r) = variant_fixture(padded);
        for n in 0..b.len() {
            assert!(decode_compact_conductor(&b[..n], r.clone()).is_err());
        }
        for record in 0..4 {
            for off in r[record].clone() {
                let local = off - r[record].start;
                if (record % 2 == 0 && (15..19).contains(&local))
                    || (record % 2 == 1 && local == 24)
                    || (!padded && record == 0 && (23..=24).contains(&local))
                {
                    continue;
                }
                let mut bad = b.clone();
                bad[off] ^= 128;
                assert!(
                    decode_compact_conductor(&bad, r.clone()).is_err(),
                    "padded={padded}, record={record}, local={local}"
                );
            }
        }
        for (from, to) in [(0, 2), (1, 3), (0, 1)] {
            let mut ranges = r.clone();
            ranges.swap(from, to);
            assert!(decode_compact_conductor(&b, ranges).is_err());
        }
        let mut ranges = r.clone();
        ranges[1] = r[0].clone();
        assert!(decode_compact_conductor(&b, ranges).is_err());
        let mut ranges = r.clone();
        ranges[2].start += 1;
        assert!(decode_compact_conductor(&b, ranges).is_err());
    }
}

#[test]
fn padding_cannot_hide_additional_tempo_meter_or_key_events() {
    let (b, r) = variant_fixture(true);
    let p = payloads(&b, &r);
    for extra in [
        vec![0, 255, 81, 3, 8, 0, 1],
        vec![0, 255, 88, 4, 4, 2, 8, 8],
        vec![0, 255, 89, 2, 0, 0],
    ] {
        for record in [0, 2] {
            let mut bad = p.clone();
            let event_end = if record == 0 { 22 } else { 21 };
            // Replace bytes in the fixed post-terminal pad without changing size.
            bad[record][event_end + 7..event_end + 7 + extra.len()].copy_from_slice(&extra);
            let (bytes, ranges) = reframe(&bad);
            assert!(decode_compact_conductor(&bytes, ranges).is_err());
            // A counted additional event before the terminal must also refuse,
            // even when framing/count/secondary lengths are updated consistently.
            let mut bad = p.clone();
            bad[record].splice(event_end..event_end, extra.iter().copied());
            bad[record][2..6].copy_from_slice(&2u32.to_be_bytes());
            bad[record + 1][6..10].copy_from_slice(&2u32.to_be_bytes());
            bad[record + 1][10..14]
                .copy_from_slice(&((event_end - 14 + extra.len() + 4) as u32).to_be_bytes());
            let (bytes, ranges) = reframe(&bad);
            assert!(decode_compact_conductor(&bytes, ranges).is_err());
        }
    }
}

#[test]
fn multiple_meter_lookalike_and_mixed_layouts_refuse() {
    let (b, r) = variant_fixture(true);
    let mut p = payloads(&b, &r);
    let meter = p[0][14..22].to_vec();
    p[0].truncate(14);
    p[0].extend(&meter);
    p[0].extend([0x8f, 0]); // Delayed second Meter: exactly 38 bytes overall.
    p[0].extend(&meter[1..]);
    p[0].extend([255, 255, 0xf0, 127, 255, 47, 0]);
    assert_eq!(p[0].len(), 38);
    p[0][2..6].copy_from_slice(&2u32.to_be_bytes());
    p[1][6..10].copy_from_slice(&2u32.to_be_bytes());
    p[1][10..14].copy_from_slice(&21u32.to_be_bytes());
    let (bytes, ranges) = reframe(&p);
    assert!(decode_compact_conductor(&bytes, ranges).is_err());
    for compact_meter in [false, true] {
        let (compact, cr) = fixture([9, 8, 7], [0, 0]);
        let (padded, pr) = variant_fixture(true);
        let mut cp = payloads(&compact, &cr);
        let pp = payloads(&padded, &pr);
        if compact_meter {
            cp[2..].clone_from_slice(&pp[2..]);
        } else {
            cp[..2].clone_from_slice(&pp[..2]);
        }
        let (bytes, ranges) = reframe(&cp);
        assert!(decode_compact_conductor(&bytes, ranges).is_err());
    }
}
