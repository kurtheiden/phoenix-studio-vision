use phoenix::app_contract::{
    CollisionPolicy, DiagnosticsLevel, ExportSequenceRequest, InspectProjectRequest,
    CONTRACT_VERSION,
};
use phoenix::app_service::AppService;
use sha2::{Digest, Sha256};
use std::{collections::HashMap, fs, path::Path};

const PROJECT: &str = "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline";
const MIDI: &str = "/Users/kurtheiden/Documents/Phoenix Research/Studio Vision MIDI Exports/Project 001/newest STUFF - Bells for her - provenance multitrack";
const PROJECT_SHA256: &str = "e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132";
const MIDI_SHA256: &str = "ffbdbb6be208a2d607c9b0c55a12b72226a18d43b9494c2b46b058d4568fc2c3";

#[derive(Clone, Debug, PartialEq, Eq)]
struct Note {
    start: u32,
    end: u32,
    channel: u8,
    pitch: u8,
    attack: u8,
    release: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ChannelEvent {
    tick: u32,
    channel: u8,
    kind: u8,
    a: u8,
    b: Option<u8>,
}

#[derive(Clone, Debug, Default)]
struct Track {
    name: Option<Vec<u8>>,
    notes: Vec<Note>,
    events: Vec<ChannelEvent>,
    tempos: Vec<(u32, u32)>,
    meters: Vec<(u32, u8, u8, u8, u8)>,
}

#[derive(Clone, Debug)]
struct Smf {
    format: u16,
    division: u16,
    tracks: Vec<Track>,
}

#[test]
fn authentic_bells_generated_midi_reconciles_with_reference() {
    let project_path = Path::new(PROJECT);
    let midi_path = Path::new(MIDI);
    if !project_path.is_file() || !midi_path.is_file() {
        return;
    }
    let project = fs::read(project_path).expect("authentic project");
    let reference_bytes = fs::read(midi_path).expect("authenticated Bells MIDI");
    assert_eq!(sha256_hex(&project), PROJECT_SHA256);
    assert_eq!(sha256_hex(&reference_bytes), MIDI_SHA256);

    let mut service = AppService::new();
    let inspected = service
        .inspect_project(InspectProjectRequest {
            contract_version: CONTRACT_VERSION,
            source_path: PROJECT.into(),
            diagnostics_level: DiagnosticsLevel::Full,
        })
        .expect("authentic project inspection");
    let sequence = inspected
        .sequences
        .iter()
        .find(|sequence| sequence.display_name == "Bells for her")
        .expect("Bells sequence");
    let destination =
        std::env::temp_dir().join(format!("phoenix-bells-reconcile-{}", std::process::id()));
    fs::create_dir_all(&destination).expect("temporary destination");
    let export = service
        .export_sequence(ExportSequenceRequest {
            contract_version: CONTRACT_VERSION,
            session_id: inspected.session_id,
            sequence_id: sequence.sequence_id.clone(),
            destination_folder: destination.to_string_lossy().into_owned(),
            filename_stem: "Bells reconciliation".into(),
            collision_policy: CollisionPolicy::FailIfExists,
            operation_id: None,
        })
        .expect("Bells export");
    assert_eq!(export.musical_track_count, 10);
    let generated_path = destination.join("Bells reconciliation.mid");
    let generated_bytes = fs::read(&generated_path).expect("generated MIDI");
    let generated = parse_smf(&generated_bytes).expect("generated SMF");
    let reference = parse_smf(&reference_bytes).expect("reference SMF");
    let result = compare_smf(&generated, &reference);
    fs::remove_dir_all(destination).ok();
    if !result.mismatches.is_empty() {
        panic!("Bells normalized reconciliation failed: {:#?}", result);
    }
    assert_eq!(result.generated_musical_tracks, 10);
    assert_eq!(result.reference_musical_tracks, 10);
    assert_eq!(result.notes, result.reference_notes);
    assert_eq!(result.note_start_matches, result.notes);
    assert_eq!(result.note_end_matches, result.notes);
    assert_eq!(result.attack_matches, result.notes);
    assert_eq!(
        result.exact_releases + result.zero_velocity_substitutions,
        result.notes
    );
    assert_eq!(result.unexplained_release_mismatches, 0);
    assert_eq!(result.controllers, result.reference_controllers);
    assert_eq!(result.controller_matches, result.controllers);
    assert_eq!(result.pressure, result.reference_pressure);
    assert_eq!(result.pressure_matches, result.pressure);
    assert_eq!(result.bends, result.reference_bends);
    assert_eq!(result.bend_matches, result.bends);
    assert_eq!(result.programs, result.reference_programs);
    assert_eq!(result.program_matches, result.programs);
    assert_eq!(result.cc0, result.reference_cc0);
    assert_eq!(result.cc0_matches, result.cc0);
    assert_eq!(result.cc32, result.reference_cc32);
    assert_eq!(result.cc32_matches, result.cc32);
    assert_eq!(result.generated_tempos, result.reference_tempos);
    assert_eq!(result.generated_meters, result.reference_meters);
    assert_eq!(result.tempo_matches, result.generated_tempos.len());
    assert_eq!(result.meter_matches, result.generated_meters.len());
}

#[derive(Debug)]
struct Comparison {
    generated_musical_tracks: usize,
    reference_musical_tracks: usize,
    notes: usize,
    reference_notes: usize,
    note_start_matches: usize,
    note_end_matches: usize,
    attack_matches: usize,
    exact_releases: usize,
    zero_velocity_substitutions: usize,
    unexplained_release_mismatches: usize,
    controllers: usize,
    reference_controllers: usize,
    controller_matches: usize,
    pressure: usize,
    reference_pressure: usize,
    pressure_matches: usize,
    bends: usize,
    reference_bends: usize,
    bend_matches: usize,
    programs: usize,
    reference_programs: usize,
    program_matches: usize,
    cc0: usize,
    reference_cc0: usize,
    cc0_matches: usize,
    cc32: usize,
    reference_cc32: usize,
    cc32_matches: usize,
    generated_tempos: Vec<(u32, u32)>,
    reference_tempos: Vec<(u32, u32)>,
    generated_meters: Vec<(u32, u8, u8, u8, u8)>,
    reference_meters: Vec<(u32, u8, u8, u8, u8)>,
    tempo_matches: usize,
    meter_matches: usize,
    mismatches: Vec<String>,
}

fn compare_smf(generated: &Smf, reference: &Smf) -> Comparison {
    let mut out = Comparison {
        generated_musical_tracks: generated.tracks.len().saturating_sub(1),
        reference_musical_tracks: reference.tracks.len().saturating_sub(1),
        notes: 0,
        reference_notes: 0,
        note_start_matches: 0,
        note_end_matches: 0,
        attack_matches: 0,
        exact_releases: 0,
        zero_velocity_substitutions: 0,
        unexplained_release_mismatches: 0,
        controllers: 0,
        reference_controllers: 0,
        controller_matches: 0,
        pressure: 0,
        reference_pressure: 0,
        pressure_matches: 0,
        bends: 0,
        reference_bends: 0,
        bend_matches: 0,
        programs: 0,
        reference_programs: 0,
        program_matches: 0,
        cc0: 0,
        reference_cc0: 0,
        cc0_matches: 0,
        cc32: 0,
        reference_cc32: 0,
        cc32_matches: 0,
        generated_tempos: generated.tracks[0].tempos.clone(),
        reference_tempos: reference.tracks[0].tempos.clone(),
        generated_meters: generated.tracks[0].meters.clone(),
        reference_meters: reference.tracks[0].meters.clone(),
        tempo_matches: 0,
        meter_matches: 0,
        mismatches: Vec::new(),
    };
    if generated.format != reference.format || generated.division != reference.division {
        mismatch(
            &mut out,
            format!(
                "header generated {:?} reference {:?}",
                (generated.format, generated.division),
                (reference.format, reference.division)
            ),
        );
    }
    if out.generated_tempos == out.reference_tempos {
        out.tempo_matches = out.generated_tempos.len();
    } else {
        mismatch(&mut out, "tempo mismatch".into());
    }
    if out.generated_meters == out.reference_meters {
        out.meter_matches = out.generated_meters.len();
    } else {
        mismatch(&mut out, "meter mismatch".into());
    }
    let names = [
        "Track 1", "Track 3", "Track 4", "Track 5", "Track 6", "Track 8", "Track 9", "Track 11",
        "Track 12", "Track 14",
    ];
    if generated.tracks.len() != 11 || reference.tracks.len() != 11 {
        mismatch(&mut out, "track count mismatch".into());
    }
    for (index, expected_name) in names.iter().enumerate() {
        let gi = index + 1;
        let (Some(g), Some(r)) = (generated.tracks.get(gi), reference.tracks.get(gi)) else {
            mismatch(&mut out, format!("missing musical track {expected_name}"));
            continue;
        };
        if g.name.as_deref() != Some(expected_name.as_bytes())
            || r.name.as_deref() != Some(expected_name.as_bytes())
        {
            mismatch(&mut out, format!("track identity {expected_name}"));
        }
        compare_track(&mut out, expected_name, g, r);
    }
    out
}

fn compare_track(out: &mut Comparison, name: &str, generated: &Track, reference: &Track) {
    out.notes += generated.notes.len();
    out.reference_notes += reference.notes.len();
    for (index, (g, r)) in generated.notes.iter().zip(&reference.notes).enumerate() {
        if g.start == r.start {
            out.note_start_matches += 1;
        } else {
            mismatch(
                out,
                format!("{name} Note {index} start {} != {}", g.start, r.start),
            );
        }
        if g.end == r.end {
            out.note_end_matches += 1;
        } else {
            mismatch(
                out,
                format!("{name} Note {index} end {} != {}", g.end, r.end),
            );
        }
        if (g.channel, g.pitch, g.attack) == (r.channel, r.pitch, r.attack) {
            out.attack_matches += 1;
        } else {
            mismatch(
                out,
                format!("{name} Note {index} identity/attack {g:?} != {r:?}"),
            );
        }
        if g.release == r.release {
            out.exact_releases += 1;
        } else if r.release == 0 {
            out.zero_velocity_substitutions += 1;
        } else {
            out.unexplained_release_mismatches += 1;
            mismatch(
                out,
                format!("{name} Note {index} release {} != {}", g.release, r.release),
            );
        }
    }
    if generated.notes.len() != reference.notes.len() {
        mismatch(
            out,
            format!(
                "{name} note count {} != {}",
                generated.notes.len(),
                reference.notes.len()
            ),
        );
    }
    let filter = |event: &&ChannelEvent| event.kind == 0xb && event.a != 0 && event.a != 32;
    let gc: Vec<_> = generated.events.iter().filter(filter).collect();
    let rc: Vec<_> = reference.events.iter().filter(filter).collect();
    out.controllers += gc.len();
    out.reference_controllers += rc.len();
    out.controller_matches += gc.iter().zip(&rc).filter(|(g, r)| *g == *r).count();
    if gc != rc {
        mismatch(
            out,
            format!(
                "{name} ordinary Controller mismatch ({} vs {})",
                gc.len(),
                rc.len()
            ),
        );
    }
    let gp: Vec<_> = generated.events.iter().filter(|e| e.kind == 0xd).collect();
    let rp: Vec<_> = reference.events.iter().filter(|e| e.kind == 0xd).collect();
    out.pressure += gp.len();
    out.reference_pressure += rp.len();
    out.pressure_matches += gp.iter().zip(&rp).filter(|(g, r)| *g == *r).count();
    if gp != rp {
        mismatch(out, format!("{name} pressure mismatch"));
    }
    let gb: Vec<_> = generated.events.iter().filter(|e| e.kind == 0xe).collect();
    let rb: Vec<_> = reference.events.iter().filter(|e| e.kind == 0xe).collect();
    out.bends += gb.len();
    out.reference_bends += rb.len();
    out.bend_matches += gb.iter().zip(&rb).filter(|(g, r)| *g == *r).count();
    if gb != rb {
        mismatch(out, format!("{name} bend mismatch"));
    }
    let gp: Vec<_> = generated.events.iter().filter(|e| e.kind == 0xc).collect();
    let rp: Vec<_> = reference.events.iter().filter(|e| e.kind == 0xc).collect();
    out.programs += gp.len();
    out.reference_programs += rp.len();
    out.program_matches += gp.iter().zip(&rp).filter(|(g, r)| *g == *r).count();
    if gp != rp {
        mismatch(out, format!("{name} Program mismatch"));
    }
    let g0: Vec<_> = generated
        .events
        .iter()
        .filter(|e| e.kind == 0xb && e.a == 0)
        .collect();
    let r0: Vec<_> = reference
        .events
        .iter()
        .filter(|e| e.kind == 0xb && e.a == 0)
        .collect();
    out.cc0 += g0.len();
    out.reference_cc0 += r0.len();
    out.cc0_matches += g0.iter().zip(&r0).filter(|(g, r)| *g == *r).count();
    if g0 != r0 {
        mismatch(out, format!("{name} CC0 mismatch"));
    }
    let g32: Vec<_> = generated
        .events
        .iter()
        .filter(|e| e.kind == 0xb && e.a == 32)
        .collect();
    let r32: Vec<_> = reference
        .events
        .iter()
        .filter(|e| e.kind == 0xb && e.a == 32)
        .collect();
    out.cc32 += g32.len();
    out.reference_cc32 += r32.len();
    out.cc32_matches += g32.iter().zip(&r32).filter(|(g, r)| *g == *r).count();
    if g32 != r32 {
        mismatch(out, format!("{name} CC32 mismatch"));
    }
}

fn mismatch(out: &mut Comparison, message: String) {
    if out.mismatches.len() < 8 {
        out.mismatches.push(message);
    }
}

fn parse_smf(bytes: &[u8]) -> Result<Smf, String> {
    if bytes.len() < 14 || &bytes[..4] != b"MThd" || be_u32(&bytes[4..8])? != 6 {
        return Err("invalid header".into());
    }
    let format = be_u16(&bytes[8..10])?;
    let division = be_u16(&bytes[12..14])?;
    let count = usize::from(be_u16(&bytes[10..12])?);
    let mut cursor = 14;
    let mut tracks = Vec::with_capacity(count);
    for _ in 0..count {
        let len = track_length(bytes, cursor)?;
        let end = cursor + 8 + len;
        tracks.push(parse_track(
            bytes.get(cursor + 8..end).ok_or("track bounds")?,
        )?);
        cursor = end;
    }
    if cursor != bytes.len() {
        return Err("trailing bytes".into());
    }
    Ok(Smf {
        format,
        division,
        tracks,
    })
}

fn track_length(bytes: &[u8], cursor: usize) -> Result<usize, String> {
    if bytes.get(cursor..cursor + 4) != Some(b"MTrk") {
        return Err("missing track".into());
    }
    usize::try_from(be_u32(bytes.get(cursor + 4..cursor + 8).ok_or("length")?)?)
        .map_err(|_| "length overflow".into())
}

fn parse_track(payload: &[u8]) -> Result<Track, String> {
    let mut track = Track::default();
    let mut cursor = 0;
    let mut tick = 0_u32;
    let mut running = None;
    let mut open: HashMap<(u8, u8), Vec<(u32, u8)>> = HashMap::new();
    while cursor < payload.len() {
        let (delta, used) = vlq(payload, cursor)?;
        cursor += used;
        tick = tick.checked_add(delta).ok_or("tick overflow")?;
        let first = *payload.get(cursor).ok_or("event")?;
        if first == 0xff {
            running = None;
            let meta = *payload.get(cursor + 1).ok_or("meta")?;
            let (len, n) = vlq(payload, cursor + 2)?;
            let start = cursor + 2 + n;
            let end = start + len as usize;
            let data = payload.get(start..end).ok_or("meta bounds")?;
            if meta == 0x03 {
                track.name = Some(data.to_vec());
            } else if meta == 0x51 && data.len() == 3 {
                track
                    .tempos
                    .push((tick, u32::from_be_bytes([0, data[0], data[1], data[2]])));
            } else if meta == 0x58 && data.len() == 4 {
                track
                    .meters
                    .push((tick, data[0], data[1], data[2], data[3]));
            }
            cursor = end;
            continue;
        }
        if first == 0xf0 || first == 0xf7 {
            running = None;
            let (len, n) = vlq(payload, cursor + 1)?;
            cursor += 1 + n + len as usize;
            continue;
        }
        let (status, data_start) = if first & 0x80 != 0 {
            running = Some(first);
            (first, cursor + 1)
        } else {
            (running.ok_or("running status")?, cursor)
        };
        let kind = status >> 4;
        let channel = status & 0x0f;
        let n = if kind == 0xc || kind == 0xd { 1 } else { 2 };
        let data = payload
            .get(data_start..data_start + n)
            .ok_or("channel bounds")?;
        cursor = data_start + n;
        match kind {
            0x9 if data[1] != 0 => open
                .entry((channel, data[0]))
                .or_default()
                .push((tick, data[1])),
            0x8 | 0x9 => {
                if let Some(queue) = open.get_mut(&(channel, data[0])) {
                    if let Some((start, attack)) = queue.first().copied() {
                        queue.remove(0);
                        track.notes.push(Note {
                            start,
                            end: tick,
                            channel: channel + 1,
                            pitch: data[0],
                            attack,
                            release: data[1],
                        });
                    }
                }
            }
            0xc => track.events.push(ChannelEvent {
                tick,
                channel: channel + 1,
                kind,
                a: data[0],
                b: None,
            }),
            0xb | 0xe => track.events.push(ChannelEvent {
                tick,
                channel: channel + 1,
                kind,
                a: data[0],
                b: Some(data[1]),
            }),
            0xd => track.events.push(ChannelEvent {
                tick,
                channel: channel + 1,
                kind,
                a: data[0],
                b: None,
            }),
            _ => {}
        }
    }
    track.notes.sort_by_key(|n| (n.start, n.channel, n.pitch));
    Ok(track)
}

fn vlq(bytes: &[u8], offset: usize) -> Result<(u32, usize), String> {
    let mut value = 0;
    for i in 0..4 {
        let b = *bytes.get(offset + i).ok_or("vlq")?;
        value = (value << 7) | u32::from(b & 0x7f);
        if b & 0x80 == 0 {
            return Ok((value, i + 1));
        }
    }
    Err("vlq overflow".into())
}
fn be_u16(bytes: &[u8]) -> Result<u16, String> {
    Ok(u16::from_be_bytes(bytes.try_into().map_err(|_| "u16")?))
}
fn be_u32(bytes: &[u8]) -> Result<u32, String> {
    Ok(u32::from_be_bytes(bytes.try_into().map_err(|_| "u32")?))
}
fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
