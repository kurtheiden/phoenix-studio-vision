use phoenix::{
    bounded_multi_patch::{decode_bounded_multi_patch_track, MultiPatchRefusal},
    midi_export::{
        adapt_track, ChannelAssignment, ChannelAssignmentProvenance, DecodedExportEvent,
        DecodedExportEventKind, PatchPolicy, PatchTranslation, TimingPolicy,
    },
    smf::{
        serialize_channel_message, serialize_named_musical_track_with_ordering, MidiChannel,
        MusicalTrackOrdering,
    },
};
use std::ops::Range;

struct Fixture {
    bytes: Vec<u8>,
    association: Range<usize>,
    events: Range<usize>,
    rows: Vec<usize>,
    device: usize,
}
fn record(b: &mut Vec<u8>, tag: u8, payload: &[u8]) -> usize {
    b.push(tag);
    b.extend((payload.len() as u32).to_be_bytes());
    let start = b.len();
    b.extend(payload);
    start
}
fn patch(delta: u8, selector: u8, p: u8, i: bool) -> Vec<u8> {
    let name = b"Synthetic";
    let label = if i { "I73".to_owned() } else { format!("G{p}") };
    let mut core = vec![
        selector,
        0,
        p,
        if i { 0 } else { 0xf8 },
        p,
        name.len() as u8,
    ];
    core.extend(name);
    core.push(label.len() as u8);
    core.extend(label.as_bytes());
    core.extend([4, 255, 255, 255, p]);
    let mut event = vec![delta, 255, 0x7c, core.len() as u8];
    event.extend(core);
    event
}
fn fixture(channel: u8) -> Fixture {
    let mut bytes = vec![0; 8];
    let mut rows = vec![];
    for ordinal in 0..4 {
        let mut row = vec![0; 36];
        row[25] = ordinal;
        row[26] = 7;
        row[27] = if ordinal == 1 { channel - 1 } else { 5 };
        rows.push(record(&mut bytes, 16, &row));
    }
    let mut device = vec![0; 40];
    device[0] = 4;
    device[1..5].copy_from_slice(b"Unit");
    device[33] = 7;
    let device = record(&mut bytes, 42, &device);
    let mut assoc = vec![255; 33];
    assoc[..5].copy_from_slice(&[2, 0, 1, 0, 2]);
    let a = record(&mut bytes, 1, &assoc);
    let association = a..a + 33;
    let mut payload = vec![0; 14];
    payload.extend(patch(0, 0, 11, false));
    payload.extend(patch(0, 1, 48, true));
    payload.extend(patch(3, 0, 127, false));
    let end = payload.len();
    payload.extend([255, 255, 255, 127, 255, 47, 0]);
    let start = record(&mut bytes, 2, &payload);
    Fixture {
        bytes,
        association,
        events: start + 14..start + end,
        rows,
        device,
    }
}
fn decode(
    f: &Fixture,
) -> Result<phoenix::bounded_multi_patch::BoundedMultiPatchTrack, MultiPatchRefusal> {
    decode_bounded_multi_patch_track(&f.bytes, f.association.clone(), f.events.clone())
}
#[test]
fn multi_targets_keep_same_source_channels_programs_order_timing_and_provenance() {
    let f = fixture(3);
    let track = decode(&f).unwrap();
    let p = track.patches();
    assert_eq!(
        p.iter()
            .map(|p| (
                p.selector,
                p.instrument_row,
                p.midi_channel,
                p.program,
                p.position
            ))
            .collect::<Vec<_>>(),
        vec![(0, 1, 3, 11, 0), (1, 2, 6, 48, 0), (0, 1, 3, 127, 3)]
    );
    assert_eq!(p[0].representation_range.start, f.events.start);
    assert_eq!(p[2].representation_range.end, f.events.end);
    assert_eq!(
        p[0].list_entry_range,
        f.association.start + 1..f.association.start + 3
    );
    assert_eq!(p[1].routing_record_range, f.rows[2] - 5..f.rows[2] + 36);
    assert_eq!(p[1].device_record_range, f.device - 5..f.device + 40);
    assert_eq!(p[1].patch_name, b"Synthetic");
    assert_eq!(
        p[1].post_name_context,
        [3, b'I', b'7', b'3', 4, 255, 255, 255]
    );
    let events = track.export_events();
    assert_eq!(events[1].source_ordinal, 1);
    assert_eq!(
        events[1].source_range,
        Some(p[1].representation_range.clone())
    );
    let adapted = track.adapt().unwrap();
    assert_eq!(adapted.counts.program_changes, 3);
    assert_eq!(
        (
            adapted.counts.controllers,
            adapted.counts.bank_select_msb,
            adapted.counts.bank_select_lsb
        ),
        (0, 0, 0)
    );
    assert_eq!(
        adapted
            .scheduled_events
            .iter()
            .map(|e| (e.absolute_tick, serialize_channel_message(&e.message)))
            .collect::<Vec<_>>(),
        vec![
            (0, vec![0xc2, 11]),
            (0, vec![0xc5, 48]),
            (3, vec![0xc2, 127])
        ]
    );
    let serialized = serialize_named_musical_track_with_ordering(
        b"Track",
        &adapted.scheduled_events,
        MusicalTrackOrdering::SourceOrder,
    )
    .unwrap();
    assert!(serialized.as_bytes().windows(3).any(|b| b == [0, 0xc2, 11]));
}
#[test]
fn independent_sources_never_borrow_resaved_channels() {
    let original = fixture(1);
    let resaved = fixture(15);
    assert_eq!(decode(&original).unwrap().patches()[0].midi_channel, 1);
    assert_eq!(decode(&resaved).unwrap().patches()[0].midi_channel, 15);
    assert_eq!(decode(&original).unwrap().patches()[0].midi_channel, 1);
}
#[test]
fn malformed_and_ambiguous_lists_routing_and_channels_refuse_transactionally() {
    let mutations: Vec<(usize, u8, MultiPatchRefusal)> = {
        let f = fixture(3);
        vec![
            (f.association.start, 0, MultiPatchRefusal::Association),
            (f.association.end - 1, 0, MultiPatchRefusal::Association),
            (
                f.association.start + 4,
                1,
                MultiPatchRefusal::RoutingIdentity,
            ),
            (
                f.association.start + 4,
                99,
                MultiPatchRefusal::RoutingIdentity,
            ),
            (f.rows[2] + 25, 1, MultiPatchRefusal::RoutingIdentity),
            (f.rows[2] + 27, 16, MultiPatchRefusal::Channel),
            (f.rows[2] + 26, 99, MultiPatchRefusal::Device),
            (f.device, 0, MultiPatchRefusal::Device),
            (f.events.start + 4, 2, MultiPatchRefusal::Selector),
        ]
    };
    for (offset, value, expected) in mutations {
        let mut f = fixture(3);
        f.bytes[offset] = value;
        assert_eq!(decode(&f), Err(expected), "offset {offset}");
    }
    let mut f = fixture(3);
    let device = f.bytes[f.device..f.device + 40].to_vec();
    record(&mut f.bytes, 42, &device);
    assert_eq!(decode(&f), Err(MultiPatchRefusal::Device));
    let f = fixture(3);
    assert_eq!(
        decode_bounded_multi_patch_track(
            &f.bytes,
            f.association.start..f.association.end - 1,
            f.events.clone()
        ),
        Err(MultiPatchRefusal::Association)
    );
}
#[test]
fn unsupported_translation_truncation_and_non_patch_tracks_refuse() {
    for relative in [5, 6, 7] {
        // opaque byte, repeated Program, wrapper kind
        let mut f = fixture(3);
        f.bytes[f.events.start + relative] ^= 1;
        assert_eq!(decode(&f), Err(MultiPatchRefusal::UnsupportedTranslation));
    }
    let mut f = fixture(3);
    f.bytes[f.events.end - 2] = 0;
    assert_eq!(decode(&f), Err(MultiPatchRefusal::UnsupportedTranslation));
    let f = fixture(3);
    assert_eq!(
        decode_bounded_multi_patch_track(
            &f.bytes,
            f.association.clone(),
            f.events.start..f.events.end - 1
        ),
        Err(MultiPatchRefusal::EventWalk)
    );
    let mut f = fixture(3);
    f.bytes[f.events.start + 2] = 0x40;
    assert_eq!(decode(&f), Err(MultiPatchRefusal::EventWalk));
    let mut f = fixture(3);
    let mut payload = vec![0; 14];
    payload.extend([0, 0x90, 60, 64, 32, 1]);
    let p = record(&mut f.bytes, 2, &payload);
    f.events = p + 14..p + 20;
    assert_eq!(decode(&f), Err(MultiPatchRefusal::UnsupportedEvent));
}
#[test]
fn targeted_program_does_not_change_neighboring_event_family_routing() {
    let f = fixture(3);
    let mut events = decode(&f).unwrap().export_events();
    for (ordinal, kind) in [
        DecodedExportEventKind::Note {
            pitch: 60,
            attack_velocity: 70,
            release_velocity: 30,
            duration: 10,
        },
        DecodedExportEventKind::Controller {
            number: 7,
            value: 99,
            has_opaque_context: false,
        },
        DecodedExportEventKind::MidiController {
            channel: 2,
            number: 10,
            value: 55,
        },
        DecodedExportEventKind::PitchBend { lsb: 0, msb: 64 },
        DecodedExportEventKind::Patch {
            program: 8,
            translation: PatchTranslation::ProgramOnlyConfirmed,
        },
    ]
    .into_iter()
    .enumerate()
    {
        events.push(DecodedExportEvent {
            absolute_position: 20,
            source_ordinal: ordinal as u64 + 3,
            source_range: None,
            kind,
        });
    }
    let adapted = adapt_track(
        &events,
        Some(ChannelAssignment {
            channel: MidiChannel::new(11).unwrap(),
            provenance: ChannelAssignmentProvenance::Synthetic,
        }),
        TimingPolicy::Identity480,
        PatchPolicy::StrictKnownOnly,
    )
    .unwrap();
    let raw: Vec<_> = adapted
        .scheduled_events
        .iter()
        .map(|e| serialize_channel_message(&e.message))
        .collect();
    for expected in [
        vec![0xc2, 11],
        vec![0xc5, 48],
        vec![0x9a, 60, 70],
        vec![0x8a, 60, 30],
        vec![0xba, 7, 99],
        vec![0xb1, 10, 55],
        vec![0xea, 0, 64],
        vec![0xca, 8],
    ] {
        assert!(raw.contains(&expected), "missing {expected:?}");
    }
}
#[test]
fn association_cannot_cross_a_sequence_boundary() {
    let mut f = fixture(3);
    record(&mut f.bytes, 1, &[0]);
    let mut payload = vec![0; 14];
    payload.extend(patch(0, 0, 11, false));
    let p = record(&mut f.bytes, 2, &payload);
    f.events = p + 14..p + payload.len();
    assert_eq!(decode(&f), Err(MultiPatchRefusal::BoundsOrOwnership));
}
