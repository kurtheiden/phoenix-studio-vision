use super::*;
use crate::{midi_export, recovery120, smf};

fn event(length: u8, delta: u8) -> Vec<u8> {
    let mut b = vec![delta, 0xf0, length, 0];
    b.extend((0..length - 2).map(|i| 0x30 + i));
    b.push(0xf7);
    b
}
fn walk(b: &[u8], previous: u32) -> Result<MixedEventWalk<'_>, MixedEventWalkError> {
    walk_bounded_120_single_events(
        b,
        MixedEventBounds {
            event_range: 0..b.len(),
        },
        MixedEventTimingBasis {
            previous_event_position: previous,
        },
    )
}

#[test]
fn guarded_sysex120_lengths_timing_ranges_handoff_and_compact_controllers() {
    for length in [9, 11] {
        let mut b = event(length, 3);
        let end = b.len();
        b.extend([5, 0xb0, 10, 127, 2, 10, 125]);
        let w = walk(&b, 100).unwrap();
        assert_eq!(w.consumed_range, 0..b.len());
        assert_eq!(w.logical_event_count(), 3);
        let MixedEventItem::Event(e) = &w.items[0] else {
            panic!()
        };
        let MixedEventKind::SysEx(s) = &e.event else {
            panic!()
        };
        assert_eq!(e.position, 103);
        assert_eq!(s.representation_range, 0..end);
        assert_eq!(
            s.framing,
            Some(LocatedByte {
                offset: 3,
                value: 0
            })
        );
        assert_eq!(s.payload.range, 4..end);
        assert_eq!(s.payload.bytes, &b[4..end]);
        assert_eq!(s.payload_length.value, length);
        assert_eq!(s.timing_delta.bytes, [3]);
        let handoff = recovery120::single_handoff(&w, None).unwrap();
        assert_eq!(
            handoff
                .iter()
                .map(|e| (e.absolute_position, e.source_ordinal))
                .collect::<Vec<_>>(),
            [(103, 0), (108, 1), (110, 2)]
        );
        let assignment = Some(midi_export::ChannelAssignment {
            channel: smf::MidiChannel::new(5).unwrap(),
            provenance: midi_export::ChannelAssignmentProvenance::Synthetic,
        });
        // Generic adaptation and generic source decoding retain their old scope.
        assert!(midi_export::adapt_track(
            &handoff,
            assignment,
            midi_export::TimingPolicy::Identity480,
            midi_export::PatchPolicy::StrictKnownOnly
        )
        .is_err());
        assert!(walk_bounded_mixed_events(
            &b,
            MixedEventBounds {
                event_range: 0..b.len()
            },
            Default::default()
        )
        .is_err());
        let a = midi_export::adapt_owned120_track(
            &handoff,
            assignment,
            midi_export::TimingPolicy::Identity480,
            midi_export::PatchPolicy::StrictKnownOnly,
        )
        .unwrap();
        let encoded = smf::serialize_channel_message(&a.scheduled_events[0].message);
        assert_eq!(
            encoded,
            [vec![0xf0, length - 1], b[4..end].to_vec()].concat()
        );
        let MixedEventItem::Event(e) = &w.items[2] else {
            panic!()
        };
        let MixedEventKind::MidiController(c) = &e.event else {
            panic!()
        };
        assert!(!c.explicit_status);
        assert_eq!(c.status.offset, end + 1);
    }
}

#[test]
fn guarded_sysex120_complete_owned_recovery_order_routing_and_accounting() {
    for length in [9, 11] {
        let mut b = event(length, 0);
        b.extend([0, 0xb0, 91, 50, 0, 93, 60, 1, 0x90, 60, 100, 64, 10]);
        let source = recovery120::tests::fixture(&b, 4);
        let r = recovery120::recover_project120(&source)
            .unwrap()
            .remove(0)
            .result
            .unwrap();
        let t = &r.tracks()[0];
        assert_eq!(r.proof().tracks()[0].declared(), Some(4));
        assert_eq!(r.proof().tracks()[0].context_records(), 0);
        assert_eq!(t.controller_targets.len(), 2);
        assert!(t.controller_targets.iter().all(|p| p.resolved_channel == 5));
        let parsed = crate::reference_smf::midi_tracks(r.midi());
        assert_eq!(parsed[1][1], (0, 0xf0, b[4..length as usize + 3].to_vec()));
        assert_eq!(parsed[1][2], (0, 0xb4, vec![91, 50]));
        assert_eq!(parsed[1][3], (0, 0xb4, vec![93, 60]));
        assert_eq!(parsed[1][4], (1, 0x94, vec![60, 100]));
        for raw in [3, 5] {
            assert!(
                recovery120::recover_project120(&recovery120::tests::fixture(&b, raw))
                    .unwrap()
                    .remove(0)
                    .result
                    .is_err()
            );
        }
        let mut invalid = b.clone();
        invalid[length as usize + 4] = 0xb1;
        assert!(
            recovery120::recover_project120(&recovery120::tests::fixture(&invalid, 4))
                .unwrap()
                .remove(0)
                .result
                .is_err()
        );
    }
}

#[test]
fn guarded_sysex120_malformed_unsupported_truncated_and_overflow_refuse() {
    for length in [9, 11] {
        let mut valid = event(length, 1);
        let end = valid.len();
        valid.extend([2, 0xb0, 10, 127, 0, 10, 125]);
        for cut in 1..end + 4 {
            assert!(walk(&valid[..cut], 0).is_err(), "cut {cut}");
        }
        for (offset, value) in [
            (2, 7),
            (2, 10),
            (2, 0x89),
            (3, 1),
            (4, 128),
            (4, 0xf7),
            (end - 1, 0),
            (end - 1, 128),
            (end + 2, 128),
            (end + 3, 128),
        ] {
            let mut b = valid.clone();
            b[offset] = value;
            assert!(walk(&b, 0).is_err(), "{offset} {value}");
        }
        for successor in [0x90, 0xc0, 0xe0, 0xf0, 0xff, 10] {
            let mut b = valid.clone();
            b[end + 1] = successor;
            assert!(walk(&b, 0).is_err());
        }
        assert!(walk(&valid, u32::MAX).is_err());
        valid[0] = 0;
        assert!(walk(&valid, u32::MAX - 1).is_err());
        // A consecutive context is not an explicit Controller successor.
        let b = [event(length, 0), event(length, 0), vec![0, 0xb0, 10, 1]].concat();
        assert!(walk(&b, 0).is_err());
    }
    let old = [
        0, 0xf0, 7, 0, 0x41, 0x32, 0, 12, 93, 0xf7, 0, 0x90, 60, 100, 64, 1,
    ];
    let w = walk_bounded_mixed_events(
        &old,
        MixedEventBounds {
            event_range: 0..old.len(),
        },
        Default::default(),
    )
    .unwrap();
    let MixedEventItem::Event(e) = &w.items[0] else {
        panic!()
    };
    let MixedEventKind::SysEx(s) = &e.event else {
        panic!()
    };
    assert_eq!(s.framing, None);
    assert_eq!(s.payload.bytes, &old[3..10]);
    assert_eq!(
        walk_bounded_120_prefix_events(
            &old,
            MixedEventBounds {
                event_range: 0..old.len()
            },
            Default::default()
        )
        .unwrap(),
        w
    );
    let handoff = old_sysex_handoff(&w);
    assert_eq!(handoff, vec![0xf0, 7, 0, 0x41, 0x32, 0, 12, 93, 0xf7]);
    assert!(walk(&old, 0).is_err());
}

#[test]
#[ignore = "requires authentic SCHOOL PROJECTS and native minuet.mid"]
fn authentic_school_sysex120_witness_payload_successor_clock_and_source_count() {
    use sha2::{Digest, Sha256};
    let b = std::fs::read(std::env::var("PHOENIX_SCHOOL_ORIGINAL_SOURCE").unwrap()).unwrap();
    let reference = std::fs::read(std::env::var("PHOENIX_MINUET_REFERENCE_MIDI").unwrap()).unwrap();
    assert_eq!(
        format!("{:x}", Sha256::digest(&b)),
        "bfd4fa1208e2cd884ec51ccfb1131d5c02723d7c90acafc98bd89597e1a20331"
    );
    assert_eq!(
        format!("{:x}", Sha256::digest(&reference)),
        "641c29017f64f5c9fc285c5a5f9433db09c8953a03e63341d5b20c91827e49b8"
    );
    let root = crate::sequence_container::parse_root_record_stream(&b).unwrap();
    let project = crate::semantic_layout120::associate_semantic_project_120(&b).unwrap();
    let s = project
        .candidates
        .iter()
        .filter_map(|c| c.association.as_ref().ok())
        .find(|s| s.source_name.bytes == b"minuet.mid")
        .unwrap();
    let t = s
        .ordinary
        .iter()
        .find(|t| t.pair.primary.record_range.start == 214118)
        .unwrap();
    assert_eq!(t.slot.label.bytes, b"program chg");
    // Ownership comes from the independently reconciled 120 association.
    // This Multi witness does not expand the strict Patch-only Multi exporter.
    let bounds = crate::sequence_container::TrackRecordPair {
        pair_ordinal: t.local_pair_ordinal,
        primary: t.pair.primary.clone(),
        secondary: t.pair.secondary.clone(),
        candidate_event_start: t.pair.primary.payload.range.start + 14,
        event_containing_range: t.pair.primary.payload.range.start + 14
            ..t.pair.primary.payload.range.end,
    }
    .validated_event_bounds()
    .unwrap()
    .event_range;
    assert_eq!(bounds, 214137..214577);
    let w = walk_bounded_120_prefix_events(
        &b,
        MixedEventBounds {
            event_range: bounds.clone(),
        },
        Default::default(),
    )
    .unwrap();
    assert_eq!(w.logical_event_count(), 28);
    assert_eq!(
        u32::from_be_bytes(t.pair.primary.payload.bytes[2..6].try_into().unwrap()),
        28
    );
    let MixedEventItem::Event(e) = &w.items[0] else {
        panic!()
    };
    let MixedEventKind::SysEx(sx) = &e.event else {
        panic!()
    };
    assert_eq!(sx.representation_range, 214137..214151);
    assert_eq!(sx.framing.unwrap().offset, 214140);
    let association = t.slot.label.range.start - 33..t.slot.label.range.start;
    let lookup = crate::controller_target120::ControllerAssociation120::from_records(
        &root.records,
        association.clone(),
        &b[association.clone()],
    )
    .unwrap();
    let targets = lookup.collect(&w, 0).unwrap();
    assert_eq!(targets.len(), 14);
    let (mut events, _) =
        crate::bounded_multi_patch::native_gs_multi_handoff(&b, association, bounds).unwrap();
    events.insert(
        0,
        midi_export::DecodedExportEvent::from_sysex(e.position, 0, sx),
    );
    // Existing Multi handoff already applies the same association targets.
    for (event, target) in events
        .iter()
        .filter(|e| {
            matches!(
                e.kind,
                midi_export::DecodedExportEventKind::MidiController { .. }
            )
        })
        .zip(&targets)
    {
        let midi_export::DecodedExportEventKind::MidiController { channel, .. } = event.kind else {
            panic!()
        };
        assert_eq!(channel, target.resolved_channel);
    }
    let a = midi_export::adapt_owned120_track(
        &events,
        Some(midi_export::ChannelAssignment {
            channel: smf::MidiChannel::new(1).unwrap(),
            provenance: midi_export::ChannelAssignmentProvenance::ParsedRouting,
        }),
        midi_export::TimingPolicy::Identity480,
        midi_export::PatchPolicy::StrictKnownOnly,
    )
    .unwrap();
    let chunk = smf::serialize_named_musical_track_with_note_provenance(
        b"witness",
        &a.scheduled_events,
        &a.note_provenance,
    )
    .unwrap();
    let out = smf::serialize_format1(480, &[chunk]).unwrap();
    let actual = crate::reference_smf::midi_tracks(&out);
    let native = crate::reference_smf::midi_tracks(&reference);
    assert_eq!(
        actual[0]
            .iter()
            .filter(|e| e.1 != 3 && e.1 != 0x2f)
            .collect::<Vec<_>>(),
        native[10]
            .iter()
            .filter(|e| e.1 != 3 && e.1 != 4 && e.1 != 0x2f)
            .collect::<Vec<_>>()
    );
}

fn old_sysex_handoff(w: &MixedEventWalk<'_>) -> Vec<u8> {
    let MixedEventItem::Event(e) = &w.items[0] else {
        panic!()
    };
    let MixedEventKind::SysEx(s) = &e.event else {
        panic!()
    };
    let events = [midi_export::DecodedExportEvent::from_sysex(
        e.position, 0, s,
    )];
    let a = midi_export::adapt_track(
        &events,
        Some(midi_export::ChannelAssignment {
            channel: smf::MidiChannel::new(1).unwrap(),
            provenance: midi_export::ChannelAssignmentProvenance::Synthetic,
        }),
        midi_export::TimingPolicy::Identity480,
        midi_export::PatchPolicy::StrictKnownOnly,
    )
    .unwrap();
    smf::serialize_channel_message(&a.scheduled_events[0].message)
}
