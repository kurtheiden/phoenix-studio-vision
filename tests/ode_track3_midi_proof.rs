use phoenix::{
    meter::{decode_bounded_initial_meter, InitialMeterBounds},
    midi_export::{
        adapt_conductor, adapt_track, ChannelAssignment, ChannelAssignmentProvenance,
        DecodedExportEvent, MeterPolicy, MidiExportError, PatchPolicy, PatchTranslation,
        TimingPolicy,
    },
    mixed_event::{
        walk_bounded_mixed_events, MixedEventBounds, MixedEventItem, MixedEventKind,
        MixedEventTimingBasis, MixedEventWalkError,
    },
    sequence_container::{parse_project_166, TrackAssociations},
    smf::{
        serialize_conductor_track, serialize_format1, serialize_named_musical_track, MidiChannel,
        SmfSerializeError,
    },
    tempo::{decode_bounded_initial_tempo, InitialTempoBounds},
};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, VecDeque},
    fmt, fs,
    ops::Range,
    path::{Path, PathBuf},
};

const PROJECT_PATH: &str = "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline";
const REFERENCE_PATH: &str = "/Users/kurtheiden/Documents/Phoenix Research/Studio Vision MIDI Exports/Project 001/Ode to Clarke Multi All";
const PROOF_PATH: &str = "/Users/kurtheiden/Documents/Phoenix Research/Phoenix MIDI Proofs/Ode to Clarke - Track 3 - Phoenix Proof.mid";
const PROJECT_SHA256: &str = "e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132";
const REFERENCE_SHA256: &str = "4f63b34ef92204d4bc5eeb78dbbe7b94d005c1f9ceb57ea0f9809533ad590f29";

#[derive(Clone, Debug)]
struct ProofManifest {
    project_sha256: &'static str,
    sequence_range: Range<usize>,
    sequence_name: &'static [u8],
    sequence_name_range: Range<usize>,
    descriptor_ordinal: usize,
    descriptor_range: Range<usize>,
    pair_ordinal: usize,
    primary_range: Range<usize>,
    event_range: Range<usize>,
    channel: u8,
}

fn manifest() -> ProofManifest {
    ProofManifest {
        project_sha256: PROJECT_SHA256,
        sequence_range: 0x02ef6f..0x03202c,
        sequence_name: b"Ode to Clarke",
        sequence_name_range: 0x02f753..0x02f760,
        descriptor_ordinal: 7,
        descriptor_range: 0x02f4c9..0x02f56f,
        pair_ordinal: 5,
        primary_range: 0x0312ed..0x03156b,
        event_range: 0x031300..0x031564,
        channel: 1,
    }
}

#[derive(Debug)]
enum ProofError {
    Io(std::io::Error),
    ArtifactHashMismatch {
        artifact: &'static str,
        observed: String,
    },
    SequenceIdentityMismatch(String),
    TrackIdentityMismatch(String),
    ManifestMismatch(String),
    ExactEventRangeMismatch(String),
    MixedWalk(MixedEventWalkError),
    Adaptation(MidiExportError),
    Serialization(SmfSerializeError),
    ReferenceParse(String),
    ComparisonMismatch(String),
}

impl fmt::Display for ProofError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "proof file I/O failure: {error}"),
            Self::ArtifactHashMismatch { artifact, observed } => {
                write!(formatter, "{artifact} hash mismatch: {observed}")
            }
            Self::SequenceIdentityMismatch(detail) => {
                write!(formatter, "sequence identity mismatch: {detail}")
            }
            Self::TrackIdentityMismatch(detail) => {
                write!(formatter, "track identity mismatch: {detail}")
            }
            Self::ManifestMismatch(detail) => write!(formatter, "manifest mismatch: {detail}"),
            Self::ExactEventRangeMismatch(detail) => {
                write!(formatter, "exact event-range mismatch: {detail}")
            }
            Self::MixedWalk(error) => write!(formatter, "mixed walker failed: {error}"),
            Self::Adaptation(error) => write!(formatter, "MIDI adaptation failed: {error}"),
            Self::Serialization(error) => write!(formatter, "SMF serialization failed: {error}"),
            Self::ReferenceParse(detail) => {
                write!(formatter, "independent SMF parse failed: {detail}")
            }
            Self::ComparisonMismatch(detail) => {
                write!(formatter, "reference comparison mismatch: {detail}")
            }
        }
    }
}

impl std::error::Error for ProofError {}

impl From<std::io::Error> for ProofError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<MixedEventWalkError> for ProofError {
    fn from(value: MixedEventWalkError) -> Self {
        Self::MixedWalk(value)
    }
}

impl From<MidiExportError> for ProofError {
    fn from(value: MidiExportError) -> Self {
        Self::Adaptation(value)
    }
}

impl From<SmfSerializeError> for ProofError {
    fn from(value: SmfSerializeError) -> Self {
        Self::Serialization(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ParsedSmf {
    format: u16,
    division: u16,
    tracks: Vec<ParsedTrack>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ParsedTrack {
    events: Vec<ParsedEvent>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct NormalizedNote {
    channel: u8,
    pitch: u8,
    start: u32,
    end: u32,
    attack_velocity: u8,
    release_velocity: Option<u8>,
}

#[derive(Clone, Debug)]
struct ProofOutcome {
    bytes: Vec<u8>,
    note_count: usize,
    explicit_reference_releases: usize,
    zero_velocity_reference_releases: usize,
    tempo_mpqn: u32,
    meter: [u8; 4],
}

#[test]
fn authentic_ode_track3_matches_studio_vision_export() {
    if !Path::new(PROJECT_PATH).exists() || !Path::new(REFERENCE_PATH).exists() {
        eprintln!("skipping authentic Ode Track 3 proof: external fixtures are absent");
        return;
    }

    let outcome = build_and_compare_proof().unwrap_or_else(|error| panic!("{error:#?}"));
    assert_eq!(outcome.note_count, 84);
    assert_eq!(outcome.tempo_mpqn, 500_000);
    assert_eq!(outcome.meter, [4, 2, 24, 8]);
    assert_eq!(
        outcome.explicit_reference_releases + outcome.zero_velocity_reference_releases,
        outcome.note_count
    );
    eprintln!(
        "Track 3 proof: {} notes; reference endings {} explicit 8n, {} Note On velocity zero",
        outcome.note_count,
        outcome.explicit_reference_releases,
        outcome.zero_velocity_reference_releases
    );

    if std::env::var_os("PHOENIX_WRITE_ODE_TRACK3_PROOF").is_some() {
        let path = PathBuf::from(PROOF_PATH);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, &outcome.bytes).unwrap();
        let reopened = fs::read(&path).unwrap();
        assert_eq!(reopened, outcome.bytes);
        let reparsed = parse_smf(&reopened).unwrap();
        assert_eq!(reparsed.format, 1);
        assert_eq!(reparsed.division, 480);
        assert_eq!(reparsed.tracks.len(), 2);
        assert_eq!(track_name(&reparsed.tracks[1]), Some(b"Track 3".as_slice()));
        assert_eq!(normalize_notes(&reparsed.tracks[1]).unwrap().len(), 84);
        eprintln!(
            "wrote {} bytes sha256={} to {}",
            reopened.len(),
            sha256_hex(&reopened),
            path.display()
        );
    }
}

fn build_and_compare_proof() -> Result<ProofOutcome, ProofError> {
    let manifest = manifest();
    let project_bytes = fs::read(PROJECT_PATH)?;
    validate_hash("project", &project_bytes, manifest.project_sha256)?;
    let reference_bytes = fs::read(REFERENCE_PATH)?;
    validate_hash("reference MIDI", &reference_bytes, REFERENCE_SHA256)?;

    let project = parse_project_166(&project_bytes)
        .map_err(|error| ProofError::SequenceIdentityMismatch(format!("{error:?}")))?;
    let matching_sequences: Vec<_> = project
        .sequences
        .iter()
        .filter(|sequence| sequence.sequence_name.bytes.bytes == manifest.sequence_name)
        .collect();
    if matching_sequences.len() != 1 {
        return Err(ProofError::SequenceIdentityMismatch(format!(
            "expected one structurally parsed Ode sequence, found {}",
            matching_sequences.len()
        )));
    }
    let sequence = matching_sequences[0];
    if sequence.sequence_range != manifest.sequence_range
        || sequence.sequence_name.bytes.range != manifest.sequence_name_range
    {
        return Err(ProofError::SequenceIdentityMismatch(format!(
            "range/name provenance differs: {:?} {:?}",
            sequence.sequence_range, sequence.sequence_name.bytes.range
        )));
    }

    let descriptor = sequence
        .descriptors
        .get(manifest.descriptor_ordinal)
        .ok_or_else(|| ProofError::TrackIdentityMismatch("descriptor ordinal absent".into()))?;
    if descriptor.ordinal != manifest.descriptor_ordinal
        || descriptor.range != manifest.descriptor_range
        || descriptor.label.as_ref().map(|label| label.bytes) != Some(b"Track 3".as_slice())
    {
        return Err(ProofError::TrackIdentityMismatch(format!(
            "descriptor provenance differs: {descriptor:?}"
        )));
    }
    let bindings = match &sequence.track_associations {
        TrackAssociations::Ordinal(bindings) => bindings,
        TrackAssociations::Unresolved { .. } => {
            return Err(ProofError::TrackIdentityMismatch(
                "descriptor/pair association is unresolved".into(),
            ));
        }
    };
    if !bindings.iter().any(|binding| {
        binding.descriptor_ordinal == manifest.descriptor_ordinal
            && binding.pair_ordinal == manifest.pair_ordinal
    }) {
        return Err(ProofError::TrackIdentityMismatch(
            "required ordinal descriptor/pair binding absent".into(),
        ));
    }
    let pair = sequence
        .track_pairs
        .get(manifest.pair_ordinal)
        .ok_or_else(|| ProofError::TrackIdentityMismatch("pair ordinal absent".into()))?;
    if pair.pair_ordinal != manifest.pair_ordinal
        || pair.primary.record_range != manifest.primary_range
    {
        return Err(ProofError::TrackIdentityMismatch(format!(
            "pair provenance differs: ordinal {} range {:?}",
            pair.pair_ordinal, pair.primary.record_range
        )));
    }
    let event_end = pair
        .primary
        .payload
        .range
        .end
        .checked_sub(7)
        .ok_or_else(|| ProofError::ExactEventRangeMismatch("primary tail underflow".into()))?;
    let event_range = pair.candidate_event_start..event_end;
    let tail = &project_bytes[event_end..pair.primary.payload.range.end];
    if tail.len() != 7
        || tail[0] != 0xff
        || tail[4..] != [0xff, 0x2f, 0x00]
        || event_range != manifest.event_range
    {
        return Err(ProofError::ExactEventRangeMismatch(format!(
            "derived range {event_range:?}, tail {tail:02x?}"
        )));
    }
    let channel = MidiChannel::new(manifest.channel)
        .map_err(|error| ProofError::ManifestMismatch(format!("invalid channel: {error:?}")))?;

    let meter = decode_bounded_initial_meter(
        &project_bytes,
        InitialMeterBounds {
            event_range: sequence.initial_meter_range.clone(),
        },
    )
    .map_err(|error| ProofError::SequenceIdentityMismatch(format!("Meter: {error:?}")))?;
    let tempo = decode_bounded_initial_tempo(
        &project_bytes,
        InitialTempoBounds {
            event_range: sequence.initial_tempo_range.clone(),
        },
    )
    .map_err(|error| ProofError::SequenceIdentityMismatch(format!("Tempo: {error:?}")))?;

    let walk = walk_bounded_mixed_events(
        &project_bytes,
        MixedEventBounds {
            event_range: event_range.clone(),
        },
        MixedEventTimingBasis {
            previous_event_position: 0,
        },
    )?;
    if walk.consumed_range != event_range {
        return Err(ProofError::ExactEventRangeMismatch(format!(
            "walker consumed {:?}",
            walk.consumed_range
        )));
    }

    let mut decoded = Vec::with_capacity(walk.logical_event_count());
    let mut ordinal = 0_u64;
    for item in &walk.items {
        match item {
            MixedEventItem::PatchToNote(transition) => {
                validate_track3_patch(transition)?;
                decoded.push(DecodedExportEvent::from_patch(
                    ordinal,
                    &transition.patch,
                    PatchTranslation::ConfirmedBankSelect { msb: 81, lsb: 2 },
                ));
                ordinal += 1;
                decoded.push(DecodedExportEvent::from_note_body(
                    transition.first_note_position,
                    ordinal,
                    &transition.first_note,
                ));
                ordinal += 1;
            }
            MixedEventItem::Event(event) => match &event.event {
                MixedEventKind::Note(note) => {
                    decoded.push(DecodedExportEvent::from_note(event.position, ordinal, note));
                    ordinal += 1;
                }
                other => {
                    return Err(ProofError::ComparisonMismatch(format!(
                        "unsupported Track 3 mixed-event family: {other:?}"
                    )));
                }
            },
        }
    }
    if decoded.len() != walk.logical_event_count() {
        return Err(ProofError::ComparisonMismatch(
            "flattened logical event count differs".into(),
        ));
    }

    let conductor = adapt_conductor(
        sequence.sequence_name.bytes.bytes,
        tempo.mpqn(),
        (
            meter.numerator.value,
            meter.denominator_exponent.value,
            meter.third_payload.value,
            meter.fourth_payload.value,
        ),
        TimingPolicy::Identity480,
        MeterPolicy::HistoricalWhenKnownOtherwiseStandard,
    )?;
    let adapted = adapt_track(
        &decoded,
        Some(ChannelAssignment {
            channel,
            provenance: ChannelAssignmentProvenance::AuthenticatedOverride,
        }),
        TimingPolicy::Identity480,
        PatchPolicy::StrictKnownOnly,
    )?;
    if adapted.counts.notes != 84
        || adapted.counts.generated_note_offs != 84
        || adapted.counts.bank_select_msb != 1
        || adapted.counts.bank_select_lsb != 1
        || adapted.counts.program_changes != 1
        || !adapted.warnings.is_empty()
    {
        return Err(ProofError::ComparisonMismatch(format!(
            "unexpected adapter inventory: {:?}, warnings {:?}",
            adapted.counts, adapted.warnings
        )));
    }
    let conductor_track = serialize_conductor_track(
        &conductor.sequence_name,
        conductor.tempo_mpqn,
        conductor.time_signature,
    )?;
    let musical_track = serialize_named_musical_track(b"Track 3", &adapted.scheduled_events)?;
    let generated_bytes = serialize_format1(conductor.ppqn, &[conductor_track, musical_track])?;

    let generated = parse_smf(&generated_bytes).map_err(ProofError::ReferenceParse)?;
    let reference = parse_smf(&reference_bytes).map_err(ProofError::ReferenceParse)?;
    if generated.format != 1 || generated.division != 480 || generated.tracks.len() != 2 {
        return Err(ProofError::ComparisonMismatch(format!(
            "generated SMF header differs: {generated:?}"
        )));
    }
    if reference.division != 480 || reference.tracks.len() <= 6 {
        return Err(ProofError::ComparisonMismatch(format!(
            "reference SMF structure differs: division {}, tracks {}",
            reference.division,
            reference.tracks.len()
        )));
    }
    if track_name(&generated.tracks[0]) != Some(b"Ode to Clarke".as_slice())
        || track_name(&generated.tracks[1]) != Some(b"Track 3".as_slice())
        || track_name(&reference.tracks[6]) != Some(b"Track 3".as_slice())
    {
        return Err(ProofError::ComparisonMismatch("track names differ".into()));
    }

    let generated_notes = normalize_notes(&generated.tracks[1])?;
    let reference_notes = normalize_notes(&reference.tracks[6])?;
    compare_notes(&generated_notes, &reference_notes)?;
    compare_patch(&generated.tracks[1], &reference.tracks[6])?;
    let generated_tempo = initial_tempo(&generated.tracks[0])?;
    let reference_tempo = initial_tempo(&reference.tracks[0])?;
    let generated_meter = initial_meter(&generated.tracks[0])?;
    let reference_meter = initial_meter(&reference.tracks[0])?;
    if generated_tempo != reference_tempo || generated_meter != reference_meter {
        return Err(ProofError::ComparisonMismatch(format!(
            "conductor differs: generated {generated_tempo}/{generated_meter:?}, reference {reference_tempo}/{reference_meter:?}"
        )));
    }
    let explicit_reference_releases = reference_notes
        .iter()
        .filter(|note| note.release_velocity.is_some())
        .count();

    Ok(ProofOutcome {
        bytes: generated_bytes,
        note_count: generated_notes.len(),
        explicit_reference_releases,
        zero_velocity_reference_releases: reference_notes.len() - explicit_reference_releases,
        tempo_mpqn: generated_tempo,
        meter: generated_meter,
    })
}

fn validate_track3_patch(
    transition: &phoenix::mixed_event::BoundedPatchToNoteTransition<'_>,
) -> Result<(), ProofError> {
    let patch = &transition.patch;
    if patch.position.value != 480
        || patch.name.bytes != b"Wavox"
        || patch.post_name_context.bytes != [0x02, 0x33, 0x30, 0x04, 0xff, 0x51, 0x02]
        || patch.program_change.value != 29
        || transition.first_note_position != 9_603
    {
        return Err(ProofError::ManifestMismatch(format!(
            "Track 3 Patch evidence differs: {transition:?}"
        )));
    }
    Ok(())
}

fn validate_hash(artifact: &'static str, bytes: &[u8], expected: &str) -> Result<(), ProofError> {
    let observed = sha256_hex(bytes);
    if observed != expected {
        return Err(ProofError::ArtifactHashMismatch { artifact, observed });
    }
    Ok(())
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn parse_smf(bytes: &[u8]) -> Result<ParsedSmf, String> {
    if bytes.len() < 14 || &bytes[..4] != b"MThd" || be_u32(&bytes[4..8])? != 6 {
        return Err("invalid MThd".into());
    }
    let format = be_u16(&bytes[8..10])?;
    let track_count = usize::from(be_u16(&bytes[10..12])?);
    let division = be_u16(&bytes[12..14])?;
    if division == 0 || division & 0x8000 != 0 {
        return Err("unsupported division".into());
    }
    let mut cursor = 14;
    let mut tracks = Vec::with_capacity(track_count);
    for _ in 0..track_count {
        if bytes.get(cursor..cursor + 4) != Some(b"MTrk") {
            return Err(format!("missing MTrk at {cursor}"));
        }
        let length = usize::try_from(be_u32(slice(bytes, cursor + 4, 4)?)?)
            .map_err(|_| "track length overflow")?;
        let payload_start = cursor + 8;
        let payload_end = payload_start
            .checked_add(length)
            .ok_or("track length overflow")?;
        let payload = bytes
            .get(payload_start..payload_end)
            .ok_or("track exceeds file")?;
        tracks.push(parse_track(payload)?);
        cursor = payload_end;
    }
    if cursor != bytes.len() {
        return Err(format!("SMF has {} trailing bytes", bytes.len() - cursor));
    }
    Ok(ParsedSmf {
        format,
        division,
        tracks,
    })
}

fn parse_track(payload: &[u8]) -> Result<ParsedTrack, String> {
    let mut cursor = 0;
    let mut tick = 0_u32;
    let mut running_status = None;
    let mut events = Vec::new();
    let mut eot_count = 0;
    while cursor < payload.len() {
        let (delta, consumed) = read_vlq(payload, cursor)?;
        cursor += consumed;
        tick = tick.checked_add(delta).ok_or("absolute tick overflow")?;
        let first = *payload.get(cursor).ok_or("missing event status/data")?;
        if first == 0xff {
            running_status = None;
            let kind = *payload.get(cursor + 1).ok_or("missing meta kind")?;
            let (length, length_bytes) = read_vlq(payload, cursor + 2)?;
            let data_start = cursor + 2 + length_bytes;
            let data_end = data_start
                .checked_add(usize::try_from(length).map_err(|_| "meta length overflow")?)
                .ok_or("meta length overflow")?;
            let data = payload
                .get(data_start..data_end)
                .ok_or("meta exceeds track")?
                .to_vec();
            events.push(ParsedEvent::Meta { tick, kind, data });
            cursor = data_end;
            if kind == 0x2f {
                eot_count += 1;
                if !matches!(events.last(), Some(ParsedEvent::Meta { data, .. }) if data.is_empty())
                {
                    return Err("invalid EOT payload".into());
                }
                if cursor != payload.len() {
                    return Err("events after EOT".into());
                }
            }
            continue;
        }
        if first == 0xf0 || first == 0xf7 {
            return Err("SysEx is outside proof parser scope".into());
        }
        let (status, data_start) = if first & 0x80 != 0 {
            if first >= 0xf0 {
                return Err(format!("unsupported status {first:02x}"));
            }
            running_status = Some(first);
            (first, cursor + 1)
        } else {
            (
                running_status.ok_or("data byte without running status")?,
                cursor,
            )
        };
        let data_length = if matches!(status >> 4, 0xc | 0xd) {
            1
        } else {
            2
        };
        let data = payload
            .get(data_start..data_start + data_length)
            .ok_or("channel message exceeds track")?;
        if data.iter().any(|byte| byte & 0x80 != 0) {
            return Err("high-bit channel data byte".into());
        }
        events.push(ParsedEvent::Channel {
            tick,
            status,
            data: data.to_vec(),
        });
        cursor = data_start + data_length;
    }
    if eot_count != 1 {
        return Err(format!("expected one EOT, observed {eot_count}"));
    }
    Ok(ParsedTrack { events })
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

fn slice(bytes: &[u8], start: usize, length: usize) -> Result<&[u8], String> {
    bytes
        .get(start..start + length)
        .ok_or_else(|| "truncated integer".into())
}

fn be_u16(bytes: &[u8]) -> Result<u16, String> {
    let bytes: [u8; 2] = bytes.try_into().map_err(|_| "invalid u16")?;
    Ok(u16::from_be_bytes(bytes))
}

fn be_u32(bytes: &[u8]) -> Result<u32, String> {
    let bytes: [u8; 4] = bytes.try_into().map_err(|_| "invalid u32")?;
    Ok(u32::from_be_bytes(bytes))
}

fn track_name(track: &ParsedTrack) -> Option<&[u8]> {
    track.events.iter().find_map(|event| match event {
        ParsedEvent::Meta {
            kind: 0x03, data, ..
        } => Some(data.as_slice()),
        _ => None,
    })
}

fn normalize_notes(track: &ParsedTrack) -> Result<Vec<NormalizedNote>, ProofError> {
    let mut active: BTreeMap<(u8, u8), VecDeque<(u32, u8)>> = BTreeMap::new();
    let mut notes = Vec::new();
    for event in &track.events {
        let ParsedEvent::Channel { tick, status, data } = event else {
            continue;
        };
        let channel = (status & 0x0f) + 1;
        match (status >> 4, data.as_slice()) {
            (0x9, [pitch, velocity]) if *velocity != 0 => active
                .entry((channel, *pitch))
                .or_default()
                .push_back((*tick, *velocity)),
            (0x8, [pitch, release]) => finish_note(
                &mut active,
                &mut notes,
                channel,
                *pitch,
                *tick,
                Some(*release),
            )?,
            (0x9, [pitch, 0]) => {
                finish_note(&mut active, &mut notes, channel, *pitch, *tick, None)?
            }
            _ => {}
        }
    }
    if active.values().any(|queue| !queue.is_empty()) {
        return Err(ProofError::ComparisonMismatch(
            "unclosed reference/generated Note On".into(),
        ));
    }
    notes.sort();
    Ok(notes)
}

fn finish_note(
    active: &mut BTreeMap<(u8, u8), VecDeque<(u32, u8)>>,
    notes: &mut Vec<NormalizedNote>,
    channel: u8,
    pitch: u8,
    end: u32,
    release_velocity: Option<u8>,
) -> Result<(), ProofError> {
    let (start, attack_velocity) = active
        .get_mut(&(channel, pitch))
        .and_then(VecDeque::pop_front)
        .ok_or_else(|| {
            ProofError::ComparisonMismatch(format!(
                "Note end without start: channel {channel}, pitch {pitch}, tick {end}"
            ))
        })?;
    notes.push(NormalizedNote {
        channel,
        pitch,
        start,
        end,
        attack_velocity,
        release_velocity,
    });
    Ok(())
}

fn compare_notes(
    generated: &[NormalizedNote],
    reference: &[NormalizedNote],
) -> Result<(), ProofError> {
    if generated.len() != reference.len() {
        return Err(ProofError::ComparisonMismatch(format!(
            "Note count: Phoenix {}, reference {}",
            generated.len(),
            reference.len()
        )));
    }
    for (index, (generated, reference)) in generated.iter().zip(reference).enumerate() {
        if generated.channel != reference.channel
            || generated.pitch != reference.pitch
            || generated.start != reference.start
            || generated.end != reference.end
            || generated.attack_velocity != reference.attack_velocity
            || (reference.release_velocity.is_some()
                && generated.release_velocity != reference.release_velocity)
        {
            return Err(ProofError::ComparisonMismatch(format!(
                "first Note mismatch at normalized index {index}: Phoenix {generated:?}, reference {reference:?}"
            )));
        }
    }
    Ok(())
}

fn compare_patch(generated: &ParsedTrack, reference: &ParsedTrack) -> Result<(), ProofError> {
    let inventory = |track: &ParsedTrack| {
        track
            .events
            .iter()
            .filter_map(|event| match event {
                ParsedEvent::Channel { tick, status, data } if matches!(status >> 4, 0xb | 0xc) => {
                    Some((*tick, *status, data.clone()))
                }
                _ => None,
            })
            .collect::<Vec<_>>()
    };
    let expected = vec![
        (480, 0xb0, vec![0, 81]),
        (480, 0xb0, vec![32, 2]),
        (480, 0xc0, vec![29]),
    ];
    if inventory(generated) != expected || inventory(reference) != expected {
        return Err(ProofError::ComparisonMismatch(format!(
            "Patch inventory differs: Phoenix {:?}, reference {:?}",
            inventory(generated),
            inventory(reference)
        )));
    }
    Ok(())
}

fn initial_tempo(track: &ParsedTrack) -> Result<u32, ProofError> {
    track
        .events
        .iter()
        .find_map(|event| match event {
            ParsedEvent::Meta {
                tick: 0,
                kind: 0x51,
                data,
            } if data.len() == 3 => {
                Some((u32::from(data[0]) << 16) | (u32::from(data[1]) << 8) | u32::from(data[2]))
            }
            _ => None,
        })
        .ok_or_else(|| ProofError::ComparisonMismatch("initial Tempo absent".into()))
}

fn initial_meter(track: &ParsedTrack) -> Result<[u8; 4], ProofError> {
    track
        .events
        .iter()
        .find_map(|event| match event {
            ParsedEvent::Meta {
                tick: 0,
                kind: 0x58,
                data,
            } if data.len() == 4 => Some([data[0], data[1], data[2], data[3]]),
            _ => None,
        })
        .ok_or_else(|| ProofError::ComparisonMismatch("initial Meter absent".into()))
}
