//! Ordinary FF41 translation evidence, independent of inclusion/export authority.
//! Private construction reparses ownership, complete event accounting and routing.
use crate::{
    bounded_routing::{decode_bounded_routing, ContextKind},
    midi_export::{DecodedExportEvent, DecodedExportEventKind},
    mixed_event::{walk_bounded_mixed_events, MixedEventBounds, MixedEventItem, MixedEventKind},
    sequence_container::{parse_project_166, TrackAssociations},
};
use std::ops::Range;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ControllerProvenance {
    pub sequence_ordinal: usize,
    pub descriptor_ordinal: usize,
    pub pair_ordinal: usize,
    pub primary_range: Range<usize>,
    pub secondary_range: Range<usize>,
    pub event_bounds: Range<usize>,
    pub declared_source_objects: u32,
    pub representation_range: Range<usize>,
    pub representation_bytes: Vec<u8>,
    pub context_range: Range<usize>,
    pub context_bytes: [u8; 3],
    pub timing_range: Range<usize>,
    pub timing_bytes: Vec<u8>,
    pub interval: u32,
    pub absolute_tick: u32,
    pub logical_source_ordinal: u64,
    pub raw_source_object_ordinal: u64,
    pub association_range: Range<usize>,
    pub instrument_row: u8,
    pub type10_record_range: Range<usize>,
    pub device_record_range: Range<usize>,
    pub channel: u8,
    pub number: u8,
    pub value: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoutingValidatedController {
    provenance: ControllerProvenance,
}
impl RoutingValidatedController {
    // Only an independently source-qualified owned composition can enter this
    // bridge. Its outer proof retains the FF60 context and both raw objects;
    // the existing adapter receives the ordinary FF41 record and final tick.
    pub(crate) fn from_context_composition(
        proof: &crate::context_controller166::OwnedContextController,
    ) -> Self {
        Self {
            provenance: proof.controller_provenance().clone(),
        }
    }

    pub fn provenance(&self) -> &ControllerProvenance {
        &self.provenance
    }
    pub fn decoded_export_event(&self) -> DecodedExportEvent {
        let p = &self.provenance;
        DecodedExportEvent {
            absolute_position: p.absolute_tick,
            source_ordinal: p.logical_source_ordinal,
            source_range: Some(p.representation_range.clone()),
            kind: DecodedExportEventKind::RoutingValidatedController166(Box::new(self.clone())),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ControllerRefusal {
    Ownership,
    Routing,
    Walk,
    Accounting,
    Context,
    MidiData,
}

/// Qualify only ordinary FF41 events. Neighboring forms are counted and retained
/// by the complete walk, but receive no translation or inclusion permission.
/// No caller-supplied event, channel, ownership or routing facts are accepted.
pub fn qualify_track(
    source: &[u8],
    sequence_ordinal: usize,
    pair_ordinal: usize,
) -> Result<Vec<RoutingValidatedController>, ControllerRefusal> {
    qualify_impl(source, sequence_ordinal, pair_ordinal, false)
}
pub(crate) fn qualify_prefix(
    source: &[u8],
    sequence_ordinal: usize,
    pair_ordinal: usize,
) -> Result<Vec<RoutingValidatedController>, ControllerRefusal> {
    qualify_impl(source, sequence_ordinal, pair_ordinal, true)
}
fn qualify_impl(
    source: &[u8],
    sequence_ordinal: usize,
    pair_ordinal: usize,
    prefix: bool,
) -> Result<Vec<RoutingValidatedController>, ControllerRefusal> {
    let project = parse_project_166(source).map_err(|_| ControllerRefusal::Ownership)?;
    let s = project
        .sequences
        .get(sequence_ordinal)
        .ok_or(ControllerRefusal::Ownership)?;
    let TrackAssociations::Ordinal(bindings) = &s.track_associations else {
        return Err(ControllerRefusal::Ownership);
    };
    if bindings.len() != s.track_pairs.len()
        || bindings.len() != s.track_descriptors().len()
        || bindings
            .iter()
            .enumerate()
            .any(|(i, b)| b.pair_ordinal != i || b.descriptor_ordinal != i + 2)
    {
        return Err(ControllerRefusal::Ownership);
    }
    let pair = s
        .track_pairs
        .get(pair_ordinal)
        .ok_or(ControllerRefusal::Ownership)?;
    let route = (if prefix {
        crate::bounded_routing::decode_prefix_routing(source)
    } else {
        decode_bounded_routing(source)
    })
    .map_err(|_| ControllerRefusal::Routing)?
    .into_iter()
    .find(|r| r.structural_ordinal == sequence_ordinal && r.descriptor_ordinal == pair_ordinal + 2)
    .ok_or(ControllerRefusal::Routing)?
    .result
    .map_err(|_| ControllerRefusal::Routing)?;
    let bounds = s
        .validated_track_event_bounds(pair_ordinal)
        .map_err(|_| ControllerRefusal::Walk)?
        .event_range;
    let mut walk = if prefix {
        crate::mixed_event::walk_bounded_mixed_event_prefix(
            source,
            MixedEventBounds {
                event_range: bounds.clone(),
            },
            Default::default(),
        )
        .map(|(walk, _)| walk)
    } else {
        walk_bounded_mixed_events(
            source,
            MixedEventBounds {
                event_range: bounds.clone(),
            },
            Default::default(),
        )
    }
    .map_err(|_| ControllerRefusal::Walk)?;
    if prefix && route.consumed_range.end < walk.consumed_range.end {
        walk.items
            .retain(|item| item.representation_range().end <= route.consumed_range.end);
        walk.consumed_range = route.consumed_range.clone();
    }
    if route.event_range != bounds || route.consumed_range != walk.consumed_range {
        return Err(ControllerRefusal::Walk);
    }
    let declared = u32::from_be_bytes(
        pair.primary.payload.bytes[2..6]
            .try_into()
            .map_err(|_| ControllerRefusal::Accounting)?,
    );
    let mut logical = 0u64;
    let mut raw = 0u64;
    let mut out = vec![];
    for item in &walk.items {
        if let MixedEventItem::Event(e) = item {
            if let MixedEventKind::Controller(c) = &e.event {
                if c.context.bytes != [0, route.instrument_candidate, 0]
                    || !route.validated_contexts.iter().any(|v| {
                        v.kind == ContextKind::Controller
                            && v.range == c.context.range
                            && v.bytes == c.context.bytes
                    })
                {
                    return Err(ControllerRefusal::Context);
                }
                if c.controller_number.value > 127 || c.controller_value.value > 127 {
                    return Err(ControllerRefusal::MidiData);
                }
                out.push(RoutingValidatedController {
                    provenance: ControllerProvenance {
                        sequence_ordinal,
                        descriptor_ordinal: pair_ordinal + 2,
                        pair_ordinal,
                        primary_range: pair.primary.record_range.clone(),
                        secondary_range: pair.secondary.record_range.clone(),
                        event_bounds: bounds.clone(),
                        declared_source_objects: declared,
                        representation_range: c.record_range.clone(),
                        representation_bytes: source[c.record_range.clone()].to_vec(),
                        context_range: c.context.range.clone(),
                        context_bytes: [0, route.instrument_candidate, 0],
                        timing_range: c.timing_delta.range.clone(),
                        timing_bytes: c.timing_delta.bytes.to_vec(),
                        interval: c.timing_delta.value,
                        absolute_tick: e.position,
                        logical_source_ordinal: logical,
                        raw_source_object_ordinal: raw,
                        association_range: route.association_range.clone(),
                        instrument_row: route.instrument_candidate,
                        type10_record_range: route.type10_record_range.clone(),
                        device_record_range: route.type2a_record_range.clone(),
                        channel: route.midi_channel,
                        number: c.controller_number.value,
                        value: c.controller_value.value,
                    },
                });
            }
        }
        let contexts = match item {
            MixedEventItem::PatchToNote(p) => {
                usize::from(p.initial_context.is_some()) + usize::from(p.context.is_some())
            }
            MixedEventItem::Event(e) => match &e.event {
                MixedEventKind::ContextMediatedNote(_)
                | MixedEventKind::ContextMediatedController(_) => 1,
                MixedEventKind::DoubleContextMediatedNote(_) => 2,
                MixedEventKind::MidiController(c) => usize::from(c.context.is_some()),
                _ => 0,
            },
            _ => 0,
        };
        logical += item.logical_event_count() as u64;
        raw += (item.logical_event_count() + contexts) as u64;
    }
    if raw > u64::from(declared) || (walk.consumed_range == bounds && raw != u64::from(declared)) {
        return Err(ControllerRefusal::Accounting);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    #[test]
    fn descriptor_proof_cannot_enter_owned120_adapter() {
        use crate::{midi_export::*, sequence_container::parse_project_166, smf::MidiChannel};
        let mut b = crate::bounded_sequence::tests::synthetic(&[0, 255, 65, 5, 0, 1, 0, 7, 64]);
        let at = parse_project_166(&b).unwrap().sequences[0].track_pairs[0]
            .primary
            .payload
            .range
            .start
            + 2;
        b[at..at + 4].copy_from_slice(&1u32.to_be_bytes());
        let proof = super::qualify_track(&b, 0, 0).unwrap().remove(0);
        assert!(adapt_owned120_track(
            &[proof.decoded_export_event()],
            Some(ChannelAssignment {
                channel: MidiChannel::new(4).unwrap(),
                provenance: ChannelAssignmentProvenance::ParsedRouting,
            }),
            TimingPolicy::Identity480,
            PatchPolicy::StrictKnownOnly
        )
        .is_err());
    }
}
