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
