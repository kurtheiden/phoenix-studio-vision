//! Evidence-bounded CC0-only Patch classification, independent of export
//! authorization, inclusion, readiness, and exact-profile policy.
//!
//! The public entry point derives routing and event evidence from one immutable
//! source. No caller-supplied bindings or decoded objects can bypass the gates.

use crate::bounded_routing::{decode_bounded_routing, BoundedRoutingResolution, RoutingRefusal};
use crate::midi_export::{DecodedExportEvent, PatchTranslation};
use crate::mixed_event::{
    walk_bounded_mixed_events, BoundedPatchToNoteTransition, MixedEventBounds, MixedEventItem,
};
use crate::routing_evidence::RoutingEvidenceError;
use std::ops::Range;

/// Separately qualified Descriptor166 bank translation. This capability is not
/// consumed by the production sequence builder or its legacy CC0 classifier.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundedBankPatch<'a> {
    source: MixedEventItem<'a>,
    event: DecodedExportEvent,
    routing: BoundedRoutingResolution,
}

impl<'a> BoundedBankPatch<'a> {
    /// Full original core/transition, including successor timing and contexts.
    pub fn source(&self) -> &MixedEventItem<'a> {
        &self.source
    }

    /// One logical Patch, expanded by the existing adapter in bank/Program order.
    /// The target comes exclusively from the unchanged same-source routing proof.
    pub fn decoded_export_event(&self) -> DecodedExportEvent {
        self.event.clone()
    }

    pub fn routing(&self) -> &BoundedRoutingResolution {
        &self.routing
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BankPatchRefusal {
    Structure,
    Routing(RoutingRefusal),
    ObjectCount,
    UnsupportedPlacement,
    UnsupportedWrapper,
}

/// Qualify the first banked Patch of one ordinal-owned Descriptor166 Track.
/// CCA-070 plus controlled bank Experiments028/029 support these wrappers only.
/// Complete event decoding is required, but no other event receives translation
/// or inclusion authority. In particular this does not authorize whole export.
pub fn qualify_descriptor166_bank_patch(
    bytes: &[u8],
    sequence_ordinal: usize,
    pair_ordinal: usize,
) -> Result<BoundedBankPatch<'_>, BankPatchRefusal> {
    use crate::midi_export::DecodedExportEventKind;
    use crate::mixed_event::MixedEventKind;
    use crate::sequence_container::parse_project_166;

    let project = parse_project_166(bytes).map_err(|_| BankPatchRefusal::Structure)?;
    let sequence = project
        .sequences
        .get(sequence_ordinal)
        .ok_or(BankPatchRefusal::Structure)?;
    let pair = sequence
        .track_pairs
        .get(pair_ordinal)
        .ok_or(BankPatchRefusal::Structure)?;
    let routing = decode_bounded_routing(bytes)
        .map_err(|_| BankPatchRefusal::Structure)?
        .into_iter()
        .find(|r| {
            r.structural_ordinal == sequence_ordinal && r.descriptor_ordinal == pair_ordinal + 2
        })
        .ok_or(BankPatchRefusal::Structure)?
        .result
        .map_err(BankPatchRefusal::Routing)?;
    let walk = walk_bounded_mixed_events(
        bytes,
        MixedEventBounds {
            event_range: routing.event_range.clone(),
        },
        Default::default(),
    )
    .map_err(|_| BankPatchRefusal::Structure)?;
    let declared = u32::from_be_bytes(
        pair.primary
            .payload
            .bytes
            .get(2..6)
            .ok_or(BankPatchRefusal::Structure)?
            .try_into()
            .map_err(|_| BankPatchRefusal::Structure)?,
    ) as usize;
    // Context objects remain separate from logical musical objects. Do not
    // turn the adapter's bank expansion into additional source objects.
    let contexts = walk
        .items
        .iter()
        .map(|item| match item {
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
        })
        .sum::<usize>();
    if walk.consumed_range != routing.event_range
        || walk.logical_event_count() + contexts != declared
    {
        return Err(BankPatchRefusal::ObjectCount);
    }
    let mut ordinal = 0;
    for (index, item) in walk.items.iter().enumerate() {
        let (position, range, p, pre, post) = match item {
            MixedEventItem::Patch(p) if index + 1 < walk.items.len() => (
                p.position,
                p.patch.representation_range.clone(),
                p.patch.program_change.value,
                p.patch.pre_name_context.bytes,
                p.patch.post_name_context.bytes,
            ),
            MixedEventItem::PatchToNote(p) if p.initial_context.is_none() => (
                p.patch_position,
                p.patch.representation_range.clone(),
                p.patch.program_change.value,
                p.patch.pre_name_context.bytes,
                p.patch.post_name_context.bytes,
            ),
            MixedEventItem::Event(e) if matches!(e.event, MixedEventKind::Controller(_)) => {
                ordinal += 1;
                continue;
            }
            _ => return Err(BankPatchRefusal::UnsupportedPlacement),
        };
        let digits = |label: &[u8]| !label.is_empty() && label.iter().all(u8::is_ascii_digit);
        let translation = if p <= 127 && post.len() >= 5 && post[0] as usize + 5 == post.len() {
            let label = &post[1..post.len() - 4];
            let tail = &post[post.len() - 4..];
            match (pre, tail) {
                ([0, 0, a, 8, b], [4, 255, 80, 255])
                    if *a == p && *b == p && label.len() == 2 && digits(label) =>
                {
                    Some(PatchTranslation::ConfirmedBankSelectMsb { msb: 80 })
                }
                ([0, 0, a, 8, b], [4, 255, 80, 255])
                    if *a == p | 128
                        && *b == p
                        && label.len() == 3
                        && label[0] == b'I'
                        && digits(&label[1..]) =>
                {
                    Some(PatchTranslation::ConfirmedBankSelectMsb { msb: 80 })
                }
                ([0, 0, a, 8, b], [4, 255, 80, 0])
                    if *a == p && *b == p && label.len() == 2 && digits(label) =>
                {
                    Some(PatchTranslation::ConfirmedBankSelect { msb: 80, lsb: 0 })
                }
                ([0, 1, a, 248, b], [4, 255, 81, 1])
                    if *a == p
                        && *b == p | 128
                        && matches!(label.len(), 1 | 2)
                        && digits(label) =>
                {
                    Some(PatchTranslation::ConfirmedBankSelect { msb: 81, lsb: 1 })
                }
                _ => None,
            }
        } else {
            None
        }
        .ok_or(BankPatchRefusal::UnsupportedWrapper)?;
        return Ok(BoundedBankPatch {
            source: item.clone(),
            event: DecodedExportEvent {
                absolute_position: position,
                source_ordinal: ordinal,
                source_range: Some(range),
                kind: DecodedExportEventKind::TargetedPatch {
                    channel: routing.midi_channel,
                    program: p,
                    translation,
                },
            },
            routing,
        });
    }
    Err(BankPatchRefusal::UnsupportedPlacement)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc0PatchRefusal {
    UnsupportedTransition,
    NonzeroPosition,
    PayloadLength,
    NameLength,
    InvalidProgram,
    UnsupportedProgram,
    PreNameContext,
    PostNameContext,
    InitialContext,
}

/// Retains the full borrowed, source-located transition, including opaque
/// contexts and successor timing. Construction is private to the classifier.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundedCc0Patch<'a> {
    source_ordinal: u64,
    source: BoundedPatchToNoteTransition<'a>,
}

impl<'a> BoundedCc0Patch<'a> {
    /// Logical event ordinal, counting the Note in earlier Patch/Note items.
    pub fn source_ordinal(&self) -> u64 {
        self.source_ordinal
    }

    pub fn source(&self) -> &BoundedPatchToNoteTransition<'a> {
        &self.source
    }

    /// Supplies translation evidence, not permission to include/export a track.
    /// The existing adapter emits CC0 immediately before PC without CC32.
    /// Consumers must preserve logical source order. The legacy SMF serializer
    /// prioritizes message families at equal ticks and is not an order-preserving
    /// native-export integration for this capability.
    pub fn decoded_export_event(&self) -> DecodedExportEvent {
        DecodedExportEvent::from_patch(
            self.source.patch_position,
            self.source_ordinal,
            &self.source.patch,
            PatchTranslation::ConfirmedBankSelectMsb { msb: 80 },
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cc0PatchResult<'a> {
    pub source_ordinal: u64,
    pub source_range: Range<usize>,
    pub result: Result<BoundedCc0Patch<'a>, Cc0PatchRefusal>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cc0TrackClassification<'a> {
    pub routing: BoundedRoutingResolution,
    /// Empty means no Patch events were found in the complete walk, not that
    /// inclusion or any other output policy has been decided.
    pub patches: Vec<Cc0PatchResult<'a>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cc0TrackResult<'a> {
    pub structural_ordinal: usize,
    pub descriptor_ordinal: usize,
    /// A track-level refusal does not imply zero Patch events: they cannot be
    /// safely classified under this gate.
    pub result: Result<Cc0TrackClassification<'a>, RoutingRefusal>,
}

/// Classifies only the observed CC0=80 form. Every Patch on a fully routed
/// track receives success or an explicit refusal; unsafe tracks remain refused.
/// Framing errors reuse the existing routing error architecture.
pub fn classify_bounded_cc0_patches(
    bytes: &[u8],
) -> Result<Vec<Cc0TrackResult<'_>>, RoutingEvidenceError> {
    Ok(decode_bounded_routing(bytes)?
        .into_iter()
        .map(|track| Cc0TrackResult {
            structural_ordinal: track.structural_ordinal,
            descriptor_ordinal: track.descriptor_ordinal,
            result: track
                .result
                .and_then(|routing| classify_track(bytes, routing)),
        })
        .collect())
}

fn classify_track(
    bytes: &[u8],
    routing: BoundedRoutingResolution,
) -> Result<Cc0TrackClassification<'_>, RoutingRefusal> {
    let walk = walk_bounded_mixed_events(
        bytes,
        MixedEventBounds {
            event_range: routing.event_range.clone(),
        },
        Default::default(),
    )
    .map_err(|error| RoutingRefusal::IncompleteEventWalk(format!("{error:?}")))?;
    if walk.consumed_range != routing.event_range {
        return Err(RoutingRefusal::IncompleteEventWalk(
            "incomplete consumption".into(),
        ));
    }
    let mut patches = Vec::new();
    let mut source_ordinal = 0;
    for item in walk.items {
        let count = item.logical_event_count() as u64;
        match item {
            MixedEventItem::Patch(patch) => patches.push(Cc0PatchResult {
                source_ordinal,
                source_range: patch.patch.representation_range,
                result: Err(Cc0PatchRefusal::UnsupportedTransition),
            }),
            MixedEventItem::PatchToNote(source) => patches.push(Cc0PatchResult {
                source_ordinal,
                source_range: source.representation_range.clone(),
                result: classify_transition(source_ordinal, *source),
            }),
            MixedEventItem::Event(_) => {}
        }
        source_ordinal += count;
    }
    Ok(Cc0TrackClassification { routing, patches })
}

fn classify_transition(
    source_ordinal: u64,
    source: BoundedPatchToNoteTransition<'_>,
) -> Result<BoundedCc0Patch<'_>, Cc0PatchRefusal> {
    let patch = &source.patch;
    if source.patch_position != 0 {
        return Err(Cc0PatchRefusal::NonzeroPosition);
    }
    // Accept only the direct explicit-Note successor, not mediated contexts.
    if source.context.is_some()
        || source.final_timing.is_some()
        || !patch.pre_note_context.bytes.is_empty()
        || patch.note_status.value != 0x90
    {
        return Err(Cc0PatchRefusal::UnsupportedTransition);
    }
    if let Some(initial) = &source.initial_context {
        if initial.context.payload_length.value != 7
            || !initial.context.payload.bytes.starts_with(&[0x57, 0x7f, 0])
        {
            return Err(Cc0PatchRefusal::InitialContext);
        }
    }
    if patch.payload_length.value != 27 {
        return Err(Cc0PatchRefusal::PayloadLength);
    }
    if patch.name_length.value != 12 || patch.name.bytes.len() != 12 {
        return Err(Cc0PatchRefusal::NameLength);
    }
    let p = patch.program_change.value;
    if p > 127 {
        return Err(Cc0PatchRefusal::InvalidProgram);
    }
    if !matches!(p, 16 | 35) {
        return Err(Cc0PatchRefusal::UnsupportedProgram);
    }
    if patch.pre_name_context.bytes != [0, 0, p | 0x80, 8, p] {
        return Err(Cc0PatchRefusal::PreNameContext);
    }
    let post = patch.post_name_context.bytes;
    if post.len() != 8
        || post[0] != 3
        || !post[1..4].is_ascii()
        || post[4..] != [4, 0xff, 0x50, 0xff]
    {
        return Err(Cc0PatchRefusal::PostNameContext);
    }
    Ok(BoundedCc0Patch {
        source_ordinal,
        source,
    })
}
