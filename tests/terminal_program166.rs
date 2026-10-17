use phoenix::{
    bounded_sequence::{assemble_bounded_sequence, build_bounded_sequence_manifest as manifest},
    midi_export::*,
    mixed_event::*,
    sequence_container::parse_project_166,
    smf::{self, serialize_channel_message, MidiChannel},
    terminal_program166::qualify_track as qualify,
};
use sha2::{Digest, Sha256};
#[path = "support/reference_smf.rs"]
mod reference;
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
fn core() -> Vec<u8> {
    let name = b"Different name";
    let mut e = vec![
        10,
        255,
        124,
        (name.len() + 15) as u8,
        0,
        0,
        52,
        0,
        52,
        name.len() as u8,
    ];
    e.extend(name);
    e.extend([3, b'I', b'7', b'5', 4, 255, 255, 255, 52]);
    e
}
fn source() -> Vec<u8> {
    let mut e = vec![0, 144, 60, 100, 64, 100, 0, 144, 65, 110, 90, 120];
    e.extend(core());
    fixture(&e, 3)
}
fn assignment(ch: u8) -> Option<ChannelAssignment> {
    Some(ChannelAssignment {
        channel: MidiChannel::new(ch).unwrap(),
        provenance: ChannelAssignmentProvenance::ParsedRouting,
    })
}
#[test]
fn terminal_proof_preserves_suffix_provenance_and_pending_note_endings() {
    let b = source();
    let q = qualify(&b, 0, 0).unwrap();
    let p = q.provenance();
    assert_eq!(
        (
            p.previous_tick,
            p.interval,
            p.absolute_tick,
            p.logical_source_ordinal,
            p.raw_source_object_ordinal
        ),
        (0, 10, 10, 2, 2)
    );
    assert_eq!(p.representation_range.end, p.event_bounds.end);
    assert_eq!(p.track_suffix_range.start, p.event_bounds.end);
    assert_eq!(p.track_suffix_range.len(), 7);
    assert_eq!(&b[p.representation_range.clone()], p.representation_bytes);
    assert_eq!(&b[p.pre_name_range.clone()], p.pre_name_bytes);
    assert_eq!(&b[p.post_name_range.clone()], p.post_name_bytes);
    let a = adapt_track(
        &[q.decoded_export_event()],
        assignment(4),
        TimingPolicy::Identity480,
        PatchPolicy::StrictKnownOnly,
    )
    .unwrap();
    assert_eq!(
        (
            a.counts.program_changes,
            a.counts.controllers,
            a.counts.bank_select_msb,
            a.counts.bank_select_lsb
        ),
        (1, 0, 0, 0)
    );
    assert_eq!(
        serialize_channel_message(&a.scheduled_events[0].message),
        [0xc3, 52]
    );
    let m = manifest(&b, 0, 480).unwrap();
    assert_eq!(m.tracks()[0].terminal_programs(), [q]);
    let midi = assemble_bounded_sequence(&m).unwrap();
    let tr = reference::midi_tracks(&midi);
    assert_eq!(
        tr[1]
            .iter()
            .filter(|(_, s, _)| *s >= 128 && *s < 240)
            .cloned()
            .collect::<Vec<_>>(),
        [
            (0, 0x93, vec![60, 100]),
            (0, 0x93, vec![65, 110]),
            (10, 0xc3, vec![52]),
            (100, 0x83, vec![60, 64]),
            (120, 0x83, vec![65, 90])
        ]
    );
}
#[test]
fn wrapper_framing_program_routing_and_accounting_negatives() {
    let b = source();
    let q = qualify(&b, 0, 0).unwrap();
    let p = q.provenance();
    let start = p.representation_range.start;
    let pre = p.pre_name_range.start;
    let post = p.post_name_range.start;
    let end = p.representation_range.end;
    for (at, value) in [
        (start + 3, 1),
        (pre, 1),
        (pre + 1, 1),
        (pre + 2, 51),
        (pre + 3, 8),
        (pre + 4, 51),
        (pre + 5, 1),
        (post, 2),
        (post + 1, b'G'),
        (post + 2, b'x'),
        (post + 4, 5),
        (post + 5, 0),
        (post + 6, 80),
        (post + 7, 0),
        (end - 1, 128),
    ] {
        let mut bad = b.clone();
        bad[at] = value;
        assert!(qualify(&bad, 0, 0).is_err(), "offset {at:x}");
    }
    let mut label = b.clone();
    label[post + 2..post + 4].copy_from_slice(b"99");
    let q = qualify(&label, 0, 0).unwrap();
    assert_eq!(q.provenance().program, 52);
    let s = parse_project_166(&b).unwrap().sequences.remove(0);
    let at = s.track_pairs[0].primary.payload.range.start;
    let descriptor = s.track_descriptors()[0].label_start;
    for count in [2u32, 4] {
        let mut bad = b.clone();
        bad[at + 2..at + 6].copy_from_slice(&count.to_be_bytes());
        assert!(qualify(&bad, 0, 0).is_err());
    }
    let mut bad = b.clone();
    bad[descriptor - 31] = 0;
    assert!(qualify(&bad, 0, 0).is_err());
    let mut bad = b.clone();
    bad.drain(
        s.track_pairs[0].primary.record_range.start..s.track_pairs[0].secondary.record_range.end,
    );
    assert!(qualify(&bad, 0, 0).is_err());
    let mut e = vec![0, 144, 60, 100, 64, 100];
    let mut c = core();
    c.pop();
    e.extend(c);
    assert!(qualify(&fixture(&e, 2), 0, 0).is_err());
}
#[test]
fn exact_end_no_suffix_consumption_nonterminal_or_banked_forms() {
    for tail in [
        vec![0],
        vec![0, 254],
        vec![255, 0, 0, 0, 255, 47, 0],
        vec![0, 144, 60, 100, 64, 12],
    ] {
        let mut e = core();
        e.extend(tail);
        assert!(qualify(&fixture(&e, 2), 0, 0).is_err());
    }
    let mut e = core();
    e.extend([0, 255, 65, 5, 0, 1, 0, 7, 64, 0, 144, 60, 100, 64, 12]);
    assert!(qualify(&fixture(&e, 3), 0, 0).is_err());
    let mut c = core();
    let n = c.len();
    c[n - 3] = 80;
    assert!(qualify(&fixture(&c, 1), 0, 0).is_err());
    let mut e = vec![0, 255, 96, 7, 87, 127, 0, 1, 1, 1, 1];
    e.extend(core());
    assert!(qualify(&fixture(&e, 2), 0, 0).is_err());
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
fn authenticated_complete_track1_native_program_and_held_notes() {
    let b = read(
        "CCA073 CONTROL",
        "e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132",
    );
    let midi = read(
        "073 CTRL xForm.mid",
        "872272ce43c389474fdb408416ad2406fa9a63de5e1d8bb94e0f6bbced7a1241",
    );
    let q = qualify(&b, 0, 0).unwrap();
    let p = q.provenance();
    assert_eq!(p.representation_range, 0x7835..0x7854);
    assert_eq!(
        (p.previous_tick, p.interval, p.absolute_tick, p.program),
        (94080, 5738, 99818, 52)
    );
    assert_eq!(p.logical_source_ordinal, 64);
    assert_eq!(p.raw_source_object_ordinal, 64);
    assert_eq!(p.declared_source_objects, 65);
    assert_eq!(q.routing().midi_channel, 15);
    assert_eq!(p.track_suffix_range, 0x7854..0x785b);
    assert_eq!(p.representation_bytes.len(), 31);
    let early = phoenix::program_only166::qualify_track(&b, 0, 0).unwrap();
    assert_eq!(early.provenance().logical_source_ordinal, 1);
    let cc = phoenix::controller166::qualify_track(&b, 0, 0).unwrap();
    let w = walk_bounded_mixed_events(
        &b,
        MixedEventBounds {
            event_range: p.event_bounds.clone(),
        },
        Default::default(),
    )
    .unwrap();
    let mut events = Vec::new();
    let mut ordinal = 0;
    for item in &w.items {
        match item {
            MixedEventItem::Event(e) => match &e.event {
                MixedEventKind::Note(n) => {
                    events.push(DecodedExportEvent::from_note(e.position, ordinal, n))
                }
                MixedEventKind::Controller(_) => events.push(cc[0].decoded_export_event()),
                _ => panic!(),
            },
            MixedEventItem::PatchToNote(_) => events.extend(early.decoded_export_events()),
            MixedEventItem::Patch(_) => events.push(q.decoded_export_event()),
        }
        ordinal += item.logical_event_count() as u64;
    }
    assert_eq!(events.len(), 65);
    let a = adapt_track(
        &events,
        assignment(15),
        TimingPolicy::Identity480,
        PatchPolicy::StrictKnownOnly,
    )
    .unwrap();
    assert_eq!(
        (
            a.counts.notes,
            a.counts.program_changes,
            a.counts.controllers
        ),
        (62, 2, 1)
    );
    assert!(a.untranslated_metadata.is_empty());
    assert_eq!((a.counts.bank_select_msb, a.counts.bank_select_lsb), (0, 0));
    let track = smf::serialize_named_musical_track_with_note_provenance(
        b"Track 1",
        &a.scheduled_events,
        &a.note_provenance,
    )
    .unwrap();
    let output = smf::serialize_format1(480, &[track]).unwrap();
    let actual = reference::midi_tracks(&output);
    let native = reference::midi_tracks(&midi);
    let named: Vec<_> = native
        .iter()
        .filter(|t| t.iter().any(|(_, s, d)| *s == 3 && d == b"Track 1"))
        .collect();
    assert_eq!(named.len(), 1);
    let expected: Vec<_> = named[0]
        .iter()
        .filter(|(_, s, _)| *s >= 128 && *s < 240)
        .map(|(t, s, d)| {
            if *s >> 4 == 9 && d[1] == 0 {
                (*t, 0x80 | (*s & 15), vec![d[0], 64])
            } else {
                (*t, *s, d.clone())
            }
        })
        .collect();
    let actual: Vec<_> = actual[0]
        .iter()
        .filter(|(_, s, _)| *s >= 128 && *s < 240)
        .cloned()
        .collect();
    assert_eq!(actual.len(), 127);
    assert_eq!(actual, expected);
    assert_eq!(
        actual
            .iter()
            .filter(|(t, _, _)| *t >= 99818)
            .cloned()
            .collect::<Vec<_>>(),
        [
            (99818, 0xce, vec![52]),
            (101575, 0x8e, vec![65, 36]),
            (101641, 0x8e, vec![70, 66])
        ]
    );
    for pair in 1..13 {
        assert!(qualify(&b, 0, pair).is_err(), "broadened pair {pair}");
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
