use phoenix::app_contract::{DiagnosticsLevel, InspectProjectRequest, Readiness, CONTRACT_VERSION};
use phoenix::app_service::AppService;
use phoenix::compatibility::TrackOutputDispositionExpectation;
use phoenix::compatibility_profiles::{bells_for_her_profile, ode_to_clarke_profile};
use phoenix::saved_mute::{collect_saved_mute_evidence, SavedMuteState, SavedMuteUnknown};
use phoenix::sequence_container::parse_project_166;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

const NOTE: &[u8] = &[0, 0x90, 60, 64, 32, 1];
const GUARD: [u8; 7] = [0, 4, 0, 0, 4, 1, 0];
const GUARD_04: [u8; 7] = [4, 4, 0, 0, 4, 1, 0];
const DESCRIPTOR: usize = 8 + 208 + 2 * 166;
const CANDIDATE: usize = DESCRIPTOR - 24;
const ROOT: &str = "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments";
const BASE: &str = "Experiment 007 - Untouched Baseline/newest STUFF baseline";

fn record(bytes: &mut Vec<u8>, kind: u8, payload: &[u8]) {
    bytes.push(kind);
    bytes.extend_from_slice(&(payload.len() as u32).to_be_bytes());
    bytes.extend_from_slice(payload);
}

fn fixture(value: u8, events: &[u8], pairs: usize) -> Vec<u8> {
    let mut bytes = vec![0; 8];
    let mut preamble = vec![0; 3 * 166 + 172];
    preamble[0] = 3;
    record(&mut bytes, 1, &preamble);
    let mut name = vec![0; 11];
    name.extend_from_slice(&[1, b'S']);
    record(&mut bytes, 7, &name);
    bytes[DESCRIPTOR + 15..DESCRIPTOR + 17].copy_from_slice(b"T\0");
    bytes[CANDIDATE] = value;
    bytes[CANDIDATE + 1..CANDIDATE + 8].copy_from_slice(&GUARD);
    record(&mut bytes, 2, &[0; 22]);
    record(&mut bytes, 41, &[]);
    record(&mut bytes, 2, &[0; 21]);
    record(&mut bytes, 41, &[]);
    let mut payload = vec![0; 14];
    payload.extend_from_slice(events);
    payload.extend_from_slice(&[255, 0, 0, 0, 255, 47, 0]);
    for _ in 0..pairs {
        record(&mut bytes, 2, &payload);
        record(&mut bytes, 41, &[]);
    }
    record(&mut bytes, 0, &[]);
    bytes
}

fn state(bytes: &[u8]) -> SavedMuteState {
    collect_saved_mute_evidence(bytes).unwrap()[0].state
}

fn guarded_fixture(value: u8, events: &[u8], pairs: usize, guard: [u8; 7]) -> Vec<u8> {
    let mut bytes = fixture(value, events, pairs);
    bytes[CANDIDATE + 1..CANDIDATE + 8].copy_from_slice(&guard);
    bytes
}

#[test]
fn exact_whitelist_and_provenance_not_arbitrary_bit_mask() {
    for value in 0..=255 {
        let bytes = fixture(value, NOTE, 1);
        let rows = collect_saved_mute_evidence(&bytes).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].candidate_offset, Some(CANDIDATE));
        assert_eq!(rows[0].candidate_value, Some(value));
        assert_eq!(rows[0].neighboring_guard, Some(GUARD));
        assert_eq!(rows[0].pair_ordinal, Some(0));
        assert_eq!(rows[0].descriptor_ordinal, 2);
        assert_eq!(
            rows[0].state,
            match value {
                0x80 => SavedMuteState::Off,
                0x88 => SavedMuteState::On,
                _ => SavedMuteState::Unknown(SavedMuteUnknown::UnsupportedValue),
            }
        );
    }
}

#[test]
fn every_guard_byte_must_match_for_both_states() {
    for guard in [GUARD, GUARD_04] {
        for value in [0x80, 0x88] {
            for index in 0..guard.len() {
                for other in 0..=255 {
                    let mut changed = guard;
                    changed[index] = other;
                    if changed == GUARD || changed == GUARD_04 {
                        continue;
                    }
                    let bytes = guarded_fixture(value, NOTE, 1, changed);
                    assert_eq!(
                        state(&bytes),
                        SavedMuteState::Unknown(SavedMuteUnknown::GuardMismatch)
                    );
                }
            }
        }
    }
}

#[test]
fn exact_04_guard_state_whitelist_and_provenance() {
    for value in 0..=255 {
        let bytes = guarded_fixture(value, NOTE, 1, GUARD_04);
        let rows = collect_saved_mute_evidence(&bytes).unwrap();
        assert_eq!(rows[0].candidate_offset, Some(CANDIDATE));
        assert_eq!(rows[0].candidate_value, Some(value));
        assert_eq!(rows[0].neighboring_guard, Some(GUARD_04));
        assert_eq!(rows[0].pair_ordinal, Some(0));
        assert_eq!(rows[0].descriptor_ordinal, 2);
        assert_eq!(
            rows[0].event_range,
            collect_saved_mute_evidence(&fixture(value, NOTE, 1)).unwrap()[0].event_range
        );
        assert_eq!(
            rows[0].state,
            match value {
                0x80 => SavedMuteState::Off,
                0x88 => SavedMuteState::On,
                _ => SavedMuteState::Unknown(SavedMuteUnknown::UnsupportedValue),
            }
        );
    }
}

#[test]
fn exact_04_guard_does_not_bypass_binding_bounds_or_complete_walk() {
    for value in [0x80, 0x88] {
        assert_eq!(
            state(&guarded_fixture(value, NOTE, 0, GUARD_04)),
            SavedMuteState::Unknown(SavedMuteUnknown::UnresolvedBinding)
        );
        assert_eq!(
            state(&guarded_fixture(value, &[], 1, GUARD_04)),
            SavedMuteState::Unknown(SavedMuteUnknown::EmptyEvents)
        );
        let mut blank = guarded_fixture(value, NOTE, 1, GUARD_04);
        blank[DESCRIPTOR + 15] = 0;
        assert_eq!(
            state(&blank),
            SavedMuteState::Unknown(SavedMuteUnknown::BlankOrUnboundedLabel)
        );
        for cut in 1..NOTE.len() {
            assert_eq!(
                state(&guarded_fixture(value, &NOTE[..cut], 1, GUARD_04)),
                SavedMuteState::Unknown(SavedMuteUnknown::IncompleteEventWalk)
            );
        }
        let mut bytes = guarded_fixture(value, NOTE, 1, GUARD_04);
        let end = parse_project_166(&bytes).unwrap().sequences[0].track_pairs[0]
            .primary
            .record_range
            .end;
        bytes[end - 7] = 0;
        assert_eq!(
            state(&bytes),
            SavedMuteState::Unknown(SavedMuteUnknown::InvalidEventBounds)
        );
    }
}

#[test]
fn blank_unterminated_and_unbound_descriptors_are_unknown() {
    let mut bytes = fixture(0x88, NOTE, 1);
    bytes[DESCRIPTOR + 15] = 0;
    assert_eq!(
        state(&bytes),
        SavedMuteState::Unknown(SavedMuteUnknown::BlankOrUnboundedLabel)
    );
    bytes[DESCRIPTOR + 15] = b' ';
    assert_eq!(
        state(&bytes),
        SavedMuteState::Unknown(SavedMuteUnknown::BlankOrUnboundedLabel)
    );
    // Equal-count binding is mandatory even if label and candidate look valid.
    assert_eq!(
        state(&fixture(0x88, NOTE, 0)),
        SavedMuteState::Unknown(SavedMuteUnknown::UnresolvedBinding)
    );
    // A missing terminator is bounded by the parser's descriptor slice.
    let mut bytes = fixture(0x88, NOTE, 1);
    bytes[DESCRIPTOR + 15..DESCRIPTOR + 166].fill(b'x');
    // This also damages the structural name record: fail at framing, not ON.
    assert!(collect_saved_mute_evidence(&bytes).is_err());
}

#[test]
fn empty_invalid_bounds_and_incomplete_walks_never_decode() {
    for value in [0x80, 0x88] {
        assert_eq!(
            state(&fixture(value, &[], 1)),
            SavedMuteState::Unknown(SavedMuteUnknown::EmptyEvents)
        );
        for cut in 1..NOTE.len() {
            assert_eq!(
                state(&fixture(value, &NOTE[..cut], 1)),
                SavedMuteState::Unknown(SavedMuteUnknown::IncompleteEventWalk)
            );
        }
        let mut events = NOTE.to_vec();
        events.push(0x81); // Valid first event followed by incomplete timing.
        assert_eq!(
            state(&fixture(value, &events, 1)),
            SavedMuteState::Unknown(SavedMuteUnknown::IncompleteEventWalk)
        );
        let mut bytes = fixture(value, NOTE, 1);
        let end = parse_project_166(&bytes).unwrap().sequences[0].track_pairs[0]
            .primary
            .record_range
            .end;
        bytes[end - 7] = 0;
        assert_eq!(
            state(&bytes),
            SavedMuteState::Unknown(SavedMuteUnknown::InvalidEventBounds)
        );
    }
}

#[test]
fn truncation_and_oversized_lengths_are_safe_and_never_supported() {
    let bytes = fixture(0x88, NOTE, 1);
    for cut in 0..bytes.len() {
        if let Ok(rows) = collect_saved_mute_evidence(&bytes[..cut]) {
            assert!(rows
                .iter()
                .all(|row| matches!(row.state, SavedMuteState::Unknown(_))));
        }
    }
    let mut bytes = bytes;
    bytes[9..13].fill(255);
    assert!(collect_saved_mute_evidence(&bytes).is_err());
}

fn authentic(relative: &str, hash: &str) -> Option<Vec<u8>> {
    let path = Path::new(ROOT).join(relative);
    if !path.exists() {
        eprintln!("optional authentic fixture absent: {}", path.display());
        return None;
    }
    let bytes = fs::read(path).unwrap();
    assert_eq!(format!("{:x}", Sha256::digest(&bytes)), hash);
    Some(bytes)
}

#[test]
fn authentic_controlled_transitions_and_relocation() {
    let fixtures = [
        (
            "Experiment 035 - Track 1 Mute On to Off:Returned from MacOS9/EXP35 CTRL",
            "b1c82a7085f98f054f7320e2800aba523447c3c987c1aa983865518b7a7dd368",
            0x2f173,
            SavedMuteState::On,
            0x2f4b1,
            SavedMuteState::On,
        ),
        (
            "Experiment 035 - Track 1 Mute On to Off:Returned from MacOS9/EXP35 EDIT",
            "7b6d507a745441727312f93d36e891221cc06a051467c5f9d9855529c033fb94",
            0x2f06c,
            SavedMuteState::Off,
            0x2f3aa,
            SavedMuteState::On,
        ),
        (
            "Experiment 036 - Track 3 Mute On to Off:Returned from MacOS9/EXP36 EDIT",
            "f90ad9087b674eb17c0f8764ae479df5e98d1dbc111af1067109c43e04baccd1",
            0x2eb56,
            SavedMuteState::Off,
            0x2ee94,
            SavedMuteState::Off,
        ),
    ];
    for (path, hash, c1, s1, c3, s3) in fixtures {
        let Some(bytes) = authentic(path, hash) else {
            continue;
        };
        let rows = collect_saved_mute_evidence(&bytes).unwrap();
        for (ordinal, offset, state) in [(2, c1, s1), (7, c3, s3)] {
            let row = rows
                .iter()
                .find(|row| row.sequence_ordinal == 14 && row.descriptor_ordinal == ordinal)
                .unwrap();
            assert_eq!(row.candidate_offset, Some(offset));
            assert_eq!(row.state, state);
        }
    }
}

#[test]
fn authentic_pm_r10_04_guard_transition_preserves_owned_music_and_routing() {
    let shared = Path::new("/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared");
    let control_path = shared.join("PMR03 CTRL");
    let edit_path = shared.join("PMR10 EDIT");
    if !control_path.is_file() || !edit_path.is_file() {
        eprintln!("optional authentic PM-R10 fixtures absent");
        return;
    }
    let control = fs::read(control_path).unwrap();
    let edit = fs::read(edit_path).unwrap();
    for (bytes, hash) in [
        (
            &control,
            "e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132",
        ),
        (
            &edit,
            "681a0446c074d87273cee70cf80829984084effcc8d0fdcb200015fa3af03ab8",
        ),
    ] {
        assert_eq!(bytes.len(), 211468);
        assert_eq!(format!("{:x}", Sha256::digest(bytes)), hash);
    }
    let a = collect_saved_mute_evidence(&control).unwrap();
    let b = collect_saved_mute_evidence(&edit).unwrap();
    assert_eq!(a.len(), 133);
    assert_eq!(a.len(), b.len());
    for (a, b) in a.iter().zip(&b) {
        assert_eq!(a.sequence_ordinal, b.sequence_ordinal);
        assert_eq!(a.descriptor_ordinal, b.descriptor_ordinal);
        assert_eq!(a.descriptor_range, b.descriptor_range);
        assert_eq!(a.label_bytes, b.label_bytes);
        assert_eq!(a.pair_ordinal, b.pair_ordinal);
        assert_eq!(a.event_range, b.event_range);
        assert_eq!(a.neighboring_guard, b.neighboring_guard);
        if (a.sequence_ordinal, a.descriptor_ordinal) == (14, 10) {
            assert_eq!(a.candidate_offset, Some(0x2f6a3));
            assert_eq!(b.candidate_offset, a.candidate_offset);
            assert_eq!(a.neighboring_guard, Some(GUARD_04));
            assert_eq!(
                (a.candidate_value, b.candidate_value),
                (Some(0x80), Some(0x88))
            );
            assert_eq!(
                (a.state, b.state),
                (SavedMuteState::Off, SavedMuteState::On)
            );
        } else {
            assert_eq!(a.state, b.state);
        }
    }
    let pa = parse_project_166(&control).unwrap();
    let pb = parse_project_166(&edit).unwrap();
    assert_eq!(pa.sequences.len(), pb.sequences.len());
    for (a, b) in pa.sequences.iter().zip(&pb.sequences) {
        assert_eq!(a.sequence_range, b.sequence_range);
        assert_eq!(a.sequence_name, b.sequence_name);
        assert_eq!(a.track_associations, b.track_associations);
    }
    let a = phoenix::sequence_container::parse_root_record_stream(&control).unwrap();
    let b = phoenix::sequence_container::parse_root_record_stream(&edit).unwrap();
    assert_eq!(a.records.len(), b.records.len());
    let mut counts = [0, 0];
    for (a, b) in a.records.iter().zip(&b.records) {
        assert_eq!(a.record_range, b.record_range);
        assert_eq!(a.record_type.value, b.record_type.value);
        match a.record_type.value {
            0x02 => {
                assert_eq!(a.payload.bytes[2..6], b.payload.bytes[2..6]);
                assert_eq!(a.payload.bytes[14..], b.payload.bytes[14..]);
                counts[0] += 1;
            }
            0x29 => {
                assert_eq!(a.payload.bytes, b.payload.bytes);
                counts[1] += 1;
            }
            _ => {}
        }
    }
    assert_eq!(counts, [168, 168]);
    let a = phoenix::bounded_routing::decode_bounded_routing(&control).unwrap();
    let b = phoenix::bounded_routing::decode_bounded_routing(&edit).unwrap();
    assert_eq!(a.len(), b.len());
    for (a, b) in a.iter().zip(&b) {
        assert_eq!(
            (a.structural_ordinal, a.descriptor_ordinal),
            (b.structural_ordinal, b.descriptor_ordinal)
        );
        // Save-rekeyed device identifiers may differ; the resolved context must not.
        assert_eq!(
            a.result.as_ref().map(|r| (
                r.pair_ordinal,
                &r.event_range,
                &r.consumed_range,
                r.midi_channel,
                &r.device_name
            )),
            b.result.as_ref().map(|r| (
                r.pair_ordinal,
                &r.event_range,
                &r.consumed_range,
                r.midi_channel,
                &r.device_name
            ))
        );
    }
}

#[test]
fn authentic_inventory_and_policy_readiness_are_separate() {
    let Some(bytes) = authentic(
        BASE,
        "e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132",
    ) else {
        return;
    };
    let inspect = || {
        AppService::new()
            .inspect_project(InspectProjectRequest {
                contract_version: CONTRACT_VERSION,
                source_path: Path::new(ROOT).join(BASE).to_string_lossy().into_owned(),
                diagnostics_level: DiagnosticsLevel::Full,
            })
            .unwrap()
            .sequences
            .into_iter()
            .map(|s| (s.display_name, s.readiness, s.export_capability.is_some()))
            .collect::<Vec<_>>()
    };
    let before = inspect();
    let rows = collect_saved_mute_evidence(&bytes).unwrap();
    assert_eq!(rows.len(), 133);
    assert_eq!(before, inspect());
    assert_eq!(
        before
            .iter()
            .filter(|(_, r, exact)| *r == Readiness::Ready && *exact)
            .count(),
        6
    );
    assert_eq!(before.iter().filter(|(_, _, exact)| !*exact).count(), 12);
    for row in rows.iter().filter(|r| r.sequence_name == b"Sequence I") {
        assert_eq!(
            row.state,
            SavedMuteState::Unknown(SavedMuteUnknown::UnresolvedBinding)
        );
    }
    // Identical saved ON interpretation does not override opposite reference policies.
    for (seq, ordinals, profile, included) in [
        (
            b"Ode to Clarke".as_slice(),
            [2, 7],
            ode_to_clarke_profile().unwrap(),
            true,
        ),
        (
            b"Bells for her".as_slice(),
            [3, 8],
            bells_for_her_profile().unwrap(),
            false,
        ),
    ] {
        for ordinal in ordinals {
            let row = rows
                .iter()
                .find(|r| r.sequence_name == seq && r.descriptor_ordinal == ordinal)
                .unwrap();
            assert_eq!(row.state, SavedMuteState::On);
            let policy = profile.sequences[0]
                .track_expectations
                .iter()
                .find(|t| t.key.descriptor_ordinal == ordinal as u32)
                .unwrap();
            assert_eq!(
                matches!(
                    policy.output,
                    TrackOutputDispositionExpectation::Included(_)
                ),
                included
            );
        }
    }
    let expected_counts = [
        ("xForm", (1, 10, 2)),
        ("Bells for her", (2, 10, 2)),
        ("Situation", (0, 6, 0)),
        ("Sequence D", (0, 5, 1)),
        ("Sequence E", (1, 5, 2)),
        ("Girl-U-Want", (0, 3, 0)),
        ("mission impossibl", (0, 10, 0)),
        ("happyone", (3, 4, 4)),
        ("Sequence I", (0, 0, 11)),
        ("newsong", (0, 3, 2)),
        ("Sequence K", (0, 1, 1)),
        ("Renaissance", (0, 4, 2)),
        ("Get on up & Dance", (1, 14, 1)),
        ("Jurrasic Park", (1, 4, 2)),
        ("Ode to Clarke", (2, 7, 0)),
        ("Over the Top", (0, 3, 0)),
        ("Sequence Q", (0, 1, 0)),
        ("Sequence R", (0, 2, 0)),
    ];
    let blocked = rows
        .iter()
        .find(|r| r.sequence_name == b"xForm" && r.label_bytes.as_deref() == Some(b"Track 11 #2"))
        .unwrap();
    assert_eq!(blocked.candidate_value, Some(0x88));
    assert_eq!(blocked.neighboring_guard, Some(GUARD));
    // The bounded composition extension now completes this event walk.
    // This adds inferred saved state, not export omission authority.
    assert_eq!(blocked.state, SavedMuteState::On);
    let newly_decoded = rows
        .iter()
        .filter(|r| {
            r.neighboring_guard == Some(GUARD_04)
                && matches!(r.state, SavedMuteState::Off | SavedMuteState::On)
        })
        .collect::<Vec<_>>();
    assert_eq!(newly_decoded.len(), 21);
    assert_eq!(
        newly_decoded
            .iter()
            .filter(|r| r.state == SavedMuteState::On)
            .count(),
        1
    );
    assert_eq!(
        newly_decoded
            .iter()
            .map(|r| r.sequence_ordinal)
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        10
    );
    let incomplete_04 = rows
        .iter()
        .find(|r| r.sequence_name == b"happyone" && r.descriptor_ordinal == 8)
        .unwrap();
    assert_eq!(incomplete_04.neighboring_guard, Some(GUARD_04));
    assert_eq!(
        incomplete_04.state,
        SavedMuteState::Unknown(SavedMuteUnknown::IncompleteEventWalk)
    );
    for (name, _, _) in &before {
        let group: Vec<_> = rows
            .iter()
            .filter(|r| r.sequence_name == name.as_bytes())
            .collect();
        let on: Vec<_> = group
            .iter()
            .filter(|r| r.state == SavedMuteState::On)
            .map(|r| String::from_utf8_lossy(r.label_bytes.as_ref().unwrap()).into_owned())
            .collect();
        let off = group
            .iter()
            .filter(|r| r.state == SavedMuteState::Off)
            .count();
        let unknown = group.len() - on.len() - off;
        assert_eq!(
            (on.len(), off, unknown),
            expected_counts.iter().find(|(n, _)| *n == name).unwrap().1,
            "{name}"
        );
        println!("{name}: ON={on:?}, OFF={off}, Unknown={unknown}");
        for row in group.iter().filter(|r| {
            matches!(
                r.state,
                SavedMuteState::Unknown(SavedMuteUnknown::IncompleteEventWalk)
            )
        }) {
            println!("  incomplete: {:?}", row.label_bytes);
        }
    }
}

#[test]
fn relocation_and_reparsing_use_current_source_not_fixed_offsets() {
    let bytes = fixture(0x88, NOTE, 1);
    let mut relocated = bytes[..8].to_vec();
    record(&mut relocated, 0xfe, &[1, 2, 3]);
    relocated.extend_from_slice(&bytes[8..]);
    let row = collect_saved_mute_evidence(&relocated).unwrap().remove(0);
    assert_eq!(row.candidate_offset, Some(CANDIDATE + 8));
    assert_eq!(row.state, SavedMuteState::On);
    relocated[CANDIDATE + 8] = 0x80;
    assert_eq!(state(&relocated), SavedMuteState::Off);
    assert_eq!(state(&bytes), SavedMuteState::On);
}
