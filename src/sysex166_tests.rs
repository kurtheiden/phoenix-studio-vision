use super::*;
use crate::{midi_export as midi, smf};

fn record(b: &mut Vec<u8>, tag: u8, p: &[u8]) {
    b.push(tag);
    b.extend((p.len() as u32).to_be_bytes());
    b.extend(p);
}
fn fixture(events: &[u8], raw: u32) -> Vec<u8> {
    let mut b = vec![0; 8];
    let start = b.len();
    let mut p = vec![0; 3 * 166 + 172];
    p[0] = 3;
    record(&mut b, 1, &p);
    let label = start + 208 + 2 * 166 + 15;
    b[label..label + 5].copy_from_slice(b"Other");
    let mut name = vec![0; 11];
    name.extend([4, b'N', b'e', b'w', b'!']);
    record(&mut b, 7, &name);
    for event in [
        vec![0, 255, 88, 4, 4, 2, 8, 8],
        vec![0, 255, 81, 3, 7, 53, 120],
    ] {
        let mut p = vec![0; 14];
        p.extend(event);
        p.extend([255, 0, 0, 0, 255, 47, 0]);
        record(&mut b, 2, &p);
        record(&mut b, 41, &[]);
    }
    let mut p = vec![0; 14];
    p[2..6].copy_from_slice(&raw.to_be_bytes());
    p.extend(events);
    p.extend([255, 0, 0, 0, 255, 47, 0]);
    record(&mut b, 2, &p);
    record(&mut b, 41, &[]);
    record(&mut b, 0, &[255; 8]);
    b
}
fn raw_walk(b: &[u8]) -> MixedEventWalk<'_> {
    let p = parse_project_166(b).unwrap();
    let bounds = p.sequences[0]
        .validated_track_event_bounds(0)
        .unwrap()
        .event_range;
    walk_bounded_mixed_events(
        b,
        MixedEventBounds {
            event_range: bounds,
        },
        Default::default(),
    )
    .unwrap()
}
fn sx(delta: &[u8], first: u8) -> Vec<u8> {
    [delta, &[0xf0, 7, first, 0x41, 0x32, 0, 12, 93, 0xf7]].concat()
}
fn adapt(
    events: &[midi::DecodedExportEvent],
) -> Result<midi::ExportTrackResult, midi::MidiExportError> {
    midi::adapt_track(
        events,
        Some(midi::ChannelAssignment {
            channel: smf::MidiChannel::new(4).unwrap(),
            provenance: midi::ChannelAssignmentProvenance::Synthetic,
        }),
        midi::TimingPolicy::Identity480,
        midi::PatchPolicy::StrictKnownOnly,
    )
}

#[test]
fn owned166_framing_successors_intervals_source_provenance_and_exact_midi() {
    let events = [
        sx(&[0x81, 0], 0),
        sx(&[3], 0),
        vec![5, 0x90, 60, 100, 64, 10],
    ]
    .concat();
    let b = fixture(&events, 3);
    let w = raw_walk(&b);
    let before = w.clone();
    let proofs = qualify_track(&b, 0, 0, &w).unwrap();
    assert_eq!(w, before);
    assert_eq!(proofs.len(), 2);
    for (i, tick, interval, successor, delta) in [(0, 128, 128, 0xf0, 3), (1, 131, 3, 0x90, 5)] {
        let proof = &proofs[i];
        let p = proof.provenance();
        assert_eq!(p.absolute_tick, tick);
        assert_eq!(p.interval, interval);
        assert_eq!(p.successor_status, successor);
        assert_eq!(p.successor_interval, delta);
        assert_eq!(p.counted_payload, [0, 0x41, 0x32, 0, 12, 93, 0xf7]);
        assert_eq!(p.representation_bytes, b[p.representation_range.clone()]);
        assert_eq!(p.timing_bytes, b[p.timing_range.clone()]);
        assert_eq!(p.framing_offset, p.counted_payload_range.start);
        assert_eq!(proof.midi_payload(), &b[p.midi_payload_range.clone()]);
        assert_eq!(p.logical_source_ordinal, i as u64);
        assert_eq!(p.raw_source_object_ordinal, i as u64);
        assert_eq!(p.declared_source_objects, 3);
    }
    let handoff: Vec<_> = proofs
        .iter()
        .cloned()
        .map(midi::DecodedExportEvent::from_source_framed_sysex07)
        .collect();
    let a = adapt(&handoff).unwrap();
    assert_eq!(
        smf::serialize_channel_message(&a.scheduled_events[0].message),
        [0xf0, 6, 0x41, 0x32, 0, 12, 93, 0xf7]
    );
    assert_eq!(a.scheduled_events[0].absolute_tick, 128);
    assert_eq!(a.scheduled_events[1].absolute_tick, 131);
    let assignment = Some(midi::ChannelAssignment {
        channel: smf::MidiChannel::new(1).unwrap(),
        provenance: midi::ChannelAssignmentProvenance::Synthetic,
    });
    assert!(midi::adapt_owned120_track(
        &handoff,
        assignment,
        midi::TimingPolicy::Identity480,
        midi::PatchPolicy::StrictKnownOnly
    )
    .is_err());
    for mutate in [0, 1, 2] {
        let mut unbound = handoff.clone();
        match mutate {
            0 => unbound[0].absolute_position += 1,
            1 => unbound[0].source_ordinal += 9,
            _ => unbound[0].source_range = Some(0..1),
        };
        assert!(adapt(&unbound).is_err());
    }
    // Ordinary MIDI payload construction/adaptation remains byte preserving.
    assert!(smf::BoundedSysExPayload::new(proofs[0].midi_payload()).is_err());
    let MixedEventItem::Event(e) = &w.items[0] else {
        panic!()
    };
    let MixedEventKind::SysEx(s) = &e.event else {
        panic!()
    };
    assert_eq!(s.framing, None);
    let old = adapt(&[midi::DecodedExportEvent::from_sysex(e.position, 0, s)]).unwrap();
    assert_eq!(
        smf::serialize_channel_message(&old.scheduled_events[0].message),
        [0xf0, 7, 0, 0x41, 0x32, 0, 12, 93, 0xf7]
    );
}

#[test]
fn owned166_nonzero_terminal_ambiguous_and_late_invalid_frames_refuse_transactionally() {
    for events in [
        [sx(&[0], 1), vec![0, 0x90, 60, 100, 64, 1]].concat(),
        sx(&[0], 0),
        [sx(&[0], 0), sx(&[0], 1), vec![0, 0x90, 60, 100, 64, 1]].concat(),
    ] {
        let b = fixture(&events, 3);
        let w = raw_walk(&b);
        assert!(qualify_track(&b, 0, 0, &w).is_err());
    }
    let b = fixture(&[sx(&[0], 0), vec![0, 0x90, 60, 100, 64, 1]].concat(), 2);
    let w = raw_walk(&b);
    assert!(qualify_track(&b, 1, 0, &w).is_err());
    assert!(qualify_track(&b, 0, 1, &w).is_err());
    let mut wrong = w.clone();
    wrong.event_range.start += 1;
    assert_eq!(
        qualify_track(&b, 0, 0, &wrong),
        Err(SysEx166Refusal::Bounds)
    );
    let mut wrong = w.clone();
    let MixedEventItem::Event(e) = &mut wrong.items[0] else {
        panic!()
    };
    e.position += 1;
    assert_eq!(qualify_track(&b, 0, 0, &wrong), Err(SysEx166Refusal::Walk));
    let mut wrong = b.clone();
    wrong[13] = 4;
    assert!(qualify_track(&wrong, 0, 0, &w).is_err());
}

#[test]
fn owned166_malformed_truncated_and_unsupported_successors_never_reach_handoff() {
    let events = [sx(&[1], 0), vec![2, 0x90, 60, 100, 64, 1]].concat();
    for (offset, value) in [(2, 6), (2, 8), (2, 0x87), (4, 128), (6, 0xf7), (9, 0)] {
        let mut bad = events.clone();
        bad[offset] = value;
        let b = fixture(&bad, 2);
        let p = parse_project_166(&b).unwrap();
        let bounds = p.sequences[0]
            .validated_track_event_bounds(0)
            .unwrap()
            .event_range;
        assert!(walk_bounded_mixed_events(
            &b,
            MixedEventBounds {
                event_range: bounds
            },
            Default::default()
        )
        .is_err());
    }
    for cut in 1..10 {
        let b = fixture(&events[..cut], 1);
        let p = parse_project_166(&b).unwrap();
        let bounds = p.sequences[0]
            .validated_track_event_bounds(0)
            .unwrap()
            .event_range;
        assert!(walk_bounded_mixed_events(
            &b,
            MixedEventBounds {
                event_range: bounds
            },
            Default::default()
        )
        .is_err());
    }
    for status in [0xb0, 0xe0, 0xc0, 0xff, 0xf7, 60] {
        let mut bad = events.clone();
        bad[11] = status;
        let b = fixture(&bad, 2);
        let p = parse_project_166(&b).unwrap();
        let bounds = p.sequences[0]
            .validated_track_event_bounds(0)
            .unwrap()
            .event_range;
        assert!(walk_bounded_mixed_events(
            &b,
            MixedEventBounds {
                event_range: bounds
            },
            Default::default()
        )
        .is_err());
    }
}

#[test]
#[ignore = "requires Experiment007 and existing authentic happyone/xForm MIDI"]
fn authentic_owned166_thirty_sysex_and_899_note_starts_match_native_without_other_changes() {
    use sha2::{Digest, Sha256};
    let b = std::fs::read(std::env::var("PHOENIX_EXPERIMENT007_SOURCE").unwrap()).unwrap();
    assert_eq!(b.len(), 211468);
    assert_eq!(
        format!("{:x}", Sha256::digest(&b)),
        "e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132"
    );
    let project = parse_project_166(&b).unwrap();
    let routes = crate::bounded_routing::decode_bounded_routing(&b).unwrap();
    let mut all_notes = 0;
    let mut all_sysex = 0;
    let mut merged_happy = vec![];
    let mut merged_xform = vec![];
    for (
        seq,
        pair,
        expected_bounds,
        notes,
        sysex,
        declared,
        variable,
        reference_hash,
        native_track,
    ) in [
        (
            7,
            5,
            0x21236..0x21714,
            138,
            26,
            166,
            "PHOENIX_HAPPYONE_MULTITRACK",
            "0061667c7bcfd7ba37478266af5fb4154905651f2ca845dc16c00cf76bcd13c7",
            5,
        ),
        (
            7,
            6,
            0x217df..0x22375,
            519,
            2,
            521,
            "PHOENIX_HAPPYONE_MULTITRACK",
            "0061667c7bcfd7ba37478266af5fb4154905651f2ca845dc16c00cf76bcd13c7",
            6,
        ),
        (
            0,
            1,
            0x79b6..0x7fb2,
            242,
            2,
            247,
            "PHOENIX_XFORM_MULTITRACK",
            "872272ce43c389474fdb408416ad2406fa9a63de5e1d8bb94e0f6bbced7a1241",
            2,
        ),
    ] {
        let range = project.sequences[seq]
            .validated_track_event_bounds(pair)
            .unwrap()
            .event_range;
        assert_eq!(range, expected_bounds);
        let walk = walk_bounded_mixed_events(
            &b,
            MixedEventBounds { event_range: range },
            Default::default(),
        )
        .unwrap();
        let before_walk = walk.clone();
        let proofs = qualify_track(&b, seq, pair, &walk).unwrap();
        assert_eq!(before_walk, walk);
        assert_eq!(proofs.len(), sysex);
        assert_eq!(
            walk.logical_event_count() as u32 + u32::from(pair == 5 || seq == 0),
            declared
        );
        let mut before = vec![];
        let mut ordinal = 0;
        for item in &walk.items {
            match item {
                MixedEventItem::Event(e) => before.push(match &e.event {
                    MixedEventKind::SysEx(s) => {
                        midi::DecodedExportEvent::from_sysex(e.position, ordinal, s)
                    }
                    MixedEventKind::Note(n) => {
                        midi::DecodedExportEvent::from_note(e.position, ordinal, n)
                    }
                    MixedEventKind::ContextMediatedNote(n) => {
                        midi::DecodedExportEvent::from_note_body(e.position, ordinal, &n.note)
                    }
                    MixedEventKind::DoubleContextMediatedNote(n) => {
                        midi::DecodedExportEvent::from_note_body(e.position, ordinal, &n.note)
                    }
                    MixedEventKind::Controller(c) => {
                        midi::DecodedExportEvent::from_controller(e.position, ordinal, c)
                    }
                    MixedEventKind::MidiController(c) => {
                        midi::DecodedExportEvent::from_midi_controller(e.position, ordinal, c)
                    }
                    MixedEventKind::ChannelPressure { entry, .. } => {
                        midi::DecodedExportEvent::from_channel_pressure(e.position, ordinal, entry)
                    }
                    MixedEventKind::PitchBend { entry, .. } => {
                        midi::DecodedExportEvent::from_pitch_bend(e.position, ordinal, entry)
                    }
                    _ => panic!("unexpected authentic family: {:?}", e.event),
                }),
                MixedEventItem::Patch(p) => before.push(midi::DecodedExportEvent {
                    absolute_position: p.position,
                    source_ordinal: ordinal,
                    source_range: Some(p.patch.representation_range.clone()),
                    kind: midi::DecodedExportEventKind::Patch {
                        program: p.patch.program_change.value,
                        translation: midi::PatchTranslation::ProgramOnlyConfirmed,
                    },
                }),
                MixedEventItem::PatchToNote(p) => {
                    before.push(midi::DecodedExportEvent::from_patch(
                        p.patch_position,
                        ordinal,
                        &p.patch,
                        midi::PatchTranslation::ProgramOnlyConfirmed,
                    ));
                    before.push(midi::DecodedExportEvent::from_note_body(
                        p.first_note_position,
                        ordinal + 1,
                        &p.first_note,
                    ));
                }
            }
            ordinal += item.logical_event_count() as u64;
        }
        let mut after = before.clone();
        for proof in proofs {
            let p = proof.provenance();
            assert_eq!(p.declared_source_objects, declared);
            assert_eq!(p.representation_bytes, b[p.representation_range.clone()]);
            assert_eq!(
                p.counted_payload.as_slice(),
                &b[p.counted_payload_range.clone()]
            );
            assert_eq!(
                proof.midi_payload().as_slice(),
                &b[p.midi_payload_range.clone()]
            );
            assert_eq!(b[p.framing_offset], 0);
            assert_eq!(p.timing_bytes, b[p.timing_range.clone()]);
            assert_eq!(
                p.raw_source_object_ordinal,
                p.logical_source_ordinal + u64::from(pair == 5 && seq == 7)
            );
            let i = after
                .iter()
                .position(|e| e.source_ordinal == p.logical_source_ordinal)
                .unwrap();
            assert_eq!(
                after[i].source_range.as_ref(),
                Some(&p.representation_range)
            );
            after[i] = midi::DecodedExportEvent::from_source_framed_sysex07(proof);
        }
        for (old, new) in before.iter().zip(&after) {
            assert_eq!(
                (old.absolute_position, old.source_ordinal, &old.source_range),
                (new.absolute_position, new.source_ordinal, &new.source_range)
            );
            if !matches!(old.kind, midi::DecodedExportEventKind::SysEx { .. }) {
                assert_eq!(old, new);
            }
        }
        let route = routes
            .iter()
            .find(|r| r.structural_ordinal == seq && r.descriptor_ordinal == pair + 2)
            .unwrap()
            .result
            .as_ref()
            .unwrap();
        let assignment = Some(midi::ChannelAssignment {
            channel: smf::MidiChannel::new(route.midi_channel).unwrap(),
            provenance: midi::ChannelAssignmentProvenance::ParsedRouting,
        });
        let old = midi::adapt_track(
            &before,
            assignment,
            midi::TimingPolicy::Identity480,
            midi::PatchPolicy::StrictKnownOnly,
        )
        .unwrap();
        let new = midi::adapt_track(
            &after,
            assignment,
            midi::TimingPolicy::Identity480,
            midi::PatchPolicy::StrictKnownOnly,
        )
        .unwrap();
        assert_eq!(old.counts, new.counts);
        assert_eq!(old.note_provenance, new.note_provenance);
        let non_sysex = |es: &[smf::ScheduledEvent]| {
            es.iter()
                .filter(|e| !matches!(e.message, smf::ChannelMessage::SysEx { .. }))
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(
            non_sysex(&old.scheduled_events),
            non_sysex(&new.scheduled_events)
        );
        let reference = std::fs::read(std::env::var(variable).unwrap()).unwrap();
        assert_eq!(format!("{:x}", Sha256::digest(&reference)), reference_hash);
        let native = crate::reference_smf::midi_tracks(&reference);
        let rendered: Vec<_> = new
            .scheduled_events
            .iter()
            .filter_map(|e| {
                let data = smf::serialize_channel_message(&e.message);
                if data[0] == 0xf0 {
                    Some((e.absolute_tick, data[0], data[2..].to_vec()))
                } else if data[0] >> 4 == 9 && data[2] > 0 {
                    Some((e.absolute_tick, data[0], data[1..].to_vec()))
                } else {
                    None
                }
            })
            .collect();
        let expected: Vec<_> = native[native_track]
            .iter()
            .filter(|e| e.1 == 0xf0 || e.1 >> 4 == 9 && e.2[1] > 0)
            .cloned()
            .collect();
        assert_eq!(
            rendered, expected,
            "joint SysEx/Note-start order, tick, channel and data"
        );
        let sys: Vec<_> = rendered.iter().filter(|e| e.1 == 0xf0).cloned().collect();
        if seq == 7 {
            merged_happy.extend(sys)
        } else {
            merged_xform.extend(sys)
        };
        assert_eq!(new.counts.notes as usize, notes);
        all_notes += notes;
        all_sysex += sysex;
    }
    assert_eq!((all_sysex, all_notes), (30, 899));
    for (variable, hash, mut actual) in [
        (
            "PHOENIX_HAPPYONE_MERGED",
            "d19342947c74e5957133a95006d9f4249f2a49d6f637c19c0502cbde83209a29",
            merged_happy,
        ),
        (
            "PHOENIX_XFORM_MERGED",
            "5e737f1536e0eabe90c7fecfb8d2d5b795fb4ac7ecee5219b1bedeb7486e4f8f",
            merged_xform,
        ),
    ] {
        let reference = std::fs::read(std::env::var(variable).unwrap()).unwrap();
        assert_eq!(format!("{:x}", Sha256::digest(&reference)), hash);
        let native = crate::reference_smf::midi_tracks(&reference);
        let expected: Vec<_> = native
            .iter()
            .flatten()
            .filter(|e| e.1 == 0xf0)
            .cloned()
            .collect();
        actual.sort_by_key(|e| e.0);
        assert_eq!(actual, expected);
    }
    // Fidelity correction supplies no new conductor or unsupported-context authority.
    assert_eq!(
        crate::bounded_sequence::build_bounded_sequence_manifest(&b, 7, 480)
            .unwrap_err()
            .stage,
        "conductor prelude"
    );
    assert!(walk_bounded_mixed_events(
        &b,
        MixedEventBounds {
            event_range: 0xc490..0xc4a5
        },
        Default::default()
    )
    .is_err());
}

#[test]
#[ignore = "requires original COMIC BOOK and published CCA-064 MIDI/sidecar"]
fn authentic_cca064_sweeper_remains_byte_identical_in_memory() {
    use sha2::{Digest, Sha256};
    let b = std::fs::read(std::env::var("PHOENIX_COMIC_ORIGINAL_SOURCE").unwrap()).unwrap();
    assert_eq!(
        format!("{:x}", Sha256::digest(&b)),
        "22a9e2589098b0d475f6e288eeebc9303bed644aa8ce09f9971a0060c9160486"
    );
    let baseline = std::path::PathBuf::from(std::env::var("PHOENIX_SWEEPER_CCA064_MIDI").unwrap());
    let previous = std::fs::read(&baseline).unwrap();
    assert_eq!(
        format!("{:x}", Sha256::digest(&previous)),
        "267e85a77e44daf1c7a69f595b89d8a02c6517adf4ae706ae739e4eadc06f075"
    );
    let (report, midi) = crate::experimental120::build(&b, 5).unwrap();
    assert_eq!(midi, previous);
    let sidecar = baseline.with_extension("limitations.json");
    let previous: serde_json::Value =
        serde_json::from_slice(&std::fs::read(sidecar).unwrap()).unwrap();
    let now = serde_json::to_value(report).unwrap();
    assert_eq!(now["partial120"], previous["partial120"]);
}
