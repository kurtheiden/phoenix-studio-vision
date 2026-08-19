use phoenix::{
    meter::{decode_bounded_initial_meter, InitialMeterBounds},
    midi_export::{
        ChannelAssignment, ChannelAssignmentProvenance, DecodedExportEvent, ExportCounts,
        MeterPolicy, PatchPolicy, PatchTranslation, TimingPolicy,
    },
    mixed_event::{
        walk_bounded_mixed_events, MixedEventBounds, MixedEventItem, MixedEventKind,
        MixedEventTimingBasis,
    },
    multitrack_export::{
        assemble_multitrack_sequence, MultitrackExportResult, MultitrackSequenceInput,
        MusicalTrackInput,
    },
    sequence_container::{parse_project_166, TrackAssociations},
    smf::MidiChannel,
    tempo::{decode_bounded_initial_tempo, InitialTempoBounds},
};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs, ops::Range, path::Path};

const SOURCE_PATH: &str = "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline";

#[derive(Clone, Debug)]
struct SequenceManifest {
    project_size: usize,
    project_sha256: &'static str,
    sequence_name: &'static [u8],
    sequence_name_range: Range<usize>,
    sequence_range: Range<usize>,
    tempo_mpqn: u32,
    meter: (u8, u8, u8, u8),
    rows: Vec<TrackRow>,
}

#[derive(Clone, Debug)]
struct TrackRow {
    descriptor_ordinal: usize,
    descriptor_range: Range<usize>,
    name: &'static [u8],
    pair_ordinal: usize,
    primary_range: Range<usize>,
    event_range: Range<usize>,
    channel: u8,
    logical_events: usize,
    notes: usize,
    patch: Option<PatchExpectation>,
}

#[derive(Clone, Debug)]
struct PatchExpectation {
    tick: u32,
    name: &'static str,
    context: &'static [u8],
    program: u8,
    translation: PatchTranslation,
}

#[allow(clippy::too_many_arguments)]
fn row(
    descriptor_ordinal: usize,
    descriptor_range: Range<usize>,
    name: &'static [u8],
    pair_ordinal: usize,
    primary_range: Range<usize>,
    event_range: Range<usize>,
    channel: u8,
    logical_events: usize,
    notes: usize,
    patch: Option<PatchExpectation>,
) -> TrackRow {
    TrackRow {
        descriptor_ordinal,
        descriptor_range,
        name,
        pair_ordinal,
        primary_range,
        event_range,
        channel,
        logical_events,
        notes,
        patch,
    }
}

fn ode_manifest() -> SequenceManifest {
    SequenceManifest {
        project_size: 211_468,
        project_sha256: "e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132",
        sequence_name: b"Ode to Clarke",
        sequence_name_range: 0x02f753..0x02f760,
        sequence_range: 0x02ef6f..0x03202c,
        tempo_mpqn: 500_000,
        meter: (4, 2, 8, 8),
        rows: vec![
            row(
                2,
                0x02f18b..0x02f231,
                b"Track 1",
                0,
                0x02f820..0x02fa7a,
                0x02f833..0x02fa73,
                1,
                92,
                91,
                Some(PatchExpectation {
                    tick: 0,
                    name: "Empty Patch",
                    context: &[2, 0x33, 0x30, 4, 0xff, 0xff, 0xff],
                    program: 61,
                    translation: PatchTranslation::ProgramOnlyConfirmed,
                }),
            ),
            row(
                3,
                0x02f231..0x02f2d7,
                b"Track 2",
                1,
                0x02fb42..0x0300df,
                0x02fb55..0x0300d8,
                2,
                212,
                211,
                Some(PatchExpectation {
                    tick: 0,
                    name: "Stereoww Bs",
                    context: &[2, 0x33, 0x38, 4, 0xff, 0x51, 1],
                    program: 37,
                    translation: PatchTranslation::ConfirmedBankSelect { msb: 81, lsb: 1 },
                }),
            ),
            row(
                4,
                0x02f2d7..0x02f37d,
                b"sys100loops",
                2,
                0x0301b7..0x03097d,
                0x0301ca..0x030976,
                10,
                322,
                322,
                None,
            ),
            row(
                5,
                0x02f37d..0x02f423,
                b"Track 4",
                3,
                0x030a17..0x030e9f,
                0x030a2a..0x030e98,
                10,
                179,
                179,
                None,
            ),
            row(
                6,
                0x02f423..0x02f4c9,
                b"Track 5",
                4,
                0x030f31..0x03125b,
                0x030f44..0x031254,
                10,
                134,
                134,
                None,
            ),
            row(
                7,
                0x02f4c9..0x02f56f,
                b"Track 3",
                5,
                0x0312ed..0x03156b,
                0x031300..0x031564,
                1,
                85,
                84,
                Some(PatchExpectation {
                    tick: 480,
                    name: "Wavox",
                    context: &[2, 0x33, 0x30, 4, 0xff, 0x51, 2],
                    program: 29,
                    translation: PatchTranslation::ConfirmedBankSelect { msb: 81, lsb: 2 },
                }),
            ),
            row(
                8,
                0x02f56f..0x02f615,
                b"Track 6",
                6,
                0x03165b..0x031805,
                0x03166e..0x0317fe,
                10,
                60,
                60,
                None,
            ),
            row(
                9,
                0x02f615..0x02f6bb,
                b"Track 3 #2",
                7,
                0x031873..0x031b05,
                0x031886..0x031afe,
                15,
                85,
                84,
                Some(PatchExpectation {
                    tick: 530,
                    name: "Ming Dynasty",
                    context: &[3, 0x49, 0x33, 0x38, 4, 0xff, 0xff, 0xff],
                    program: 23,
                    translation: PatchTranslation::ProgramOnlyConfirmed,
                }),
            ),
            row(
                10,
                0x02f6bb..0x02f761,
                b"Track 7",
                8,
                0x031bf5..0x031fa3,
                0x031c08..0x031f9c,
                10,
                143,
                143,
                None,
            ),
        ],
    }
}

#[derive(Debug)]
enum D2Error {
    Project(String),
    Sequence(String),
    Manifest(String),
    Decode(String),
    Walk(usize, String),
    Patch(usize, String),
    Assembly(String),
    Smf(String),
}

impl std::fmt::Display for D2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Project(x) => write!(f, "project provenance: {x}"),
            Self::Sequence(x) => write!(f, "sequence identity: {x}"),
            Self::Manifest(x) => write!(f, "manifest: {x}"),
            Self::Decode(x) => write!(f, "initial metadata: {x}"),
            Self::Walk(row, x) => write!(f, "row {row} walk: {x}"),
            Self::Patch(row, x) => write!(f, "row {row} Patch: {x}"),
            Self::Assembly(x) => write!(f, "D1 assembly: {x}"),
            Self::Smf(x) => write!(f, "SMF: {x}"),
        }
    }
}

impl std::error::Error for D2Error {}

struct D2Outcome {
    result: MultitrackExportResult,
    logical_events: usize,
    notes: usize,
    patches: usize,
}

fn validate_and_assemble(bytes: &[u8], manifest: &SequenceManifest) -> Result<D2Outcome, D2Error> {
    if bytes.len() != manifest.project_size {
        return Err(D2Error::Project(format!(
            "size {} != {}",
            bytes.len(),
            manifest.project_size
        )));
    }
    let observed_hash = sha256_hex(bytes);
    if observed_hash != manifest.project_sha256 {
        return Err(D2Error::Project(format!("SHA-256 {observed_hash} differs")));
    }
    let project =
        parse_project_166(bytes).map_err(|error| D2Error::Project(format!("{error:?}")))?;
    let matches: Vec<_> = project
        .sequences
        .iter()
        .filter(|sequence| sequence.sequence_name.bytes.bytes == manifest.sequence_name)
        .collect();
    if matches.len() != 1 {
        return Err(D2Error::Sequence(format!(
            "expected one parsed name match, found {}",
            matches.len()
        )));
    }
    let sequence = matches[0];
    if sequence.sequence_range != manifest.sequence_range
        || sequence.sequence_name.bytes.range != manifest.sequence_name_range
    {
        return Err(D2Error::Sequence("locked ranges differ".into()));
    }
    let bindings = match &sequence.track_associations {
        TrackAssociations::Ordinal(bindings) => bindings,
        TrackAssociations::Unresolved { .. } => {
            return Err(D2Error::Manifest("association unresolved".into()));
        }
    };
    if sequence.track_descriptors().len() != manifest.rows.len()
        || sequence.track_pairs.len() != manifest.rows.len()
        || bindings.len() != manifest.rows.len()
    {
        return Err(D2Error::Manifest(format!(
            "coverage: {} descriptors, {} pairs, {} bindings, {} rows",
            sequence.track_descriptors().len(),
            sequence.track_pairs.len(),
            bindings.len(),
            manifest.rows.len()
        )));
    }
    let descriptors: BTreeSet<_> = manifest.rows.iter().map(|x| x.descriptor_ordinal).collect();
    let pairs: BTreeSet<_> = manifest.rows.iter().map(|x| x.pair_ordinal).collect();
    if descriptors.len() != manifest.rows.len()
        || pairs.len() != manifest.rows.len()
        || !manifest
            .rows
            .windows(2)
            .all(|x| x[0].descriptor_ordinal < x[1].descriptor_ordinal)
    {
        return Err(D2Error::Manifest("duplicate/out-of-order rows".into()));
    }

    let tempo = decode_bounded_initial_tempo(
        bytes,
        InitialTempoBounds {
            event_range: sequence.initial_tempo_range.clone(),
        },
    )
    .map_err(|error| D2Error::Decode(format!("{error:?}")))?;
    let meter = decode_bounded_initial_meter(
        bytes,
        InitialMeterBounds {
            event_range: sequence.initial_meter_range.clone(),
        },
    )
    .map_err(|error| D2Error::Decode(format!("{error:?}")))?;
    let meter_values = (
        meter.numerator.value,
        meter.denominator_exponent.value,
        meter.third_payload.value,
        meter.fourth_payload.value,
    );
    if tempo.mpqn() != manifest.tempo_mpqn || meter_values != manifest.meter {
        return Err(D2Error::Decode("authenticated values differ".into()));
    }

    let mut flattened = Vec::with_capacity(manifest.rows.len());
    let mut contexts = Vec::with_capacity(manifest.rows.len());
    let mut channels = Vec::with_capacity(manifest.rows.len());
    let mut logical_total = 0;
    let mut note_total = 0;
    let mut patch_total = 0;

    for (row_index, row) in manifest.rows.iter().enumerate() {
        let descriptor = sequence
            .descriptors
            .get(row.descriptor_ordinal)
            .ok_or_else(|| D2Error::Manifest(format!("row {row_index} descriptor absent")))?;
        if descriptor.ordinal != row.descriptor_ordinal
            || descriptor.range != row.descriptor_range
            || descriptor.label.as_ref().map(|x| x.bytes) != Some(row.name)
        {
            return Err(D2Error::Manifest(format!(
                "row {row_index} descriptor differs"
            )));
        }
        if !bindings.iter().any(|binding| {
            binding.descriptor_ordinal == row.descriptor_ordinal
                && binding.pair_ordinal == row.pair_ordinal
        }) {
            return Err(D2Error::Manifest(format!("row {row_index} binding absent")));
        }
        let pair = sequence
            .track_pairs
            .get(row.pair_ordinal)
            .ok_or_else(|| D2Error::Manifest(format!("row {row_index} pair absent")))?;
        if pair.pair_ordinal != row.pair_ordinal || pair.primary.record_range != row.primary_range {
            return Err(D2Error::Manifest(format!("row {row_index} pair differs")));
        }
        let event_end = pair
            .primary
            .payload
            .range
            .end
            .checked_sub(7)
            .ok_or_else(|| D2Error::Manifest(format!("row {row_index} tail underflow")))?;
        let event_range = pair.candidate_event_start..event_end;
        let tail = bytes
            .get(event_end..pair.primary.payload.range.end)
            .ok_or_else(|| D2Error::Manifest(format!("row {row_index} tail exceeds input")))?;
        if tail.len() != 7
            || tail[0] != 0xff
            || tail[4..] != [0xff, 0x2f, 0]
            || event_range != row.event_range
        {
            return Err(D2Error::Manifest(format!(
                "row {row_index} event range/tail differs"
            )));
        }
        let channel = MidiChannel::new(row.channel)
            .map_err(|error| D2Error::Manifest(format!("row {row_index}: {error:?}")))?;
        let walk = walk_bounded_mixed_events(
            bytes,
            MixedEventBounds {
                event_range: event_range.clone(),
            },
            MixedEventTimingBasis {
                previous_event_position: 0,
            },
        )
        .map_err(|error| D2Error::Walk(row_index, format!("{error:?}")))?;
        if walk.consumed_range != event_range || walk.logical_event_count() != row.logical_events {
            return Err(D2Error::Walk(row_index, "range/count differs".into()));
        }

        let mut events = Vec::with_capacity(walk.logical_event_count());
        let mut ordinal = 0_u64;
        let mut notes = 0;
        let mut patches = 0;
        for item in &walk.items {
            match item {
                MixedEventItem::PatchToNote(transition) => {
                    let expected = row
                        .patch
                        .as_ref()
                        .ok_or_else(|| D2Error::Patch(row_index, "unexpected Patch".into()))?;
                    validate_patch(row_index, transition, expected)?;
                    events.push(DecodedExportEvent::from_patch(
                        ordinal,
                        &transition.patch,
                        expected.translation,
                    ));
                    ordinal += 1;
                    events.push(DecodedExportEvent::from_note_body(
                        transition.first_note_position,
                        ordinal,
                        &transition.first_note,
                    ));
                    ordinal += 1;
                    notes += 1;
                    patches += 1;
                }
                MixedEventItem::Event(positioned) => match &positioned.event {
                    MixedEventKind::Note(note) => {
                        events.push(DecodedExportEvent::from_note(
                            positioned.position,
                            ordinal,
                            note,
                        ));
                        ordinal += 1;
                        notes += 1;
                    }
                    other => {
                        return Err(D2Error::Walk(
                            row_index,
                            format!("unsupported family {other:?}"),
                        ));
                    }
                },
            }
        }
        if events.len() != row.logical_events
            || notes != row.notes
            || patches != usize::from(row.patch.is_some())
        {
            return Err(D2Error::Walk(
                row_index,
                format!(
                    "flattened {}, Notes {notes}, Patches {patches}",
                    events.len()
                ),
            ));
        }
        logical_total += events.len();
        note_total += notes;
        patch_total += patches;
        flattened.push(events);
        contexts.push(format!(
            "descriptor {} / pair {}",
            row.descriptor_ordinal, row.pair_ordinal
        ));
        channels.push(channel);
    }

    let tracks: Vec<_> = manifest
        .rows
        .iter()
        .enumerate()
        .map(|(index, row)| MusicalTrackInput {
            context: &contexts[index],
            name: row.name,
            channel_assignment: ChannelAssignment {
                channel: channels[index],
                provenance: ChannelAssignmentProvenance::AuthenticatedOverride,
            },
            events: &flattened[index],
            patch_policy: PatchPolicy::StrictKnownOnly,
        })
        .collect();
    let input = MultitrackSequenceInput {
        sequence_name: sequence.sequence_name.bytes.bytes,
        tempo_mpqn: tempo.mpqn(),
        meter_values,
        timing_policy: TimingPolicy::Identity480,
        meter_policy: MeterPolicy::HistoricalWhenKnownOtherwiseStandard,
        tracks: &tracks,
    };
    let result = assemble_multitrack_sequence(&input)
        .map_err(|error| D2Error::Assembly(format!("{error:?}")))?;
    validate_report(&result, manifest)?;
    validate_smf(&result.smf_bytes, manifest)?;
    Ok(D2Outcome {
        result,
        logical_events: logical_total,
        notes: note_total,
        patches: patch_total,
    })
}

fn validate_patch(
    row: usize,
    transition: &phoenix::mixed_event::BoundedPatchToNoteTransition<'_>,
    expected: &PatchExpectation,
) -> Result<(), D2Error> {
    let patch = &transition.patch;
    if transition.patch_position != expected.tick
        || patch.position.value != expected.tick
        || patch.name.text != expected.name
        || patch.post_name_context.bytes != expected.context
        || patch.program_change.value != expected.program
    {
        return Err(D2Error::Patch(row, "decoded evidence differs".into()));
    }
    Ok(())
}

fn validate_report(
    result: &MultitrackExportResult,
    manifest: &SequenceManifest,
) -> Result<(), D2Error> {
    let expected = ExportCounts {
        notes: 1_308,
        generated_note_offs: 1_308,
        controllers: 0,
        bank_select_msb: 2,
        bank_select_lsb: 2,
        program_changes: 4,
        channel_pressure: 0,
        pitch_bend: 0,
        tempo: 1,
        meter: 1,
    };
    if result.report.sequence_name != manifest.sequence_name
        || result.report.musical_track_count != 9
        || result.report.total_smf_track_count != 10
        || result.report.tracks.len() != 9
        || result.report.totals != expected
        || !result.report.warnings.is_empty()
    {
        return Err(D2Error::Assembly("aggregate report differs".into()));
    }
    for (report, row) in result.report.tracks.iter().zip(&manifest.rows) {
        if report.name != row.name
            || report.channel_assignment.channel.get() != row.channel
            || report.channel_assignment.provenance
                != ChannelAssignmentProvenance::AuthenticatedOverride
            || report.counts.notes != u64::try_from(row.notes).unwrap()
        {
            return Err(D2Error::Assembly("per-track report differs".into()));
        }
    }
    Ok(())
}

fn validate_smf(bytes: &[u8], manifest: &SequenceManifest) -> Result<(), D2Error> {
    if bytes.len() < 14
        || &bytes[..4] != b"MThd"
        || u32::from_be_bytes(bytes[4..8].try_into().unwrap()) != 6
        || u16::from_be_bytes(bytes[8..10].try_into().unwrap()) != 1
        || u16::from_be_bytes(bytes[10..12].try_into().unwrap()) != 10
        || u16::from_be_bytes(bytes[12..14].try_into().unwrap()) != 480
    {
        return Err(D2Error::Smf("header differs".into()));
    }
    let mut cursor = 14;
    let mut identities = Vec::new();
    for index in 0..10 {
        if bytes.get(cursor..cursor + 4) != Some(b"MTrk") {
            return Err(D2Error::Smf(format!("missing MTrk {index}")));
        }
        let length = usize::try_from(u32::from_be_bytes(
            bytes[cursor + 4..cursor + 8].try_into().unwrap(),
        ))
        .map_err(|_| D2Error::Smf("track length overflow".into()))?;
        let start = cursor + 8;
        let end = start + length;
        identities.push(inspect_track(
            bytes
                .get(start..end)
                .ok_or_else(|| D2Error::Smf("track exceeds file".into()))?,
        )?);
        cursor = end;
    }
    if cursor != bytes.len()
        || identities[0].0.as_deref() != Some(manifest.sequence_name)
        || !identities[0].1.is_empty()
    {
        return Err(D2Error::Smf("EOF/conductor differs".into()));
    }
    for (index, row) in manifest.rows.iter().enumerate() {
        if identities[index + 1].0.as_deref() != Some(row.name)
            || identities[index + 1].1 != BTreeSet::from([row.channel])
        {
            return Err(D2Error::Smf(format!("track {index} differs")));
        }
    }
    Ok(())
}

fn inspect_track(bytes: &[u8]) -> Result<(Option<Vec<u8>>, BTreeSet<u8>), D2Error> {
    let mut cursor = 0;
    let mut running = None;
    let mut name = None;
    let mut channels = BTreeSet::new();
    let mut eot = 0;
    while cursor < bytes.len() {
        let (_, width) = read_vlq(bytes, cursor)?;
        cursor += width;
        let first = *bytes
            .get(cursor)
            .ok_or_else(|| D2Error::Smf("missing event".into()))?;
        if first == 0xff {
            running = None;
            let kind = *bytes
                .get(cursor + 1)
                .ok_or_else(|| D2Error::Smf("missing meta kind".into()))?;
            let (length, width) = read_vlq(bytes, cursor + 2)?;
            let start = cursor + 2 + width;
            let end = start + usize::try_from(length).unwrap();
            let data = bytes
                .get(start..end)
                .ok_or_else(|| D2Error::Smf("meta exceeds track".into()))?;
            if kind == 3 {
                name = Some(data.to_vec());
            }
            cursor = end;
            if kind == 0x2f {
                eot += 1;
                if !data.is_empty() || cursor != bytes.len() {
                    return Err(D2Error::Smf("invalid EOT".into()));
                }
            }
            continue;
        }
        let (status, start) = if first & 0x80 != 0 {
            running = Some(first);
            (first, cursor + 1)
        } else {
            (
                running.ok_or_else(|| D2Error::Smf("missing running status".into()))?,
                cursor,
            )
        };
        if status >= 0xf0 {
            return Err(D2Error::Smf("system status".into()));
        }
        channels.insert((status & 0x0f) + 1);
        let length = if matches!(status >> 4, 0xc | 0xd) {
            1
        } else {
            2
        };
        let data = bytes
            .get(start..start + length)
            .ok_or_else(|| D2Error::Smf("channel event exceeds track".into()))?;
        if data.iter().any(|byte| byte & 0x80 != 0) {
            return Err(D2Error::Smf("invalid data byte".into()));
        }
        cursor = start + length;
    }
    if eot != 1 {
        return Err(D2Error::Smf(format!("EOT count {eot}")));
    }
    Ok((name, channels))
}

fn read_vlq(bytes: &[u8], offset: usize) -> Result<(u32, usize), D2Error> {
    let mut value = 0;
    for index in 0..4 {
        let byte = *bytes
            .get(offset + index)
            .ok_or_else(|| D2Error::Smf("truncated VLQ".into()))?;
        value = (value << 7) | u32::from(byte & 0x7f);
        if byte & 0x80 == 0 {
            return Ok((value, index + 1));
        }
    }
    Err(D2Error::Smf("overlong VLQ".into()))
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn fixture() -> Option<Vec<u8>> {
    Path::new(SOURCE_PATH)
        .exists()
        .then(|| fs::read(SOURCE_PATH).unwrap())
}

#[test]
fn authenticated_ode_nine_tracks_feed_d1_in_memory() {
    let Some(bytes) = fixture() else {
        eprintln!("skipping D2: authentic external source fixture is absent");
        return;
    };
    let outcome = validate_and_assemble(&bytes, &ode_manifest()).unwrap();
    assert_eq!(outcome.logical_events, 1_312);
    assert_eq!((outcome.notes, outcome.patches), (1_308, 4));
    assert_eq!(outcome.result.report.total_smf_track_count, 10);
}

#[test]
fn manifest_mutations_fail_without_a_multitrack_result() {
    let Some(bytes) = fixture() else {
        eprintln!("skipping D2 mutations: authentic fixture is absent");
        return;
    };
    let mut mutations = Vec::new();
    let mut value = ode_manifest();
    value.project_sha256 = "05a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132";
    mutations.push(value);
    let mut value = ode_manifest();
    value.sequence_range.start += 1;
    mutations.push(value);
    let mut value = ode_manifest();
    value.rows[0].descriptor_range.start += 1;
    mutations.push(value);
    let mut value = ode_manifest();
    value.rows[0].pair_ordinal = 1;
    mutations.push(value);
    let mut value = ode_manifest();
    value.rows[0].event_range.end -= 1;
    mutations.push(value);
    let mut value = ode_manifest();
    value.rows.pop();
    mutations.push(value);
    let mut value = ode_manifest();
    value.rows.push(value.rows[0].clone());
    mutations.push(value);
    let mut value = ode_manifest();
    value.rows[0].patch.as_mut().unwrap().program = 62;
    mutations.push(value);

    for mutated in mutations {
        assert!(validate_and_assemble(&bytes, &mutated).is_err());
    }
}
