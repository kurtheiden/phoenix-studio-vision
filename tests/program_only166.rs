#[path = "support/xform_application.rs"]
mod xform_application;
use phoenix::{
    bounded_sequence::{assemble_bounded_sequence, build_bounded_sequence_manifest as manifest},
    midi_export::*,
    program_only166::qualify_track as qualify,
    sequence_container::parse_project_166,
    smf::{serialize_channel_message, MidiChannel},
};
use sha2::{Digest, Sha256};
#[path = "support/reference_smf.rs"]
mod reference;
const CC: &[u8] = &[20, 255, 65, 5, 0, 1, 0, 7, 108];
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
fn composition() -> Vec<u8> {
    let name = b"Different name";
    let mut e = vec![
        0x83,
        0x4c,
        255,
        124,
        (name.len() + 15) as u8,
        0,
        0,
        36,
        0,
        36,
        name.len() as u8,
    ];
    e.extend(name);
    e.extend([
        3, b'I', b'5', b'5', 4, 255, 255, 255, 36, 0x8b, 0x20, 144, 70, 102, 90, 0xbb, 0x0e,
    ]);
    e
}
fn source() -> Vec<u8> {
    let mut e = CC.to_vec();
    e.extend(composition());
    fixture(&e, 3)
}
fn adapt(q: &phoenix::program_only166::ProgramOnlyPatch) -> ExportTrackResult {
    adapt_track(
        &q.decoded_export_events(),
        Some(ChannelAssignment {
            channel: MidiChannel::new(q.routing().midi_channel).unwrap(),
            provenance: ChannelAssignmentProvenance::ParsedRouting,
        }),
        TimingPolicy::Identity480,
        PatchPolicy::StrictKnownOnly,
    )
    .unwrap()
}
#[test]
fn separate_program_only_handoff_retains_bytes_ticks_ordinals_and_no_banks() {
    let b = source();
    let q = qualify(&b, 0, 0).unwrap();
    let p = q.provenance();
    assert_eq!(
        (
            p.previous_tick,
            p.patch_interval,
            p.patch_tick,
            p.note_interval,
            p.note_tick
        ),
        (20, 460, 480, 1440, 1920)
    );
    assert_eq!(p.logical_source_ordinal, 1);
    assert_eq!(p.raw_source_object_ordinal, 1);
    assert_eq!(p.declared_source_objects, 3);
    assert_eq!(&b[p.composition_range.clone()], p.composition_bytes);
    assert_eq!(&b[p.pre_name_range.clone()], p.pre_name_bytes);
    assert_eq!(&b[p.post_name_range.clone()], p.post_name_bytes);
    assert_eq!(q.controller_prefix().len(), 1);
    let a = adapt(&q);
    assert_eq!(
        (
            a.counts.program_changes,
            a.counts.notes,
            a.counts.generated_note_offs,
            a.counts.bank_select_msb,
            a.counts.bank_select_lsb
        ),
        (1, 1, 1, 0, 0)
    );
    assert!(a.untranslated_metadata.is_empty());
    assert_eq!(
        a.scheduled_events
            .iter()
            .map(|e| (e.absolute_tick, serialize_channel_message(&e.message)))
            .collect::<Vec<_>>(),
        [
            (480, vec![0xc3, 36]),
            (1920, vec![0x93, 70, 102]),
            (9486, vec![0x83, 70, 90])
        ]
    );
    let m = manifest(&b, 0, 480).unwrap();
    assert_eq!(m.tracks()[0].program_only_patches(), [q]);
    let midi = assemble_bounded_sequence(&m).unwrap();
    let tracks = reference::midi_tracks(&midi);
    assert_eq!(
        tracks[1]
            .iter()
            .filter(|(_, s, _)| *s >= 128 && *s < 240)
            .cloned()
            .collect::<Vec<_>>(),
        [
            (20, 0xb3, vec![7, 108]),
            (480, 0xc3, vec![36]),
            (1920, 0x93, vec![70, 102]),
            (9486, 0x83, vec![70, 90])
        ]
    );
    let old = phoenix::bounded_patch_translation::classify_bounded_cc0_patches(&b).unwrap();
    assert_eq!(
        old[0].result.as_ref().unwrap().patches[0]
            .result
            .as_ref()
            .unwrap_err(),
        &phoenix::bounded_patch_translation::Cc0PatchRefusal::NonzeroPosition
    );
}
#[test]
fn equal_tick_source_order_and_labels_are_not_program_values() {
    let mut b = source();
    let q = qualify(&b, 0, 0).unwrap();
    let p = q.provenance();
    let post = p.post_name_range.clone();
    b[post.start + 2..post.start + 4].copy_from_slice(b"99");
    let q = qualify(&b, 0, 0).unwrap();
    let p = q.provenance();
    let start = p.note_timing_range.start;
    let end = p.note_timing_range.end;
    // Preserve a two-byte valid VLQ to change only the interval to zero.
    b[start..end].copy_from_slice(&[0x80, 0]);
    let q = qualify(&b, 0, 0).unwrap();
    assert_eq!(q.provenance().note_tick, 480);
    let m = manifest(&b, 0, 480).unwrap();
    let midi = assemble_bounded_sequence(&m).unwrap();
    let tr = reference::midi_tracks(&midi);
    let at: Vec<_> = tr[1]
        .iter()
        .filter(|(t, s, _)| *t == 480 && *s >= 128 && *s < 240)
        .cloned()
        .collect();
    assert_eq!(at, [(480, 0xc3, vec![36]), (480, 0x93, vec![70, 102])]);
}
#[test]
fn wrappers_counts_routing_and_explicit_successor_mutations_refuse() {
    let b = source();
    let q = qualify(&b, 0, 0).unwrap();
    let p = q.provenance();
    let patch = p.patch_range.start;
    let post = p.post_name_range.start;
    let pre = p.pre_name_range.start;
    let note = p.note_range.start;
    for (at, value) in [
        (pre, 1),
        (pre + 1, 1),
        (pre + 2, 35),
        (pre + 3, 8),
        (pre + 4, 35),
        (post, 2),
        (post + 1, b'G'),
        (post + 2, b'x'),
        (post + 4, 5),
        (post + 5, 0),
        (post + 6, 80),
        (post + 7, 0),
        (patch + 4, 30),
        (patch + 10, 2),
        (post + 8, 128),
        (note, 0x91),
        (note + 1, 128),
        (note + 2, 128),
        (note + 3, 128),
    ] {
        let mut bad = b.clone();
        bad[at] = value;
        assert!(qualify(&bad, 0, 0).is_err(), "offset {at:x}");
    }
    let at = parse_project_166(&b).unwrap().sequences[0].track_pairs[0]
        .primary
        .payload
        .range
        .start;
    for count in [2u32, 4] {
        let mut bad = b.clone();
        bad[at + 2..at + 6].copy_from_slice(&count.to_be_bytes());
        assert!(qualify(&bad, 0, 0).is_err());
    }
    let mut bad = b.clone();
    bad[at + 14 + 5] = 0;
    assert!(qualify(&bad, 0, 0).is_err());
    let mut bad = b.clone();
    let range = parse_project_166(&bad).unwrap().sequences[0].track_pairs[0]
        .secondary
        .record_range
        .clone();
    bad.drain(at - 5..range.end);
    assert!(qualify(&bad, 0, 0).is_err());
    let mut e = CC.to_vec();
    e.extend(composition());
    e.pop();
    assert!(qualify(&fixture(&e, 3), 0, 0).is_err());
    let mut e = CC.to_vec();
    e.extend(composition());
    e.extend([0, 254]);
    assert!(qualify(&fixture(&e, 4), 0, 0).is_err());
}
#[test]
fn only_first_patch_after_ordinary_prefix_and_no_contexts() {
    let mut initial = CC.to_vec();
    initial.extend([0, 255, 96, 7, 87, 127, 0, 1, 2, 3, 4]);
    initial.extend(composition());
    assert!(qualify(&fixture(&initial, 4), 0, 0).is_err());
    assert!(qualify(&fixture(&composition(), 2), 0, 0).is_err());
    let mut e = vec![0, 144, 60, 100, 64, 12];
    e.extend(CC);
    e.extend(composition());
    assert!(qualify(&fixture(&e, 4), 0, 0).is_err());
    let mut e = CC.to_vec();
    let mut c = composition();
    c.truncate(c.len() - 9);
    assert!(qualify(&fixture(&[e.clone(), c].concat(), 2), 0, 0).is_err());
    let mut c = composition();
    let note = c.len() - 6;
    c.splice(note..note, [255, 96, 7, 87, 127, 0, 1, 1, 1, 1, 0]);
    e.extend(c);
    assert!(qualify(&fixture(&e, 4), 0, 0).is_err());
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
fn authenticated_composition_and_native_order_with_remaining_refusals() {
    let b = read(
        "CCA073 CONTROL",
        "e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132",
    );
    let native = read(
        "073 CTRL xForm.mid",
        "872272ce43c389474fdb408416ad2406fa9a63de5e1d8bb94e0f6bbced7a1241",
    );
    let q = qualify(&b, 0, 0).unwrap();
    let p = q.provenance();
    assert_eq!(p.composition_range, 0x767a..0x76a6);
    assert_eq!(p.patch_range, 0x767a..0x76a1);
    assert_eq!(p.note_range, 0x76a0..0x76a6);
    assert_eq!(q.routing().midi_channel, 15);
    assert_eq!((p.patch_tick, p.note_tick), (480, 1920));
    let a = adapt(&q);
    let events: Vec<_> = a
        .scheduled_events
        .iter()
        .map(|e| (e.absolute_tick, serialize_channel_message(&e.message)))
        .collect();
    assert_eq!(
        events,
        [
            (480, vec![0xce, 36]),
            (1920, vec![0x9e, 70, 102]),
            (9486, vec![0x8e, 70, 90])
        ]
    );
    let tr = reference::midi_tracks(&native);
    let track = tr
        .iter()
        .find(|t| t.iter().any(|(_, s, d)| *s == 3 && d == b"Track 1"))
        .unwrap();
    let expected: Vec<_> = track
        .iter()
        .filter(|(t, s, d)| {
            (*t == 480 && *s == 0xce)
                || (*t == 1920 && *s == 0x9e && d[0] == 70)
                || (*t == 9486 && *s == 0x8e && d[0] == 70)
        })
        .map(|(t, s, d)| (*t, [vec![*s], d.clone()].concat()))
        .collect();
    assert_eq!(events, expected);
    assert!(!track
        .iter()
        .any(|(_, s, d)| *s >> 4 == 11 && matches!(d[0], 0 | 32)));
    let simultaneous: Vec<_> = track
        .iter()
        .filter(|(t, s, _)| *t == 1920 && *s == 0x9e)
        .map(|(_, _, d)| d[0])
        .collect();
    assert_eq!(simultaneous, [70, 65, 73]);
    for pair in 1..13 {
        assert!(qualify(&b, 0, pair).is_err(), "pair {pair} broadened");
    }
    let err = manifest(&b, 0, 480).unwrap_err();
    assert_eq!(err.stage, "mute");
    assert_eq!(err.detail, "On");
    let mut app = phoenix::app_service::AppService::new();
    let response = app
        .inspect_project(phoenix::app_contract::InspectProjectRequest {
            contract_version: phoenix::app_contract::CONTRACT_VERSION,
            source_path: "/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/CCA073 CONTROL"
                .into(),
            diagnostics_level: phoenix::app_contract::DiagnosticsLevel::Summary,
        })
        .unwrap();
    let row = response
        .sequences
        .iter()
        .find(|s| s.display_name == "xForm")
        .unwrap();
    assert!(row.export_capability.is_none());
    xform_application::assert_capability(row, true);
}
