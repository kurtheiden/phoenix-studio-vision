use super::*;
use crate::mixed_event::{walk_bounded_mixed_events, MixedEventBounds};
use crate::reference_smf;
use crate::sequence_container::parse_root_record_stream;

struct Fixture {
    bytes: Vec<u8>,
    association: Range<usize>,
    events: Range<usize>,
    rows: Vec<usize>,
}
fn record(b: &mut Vec<u8>, tag: u8, p: &[u8]) -> usize {
    b.push(tag);
    b.extend((p.len() as u32).to_be_bytes());
    let start = b.len();
    b.extend(p);
    start
}
fn patch() -> Vec<u8> {
    let core = [
        0, 0, 11, 0xf8, 11, 3, b'N', b'e', b'w', 3, b'G', b'1', b'1', 4, 255, 255, 255, 11,
    ];
    [vec![0, 255, 0x7c, core.len() as u8], core.to_vec()].concat()
}
fn fixture(entries: &[u16], events: &[u8], raw: u32, gs: bool) -> Fixture {
    let mut bytes = vec![0; 8];
    let mut rows = vec![];
    for (row, channel) in [0, 8, 2, 14, 0].into_iter().enumerate() {
        let mut p = vec![0; 36];
        p[25] = row as u8;
        p[26] = 7;
        p[27] = channel;
        rows.push(record(&mut bytes, 16, &p));
    }
    let mut p = vec![0; 40];
    p[0] = 8;
    p[1..9].copy_from_slice(b"OtherBox");
    p[33] = 7;
    record(&mut bytes, 42, &p);
    let mut association = vec![255; 33];
    association[0] = entries.len() as u8;
    for (i, row) in entries.iter().enumerate() {
        association[1 + 2 * i..3 + 2 * i].copy_from_slice(&row.to_be_bytes());
    }
    let a = record(&mut bytes, 1, &association);
    let mut p = vec![0; 14];
    p[..2].copy_from_slice(&[0, 1]);
    p[2..6].copy_from_slice(&raw.to_be_bytes());
    if gs {
        p.extend([
            0, 0xf0, 11, 0, 0x41, 0x10, 0x42, 0x12, 0x40, 0, 0x7f, 0, 0x41, 0xf7,
        ]);
    }
    p.extend(events);
    let end = p.len();
    p.extend([255, 255, 255, 127, 255, 47, 0]);
    let start = record(&mut bytes, 2, &p);
    Fixture {
        bytes,
        association: a..a + 33,
        events: start + 14..start + end,
        rows,
    }
}
fn resolve(f: &Fixture) -> Result<Vec<ControllerTargetProvenance>, ControllerTargetRefusal> {
    let root = parse_root_record_stream(&f.bytes).unwrap();
    let lookup = ControllerAssociation120::from_records(
        &root.records,
        f.association.clone(),
        &f.bytes[f.association.clone()],
    )?;
    let walk = walk_bounded_mixed_events(
        &f.bytes,
        MixedEventBounds {
            event_range: f.events.clone(),
        },
        Default::default(),
    )
    .unwrap();
    lookup.collect(&walk, 0)
}

#[test]
fn list_permutations_resolve_individual_multi_targets_and_compact_status() {
    let events = [
        patch(),
        vec![0, 0xb0, 91, 50, 2, 91, 51, 3, 0xb1, 93, 60, 1, 93, 61],
    ]
    .concat();
    for (entries, channels) in [([1, 3], [9, 15]), ([3, 1], [15, 9]), ([4, 2], [1, 3])] {
        let f = fixture(&entries, &events, 6, true);
        let (out, p) = crate::bounded_multi_patch::native_gs_multi_handoff(
            &f.bytes,
            f.association.clone(),
            f.events.clone(),
        )
        .unwrap();
        let c: Vec<_> = out
            .iter()
            .filter_map(|e| match e.kind {
                DecodedExportEventKind::MidiController {
                    channel,
                    number,
                    value,
                } => Some((e.absolute_position, channel, number, value)),
                _ => None,
            })
            .collect();
        assert_eq!(
            c,
            [
                (0, channels[0], 91, 50),
                (2, channels[0], 91, 51),
                (5, channels[1], 93, 60),
                (6, channels[1], 93, 61)
            ]
        );
        assert_eq!(
            p.iter().map(|p| p.instrument_row).collect::<Vec<_>>(),
            [
                entries[0] as u8,
                entries[0] as u8,
                entries[1] as u8,
                entries[1] as u8
            ]
        );
        assert_eq!(
            p.iter().map(|p| p.source_ordinal).collect::<Vec<_>>(),
            [2, 3, 4, 5]
        );
        assert_eq!(p[0].status_offset, p[1].status_offset);
        assert!(p[0].explicit_status);
        assert!(!p[1].explicit_status);
        assert_eq!(p[2].status_offset, p[3].status_offset);
        assert_eq!(
            p[2].list_entry_range,
            f.association.start + 3..f.association.start + 5
        );
        assert_eq!(
            p[2].routing_record_range,
            f.rows[usize::from(entries[1])] - 5..f.rows[usize::from(entries[1])] + 36
        );
        // This task does not broaden the strict Patch-only Multi grammar.
        assert!(
            crate::bounded_multi_patch::decode_bounded_multi_patch_track(
                &f.bytes,
                f.association.clone(),
                f.events.clone()
            )
            .is_err()
        );
    }
}

#[test]
fn invalid_selectors_refuse_single_and_multi_without_partial_output() {
    for selector in [1, 2, 15] {
        let f = fixture(&[1], &[0, 0xb0 | selector, 91, 50], 1, false);
        assert_eq!(
            resolve(&f).unwrap_err(),
            ControllerTargetRefusal::Selector(selector)
        );
    }
    for selector in [2, 15] {
        let f = fixture(
            &[1, 3],
            &[patch(), vec![0, 0xb0, 91, 50, 1, 0xb0 | selector, 93, 60]].concat(),
            4,
            true,
        );
        assert!(
            matches!(crate::bounded_multi_patch::native_gs_multi_handoff(&f.bytes,f.association.clone(),f.events.clone()),
            Err(crate::bounded_multi_patch::MultiPatchRefusal::ControllerTarget(ControllerTargetRefusal::Selector(s))) if s==selector)
        );
    }
}

#[test]
fn table_identity_device_channel_and_list_guards_fail_closed() {
    for entries in [vec![0], vec![99], vec![1, 1], vec![256], vec![]] {
        let f = fixture(&entries, &[0, 0xb0, 91, 50], 1, false);
        assert!(resolve(&f).is_err());
    }
    for (at, val) in [(25, 9), (26, 77), (27, 16)] {
        let mut f = fixture(&[1], &[0, 0xb0, 91, 50], 1, false);
        f.bytes[f.rows[1] + at] = val;
        assert!(resolve(&f).is_err());
    }
    let mut f = fixture(&[1], &[0, 0xb0, 91, 50], 1, false);
    f.bytes[f.association.start + 3] = 42;
    assert!(resolve(&f).is_err());
}

#[test]
fn handoff_is_transactional_preserves_noncontrollers_and_checks_all_bindings() {
    let events = [
        patch(),
        vec![0, 0x90, 60, 100, 64, 12, 2, 0xb0, 91, 50, 1, 93, 51],
    ]
    .concat();
    let f = fixture(&[1], &events, 4, false);
    let p = resolve(&f).unwrap();
    let w = walk_bounded_mixed_events(
        &f.bytes,
        MixedEventBounds {
            event_range: f.events.clone(),
        },
        Default::default(),
    )
    .unwrap();
    let raw = crate::recovery120::single_handoff(&w, None).unwrap();
    let before = raw.clone();
    assert_eq!(p[0].source_ordinal, 2); // Patch/Note composition owns two ordinals
    let effective = apply_targets(&raw, &p).unwrap();
    assert_eq!(raw, before);
    assert_eq!(effective[..2], raw[..2]);
    for (old, new) in raw.iter().zip(&effective).skip(2) {
        let mut expected = old.clone();
        let DecodedExportEventKind::MidiController { channel, .. } = &mut expected.kind else {
            panic!()
        };
        *channel = 9;
        assert_eq!(*new, expected);
    }
    assert!(apply_targets(&raw, &p[..1]).is_err());
    let mut duplicate = p.clone();
    duplicate.push(p[0].clone());
    assert!(apply_targets(&raw, &duplicate).is_err());
    for which in 0..5 {
        let mut bad = p.clone();
        match which {
            0 => bad[0].position += 1,
            1 => bad[0].representation_range.start += 1,
            2 => bad[0].controller_value ^= 1,
            3 => bad[0].resolved_channel = 17,
            _ => bad[0].source_ordinal = 99,
        }
        assert!(apply_targets(&raw, &bad).is_err());
    }
    assert!(apply_targets(&raw[..2], &p).is_err());
}

#[test]
fn complete_owned_recovery_keeps_raw_handoff_and_generic_midi_channels_unchanged() {
    let events = [0, 0xb0, 91, 50, 2, 93, 51, 3, 0x90, 60, 100, 64, 12];
    let b = crate::recovery120::tests::fixture(&events, 3);
    let results = crate::recovery120::recover_project120(&b).unwrap();
    let r = results[0].result.as_ref().unwrap();
    let t = &r.tracks()[0];
    assert_eq!(t.controller_targets.len(), 2);
    assert!(matches!(
        t.handoff[0].kind,
        DecodedExportEventKind::MidiController { channel: 1, .. }
    ));
    assert!(t
        .scheduled
        .iter()
        .filter_map(|e| match &e.message {
            crate::smf::ChannelMessage::ControlChange { channel, .. } => Some(channel.get()),
            _ => None,
        })
        .all(|c| c == 5));
    let generic = crate::midi_export::adapt_track(
        &t.handoff,
        t.channel_assignment,
        crate::midi_export::TimingPolicy::Identity480,
        crate::midi_export::PatchPolicy::StrictKnownOnly,
    )
    .unwrap();
    assert!(generic
        .scheduled_events
        .iter()
        .filter_map(|e| match &e.message {
            crate::smf::ChannelMessage::ControlChange { channel, .. } => Some(channel.get()),
            _ => None,
        })
        .all(|c| c == 1));
    let mut bad = events;
    bad[1] = 0xb1;
    assert!(
        crate::recovery120::recover_project120(&crate::recovery120::tests::fixture(&bad, 3))
            .unwrap()[0]
            .result
            .is_err()
    );
}

#[test]
fn ff60_context_and_source_accounting_survive_target_adaptation() {
    let events = [
        2, 255, 0x60, 7, 0x57, 0x7f, 0, 1, 2, 0x83, 4, 3, 0xb0, 6, 8, 2, 6, 12,
    ];
    let b = crate::recovery120::tests::fixture(&events, 3);
    let r = crate::recovery120::recover_project120(&b).unwrap();
    let r = r[0].result.as_ref().unwrap();
    let proof = &r.proof().tracks()[0];
    assert_eq!(proof.context_records(), 1);
    assert_eq!(proof.declared(), Some(3));
    let MixedEventItem::Event(e) = &proof.walk().unwrap().items[0] else {
        panic!()
    };
    let MixedEventKind::MidiController(c) = &e.event else {
        panic!()
    };
    let ctx = c.context.as_ref().unwrap();
    assert_eq!(ctx.leading_timing.value, 2);
    assert_eq!(c.timing_delta.value, 3);
    assert_eq!(
        r.tracks()[0].controller_targets[0].representation_range,
        c.representation_range
    );
    assert_eq!(r.tracks()[0].controller_targets[0].resolved_channel, 5);
    assert_eq!(ctx.context.payload.bytes, [0x57, 0x7f, 0, 1, 2, 0x83, 4]);
}

#[test]
#[ignore = "requires independent SCHOOL PROJECTS and native M-vegetarian del MIDI"]
fn authentic_school_six_owned_setup_controllers_match_native() {
    use sha2::{Digest, Sha256};
    let b = std::fs::read(std::env::var("PHOENIX_SCHOOL_ORIGINAL_SOURCE").unwrap()).unwrap();
    let reference =
        std::fs::read(std::env::var("PHOENIX_SCHOOL_TRANSPOSE_REFERENCE").unwrap()).unwrap();
    assert_eq!(
        format!("{:x}", Sha256::digest(&b)),
        "bfd4fa1208e2cd884ec51ccfb1131d5c02723d7c90acafc98bd89597e1a20331"
    );
    assert_eq!(
        format!("{:x}", Sha256::digest(&reference)),
        "59a14a9313bea0259ec964f9219aac3b1e0b82cbf6e422a78f5a558b4991378f"
    );
    let project = crate::semantic_layout120::associate_semantic_project_120(&b).unwrap();
    let s = project
        .candidates
        .iter()
        .filter_map(|c| c.association.as_ref().ok())
        .find(|s| s.source_name.bytes == b"M-vegetarian del")
        .unwrap();
    let t = s
        .ordinary
        .iter()
        .find(|t| t.pair.primary.record_range.start == 240432)
        .unwrap();
    let assoc = t.slot.label.range.start - 33..t.slot.label.range.start;
    assert_eq!(assoc, 222047..222080);
    let bounds = crate::sequence_container::TrackRecordPair {
        pair_ordinal: t.local_pair_ordinal,
        primary: t.pair.primary.clone(),
        secondary: t.pair.secondary.clone(),
        candidate_event_start: t.pair.primary.payload.range.start + 14,
        event_containing_range: t.pair.primary.payload.range.start + 14
            ..t.pair.primary.payload.range.end,
    }
    .validated_event_bounds()
    .unwrap();
    assert_eq!(bounds.event_range, 240451..240759);
    let root = parse_root_record_stream(&b).unwrap();
    let w = walk_bounded_mixed_events(
        &b,
        MixedEventBounds {
            event_range: bounds.event_range,
        },
        Default::default(),
    )
    .unwrap();
    let lookup =
        ControllerAssociation120::from_records(&root.records, assoc.clone(), &b[assoc]).unwrap();
    let targets = lookup.collect(&w, 0).unwrap();
    assert_eq!(targets.len(), 6);
    assert_eq!(
        targets
            .iter()
            .map(|p| (p.selector, p.instrument_row, p.resolved_channel))
            .collect::<Vec<_>>(),
        [
            (0, 21, 2),
            (0, 21, 2),
            (1, 76, 9),
            (1, 76, 9),
            (2, 78, 11),
            (2, 78, 11)
        ]
    );
    let mut raw = Vec::new();
    let mut ordinal = 0;
    for item in &w.items {
        if let MixedEventItem::Event(e) = item {
            if let MixedEventKind::MidiController(c) = &e.event {
                raw.push(DecodedExportEvent::from_midi_controller(
                    e.position, ordinal, c,
                ));
            }
        }
        ordinal += item.logical_event_count() as u64;
    }
    let actual = apply_targets(&raw, &targets).unwrap();
    let all: Vec<_> = reference_smf::midi_tracks(&reference)
        .into_iter()
        .flatten()
        .collect();
    for e in actual {
        let DecodedExportEventKind::MidiController {
            channel,
            number,
            value,
        } = e.kind
        else {
            panic!()
        };
        assert!(all.contains(&(e.absolute_position, 0xb0 + channel - 1, vec![number, value])));
    }
}
