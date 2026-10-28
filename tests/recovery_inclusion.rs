use phoenix::{
    bounded_sequence::*, mixed_event::*, saved_mute::*, sequence_container::parse_project_166,
};
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

fn read(name: &str, hash: &str) -> Vec<u8> {
    let b = std::fs::read(format!(
        "/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/{name}"
    ))
    .unwrap();
    assert_eq!(format!("{:x}", Sha256::digest(&b)), hash);
    b
}
const SOURCE_HASH: &str = "e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132";
const NATIVE_HASH: &str = "872272ce43c389474fdb408416ad2406fa9a63de5e1d8bb94e0f6bbced7a1241";
fn control() -> Vec<u8> {
    read("CCA073 CONTROL", SOURCE_HASH)
}
fn events(t: &[(u32, u8, Vec<u8>)], normalize: bool) -> Vec<(u32, u8, Vec<u8>)> {
    t.iter()
        .filter(|(_, s, _)| *s >= 128)
        .map(|(tick, s, d)| {
            if normalize && *s >> 4 == 9 && d[1] == 0 {
                (*tick, 0x80 | (*s & 15), vec![d[0], 64])
            } else {
                (*tick, *s, d.clone())
            }
        })
        .collect()
}
#[test]
fn authentic_complete_xform_both_modes_and_native_reconciliation() {
    let b = control();
    let n = read("073 CTRL xForm.mid", NATIVE_HASH);
    let native = reference::midi_tracks(&n);
    for mode in [
        RecoveryInclusion::default(),
        RecoveryInclusion::ExcludeMuted,
    ] {
        let m = build_recovery_sequence_manifest(&b, 0, 480, mode).unwrap();
        assert_eq!(m.tracks().len(), 13);
        let project = parse_project_166(&b).unwrap();
        for (i, t) in m.tracks().iter().enumerate() {
            assert_eq!(t.source_ordinal(), i);
            assert_eq!(t.descriptor_ordinal(), i + 2);
            assert_eq!(
                t.primary_range(),
                project.sequences[0].track_pairs[i].primary.record_range
            );
            assert_eq!(
                t.secondary_range(),
                project.sequences[0].track_pairs[i].secondary.record_range
            );
            assert_eq!(t.routing().event_range, t.routing().consumed_range);
            assert_eq!(t.saved_mute().pair_ordinal, Some(i));
            if i < 12 {
                assert_eq!(t.saved_mute().state, SavedMuteState::Off);
                assert_eq!(m.disposition(i), Some(RecoveryTrackDisposition::Included));
            }
        }
        assert_eq!(m.inclusion(), mode);
        assert_eq!(m.sequence_range(), project.sequences[0].sequence_range);
        assert_eq!(m.conductor().tempo().mpqn(), 582524);
        let result = assemble_recovery_sequence(&m).unwrap();
        let generated = reference::midi_tracks(&result.smf_bytes);
        let included = if mode == RecoveryInclusion::IncludeMuted {
            13
        } else {
            12
        };
        assert_eq!(generated.len(), included + 1);
        assert_eq!(
            result.smf_bytes.len(),
            if mode == RecoveryInclusion::IncludeMuted {
                26450
            } else {
                25803
            }
        );
        assert_eq!(
            result.report.totals.notes,
            if mode == RecoveryInclusion::IncludeMuted {
                2375
            } else {
                2307
            }
        );
        assert_eq!(
            &result.smf_bytes[..14],
            &[
                77,
                84,
                104,
                100,
                0,
                0,
                0,
                6,
                0,
                1,
                0,
                (included + 1) as u8,
                1,
                224
            ]
        );
        assert_eq!(result.report.musical_track_count, included);
        assert_eq!(result.report.total_smf_track_count, included + 1);
        assert!(result.report.warnings.is_empty());
        assert!(result.report.untranslated_metadata.is_empty());
        let conductor = |t: &Vec<(u32, u8, Vec<u8>)>| {
            t.iter()
                .filter(|(_, s, _)| matches!(s, 3 | 81 | 88))
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(conductor(&generated[0]), conductor(&native[0]));
        for i in 1..=12 {
            assert_eq!(
                generated[i].iter().find(|(_, s, _)| *s == 3),
                native[i].iter().find(|(_, s, _)| *s == 3)
            );
            assert_eq!(
                events(&generated[i], false),
                events(&native[i], true),
                "track {i}"
            );
        }
        assert_eq!(
            generated
                .iter()
                .skip(1)
                .take(12)
                .map(|t| events(t, false).iter().filter(|(_, s, _)| *s < 240).count())
                .sum::<usize>(),
            6084
        );
        assert_eq!(
            generated
                .iter()
                .flatten()
                .filter(|(_, s, _)| *s == 240)
                .count(),
            2
        );
        // Native-only metadata: SMPTE Offset, Instrument Names and tail padding.
        assert_eq!(native[0].iter().filter(|(_, s, _)| *s == 84).count(), 1);
        assert!(generated
            .iter()
            .flatten()
            .all(|(_, s, _)| *s != 84 && *s != 4));
        assert!(native.iter().all(|t| t.last().unwrap().0 == 155520));
        assert_eq!(
            native.iter().flatten().filter(|(_, s, _)| *s == 4).count(),
            12
        );
        for t in &generated {
            let last_musical = t
                .iter()
                .filter(|(_, s, _)| *s >= 128)
                .map(|(tick, _, _)| *tick)
                .max()
                .unwrap_or(0);
            assert_eq!(t.last().unwrap().0, last_musical);
        }
        assert_eq!(
            generated[0].iter().find(|(_, s, _)| *s == 81).unwrap(),
            &(0, 81, vec![8, 227, 124])
        );
        assert_eq!(
            generated[0].iter().find(|(_, s, _)| *s == 88).unwrap(),
            &(0, 88, vec![4, 2, 24, 8])
        );
        let muted = &m.tracks()[12];
        assert_eq!(muted.saved_mute().state, SavedMuteState::On);
        assert_eq!(muted.saved_mute().candidate_offset, Some(0x7488));
        assert_eq!(
            muted.saved_mute().neighboring_guard,
            Some([0, 4, 0, 0, 4, 1, 0])
        );
        assert_eq!(
            muted.primary_range(),
            parse_project_166(&b).unwrap().sequences[0].track_pairs[12]
                .primary
                .record_range
        );
        assert_eq!(muted.routing().event_range, 0xdcc7..0xdec8);
        assert_eq!(muted.events().len(), 73);
        assert_eq!(
            m.disposition(12),
            Some(if mode == RecoveryInclusion::IncludeMuted {
                RecoveryTrackDisposition::Included
            } else {
                RecoveryTrackDisposition::ExcludedSavedMute
            })
        );
        if mode == RecoveryInclusion::IncludeMuted {
            assert_eq!(events(&generated[13], false), muted_source_messages(&b));
            assert_eq!(events(&generated[13], false).len(), 142);
            assert_eq!(
                generated[13].iter().find(|(_, s, _)| *s == 3).unwrap().2,
                b"Track 11 #2"
            );
        }
    }
    assert_eq!(
        build_bounded_sequence_manifest(&b, 0, 480)
            .unwrap_err()
            .stage,
        "mute"
    );
}

/// Independent MIDI-domain expectation from the established source walk. This
/// deliberately does not use the manifest's translated events or MIDI adapter.
fn muted_source_messages(b: &[u8]) -> Vec<(u32, u8, Vec<u8>)> {
    let project = parse_project_166(b).unwrap();
    let pair = &project.sequences[0].track_pairs[12];
    assert_eq!(
        u32::from_be_bytes(pair.primary.payload.bytes[2..6].try_into().unwrap()),
        76
    );
    let range = project.sequences[0]
        .validated_track_event_bounds(12)
        .unwrap()
        .event_range;
    let walk = walk_bounded_mixed_events(
        b,
        MixedEventBounds {
            event_range: range.clone(),
        },
        Default::default(),
    )
    .unwrap();
    assert_eq!(walk.consumed_range, range);
    assert_eq!(walk.logical_event_count(), 73);
    let mut expected = Vec::new();
    let (mut ordinal, mut notes, mut controllers, mut contexts) = (0u64, 0, 0, 0);
    for item in &walk.items {
        match item {
            MixedEventItem::Event(e) => {
                let values = match &e.event {
                    MixedEventKind::Note(n) => Some((
                        n.pitch.value,
                        n.attack_velocity.value,
                        n.release_velocity.value,
                        n.duration.value,
                    )),
                    MixedEventKind::ContextMediatedNote(n) => {
                        contexts += 1;
                        Some((
                            n.note.pitch.value,
                            n.note.attack_velocity.value,
                            n.note.release_velocity.value,
                            n.note.duration.value,
                        ))
                    }
                    MixedEventKind::Controller(c) => {
                        controllers += 1;
                        expected.push((
                            e.position,
                            ordinal * 2,
                            0xbd,
                            vec![c.controller_number.value, c.controller_value.value],
                        ));
                        None
                    }
                    _ => panic!("unexpected muted-track family"),
                };
                if let Some((key, attack, release, duration)) = values {
                    notes += 1;
                    expected.push((e.position, ordinal * 2, 0x9d, vec![key, attack]));
                    expected.push((
                        e.position + duration,
                        ordinal * 2 + 1,
                        0x8d,
                        vec![key, release],
                    ));
                }
            }
            MixedEventItem::Patch(p) => {
                let q = phoenix::standalone_bank166::qualify_track(b, 0, 12).unwrap();
                let proof = q.provenance();
                assert_eq!(proof.patch_range, 0xdcc7..0xdce5);
                assert_eq!(p.position, 240);
                assert_eq!(b[proof.program_offset], 63);
                expected.push((240, ordinal * 2, 0xbd, vec![0, 80]));
                expected.push((240, ordinal * 2, 0xcd, vec![63]));
            }
            _ => panic!("unexpected muted-track composition"),
        }
        ordinal += item.logical_event_count() as u64;
    }
    assert_eq!((notes, controllers, contexts), (68, 4, 3));
    assert_eq!(ordinal, 73);
    expected.sort_by_key(|(tick, ordinal, _, _)| (*tick, *ordinal));
    expected.into_iter().map(|(t, _, s, d)| (t, s, d)).collect()
}

const NOTE: &[u8] = &[0, 144, 60, 100, 64, 12];
fn mute_offset(b: &[u8]) -> usize {
    parse_project_166(b).unwrap().sequences[0].track_descriptors()[0].label_start - 39
}
#[test]
fn default_includes_saved_on_and_explicit_exclusion_keeps_complete_ledger() {
    let mut b = fixture(NOTE, 1);
    let at = mute_offset(&b);
    b[at] = 0x88;
    assert_eq!(
        RecoveryInclusion::default(),
        RecoveryInclusion::IncludeMuted
    );
    for (mode, count) in [
        (RecoveryInclusion::default(), 1),
        (RecoveryInclusion::ExcludeMuted, 0),
    ] {
        let m = build_recovery_sequence_manifest(&b, 0, 480, mode).unwrap();
        assert_eq!(m.tracks().len(), 1);
        assert_eq!(m.tracks()[0].events().len(), 1);
        assert_eq!(m.tracks()[0].saved_mute().state, SavedMuteState::On);
        let result = assemble_recovery_sequence(&m).unwrap();
        assert_eq!(result.report.musical_track_count, count);
        assert_eq!(reference::midi_tracks(&result.smf_bytes).len(), count + 1);
    }
    assert_eq!(
        build_bounded_sequence_manifest(&b, 0, 480)
            .unwrap_err()
            .stage,
        "mute"
    );
}
#[test]
fn unknown_mute_is_included_in_both_modes_and_never_relabelled_off() {
    for mutation in 0..2 {
        let mut b = fixture(NOTE, 1);
        let at = mute_offset(&b);
        if mutation == 0 {
            b[at] = 0x89;
        } else {
            b[at] = 0x88;
            b[at + 1] = 0x55;
        }
        for mode in [
            RecoveryInclusion::default(),
            RecoveryInclusion::ExcludeMuted,
        ] {
            let m = build_recovery_sequence_manifest(&b, 0, 480, mode).unwrap();
            assert!(matches!(
                m.tracks()[0].saved_mute().state,
                SavedMuteState::Unknown(_)
            ));
            assert_eq!(m.disposition(0), Some(RecoveryTrackDisposition::Included));
            assert_eq!(
                assemble_recovery_sequence(&m)
                    .unwrap()
                    .report
                    .musical_track_count,
                1
            );
        }
    }
}
#[test]
fn forged_detached_or_rebound_mute_evidence_cannot_drive_inclusion() {
    let mut b = fixture(NOTE, 1);
    let at = mute_offset(&b);
    let m = build_recovery_sequence_manifest(&b, 0, 480, RecoveryInclusion::ExcludeMuted).unwrap();
    let mut detached = m.tracks()[0].saved_mute().clone();
    detached.state = SavedMuteState::On;
    detached.candidate_offset = Some(at + 1);
    detached.pair_ordinal = Some(99);
    assert_ne!(&detached, m.tracks()[0].saved_mute());
    assert_eq!(m.disposition(0), Some(RecoveryTrackDisposition::Included));
    assert_eq!(
        assemble_recovery_sequence(&m)
            .unwrap()
            .report
            .musical_track_count,
        1
    );
    // A valid ON window at a different offset is not a bound candidate.
    b[at - 8..at].copy_from_slice(&[0x88, 0, 4, 0, 0, 4, 1, 0]);
    let fresh =
        build_recovery_sequence_manifest(&b, 0, 480, RecoveryInclusion::ExcludeMuted).unwrap();
    assert_eq!(
        fresh.disposition(0),
        Some(RecoveryTrackDisposition::Included)
    );
    // Immutable source revalidation, rather than detached caller evidence, changes disposition.
    b[at] = 0x88;
    let fresh =
        build_recovery_sequence_manifest(&b, 0, 480, RecoveryInclusion::ExcludeMuted).unwrap();
    assert_eq!(
        fresh.disposition(0),
        Some(RecoveryTrackDisposition::ExcludedSavedMute)
    );
    assert_eq!(m.disposition(0), Some(RecoveryTrackDisposition::Included));
}
#[test]
fn exclusions_do_not_hide_incomplete_accounting_translation_or_ordering() {
    let cases = [
        (vec![0, 144, 60, 100, 64, 12], 2), // declared source count mismatch
        (vec![0, 0xf1, 0], 1),              // unsupported source form
        (vec![0, 144, 60, 100, 64, 12, 0, 144, 60, 100, 64, 10], 2),
    ]; // reversed simultaneous endings
    for (events, declared) in cases {
        let mut b = fixture(&events, declared);
        let at = mute_offset(&b);
        b[at] = 0x88;
        for mode in [
            RecoveryInclusion::default(),
            RecoveryInclusion::ExcludeMuted,
        ] {
            assert!(build_recovery_sequence_manifest(&b, 0, 480, mode).is_err());
        }
    }
    let mut b = fixture(NOTE, 1);
    let p = parse_project_166(&b).unwrap();
    let start = p.sequences[0].meter_primary.payload.range.start;
    let tempo = p.sequences[0].tempo_secondary.payload.range.end - 4;
    b[start + 10] = 0; // opaque conductor context may vary safely
    b[tempo] ^= 1;
    assert_eq!(
        build_recovery_sequence_manifest(&b, 0, 480, RecoveryInclusion::ExcludeMuted)
            .unwrap_err()
            .stage,
        "conductor coverage"
    );
}
#[test]
fn unsupported_layout_ownership_and_division_cannot_gain_recovery_authority() {
    let b = fixture(NOTE, 1);
    assert_eq!(
        build_recovery_sequence_manifest(&b, 0, 96, RecoveryInclusion::default())
            .unwrap_err()
            .stage,
        "division"
    );
    assert!(build_recovery_sequence_manifest(
        &b[..b.len() - 1],
        0,
        480,
        RecoveryInclusion::default()
    )
    .is_err());
    let mut missing = b.clone();
    let p = parse_project_166(&b).unwrap();
    let pair = &p.sequences[0].track_pairs[0];
    missing.drain(pair.primary.record_range.start..pair.secondary.record_range.end);
    assert_eq!(
        build_recovery_sequence_manifest(&missing, 0, 480, RecoveryInclusion::ExcludeMuted)
            .unwrap_err()
            .stage,
        "ownership"
    );
    let mut changed = b.clone();
    let p = parse_project_166(&b).unwrap();
    let label = p.sequences[0].track_descriptors()[0].label_start;
    changed[label..label + 5].fill(b' ');
    assert!(
        build_recovery_sequence_manifest(&changed, 0, 480, RecoveryInclusion::ExcludeMuted)
            .is_err()
    );
    // Other Studio Vision layouts are not interpreted using a Descriptor166 mute slot.
    assert!(
        build_recovery_sequence_manifest(&[0; 120], 0, 480, RecoveryInclusion::default()).is_err()
    );
}

#[test]
fn recovery_inclusion_does_not_change_profiles_or_application_authority() {
    use phoenix::{
        app_contract::*, app_service::AppService, compatibility::TrackOutputDispositionExpectation,
    };
    let inspect = |app: &mut AppService| {
        app.inspect_project(InspectProjectRequest {
            contract_version: CONTRACT_VERSION,
            source_path: "/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/CCA073 CONTROL"
                .into(),
            diagnostics_level: DiagnosticsLevel::Summary,
        })
        .unwrap()
        .sequences
        .into_iter()
        .map(|s| {
            (
                s.display_name,
                s.readiness,
                s.export_capability,
                s.bounded_export_capability,
            )
        })
        .collect::<Vec<_>>()
    };
    let mut app = AppService::new();
    let before = inspect(&mut app);
    let b = control();
    for mode in [
        RecoveryInclusion::default(),
        RecoveryInclusion::ExcludeMuted,
    ] {
        let m = build_recovery_sequence_manifest(&b, 0, 480, mode).unwrap();
        assert!(assemble_recovery_sequence(&m).is_ok());
    }
    assert_eq!(inspect(&mut app), before);
    let xform = before.iter().find(|s| s.0 == "xForm").unwrap();
    assert_eq!(xform.1, Readiness::Ready);
    assert!(xform.2.is_none());
    assert_eq!(
        xform.3.as_ref().unwrap().contract_id,
        "generalized_sequence_recovery"
    );
    assert_eq!(
        before.iter().filter(|s| s.1 == Readiness::Ready).count(),
        11
    );
    let bells = phoenix::compatibility_profiles::bells_for_her_profile().unwrap();
    for descriptor in [3, 8] {
        assert!(matches!(
            bells.sequences[0]
                .track_expectations
                .iter()
                .find(|t| t.key.descriptor_ordinal == descriptor)
                .unwrap()
                .output,
            TrackOutputDispositionExpectation::Omitted(_)
        ));
    }
    let ode = phoenix::compatibility_profiles::ode_to_clarke_profile().unwrap();
    for descriptor in [2, 7] {
        assert!(matches!(
            ode.sequences[0]
                .track_expectations
                .iter()
                .find(|t| t.key.descriptor_ordinal == descriptor)
                .unwrap()
                .output,
            TrackOutputDispositionExpectation::Included(_)
        ));
    }
}
#[test]
fn authenticated_edit_conductor_still_refuses_in_both_recovery_modes() {
    let b = read(
        "CCA073 EDIT",
        "3f12c7bb9371d76dd4d4da081ccf347101cd5d4f187fd039a9562665d87d0c87",
    );
    for mode in [
        RecoveryInclusion::default(),
        RecoveryInclusion::ExcludeMuted,
    ] {
        assert_eq!(
            build_recovery_sequence_manifest(&b, 0, 480, mode)
                .unwrap_err()
                .stage,
            "conductor coverage"
        );
    }
}

fn empty_fixture() -> Vec<u8> {
    let mut b = fixture(&[], 0);
    let p = parse_project_166(&b).unwrap();
    let pair = &p.sequences[0].track_pairs[0];
    let tail = pair.validated_event_bounds().unwrap().tail_range;
    let secondary = pair.secondary.record_range.clone();
    b[tail].copy_from_slice(&[255, 255, 255, 127, 255, 47, 0]);
    let payload = [
        0, 1, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 4, 15, 255, 255, 255, 47,
        197, 255, 255, 255,
    ];
    let mut record = vec![41, 0, 0, 0, 31];
    record.extend(payload);
    b.splice(secondary, record);
    b
}

#[test]
fn privately_qualified_empty_track_keeps_identity_and_has_no_midi_channel() {
    let b = empty_fixture();
    let p = parse_project_166(&b).unwrap();
    for mode in [
        RecoveryInclusion::IncludeMuted,
        RecoveryInclusion::ExcludeMuted,
    ] {
        let m = build_recovery_sequence_manifest(&b, 0, 480, mode).unwrap();
        assert_eq!(m.source_track_count(), 1);
        assert!(m.tracks().is_empty());
        assert_eq!(
            m.disposition(0),
            Some(RecoveryTrackDisposition::AccountedEmpty)
        );
        assert_eq!(m.disposition(1), None);
        let e = &m.empty_tracks()[0];
        assert_eq!(e.source_ordinal(), 0);
        assert_eq!(e.descriptor_ordinal(), 2);
        assert_eq!(e.descriptor_range(), p.sequences[0].descriptors[2].range);
        assert_eq!(
            e.primary_range(),
            p.sequences[0].track_pairs[0].primary.record_range
        );
        assert_eq!(
            e.secondary_range(),
            p.sequences[0].track_pairs[0].secondary.record_range
        );
        assert!(e.event_range().is_empty());
        assert_eq!(e.label(), b"Other");
        let out = assemble_recovery_sequence(&m).unwrap();
        assert_eq!(reference::midi_tracks(&out.smf_bytes).len(), 1);
        assert!(out.report.tracks.is_empty());
        assert_eq!(out.report.totals.notes, 0);
    }
    // The legacy construction route deliberately retains its previous refusal.
    assert!(build_bounded_sequence_manifest(&b, 0, 480).is_err());
}

#[test]
fn empty_track_rejects_contradictions_hidden_objects_and_unproven_framing() {
    let b = empty_fixture();
    let p = parse_project_166(&b).unwrap();
    let pair = &p.sequences[0].track_pairs[0];
    let primary = pair.primary.payload.range.start;
    let secondary = pair.secondary.payload.range.start;
    let tail = pair.validated_event_bounds().unwrap().tail_range.start;
    let count = p.sequences[0].descriptor_count.offset;
    for offset in (primary..primary + 10)
        .chain(tail..tail + 7)
        .chain(secondary..secondary + 31)
        .chain(std::iter::once(count))
    {
        let mut forged = b.clone();
        forged[offset] ^= 1;
        assert!(
            build_recovery_sequence_manifest(&forged, 0, 480, RecoveryInclusion::IncludeMuted)
                .is_err(),
            "accepted mutation at {offset}"
        );
        assert!(
            build_recovery_sequence_manifest(&forged, 0, 480, RecoveryInclusion::ExcludeMuted)
                .is_err()
        );
    }
    // A syntactically valid Note cannot hide behind a zero-object declaration.
    let hidden = fixture(&[0, 144, 60, 100, 64, 12], 0);
    assert!(
        build_recovery_sequence_manifest(&hidden, 0, 480, RecoveryInclusion::IncludeMuted).is_err()
    );
    let mut truncated = b;
    truncated.truncate(secondary + 20);
    assert!(
        build_recovery_sequence_manifest(&truncated, 0, 480, RecoveryInclusion::IncludeMuted)
            .is_err()
    );
}

#[test]
fn authenticated_jurrasic_park_complete_recovery_and_independent_readback() {
    use phoenix::midi_export::*;
    use phoenix::smf::ChannelMessage::*;
    let b = control();
    let project = parse_project_166(&b).unwrap();

    for mode in [
        RecoveryInclusion::IncludeMuted,
        RecoveryInclusion::ExcludeMuted,
    ] {
        let m = build_recovery_sequence_manifest(&b, 13, 480, mode).unwrap();
        assert_eq!(m.source_track_count(), 7);
        assert_eq!(m.tracks().len(), 5);
        assert_eq!(
            m.empty_tracks()
                .iter()
                .map(|e| e.source_ordinal())
                .collect::<Vec<_>>(),
            vec![4, 6]
        );
        for e in m.empty_tracks() {
            let pair = &project.sequences[13].track_pairs[e.source_ordinal()];
            assert_eq!(e.primary_range(), pair.primary.record_range);
            assert_eq!(e.secondary_range(), pair.secondary.record_range);
            assert_eq!(
                m.disposition(e.source_ordinal()),
                Some(RecoveryTrackDisposition::AccountedEmpty)
            );
        }
        assert_eq!(
            m.disposition(0),
            Some(if mode == RecoveryInclusion::IncludeMuted {
                RecoveryTrackDisposition::Included
            } else {
                RecoveryTrackDisposition::ExcludedSavedMute
            })
        );
        let out = assemble_recovery_sequence(&m).unwrap();
        let readback = reference::midi_tracks(&out.smf_bytes);
        let included = m
            .tracks()
            .iter()
            .filter(|t| {
                m.disposition(t.source_ordinal()) == Some(RecoveryTrackDisposition::Included)
            })
            .collect::<Vec<_>>();
        let total = if mode == RecoveryInclusion::IncludeMuted {
            6
        } else {
            5
        };
        assert_eq!(
            &out.smf_bytes[..14],
            &[77, 84, 104, 100, 0, 0, 0, 6, 0, 1, 0, total, 1, 224]
        );
        assert_eq!(readback.len(), total as usize);
        assert_eq!(out.report.musical_track_count, included.len());
        let message_counts = readback[1..]
            .iter()
            .map(|t| events(t, false).len())
            .collect::<Vec<_>>();
        assert_eq!(
            message_counts,
            if mode == RecoveryInclusion::IncludeMuted {
                vec![94, 261, 57, 347, 36]
            } else {
                vec![261, 57, 347, 36]
            }
        );
        let tempo = readback[0].iter().find(|(_, s, _)| *s == 81).unwrap();
        assert_eq!(tempo.0, 0);
        let mpqn = m.conductor().tempo().mpqn();
        assert_eq!(tempo.2, mpqn.to_be_bytes()[1..]);
        let meter = readback[0].iter().find(|(_, s, _)| *s == 88).unwrap();
        let source_meter = m.conductor().meter();
        assert_eq!(meter.0, 0);
        assert_eq!(
            meter.2[..2],
            [
                source_meter.numerator.value,
                source_meter.denominator_exponent.value
            ]
        );
        for (index, t) in included.iter().enumerate() {
            let adapted = adapt_track(
                t.events(),
                Some(ChannelAssignment {
                    channel: phoenix::smf::MidiChannel::new(t.routing().midi_channel).unwrap(),
                    provenance: ChannelAssignmentProvenance::ParsedRouting,
                }),
                TimingPolicy::Identity480,
                PatchPolicy::StrictKnownOnly,
            )
            .unwrap();
            let mut scheduled = adapted.scheduled_events;
            scheduled.sort_by_key(|e| (e.absolute_tick, e.stable_ordinal));
            let expected = scheduled
                .iter()
                .map(|e| {
                    let (s, d) = match &e.message {
                        NoteOn {
                            channel,
                            key,
                            attack_velocity,
                        } => (
                            0x90 | (channel.get() - 1),
                            vec![key.get(), attack_velocity.get()],
                        ),
                        NoteOff {
                            channel,
                            key,
                            release_velocity,
                        } => (
                            0x80 | (channel.get() - 1),
                            vec![key.get(), release_velocity.get()],
                        ),
                        ControlChange {
                            channel,
                            controller,
                            value,
                        } => (
                            0xb0 | (channel.get() - 1),
                            vec![controller.get(), value.get()],
                        ),
                        ProgramChange { channel, program } => {
                            (0xc0 | (channel.get() - 1), vec![program.get()])
                        }
                        _ => panic!("unexpected message"),
                    };
                    (e.absolute_tick, s, d)
                })
                .collect::<Vec<_>>();
            assert_eq!(events(&readback[index + 1], false), expected);
            assert_eq!(
                readback[index + 1]
                    .iter()
                    .find(|(_, s, _)| *s == 3)
                    .unwrap()
                    .2,
                t.routing().label_bytes
            );
        }
        if mode == RecoveryInclusion::IncludeMuted {
            assert_eq!(out.report.totals.controllers, 210);
            assert_eq!(out.report.totals.notes, 288);
        }
        println!(
            "dispositions {:?}",
            (0..7).map(|i| m.disposition(i)).collect::<Vec<_>>()
        );
        println!(
            "Jurrasic {mode:?}: bytes={} totals={:?}",
            out.smf_bytes.len(),
            out.report.totals
        );
    }
    let mut app = phoenix::app_service::AppService::new();
    let inspected = app
        .inspect_project(phoenix::app_contract::InspectProjectRequest {
            contract_version: 1,
            source_path: "/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/CCA073 CONTROL"
                .into(),
            diagnostics_level: phoenix::app_contract::DiagnosticsLevel::Summary,
        })
        .unwrap();
    assert_eq!(
        inspected.sequences[13].readiness,
        phoenix::app_contract::Readiness::Ready
    );
    assert_eq!(
        inspected.sequences[13]
            .bounded_export_capability
            .as_ref()
            .unwrap()
            .contract_id,
        "generalized_sequence_recovery"
    );
}

#[test]
fn whole_project_empty_track_impact_keeps_other_refusals_and_authority() {
    let b = control();
    let expected = [
        None,
        Some(("Patch", "PayloadLength")),
        None,
        Some(("conductor prelude", "UnmatchedOrAmbiguous")),
        Some(("conductor prelude", "UnmatchedOrAmbiguous")),
        None,
        Some(("source ordering", "AmbiguousSourceOrder")),
        Some(("conductor prelude", "UnmatchedOrAmbiguous")),
        Some(("ownership", "descriptor_count: 11, pair_count: 10")),
        None,
        None,
        Some(("Patch", "PayloadLength")),
        Some(("Patch", "unsupported standalone Patch")),
        None,
        Some(("Patch", "PayloadLength")),
        None,
        None,
        None,
    ];
    let project = parse_project_166(&b).unwrap();
    assert_eq!(project.sequences.len(), 18);
    for mode in [
        RecoveryInclusion::IncludeMuted,
        RecoveryInclusion::ExcludeMuted,
    ] {
        let mut passed = Vec::new();
        for (ordinal, expected) in expected.iter().enumerate() {
            match (
                build_recovery_sequence_manifest(&b, ordinal, 480, mode),
                expected,
            ) {
                (Ok(m), None) => {
                    assert_eq!(
                        m.source_track_count(),
                        project.sequences[ordinal].track_descriptors().len()
                    );
                    let result = assemble_recovery_sequence(&m).unwrap();
                    assert_eq!(
                        reference::midi_tracks(&result.smf_bytes).len(),
                        result.report.total_smf_track_count
                    );
                    passed.push(ordinal);
                }
                (Err(e), Some((stage, detail))) => {
                    assert_eq!(e.stage, *stage, "ordinal {ordinal}");
                    assert!(e.detail.contains(detail), "ordinal {ordinal}: {e}");
                }
                (result, expected) => panic!("ordinal {ordinal}: {result:?} expected {expected:?}"),
            }
        }
        assert_eq!(passed, vec![0, 2, 5, 9, 10, 13, 15, 16, 17]);
    }
}
