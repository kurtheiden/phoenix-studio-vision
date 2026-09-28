//! Synthetic boundaries and separately authenticated native-export evidence.
use phoenix::bounded_patch_translation::{classify_bounded_cc0_patches, Cc0PatchRefusal};
use phoenix::bounded_routing::RoutingRefusal;
use phoenix::midi_export::{
    adapt_track, ChannelAssignment, ChannelAssignmentProvenance, DecodedExportEvent, PatchPolicy,
    TimingPolicy,
};
use phoenix::mixed_event::{
    walk_bounded_mixed_events, MixedEventBounds, MixedEventItem, MixedEventKind,
};
use phoenix::sequence_container::{parse_project_166, parse_root_record_stream};
use phoenix::smf::{serialize_musical_track, MidiChannel};
use sha2::{Digest, Sha256};

const PROJECT: &str = "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline";
const MIDI: &str = "/Users/kurtheiden/Documents/Phoenix Research/Studio Vision MIDI Exports/Project 001/Seq R Test";
const NOTE: &[u8] = &[0, 0x90, 60, 127, 113, 102];

fn record(b: &mut Vec<u8>, tag: u8, p: &[u8]) {
    b.push(tag);
    b.extend((p.len() as u32).to_be_bytes());
    b.extend(p);
}
fn synthetic(events: &[u8]) -> Vec<u8> {
    let mut b = vec![0; 8];
    for i in 0..2 {
        let mut p = vec![0; 36];
        p[25] = i;
        p[26] = 7;
        p[27] = 3;
        record(&mut b, 0x10, &p);
    }
    let mut device = vec![0; 40];
    device[0] = 4;
    device[1..5].copy_from_slice(b"Test");
    device[33] = 7;
    record(&mut b, 0x2a, &device);
    let start = b.len();
    let mut p = vec![0; 3 * 166 + 172];
    p[0] = 3;
    record(&mut b, 1, &p);
    let label = start + 208 + 2 * 166 + 15;
    b[label..label + 5].copy_from_slice(b"Other");
    b[label - 33] = 1;
    b[label - 31] = 1;
    b[label - 30..label].fill(0xff);
    let mut name = vec![0; 11];
    name.extend([4, b'N', b'e', b'w', b'!']);
    record(&mut b, 7, &name);
    record(&mut b, 2, &[0; 22]);
    record(&mut b, 0x29, &[]);
    record(&mut b, 2, &[0; 21]);
    record(&mut b, 0x29, &[]);
    let mut p = vec![0; 14];
    p.extend(events);
    p.extend([0xff, 0, 0, 0, 0xff, 0x2f, 0]);
    record(&mut b, 2, &p);
    record(&mut b, 0x29, &[]);
    record(&mut b, 0, &[]);
    b
}
fn patch(p: u8) -> Vec<u8> {
    let mut e = vec![0, 0xff, 0x7c, 27, 0, 0, p | 0x80, 8, p, 12];
    e.extend(b"Unseen name!");
    e.extend([3, b'X', b'Y', b'Z', 4, 0xff, 0x50, 0xff, p]);
    e.extend(NOTE);
    e
}
fn refusal(e: &[u8]) -> Cc0PatchRefusal {
    let b = synthetic(e);
    let rows = classify_bounded_cc0_patches(&b).unwrap();
    rows[0].result.as_ref().unwrap().patches[0]
        .result
        .as_ref()
        .unwrap_err()
        .to_owned()
}
fn authentic() -> Vec<u8> {
    let b = std::fs::read(PROJECT).expect("authenticated source required");
    assert_eq!(b.len(), 211468);
    assert_eq!(
        format!("{:x}", Sha256::digest(&b)),
        "e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132"
    );
    b
}

#[test]
fn unseen_names_ascii_context_and_both_programs_are_structural() {
    for p in [16, 35] {
        for initial in [false, true] {
            let mut e = Vec::new();
            if initial {
                e.extend([0, 0xff, 0x60, 7, 0x57, 0x7f, 0, 1, 2, 3, 4]);
            } else {
                e.extend(NOTE);
            }
            e.extend(patch(p));
            let b = synthetic(&e);
            let rows = classify_bounded_cc0_patches(&b).unwrap();
            let t = rows[0].result.as_ref().unwrap();
            let c = t.patches[0].result.as_ref().unwrap();
            assert_eq!(c.source().patch.name.text, "Unseen name!");
            assert_eq!(
                c.source().patch.post_name_context.bytes,
                &[3, b'X', b'Y', b'Z', 4, 255, 80, 255]
            );
            assert_eq!(c.source_ordinal(), if initial { 0 } else { 1 });
            assert_eq!(t.routing.midi_channel, 4);
            assert_eq!(c.source().initial_context.is_some(), initial);
            assert_eq!(c.decoded_export_event().source_ordinal, c.source_ordinal());
            assert_eq!(c.decoded_export_event().absolute_position, 0);
            assert_eq!(c.source().patch.program_change.value, p);
            assert_eq!(
                &b[c.source().patch.pre_name_context.range.clone()],
                c.source().patch.pre_name_context.bytes
            );
            assert_eq!(
                c.decoded_export_event().kind,
                phoenix::midi_export::DecodedExportEventKind::Patch {
                    program: p,
                    translation: phoenix::midi_export::PatchTranslation::ConfirmedBankSelectMsb {
                        msb: 80
                    }
                }
            );
        }
    }
}

#[test]
fn bank_tail_and_ascii_shape_refusals() {
    for (offset, value) in [(28, 81), (29, 0), (27, 0), (26, 5), (22, 2), (23, 128)] {
        let mut e = patch(35);
        e[offset] = value;
        assert_eq!(refusal(&e), Cc0PatchRefusal::PostNameContext, "{offset}");
    }
}
#[test]
fn pre_name_and_program_refusals() {
    for offset in 5..9 {
        let mut e = patch(35);
        e[offset] ^= 1;
        assert_eq!(refusal(&e), Cc0PatchRefusal::PreNameContext);
    }
    for p in [0, 17, 34, 36, 127] {
        assert_eq!(refusal(&patch(p)), Cc0PatchRefusal::UnsupportedProgram);
    }
    for p in [128, 255] {
        assert_eq!(refusal(&patch(p)), Cc0PatchRefusal::InvalidProgram);
    }
}
#[test]
fn framed_payload_and_name_length_refusals() {
    let mut e = patch(35);
    e[3] += 1;
    e.insert(30, 0);
    assert_eq!(refusal(&e), Cc0PatchRefusal::PayloadLength);
    let mut e = patch(35);
    e[9] = 11;
    assert_eq!(refusal(&e), Cc0PatchRefusal::NameLength);
    let mut e = patch(35);
    e[3] -= 1;
    e.remove(21);
    assert_eq!(refusal(&e), Cc0PatchRefusal::PayloadLength);
}
#[test]
fn position_and_transition_refusals() {
    let mut initial = vec![0, 0xff, 0x60, 8, 0x57, 0x7f, 0, 1, 2, 3, 4, 5];
    initial.extend(patch(35));
    assert_eq!(refusal(&initial), Cc0PatchRefusal::InitialContext);

    let mut e = patch(35);
    e[0] = 1;
    assert_eq!(refusal(&e), Cc0PatchRefusal::NonzeroPosition);
    let mut e = patch(35);
    e.truncate(31);
    assert_eq!(refusal(&e), Cc0PatchRefusal::UnsupportedTransition);
    let mut e = patch(35);
    e.splice(32..32, [0xff, 0x60, 7, 0x57, 0x7f, 0, 1, 2, 3, 4, 0]);
    assert_eq!(refusal(&e), Cc0PatchRefusal::UnsupportedTransition);
}
#[test]
fn track_level_gates_cannot_be_bypassed() {
    let mut e = patch(35);
    e.push(0x81);
    let b = synthetic(&e);
    assert!(matches!(
        classify_bounded_cc0_patches(&b).unwrap()[0].result,
        Err(RoutingRefusal::IncompleteEventWalk(_))
    ));
    let mut b = synthetic(&patch(35));
    let root = parse_root_record_stream(&b).unwrap();
    let p = root
        .records
        .iter()
        .filter(|r| r.record_type.value == 0x10)
        .nth(1)
        .unwrap()
        .payload
        .range
        .start;
    b[p + 27] = 16;
    assert_eq!(
        classify_bounded_cc0_patches(&b).unwrap()[0].result,
        Err(RoutingRefusal::InvalidChannel)
    );
    let mut e = vec![0, 0xff, 0x60, 7, 0x56, 0x7f, 0, 1, 2, 3, 4];
    e.extend(patch(35));
    let b = synthetic(&e);
    assert!(matches!(
        classify_bounded_cc0_patches(&b).unwrap()[0].result,
        Err(RoutingRefusal::ConflictingContext { .. })
    ));
    let mut b = synthetic(&patch(35));
    let p = parse_project_166(&b).unwrap();
    let label = p.sequences[0].descriptors[2].label_start;
    b[label] = 0;
    assert_eq!(
        classify_bounded_cc0_patches(&b).unwrap()[0].result,
        Err(RoutingRefusal::BlankOrUnboundedLabel)
    );
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
fn bank_pc(m: &Message) -> bool {
    m.bytes[0] >> 4 == 12 || (m.bytes[0] >> 4 == 11 && matches!(m.bytes[1], 0 | 32))
}

#[test]
fn authenticated_sequence_r_matches_native_bank_program_and_same_tick_order() {
    let b = authentic();
    let native = std::fs::read(MIDI).expect("owner native export required, never skipped");
    assert_eq!(native.len(), 1025);
    assert_eq!(
        format!("{:x}", Sha256::digest(&native)),
        "a97c0c3e99e95bc0e97941c8823e90f81758c94112c7dd1a64ae707250eca887"
    );
    let native = smf(&native);
    assert_eq!(native[0].name.as_deref(), Some(b"Sequence R".as_slice()));
    let rows = classify_bounded_cc0_patches(&b).unwrap();
    for (d, ch, p, name) in [
        (2, 3, 35, b"Track 1".as_slice()),
        (3, 4, 16, b"Track 2".as_slice()),
    ] {
        let t = rows
            .iter()
            .find(|r| r.structural_ordinal == 17 && r.descriptor_ordinal == d)
            .unwrap()
            .result
            .as_ref()
            .unwrap();
        assert_eq!(t.patches.len(), 1);
        let c = t.patches[0].result.as_ref().unwrap();
        assert_eq!(t.routing.midi_channel, ch);
        let w = walk_bounded_mixed_events(
            &b,
            MixedEventBounds {
                event_range: t.routing.event_range.clone(),
            },
            Default::default(),
        )
        .unwrap();
        let mut events = Vec::new();
        let mut ordinal = 0;
        for item in w.items {
            let count = item.logical_event_count() as u64;
            match item {
                MixedEventItem::Event(e) => {
                    let MixedEventKind::Note(n) = e.event else {
                        panic!("unexpected R event")
                    };
                    events.push(DecodedExportEvent::from_note(e.position, ordinal, &n));
                }
                MixedEventItem::PatchToNote(n) => {
                    assert_eq!(c.source_ordinal(), ordinal);
                    events.push(c.decoded_export_event());
                    events.push(DecodedExportEvent::from_note_body(
                        n.first_note_position,
                        ordinal + 1,
                        &n.first_note,
                    ));
                }
                _ => panic!("unexpected standalone Patch"),
            };
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
        assert_eq!(
            (
                adapted.counts.bank_select_msb,
                adapted.counts.bank_select_lsb,
                adapted.counts.program_changes
            ),
            (1, 0, 1)
        );
        // The classifier/adapter boundary preserves source order. The legacy
        // serializer has its own message-priority policy; this task does not
        // authorize changing exact-profile export or that policy.
        let source_order = adapted
            .scheduled_events
            .iter()
            .filter(|e| e.absolute_tick == 0)
            .map(|e| {
                use phoenix::smf::ChannelMessage;
                let bytes = match e.message {
                    ChannelMessage::NoteOn {
                        channel,
                        key,
                        attack_velocity,
                    } => vec![0x90 | (channel.get() - 1), key.get(), attack_velocity.get()],
                    ChannelMessage::ControlChange {
                        channel,
                        controller,
                        value,
                    } => vec![0xb0 | (channel.get() - 1), controller.get(), value.get()],
                    ChannelMessage::ProgramChange { channel, program } => {
                        vec![0xc0 | (channel.get() - 1), program.get()]
                    }
                    _ => panic!("unexpected tick-zero event"),
                };
                Message {
                    tick: e.absolute_tick,
                    bytes,
                }
            })
            .collect::<Vec<_>>();
        let generated = serialize_musical_track(&adapted.scheduled_events).unwrap();
        let generated = track(&generated.as_bytes()[8..]);
        let native = native
            .iter()
            .find(|t| t.name.as_deref() == Some(name))
            .unwrap();
        let expected = vec![
            Message {
                tick: 0,
                bytes: vec![0xb0 | (ch - 1), 0, 80],
            },
            Message {
                tick: 0,
                bytes: vec![0xc0 | (ch - 1), p],
            },
        ];
        assert_eq!(
            native
                .messages
                .iter()
                .filter(|m| bank_pc(m))
                .collect::<Vec<_>>(),
            expected.iter().collect::<Vec<_>>()
        );
        assert_eq!(
            generated
                .messages
                .iter()
                .filter(|m| bank_pc(m))
                .collect::<Vec<_>>(),
            expected.iter().collect::<Vec<_>>()
        );
        let mut at_zero = Vec::new();
        if d == 3 {
            at_zero.push(Message {
                tick: 0,
                bytes: vec![0x93, 60, 127],
            });
        }
        at_zero.extend(expected);
        assert_eq!(
            native
                .messages
                .iter()
                .filter(|m| m.tick == 0)
                .collect::<Vec<_>>(),
            at_zero.iter().collect::<Vec<_>>()
        );
        assert_eq!(source_order, at_zero);
        println!("Sequence R d{d}: channel {ch}, tick 0, CC0 80 then PC {p}; CC32 absent; same-tick order matched");
    }
}

#[test]
fn authenticated_corpus_acceptance_is_exactly_two_sequence_r_patches() {
    let b = authentic();
    let p = parse_project_166(&b).unwrap();
    assert_eq!(p.sequences.len(), 18);
    let rows = classify_bounded_cc0_patches(&b).unwrap();
    let (mut accepted, mut rejected_patches, mut refused_tracks) = (Vec::new(), 0, 0);
    for row in &rows {
        match &row.result {
            Ok(t) => {
                for patch in &t.patches {
                    match &patch.result {
                        Ok(c) => {
                            accepted.push((
                                row.structural_ordinal,
                                row.descriptor_ordinal,
                                c.source_ordinal(),
                            ));
                            println!(
                                "ACCEPT {} d{} source_ordinal={} PC={} channel={}",
                                p.sequences[row.structural_ordinal]
                                    .sequence_name
                                    .as_utf8()
                                    .unwrap(),
                                row.descriptor_ordinal,
                                c.source_ordinal(),
                                c.source().patch.program_change.value,
                                t.routing.midi_channel
                            );
                        }
                        Err(reason) => {
                            rejected_patches += 1;
                            println!(
                                "REFUSE {} d{} ordinal={} {:?}",
                                p.sequences[row.structural_ordinal]
                                    .sequence_name
                                    .as_utf8()
                                    .unwrap(),
                                row.descriptor_ordinal,
                                patch.source_ordinal,
                                reason
                            );
                        }
                    }
                }
            }
            Err(_) => refused_tracks += 1,
        }
    }
    assert_eq!(accepted, vec![(17, 2, 0), (17, 3, 1)]);
    println!("CORPUS accepted={} rejected_patches={rejected_patches} refused_tracks={refused_tracks} descriptor_results={}",accepted.len(),rows.len());
}

#[test]
fn classification_does_not_change_profile_evidence_or_readiness() {
    use phoenix::app_contract::{
        DiagnosticsLevel, InspectProjectRequest, Readiness, CONTRACT_VERSION,
    };
    use phoenix::app_service::AppService;
    let mut app = AppService::new();
    let before = app
        .inspect_project(InspectProjectRequest {
            contract_version: CONTRACT_VERSION,
            source_path: PROJECT.into(),
            diagnostics_level: DiagnosticsLevel::Full,
        })
        .unwrap();
    let evidence = app.profile_evidence(&before.session_id).unwrap();
    classify_bounded_cc0_patches(&authentic()).unwrap();
    assert_eq!(app.get_inspection(&before.session_id).unwrap(), before);
    assert_eq!(app.profile_evidence(&before.session_id).unwrap(), evidence);
    assert_eq!(
        before
            .sequences
            .iter()
            .filter(|s| s.readiness == Readiness::Ready && s.export_capability.is_some())
            .count(),
        6
    );
    assert_eq!(
        before
            .sequences
            .iter()
            .filter(|s| s.export_capability.is_none())
            .count(),
        12
    );
}
