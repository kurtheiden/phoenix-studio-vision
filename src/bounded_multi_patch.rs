//! Bounded standalone Multi Patch recovery, independent of export authority.
//!
//! Callers supply the established association and event bounds for ONE ordinary
//! track. All fields and routing records are reparsed from that same source.
//! This does not discover tracks, select mute policy, or confer readiness.
use crate::midi_export::{
    adapt_track, ChannelAssignment, ChannelAssignmentProvenance, DecodedExportEvent,
    DecodedExportEventKind, ExportTrackResult, MidiExportError, PatchPolicy, TimingPolicy,
};
use crate::mixed_event::{walk_bounded_mixed_events, MixedEventBounds, MixedEventItem};
use crate::sequence_container::parse_root_record_stream;
use crate::smf::MidiChannel;
use std::ops::Range;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MultiPatchRefusal {
    Framing,
    BoundsOrOwnership,
    Association,
    Selector,
    RoutingTable,
    RoutingIdentity,
    Channel,
    Device,
    EventWalk,
    UnsupportedEvent,
    UnsupportedTranslation,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedMultiPatch {
    pub position: u32,
    pub source_ordinal: u64,
    pub representation_range: Range<usize>,
    pub selector: u8,
    pub association_range: Range<usize>,
    pub list_entry_range: Range<usize>,
    pub instrument_row: u8,
    pub routing_record_range: Range<usize>,
    pub device_record_range: Range<usize>,
    pub device_identifier: u8,
    pub device_name: Vec<u8>,
    pub midi_channel: u8,
    pub program: u8,
    pub patch_name: Vec<u8>,
    pub pre_name_context: Vec<u8>,
    pub post_name_context: Vec<u8>,
}
impl ResolvedMultiPatch {
    /// Program-only classification has already passed the bounded predicates.
    pub fn export_event(&self) -> DecodedExportEvent {
        DecodedExportEvent {
            absolute_position: self.position,
            source_ordinal: self.source_ordinal,
            source_range: Some(self.representation_range.clone()),
            kind: DecodedExportEventKind::TargetedProgram {
                channel: self.midi_channel,
                program: self.program,
            },
        }
    }
}

/// A transactionally validated Patch-only track. Fields cannot be constructed
/// externally to bypass the source/association/translation checks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundedMultiPatchTrack {
    patches: Vec<ResolvedMultiPatch>,
}
impl BoundedMultiPatchTrack {
    pub fn patches(&self) -> &[ResolvedMultiPatch] {
        &self.patches
    }
    pub fn export_events(&self) -> Vec<DecodedExportEvent> {
        self.patches
            .iter()
            .map(ResolvedMultiPatch::export_event)
            .collect()
    }
    /// Uses the common adapter. Its legacy track summary assignment represents
    /// the first resolved target, not a common route or fallback for other events.
    /// Every TargetedProgram supplies its own independently validated channel.
    pub fn adapt(&self) -> Result<ExportTrackResult, MidiExportError> {
        let first = &self.patches[0]; // construction requires a nonempty track
        adapt_track(
            &self.export_events(),
            Some(ChannelAssignment {
                channel: MidiChannel::new(first.midi_channel).expect("validated channel"),
                provenance: ChannelAssignmentProvenance::ParsedRouting,
            }),
            TimingPolicy::Identity480,
            PatchPolicy::StrictKnownOnly,
        )
    }
}

/// Recover only the established counted 33-byte list with an ff-filled unused
/// tail, agreeing instrument-row IDs, and complete standalone Program-only cores.
/// Caller bounds must already establish which descriptor/list owns this track;
/// this additional framing check also refuses cross-sequence associations.
pub fn decode_bounded_multi_patch_track(
    source: &[u8],
    association_range: Range<usize>,
    event_range: Range<usize>,
) -> Result<BoundedMultiPatchTrack, MultiPatchRefusal> {
    let root = parse_root_record_stream(source).map_err(|_| MultiPatchRefusal::Framing)?;
    let assoc = source
        .get(association_range.clone())
        .ok_or(MultiPatchRefusal::Association)?;
    if assoc.len() != 33 || !(2..=15).contains(&assoc[0]) {
        return Err(MultiPatchRefusal::Association);
    }
    let count = usize::from(assoc[0]);
    if !assoc[1 + 2 * count..].iter().all(|b| *b == 0xff) {
        return Err(MultiPatchRefusal::Association);
    }
    let owner = root
        .records
        .iter()
        .position(|r| {
            r.record_type.value == 1
                && r.payload.range.start <= association_range.start
                && association_range.end <= r.payload.range.end
        })
        .ok_or(MultiPatchRefusal::BoundsOrOwnership)?;
    let primary = root
        .records
        .iter()
        .position(|r| {
            r.record_type.value == 2
                && r.payload.range.start + 14 == event_range.start
                && event_range.start < event_range.end
                && event_range.end <= r.payload.range.end
        })
        .ok_or(MultiPatchRefusal::BoundsOrOwnership)?;
    if primary <= owner
        || root.records[owner + 1..=primary]
            .iter()
            .any(|r| r.record_type.value == 1)
    {
        return Err(MultiPatchRefusal::BoundsOrOwnership);
    }
    let table: Vec<_> = root
        .records
        .iter()
        .filter(|r| r.record_type.value == 0x10)
        .collect();
    if table.is_empty() || table.len() > 99 || table.iter().any(|r| r.payload.bytes.len() != 36) {
        return Err(MultiPatchRefusal::RoutingTable);
    }
    if table
        .iter()
        .enumerate()
        .any(|(i, r)| usize::from(r.payload.bytes[25]) != i)
    {
        return Err(MultiPatchRefusal::RoutingIdentity);
    }
    let entries: Vec<_> = assoc[1..1 + count * 2]
        .chunks_exact(2)
        .map(|b| u16::from_be_bytes([b[0], b[1]]))
        .collect();
    let mut unique = std::collections::BTreeSet::new();
    if entries
        .iter()
        .any(|i| *i == 0 || usize::from(*i) >= table.len() || !unique.insert(*i))
    {
        return Err(MultiPatchRefusal::RoutingIdentity);
    }
    let devices: Vec<_> = root
        .records
        .iter()
        .filter(|r| r.record_type.value == 0x2a)
        .collect();
    if devices.iter().any(|r| {
        let b = r.payload.bytes;
        b.len() != 40
            || !(1..=32).contains(&b[0])
            || b[1..1 + usize::from(b[0])]
                .iter()
                .all(u8::is_ascii_whitespace)
    }) {
        return Err(MultiPatchRefusal::Device);
    }
    let walk = walk_bounded_mixed_events(
        source,
        MixedEventBounds {
            event_range: event_range.clone(),
        },
        Default::default(),
    )
    .map_err(|_| MultiPatchRefusal::EventWalk)?;
    if walk.consumed_range != event_range || walk.items.is_empty() {
        return Err(MultiPatchRefusal::EventWalk);
    }
    let mut patches = Vec::new();
    for (ordinal, item) in walk.items.iter().enumerate() {
        let MixedEventItem::Patch(event) = item else {
            return Err(MultiPatchRefusal::UnsupportedEvent);
        };
        let core = &event.patch;
        let p = core.program_change.value;
        let pre = core.pre_name_context.bytes;
        let post = core.post_name_context.bytes;
        if p > 127
            || pre.len() != 5
            || pre[1] != 0
            || pre[2] != p
            || pre[4] != p
            || post.len() < 7
            || post[0] as usize + 5 != post.len()
            || post[post.len() - 4..] != [4, 0xff, 0xff, 0xff]
        {
            return Err(MultiPatchRefusal::UnsupportedTranslation);
        }
        let label = &post[1..post.len() - 4];
        let g = pre[3] == 0xf8 && label == format!("G{p}").as_bytes();
        let i = pre[3] == 0
            && label.len() == 3
            && label[0] == b'I'
            && label[1..].iter().all(u8::is_ascii_digit);
        if !g && !i {
            return Err(MultiPatchRefusal::UnsupportedTranslation);
        }
        let selector = pre[0];
        let row = *entries
            .get(usize::from(selector))
            .ok_or(MultiPatchRefusal::Selector)?;
        let routing = table[usize::from(row)];
        let b = routing.payload.bytes;
        if b[27] > 15 {
            return Err(MultiPatchRefusal::Channel);
        }
        let matches: Vec<_> = devices
            .iter()
            .filter(|r| r.payload.bytes[33] == b[26])
            .collect();
        let [device] = matches.as_slice() else {
            return Err(MultiPatchRefusal::Device);
        };
        let db = device.payload.bytes;
        let entry_start = association_range.start + 1 + usize::from(selector) * 2;
        patches.push(ResolvedMultiPatch {
            position: event.position,
            source_ordinal: ordinal as u64,
            representation_range: core.representation_range.clone(),
            selector,
            association_range: association_range.clone(),
            list_entry_range: entry_start..entry_start + 2,
            instrument_row: row as u8,
            routing_record_range: routing.record_range.clone(),
            device_record_range: device.record_range.clone(),
            device_identifier: b[26],
            device_name: db[1..1 + usize::from(db[0])].to_vec(),
            midi_channel: b[27] + 1,
            program: p,
            patch_name: core.name.bytes.to_vec(),
            pre_name_context: pre.to_vec(),
            post_name_context: post.to_vec(),
        });
    }
    Ok(BoundedMultiPatchTrack { patches })
}
