use phoenix::{
    midi_export::{
        adapt_conductor, adapt_meter_values, adapt_tempo_mpqn, adapt_text, adapt_track,
        ChannelAssignment, ChannelAssignmentProvenance, DecodedExportEvent, DecodedExportEventKind,
        ExportWarning, MeterPolicy, MidiExportError, PatchPolicy, PatchTranslation,
        TextConversionReason, TimingPolicy,
    },
    smf::{
        serialize_conductor_track, serialize_format1, serialize_musical_track,
        serialize_time_signature, ChannelMessage, MidiChannel, SmfSerializeError,
    },
};

fn assignment(channel: u8) -> ChannelAssignment {
    ChannelAssignment {
        channel: MidiChannel::new(channel).unwrap(),
        provenance: ChannelAssignmentProvenance::Synthetic,
    }
}

fn note(
    position: u32,
    ordinal: u64,
    pitch: u8,
    attack: u8,
    release: u8,
    duration: u32,
) -> DecodedExportEvent {
    DecodedExportEvent::from_note_fields(position, ordinal, None, pitch, attack, release, duration)
}

fn event(position: u32, ordinal: u64, kind: DecodedExportEventKind) -> DecodedExportEvent {
    DecodedExportEvent {
        absolute_position: position,
        source_ordinal: ordinal,
        source_range: None,
        kind,
    }
}

fn adapt(
    events: &[DecodedExportEvent],
    channel: u8,
) -> Result<phoenix::midi_export::ExportTrackResult, MidiExportError> {
    adapt_track(
        events,
        Some(assignment(channel)),
        TimingPolicy::Identity480,
        PatchPolicy::StrictKnownOnly,
    )
}

#[test]
fn note_creates_start_and_generated_end_with_release_velocity() {
    let result = adapt(&[note(120, 4, 60, 100, 37, 480)], 1).unwrap();
    assert_eq!(result.scheduled_events.len(), 2);
    assert_eq!(result.counts.notes, 1);
    assert_eq!(result.counts.generated_note_offs, 1);

    let on = result
        .scheduled_events
        .iter()
        .find(|scheduled| matches!(scheduled.message, ChannelMessage::NoteOn { .. }))
        .unwrap();
    let off = result
        .scheduled_events
        .iter()
        .find(|scheduled| matches!(scheduled.message, ChannelMessage::NoteOff { .. }))
        .unwrap();
    assert_eq!((on.absolute_tick, on.stable_ordinal), (120, 8));
    assert_eq!((off.absolute_tick, off.stable_ordinal), (600, 9));
    assert!(matches!(
        on.message,
        ChannelMessage::NoteOn { key, attack_velocity, .. }
            if key.get() == 60 && attack_velocity.get() == 100
    ));
    assert!(matches!(
        off.message,
        ChannelMessage::NoteOff { key, release_velocity, .. }
            if key.get() == 60 && release_velocity.get() == 37
    ));
}

#[test]
fn zero_duration_sorts_note_off_before_note_on() {
    let result = adapt(&[note(50, 0, 64, 90, 45, 0)], 1).unwrap();
    let serialized = serialize_musical_track(&result.scheduled_events).unwrap();
    assert_eq!(
        &serialized.as_bytes()[8..],
        &[50, 0x80, 64, 45, 0, 0x90, 64, 90, 0, 0xff, 0x2f, 0]
    );
}

#[test]
fn note_position_overflow_and_invalid_values_fail_transactionally() {
    assert_eq!(
        adapt(&[note(u32::MAX, 7, 60, 100, 64, 1)], 1),
        Err(MidiExportError::PositionOverflow {
            source_ordinal: 7,
            position: u32::MAX,
            duration: 1,
        })
    );
    assert_eq!(
        adapt(&[note(0, 8, 128, 100, 64, 1)], 1),
        Err(MidiExportError::InvalidMidiValue {
            source_ordinal: Some(8),
            source_range: None,
            source: SmfSerializeError::InvalidDataByte { value: 128 },
        })
    );

    let valid_then_invalid = [note(0, 0, 60, 100, 64, 10), note(20, 1, 61, 128, 64, 10)];
    assert!(adapt(&valid_then_invalid, 1).is_err());
}

#[test]
fn generated_ordinals_are_deterministic_for_equal_end_ticks() {
    let result = adapt(
        &[note(10, 2, 60, 100, 40, 90), note(20, 5, 61, 101, 41, 80)],
        1,
    )
    .unwrap();
    let endings: Vec<_> = result
        .scheduled_events
        .iter()
        .filter(|scheduled| matches!(scheduled.message, ChannelMessage::NoteOff { .. }))
        .map(|scheduled| (scheduled.absolute_tick, scheduled.stable_ordinal))
        .collect();
    assert_eq!(endings, [(100, 5), (100, 11)]);
}

#[test]
fn overlapping_same_pitch_notes_are_preserved_independently() {
    let result = adapt(
        &[note(0, 0, 60, 90, 30, 100), note(50, 1, 60, 91, 31, 100)],
        1,
    )
    .unwrap();
    assert_eq!(result.counts.notes, 2);
    assert_eq!(result.counts.generated_note_offs, 2);
    assert_eq!(result.scheduled_events.len(), 4);
    assert!(result.scheduled_events.iter().any(|scheduled| {
        scheduled.absolute_tick == 100
            && matches!(scheduled.message, ChannelMessage::NoteOff { .. })
    }));
    assert!(result.scheduled_events.iter().any(|scheduled| {
        scheduled.absolute_tick == 150
            && matches!(scheduled.message, ChannelMessage::NoteOff { .. })
    }));
}

#[test]
fn controller_maps_exactly_on_channels_one_and_sixteen() {
    let source = event(
        240,
        0,
        DecodedExportEventKind::Controller {
            number: 7,
            value: 127,
            has_opaque_context: true,
        },
    );
    for expected_channel in [1, 16] {
        let result = adapt(std::slice::from_ref(&source), expected_channel).unwrap();
        assert!(matches!(
            result.scheduled_events[0].message,
            ChannelMessage::ControlChange { channel, controller, value }
                if channel.get() == expected_channel && controller.get() == 7 && value.get() == 127
        ));
        assert_eq!(result.counts.controllers, 1);
        assert_eq!(result.untranslated_metadata.len(), 1);
    }
}

#[test]
fn controller_pressure_and_bend_reject_invalid_data() {
    let cases = [
        event(
            0,
            0,
            DecodedExportEventKind::Controller {
                number: 128,
                value: 1,
                has_opaque_context: false,
            },
        ),
        event(
            0,
            0,
            DecodedExportEventKind::Controller {
                number: 1,
                value: 128,
                has_opaque_context: false,
            },
        ),
        event(0, 0, DecodedExportEventKind::ChannelPressure { value: 128 }),
        event(
            0,
            0,
            DecodedExportEventKind::PitchBend { lsb: 128, msb: 64 },
        ),
        event(0, 0, DecodedExportEventKind::PitchBend { lsb: 0, msb: 128 }),
    ];
    for source in cases {
        assert!(matches!(
            adapt(&[source], 1),
            Err(MidiExportError::InvalidMidiValue {
                source: SmfSerializeError::InvalidDataByte { value: 128 },
                ..
            })
        ));
    }
}

#[test]
fn pressure_and_pitch_bend_map_without_value_transformation() {
    let result = adapt(
        &[
            event(10, 0, DecodedExportEventKind::ChannelPressure { value: 79 }),
            event(20, 1, DecodedExportEventKind::PitchBend { lsb: 63, msb: 3 }),
        ],
        16,
    )
    .unwrap();
    assert!(matches!(
        result.scheduled_events[0].message,
        ChannelMessage::ChannelPressure { channel, pressure }
            if channel.get() == 16 && pressure.get() == 79
    ));
    assert!(matches!(
        result.scheduled_events[1].message,
        ChannelMessage::PitchBend { channel, lsb, msb }
            if channel.get() == 16 && lsb.get() == 63 && msb.get() == 3
    ));
    assert_eq!(result.counts.channel_pressure, 1);
    assert_eq!(result.counts.pitch_bend, 1);
}

#[test]
fn patch_supports_confirmed_program_only_msb_only_and_full_bank_translation() {
    let program_only = event(
        100,
        0,
        DecodedExportEventKind::Patch {
            program: 23,
            translation: PatchTranslation::ProgramOnlyConfirmed,
        },
    );
    let result = adapt(&[program_only], 15).unwrap();
    assert_eq!(result.scheduled_events.len(), 1);
    assert!(matches!(
        result.scheduled_events[0].message,
        ChannelMessage::ProgramChange { channel, program }
            if channel.get() == 15 && program.get() == 23
    ));

    let msb_only = event(
        290,
        1,
        DecodedExportEventKind::Patch {
            program: 35,
            translation: PatchTranslation::ConfirmedBankSelectMsb { msb: 81 },
        },
    );
    let result = adapt(&[msb_only], 12).unwrap();
    assert_eq!(result.scheduled_events.len(), 2);
    assert_eq!(result.counts.bank_select_msb, 1);
    assert_eq!(result.counts.bank_select_lsb, 0);
    assert_eq!(result.counts.program_changes, 1);
    assert!(result
        .scheduled_events
        .iter()
        .all(|scheduled| scheduled.absolute_tick == 290));
    let serialized = serialize_musical_track(&result.scheduled_events).unwrap();
    assert_eq!(
        &serialized.as_bytes()[8..],
        &[0x82, 0x22, 0xbb, 0, 81, 0, 0xcb, 35, 0, 0xff, 0x2f, 0]
    );

    let banked = event(
        480,
        1,
        DecodedExportEventKind::Patch {
            program: 29,
            translation: PatchTranslation::ConfirmedBankSelect { msb: 81, lsb: 2 },
        },
    );
    let result = adapt(&[banked], 1).unwrap();
    assert_eq!(result.scheduled_events.len(), 3);
    assert_eq!(result.counts.bank_select_msb, 1);
    assert_eq!(result.counts.bank_select_lsb, 1);
    assert_eq!(result.counts.program_changes, 1);
    assert!(result
        .scheduled_events
        .iter()
        .all(|scheduled| scheduled.absolute_tick == 480));
    let serialized = serialize_musical_track(&result.scheduled_events).unwrap();
    assert_eq!(
        &serialized.as_bytes()[8..],
        &[0x83, 0x60, 0xb0, 0, 81, 0, 0xb0, 32, 2, 0, 0xc0, 29, 0, 0xff, 0x2f, 0]
    );
}

#[test]
fn unsupported_patch_and_event_fail_without_partial_output() {
    let unsupported = event(
        0,
        4,
        DecodedExportEventKind::Patch {
            program: 1,
            translation: PatchTranslation::UnsupportedOpaque,
        },
    );
    assert_eq!(
        adapt(&[unsupported], 1),
        Err(MidiExportError::UnsupportedPatchTranslation {
            source_ordinal: 4,
            source_range: None,
        })
    );
    let family = event(
        0,
        5,
        DecodedExportEventKind::Unsupported { family: "SysEx" },
    );
    assert_eq!(
        adapt(&[family], 1),
        Err(MidiExportError::UnsupportedEvent {
            source_ordinal: 5,
            family: "SysEx",
        })
    );
}

#[test]
fn explicit_channel_timing_and_ordinal_requirements_are_enforced() {
    assert_eq!(
        adapt_track(
            &[],
            None,
            TimingPolicy::Identity480,
            PatchPolicy::StrictKnownOnly
        ),
        Err(MidiExportError::UnknownChannel)
    );
    assert_eq!(
        adapt_track(
            &[],
            Some(assignment(1)),
            TimingPolicy::Unsupported,
            PatchPolicy::StrictKnownOnly
        ),
        Err(MidiExportError::UnsupportedTimingConversion)
    );
    assert_eq!(
        adapt(&[note(0, 3, 60, 1, 1, 1), note(2, 3, 61, 1, 1, 1)], 1),
        Err(MidiExportError::DuplicateSourceOrdinal { source_ordinal: 3 })
    );
}

#[test]
fn meter_policies_cover_historical_and_fallback_paths() {
    let (four_four, warnings) = adapt_meter_values(
        4,
        2,
        8,
        8,
        MeterPolicy::HistoricalWhenKnownOtherwiseStandard,
    )
    .unwrap();
    assert_eq!(
        serialize_time_signature(four_four),
        [0xff, 0x58, 4, 4, 2, 24, 8]
    );
    assert!(warnings.is_empty());

    let (six_eight, warnings) = adapt_meter_values(
        6,
        3,
        6,
        8,
        MeterPolicy::HistoricalWhenKnownOtherwiseStandard,
    )
    .unwrap();
    assert_eq!(
        serialize_time_signature(six_eight),
        [0xff, 0x58, 4, 6, 3, 12, 8]
    );
    assert!(warnings.is_empty());

    let (fallback, warnings) = adapt_meter_values(
        5,
        2,
        9,
        200,
        MeterPolicy::HistoricalWhenKnownOtherwiseStandard,
    )
    .unwrap();
    assert_eq!(
        serialize_time_signature(fallback),
        [0xff, 0x58, 4, 5, 2, 24, 8]
    );
    assert_eq!(
        warnings,
        [
            ExportWarning::MeterClocksFallback {
                source_third_payload: 9,
                used: 24
            },
            ExportWarning::MeterThirtySecondsFallback {
                source_fourth_payload: 200,
                used: 8
            },
        ]
    );
    assert_eq!(
        adapt_meter_values(5, 2, 9, 8, MeterPolicy::KnownHistoricalOnly),
        Err(MidiExportError::UnsupportedMeterMapping { third_payload: 9 })
    );
}

#[test]
fn tempo_and_text_adaptation_are_narrow_and_explicit() {
    assert_eq!(adapt_tempo_mpqn(500_000).unwrap(), 500_000);
    assert_eq!(adapt_tempo_mpqn(461_538).unwrap(), 461_538);
    for invalid in [0, 0x0100_0000] {
        assert_eq!(
            adapt_tempo_mpqn(invalid),
            Err(MidiExportError::InvalidMidiValue {
                source_ordinal: None,
                source_range: None,
                source: SmfSerializeError::InvalidTempo { mpqn: invalid },
            })
        );
    }
    assert_eq!(
        adapt_text("Clarke ☃".as_bytes()).unwrap(),
        "Clarke ☃".as_bytes()
    );
    assert_eq!(
        adapt_text(b"bad\0name"),
        Err(MidiExportError::TextConversion {
            reason: TextConversionReason::InteriorNul
        })
    );
    assert_eq!(
        adapt_text(&[0x8e]),
        Err(MidiExportError::TextConversion {
            reason: TextConversionReason::MacRomanDeferred
        })
    );
}

#[test]
fn synthetic_adapter_to_complete_format_one_smf_is_valid() {
    let conductor = adapt_conductor(
        b"Synthetic",
        500_000,
        (4, 2, 8, 8),
        TimingPolicy::Identity480,
        MeterPolicy::HistoricalWhenKnownOtherwiseStandard,
    )
    .unwrap();
    let track = adapt(
        &[
            event(
                0,
                0,
                DecodedExportEventKind::Patch {
                    program: 10,
                    translation: PatchTranslation::ProgramOnlyConfirmed,
                },
            ),
            note(480, 1, 64, 100, 50, 480),
        ],
        16,
    )
    .unwrap();
    let conductor_track = serialize_conductor_track(
        &conductor.sequence_name,
        conductor.tempo_mpqn,
        conductor.time_signature,
    )
    .unwrap();
    let musical_track = serialize_musical_track(&track.scheduled_events).unwrap();
    let file = serialize_format1(conductor.ppqn, &[conductor_track, musical_track]).unwrap();

    independent_validate_synthetic(&file);
}

fn independent_validate_synthetic(file: &[u8]) {
    assert_eq!(&file[..14], b"MThd\0\0\0\x06\0\x01\0\x02\x01\xe0");
    let mut cursor = 14;
    let mut payloads = Vec::new();
    for _ in 0..2 {
        assert_eq!(&file[cursor..cursor + 4], b"MTrk");
        let length = u32::from_be_bytes(file[cursor + 4..cursor + 8].try_into().unwrap()) as usize;
        payloads.push(&file[cursor + 8..cursor + 8 + length]);
        cursor += 8 + length;
    }
    assert_eq!(cursor, file.len());
    assert!(payloads[0]
        .windows(6)
        .any(|window| window == [0xff, 0x51, 3, 7, 0xa1, 0x20]));
    assert!(payloads[0]
        .windows(7)
        .any(|window| window == [0xff, 0x58, 4, 4, 2, 24, 8]));
    assert!(payloads[1].windows(2).any(|window| window == [0xcf, 10]));
    assert!(payloads[1]
        .windows(3)
        .any(|window| window == [0x9f, 64, 100]));
    assert!(payloads[1]
        .windows(3)
        .any(|window| window == [0x8f, 64, 50]));
    for payload in payloads {
        assert!(payload.ends_with(&[0, 0xff, 0x2f, 0]));
        assert_eq!(
            payload
                .windows(3)
                .filter(|window| *window == [0xff, 0x2f, 0])
                .count(),
            1
        );
    }
}
