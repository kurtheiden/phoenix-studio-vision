use phoenix::bounded_patch_translation::{
    qualify_descriptor166_bank_patch as qualify, BankPatchRefusal,
};
use phoenix::midi_export::{
    adapt_track, ChannelAssignment, ChannelAssignmentProvenance, PatchPolicy, TimingPolicy,
};
use phoenix::mixed_event::{walk_bounded_mixed_events, MixedEventBounds, MixedEventItem};
use phoenix::sequence_container::parse_project_166;
use phoenix::smf::{serialize_channel_message, MidiChannel};
use sha2::{Digest, Sha256};
#[path = "support/reference_smf.rs"]
mod reference_smf;
const NOTE: &[u8] = &[0, 0x90, 60, 100, 64, 12];
fn record(b: &mut Vec<u8>, tag: u8, p: &[u8]) {
    b.push(tag);
    b.extend((p.len() as u32).to_be_bytes());
    b.extend(p);
}
fn synthetic(events: &[u8], count: u32) -> Vec<u8> {
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
    p[2..6].copy_from_slice(&count.to_be_bytes());
    p.extend(events);
    p.extend([0xff, 0, 0, 0, 0xff, 0x2f, 0]);
    record(&mut b, 2, &p);
    record(&mut b, 0x29, &[]);
    record(&mut b, 0, &[]);
    b
}

// Programs/labels deliberately differ from the native examples. Labels do not
// supply Program values, and absolute source offsets are never predicates.
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
fn messages(b: &[u8]) -> Vec<(u32, Vec<u8>)> {
    let q = qualify(b, 0, 0).unwrap();
    let a = adapt_track(
        &[q.decoded_export_event()],
        Some(ChannelAssignment {
            channel: MidiChannel::new(4).unwrap(),
            provenance: ChannelAssignmentProvenance::ParsedRouting,
        }),
        TimingPolicy::Identity480,
        PatchPolicy::StrictKnownOnly,
    )
    .unwrap();
    a.scheduled_events
        .iter()
        .map(|e| (e.absolute_tick, serialize_channel_message(&e.message)))
        .collect()
}
#[test]
fn structural_cc0_and_cc0_cc32_expansion_order_and_timing() {
    for family in 0..4 {
        for standalone in [false, true] {
            let b = synthetic(
                &patch(family, 37, standalone),
                if standalone { 3 } else { 2 },
            );
            let msb = if family == 3 { 81 } else { 80 };
            let mut expected = vec![(37, vec![0xb3, 0, msb])];
            if family >= 2 {
                expected.push((37, vec![0xb3, 32, if family == 3 { 1 } else { 0 }]));
            }
            expected.push((37, vec![0xc3, 42]));
            assert_eq!(messages(&b), expected);
        }
    }
}
#[test]
fn provenance_and_source_accounting_survive_expansion() {
    let mut e = vec![9, 255, 65, 5, 0, 1, 0, 7, 90];
    e.extend(patch(2, 37, false));
    let b = synthetic(&e, 3);
    let q = qualify(&b, 0, 0).unwrap();
    let event = q.decoded_export_event();
    assert_eq!(event.absolute_position, 46);
    assert_eq!(event.source_ordinal, 1);
    let MixedEventItem::PatchToNote(p) = q.source() else {
        panic!()
    };
    assert_eq!(
        event.source_range,
        Some(p.patch.representation_range.clone())
    );
    assert_eq!(
        p.patch.pre_name_context.bytes,
        &b[p.patch.pre_name_context.range.clone()]
    );
    assert_eq!(p.first_note_position, 46);
    assert_eq!(q.routing().midi_channel, 4);
    let mut bad = b.clone();
    let project = parse_project_166(&b).unwrap();
    let at = project.sequences[0].track_pairs[0]
        .primary
        .payload
        .range
        .start
        + 2;
    bad[at..at + 4].copy_from_slice(&4u32.to_be_bytes());
    assert_eq!(
        qualify(&bad, 0, 0).unwrap_err(),
        BankPatchRefusal::ObjectCount
    );
}
#[test]
fn unsupported_wrappers_and_bank_neighbors_refuse() {
    for family in 0..4 {
        let e = patch(family, 37, false);
        // Header, pre-name flags/Program agreement, post-name marker and bank tail.
        for (at, value) in [
            (4, 1),
            (8, 43),
            (e.len() - NOTE.len() - 5, 5),
            (e.len() - NOTE.len() - 3, 82),
        ] {
            let mut bad = e.clone();
            bad[at] = value;
            assert!(qualify(&synthetic(&bad, 2), 0, 0).is_err(), "{family}/{at}");
        }
        let mut trailing = e.clone();
        trailing.push(0x7f);
        assert!(qualify(&synthetic(&trailing, 2), 0, 0).is_err());
        let mut malformed = e.clone();
        malformed[3] = 255;
        assert!(qualify(&synthetic(&malformed, 2), 0, 0).is_err());
    }
}
#[test]
fn earlier_notes_ambiguous_targets_and_ff60_authority_stay_refused() {
    let mut e = NOTE.to_vec();
    e.extend(patch(2, 37, false));
    assert_eq!(
        qualify(&synthetic(&e, 3), 0, 0).unwrap_err(),
        BankPatchRefusal::UnsupportedPlacement
    );
    let e = patch(2, 37, false);
    let mut b = synthetic(&e, 2);
    let p = parse_project_166(&b).unwrap();
    let d = &p.sequences[0].descriptors[2];
    let at = d.label_start - 31;
    b[at] = 99;
    assert!(matches!(
        qualify(&b, 0, 0),
        Err(BankPatchRefusal::Routing(_))
    ));
    let mut e = vec![0, 255, 96, 7, 35, 96, 0, 1, 2, 3, 4];
    e.extend(patch(2, 37, false));
    assert!(qualify(&synthetic(&e, 3), 0, 0).is_err());
}
#[test]
fn bounded_context_successor_retains_timing_and_context() {
    let mut e = patch(1, 37, false);
    e.truncate(e.len() - NOTE.len());
    e.extend([4, 255, 96, 7, 87, 127, 0, 1, 2, 3, 4, 5]);
    e.extend(NOTE.iter().skip(1));
    let b = synthetic(&e, 3);
    let q = qualify(&b, 0, 0).unwrap();
    let MixedEventItem::PatchToNote(p) = q.source() else {
        panic!()
    };
    assert_eq!(p.patch_position, 37);
    assert_eq!(p.first_note_position, 46);
    assert!(p.context.is_some());
    assert_eq!(messages(&b)[0].0, 37);
}
#[test]
fn production_classifier_and_manifest_authority_are_unchanged() {
    let b = synthetic(&patch(2, 37, false), 2);
    assert!(qualify(&b, 0, 0).is_ok());
    let rows = phoenix::bounded_patch_translation::classify_bounded_cc0_patches(&b).unwrap();
    assert!(rows[0].result.as_ref().unwrap().patches[0].result.is_err());
    assert!(phoenix::bounded_sequence::build_bounded_sequence_manifest(&b, 0, 480).is_err());
}
#[test]
fn authenticated_eight_patch_neighborhoods_match_native_when_available() {
    let path="/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline";
    let reference="/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/MIDI Exports from SVP/xForm Multitrack";
    let (Ok(b), Ok(m)) = (std::fs::read(path), std::fs::read(reference)) else {
        return;
    };
    assert_eq!(
        format!("{:x}", Sha256::digest(&b)),
        "e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132"
    );
    assert_eq!(
        format!("{:x}", Sha256::digest(&m)),
        "872272ce43c389474fdb408416ad2406fa9a63de5e1d8bb94e0f6bbced7a1241"
    );
    let native = reference_smf::midi_tracks(&m);
    let project = parse_project_166(&b).unwrap();
    let mut banks = 0;
    for (pair, index, channel, tick) in [
        (1, 2, 1, 60),
        (3, 4, 9, 80),
        (5, 6, 4, 100),
        (6, 7, 10, 120),
        (7, 8, 12, 140),
        (9, 10, 2, 960),
        (10, 11, 11, 0),
        (11, 12, 7, 480),
    ] {
        let q = qualify(&b, 0, pair).unwrap();
        assert_eq!(q.routing().midi_channel, channel);
        let ev = q.decoded_export_event();
        assert_eq!(ev.absolute_position, tick);
        let a = adapt_track(
            &[ev],
            Some(ChannelAssignment {
                channel: MidiChannel::new(channel).unwrap(),
                provenance: ChannelAssignmentProvenance::ParsedRouting,
            }),
            TimingPolicy::Identity480,
            PatchPolicy::StrictKnownOnly,
        )
        .unwrap();
        let expected: Vec<_> = a
            .scheduled_events
            .iter()
            .map(|e| {
                let wire = serialize_channel_message(&e.message);
                (e.absolute_tick, wire[0], wire[1..].to_vec())
            })
            .collect();
        let actual: Vec<_> = native[index]
            .iter()
            .filter(|(t, s, d)| {
                *t == tick && (*s >> 4 == 12 || (*s >> 4 == 11 && matches!(d[0], 0 | 32)))
            })
            .cloned()
            .collect();
        assert_eq!(expected, actual);
        banks += expected.len() - 1;
        let bounds = project.sequences[0]
            .validated_track_event_bounds(pair)
            .unwrap()
            .event_range;
        let walk = walk_bounded_mixed_events(
            &b,
            MixedEventBounds {
                event_range: bounds.clone(),
            },
            Default::default(),
        )
        .unwrap();
        assert_eq!(walk.consumed_range, bounds);
        assert!(walk.items.contains(q.source()));
    }
    assert_eq!(banks, 13);
    assert!(matches!(
        qualify(&b, 0, 8),
        Err(BankPatchRefusal::UnsupportedPlacement)
    ));
    assert_eq!(
        phoenix::bounded_sequence::build_bounded_sequence_manifest(&b, 0, 480)
            .unwrap_err()
            .stage,
        "translation"
    );
    assert_eq!(
        format!("{:x}", Sha256::digest(std::fs::read(path).unwrap())),
        format!("{:x}", Sha256::digest(&b))
    );
}
