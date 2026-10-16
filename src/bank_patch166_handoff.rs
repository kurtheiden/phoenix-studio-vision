//! Owned handoff for CCA-071-qualified Patch-to-Note items; no new Patch rules.
use crate::{
    bounded_patch_translation::{qualify_descriptor166_bank_patch, BoundedBankPatch},
    bounded_routing::BoundedRoutingResolution,
    midi_export::{DecodedExportEvent, DecodedExportEventKind},
    mixed_event::{walk_bounded_mixed_events, MixedEventBounds, MixedEventItem},
    sequence_container::parse_project_166,
};
use std::ops::Range;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EmbeddedContext {
    pub range: Range<usize>,
    pub bytes: Vec<u8>,
    pub payload_range: Range<usize>,
    pub payload: Vec<u8>,
    pub final_timing_range: Range<usize>,
    pub final_interval: u32,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BankPatchProvenance {
    pub sequence_ordinal: usize,
    pub descriptor_ordinal: usize,
    pub pair_ordinal: usize,
    pub primary_range: Range<usize>,
    pub secondary_range: Range<usize>,
    pub event_bounds: Range<usize>,
    pub declared_source_objects: u32,
    pub composition_range: Range<usize>,
    pub composition_bytes: Vec<u8>,
    pub patch_range: Range<usize>,
    pub patch_timing_range: Range<usize>,
    pub patch_interval: u32,
    pub patch_tick: u32,
    pub post_pc_timing_range: Range<usize>,
    pub post_pc_interval: u32,
    pub note_range: Range<usize>,
    pub note_tick: u32,
    pub logical_source_ordinal: u64,
    pub raw_patch_ordinal: u64,
    pub raw_note_ordinal: u64,
    pub context: Option<EmbeddedContext>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OwnedBankPatch {
    provenance: BankPatchProvenance,
    routing: BoundedRoutingResolution,
    events: [DecodedExportEvent; 2],
}
impl OwnedBankPatch {
    pub fn provenance(&self) -> &BankPatchProvenance {
        &self.provenance
    }
    pub fn routing(&self) -> &BoundedRoutingResolution {
        &self.routing
    }
    pub fn decoded_export_events(&self) -> [DecodedExportEvent; 2] {
        self.events.clone()
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BankHandoffRefusal {
    Qualification,
    Ownership,
    Binding,
    Placement,
}

fn bind(
    bytes: &[u8],
    sequence_ordinal: usize,
    pair_ordinal: usize,
    current: &MixedEventItem<'_>,
    routing: &BoundedRoutingResolution,
    ordinal: u64,
    q: &BoundedBankPatch<'_>,
) -> Result<OwnedBankPatch, BankHandoffRefusal> {
    let MixedEventItem::PatchToNote(n) = current else {
        return Err(BankHandoffRefusal::Placement);
    };
    let event = q.decoded_export_event();
    if q.source() != current
        || q.routing() != routing
        || event.source_ordinal != ordinal
        || event.source_range.as_ref() != Some(&n.patch.representation_range)
        || event.absolute_position != n.patch_position
        || !matches!(event.kind,DecodedExportEventKind::TargetedPatch{channel,..} if channel==routing.midi_channel)
    {
        return Err(BankHandoffRefusal::Binding);
    }
    let project = parse_project_166(bytes).map_err(|_| BankHandoffRefusal::Ownership)?;
    let s = project
        .sequences
        .get(sequence_ordinal)
        .ok_or(BankHandoffRefusal::Ownership)?;
    let pair = s
        .track_pairs
        .get(pair_ordinal)
        .ok_or(BankHandoffRefusal::Ownership)?;
    if routing.pair_ordinal != pair_ordinal
        || s.descriptors.get(pair_ordinal + 2).map(|d| &d.range) != Some(&routing.descriptor_range)
        || s.validated_track_event_bounds(pair_ordinal)
            .map_err(|_| BankHandoffRefusal::Ownership)?
            .event_range
            != routing.event_range
        || routing.event_range != routing.consumed_range
        || bytes.get(n.representation_range.clone()).is_none()
        || n.representation_range.start < routing.event_range.start
        || n.representation_range.end > routing.event_range.end
    {
        return Err(BankHandoffRefusal::Ownership);
    }
    let context = match (&n.context, &n.final_timing) {
        (Some(c), Some(t)) => Some(EmbeddedContext {
            range: c.range.clone(),
            bytes: bytes[c.range.clone()].to_vec(),
            payload_range: c.payload.range.clone(),
            payload: c.payload.bytes.to_vec(),
            final_timing_range: t.range.clone(),
            final_interval: t.value,
        }),
        (None, None) => None,
        _ => return Err(BankHandoffRefusal::Binding),
    };
    // CCA-071 admits only ordinary Controllers before the first Patch, so
    // logical and raw Patch ordinals agree. Its context is a separate raw object.
    let raw_note_ordinal = ordinal + 1 + u64::from(context.is_some());
    let note =
        DecodedExportEvent::from_note_body(n.first_note_position, ordinal + 1, &n.first_note);
    Ok(OwnedBankPatch {
        provenance: BankPatchProvenance {
            sequence_ordinal,
            descriptor_ordinal: pair_ordinal + 2,
            pair_ordinal,
            primary_range: pair.primary.record_range.clone(),
            secondary_range: pair.secondary.record_range.clone(),
            event_bounds: routing.event_range.clone(),
            declared_source_objects: u32::from_be_bytes(
                pair.primary.payload.bytes[2..6]
                    .try_into()
                    .map_err(|_| BankHandoffRefusal::Ownership)?,
            ),
            composition_range: n.representation_range.clone(),
            composition_bytes: bytes[n.representation_range.clone()].to_vec(),
            patch_range: n.patch.representation_range.clone(),
            patch_timing_range: n.patch.position.range.clone(),
            patch_interval: n.patch.position.value,
            patch_tick: n.patch_position,
            post_pc_timing_range: n.patch.post_pc_timing_component.range.clone(),
            post_pc_interval: n.patch.post_pc_timing_component.value,
            note_range: n.first_note.representation_range.clone(),
            note_tick: n.first_note_position,
            logical_source_ordinal: ordinal,
            raw_patch_ordinal: ordinal,
            raw_note_ordinal,
            context,
        },
        routing: routing.clone(),
        events: [event, note],
    })
}

/// Builder seam: qualification is source-derived, and must bind to this exact
/// currently owned item. Caller-supplied facts cannot manufacture a bank class.
pub(crate) fn qualify_current(
    bytes: &[u8],
    sequence_ordinal: usize,
    pair_ordinal: usize,
    current: &MixedEventItem<'_>,
    routing: &BoundedRoutingResolution,
    ordinal: u64,
) -> Result<OwnedBankPatch, BankHandoffRefusal> {
    let q = qualify_descriptor166_bank_patch(bytes, sequence_ordinal, pair_ordinal)
        .map_err(|_| BankHandoffRefusal::Qualification)?;
    bind(
        bytes,
        sequence_ordinal,
        pair_ordinal,
        current,
        routing,
        ordinal,
        &q,
    )
}
/// Read-only evidence entry point; does not decide inclusion or export authority.
pub fn qualify_track(
    bytes: &[u8],
    sequence_ordinal: usize,
    pair_ordinal: usize,
) -> Result<OwnedBankPatch, BankHandoffRefusal> {
    let q = qualify_descriptor166_bank_patch(bytes, sequence_ordinal, pair_ordinal)
        .map_err(|_| BankHandoffRefusal::Qualification)?;
    let w = walk_bounded_mixed_events(
        bytes,
        MixedEventBounds {
            event_range: q.routing().event_range.clone(),
        },
        Default::default(),
    )
    .map_err(|_| BankHandoffRefusal::Binding)?;
    let mut ordinal = 0;
    for item in &w.items {
        if ordinal == q.decoded_export_event().source_ordinal {
            return bind(
                bytes,
                sequence_ordinal,
                pair_ordinal,
                item,
                q.routing(),
                ordinal,
                &q,
            );
        }
        ordinal += item.logical_event_count() as u64;
    }
    Err(BankHandoffRefusal::Binding)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> Vec<u8> {
        let mut e = vec![
            9, 255, 65, 5, 0, 1, 0, 7, 90, 37, 255, 124, 19, 0, 0, 42, 8, 42, 5,
        ];
        e.extend(b"Fresh");
        e.extend([2, b'9', b'9', 4, 255, 80, 255, 42, 0, 144, 60, 100, 64, 12]);
        let mut b = crate::bounded_sequence::tests::synthetic(&e);
        let at = parse_project_166(&b).unwrap().sequences[0].track_pairs[0]
            .primary
            .payload
            .range
            .start
            + 2;
        b[at..at + 4].copy_from_slice(&3u32.to_be_bytes());
        b
    }
    #[test]
    fn exact_current_item_and_owned_route_binding_refuse_mutated_assertions() {
        let b = fixture();
        let q = qualify_descriptor166_bank_patch(&b, 0, 0).unwrap();
        let current = q.source().clone();
        let r = q.routing().clone();
        assert!(bind(&b, 0, 0, &current, &r, 1, &q).is_ok());
        for field in 0..4 {
            let mut changed = current.clone();
            let MixedEventItem::PatchToNote(n) = &mut changed else {
                panic!()
            };
            match field {
                0 => n.patch_position += 1,
                1 => n.first_note_position += 1,
                2 => n.patch.representation_range.start += 1,
                _ => n.first_note.representation_range.end -= 1,
            };
            assert_eq!(
                bind(&b, 0, 0, &changed, &r, 1, &q).unwrap_err(),
                BankHandoffRefusal::Binding
            );
        }
        for field in 0..5 {
            let mut changed = r.clone();
            match field {
                0 => changed.midi_channel = 5,
                1 => changed.instrument_candidate = 2,
                2 => changed.descriptor_range.start += 1,
                3 => changed.pair_ordinal = 1,
                _ => changed.event_range.end -= 1,
            };
            assert!(bind(&b, 0, 0, &current, &changed, 1, &q).is_err());
        }
        for ordinal in [0, 2] {
            assert!(bind(&b, 0, 0, &current, &r, ordinal, &q).is_err());
        }
        assert!(bind(&b, 1, 0, &current, &r, 1, &q).is_err());
        assert!(bind(&b, 0, 1, &current, &r, 1, &q).is_err());
        let mut other = b.clone();
        let at = r.type10_payload_range.start + 27;
        other[at] = 8;
        let other_q = qualify_descriptor166_bank_patch(&other, 0, 0).unwrap();
        assert!(bind(&b, 0, 0, &current, &r, 1, &other_q).is_err());
    }
}
