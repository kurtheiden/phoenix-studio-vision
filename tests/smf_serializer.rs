use phoenix::smf::{
    encode_midi_vlq, serialize_channel_message, serialize_conductor_track, serialize_end_of_track,
    serialize_format1, serialize_musical_track, serialize_set_tempo, serialize_time_signature,
    serialize_track_name, ChannelMessage, MidiChannel, MidiDataByte, ScheduledEvent,
    SmfSerializeError, TimeSignature, MAX_MIDI_VLQ,
};

fn channel(value: u8) -> MidiChannel {
    MidiChannel::new(value).unwrap()
}

fn data(value: u8) -> MidiDataByte {
    MidiDataByte::new(value).unwrap()
}

fn signature() -> TimeSignature {
    TimeSignature::new(data(4), data(2), data(24), data(8)).unwrap()
}

fn event(absolute_tick: u32, stable_ordinal: u64, message: ChannelMessage) -> ScheduledEvent {
    ScheduledEvent {
        absolute_tick,
        stable_ordinal,
        message,
    }
}

#[test]
fn format_one_header_contains_track_count_and_ppqn() {
    let conductor = serialize_conductor_track(b"Sequence", 500_000, signature()).unwrap();
    let musical = serialize_musical_track(&[]).unwrap();
    let bytes = serialize_format1(480, &[conductor, musical]).unwrap();

    assert_eq!(&bytes[..14], b"MThd\0\0\0\x06\0\x01\0\x02\x01\xe0");
    assert_eq!(
        serialize_format1(0, &[serialize_musical_track(&[]).unwrap()]),
        Err(SmfSerializeError::InvalidPpqn { value: 0 })
    );
    assert_eq!(
        serialize_format1(0x8000, &[serialize_musical_track(&[]).unwrap()]),
        Err(SmfSerializeError::InvalidPpqn { value: 0x8000 })
    );
    assert_eq!(
        serialize_format1(480, &[]),
        Err(SmfSerializeError::TrackCountOverflow { count: 0 })
    );
}

#[test]
fn midi_vlq_encodes_every_required_boundary_and_rejects_overflow() {
    let cases: &[(u32, &[u8])] = &[
        (0, &[0x00]),
        (0x7f, &[0x7f]),
        (0x80, &[0x81, 0x00]),
        (0x3fff, &[0xff, 0x7f]),
        (0x4000, &[0x81, 0x80, 0x00]),
        (0x1f_ffff, &[0xff, 0xff, 0x7f]),
        (0x20_0000, &[0x81, 0x80, 0x80, 0x00]),
        (MAX_MIDI_VLQ, &[0xff, 0xff, 0xff, 0x7f]),
    ];
    for (value, expected) in cases {
        assert_eq!(encode_midi_vlq(*value).unwrap(), *expected);
    }
    assert_eq!(
        encode_midi_vlq(MAX_MIDI_VLQ + 1),
        Err(SmfSerializeError::MidiVlqOverflow {
            value: MAX_MIDI_VLQ + 1
        })
    );
}

#[test]
fn channels_are_human_numbered_and_data_bytes_are_checked() {
    assert_eq!(MidiChannel::new(1).unwrap().get(), 1);
    assert_eq!(MidiChannel::new(16).unwrap().get(), 16);
    assert_eq!(
        MidiChannel::new(0),
        Err(SmfSerializeError::InvalidChannel { value: 0 })
    );
    assert_eq!(
        MidiChannel::new(17),
        Err(SmfSerializeError::InvalidChannel { value: 17 })
    );
    assert_eq!(
        MidiDataByte::new(128),
        Err(SmfSerializeError::InvalidDataByte { value: 128 })
    );

    let channel_one = ChannelMessage::ProgramChange {
        channel: channel(1),
        program: data(1),
    };
    let channel_sixteen = ChannelMessage::ProgramChange {
        channel: channel(16),
        program: data(1),
    };
    assert_eq!(serialize_channel_message(&channel_one), [0xc0, 1]);
    assert_eq!(serialize_channel_message(&channel_sixteen), [0xcf, 1]);
}

#[test]
fn all_supported_channel_messages_have_exact_explicit_status_bytes() {
    let ch = channel(16);
    let messages = [
        (
            ChannelMessage::NoteOff {
                channel: ch,
                key: data(60),
                release_velocity: data(64),
            },
            vec![0x8f, 60, 64],
        ),
        (
            ChannelMessage::NoteOn {
                channel: ch,
                key: data(60),
                attack_velocity: data(100),
            },
            vec![0x9f, 60, 100],
        ),
        (
            ChannelMessage::ControlChange {
                channel: ch,
                controller: data(7),
                value: data(127),
            },
            vec![0xbf, 7, 127],
        ),
        (
            ChannelMessage::ProgramChange {
                channel: ch,
                program: data(23),
            },
            vec![0xcf, 23],
        ),
        (
            ChannelMessage::ChannelPressure {
                channel: ch,
                pressure: data(79),
            },
            vec![0xdf, 79],
        ),
        (
            ChannelMessage::PitchBend {
                channel: ch,
                lsb: data(0),
                msb: data(64),
            },
            vec![0xef, 0, 64],
        ),
    ];
    for (message, expected) in messages {
        assert_eq!(serialize_channel_message(&message), expected);
    }
}

#[test]
fn supported_meta_events_have_exact_bytes_and_validation() {
    assert_eq!(
        serialize_track_name(b"Piano").unwrap(),
        b"\xff\x03\x05Piano"
    );
    let long_name = vec![b'x'; 128];
    let serialized = serialize_track_name(&long_name).unwrap();
    assert_eq!(&serialized[..4], &[0xff, 0x03, 0x81, 0x00]);
    assert_eq!(serialized.len(), 132);
    assert_eq!(
        serialize_set_tempo(500_000).unwrap(),
        [0xff, 0x51, 0x03, 0x07, 0xa1, 0x20]
    );
    assert_eq!(
        serialize_set_tempo(0),
        Err(SmfSerializeError::InvalidTempo { mpqn: 0 })
    );
    assert_eq!(
        serialize_set_tempo(0x0100_0000),
        Err(SmfSerializeError::InvalidTempo { mpqn: 0x0100_0000 })
    );
    assert_eq!(
        serialize_time_signature(signature()),
        [0xff, 0x58, 0x04, 4, 2, 24, 8]
    );
    assert_eq!(serialize_end_of_track(), [0xff, 0x2f, 0]);
    assert_eq!(
        TimeSignature::new(data(0), data(2), data(24), data(8)),
        Err(SmfSerializeError::InvalidTimeSignatureNumerator { value: 0 })
    );
}

#[test]
fn tiny_track_has_checked_length_and_one_final_eot() {
    let track = serialize_musical_track(&[event(
        0,
        0,
        ChannelMessage::NoteOn {
            channel: channel(1),
            key: data(60),
            attack_velocity: data(100),
        },
    )])
    .unwrap();
    assert_eq!(
        track.as_bytes(),
        b"MTrk\0\0\0\x08\0\x90\x3c\x64\0\xff\x2f\0"
    );
    assert_eq!(
        track
            .as_bytes()
            .windows(3)
            .filter(|window| *window == [0xff, 0x2f, 0])
            .count(),
        1
    );
    assert!(track.as_bytes().ends_with(&[0, 0xff, 0x2f, 0]));
}

#[test]
fn absolute_ticks_become_deltas_and_simultaneous_events_use_zero() {
    let events = [
        event(
            128,
            0,
            ChannelMessage::ProgramChange {
                channel: channel(1),
                program: data(1),
            },
        ),
        event(
            128,
            1,
            ChannelMessage::NoteOn {
                channel: channel(1),
                key: data(60),
                attack_velocity: data(1),
            },
        ),
    ];
    let track = serialize_musical_track(&events).unwrap();
    assert_eq!(
        &track.as_bytes()[8..],
        &[0x81, 0x00, 0xc0, 1, 0, 0x90, 60, 1, 0, 0xff, 0x2f, 0]
    );

    let long = serialize_musical_track(&[event(
        MAX_MIDI_VLQ,
        0,
        ChannelMessage::ProgramChange {
            channel: channel(1),
            program: data(1),
        },
    )])
    .unwrap();
    assert_eq!(&long.as_bytes()[8..12], &[0xff, 0xff, 0xff, 0x7f]);
    assert_eq!(
        serialize_musical_track(&[event(
            MAX_MIDI_VLQ + 1,
            0,
            ChannelMessage::ProgramChange {
                channel: channel(1),
                program: data(1),
            },
        )]),
        Err(SmfSerializeError::MidiVlqOverflow {
            value: MAX_MIDI_VLQ + 1
        })
    );
}

#[test]
fn same_tick_policy_and_stable_ordinals_are_explicit() {
    let ch = channel(1);
    let events = vec![
        event(
            0,
            8,
            ChannelMessage::NoteOn {
                channel: ch,
                key: data(60),
                attack_velocity: data(8),
            },
        ),
        event(
            0,
            7,
            ChannelMessage::NoteOff {
                channel: ch,
                key: data(60),
                release_velocity: data(7),
            },
        ),
        event(
            0,
            6,
            ChannelMessage::ChannelPressure {
                channel: ch,
                pressure: data(6),
            },
        ),
        event(
            0,
            5,
            ChannelMessage::PitchBend {
                channel: ch,
                lsb: data(5),
                msb: data(0),
            },
        ),
        event(
            0,
            41,
            ChannelMessage::ControlChange {
                channel: ch,
                controller: data(1),
                value: data(41),
            },
        ),
        event(
            0,
            40,
            ChannelMessage::ControlChange {
                channel: ch,
                controller: data(1),
                value: data(40),
            },
        ),
        event(
            0,
            3,
            ChannelMessage::ProgramChange {
                channel: ch,
                program: data(3),
            },
        ),
        event(
            0,
            2,
            ChannelMessage::ControlChange {
                channel: ch,
                controller: data(32),
                value: data(2),
            },
        ),
        event(
            0,
            1,
            ChannelMessage::ControlChange {
                channel: ch,
                controller: data(0),
                value: data(1),
            },
        ),
    ];
    let track = serialize_musical_track(&events).unwrap();
    assert_eq!(
        &track.as_bytes()[8..],
        &[
            0, 0xb0, 0, 1, 0, 0xb0, 32, 2, 0, 0xc0, 3, 0, 0xb0, 1, 40, 0, 0xb0, 1, 41, 0, 0xe0, 5,
            0, 0, 0xd0, 6, 0, 0x80, 60, 7, 0, 0x90, 60, 8, 0, 0xff, 0x2f, 0,
        ]
    );
}

#[test]
fn conductor_track_has_fixed_tick_zero_order_and_eot() {
    let track = serialize_conductor_track(b"Test", 500_000, signature()).unwrap();
    assert_eq!(
        track.as_bytes(),
        &[
            b'M', b'T', b'r', b'k', 0, 0, 0, 27, 0, 0xff, 0x03, 4, b'T', b'e', b's', b't', 0, 0xff,
            0x51, 3, 0x07, 0xa1, 0x20, 0, 0xff, 0x58, 4, 4, 2, 24, 8, 0, 0xff, 0x2f, 0,
        ]
    );
}

#[test]
fn independently_parses_a_complete_synthetic_format_one_file() {
    let conductor = serialize_conductor_track(b"Independent", 500_000, signature()).unwrap();
    let musical = serialize_musical_track(&[
        event(
            0,
            0,
            ChannelMessage::ProgramChange {
                channel: channel(16),
                program: data(10),
            },
        ),
        event(
            480,
            1,
            ChannelMessage::NoteOn {
                channel: channel(16),
                key: data(64),
                attack_velocity: data(100),
            },
        ),
        event(
            960,
            2,
            ChannelMessage::NoteOff {
                channel: channel(16),
                key: data(64),
                release_velocity: data(50),
            },
        ),
    ])
    .unwrap();
    let file = serialize_format1(480, &[conductor, musical]).unwrap();

    independent_validate(&file, 2, 480);
}

fn independent_validate(file: &[u8], expected_tracks: u16, expected_ppqn: u16) {
    assert_eq!(&file[0..4], b"MThd");
    assert_eq!(u32::from_be_bytes(file[4..8].try_into().unwrap()), 6);
    assert_eq!(u16::from_be_bytes(file[8..10].try_into().unwrap()), 1);
    assert_eq!(
        u16::from_be_bytes(file[10..12].try_into().unwrap()),
        expected_tracks
    );
    assert_eq!(
        u16::from_be_bytes(file[12..14].try_into().unwrap()),
        expected_ppqn
    );
    assert_eq!(expected_ppqn & 0x8000, 0);

    let mut offset = 14;
    for _ in 0..expected_tracks {
        assert_eq!(&file[offset..offset + 4], b"MTrk");
        let length = u32::from_be_bytes(file[offset + 4..offset + 8].try_into().unwrap()) as usize;
        let payload = &file[offset + 8..offset + 8 + length];
        validate_track_payload(payload);
        offset += 8 + length;
    }
    assert_eq!(offset, file.len());
}

fn validate_track_payload(payload: &[u8]) {
    let mut offset = 0;
    let mut eot_count = 0;
    while offset < payload.len() {
        let (_, next) = independent_vlq(payload, offset);
        offset = next;
        let status = payload[offset];
        offset += 1;
        match status {
            0xff => {
                let kind = payload[offset];
                offset += 1;
                let (length, next) = independent_vlq(payload, offset);
                offset = next;
                assert!(offset + length <= payload.len());
                if kind == 0x2f {
                    assert_eq!(length, 0);
                    eot_count += 1;
                    assert_eq!(offset, payload.len());
                } else {
                    assert!(matches!(kind, 0x03 | 0x51 | 0x58));
                    offset += length;
                }
            }
            0x80..=0xbf | 0xe0..=0xef => {
                assert!(payload[offset] < 0x80 && payload[offset + 1] < 0x80);
                offset += 2;
            }
            0xc0..=0xdf => {
                assert!(payload[offset] < 0x80);
                offset += 1;
            }
            _ => panic!("unsupported or running status {status:02x}"),
        }
    }
    assert_eq!(eot_count, 1);
}

fn independent_vlq(bytes: &[u8], mut offset: usize) -> (usize, usize) {
    let mut value = 0_usize;
    let start = offset;
    loop {
        assert!(offset - start < 4);
        let byte = bytes[offset];
        offset += 1;
        value = (value << 7) | usize::from(byte & 0x7f);
        if byte & 0x80 == 0 {
            return (value, offset);
        }
    }
}
