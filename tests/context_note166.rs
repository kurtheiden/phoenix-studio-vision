use phoenix::{
    bounded_sequence::build_bounded_sequence_manifest as manifest,
    context_note166::{qualify_track as qualify, ContextNoteRefusal},
    midi_export::*,
    sequence_container::parse_project_166,
    smf::{serialize_channel_message, MidiChannel},
};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
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
fn entry(prefix: [u8; 3], len: u8, leading: u8, final_delta: u8) -> Vec<u8> {
    let mut e = vec![leading, 255, 96, len];
    let mut payload = prefix.to_vec();
    payload.resize(len as usize, 1);
    e.extend(payload);
    e.extend([final_delta, 144, 60, 100, 64, 12]);
    e
}
fn adapt(q: &[phoenix::context_note166::OwnedContextNote]) -> ExportTrackResult {
    adapt_track(
        &q.iter()
            .map(|p| p.decoded_export_event())
            .collect::<Vec<_>>(),
        Some(ChannelAssignment {
            channel: MidiChannel::new(q[0].routing().midi_channel).unwrap(),
            provenance: ChannelAssignmentProvenance::ParsedRouting,
        }),
        TimingPolicy::Identity480,
        PatchPolicy::StrictKnownOnly,
    )
    .unwrap()
}
#[test]
fn five_forms_reuse_ticks_and_retain_every_component_with_no_context_midi() {
    for (prefix, len) in [
        ([87, 127, 0], 6),
        ([87, 127, 0], 7),
        ([87, 127, 0], 8),
        ([35, 96, 0], 6),
        ([35, 96, 0], 7),
    ] {
        let mut e = vec![9, 144, 62, 101, 65, 12];
        e.extend(entry(prefix, len, 4, 5));
        let b = fixture(&e, 3);
        let q = qualify(&b, 0, 0).unwrap();
        assert_eq!(q.len(), 1);
        let p = q[0].provenance();
        assert_eq!(
            (
                p.absolute_tick,
                p.ending_tick,
                p.leading_interval,
                p.final_interval
            ),
            (18, 30, 4, 5)
        );
        assert_eq!(
            (
                p.logical_source_ordinal,
                p.raw_context_ordinal,
                p.raw_note_ordinal
            ),
            (1, 1, 2)
        );
        assert_eq!(
            (
                p.declared_source_objects,
                p.logical_track_objects,
                p.retained_track_contexts
            ),
            (3, 2, 1)
        );
        for (r, v) in [
            (p.composition_range.clone(), &p.composition_bytes),
            (p.context_range.clone(), &p.context_bytes),
            (p.payload_range.clone(), &p.payload),
            (p.leading_timing_range.clone(), &p.leading_timing_bytes),
            (p.final_timing_range.clone(), &p.final_timing_bytes),
            (p.note_range.clone(), &p.note_bytes),
        ] {
            assert_eq!(&b[r], v);
        }
        let a = adapt(&q);
        assert_eq!(
            (
                a.counts.notes,
                a.counts.generated_note_offs,
                a.counts.controllers
            ),
            (1, 1, 0)
        );
        assert_eq!(
            a.scheduled_events
                .iter()
                .map(|e| (e.absolute_tick, serialize_channel_message(&e.message)))
                .collect::<Vec<_>>(),
            [(18, vec![0x93, 60, 100]), (30, vec![0x83, 60, 64])]
        );
        let m = manifest(&b, 0, 480).unwrap();
        assert_eq!(m.tracks()[0].context_notes(), q);
        assert_eq!(m.tracks()[0].events().len(), 2);
    }
}
#[test]
fn duplicate_equal_tick_notes_keep_logical_ordinals_and_source_order() {
    let mut e = entry([87, 127, 0], 7, 0, 0);
    e.extend(entry([87, 127, 0], 7, 0, 0));
    let b = fixture(&e, 4);
    let q = qualify(&b, 0, 0).unwrap();
    assert_eq!(q.len(), 2);
    assert_eq!(
        q.iter()
            .map(|q| (
                q.provenance().logical_source_ordinal,
                q.provenance().raw_context_ordinal,
                q.provenance().raw_note_ordinal
            ))
            .collect::<Vec<_>>(),
        [(0, 0, 1), (1, 2, 3)]
    );
    let events = q
        .iter()
        .map(|q| q.decoded_export_event())
        .collect::<Vec<_>>();
    let adapted = adapt_track(
        &events,
        Some(ChannelAssignment {
            channel: MidiChannel::new(4).unwrap(),
            provenance: ChannelAssignmentProvenance::ParsedRouting,
        }),
        TimingPolicy::Identity480,
        PatchPolicy::StrictKnownOnly,
    )
    .unwrap();
    assert_eq!(
        adapted
            .scheduled_events
            .iter()
            .map(|e| (e.absolute_tick, serialize_channel_message(&e.message)))
            .collect::<Vec<_>>(),
        [
            (0, vec![0x93, 60, 100]),
            (12, vec![0x83, 60, 64]),
            (0, vec![0x93, 60, 100]),
            (12, vec![0x83, 60, 64]),
        ]
    );
    // Preserve the existing conservative equal-tick export-ordering gate.
    assert_eq!(manifest(&b, 0, 480).unwrap_err().stage, "source ordering");
}
#[test]
fn form_bounds_data_ownership_association_and_accounting_refuse() {
    for len in [0, 1, 3, 5, 9, 10] {
        let e = entry([87, 127, 0], len, 0, 0);
        assert!(qualify(&fixture(&e, 2), 0, 0).is_err());
    }
    for (prefix, len) in [
        ([35, 96, 0], 8),
        ([0, 0, 0], 7),
        ([87, 0, 0], 8),
        ([87, 126, 0], 7),
    ] {
        assert!(qualify(&fixture(&entry(prefix, len, 0, 0), 2), 0, 0).is_err());
    }
    let e = entry([87, 127, 0], 7, 0, 0);
    for len in 0..e.len() {
        assert!(qualify(&fixture(&e[..len], 2), 0, 0).is_err());
    }
    for at in [e.len() - 5, e.len() - 4, e.len() - 3] {
        let mut bad = e.clone();
        bad[at] = 128;
        assert!(qualify(&fixture(&bad, 2), 0, 0).is_err());
    }
    let mut bad = e.clone();
    bad.pop();
    bad.extend([128; 5]);
    assert!(qualify(&fixture(&bad, 2), 0, 0).is_err());
    for count in [1, 3] {
        assert!(qualify(&fixture(&e, count), 0, 0).is_err());
    }
    let b = fixture(&e, 2);
    let s = parse_project_166(&b).unwrap().sequences.remove(0);
    let mut bad = b.clone();
    bad[s.track_descriptors()[0].label_start - 31] = 99;
    assert!(qualify(&bad, 0, 0).is_err());
    let mut unowned = b.clone();
    unowned.drain(
        s.track_pairs[0].primary.record_range.start..s.track_pairs[0].secondary.record_range.end,
    );
    assert!(qualify(&unowned, 0, 0).is_err());
    let mut muted = b.clone();
    muted[s.track_descriptors()[0].label_start - 39] = 0x88;
    assert!(qualify(&muted, 0, 0).is_ok());
    assert_eq!(manifest(&muted, 0, 480).unwrap_err().stage, "mute");
    let mut wrong_channel = b.clone();
    wrong_channel[49 + 5 + 27] = 16;
    assert!(qualify(&wrong_channel, 0, 0).is_err());
    // This zero-prefix context is routable only after a MIDI Controller prefix;
    // it must still receive no new translation authority.
    let mut z = vec![0, 0xb0, 7, 64];
    z.extend(entry([87, 0, 0], 8, 0, 0));
    assert!(qualify(&fixture(&z, 3), 0, 0).is_err());
}
#[test]
fn ending_overflow_and_excluded_compositions_remain_refused() {
    let mut e = Vec::new();
    for _ in 0..16 {
        e.extend([255, 255, 255, 127, 144, 62, 100, 64, 0]);
    }
    let mut n = entry([87, 127, 0], 7, 0, 0);
    *n.last_mut().unwrap() = 32;
    e.extend(n);
    assert_eq!(
        qualify(&fixture(&e, 18), 0, 0).unwrap_err(),
        ContextNoteRefusal::Ending
    );
    let mut double = vec![0, 255, 96, 6, 87, 127, 0, 1, 1, 1, 0];
    double.extend(entry([87, 127, 0], 7, 0, 0).iter().skip(1));
    let b = fixture(&double, 3);
    assert!(qualify(&b, 0, 0).unwrap().is_empty());
    assert!(manifest(&b, 0, 480).is_err());
    let controller = vec![
        4, 255, 96, 7, 35, 96, 0, 114, 108, 131, 76, 1, 255, 65, 5, 0, 1, 0, 1, 58,
    ];
    let b = fixture(&controller, 2);
    assert!(qualify(&b, 0, 0).unwrap().is_empty());
    assert_eq!(
        manifest(&b, 0, 480).unwrap().tracks()[0]
            .context_controllers()
            .len(),
        1
    );
    let mut alternate = entry([87, 127, 0], 7, 0, 0);
    let at = alternate.len() - 6;
    alternate[at] = 0x91;
    assert!(qualify(&fixture(&alternate, 2), 0, 0).is_err());
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
fn authenticated_all29_and_native26_with_control_edit_routes() {
    let mut control_target = Vec::new();
    let mut edit_target = Vec::new();
    for (name, hash, mid, mid_hash, edit) in [
        (
            "CCA073 CONTROL",
            "e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132",
            "073 CTRL xForm.mid",
            "872272ce43c389474fdb408416ad2406fa9a63de5e1d8bb94e0f6bbced7a1241",
            false,
        ),
        (
            "CCA073 EDIT",
            "3f12c7bb9371d76dd4d4da081ccf347101cd5d4f187fd039a9562665d87d0c87",
            "073 EDIT xForm.mid",
            "50eeb37411243e3c8083b8c86d499ee1f5b755444f0024d7b72fbeb182567145",
            true,
        ),
    ] {
        let b = read(name, hash);
        let midi = read(mid, mid_hash);
        let native = reference::midi_tracks(&midi);
        let mut total = 0;
        let mut represented = 0;
        let mut forms = BTreeMap::new();
        for pair in 0..13 {
            let q = qualify(&b, 0, pair).unwrap();
            assert_eq!(q.len(), [0, 1, 3, 1, 0, 2, 3, 1, 3, 6, 2, 4, 3][pair]);
            total += q.len();
            if q.is_empty() {
                continue;
            }
            let a = adapt(&q);
            assert_eq!(a.counts.notes, q.len() as u64);
            assert_eq!(a.counts.controllers, 0);
            for proof in &q {
                let p = proof.provenance();
                *forms
                    .entry((p.payload[..3].to_vec(), p.payload.len()))
                    .or_insert(0) += 1;
                assert_eq!(&b[p.composition_range.clone()], p.composition_bytes);
                assert_eq!(&b[p.note_range.clone()], p.note_bytes);
                assert_eq!(&b[p.payload_range.clone()], p.payload);
                assert_eq!(&b[p.leading_timing_range.clone()], p.leading_timing_bytes);
                assert_eq!(&b[p.final_timing_range.clone()], p.final_timing_bytes);
                assert_eq!(
                    p.declared_source_objects as usize,
                    p.logical_track_objects + p.retained_track_contexts
                );
                assert_eq!(p.raw_note_ordinal, p.raw_context_ordinal + 1);
                let event = proof.decoded_export_event();
                let DecodedExportEventKind::Note {
                    pitch,
                    attack_velocity,
                    release_velocity,
                    duration,
                } = event.kind
                else {
                    panic!()
                };
                assert_eq!(event.absolute_position, p.absolute_tick);
                assert_eq!(event.source_ordinal, p.logical_source_ordinal);
                assert_eq!(p.ending_tick, p.absolute_tick + duration);
                if pair < 12 {
                    let matches: Vec<_> = native
                        .iter()
                        .filter(|t| {
                            t.iter()
                                .any(|(_, s, d)| *s == 3 && d == &proof.routing().label_bytes)
                        })
                        .collect();
                    assert_eq!(matches.len(), 1);
                    let track = matches[0];
                    let channel = proof.routing().midi_channel - 1;
                    let on = track
                        .iter()
                        .position(|(t, s, d)| {
                            *t == p.absolute_tick
                                && *s == 0x90 | channel
                                && d.as_slice() == [pitch, attack_velocity]
                        })
                        .expect("native onset");
                    let off = track
                        .iter()
                        .position(|(t, s, d)| {
                            *t == p.ending_tick
                                && ((*s == 0x80 | channel
                                    && d.as_slice() == [pitch, release_velocity])
                                    || (*s == 0x90 | channel
                                        && d.as_slice() == [pitch, 0]
                                        && release_velocity == 64))
                        })
                        .expect("native ending");
                    assert!(on < off);
                    represented += 1;
                }
                if pair == 1 {
                    assert_eq!(p.composition_range, 0x7c08..0x7c1d);
                    assert_eq!(
                        (
                            p.absolute_tick,
                            p.ending_tick,
                            p.leading_interval,
                            p.final_interval
                        ),
                        (71008, 72856, 6994, 654)
                    );
                    assert_eq!(
                        (pitch, attack_velocity, release_velocity, duration),
                        (51, 5, 64, 1848)
                    );
                    assert_eq!(
                        (
                            p.logical_source_ordinal,
                            p.raw_context_ordinal,
                            p.raw_note_ordinal
                        ),
                        (89, 89, 90)
                    );
                    assert_eq!(proof.routing().midi_channel, 1);
                }
                if pair == 8 {
                    assert_eq!(proof.routing().midi_channel, if edit { 3 } else { 15 });
                    let value = (p.absolute_tick, p.ending_tick, p.composition_bytes.clone());
                    if edit {
                        edit_target.push(value)
                    } else {
                        control_target.push(value)
                    }
                }
            }
        }
        assert_eq!((total, represented), (29, 26));
        assert_eq!(forms.into_values().collect::<Vec<_>>(), [1, 2, 2, 20, 4]);
        let e = manifest(&b, 0, 480).unwrap_err();
        assert_eq!(e.stage, if edit { "conductor coverage" } else { "mute" });
        if !edit {
            assert_eq!(e.detail, "On");
        }
        let mut app = phoenix::app_service::AppService::new();
        let r = app
            .inspect_project(phoenix::app_contract::InspectProjectRequest {
                contract_version: phoenix::app_contract::CONTRACT_VERSION,
                source_path: format!(
                    "/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/{name}"
                ),
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
    assert_eq!(control_target, edit_target);
}
