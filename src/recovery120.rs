//! Complete in-memory recovery of the established bounded 120 subprofile.
//! No AppService capability, filesystem output or format fallback is provided.
use crate::authority120::{prove_project120, SequenceProof120, TrackRoute120};
use crate::bounded_conductor::{decode_compact_conductor, CompactConductor};
use crate::bounded_terminal::{assess_terminal_coverage, TerminalDiagnostic};
use crate::midi_export::{
    self, ChannelAssignment, ChannelAssignmentProvenance, DecodedExportEvent, ExportCounts,
    PatchPolicy, PatchTranslation, TimingPolicy,
};
use crate::mixed_event::{MixedEventItem, MixedEventKind};
use crate::semantic_layout120::{associate_project_120, Source120SequenceAssociation};
use crate::smf::{self, MidiChannel, ScheduledEvent};
use sha2::{Digest, Sha256};
use std::ops::Range;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Recovery120Refusal {
    pub stage: &'static str,
    pub track_ordinal: Option<usize>,
    pub detail: String,
}
fn refusal(
    stage: &'static str,
    track_ordinal: Option<usize>,
    e: impl std::fmt::Debug,
) -> Recovery120Refusal {
    Recovery120Refusal {
        stage,
        track_ordinal,
        detail: format!("{e:?}"),
    }
}
#[derive(Debug)]
pub struct RecoveredTrack120 {
    pub ordinal: usize,
    pub name: Vec<u8>,
    /// None for Tracks without events; never a synthetic channel assignment.
    pub channel_assignment: Option<ChannelAssignment>,
    pub handoff: Vec<DecodedExportEvent>,
    pub scheduled: Vec<ScheduledEvent>,
    pub counts: ExportCounts,
}
/// Immutable success with exact source/Sequence identity and retained proof.
/// Readback compared all emitted events, metadata and EOT against the handoff.
#[derive(Debug)]
pub struct Recovery120<'a> {
    proof: SequenceProof120<'a>,
    source_sha256: String,
    candidate_ordinal: usize,
    conductor: CompactConductor,
    terminal: Option<TerminalDiagnostic>,
    tracks: Vec<RecoveredTrack120>,
    counts: ExportCounts,
    midi: Vec<u8>,
}
impl<'a> Recovery120<'a> {
    pub fn proof(&self) -> &SequenceProof120<'a> {
        &self.proof
    }
    pub fn source_sha256(&self) -> &str {
        &self.source_sha256
    }
    pub fn candidate_ordinal(&self) -> usize {
        self.candidate_ordinal
    }
    pub fn sequence_range(&self) -> Range<usize> {
        self.proof.range()
    }
    pub fn name(&self) -> &'a [u8] {
        self.proof.name()
    }
    pub fn conductor(&self) -> &CompactConductor {
        &self.conductor
    }
    pub fn terminal(&self) -> Option<&TerminalDiagnostic> {
        self.terminal.as_ref()
    }
    pub fn tracks(&self) -> &[RecoveredTrack120] {
        &self.tracks
    }
    pub fn counts(&self) -> &ExportCounts {
        &self.counts
    }
    pub fn midi(&self) -> &[u8] {
        &self.midi
    }
    pub fn mechanically_verified(&self) -> bool {
        true
    }
}
#[derive(Debug)]
pub struct CandidateRecovery120<'a> {
    pub ordinal: usize,
    pub name: &'a [u8],
    pub result: Result<Recovery120<'a>, Recovery120Refusal>,
}
/// Recover every observed candidate independently. No partial Sequence success.
/// All successful MIDI is built here; no research reports/artifacts are read.
pub fn recover_project120(
    source: &[u8],
) -> Result<Vec<CandidateRecovery120<'_>>, Recovery120Refusal> {
    let proofs = prove_project120(source).map_err(|e| refusal("authority", None, e))?;
    let bridge = associate_project_120(source).map_err(|e| refusal("association", None, e))?;
    if proofs.len() != bridge.candidates.len() {
        return Err(refusal("identity", None, "candidate inventory mismatch"));
    }
    let digest = format!("{:x}", Sha256::digest(source));
    proofs
        .into_iter()
        .zip(bridge.candidates)
        .map(|(p, c)| {
            let ordinal = p.ordinal;
            let name = p.name;
            let result = p
                .result
                .map_err(|e| refusal("authority", None, e))
                .and_then(|proof| {
                    let s = c.association.map_err(|e| refusal("association", None, e))?;
                    if proof.range() != s.sequence_range || proof.name() != s.source_name.bytes {
                        return Err(refusal("identity", None, "association mismatch"));
                    }
                    recover(proof, ordinal, digest.clone(), s)
                });
            Ok(CandidateRecovery120 {
                ordinal,
                name,
                result,
            })
        })
        .collect()
}
fn recover<'a>(
    proof: SequenceProof120<'a>,
    ordinal: usize,
    digest: String,
    s: Source120SequenceAssociation<'a>,
) -> Result<Recovery120<'a>, Recovery120Refusal> {
    let source = proof.source();
    let ranges = [
        s.leading_special[0].pair.primary.record_range.clone(),
        s.leading_special[0].pair.secondary.record_range.clone(),
        s.leading_special[1].pair.primary.record_range.clone(),
        s.leading_special[1].pair.secondary.record_range.clone(),
    ];
    let conductor = decode_compact_conductor(source, ranges.clone())
        .map_err(|e| refusal("conductor", None, e))?;
    let c = conductor
        .adapt(proof.name())
        .map_err(|e| refusal("conductor adaptation", None, e))?;
    let key = conductor.key().map(|k| (k.accidentals, k.minor));
    let mut chunks = vec![smf::serialize_conductor_track_with_key(
        &c.sequence_name,
        c.tempo_mpqn,
        c.time_signature,
        key,
    )
    .map_err(|e| refusal("conductor serialization", None, e))?];
    let mut expected = vec![crate::readback120::conductor_expected(&c, key)];
    let mut counts = c.counts.clone();
    let mut tracks = Vec::new();
    let mut coverage = ranges.to_vec();
    for t in proof.tracks() {
        let at = Some(t.ordinal());
        if let Some(ranges) = t.pair_ranges() {
            coverage.extend_from_slice(ranges);
        }
        let name = midi_export::adapt_text(t.name()).map_err(|e| refusal("track name", at, e))?;
        let (handoff, channel_assignment) = match t.route() {
            TrackRoute120::NoEvents => (Vec::new(), None),
            TrackRoute120::Single(r) => (
                single_handoff(
                    t.walk()
                        .ok_or_else(|| refusal("authority", at, "missing walk"))?,
                    at,
                )?,
                Some(ChannelAssignment {
                    channel: MidiChannel::new(r.channel).map_err(|e| refusal("channel", at, e))?,
                    provenance: ChannelAssignmentProvenance::ParsedRouting,
                }),
            ),
            TrackRoute120::Multi(m) => {
                let channel = m
                    .patches()
                    .first()
                    .ok_or_else(|| refusal("authority", at, "empty Multi"))?
                    .midi_channel;
                (
                    m.export_events(),
                    Some(ChannelAssignment {
                        channel: MidiChannel::new(channel)
                            .map_err(|e| refusal("channel", at, e))?,
                        provenance: ChannelAssignmentProvenance::ParsedRouting,
                    }),
                )
            }
        };
        let (scheduled, provenance, track_counts) = if handoff.is_empty() {
            (Vec::new(), Vec::new(), ExportCounts::default())
        } else {
            let a = midi_export::adapt_track(
                &handoff,
                channel_assignment,
                TimingPolicy::Identity480,
                PatchPolicy::StrictKnownOnly,
            )
            .map_err(|e| refusal("adaptation", at, e))?;
            if !a.warnings.is_empty() || !a.untranslated_metadata.is_empty() {
                return Err(refusal("adaptation", at, "incomplete translation"));
            }
            (a.scheduled_events, a.note_provenance, a.counts)
        };
        chunks.push(
            smf::serialize_named_musical_track_with_note_provenance(&name, &scheduled, &provenance)
                .map_err(|e| refusal("source order/serialization", at, e))?,
        );
        expected.push(crate::readback120::musical_expected(&name, &scheduled));
        counts.add_assign(&track_counts);
        tracks.push(RecoveredTrack120 {
            ordinal: t.ordinal(),
            name,
            channel_assignment,
            handoff,
            scheduled,
            counts: track_counts,
        });
    }
    coverage.sort_by_key(|r| r.start);
    let terminal = assess_terminal_coverage(source, proof.range(), &coverage)
        .map_err(|e| refusal("terminal", None, e))?;
    let midi = smf::serialize_format1(c.ppqn, &chunks).map_err(|e| refusal("SMF", None, e))?;
    crate::readback120::verify(&midi, c.ppqn, &expected)
        .map_err(|e| refusal("readback", None, e))?;
    Ok(Recovery120 {
        proof,
        source_sha256: digest,
        candidate_ordinal: ordinal,
        conductor,
        terminal,
        tracks,
        counts,
        midi,
    })
}

fn program_only(p: u8, pre: &[u8], post: &[u8]) -> Result<PatchTranslation, &'static str> {
    // Narrow subset of the established G/f8 Program-only production predicate
    // in bounded_multi_patch, with single-Instrument context prefix00. No new
    // bank/wrapper semantics and no value inferred from a display label.
    if p <= 127
        && pre == [0, 0, p, 0xf8, p]
        && post.len() >= 7
        && usize::from(post[0]) + 5 == post.len()
        && post[post.len() - 4..] == [4, 0xff, 0xff, 0xff]
        && post[1..post.len() - 4] == *format!("G{p}").as_bytes()
    {
        Ok(PatchTranslation::ProgramOnlyConfirmed)
    } else {
        Err("unsupported single-Instrument Patch translation")
    }
}
fn single_handoff(
    w: &crate::mixed_event::MixedEventWalk<'_>,
    at: Option<usize>,
) -> Result<Vec<DecodedExportEvent>, Recovery120Refusal> {
    let mut out = Vec::new();
    let mut ordinal = 0u64;
    for item in &w.items {
        match item {
            MixedEventItem::Event(e) => out.push(match &e.event {
                MixedEventKind::Note(n) => DecodedExportEvent::from_note(e.position, ordinal, n),
                MixedEventKind::ContextMediatedNote(n) => {
                    DecodedExportEvent::from_note_body(e.position, ordinal, &n.note)
                }
                MixedEventKind::DoubleContextMediatedNote(n) => {
                    DecodedExportEvent::from_note_body(e.position, ordinal, &n.note)
                }
                MixedEventKind::Controller(n) => {
                    DecodedExportEvent::from_controller(e.position, ordinal, n)
                }
                MixedEventKind::MidiController(n) => {
                    DecodedExportEvent::from_midi_controller(e.position, ordinal, n)
                }
                MixedEventKind::PitchBend { entry, .. } => {
                    DecodedExportEvent::from_pitch_bend(e.position, ordinal, entry)
                }
                MixedEventKind::ChannelPressure { entry, .. } => {
                    DecodedExportEvent::from_channel_pressure(e.position, ordinal, entry)
                }
            }),
            MixedEventItem::Patch(p) => {
                let tr = program_only(
                    p.patch.program_change.value,
                    p.patch.pre_name_context.bytes,
                    p.patch.post_name_context.bytes,
                )
                .map_err(|e| refusal("Patch translation", at, e))?;
                out.push(DecodedExportEvent {
                    absolute_position: p.position,
                    source_ordinal: ordinal,
                    source_range: Some(p.patch.representation_range.clone()),
                    kind: midi_export::DecodedExportEventKind::Patch {
                        program: p.patch.program_change.value,
                        translation: tr,
                    },
                });
            }
            MixedEventItem::PatchToNote(p) => {
                let tr = program_only(
                    p.patch.program_change.value,
                    p.patch.pre_name_context.bytes,
                    p.patch.post_name_context.bytes,
                )
                .map_err(|e| refusal("Patch translation", at, e))?;
                out.push(DecodedExportEvent::from_patch(
                    p.patch_position,
                    ordinal,
                    &p.patch,
                    tr,
                ));
                out.push(DecodedExportEvent::from_note_body(
                    p.first_note_position,
                    ordinal
                        .checked_add(1)
                        .ok_or_else(|| refusal("ordinal", at, "overflow"))?,
                    &p.first_note,
                ));
            }
        }
        ordinal = ordinal
            .checked_add(item.logical_event_count() as u64)
            .ok_or_else(|| refusal("ordinal", at, "overflow"))?;
    }
    Ok(out)
}
#[cfg(test)]
#[path = "recovery120_tests.rs"]
mod tests;
