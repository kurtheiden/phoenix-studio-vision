//! Pure, fail-closed construction evidence. This module grants no export authority.
//!
//! Only the Descriptor166, initial-only conductor, unmuted nonempty Note/CC0
//! Patch subset is admitted. Accepted values have private construction and are
//! independent of compatibility profiles, paths, identities and application policy.
use std::{fmt, ops::Range};

use crate::{
    bounded_patch_translation::classify_bounded_cc0_patches,
    bounded_routing::BoundedRoutingResolution,
    meter::{decode_bounded_initial_meter, InitialMeterBounds, InitialMeterEvent},
    midi_export::{
        self, ChannelAssignment, ChannelAssignmentProvenance, DecodedExportEvent, MeterPolicy,
        PatchPolicy, TimingPolicy,
    },
    mixed_event::{walk_bounded_mixed_events, MixedEventBounds, MixedEventItem, MixedEventKind},
    multitrack_export::{MultitrackExportReport, MultitrackExportResult, MusicalTrackExportReport},
    saved_mute::{collect_saved_mute_evidence, SavedMuteEvidence, SavedMuteState},
    sequence_container::{parse_project_166, FramedRecord, TrackAssociations},
    smf::{self, MidiChannel},
    tempo::{decode_bounded_initial_tempo, InitialTempoBounds, InitialTempoEvent},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ManifestRefusal {
    pub stage: &'static str,
    pub detail: String,
}
impl fmt::Display for ManifestRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.stage, self.detail)
    }
}
impl std::error::Error for ManifestRefusal {}
fn refusal(stage: &'static str, detail: impl fmt::Debug) -> ManifestRefusal {
    ManifestRefusal {
        stage,
        detail: format!("{detail:?}"),
    }
}

/// Immutable proof of complete initial-only conductor coverage. The ranges
/// include both primary and secondary records, not just the decoded slices.
#[derive(Clone, Debug)]
pub struct BoundedConductorManifest {
    tempo: InitialTempoEvent,
    meter: InitialMeterEvent,
    record_ranges: [Range<usize>; 4],
}
impl BoundedConductorManifest {
    pub fn tempo(&self) -> &InitialTempoEvent {
        &self.tempo
    }
    pub fn meter(&self) -> &InitialMeterEvent {
        &self.meter
    }
    pub fn record_ranges(&self) -> &[Range<usize>; 4] {
        &self.record_ranges
    }
}

#[derive(Clone, Debug)]
pub struct BoundedTrackManifest {
    source_ordinal: usize,
    descriptor_ordinal: usize,
    routing: BoundedRoutingResolution,
    mute: SavedMuteEvidence,
    secondary_range: Range<usize>,
    events: Vec<DecodedExportEvent>,
}
impl BoundedTrackManifest {
    pub fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }
    pub fn descriptor_ordinal(&self) -> usize {
        self.descriptor_ordinal
    }
    pub fn routing(&self) -> &BoundedRoutingResolution {
        &self.routing
    }
    pub fn saved_mute(&self) -> &SavedMuteEvidence {
        &self.mute
    }
    pub fn secondary_range(&self) -> Range<usize> {
        self.secondary_range.clone()
    }
    pub fn events(&self) -> &[DecodedExportEvent] {
        &self.events
    }
}

/// Construction capability, deliberately not convertible to an application
/// export capability. No mutable access or caller-constructed proof is exposed.
#[derive(Clone, Debug)]
pub struct BoundedSequenceManifest {
    sequence_range: Range<usize>,
    name: Vec<u8>,
    ppqn: u16,
    conductor: BoundedConductorManifest,
    tracks: Vec<BoundedTrackManifest>,
}
impl BoundedSequenceManifest {
    pub fn sequence_range(&self) -> Range<usize> {
        self.sequence_range.clone()
    }
    pub fn name(&self) -> &[u8] {
        &self.name
    }
    pub fn ppqn(&self) -> u16 {
        self.ppqn
    }
    pub fn conductor(&self) -> &BoundedConductorManifest {
        &self.conductor
    }
    pub fn tracks(&self) -> &[BoundedTrackManifest] {
        &self.tracks
    }
}

// Exact bounded initial-only record envelope. Bytes 10..14 in the primary
// are opaque prefix context, not interpreted as a position or event.
// Secondary value copies must agree, including the full surrounding framing.
fn initial_only(
    primary: &FramedRecord<'_>,
    secondary: &FramedRecord<'_>,
    event: &[u8],
) -> Result<(), ManifestRefusal> {
    let p = primary.payload.bytes;
    let s = secondary.payload.bytes;
    let copy = &event[2..3];
    let mut value = copy.to_vec();
    value.extend_from_slice(&event[4..]);
    let size = value.len() + 35;
    let mut expected = vec![0, 1, 0, 0, 0, 0];
    expected.extend_from_slice(&((size - 13) as u32).to_be_bytes());
    expected.extend_from_slice(&((size - 13) as u32).to_be_bytes());
    // The observed length fields start at +6 and +10; the entry count at +14.
    expected.extend_from_slice(&1u32.to_be_bytes());
    expected.extend_from_slice(&((size - 28) as u32).to_be_bytes());
    expected.extend_from_slice(&[0, 255, 255, 255, 47, 197, 0, 0, 0, 0]);
    expected.extend(value);
    expected.extend([255; 3]);
    let end = 14 + event.len();
    if p.len() != 38
        || p[..10] != [0, 1, 0, 0, 0, 1, 0, 0, 0, 0]
        || p[14..end] != *event
        || p[end..end + 7] != [135, 255, 255, 127, 255, 47, 0]
        || p[end + 7..].iter().any(|b| *b != 0)
        || s != expected
    {
        return Err(refusal(
            "conductor coverage",
            "unsupported initial-only record envelope or contradictory secondary copy",
        ));
    }
    Ok(())
}

/// Reparse one immutable source, selecting a sequence by structural ordinal.
/// Identity480 is the established timing contract, not a decoded source PPQN
/// field. A different requested division is refused, never silently rescaled.
pub fn build_bounded_sequence_manifest(
    bytes: &[u8],
    sequence_ordinal: usize,
    division: u16,
) -> Result<BoundedSequenceManifest, ManifestRefusal> {
    if division != midi_export::IDENTITY_480_PPQN {
        return Err(refusal("division", division));
    }
    let project = parse_project_166(bytes).map_err(|e| refusal("structure", e))?;
    let sequence = project
        .sequences
        .get(sequence_ordinal)
        .ok_or_else(|| refusal("sequence", sequence_ordinal))?;
    let TrackAssociations::Ordinal(bindings) = &sequence.track_associations else {
        return Err(refusal("ownership", &sequence.track_associations));
    };
    if bindings.is_empty()
        || bindings.len() != sequence.track_pairs.len()
        || bindings.len() != sequence.track_descriptors().len()
        || bindings
            .iter()
            .enumerate()
            .any(|(i, b)| b.pair_ordinal != i || b.descriptor_ordinal != i + 2)
    {
        return Err(refusal("ownership", "nonexhaustive ordinal binding"));
    }
    if !sequence.prelude_records.is_empty() {
        return Err(refusal("conductor prelude", "unsupported records"));
    }
    if sequence.terminal_record.payload.bytes != [255; 8] {
        return Err(refusal("sequence terminal", "unsupported content"));
    }
    let tempo = decode_bounded_initial_tempo(
        bytes,
        InitialTempoBounds {
            event_range: sequence.initial_tempo_range.clone(),
        },
    )
    .map_err(|e| refusal("tempo", e))?;
    let meter = decode_bounded_initial_meter(
        bytes,
        InitialMeterBounds {
            event_range: sequence.initial_meter_range.clone(),
        },
    )
    .map_err(|e| refusal("meter", e))?;
    initial_only(
        &sequence.tempo_primary,
        &sequence.tempo_secondary,
        &bytes[tempo.event_range.clone()],
    )?;
    initial_only(
        &sequence.meter_primary,
        &sequence.meter_secondary,
        &bytes[meter.event_range.clone()],
    )?;
    let conductor = BoundedConductorManifest {
        tempo,
        meter,
        record_ranges: [
            sequence.meter_primary.record_range.clone(),
            sequence.meter_secondary.record_range.clone(),
            sequence.tempo_primary.record_range.clone(),
            sequence.tempo_secondary.record_range.clone(),
        ],
    };
    let mutes = collect_saved_mute_evidence(bytes).map_err(|e| refusal("mute", e))?;
    let classified = classify_bounded_cc0_patches(bytes).map_err(|e| refusal("routing", e))?;
    let mut tracks = Vec::new();
    for binding in bindings {
        let mute = mutes
            .iter()
            .find(|r| {
                r.sequence_ordinal == sequence_ordinal
                    && r.descriptor_ordinal == binding.descriptor_ordinal
            })
            .ok_or_else(|| refusal("mute", "missing evidence"))?;
        if mute.state != SavedMuteState::Off {
            return Err(refusal("mute", mute.state));
        }
        let classified = classified
            .iter()
            .find(|r| {
                r.structural_ordinal == sequence_ordinal
                    && r.descriptor_ordinal == binding.descriptor_ordinal
            })
            .ok_or_else(|| refusal("routing", "missing evidence"))?
            .result
            .as_ref()
            .map_err(|e| refusal("routing", e))?;
        let routing = &classified.routing;
        let walk = walk_bounded_mixed_events(
            bytes,
            MixedEventBounds {
                event_range: routing.event_range.clone(),
            },
            Default::default(),
        )
        .map_err(|e| refusal("event walk", e))?;
        if walk.consumed_range != routing.event_range || walk.items.is_empty() {
            return Err(refusal("event walk", "incomplete or empty"));
        }
        let mut events = Vec::new();
        let mut ordinal = 0;
        let mut patches = 0;
        for item in &walk.items {
            match item {
                MixedEventItem::Event(e) => {
                    let MixedEventKind::Note(n) = &e.event else {
                        return Err(refusal("translation", "unsupported parsed event"));
                    };
                    events.push(DecodedExportEvent::from_note(e.position, ordinal, n));
                }
                MixedEventItem::PatchToNote(n) => {
                    let patch = classified
                        .patches
                        .get(patches)
                        .filter(|p| p.source_ordinal == ordinal)
                        .ok_or_else(|| refusal("translation", "missing Patch accounting"))?
                        .result
                        .as_ref()
                        .map_err(|e| refusal("Patch", e))?;
                    events.push(patch.decoded_export_event());
                    events.push(DecodedExportEvent::from_note_body(
                        n.first_note_position,
                        ordinal + 1,
                        &n.first_note,
                    ));
                    patches += 1;
                }
                MixedEventItem::Patch(_) => {
                    return Err(refusal("Patch", "unsupported standalone Patch"))
                }
            }
            ordinal += item.logical_event_count() as u64;
        }
        if events.len() != walk.logical_event_count() || patches != classified.patches.len() {
            return Err(refusal("translation", "incomplete accounting"));
        }
        tracks.push(BoundedTrackManifest {
            source_ordinal: binding.pair_ordinal,
            descriptor_ordinal: binding.descriptor_ordinal,
            routing: routing.clone(),
            mute: mute.clone(),
            events,
            secondary_range: sequence.track_pairs[binding.pair_ordinal]
                .secondary
                .record_range
                .clone(),
        });
    }
    let manifest = BoundedSequenceManifest {
        sequence_range: sequence.sequence_range.clone(),
        name: sequence.sequence_name.bytes.bytes.to_vec(),
        ppqn: division,
        conductor,
        tracks,
    };
    // Acceptance includes all adapter and source-order safety checks. This
    // prevents an accepted proof from hiding an unsupported value or tie.
    assemble_bounded_sequence_with_report(&manifest)?;
    Ok(manifest)
}

pub(crate) fn assemble_bounded_sequence_with_report(
    manifest: &BoundedSequenceManifest,
) -> Result<MultitrackExportResult, ManifestRefusal> {
    let c = &manifest.conductor;
    let m = &c.meter;
    let conductor = midi_export::adapt_conductor(
        &manifest.name,
        c.tempo.mpqn(),
        (
            m.numerator.value,
            m.denominator_exponent.value,
            m.third_payload.value,
            m.fourth_payload.value,
        ),
        TimingPolicy::Identity480,
        MeterPolicy::KnownHistoricalOnly,
    )
    .map_err(|e| refusal("conductor adaptation", e))?;
    if !conductor.warnings.is_empty() || m.numerator.value == 0 || m.denominator().is_none() {
        return Err(refusal("meter", "fallback or invalid meter"));
    }
    let mut tracks = vec![smf::serialize_conductor_track(
        &conductor.sequence_name,
        conductor.tempo_mpqn,
        conductor.time_signature,
    )
    .map_err(|e| refusal("conductor serialization", e))?];
    let mut totals = conductor.counts.clone();
    let mut reports = Vec::new();
    for t in &manifest.tracks {
        let name =
            midi_export::adapt_text(&t.routing.label_bytes).map_err(|e| refusal("label", e))?;
        let channel =
            MidiChannel::new(t.routing.midi_channel).map_err(|e| refusal("channel", e))?;
        let adapted = midi_export::adapt_track(
            &t.events,
            Some(ChannelAssignment {
                channel,
                provenance: ChannelAssignmentProvenance::ParsedRouting,
            }),
            TimingPolicy::Identity480,
            PatchPolicy::StrictKnownOnly,
        )
        .map_err(|e| refusal("track adaptation", e))?;
        if !adapted.warnings.is_empty() || !adapted.untranslated_metadata.is_empty() {
            return Err(refusal("track adaptation", "incomplete translation"));
        }
        totals.add_assign(&adapted.counts);
        reports.push(MusicalTrackExportReport {
            context: format!("track {}", t.source_ordinal),
            name: name.clone(),
            channel_assignment: adapted.channel_assignment,
            counts: adapted.counts,
            warnings: adapted.warnings,
            untranslated_metadata: adapted.untranslated_metadata,
        });
        tracks.push(
            smf::serialize_named_musical_track_with_note_provenance(
                &name,
                &adapted.scheduled_events,
                &adapted.note_provenance,
            )
            .map_err(|e| refusal("source ordering", e))?,
        );
    }
    let smf_bytes = smf::serialize_format1(manifest.ppqn, &tracks)
        .map_err(|e| refusal("Format 1 assembly", e))?;
    Ok(MultitrackExportResult {
        smf_bytes,
        report: MultitrackExportReport {
            sequence_name: conductor.sequence_name,
            musical_track_count: reports.len(),
            total_smf_track_count: tracks.len(),
            tracks: reports,
            totals,
            warnings: conductor.warnings,
            untranslated_metadata: Vec::new(),
        },
    })
}

/// Construct Format 1 bytes in memory only. No publication or authorization.
pub fn assemble_bounded_sequence(
    manifest: &BoundedSequenceManifest,
) -> Result<Vec<u8>, ManifestRefusal> {
    Ok(assemble_bounded_sequence_with_report(manifest)?.smf_bytes)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    pub(crate) const NOTE: &[u8] = &[0, 0x90, 60, 127, 113, 102];

    fn record(b: &mut Vec<u8>, tag: u8, p: &[u8]) {
        b.push(tag);
        b.extend((p.len() as u32).to_be_bytes());
        b.extend(p);
    }
    pub(crate) fn synthetic(events: &[u8]) -> Vec<u8> {
        let mut b = vec![0; 8];
        for i in 0..2 {
            let mut p = vec![0; 36];
            p[25] = i;
            p[26] = 7;
            p[27] = 3;
            record(&mut b, 0x10, &p);
        }
        let mut device = vec![0; 40];
        device[0] = 4;
        device[1..5].copy_from_slice(b"Test");
        device[33] = 7;
        record(&mut b, 0x2a, &device);
        let start = b.len();
        let mut p = vec![0; 3 * 166 + 172];
        p[0] = 3;
        record(&mut b, 1, &p);
        let label = start + 208 + 2 * 166 + 15;
        b[label..label + 5].copy_from_slice(b"Other");
        b[label - 39..label - 31].copy_from_slice(&[128, 0, 4, 0, 0, 4, 1, 0]);
        b[label - 33] = 1;
        b[label - 31] = 1;
        b[label - 30..label].fill(0xff);
        let mut name = vec![0; 11];
        name.extend([4, b'N', b'e', b'w', b'!']);
        record(&mut b, 7, &name);
        conductor_pair(&mut b, &[0, 255, 88, 4, 4, 2, 8, 8]);
        conductor_pair(&mut b, &[0, 255, 81, 3, 7, 53, 120]);
        let mut p = vec![0; 14];
        p.extend(events);
        p.extend([0xff, 0, 0, 0, 0xff, 0x2f, 0]);
        record(&mut b, 2, &p);
        record(&mut b, 0x29, &[]);
        record(&mut b, 0, &[255; 8]);
        b
    }
    fn patch(p: u8) -> Vec<u8> {
        let mut e = vec![0, 0xff, 0x7c, 27, 0, 0, p | 0x80, 8, p, 12];
        e.extend(b"Unseen name!");
        e.extend([3, b'X', b'Y', b'Z', 4, 0xff, 0x50, 0xff, p]);
        e.extend(NOTE);
        e
    }

    fn conductor_pair(b: &mut Vec<u8>, event: &[u8]) {
        let mut primary = vec![0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 12, 34, 56, 78];
        primary.extend(event);
        primary.extend([135, 255, 255, 127, 255, 47, 0]);
        primary.resize(38, 0);
        record(b, 2, &primary);
        let n = event.len() as u8;
        let mut secondary = vec![
            0,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            n + 19,
            0,
            0,
            0,
            n + 19,
            0,
            0,
            0,
            1,
            0,
            0,
            0,
            n + 4,
            0,
            255,
            255,
            255,
            47,
            197,
            0,
            0,
            0,
            0,
        ];
        secondary.push(event[2]);
        secondary.extend(&event[4..]);
        secondary.extend([255; 3]);
        record(b, 41, &secondary);
    }

    #[test]
    fn report_bridge_uses_identical_source_order_and_adapter_counts() {
        let bytes = synthetic(&patch(16));
        let manifest = build_bounded_sequence_manifest(&bytes, 0, 480).unwrap();
        let result = assemble_bounded_sequence_with_report(&manifest).unwrap();
        assert_eq!(
            result.smf_bytes,
            assemble_bounded_sequence(&manifest).unwrap()
        );
        assert_eq!(result.report.totals.notes, 1);
        assert_eq!(result.report.totals.generated_note_offs, 1);
        assert_eq!(result.report.totals.bank_select_msb, 1);
        assert_eq!(result.report.totals.bank_select_lsb, 0);
        assert_eq!(result.report.totals.program_changes, 1);
        assert_eq!(result.report.totals.tempo, 1);
        assert_eq!(result.report.totals.meter, 1);
        assert_eq!(result.report.tracks[0].channel_assignment.channel.get(), 4);
        assert!(result.report.warnings.is_empty());
        assert!(result.report.untranslated_metadata.is_empty());
    }
}
