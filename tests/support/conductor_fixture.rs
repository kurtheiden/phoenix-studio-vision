//! Synthetic initial-conductor envelopes shared by integration and parity tests.
use std::ops::Range;

pub(crate) fn fixture(mpqn: [u8; 3], key: [u8; 2]) -> (Vec<u8>, [Range<usize>; 4]) {
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

pub(crate) fn reframe(payloads: &[Vec<u8>; 4]) -> (Vec<u8>, [Range<usize>; 4]) {
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

pub(crate) fn payloads(bytes: &[u8], ranges: &[Range<usize>; 4]) -> [Vec<u8>; 4] {
    std::array::from_fn(|i| bytes[ranges[i].start + 5..ranges[i].end].to_vec())
}

pub(crate) fn variant_fixture(padded: bool) -> (Vec<u8>, [Range<usize>; 4]) {
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
