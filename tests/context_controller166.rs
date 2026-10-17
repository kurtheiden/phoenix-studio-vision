use phoenix::{
    bounded_sequence::{assemble_bounded_sequence, build_bounded_sequence_manifest as manifest},
    context_controller166::qualify_at as qualify,
    midi_export::*,
    mixed_event::*,
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
fn vlq(mut v: u32) -> Vec<u8> {
    let mut out = vec![(v & 127) as u8];
    v >>= 7;
    while v != 0 {
        out.push((v as u8 & 127) | 128);
        v >>= 7;
    }
    out.reverse();
    out
}
fn bridge(leading: u32, following: u32) -> Vec<u8> {
    let mut e = vlq(leading);
    e.extend([255, 96, 7, 35, 96, 0, 114, 108, 131, 76]);
    e.extend(vlq(following));
    e.extend([255, 65, 5, 0, 1, 0, 1, 58]);
    e
}
fn assignment(channel: u8) -> Option<ChannelAssignment> {
    Some(ChannelAssignment {
        channel: MidiChannel::new(channel).unwrap(),
        provenance: ChannelAssignmentProvenance::ParsedRouting,
    })
}
fn adapt(q: &phoenix::context_controller166::OwnedContextController) -> ExportTrackResult {
    adapt_track(
        &[q.decoded_export_event()],
        assignment(q.routing().midi_channel),
        TimingPolicy::Identity480,
        PatchPolicy::StrictKnownOnly,
    )
    .unwrap()
}
fn messages(a: &ExportTrackResult) -> Vec<(u32, Vec<u8>)> {
    // Match the serializer's timeline and stable source-order scheduling.
    let mut scheduled = a.scheduled_events.clone();
    scheduled.sort_by_key(|e| (e.absolute_tick, e.stable_ordinal));
    scheduled
        .iter()
        .map(|e| (e.absolute_tick, serialize_channel_message(&e.message)))
        .collect()
}

#[test]
fn separate_proof_one_controller_two_raw_objects_and_no_context_output() {
    let b = fixture(&bridge(4, 1), 2);
    let q = qualify(&b, 0, 0, 0).unwrap();
    let p = q.provenance();
    let cp = q.controller_provenance();
    assert_eq!(
        (p.previous_tick, p.context_tick, p.controller_tick),
        (0, 4, 5)
    );
    assert_eq!(
        (
            p.logical_track_objects,
            p.retained_track_contexts,
            p.declared_source_objects
        ),
        (1, 1, 2)
    );
    assert_eq!(
        (
            p.logical_source_ordinal,
            p.raw_context_ordinal,
            p.raw_controller_ordinal
        ),
        (0, 0, 1)
    );
    assert_eq!(p.composition_bytes, b[p.composition_range.clone()]);
    assert_eq!(p.context_bytes, b[p.context_range.clone()]);
    assert_eq!(p.payload, b[p.payload_range.clone()]);
    assert_eq!(p.controller_bytes, b[p.controller_range.clone()]);
    assert_eq!(p.context_range.end, p.controller_range.start);
    assert_eq!(p.controller_range.end, p.composition_range.end);
    assert_eq!(
        (
            cp.logical_source_ordinal,
            cp.raw_source_object_ordinal,
            cp.absolute_tick,
            cp.interval
        ),
        (0, 1, 5, 1)
    );
    assert_eq!(cp.representation_range, p.controller_range);
    assert!(phoenix::controller166::qualify_track(&b, 0, 0)
        .unwrap()
        .is_empty());
    assert!(phoenix::context_note166::qualify_track(&b, 0, 0)
        .unwrap()
        .is_empty());
    let a = adapt(&q);
    assert_eq!(messages(&a), [(5, vec![0xb3, 1, 58])]);
    assert_eq!(a.counts.controllers, 1);
    assert!(a.untranslated_metadata.is_empty());
    let m = manifest(&b, 0, 480).unwrap();
    assert_eq!(m.tracks()[0].events().len(), 1);
    assert_eq!(m.tracks()[0].context_controllers(), [q]);
    let out = assemble_bounded_sequence(&m).unwrap();
    let tracks = reference::midi_tracks(&out);
    let channel: Vec<_> = tracks[1]
        .iter()
        .filter(|(_, s, _)| *s >= 128 && *s < 240)
        .cloned()
        .collect();
    assert_eq!(channel, [(5, 0xb3, vec![1, 58])]);
}
#[test]
fn independent_multibyte_timing_opaque_tail_and_raw_prefix_accounting() {
    for (lead, follow, tick) in [(4, 1, 5), (5, 1, 6), (4, 2, 6), (130, 131, 261)] {
        let b = fixture(&bridge(lead, follow), 2);
        let q = qualify(&b, 0, 0, 0).unwrap();
        assert_eq!(
            (q.provenance().context_tick, q.provenance().controller_tick),
            (lead, tick)
        );
        assert_eq!(messages(&adapt(&q)), [(tick, vec![0xb3, 1, 58])]);
    }
    let mut e = vec![10, 144, 60, 64, 64, 12];
    e.extend(bridge(130, 131));
    let b = fixture(&e, 3);
    let q = qualify(&b, 0, 0, 1).unwrap();
    assert_eq!(
        (
            q.provenance().previous_tick,
            q.provenance().context_tick,
            q.provenance().controller_tick
        ),
        (10, 140, 271)
    );
    let mut changed = b.clone();
    let range = q.provenance().payload_range.clone();
    changed[range.start + 3..range.end].copy_from_slice(&[255, 0, 128, 1]);
    let q2 = qualify(&changed, 0, 0, 1).unwrap();
    assert_eq!(q2.provenance().payload, [35, 96, 0, 255, 0, 128, 1]);
    assert_eq!(messages(&adapt(&q)), messages(&adapt(&q2)));
    let mut prefix = vec![
        0, 144, 60, 64, 64, 12, 2, 255, 96, 6, 35, 96, 0, 60, 60, 1, 3, 144, 61, 64, 64, 12,
    ];
    prefix.extend(bridge(4, 1));
    let b = fixture(&prefix, 5);
    let q = qualify(&b, 0, 0, 2).unwrap();
    let p = q.provenance();
    assert_eq!(
        (
            p.logical_source_ordinal,
            p.raw_context_ordinal,
            p.raw_controller_ordinal
        ),
        (2, 3, 4)
    );
    assert_eq!(
        (p.previous_tick, p.context_tick, p.controller_tick),
        (5, 9, 10)
    );
    assert_eq!((p.logical_track_objects, p.retained_track_contexts), (3, 2));
    assert!(manifest(&b, 0, 480).is_ok());
}
#[test]
fn same_tick_controller_order_and_duplicate_or_omitted_handoffs() {
    let mut e = bridge(0, 0);
    e.extend([0, 255, 65, 5, 0, 1, 0, 7, 99, 0, 144, 60, 64, 64, 12]);
    let b = fixture(&e, 4);
    let m = manifest(&b, 0, 480).unwrap();
    let out = assemble_bounded_sequence(&m).unwrap();
    let tracks = reference::midi_tracks(&out);
    let at: Vec<_> = tracks[1]
        .iter()
        .filter(|(t, s, _)| *t == 0 && *s >= 128 && *s < 240)
        .cloned()
        .collect();
    assert_eq!(
        at,
        [
            (0, 0xb3, vec![1, 58]),
            (0, 0xb3, vec![7, 99]),
            (0, 0x93, vec![60, 64])
        ]
    );
    let q = qualify(&b, 0, 0, 0).unwrap();
    let event = q.decoded_export_event();
    assert!(matches!(
        adapt_track(
            &[event.clone(), event],
            assignment(4),
            TimingPolicy::Identity480,
            PatchPolicy::StrictKnownOnly
        ),
        Err(MidiExportError::DuplicateSourceOrdinal { .. })
    ));
    for count in [1, 3] {
        assert!(qualify(&fixture(&bridge(4, 1), count), 0, 0, 0).is_err());
        assert!(manifest(&fixture(&bridge(4, 1), count), 0, 480).is_err());
    }
    assert!(qualify(&b, 0, 0, 1).is_err());
    assert!(qualify(&b, 0, 0, 2).is_err());
    assert!(qualify(&b, 0, 0, u64::MAX).is_err());
}
#[test]
fn unsupported_prefix_lengths_successors_chains_and_bad_sources_refuse() {
    let e = bridge(4, 1);
    for (at, value) in [
        (3, 6),
        (3, 8),
        (4, 0x57),
        (5, 0x7f),
        (6, 1),
        (12, 0xb0),
        (13, 0x60),
        (14, 4),
        (16, 2),
        (18, 128),
        (19, 128),
    ] {
        let mut bad = e.clone();
        bad[at] = value;
        assert!(qualify(&fixture(&bad, 2), 0, 0, 0).is_err(), "{at:x}");
    }
    let mut chain = e[..11].to_vec();
    chain.extend(&e);
    assert!(qualify(&fixture(&chain, 3), 0, 0, 0).is_err());
    // CCA-059 is still a separate decoded MIDI-status Controller family.
    let midi = vec![4, 255, 96, 7, 87, 127, 0, 1, 2, 3, 4, 1, 0xb0, 1, 58];
    assert!(walk_bounded_mixed_events(
        &midi,
        MixedEventBounds {
            event_range: 0..midi.len()
        },
        Default::default()
    )
    .is_ok());
    assert!(qualify(&fixture(&midi, 2), 0, 0, 0).is_err());
    for end in 1..e.len() {
        assert!(qualify(&fixture(&e[..end], 2), 0, 0, 0).is_err());
    }
    let mut tail = e.clone();
    tail.extend([0, 254]);
    assert!(qualify(&fixture(&tail, 3), 0, 0, 0).is_err());
    let b = fixture(&e, 2);
    let q = qualify(&b, 0, 0, 0).unwrap();
    for at in [
        q.routing().association_range.start,
        q.routing().type10_payload_range.start + 25,
        q.routing().type10_payload_range.start + 27,
    ] {
        let mut bad = b.clone();
        bad[at] = 255;
        assert!(qualify(&bad, 0, 0, 0).is_err());
    }
    let mut no_owner = b.clone();
    let project = parse_project_166(&b).unwrap();
    let pair = &project.sequences[0].track_pairs[0];
    no_owner.drain(pair.primary.record_range.start..pair.secondary.record_range.end);
    assert!(qualify(&no_owner, 0, 0, 0).is_err());
    assert!(qualify(&b, 1, 0, 0).is_err());
    assert!(qualify(&b, 0, 1, 0).is_err());
}
#[test]
fn existing_adapter_rejects_forged_event_route_and_muted_inclusion() {
    let b = fixture(&bridge(4, 1), 2);
    let q = qualify(&b, 0, 0, 0).unwrap();
    let event = q.decoded_export_event();
    for field in 0..3 {
        let mut forged = event.clone();
        match field {
            0 => forged.absolute_position += 1,
            1 => forged.source_ordinal += 1,
            _ => forged.source_range.as_mut().unwrap().start -= 1,
        };
        assert!(adapt_track(
            &[forged],
            assignment(4),
            TimingPolicy::Identity480,
            PatchPolicy::StrictKnownOnly
        )
        .is_err());
    }
    assert!(adapt_track(
        std::slice::from_ref(&event),
        assignment(5),
        TimingPolicy::Identity480,
        PatchPolicy::StrictKnownOnly
    )
    .is_err());
    let mut override_route = assignment(4).unwrap();
    override_route.provenance = ChannelAssignmentProvenance::AuthenticatedOverride;
    assert!(adapt_track(
        &[event],
        Some(override_route),
        TimingPolicy::Identity480,
        PatchPolicy::StrictKnownOnly
    )
    .is_err());
    let mut changed = b.clone();
    changed[q.routing().type10_payload_range.start + 27] = 5;
    let q2 = qualify(&changed, 0, 0, 0).unwrap();
    assert_eq!(messages(&adapt(&q2)), [(5, vec![0xb5, 1, 58])]);
    let mut muted = b.clone();
    let project = parse_project_166(&b).unwrap();
    muted[project.sequences[0].track_descriptors()[0].label_start - 39] = 0x88;
    assert!(qualify(&muted, 0, 0, 0).is_ok());
    assert_eq!(manifest(&muted, 0, 480).unwrap_err().stage, "mute");
    for (lead, follow) in [(0x0fff_ffff, 0), (0, 0x0fff_ffff)] {
        let mut e = vec![];
        for _ in 0..16 {
            e.extend([0xff, 0xff, 0xff, 0x7f, 144, 60, 64, 64, 0]);
        }
        e.extend(bridge(lead, follow));
        assert!(qualify(&fixture(&e, 18), 0, 0, 16).is_err());
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
fn authenticated_control_edit_all1568_messages_and_next_mute_guard() {
    for (source, hash, midi, midi_hash, row, channel) in [
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
        let b = read(source, hash);
        let native = read(midi, midi_hash);
        let q = qualify(&b, 0, 8, 795).unwrap();
        let p = q.provenance();
        assert_eq!(p.composition_range, 0xc490..0xc4a4);
        assert_eq!(
            (p.previous_tick, p.context_tick, p.controller_tick),
            (128704, 128708, 128709)
        );
        assert_eq!(
            (
                p.logical_source_ordinal,
                p.raw_context_ordinal,
                p.raw_controller_ordinal
            ),
            (795, 798, 799)
        );
        assert_eq!(
            (
                p.logical_track_objects,
                p.retained_track_contexts,
                p.declared_source_objects
            ),
            (1409, 4, 1413)
        );
        assert_eq!(
            (q.routing().instrument_candidate, q.routing().midi_channel),
            (row, channel)
        );
        assert_eq!(p.composition_bytes, b[p.composition_range.clone()]);
        assert_eq!(
            messages(&adapt(&q)),
            [(128709, vec![0xb0 + channel - 1, 1, 58])]
        );
        let ordinary = phoenix::controller166::qualify_track(&b, 0, 8).unwrap();
        assert_eq!(ordinary.len(), 443);
        assert!(!ordinary
            .iter()
            .any(|p| p.provenance().logical_source_ordinal == 795));
        let notes = phoenix::context_note166::qualify_track(&b, 0, 8).unwrap();
        assert_eq!(notes.len(), 3);
        let w = walk_bounded_mixed_events(
            &b,
            MixedEventBounds {
                event_range: q.routing().event_range.clone(),
            },
            Default::default(),
        )
        .unwrap();
        let mut events = vec![];
        let mut contexts = 0;
        for (logical, item) in w.items.iter().enumerate() {
            let logical = logical as u64;
            let MixedEventItem::Event(e) = item else {
                panic!()
            };
            events.push(match &e.event {
                MixedEventKind::Note(n) => DecodedExportEvent::from_note(e.position, logical, n),
                MixedEventKind::ContextMediatedNote(n) => notes
                    .iter()
                    .find(|p| p.provenance().composition_range == n.representation_range)
                    .unwrap()
                    .decoded_export_event(),
                MixedEventKind::Controller(c) => ordinary
                    .iter()
                    .find(|p| p.provenance().representation_range == c.record_range)
                    .unwrap()
                    .decoded_export_event(),
                MixedEventKind::ContextMediatedController(_) => {
                    contexts += 1;
                    qualify(&b, 0, 8, logical).unwrap().decoded_export_event()
                }
                MixedEventKind::PitchBend { entry, .. } => {
                    DecodedExportEvent::from_pitch_bend(e.position, logical, entry)
                }
                _ => panic!(),
            });
        }
        assert_eq!(contexts, 1);
        assert_eq!(events.len(), 1409);
        let a = adapt_track(
            &events,
            assignment(channel),
            TimingPolicy::Identity480,
            PatchPolicy::StrictKnownOnly,
        )
        .unwrap();
        assert_eq!(
            (a.counts.notes, a.counts.controllers, a.counts.pitch_bend),
            (159, 444, 806)
        );
        assert!(a.untranslated_metadata.is_empty());
        let tracks = reference::midi_tracks(&native);
        let expected: Vec<_> = tracks[9]
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
        assert_eq!(expected.len(), 1568);
        assert_eq!(messages(&a), expected);
        let project = parse_project_166(&b).unwrap();
        let mut total = 0;
        for pair in 0..13 {
            let range = project.sequences[0]
                .validated_track_event_bounds(pair)
                .unwrap()
                .event_range;
            let walk = walk_bounded_mixed_events(
                &b,
                MixedEventBounds { event_range: range },
                Default::default(),
            )
            .unwrap();
            total+=walk.items.iter().filter(|i|matches!(i,MixedEventItem::Event(e) if matches!(e.event,MixedEventKind::ContextMediatedController(_)))).count();
        }
        assert_eq!(total, 1);
        let e = manifest(&b, 0, 480).unwrap_err();
        if row == 1 {
            assert_eq!(e.stage, "mute");
            assert_eq!(e.detail, "On");
            let mutes = phoenix::saved_mute::collect_saved_mute_evidence(&b).unwrap();
            assert!(mutes
                .iter()
                .filter(|m| m.sequence_ordinal == 0 && m.descriptor_ordinal < 14)
                .all(|m| m.state == phoenix::saved_mute::SavedMuteState::Off));
            assert_eq!(
                mutes
                    .iter()
                    .find(|m| m.sequence_ordinal == 0 && m.descriptor_ordinal == 14)
                    .unwrap()
                    .state,
                phoenix::saved_mute::SavedMuteState::On
            );
        } else {
            assert_eq!(e.stage, "conductor coverage");
        }
        let mut app = phoenix::app_service::AppService::new();
        let response = app
            .inspect_project(phoenix::app_contract::InspectProjectRequest {
                contract_version: phoenix::app_contract::CONTRACT_VERSION,
                source_path: format!(
                    "/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/{source}"
                ),
                diagnostics_level: phoenix::app_contract::DiagnosticsLevel::Summary,
            })
            .unwrap();
        let seq = response
            .sequences
            .iter()
            .find(|s| s.display_name == "xForm")
            .unwrap();
        assert!(seq.export_capability.is_none());
        assert!(seq.bounded_export_capability.is_none());
    }
}
