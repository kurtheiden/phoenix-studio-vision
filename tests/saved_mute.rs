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
    for value in [0x80, 0x88] {
        for (index, expected) in GUARD.iter().enumerate() {
            for other in 0..=255 {
                if other == *expected {
                    continue;
                }
                let mut bytes = fixture(value, NOTE, 1);
                bytes[CANDIDATE + 1 + index] = other;
                assert_eq!(
                    state(&bytes),
                    SavedMuteState::Unknown(SavedMuteUnknown::GuardMismatch)
                );
            }
        }
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
            .map(|s| (s.display_name, s.readiness))
            .collect::<Vec<_>>()
    };
    let before = inspect();
    let rows = collect_saved_mute_evidence(&bytes).unwrap();
    assert_eq!(rows.len(), 133);
    assert_eq!(before, inspect());
    assert_eq!(
        before
            .iter()
            .filter(|(_, r)| *r == Readiness::Ready)
            .count(),
        6
    );
    assert_eq!(
        before
            .iter()
            .filter(|(_, r)| *r == Readiness::PartiallySupported)
            .count(),
        12
    );
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
        ("xForm", (0, 4, 9)),
        ("Bells for her", (2, 7, 5)),
        ("Situation", (0, 4, 2)),
        ("Sequence D", (0, 3, 3)),
        ("Sequence E", (1, 2, 5)),
        ("Girl-U-Want", (0, 2, 1)),
        ("mission impossibl", (0, 5, 5)),
        ("happyone", (3, 4, 4)),
        ("Sequence I", (0, 0, 11)),
        ("newsong", (0, 1, 4)),
        ("Sequence K", (0, 1, 1)),
        ("Renaissance", (0, 3, 3)),
        ("Get on up & Dance", (1, 11, 4)),
        ("Jurrasic Park", (0, 4, 3)),
        ("Ode to Clarke", (2, 6, 1)),
        ("Over the Top", (0, 3, 0)),
        ("Sequence Q", (0, 1, 0)),
        ("Sequence R", (0, 1, 1)),
    ];
    let blocked = rows
        .iter()
        .find(|r| r.sequence_name == b"xForm" && r.label_bytes.as_deref() == Some(b"Track 11 #2"))
        .unwrap();
    assert_eq!(blocked.candidate_value, Some(0x88));
    assert_eq!(blocked.neighboring_guard, Some(GUARD));
    assert_eq!(
        blocked.state,
        SavedMuteState::Unknown(SavedMuteUnknown::IncompleteEventWalk)
    );
    for (name, _) in &before {
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
