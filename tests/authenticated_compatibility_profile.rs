use phoenix::app_contract::{
    CollisionPolicy, DiagnosticsLevel, ExportSequenceRequest, InspectProjectRequest,
    CONTRACT_VERSION,
};
use phoenix::app_service::AppService;
use phoenix::compatibility::{
    ByteRange, CompatibilityRegistry, EvidenceEventFamily, PatchTranslationPolicy, ProfileMatch,
    ProfileMismatchReason, ResolvedTrackOutputDisposition,
};
use phoenix::compatibility_profiles::{
    built_in_compatibility_registry, girl_u_want_profile, sequence_k_profile, sequence_q_profile,
};
use phoenix::mixed_event::{
    walk_bounded_mixed_events, MixedEventBounds, MixedEventItem, MixedEventKind,
    MixedEventTimingBasis,
};
use phoenix::sequence_container::parse_project_166;
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

const SOURCE: &str = "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline";
const GIRL_REFERENCE: &str = "/Users/kurtheiden/Documents/Phoenix Research/Studio Vision MIDI Exports/Project 001/Girl-U-Want - SVP ref";
const REQUIRE_SEQUENCE_Q_AUTHENTIC: &str = "PHOENIX_REQUIRE_AUTHENTIC_SEQUENCE_Q";

static NEXT_TEMP: AtomicU64 = AtomicU64::new(1);

fn authentic_sequence_q_source_available(path: &Path, required: bool) -> bool {
    if path.is_file() {
        eprintln!("running authentic Sequence Q profile validation");
        return true;
    }
    if required {
        panic!(
            "required authentic Sequence Q source is unavailable: {}",
            path.display()
        );
    }
    eprintln!(
        "skipping authentic Sequence Q profile validation: source is unavailable: {}",
        path.display()
    );
    false
}

fn inspect(path: &Path) -> (AppService, phoenix::app_contract::InspectProjectResponse) {
    let mut service = AppService::new();
    let response = service
        .inspect_project(InspectProjectRequest {
            contract_version: CONTRACT_VERSION,
            source_path: path.to_string_lossy().into_owned(),
            diagnostics_level: DiagnosticsLevel::Full,
        })
        .expect("authentic source should inspect");
    (service, response)
}

fn assess(path: &Path) -> ProfileMatch {
    assess_named(path, "Ode to Clarke")
}

fn assess_named(path: &Path, sequence_name: &str) -> ProfileMatch {
    let (service, response) = inspect(path);
    let sequence = response
        .sequences
        .iter()
        .find(|sequence| sequence.display_name == sequence_name)
        .expect("target sequence");
    let ordinal = service
        .sequence_ordinal_for_id(&response.session_id, &sequence.sequence_id)
        .expect("sequence identity mapping");
    let evidence = service
        .profile_evidence(&response.session_id)
        .expect("owned evidence");
    built_in_compatibility_registry()
        .expect("built-in profile validates")
        .assess(&evidence, ordinal)
        .expect("assessment should not be ambiguous")
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct GirlNoteKey {
    start: u32,
    end: u32,
    pitch: u8,
    attack: u8,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct MidiTrackFacts {
    name: Option<Vec<u8>>,
    instrument_name: Option<Vec<u8>>,
    channels: Vec<u8>,
    note_ons: Vec<(u32, u8, u8)>,
    note_ends: Vec<(u32, u8, u8)>,
    programs: Vec<(u32, u8, u8)>,
    controllers: Vec<(u32, u8, u8, u8)>,
    pitch_bends: usize,
    pressures: usize,
    sysex: usize,
    tempo: Vec<(u32, Vec<u8>)>,
    meter: Vec<(u32, Vec<u8>)>,
}

fn read_midi_vlq(bytes: &[u8], cursor: &mut usize) -> u32 {
    let mut value = 0;
    loop {
        let byte = bytes[*cursor];
        *cursor += 1;
        value = (value << 7) | u32::from(byte & 0x7f);
        if byte & 0x80 == 0 {
            return value;
        }
    }
}

fn parse_midi(bytes: &[u8]) -> (u16, u16, Vec<MidiTrackFacts>) {
    assert_eq!(&bytes[..4], b"MThd");
    let header_length = u32::from_be_bytes(bytes[4..8].try_into().unwrap()) as usize;
    let format = u16::from_be_bytes(bytes[8..10].try_into().unwrap());
    let track_count = u16::from_be_bytes(bytes[10..12].try_into().unwrap());
    let division = u16::from_be_bytes(bytes[12..14].try_into().unwrap());
    let mut cursor = 8 + header_length;
    let mut tracks = Vec::new();
    for _ in 0..track_count {
        assert_eq!(&bytes[cursor..cursor + 4], b"MTrk");
        let length = u32::from_be_bytes(bytes[cursor + 4..cursor + 8].try_into().unwrap()) as usize;
        let end = cursor + 8 + length;
        let data = &bytes[cursor + 8..end];
        cursor = end;

        let mut facts = MidiTrackFacts::default();
        let mut position = 0;
        let mut tick = 0;
        let mut running_status = None;
        while position < data.len() {
            tick += read_midi_vlq(data, &mut position);
            let byte = data[position];
            let status = if byte & 0x80 != 0 {
                position += 1;
                running_status = Some(byte);
                byte
            } else {
                running_status.expect("running MIDI status")
            };
            if status == 0xff {
                let meta_type = data[position];
                position += 1;
                let length = read_midi_vlq(data, &mut position) as usize;
                let payload = data[position..position + length].to_vec();
                position += length;
                match meta_type {
                    0x02 => facts.tempo.push((tick, payload)),
                    0x03 => facts.name = Some(payload),
                    0x04 => facts.instrument_name = Some(payload),
                    0x58 => facts.meter.push((tick, payload)),
                    0x2f => break,
                    _ => {}
                }
                continue;
            }
            if status == 0xf0 || status == 0xf7 {
                let length = read_midi_vlq(data, &mut position) as usize;
                position += length;
                facts.sysex += 1;
                continue;
            }

            let channel = (status & 0x0f) + 1;
            if !facts.channels.contains(&channel) {
                facts.channels.push(channel);
            }
            match status & 0xf0 {
                0x80 => {
                    let pitch = data[position];
                    let velocity = data[position + 1];
                    position += 2;
                    facts.note_ends.push((tick, pitch, velocity));
                }
                0x90 => {
                    let pitch = data[position];
                    let velocity = data[position + 1];
                    position += 2;
                    if velocity == 0 {
                        facts.note_ends.push((tick, pitch, velocity));
                    } else {
                        facts.note_ons.push((tick, pitch, velocity));
                    }
                }
                0xb0 => {
                    let controller = data[position];
                    let value = data[position + 1];
                    position += 2;
                    facts.controllers.push((tick, channel, controller, value));
                }
                0xc0 => {
                    facts.programs.push((tick, channel, data[position]));
                    position += 1;
                }
                0xd0 => {
                    position += 1;
                    facts.pressures += 1;
                }
                0xe0 => {
                    position += 2;
                    facts.pitch_bends += 1;
                }
                _ => position += 2,
            }
        }
        tracks.push(facts);
    }
    (format, division, tracks)
}

fn read_girl_reference() -> (u16, u16, Vec<MidiTrackFacts>) {
    let bytes = fs::read(GIRL_REFERENCE).expect("Girl-U-Want reference MIDI");
    parse_midi(&bytes)
}

fn girl_source_note_keys() -> Vec<Vec<GirlNoteKey>> {
    let bytes = fs::read(SOURCE).expect("Girl-U-Want source");
    let project = parse_project_166(&bytes).expect("Descriptor166 source");
    let sequence = &project.sequences[5];
    sequence
        .track_pairs
        .iter()
        .enumerate()
        .map(|(pair_ordinal, _)| {
            let bounds = sequence
                .validated_track_event_bounds(pair_ordinal)
                .expect("Girl-U-Want event bounds");
            let walk = walk_bounded_mixed_events(
                &bytes,
                MixedEventBounds {
                    event_range: bounds.event_range,
                },
                MixedEventTimingBasis::default(),
            )
            .expect("Girl-U-Want Note walk");
            walk.items
                .into_iter()
                .map(|item| match item {
                    MixedEventItem::Event(event) => match event.event {
                        MixedEventKind::Note(note) => GirlNoteKey {
                            start: event.position,
                            end: event.position + note.duration.value,
                            pitch: note.pitch.value,
                            attack: note.attack_velocity.value,
                        },
                        other => panic!("unexpected Girl-U-Want event: {other:?}"),
                    },
                    other => panic!("unexpected Girl-U-Want item: {other:?}"),
                })
                .collect()
        })
        .collect()
}

fn midi_note_keys(track: &MidiTrackFacts, expected: &[GirlNoteKey]) -> Vec<GirlNoteKey> {
    let mut remaining_ons = track.note_ons.clone();
    let mut remaining_ends = track.note_ends.clone();
    let mut keys = Vec::new();
    for note in expected {
        let on = (note.start, note.pitch, note.attack);
        let on_index = remaining_ons
            .iter()
            .position(|candidate| *candidate == on)
            .expect("reference Note On");
        remaining_ons.remove(on_index);
        let end_index = remaining_ends
            .iter()
            .position(|candidate| candidate.0 == note.end && candidate.1 == note.pitch)
            .expect("reference note ending");
        remaining_ends.remove(end_index);
        keys.push(note.clone());
    }
    assert!(remaining_ons.is_empty());
    assert!(remaining_ends.is_empty());
    keys
}

#[test]
fn authentic_sequence_k_profile_requires_exact_identity_and_complete_policy() {
    let path = Path::new(SOURCE);
    if !path.is_file() {
        return;
    }
    let (service, response) = inspect(path);
    let sequence = response
        .sequences
        .iter()
        .find(|sequence| sequence.display_name == "Sequence K")
        .expect("Sequence K");
    let ordinal = service
        .sequence_ordinal_for_id(&response.session_id, &sequence.sequence_id)
        .unwrap();
    assert_eq!(ordinal, 10);
    let evidence = service.profile_evidence(&response.session_id).unwrap();
    let registry = CompatibilityRegistry::new(vec![sequence_k_profile().unwrap()]).unwrap();
    let ProfileMatch::Matched {
        capability,
        resolved_policy,
    } = registry.assess(&evidence, ordinal).unwrap()
    else {
        panic!("authenticated Sequence K must match");
    };
    assert_eq!(capability.profile_id, "studio_vision_sequence_k_v1");
    assert_eq!(resolved_policy.track_manifest.len(), 2);
    assert!(matches!(
        &resolved_policy.track_manifest[0].output,
        ResolvedTrackOutputDisposition::Included {
            midi_channel: 15,
            patches,
        } if patches == &vec![PatchTranslationPolicy::ProgramOnly { program: 19 }]
    ));
    assert!(matches!(
        resolved_policy.track_manifest[1].output,
        ResolvedTrackOutputDisposition::OmittedStructuralEmpty
    ));
    assert_eq!(evidence.sequences[10].tracks[0].decoded_event_count, 29);
    assert_eq!(
        evidence.sequences[10].tracks[0].decoded_event_families,
        vec![EvidenceEventFamily::Patch, EvidenceEventFamily::Note]
    );
    assert_eq!(evidence.sequences[10].tracks[1].decoded_event_count, 0);
    assert!(evidence.sequences[10].tracks[1]
        .decoded_event_families
        .is_empty());

    let mut changed = evidence.clone();
    changed.source_sha256 = "0".repeat(64);
    assert!(matches!(
        registry.assess(&changed, ordinal).unwrap(),
        ProfileMatch::NoMatch
    ));
    let mut changed = evidence.clone();
    changed.sequences[10].name_bytes = b"Different".to_vec();
    assert!(matches!(
        registry.assess(&changed, ordinal).unwrap(),
        ProfileMatch::Rejected {
            reason: ProfileMismatchReason::SequenceIdentityMismatch,
            ..
        }
    ));
    let mut changed = evidence.clone();
    changed.sequences[10].tracks[0].exact_event_range = Some(ByteRange::new(1, 2).unwrap());
    assert!(matches!(
        registry.assess(&changed, ordinal).unwrap(),
        ProfileMatch::Rejected {
            reason: ProfileMismatchReason::TrackManifestMismatch,
            ..
        }
    ));
    let mut changed = evidence.clone();
    changed.sequences[10].tracks[0].observed_channel = Some(14);
    assert!(matches!(
        registry.assess(&changed, ordinal).unwrap(),
        ProfileMatch::Rejected {
            reason: ProfileMismatchReason::ChannelPolicyMismatch,
            ..
        }
    ));
    let mut changed = evidence.clone();
    changed.sequences[10].tracks[0].patch_evidence[0].decoded_program = 20;
    assert!(matches!(
        registry.assess(&changed, ordinal).unwrap(),
        ProfileMatch::Rejected {
            reason: ProfileMismatchReason::PatchPolicyMismatch,
            ..
        }
    ));
    let mut changed = evidence.clone();
    changed.sequences[10].tracks[1].decoded_event_count = 1;
    assert!(matches!(
        registry.assess(&changed, ordinal).unwrap(),
        ProfileMatch::Rejected {
            reason: ProfileMismatchReason::TrackManifestMismatch,
            ..
        }
    ));
    assert!(matches!(
        registry.assess(&evidence, 14).unwrap(),
        ProfileMatch::Rejected {
            reason: ProfileMismatchReason::SequenceIdentityMismatch,
            ..
        }
    ));
}

#[test]
fn authentic_sequence_q_profile_requires_exact_identity_event_evidence_and_channel_policy() {
    let path = Path::new(SOURCE);
    if !authentic_sequence_q_source_available(
        path,
        std::env::var_os(REQUIRE_SEQUENCE_Q_AUTHENTIC).is_some(),
    ) {
        return;
    }
    let (service, response) = inspect(path);
    let sequence = response
        .sequences
        .iter()
        .find(|sequence| sequence.display_name == "Sequence Q")
        .expect("Sequence Q");
    let ordinal = service
        .sequence_ordinal_for_id(&response.session_id, &sequence.sequence_id)
        .unwrap();
    assert_eq!(ordinal, 16);
    let evidence = service.profile_evidence(&response.session_id).unwrap();
    let registry = CompatibilityRegistry::new(vec![sequence_q_profile().unwrap()]).unwrap();
    let ProfileMatch::Matched {
        capability,
        resolved_policy,
    } = registry.assess(&evidence, ordinal).unwrap()
    else {
        panic!("authenticated Sequence Q must match");
    };
    assert_eq!(capability.profile_id, "studio_vision_sequence_q_v1");
    assert_eq!(resolved_policy.track_manifest.len(), 1);
    assert!(matches!(
        &resolved_policy.track_manifest[0].output,
        ResolvedTrackOutputDisposition::Included {
            midi_channel: 2,
            patches,
        } if patches.is_empty()
    ));
    let observed = &evidence.sequences[16].tracks[0];
    assert_eq!(observed.decoded_event_count, 185);
    assert_eq!(
        observed.decoded_event_families,
        vec![EvidenceEventFamily::Note]
    );
    assert!(observed.patch_evidence.is_empty());

    let mut changed = evidence.clone();
    changed.source_sha256 = "0".repeat(64);
    assert!(matches!(
        registry.assess(&changed, ordinal).unwrap(),
        ProfileMatch::NoMatch
    ));
    let mut changed = evidence.clone();
    changed.sequences[16].sequence_range = ByteRange::new(1, 2).unwrap();
    assert!(matches!(
        registry.assess(&changed, ordinal).unwrap(),
        ProfileMatch::Rejected {
            reason: ProfileMismatchReason::SequenceIdentityMismatch,
            ..
        }
    ));
    let mut changed = evidence.clone();
    changed.sequences[16].tracks[0].descriptor_range = ByteRange::new(1, 2).unwrap();
    assert!(matches!(
        registry.assess(&changed, ordinal).unwrap(),
        ProfileMatch::Rejected {
            reason: ProfileMismatchReason::TrackManifestMismatch,
            ..
        }
    ));
    let mut changed = evidence.clone();
    changed.sequences[16].tracks[0].primary_range = ByteRange::new(1, 2).unwrap();
    assert!(matches!(
        registry.assess(&changed, ordinal).unwrap(),
        ProfileMatch::Rejected {
            reason: ProfileMismatchReason::TrackManifestMismatch,
            ..
        }
    ));
    let mut changed = evidence.clone();
    changed.sequences[16].tracks[0].exact_event_range = Some(ByteRange::new(1, 2).unwrap());
    assert!(matches!(
        registry.assess(&changed, ordinal).unwrap(),
        ProfileMatch::Rejected {
            reason: ProfileMismatchReason::TrackManifestMismatch,
            ..
        }
    ));
    let mut changed = evidence.clone();
    changed.sequences[16].tracks[0].label_bytes = b"Different".to_vec();
    assert!(matches!(
        registry.assess(&changed, ordinal).unwrap(),
        ProfileMatch::Rejected {
            reason: ProfileMismatchReason::TrackManifestMismatch,
            ..
        }
    ));
    let mut changed = evidence.clone();
    changed.sequences[16].tracks[0].decoded_event_count = 184;
    assert!(matches!(
        registry.assess(&changed, ordinal).unwrap(),
        ProfileMatch::Rejected {
            reason: ProfileMismatchReason::TrackManifestMismatch,
            ..
        }
    ));
    let mut changed = evidence.clone();
    changed.sequences[16].tracks[0].decoded_event_families =
        vec![EvidenceEventFamily::Note, EvidenceEventFamily::Controller];
    assert!(matches!(
        registry.assess(&changed, ordinal).unwrap(),
        ProfileMatch::Rejected {
            reason: ProfileMismatchReason::TrackManifestMismatch,
            ..
        }
    ));
    let mut changed = evidence.clone();
    changed.sequences[16].tracks[0].observed_channel = Some(3);
    assert!(matches!(
        registry.assess(&changed, ordinal).unwrap(),
        ProfileMatch::Rejected {
            reason: ProfileMismatchReason::ChannelPolicyMismatch,
            ..
        }
    ));
}

#[test]
fn authentic_girl_u_want_profile_matches_exact_manifest_and_fails_closed() {
    let path = Path::new(SOURCE);
    if !path.is_file() || !Path::new(GIRL_REFERENCE).is_file() {
        return;
    }
    let (service, response) = inspect(path);
    let sequence = response
        .sequences
        .iter()
        .find(|sequence| sequence.display_name == "Girl-U-Want")
        .expect("Girl-U-Want");
    let ordinal = service
        .sequence_ordinal_for_id(&response.session_id, &sequence.sequence_id)
        .unwrap();
    assert_eq!(ordinal, 5);
    assert_eq!(sequence.readiness, phoenix::app_contract::Readiness::Ready);
    assert_eq!(
        sequence
            .export_capability
            .as_ref()
            .map(|capability| capability.profile_id.as_str()),
        Some("studio_vision_girl_u_want_v1")
    );

    let evidence = service.profile_evidence(&response.session_id).unwrap();
    let registry = CompatibilityRegistry::new(vec![girl_u_want_profile().unwrap()]).unwrap();
    let ProfileMatch::Matched {
        capability,
        resolved_policy,
    } = registry.assess(&evidence, ordinal).unwrap()
    else {
        panic!("authenticated Girl-U-Want source must match");
    };
    assert_eq!(capability.profile_id, "studio_vision_girl_u_want_v1");
    assert_eq!(resolved_policy.track_manifest.len(), 3);
    assert_eq!(
        resolved_policy
            .track_manifest
            .iter()
            .map(|entry| match entry.output {
                ResolvedTrackOutputDisposition::Included {
                    midi_channel,
                    ref patches,
                } => {
                    assert!(patches.is_empty());
                    midi_channel
                }
                _ => panic!("Girl-U-Want tracks must all be included"),
            })
            .collect::<Vec<_>>(),
        vec![2, 10, 1]
    );
    for (track, expected_count) in evidence.sequences[5].tracks.iter().zip([136, 109, 87]) {
        assert_eq!(track.decoded_event_count, expected_count);
        assert_eq!(
            track.decoded_event_families,
            vec![EvidenceEventFamily::Note]
        );
        assert!(track.patch_evidence.is_empty());
    }

    let mut changed = evidence.clone();
    changed.source_sha256 = "0".repeat(64);
    assert!(matches!(
        registry.assess(&changed, ordinal).unwrap(),
        ProfileMatch::NoMatch
    ));
    let mut changed = evidence.clone();
    changed.sequences[5].sequence_range = ByteRange::new(1, 2).unwrap();
    assert!(matches!(
        registry.assess(&changed, ordinal).unwrap(),
        ProfileMatch::Rejected {
            reason: ProfileMismatchReason::SequenceIdentityMismatch,
            ..
        }
    ));
    let mut changed = evidence.clone();
    changed.sequences[5].tracks[1].observed_channel = Some(9);
    assert!(matches!(
        registry.assess(&changed, ordinal).unwrap(),
        ProfileMatch::Rejected {
            reason: ProfileMismatchReason::ChannelPolicyMismatch,
            ..
        }
    ));
    assert!(matches!(
        registry.assess(&evidence, 4).unwrap(),
        ProfileMatch::Rejected {
            reason: ProfileMismatchReason::SequenceIdentityMismatch,
            ..
        }
    ));
}

#[test]
fn girl_u_want_reference_matches_all_notes_and_understood_zero_endings() {
    let source = Path::new(SOURCE);
    let reference = Path::new(GIRL_REFERENCE);
    if !source.is_file() || !reference.is_file() {
        return;
    }
    let expected = girl_source_note_keys();
    assert_eq!(expected.iter().map(Vec::len).sum::<usize>(), 332);
    let (format, division, tracks) = read_girl_reference();
    assert_eq!(format, 1);
    assert_eq!(division, 480);
    assert_eq!(tracks.len(), 4);
    assert_eq!(
        tracks
            .iter()
            .skip(1)
            .map(|track| track.channels.as_slice())
            .collect::<Vec<_>>(),
        vec![&[2][..], &[10][..], &[1][..]]
    );
    assert_eq!(
        tracks
            .iter()
            .skip(1)
            .map(|track| track.note_ons.len())
            .collect::<Vec<_>>(),
        vec![136, 109, 87]
    );
    assert_eq!(
        tracks
            .iter()
            .skip(1)
            .map(|track| track.note_ends.len())
            .collect::<Vec<_>>(),
        vec![136, 109, 87]
    );
    assert_eq!(
        tracks
            .iter()
            .skip(1)
            .map(|track| track.programs.len())
            .sum::<usize>(),
        0
    );
    assert_eq!(
        tracks
            .iter()
            .skip(1)
            .map(|track| track.controllers.len())
            .sum::<usize>(),
        0
    );
    assert_eq!(
        tracks
            .iter()
            .skip(1)
            .map(|track| track.pitch_bends + track.pressures + track.sysex)
            .sum::<usize>(),
        0
    );
    for (track, notes) in tracks.iter().skip(1).zip(expected.iter()) {
        let normalized = midi_note_keys(track, notes);
        assert_eq!(normalized.as_slice(), notes.as_slice());
    }
    let zero_velocity_endings = tracks[1]
        .note_ends
        .iter()
        .chain(tracks[2].note_ends.iter())
        .chain(tracks[3].note_ends.iter())
        .filter(|ending| ending.2 == 0)
        .count();
    assert_eq!(zero_velocity_endings, 3);
}

#[test]
fn girl_u_want_export_is_ready_without_promoting_other_partial_sequences() {
    let path = Path::new(SOURCE);
    if !path.is_file() {
        return;
    }
    let mut service = AppService::new();
    let response = service
        .inspect_project(InspectProjectRequest {
            contract_version: CONTRACT_VERSION,
            source_path: path.to_string_lossy().into_owned(),
            diagnostics_level: DiagnosticsLevel::Full,
        })
        .expect("authentic source should inspect");
    let girl = response
        .sequences
        .iter()
        .find(|sequence| sequence.display_name == "Girl-U-Want")
        .expect("Girl-U-Want sequence");
    assert!(matches!(
        girl.readiness,
        phoenix::app_contract::Readiness::Ready
    ));

    let destination = std::env::temp_dir().join(format!(
        "phoenix-girl-u-want-profile-{}-{}",
        std::process::id(),
        NEXT_TEMP.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(&destination).expect("temporary destination");
    let exported = service
        .export_sequence(ExportSequenceRequest {
            contract_version: CONTRACT_VERSION,
            session_id: response.session_id.clone(),
            sequence_id: girl.sequence_id.clone(),
            destination_folder: destination.to_string_lossy().into_owned(),
            filename_stem: "Girl-U-Want".into(),
            collision_policy: CollisionPolicy::FailIfExists,
            operation_id: None,
        })
        .expect("Girl-U-Want export should be conversion-ready");
    assert_eq!(exported.musical_track_count, 3);
    assert_eq!(exported.total_smf_track_count, 4);
    assert_eq!(exported.counts.notes, 332);
    assert_eq!(exported.counts.programs, 0);
    assert_eq!(exported.counts.controllers, 0);
    let generated = fs::read(destination.join("Girl-U-Want.mid")).expect("generated SMF");
    assert_eq!(&generated[..4], b"MThd");
    assert_eq!(u16::from_be_bytes(generated[10..12].try_into().unwrap()), 4);
    let (generated_format, generated_division, generated_tracks) = parse_midi(&generated);
    assert_eq!(generated_format, 1);
    assert_eq!(generated_division, 480);
    assert_eq!(
        generated_tracks
            .iter()
            .skip(1)
            .map(|track| track.channels.as_slice())
            .collect::<Vec<_>>(),
        vec![&[2][..], &[10][..], &[1][..]]
    );
    let expected = girl_source_note_keys();
    for (track, notes) in generated_tracks.iter().skip(1).zip(expected.iter()) {
        assert_eq!(midi_note_keys(track, notes).as_slice(), notes.as_slice());
        assert!(track.programs.is_empty());
        assert!(track.controllers.is_empty());
        assert_eq!(track.pitch_bends, 0);
        assert_eq!(track.pressures, 0);
        assert_eq!(track.sysex, 0);
    }
    fs::remove_dir_all(destination).ok();

    for sequence in &response.sequences {
        if matches!(
            sequence.display_name.as_str(),
            "Ode to Clarke" | "Bells for her" | "Sequence K" | "Sequence Q" | "Girl-U-Want"
        ) {
            assert_eq!(sequence.readiness, phoenix::app_contract::Readiness::Ready);
        } else {
            assert_ne!(sequence.readiness, phoenix::app_contract::Readiness::Ready);
        }
    }
}

#[test]
#[should_panic(expected = "required authentic Sequence Q source is unavailable")]
fn required_authentic_sequence_q_profile_mode_rejects_missing_source() {
    let missing = std::env::temp_dir().join(format!(
        "phoenix-missing-sequence-q-source-{}-{}",
        std::process::id(),
        NEXT_TEMP.fetch_add(1, Ordering::Relaxed)
    ));
    assert!(!missing.exists());
    authentic_sequence_q_source_available(&missing, true);
}

#[test]
fn authentic_bells_profile_matches_complete_manifest() {
    let path = Path::new(SOURCE);
    if !path.is_file() {
        return;
    }
    let result = assess_named(path, "Bells for her");
    let ProfileMatch::Matched {
        capability,
        resolved_policy,
    } = result
    else {
        panic!("authenticated Bells source must match built-in profile");
    };
    assert_eq!(capability.profile_id, "studio_vision_bells_for_her_v1");
    assert_eq!(capability.profile_version, 1);
    assert_eq!(
        capability.display_label,
        "Validated research profile — Bells for her"
    );
    assert_eq!(resolved_policy.sequence.structural_ordinal, 1);
    assert_eq!(resolved_policy.track_manifest.len(), 14);
    let included = [1, 3, 4, 5, 6, 8, 9, 11, 12, 14];
    let nonempty_omitted = [2, 7];
    let empty_omitted = [10, 13];
    let mut included_ordinals = Vec::new();
    let mut nonempty_ordinals = Vec::new();
    let mut empty_ordinals = Vec::new();
    let mut channels = Vec::new();
    let mut patch_policies = Vec::new();
    for entry in &resolved_policy.track_manifest {
        let ordinal = entry.key.pair_ordinal + 1;
        match &entry.output {
            ResolvedTrackOutputDisposition::Included {
                midi_channel,
                patches,
            } => {
                included_ordinals.push(ordinal);
                channels.push(*midi_channel);
                patch_policies.push((ordinal, patches.clone()));
            }
            ResolvedTrackOutputDisposition::OmittedAuthenticatedNonempty { .. } => {
                nonempty_ordinals.push(ordinal)
            }
            ResolvedTrackOutputDisposition::OmittedStructuralEmpty => empty_ordinals.push(ordinal),
        }
    }
    assert_eq!(included_ordinals, included);
    assert_eq!(nonempty_ordinals, nonempty_omitted);
    assert_eq!(empty_ordinals, empty_omitted);
    for entry in &resolved_policy.track_manifest {
        match entry.output {
            ResolvedTrackOutputDisposition::OmittedAuthenticatedNonempty {
                decoded_event_count,
                ref decoded_event_families,
                ref patches,
            } => {
                let expected_count = match entry.key.pair_ordinal + 1 {
                    2 => 83,
                    7 => 165,
                    ordinal => panic!("unexpected nonempty omission {ordinal}"),
                };
                assert_eq!(decoded_event_count, expected_count);
                assert_eq!(
                    decoded_event_families,
                    &[
                        phoenix::compatibility::EvidenceEventFamily::Patch,
                        phoenix::compatibility::EvidenceEventFamily::Note
                    ]
                );
                assert_eq!(patches.len(), 1);
            }
            ResolvedTrackOutputDisposition::OmittedStructuralEmpty => {}
            ResolvedTrackOutputDisposition::Included { .. } => {}
        }
    }
    assert_eq!(channels, vec![1, 16, 2, 3, 1, 16, 12, 8, 10, 15]);
    assert_eq!(
        patch_policies,
        vec![
            (1, vec![PatchTranslationPolicy::ProgramOnly { program: 16 }]),
            (
                3,
                vec![PatchTranslationPolicy::BankSelectMsbAndProgram {
                    msb: 81,
                    program: 25
                }]
            ),
            (
                4,
                vec![PatchTranslationPolicy::BankSelectAndProgram {
                    msb: 81,
                    lsb: 1,
                    program: 34
                }]
            ),
            (
                5,
                vec![PatchTranslationPolicy::BankSelectAndProgram {
                    msb: 80,
                    lsb: 0,
                    program: 70
                }]
            ),
            (
                6,
                vec![PatchTranslationPolicy::BankSelectMsbAndProgram {
                    msb: 81,
                    program: 35
                }]
            ),
            (
                8,
                vec![PatchTranslationPolicy::BankSelectMsbAndProgram {
                    msb: 81,
                    program: 25
                }]
            ),
            (
                9,
                vec![PatchTranslationPolicy::BankSelectMsbAndProgram {
                    msb: 81,
                    program: 122
                }]
            ),
            (
                11,
                vec![PatchTranslationPolicy::BankSelectAndProgram {
                    msb: 80,
                    lsb: 0,
                    program: 12
                }]
            ),
            (12, Vec::new()),
            (14, Vec::new()),
        ]
    );
}

#[test]
fn authentic_bells_is_ready_and_exports_only_included_tracks() {
    let path = Path::new(SOURCE);
    if !path.is_file() {
        return;
    }
    let mut service = AppService::new();
    let response = service
        .inspect_project(InspectProjectRequest {
            contract_version: CONTRACT_VERSION,
            source_path: path.to_string_lossy().into_owned(),
            diagnostics_level: DiagnosticsLevel::Full,
        })
        .expect("authentic Bells source should inspect");
    let sequence = response
        .sequences
        .iter()
        .find(|sequence| sequence.display_name == "Bells for her")
        .expect("Bells sequence");
    assert_eq!(
        sequence
            .export_capability
            .as_ref()
            .map(|c| c.profile_id.as_str()),
        Some("studio_vision_bells_for_her_v1")
    );
    assert!(matches!(
        sequence.readiness,
        phoenix::app_contract::Readiness::Ready
    ));
    let destination = std::env::temp_dir().join(format!(
        "phoenix-bells-profile-{}-{}",
        std::process::id(),
        NEXT_TEMP.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(&destination).expect("temporary destination");
    let exported = service
        .export_sequence(ExportSequenceRequest {
            contract_version: CONTRACT_VERSION,
            session_id: response.session_id,
            sequence_id: sequence.sequence_id.clone(),
            destination_folder: destination.to_string_lossy().into_owned(),
            filename_stem: "Bells for her".into(),
            collision_policy: CollisionPolicy::FailIfExists,
            operation_id: None,
        })
        .expect("Bells export should be conversion-ready");
    assert_eq!(exported.musical_track_count, 10);
    assert_eq!(exported.total_smf_track_count, 11);
    assert_eq!(exported.counts.notes, 3_186);
    assert_eq!(exported.counts.programs, 8);
    assert_eq!(exported.counts.bank_select_msb, 7);
    assert_eq!(exported.counts.bank_select_lsb, 3);
    assert_eq!(exported.counts.controllers, 395);
    assert_eq!(exported.counts.pressure, 32);
    assert_eq!(exported.counts.pitch_bend, 102);
    assert!(destination.join("Bells for her.mid").is_file());
    let generated = fs::read(destination.join("Bells for her.mid")).expect("generated SMF");
    assert_eq!(&generated[..4], b"MThd");
    assert_eq!(&generated[8..10], &[0, 1]);
    assert_eq!(&generated[10..12], &[0, 11]);
    assert_eq!(&generated[12..14], &[1, 224]);
    let names = [
        b"Track 1".as_slice(),
        b"Track 3",
        b"Track 4",
        b"Track 5",
        b"Track 6",
        b"Track 8",
        b"Track 9",
        b"Track 11",
        b"Track 12",
        b"Track 14",
    ];
    let positions = names
        .iter()
        .map(|name| {
            generated
                .windows(name.len())
                .position(|window| window == *name)
                .expect("generated musical track name")
        })
        .collect::<Vec<_>>();
    assert!(positions.windows(2).all(|pair| pair[0] < pair[1]));
    fs::remove_dir_all(destination).ok();
}

#[test]
fn authentic_profile_matches_complete_policy() {
    let path = Path::new(SOURCE);
    if !path.is_file() {
        return;
    }
    let ProfileMatch::Matched {
        capability,
        resolved_policy,
    } = assess(path)
    else {
        panic!("authenticated source must match built-in profile");
    };
    assert_eq!(capability.profile_id, "studio_vision_ode_to_clarke_v1");
    assert_eq!(capability.profile_version, 1);
    assert_eq!(
        capability.display_label,
        "Validated research profile — Ode to Clarke"
    );
    assert_eq!(resolved_policy.sequence.structural_ordinal, 14);
    assert_eq!(resolved_policy.track_manifest.len(), 9);
    assert_eq!(
        resolved_policy
            .track_manifest
            .iter()
            .map(|track| match track.output {
                ResolvedTrackOutputDisposition::Included { midi_channel, .. } => midi_channel,
                _ => panic!("Ode tracks must remain included"),
            })
            .collect::<Vec<_>>(),
        vec![1, 2, 10, 10, 10, 1, 10, 15, 10]
    );
    assert_eq!(
        resolved_policy
            .track_manifest
            .iter()
            .map(|track| match &track.output {
                ResolvedTrackOutputDisposition::Included { patches, .. } => patches.len(),
                _ => panic!("Ode tracks must remain included"),
            })
            .sum::<usize>(),
        4
    );
}

#[test]
fn renamed_identical_bytes_match_and_mutated_bytes_do_not() {
    let source = Path::new(SOURCE);
    if !source.is_file() {
        return;
    }
    let bytes = fs::read(source).expect("source bytes");
    let nonce = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
    let renamed = std::env::temp_dir().join(format!("phoenix-ode-renamed-{nonce}"));
    fs::write(&renamed, &bytes).expect("renamed copy");
    assert!(matches!(assess(&renamed), ProfileMatch::Matched { .. }));

    let mutated = std::env::temp_dir().join(format!("phoenix-ode-mutated-{nonce}"));
    let mut changed = bytes;
    changed[0] ^= 1;
    fs::write(&mutated, changed).expect("mutated copy");
    assert!(matches!(assess(&mutated), ProfileMatch::NoMatch));
    fs::remove_file(renamed).ok();
    fs::remove_file(mutated).ok();
}

#[test]
fn same_name_unrelated_input_and_other_sequence_never_match() {
    let source = Path::new(SOURCE);
    if !source.is_file() {
        return;
    }
    let unrelated = std::env::temp_dir().join("phoenix-ode-same-name-unrelated");
    fs::write(&unrelated, b"Ode to Clarke").expect("unrelated input");
    let mut unrelated_service = AppService::new();
    let unrelated_response = unrelated_service
        .inspect_project(InspectProjectRequest {
            contract_version: CONTRACT_VERSION,
            source_path: unrelated.to_string_lossy().into_owned(),
            diagnostics_level: DiagnosticsLevel::None,
        })
        .expect("readable unrelated input");
    assert!(unrelated_response.sequences.is_empty());
    let (service, response) = inspect(source);
    let target = response
        .sequences
        .iter()
        .find(|sequence| sequence.display_name == "Ode to Clarke")
        .unwrap();
    let ordinal = service
        .sequence_ordinal_for_id(&response.session_id, &target.sequence_id)
        .unwrap();
    let mut unrelated_evidence = service.profile_evidence(&response.session_id).unwrap();
    unrelated_evidence.source_sha256 = "0".repeat(64);
    assert!(matches!(
        built_in_compatibility_registry()
            .unwrap()
            .assess(&unrelated_evidence, ordinal)
            .unwrap(),
        ProfileMatch::NoMatch
    ));
    fs::remove_file(unrelated).ok();

    let (service, response) = inspect(source);
    let other = response
        .sequences
        .iter()
        .find(|sequence| sequence.display_name != "Ode to Clarke")
        .expect("another sequence");
    let other_ordinal = service
        .sequence_ordinal_for_id(&response.session_id, &other.sequence_id)
        .unwrap();
    let result = built_in_compatibility_registry()
        .unwrap()
        .assess(
            &service.profile_evidence(&response.session_id).unwrap(),
            other_ordinal,
        )
        .unwrap();
    assert!(!matches!(result, ProfileMatch::Matched { .. }));
}

#[test]
fn structural_and_patch_drift_reject_after_provenance_candidate() {
    let source = Path::new(SOURCE);
    if !source.is_file() {
        return;
    }
    let (service, response) = inspect(source);
    let sequence = response
        .sequences
        .iter()
        .find(|sequence| sequence.display_name == "Ode to Clarke")
        .unwrap();
    let ordinal = service
        .sequence_ordinal_for_id(&response.session_id, &sequence.sequence_id)
        .unwrap();
    let mut evidence = service.profile_evidence(&response.session_id).unwrap();
    evidence.sequences[ordinal as usize].sequence_range =
        phoenix::compatibility::ByteRange::new(1, 2).unwrap();
    assert!(matches!(
        built_in_compatibility_registry()
            .unwrap()
            .assess(&evidence, ordinal)
            .unwrap(),
        ProfileMatch::Rejected {
            reason: ProfileMismatchReason::SequenceIdentityMismatch,
            ..
        }
    ));

    let mut evidence = service.profile_evidence(&response.session_id).unwrap();
    evidence.sequences[ordinal as usize].tracks[0].patch_evidence[0].decoded_program ^= 1;
    assert!(matches!(
        built_in_compatibility_registry()
            .unwrap()
            .assess(&evidence, ordinal)
            .unwrap(),
        ProfileMatch::Rejected {
            reason: ProfileMismatchReason::PatchPolicyMismatch,
            ..
        }
    ));
}
