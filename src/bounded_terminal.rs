//! Bounded non-event terminal coverage; never readiness or export authority.
//!
//! Callers must establish complete correspondence and musical recovery of the
//! supplied paired records. This check proves their exhaustive framed coverage,
//! not their event semantics. Descriptor166 manifest policy is unchanged.
use std::ops::Range;

use crate::sequence_container::parse_root_record_stream;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TerminalRefusal {
    Framing,
    SequenceBounds,
    Neighborhood,
    IncompleteCoverage,
    UnsupportedMaterial,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TerminalDiagnostic {
    pub record_range: Range<usize>,
    pub payload_range: Range<usize>,
    pub source_bytes: Vec<u8>,
}
impl TerminalDiagnostic {
    pub fn message(&self) -> &'static str {
        "Uninterpreted terminal material remains outside recovered MIDI event data."
    }
}

/// Assess the evidenced sequence/type07/paired-record/type00 neighborhood.
/// `recovered_records` must be the complete conductor and ordinary paired-record
/// ranges in source order, supplied only after their musical gates pass.
/// Nonempty all-ff terminals retain existing behavior. An ff-filled terminal
/// with one internal fe and trailing ff is accepted with retained provenance.
/// This does not assign any semantic meaning to those bytes.
pub fn assess_terminal_coverage(
    bytes: &[u8],
    sequence_range: Range<usize>,
    recovered_records: &[Range<usize>],
) -> Result<Option<TerminalDiagnostic>, TerminalRefusal> {
    let root = parse_root_record_stream(bytes).map_err(|_| TerminalRefusal::Framing)?;
    let first = root
        .records
        .iter()
        .position(|r| r.record_range.start == sequence_range.start)
        .ok_or(TerminalRefusal::SequenceBounds)?;
    let last = root
        .records
        .iter()
        .position(|r| r.record_range.end == sequence_range.end)
        .ok_or(TerminalRefusal::SequenceBounds)?;
    let records = root
        .records
        .get(first..=last)
        .ok_or(TerminalRefusal::SequenceBounds)?;
    if records.len() < 7
        || records[0].record_type.value != 1
        || records[1].record_type.value != 7
        || records.last().unwrap().record_type.value != 0
        || records
            .windows(2)
            .any(|r| r[0].record_range.end != r[1].record_range.start)
    {
        return Err(TerminalRefusal::Neighborhood);
    }
    let pairs = &records[2..records.len() - 1];
    if pairs.len() % 2 != 0
        || pairs
            .chunks_exact(2)
            .any(|p| p[0].record_type.value != 2 || p[1].record_type.value != 0x29)
    {
        return Err(TerminalRefusal::Neighborhood);
    }
    if pairs
        .iter()
        .map(|r| &r.record_range)
        .ne(recovered_records.iter())
    {
        return Err(TerminalRefusal::IncompleteCoverage);
    }
    let terminal = records.last().unwrap();
    let payload = terminal.payload.bytes;
    if !payload.is_empty() && payload.iter().all(|v| *v == 0xff) {
        return Ok(None);
    }
    let deviations: Vec<_> = payload
        .iter()
        .enumerate()
        .filter(|(_, v)| **v != 0xff)
        .collect();
    if !matches!(deviations.as_slice(), [(i, v)] if **v == 0xfe && *i > 0 && *i + 1 < payload.len())
    {
        return Err(TerminalRefusal::UnsupportedMaterial);
    }
    Ok(Some(TerminalDiagnostic {
        record_range: terminal.record_range.clone(),
        payload_range: terminal.payload.range.clone(),
        source_bytes: payload.to_vec(),
    }))
}
