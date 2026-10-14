//! Source-derived Instrument pitch processing in the established 36-byte table.
//! Only independently corroborated encodings 00 and F4 are admitted. Selection
//! and ownership are supplied by the existing same-source routing proof; no
//! project identity, MIDI reference or musical expectation is consulted.
use crate::midi_export::{DecodedExportEvent, DecodedExportEventKind};
use crate::sequence_container::FramedRecord;
use serde::Serialize;
use std::ops::Range;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct InstrumentTransposition {
    pub association_range: Range<usize>,
    pub instrument_row: u8,
    pub instrument_record_range: Range<usize>,
    pub encoding_offset: usize,
    pub encoded_value: u8,
    pub semitones: i8,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct NotePitchTransformation {
    pub source_ordinal: u64,
    pub source_range: Range<usize>,
    pub original_pitch: u8,
    pub exported_pitch: u8,
    pub applied_semitones: i8,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct InstrumentPitchProvenance {
    pub instrument: InstrumentTransposition,
    /// Every changed Note, not only a track-level correction. Zero-transpose
    /// tracks retain their unchanged raw Note handoff and Instrument metadata.
    pub changed_notes: Vec<NotePitchTransformation>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TranspositionRefusal {
    InstrumentShape,
    UnsupportedEncoding(u8),
    PitchRange {
        source_ordinal: u64,
        original_pitch: u8,
        semitones: i8,
    },
    MissingNoteProvenance,
}
impl InstrumentTransposition {
    pub(crate) fn from_selected_record(
        selected: &FramedRecord<'_>,
        association_range: Range<usize>,
        instrument_row: u8,
    ) -> Result<Self, TranspositionRefusal> {
        let p = selected.payload.bytes;
        if selected.record_type.value != 0x10 || p.len() != 36 || p[25] != instrument_row {
            return Err(TranspositionRefusal::InstrumentShape);
        }
        let semitones = match p[34] {
            0 => 0,
            0xf4 => -12,
            other => return Err(TranspositionRefusal::UnsupportedEncoding(other)),
        };
        Ok(Self {
            association_range,
            instrument_row,
            instrument_record_range: selected.record_range.clone(),
            encoding_offset: selected.payload.range.start + 34,
            encoded_value: p[34],
            semitones,
        })
    }

    /// Clone for adaptation; original decoder and handoff values stay untouched.
    /// Validate every Note transactionally before returning a transformed stream.
    pub(crate) fn handoff(
        &self,
        original: &[DecodedExportEvent],
    ) -> Result<(Vec<DecodedExportEvent>, InstrumentPitchProvenance), TranspositionRefusal> {
        let mut events = original.to_vec();
        let mut changed_notes = Vec::new();
        for event in &mut events {
            if let DecodedExportEventKind::Note { pitch, .. } = &mut event.kind {
                let effective = i16::from(*pitch) + i16::from(self.semitones);
                if *pitch > 127 || !(0..=127).contains(&effective) {
                    return Err(TranspositionRefusal::PitchRange {
                        source_ordinal: event.source_ordinal,
                        original_pitch: *pitch,
                        semitones: self.semitones,
                    });
                }
                if self.semitones != 0 {
                    changed_notes.push(NotePitchTransformation {
                        source_ordinal: event.source_ordinal,
                        source_range: event
                            .source_range
                            .clone()
                            .ok_or(TranspositionRefusal::MissingNoteProvenance)?,
                        original_pitch: *pitch,
                        exported_pitch: effective as u8,
                        applied_semitones: self.semitones,
                    });
                    *pitch = effective as u8;
                }
            }
        }
        Ok((
            events,
            InstrumentPitchProvenance {
                instrument: self.clone(),
                changed_notes,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recovery120::recover_project120;
    use crate::sequence_container::parse_root_record_stream;

    fn fixture(pitch: u8, encoding: u8) -> Vec<u8> {
        let mut b = crate::recovery120::tests::fixture(&[0, 0x90, pitch, 100, 64, 10], 1);
        let root = parse_root_record_stream(&b).unwrap();
        let offset = root
            .records
            .iter()
            .find(|r| r.record_type.value == 16 && r.payload.bytes[25] == 1)
            .unwrap()
            .payload
            .range
            .start
            + 34;
        b[offset] = encoding;
        b
    }
    #[test]
    fn source_handoff_is_raw_and_both_midi_endpoints_use_effective_pitch() {
        for (encoding, raw, effective) in
            [(0, 60, 60), (0xf4, 60, 48), (0xf4, 12, 0), (0xf4, 127, 115)]
        {
            let b = fixture(raw, encoding);
            let r = recover_project120(&b).unwrap().remove(0).result.unwrap();
            let t = &r.tracks()[0];
            assert!(
                matches!(t.handoff[0].kind, DecodedExportEventKind::Note { pitch, .. } if pitch == raw)
            );
            let provenance = t.pitch_provenance.as_ref().unwrap();
            assert_eq!(provenance.instrument.encoded_value, encoding);
            assert_eq!(provenance.changed_notes.len(), usize::from(encoding != 0));
            if encoding != 0 {
                assert_eq!(
                    (
                        provenance.changed_notes[0].original_pitch,
                        provenance.changed_notes[0].exported_pitch
                    ),
                    (raw, effective)
                );
            }
            assert!(
                matches!(t.scheduled[0].message, crate::smf::ChannelMessage::NoteOn { key, .. } if key.get() == effective)
            );
            assert!(
                matches!(t.scheduled[1].message, crate::smf::ChannelMessage::NoteOff { key, .. } if key.get() == effective)
            );
            assert_eq!(r.proof().source(), b);
        }
    }
    #[test]
    fn unsupported_encodings_and_underflow_fail_closed() {
        for value in [1, 12, 0xff, 0xf3, 0x80] {
            let b = fixture(60, value);
            assert!(recover_project120(&b).unwrap().remove(0).result.is_err());
        }
        for pitch in 0..12 {
            let b = fixture(pitch, 0xf4);
            assert!(recover_project120(&b).unwrap().remove(0).result.is_err());
        }
    }
    #[test]
    fn transposition_follows_selected_row_not_channel_or_table_order_guess() {
        let mut b = fixture(60, 0);
        let device = parse_root_record_stream(&b)
            .unwrap()
            .records
            .iter()
            .find(|r| r.record_type.value == 42)
            .unwrap()
            .record_range
            .start;
        let mut row = vec![0; 36];
        row[25] = 2;
        row[26] = 7;
        row[27] = 4;
        row[34] = 0xf4;
        let mut record = vec![16];
        record.extend(36u32.to_be_bytes());
        record.extend(row);
        b.splice(device..device, record);
        // Same channel/device, another structurally selected Instrument row.
        b[8 + 121 + 240 - 31] = 2;
        let r = recover_project120(&b).unwrap().remove(0).result.unwrap();
        assert_eq!(
            r.tracks()[0]
                .pitch_provenance
                .as_ref()
                .unwrap()
                .instrument
                .instrument_row,
            2
        );
        assert!(
            matches!(r.tracks()[0].scheduled[0].message, crate::smf::ChannelMessage::NoteOn { key, .. } if key.get() == 48)
        );
        b[8 + 121 + 240 - 31] = 1;
        let r = recover_project120(&b).unwrap().remove(0).result.unwrap();
        assert!(
            matches!(r.tracks()[0].scheduled[0].message, crate::smf::ChannelMessage::NoteOn { key, .. } if key.get() == 60)
        );
    }
    #[test]
    fn transformation_is_transactional_and_does_not_change_non_note_events() {
        let b = fixture(60, 0xf4);
        let root = parse_root_record_stream(&b).unwrap();
        let selected = root
            .records
            .iter()
            .find(|r| r.record_type.value == 16 && r.payload.bytes[25] == 1)
            .unwrap();
        let rule = InstrumentTransposition::from_selected_record(selected, 336..369, 1).unwrap();
        let events = vec![
            DecodedExportEvent::from_note_fields(123, 0, Some(10..16), 60, 100, 64, 10),
            DecodedExportEvent {
                absolute_position: 124,
                source_ordinal: 1,
                source_range: Some(16..20),
                kind: DecodedExportEventKind::MidiController {
                    channel: 1,
                    number: 1,
                    value: 42,
                },
            },
        ];
        let before = events.clone();
        let (effective, p) = rule.handoff(&events).unwrap();
        assert_eq!(events, before);
        assert_eq!(effective[1], before[1]);
        assert_eq!(p.changed_notes[0].source_range, 10..16);
        let mut bad = events.clone();
        bad.push(DecodedExportEvent::from_note_fields(
            125,
            2,
            Some(20..26),
            11,
            100,
            64,
            10,
        ));
        let original = bad.clone();
        assert!(rule.handoff(&bad).is_err());
        assert_eq!(bad, original);
    }
    #[test]
    fn malformed_selected_records_and_missing_note_provenance_refuse() {
        let b = fixture(60, 0xf4);
        let root = parse_root_record_stream(&b).unwrap();
        let selected = root
            .records
            .iter()
            .find(|r| r.record_type.value == 16 && r.payload.bytes[25] == 1)
            .unwrap();
        assert!(InstrumentTransposition::from_selected_record(selected, 336..369, 2).is_err());
        let mut short = selected.clone();
        short.payload.bytes = &short.payload.bytes[..34];
        assert!(InstrumentTransposition::from_selected_record(&short, 336..369, 1).is_err());
        let rule = InstrumentTransposition::from_selected_record(selected, 336..369, 1).unwrap();
        let missing = [DecodedExportEvent::from_note_fields(
            0, 0, None, 60, 100, 64, 10,
        )];
        assert_eq!(
            rule.handoff(&missing),
            Err(TranspositionRefusal::MissingNoteProvenance)
        );
        let zero = InstrumentTransposition {
            semitones: 0,
            encoded_value: 0,
            ..rule
        };
        let invalid = [DecodedExportEvent::from_note_fields(
            0,
            0,
            Some(10..16),
            128,
            100,
            64,
            10,
        )];
        assert!(zero.handoff(&invalid).is_err());
    }
    #[test]
    #[ignore = "requires authentic SCHOOL PROJECTS and existing native MIDI"]
    fn authentic_school_source_rule_and_held_out_parts_match_native() {
        let source =
            std::fs::read(std::env::var("PHOENIX_SCHOOL_ORIGINAL_SOURCE").unwrap()).unwrap();
        let reference =
            std::fs::read(std::env::var("PHOENIX_SCHOOL_TRANSPOSE_REFERENCE").unwrap()).unwrap();
        let bridge = crate::semantic_layout120::associate_semantic_project_120(&source).unwrap();
        let s = bridge
            .candidates
            .iter()
            .filter_map(|c| c.association.as_ref().ok())
            .find(|s| s.source_name.bytes == b"M-vegetarian del")
            .expect("original sequence association");
        let root = parse_root_record_stream(&source).unwrap();
        for (label, expected) in [
            (b"c-b13 dyna tom lo".as_slice(), 317),
            (b"kick/snare".as_slice(), 225),
            (b"agogo/bongo".as_slice(), 158),
        ] {
            let t = s
                .ordinary
                .iter()
                .find(|t| t.slot.label.bytes == label)
                .unwrap();
            let bounds = crate::sequence_container::TrackRecordPair {
                pair_ordinal: t.local_pair_ordinal,
                primary: t.pair.primary.clone(),
                secondary: t.pair.secondary.clone(),
                candidate_event_start: t.pair.primary.payload.range.start + 14,
                event_containing_range: t.pair.primary.payload.range.start + 14
                    ..t.pair.primary.payload.range.end,
            }
            .validated_event_bounds()
            .unwrap();
            let walk = crate::mixed_event::walk_bounded_mixed_events(
                &source,
                crate::mixed_event::MixedEventBounds {
                    event_range: bounds.event_range,
                },
                Default::default(),
            )
            .unwrap();
            let start = t.slot.label.range.start - 33;
            let route = crate::authority120::single_route(
                &root.records,
                start..t.slot.label.range.start,
                &source[start..t.slot.label.range.start],
                &walk,
            )
            .unwrap();
            let raw = crate::recovery120::single_handoff(&walk, Some(t.slot.ordinal)).unwrap();
            let original = raw.clone();
            let (effective, provenance) = route.transposition.handoff(&raw).unwrap();
            assert_eq!(raw, original);
            assert_eq!(route.transposition.semitones, -12);
            assert_eq!(provenance.changed_notes.len(), expected);
            let actual: Vec<_> = effective
                .iter()
                .filter_map(|e| match e.kind {
                    DecodedExportEventKind::Note {
                        pitch,
                        attack_velocity,
                        ..
                    } => Some((e.absolute_position, route.channel, pitch, attack_velocity)),
                    _ => None,
                })
                .collect();
            assert_eq!(actual, reference_note_ons(&reference, label));
        }
    }

    // Independent test-only SMF reader. No reference is accessed by runtime code.
    fn reference_note_ons(b: &[u8], wanted: &[u8]) -> Vec<(u32, u8, u8, u8)> {
        fn vlq(b: &[u8], q: &mut usize) -> u32 {
            let mut v = 0;
            for _ in 0..4 {
                let x = b[*q];
                *q += 1;
                v = v * 128 + u32::from(x & 127);
                if x < 128 {
                    return v;
                }
            }
            panic!("invalid MIDI VLQ")
        }
        assert_eq!(&b[..4], b"MThd");
        let mut at = 8 + u32::from_be_bytes(b[4..8].try_into().unwrap()) as usize;
        while at < b.len() {
            assert_eq!(&b[at..at + 4], b"MTrk");
            let len = u32::from_be_bytes(b[at + 4..at + 8].try_into().unwrap()) as usize;
            let d = &b[at + 8..at + 8 + len];
            at += 8 + len;
            let (mut q, mut tick, mut running) = (0, 0, 0);
            let mut name = Vec::new();
            let mut notes = Vec::new();
            while q < d.len() {
                tick += vlq(d, &mut q);
                let mut status = d[q];
                if status >= 128 {
                    q += 1;
                } else {
                    status = running;
                }
                if status == 255 {
                    let kind = d[q];
                    q += 1;
                    let n = vlq(d, &mut q) as usize;
                    if kind == 3 {
                        name = d[q..q + n].to_vec();
                    }
                    q += n;
                } else if status == 240 || status == 247 {
                    let n = vlq(d, &mut q) as usize;
                    q += n;
                } else {
                    running = status;
                    let n = if status >> 4 == 12 || status >> 4 == 13 {
                        1
                    } else {
                        2
                    };
                    if status >> 4 == 9 && d[q + 1] != 0 {
                        notes.push((tick, (status & 15) + 1, d[q], d[q + 1]));
                    }
                    q += n;
                }
            }
            if name == wanted {
                return notes;
            }
        }
        panic!("reference part missing")
    }
}
