//! Partial translation of source-owned, production-decoded representations.
//! Translation failures never restart the parser or confer new event authority.
use crate::{
    bounded_routing::BoundedRoutingResolution,
    experimental_export::ExperimentalExportError as Error,
    midi_export::DecodedExportEvent,
    mixed_event::{MixedEventItem, MixedEventKind, MixedEventWalk},
};
use serde::Serialize;
use std::ops::Range;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct UnresolvedSpan {
    pub range: Range<usize>,
    pub reason: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct TrackRecovery {
    pub descriptor_ordinal: usize,
    pub status: String,
    pub event_range: Range<usize>,
    pub recovered_ranges: Vec<Range<usize>>,
    pub unresolved: Vec<UnresolvedSpan>,
    pub notes: usize,
    pub controllers: usize,
    pub programs: usize,
}

pub(crate) fn translate(
    source: &[u8],
    sequence: usize,
    pair: usize,
    walk: &MixedEventWalk<'_>,
    route: &BoundedRoutingResolution,
    declared: usize,
) -> Result<(Vec<DecodedExportEvent>, TrackRecovery), Error> {
    let raw: usize = walk
        .items
        .iter()
        .map(|i| i.logical_event_count() + crate::bounded_patch_translation::retained_contexts(i))
        .sum();
    if raw > declared || (walk.consumed_range == walk.event_range && raw != declared) {
        return Err(Error("source-object accounting mismatch".into()));
    }
    let mut report = TrackRecovery {
        descriptor_ordinal: pair + 2,
        status: "complete track".into(),
        event_range: walk.event_range.clone(),
        recovered_ranges: vec![],
        unresolved: vec![],
        notes: 0,
        controllers: 0,
        programs: 0,
    };
    let ordinary_controllers =
        crate::controller166::qualify_prefix(source, sequence, pair).unwrap_or_default();
    let context_notes =
        crate::context_note166::qualify_track(source, sequence, pair).unwrap_or_default();
    let sysex = crate::sysex166::qualify_track(source, sequence, pair, walk).unwrap_or_default();
    let cc0 = crate::bounded_patch_translation::classify_bounded_cc0_patches(source)
        .ok()
        .and_then(|rows| {
            rows.into_iter()
                .find(|r| r.structural_ordinal == sequence && r.descriptor_ordinal == pair + 2)
        })
        .and_then(|row| row.result.ok());
    let mut events = vec![];
    let mut ordinal = 0;
    for item in &walk.items {
        let span = item.representation_range();
        let translated: Option<Vec<DecodedExportEvent>> = match item {
            MixedEventItem::Event(e) => match &e.event {
                MixedEventKind::Note(n) => {
                    Some(vec![DecodedExportEvent::from_note(e.position, ordinal, n)])
                }
                MixedEventKind::ChannelPressure { entry, .. } => {
                    Some(vec![DecodedExportEvent::from_channel_pressure(
                        e.position, ordinal, entry,
                    )])
                }
                MixedEventKind::PitchBend { entry, .. } => {
                    Some(vec![DecodedExportEvent::from_pitch_bend(
                        e.position, ordinal, entry,
                    )])
                }
                MixedEventKind::Controller(_) => ordinary_controllers
                    .iter()
                    .find(|p| p.provenance().logical_source_ordinal == ordinal)
                    .map(|p| vec![p.decoded_export_event()]),
                MixedEventKind::ContextMediatedController(_) => {
                    crate::context_controller166::qualify_current(
                        source, sequence, pair, item, route, ordinal,
                    )
                    .ok()
                    .map(|p| vec![p.decoded_export_event()])
                }
                MixedEventKind::ContextMediatedNote(_) => context_notes
                    .iter()
                    .find(|p| p.provenance().logical_source_ordinal == ordinal)
                    .map(|p| vec![p.decoded_export_event()]),
                MixedEventKind::SysEx(_) => sysex
                    .iter()
                    .find(|p| p.provenance().logical_source_ordinal == ordinal)
                    .map(|p| vec![DecodedExportEvent::from_source_framed_sysex07(p.clone())]),
                _ => None,
            },
            MixedEventItem::PatchToNote(transition) => cc0
                .as_ref()
                .and_then(|classified| {
                    classified
                        .patches
                        .iter()
                        .find(|p| p.source_ordinal == ordinal)
                        .and_then(|p| p.result.as_ref().ok())
                })
                .map(|patch| {
                    vec![
                        patch.decoded_export_event(),
                        DecodedExportEvent::from_note_body(
                            transition.first_note_position,
                            ordinal + 1,
                            &transition.first_note,
                        ),
                    ]
                })
                .or_else(|| {
                    crate::bank_patch166_handoff::qualify_current(
                        source, sequence, pair, item, route, ordinal,
                    )
                    .ok()
                    .map(|p| p.decoded_export_events().to_vec())
                    .or_else(|| {
                        crate::numeric_program166::qualify_current(
                            source, sequence, pair, item, route, ordinal,
                        )
                        .ok()
                        .map(|p| p.decoded_export_events().to_vec())
                    })
                    .or_else(|| {
                        crate::program_only166::qualify_track(source, sequence, pair)
                            .ok()
                            .filter(|p| {
                                p.provenance().logical_source_ordinal == ordinal
                                    && p.provenance().composition_range == span
                            })
                            .map(|p| p.decoded_export_events().to_vec())
                    })
                }),
            MixedEventItem::Patch(_) => crate::standalone_bank166::qualify_current(
                source, sequence, pair, item, route, ordinal,
            )
            .ok()
            .map(|p| vec![p.decoded_export_event()])
            .or_else(|| {
                crate::terminal_program166::qualify_track(source, sequence, pair)
                    .ok()
                    .filter(|p| {
                        p.provenance().logical_source_ordinal == ordinal
                            && p.provenance().representation_range == span
                    })
                    .map(|p| vec![p.decoded_export_event()])
            }),
        };
        if translated.is_none() {
            if let MixedEventItem::PatchToNote(transition) = item {
                // The shared decoder and routing validator proved this Note body
                // and its independent final timing, even when Patch translation failed.
                let note_range = transition.first_note.representation_range.clone();
                report.unresolved.push(UnresolvedSpan { range: span.start..note_range.start,
                    reason: "Bounded Patch/context/timing prefix has no musical Patch translation; independently decoded successor Note retained. Timbre is unresolved.".into() });
                report.recovered_ranges.push(note_range);
                events.push(DecodedExportEvent::from_note_body(
                    transition.first_note_position,
                    ordinal + 1,
                    &transition.first_note,
                ));
                ordinal += item.logical_event_count() as u64;
                continue;
            }
        }
        if let Some(decoded) = translated {
            report.recovered_ranges.push(span);
            events.extend(decoded);
        } else {
            report.unresolved.push(UnresolvedSpan {range:span,reason:"Decoded boundary and timing; musical translation lacks an established source proof. No events emitted for this representation.".into()});
        }
        ordinal += item.logical_event_count() as u64;
    }
    if walk.consumed_range.end < walk.event_range.end {
        report.unresolved.push(UnresolvedSpan {range:walk.consumed_range.end..walk.event_range.end,
            reason:"Stopped at the first undecodable or unrouteable dispatch. Remaining boundaries, timing and musical content are unknown; no restart attempted.".into()});
    }
    if !report.unresolved.is_empty() {
        report.status = "partially recovered track".into();
    }
    if events.is_empty() {
        return Err(Error("no independently translated events".into()));
    }
    Ok((events, report))
}
