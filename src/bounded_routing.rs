//! Agreement-gated single-Instrument routing for Descriptor166 projects.
//!
//! Pure observations, independent of inclusion, Patch translation, readiness,
//! and export authority. No filename, digest, or compatibility-profile lookup.

use crate::mixed_event::{
    walk_bounded_mixed_events, MixedEventBounds, MixedEventItem, MixedEventKind,
};
use crate::patch::LocatedBytes;
use crate::routing_evidence::{collect_routing_evidence, RoutingEvidence, RoutingEvidenceError};
use crate::sequence_container::{
    parse_project_166, SequenceContainer, SequenceDescriptor, TrackAssociations,
};
use std::ops::Range;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AssociationForm {
    F,
    Z,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContextKind {
    Patch,
    Controller,
    Ff60,
}

/// Every guarded context, at a decoder-validated boundary, is retained.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidatedContext {
    pub kind: ContextKind,
    pub range: Range<usize>,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundedRoutingResolution {
    pub pair_ordinal: usize,
    pub descriptor_range: Range<usize>,
    pub label_range: Range<usize>,
    pub label_bytes: Vec<u8>,
    pub association_range: Range<usize>,
    pub association_form: AssociationForm,
    pub first_slot: [u8; 2],
    pub instrument_candidate: u8,
    pub table_count: usize,
    pub type10_record_range: Range<usize>,
    pub type10_payload_range: Range<usize>,
    /// H1 physical position and H2 unique +25 lookup must be equal.
    pub h1_position: usize,
    pub h2_position: usize,
    pub table_wide_position_agreement: bool,
    pub selected_25: u8,
    pub selected_26: u8,
    pub selected_27: u8,
    pub type2a_position: usize,
    pub type2a_record_range: Range<usize>,
    pub device_name: Vec<u8>,
    pub midi_channel: u8,
    pub event_range: Range<usize>,
    pub consumed_range: Range<usize>,
    pub validated_contexts: Vec<ValidatedContext>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RoutingRefusal {
    AmbiguousOwnership,
    BlankOrUnboundedLabel,
    AssociationOutOfBounds,
    UnsupportedAssociation,
    UnsupportedNumericScope,
    MalformedType10Table,
    AmbiguousType10Lookup,
    Type10AgreementFailure,
    InvalidChannel,
    MalformedDeviceTable,
    MissingDevice,
    AmbiguousDevice,
    InvalidEventBounds(String),
    EmptyEventRegion,
    IncompleteEventWalk(String),
    ConflictingContext {
        kind: ContextKind,
        range: Range<usize>,
        bytes: Vec<u8>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackRoutingResult {
    pub structural_ordinal: usize,
    pub descriptor_ordinal: usize,
    pub result: Result<BoundedRoutingResolution, RoutingRefusal>,
}

/// Reparse the same supplied source for all evidence; callers cannot mix
/// externally constructed bindings, tables, or event walks from other files.
/// A framing failure is unmeasurable; per-track refusals remain explicit.
pub fn decode_bounded_routing(
    bytes: &[u8],
) -> Result<Vec<TrackRoutingResult>, RoutingEvidenceError> {
    let project = parse_project_166(bytes)
        .map_err(|e| RoutingEvidenceError::ProjectStructure(format!("{e:?}")))?;
    let evidence = collect_routing_evidence(bytes)?;
    let mut results = Vec::new();
    for (structural_ordinal, sequence) in project.sequences.iter().enumerate() {
        for descriptor in sequence.track_descriptors() {
            results.push(TrackRoutingResult {
                structural_ordinal,
                descriptor_ordinal: descriptor.ordinal,
                result: resolve(bytes, sequence, descriptor, &evidence),
            });
        }
    }
    Ok(results)
}

fn association(neighborhood: &[u8]) -> Result<(AssociationForm, u8), RoutingRefusal> {
    if neighborhood.len() != 33 || neighborhood[0] != 1 || neighborhood[1] != 0 {
        return Err(RoutingRefusal::UnsupportedAssociation);
    }
    let tail = &neighborhood[3..];
    let form = if tail.iter().all(|b| *b == 0xff) {
        AssociationForm::F
    } else if tail.iter().all(|b| *b == 0) {
        AssociationForm::Z
    } else {
        return Err(RoutingRefusal::UnsupportedAssociation);
    };
    Ok((form, neighborhood[2]))
}

fn resolve(
    bytes: &[u8],
    sequence: &SequenceContainer<'_>,
    descriptor: &SequenceDescriptor<'_>,
    evidence: &RoutingEvidence,
) -> Result<BoundedRoutingResolution, RoutingRefusal> {
    let TrackAssociations::Ordinal(bindings) = &sequence.track_associations else {
        return Err(RoutingRefusal::AmbiguousOwnership);
    };
    let binding = bindings
        .iter()
        .find(|b| b.descriptor_ordinal == descriptor.ordinal)
        .ok_or(RoutingRefusal::AmbiguousOwnership)?;
    let label = descriptor
        .label
        .as_ref()
        .filter(|l| !l.bytes.is_empty() && l.bytes.iter().any(|b| !b.is_ascii_whitespace()))
        .ok_or(RoutingRefusal::BlankOrUnboundedLabel)?;
    let bounds = sequence
        .validated_track_event_bounds(binding.pair_ordinal)
        .map_err(|e| RoutingRefusal::InvalidEventBounds(format!("{e:?}")))?;
    if bounds.event_range.is_empty() {
        return Err(RoutingRefusal::EmptyEventRegion);
    }
    let start = label
        .range
        .start
        .checked_sub(33)
        .filter(|start| *start >= sequence.sequence_range.start)
        .ok_or(RoutingRefusal::AssociationOutOfBounds)?;
    let association_range = start..label.range.start;
    let neighborhood = bytes
        .get(association_range.clone())
        .ok_or(RoutingRefusal::AssociationOutOfBounds)?;
    let (association_form, i) = association(neighborhood)?;
    let table = &evidence.type10_records;
    if i == 0 || usize::from(i) >= table.len() || table.len() > 99 {
        return Err(RoutingRefusal::UnsupportedNumericScope);
    }
    if table.iter().any(|r| r.framed.payload.len() != 36) {
        return Err(RoutingRefusal::MalformedType10Table);
    }
    let matches: Vec<_> = table
        .iter()
        .enumerate()
        .filter(|(_, r)| r.framed.payload[25] == i)
        .map(|(p, _)| p)
        .collect();
    if matches.len() > 1 {
        return Err(RoutingRefusal::AmbiguousType10Lookup);
    }
    if matches.as_slice() != [usize::from(i)]
        || table
            .iter()
            .enumerate()
            .any(|(p, r)| usize::from(r.framed.payload[25]) != p)
    {
        return Err(RoutingRefusal::Type10AgreementFailure);
    }
    let selected = &table[usize::from(i)];
    let payload = &selected.framed.payload;
    if payload[27] > 15 {
        return Err(RoutingRefusal::InvalidChannel);
    }
    // The observed relationship is a 40-byte, name-bearing type-0x2a record
    // with its identifier at +33. This is an applicability guard only.
    if evidence.type2a_records.iter().any(|r| {
        r.framed.payload.len() != 40
            || r.name_bytes.as_ref().map_or(true, |n| {
                n.is_empty() || n.len() > 32 || n.iter().all(u8::is_ascii_whitespace)
            })
    }) {
        return Err(RoutingRefusal::MalformedDeviceTable);
    }
    let devices: Vec<_> = evidence
        .type2a_records
        .iter()
        .enumerate()
        .filter(|(_, r)| r.identifier_field.map(|f| f.0) == Some(payload[26]))
        .collect();
    let (device_position, device) = match devices.as_slice() {
        [] => return Err(RoutingRefusal::MissingDevice),
        [one] => *one,
        _ => return Err(RoutingRefusal::AmbiguousDevice),
    };
    let walk = walk_bounded_mixed_events(
        bytes,
        MixedEventBounds {
            event_range: bounds.event_range.clone(),
        },
        Default::default(),
    )
    .map_err(|e| RoutingRefusal::IncompleteEventWalk(format!("{e:?}")))?;
    if walk.consumed_range != bounds.event_range {
        return Err(RoutingRefusal::IncompleteEventWalk(
            "incomplete consumption".into(),
        ));
    }
    let mut contexts = Vec::new();
    for item in &walk.items {
        match item {
            MixedEventItem::Patch(p) => guard(
                &mut contexts,
                ContextKind::Patch,
                &p.patch.pre_name_context,
                i,
            )?,
            MixedEventItem::PatchToNote(p) => {
                guard(
                    &mut contexts,
                    ContextKind::Patch,
                    &p.patch.pre_name_context,
                    i,
                )?;
                if let Some(c) = &p.initial_context {
                    guard(&mut contexts, ContextKind::Ff60, &c.context.payload, i)?;
                }
                if let Some(c) = &p.context {
                    guard(&mut contexts, ContextKind::Ff60, &c.payload, i)?;
                }
            }
            MixedEventItem::Event(e) => match &e.event {
                MixedEventKind::MidiController(_) => {
                    return Err(RoutingRefusal::IncompleteEventWalk(
                        "source channel Controller requires event-channel routing agreement".into(),
                    ));
                }
                MixedEventKind::Controller(c) => {
                    guard(&mut contexts, ContextKind::Controller, &c.context, i)?
                }
                MixedEventKind::ContextMediatedNote(c) => {
                    guard(&mut contexts, ContextKind::Ff60, &c.context.payload, i)?
                }
                MixedEventKind::DoubleContextMediatedNote(c) => {
                    guard(
                        &mut contexts,
                        ContextKind::Ff60,
                        &c.first_context.payload,
                        i,
                    )?;
                    guard(
                        &mut contexts,
                        ContextKind::Ff60,
                        &c.second_context.payload,
                        i,
                    )?;
                }
                MixedEventKind::Note(_)
                | MixedEventKind::ChannelPressure { .. }
                | MixedEventKind::PitchBend { .. } => {}
            },
        }
    }
    let range = |r: crate::compatibility::ByteRange| r.start() as usize..r.end_exclusive() as usize;
    Ok(BoundedRoutingResolution {
        pair_ordinal: binding.pair_ordinal,
        descriptor_range: descriptor.range.clone(),
        label_range: label.range.clone(),
        label_bytes: label.bytes.to_vec(),
        association_range,
        association_form,
        first_slot: [0, i],
        instrument_candidate: i,
        table_count: table.len(),
        type10_record_range: range(selected.framed.record_range),
        type10_payload_range: range(selected.framed.payload_range),
        h1_position: usize::from(i),
        h2_position: matches[0],
        table_wide_position_agreement: true,
        selected_25: payload[25],
        selected_26: payload[26],
        selected_27: payload[27],
        type2a_position: device_position,
        type2a_record_range: range(device.framed.record_range),
        device_name: device.name_bytes.clone().unwrap_or_default(),
        midi_channel: payload[27] + 1,
        event_range: bounds.event_range,
        consumed_range: walk.consumed_range,
        validated_contexts: contexts,
    })
}

fn guard(
    contexts: &mut Vec<ValidatedContext>,
    kind: ContextKind,
    context: &LocatedBytes<'_>,
    i: u8,
) -> Result<(), RoutingRefusal> {
    let valid = match kind {
        ContextKind::Patch => context.bytes.starts_with(&[0]),
        ContextKind::Controller => context.bytes == [0, i, 0],
        ContextKind::Ff60 => context.bytes.starts_with(&[0x57, 0x7f, 0]),
    };
    if !valid {
        return Err(RoutingRefusal::ConflictingContext {
            kind,
            range: context.range.clone(),
            bytes: context.bytes.to_vec(),
        });
    }
    contexts.push(ValidatedContext {
        kind,
        range: context.range.clone(),
        bytes: context.bytes.to_vec(),
    });
    Ok(())
}
