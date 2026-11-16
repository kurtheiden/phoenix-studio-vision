//! Complete bounded initial conductor coverage, without readiness/export authority.
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

// Synthetic parity only: no production caller or export authority.
#[cfg(test)]
#[path = "initial_conductor_parity_tests.rs"]
mod parity_tests;

/// Bounded failure context only; neither record nor field is qualified by this value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ConductorDiagnostic {
    pub predicate: &'static str,
    pub role: &'static str,
    pub record_range: Range<usize>,
    pub field_range: Range<usize>,
}
impl ConductorDiagnostic {
    pub(crate) fn new(
        record: &crate::sequence_container::FramedRecord<'_>,
        role: &'static str,
        predicate: &'static str,
        relative: Range<usize>,
    ) -> Self {
        let start = record.payload.range.start;
        Self {
            predicate,
            role,
            record_range: record.record_range.clone(),
            field_range: start + relative.start.min(record.payload.bytes.len())
                ..start + relative.end.min(record.payload.bytes.len()),
        }
    }
    pub(crate) fn describe(&self) -> String {
        format!("predicate={}, role={}, record={:?}, field={:?}; candidate only; applicability unresolved", self.predicate, self.role, self.record_range, self.field_range)
    }
}

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

/// Bounded non-event envelope bytes, retained without assigning semantics.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConductorEnvelopeContext {
    pub source_range: Range<usize>,
    pub source_bytes: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct CompactConductor {
    tempo: InitialTempoEvent,
    meter: InitialMeterEvent,
    key: Option<InitialKey>,
    records: [Range<usize>; 4],
    opaque_context: [Vec<u8>; 2],
    envelope_context: Vec<ConductorEnvelopeContext>,
}
impl CompactConductor {
    pub fn tempo(&self) -> &InitialTempoEvent {
        &self.tempo
    }
    pub fn meter(&self) -> &InitialMeterEvent {
        &self.meter
    }
    /// No Key event is synthesized for the Meter-only initial form.
    pub fn key(&self) -> Option<&InitialKey> {
        self.key.as_ref()
    }
    pub fn record_ranges(&self) -> &[Range<usize>; 4] {
        &self.records
    }
    pub fn opaque_context(&self) -> &[Vec<u8>; 2] {
        &self.opaque_context
    }
    pub fn envelope_context(&self) -> &[ConductorEnvelopeContext] {
        &self.envelope_context
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
    decode_conductor_impl(bytes, ranges, false, &mut None)
}

pub(crate) fn decode_reference_qualified_conductor(
    bytes: &[u8],
    ranges: [Range<usize>; 4],
    qualification: &crate::compatibility_profiles::NativeReference120Qualification<'_>,
) -> Result<CompactConductor, ConductorRefusal> {
    if !qualification.conductor_matches(bytes, &ranges) {
        return Err(ConductorRefusal::RecordBounds);
    }
    decode_conductor_impl(bytes, ranges, true, &mut None)
}

pub(crate) fn decode_compact_conductor_diagnostic(
    bytes: &[u8],
    ranges: [Range<usize>; 4],
) -> Result<CompactConductor, (ConductorRefusal, Option<ConductorDiagnostic>)> {
    let mut diagnostic = None;
    decode_conductor_impl(bytes, ranges, false, &mut diagnostic).map_err(|e| (e, diagnostic))
}

fn decode_conductor_impl(
    bytes: &[u8],
    ranges: [Range<usize>; 4],
    reference_header02: bool,
    diagnostic: &mut Option<ConductorDiagnostic>,
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
    let p = meter.payload.bytes;
    // Two complete layouts, selected by exact payload extent, never by scanning
    // for a recognizable event or by accepting a prefix of a larger map.
    let with_key = match p.len() {
        35 => true,
        38 => false,
        _ => {
            *diagnostic = Some(ConductorDiagnostic::new(
                meter,
                "Meter primary",
                "payload extent",
                0..p.len(),
            ));
            return Err(ConductorRefusal::Envelope);
        }
    };
    let meter_start = if with_key { 20 } else { 14 };
    let meter_event_bytes = if with_key { 14 } else { 8 };
    check_pair(
        p,
        records[1].payload.bytes,
        if with_key { 2 } else { 1 },
        meter_event_bytes,
        meter_start,
        8,
        PairEnvelope {
            padded: !with_key,
            reference_header02,
        },
    )
    .map_err(|(secondary, predicate, range)| {
        let (record, role) = if secondary {
            (records[1], "Meter secondary")
        } else {
            (meter, "Meter primary")
        };
        *diagnostic = Some(ConductorDiagnostic::new(record, role, predicate, range));
        ConductorRefusal::Envelope
    })?;
    check_pair(
        tempo.payload.bytes,
        records[3].payload.bytes,
        1,
        7,
        14,
        7,
        PairEnvelope {
            padded: !with_key,
            reference_header02,
        },
    )
    .map_err(|(secondary, predicate, range)| {
        let (record, role) = if secondary {
            (records[3], "Tempo secondary")
        } else {
            (tempo, "Tempo primary")
        };
        *diagnostic = Some(ConductorDiagnostic::new(record, role, predicate, range));
        ConductorRefusal::Envelope
    })?;
    let ms = meter.payload.range.start;
    let ts = tempo.payload.range.start;
    let key = if with_key {
        if p[14..18] != [0, 0xff, 0x59, 2] {
            return Err(ConductorRefusal::Key);
        }
        let accidentals = p[18] as i8;
        if !(-7..=7).contains(&accidentals) || p[19] > 1 {
            return Err(ConductorRefusal::Key);
        }
        Some(InitialKey {
            source_range: ms + 14..ms + 20,
            accidentals,
            minor: p[19] == 1,
        })
    } else {
        None
    };
    let mut envelope_context = Vec::new();
    for (primary, secondary, event_bytes) in [
        (records[0], records[1], meter_event_bytes),
        (records[2], records[3], 7),
    ] {
        let start = primary.payload.range.start;
        let secondary_start = secondary.payload.range.start;
        for range in [
            start + 1..start + 2,
            start + 10..start + 14,
            secondary_start + 19..secondary_start + 20,
            start + 14 + event_bytes + 7..primary.payload.range.end,
        ] {
            if !range.is_empty() {
                envelope_context.push(ConductorEnvelopeContext {
                    source_bytes: bytes[range.clone()].to_vec(),
                    source_range: range,
                });
            }
        }
    }
    let result = CompactConductor {
        meter: decode_bounded_initial_meter(
            bytes,
            InitialMeterBounds {
                event_range: ms + meter_start..ms + meter_start + 8,
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
        key,
        records: ranges,
        opaque_context: [p[10..14].to_vec(), tempo.payload.bytes[10..14].to_vec()],
        envelope_context,
    };
    result.adapt(b"")?;
    Ok(result)
}
struct PairEnvelope {
    padded: bool,
    reference_header02: bool,
}

fn check_pair(
    p: &[u8],
    s: &[u8],
    count: u32,
    event_bytes: usize,
    final_start: usize,
    final_len: usize,
    envelope: PairEnvelope,
) -> Result<(), (bool, &'static str, Range<usize>)> {
    let padded = envelope.padded;
    let tail_start = 14 + event_bytes;
    let tail_end = tail_start + 7;
    if p.len() != if padded { 38 } else { tail_end } {
        return Err((false, "payload extent", 0..p.len()));
    }
    if p[0] != 0 {
        return Err((false, "header byte 0", 0..1));
    }
    if !(p[1] == 1
        || (!padded && count == 2 && p[1] == 3)
        || (padded && envelope.reference_header02 && p[1] == 2))
    {
        return Err((false, "header byte 1", 1..2));
    }
    if p[2..6] != count.to_be_bytes() {
        return Err((false, "declared count", 2..6));
    }
    if p[6..10] != [0; 4] {
        return Err((false, "zero prefix", 6..10));
    }
    if p[tail_start..tail_end] != [0x87, 0xff, 0xff, 0x7f, 0xff, 0x2f, 0] {
        return Err((false, "terminal marker", tail_start..tail_end));
    }
    if p[tail_end..].iter().any(|b| *b != 0) {
        return Err((false, "zero padding", tail_end..p.len()));
    }
    let event = &p[final_start..final_start + final_len];
    let mut copy = vec![event[2]];
    copy.extend_from_slice(&event[4..]);
    let mut expected = vec![0, 1, 0, 0, 0, (24 + copy.len() - 6) as u8];
    expected.extend(count.to_be_bytes());
    expected.extend(((event_bytes + 4) as u32).to_be_bytes());
    expected.extend([0, 0xff, 0xff, 0xff, 0x2f, 0xff, 0, 0, 0, 0]);
    expected.extend(copy);
    // One bounded context byte between the fixed 2f marker and four zero bytes
    // varies independently of the complete event/value copy. It is not an
    // event region, length, count, position or a default MIDI value.
    if s.len() != expected.len() {
        return Err((true, "secondary extent", 0..s.len()));
    }
    if s[..19] != expected[..19] {
        return Err((true, "secondary framing/count/length", 0..19));
    }
    if s[20..] != expected[20..] {
        return Err((true, "secondary value copy/zero fields", 20..s.len()));
    }
    Ok(())
}
