use phoenix::{
    smf::{self, TimeSignature},
    tempo::{decode_bounded_tempo_map, TempoMapBounds, TempoMapError},
};
use std::ops::Range;

fn decode(b: &[u8], count: u32) -> Result<phoenix::tempo::BoundedTempoMap<'_>, TempoMapError> {
    decode_bounded_tempo_map(
        b,
        TempoMapBounds {
            event_range: 0..b.len(),
            declared_count: count,
        },
    )
}
fn signature() -> TimeSignature {
    TimeSignature::new(
        smf::MidiDataByte::new(4).unwrap(),
        smf::MidiDataByte::new(2).unwrap(),
        smf::MidiDataByte::new(24).unwrap(),
        smf::MidiDataByte::new(8).unwrap(),
    )
    .unwrap()
}
fn body(events: &[(u32, u32)]) -> Vec<u8> {
    events
        .iter()
        .flat_map(|(delta, mpqn)| {
            let mut b = smf::encode_midi_vlq(*delta).unwrap();
            b.extend(smf::serialize_set_tempo(*mpqn).unwrap());
            b
        })
        .collect()
}
// Independent MIDI meta-track parser: no production Tempo decoder or readback.
fn vlq(b: &[u8], p: &mut usize) -> u32 {
    let mut v = 0;
    for _ in 0..4 {
        let x = b[*p];
        *p += 1;
        v = v * 128 + u32::from(x & 127);
        if x < 128 {
            return v;
        }
    }
    panic!("invalid test/reference MIDI VLQ")
}
fn meta(chunk: &[u8]) -> Vec<(u32, u8, Vec<u8>)> {
    assert_eq!(&chunk[..4], b"MTrk");
    assert_eq!(
        u32::from_be_bytes(chunk[4..8].try_into().unwrap()) as usize,
        chunk.len() - 8
    );
    let mut p = 8;
    let mut tick = 0u32;
    let mut events = Vec::new();
    while p < chunk.len() {
        tick = tick.checked_add(vlq(chunk, &mut p)).unwrap();
        assert_eq!(chunk[p], 255);
        let tag = chunk[p + 1];
        p += 2;
        let len = vlq(chunk, &mut p) as usize;
        let data = chunk[p..p + len].to_vec();
        p += len;
        events.push((tick, tag, data));
    }
    assert_eq!(p, chunk.len());
    assert_eq!(events.last().unwrap().1, 47);
    events
}
fn tempos(chunk: &[u8]) -> Vec<(u32, u32)> {
    meta(chunk)
        .into_iter()
        .filter(|e| e.1 == 81)
        .map(|(t, _, v)| {
            assert_eq!(v.len(), 3);
            (t, u32::from_be_bytes([0, v[0], v[1], v[2]]))
        })
        .collect()
}
#[test]
fn independent_wire_fixture_positions_values_order_and_zero_delta() {
    // Explicit bytes, not encoder-generated expected timings/MPQN.
    let b = [
        0, 255, 81, 3, 7, 161, 32, 0, 255, 81, 3, 9, 137, 104, 0x81, 0x70, 255, 81, 3, 14, 78, 28,
    ];
    let map = decode(&b, 3).unwrap();
    assert_eq!(map.event_range(), 0..b.len());
    assert_eq!(map.ppqn(), 480);
    assert_eq!(
        map.events()
            .iter()
            .map(|e| (e.absolute_tick, e.mpqn()))
            .collect::<Vec<_>>(),
        [(0, 500000), (0, 625000), (240, 937500)]
    );
    assert_eq!(map.events()[2].source_range, 14..22);
    assert_eq!(map.events()[2].delta.range, 14..16);
    assert_eq!(map.events()[2].delta.bytes, [0x81, 0x70]);
    assert_eq!(map.events()[2].mpqn_bytes[2].offset, 21);
    let chunk = smf::serialize_tempo_map_conductor(b"S", &map, signature(), None).unwrap();
    assert_eq!(
        tempos(chunk.as_bytes()),
        [(0, 500000), (0, 625000), (240, 937500)]
    );
    let midi = smf::serialize_format1(map.ppqn(), &[chunk]).unwrap();
    assert_eq!(
        &midi[..14],
        &[77, 84, 104, 100, 0, 0, 0, 6, 0, 1, 0, 1, 1, 224]
    );
}
#[test]
fn nonzero_first_delta_is_not_shifted_or_given_synthetic_initial_tempo() {
    let b = [
        0x8b, 0x20, 255, 81, 3, 7, 161, 32, 0x81, 0x70, 255, 81, 3, 9, 137, 104,
    ];
    let map = decode(&b, 2).unwrap();
    let chunk =
        smf::serialize_tempo_map_conductor(b"S", &map, signature(), Some((1, false))).unwrap();
    assert_eq!(tempos(chunk.as_bytes()), [(1440, 500000), (1680, 625000)]);
    let events = meta(chunk.as_bytes());
    assert_eq!(events[1], (0, 89, vec![1, 0]));
    assert_eq!(events[2], (0, 88, vec![4, 2, 24, 8]));
    assert_eq!(events.last().unwrap().0, 1680);
}
#[test]
fn initial_single_tempo_and_meter_key_bytes_unchanged() {
    let b = [0, 255, 81, 3, 7, 161, 32];
    let map = decode(&b, 1).unwrap();
    let initial = phoenix::tempo::decode_bounded_initial_tempo(
        &b,
        phoenix::tempo::InitialTempoBounds { event_range: 0..7 },
    )
    .unwrap();
    assert_eq!(map.events()[0].mpqn(), initial.mpqn());
    for key in [None, Some((-3, true))] {
        let new = smf::serialize_tempo_map_conductor(b"S", &map, signature(), key).unwrap();
        let old = smf::serialize_conductor_track_with_key(b"S", 500000, signature(), key).unwrap();
        assert_eq!(new.as_bytes(), old.as_bytes());
    }
    assert!(smf::serialize_tempo_map_conductor(b"S", &map, signature(), Some((8, false))).is_err());
}
#[test]
fn duplicate_events_preserved_without_sorting_or_deduplication() {
    let b = body(&[(240, 500000), (0, 500000), (0, 625000)]);
    let map = decode(&b, 3).unwrap();
    assert_eq!(
        tempos(
            smf::serialize_tempo_map_conductor(b"S", &map, signature(), None)
                .unwrap()
                .as_bytes()
        ),
        [(240, 500000), (240, 500000), (240, 625000)]
    );
}
#[test]
fn exact_nonzero_bounds_and_count_no_scanning_or_trailing_byte_loss() {
    let event = body(&[(0, 500000)]);
    let b = [vec![42, 43], event.clone(), vec![44]].concat();
    let map = decode_bounded_tempo_map(
        &b,
        TempoMapBounds {
            event_range: 2..9,
            declared_count: 1,
        },
    )
    .unwrap();
    assert_eq!(map.events()[0].source_range, 2..9);
    assert_eq!(map.events()[0].mpqn_bytes[0].offset, 6);
    for range in [0..0, Range { start: 9, end: 2 }, 0..11] {
        assert_eq!(
            decode_bounded_tempo_map(
                &b,
                TempoMapBounds {
                    event_range: range,
                    declared_count: 1
                }
            ),
            Err(TempoMapError::Bounds)
        );
    }
    for count in [0, 2, u32::MAX] {
        assert!(decode(&event, count).is_err());
    }
    assert!(matches!(
        decode(&[event.clone(), vec![0]].concat(), 1),
        Err(TempoMapError::Unconsumed { .. })
    ));
    assert!(matches!(
        decode(&[event.clone(), event].concat(), 1),
        Err(TempoMapError::Unconsumed { .. })
    ));
    assert!(decode(&b, 1).is_err());
}
#[test]
fn malformed_vlq_payload_tag_and_every_truncation_refuse() {
    let b = body(&[(1440, 500000), (240, 625000)]);
    for end in 0..b.len() {
        assert!(decode(&b[..end], 2).is_err(), "end{end}");
    }
    assert!(matches!(
        decode(&[0x80, 0x80, 0x80, 0x80, 0, 255, 81, 3, 7, 161, 32], 1),
        Err(TempoMapError::Timing(_))
    ));
    assert!(decode(&[0x80; 7], 1).is_err());
    assert!(matches!(
        decode(&[0x81, 0x80, 0x80, 0, 255, 81, 3, 7, 161], 1),
        Err(TempoMapError::Truncated { .. })
    ));
    for (offset, value) in [(1, 254), (2, 88), (3, 4)] {
        let mut bad = vec![0, 255, 81, 3, 7, 161, 32];
        bad[offset] = value;
        assert!(matches!(
            decode(&bad, 1),
            Err(TempoMapError::UnsupportedEvent { .. })
        ));
    }
    assert!(matches!(
        decode(&[0, 255, 81, 3, 0, 0, 0], 1),
        Err(TempoMapError::InvalidTempo { .. })
    ));
}
#[test]
fn checked_cumulative_position_overflow_and_maximum_smf_delta() {
    let b = body(&[(0x0fff_ffff, 1); 16]);
    let map = decode(&b, 16).unwrap();
    assert_eq!(map.events().last().unwrap().absolute_tick, 0xffff_fff0);
    assert_eq!(
        tempos(
            smf::serialize_tempo_map_conductor(b"S", &map, signature(), None)
                .unwrap()
                .as_bytes()
        )
        .len(),
        16
    );
    let b = body(&[(0x0fff_ffff, 1); 17]);
    assert!(matches!(
        decode(&b, 17),
        Err(TempoMapError::PositionOverflow { .. })
    ));
    assert!(smf::encode_midi_vlq(0x1000_0000).is_err());
}
fn authentic(var: &str, size: usize, hash: &str) -> Vec<u8> {
    use sha2::{Digest, Sha256};
    let b = std::fs::read(std::env::var(var).expect("explicit authentic fixture env required"))
        .unwrap();
    assert_eq!(b.len(), size);
    assert_eq!(format!("{:x}", Sha256::digest(&b)), hash);
    b
}
fn map_at(
    b: &[u8],
    primary: Range<usize>,
    range: Range<usize>,
    count: u32,
) -> phoenix::tempo::BoundedTempoMap<'_> {
    let root = phoenix::sequence_container::parse_root_record_stream(b).unwrap();
    let r = root
        .records
        .iter()
        .find(|r| r.record_range == primary)
        .unwrap();
    assert_eq!(r.record_type.value, 2);
    assert_eq!(
        u32::from_be_bytes(r.payload.bytes[2..6].try_into().unwrap()),
        count
    );
    assert_eq!(r.payload.range.start + 14, range.start);
    assert_eq!(r.payload.range.end - 7, range.end);
    decode_bounded_tempo_map(
        b,
        TempoMapBounds {
            event_range: range,
            declared_count: count,
        },
    )
    .unwrap()
}
fn assert_source_accounting(b: &[u8], map: &phoenix::tempo::BoundedTempoMap<'_>) {
    let mut cursor = map.event_range().start;
    for e in map.events() {
        assert_eq!(e.source_range.start, cursor);
        assert_eq!(&b[e.delta.range.clone()], e.delta.bytes);
        assert_eq!(&b[e.delta.range.end..e.delta.range.end + 3], &[255, 81, 3]);
        assert_eq!(e.source_range.end, e.mpqn_bytes[2].offset + 1);
        cursor = e.source_range.end;
    }
    assert_eq!(cursor, map.event_range().end);
}
#[test]
#[ignore = "requires explicitly supplied authentic original COMIC BOOK"]
fn authentic_original_sweeper_26_events_and_export_firewall() {
    let b = authentic(
        "PHOENIX_COMIC_ORIGINAL_SOURCE",
        452284,
        "22a9e2589098b0d475f6e288eeebc9303bed644aa8ce09f9971a0060c9160486",
    );
    let map = map_at(&b, 36314..36548, 36333..36541, 26);
    // CCA-042/045 independent raw source inventory, not decoder-derived expectations.
    let values = [
        625000, 631509, 644521, 657644, 670594, 683589, 696634, 709646, 722623, 735759, 748654,
        761718, 774743, 787717, 800775, 813782, 826717, 839951, 852938, 865819, 878837, 891716,
        904810, 917722, 931950, 937500,
    ];
    let ticks = [
        0, 138240, 138480, 138720, 138960, 139200, 139440, 139680, 139920, 140160, 140400, 140640,
        140880, 141120, 141360, 141600, 141840, 142080, 142320, 142560, 142800, 143040, 143280,
        143520, 143760, 144001,
    ];
    let expected = ticks.into_iter().zip(values).collect::<Vec<_>>();
    assert_eq!(
        map.events()
            .iter()
            .map(|e| (e.absolute_tick, e.mpqn()))
            .collect::<Vec<_>>(),
        expected
    );
    assert_source_accounting(&b, &map);
    assert!(map
        .events()
        .windows(2)
        .all(|w| w[0].absolute_tick < w[1].absolute_tick && w[0].mpqn() < w[1].mpqn()));
    let chunk = smf::serialize_tempo_map_conductor(b"sweeper", &map, signature(), None).unwrap();
    assert_eq!(tempos(chunk.as_bytes()), expected);
    let meter = phoenix::meter::decode_bounded_initial_meter(
        &b,
        phoenix::meter::InitialMeterBounds {
            event_range: 36253..36261,
        },
    )
    .unwrap();
    assert_eq!(
        (
            meter.numerator.value,
            meter.denominator_exponent.value,
            meter.third_payload.value,
            meter.fourth_payload.value
        ),
        (4, 2, 8, 8)
    );
    let assembled = integrated(&map);
    assert_eq!(assembled.report.totals.tempo, 26);
    assert_eq!(tempos(conductor_chunk(&assembled.smf_bytes)), expected);
    // Whole conductor/source qualification is unchanged, not waived by assembly.
    assert!(phoenix::bounded_conductor::decode_compact_conductor(
        &b,
        [36234..36277, 36277..36314, 36314..36548, 36548..36800]
    )
    .is_err());
    assert!(phoenix::authority120::prove_project120(&b).unwrap()[5]
        .result
        .is_err());
    assert!(phoenix::recovery120::recover_project120(&b).unwrap()[5]
        .result
        .is_err());
    let mut service = phoenix::app_service::AppService::new();
    let response = service
        .inspect_project(phoenix::app_contract::InspectProjectRequest {
            contract_version: phoenix::app_contract::CONTRACT_VERSION,
            source_path: std::env::var("PHOENIX_COMIC_ORIGINAL_SOURCE").unwrap(),
            diagnostics_level: phoenix::app_contract::DiagnosticsLevel::Full,
        })
        .unwrap();
    assert!(response.sequences[5].bounded_export_capability.is_none());
    assert!(service
        .preview_experimental_sequence(&response.session_id, &response.sequences[5].sequence_id)
        .is_err());
}
#[test]
#[ignore = "requires explicitly supplied SCHOOL, standalone Minuet and existing MIDI reference"]
fn authentic_minuet_152_events_match_independent_midi_and_distinct_first_prefix() {
    let school = authentic(
        "PHOENIX_SCHOOL_OBSERVATION_SOURCE",
        343875,
        "bfd4fa1208e2cd884ec51ccfb1131d5c02723d7c90acafc98bd89597e1a20331",
    );
    let single = authentic(
        "PHOENIX_MINUET_STANDALONE_SOURCE",
        30536,
        "f95c8e75dbce342fdbfbce9083d0d91fcbf55f9dae585c3bc129bce5a9bc9a4a",
    );
    let reference = authentic(
        "PHOENIX_MINUET_REFERENCE_MIDI",
        20993,
        "641c29017f64f5c9fc285c5a5f9433db09c8953a03e63341d5b20c91827e49b8",
    );
    assert_eq!(&reference[..4], b"MThd");
    assert_eq!(&reference[12..14], &480u16.to_be_bytes());
    let end = 22 + u32::from_be_bytes(reference[18..22].try_into().unwrap()) as usize;
    let expected = tempos(&reference[14..end]);
    assert_eq!(expected.len(), 152);
    assert_eq!(expected[0], (1440, 499234));
    assert_eq!(expected[151], (152640, 600000));
    let a = map_at(&school, 197491..198734, 197510..198727, 152);
    let b = map_at(&single, 4013..5255, 4032..5248, 152);
    assert_source_accounting(&school, &a);
    assert_source_accounting(&single, &b);
    assert_eq!(
        a.events()
            .iter()
            .map(|e| (e.absolute_tick, e.mpqn()))
            .collect::<Vec<_>>(),
        expected
    );
    for ((a, b), (t, v)) in a.events().iter().zip(b.events()).zip(&expected) {
        assert_eq!(a.mpqn(), *v);
        assert_eq!(b.mpqn(), *v);
        assert_eq!(b.absolute_tick.checked_add(1440), Some(*t));
    }
    assert_eq!(&school[197518..198727], &single[4039..5248]);
    assert_eq!(a.events()[0].delta.bytes, [0x8b, 0x20]);
    assert_eq!(b.events()[0].delta.bytes, [0]);
    let reference_meta = meta(&reference[14..end]);
    let initial_meter = reference_meta.iter().find(|e| e.1 == 88).unwrap();
    assert_eq!(initial_meter.0, 0);
    assert_eq!(initial_meter.2, [3, 2, 24, 8]);
    let v = &initial_meter.2;
    let meter = TimeSignature::new(
        smf::MidiDataByte::new(v[0]).unwrap(),
        smf::MidiDataByte::new(v[1]).unwrap(),
        smf::MidiDataByte::new(v[2]).unwrap(),
        smf::MidiDataByte::new(v[3]).unwrap(),
    )
    .unwrap();
    let chunk = smf::serialize_tempo_map_conductor(b"minuet.mid", &a, meter, None).unwrap();
    assert_eq!(
        meta(chunk.as_bytes())
            .into_iter()
            .find(|e| e.1 == 88)
            .unwrap(),
        *initial_meter
    );
    assert_eq!(tempos(chunk.as_bytes()), expected);
}

// The production assembler's conductor branch, not a direct serializer test.
fn integrated(
    map: &phoenix::tempo::BoundedTempoMap<'_>,
) -> phoenix::multitrack_export::MultitrackExportResult {
    let input = phoenix::multitrack_export::MultitrackSequenceInput {
        sequence_name: b"S",
        tempo_mpqn: map.events()[0].mpqn(),
        meter_values: (4, 2, 8, 8),
        timing_policy: phoenix::midi_export::TimingPolicy::Identity480,
        meter_policy: phoenix::midi_export::MeterPolicy::KnownHistoricalOnly,
        tracks: &[],
    };
    phoenix::multitrack_export::assemble_multitrack_sequence_with_tempo_map(&input, map).unwrap()
}
fn conductor_chunk(file: &[u8]) -> &[u8] {
    assert_eq!(
        &file[..14],
        &[77, 84, 104, 100, 0, 0, 0, 6, 0, 1, 0, 1, 1, 224]
    );
    &file[14..]
}
#[test]
fn production_assembler_map_counts_positions_and_single_tempo_parity() {
    let raw = [
        0, 255, 81, 3, 7, 161, 32, 0x81, 0x70, 255, 81, 3, 9, 137, 104, 0, 255, 81, 3, 14, 78, 28,
    ];
    let map = decode(&raw, 3).unwrap();
    let result = integrated(&map);
    assert_eq!(result.report.totals.tempo, 3);
    assert_eq!(result.report.totals.meter, 1);
    assert_eq!(
        tempos(conductor_chunk(&result.smf_bytes)),
        [(0, 500000), (240, 625000), (240, 937500)]
    );
    let single = decode(&raw[..7], 1).unwrap();
    let input = phoenix::multitrack_export::MultitrackSequenceInput {
        sequence_name: b"S",
        tempo_mpqn: 500000,
        meter_values: (4, 2, 8, 8),
        timing_policy: phoenix::midi_export::TimingPolicy::Identity480,
        meter_policy: phoenix::midi_export::MeterPolicy::KnownHistoricalOnly,
        tracks: &[],
    };
    assert_eq!(
        integrated(&single),
        phoenix::multitrack_export::assemble_multitrack_sequence(&input).unwrap()
    );
    // Reuse the same production musical-track branch; a Tempo map cannot bypass
    // unsupported-event refusals or affect otherwise identical musical chunks.
    use phoenix::midi_export::{
        ChannelAssignment, ChannelAssignmentProvenance, DecodedExportEvent, DecodedExportEventKind,
        PatchPolicy,
    };
    let events = vec![DecodedExportEvent {
        absolute_position: 0,
        source_ordinal: 0,
        source_range: None,
        kind: DecodedExportEventKind::Note {
            pitch: 60,
            attack_velocity: 90,
            release_velocity: 45,
            duration: 480,
        },
    }];
    let track = phoenix::multitrack_export::MusicalTrackInput {
        context: "test",
        name: b"Piano",
        channel_assignment: ChannelAssignment {
            channel: smf::MidiChannel::new(2).unwrap(),
            provenance: ChannelAssignmentProvenance::ParsedRouting,
        },
        events: &events,
        patch_policy: PatchPolicy::StrictKnownOnly,
    };
    let tracks = [track];
    let musical_input = phoenix::multitrack_export::MultitrackSequenceInput {
        tracks: &tracks,
        ..input.clone()
    };
    let mapped = phoenix::multitrack_export::assemble_multitrack_sequence_with_tempo_map(
        &musical_input,
        &map,
    )
    .unwrap();
    let old = phoenix::multitrack_export::assemble_multitrack_sequence(&musical_input).unwrap();
    fn musical_tail(b: &[u8]) -> &[u8] {
        let n = u32::from_be_bytes(b[18..22].try_into().unwrap()) as usize;
        &b[22 + n..]
    }
    assert_eq!(
        musical_tail(&mapped.smf_bytes),
        musical_tail(&old.smf_bytes)
    );
    assert_eq!(mapped.report.tracks, old.report.tracks);
    let mut bad_events = events.clone();
    bad_events[0].kind = DecodedExportEventKind::Unsupported {
        family: "unsupported conductor cannot waive music",
    };
    let bad_tracks = [phoenix::multitrack_export::MusicalTrackInput {
        events: &bad_events,
        ..tracks[0].clone()
    }];
    let bad_input = phoenix::multitrack_export::MultitrackSequenceInput {
        tracks: &bad_tracks,
        ..input.clone()
    };
    assert!(
        phoenix::multitrack_export::assemble_multitrack_sequence_with_tempo_map(&bad_input, &map)
            .is_err()
    );
    let delayed = decode(&raw[7..15], 1).unwrap();
    assert_eq!(
        tempos(conductor_chunk(&integrated(&delayed).smf_bytes)),
        [(240, 625000)]
    );
}
#[test]
fn integrated_conductor_conflicts_and_unsupported_policies_refuse() {
    use phoenix::{
        midi_export::{MeterPolicy, TimingPolicy},
        multitrack_export::{assemble_multitrack_sequence_with_tempo_map, MultitrackSequenceInput},
    };
    let raw = body(&[(0, 500000), (0, 625000)]);
    let duplicate_initial = decode(&raw, 2).unwrap();
    let single = decode(&raw[..7], 1).unwrap();
    let mut input = MultitrackSequenceInput {
        sequence_name: b"S",
        tempo_mpqn: 500000,
        meter_values: (4, 2, 8, 8),
        timing_policy: TimingPolicy::Identity480,
        meter_policy: MeterPolicy::KnownHistoricalOnly,
        tracks: &[],
    };
    assert!(assemble_multitrack_sequence_with_tempo_map(&input, &duplicate_initial).is_err());
    input.tempo_mpqn = 625000;
    assert!(assemble_multitrack_sequence_with_tempo_map(&input, &single).is_err());
    input.tempo_mpqn = 500000;
    input.timing_policy = TimingPolicy::Unsupported;
    assert!(assemble_multitrack_sequence_with_tempo_map(&input, &single).is_err());
    input.timing_policy = TimingPolicy::Identity480;
    input.meter_values = (4, 2, 17, 8);
    assert!(assemble_multitrack_sequence_with_tempo_map(&input, &single).is_err());
}
