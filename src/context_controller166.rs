//! Owned Descriptor166 23/60 length-7 FF60 -> FF41 handoff. No inclusion authority.
use crate::{
    bounded_routing::{decode_bounded_routing, BoundedRoutingResolution, ContextKind},
    controller166::{
        qualify_track as qualify_accounting, ControllerProvenance, RoutingValidatedController,
    },
    midi_export::DecodedExportEvent,
    mixed_event::{walk_bounded_mixed_events, MixedEventBounds, MixedEventItem, MixedEventKind},
    sequence_container::parse_project_166,
};
use std::ops::Range;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContextControllerProvenance {
    pub sequence_ordinal: usize,
    pub descriptor_ordinal: usize,
    pub pair_ordinal: usize,
    pub primary_range: Range<usize>,
    pub secondary_range: Range<usize>,
    pub event_bounds: Range<usize>,
    pub declared_source_objects: u32,
    pub logical_track_objects: usize,
    pub retained_track_contexts: usize,
    pub composition_range: Range<usize>,
    pub composition_bytes: Vec<u8>,
    pub context_range: Range<usize>,
    pub context_bytes: Vec<u8>,
    pub payload_range: Range<usize>,
    pub payload: [u8; 7],
    pub context_timing_range: Range<usize>,
    pub context_timing_bytes: Vec<u8>,
    pub controller_range: Range<usize>,
    pub controller_bytes: Vec<u8>,
    pub controller_timing_range: Range<usize>,
    pub controller_timing_bytes: Vec<u8>,
    pub previous_tick: u32,
    pub context_interval: u32,
    pub controller_interval: u32,
    pub context_tick: u32,
    pub controller_tick: u32,
    pub logical_source_ordinal: u64,
    pub raw_context_ordinal: u64,
    pub raw_controller_ordinal: u64,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OwnedContextController {
    provenance: ContextControllerProvenance,
    routing: BoundedRoutingResolution,
    controller: ControllerProvenance,
}
impl OwnedContextController {
    pub fn provenance(&self) -> &ContextControllerProvenance {
        &self.provenance
    }
    pub fn routing(&self) -> &BoundedRoutingResolution {
        &self.routing
    }
    pub fn controller_provenance(&self) -> &ControllerProvenance {
        &self.controller
    }
    pub fn decoded_export_event(&self) -> DecodedExportEvent {
        RoutingValidatedController::from_context_composition(self).decoded_export_event()
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContextControllerRefusal {
    Ownership,
    Accounting,
    Routing,
    Walk,
    Placement,
    Form,
    MidiData,
    Timing,
    Binding,
}
fn checked_ticks(
    previous: u32,
    context: u32,
    controller: u32,
) -> Result<(u32, u32), ContextControllerRefusal> {
    let middle = previous
        .checked_add(context)
        .ok_or(ContextControllerRefusal::Timing)?;
    Ok((
        middle,
        middle
            .checked_add(controller)
            .ok_or(ContextControllerRefusal::Timing)?,
    ))
}
/// All authority comes from one immutable source; the ordinal only selects an
/// item in its complete walk. Context bytes beyond the prefix remain opaque.
pub fn qualify_at(
    bytes: &[u8],
    sequence_ordinal: usize,
    pair_ordinal: usize,
    ordinal: u64,
) -> Result<OwnedContextController, ContextControllerRefusal> {
    use ContextControllerRefusal::*;
    qualify_accounting(bytes, sequence_ordinal, pair_ordinal).map_err(|_| Accounting)?;
    let project = parse_project_166(bytes).map_err(|_| Ownership)?;
    let s = project.sequences.get(sequence_ordinal).ok_or(Ownership)?;
    let pair = s.track_pairs.get(pair_ordinal).ok_or(Ownership)?;
    let route = decode_bounded_routing(bytes)
        .map_err(|_| Routing)?
        .into_iter()
        .find(|r| {
            r.structural_ordinal == sequence_ordinal && r.descriptor_ordinal == pair_ordinal + 2
        })
        .ok_or(Routing)?
        .result
        .map_err(|_| Routing)?;
    let bounds = s
        .validated_track_event_bounds(pair_ordinal)
        .map_err(|_| Walk)?
        .event_range;
    let walk = walk_bounded_mixed_events(
        bytes,
        MixedEventBounds {
            event_range: bounds.clone(),
        },
        Default::default(),
    )
    .map_err(|_| Walk)?;
    if walk.consumed_range != bounds
        || route.event_range != bounds
        || route.consumed_range != bounds
    {
        return Err(Walk);
    }
    let declared = u32::from_be_bytes(
        pair.primary.payload.bytes[2..6]
            .try_into()
            .map_err(|_| Accounting)?,
    );
    let logical_track_objects = walk.logical_event_count();
    let retained_track_contexts = (declared as usize)
        .checked_sub(logical_track_objects)
        .ok_or(Accounting)?;
    let (mut logical, mut raw, mut previous) = (0u64, 0u64, 0u32);
    for item in &walk.items {
        if logical == ordinal {
            let MixedEventItem::Event(e) = item else {
                return Err(Placement);
            };
            let MixedEventKind::ContextMediatedController(c) = &e.event else {
                return Err(Placement);
            };
            let context = &c.context.context;
            let cc = &c.controller;
            if context.payload_length.value != 7
                || context.payload.bytes.len() != 7
                || !context.payload.bytes.starts_with(&[0x23, 0x60, 0])
                || cc.payload_length.value != 5
                || cc.context.bytes != [0, route.instrument_candidate, 0]
                || bytes.get(cc.event_tag_range.clone()) != Some(&[0xff, 0x41])
                || c.representation_range.start != c.context.leading_timing.range.start
                || c.context.leading_timing.range.end != context.range.start
                || context.range.end != cc.record_range.start
                || cc.record_range.end != c.representation_range.end
                || !route.validated_contexts.iter().any(|v| {
                    v.kind == ContextKind::Ff60
                        && v.range == context.payload.range
                        && v.bytes == context.payload.bytes
                })
                || !route.validated_contexts.iter().any(|v| {
                    v.kind == ContextKind::Controller
                        && v.range == cc.context.range
                        && v.bytes == cc.context.bytes
                })
            {
                return Err(Form);
            }
            if cc.controller_number.value > 127 || cc.controller_value.value > 127 {
                return Err(MidiData);
            }
            let (context_tick, controller_tick) = checked_ticks(
                previous,
                c.context.leading_timing.value,
                cc.timing_delta.value,
            )?;
            if controller_tick != e.position {
                return Err(Timing);
            }
            return Ok(OwnedContextController {
                provenance: ContextControllerProvenance {
                    sequence_ordinal,
                    descriptor_ordinal: pair_ordinal + 2,
                    pair_ordinal,
                    primary_range: pair.primary.record_range.clone(),
                    secondary_range: pair.secondary.record_range.clone(),
                    event_bounds: bounds,
                    declared_source_objects: declared,
                    logical_track_objects,
                    retained_track_contexts,
                    composition_range: c.representation_range.clone(),
                    composition_bytes: bytes[c.representation_range.clone()].to_vec(),
                    context_range: context.range.clone(),
                    context_bytes: bytes[context.range.clone()].to_vec(),
                    payload_range: context.payload.range.clone(),
                    payload: context.payload.bytes.try_into().map_err(|_| Form)?,
                    context_timing_range: c.context.leading_timing.range.clone(),
                    context_timing_bytes: c.context.leading_timing.bytes.to_vec(),
                    controller_range: cc.record_range.clone(),
                    controller_bytes: bytes[cc.record_range.clone()].to_vec(),
                    controller_timing_range: cc.timing_delta.range.clone(),
                    controller_timing_bytes: cc.timing_delta.bytes.to_vec(),
                    previous_tick: previous,
                    context_interval: c.context.leading_timing.value,
                    controller_interval: cc.timing_delta.value,
                    context_tick,
                    controller_tick,
                    logical_source_ordinal: logical,
                    raw_context_ordinal: raw,
                    raw_controller_ordinal: raw + 1,
                },
                controller: ControllerProvenance {
                    sequence_ordinal,
                    descriptor_ordinal: pair_ordinal + 2,
                    pair_ordinal,
                    primary_range: pair.primary.record_range.clone(),
                    secondary_range: pair.secondary.record_range.clone(),
                    event_bounds: route.event_range.clone(),
                    declared_source_objects: declared,
                    representation_range: cc.record_range.clone(),
                    representation_bytes: bytes[cc.record_range.clone()].to_vec(),
                    context_range: cc.context.range.clone(),
                    context_bytes: [0, route.instrument_candidate, 0],
                    timing_range: cc.timing_delta.range.clone(),
                    timing_bytes: cc.timing_delta.bytes.to_vec(),
                    interval: cc.timing_delta.value,
                    absolute_tick: controller_tick,
                    logical_source_ordinal: logical,
                    raw_source_object_ordinal: raw + 1,
                    association_range: route.association_range.clone(),
                    instrument_row: route.instrument_candidate,
                    type10_record_range: route.type10_record_range.clone(),
                    device_record_range: route.type2a_record_range.clone(),
                    channel: route.midi_channel,
                    number: cc.controller_number.value,
                    value: cc.controller_value.value,
                },
                routing: route,
            });
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
        previous = match item {
            MixedEventItem::Event(e) => e.position,
            MixedEventItem::Patch(p) => p.position,
            MixedEventItem::PatchToNote(p) => p.first_note_position,
        };
    }
    Err(Placement)
}
/// Bind every caller-supplied fact to a fresh source proof and its exact walker item.
pub(crate) fn qualify_current(
    bytes: &[u8],
    sequence: usize,
    pair: usize,
    current: &MixedEventItem<'_>,
    routing: &BoundedRoutingResolution,
    ordinal: u64,
) -> Result<OwnedContextController, ContextControllerRefusal> {
    let proof = qualify_at(bytes, sequence, pair, ordinal)?;
    if proof.routing() != routing {
        return Err(ContextControllerRefusal::Binding);
    }
    let walk = walk_bounded_mixed_events(
        bytes,
        MixedEventBounds {
            event_range: proof.routing.event_range.clone(),
        },
        Default::default(),
    )
    .map_err(|_| ContextControllerRefusal::Walk)?;
    let mut logical = 0;
    for item in &walk.items {
        if logical == ordinal {
            return if item == current {
                Ok(proof)
            } else {
                Err(ContextControllerRefusal::Binding)
            };
        }
        logical += item.logical_event_count() as u64;
    }
    Err(ContextControllerRefusal::Binding)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> Vec<u8> {
        let e = [
            10, 144, 60, 64, 64, 1, 4, 255, 96, 7, 35, 96, 0, 114, 108, 131, 76, 1, 255, 65, 5, 0,
            1, 0, 1, 58,
        ];
        let mut b = crate::bounded_sequence::tests::synthetic(&e);
        let at = parse_project_166(&b).unwrap().sequences[0].track_pairs[0]
            .primary
            .payload
            .range
            .start;
        b[at + 2..at + 6].copy_from_slice(&3u32.to_be_bytes());
        b
    }
    #[test]
    fn exact_current_binding_refuses_forged_fields_routing_ordinals_and_other_source() {
        let b = fixture();
        let q = qualify_at(&b, 0, 0, 1).unwrap();
        let w = walk_bounded_mixed_events(
            &b,
            MixedEventBounds {
                event_range: q.routing.event_range.clone(),
            },
            Default::default(),
        )
        .unwrap();
        let current = &w.items[1];
        assert_eq!(
            qualify_current(&b, 0, 0, current, &q.routing, 1).unwrap(),
            q
        );
        for field in 0..8 {
            let mut changed = current.clone();
            let MixedEventItem::Event(e) = &mut changed else {
                panic!()
            };
            let MixedEventKind::ContextMediatedController(c) = &mut e.event else {
                panic!()
            };
            match field {
                0 => e.position += 1,
                1 => c.context.leading_timing.value += 1,
                2 => c.controller.timing_delta.value += 1,
                3 => c.representation_range.start += 1,
                4 => c.controller.record_range.start += 1,
                5 => c.context.context.payload.range.end -= 1,
                6 => c.context.context.payload.bytes = &[35, 96, 0, 1, 2, 3, 4],
                _ => c.controller.controller_value.value += 1,
            }
            assert_eq!(
                qualify_current(&b, 0, 0, &changed, &q.routing, 1).unwrap_err(),
                ContextControllerRefusal::Binding
            );
        }
        for field in 0..6 {
            let mut route = q.routing.clone();
            match field {
                0 => route.midi_channel = 5,
                1 => route.instrument_candidate = 2,
                2 => route.pair_ordinal = 1,
                3 => route.event_range.end -= 1,
                4 => route.descriptor_range.start += 1,
                _ => route.type10_record_range.start += 1,
            }
            assert_eq!(
                qualify_current(&b, 0, 0, current, &route, 1).unwrap_err(),
                ContextControllerRefusal::Binding
            );
        }
        for ordinal in [0, 2, u64::MAX] {
            assert!(qualify_current(&b, 0, 0, current, &q.routing, ordinal).is_err());
        }
        assert!(qualify_current(&b, 1, 0, current, &q.routing, 1).is_err());
        assert!(qualify_current(&b, 0, 1, current, &q.routing, 1).is_err());
        let mut other = b.clone();
        other[q.routing.type10_payload_range.start + 27] = 5;
        assert!(qualify_current(&other, 0, 0, current, &q.routing, 1).is_err());
        let mut other = b.clone();
        other[q.provenance.payload_range.start + 3] = 42;
        assert!(qualify_current(&other, 0, 0, current, &q.routing, 1).is_err());
        let mut provenance_copy = q.provenance.clone();
        provenance_copy.raw_controller_ordinal = 99;
        assert_eq!(q.provenance.raw_controller_ordinal, 2);
    }
    #[test]
    fn both_checked_additions_and_source120_adapter_boundary_remain_guarded() {
        assert_eq!(
            checked_ticks(u32::MAX, 1, 0),
            Err(ContextControllerRefusal::Timing)
        );
        assert_eq!(
            checked_ticks(u32::MAX - 4, 4, 1),
            Err(ContextControllerRefusal::Timing)
        );
        assert_eq!(
            checked_ticks(u32::MAX - 5, 4, 1),
            Ok((u32::MAX - 1, u32::MAX))
        );
        let b = fixture();
        let q = qualify_at(&b, 0, 0, 1).unwrap();
        assert!(crate::midi_export::adapt_owned120_track(
            &[q.decoded_export_event()],
            Some(crate::midi_export::ChannelAssignment {
                channel: crate::smf::MidiChannel::new(4).unwrap(),
                provenance: crate::midi_export::ChannelAssignmentProvenance::ParsedRouting
            }),
            crate::midi_export::TimingPolicy::Identity480,
            crate::midi_export::PatchPolicy::StrictKnownOnly
        )
        .is_err());
    }
}
