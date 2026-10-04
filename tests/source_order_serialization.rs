//! Synthetic ordering boundaries and mandatory authenticated Sequence R evidence.
use phoenix::bounded_patch_translation::classify_bounded_cc0_patches;
use phoenix::midi_export::{
    adapt_track, ChannelAssignment, ChannelAssignmentProvenance, DecodedExportEvent,
    DecodedExportEventKind, PatchPolicy, PatchTranslation, TimingPolicy,
};
use phoenix::mixed_event::{
    walk_bounded_mixed_events, MixedEventBounds, MixedEventItem, MixedEventKind,
};
use phoenix::smf::*;
use sha2::{Digest, Sha256};

fn adapt(events: &[DecodedExportEvent]) -> Vec<ScheduledEvent> {
    adapt_track(
        events,
        Some(ChannelAssignment {
            channel: MidiChannel::new(4).unwrap(),
            provenance: ChannelAssignmentProvenance::Synthetic,
        }),
        TimingPolicy::Identity480,
        PatchPolicy::StrictKnownOnly,
    )
    .unwrap()
    .scheduled_events
}
fn note(tick: u32, ordinal: u64, key: u8, duration: u32) -> DecodedExportEvent {
    DecodedExportEvent::from_note_fields(tick, ordinal, None, key, 127, 64, duration)
}
fn patch(tick: u32, ordinal: u64) -> DecodedExportEvent {
    DecodedExportEvent {
        absolute_position: tick,
        source_ordinal: ordinal,
        source_range: None,
        kind: DecodedExportEventKind::Patch {
            program: 16,
            translation: PatchTranslation::ConfirmedBankSelectMsb { msb: 80 },
        },
    }
}
fn source(events: &[ScheduledEvent]) -> Vec<Message> {
    let result =
        serialize_musical_track_with_ordering(events, MusicalTrackOrdering::SourceOrder).unwrap();
    track(&result.as_bytes()[8..]).messages
}
fn bytes_at(messages: &[Message], tick: u32) -> Vec<Vec<u8>> {
    messages
        .iter()
        .filter(|m| m.tick == tick)
        .map(|m| m.bytes.clone())
        .collect()
}
#[test]
fn default_and_explicit_priority_retain_family_order() {
    let events = adapt(&[note(0, 0, 60, 10), patch(0, 1)]);
    let default = serialize_musical_track(&events).unwrap();
    assert_eq!(
        default,
        serialize_musical_track_with_ordering(&events, MusicalTrackOrdering::ExistingPriority)
            .unwrap()
    );
    assert_eq!(
        bytes_at(&track(&default.as_bytes()[8..]).messages, 0),
        vec![vec![0xb3, 0, 80], vec![0xc3, 16], vec![0x93, 60, 127]]
    );
    assert_eq!(
        serialize_named_musical_track(b"name", &events).unwrap(),
        serialize_named_musical_track_with_ordering(
            b"name",
            &events,
            MusicalTrackOrdering::ExistingPriority
        )
        .unwrap()
    );
}
#[test]
fn source_note_precedes_adjacent_patch_messages_and_preserves_input() {
    let events = adapt(&[note(0, 0, 60, 10), patch(0, 1)]);
    let before = events.clone();
    assert_eq!(
        bytes_at(&source(&events), 0),
        vec![vec![0x93, 60, 127], vec![0xb3, 0, 80], vec![0xc3, 16]]
    );
    assert_eq!(events, before);
    let named = serialize_named_musical_track_with_ordering(
        b"name",
        &events,
        MusicalTrackOrdering::SourceOrder,
    )
    .unwrap();
    let named = track(&named.as_bytes()[8..]);
    assert_eq!(named.name.as_deref(), Some(b"name".as_slice()));
    assert_eq!(named.messages, source(&events));
}
#[test]
fn bank_pair_expansion_retains_established_adapter_suborder() {
    let mut p = patch(0, 0);
    p.kind = DecodedExportEventKind::Patch {
        program: 16,
        translation: PatchTranslation::ConfirmedBankSelect { msb: 80, lsb: 1 },
    };
    assert_eq!(
        bytes_at(&source(&adapt(&[p])), 0),
        vec![vec![0xb3, 0, 80], vec![0xb3, 32, 1], vec![0xc3, 16]]
    );
}
#[test]
fn different_ticks_are_chronological_even_with_reversed_input() {
    let mut events = adapt(&[note(10, 0, 60, 10), note(5, 1, 61, 1)]);
    events.reverse();
    assert_eq!(
        source(&events).iter().map(|m| m.tick).collect::<Vec<_>>(),
        vec![5, 6, 10, 20]
    );
}
#[test]
fn generated_end_precedes_later_source_at_tied_tick() {
    let events = adapt(&[note(0, 0, 60, 10), patch(10, 1)]);
    assert_eq!(
        bytes_at(&source(&events), 10),
        vec![vec![0x83, 60, 64], vec![0xb3, 0, 80], vec![0xc3, 16]]
    );
}
#[test]
fn zero_duration_preserves_own_start_then_end() {
    let events = adapt(&[note(0, 0, 60, 0)]);
    assert_eq!(
        bytes_at(&source(&events), 0),
        vec![vec![0x93, 60, 127], vec![0x83, 60, 64]]
    );
    // The old policy is deliberately unchanged, including its opposite order.
    let old = serialize_musical_track(&events).unwrap();
    assert_eq!(
        bytes_at(&track(&old.as_bytes()[8..]).messages, 0),
        vec![vec![0x83, 60, 64], vec![0x93, 60, 127]]
    );
}
#[test]
fn same_pitch_retrigger_follows_prior_end_including_zero_duration() {
    let events = adapt(&[note(0, 0, 60, 10), note(10, 1, 60, 0), note(10, 2, 60, 5)]);
    assert_eq!(
        bytes_at(&source(&events), 10),
        vec![
            vec![0x83, 60, 64],
            vec![0x93, 60, 127],
            vec![0x83, 60, 64],
            vec![0x93, 60, 127]
        ]
    );
}
#[test]
fn unsafe_end_after_unrelated_retrigger_refuses_only_source_mode() {
    let events = adapt(&[note(10, 0, 60, 5), note(0, 1, 60, 10)]);
    assert_eq!(
        serialize_musical_track_with_ordering(&events, MusicalTrackOrdering::SourceOrder),
        Err(SmfSerializeError::UnsafeNoteOffOrder {
            tick: 10,
            channel: 4,
            key: 60
        })
    );
    assert!(serialize_musical_track(&events).is_ok());
}
#[test]
fn unknown_equal_ordinal_and_reversed_patch_groups_refuse() {
    let mut events = adapt(&[note(0, 0, 60, 10), patch(0, 1)]);
    events[2].stable_ordinal = 0;
    assert!(matches!(
        serialize_musical_track_with_ordering(&events, MusicalTrackOrdering::SourceOrder),
        Err(SmfSerializeError::AmbiguousSourceOrder { .. })
    ));
    let mut events = adapt(&[patch(0, 0)]);
    events.reverse();
    assert!(matches!(
        serialize_musical_track_with_ordering(&events, MusicalTrackOrdering::SourceOrder),
        Err(SmfSerializeError::AmbiguousSourceOrder { .. })
    ));
}
#[test]
fn simultaneous_same_pitch_attacks_without_distinct_end_refuse() {
    let events = adapt(&[note(0, 0, 60, 10), note(0, 1, 60, 20)]);
    assert!(matches!(
        serialize_musical_track_with_ordering(&events, MusicalTrackOrdering::SourceOrder),
        Err(SmfSerializeError::AmbiguousSourceOrder { .. })
    ));
}

#[test]
fn velocity_zero_ending_after_retrigger_is_also_refused() {
    let mut events = adapt(&[note(10, 0, 60, 5), note(0, 1, 60, 10)]);
    events[3].message = ChannelMessage::NoteOn {
        channel: MidiChannel::new(4).unwrap(),
        key: MidiDataByte::new(60).unwrap(),
        attack_velocity: MidiDataByte::new(0).unwrap(),
    };
    assert_eq!(
        serialize_musical_track_with_ordering(&events, MusicalTrackOrdering::SourceOrder),
        Err(SmfSerializeError::UnsafeNoteOffOrder {
            tick: 10,
            channel: 4,
            key: 60
        })
    );
}

#[test]
fn note_tie_checks_are_channel_and_key_local() {
    let mut events = adapt(&[note(10, 0, 60, 5), note(0, 1, 60, 10)]);
    if let ChannelMessage::NoteOff { channel, .. } = &mut events[3].message {
        *channel = MidiChannel::new(3).unwrap();
    }
    assert_eq!(source(&events).len(), 4);
    let events = adapt(&[note(10, 0, 60, 5), note(0, 1, 61, 10)]);
    assert_eq!(source(&events).len(), 4);
}

// Strict small SMF reader for direct native-vs-generated message comparison.
// Retains order and resolved status bytes; does not infer bank semantics.
#[derive(Debug, PartialEq, Eq)]
struct Message {
    tick: u32,
    bytes: Vec<u8>,
}
struct Track {
    name: Option<Vec<u8>>,
    messages: Vec<Message>,
}
fn vlq(b: &[u8], pos: &mut usize) -> u32 {
    let mut v = 0;
    for _ in 0..4 {
        let x = b[*pos];
        *pos += 1;
        v = (v << 7) | u32::from(x & 127);
        if x < 128 {
            return v;
        }
    }
    panic!("invalid VLQ")
}
fn track(b: &[u8]) -> Track {
    let (mut pos, mut tick, mut status, mut name, mut messages, mut eot) =
        (0, 0, None, None, Vec::new(), 0);
    while pos < b.len() {
        tick += vlq(b, &mut pos);
        let first = b[pos];
        let s = if first >= 128 {
            pos += 1;
            first
        } else {
            status.expect("running status")
        };
        match s {
            255 => {
                status = None;
                let kind = b[pos];
                pos += 1;
                let len = vlq(b, &mut pos) as usize;
                let data = &b[pos..pos + len];
                pos += len;
                if kind == 3 {
                    name = Some(data.to_vec());
                }
                if kind == 47 {
                    assert_eq!(len, 0);
                    assert_eq!(pos, b.len());
                    eot += 1;
                }
            }
            240 | 247 => {
                status = None;
                let len = vlq(b, &mut pos) as usize;
                pos += len;
                assert!(pos <= b.len());
            }
            _ => {
                assert!((128..240).contains(&s));
                status = Some(s);
                let len = if matches!(s >> 4, 12 | 13) { 1 } else { 2 };
                let data = &b[pos..pos + len];
                assert!(data.iter().all(|x| *x < 128));
                pos += len;
                let mut bytes = vec![s];
                bytes.extend(data);
                messages.push(Message { tick, bytes });
            }
        }
    }
    assert_eq!(eot, 1);
    Track { name, messages }
}
fn smf(b: &[u8]) -> Vec<Track> {
    assert_eq!(
        &b[..14],
        &[b'M', b'T', b'h', b'd', 0, 0, 0, 6, 0, 1, 0, 3, 1, 224]
    );
    let mut pos = 14;
    let mut out = Vec::new();
    for _ in 0..3 {
        assert_eq!(&b[pos..pos + 4], b"MTrk");
        let len = u32::from_be_bytes(b[pos + 4..pos + 8].try_into().unwrap()) as usize;
        out.push(track(&b[pos + 8..pos + 8 + len]));
        pos += 8 + len;
    }
    assert_eq!(pos, b.len());
    out
}

fn reconcile(descriptor: usize, expected_messages: usize, expected_notes: u64) {
    const PROJECT: &str = "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline";
    const MIDI: &str = "/Users/kurtheiden/Documents/Phoenix Research/Studio Vision MIDI Exports/Project 001/Seq R Test";
    let b = std::fs::read(PROJECT).expect("authenticated source required; never skipped");
    assert_eq!(b.len(), 211468);
    assert_eq!(
        format!("{:x}", Sha256::digest(&b)),
        "e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132"
    );
    let native = std::fs::read(MIDI).expect("authenticated native export required; never skipped");
    assert_eq!(native.len(), 1025);
    assert_eq!(
        format!("{:x}", Sha256::digest(&native)),
        "a97c0c3e99e95bc0e97941c8823e90f81758c94112c7dd1a64ae707250eca887"
    );
    let mut native = smf(&native);
    let rows = classify_bounded_cc0_patches(&b).unwrap();
    let t = rows
        .iter()
        .find(|r| r.structural_ordinal == 17 && r.descriptor_ordinal == descriptor)
        .unwrap()
        .result
        .as_ref()
        .unwrap();
    let walk = walk_bounded_mixed_events(
        &b,
        MixedEventBounds {
            event_range: t.routing.event_range.clone(),
        },
        Default::default(),
    )
    .unwrap();
    let mut events = Vec::new();
    let mut ordinal = 0;
    for item in walk.items {
        let count = item.logical_event_count() as u64;
        match item {
            MixedEventItem::Event(e) => {
                let MixedEventKind::Note(n) = e.event else {
                    panic!("unexpected event")
                };
                events.push(DecodedExportEvent::from_note(e.position, ordinal, &n));
            }
            MixedEventItem::PatchToNote(n) => {
                let patch = t
                    .patches
                    .iter()
                    .find(|p| p.source_ordinal == ordinal)
                    .unwrap()
                    .result
                    .as_ref()
                    .unwrap();
                events.push(patch.decoded_export_event());
                events.push(DecodedExportEvent::from_note_body(
                    n.first_note_position,
                    ordinal + 1,
                    &n.first_note,
                ));
            }
            _ => panic!("unexpected transition"),
        }
        ordinal += count;
    }
    let adapted = adapt_track(
        &events,
        Some(ChannelAssignment {
            channel: MidiChannel::new(t.routing.midi_channel).unwrap(),
            provenance: ChannelAssignmentProvenance::ParsedRouting,
        }),
        TimingPolicy::Identity480,
        PatchPolicy::StrictKnownOnly,
    )
    .unwrap();
    assert_eq!(adapted.counts.notes, expected_notes);
    assert_eq!(
        (
            adapted.counts.bank_select_msb,
            adapted.counts.bank_select_lsb,
            adapted.counts.program_changes,
            adapted.counts.controllers
        ),
        (1, 0, 1, 0)
    );
    let mut generated = source(&adapted.scheduled_events);
    let native = &mut native[descriptor - 1].messages;
    assert_eq!(generated.len(), expected_messages);
    assert_eq!(native.len(), expected_messages);
    if descriptor == 3 {
        assert_eq!(
            bytes_at(&generated, 0),
            vec![vec![0x93, 60, 127], vec![0xb3, 0, 80], vec![0xc3, 16]]
        );
        assert_eq!(bytes_at(native, 0), bytes_at(&generated, 0));
    }
    // Normalize only native velocity-zero endings and their corresponding
    // generated release representation; all other bytes, ticks and order exact.
    let mut normalized = 0;
    for (actual, expected) in generated.iter_mut().zip(native.iter_mut()) {
        if expected.bytes[0] >> 4 == 9 && expected.bytes[2] == 0 {
            assert_eq!(actual.tick, expected.tick);
            assert_eq!(actual.bytes[0], 0x80 | (expected.bytes[0] & 15));
            assert_eq!(actual.bytes[1], expected.bytes[1]);
            expected.bytes[0] = actual.bytes[0];
            expected.bytes[2] = actual.bytes[2];
            normalized += 1;
        }
    }
    assert_eq!(generated, *native);
    assert_eq!(normalized, usize::from(descriptor == 3));
    println!("d{descriptor}: {expected_messages} messages, {expected_notes} Notes, CC0=1 PC=1 CC32=0 ordinaryCC=0; full ordered stream reconciled ({normalized} normalized endings)");
}
#[test]
fn authenticated_sequence_r_track_1_complete_channel_stream() {
    reconcile(2, 76, 37);
}
#[test]
fn authenticated_sequence_r_track_2_complete_channel_stream() {
    reconcile(3, 126, 62);
}

fn duplicate_fixture() -> phoenix::midi_export::ExportTrackResult {
    let mut a = note(10, 2, 60, 20);
    a.source_range = Some(100..108);
    let mut b = note(10, 3, 60, 20);
    b.source_range = Some(108..114);
    adapt_track(
        &[a, b],
        Some(ChannelAssignment {
            channel: MidiChannel::new(4).unwrap(),
            provenance: ChannelAssignmentProvenance::Synthetic,
        }),
        TimingPolicy::Identity480,
        PatchPolicy::StrictKnownOnly,
    )
    .unwrap()
}
fn duplicate_result(
    a: &phoenix::midi_export::ExportTrackResult,
) -> Result<SerializedTrack, SmfSerializeError> {
    serialize_named_musical_track_with_note_provenance(
        b"synthetic",
        &a.scheduled_events,
        &a.note_provenance,
    )
}
#[test]
fn equivalent_duplicates_preserve_both_attacks_and_releases_with_source_order() {
    let mut a = duplicate_fixture();
    assert_eq!(
        a.note_provenance,
        vec![
            NoteSourceProvenance {
                start_ordinal: 4,
                source_range: 100..108
            },
            NoteSourceProvenance {
                start_ordinal: 6,
                source_range: 108..114
            },
        ]
    );
    let expected = duplicate_result(&a).unwrap();
    let messages = track(&expected.as_bytes()[8..]).messages;
    assert_eq!(bytes_at(&messages, 10), vec![vec![0x93, 60, 127]; 2]);
    assert_eq!(bytes_at(&messages, 30), vec![vec![0x83, 60, 64]; 2]);
    assert_eq!(messages.len(), 4);
    // Input order is irrelevant; unique retained ordinals determine order.
    a.scheduled_events.reverse();
    assert_eq!(duplicate_result(&a).unwrap(), expected);
    // The old API has no source geometry and must remain fail-closed.
    assert!(serialize_named_musical_track_with_ordering(
        b"synthetic",
        &a.scheduled_events,
        MusicalTrackOrdering::SourceOrder
    )
    .is_err());
}
#[test]
fn non_equivalent_duplicate_values_and_end_parentage_refuse() {
    for mutation in 0..6 {
        let mut a = duplicate_fixture();
        match mutation {
            0 => {
                a.scheduled_events[2].message = ChannelMessage::NoteOn {
                    channel: MidiChannel::new(4).unwrap(),
                    key: MidiDataByte::new(60).unwrap(),
                    attack_velocity: MidiDataByte::new(126).unwrap(),
                }
            }
            1 => {
                a.scheduled_events[3].message = ChannelMessage::NoteOff {
                    channel: MidiChannel::new(4).unwrap(),
                    key: MidiDataByte::new(60).unwrap(),
                    release_velocity: MidiDataByte::new(63).unwrap(),
                }
            }
            2 => a.scheduled_events[3].absolute_tick += 1,
            3 => {
                a.scheduled_events.remove(3);
            }
            4 => a.scheduled_events[3].stable_ordinal = 9,
            5 => {
                let end = a.scheduled_events[3].clone();
                a.scheduled_events.push(end);
            }
            _ => unreachable!(),
        }
        assert!(duplicate_result(&a).is_err(), "mutation {mutation}");
    }
}
#[test]
fn absent_ambiguous_overlapping_and_reversed_source_provenance_refuse() {
    for mutation in 0..7 {
        let mut a = duplicate_fixture();
        match mutation {
            0 => a.note_provenance.clear(),
            1 => {
                a.note_provenance.pop();
            }
            2 => a.note_provenance[1].source_range = 107..114,
            3 => a.note_provenance[1].source_range = 99..100,
            4 => a.note_provenance[1].source_range = 108..108,
            5 => {
                a.note_provenance.push(a.note_provenance[0].clone());
            }
            6 => a.scheduled_events[2].stable_ordinal = 4,
            _ => unreachable!(),
        }
        assert!(duplicate_result(&a).is_err(), "mutation {mutation}");
    }
}
#[test]
fn independent_duplicate_pairs_with_unrelated_events_preserve_every_message() {
    let mut a = duplicate_fixture();
    let original = a.scheduled_events.clone();
    for mut event in original {
        event.absolute_tick += 100;
        event.stable_ordinal += 4;
        a.scheduled_events.push(event);
    }
    a.note_provenance.extend([
        NoteSourceProvenance {
            start_ordinal: 8,
            source_range: 120..128,
        },
        NoteSourceProvenance {
            start_ordinal: 10,
            source_range: 128..134,
        },
    ]);
    a.scheduled_events.push(ScheduledEvent {
        absolute_tick: 10,
        stable_ordinal: 12,
        message: ChannelMessage::ControlChange {
            channel: MidiChannel::new(7).unwrap(),
            controller: MidiDataByte::new(1).unwrap(),
            value: MidiDataByte::new(99).unwrap(),
        },
    });
    let result = duplicate_result(&a).unwrap();
    let m = track(&result.as_bytes()[8..]).messages;
    assert_eq!(m.len(), 9);
    assert_eq!(
        bytes_at(&m, 10),
        vec![vec![0x93, 60, 127], vec![0x93, 60, 127], vec![0xb6, 1, 99]]
    );
    assert_eq!(bytes_at(&m, 30), vec![vec![0x83, 60, 64]; 2]);
    assert_eq!(bytes_at(&m, 110), vec![vec![0x93, 60, 127]; 2]);
    assert_eq!(bytes_at(&m, 130), vec![vec![0x83, 60, 64]; 2]);
}
#[test]
fn third_coincident_attack_refuses_and_distinct_channels_use_existing_semantics() {
    let mut a = duplicate_fixture();
    let mut third = a.scheduled_events[2].clone();
    third.stable_ordinal = 8;
    let mut end = a.scheduled_events[3].clone();
    end.stable_ordinal = 9;
    a.scheduled_events.extend([third, end]);
    a.note_provenance.push(NoteSourceProvenance {
        start_ordinal: 8,
        source_range: 114..120,
    });
    assert!(duplicate_result(&a).is_err());
    let mut a = duplicate_fixture();
    for event in &mut a.scheduled_events[2..] {
        match &mut event.message {
            ChannelMessage::NoteOn { channel, .. } | ChannelMessage::NoteOff { channel, .. } => {
                *channel = MidiChannel::new(5).unwrap()
            }
            _ => unreachable!(),
        }
    }
    a.note_provenance.clear();
    assert!(duplicate_result(&a).is_ok());
}
