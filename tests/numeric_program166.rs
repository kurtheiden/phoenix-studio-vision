#[path = "support/xform_application.rs"]
mod xform_application;
use phoenix::{
    bounded_sequence::{assemble_bounded_sequence, build_bounded_sequence_manifest as manifest},
    midi_export::*,
    mixed_event::*,
    numeric_program166::qualify_at as qualify,
    sequence_container::parse_project_166,
    smf::{serialize_channel_message, MidiChannel},
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
fn vlq(mut value: u32) -> Vec<u8> {
    let mut out = vec![(value & 127) as u8];
    value >>= 7;
    while value != 0 {
        out.push((value as u8 & 127) | 128);
        value >>= 7;
    }
    out.reverse();
    out
}
fn composition(opening: bool, delta: u32, note_delta: u32) -> Vec<u8> {
    let (pre, post, program): (&[u8], &[u8], u8) = if opening {
        (&[0, 0, 1, 0, 1], &[1, b'2', 4, 255, 255, 255], 1)
    } else {
        (&[0, 0, 63, 8, 31], &[2, b'3', b'2', 4, 255, 255, 255], 63)
    };
    let name = b"Unseen name";
    let mut e = vlq(delta);
    e.extend([255, 124, (5 + 1 + name.len() + post.len() + 1) as u8]);
    e.extend(pre);
    e.push(name.len() as u8);
    e.extend(name);
    e.extend(post);
    e.push(program);
    e.extend(vlq(note_delta));
    e.extend([144, 60, 100, 64, 33]);
    e
}
fn source() -> Vec<u8> {
    let mut e = composition(true, 40, 100);
    e.extend([10, 62, 90, 64, 12]);
    e.extend(composition(false, 480, 464));
    fixture(&e, 5)
}
fn assignment(channel: u8) -> Option<ChannelAssignment> {
    Some(ChannelAssignment {
        channel: MidiChannel::new(channel).unwrap(),
        provenance: ChannelAssignmentProvenance::ParsedRouting,
    })
}
fn messages(a: &ExportTrackResult) -> Vec<(u32, Vec<u8>)> {
    a.scheduled_events
        .iter()
        .map(|e| (e.absolute_tick, serialize_channel_message(&e.message)))
        .collect()
}
#[test]
fn exact_tuples_manifest_handoff_order_and_owned_provenance() {
    let b = source();
    let m = manifest(&b, 0, 480).unwrap();
    let ps = m.tracks()[0].numeric_programs();
    assert_eq!(ps.len(), 2);
    for (q, ordinal) in ps.iter().zip([0, 3]) {
        let p = q.provenance();
        assert_eq!(
            (
                p.logical_patch_ordinal,
                p.logical_note_ordinal,
                p.raw_patch_ordinal,
                p.raw_note_ordinal
            ),
            (ordinal, ordinal + 1, ordinal, ordinal + 1)
        );
        assert_eq!(p.declared_source_objects, 5);
        assert_eq!(p.composition_bytes, b[p.composition_range.clone()]);
        assert_eq!(p.pre_name_bytes, b[p.pre_name_range.clone()]);
        assert_eq!(p.post_name_bytes, b[p.post_name_range.clone()]);
        assert_eq!(p.patch_range.end, p.note_range.start + 1);
        assert_eq!(q.routing().midi_channel, 4);
    }
    assert_eq!(ps[1].provenance().previous_tick, 150);
    let out = assemble_bounded_sequence(&m).unwrap();
    let tr = reference::midi_tracks(&out);
    let actual: Vec<_> = tr[1]
        .iter()
        .filter(|(_, s, _)| *s >= 128 && *s < 240)
        .cloned()
        .collect();
    assert_eq!(
        actual,
        [
            (40, 0xc3, vec![1]),
            (140, 0x93, vec![60, 100]),
            (150, 0x93, vec![62, 90]),
            (162, 0x83, vec![62, 64]),
            (173, 0x83, vec![60, 64]),
            (630, 0xc3, vec![63]),
            (1094, 0x93, vec![60, 100]),
            (1127, 0x83, vec![60, 64])
        ]
    );
    assert!(phoenix::program_only166::qualify_track(&b, 0, 0).is_err());
    assert!(phoenix::terminal_program166::qualify_track(&b, 0, 0).is_err());
    assert!(phoenix::bank_patch166_handoff::qualify_track(&b, 0, 0).is_err());
    for ordinal in [1, 2, 4, 5] {
        assert!(qualify(&b, 0, 0, ordinal).is_err());
    }
    assert!(qualify(&b, 1, 0, 0).is_err());
    assert!(qualify(&b, 0, 1, 0).is_err());
}
#[test]
fn every_wrapper_field_program_and_successor_mutation_refuses() {
    let b = source();
    for ordinal in [0, 3] {
        let q = qualify(&b, 0, 0, ordinal).unwrap();
        let p = q.provenance();
        let mut fields: Vec<_> = p
            .pre_name_range
            .clone()
            .chain(p.post_name_range.clone())
            .collect();
        fields.extend([
            p.payload_length_offset,
            p.name_length_offset,
            p.program_offset,
            p.note_range.start,
        ]);
        for at in fields {
            let mut bad = b.clone();
            bad[at] ^= 1;
            assert!(
                qualify(&bad, 0, 0, ordinal).is_err(),
                "ordinal {ordinal} offset {at:x}"
            );
        }
        for at in p.note_range.start + 1..p.note_range.start + 4 {
            let mut bad = b.clone();
            bad[at] = 128;
            assert!(qualify(&bad, 0, 0, ordinal).is_err());
        }
        for value in [31, 32, 128] {
            let mut bad = b.clone();
            bad[p.program_offset] = value;
            assert!(qualify(&bad, 0, 0, ordinal).is_err());
        }
        // The wrapper's numeric label cannot select another Program.
        let mut bad = b.clone();
        bad[p.post_name_range.start + 1] = b'9';
        assert!(qualify(&bad, 0, 0, ordinal).is_err());
    }
    // Individually valid pieces do not establish a new combination or generic 08 rule.
    for (opening, pre) in [
        (true, [0, 0, 63, 8, 31]),
        (false, [0, 0, 1, 0, 1]),
        (false, [0, 0, 63, 8, 63]),
        (false, [0, 0, 31, 8, 31]),
    ] {
        let mut bad = b.clone();
        let ordinal = if opening { 0 } else { 3 };
        let p = qualify(&b, 0, 0, ordinal).unwrap().provenance().clone();
        bad[p.pre_name_range].copy_from_slice(&pre);
        assert!(qualify(&bad, 0, 0, ordinal).is_err());
    }
}
#[test]
fn contexts_truncation_placement_counts_and_routing_refuse() {
    let b = source();
    let q = qualify(&b, 0, 0, 0).unwrap();
    let p = q.provenance();
    let project = parse_project_166(&b).unwrap();
    let count = project.sequences[0].track_pairs[0]
        .primary
        .payload
        .range
        .start
        + 2;
    for n in [4u32, 6] {
        let mut bad = b.clone();
        bad[count..count + 4].copy_from_slice(&n.to_be_bytes());
        assert!(qualify(&bad, 0, 0, 0).is_err());
    }
    for at in [
        q.routing().association_range.start,
        q.routing().type10_payload_range.start + 25,
        q.routing().type10_payload_range.start + 27,
    ] {
        let mut bad = b.clone();
        bad[at] = 255;
        assert!(qualify(&bad, 0, 0, 0).is_err());
    }
    for end in p.composition_range.clone() {
        assert!(qualify(&b[..end], 0, 0, 0).is_err());
    }
    let a = composition(true, 40, 100);
    for at in [0, a.len() - 5] {
        let mut e = a.clone();
        e.splice(at..at, [0, 255, 96, 7, 87, 127, 0, 1, 1, 1, 1]);
        assert!(qualify(&fixture(&e, 3), 0, 0, 0).is_err());
    }
    for status in [0x80, 0x91, 0xa0, 0xb0] {
        let mut e = a.clone();
        let at = e.len() - 5;
        e[at] = status;
        assert!(qualify(&fixture(&e, 2), 0, 0, 0).is_err());
    }
    let mut e = a.clone();
    e.truncate(e.len() - 5);
    assert!(qualify(&fixture(&e, 1), 0, 0, 0).is_err());
    // B cannot open; A cannot appear later; Controllers cannot replace the Note prefix.
    assert!(qualify(&fixture(&composition(false, 40, 100), 2), 0, 0, 0).is_err());
    let mut e = vec![0, 144, 60, 64, 64, 12];
    e.extend(a.clone());
    assert!(qualify(&fixture(&e, 3), 0, 0, 1).is_err());
    let mut e = vec![0, 255, 65, 5, 0, 1, 0, 7, 64];
    e.extend(composition(false, 40, 100));
    assert!(qualify(&fixture(&e, 3), 0, 0, 1).is_err());
    let mut e = a;
    e.extend([0, 254]);
    assert!(qualify(&fixture(&e, 3), 0, 0, 0).is_err());
}
#[test]
fn timing_channel_and_name_changes_derive_from_source() {
    let b = source();
    let q = qualify(&b, 0, 0, 0).unwrap();
    let mut changed = b.clone();
    let p = q.provenance();
    changed[p.patch_timing_range.start] = 41;
    changed[p.note_timing_range.start] = 101;
    changed[p.name_range.clone()].fill(b'Z');
    changed[q.routing().type10_payload_range.start + 27] = 5;
    let q = qualify(&changed, 0, 0, 0).unwrap();
    assert_eq!(
        (
            q.provenance().patch_tick,
            q.provenance().note_tick,
            q.routing().midi_channel
        ),
        (41, 142, 6)
    );
    let a = adapt_track(
        &q.decoded_export_events(),
        assignment(6),
        TimingPolicy::Identity480,
        PatchPolicy::StrictKnownOnly,
    )
    .unwrap();
    assert_eq!(
        messages(&a),
        [
            (41, vec![0xc5, 1]),
            (142, vec![0x95, 60, 100]),
            (175, vec![0x85, 60, 64])
        ]
    );
    // Equal tick ordering remains Patch before Note.
    let b = fixture(&composition(true, 40, 0), 2);
    let q = qualify(&b, 0, 0, 0).unwrap();
    let a = adapt_track(
        &q.decoded_export_events(),
        assignment(4),
        TimingPolicy::Identity480,
        PatchPolicy::StrictKnownOnly,
    )
    .unwrap();
    assert_eq!(
        &messages(&a)[..2],
        &[(40, vec![0xc3, 1]), (40, vec![0x93, 60, 100])]
    );
    // Accumulated source timing exceeds u32, despite each VLQ being valid.
    let mut e = vec![];
    for _ in 0..17 {
        e.extend(vlq(0x0fff_ffff));
        e.extend([144, 60, 64, 64, 0]);
    }
    e.extend(composition(false, 480, 464));
    assert!(qualify(&fixture(&e, 19), 0, 0, 17).is_err());
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
fn authenticated_track5_all_610_messages_and_next_refusal() {
    let b = read(
        "CCA073 CONTROL",
        "e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132",
    );
    let native = read(
        "073 CTRL xForm.mid",
        "872272ce43c389474fdb408416ad2406fa9a63de5e1d8bb94e0f6bbced7a1241",
    );
    let a = qualify(&b, 0, 4, 0).unwrap();
    let z = qualify(&b, 0, 4, 299).unwrap();
    for (q, range, patch, note, previous, interval, note_interval, program) in [
        (&a, 0x99a5..0x99c8, 40, 25080, 0, 40, 25040, 1),
        (&z, 0xa03f..0xa065, 147600, 148064, 147120, 480, 464, 63),
    ] {
        let p = q.provenance();
        assert_eq!(p.composition_range, range);
        assert_eq!(
            (
                p.patch_tick,
                p.note_tick,
                p.previous_tick,
                p.patch_interval,
                p.note_interval,
                p.program
            ),
            (patch, note, previous, interval, note_interval, program)
        );
        assert_eq!(p.declared_source_objects, 306);
        assert_eq!(p.composition_bytes, b[p.composition_range.clone()]);
        assert_eq!(q.routing().instrument_candidate, 3);
        assert_eq!(q.routing().device_name, b"Juno-106");
        assert_eq!(q.routing().midi_channel, 1);
    }
    let w = walk_bounded_mixed_events(
        &b,
        MixedEventBounds {
            event_range: a.routing().event_range.clone(),
        },
        Default::default(),
    )
    .unwrap();
    let mut events = vec![];
    let mut ordinal = 0;
    for item in &w.items {
        match item {
            MixedEventItem::PatchToNote(_) => {
                let q = qualify(&b, 0, 4, ordinal).unwrap();
                events.extend(q.decoded_export_events());
            }
            MixedEventItem::Event(e) => {
                let MixedEventKind::Note(n) = &e.event else {
                    panic!()
                };
                events.push(DecodedExportEvent::from_note(e.position, ordinal, n));
            }
            _ => panic!(),
        }
        ordinal += item.logical_event_count() as u64;
    }
    assert_eq!(ordinal, 306);
    assert_eq!(events.len(), 306);
    let result = adapt_track(
        &events,
        assignment(1),
        TimingPolicy::Identity480,
        PatchPolicy::StrictKnownOnly,
    )
    .unwrap();
    assert_eq!(
        (
            result.counts.notes,
            result.counts.generated_note_offs,
            result.counts.program_changes,
            result.counts.bank_select_msb,
            result.counts.bank_select_lsb
        ),
        (304, 304, 2, 0, 0)
    );
    assert!(result.untranslated_metadata.is_empty());
    let tr = reference::midi_tracks(&native);
    let named: Vec<_> = tr
        .iter()
        .filter(|t| t.iter().any(|(_, s, d)| *s == 3 && d == b"Track 5"))
        .collect();
    assert_eq!(named.len(), 1);
    let expected: Vec<_> = named[0]
        .iter()
        .filter(|(_, s, _)| *s >= 128 && *s < 240)
        .map(|(t, s, d)| {
            if *s >> 4 == 9 && d[1] == 0 {
                (*t, vec![0x80 | (*s & 15), d[0], 64])
            } else {
                (*t, [vec![*s], d.clone()].concat())
            }
        })
        .collect();
    assert_eq!(expected.len(), 610);
    assert_eq!(messages(&result), expected);
    for pair in (0..13).filter(|p| *p != 4) {
        assert!(qualify(&b, 0, pair, 0).is_err());
    }
    let err = manifest(&b, 0, 480).unwrap_err();
    assert_eq!(err.stage, "mute");
    assert_eq!(err.detail, "On");
    let routes = phoenix::bounded_routing::decode_bounded_routing(&b).unwrap();
    let route = &routes
        .iter()
        .find(|r| r.structural_ordinal == 0 && r.descriptor_ordinal == 7)
        .unwrap()
        .result
        .as_ref()
        .unwrap();
    let w = walk_bounded_mixed_events(
        &b,
        MixedEventBounds {
            event_range: route.event_range.clone(),
        },
        Default::default(),
    )
    .unwrap();
    let MixedEventItem::Patch(p) = &w.items[0] else {
        panic!()
    };
    assert_eq!(p.patch.representation_range, 0xa1c7..0xa1e2);
    assert_eq!(p.position, 100);
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
    xform_application::assert_capability(row, true);
}

#[test]
fn missing_or_duplicate_handoffs_and_nonterminal_standalone_cores_refuse() {
    let b = source();
    let proof = qualify(&b, 0, 0, 0).unwrap();
    let events = proof.decoded_export_events();
    assert_eq!(events.len(), 2);
    assert_eq!((events[0].source_ordinal, events[1].source_ordinal), (0, 1));
    let duplicate = [events[0].clone(), events[1].clone(), events[0].clone()];
    assert!(matches!(
        adapt_track(
            &duplicate,
            assignment(4),
            TimingPolicy::Identity480,
            PatchPolicy::StrictKnownOnly
        ),
        Err(MidiExportError::DuplicateSourceOrdinal { source_ordinal: 0 })
    ));
    // Omitting a source object while retaining its declaration is rejected
    // before a manifest or any composition proof can authorize handoff.
    let opening = composition(true, 40, 100);
    assert!(manifest(&fixture(&opening, 3), 0, 480).is_err());
    let mut duplicate = opening.clone();
    duplicate.extend(&opening);
    assert!(manifest(&fixture(&duplicate, 4), 0, 480).is_err());
    // A source core with a non-Note successor stays in the standalone branch.
    let q = qualify(&fixture(&opening, 2), 0, 0, 0).unwrap();
    let core_len = q.provenance().program_offset + 1 - q.provenance().composition_range.start;
    let mut core = opening[..core_len].to_vec();
    core.extend([0, 0xb0, 7, 64]);
    let b = fixture(&core, 2);
    assert!(qualify(&b, 0, 0, 0).is_err());
    let e = manifest(&b, 0, 480).unwrap_err();
    assert_eq!(e.stage, "Patch");
    assert!(e.detail.contains("unsupported standalone Patch"));
    // A bank tail is never admitted by the new Program-only tuple qualifier.
    let mut b = source();
    let p = qualify(&b, 0, 0, 0).unwrap().provenance().clone();
    b[p.post_name_range.end - 2] = 80;
    assert!(qualify(&b, 0, 0, 0).is_err());
}
