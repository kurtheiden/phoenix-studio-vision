use phoenix::{
    bounded_sequence::{assemble_bounded_sequence, build_bounded_sequence_manifest as manifest},
    midi_export::*,
    mixed_event::*,
    sequence_container::parse_project_166,
    smf::{serialize_channel_message, MidiChannel},
    standalone_bank166::qualify_track as qualify,
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
fn source(family: u8, prefix: bool) -> Vec<u8> {
    let mut e = patch(family, 37, true);
    let core_end = e.len() - 15;
    e[core_end] = 0;
    if prefix {
        e.splice(0..0, [9, 255, 65, 5, 0, 1, 0, 7, 90]);
    }
    fixture(&e, if prefix { 4 } else { 3 })
}
#[test]
fn all_existing_bank_families_keep_one_object_and_same_tick_successor_order() {
    for family in 0..4 {
        for prefix in [false, true] {
            let b = source(family, prefix);
            let q = qualify(&b, 0, 0).unwrap();
            let p = q.provenance();
            let ordinal = u64::from(prefix);
            let previous = if prefix { 9 } else { 0 };
            let tick = previous + 37;
            assert_eq!(
                (p.previous_tick, p.interval, p.absolute_tick),
                (previous, 37, tick)
            );
            assert_eq!(
                (p.logical_source_ordinal, p.raw_source_object_ordinal),
                (ordinal, ordinal)
            );
            assert_eq!(p.declared_source_objects, if prefix { 4 } else { 3 });
            assert_eq!(p.logical_track_objects, p.declared_source_objects as usize);
            assert_eq!(p.retained_track_contexts, 0);
            assert_eq!(p.patch_bytes, b[p.patch_range.clone()]);
            assert_eq!(p.timing_bytes, b[p.timing_range.clone()]);
            assert_eq!(p.pre_name_bytes, b[p.pre_name_range.clone()]);
            assert_eq!(p.post_name_bytes, b[p.post_name_range.clone()]);
            assert_eq!(p.patch_range.end, p.program_offset + 1);
            let m = manifest(&b, 0, 480).unwrap();
            let t = &m.tracks()[0];
            assert_eq!(t.standalone_banks(), std::slice::from_ref(&q));
            assert!(t.bank_patches().is_empty());
            assert!(t.terminal_programs().is_empty());
            assert_eq!(t.events().len(), p.declared_source_objects as usize);
            assert!(phoenix::bank_patch166_handoff::qualify_track(&b, 0, 0).is_err());
            let a = adapt_track(
                &[q.decoded_export_event()],
                assignment(4),
                TimingPolicy::Identity480,
                PatchPolicy::StrictKnownOnly,
            )
            .unwrap();
            assert_eq!(a.counts.program_changes, 1);
            assert_eq!(a.counts.bank_select_msb, 1);
            assert_eq!(a.counts.bank_select_lsb, u64::from(family >= 2));
            let bank = if family == 3 { 81 } else { 80 };
            let mut expected = vec![(tick, vec![0xb3, 0, bank])];
            if family >= 2 {
                expected.push((tick, vec![0xb3, 32, if family == 3 { 1 } else { 0 }]));
            }
            expected.push((tick, vec![0xc3, 42]));
            assert_eq!(messages(&a), expected);
            let midi = assemble_bounded_sequence(&m).unwrap();
            let tr = reference::midi_tracks(&midi);
            let at: Vec<_> = tr[1]
                .iter()
                .filter(|(t, s, _)| *t == tick && *s >= 128 && *s < 240)
                .map(|(t, s, d)| (*t, [vec![*s], d.clone()].concat()))
                .collect();
            expected.extend([(tick, vec![0xb3, 7, 90]), (tick, vec![0x93, 60, 100])]);
            assert_eq!(at, expected);
            // Copies returned to a caller cannot mutate the owned proof's event.
            let mut copy = q.decoded_export_event();
            copy.absolute_position += 1;
            assert_ne!(copy, q.decoded_export_event());
        }
    }
}
#[test]
fn wrapper_program_lengths_counts_and_incomplete_sources_refuse() {
    for family in 0..4 {
        let b = source(family, false);
        let q = qualify(&b, 0, 0).unwrap();
        let p = q.provenance();
        let mut fields: Vec<_> = p
            .pre_name_range
            .clone()
            .chain(p.post_name_range.clone())
            .collect();
        fields.extend([
            p.marker_range.start,
            p.marker_range.start + 1,
            p.payload_length_offset,
            p.name_length_offset,
            p.program_offset,
        ]);
        for at in fields {
            let mut bad = b.clone();
            bad[at] = if p.post_name_range.contains(&at) && b[at].is_ascii_digit() {
                b'x'
            } else {
                b[at] ^ 1
            };
            assert!(
                qualify(&bad, 0, 0).is_err(),
                "family {family} offset {at:x}"
            );
            assert!(manifest(&bad, 0, 480).is_err());
        }
        let count = parse_project_166(&b).unwrap().sequences[0].track_pairs[0]
            .primary
            .payload
            .range
            .start
            + 2;
        for n in [2u32, 4] {
            let mut bad = b.clone();
            bad[count..count + 4].copy_from_slice(&n.to_be_bytes());
            assert!(qualify(&bad, 0, 0).is_err());
            assert!(manifest(&bad, 0, 480).is_err());
        }
        for end in p.patch_range.clone() {
            assert!(qualify(&b[..end], 0, 0).is_err());
        }
        let mut e = patch(family, 37, true);
        e.extend([0, 254]);
        assert!(qualify(&fixture(&e, 4), 0, 0).is_err());
        let mut unowned = b.clone();
        let project = parse_project_166(&b).unwrap();
        let pair = &project.sequences[0].track_pairs[0];
        unowned.drain(pair.primary.record_range.start..pair.secondary.record_range.end);
        assert!(qualify(&unowned, 0, 0).is_err());
        for at in [
            q.routing().association_range.start,
            q.routing().type10_payload_range.start + 25,
            q.routing().type10_payload_range.start + 27,
        ] {
            let mut bad = b.clone();
            bad[at] = 255;
            assert!(qualify(&bad, 0, 0).is_err());
        }
    }
}
#[test]
fn terminal_unbanked_placement_contexts_and_duplicate_handoffs_stay_blocked() {
    let b = source(2, false);
    let q = qualify(&b, 0, 0).unwrap();
    let len = q.provenance().patch_range.len();
    let e = patch(2, 37, true);
    let core = e[..len].to_vec();
    assert!(qualify(&fixture(&core, 1), 0, 0).is_err());
    assert!(manifest(&fixture(&core, 1), 0, 480).is_err());
    let mut late = NOTE.to_vec();
    late.extend(&e);
    assert!(qualify(&fixture(&late, 4), 0, 0).is_err());
    let mut repeated = e.clone();
    repeated.extend(&e);
    assert!(manifest(&fixture(&repeated, 6), 0, 480).is_err());
    for at in [0, len] {
        let mut context = e.clone();
        context.splice(at..at, [0, 255, 96, 7, 87, 127, 0, 1, 1, 1, 1]);
        assert!(qualify(&fixture(&context, 4), 0, 0).is_err());
    }
    // Numeric Program-only and I/00 terminal Program-only cores gain no bank authority.
    for (pre, post, program) in [
        ([0, 0, 1, 0, 1], vec![1, b'2', 4, 255, 255, 255], 1),
        ([0, 0, 63, 8, 31], vec![2, b'3', b'2', 4, 255, 255, 255], 63),
        (
            [0, 0, 42, 0, 42],
            vec![3, b'I', b'4', b'2', 4, 255, 255, 255],
            42,
        ),
    ] {
        let mut e = vec![0, 255, 124, (5 + 1 + 5 + post.len() + 1) as u8];
        e.extend(pre);
        e.push(5);
        e.extend(b"Fresh");
        e.extend(post);
        e.push(program);
        e.extend([0, 0xb0, 7, 90]);
        let b = fixture(&e, 2);
        assert!(qualify(&b, 0, 0).is_err());
        assert!(manifest(&b, 0, 480).is_err());
    }
    let event = q.decoded_export_event();
    let err = adapt_track(
        &[event.clone(), event],
        assignment(4),
        TimingPolicy::Identity480,
        PatchPolicy::StrictKnownOnly,
    )
    .unwrap_err();
    assert!(matches!(
        err,
        MidiExportError::DuplicateSourceOrdinal { .. }
    ));
    assert!(manifest(&fixture(&core, 2), 0, 480).is_err()); // Omitted source/successor.
}
#[test]
fn timing_names_routing_and_saved_mute_are_source_derived() {
    let b = source(2, true);
    let q = qualify(&b, 0, 0).unwrap();
    let mut changed = b.clone();
    let p = q.provenance();
    changed[p.timing_range.start] = 38;
    changed[p.name_range.clone()].fill(b'Z');
    changed[p.post_name_range.start + 1..p.post_name_range.start + 3].copy_from_slice(b"12");
    changed[q.routing().type10_payload_range.start + 27] = 5;
    let other = qualify(&changed, 0, 0).unwrap();
    assert_eq!(
        (
            other.provenance().previous_tick,
            other.provenance().absolute_tick,
            other.routing().midi_channel
        ),
        (9, 47, 6)
    );
    let a = adapt_track(
        &[other.decoded_export_event()],
        assignment(6),
        TimingPolicy::Identity480,
        PatchPolicy::StrictKnownOnly,
    )
    .unwrap();
    assert_eq!(
        messages(&a),
        [
            (47, vec![0xb5, 0, 80]),
            (47, vec![0xb5, 32, 0]),
            (47, vec![0xc5, 42])
        ]
    );
    let mut muted = b.clone();
    let project = parse_project_166(&b).unwrap();
    muted[project.sequences[0].track_descriptors()[0].label_start - 39] = 0x88;
    assert!(qualify(&muted, 0, 0).is_ok());
    assert_eq!(manifest(&muted, 0, 480).unwrap_err().stage, "mute");
    let mut overflow = vec![];
    for _ in 0..17 {
        overflow.extend([0xff, 0xff, 0xff, 0x7f, 255, 65, 5, 0, 1, 0, 7, 90]);
    }
    overflow.extend(patch(2, 37, true));
    assert!(qualify(&fixture(&overflow, 20), 0, 0).is_err());
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
fn authenticated_four_cores_and_three_native_sequences_with_muted_exclusion() {
    let b = read(
        "CCA073 CONTROL",
        "e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132",
    );
    let n = read(
        "073 CTRL xForm.mid",
        "872272ce43c389474fdb408416ad2406fa9a63de5e1d8bb94e0f6bbced7a1241",
    );
    let native = reference::midi_tracks(&n);
    let mutes = phoenix::saved_mute::collect_saved_mute_evidence(&b).unwrap();
    for (pair, range, tick, channel, program, lsb, declared, logical, contexts, row) in [
        (5, 0xa1c7..0xa1e2, 100, 4, 50, true, 137, 135, 2, 7),
        (10, 0xd51b..0xd538, 0, 11, 63, false, 74, 72, 2, 30),
        (11, 0xd82c..0xd849, 480, 7, 24, true, 127, 123, 4, 10),
        (12, 0xdcc7..0xdce5, 240, 14, 63, false, 76, 73, 3, 33),
    ] {
        let q = qualify(&b, 0, pair).unwrap();
        let p = q.provenance();
        assert_eq!(p.patch_range, range);
        assert_eq!(p.patch_bytes, b[range]);
        assert_eq!(
            (p.previous_tick, p.interval, p.absolute_tick),
            (0, tick, tick)
        );
        assert_eq!(
            (p.logical_source_ordinal, p.raw_source_object_ordinal),
            (0, 0)
        );
        assert_eq!(
            (
                p.declared_source_objects,
                p.logical_track_objects,
                p.retained_track_contexts
            ),
            (declared, logical, contexts)
        );
        assert_eq!(
            (q.routing().instrument_candidate, q.routing().midi_channel),
            (row, channel)
        );
        assert_eq!(p.patch_range.end, p.program_offset + 1);
        let a = adapt_track(
            &[q.decoded_export_event()],
            assignment(channel),
            TimingPolicy::Identity480,
            PatchPolicy::StrictKnownOnly,
        )
        .unwrap();
        let mut expected = vec![(tick, vec![0xb0 + channel - 1, 0, 80])];
        if lsb {
            expected.push((tick, vec![0xb0 + channel - 1, 32, 0]));
        }
        expected.push((tick, vec![0xc0 + channel - 1, program]));
        assert_eq!(messages(&a), expected);
        assert_eq!(
            (
                a.counts.program_changes,
                a.counts.bank_select_msb,
                a.counts.bank_select_lsb
            ),
            (1, 1, u64::from(lsb))
        );
        let tracks: Vec<_> = native
            .iter()
            .filter(|t| {
                t.iter()
                    .any(|(_, s, d)| *s == 3 && *d == q.routing().label_bytes)
            })
            .collect();
        let mute = &mutes
            .iter()
            .find(|m| m.sequence_ordinal == 0 && m.descriptor_ordinal == pair + 2)
            .unwrap()
            .state;
        if pair == 12 {
            assert_eq!(*mute, phoenix::saved_mute::SavedMuteState::On);
            assert!(tracks.is_empty());
            continue;
        }
        assert_eq!(*mute, phoenix::saved_mute::SavedMuteState::Off);
        assert_eq!(tracks.len(), 1);
        let actual: Vec<_> = tracks[0]
            .iter()
            .filter(|(t, s, d)| {
                *t == tick && (*s >> 4 == 12 || (*s >> 4 == 11 && matches!(d[0], 0 | 32)))
            })
            .map(|(t, s, d)| (*t, [vec![*s], d.clone()].concat()))
            .collect();
        assert_eq!(actual, expected);
        let cc = phoenix::controller166::qualify_track(&b, 0, pair)
            .unwrap()
            .into_iter()
            .find(|c| c.provenance().logical_source_ordinal == 1)
            .unwrap();
        assert_eq!(
            cc.provenance().representation_range.start,
            p.patch_range.end
        );
        let a = adapt_track(
            &[q.decoded_export_event(), cc.decoded_export_event()],
            assignment(channel),
            TimingPolicy::Identity480,
            PatchPolicy::StrictKnownOnly,
        )
        .unwrap();
        let following = cc.provenance();
        expected.push((
            following.absolute_tick,
            vec![0xb0 + channel - 1, 7, following.value],
        ));
        assert_eq!(messages(&a), expected);
        let first: Vec<_> = tracks[0]
            .iter()
            .filter(|(_, s, _)| *s >= 128 && *s < 240)
            .take(expected.len())
            .map(|(t, s, d)| (*t, [vec![*s], d.clone()].concat()))
            .collect();
        assert_eq!(first, expected);
    }
    for pair in [0, 1, 2, 3, 4, 6, 7, 8, 9] {
        assert!(qualify(&b, 0, pair).is_err());
    }
    let e = manifest(&b, 0, 480).unwrap_err();
    assert_eq!(e.stage, "mute");
    assert_eq!(e.detail, "On");
    let route = phoenix::bounded_routing::decode_bounded_routing(&b)
        .unwrap()
        .into_iter()
        .find(|r| r.structural_ordinal == 0 && r.descriptor_ordinal == 10)
        .unwrap()
        .result
        .unwrap();
    let w = walk_bounded_mixed_events(
        &b,
        MixedEventBounds {
            event_range: route.event_range,
        },
        Default::default(),
    )
    .unwrap();
    let MixedEventItem::Event(e) = &w.items[795] else {
        panic!()
    };
    let MixedEventKind::ContextMediatedController(c) = &e.event else {
        panic!()
    };
    assert_eq!(c.representation_range, 0xc490..0xc4a4);
    assert_eq!(e.position, 128709);
    let mut app = phoenix::app_service::AppService::new();
    let r = app
        .inspect_project(phoenix::app_contract::InspectProjectRequest {
            contract_version: phoenix::app_contract::CONTRACT_VERSION,
            source_path: "/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/CCA073 CONTROL"
                .into(),
            diagnostics_level: phoenix::app_contract::DiagnosticsLevel::Summary,
        })
        .unwrap();
    let s = r
        .sequences
        .iter()
        .find(|s| s.display_name == "xForm")
        .unwrap();
    assert!(s.export_capability.is_none());
    assert!(s.bounded_export_capability.is_none());
    assert_eq!(
        s.readiness,
        phoenix::app_contract::Readiness::PartiallySupported
    );
}

#[test]
fn opening_bank_and_later_terminal_program_keep_independent_current_item_proofs() {
    let mut e = patch(2, 37, true);
    e.extend([0, 255, 124, 20, 0, 0, 52, 0, 52, 5]);
    e.extend(b"Fresh");
    e.extend([3, b'I', b'5', b'3', 4, 255, 255, 255, 52]);
    let b = fixture(&e, 4);
    let m = manifest(&b, 0, 480).unwrap();
    let t = &m.tracks()[0];
    assert_eq!(t.standalone_banks().len(), 1);
    assert_eq!(t.terminal_programs().len(), 1);
    assert_eq!(t.events().len(), 4);
    let terminal = &t.terminal_programs()[0];
    assert_eq!(terminal.provenance().logical_source_ordinal, 3);
    assert_eq!(
        terminal.provenance().representation_range.end,
        t.routing().event_range.end
    );
    assert_eq!(t.standalone_banks()[0], qualify(&b, 0, 0).unwrap());
    assert_eq!(
        terminal,
        &phoenix::terminal_program166::qualify_track(&b, 0, 0).unwrap()
    );
}
