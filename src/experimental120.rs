//! Explicit partial120 recovery. Only complete independently proved ordinary
//! Tracks and explicitly ledgered bounded spans are rendered. Unsupported
//! suffixes and bodies stay unresolved in the ledger.
//! No complete conductor-envelope or verified export authority is conferred.
use crate::{
    authority120::{prove_track, TrackRoute120},
    experimental_export::{
        ExperimentalExportError, ExperimentalRegion, ExperimentalReport, ExperimentalTrackReport,
        LABEL,
    },
    midi_export::{
        self, ChannelAssignment, ChannelAssignmentProvenance, ExportCounts, MeterPolicy,
        PatchPolicy, TimingPolicy,
    },
    semantic_layout120::associate_semantic_project_120,
    sequence_container::parse_root_record_stream,
    smf,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::ops::Range;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Partial120TrackAccounting {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sysex_framing: Vec<SysExFramingProvenance>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub controller_targets: Vec<crate::controller_target120::ControllerTargetProvenance>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ff60_controller_bridges: Vec<Ff60ControllerBridgeProvenance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument_pitch_provenance:
        Option<crate::instrument_transposition::InstrumentPitchProvenance>,
    pub ordinal: usize,
    pub primary_range: Range<usize>,
    pub declared_source_objects: u32,
    pub exported: ExportCountsView,
    pub contexts: Option<usize>,
    pub unresolved_source_objects: u32,
    pub refusal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounded_prefix: Option<BoundedPrefixAccounting>,
}

/// Counted source framing is retained separately from the exported MIDI bytes.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct SysExFramingProvenance {
    pub representation_range: Range<usize>,
    pub source_bytes: Vec<u8>,
    pub status_offset: usize,
    pub stored_length_offset: usize,
    pub stored_length: u8,
    pub framing_offset: usize,
    pub framing_value: u8,
    pub payload_range: Range<usize>,
    pub midi_payload: Vec<u8>,
    pub timing_range: Range<usize>,
    pub timing_bytes: Vec<u8>,
    pub interval: u32,
    pub absolute_tick: u32,
    pub logical_source_ordinal: u64,
}

fn sysex_framing(
    walk: &crate::mixed_event::MixedEventWalk<'_>,
    source: &[u8],
) -> Vec<SysExFramingProvenance> {
    use crate::mixed_event::{MixedEventItem, MixedEventKind};
    let mut ordinal = 0;
    let mut out = vec![];
    for item in &walk.items {
        if let MixedEventItem::Event(e) = item {
            if let MixedEventKind::SysEx(s) = &e.event {
                if let Some(framing) = s.framing {
                    out.push(SysExFramingProvenance {
                        representation_range: s.representation_range.clone(),
                        source_bytes: source[s.representation_range.clone()].to_vec(),
                        status_offset: s.status.offset,
                        stored_length_offset: s.payload_length.offset,
                        stored_length: s.payload_length.value,
                        framing_offset: framing.offset,
                        framing_value: framing.value,
                        payload_range: s.payload.range.clone(),
                        midi_payload: s.payload.bytes.to_vec(),
                        timing_range: s.timing_delta.range.clone(),
                        timing_bytes: s.timing_delta.bytes.to_vec(),
                        interval: s.timing_delta.value,
                        absolute_tick: e.position,
                        logical_source_ordinal: ordinal,
                    });
                }
            }
        }
        ordinal += item.logical_event_count() as u64;
    }
    out
}

/// Both source timing components retained separately from their summed clock.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Ff60ControllerBridgeProvenance {
    pub representation_range: Range<usize>,
    pub context_range: Range<usize>,
    pub context_bytes: Vec<u8>,
    pub leading_timing_range: Range<usize>,
    pub leading_timing_bytes: Vec<u8>,
    pub leading_interval: u32,
    pub successor_timing_range: Range<usize>,
    pub successor_timing_bytes: Vec<u8>,
    pub successor_interval: u32,
    pub controller_position: u32,
}

fn controller_bridges(
    source: &[u8],
    walk: &crate::mixed_event::MixedEventWalk<'_>,
) -> Vec<Ff60ControllerBridgeProvenance> {
    use crate::mixed_event::{MixedEventItem, MixedEventKind};
    walk.items
        .iter()
        .filter_map(|item| {
            let MixedEventItem::Event(e) = item else {
                return None;
            };
            let MixedEventKind::MidiController(c) = &e.event else {
                return None;
            };
            let bridge = c.context.as_ref()?;
            Some(Ff60ControllerBridgeProvenance {
                representation_range: c.representation_range.clone(),
                context_range: bridge.context.range.clone(),
                context_bytes: source[bridge.context.range.clone()].to_vec(),
                leading_timing_range: bridge.leading_timing.range.clone(),
                leading_timing_bytes: bridge.leading_timing.bytes.to_vec(),
                leading_interval: bridge.leading_timing.value,
                successor_timing_range: c.timing_delta.range.clone(),
                successor_timing_bytes: c.timing_delta.bytes.to_vec(),
                successor_interval: c.timing_delta.value,
                controller_position: e.position,
            })
        })
        .collect()
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct BoundedPrefixAccounting {
    pub accounted_source_range: Range<usize>,
    pub unresolved_source_range: Range<usize>,
    pub accounted_source_objects: u32,
    pub native_displayed_track_events: Option<u32>,
    pub evidence: String,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct ExportCountsView {
    pub notes: u64,
    pub controllers: u64,
    pub patches: u64,
    pub pitch_bend: u64,
    pub sysex: u64,
    pub tempo: u64,
    pub meter: u64,
}
impl From<&ExportCounts> for ExportCountsView {
    fn from(c: &ExportCounts) -> Self {
        Self {
            notes: c.notes,
            controllers: c.controllers,
            patches: c.program_changes,
            pitch_bend: c.pitch_bend,
            sysex: 0,
            tempo: c.tempo,
            meter: c.meter,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Partial120Accounting {
    pub tracks: Vec<Partial120TrackAccounting>,
    pub exported: ExportCountsView,
    /// Raw primary objects include contexts; this is NOT a count of MIDI events.
    pub unresolved_source_objects: u64,
}
fn err(e: impl std::fmt::Debug) -> ExperimentalExportError {
    ExperimentalExportError(format!("{e:?}"))
}

pub(crate) fn build(
    source: &[u8],
    ordinal: usize,
) -> Result<(ExperimentalReport, Vec<u8>), ExperimentalExportError> {
    let bridge = associate_semantic_project_120(source).map_err(err)?;
    let s = bridge
        .candidates
        .into_iter()
        .nth(ordinal)
        .ok_or_else(|| err("unknown sequence"))?
        .association
        .map_err(err)?;
    // Scope is the established bounded single-primary-only neighborhood, not a
    // fallback for failed strict decoding of arbitrary paired projects.
    if s.primary_only.len() != 1 || !s.no_event_data.is_empty() {
        return Err(err(
            "partial120 requires one bounded primary-only neighborhood",
        ));
    }
    let root = parse_root_record_stream(source).map_err(err)?;
    let verified_refusal = crate::recovery120::recover_project120(source)
        .map_err(err)?
        .into_iter()
        .find(|c| c.ordinal == ordinal)
        .ok_or_else(|| err("unknown verified object"))?
        .result
        .err()
        .ok_or_else(|| err("already verified"))?;
    let mp = &s.leading_special[0].pair.primary;
    let tp = &s.leading_special[1].pair.primary;
    let m = mp.payload.bytes;
    let t = tp.payload.bytes;
    // These checks establish event-only extent; secondary/applicability semantics
    // remain explicitly unresolved. Never waive the production conductor gate.
    if m.len() != 38
        || !matches!(&m[..2], [0, 1] | [0, 2])
        || m[2..6] != 1u32.to_be_bytes()
        || m[6..10] != [0; 4]
        || m[22..29] != [135, 255, 255, 127, 255, 47, 0]
        || m[29..].iter().any(|b| *b != 0)
    {
        return Err(err("unsupported bounded initial meter"));
    }
    if t.len() < 28 || t[..2] != [0, 1] || t[6..10] != [0; 4] || t[t.len() - 3..] != [255, 47, 0] {
        return Err(err("unsupported Tempo body envelope"));
    }
    let meter = crate::meter::decode_bounded_initial_meter(
        source,
        crate::meter::InitialMeterBounds {
            event_range: mp.payload.range.start + 14..mp.payload.range.start + 22,
        },
    )
    .map_err(err)?;
    let map = crate::tempo::decode_bounded_tempo_map(
        source,
        crate::tempo::TempoMapBounds {
            event_range: tp.payload.range.start + 14..tp.payload.range.end - 7,
            declared_count: u32::from_be_bytes(t[2..6].try_into().map_err(err)?),
        },
    )
    .map_err(err)?;
    if map.events()[0].absolute_tick != 0
        || map.events().get(1).is_some_and(|e| e.absolute_tick == 0)
    {
        return Err(err("ambiguous initial conductor ordering"));
    }
    let meter_values = (
        meter.numerator.value,
        meter.denominator_exponent.value,
        meter.third_payload.value,
        meter.fourth_payload.value,
    );
    let title = format!("{LABEL}: {}", String::from_utf8_lossy(s.source_name.bytes));
    let c = midi_export::adapt_conductor(
        title.as_bytes(),
        map.events()[0].mpqn(),
        meter_values,
        TimingPolicy::Identity480,
        MeterPolicy::KnownHistoricalOnly,
    )
    .map_err(err)?;
    if !c.warnings.is_empty() {
        return Err(err("unsupported meter adaptation"));
    }
    let mut chunks =
        vec![
            smf::serialize_tempo_map_conductor(&c.sequence_name, &map, c.time_signature, None)
                .map_err(err)?,
        ];
    let mut conductor_expected = crate::readback120::conductor_expected(&c, None);
    conductor_expected.pop();
    for e in &map.events()[1..] {
        conductor_expected.push((e.absolute_tick, 0x51, e.mpqn().to_be_bytes()[1..].to_vec()));
    }
    conductor_expected.push((map.events().last().unwrap().absolute_tick, 0x2f, vec![]));
    let mut expected = vec![conductor_expected];
    let mut reports = Vec::new();
    let mut accounting = Vec::new();
    let mut counts = c.counts.clone();
    counts.tempo = u64::try_from(map.events().len()).map_err(err)?;
    let mut unresolved = 0u64;
    let mut units: Vec<_> = s
        .ordinary
        .iter()
        .map(|t| (t.slot.ordinal, Some(t)))
        .collect();
    units.extend(s.primary_only.iter().map(|t| (t.slot.ordinal, None)));
    units.sort_by_key(|t| t.0);
    for (ordinal, paired) in units {
        let (label, primary, secondary) = if let Some(t) = paired {
            (t.slot.label.bytes, &t.pair.primary, Some(&t.pair.secondary))
        } else {
            let t = &s.primary_only[0];
            (t.slot.label.bytes, &t.primary, None)
        };
        let declared = u32::from_be_bytes(
            primary
                .payload
                .bytes
                .get(2..6)
                .ok_or_else(|| err("truncated primary"))?
                .try_into()
                .map_err(err)?,
        );
        // CCA-038 independently witnessed this exact source envelope/message at
        // local tick zero. Do NOT generalize stripping byte00 to other SysEx.
        let native_gs = paired.is_none()
            && declared == 46
            && primary.payload.bytes[14..].starts_with(&[
                0, 0xf0, 0x0b, 0, 0x41, 0x10, 0x42, 0x12, 0x40, 0, 0x7f, 0, 0x41, 0xf7,
            ]);
        if native_gs {
            // Mixed suffix is admitted only transactionally by the shared Multi
            // selector/table/program grammar, not by names or the first channel.
            let association = primary_only_association(&s, source, ordinal)?;
            let full_range = s.primary_only[0].event_range.clone();
            let mixed = crate::bounded_multi_patch::native_gs_multi_handoff(
                source,
                association,
                full_range,
            );
            let name = format!("{LABEL}: {}", String::from_utf8_lossy(label)).into_bytes();
            let payload = &primary.payload.bytes[18..28];
            let mut view = ExportCountsView {
                sysex: 1,
                ..Default::default()
            };
            let mut remaining = declared.checked_sub(1).ok_or_else(|| err("SysEx count"))?;
            let mut failure = Some(
                "Mixed suffix Patch/Controller routing not admitted; no partial suffix parse"
                    .into(),
            );
            let mut controller_targets = vec![];
            let chunk = if let Ok((events, targets)) = mixed {
                controller_targets = targets;
                let first = events
                    .iter()
                    .find_map(|e| match e.kind {
                        midi_export::DecodedExportEventKind::TargetedProgram {
                            channel, ..
                        } => Some(channel),
                        _ => None,
                    })
                    .ok_or_else(|| err("missing Multi target"))?;
                let a = midi_export::adapt_track(
                    &events,
                    Some(ChannelAssignment {
                        channel: smf::MidiChannel::new(first).map_err(err)?,
                        provenance: ChannelAssignmentProvenance::ParsedRouting,
                    }),
                    TimingPolicy::Identity480,
                    PatchPolicy::StrictKnownOnly,
                )
                .map_err(err)?;
                if !a.warnings.is_empty() || !a.untranslated_metadata.is_empty() {
                    return Err(err("unreported Multi translation"));
                }
                let mut expect = crate::readback120::musical_expected(&name, &a.scheduled_events);
                expect.insert(1, (0, 0xf0, payload.to_vec()));
                expected.push(expect);
                let chunk =
                    smf::serialize_named_sysex_prefix_track(&name, payload, &a.scheduled_events)
                        .map_err(err)?;
                view = ExportCountsView {
                    sysex: 1,
                    ..ExportCountsView::from(&a.counts)
                };
                counts.add_assign(&a.counts);
                remaining = 0;
                failure = None;
                chunk
            } else {
                expected.push(vec![
                    (0, 3, name.clone()),
                    (0, 0xf0, payload.to_vec()),
                    (0, 0x2f, vec![]),
                ]);
                smf::serialize_named_sysex_track(&name, 0, payload).map_err(err)?
            };
            chunks.push(chunk);
            unresolved = unresolved
                .checked_add(u64::from(remaining))
                .ok_or_else(|| err("count overflow"))?;
            reports.push(ExperimentalTrackReport {ordinal,name:String::from_utf8_lossy(label).into_owned(),notes:0,channel:None,pair_ranges:None,saved_mute:"Unknown; no automatic mute omission".into(),inclusion:format!("Native-confirmed GS Reset at tickzero; {} routed Patch and {} Instrument-association-targeted CC messages; {remaining} source objects unresolved. No common channel invented.",view.patches,view.controllers)});
            accounting.push(Partial120TrackAccounting {
                sysex_framing: vec![],
                controller_targets,
                ff60_controller_bridges: vec![],
                instrument_pitch_provenance: None,
                ordinal,
                primary_range: primary.record_range.clone(),
                declared_source_objects: declared,
                exported: view,
                contexts: Some(0),
                unresolved_source_objects: remaining,
                refusal: failure,
                bounded_prefix: None,
            });
            continue;
        }
        // Existing specimen spans and general current-cursor prefix recovery
        // retain their separate authority; unsupported suffixes remain refused.
        if let Some(t) = paired {
            if let Ok((walk, route, contexts, prefix)) =
                native_pipe_span(source, &root.records, &s, t)
                    .or_else(|_| coordinated_span(source, &root.records, &s, t))
                    .or_else(|_| proved_event_prefix(source, &root.records, &s, t))
            {
                let events =
                    crate::recovery120::single_handoff(&walk, Some(ordinal)).map_err(err)?;
                let events =
                    crate::controller_target120::apply_targets(&events, &route.controller_targets)
                        .map_err(err)?;
                let (events, pitch_provenance) =
                    route.transposition.handoff(&events).map_err(err)?;
                let a = midi_export::adapt_owned120_track(
                    &events,
                    Some(ChannelAssignment {
                        channel: smf::MidiChannel::new(route.channel).map_err(err)?,
                        provenance: ChannelAssignmentProvenance::ParsedRouting,
                    }),
                    TimingPolicy::Identity480,
                    PatchPolicy::StrictKnownOnly,
                )
                .map_err(err)?;
                if !a.warnings.is_empty() || !a.untranslated_metadata.is_empty() {
                    return Err(err("incomplete bounded musical translation"));
                }
                let remaining = declared
                    .checked_sub(prefix.accounted_source_objects)
                    .ok_or_else(|| err("prefix count overflow"))?;
                let scope = if remaining == 0 {
                    "RECOVERED"
                } else {
                    "PARTIAL"
                };
                let name =
                    format!("{LABEL}: {scope} {}", String::from_utf8_lossy(label)).into_bytes();
                chunks.push(
                    smf::serialize_named_musical_track_with_note_provenance(
                        &name,
                        &a.scheduled_events,
                        &a.note_provenance,
                    )
                    .map_err(err)?,
                );
                expected.push(crate::readback120::musical_expected(
                    &name,
                    &a.scheduled_events,
                ));
                counts.add_assign(&a.counts);
                unresolved = unresolved
                    .checked_add(u64::from(remaining))
                    .ok_or_else(|| err("count overflow"))?;
                reports.push(ExperimentalTrackReport { ordinal, name: String::from_utf8_lossy(label).into_owned(), notes: usize::try_from(a.counts.notes).map_err(err)?, channel: Some(route.channel), pair_ranges: Some([primary.record_range.clone(),t.pair.secondary.record_range.clone()]), saved_mute: "Unknown; no automatic mute omission".into(), inclusion: format!("Source-supported bounded span: {} Notes on routed channel {}; {} Controllers use proven Instrument-association targets with encoded selectors retained. {} raw objects unresolved after byte {}. Native displayed counts and serialized-object counts are distinguished in the ledger.",a.counts.notes,route.channel,a.counts.controllers,remaining,prefix.unresolved_source_range.start) });
                accounting.push(Partial120TrackAccounting {
                    sysex_framing: sysex_framing(&walk, source),
                    controller_targets: route.controller_targets.clone(),
                    ff60_controller_bridges: controller_bridges(source, &walk),
                    instrument_pitch_provenance: Some(pitch_provenance),
                    ordinal,
                    primary_range: primary.record_range.clone(),
                    declared_source_objects: declared,
                    exported: ExportCountsView { sysex: sysex_framing(&walk, source).len() as u64, ..ExportCountsView::from(&a.counts) },
                    contexts: Some(contexts),
                    unresolved_source_objects: remaining,
                    refusal: (remaining > 0).then(|| {
                        "Stopped before refused source object; entire suffix unresolved, no clock inferred".into()
                    }),
                    bounded_prefix: Some(prefix),
                });
                continue;
            }
        }
        let result = (|| {
            let t = paired.ok_or_else(|| {
                err("primary-only SysEx/mixed routing not admitted: whole Track unresolved")
            })?;
            let proof = prove_track(source, &root.records, &s, t).map_err(err)?;
            let (events, channel) = match proof.route() {
                TrackRoute120::NoEvents => (vec![], None),
                TrackRoute120::Single(r) => (
                    crate::recovery120::single_handoff(
                        proof.walk().ok_or_else(|| err("missing walk"))?,
                        Some(ordinal),
                    )
                    .map_err(err)?,
                    Some(r.channel),
                ),
                TrackRoute120::Multi(_) => {
                    return Err(err("mixed/Multi event routing not admitted"))
                }
            };
            let assignment = channel
                .map(|ch| {
                    smf::MidiChannel::new(ch).map(|channel| ChannelAssignment {
                        channel,
                        provenance: ChannelAssignmentProvenance::ParsedRouting,
                    })
                })
                .transpose()
                .map_err(err)?;
            let (events, pitch_provenance) = match proof.route() {
                TrackRoute120::Single(r) => {
                    let events =
                        crate::controller_target120::apply_targets(&events, &r.controller_targets)
                            .map_err(err)?;
                    let (events, provenance) = r.transposition.handoff(&events).map_err(err)?;
                    (events, Some(provenance))
                }
                _ => (events, None),
            };
            let (scheduled, provenance, counts) = if events.is_empty() {
                (vec![], vec![], ExportCounts::default())
            } else {
                let a = midi_export::adapt_owned120_track(
                    &events,
                    assignment,
                    TimingPolicy::Identity480,
                    PatchPolicy::StrictKnownOnly,
                )
                .map_err(err)?;
                if !a.warnings.is_empty() || !a.untranslated_metadata.is_empty() {
                    return Err(err("incomplete translation"));
                }
                (a.scheduled_events, a.note_provenance, a.counts)
            };
            let name = midi_export::adapt_text(label).map_err(err)?;
            let chunk = smf::serialize_named_musical_track_with_note_provenance(
                &name,
                &scheduled,
                &provenance,
            )
            .map_err(err)?;
            Ok((
                chunk,
                crate::readback120::musical_expected(&name, &scheduled),
                counts,
                channel,
                proof.context_records(),
                pitch_provenance,
                proof
                    .walk()
                    .map(|w| sysex_framing(w, source))
                    .unwrap_or_default(),
                proof
                    .walk()
                    .map(|w| controller_bridges(source, w))
                    .unwrap_or_default(),
                match proof.route() {
                    TrackRoute120::Single(r) => r.controller_targets.clone(),
                    _ => vec![],
                },
            ))
        })();
        let (track_counts, channel, contexts, refusal, pitch_provenance, sysex, bridges, targets) =
            match result {
                Ok((chunk, expect, tc, ch, ctx, pp, sysex, bridges, targets)) => {
                    chunks.push(chunk);
                    expected.push(expect);
                    counts.add_assign(&tc);
                    (tc, ch, Some(ctx), None, pp, sysex, bridges, targets)
                }
                Err(e) => {
                    let name = format!("{LABEL}: UNRESOLVED {}", String::from_utf8_lossy(label))
                        .into_bytes();
                    chunks.push(smf::serialize_named_musical_track(&name, &[]).map_err(err)?);
                    expected.push(crate::readback120::musical_expected(&name, &[]));
                    unresolved = unresolved
                        .checked_add(u64::from(declared))
                        .ok_or_else(|| err("count overflow"))?;
                    (
                        ExportCounts::default(),
                        None,
                        None,
                        Some(e.0),
                        None,
                        vec![],
                        vec![],
                        vec![],
                    )
                }
            };
        let failed = refusal.is_some();
        reports.push(ExperimentalTrackReport{ordinal,name:String::from_utf8_lossy(label).into_owned(),notes:usize::try_from(track_counts.notes).map_err(err)?,channel,pair_ranges:secondary.map(|r|[primary.record_range.clone(),r.record_range.clone()]),saved_mute:"Unknown; no automatic mute omission".into(),inclusion:if failed{format!("UNRESOLVED whole Track: {} raw source objects; {:?}",declared,refusal)}else{"All completely proved MIDI events included; opaque counted contexts retained, not translated".into()}});
        accounting.push(Partial120TrackAccounting {
            sysex_framing: sysex.clone(),
            controller_targets: targets,
            ff60_controller_bridges: bridges,
            instrument_pitch_provenance: pitch_provenance,
            ordinal,
            primary_range: primary.record_range.clone(),
            declared_source_objects: declared,
            exported: ExportCountsView {
                sysex: sysex.len() as u64,
                ..ExportCountsView::from(&track_counts)
            },
            contexts,
            unresolved_source_objects: if failed { declared } else { 0 },
            refusal,
            bounded_prefix: None,
        });
    }
    if counts.notes == 0 {
        return Err(err("no playable proved Notes"));
    }
    let mut cursor = s.sequence_range.start;
    let mut regions = Vec::new();
    for r in root.records.iter().filter(|r| {
        r.record_range.start >= s.sequence_range.start && r.record_range.end <= s.sequence_range.end
    }) {
        if r.record_range.start != cursor {
            return Err(err("unreported source bytes"));
        }
        cursor = r.record_range.end;
        regions.push(ExperimentalRegion{range:r.record_range.clone(),record_type:r.record_type.value,role:"Exact framed provenance retained; Track accounting determines rendered subset. Non-event/context/applicability semantics unresolved.".into(),sha256:format!("{:x}",Sha256::digest(&source[r.record_range.clone()]))});
    }
    if cursor != s.sequence_range.end {
        return Err(err("unreported terminal bytes"));
    }
    let sysex_count = accounting.iter().map(|t| t.exported.sysex).sum();
    let report=ExperimentalReport{label:LABEL.into(),source_sha256:format!("{:x}",Sha256::digest(source)),sequence_ordinal:ordinal,sequence_range:s.sequence_range.clone(),source_name:String::from_utf8_lossy(s.source_name.bytes).into_owned(),verified_refusal:format!("{verified_refusal:?}"),ppqn:480,observed_initial_mpqn:map.events()[0].mpqn(),observed_source_meter:meter_values,included_notes:usize::try_from(counts.notes).map_err(err)?,omitted_known_notes:0,unresolved_semantic_event_count:None,regions,selected_scope166:None,partial120:Some(Partial120Accounting{tracks:accounting,exported:ExportCountsView{sysex:sysex_count,..ExportCountsView::from(&counts)},unresolved_source_objects:unresolved}),tracks:reports,limitations:vec![
        "EXPLICITLY INCOMPLETE: failed Tracks are empty labelled placeholders in source order, NOT absent original music. Omitted Notes count is UNKNOWN; zero omitted_known_notes means no separately counted omitted Notes, not no omissions.".into(),
        "Ordinary Tracks require complete proof, except explicitly ledgered, independently validated prefixes and original-source coordinated spans. Their unsupported later contexts remain unresolved; no failed-walk suffix or guessed clock exported.".into(),
        "All bounded Tempo events and supported initial meter included. Conductor header/secondary/index applicability, contexts, terminal duration and project metadata remain unresolved; no complete conductor-envelope claim.".into(),
        "Primary-only mixed suffix included only when complete shared Multi routing/translation and raw count agree. Only exact native-confirmed GS Reset framing at zero is interpreted. Otherwise suffix omitted explicitly. No GM/GS/XG initialization or missing Program/Bank event invented; other unresolved Tracks may change timbre.".into(),
        "Raw unresolved source-object counts include contexts and are not a MIDI event count. Every framed region retained by range/hash; mute states unknown. EOT closes last rendered event/release, not historical sequence length.".into(),
    ]};
    let midi = smf::serialize_format1(480, &chunks).map_err(err)?;
    crate::readback120::verify(&midi, 480, &expected).map_err(err)?;
    Ok((report, midi))
}

fn primary_only_association(
    s: &crate::semantic_layout120::Source120SequenceAssociation<'_>,
    source: &[u8],
    ordinal: usize,
) -> Result<Range<usize>, ExperimentalExportError> {
    let t = s
        .primary_only
        .iter()
        .find(|t| t.slot.ordinal == ordinal)
        .ok_or_else(|| err("missing primary-only owner"))?;
    let start = t
        .slot
        .label
        .range
        .start
        .checked_sub(33)
        .filter(|p| *p >= s.sequence_range.start)
        .ok_or_else(|| err("association bounds"))?;
    let range = start..t.slot.label.range.start;
    source
        .get(range.clone())
        .ok_or_else(|| err("association bounds"))?;
    Ok(range)
}

/// Recover only an exact prefix already accepted by the unchanged event grammar.
/// The failing decoder cursor establishes the boundary; never scan or resume
/// beyond an unsupported context, and never infer its clock.
fn proved_event_prefix<'a>(
    source: &'a [u8],
    records: &[crate::sequence_container::FramedRecord<'a>],
    s: &crate::semantic_layout120::Source120SequenceAssociation<'a>,
    t: &crate::semantic_layout120::OrdinaryBinding<'a>,
) -> Result<
    (
        crate::mixed_event::MixedEventWalk<'a>,
        crate::authority120::SingleRoute120,
        usize,
        BoundedPrefixAccounting,
    ),
    ExperimentalExportError,
> {
    use crate::mixed_event::*;
    let body = crate::sequence_container::TrackRecordPair {
        pair_ordinal: t.local_pair_ordinal,
        primary: t.pair.primary.clone(),
        secondary: t.pair.secondary.clone(),
        candidate_event_start: t.pair.primary.payload.range.start + 14,
        event_containing_range: t.pair.primary.payload.range.start + 14
            ..t.pair.primary.payload.range.end,
    }
    .validated_event_bounds()
    .map_err(err)?
    .event_range;
    let end = match walk_bounded_120_prefix_events(
        source,
        MixedEventBounds {
            event_range: body.clone(),
        },
        Default::default(),
    ) {
        Err(MixedEventWalkError::PatchContextMismatch {
            cursor,
            observed: Some(0xb0),
            ..
        }) if cursor > body.start && cursor < body.end => cursor,
        // Unsupported SysEx framing and an incomplete terminal timing/context
        // cannot invalidate earlier independently consumed source events. Retain
        // the entire failed object and suffix as unresolved; never resume there.
        Err(MixedEventWalkError::MalformedSysEx { cursor, .. })
        | Err(MixedEventWalkError::TimingVlq { cursor, .. })
            if cursor > body.start && cursor < body.end =>
        {
            cursor
        }
        _ => return Err(err("no independently bounded refused-object prefix")),
    };
    let walk = walk_bounded_120_prefix_events(
        source,
        MixedEventBounds {
            event_range: body.start..end,
        },
        Default::default(),
    )
    .map_err(err)?;
    let mut contexts = 0usize;
    for item in &walk.items {
        match item {
            MixedEventItem::Event(e) => match &e.event {
                MixedEventKind::ContextMediatedNote(_) => contexts += 1,
                MixedEventKind::DoubleContextMediatedNote(_) => contexts += 2,
                MixedEventKind::MidiController(c) => contexts += usize::from(c.context.is_some()),
                MixedEventKind::SysEx(s) if s.framing.is_some() => {}
                MixedEventKind::SysEx(_) => return Err(err("unsupported prefix SysEx")),
                _ => {}
            },
            _ => return Err(err("unsupported prefix Patch accounting")),
        }
    }
    let declared = u32::from_be_bytes(t.pair.primary.payload.bytes[2..6].try_into().map_err(err)?);
    let accounted = walk
        .logical_event_count()
        .checked_add(contexts)
        .and_then(|n| u32::try_from(n).ok())
        .ok_or_else(|| err("prefix count overflow"))?;
    if accounted == 0 || accounted >= declared {
        return Err(err("prefix count contradicts unresolved suffix"));
    }
    let start = t
        .slot
        .label
        .range
        .start
        .checked_sub(33)
        .filter(|p| *p >= s.sequence_range.start)
        .ok_or_else(|| err("association bounds"))?;
    let association = start..t.slot.label.range.start;
    let route = crate::authority120::single_route(
        records,
        association.clone(),
        source
            .get(association)
            .ok_or_else(|| err("association bounds"))?,
        &walk,
    )
    .map_err(err)?;
    Ok((walk, route, contexts, BoundedPrefixAccounting {
        accounted_source_range: body.start..end, unresolved_source_range: end..body.end,
        accounted_source_objects: accounted, native_displayed_track_events: None,
        evidence: "Exact unchanged mixed-event grammar consumes the prefix from Track origin. Semantic ownership and existing routing independently validated. Stop at decoder-established refused-object boundary; no suffix resynchronization or clock assumption.".into(),
    }))
}

/// CCA-057 owner-reported read-only native Event List witness. Authority is
/// confined to one transition in this exact original source, not a FF60 grammar.
fn native_pipe_span<'a>(
    source: &'a [u8],
    records: &[crate::sequence_container::FramedRecord<'a>],
    s: &crate::semantic_layout120::Source120SequenceAssociation<'a>,
    t: &crate::semantic_layout120::OrdinaryBinding<'a>,
) -> Result<
    (
        crate::mixed_event::MixedEventWalk<'a>,
        crate::authority120::SingleRoute120,
        usize,
        BoundedPrefixAccounting,
    ),
    ExperimentalExportError,
> {
    use crate::mixed_event::*;
    if source.len() != 452284
        || format!("{:x}", Sha256::digest(source))
            != "22a9e2589098b0d475f6e288eeebc9303bed644aa8ce09f9971a0060c9160486"
        || s.sequence_range != (33024..105985)
        || t.slot.ordinal != 10
        || t.pair.primary.record_range != (70919..75079)
        || t.pair.secondary.record_range != (75079..75340)
        || source.get(74529..74540)
            != Some(&[0x81, 0x6c, 0xff, 0x60, 6, 0x57, 0x7f, 0, 0x60, 0x62, 0x16])
    {
        return Err(err(
            "no native Pipe transition witness for this source/binding",
        ));
    }
    let (mut walk, route, mut contexts, mut accounting) =
        proved_event_prefix(source, records, s, t)?;
    if route.channel != 5
        || accounting.accounted_source_range != (70938..74529)
        || accounting.accounted_source_objects != 1028
        || contexts != 7
        || accounting.unresolved_source_range != (74529..75072)
        || !matches!(walk.items.last(), Some(MixedEventItem::Event(e)) if e.position == 121796)
    {
        return Err(err("native Pipe prefix clock/ownership/count mismatch"));
    }
    // Include the witnessed 236-tick leading interval exactly once. The context
    // is counted separately; unchanged grammar owns all following MIDI records.
    let basis = MixedEventTimingBasis {
        previous_event_position: 121796u32
            .checked_add(
                crate::track7::decode_7bit_be_vlq(source, 74529, 75072)
                    .map_err(err)?
                    .value,
            )
            .ok_or_else(|| err("Pipe bridge clock overflow"))?,
    };
    let covered = match walk_bounded_mixed_events(
        source,
        MixedEventBounds {
            event_range: 74540..75072,
        },
        basis,
    ) {
        Ok(_) => 75072,
        Err(MixedEventWalkError::PatchContextMismatch {
            cursor,
            observed: Some(0xb0),
            ..
        })
        | Err(MixedEventWalkError::MalformedSysEx { cursor, .. })
        | Err(MixedEventWalkError::TimingVlq { cursor, .. })
            if cursor > 74540 && cursor < 75072 =>
        {
            cursor
        }
        Err(e) => return Err(err(e)),
    };
    let suffix = walk_bounded_mixed_events(
        source,
        MixedEventBounds {
            event_range: 74540..covered,
        },
        basis,
    )
    .map_err(err)?;
    let witnessed = [
        (122033, 4),
        (122040, 21),
        (122044, 46),
        (122051, 70),
        (122061, 90),
    ];
    for (position, value) in witnessed {
        if !suffix.items.iter().any(|item| matches!(item, MixedEventItem::Event(e)
            if e.position == position && matches!(&e.event, MixedEventKind::MidiController(c)
                if c.channel == 1 && c.controller_number.value == 1 && c.controller_value.value == value))) {
            return Err(err("native Pipe Controller witness mismatch"));
        }
    }
    if !suffix.items.iter().any(|item| {
        matches!(item, MixedEventItem::Event(e)
        if e.position == 122037 && matches!(&e.event, MixedEventKind::Note(n)
            if n.pitch.value == 75 && n.attack_velocity.value == 96
                && n.release_velocity.value == 102 && n.duration.value == 49))
    }) {
        return Err(err("native Pipe Note witness mismatch"));
    }
    for item in &suffix.items {
        match item {
            MixedEventItem::Event(e) => match &e.event {
                MixedEventKind::ContextMediatedNote(_) => contexts += 1,
                MixedEventKind::DoubleContextMediatedNote(_) => contexts += 2,
                MixedEventKind::SysEx(_) => {
                    return Err(err("unsupported Pipe suffix SysEx accounting"))
                }
                _ => {}
            },
            _ => return Err(err("unsupported Pipe suffix Patch accounting")),
        }
    }
    contexts += 1;
    walk.items.extend(suffix.items);
    walk.event_range = 70938..covered;
    walk.consumed_range = walk.event_range.clone();
    let accounted = u32::try_from(
        walk.logical_event_count()
            .checked_add(contexts)
            .ok_or_else(|| err("Pipe count overflow"))?,
    )
    .map_err(err)?;
    let declared = u32::from_be_bytes(t.pair.primary.payload.bytes[2..6].try_into().map_err(err)?);
    if declared != 1177 || accounted > declared || (covered == 75072 && accounted != declared) {
        return Err(err("native Pipe source-object accounting mismatch"));
    }
    let start = t
        .slot
        .label
        .range
        .start
        .checked_sub(33)
        .filter(|p| *p >= s.sequence_range.start)
        .ok_or_else(|| err("Pipe association bounds"))?;
    let association = start..t.slot.label.range.start;
    let combined_route = crate::authority120::single_route(
        records,
        association.clone(),
        source
            .get(association)
            .ok_or_else(|| err("Pipe association bounds"))?,
        &walk,
    )
    .map_err(err)?;
    if combined_route.channel != route.channel {
        return Err(err("Pipe suffix routing disagreement"));
    }
    accounting.accounted_source_range = walk.event_range.clone();
    accounting.unresolved_source_range = covered..75072;
    accounting.accounted_source_objects = accounted;
    accounting.evidence = "CCA-057 owner-reported original Sweeper read-only native Pipe Event List: CC1=4 at 64·3·113 and pitch75/attack96/release102/duration49 at 64·3·117; subsequent CC1 at 64·3·120/124/131/141. INCLUDE 236 ticks authorized only at original-source byte74529; no authority for other FF60-to-Controller contexts. Screenshot not independently inspected. Earlier prefix and unchanged suffix grammar retain provenance, channels and ordering.".into();
    Ok((walk, combined_route, contexts, accounting))
}

/// Limited to the original source and semantically owned target Tracks.
/// Initial eight-byte context form reuses CCA-053's native-confirmed composition;
/// six-byte contexts admit only zero-delta CCs closing on an explicit Note.
fn coordinated_span<'a>(
    source: &'a [u8],
    records: &[crate::sequence_container::FramedRecord<'a>],
    s: &crate::semantic_layout120::Source120SequenceAssociation<'a>,
    t: &crate::semantic_layout120::OrdinaryBinding<'a>,
) -> Result<
    (
        crate::mixed_event::MixedEventWalk<'a>,
        crate::authority120::SingleRoute120,
        usize,
        BoundedPrefixAccounting,
    ),
    ExperimentalExportError,
> {
    if t.slot.ordinal == 11 {
        return native_chord_prefix(source, records, s, t);
    }
    if source.len() != 452284
        || format!("{:x}", Sha256::digest(source))
            != "22a9e2589098b0d475f6e288eeebc9303bed644aa8ce09f9971a0060c9160486"
        || s.sequence_range != (33024..105985)
    {
        return Err(err("no original-source composition evidence"));
    }
    let (primary, secondary, body, declared, channel) = match t.slot.ordinal {
        4 => (39936..43452, 43452..43727, 39955..43445, 741u32, 2),
        7 => (43835..64563, 64563..65015, 43854..64556, 6101u32, 4),
        _ => return Err(err("outside coordinated scope")),
    };
    if t.pair.primary.record_range != primary
        || t.pair.secondary.record_range != secondary
        || t.pair.primary.payload.bytes.get(2..6) != Some(&declared.to_be_bytes())
    {
        return Err(err("target ownership/count"));
    }
    let bounds = crate::sequence_container::TrackRecordPair {
        pair_ordinal: t.local_pair_ordinal,
        primary: t.pair.primary.clone(),
        secondary: t.pair.secondary.clone(),
        candidate_event_start: body.start,
        event_containing_range: body.start..t.pair.primary.payload.range.end,
    }
    .validated_event_bounds()
    .map_err(err)?;
    if bounds.event_range != body {
        return Err(err("target event bounds"));
    }
    let (walk, extra, covered, description) = if t.slot.ordinal == 4 {
        let (after, leading) = initial_eight_context(source, body.clone())?;
        let w = crate::mixed_event::walk_bounded_mixed_events(
            source,
            crate::mixed_event::MixedEventBounds {
                event_range: after..body.end,
            },
            crate::mixed_event::MixedEventTimingBasis {
                previous_event_position: leading,
            },
        )
        .map_err(err)?;
        (w,1,body.end,"Same initial FF60/08 prefix 57 7F 00 with duplicated fields and explicit B0 successor as native-confirmed Track 7; leading and final VLQs retained once. Whole suffix consumes exactly.")
    } else {
        let (w, c, end) = walk_zero_controller_note_bridges(source, body.clone())?;
        (w,c,end,"Existing FF60/06 zero-delta Controller/Note bridges retained. Independently validated FF60/07 prefix 57 7F 00 with explicit Controller includes leading and successor intervals once, then resumes unchanged bounded grammar. Stop before unsupported length-6/nonzero bridge; no resynchronization.")
    };
    let mut contexts = extra;
    for item in &walk.items {
        match item {
            crate::mixed_event::MixedEventItem::Event(e) => match &e.event {
                crate::mixed_event::MixedEventKind::ContextMediatedNote(_) => contexts += 1,
                crate::mixed_event::MixedEventKind::DoubleContextMediatedNote(_) => contexts += 2,
                crate::mixed_event::MixedEventKind::MidiController(c) => {
                    contexts += usize::from(c.context.is_some())
                }
                crate::mixed_event::MixedEventKind::SysEx(_) => {
                    return Err(err("unsupported target SysEx accounting"))
                }
                _ => {}
            },
            _ => return Err(err("unsupported target Patch composition")),
        }
    }
    let accounted = walk
        .logical_event_count()
        .checked_add(contexts)
        .and_then(|n| u32::try_from(n).ok())
        .ok_or_else(|| err("count overflow"))?;
    if accounted > declared || (covered == body.end && accounted != declared) {
        return Err(err("target source-count mismatch"));
    }
    let a = t
        .slot
        .label
        .range
        .start
        .checked_sub(33)
        .ok_or_else(|| err("association bounds"))?..t.slot.label.range.start;
    if a.start < s.sequence_range.start {
        return Err(err("association bounds"));
    }
    let route = crate::authority120::single_route(
        records,
        a.clone(),
        source.get(a).ok_or_else(|| err("association bounds"))?,
        &walk,
    )
    .map_err(err)?;
    if route.channel != channel {
        return Err(err("routing disagreement"));
    }
    Ok((
        walk,
        route,
        contexts,
        BoundedPrefixAccounting {
            accounted_source_range: body.start..covered,
            unresolved_source_range: covered..body.end,
            accounted_source_objects: accounted,
            native_displayed_track_events: None,
            evidence: description.into(),
        },
    ))
}

fn initial_eight_context(
    source: &[u8],
    body: Range<usize>,
) -> Result<(usize, u32), ExperimentalExportError> {
    if body.start > body.end || body.end > source.len() {
        return Err(err("invalid initial context bounds"));
    }
    let leading = crate::track7::decode_7bit_be_vlq(source, body.start, body.end).map_err(err)?;
    let c = body
        .start
        .checked_add(leading.bytes_consumed)
        .ok_or_else(|| err("bounds"))?;
    let end = c
        .checked_add(11)
        .filter(|n| *n <= body.end)
        .ok_or_else(|| err("truncated initial context"))?;
    let b = source.get(c..end).ok_or_else(|| err("context bounds"))?;
    if b[..6] != [0xff, 0x60, 8, 0x57, 0x7f, 0] || b[6] != b[7] {
        return Err(err("unwitnessed initial context shape"));
    }
    let final_t = crate::track7::decode_7bit_be_vlq(source, end, body.end).map_err(err)?;
    let status = end
        .checked_add(final_t.bytes_consumed)
        .filter(|p| *p < body.end)
        .ok_or_else(|| err("missing bounded initial status"))?;
    if source.get(status) != Some(&0xb0) {
        return Err(err("initial context successor"));
    }
    Ok((end, leading.value))
}

/// Replays only exact prefix bounds returned by the existing current-cursor
/// refusal. No signature scan or restart after unknown bytes. Unsupported
/// bridges terminate the covered span, with their entire suffix unresolved.
fn walk_zero_controller_note_bridges(
    source: &[u8],
    body: Range<usize>,
) -> Result<(crate::mixed_event::MixedEventWalk<'_>, usize, usize), ExperimentalExportError> {
    use crate::mixed_event::*;
    let mut cursor = body.start;
    let mut previous = 0;
    let mut items = Vec::new();
    let mut contexts = 0;
    while cursor < body.end {
        let prefix_end = match walk_bounded_mixed_events(
            source,
            MixedEventBounds {
                event_range: cursor..body.end,
            },
            MixedEventTimingBasis {
                previous_event_position: previous,
            },
        ) {
            Ok(w) => {
                items.extend(w.items);
                cursor = body.end;
                break;
            }
            Err(MixedEventWalkError::PatchContextMismatch {
                cursor: at,
                observed: Some(0xb0),
                ..
            }) if at >= cursor => at,
            Err(_) => break,
        };
        if prefix_end > cursor {
            let w = walk_bounded_mixed_events(
                source,
                MixedEventBounds {
                    event_range: cursor..prefix_end,
                },
                MixedEventTimingBasis {
                    previous_event_position: previous,
                },
            )
            .map_err(err)?;
            previous = match w.items.last() {
                Some(MixedEventItem::Event(e)) => e.position,
                _ => return Err(err("unsupported prefix timing owner")),
            };
            items.extend(w.items);
            cursor = prefix_end;
        }
        match zero_controller_note_bridge(source, cursor, body.end, previous) {
            Ok((w, next, position)) => {
                items.extend(w.items);
                cursor = next;
                previous = position;
                contexts += 1;
            }
            Err(_) => break,
        }
    }
    if items.is_empty() {
        return Err(err("no bounded target events"));
    }
    Ok((
        crate::mixed_event::MixedEventWalk {
            event_range: body.start..cursor,
            consumed_range: body.start..cursor,
            items,
        },
        contexts,
        cursor,
    ))
}

/// Equivalent existing timed context-to-Note form with only intervening
/// same-position Controllers. Final Note anchors the context's existing clock.
fn zero_controller_note_bridge(
    source: &[u8],
    start: usize,
    end: usize,
    previous: u32,
) -> Result<(crate::mixed_event::MixedEventWalk<'_>, usize, u32), ExperimentalExportError> {
    use crate::mixed_event::*;
    if start > end || end > source.len() {
        return Err(err("invalid bridge bounds"));
    }
    let lead = crate::track7::decode_7bit_be_vlq(source, start, end).map_err(err)?;
    let c = start
        .checked_add(lead.bytes_consumed)
        .ok_or_else(|| err("bounds"))?;
    let after = c
        .checked_add(9)
        .filter(|n| *n <= end)
        .ok_or_else(|| err("truncated bridge context"))?;
    let raw = source.get(c..after).ok_or_else(|| err("bounds"))?;
    if raw[..6] != [0xff, 0x60, 6, 0x57, 0x7f, 0] {
        return Err(err("unsupported bridge context"));
    }
    let position = previous
        .checked_add(lead.value)
        .ok_or_else(|| err("bridge clock overflow"))?;
    let mut cursor = after;
    let mut count = 0;
    loop {
        let delta = crate::track7::decode_7bit_be_vlq(source, cursor, end).map_err(err)?;
        if delta.value != 0 {
            return Err(err("nonzero Controller/Note bridge timing unproven"));
        }
        let status = cursor
            .checked_add(delta.bytes_consumed)
            .filter(|n| *n < end)
            .ok_or_else(|| err("missing bridge status"))?;
        if count > 0 && source[status] == 0x90 {
            break;
        }
        if count == 2 {
            return Err(err("too many bridge Controllers"));
        }
        let data = if count == 0 {
            if source[status] != 0xb0 {
                return Err(err("explicit bridge Controller required"));
            }
            status + 1
        } else {
            if source[status] > 127 {
                return Err(err("compact bridge Controller required"));
            }
            status
        };
        cursor = data
            .checked_add(2)
            .filter(|n| *n <= end)
            .ok_or_else(|| err("truncated bridge Controller"))?;
        if source[data..cursor].iter().any(|b| *b > 127) {
            return Err(err("invalid bridge data"));
        }
        count += 1;
    }
    let (note, next) = crate::track7::decode_note_at(source, cursor, end, true).map_err(err)?;
    if note.timing.value != 0 {
        return Err(err("nonzero closing Note"));
    }
    let mut w = walk_bounded_mixed_events(
        source,
        MixedEventBounds {
            event_range: after..cursor,
        },
        MixedEventTimingBasis {
            previous_event_position: position,
        },
    )
    .map_err(err)?;
    let n = walk_bounded_mixed_events(
        source,
        MixedEventBounds {
            event_range: cursor..next,
        },
        MixedEventTimingBasis {
            previous_event_position: position,
        },
    )
    .map_err(err)?;
    w.items.extend(n.items);
    w.event_range = start..next;
    w.consumed_range = start..next;
    Ok((w, next, position))
}

/// CCA-053 native witness, not a reusable FF60-to-Controller grammar claim.
/// Exact hash/ranges prevent this observation granting authority to other files.
fn native_chord_prefix<'a>(
    source: &'a [u8],
    records: &[crate::sequence_container::FramedRecord<'a>],
    s: &crate::semantic_layout120::Source120SequenceAssociation<'a>,
    t: &crate::semantic_layout120::OrdinaryBinding<'a>,
) -> Result<
    (
        crate::mixed_event::MixedEventWalk<'a>,
        crate::authority120::SingleRoute120,
        usize,
        BoundedPrefixAccounting,
    ),
    ExperimentalExportError,
> {
    use crate::mixed_event::{
        walk_bounded_mixed_events, MixedEventBounds, MixedEventItem, MixedEventKind,
        MixedEventTimingBasis,
    };
    if source.len() != 452284
        || t.slot.ordinal != 11
        || t.pair.primary.record_range != (75340..81809)
        || t.pair.secondary.record_range != (81809..82173)
        || s.sequence_range != (33024..105985)
        || format!("{:x}", Sha256::digest(source))
            != "22a9e2589098b0d475f6e288eeebc9303bed644aa8ce09f9971a0060c9160486"
    {
        return Err(err(
            "no native chord timing witness for this source/binding",
        ));
    }
    let bounds = crate::sequence_container::TrackRecordPair {
        pair_ordinal: t.local_pair_ordinal,
        primary: t.pair.primary.clone(),
        secondary: t.pair.secondary.clone(),
        candidate_event_start: 75359,
        event_containing_range: 75359..81809,
    }
    .validated_event_bounds()
    .map_err(err)?;
    if bounds.event_range != (75359..81802)
        || t.pair.primary.payload.bytes[2..6] != 2025u32.to_be_bytes()
        || source.get(75359..75372)
            != Some(&[
                0x88, 2, 0xff, 0x60, 8, 0x57, 0x7f, 0, 0x5c, 0x5c, 0x81, 0xaf, 0x0a,
            ])
        || source.get(79617..79629)
            != Some(&[6, 0xff, 0x60, 6, 0x57, 0x7f, 0, 0x7c, 0x7e, 0x60, 16, 0xb0])
    {
        return Err(err("native prefix bounds/context mismatch"));
    }
    let walk = walk_bounded_mixed_events(
        source,
        MixedEventBounds {
            event_range: 75372..79617,
        },
        MixedEventTimingBasis {
            previous_event_position: 1026,
        },
    )
    .map_err(err)?;
    // The 13 preceding bytes own the initial timed opaque context. The walker
    // consumes the independently established prefix extent exactly, not a scan
    // or truncation of an error result. Later contexts are never attempted here.
    if walk.event_range != (75372..79617) || walk.consumed_range != walk.event_range {
        return Err(err("prefix consumption"));
    }
    let mut contexts = 1usize;
    let mut notes = Vec::new();
    let mut controllers = 0usize;
    for item in &walk.items {
        match item {
            MixedEventItem::Event(e) => match &e.event {
                MixedEventKind::Note(n) => notes.push((
                    e.position,
                    n.pitch.value,
                    n.attack_velocity.value,
                    n.duration.value,
                )),
                MixedEventKind::ContextMediatedNote(n) => {
                    contexts = contexts
                        .checked_add(1)
                        .ok_or_else(|| err("context overflow"))?;
                    notes.push((
                        e.position,
                        n.note.pitch.value,
                        n.note.attack_velocity.value,
                        n.note.duration.value,
                    ));
                }
                MixedEventKind::MidiController(c) if c.channel == 1 => {
                    controllers += 1;
                }
                _ => return Err(err("unsupported prefix event family/channel")),
            },
            _ => return Err(err("unsupported prefix item")),
        }
    }
    // Independent native positions/pitches/attack, source-derived full durations.
    // Transcribed 231/216/219 are not substituted for the serialized durations.
    if notes.get(..3)
        != Some(&[
            (3840, 58, 77, 3591),
            (3842, 61, 77, 3576),
            (3843, 66, 77, 3579),
        ])
        || notes.len() != 55
        || controllers != 1287
        || contexts != 2
    {
        return Err(err("native prefix accounting/witness disagreement"));
    }
    let label_start = t.slot.label.range.start;
    let association = label_start
        .checked_sub(33)
        .ok_or_else(|| err("association bounds"))?..label_start;
    if association.start < s.sequence_range.start {
        return Err(err("association outside sequence"));
    }
    let route = crate::authority120::single_route(
        records,
        association.clone(),
        source
            .get(association)
            .ok_or_else(|| err("association bounds"))?,
        &walk,
    )
    .map_err(err)?;
    if route.channel != 6 {
        return Err(err("native Note routing disagreement"));
    }
    let accounted = walk
        .logical_event_count()
        .checked_add(contexts)
        .and_then(|n| u32::try_from(n).ok())
        .ok_or_else(|| err("prefix count overflow"))?;
    Ok((walk,route,contexts,BoundedPrefixAccounting { accounted_source_range: 75359..79617, unresolved_source_range: 79617..81802, accounted_source_objects: accounted, native_displayed_track_events: Some(2020), evidence: "CCA-053 owner-supplied native Event List: Pan at 2·3·0; Notes 3·1·0/2/3, pitches 58/61/66, attack 77. Screenshots not independently inspected. Initial context interval included only in this original-source bounded span; full serialized durations retained.".into() }))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore = "requires original authentic COMIC BOOK"]
    fn authentic_pipe_witness_is_source_and_transition_specific() {
        let source =
            std::fs::read(std::env::var("PHOENIX_COMIC_ORIGINAL_SOURCE").unwrap()).unwrap();
        let bridge = associate_semantic_project_120(&source).unwrap();
        let s = bridge.candidates[5].association.as_ref().unwrap();
        let root = parse_root_record_stream(&source).unwrap();
        let pipe = s.ordinary.iter().find(|t| t.slot.ordinal == 10).unwrap();
        let (walk, route, contexts, a) = native_pipe_span(&source, &root.records, s, pipe).unwrap();
        assert_eq!(route.channel, 5);
        assert_eq!(contexts, 11);
        assert_eq!(walk.logical_event_count(), 1166);
        assert_eq!(a.accounted_source_objects, 1177);
        assert_eq!(a.unresolved_source_range, 75072..75072);
        // The strict decoder and earlier zero-only bridge remain refusing.
        assert!(crate::mixed_event::walk_bounded_mixed_events(
            &source,
            crate::mixed_event::MixedEventBounds {
                event_range: 70938..75072
            },
            Default::default()
        )
        .is_err());
        assert!(zero_controller_note_bridge(&source, 74529, 75072, 121796).is_err());
        for ordinal in [7, 11] {
            let t = s
                .ordinary
                .iter()
                .find(|t| t.slot.ordinal == ordinal)
                .unwrap();
            assert!(native_pipe_span(&source, &root.records, s, t).is_err());
        }
        for offset in [70926, 74529, 74534, 74541, 74548] {
            let mut altered = source.clone();
            altered[offset] ^= 1;
            assert!(native_pipe_span(&altered, &root.records, s, pipe).is_err());
        }
    }
    #[test]
    fn no_partial_scope_from_arbitrary_bytes_or_paired_project() {
        for b in [vec![], vec![0; 40]] {
            assert!(build(&b, 0).is_err());
        }
        let b = crate::recovery120::tests::fixture(&[0, 60, 90, 45, 10], 1);
        assert!(build(&b, 0).is_err());
    }
    #[test]
    fn zero_controller_bridge_uses_existing_note_clock_and_preserves_status() {
        let b = [
            3, 0xff, 0x60, 6, 0x57, 0x7f, 0, 1, 2, 3, 0, 0xb0, 99, 1, 0, 98, 32, 0, 0x90, 42, 127,
            102, 41,
        ];
        let (w, next, position) = zero_controller_note_bridge(&b, 0, b.len(), 17).unwrap();
        assert_eq!((next, position, w.logical_event_count()), (b.len(), 20, 3));
        let e = crate::recovery120::single_handoff(&w, None).unwrap();
        assert!(e.iter().all(|e| e.absolute_position == 20));
        let crate::mixed_event::MixedEventItem::Event(n) = &w.items[2] else {
            panic!("Note")
        };
        let crate::mixed_event::MixedEventKind::Note(n) = &n.event else {
            panic!("Note")
        };
        assert_eq!(
            (
                n.pitch.value,
                n.attack_velocity.value,
                n.release_velocity.value,
                n.duration.value
            ),
            (42, 127, 102, 41)
        );
        assert_eq!(w.event_range, 0..b.len());
        // The ordinary production walker still refuses this new composition.
        assert!(crate::mixed_event::walk_bounded_mixed_events(
            &b,
            crate::mixed_event::MixedEventBounds {
                event_range: 0..b.len()
            },
            Default::default()
        )
        .is_err());
        for offset in [2, 3, 4, 10, 14, 17, 18] {
            let mut bad = b;
            bad[offset] ^= 1;
            assert!(
                zero_controller_note_bridge(&bad, 0, bad.len(), 17).is_err(),
                "offset {offset}"
            );
        }
        let mut high_bit = b;
        high_bit[19] = 128;
        assert!(zero_controller_note_bridge(&high_bit, 0, high_bit.len(), 17).is_err());
        let mut another_pitch = b;
        another_pitch[19] = 43;
        assert!(zero_controller_note_bridge(&another_pitch, 0, another_pitch.len(), 17).is_ok());
        for end in 0..b.len() {
            assert!(zero_controller_note_bridge(&b, 0, end, 17).is_err());
        }
        assert!(zero_controller_note_bridge(&b, 0, b.len() + 1, 17).is_err());
        assert!(zero_controller_note_bridge(&b, 0, b.len(), u32::MAX).is_err());
    }
    #[test]
    fn initial_eight_context_bounds_and_profile_are_strict() {
        let b = [
            0x88, 2, 0xff, 0x60, 8, 0x57, 0x7f, 0, 0x5c, 0x5c, 0x81, 0xaf, 10, 0x8e, 0x3e, 0xb0,
            10, 60,
        ];
        assert_eq!(initial_eight_context(&b, 0..b.len()).unwrap(), (13, 1026));
        for offset in [3, 4, 5, 6, 7, 8, 15] {
            let mut bad = b;
            bad[offset] ^= 1;
            assert!(initial_eight_context(&bad, 0..bad.len()).is_err());
        }
        for end in 0..16 {
            assert!(initial_eight_context(&b, 0..end).is_err());
        }
    }
    #[test]
    fn variable_sysex_payload_validation_and_order() {
        let p = [0x41, 0x10, 0x42, 0x12, 0x40, 0, 0x7f, 0, 0x41, 0xf7];
        let event = smf::ScheduledEvent {
            absolute_tick: 70,
            stable_ordinal: 1,
            message: smf::ChannelMessage::ProgramChange {
                channel: smf::MidiChannel::new(7).unwrap(),
                program: smf::MidiDataByte::new(122).unwrap(),
            },
        };
        let out =
            smf::serialize_named_sysex_prefix_track(b"multi", &p, std::slice::from_ref(&event))
                .unwrap();
        let midi = smf::serialize_format1(480, &[out]).unwrap();
        let expected = vec![
            (0, 3, b"multi".to_vec()),
            (0, 0xf0, p.to_vec()),
            (70, 0xc6, vec![122]),
            (70, 0x2f, vec![]),
        ];
        crate::readback120::verify(&midi, 480, std::slice::from_ref(&expected)).unwrap();
        let mut altered = expected;
        altered[1].2[0] = 0x42;
        assert!(crate::readback120::verify(&midi, 480, &[altered]).is_err());
        for bad in [
            vec![],
            vec![0xf7],
            vec![0x41, 0],
            vec![128, 0xf7],
            vec![0xf0, 0xf7],
        ] {
            assert!(smf::serialize_named_sysex_track(b"x", 0, &bad).is_err());
        }
        assert!(smf::serialize_named_sysex_track(b"x", 0x10000000, &p).is_err());
        // Existing narrow SysEx constructor has not been broadened.
        assert!(smf::BoundedSysExPayload::new(&p).is_err());
    }
    #[test]
    #[ignore = "requires original authentic COMIC BOOK"]
    fn authentic_ending_prefixes_keep_failed_objects_unresolved() {
        let bytes = std::fs::read(std::env::var("PHOENIX_COMIC_ORIGINAL_SOURCE").unwrap()).unwrap();
        for (ordinal, expected, remaining, count_offset, accounted) in [
            (17, 98334..98578, 98578..100653, 98322, 69u32),
            (22, 104090..105015, 105015..105025, 104078, 153u32),
        ] {
            let check = |b: &[u8]| {
                let bridge = associate_semantic_project_120(b).unwrap();
                let s = bridge
                    .candidates
                    .into_iter()
                    .nth(5)
                    .unwrap()
                    .association
                    .unwrap();
                let t = s
                    .ordinary
                    .iter()
                    .find(|t| t.slot.ordinal == ordinal)
                    .unwrap();
                let root = parse_root_record_stream(b).unwrap();
                proved_event_prefix(b, &root.records, &s, t).map(|(_, _, _, p)| p)
            };
            let p = check(&bytes).unwrap();
            assert_eq!(p.accounted_source_range, expected);
            assert_eq!(p.unresolved_source_range, remaining);
            assert_eq!(p.accounted_source_objects, accounted);
            let mut bad = bytes.clone();
            bad[count_offset..count_offset + 4].copy_from_slice(&accounted.to_be_bytes());
            assert!(check(&bad).is_err()); // failed source object cannot disappear from ledger
        }
    }
    #[test]
    #[ignore = "requires original authentic COMIC BOOK"]
    fn authentic_prefix_requires_count_bounds_and_routing() {
        let b = std::fs::read(std::env::var("PHOENIX_COMIC_ORIGINAL_SOURCE").unwrap()).unwrap();
        let check = |bytes: &[u8]| {
            let bridge = associate_semantic_project_120(bytes).unwrap();
            let s = bridge
                .candidates
                .into_iter()
                .nth(5)
                .unwrap()
                .association
                .unwrap();
            let t = s.ordinary.iter().find(|t| t.slot.ordinal == 10).unwrap();
            let records = parse_root_record_stream(bytes).unwrap();
            proved_event_prefix(bytes, &records.records, &s, t)
                .map(|(_, route, contexts, p)| (route.channel, contexts, p))
        };
        let (channel, contexts, p) = check(&b).unwrap();
        assert_eq!((channel, contexts), (5, 7));
        assert_eq!(p.accounted_source_range, 70938..74529);
        assert_eq!(p.accounted_source_objects, 1028);
        let mut bad = b.clone();
        bad[70926..70930].copy_from_slice(&1028u32.to_be_bytes());
        assert!(check(&bad).is_err());
        let mut bad = b.clone();
        bad[70938..70942].fill(0xff); // malformed timing/status, not a recoverable boundary
        assert!(check(&bad).is_err());
        let mut bad = b.clone();
        bad[70938] = 0xff; // still a legal timing VLQ: structural rule accepts it
        assert!(check(&bad).is_ok());
        let bridge = associate_semantic_project_120(&b).unwrap();
        let s = bridge
            .candidates
            .into_iter()
            .nth(5)
            .unwrap()
            .association
            .unwrap();
        let label = s
            .ordinary
            .iter()
            .find(|t| t.slot.ordinal == 10)
            .unwrap()
            .slot
            .label
            .range
            .start;
        let mut bad = b.clone();
        bad[label - 33] = 0; // invalid routing association shape
        assert!(check(&bad).is_err());
    }
    #[test]
    #[ignore = "requires original authentic COMIC BOOK"]
    fn authentic_negative_forms_do_not_gain_event_authority() {
        let b = std::fs::read(std::env::var("PHOENIX_COMIC_ORIGINAL_SOURCE").unwrap()).unwrap();
        for offset in [36241, 36254, 36321, 36334, 36545] {
            let mut bad = b.clone();
            bad[offset] ^= 128;
            assert!(build(&bad, 5).is_err(), "offset{offset}");
        }
        // A native witness does not transfer to malformed, changed or truncated
        // prefix contexts, later bounds, timing, count or Note properties.
        for offset in [75359, 75363, 75347, 75561, 79620, 34434] {
            let mut bad = b.clone();
            bad[offset] ^= 1;
            let (r, _) = build(&bad, 5).unwrap();
            let a = r.partial120.unwrap();
            let t = a.tracks.iter().find(|t| t.ordinal == 11).unwrap();
            assert!(t.bounded_prefix.is_none(), "offset {offset}");
            assert_eq!(t.exported.notes, 0);
            assert_eq!(t.unresolved_source_objects, t.declared_source_objects);
        }
        // Malformed counted ordinary Track is omitted transactionally, not partly
        // recovered; source objects and reason remain in the ledger.
        let mut bad = b.clone();
        bad[82180] ^= 1; // Track8 count field
        let (r, _) = build(&bad, 5).unwrap();
        let t = r
            .partial120
            .unwrap()
            .tracks
            .into_iter()
            .find(|t| t.ordinal == 12)
            .unwrap();
        assert!(t.refusal.is_some());
        assert_eq!(t.exported.notes, 0);
        assert!(t.unresolved_source_objects > 0);
        let root = crate::sequence_container::parse_root_record_stream(&b).unwrap();
        let row = root
            .records
            .iter()
            .find(|r| r.record_type.value == 0x10)
            .unwrap();
        let mut malformed = b.clone();
        malformed[row.payload.range.start + 25] ^= 1;
        assert!(crate::bounded_multi_patch::native_gs_multi_handoff(
            &malformed,
            34192..34225,
            70377..70912
        )
        .is_err());
        let mut mismatch = b.clone();
        mismatch[70368] ^= 1;
        assert!(crate::bounded_multi_patch::native_gs_multi_handoff(
            &mismatch,
            34192..34225,
            70377..70912
        )
        .is_err());
        // Native-only interpretation is not applied to a changed message.
        let mut bad = b.clone();
        bad[70382] ^= 1;
        let (r, _) = build(&bad, 5).unwrap();
        let accounting = r.partial120.unwrap();
        assert_eq!(accounting.exported.sysex, 3); // independently proved Seashore messages remain
        assert_eq!(
            accounting
                .tracks
                .iter()
                .find(|t| t.ordinal == 9)
                .unwrap()
                .exported
                .sysex,
            0
        );
        assert!(crate::bounded_multi_patch::native_gs_multi_handoff(
            &bad,
            34192..34225,
            70377..70912
        )
        .is_err());
    }
}
