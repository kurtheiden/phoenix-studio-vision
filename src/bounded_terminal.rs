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
    pub fn classification(&self) -> &'static str {
        "UNINTERPRETED NON-EVENT TERMINAL MATERIAL"
    }
    pub fn message(&self) -> &'static str {
        "Uninterpreted terminal material remains outside recovered MIDI event data."
    }
}

/// Assess the evidenced sequence/type07/paired-record/type00 neighborhood.
/// `recovered_records` must be the complete conductor and ordinary paired-record
/// ranges in source order, supplied only after their musical gates pass.
/// Complete event decoding and resolved count/ownership coverage are caller
/// prerequisites; a byte-family match never substitutes for those gates.
/// Nonempty all-ff terminals retain existing behavior. Diagnostic forms are
/// ff+ fe ff*, exactly two zero bytes followed by ff+ fe ff+, or the unprefixed
/// repeated form ff+ (fe ff+){2,}. Only the previous single-fe unprefixed form
/// may terminate with fe. These are whole-payload
/// predicates, not permission to ignore an arbitrary post-event suffix.
/// This does not assign any semantic meaning to those bytes.
pub fn assess_terminal_coverage(
    bytes: &[u8],
    sequence_range: Range<usize>,
    recovered_records: &[Range<usize>],
) -> Result<Option<TerminalDiagnostic>, TerminalRefusal> {
    assess_terminal_impl(bytes, sequence_range, recovered_records, None)
}

pub(crate) fn assess_reference_qualified_terminal(
    bytes: &[u8],
    sequence_range: Range<usize>,
    recovered_records: &[Range<usize>],
    qualification: &crate::compatibility_profiles::NativeReference120Qualification<'_>,
) -> Result<Option<TerminalDiagnostic>, TerminalRefusal> {
    assess_terminal_impl(
        bytes,
        sequence_range,
        recovered_records,
        Some(qualification),
    )
}

fn assess_terminal_impl(
    bytes: &[u8],
    sequence_range: Range<usize>,
    recovered_records: &[Range<usize>],
    qualification: Option<&crate::compatibility_profiles::NativeReference120Qualification<'_>>,
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
    let (prefixed, body) = match payload.strip_prefix(&[0, 0]) {
        Some(body) => (true, body),
        None => (false, payload),
    };
    let deviation = body.iter().position(|v| *v != 0xff);
    let single = deviation.is_some_and(|i| {
        i > 0
            && body[i] == 0xfe
            && (!prefixed || i + 1 < body.len())
            && body[i + 1..].iter().all(|v| *v == 0xff)
    });
    // Each repeated marker must be isolated between nonempty ff runs. This
    // extension is unprefixed only, with no default or semantics for either byte.
    let repeated = !prefixed
        && payload.first() == Some(&0xff)
        && payload.last() == Some(&0xff)
        && payload.iter().filter(|v| **v == 0xfe).count() >= 2
        && payload.iter().all(|v| matches!(v, 0xff | 0xfe))
        && !payload.windows(2).any(|pair| pair == [0xfe, 0xfe]);
    let reference_qualified = qualification.is_some_and(|q| {
        q.terminal_matches(bytes, &sequence_range, &terminal.record_range, payload)
    });
    if !single && !repeated && !reference_qualified {
        return Err(TerminalRefusal::UnsupportedMaterial);
    }
    Ok(Some(TerminalDiagnostic {
        record_range: terminal.record_range.clone(),
        payload_range: terminal.payload.range.clone(),
        source_bytes: payload.to_vec(),
    }))
}
