//! Descriptor166 terminal correspondence, not terminal semantics or export authority.
use std::ops::Range;

use crate::sequence_container::{
    parse_project_166, FramedRecord, SequenceContainer, TrackAssociations,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TerminalSlot {
    pub descriptor_ordinal: usize,
    pub descriptor_range: Range<usize>,
    pub primary_range: Range<usize>,
    pub primary_header_range: Range<usize>,
    pub primary_header: u16,
    pub word_range: Range<usize>,
    pub word: u16,
    pub project_rank: Option<u16>,
}

/// Privately constructed proof. All bytes remain uninterpreted source provenance.
#[derive(Clone, Debug)]
pub struct TerminalCoverage {
    record_range: Range<usize>,
    payload_range: Range<usize>,
    payload: Vec<u8>,
    record_bytes: Vec<u8>,
    slots: Vec<TerminalSlot>,
}
impl TerminalCoverage {
    pub fn record_range(&self) -> Range<usize> {
        self.record_range.clone()
    }
    pub fn payload_range(&self) -> Range<usize> {
        self.payload_range.clone()
    }
    pub fn record_bytes(&self) -> &[u8] {
        &self.record_bytes
    }
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
    pub fn slots(&self) -> &[TerminalSlot] {
        &self.slots
    }
}

fn primaries<'a, 'b>(s: &'b SequenceContainer<'a>) -> Vec<&'b FramedRecord<'a>> {
    std::iter::once(&s.meter_primary)
        .chain(std::iter::once(&s.tempo_primary))
        .chain(s.track_pairs.iter().map(|p| &p.primary))
        .collect()
}
fn rank_word(rank: usize) -> Result<u16, String> {
    if rank >= 0xfffe {
        return Err("project rank collides with reserved sentinel".into());
    }
    Ok(rank as u16)
}

/// Reparse the contiguous root stream and derive ranks from framed primaries alone.
/// This classifier intentionally does not implement the independent legacy eight-FF path.
pub fn classify_terminal_coverage(
    bytes: &[u8],
    ordinal: usize,
) -> Result<TerminalCoverage, String> {
    let project = parse_project_166(bytes).map_err(|e| format!("{e:?}"))?;
    let selected = project.sequences.get(ordinal).ok_or("missing sequence")?;
    let TrackAssociations::Ordinal(bindings) = &selected.track_associations else {
        return Err("ambiguous descriptor ownership".into());
    };
    if selected.descriptors.len() != selected.track_pairs.len() + 2
        || bindings.len() != selected.track_pairs.len()
        || bindings
            .iter()
            .enumerate()
            .any(|(i, b)| b.descriptor_ordinal != i + 2 || b.pair_ordinal != i)
        || selected
            .descriptors
            .iter()
            .enumerate()
            .any(|(i, d)| d.ordinal != i)
    {
        return Err("nonexhaustive descriptor ownership".into());
    }
    let terminal = &selected.terminal_record;
    let last_end = selected
        .track_pairs
        .last()
        .map_or(selected.tempo_secondary.record_range.end, |p| {
            p.secondary.record_range.end
        });
    if terminal.record_type.value != 0
        || terminal.record_range.end != selected.sequence_range.end
        || terminal.record_range.start != last_end
        || terminal.payload.bytes.len() != 2 * selected.descriptors.len()
    {
        return Err("unsupported terminal framing or slot count".into());
    }
    // Every independently framed root primary must occur exactly once in a
    // parsed sequence, in the same source order. Orphan records cannot silently
    // disappear from the project-wide rank calculation.
    let owned: Vec<_> = project.sequences.iter().flat_map(primaries).collect();
    let framed: Vec<_> = project
        .records
        .iter()
        .filter(|r| r.record_type.value == 2)
        .collect();
    if owned.len() != framed.len()
        || owned
            .iter()
            .zip(&framed)
            .any(|(a, b)| a.record_range != b.record_range)
    {
        return Err("unowned or duplicate project primary".into());
    }
    let mut rank = 0usize;
    let mut slots = Vec::new();
    let mut previous_end = 0;
    for (si, sequence) in project.sequences.iter().enumerate() {
        for (di, primary) in primaries(sequence).iter().enumerate() {
            if primary.record_type.value != 2 || primary.record_range.start < previous_end {
                return Err("ambiguous primary framing or source order".into());
            }
            previous_end = primary.record_range.end;
            let header_bytes = primary
                .payload
                .bytes
                .get(..2)
                .ok_or("missing primary header")?;
            let header = u16::from_be_bytes([header_bytes[0], header_bytes[1]]);
            let project_rank = match header {
                1 => None,
                2 => {
                    let value = rank_word(rank)?;
                    rank += 1;
                    Some(value)
                }
                _ => return Err("unsupported primary header".into()),
            };
            if si != ordinal {
                continue;
            }
            let at = terminal.payload.range.start + 2 * di;
            let word = u16::from_be_bytes([bytes[at], bytes[at + 1]]);
            if word == 0xfffe || word != project_rank.unwrap_or(0xffff) {
                return Err(format!(
                    "terminal slot {di} disagrees with independently framed primary"
                ));
            }
            slots.push(TerminalSlot {
                descriptor_ordinal: di,
                descriptor_range: selected.descriptors[di].range.clone(),
                primary_range: primary.record_range.clone(),
                primary_header_range: primary.payload.range.start..primary.payload.range.start + 2,
                primary_header: header,
                word_range: at..at + 2,
                word,
                project_rank,
            });
        }
    }
    if slots.len() != selected.descriptors.len() {
        return Err("unmatched terminal slot".into());
    }
    Ok(TerminalCoverage {
        record_bytes: bytes[terminal.record_range.clone()].to_vec(),
        record_range: terminal.record_range.clone(),
        payload_range: terminal.payload.range.clone(),
        payload: terminal.payload.bytes.to_vec(),
        slots,
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn ranks_cannot_collide_with_sentinels() {
        assert_eq!(super::rank_word(0xfffd).unwrap(), 0xfffd);
        for rank in [0xfffe, 0xffff, 0x10000] {
            assert!(super::rank_word(rank).is_err());
        }
    }
}
