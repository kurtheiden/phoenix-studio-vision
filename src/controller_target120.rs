//! Controller target lookup for caller-proved, owned Studio Vision 120 tracks.
//! No event decoding, track discovery, export authority or MIDI reinterpretation.
use crate::{
    midi_export::{DecodedExportEvent, DecodedExportEventKind},
    mixed_event::{BoundedMidiController, MixedEventItem, MixedEventKind, MixedEventWalk},
    sequence_container::FramedRecord,
};
use serde::Serialize;
use std::{collections::BTreeMap, ops::Range};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ControllerTargetRefusal {
    Association,
    RoutingTable,
    Instrument,
    Device,
    Channel,
    Selector(u8),
    Handoff,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ControllerTargetProvenance {
    pub source_ordinal: u64,
    pub representation_range: Range<usize>,
    pub position: u32,
    pub encoded_status: u8,
    pub status_offset: usize,
    pub explicit_status: bool,
    pub selector: u8,
    pub controller_number: u8,
    pub controller_value: u8,
    pub controller_number_offset: usize,
    pub controller_value_offset: usize,
    pub association_range: Range<usize>,
    pub list_entry_range: Range<usize>,
    pub instrument_row: u8,
    pub routing_record_range: Range<usize>,
    pub instrument_identity_offset: usize,
    pub device_record_range: Range<usize>,
    pub device_identifier: u8,
    pub channel_offset: usize,
    pub resolved_channel: u8,
}

#[derive(Clone)]
struct InstrumentTarget {
    row: u8,
    record_range: Range<usize>,
    identity_offset: usize,
    device_range: Range<usize>,
    device_identifier: u8,
    channel_offset: usize,
    channel: u8,
}

/// Private lookup construction; callers must separately prove same-source
/// ownership/event bounds. This mirrors existing Single/Multi table guards.
pub(crate) struct ControllerAssociation120 {
    range: Range<usize>,
    slots: Vec<InstrumentTarget>,
}
impl ControllerAssociation120 {
    pub(crate) fn from_records(
        records: &[FramedRecord<'_>],
        range: Range<usize>,
        association: &[u8],
    ) -> Result<Self, ControllerTargetRefusal> {
        use ControllerTargetRefusal as E;
        if range.end.checked_sub(range.start) != Some(33)
            || association.len() != 33
            || !(1..=15).contains(&association[0])
        {
            return Err(E::Association);
        }
        let count = usize::from(association[0]);
        let tail = &association[1 + count * 2..];
        if !tail.iter().all(|b| *b == 255) && !(count == 1 && tail.iter().all(|b| *b == 0)) {
            return Err(E::Association);
        }
        let table: Vec<_> = records
            .iter()
            .filter(|r| r.record_type.value == 16)
            .collect();
        if table.is_empty()
            || table.len() > 99
            || table
                .iter()
                .enumerate()
                .any(|(i, r)| r.payload.bytes.len() != 36 || usize::from(r.payload.bytes[25]) != i)
        {
            return Err(E::RoutingTable);
        }
        let devices: Vec<_> = records
            .iter()
            .filter(|r| r.record_type.value == 42)
            .collect();
        if devices.iter().any(|r| {
            let b = r.payload.bytes;
            b.len() != 40
                || !(1..=32).contains(&b[0])
                || b[1..1 + usize::from(b[0])]
                    .iter()
                    .all(u8::is_ascii_whitespace)
        }) {
            return Err(E::Device);
        }
        let mut unique = std::collections::BTreeSet::new();
        let mut slots = Vec::new();
        for entry in association[1..1 + 2 * count].chunks_exact(2) {
            let row = u16::from_be_bytes([entry[0], entry[1]]);
            if row == 0 || usize::from(row) >= table.len() || !unique.insert(row) {
                return Err(E::Instrument);
            }
            let selected = table[usize::from(row)];
            let b = selected.payload.bytes;
            if b[27] > 15 {
                return Err(E::Channel);
            }
            let matches: Vec<_> = devices
                .iter()
                .filter(|r| r.payload.bytes[33] == b[26])
                .collect();
            let [device] = matches.as_slice() else {
                return Err(E::Device);
            };
            slots.push(InstrumentTarget {
                row: row as u8,
                record_range: selected.record_range.clone(),
                identity_offset: selected.payload.range.start + 25,
                device_range: device.record_range.clone(),
                device_identifier: b[26],
                channel_offset: selected.payload.range.start + 27,
                channel: b[27] + 1,
            });
        }
        Ok(Self { range, slots })
    }

    pub(crate) fn resolve(
        &self,
        c: &BoundedMidiController<'_>,
        position: u32,
        ordinal: u64,
    ) -> Result<ControllerTargetProvenance, ControllerTargetRefusal> {
        if !(0xb0..=0xbf).contains(&c.status.value) || c.channel != (c.status.value & 15) + 1 {
            return Err(ControllerTargetRefusal::Handoff);
        }
        let selector = c.status.value & 15;
        let target = self
            .slots
            .get(usize::from(selector))
            .ok_or(ControllerTargetRefusal::Selector(selector))?;
        let start = self.range.start + 1 + usize::from(selector) * 2;
        Ok(ControllerTargetProvenance {
            source_ordinal: ordinal,
            representation_range: c.representation_range.clone(),
            position,
            encoded_status: c.status.value,
            status_offset: c.status.offset,
            explicit_status: c.explicit_status,
            selector,
            controller_number: c.controller_number.value,
            controller_value: c.controller_value.value,
            controller_number_offset: c.controller_number.offset,
            controller_value_offset: c.controller_value.offset,
            association_range: self.range.clone(),
            list_entry_range: start..start + 2,
            instrument_row: target.row,
            routing_record_range: target.record_range.clone(),
            instrument_identity_offset: target.identity_offset,
            device_record_range: target.device_range.clone(),
            device_identifier: target.device_identifier,
            channel_offset: target.channel_offset,
            resolved_channel: target.channel,
        })
    }

    pub(crate) fn collect(
        &self,
        walk: &MixedEventWalk<'_>,
        first_ordinal: u64,
    ) -> Result<Vec<ControllerTargetProvenance>, ControllerTargetRefusal> {
        let mut targets = Vec::new();
        let mut ordinal = first_ordinal;
        for item in &walk.items {
            if let MixedEventItem::Event(e) = item {
                if let MixedEventKind::MidiController(c) = &e.event {
                    targets.push(self.resolve(c, e.position, ordinal)?);
                }
            }
            ordinal = ordinal
                .checked_add(item.logical_event_count() as u64)
                .ok_or(ControllerTargetRefusal::Handoff)?;
        }
        Ok(targets)
    }
}

/// Clone the raw handoff and change only proven Controller destinations. All
/// bindings must match; missing, duplicate or surplus provenance refuses before
/// any adapted handoff is returned. Generic MIDI adaptation is unchanged.
pub(crate) fn apply_targets(
    raw: &[DecodedExportEvent],
    targets: &[ControllerTargetProvenance],
) -> Result<Vec<DecodedExportEvent>, ControllerTargetRefusal> {
    let mut by_ordinal = BTreeMap::new();
    for p in targets {
        if by_ordinal.insert(p.source_ordinal, p).is_some() {
            return Err(ControllerTargetRefusal::Handoff);
        }
    }
    let mut effective = raw.to_vec();
    for e in &mut effective {
        if let DecodedExportEventKind::MidiController {
            channel,
            number,
            value,
        } = &mut e.kind
        {
            let p = by_ordinal
                .remove(&e.source_ordinal)
                .ok_or(ControllerTargetRefusal::Handoff)?;
            if e.source_range.as_ref() != Some(&p.representation_range)
                || e.absolute_position != p.position
                || *number != p.controller_number
                || *value != p.controller_value
                || *channel != p.selector + 1
                || !(1..=16).contains(&p.resolved_channel)
            {
                return Err(ControllerTargetRefusal::Handoff);
            }
            *channel = p.resolved_channel;
        }
    }
    if !by_ordinal.is_empty() {
        return Err(ControllerTargetRefusal::Handoff);
    }
    Ok(effective)
}

#[cfg(test)]
#[path = "controller_target120_tests.rs"]
mod tests;
