use phoenix::{
    midi_export::{
        ChannelAssignment, ChannelAssignmentProvenance, DecodedExportEvent, DecodedExportEventKind,
        ExportCounts, MeterPolicy, MidiExportError, PatchPolicy, PatchTranslation, TimingPolicy,
    },
    multitrack_export::{
        assemble_multitrack_sequence, MultitrackExportError, MultitrackSequenceInput,
        MusicalTrackInput,
    },
    smf::{MidiChannel, SmfSerializeError},
};

fn assignment(channel: u8) -> ChannelAssignment {
    ChannelAssignment {
        channel: MidiChannel::new(channel).unwrap(),
        provenance: ChannelAssignmentProvenance::Synthetic,
    }
}

fn event(tick: u32, ordinal: u64, kind: DecodedExportEventKind) -> DecodedExportEvent {
    DecodedExportEvent {
        absolute_position: tick,
        source_ordinal: ordinal,
        source_range: None,
        kind,
    }
}

fn note(
    tick: u32,
    ordinal: u64,
    pitch: u8,
    attack: u8,
    release: u8,
    duration: u32,
) -> DecodedExportEvent {
    event(
        tick,
        ordinal,
        DecodedExportEventKind::Note {
            pitch,
            attack_velocity: attack,
            release_velocity: release,
            duration,
        },
    )
}

fn track<'a>(
    context: &'a str,
    name: &'a [u8],
    channel: u8,
    events: &'a [DecodedExportEvent],
) -> MusicalTrackInput<'a> {
    MusicalTrackInput {
        context,
        name,
        channel_assignment: assignment(channel),
        events,
        patch_policy: PatchPolicy::StrictKnownOnly,
    }
}

fn sequence<'a>(tracks: &'a [MusicalTrackInput<'a>]) -> MultitrackSequenceInput<'a> {
    MultitrackSequenceInput {
        sequence_name: b"Synthetic Suite",
        tempo_mpqn: 500_000,
        meter_values: (4, 2, 8, 8),
        timing_policy: TimingPolicy::Identity480,
        meter_policy: MeterPolicy::HistoricalWhenKnownOtherwiseStandard,
        tracks,
    }
}

#[derive(Clone, Debug)]
struct ParsedTrack {
    events: Vec<ParsedEvent>,
}

#[derive(Clone, Debug)]
enum ParsedEvent {
    Meta {
        tick: u32,
        kind: u8,
        data: Vec<u8>,
    },
    Channel {
        tick: u32,
        status: u8,
        data: Vec<u8>,
    },
}

#[derive(Clone, Debug)]
struct ParsedSmf {
    format: u16,
    division: u16,
    declared_tracks: usize,
    tracks: Vec<ParsedTrack>,
}

fn parse_smf(bytes: &[u8]) -> Result<ParsedSmf, String> {
    if bytes.len() < 14 || &bytes[..4] != b"MThd" || be_u32(&bytes[4..8])? != 6 {
        return Err("invalid header".into());
    }
    let format = be_u16(&bytes[8..10])?;
    let declared_tracks = usize::from(be_u16(&bytes[10..12])?);
    let division = be_u16(&bytes[12..14])?;
    let mut cursor = 14;
    let mut tracks = Vec::new();
    while cursor < bytes.len() {
        if bytes.get(cursor..cursor + 4) != Some(b"MTrk") {
            return Err(format!("missing MTrk at {cursor}"));
        }
        let length = usize::try_from(be_u32(
            bytes
                .get(cursor + 4..cursor + 8)
                .ok_or("truncated length")?,
        )?)
        .map_err(|_| "track length overflow")?;
        let start = cursor + 8;
        let end = start.checked_add(length).ok_or("track length overflow")?;
        tracks.push(parse_track(
            bytes.get(start..end).ok_or("track exceeds file")?,
        )?);
        cursor = end;
    }
    if cursor != bytes.len() || tracks.len() != declared_tracks {
        return Err("track count or EOF mismatch".into());
    }
    Ok(ParsedSmf {
        format,
        division,
        declared_tracks,
        tracks,
    })
}

fn parse_track(bytes: &[u8]) -> Result<ParsedTrack, String> {
    let mut cursor = 0;
    let mut tick = 0_u32;
    let mut running_status = None;
    let mut events = Vec::new();
    let mut eot_count = 0;
    while cursor < bytes.len() {
        let (delta, width) = read_vlq(bytes, cursor)?;
        cursor += width;
        tick = tick.checked_add(delta).ok_or("tick overflow")?;
        let first = *bytes.get(cursor).ok_or("missing event")?;
        if first == 0xff {
            running_status = None;
            let kind = *bytes.get(cursor + 1).ok_or("missing meta kind")?;
            let (length, length_width) = read_vlq(bytes, cursor + 2)?;
            let start = cursor + 2 + length_width;
            let end = start
                .checked_add(usize::try_from(length).map_err(|_| "meta length overflow")?)
                .ok_or("meta length overflow")?;
            let data = bytes.get(start..end).ok_or("meta exceeds track")?.to_vec();
            events.push(ParsedEvent::Meta { tick, kind, data });
            cursor = end;
            if kind == 0x2f {
                eot_count += 1;
                if !matches!(events.last(), Some(ParsedEvent::Meta { data, .. }) if data.is_empty())
                    || cursor != bytes.len()
                {
                    return Err("invalid or non-final EOT".into());
                }
            }
            continue;
        }
        let (status, start) = if first & 0x80 != 0 {
            running_status = Some(first);
            (first, cursor + 1)
        } else {
            (running_status.ok_or("data without running status")?, cursor)
        };
        if status >= 0xf0 {
            return Err("unsupported system event".into());
        }
        let length = if matches!(status >> 4, 0xc | 0xd) {
            1
        } else {
            2
        };
        let data = bytes
            .get(start..start + length)
            .ok_or("channel event exceeds track")?;
        if data.iter().any(|byte| byte & 0x80 != 0) {
            return Err("invalid data byte".into());
        }
        events.push(ParsedEvent::Channel {
            tick,
            status,
            data: data.to_vec(),
        });
        cursor = start + length;
    }
    if eot_count != 1 {
        return Err(format!("expected one EOT, got {eot_count}"));
    }
    Ok(ParsedTrack { events })
}

fn read_vlq(bytes: &[u8], offset: usize) -> Result<(u32, usize), String> {
    let mut value = 0;
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

fn name(track: &ParsedTrack) -> Option<&[u8]> {
    track.events.iter().find_map(|event| match event {
        ParsedEvent::Meta {
            kind: 0x03, data, ..
        } => Some(data.as_slice()),
        _ => None,
    })
}

fn meta(track: &ParsedTrack, kind: u8) -> Option<(u32, &[u8])> {
    track.events.iter().find_map(|event| match event {
        ParsedEvent::Meta {
            tick,
            kind: observed,
            data,
        } if *observed == kind => Some((*tick, data.as_slice())),
        _ => None,
    })
}

fn channels(track: &ParsedTrack) -> Vec<u8> {
    let mut channels: Vec<_> = track
        .events
        .iter()
        .filter_map(|event| match event {
            ParsedEvent::Channel { status, .. } => Some((status & 0x0f) + 1),
            _ => None,
        })
        .collect();
    channels.sort_unstable();
    channels.dedup();
    channels
}

fn eot_tick(track: &ParsedTrack) -> u32 {
    track
        .events
        .iter()
        .find_map(|event| match event {
            ParsedEvent::Meta {
                tick, kind: 0x2f, ..
            } => Some(*tick),
            _ => None,
        })
        .unwrap()
}

#[test]
fn one_musical_track_produces_format_one_conductor_plus_track() {
    let notes = [note(120, 0, 60, 100, 45, 240)];
    let tracks = [track("one", b"Piano", 1, &notes)];
    let result = assemble_multitrack_sequence(&sequence(&tracks)).unwrap();
    let parsed = parse_smf(&result.smf_bytes).unwrap();

    assert_eq!((parsed.format, parsed.division), (1, 480));
    assert_eq!((parsed.declared_tracks, parsed.tracks.len()), (2, 2));
    assert_eq!(name(&parsed.tracks[0]), Some(b"Synthetic Suite".as_slice()));
    assert_eq!(
        meta(&parsed.tracks[0], 0x51),
        Some((0, &[0x07, 0xa1, 0x20][..]))
    );
    assert_eq!(meta(&parsed.tracks[0], 0x58), Some((0, &[4, 2, 24, 8][..])));
    assert_eq!(eot_tick(&parsed.tracks[0]), 0);
    assert_eq!(name(&parsed.tracks[1]), Some(b"Piano".as_slice()));
    assert_eq!(channels(&parsed.tracks[1]), [1]);
    assert_eq!(result.report.musical_track_count, 1);
    assert_eq!(result.report.total_smf_track_count, 2);
}

#[test]
fn multiple_tracks_preserve_order_and_all_supported_families() {
    let piano_events = [
        note(100, 0, 60, 101, 37, 300),
        event(
            90,
            1,
            DecodedExportEventKind::Controller {
                number: 7,
                value: 110,
                has_opaque_context: true,
            },
        ),
    ];
    let bass_events = [
        event(
            0,
            0,
            DecodedExportEventKind::Patch {
                program: 33,
                translation: PatchTranslation::ConfirmedBankSelect { msb: 4, lsb: 2 },
            },
        ),
        event(20, 1, DecodedExportEventKind::PitchBend { lsb: 1, msb: 64 }),
    ];
    let lead_events = [
        event(
            10,
            0,
            DecodedExportEventKind::Patch {
                program: 81,
                translation: PatchTranslation::ProgramOnlyConfirmed,
            },
        ),
        event(30, 1, DecodedExportEventKind::ChannelPressure { value: 72 }),
    ];
    let tracks = [
        track("piano-id", b"Piano", 1, &piano_events),
        track("bass-id", b"Bass", 2, &bass_events),
        track("lead-id", b"Lead", 16, &lead_events),
    ];
    let result = assemble_multitrack_sequence(&sequence(&tracks)).unwrap();
    let parsed = parse_smf(&result.smf_bytes).unwrap();

    assert_eq!(parsed.declared_tracks, 4);
    assert_eq!(
        parsed.tracks.iter().map(name).collect::<Vec<_>>(),
        [
            Some(b"Synthetic Suite".as_slice()),
            Some(b"Piano".as_slice()),
            Some(b"Bass".as_slice()),
            Some(b"Lead".as_slice()),
        ]
    );
    assert_eq!(channels(&parsed.tracks[1]), [1]);
    assert_eq!(channels(&parsed.tracks[2]), [2]);
    assert_eq!(channels(&parsed.tracks[3]), [16]);

    let counts = &result.report.totals;
    assert_eq!(
        counts,
        &ExportCounts {
            notes: 1,
            generated_note_offs: 1,
            controllers: 1,
            bank_select_msb: 1,
            bank_select_lsb: 1,
            program_changes: 2,
            channel_pressure: 1,
            pitch_bend: 1,
            tempo: 1,
            meter: 1,
        }
    );
    assert_eq!(result.report.untranslated_metadata.len(), 1);
    assert_eq!(result.report.tracks.len(), 3);
    let mut summed = ExportCounts {
        tempo: 1,
        meter: 1,
        ..ExportCounts::default()
    };
    for report in &result.report.tracks {
        summed.add_assign(&report.counts);
    }
    assert_eq!(summed, result.report.totals);

    assert!(parsed.tracks[1].events.iter().any(|event| matches!(
        event,
        ParsedEvent::Channel { tick: 400, status: 0x80, data }
            if data == &[60, 37]
    )));
    assert_eq!(eot_tick(&parsed.tracks[1]), 400);
    assert!(parsed.tracks[2].events.iter().any(|event| matches!(
        event,
        ParsedEvent::Channel { tick: 0, status: 0xb1, data }
            if data == &[0, 4]
    )));
    assert!(parsed.tracks[3].events.iter().any(|event| matches!(
        event,
        ParsedEvent::Channel { tick: 30, status: 0xdf, data }
            if data == &[72]
    )));
}

#[test]
fn empty_duplicate_name_and_duplicate_channel_tracks_remain_distinct() {
    let empty: [DecodedExportEvent; 0] = [];
    let first_events = [note(0, 0, 48, 90, 40, 10)];
    let second_events = [note(20, 0, 50, 91, 41, 10)];
    let tracks = [
        track("empty", b"Drums", 10, &empty),
        track("first", b"Lead", 5, &first_events),
        track("second", b"Lead", 5, &second_events),
    ];
    let result = assemble_multitrack_sequence(&sequence(&tracks)).unwrap();
    let parsed = parse_smf(&result.smf_bytes).unwrap();

    assert_eq!(parsed.tracks.len(), 4);
    assert_eq!(name(&parsed.tracks[1]), Some(b"Drums".as_slice()));
    assert_eq!(eot_tick(&parsed.tracks[1]), 0);
    assert!(channels(&parsed.tracks[1]).is_empty());
    assert_eq!(result.report.tracks[0].counts, ExportCounts::default());
    assert_eq!(name(&parsed.tracks[2]), Some(b"Lead".as_slice()));
    assert_eq!(name(&parsed.tracks[3]), Some(b"Lead".as_slice()));
    assert_eq!(channels(&parsed.tracks[2]), [5]);
    assert_eq!(channels(&parsed.tracks[3]), [5]);
    assert_eq!(result.report.tracks[1].context, "first");
    assert_eq!(result.report.tracks[2].context, "second");
}

#[test]
fn later_track_failure_aborts_the_complete_sequence_with_context() {
    let valid = [note(0, 0, 60, 100, 64, 10)];
    let invalid = [note(0, 0, 128, 100, 64, 10)];
    let tracks = [
        track("valid-first", b"Piano", 1, &valid),
        track("invalid-second", b"Bass", 2, &invalid),
    ];

    assert_eq!(
        assemble_multitrack_sequence(&sequence(&tracks)),
        Err(MultitrackExportError::MusicalTrackAdaptation {
            track_index: 1,
            context: "invalid-second".into(),
            source: MidiExportError::InvalidMidiValue {
                source_ordinal: Some(0),
                source_range: None,
                source: SmfSerializeError::InvalidDataByte { value: 128 },
            },
        })
    );
}

#[test]
fn unsupported_patch_aborts_the_complete_sequence() {
    let valid = [note(0, 0, 60, 100, 64, 10)];
    let unsupported = [event(
        0,
        0,
        DecodedExportEventKind::Patch {
            program: 10,
            translation: PatchTranslation::UnsupportedOpaque,
        },
    )];
    let tracks = [
        track("valid", b"Piano", 1, &valid),
        track("opaque-patch", b"Bass", 2, &unsupported),
    ];

    assert!(matches!(
        assemble_multitrack_sequence(&sequence(&tracks)),
        Err(MultitrackExportError::MusicalTrackAdaptation {
            track_index: 1,
            context,
            source: MidiExportError::UnsupportedPatchTranslation { source_ordinal: 0, .. },
        }) if context == "opaque-patch"
    ));
}

#[test]
fn conductor_failure_aborts_before_track_results_exist() {
    let notes = [note(0, 0, 60, 100, 64, 10)];
    let tracks = [track("never-returned", b"Piano", 1, &notes)];
    let mut input = sequence(&tracks);
    input.tempo_mpqn = 0;

    assert!(matches!(
        assemble_multitrack_sequence(&input),
        Err(MultitrackExportError::ConductorAdaptation {
            source: MidiExportError::InvalidMidiValue {
                source: SmfSerializeError::InvalidTempo { mpqn: 0 },
                ..
            }
        })
    ));
}
