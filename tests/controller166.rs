use phoenix::{
    bounded_sequence::{assemble_bounded_sequence, build_bounded_sequence_manifest as manifest},
    controller166::{qualify_track, ControllerRefusal},
    midi_export::*,
    mixed_event::*,
    sequence_container::parse_project_166,
    smf::{serialize_channel_message, MidiChannel},
};
use sha2::{Digest, Sha256};
#[path = "support/reference_smf.rs"]
mod reference;
const CC: &[u8] = &[20, 255, 65, 5, 0, 1, 0, 7, 108];
fn record(b: &mut Vec<u8>, t: u8, p: &[u8]) {
    b.push(t);
    b.extend((p.len() as u32).to_be_bytes());
    b.extend(p);
}
fn fixture(events: &[u8], declared: u32) -> Vec<u8> {
    let tracks = 1;
    let mut b = vec![0; 8];
    for i in 0..2 {
        let mut p = vec![0; 36];
        p[25] = i;
        p[26] = 7;
        p[27] = 3;
        record(&mut b, 16, &p);
    }
    let mut device = vec![0; 40];
    device[0] = 4;
    device[1..5].copy_from_slice(b"Test");
    device[33] = 7;
    record(&mut b, 42, &device);
    let start = b.len();
    let mut p = vec![0; (tracks + 2) * 166 + 172];
    p[0] = (tracks + 2) as u8;
    record(&mut b, 1, &p);
    for i in 0..tracks {
        let l = start + 208 + (i + 2) * 166 + 15;
        b[l..l + 5].copy_from_slice(b"Other");
        b[l - 39..l - 31].copy_from_slice(&[128, 0, 4, 0, 0, 4, 1, 0]);
        b[l - 33] = 1;
        b[l - 31] = 1;
        b[l - 30..l].fill(255);
    }
    let mut name = vec![0; 11];
    name.extend([4, b'T', b'e', b's', b't']);
    record(&mut b, 7, &name);
    conductor_pair(&mut b, &[0, 255, 88, 4, 4, 2, 8, 8]);
    conductor_pair(&mut b, &[0, 255, 81, 3, 7, 53, 120]);
    for _ in 0..tracks {
        let mut p = vec![0; 14];
        p[1] = 1;
        p[2..6].copy_from_slice(&declared.to_be_bytes());
        p.extend(events);
        p.extend([255, 0, 0, 0, 255, 47, 0]);
        record(&mut b, 2, &p);
        record(&mut b, 41, &[]);
    }
    record(&mut b, 0, &[255; 8]);
    b
}
fn conductor_pair(b: &mut Vec<u8>, event: &[u8]) {
    let mut primary = vec![0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 12, 34, 56, 78];
    primary.extend(event);
    primary.extend([135, 255, 255, 127, 255, 47, 0]);
    primary.resize(38, 0);
    record(b, 2, &primary);
    let n = event.len() as u8;
    let mut secondary = vec![
        0,
        1,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        n + 19,
        0,
        0,
        0,
        n + 19,
        0,
        0,
        0,
        1,
        0,
        0,
        0,
        n + 4,
        0,
        255,
        255,
        255,
        47,
        197,
        0,
        0,
        0,
        0,
    ];
    secondary.push(event[2]);
    secondary.extend(&event[4..]);
    secondary.extend([255; 3]);
    record(b, 41, &secondary);
}
fn adapt(proofs: &[phoenix::controller166::RoutingValidatedController]) -> ExportTrackResult {
    let channel = proofs[0].provenance().channel;
    adapt_track(
        &proofs
            .iter()
            .map(|p| p.decoded_export_event())
            .collect::<Vec<_>>(),
        Some(ChannelAssignment {
            channel: MidiChannel::new(channel).unwrap(),
            provenance: ChannelAssignmentProvenance::ParsedRouting,
        }),
        TimingPolicy::Identity480,
        PatchPolicy::StrictKnownOnly,
    )
    .unwrap()
}
#[test]
fn bounded_handoff_preserves_context_timing_order_and_accounting() {
    let mut events = CC.to_vec();
    events.extend([0, 255, 65, 5, 0, 1, 0, 10, 14]);
    let b = fixture(&events, 2);
    let q = qualify_track(&b, 0, 0).unwrap();
    assert_eq!(q.len(), 2);
    for (i, p) in q.iter().map(|p| p.provenance()).enumerate() {
        assert_eq!(p.logical_source_ordinal, i as u64);
        assert_eq!(p.raw_source_object_ordinal, i as u64);
        assert_eq!(p.absolute_tick, 20);
        assert_eq!(p.interval, if i == 0 { 20 } else { 0 });
        assert_eq!(p.context_bytes, [0, 1, 0]);
        assert_eq!(&b[p.context_range.clone()], p.context_bytes);
        assert_eq!(&b[p.representation_range.clone()], p.representation_bytes);
        assert_eq!(&b[p.timing_range.clone()], p.timing_bytes);
        assert_eq!(p.declared_source_objects, 2);
    }
    let a = adapt(&q);
    assert_eq!(a.counts.controllers, 2);
    assert!(a.untranslated_metadata.is_empty());
    assert_eq!(
        a.scheduled_events
            .iter()
            .map(|e| (
                e.absolute_tick,
                e.stable_ordinal,
                serialize_channel_message(&e.message)
            ))
            .collect::<Vec<_>>(),
        [(20, 0, vec![0xb3, 7, 108]), (20, 2, vec![0xb3, 10, 14])]
    );
    let m = manifest(&b, 0, 480).unwrap();
    assert_eq!(m.tracks()[0].events().len(), 2);
    let midi = assemble_bounded_sequence(&m).unwrap();
    let tracks = reference::midi_tracks(&midi);
    assert_eq!(
        tracks[1]
            .iter()
            .filter(|(_, s, _)| *s == 0xb3)
            .cloned()
            .collect::<Vec<_>>(),
        [(20, 0xb3, vec![7, 108]), (20, 0xb3, vec![10, 14])]
    );
}
#[test]
fn wrong_context_malformed_data_ownership_and_counts_refuse() {
    for (at, value) in [(4, 1), (5, 0), (5, 2), (6, 1), (3, 4), (3, 6), (1, 254)] {
        let mut e = CC.to_vec();
        e[at] = value;
        assert!(qualify_track(&fixture(&e, 1), 0, 0).is_err());
    }
    for len in [4, 6, 8] {
        assert!(qualify_track(&fixture(&CC[..len], 1), 0, 0).is_err());
    }
    for at in [7, 8] {
        let mut e = CC.to_vec();
        e[at] = 128;
        assert_eq!(
            qualify_track(&fixture(&e, 1), 0, 0),
            Err(ControllerRefusal::MidiData)
        );
    }
    for n in [0, 2] {
        assert_eq!(
            qualify_track(&fixture(CC, n), 0, 0),
            Err(ControllerRefusal::Accounting)
        );
    }
    let mut e = CC.to_vec();
    e.extend([0, 254]);
    assert!(qualify_track(&fixture(&e, 2), 0, 0).is_err());
    let mut b = fixture(CC, 1);
    let s = parse_project_166(&b).unwrap().sequences.remove(0);
    b.drain(
        s.track_pairs[0].primary.record_range.start..s.track_pairs[0].secondary.record_range.end,
    );
    assert!(qualify_track(&b, 0, 0).is_err());
    let mut b = fixture(CC, 1);
    let start = parse_project_166(&b).unwrap().sequences[0].track_pairs[0]
        .primary
        .record_range
        .start;
    b[start + 1..start + 5].copy_from_slice(&999999u32.to_be_bytes());
    assert!(qualify_track(&b, 0, 0).is_err());
    assert!(qualify_track(&fixture(CC, 1), 1, 0).is_err());
    assert!(qualify_track(&fixture(CC, 1), 0, 1).is_err());
}
#[test]
fn generic_opaque_context_and_proof_binding_guards_remain() {
    let b = fixture(CC, 1);
    let q = qualify_track(&b, 0, 0).unwrap();
    let e = q[0].decoded_export_event();
    let route = Some(ChannelAssignment {
        channel: MidiChannel::new(4).unwrap(),
        provenance: ChannelAssignmentProvenance::ParsedRouting,
    });
    for field in 0..5 {
        let mut changed = e.clone();
        let mut assignment = route;
        match field {
            0 => changed.absolute_position += 1,
            1 => changed.source_ordinal += 1,
            2 => changed.source_range = Some(0..1),
            3 => assignment.as_mut().unwrap().channel = MidiChannel::new(5).unwrap(),
            _ => assignment.as_mut().unwrap().provenance = ChannelAssignmentProvenance::Synthetic,
        };
        assert!(adapt_track(
            &[changed],
            assignment,
            TimingPolicy::Identity480,
            PatchPolicy::StrictKnownOnly
        )
        .is_err());
    }
    let s = parse_project_166(&b).unwrap().sequences.remove(0);
    let bounds = s.validated_track_event_bounds(0).unwrap().event_range;
    let walk = walk_bounded_mixed_events(
        &b,
        MixedEventBounds {
            event_range: bounds,
        },
        Default::default(),
    )
    .unwrap();
    let MixedEventItem::Event(e) = &walk.items[0] else {
        panic!()
    };
    let MixedEventKind::Controller(c) = &e.event else {
        panic!()
    };
    let old = DecodedExportEvent::from_controller(e.position, 0, c);
    let a = adapt_track(
        &[old],
        route,
        TimingPolicy::Identity480,
        PatchPolicy::StrictKnownOnly,
    )
    .unwrap();
    assert_eq!(
        a.untranslated_metadata,
        [UntranslatedMetadata::ControllerContext { source_ordinal: 0 }]
    );
    assert!(adapt_track(
        &[q[0].decoded_export_event()],
        None,
        TimingPolicy::Identity480,
        PatchPolicy::StrictKnownOnly
    )
    .is_err());
}
#[test]
fn ff60_controller_and_context_note_are_not_translated() {
    for (events, n) in [
        (vec![0, 0xb0, 7, 64], 1),
        (
            vec![
                4, 255, 96, 7, 35, 96, 0, 114, 108, 131, 76, 1, 255, 65, 5, 0, 1, 0, 1, 58,
            ],
            2,
        ),
        (
            vec![
                4, 255, 96, 7, 35, 96, 0, 114, 108, 131, 76, 1, 144, 60, 100, 64, 12,
            ],
            2,
        ),
    ] {
        let b = fixture(&events, n);
        assert!(qualify_track(&b, 0, 0).unwrap().is_empty());
        assert_eq!(manifest(&b, 0, 480).unwrap_err().stage, "translation");
    }
}
fn read(name: &str, hash: &str) -> Vec<u8> {
    let b = std::fs::read(format!(
        "/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/{name}"
    ))
    .unwrap();
    assert_eq!(format!("{:x}", Sha256::digest(&b)), hash);
    b
}
#[test]
fn authenticated_all598_native594_and_instrument_reassignment() {
    let mut track9 = Vec::new();
    for (name, hash, mid, mid_hash, association, channel) in [
        (
            "CCA073 CONTROL",
            "e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132",
            "073 CTRL xForm.mid",
            "872272ce43c389474fdb408416ad2406fa9a63de5e1d8bb94e0f6bbced7a1241",
            1,
            15,
        ),
        (
            "CCA073 EDIT",
            "3f12c7bb9371d76dd4d4da081ccf347101cd5d4f187fd039a9562665d87d0c87",
            "073 EDIT xForm.mid",
            "50eeb37411243e3c8083b8c86d499ee1f5b755444f0024d7b72fbeb182567145",
            6,
            3,
        ),
    ] {
        let b = read(name, hash);
        let midi = read(mid, mid_hash);
        let native = reference::midi_tracks(&midi);
        let project = parse_project_166(&b).unwrap();
        let mut total = 0;
        let mut represented = 0;
        let mut target = Vec::new();
        for pair in 0..13 {
            let q = qualify_track(&b, 0, pair).unwrap();
            assert_eq!(q.len(), [1, 1, 0, 0, 0, 68, 0, 0, 443, 77, 3, 1, 4][pair]);
            total += q.len();
            if q.is_empty() {
                continue;
            }
            let a = adapt(&q);
            assert_eq!(a.counts.controllers, q.len() as u64);
            assert!(a.untranslated_metadata.is_empty());
            let actual: Vec<_> = a
                .scheduled_events
                .iter()
                .map(|e| (e.absolute_tick, serialize_channel_message(&e.message)))
                .collect();
            if pair < 12 {
                let label = project.sequences[0].track_descriptors()[pair]
                    .label
                    .as_ref()
                    .unwrap()
                    .bytes;
                let matches: Vec<_> = native
                    .iter()
                    .enumerate()
                    .filter(|(_, events)| events.iter().any(|(_, s, d)| *s == 3 && d == label))
                    .collect();
                assert_eq!(matches.len(), 1);
                let (_, track) = matches[0];
                // Bank Controllers belong to Patch expansions, not ordinary FF41.
                // The sole context-mediated CC1 at tick128709 remains excluded.
                let expected: Vec<_> = track
                    .iter()
                    .filter(|(t, s, d)| {
                        *s >> 4 == 11
                            && !matches!(d[0], 0 | 32)
                            && !(pair == 8 && *t == 128709 && d.as_slice() == [1, 58])
                    })
                    .map(|(t, s, d)| (*t, vec![*s, d[0], d[1]]))
                    .collect();
                assert_eq!(actual, expected, "{name} pair {pair}");
                represented += q.len();
            }
            if pair == 0 {
                let p = q[0].provenance();
                assert_eq!(
                    (p.absolute_tick, p.number, p.value, p.channel),
                    (20, 7, 108, 15)
                );
                assert_eq!(p.representation_range, 0x7671..0x767a);
                assert_eq!(p.logical_source_ordinal, 0);
            }
            if pair == 8 {
                for proof in &q {
                    let p = proof.provenance();
                    assert_eq!(p.context_bytes, [0, association, 0]);
                    assert_eq!(p.channel, channel);
                }
                target = actual
                    .iter()
                    .map(|(t, b)| (*t, b[1..].to_vec()))
                    .collect::<Vec<_>>();
            }
            for proof in &q {
                let p = proof.provenance();
                assert_eq!(&b[p.representation_range.clone()], p.representation_bytes);
                assert_eq!(&b[p.context_range.clone()], p.context_bytes);
            }
        }
        assert_eq!((total, represented), (598, 594));
        track9.push(target);
        let e = manifest(&b, 0, 480).unwrap_err();
        assert_eq!(
            e.stage,
            if association == 1 {
                "translation"
            } else {
                "conductor coverage"
            }
        );
        if association == 1 {
            assert!(e.detail.contains("unsupported parsed event"));
        }
        assert_eq!(format!("{:x}", Sha256::digest(read(name, hash))), hash);
    }
    assert_eq!(track9[0], track9[1]);
}
