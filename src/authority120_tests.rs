use super::*;
use crate::observed_layout120::tests::synthetic_marker;

fn record(b: &mut Vec<u8>, kind: u8, p: &[u8]) -> usize {
    let at = b.len();
    b.push(kind);
    b.extend((p.len() as u32).to_be_bytes());
    b.extend(p);
    at
}
fn fixture(events: &[u8], raw: u32, pairs: usize, absent: bool) -> Vec<u8> {
    let mut b = synthetic_marker(3, b"Portable", [0xfe, 0xff]);
    b.truncate(8 + 75 + 120 * 3);
    for (i, label) in [b"Meter Track".as_slice(), b"Tempo Track", b"Part"]
        .iter()
        .enumerate()
    {
        let at = 8 + 121 + 120 * i;
        b[at..at + label.len()].copy_from_slice(label);
        b[at - 39..at - 31].copy_from_slice(&[0x80, 0, 4, 0, 0, 4, 1, 0]);
    }
    let label = 8 + 121 + 240;
    b[label - 33..label].fill(0xff);
    b[label - 33..label - 30].copy_from_slice(&[1, 0, 1]);
    if absent {
        b[label - 39..label - 31].copy_from_slice(&[0, 0, 4, 0, 0, 4, 0, 0]);
    }
    record(&mut b, 7, &[0; 10]);
    for i in 0..pairs {
        let mut p = vec![0; 14];
        if i >= 2 {
            p[2..6].copy_from_slice(&raw.to_be_bytes());
            p.extend(events);
        }
        p.extend([0xff, 0, 0, 0, 0xff, 0x2f, 0]);
        record(&mut b, 2, &p);
        record(&mut b, 0x29, &[0; 23]);
    }
    record(&mut b, 0, &[0xff; 4]);
    for i in 0..2 {
        let mut p = vec![0; 36];
        p[25] = i;
        p[26] = 7;
        p[27] = 4;
        record(&mut b, 0x10, &p);
    }
    let mut device = vec![0; 40];
    device[0] = 3;
    device[1..4].copy_from_slice(b"Box");
    device[33] = 7;
    record(&mut b, 0x2a, &device);
    b
}
fn note() -> Vec<u8> {
    vec![0, 0x90, 60, 100, 64, 0x81, 0x70]
}
fn result(b: &[u8]) -> Result<SequenceProof120<'_>, Authority120Refusal> {
    prove_project120(b).unwrap().remove(0).result
}

#[test]
fn owned_source_and_counted_note_resolve_without_166_authority() {
    let b = fixture(&note(), 1, 3, false);
    assert!(crate::sequence_container::parse_project_166(&b).is_err());
    let p = result(&b).unwrap();
    assert_eq!(p.source(), b);
    let t = &p.tracks()[0];
    assert_eq!(t.ordinal(), 2);
    assert_eq!(t.name(), b"Part");
    assert_eq!(t.declared(), Some(1));
    assert_eq!(t.walk().unwrap().logical_event_count(), 1);
    let TrackRoute120::Single(r) = t.route() else {
        panic!("single route")
    };
    assert_eq!(r.channel, 5);
    assert_eq!(r.instrument_row, 1);
}
#[test]
fn declared_shortage_and_surplus_refuse() {
    for raw in [0, 2] {
        assert!(matches!(
            result(&fixture(&note(), raw, 3, false)),
            Err(Authority120Refusal::Accounting { .. })
        ));
    }
}
#[test]
fn cardinality_shortages_excess_and_missing_pair_never_grant_ownership() {
    for pairs in [0, 1, 2, 4] {
        assert!(matches!(
            result(&fixture(&note(), 1, pairs, false)),
            Err(Authority120Refusal::Association(_))
        ));
    }
}
#[test]
fn no_event_data_and_paired_empty_are_distinct() {
    let b = fixture(&[], 0, 2, true);
    let p = result(&b).unwrap();
    let t = &p.tracks()[0];
    assert_eq!(t.declared(), None);
    assert!(t.walk().is_none());
    assert!(t.pair_ranges().is_none());
    assert!(t.absent_pair_state_range().is_some());
    let b = fixture(&[], 0, 3, false);
    let p = result(&b).unwrap();
    let t = &p.tracks()[0];
    assert_eq!(t.declared(), Some(0));
    assert!(t.walk().unwrap().items.is_empty());
    assert!(t.pair_ranges().is_some());
    assert!(matches!(
        result(&fixture(&[], 1, 3, false)),
        Err(Authority120Refusal::Accounting { .. })
    ));
}
#[test]
fn contexts_count_as_records_not_additional_notes() {
    for second in [None, Some(7), Some(8)] {
        let mut e = vec![0, 0xff, 0x60, 6, 0x57, 0x7f, 0, 1, 2, 3];
        if let Some(len) = second {
            e.extend([0, 0xff, 0x60, len]);
            e.extend([0x57, 0x7f, 0]);
            e.extend(vec![4; usize::from(len) - 3]);
        }
        e.extend(note());
        let c = if second.is_some() { 2 } else { 1 };
        let b = fixture(&e, 1 + c, 3, false);
        let p = result(&b).unwrap();
        assert_eq!(p.tracks()[0].context_records(), c as usize);
        assert_eq!(p.tracks()[0].walk().unwrap().logical_event_count(), 1);
        assert!(matches!(
            result(&fixture(&e, 1, 3, false)),
            Err(Authority120Refusal::Accounting { .. })
        ));
    }
}
#[test]
fn unknown_event_data_never_reconciles_merely_by_declared_count() {
    for e in [
        vec![0, 0xff, 0x40, 0],
        vec![0, 0xf0, 1, 2],
        vec![0, 0xff, 0x60, 1, 0x57],
        {
            let mut e = note();
            e.push(0xfe);
            e
        },
    ] {
        assert!(result(&fixture(&e, 1, 3, false)).is_err());
    }
}
#[test]
fn missing_ambiguous_tables_devices_and_invalid_channel_refuse() {
    for field in [25, 27] {
        let mut b = fixture(&note(), 1, 3, false);
        let root = parse_root_record_stream(&b).unwrap();
        let at = root
            .records
            .iter()
            .filter(|r| r.record_type.value == 0x10)
            .nth(1)
            .unwrap()
            .payload
            .range
            .start;
        b[at + field] = if field == 27 { 16 } else { 0 };
        assert!(result(&b).is_err());
    }
    let mut b = fixture(&note(), 1, 3, false);
    let r = parse_root_record_stream(&b).unwrap();
    let at = r
        .records
        .iter()
        .find(|r| r.record_type.value == 0x2a)
        .unwrap()
        .record_range
        .start;
    let duplicate = b[at..].to_vec();
    b.extend(duplicate);
    assert!(matches!(result(&b), Err(Authority120Refusal::Device)));
    b.truncate(at);
    assert!(matches!(result(&b), Err(Authority120Refusal::Device)));
    let mut b = fixture(&note(), 1, 3, false);
    let r = parse_root_record_stream(&b).unwrap();
    let at = r
        .records
        .iter()
        .find(|r| r.record_type.value == 0x10)
        .unwrap()
        .record_range
        .start;
    b.truncate(at);
    assert!(matches!(result(&b), Err(Authority120Refusal::RoutingTable)));
}
#[test]
fn altered_association_and_conflicting_context_refuse() {
    let mut b = fixture(&note(), 1, 3, false);
    b[8 + 121 + 240 - 33 + 3] = 2;
    assert!(matches!(
        result(&b),
        Err(Authority120Refusal::AssociationShape)
    ));
    let mut e = vec![0, 0xff, 0x60, 6, 0, 0, 0, 1, 2, 3];
    e.extend(note());
    assert!(matches!(
        result(&fixture(&e, 2, 3, false)),
        Err(Authority120Refusal::Context(_))
    ));
}
#[test]
fn framing_truncation_and_uncovered_extent_refuse() {
    let b = fixture(&note(), 1, 3, false);
    for end in [7, 50, b.len() - 1] {
        assert!(prove_project120(&b[..end]).is_err());
    }
    let note_bytes = note();
    let walk = walk_bounded_mixed_events(
        &note_bytes,
        MixedEventBounds { event_range: 0..7 },
        Default::default(),
    )
    .unwrap();
    assert!(matches!(
        reconcile(&walk, &(0..6), 1),
        Err(Authority120Refusal::Bounds)
    ));
}

#[test]
fn authentic_external_supported_and_blocked_proof_parity_when_available() {
    use sha2::{Digest, Sha256};
    let path="/Users/kurtheiden/Documents/Phoenix Research/Authentic Studio Vision Projects/SCHOOL PROJECTS";
    let Ok(b) = std::fs::read(path) else {
        eprintln!("external SCHOOL fixture unavailable; portable tests still run");
        return;
    };
    assert_eq!(b.len(), 343875);
    assert_eq!(
        format!("{:x}", Sha256::digest(&b)),
        "bfd4fa1208e2cd884ec51ccfb1131d5c02723d7c90acafc98bd89597e1a20331"
    );
    let proofs = prove_project120(&b).unwrap();
    assert_eq!(proofs.len(), 15);
    let accepted = [0, 1, 4, 7, 8, 9, 11, 12];
    let mut paired = 0;
    let mut absent = 0;
    for c in proofs {
        eprintln!(
            "candidate {} proof {:?}",
            c.ordinal + 1,
            c.result.as_ref().map(|p| p.tracks().len())
        );
        assert_eq!(c.result.is_ok(), accepted.contains(&c.ordinal));
        if let Ok(p) = c.result {
            for t in p.tracks() {
                if let Some(w) = t.walk() {
                    paired += 1;
                    assert_eq!(
                        usize::try_from(t.declared().unwrap()).unwrap(),
                        w.logical_event_count() + t.context_records()
                    );
                } else {
                    absent += 1;
                    assert!(t.absent_pair_state_range().is_some());
                }
            }
        }
    }
    assert_eq!((paired, absent), (64, 7));
}

#[test]
fn unrelated_pair_framing_and_zero_state_with_an_extra_pair_refuse() {
    let mut b = fixture(&note(), 1, 3, false);
    let root = parse_root_record_stream(&b).unwrap();
    let secondary = root
        .records
        .iter()
        .filter(|r| r.record_type.value == 0x29)
        .nth(2)
        .unwrap()
        .record_range
        .start;
    b[secondary] = 0x10;
    assert!(matches!(
        result(&b),
        Err(Authority120Refusal::Association(_))
    ));
    assert!(matches!(
        result(&fixture(&note(), 1, 3, true)),
        Err(Authority120Refusal::Association(_))
    ));
}

#[test]
fn source_snapshot_and_different_channel_cannot_be_mixed() {
    let a = fixture(&note(), 1, 3, false);
    let mut b = a.clone();
    let records = parse_root_record_stream(&b).unwrap();
    let row = records
        .records
        .iter()
        .filter(|r| r.record_type.value == 0x10)
        .nth(1)
        .unwrap()
        .payload
        .range
        .start;
    b[row + 27] = 9;
    let p = result(&a).unwrap();
    let q = result(&b).unwrap();
    let TrackRoute120::Single(x) = p.tracks()[0].route() else {
        panic!()
    };
    let TrackRoute120::Single(y) = q.tracks()[0].route() else {
        panic!()
    };
    assert_eq!((x.channel, y.channel), (5, 10));
    assert_eq!(p.source(), a);
    assert_eq!(q.source(), b);
    // There is no public constructor accepting a foreign walk, pair or route.
}
