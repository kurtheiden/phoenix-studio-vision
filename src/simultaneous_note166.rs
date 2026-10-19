//! Source-owned adjacent simultaneous Notes with strictly ordered positive endings.
//! This proof supplies ordering evidence only, never inclusion or export authority.
use crate::{
    bounded_routing::decode_bounded_routing,
    midi_export::{
        adapt_track, ChannelAssignment, ChannelAssignmentProvenance, DecodedExportEvent,
        PatchPolicy, TimingPolicy,
    },
    mixed_event::{walk_bounded_mixed_events, MixedEventBounds, MixedEventItem, MixedEventKind},
    smf::{ChannelMessage, MidiChannel, NoteSourceProvenance, ScheduledEvent},
};

/// No public constructor: ranges and all four MIDI messages come from one source walk.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OwnedSimultaneousNotes {
    events: [ScheduledEvent; 4],
    provenance: [NoteSourceProvenance; 2],
}
impl OwnedSimultaneousNotes {
    pub(crate) fn first_tick(&self) -> u32 {
        self.events[0].absolute_tick
    }
    pub(crate) fn first_ordinal(&self) -> u64 {
        self.events[0].stable_ordinal
    }
    pub(crate) fn binds(
        &self,
        events: &[ScheduledEvent],
        provenance: &[NoteSourceProvenance],
    ) -> bool {
        // Exactly one occurrence of each attack/release ordinal, with exact values.
        if !self.events.iter().all(|expected| {
            let mut found = events
                .iter()
                .filter(|e| e.stable_ordinal == expected.stable_ordinal);
            found.next() == Some(expected) && found.next().is_none()
        }) {
            return false;
        }
        // No additional provenance may claim either owned source representation,
        // including a duplicate range disguised under a different ordinal.
        if provenance.iter().any(|p| {
            self.provenance.iter().any(|owned| {
                p.start_ordinal != owned.start_ordinal
                    && p.source_range.start < owned.source_range.end
                    && owned.source_range.start < p.source_range.end
            })
        }) {
            return false;
        }
        let mut positions = Vec::new();
        for expected in &self.provenance {
            let found: Vec<_> = provenance
                .iter()
                .enumerate()
                .filter(|(_, p)| p.start_ordinal == expected.start_ordinal)
                .collect();
            if found.len() != 1 || found[0].1 != expected {
                return false;
            }
            positions.push(found[0].0);
        }
        positions[0].checked_add(1) == Some(positions[1])
    }
    pub(crate) fn matches_attacks(&self, a: &ScheduledEvent, b: &ScheduledEvent) -> bool {
        self.events[0] == *a && self.events[2] == *b
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SimultaneousNoteRefusal {
    Accounting,
    Routing,
    Walk,
    Adaptation,
}

/// Qualifies only two adjacent ordinary Notes with simultaneous nonzero attacks
/// on the same channel/key; each attack velocity remains exactly source-bound.
/// Both Notes require consecutive logical ordinals and increasing, distinct
/// positive end ticks.
/// Other groups receive no proof and retain the existing serializer refusals.
pub fn qualify_track(
    bytes: &[u8],
    sequence: usize,
    pair: usize,
) -> Result<Vec<OwnedSimultaneousNotes>, SimultaneousNoteRefusal> {
    use SimultaneousNoteRefusal::*;
    crate::controller166::qualify_track(bytes, sequence, pair).map_err(|_| Accounting)?;
    let route = decode_bounded_routing(bytes)
        .map_err(|_| Routing)?
        .into_iter()
        .find(|r| r.structural_ordinal == sequence && r.descriptor_ordinal == pair + 2)
        .ok_or(Routing)?
        .result
        .map_err(|_| Routing)?;
    let walk = walk_bounded_mixed_events(
        bytes,
        MixedEventBounds {
            event_range: route.event_range.clone(),
        },
        Default::default(),
    )
    .map_err(|_| Walk)?;
    if walk.consumed_range != route.event_range {
        return Err(Walk);
    }
    let mut ordinal = 0;
    let mut notes = Vec::new();
    for item in &walk.items {
        if let MixedEventItem::Event(e) = item {
            if let MixedEventKind::Note(n) = &e.event {
                notes.push(DecodedExportEvent::from_note(e.position, ordinal, n));
            }
        }
        ordinal += item.logical_event_count() as u64;
    }
    let adapted = adapt_track(
        &notes,
        Some(ChannelAssignment {
            channel: MidiChannel::new(route.midi_channel).map_err(|_| Routing)?,
            provenance: ChannelAssignmentProvenance::ParsedRouting,
        }),
        TimingPolicy::Identity480,
        PatchPolicy::StrictKnownOnly,
    )
    .map_err(|_| Adaptation)?;
    let mut proofs = Vec::new();
    for (i, group) in adapted
        .scheduled_events
        .chunks_exact(2)
        .collect::<Vec<_>>()
        .windows(2)
        .enumerate()
    {
        let (a, b) = (group[0], group[1]);
        let (pa, pb) = (&adapted.note_provenance[i], &adapted.note_provenance[i + 1]);
        if a[0].absolute_tick == b[0].absolute_tick
            && matches!((&a[0].message, &b[0].message),
                (ChannelMessage::NoteOn { channel: ca, key: ka, attack_velocity: va },
                 ChannelMessage::NoteOn { channel: cb, key: kb, attack_velocity: vb })
                if ca == cb && ka == kb && va.get() > 0 && vb.get() > 0)
            && a[0].stable_ordinal.checked_add(2) == Some(b[0].stable_ordinal)
            && pa.source_range.start < pa.source_range.end
            && pa.source_range.end == pb.source_range.start
            && pb.source_range.start < pb.source_range.end
            && a[1].absolute_tick > a[0].absolute_tick
            && b[1].absolute_tick > a[1].absolute_tick
        {
            proofs.push(OwnedSimultaneousNotes {
                events: [a[0].clone(), a[1].clone(), b[0].clone(), b[1].clone()],
                provenance: [pa.clone(), pb.clone()],
            });
        }
    }
    Ok(proofs)
}
