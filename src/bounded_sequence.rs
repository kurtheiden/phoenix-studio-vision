//! Pure, fail-closed construction evidence. This module grants no export authority.
//!
//! Only qualified Descriptor166 tracks and complete initial-only conductors are admitted.
//! Legacy application construction still requires saved Mute OFF. Explicit recovery
//! inclusion is a separate pure interface with no application authority. Values have
//! private construction, independent of profiles, paths and project identities.
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
    preludes: Vec<crate::bounded_prelude166::PreludeCoverage>,
}
impl BoundedConductorManifest {
    pub fn preludes(&self) -> &[crate::bounded_prelude166::PreludeCoverage] {
        &self.preludes
    }
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
    primary_range: Range<usize>,
    secondary_range: Range<usize>,
    events: Vec<DecodedExportEvent>,
    simultaneous_notes: Vec<crate::simultaneous_note166::OwnedSimultaneousNotes>,
    program_only_patches: Vec<crate::program_only166::ProgramOnlyPatch>,
    terminal_programs: Vec<crate::terminal_program166::TerminalProgram>,
    bank_patches: Vec<crate::bank_patch166_handoff::OwnedBankPatch>,
    context_notes: Vec<crate::context_note166::OwnedContextNote>,
    context_controllers: Vec<crate::context_controller166::OwnedContextController>,
    numeric_programs: Vec<crate::numeric_program166::NumericProgramPatch>,
    standalone_banks: Vec<crate::standalone_bank166::OwnedStandaloneBankPatch>,
}
impl BoundedTrackManifest {
    pub fn context_controllers(&self) -> &[crate::context_controller166::OwnedContextController] {
        &self.context_controllers
    }
    pub fn standalone_banks(&self) -> &[crate::standalone_bank166::OwnedStandaloneBankPatch] {
        &self.standalone_banks
    }
    pub fn numeric_programs(&self) -> &[crate::numeric_program166::NumericProgramPatch] {
        &self.numeric_programs
    }
    pub fn context_notes(&self) -> &[crate::context_note166::OwnedContextNote] {
        &self.context_notes
    }
    pub fn bank_patches(&self) -> &[crate::bank_patch166_handoff::OwnedBankPatch] {
        &self.bank_patches
    }
    pub fn terminal_programs(&self) -> &[crate::terminal_program166::TerminalProgram] {
        &self.terminal_programs
    }
    pub fn program_only_patches(&self) -> &[crate::program_only166::ProgramOnlyPatch] {
        &self.program_only_patches
    }
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
    pub fn primary_range(&self) -> Range<usize> {
        self.primary_range.clone()
    }
    pub fn secondary_range(&self) -> Range<usize> {
        self.secondary_range.clone()
    }
    pub fn events(&self) -> &[DecodedExportEvent] {
        &self.events
    }
}

/// Source-owned zero-object Descriptor166 record pair. No routing is inferred.
/// Fields are private; recovery constructs this only from reparsed ownership.
#[derive(Clone, Debug)]
pub struct EmptyTrackManifest {
    source_ordinal: usize,
    descriptor_ordinal: usize,
    descriptor_range: Range<usize>,
    primary_range: Range<usize>,
    secondary_range: Range<usize>,
    event_range: Range<usize>,
    label: Vec<u8>,
}
impl EmptyTrackManifest {
    pub fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }
    pub fn descriptor_ordinal(&self) -> usize {
        self.descriptor_ordinal
    }
    pub fn descriptor_range(&self) -> Range<usize> {
        self.descriptor_range.clone()
    }
    pub fn primary_range(&self) -> Range<usize> {
        self.primary_range.clone()
    }
    pub fn secondary_range(&self) -> Range<usize> {
        self.secondary_range.clone()
    }
    pub fn event_range(&self) -> Range<usize> {
        self.event_range.clone()
    }
    pub fn label(&self) -> &[u8] {
        &self.label
    }
}

pub(crate) fn qualify_empty_track(
    sequence: &crate::sequence_container::SequenceContainer<'_>,
    binding: &crate::sequence_container::TrackBinding,
) -> Result<EmptyTrackManifest, ManifestRefusal> {
    // build_manifest has independently verified exhaustive ordinal ownership.
    let pair = &sequence.track_pairs[binding.pair_ordinal];
    let descriptor = &sequence.descriptors[binding.descriptor_ordinal];
    let bounds = pair
        .validated_event_bounds()
        .map_err(|e| refusal("empty track bounds", e))?;
    let p = pair.primary.payload.bytes;
    // Exact observed zero-object primary and secondary framing. The primary's
    // four opaque context bytes are retained in its source range, not decoded.
    let secondary = [
        0, 1, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 4, 15, 255, 255, 255, 47,
        197, 255, 255, 255,
    ];
    if !bounds.event_range.is_empty()
        || p.len() != 21
        || p[..10] != [0, 1, 0, 0, 0, 0, 0, 0, 0, 0]
        || p[14..] != [255, 255, 255, 127, 255, 47, 0]
        || pair.secondary.payload.bytes != secondary
    {
        return Err(refusal("empty track framing", binding.pair_ordinal));
    }
    let label = descriptor
        .label
        .as_ref()
        .ok_or_else(|| refusal("empty track label", "unbounded"))?;
    midi_export::adapt_text(label.bytes).map_err(|e| refusal("empty track label", e))?;
    Ok(EmptyTrackManifest {
        source_ordinal: binding.pair_ordinal,
        descriptor_ordinal: binding.descriptor_ordinal,
        descriptor_range: descriptor.range.clone(),
        primary_range: pair.primary.record_range.clone(),
        secondary_range: pair.secondary.record_range.clone(),
        event_range: bounds.event_range,
        label: label.bytes.to_vec(),
    })
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
    empty_tracks: Vec<EmptyTrackManifest>,
    terminal_coverage: Option<crate::bounded_terminal166::TerminalCoverage>,
}
impl BoundedSequenceManifest {
    /// None denotes the unchanged legacy eight-FF acceptance path.
    pub fn terminal_coverage(&self) -> Option<&crate::bounded_terminal166::TerminalCoverage> {
        self.terminal_coverage.as_ref()
    }
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

/// Explicit recovery preference. No default is inferred from UI or native MIDI.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RecoveryInclusion {
    /// Recover all qualified musical tracks, regardless of saved mute state.
    #[default]
    IncludeMuted,
    /// Exclude only source-proven saved ON tracks; Unknown remains included.
    ExcludeMuted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecoveryTrackDisposition {
    Included,
    ExcludedSavedMute,
    AccountedEmpty,
}

/// Pure recovery ledger, deliberately distinct from application construction
/// authority. Every source track remains fully qualified, even when excluded.
/// No caller-supplied mute evidence or conversion to BoundedSequenceManifest.
#[derive(Clone, Debug)]
pub struct RecoverySequenceManifest {
    manifest: BoundedSequenceManifest,
    inclusion: RecoveryInclusion,
}
impl RecoverySequenceManifest {
    pub fn inclusion(&self) -> RecoveryInclusion {
        self.inclusion
    }
    /// Event-bearing source tracks, including omitted descriptors, primary/secondary ranges,
    /// event ranges, saved-mute evidence and fully translated events.
    pub fn tracks(&self) -> &[BoundedTrackManifest] {
        &self.manifest.tracks
    }
    /// Source-accounted zero-object tracks. No MIDI track or channel is invented.
    pub fn empty_tracks(&self) -> &[EmptyTrackManifest] {
        &self.manifest.empty_tracks
    }
    pub fn source_track_count(&self) -> usize {
        self.tracks().len() + self.empty_tracks().len()
    }
    pub fn conductor(&self) -> &BoundedConductorManifest {
        &self.manifest.conductor
    }
    pub fn sequence_range(&self) -> Range<usize> {
        self.manifest.sequence_range()
    }
    pub fn disposition(&self, source_ordinal: usize) -> Option<RecoveryTrackDisposition> {
        if self
            .empty_tracks()
            .iter()
            .any(|t| t.source_ordinal == source_ordinal)
        {
            return Some(RecoveryTrackDisposition::AccountedEmpty);
        }
        self.tracks()
            .iter()
            .find(|t| t.source_ordinal == source_ordinal)
            .map(|t| {
                if excluded(t, Some(self.inclusion)) {
                    RecoveryTrackDisposition::ExcludedSavedMute
                } else {
                    RecoveryTrackDisposition::Included
                }
            })
    }
}
fn excluded(track: &BoundedTrackManifest, inclusion: Option<RecoveryInclusion>) -> bool {
    inclusion == Some(RecoveryInclusion::ExcludeMuted) && track.mute.state == SavedMuteState::On
}

/// Reparse and qualify the entire sequence with explicit recovery inclusion.
/// Unknown mute states are retained, never treated as ON or relabeled OFF.
/// Excluded tracks must also pass complete translation and serializer preflight.
/// This interface is not used by compatibility profiles or application export.
pub fn build_recovery_sequence_manifest(
    bytes: &[u8],
    sequence_ordinal: usize,
    division: u16,
    inclusion: RecoveryInclusion,
) -> Result<RecoverySequenceManifest, ManifestRefusal> {
    Ok(RecoverySequenceManifest {
        manifest: build_manifest(bytes, sequence_ordinal, division, Some(inclusion))?,
        inclusion,
    })
}

/// Complete Format 1 construction in memory only. No filesystem publication,
/// application capability or native-reference input is accepted.
pub fn assemble_recovery_sequence(
    manifest: &RecoverySequenceManifest,
) -> Result<MultitrackExportResult, ManifestRefusal> {
    assemble_with_inclusion(&manifest.manifest, Some(manifest.inclusion))
}

// Exact bounded initial-only record envelope. Bytes 10..14 in the primary
// are opaque prefix context, not interpreted as a position or event.
// Secondary value copies must agree, including the full surrounding framing.
pub(crate) fn initial_only(
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
    build_manifest(bytes, sequence_ordinal, division, None)
}

fn build_manifest(
    bytes: &[u8],
    sequence_ordinal: usize,
    division: u16,
    inclusion: Option<RecoveryInclusion>,
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
    let preludes = crate::bounded_prelude166::classify_prelude_coverage(bytes, sequence_ordinal)
        .map_err(|e| refusal("conductor prelude", e))?;
    let terminal_coverage = if sequence.terminal_record.payload.bytes == [255; 8] {
        None
    } else {
        Some(
            crate::bounded_terminal166::classify_terminal_coverage(bytes, sequence_ordinal)
                .map_err(|e| refusal("sequence terminal", e))?,
        )
    };
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
        preludes,
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
    let mut empty_tracks = Vec::new();
    for binding in bindings {
        if inclusion.is_some() {
            let pair = &sequence.track_pairs[binding.pair_ordinal];
            let bounds = pair
                .validated_event_bounds()
                .map_err(|e| refusal("source accounting", e))?;
            let declared = u32::from_be_bytes(pair.primary.payload.bytes[2..6].try_into().unwrap());
            if declared == 0 || bounds.event_range.is_empty() {
                empty_tracks.push(qualify_empty_track(sequence, binding)?);
                continue;
            }
        }
        let mute = mutes
            .iter()
            .find(|r| {
                r.sequence_ordinal == sequence_ordinal
                    && r.descriptor_ordinal == binding.descriptor_ordinal
            })
            .ok_or_else(|| refusal("mute", "missing evidence"))?;
        if inclusion.is_none() && mute.state != SavedMuteState::Off {
            return Err(refusal("mute", mute.state));
        }
        // Recovery inclusion never bypasses declared raw-object accounting,
        // even for saved-ON tracks that will not be emitted.
        if inclusion.is_some() {
            crate::controller166::qualify_track(bytes, sequence_ordinal, binding.pair_ordinal)
                .map_err(|e| refusal("source accounting", (binding.pair_ordinal, e)))?;
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
        tracks.push(qualify_track_translation(
            bytes,
            sequence_ordinal,
            binding,
            sequence,
            mute,
            classified,
            &walk,
        )?);
    }
    let manifest = BoundedSequenceManifest {
        sequence_range: sequence.sequence_range.clone(),
        name: sequence.sequence_name.bytes.bytes.to_vec(),
        ppqn: division,
        terminal_coverage,
        conductor,
        tracks,
        empty_tracks,
    };
    // Acceptance includes all adapter and source-order safety checks. This
    // prevents an accepted proof from hiding an unsupported value or tie.
    assemble_with_inclusion(&manifest, inclusion)?;
    Ok(manifest)
}

// Shared unchanged production qualification; census supplies only same-source
// parser/routing evidence. This helper never grants export authority.
pub(crate) fn qualify_track_translation(
    bytes: &[u8],
    sequence_ordinal: usize,
    binding: &crate::sequence_container::TrackBinding,
    sequence: &crate::sequence_container::SequenceContainer<'_>,
    mute: &SavedMuteEvidence,
    classified: &crate::bounded_patch_translation::Cc0TrackClassification<'_>,
    walk: &crate::mixed_event::MixedEventWalk<'_>,
) -> Result<BoundedTrackManifest, ManifestRefusal> {
    let routing = &classified.routing;
    let sysex = if walk.items.iter().any(|item| {
        matches!(item,
            MixedEventItem::Event(e) if matches!(e.event, MixedEventKind::SysEx(_)))
    }) {
        crate::sysex166::qualify_track(bytes, sequence_ordinal, binding.pair_ordinal, walk)
            .map_err(|e| refusal("SysEx framing", e))?
    } else {
        vec![]
    };
    let controllers = if walk.items.iter().any(|item| {
        matches!(item,
            MixedEventItem::Event(e) if matches!(e.event, MixedEventKind::Controller(_)))
    }) {
        crate::controller166::qualify_track(bytes, sequence_ordinal, binding.pair_ordinal)
            .map_err(|e| refusal("translation", e))?
    } else {
        vec![]
    };
    let context_notes = if walk.items.iter().any(|item| {
        matches!(item,
            MixedEventItem::Event(e) if matches!(e.event, MixedEventKind::ContextMediatedNote(_)))
    }) {
        crate::context_note166::qualify_track(bytes, sequence_ordinal, binding.pair_ordinal)
            .map_err(|e| refusal("translation", e))?
    } else {
        vec![]
    };
    let mut events = Vec::new();
    let mut ordinal = 0;
    let mut patches = 0;
    let mut program_only_patches = Vec::new();
    let mut terminal_programs = Vec::new();
    let mut bank_patches = Vec::new();
    let mut numeric_programs = Vec::new();
    let mut standalone_banks = Vec::new();
    let mut context_controllers = Vec::new();
    for item in &walk.items {
        match item {
            MixedEventItem::Event(e) => {
                events.push(match &e.event {
                    MixedEventKind::SysEx(s) => {
                        let proof = sysex
                            .iter()
                            .find(|proof| {
                                let p = proof.provenance();
                                p.logical_source_ordinal == ordinal
                                    && p.representation_range == s.representation_range
                            })
                            .ok_or_else(|| refusal("SysEx framing", "missing owned proof"))?;
                        DecodedExportEvent::from_source_framed_sysex07(proof.clone())
                    }
                    MixedEventKind::Controller(c) => controllers
                        .iter()
                        .find(|proof| {
                            let p = proof.provenance();
                            p.logical_source_ordinal == ordinal
                                && p.representation_range == c.record_range
                        })
                        .ok_or_else(|| refusal("translation", "missing Controller proof"))?
                        .decoded_export_event(),
                    MixedEventKind::ContextMediatedController(_) => {
                        let proof = crate::context_controller166::qualify_current(
                            bytes,
                            sequence_ordinal,
                            binding.pair_ordinal,
                            item,
                            routing,
                            ordinal,
                        )
                        .map_err(|e| refusal("context Controller", e))?;
                        let event = proof.decoded_export_event();
                        context_controllers.push(proof);
                        event
                    }
                    MixedEventKind::ContextMediatedNote(n) => context_notes
                        .iter()
                        .find(|proof| {
                            let p = proof.provenance();
                            p.logical_source_ordinal == ordinal
                                && p.composition_range == n.representation_range
                                && p.composition_bytes == bytes[n.representation_range.clone()]
                                && p.absolute_tick == e.position
                                && proof.routing() == routing
                        })
                        .ok_or_else(|| refusal("translation", "missing context Note proof"))?
                        .decoded_export_event(),
                    MixedEventKind::Note(n) => {
                        DecodedExportEvent::from_note(e.position, ordinal, n)
                    }
                    MixedEventKind::ChannelPressure { entry, .. } => {
                        DecodedExportEvent::from_channel_pressure(e.position, ordinal, entry)
                    }
                    MixedEventKind::PitchBend { entry, .. } => {
                        DecodedExportEvent::from_pitch_bend(e.position, ordinal, entry)
                    }
                    _ => return Err(refusal("translation", "unsupported parsed event")),
                });
            }
            MixedEventItem::PatchToNote(n) => {
                let evidence = classified
                    .patches
                    .get(patches)
                    .filter(|p| p.source_ordinal == ordinal)
                    .ok_or_else(|| refusal("translation", "missing Patch accounting"))?;
                match &evidence.result {
                    Ok(patch) => {
                        events.push(patch.decoded_export_event());
                        events.push(DecodedExportEvent::from_note_body(
                            n.first_note_position,
                            ordinal + 1,
                            &n.first_note,
                        ));
                    }
                    Err(original) => {
                        if let Some(proof) = crate::program_only166::qualify_track(
                            bytes,
                            sequence_ordinal,
                            binding.pair_ordinal,
                        )
                        .ok()
                        .filter(|proof| {
                            let p = proof.provenance();
                            p.logical_source_ordinal == ordinal
                                && p.composition_range == n.representation_range
                        }) {
                            events.extend(proof.decoded_export_events());
                            program_only_patches.push(proof);
                        } else if let Ok(proof) = crate::numeric_program166::qualify_current(
                            bytes,
                            sequence_ordinal,
                            binding.pair_ordinal,
                            item,
                            routing,
                            ordinal,
                        ) {
                            events.extend(proof.decoded_export_events());
                            numeric_programs.push(proof);
                        } else {
                            let proof = crate::bank_patch166_handoff::qualify_current(
                                bytes,
                                sequence_ordinal,
                                binding.pair_ordinal,
                                item,
                                routing,
                                ordinal,
                            )
                            .map_err(|_| refusal("Patch", original))?;
                            events.extend(proof.decoded_export_events());
                            bank_patches.push(proof);
                        }
                    }
                }
                patches += 1;
            }
            MixedEventItem::Patch(patch) => {
                let evidence = classified
                    .patches
                    .get(patches)
                    .filter(|p| {
                        p.source_ordinal == ordinal
                            && p.source_range == patch.patch.representation_range
                    })
                    .ok_or_else(|| refusal("translation", "missing Patch accounting"))?;
                if let Some(proof) = crate::terminal_program166::qualify_track(
                    bytes,
                    sequence_ordinal,
                    binding.pair_ordinal,
                )
                .ok()
                .filter(|proof| {
                    let p = proof.provenance();
                    p.logical_source_ordinal == evidence.source_ordinal
                        && p.representation_range == patch.patch.representation_range
                }) {
                    events.push(proof.decoded_export_event());
                    terminal_programs.push(proof);
                } else {
                    let proof = crate::standalone_bank166::qualify_current(
                        bytes,
                        sequence_ordinal,
                        binding.pair_ordinal,
                        item,
                        routing,
                        ordinal,
                    )
                    .map_err(|_| refusal("Patch", "unsupported standalone Patch"))?;
                    events.push(proof.decoded_export_event());
                    standalone_banks.push(proof);
                }
                patches += 1;
            }
        }
        ordinal += item.logical_event_count() as u64;
    }
    if events.len() != walk.logical_event_count() || patches != classified.patches.len() {
        return Err(refusal("translation", "incomplete accounting"));
    }
    // Qualification failure supplies no exception. Existing preflight remains
    // authoritative; older supported tracks need no new accounting contract.
    let simultaneous_notes =
        crate::simultaneous_note166::qualify_track(bytes, sequence_ordinal, binding.pair_ordinal)
            .unwrap_or_default();
    Ok(BoundedTrackManifest {
        simultaneous_notes,
        program_only_patches,
        numeric_programs,
        standalone_banks,
        terminal_programs,
        bank_patches,
        context_notes,
        context_controllers,
        source_ordinal: binding.pair_ordinal,
        descriptor_ordinal: binding.descriptor_ordinal,
        routing: routing.clone(),
        mute: mute.clone(),
        events,
        primary_range: sequence.track_pairs[binding.pair_ordinal]
            .primary
            .record_range
            .clone(),
        secondary_range: sequence.track_pairs[binding.pair_ordinal]
            .secondary
            .record_range
            .clone(),
    })
}

pub(crate) fn assemble_bounded_sequence_with_report(
    manifest: &BoundedSequenceManifest,
) -> Result<MultitrackExportResult, ManifestRefusal> {
    assemble_with_inclusion(manifest, None)
}

// Same adapter/order checks are used before exclusion and by read-only census.
pub(crate) fn prepare_track_export(
    t: &BoundedTrackManifest,
) -> Result<
    (
        Vec<u8>,
        midi_export::ExportTrackResult,
        smf::SerializedTrack,
    ),
    ManifestRefusal,
> {
    let name = midi_export::adapt_text(&t.routing.label_bytes).map_err(|e| refusal("label", e))?;
    let channel = MidiChannel::new(t.routing.midi_channel).map_err(|e| refusal("channel", e))?;
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
    // Serialize before exclusion: no unsupported ordering can hide in an
    // omitted track. The ledger retains its complete source coverage.
    let serialized = smf::serialize_named_musical_track_with_owned_simultaneous_notes(
        &name,
        &adapted.scheduled_events,
        &adapted.note_provenance,
        &t.simultaneous_notes,
    )
    .map_err(|e| refusal("source ordering", e))?;
    Ok((name, adapted, serialized))
}

// Conductor adaptation is independently evaluable after the two bounded decoders.
pub(crate) fn prepare_conductor_export(
    name: &[u8],
    tempo: &InitialTempoEvent,
    m: &InitialMeterEvent,
) -> Result<(midi_export::ConductorResult, smf::SerializedTrack), ManifestRefusal> {
    let conductor = midi_export::adapt_conductor(
        name,
        tempo.mpqn(),
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
    let serialized = smf::serialize_conductor_track(
        &conductor.sequence_name,
        conductor.tempo_mpqn,
        conductor.time_signature,
    )
    .map_err(|e| refusal("conductor serialization", e))?;
    Ok((conductor, serialized))
}

fn assemble_with_inclusion(
    manifest: &BoundedSequenceManifest,
    inclusion: Option<RecoveryInclusion>,
) -> Result<MultitrackExportResult, ManifestRefusal> {
    let c = &manifest.conductor;
    let (conductor, serialized) = prepare_conductor_export(&manifest.name, &c.tempo, &c.meter)?;
    let mut tracks = vec![serialized];
    let mut totals = conductor.counts.clone();
    let mut reports = Vec::new();
    for ordinal in 0..manifest.tracks.len() + manifest.empty_tracks.len() {
        if manifest
            .empty_tracks
            .iter()
            .any(|t| t.source_ordinal == ordinal)
        {
            // Explicit AccountedEmpty disposition: complete zero-object source
            // ranges remain in the ledger; there is no routed MIDI part.
            continue;
        }
        let t = manifest
            .tracks
            .iter()
            .find(|t| t.source_ordinal == ordinal)
            .ok_or_else(|| refusal("source accounting", "missing track ordinal"))?;
        let (name, adapted, serialized) = prepare_track_export(t)?;
        if excluded(t, inclusion) {
            continue;
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
        tracks.push(serialized);
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
