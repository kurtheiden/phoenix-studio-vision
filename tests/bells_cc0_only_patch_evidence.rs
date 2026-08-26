use phoenix::{
    midi_export::{
        adapt_track, ChannelAssignment, ChannelAssignmentProvenance, DecodedExportEvent,
        DecodedExportEventKind, PatchPolicy, PatchTranslation, TimingPolicy,
    },
    mixed_event::{
        walk_bounded_mixed_events, MixedEventBounds, MixedEventItem, MixedEventTimingBasis,
    },
    sequence_container::parse_project_166,
    smf::{serialize_musical_track, MidiChannel},
};
use sha2::{Digest, Sha256};
use std::{fs, ops::Range, path::Path};

const PROJECT: &str = "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline";
const MIDI: &str = "/Users/kurtheiden/Documents/Phoenix Research/Studio Vision MIDI Exports/Project 001/newest STUFF - Bells for her - provenance multitrack";
const PROJECT_SHA256: &str = "e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132";
const MIDI_SHA256: &str = "ffbdbb6be208a2d607c9b0c55a12b72226a18d43b9494c2b46b058d4568fc2c3";

#[derive(Clone, Debug, PartialEq, Eq)]
struct ChannelEvent {
    tick: u32,
    status: u8,
    data: Vec<u8>,
}

#[derive(Clone, Debug)]
struct ParsedTrack {
    name: Option<Vec<u8>>,
    channel_events: Vec<ChannelEvent>,
}

#[derive(Clone, Debug)]
struct Case {
    ordinal: usize,
    label: &'static [u8],
    event_range: Range<usize>,
    tick: u32,
    channel: u8,
    program: u8,
}

fn cases() -> [Case; 4] {
    [
        Case {
            ordinal: 3,
            label: b"Track 3",
            event_range: 0x010a4d..0x0110c8,
            tick: 480,
            channel: 16,
            program: 25,
        },
        Case {
            ordinal: 6,
            label: b"Track 6",
            event_range: 0x011eac..0x0123dd,
            tick: 290,
            channel: 1,
            program: 35,
        },
        Case {
            ordinal: 8,
            label: b"Track 8",
            event_range: 0x012a9a..0x01425a,
            tick: 370,
            channel: 16,
            program: 25,
        },
        Case {
            ordinal: 9,
            label: b"Track 9",
            event_range: 0x0143c8..0x014957,
            tick: 1_920,
            channel: 12,
            program: 122,
        },
    ]
}

#[test]
fn authentic_bells_four_tracks_establish_explicit_cc0_only_policy() {
    let project_path = Path::new(PROJECT);
    let midi_path = Path::new(MIDI);
    if !project_path.is_file() || !midi_path.is_file() {
        return;
    }
    let project_bytes = fs::read(project_path).expect("authentic project");
    let midi_bytes = fs::read(midi_path).expect("authenticated Bells MIDI");
    assert_eq!(sha256_hex(&project_bytes), PROJECT_SHA256);
    assert_eq!(sha256_hex(&midi_bytes), MIDI_SHA256);

    let project = parse_project_166(&project_bytes).expect("Descriptor166 project");
    let bells = project
        .sequences
        .iter()
        .find(|sequence| sequence.sequence_name.as_utf8() == Some("Bells for her"))
        .expect("Bells sequence");
    let reference_tracks = parse_smf(&midi_bytes).expect("authenticated Bells SMF");

    for case in cases() {
        let pair = &bells.track_pairs[case.ordinal - 1];
        let bounds = pair.validated_event_bounds().expect("exact event bounds");
        assert_eq!(bounds.event_range, case.event_range);
        let walk = walk_bounded_mixed_events(
            &project_bytes,
            MixedEventBounds {
                event_range: bounds.event_range,
            },
            MixedEventTimingBasis::default(),
        )
        .expect("exact mixed-event walk");
        let patches = walk
            .items
            .iter()
            .filter_map(|item| match item {
                MixedEventItem::Patch(patch) => {
                    Some((patch.position, patch.patch.program_change.value))
                }
                MixedEventItem::PatchToNote(transition) => Some((
                    transition.patch_position,
                    transition.patch.program_change.value,
                )),
                MixedEventItem::Event(_) => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(patches, [(case.tick, case.program)]);

        let reference = reference_tracks
            .iter()
            .find(|track| track.name.as_deref() == Some(case.label))
            .expect("authenticated named SMF track");
        let expected = expected_patch_events(&case);
        assert_eq!(patch_inventory(reference), expected);

        let event = DecodedExportEvent {
            absolute_position: case.tick,
            source_ordinal: 0,
            source_range: Some(case.event_range.clone()),
            kind: DecodedExportEventKind::Patch {
                program: case.program,
                translation: PatchTranslation::ConfirmedBankSelectMsb { msb: 81 },
            },
        };
        let adapted = adapt_track(
            &[event],
            Some(ChannelAssignment {
                channel: MidiChannel::new(case.channel).expect("authenticated channel"),
                provenance: ChannelAssignmentProvenance::AuthenticatedOverride,
            }),
            TimingPolicy::Identity480,
            PatchPolicy::StrictKnownOnly,
        )
        .expect("explicit authenticated policy adapts");
        assert_eq!(adapted.counts.bank_select_msb, 1);
        assert_eq!(adapted.counts.bank_select_lsb, 0);
        assert_eq!(adapted.counts.program_changes, 1);
        assert_eq!(
            adapted.channel_assignment.provenance,
            ChannelAssignmentProvenance::AuthenticatedOverride
        );

        let serialized = serialize_musical_track(&adapted.scheduled_events).expect("serialize");
        let generated = parse_track_chunk(serialized.as_bytes()).expect("generated track");
        assert_eq!(patch_inventory(&generated), expected);
    }
}

fn expected_patch_events(case: &Case) -> Vec<ChannelEvent> {
    let channel_nibble = case.channel - 1;
    vec![
        ChannelEvent {
            tick: case.tick,
            status: 0xb0 | channel_nibble,
            data: vec![0, 81],
        },
        ChannelEvent {
            tick: case.tick,
            status: 0xc0 | channel_nibble,
            data: vec![case.program],
        },
    ]
}

fn patch_inventory(track: &ParsedTrack) -> Vec<ChannelEvent> {
    track
        .channel_events
        .iter()
        .filter(|event| {
            event.status >> 4 == 0xc
                || (event.status >> 4 == 0xb && matches!(event.data.as_slice(), [0, _] | [32, _]))
        })
        .cloned()
        .collect()
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn parse_smf(bytes: &[u8]) -> Result<Vec<ParsedTrack>, String> {
    if bytes.len() < 14 || &bytes[..4] != b"MThd" || be_u32(&bytes[4..8])? != 6 {
        return Err("invalid SMF header".into());
    }
    if be_u16(&bytes[8..10])? != 1 || be_u16(&bytes[12..14])? != 480 {
        return Err("unexpected SMF format/division".into());
    }
    let count = usize::from(be_u16(&bytes[10..12])?);
    let mut cursor = 14;
    let mut tracks = Vec::with_capacity(count);
    for _ in 0..count {
        let length = track_length(bytes, cursor)?;
        let end = cursor + 8 + length;
        tracks.push(parse_track_payload(
            bytes.get(cursor + 8..end).ok_or("track exceeds SMF")?,
        )?);
        cursor = end;
    }
    if cursor != bytes.len() {
        return Err("trailing SMF bytes".into());
    }
    Ok(tracks)
}

fn parse_track_chunk(bytes: &[u8]) -> Result<ParsedTrack, String> {
    let length = track_length(bytes, 0)?;
    if length + 8 != bytes.len() {
        return Err("track chunk length mismatch".into());
    }
    parse_track_payload(&bytes[8..])
}

fn track_length(bytes: &[u8], cursor: usize) -> Result<usize, String> {
    if bytes.get(cursor..cursor + 4) != Some(b"MTrk") {
        return Err("missing MTrk".into());
    }
    usize::try_from(be_u32(
        bytes
            .get(cursor + 4..cursor + 8)
            .ok_or("missing track length")?,
    )?)
    .map_err(|_| "track length overflow".into())
}

fn parse_track_payload(payload: &[u8]) -> Result<ParsedTrack, String> {
    let mut cursor = 0;
    let mut tick = 0_u32;
    let mut running_status = None;
    let mut name = None;
    let mut channel_events = Vec::new();
    while cursor < payload.len() {
        let (delta, consumed) = read_vlq(payload, cursor)?;
        cursor += consumed;
        tick = tick.checked_add(delta).ok_or("tick overflow")?;
        let first = *payload.get(cursor).ok_or("missing event")?;
        if first == 0xff {
            running_status = None;
            let kind = *payload.get(cursor + 1).ok_or("missing meta type")?;
            let (length, length_bytes) = read_vlq(payload, cursor + 2)?;
            let start = cursor + 2 + length_bytes;
            let end = start + usize::try_from(length).map_err(|_| "meta length overflow")?;
            let data = payload.get(start..end).ok_or("meta exceeds track")?;
            if kind == 0x03 {
                name = Some(data.to_vec());
            }
            cursor = end;
            continue;
        }
        if first == 0xf0 || first == 0xf7 {
            running_status = None;
            let (length, length_bytes) = read_vlq(payload, cursor + 1)?;
            cursor += 1 + length_bytes + usize::try_from(length).map_err(|_| "SysEx length")?;
            continue;
        }
        let (status, data_start) = if first & 0x80 != 0 {
            running_status = Some(first);
            (first, cursor + 1)
        } else {
            (running_status.ok_or("data without running status")?, cursor)
        };
        let length = if matches!(status >> 4, 0xc | 0xd) {
            1
        } else {
            2
        };
        let end = data_start + length;
        let data = payload
            .get(data_start..end)
            .ok_or("channel event exceeds track")?;
        channel_events.push(ChannelEvent {
            tick,
            status,
            data: data.to_vec(),
        });
        cursor = end;
    }
    Ok(ParsedTrack {
        name,
        channel_events,
    })
}

fn read_vlq(bytes: &[u8], offset: usize) -> Result<(u32, usize), String> {
    let mut value = 0_u32;
    for index in 0..4 {
        let byte = *bytes.get(offset + index).ok_or("truncated VLQ")?;
        value = (value << 7) | u32::from(byte & 0x7f);
        if byte & 0x80 == 0 {
            return Ok((value, index + 1));
        }
    }
    Err("VLQ exceeds four bytes".into())
}

fn be_u16(bytes: &[u8]) -> Result<u16, String> {
    Ok(u16::from_be_bytes(
        bytes.try_into().map_err(|_| "invalid u16")?,
    ))
}

fn be_u32(bytes: &[u8]) -> Result<u32, String> {
    Ok(u32::from_be_bytes(
        bytes.try_into().map_err(|_| "invalid u32")?,
    ))
}
