use phoenix::{
    bank_patch166_handoff::qualify_track as qualify,
    bounded_sequence::{assemble_bounded_sequence, build_bounded_sequence_manifest as manifest},
    midi_export::*,
    sequence_container::parse_project_166,
    smf::{serialize_channel_message, MidiChannel},
};
use sha2::{Digest, Sha256};
#[path = "support/reference_smf.rs"]
mod reference;
const NOTE: &[u8] = &[0, 144, 60, 100, 64, 12];
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
fn patch(family: u8, tick: u8, standalone: bool) -> Vec<u8> {
    let p = 42;
    let (pre, post): (Vec<u8>, Vec<u8>) = match family {
        0 => (vec![0, 0, p, 8, p], vec![2, b'9', b'9', 4, 255, 80, 255]),
        1 => (
            vec![0, 0, p | 128, 8, p],
            vec![3, b'I', b'9', b'9', 4, 255, 80, 255],
        ),
        2 => (vec![0, 0, p, 8, p], vec![2, b'9', b'9', 4, 255, 80, 0]),
        3 => (vec![0, 1, p, 248, p | 128], vec![1, b'9', 4, 255, 81, 1]),
        _ => panic!(),
    };
    let mut payload = pre;
    payload.push(5);
    payload.extend(b"Fresh");
    payload.extend(post);
    payload.push(p);
    let mut e = vec![tick, 255, 124, payload.len() as u8];
    e.extend(payload);
    if standalone {
        e.extend([3, 255, 65, 5, 0, 1, 0, 7, 90]);
    }
    e.extend(NOTE);
    e
}
fn adapt(q: &phoenix::bank_patch166_handoff::OwnedBankPatch) -> ExportTrackResult {
    adapt_track(
        &q.decoded_export_events(),
        Some(ChannelAssignment {
            channel: MidiChannel::new(q.routing().midi_channel).unwrap(),
            provenance: ChannelAssignmentProvenance::ParsedRouting,
        }),
        TimingPolicy::Identity480,
        PatchPolicy::StrictKnownOnly,
    )
    .unwrap()
}
#[test]
fn integrated_families_keep_bank_absence_zero_source_order_and_two_objects() {
    for family in 0..4 {
        let mut e = vec![9, 255, 65, 5, 0, 1, 0, 7, 90];
        e.extend(patch(family, 37, false));
        let b = fixture(&e, 3);
        let q = qualify(&b, 0, 0).unwrap();
        let p = q.provenance();
        assert_eq!(
            (
                p.patch_tick,
                p.note_tick,
                p.logical_source_ordinal,
                p.declared_source_objects
            ),
            (46, 46, 1, 3)
        );
        assert_eq!(p.raw_note_ordinal, 2);
        let a = adapt(&q);
        let messages: Vec<_> = a
            .scheduled_events
            .iter()
            .map(|e| (e.absolute_tick, serialize_channel_message(&e.message)))
            .collect();
        let msb = if family == 3 { 81 } else { 80 };
        let mut expected = vec![(46, vec![0xb3, 0, msb])];
        if family >= 2 {
            expected.push((46, vec![0xb3, 32, if family == 3 { 1 } else { 0 }]));
        }
        expected.extend([
            (46, vec![0xc3, 42]),
            (46, vec![0x93, 60, 100]),
            (58, vec![0x83, 60, 64]),
        ]);
        assert_eq!(messages, expected);
        let m = manifest(&b, 0, 480).unwrap();
        assert_eq!(m.tracks()[0].events().len(), 3);
        assert_eq!(m.tracks()[0].bank_patches(), [q]);
        let midi = assemble_bounded_sequence(&m).unwrap();
        let tr = reference::midi_tracks(&midi);
        let actual: Vec<_> = tr[1]
            .iter()
            .filter(|(_, s, _)| *s >= 128 && *s < 240)
            .map(|(t, s, d)| (*t, [vec![*s], d.clone()].concat()))
            .collect();
        let mut expected = vec![(9, vec![0xb3, 7, 90])];
        expected.extend(messages);
        assert_eq!(actual, expected);
    }
}
#[test]
fn embedded_context_is_owned_provenance_not_an_extra_musical_object() {
    let mut e = patch(1, 37, false);
    e.truncate(e.len() - NOTE.len());
    e.extend([4, 255, 96, 7, 87, 127, 0, 1, 2, 3, 4, 5]);
    e.extend(NOTE.iter().skip(1));
    let b = fixture(&e, 3);
    let q = qualify(&b, 0, 0).unwrap();
    let p = q.provenance();
    let c = p.context.as_ref().unwrap();
    assert_eq!(p.note_tick, 46);
    assert_eq!(p.raw_note_ordinal, 2);
    assert_eq!(p.logical_source_ordinal, 0);
    assert_eq!(&b[c.range.clone()], c.bytes);
    assert_eq!(&b[c.payload_range.clone()], c.payload);
    assert_eq!(c.final_interval, 5);
    let m = manifest(&b, 0, 480).unwrap();
    assert_eq!(m.tracks()[0].events().len(), 2);
    assert_eq!(m.tracks()[0].bank_patches(), [q]);
}
#[test]
fn malformed_unqualified_standalone_and_accounting_forms_stay_blocked() {
    for family in 0..4 {
        let e = patch(family, 37, false);
        let b = fixture(&e, 2);
        let q = qualify(&b, 0, 0).unwrap();
        let p = q.provenance();
        let start = p.composition_range.start;
        for (at, v) in [
            (start + 3, 255),
            (start + 4, 1),
            (start + 8, 43),
            (p.patch_range.end - 6, 82),
        ] {
            let mut bad = b.clone();
            bad[at] = v;
            assert!(qualify(&bad, 0, 0).is_err());
            assert!(manifest(&bad, 0, 480).is_err());
        }
        let s = parse_project_166(&b).unwrap().sequences.remove(0);
        let at = s.track_pairs[0].primary.payload.range.start + 2;
        let mut bad = b.clone();
        bad[at..at + 4].copy_from_slice(&3u32.to_be_bytes());
        assert!(qualify(&bad, 0, 0).is_err());
        assert!(manifest(&bad, 0, 480).is_err());
        let mut muted = b.clone();
        muted[s.track_descriptors()[0].label_start - 39] = 0x88;
        assert!(qualify(&muted, 0, 0).is_ok());
        assert_eq!(manifest(&muted, 0, 480).unwrap_err().stage, "mute");
        let mut unowned = b.clone();
        unowned.drain(
            s.track_pairs[0].primary.record_range.start
                ..s.track_pairs[0].secondary.record_range.end,
        );
        assert!(qualify(&unowned, 0, 0).is_err());
        assert!(manifest(&unowned, 0, 480).is_err());
        let mut bad = b.clone();
        bad[s.track_descriptors()[0].label_start - 31] = 99;
        assert!(qualify(&bad, 0, 0).is_err());
        let standalone = fixture(&patch(family, 37, true), 3);
        assert!(qualify(&standalone, 0, 0).is_err());
        assert!(manifest(&standalone, 0, 480).is_err());
    }
    let mut e = NOTE.to_vec();
    e.extend(patch(2, 37, false));
    assert!(qualify(&fixture(&e, 3), 0, 0).is_err());
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
fn authenticated_five_compositions_match_native_bank_program_note_and_ending() {
    let b = read(
        "CCA073 CONTROL",
        "e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132",
    );
    let midi = read(
        "073 CTRL xForm.mid",
        "872272ce43c389474fdb408416ad2406fa9a63de5e1d8bb94e0f6bbced7a1241",
    );
    let tracks = reference::midi_tracks(&midi);
    for (pair, patch_tick, note_tick, channel, raw) in [
        (1, 60, 2852, 1, 247),
        (3, 80, 24960, 9, 196),
        (6, 120, 73655, 10, 295),
        (7, 140, 70800, 12, 104),
        (9, 960, 94080, 2, 108),
    ] {
        let q = qualify(&b, 0, pair).unwrap();
        let p = q.provenance();
        assert_eq!(
            (
                p.patch_tick,
                p.note_tick,
                q.routing().midi_channel,
                p.declared_source_objects
            ),
            (patch_tick, note_tick, channel, raw)
        );
        assert_eq!(&b[p.composition_range.clone()], p.composition_bytes);
        assert_eq!(p.context.is_some(), matches!(pair, 3 | 7 | 9));
        if let Some(c) = &p.context {
            assert_eq!(&b[c.range.clone()], c.bytes);
            assert_eq!(&b[c.payload_range.clone()], c.payload);
        }
        let events = q.decoded_export_events();
        let DecodedExportEventKind::Note {
            pitch,
            attack_velocity,
            release_velocity,
            duration,
        } = events[1].kind
        else {
            panic!()
        };
        let end = note_tick + duration;
        let a = adapt(&q);
        assert_eq!(a.counts.notes, 1);
        assert_eq!(a.counts.program_changes, 1);
        let actual: Vec<_> = a
            .scheduled_events
            .iter()
            .map(|e| (e.absolute_tick, serialize_channel_message(&e.message)))
            .collect();
        let named: Vec<_> = tracks
            .iter()
            .filter(|t| {
                t.iter()
                    .any(|(_, s, d)| *s == 3 && d == &q.routing().label_bytes)
            })
            .collect();
        assert_eq!(named.len(), 1);
        let expected: Vec<_> = named[0]
            .iter()
            .filter(|(t, s, d)| {
                (*t == patch_tick && (*s >> 4 == 12 || (*s >> 4 == 11 && matches!(d[0], 0 | 32))))
                    || (*t == note_tick && *s >> 4 == 9 && d[0] == pitch && d[1] == attack_velocity)
                    || (*t == end && d[0] == pitch && (*s >> 4 == 8 || (*s >> 4 == 9 && d[1] == 0)))
            })
            .map(|(t, s, d)| {
                if *s >> 4 == 9 && d[1] == 0 {
                    (*t, vec![0x80 | (*s & 15), d[0], 64])
                } else {
                    (*t, [vec![*s], d.clone()].concat())
                }
            })
            .collect();
        assert_eq!(actual, expected, "pair {pair}");
        if pair == 1 {
            assert_eq!(p.composition_range, 0x79bf..0x79e5);
            assert_eq!(
                (
                    p.patch_interval,
                    p.post_pc_interval,
                    p.logical_source_ordinal
                ),
                (40, 2792, 1)
            );
            assert_eq!(
                (pitch, attack_velocity, release_velocity, duration),
                (65, 64, 64, 2745)
            );
            assert_eq!(
                actual,
                [
                    (60, vec![0xb0, 0, 80]),
                    (60, vec![0xc0, 33]),
                    (2852, vec![0x90, 65, 64]),
                    (5597, vec![0x80, 65, 64])
                ]
            );
        }
    }
    for pair in [0, 2, 4, 5, 8, 10, 11, 12] {
        assert!(qualify(&b, 0, pair).is_err());
    }
    let err = manifest(&b, 0, 480).unwrap_err();
    assert_eq!(err.stage, "translation");
    assert!(err.detail.contains("unsupported parsed event"));
    let mut app = phoenix::app_service::AppService::new();
    let r = app
        .inspect_project(phoenix::app_contract::InspectProjectRequest {
            contract_version: phoenix::app_contract::CONTRACT_VERSION,
            source_path: "/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/CCA073 CONTROL"
                .into(),
            diagnostics_level: phoenix::app_contract::DiagnosticsLevel::Summary,
        })
        .unwrap();
    let row = r
        .sequences
        .iter()
        .find(|s| s.display_name == "xForm")
        .unwrap();
    assert!(row.export_capability.is_none());
    assert!(row.bounded_export_capability.is_none());
    assert_eq!(
        row.readiness,
        phoenix::app_contract::Readiness::PartiallySupported
    );
}
