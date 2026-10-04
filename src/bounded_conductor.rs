//! Complete compact initial conductor coverage, without readiness/export authority.
//!
//! Callers establish same-source sequence ownership of the four supplied record
//! ranges. No scanning, sequence discovery, opaque-context translation or defaults.
use std::ops::Range;

use crate::{
    meter::{decode_bounded_initial_meter, InitialMeterBounds, InitialMeterEvent},
    midi_export::{adapt_conductor, ConductorResult, MeterPolicy, TimingPolicy},
    sequence_container::parse_root_record_stream,
    tempo::{decode_bounded_initial_tempo, InitialTempoBounds, InitialTempoEvent},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConductorRefusal {
    Framing,
    RecordBounds,
    Envelope,
    Event,
    Key,
    Adaptation,
}

/// Validated key annotation is retained independently of musical timing.
#[derive(Clone, Debug)]
pub struct InitialKey {
    pub source_range: Range<usize>,
    pub accidentals: i8,
    pub minor: bool,
}

#[derive(Clone, Debug)]
pub struct CompactConductor {
    tempo: InitialTempoEvent,
    meter: InitialMeterEvent,
    key: InitialKey,
    records: [Range<usize>; 4],
    opaque_context: [Vec<u8>; 2],
}
impl CompactConductor {
    pub fn tempo(&self) -> &InitialTempoEvent {
        &self.tempo
    }
    pub fn meter(&self) -> &InitialMeterEvent {
        &self.meter
    }
    pub fn key(&self) -> &InitialKey {
        &self.key
    }
    pub fn record_ranges(&self) -> &[Range<usize>; 4] {
        &self.records
    }
    pub fn opaque_context(&self) -> &[Vec<u8>; 2] {
        &self.opaque_context
    }
    pub fn adapt(&self, name: &[u8]) -> Result<ConductorResult, ConductorRefusal> {
        let m = &self.meter;
        let result = adapt_conductor(
            name,
            self.tempo.mpqn(),
            (
                m.numerator.value,
                m.denominator_exponent.value,
                m.third_payload.value,
                m.fourth_payload.value,
            ),
            TimingPolicy::Identity480,
            MeterPolicy::KnownHistoricalOnly,
        )
        .map_err(|_| ConductorRefusal::Adaptation)?;
        if !result.warnings.is_empty() || m.numerator.value == 0 || m.denominator().is_none() {
            return Err(ConductorRefusal::Adaptation);
        }
        Ok(result)
    }
}

/// Ranges are Meter primary/secondary, Tempo primary/secondary in source order.
/// Each is reparsed from this source; callers must also establish that these are
/// the complete conductor pairs of the selected sequence, with no extra prelude.
pub fn decode_compact_conductor(
    bytes: &[u8],
    ranges: [Range<usize>; 4],
) -> Result<CompactConductor, ConductorRefusal> {
    let root = parse_root_record_stream(bytes).map_err(|_| ConductorRefusal::Framing)?;
    let mut records = Vec::new();
    for (i, range) in ranges.iter().enumerate() {
        let r = root
            .records
            .iter()
            .find(|r| r.record_range == *range)
            .ok_or(ConductorRefusal::RecordBounds)?;
        if r.record_type.value != if i % 2 == 0 { 0x02 } else { 0x29 }
            || (i > 0 && ranges[i - 1].end != range.start)
        {
            return Err(ConductorRefusal::RecordBounds);
        }
        records.push(r);
    }
    let meter = records[0];
    let tempo = records[2];
    check_pair(meter.payload.bytes, records[1].payload.bytes, 2, 14, 20, 8)?;
    check_pair(tempo.payload.bytes, records[3].payload.bytes, 1, 7, 14, 7)?;
    let p = meter.payload.bytes;
    if p[14..18] != [0, 0xff, 0x59, 2] {
        return Err(ConductorRefusal::Key);
    }
    let accidentals = p[18] as i8;
    if !(-7..=7).contains(&accidentals) || p[19] > 1 {
        return Err(ConductorRefusal::Key);
    }
    let ms = meter.payload.range.start;
    let ts = tempo.payload.range.start;
    let result = CompactConductor {
        meter: decode_bounded_initial_meter(
            bytes,
            InitialMeterBounds {
                event_range: ms + 20..ms + 28,
            },
        )
        .map_err(|_| ConductorRefusal::Event)?,
        tempo: decode_bounded_initial_tempo(
            bytes,
            InitialTempoBounds {
                event_range: ts + 14..ts + 21,
            },
        )
        .map_err(|_| ConductorRefusal::Event)?,
        key: InitialKey {
            source_range: ms + 14..ms + 20,
            accidentals,
            minor: p[19] == 1,
        },
        records: ranges,
        opaque_context: [p[10..14].to_vec(), tempo.payload.bytes[10..14].to_vec()],
    };
    result.adapt(b"")?;
    Ok(result)
}
fn check_pair(
    p: &[u8],
    s: &[u8],
    count: u32,
    event_bytes: usize,
    final_start: usize,
    final_len: usize,
) -> Result<(), ConductorRefusal> {
    if p.len() != 14 + event_bytes + 7
        || p[..2] != [0, 1]
        || p[2..6] != count.to_be_bytes()
        || p[6..10] != [0; 4]
        || p[14 + event_bytes..] != [0x87, 0xff, 0xff, 0x7f, 0xff, 0x2f, 0]
    {
        return Err(ConductorRefusal::Envelope);
    }
    let event = &p[final_start..final_start + final_len];
    let mut copy = vec![event[2]];
    copy.extend_from_slice(&event[4..]);
    let mut expected = vec![0, 1, 0, 0, 0, (24 + copy.len() - 6) as u8];
    expected.extend(count.to_be_bytes());
    expected.extend(((event_bytes + 4) as u32).to_be_bytes());
    expected.extend([0, 0xff, 0xff, 0xff, 0x2f, 0xff, 0, 0, 0, 0]);
    expected.extend(copy);
    if s != expected {
        return Err(ConductorRefusal::Envelope);
    }
    Ok(())
}
