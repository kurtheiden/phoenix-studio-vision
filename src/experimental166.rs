//! Selected direct-Note local rendering; no complete-recovery authority.
use crate::experimental_export::{
    ExperimentalExportError as Error, ExperimentalRegion, ExperimentalReport,
    ExperimentalTrackReport, LABEL,
};
use crate::midi_export::{
    self, ChannelAssignment, ChannelAssignmentProvenance, DecodedExportEvent, MeterPolicy,
    PatchPolicy, TimingPolicy,
};
use crate::mixed_event::{
    walk_bounded_mixed_events, MixedEventBounds, MixedEventItem, MixedEventKind,
};
use crate::sequence_container::{parse_project_166, SequenceContainer, TrackAssociations};
use crate::smf;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::ops::Range;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct TrackIdentity166 {
    pub source_sha256: String,
    pub sequence_ordinal: usize,
    pub sequence_range: Range<usize>,
    pub descriptor_ordinal: usize,
    pub descriptor_range: Range<usize>,
    pub pair_ordinal: usize,
    pub pair_ranges: [Range<usize>; 2],
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ExcludedTrack166 {
    pub identity: TrackIdentity166,
    pub name: String,
    pub known_notes: Option<usize>,
    pub saved_mute: String,
    pub reason: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct SelectedScope166 {
    pub selected: Vec<TrackIdentity166>,
    pub excluded: Vec<ExcludedTrack166>,
}
fn err(e: impl std::fmt::Debug) -> Error {
    Error(format!("{e:?}"))
}
fn identities(
    source: &[u8],
    ordinal: usize,
    s: &SequenceContainer<'_>,
) -> Result<Vec<TrackIdentity166>, Error> {
    let TrackAssociations::Ordinal(bindings) = &s.track_associations else {
        return Err(err("ambiguous ownership"));
    };
    if bindings.is_empty()
        || bindings.len() != s.track_pairs.len()
        || bindings.len() != s.track_descriptors().len()
        || bindings
            .iter()
            .enumerate()
            .any(|(i, b)| b.pair_ordinal != i || b.descriptor_ordinal != i + 2)
    {
        return Err(err("nonexhaustive ownership"));
    }
    let hash = format!("{:x}", Sha256::digest(source));
    Ok(bindings
        .iter()
        .map(|b| {
            let d = &s.descriptors[b.descriptor_ordinal];
            let p = &s.track_pairs[b.pair_ordinal];
            TrackIdentity166 {
                source_sha256: hash.clone(),
                sequence_ordinal: ordinal,
                sequence_range: s.sequence_range.clone(),
                descriptor_ordinal: d.ordinal,
                descriptor_range: d.range.clone(),
                pair_ordinal: b.pair_ordinal,
                pair_ranges: [
                    p.primary.record_range.clone(),
                    p.secondary.record_range.clone(),
                ],
            }
        })
        .collect())
}
pub(crate) fn inventory(source: &[u8], ordinal: usize) -> Result<Vec<TrackIdentity166>, Error> {
    let p = parse_project_166(source).map_err(err)?;
    identities(
        source,
        ordinal,
        p.sequences
            .get(ordinal)
            .ok_or_else(|| err("unknown sequence"))?,
    )
}
pub(crate) fn build(
    source: &[u8],
    ordinal: usize,
    selection: &[TrackIdentity166],
) -> Result<(ExperimentalReport, Vec<u8>), Error> {
    let project = parse_project_166(source).map_err(err)?;
    let s = project
        .sequences
        .get(ordinal)
        .ok_or_else(|| err("unknown sequence"))?;
    let all = identities(source, ordinal, s)?;
    if selection.is_empty()
        || selection
            .iter()
            .enumerate()
            .any(|(i, id)| !all.contains(id) || selection[..i].contains(id))
    {
        return Err(err("unknown, duplicate or mismatched Track selection"));
    }
    // Selection membership is explicit; output always retains structural source order.
    let selected: Vec<_> = all
        .iter()
        .filter(|id| selection.contains(id))
        .cloned()
        .collect();
    let tempo = crate::tempo::decode_bounded_initial_tempo(
        source,
        crate::tempo::InitialTempoBounds {
            event_range: s.initial_tempo_range.clone(),
        },
    )
    .map_err(err)?;
    let meter = crate::meter::decode_bounded_initial_meter(
        source,
        crate::meter::InitialMeterBounds {
            event_range: s.initial_meter_range.clone(),
        },
    )
    .map_err(err)?;
    crate::bounded_sequence::initial_only(
        &s.tempo_primary,
        &s.tempo_secondary,
        &source[tempo.event_range.clone()],
    )
    .map_err(err)?;
    crate::bounded_sequence::initial_only(
        &s.meter_primary,
        &s.meter_secondary,
        &source[meter.event_range.clone()],
    )
    .map_err(err)?;
    let meter_values = (
        meter.numerator.value,
        meter.denominator_exponent.value,
        meter.third_payload.value,
        meter.fourth_payload.value,
    );
    let title = format!(
        "{LABEL} / SELECTED TRACKS: {}",
        String::from_utf8_lossy(s.sequence_name.bytes.bytes)
    );
    let c = midi_export::adapt_conductor(
        title.as_bytes(),
        tempo.mpqn(),
        meter_values,
        TimingPolicy::Identity480,
        MeterPolicy::KnownHistoricalOnly,
    )
    .map_err(err)?;
    let routes = crate::bounded_routing::decode_bounded_routing(source).map_err(err)?;
    let mutes = crate::saved_mute::collect_saved_mute_evidence(source).map_err(err)?;
    let mut chunks = Vec::new();
    let mut expected = vec![crate::readback120::conductor_expected(&c, None)];
    let mut tracks = Vec::new();
    let mut excluded = Vec::new();
    let mut omitted = 0;
    for id in &all {
        let d = &s.descriptors[id.descriptor_ordinal];
        let name = d
            .label
            .as_ref()
            .map(|l| String::from_utf8_lossy(l.bytes).into_owned())
            .unwrap_or_default();
        let mute = mutes
            .iter()
            .find(|m| {
                m.sequence_ordinal == ordinal && m.descriptor_ordinal == id.descriptor_ordinal
            })
            .ok_or_else(|| err("missing saved-state row"))?;
        let state = format!("{:?}", mute.state);
        let bounds = s.validated_track_event_bounds(id.pair_ordinal).map_err(err);
        let walk = bounds.as_ref().map_err(err).and_then(|b| {
            walk_bounded_mixed_events(
                source,
                MixedEventBounds {
                    event_range: b.event_range.clone(),
                },
                Default::default(),
            )
            .map_err(err)
        });
        if !selected.contains(id) {
            // Known direct-Note lower bound only. Other events/context remain unclassified.
            let known=walk.as_ref().ok().map(|w|w.items.iter().filter(|i|matches!(i,MixedEventItem::Event(e) if matches!(e.event,MixedEventKind::Note(_)))).count());
            omitted += known.unwrap_or(0);
            excluded.push(ExcludedTrack166{identity:id.clone(),name,known_notes:known,saved_mute:state,reason:"Not explicitly selected; no events rendered. Note count is a direct-Note lower bound, not complete semantic coverage.".into()});
            continue;
        }
        let bounds = bounds?;
        let walk = walk?;
        if walk.consumed_range != bounds.event_range || walk.items.is_empty() {
            return Err(err("incomplete or empty selected body"));
        }
        let declared = u32::from_be_bytes(
            s.track_pairs[id.pair_ordinal].primary.payload.bytes[2..6]
                .try_into()
                .map_err(err)?,
        ) as usize;
        if declared != walk.items.len() {
            return Err(err("selected Note count mismatch"));
        }
        let mut events = Vec::new();
        for (i, item) in walk.items.iter().enumerate() {
            let MixedEventItem::Event(e) = item else {
                return Err(err("unsupported selected Patch/context"));
            };
            let MixedEventKind::Note(note) = &e.event else {
                return Err(err("unsupported selected musical family/context"));
            };
            events.push(DecodedExportEvent::from_note(e.position, i as u64, note));
        }
        let r = routes
            .iter()
            .find(|r| {
                r.structural_ordinal == ordinal && r.descriptor_ordinal == id.descriptor_ordinal
            })
            .ok_or_else(|| err("missing route"))?
            .result
            .as_ref()
            .map_err(err)?;
        if r.event_range != bounds.event_range || r.consumed_range != bounds.event_range {
            return Err(err("route/body mismatch"));
        }
        let adapted = midi_export::adapt_track(
            &events,
            Some(ChannelAssignment {
                channel: smf::MidiChannel::new(r.midi_channel).map_err(err)?,
                provenance: ChannelAssignmentProvenance::ParsedRouting,
            }),
            TimingPolicy::Identity480,
            PatchPolicy::StrictKnownOnly,
        )
        .map_err(err)?;
        if !adapted.warnings.is_empty() || !adapted.untranslated_metadata.is_empty() {
            return Err(err("unreported translation"));
        }
        let label = midi_export::adapt_text(
            d.label
                .as_ref()
                .ok_or_else(|| err("unbounded label"))?
                .bytes,
        )
        .map_err(err)?;
        chunks.push(
            smf::serialize_named_musical_track_with_note_provenance(
                &label,
                &adapted.scheduled_events,
                &adapted.note_provenance,
            )
            .map_err(err)?,
        );
        expected.push(crate::readback120::musical_expected(
            &label,
            &adapted.scheduled_events,
        ));
        tracks.push(ExperimentalTrackReport {
            ordinal: id.descriptor_ordinal,
            name,
            notes: events.len(),
            channel: Some(r.midi_channel),
            pair_ranges: Some(id.pair_ranges.clone()),
            saved_mute: state,
            inclusion: "Explicit selected Track; original saved mute does not silently omit music"
                .into(),
        });
    }
    let mut cursor = s.sequence_range.start;
    let mut regions = Vec::new();
    for r in project.records.iter().filter(|r| {
        r.record_range.start >= s.sequence_range.start && r.record_range.end <= s.sequence_range.end
    }) {
        if r.record_range.start != cursor {
            return Err(err("unreported source bytes"));
        }
        cursor = r.record_range.end;
        let role = if selected
            .iter()
            .any(|id| id.pair_ranges.contains(&r.record_range))
        {
            "Selected Note pair; framing retained as provenance"
        } else {
            "Excluded Track or unresolved project/conductor/terminal semantics; NOT rendered"
        };
        regions.push(ExperimentalRegion {
            range: r.record_range.clone(),
            record_type: r.record_type.value,
            role: role.into(),
            sha256: format!("{:x}", Sha256::digest(&source[r.record_range.clone()])),
        });
    }
    if cursor != s.sequence_range.end {
        return Err(err("unreported terminal bytes"));
    }
    let refusal =
        match crate::bounded_sequence::build_bounded_sequence_manifest(source, ordinal, 480) {
            Ok(_) => "Selected subset is not a whole-sequence verified authorization".into(),
            Err(e) => format!("{e:?}"),
        };
    let report=ExperimentalReport{label:LABEL.into(),source_sha256:format!("{:x}",Sha256::digest(source)),sequence_ordinal:ordinal,sequence_range:s.sequence_range.clone(),source_name:String::from_utf8_lossy(s.sequence_name.bytes.bytes).into_owned(),verified_refusal:refusal,ppqn:c.ppqn,observed_initial_mpqn:tempo.mpqn(),observed_source_meter:meter_values,included_notes:tracks.iter().map(|t|t.notes).sum(),tracks,omitted_known_notes:omitted,unresolved_semantic_event_count:None,regions,partial120:None,selected_scope166:Some(SelectedScope166{selected,excluded}),limitations:vec![
        "EXPERIMENTAL SELECTED TRACKS: incomplete conditional local timeline, NOT complete or verified recovery.".into(),
        "Observed matching initial tempo/meter copies used; prelude applicability, full map and synchronization remain UNRESOLVED. No default values invented.".into(),
        "Excluded Track known_notes and omitted_known_notes are direct-Note lower bounds only; unknown/unsupported semantics are UNKNOWN, never zero. No individual included event skipped.".into(),
        "Track selection explicitly includes original ON or Unknown saved states when selected; historical inclusion and Instrument/OMS timbre not reproduced.".into(),
        "Terminal meaning and original duration unresolved. EOT closes rendered releases only. Labels can be stripped by other applications.".into()]};
    let json = serde_json::to_vec(&report).map_err(err)?;
    chunks.insert(
        0,
        smf::serialize_conductor_track_with_text(
            &c.sequence_name,
            c.tempo_mpqn,
            c.time_signature,
            &json,
        )
        .map_err(err)?,
    );
    let last = expected[0].len() - 1;
    expected[0].insert(last, (0, 1, json));
    let midi = smf::serialize_format1(c.ppqn, &chunks).map_err(err)?;
    crate::readback120::verify(&midi, c.ppqn, &expected).map_err(err)?;
    Ok((report, midi))
}
