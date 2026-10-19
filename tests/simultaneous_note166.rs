use phoenix::{midi_export::*, mixed_event::*, simultaneous_note166::qualify_track, smf::*};
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

fn adapt_source(b: &[u8], pair: usize) -> ExportTrackResult {
    let c = phoenix::bounded_patch_translation::classify_bounded_cc0_patches(b)
        .unwrap()
        .into_iter()
        .find(|r| r.structural_ordinal == 0 && r.descriptor_ordinal == pair + 2)
        .unwrap()
        .result
        .unwrap();
    let w = walk_bounded_mixed_events(
        b,
        MixedEventBounds {
            event_range: c.routing.event_range.clone(),
        },
        Default::default(),
    )
    .unwrap();
    let mut events = vec![];
    let mut ordinal = 0;
    for item in &w.items {
        match item {
            MixedEventItem::Event(e) => match &e.event {
                MixedEventKind::Note(n) => {
                    events.push(DecodedExportEvent::from_note(e.position, ordinal, n))
                }
                MixedEventKind::ContextMediatedNote(n) => events.push(
                    phoenix::context_note166::qualify_track(b, 0, pair)
                        .unwrap()
                        .into_iter()
                        .find(|p| p.provenance().composition_range == n.representation_range)
                        .unwrap()
                        .decoded_export_event(),
                ),
                _ => panic!("unexpected event"),
            },
            MixedEventItem::PatchToNote(_) => events.extend(
                phoenix::bank_patch166_handoff::qualify_track(b, 0, pair)
                    .unwrap()
                    .decoded_export_events(),
            ),
            _ => panic!("unexpected item"),
        }
        ordinal += item.logical_event_count() as u64;
    }
    assert_eq!(events.len(), w.logical_event_count());
    adapt_track(
        &events,
        Some(ChannelAssignment {
            channel: MidiChannel::new(c.routing.midi_channel).unwrap(),
            provenance: ChannelAssignmentProvenance::ParsedRouting,
        }),
        TimingPolicy::Identity480,
        PatchPolicy::StrictKnownOnly,
    )
    .unwrap()
}
fn serialize(
    a: &ExportTrackResult,
    p: &[phoenix::simultaneous_note166::OwnedSimultaneousNotes],
) -> Result<SerializedTrack, SmfSerializeError> {
    serialize_named_musical_track_with_owned_simultaneous_notes(
        b"Track 7",
        &a.scheduled_events,
        &a.note_provenance,
        p,
    )
}
fn two_notes() -> Vec<u8> {
    vec![10, 144, 42, 127, 123, 22, 0, 144, 42, 127, 127, 41]
}
#[test]
fn independent_source_pair_preserves_values_and_requires_owned_proof() {
    let b = fixture(&two_notes(), 2);
    let a = adapt_source(&b, 0);
    let p = qualify_track(&b, 0, 0).unwrap();
    assert_eq!(p.len(), 1);
    assert!(serialize(&a, &[]).is_err());
    assert!(serialize(&a, &p).is_ok());
    let manifest = phoenix::bounded_sequence::build_bounded_sequence_manifest(&b, 0, 480).unwrap();
    assert!(phoenix::bounded_sequence::assemble_bounded_sequence(&manifest).is_ok());
    assert!(serialize_named_musical_track_with_note_provenance(
        b"old API",
        &a.scheduled_events,
        &a.note_provenance
    )
    .is_err());
    // Source-derived routing, pitch, positions and values are not corpus constants.
    let mut bytes = two_notes();
    bytes[2] = 55;
    bytes[8] = 55;
    bytes[0] = 20;
    bytes[5] = 12;
    bytes[11] = 31;
    let b = fixture(&bytes, 2);
    let a = adapt_source(&b, 0);
    assert!(serialize(&a, &qualify_track(&b, 0, 0).unwrap()).is_ok());
}
fn varying_notes() -> Vec<u8> {
    let mut notes = two_notes();
    notes[3] = 123;
    notes[4] = 110;
    notes[11] = 39;
    notes
}
#[test]
fn distinct_attack_velocities_are_source_bound_and_preserved() {
    let b = fixture(&varying_notes(), 2);
    let a = adapt_source(&b, 0);
    let proofs = qualify_track(&b, 0, 0).unwrap();
    assert_eq!(proofs.len(), 1);
    assert!(serialize(&a, &[]).is_err());
    assert!(serialize_named_musical_track_with_note_provenance(
        b"old API",
        &a.scheduled_events,
        &a.note_provenance
    )
    .is_err());
    let t = serialize(&a, &proofs).unwrap();
    let mut file = b"MThd\0\0\0\x06\0\x01\0\x01\x01\xe0".to_vec();
    file.extend(t.as_bytes());
    let readback = reference::midi_tracks(&file);
    let messages: Vec<_> = readback[0]
        .iter()
        .filter(|(_, s, _)| *s >= 128 && *s < 240)
        .cloned()
        .collect();
    assert_eq!(
        messages,
        vec![
            (10, 0x93, vec![42, 123]),
            (10, 0x93, vec![42, 127]),
            (32, 0x83, vec![42, 110]),
            (49, 0x83, vec![42, 127])
        ]
    );
    // Exact values, not merely same-channel/key claims, bind the proof.
    for index in [0, 2] {
        let mut altered = a.clone();
        let ChannelMessage::NoteOn {
            attack_velocity, ..
        } = &mut altered.scheduled_events[index].message
        else {
            panic!()
        };
        *attack_velocity = MidiDataByte::new(126).unwrap();
        assert!(serialize(&altered, &proofs).is_err());
    }
    let mut swapped = a.clone();
    let first = swapped.scheduled_events[0].message.clone();
    swapped.scheduled_events[0].message = swapped.scheduled_events[2].message.clone();
    swapped.scheduled_events[2].message = first;
    assert!(serialize(&swapped, &proofs).is_err());
    let mut other = varying_notes();
    other[3] = 122;
    assert!(serialize(&a, &qualify_track(&fixture(&other, 2), 0, 0).unwrap()).is_err());
    let manifest = phoenix::bounded_sequence::build_bounded_sequence_manifest(&b, 0, 480).unwrap();
    assert!(phoenix::bounded_sequence::assemble_bounded_sequence(&manifest).is_ok());
}
#[test]
fn forged_incomplete_duplicate_reordered_and_rebound_evidence_refuses() {
    for notes in [two_notes(), varying_notes()] {
        let b = fixture(&notes, 2);
        let original = adapt_source(&b, 0);
        let p = qualify_track(&b, 0, 0).unwrap();
        for mutation in 0..16 {
            let mut a = original.clone();
            match mutation {
                0 => a.note_provenance.clear(),
                1 => {
                    a.note_provenance.pop();
                }
                2 => a.note_provenance.reverse(),
                3 => a.note_provenance.push(a.note_provenance[0].clone()),
                4 => a.note_provenance[1].source_range.start -= 1,
                5 => a.note_provenance[0].start_ordinal += 2,
                6 => a.scheduled_events[0].absolute_tick += 1,
                7 => a.scheduled_events[1].absolute_tick += 1,
                8 => a.scheduled_events[3].stable_ordinal += 2,
                9 => {
                    a.scheduled_events.remove(3);
                }
                10 => a.scheduled_events.push(a.scheduled_events[1].clone()),
                11 => {
                    a.scheduled_events[0].message = ChannelMessage::NoteOn {
                        channel: MidiChannel::new(5).unwrap(),
                        key: MidiDataByte::new(42).unwrap(),
                        attack_velocity: MidiDataByte::new(127).unwrap(),
                    }
                }
                12 => {
                    a.scheduled_events[3].message = ChannelMessage::NoteOff {
                        channel: MidiChannel::new(4).unwrap(),
                        key: MidiDataByte::new(42).unwrap(),
                        release_velocity: MidiDataByte::new(126).unwrap(),
                    }
                }
                13 => a.scheduled_events[2].stable_ordinal = 0,
                14 => a.note_provenance[1].source_range.end += 1,
                15 => {
                    let mut forged = a.note_provenance[0].clone();
                    forged.start_ordinal = 999;
                    a.note_provenance.push(forged);
                }
                _ => unreachable!(),
            }
            assert!(serialize(&a, &p).is_err(), "mutation {mutation}");
        }
        assert!(serialize(&original, &[p[0].clone(), p[0].clone()]).is_err());
        let mut changed = notes.clone();
        changed[11] += 1;
        let other = fixture(&changed, 2);
        assert!(serialize(&original, &qualify_track(&other, 0, 0).unwrap()).is_err());
        // Malformed source accounting cannot mint authority.
        assert!(qualify_track(&fixture(&notes, 3), 0, 0).is_err());
    }
}
#[test]
fn multiple_proofs_must_be_unique_consumed_and_in_source_order() {
    let mut notes = two_notes();
    notes.extend(two_notes());
    let b = fixture(&notes, 4);
    let a = adapt_source(&b, 0);
    let mut p = qualify_track(&b, 0, 0).unwrap();
    assert_eq!(p.len(), 2);
    assert!(serialize(&a, &p).is_ok());
    p.reverse();
    assert!(serialize(&a, &p).is_err());
    p.reverse();
    let mut incomplete = a.clone();
    incomplete.scheduled_events.retain(|e| e.stable_ordinal < 4);
    incomplete.note_provenance.retain(|p| p.start_ordinal < 4);
    assert!(serialize(&incomplete, &p).is_err());
    assert!(serialize(&a, &p[..1]).is_err());
}
#[test]
fn unsupported_groups_and_reversed_endings_keep_refusing() {
    for mutation in 0..4 {
        let mut notes = two_notes();
        match mutation {
            0 => notes.extend([0, 144, 42, 127, 127, 50]),
            1 => notes[11] = 22,
            2 => {
                notes[5] = 50;
            }
            3 => notes[9] = 0,
            _ => unreachable!(),
        }
        let b = fixture(&notes, if mutation == 0 { 3 } else { 2 });
        let a = adapt_source(&b, 0);
        assert!(
            serialize(&a, &qualify_track(&b, 0, 0).unwrap()).is_err(),
            "mutation {mutation}"
        );
    }
}
#[test]
fn authenticated_complete_track7_native_reconciliation_and_production_preflight() {
    let root = "/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/";
    let b = std::fs::read(format!("{root}CCA073 CONTROL")).unwrap();
    assert_eq!(
        format!("{:x}", Sha256::digest(&b)),
        "e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132"
    );
    let n = std::fs::read(format!("{root}073 CTRL xForm.mid")).unwrap();
    assert_eq!(
        format!("{:x}", Sha256::digest(&n)),
        "872272ce43c389474fdb408416ad2406fa9a63de5e1d8bb94e0f6bbced7a1241"
    );
    let a = adapt_source(&b, 6);
    assert_eq!(a.counts.notes, 291);
    assert_eq!(a.scheduled_events.len(), 585);
    let old = serialize(&a, &[]).unwrap_err();
    assert_eq!(
        old,
        SmfSerializeError::AmbiguousSourceOrder {
            tick: 102720,
            stable_ordinal: 162
        }
    );
    let p = qualify_track(&b, 0, 6).unwrap();
    assert_eq!(p.len(), 2);
    // The original proof alone still stops precisely at the velocity-varying pair.
    assert_eq!(
        serialize(&a, &p[..1]).unwrap_err(),
        SmfSerializeError::AmbiguousSourceOrder {
            tick: 118080,
            stable_ordinal: 466
        }
    );
    let second_ranges: Vec<_> = a
        .note_provenance
        .iter()
        .filter(|p| (464..468).contains(&p.start_ordinal))
        .cloned()
        .collect();
    assert_eq!(
        second_ranges,
        vec![
            NoteSourceProvenance {
                start_ordinal: 464,
                source_range: 44049..44054
            },
            NoteSourceProvenance {
                start_ordinal: 466,
                source_range: 44054..44059
            }
        ]
    );
    let second_messages: Vec<_> = a
        .scheduled_events
        .iter()
        .filter(|e| (464..468).contains(&e.stable_ordinal))
        .map(|e| {
            (
                e.absolute_tick,
                e.stable_ordinal,
                serialize_channel_message(&e.message),
            )
        })
        .collect();
    assert_eq!(
        second_messages,
        vec![
            (118080, 464, vec![0x99, 42, 123]),
            (118102, 465, vec![0x89, 42, 110]),
            (118080, 466, vec![0x99, 42, 127]),
            (118119, 467, vec![0x89, 42, 127])
        ]
    );
    let complete = serialize(&a, &p).unwrap();
    // Retain the original CCA-097 pair's serialized-payload regression.
    let mut pair = a.clone();
    pair.scheduled_events
        .retain(|e| (160..164).contains(&e.stable_ordinal));
    pair.note_provenance
        .retain(|p| (160..164).contains(&p.start_ordinal));
    let t = serialize(&pair, &p[..1]).unwrap();
    let mut scheduled = a.scheduled_events.clone();
    scheduled.sort_by_key(|e| (e.absolute_tick, e.stable_ordinal));
    let actual: Vec<_> = scheduled
        .iter()
        .map(|e| (e.absolute_tick, serialize_channel_message(&e.message)))
        .collect();
    let native = reference::midi_tracks(&n);
    let expected: Vec<_> = native[7]
        .iter()
        .filter(|(_, s, _)| *s >= 128 && *s < 240)
        .map(|(t, s, d)| {
            (
                *t,
                if *s >> 4 == 9 && d[1] == 0 {
                    vec![0x80 | (*s & 15), d[0], 64]
                } else {
                    [vec![*s], d.clone()].concat()
                },
            )
        })
        .collect();
    assert_eq!(actual, expected);
    // Independently read the exact production serializer payload, not only the schedule.
    let mut file = b"MThd\0\0\0\x06\0\x01\0\x01\x01\xe0".to_vec();
    file.extend(t.as_bytes());
    let readback = reference::midi_tracks(&file);
    let rendered: Vec<_> = readback[0]
        .iter()
        .filter(|(_, s, _)| *s >= 128 && *s < 240)
        .map(|(t, s, d)| (*t, [vec![*s], d.clone()].concat()))
        .collect();
    assert_eq!(
        rendered,
        vec![
            (102720, vec![0x99, 42, 127]),
            (102720, vec![0x99, 42, 127]),
            (102742, vec![0x89, 42, 123]),
            (102761, vec![0x89, 42, 127])
        ]
    );
    for event in &rendered {
        assert!(expected.contains(event));
    }
    let mut file = b"MThd\0\0\0\x06\0\x01\0\x01\x01\xe0".to_vec();
    file.extend(complete.as_bytes());
    let readback = reference::midi_tracks(&file);
    let rendered: Vec<_> = readback[0]
        .iter()
        .filter(|(_, s, _)| *s >= 128 && *s < 240)
        .map(|(t, s, d)| (*t, [vec![*s], d.clone()].concat()))
        .collect();
    assert_eq!(rendered, expected);
    // This task does not bypass the sequence's saved-mute or application gates.
    let e = phoenix::bounded_sequence::build_bounded_sequence_manifest(&b, 0, 480).unwrap_err();
    assert_eq!(e.stage, "mute");
    assert_eq!(e.detail, "On");
}
