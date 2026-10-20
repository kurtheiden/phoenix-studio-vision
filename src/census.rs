//! Read-only Descriptor166 census. Evidence never grants recovery/export authority.
//! Composite qualifiers retain their own evaluation order; no cursor resumption.
use crate::{
    app_contract::*,
    app_service::AppService,
    bounded_sequence::*,
    mixed_event::{MixedEventBounds, MixedEventItem, MixedEventKind, MixedEventWalkError},
    saved_mute::{SavedMuteState, SavedMuteUnknown},
    sequence_container::TrackAssociations,
};
use serde::Serialize;
use std::{collections::BTreeMap, fmt::Debug, ops::Range, path::Path};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Outcome {
    Passed,
    Failed,
    NotEvaluated,
    NotApplicable,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Stage {
    Structure,
    Ownership,
    Prelude,
    Terminal,
    Tempo,
    TempoCoverage,
    Meter,
    MeterCoverage,
    ConductorExport,
    EventBounds,
    EmptyTrack,
    EventWalk,
    SourceAccounting,
    Routing,
    SavedMute,
    PatchTranslation,
    ControllerTranslation,
    ContextNoteTranslation,
    ContextControllerTranslation,
    SysExTranslation,
    Translation,
    TrackExport,
    SequenceAssembly,
    ProductionRecovery,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum Family {
    Structure,
    Ownership,
    Conductor,
    Terminal,
    Bounds,
    Decoding,
    Accounting,
    Routing,
    SavedMuteEvidence,
    Translation,
    Adaptation,
    Ordering,
    Serialization,
    Authorization,
    ApplicationPreparation,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Diagnostic {
    /// Index within the sequence's diagnostics (or project diagnostics).
    pub id: usize,
    pub descriptor_ordinal: Option<usize>,
    pub pair_ordinal: Option<usize>,
    pub stage: Stage,
    pub outcome: Outcome,
    pub blocker_family: Option<Family>,
    pub reason_code: String,
    /// Evidence-only unknown mute states do not block explicit recovery.
    pub blocks_general_recovery: bool,
    pub source_range: Option<Range<usize>>,
    pub cursor: Option<usize>,
    pub prerequisites: Vec<usize>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Probe {
    pub outcome: Outcome,
    pub stage: String,
    pub reason_code: String,
    pub blocker_family: Option<Family>,
    pub source_range: Option<Range<usize>>,
    pub cursor: Option<usize>,
}
impl Probe {
    fn passed() -> Self {
        Self {
            outcome: Outcome::Passed,
            stage: "complete".into(),
            reason_code: "passed".into(),
            blocker_family: None,
            source_range: None,
            cursor: None,
        }
    }
    fn skipped(reason: &str) -> Self {
        Self {
            outcome: Outcome::NotEvaluated,
            stage: "application_preparation".into(),
            reason_code: reason.into(),
            blocker_family: None,
            source_range: None,
            cursor: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct SequenceCensus {
    pub ordinal: usize,
    pub descriptor_count: usize,
    pub pair_count: usize,
    pub general_include_muted: Probe,
    pub general_exclude_muted: Probe,
    pub application_default: Probe,
    pub application_include_muted: Probe,
    pub application_exclude_muted: Probe,
    pub proven_policy_only: bool,
    pub diagnostics: Vec<Diagnostic>,
    pub coverage_limitations: Vec<String>,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct Metrics {
    pub complete_include_muted: usize,
    pub complete_exclude_muted: usize,
    pub application_authorized: usize,
    pub application_not_evaluated: usize,
    pub proven_policy_only: usize,
    pub sequences_with_decoding_blockers: usize,
    pub sequences_with_translation_blockers: usize,
    pub blocker_instances: BTreeMap<Family, usize>,
    pub evidence_limitations: BTreeMap<Family, usize>,
    pub not_evaluated_by_cause: BTreeMap<String, usize>,
}
impl std::ops::AddAssign<&Metrics> for Metrics {
    fn add_assign(&mut self, other: &Metrics) {
        self.complete_include_muted += other.complete_include_muted;
        self.complete_exclude_muted += other.complete_exclude_muted;
        self.application_authorized += other.application_authorized;
        self.application_not_evaluated += other.application_not_evaluated;
        self.proven_policy_only += other.proven_policy_only;
        self.sequences_with_decoding_blockers += other.sequences_with_decoding_blockers;
        self.sequences_with_translation_blockers += other.sequences_with_translation_blockers;
        for (family, count) in &other.blocker_instances {
            *self.blocker_instances.entry(*family).or_default() += count;
        }
        for (family, count) in &other.evidence_limitations {
            *self.evidence_limitations.entry(*family).or_default() += count;
        }
        for (cause, count) in &other.not_evaluated_by_cause {
            *self
                .not_evaluated_by_cause
                .entry(cause.clone())
                .or_default() += count;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ProjectCensus {
    pub schema_version: u32,
    pub diagnostics: Vec<Diagnostic>,
    pub sequences: Vec<SequenceCensus>,
    pub metrics: Metrics,
    pub coverage_limitations: Vec<String>,
}

// Only the enum discriminant is exposed. Debug payloads can contain source bytes,
// strings or paths and must never enter a shareable report.
fn code(error: impl Debug) -> String {
    let text = format!("{error:?}");
    if text.starts_with('"') {
        return "composite_refusal".into();
    }
    text.chars()
        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
        .collect()
}
fn manifest_probe(error: ManifestRefusal) -> Probe {
    let reason_code: String = match error.detail.as_str() {
        "\"unsupported standalone Patch\"" => "unsupported_standalone_patch".into(),
        "\"unsupported parsed event\"" => "unsupported_parsed_event".into(),
        _ => {
            if error.detail.starts_with('"') {
                "composite_refusal".into()
            } else {
                error
                    .detail
                    .chars()
                    .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                    .collect()
            }
        }
    };
    Probe {
        outcome: Outcome::Failed,
        stage: error.stage.into(),
        blocker_family: Some(match error.stage {
            "ownership" => Family::Ownership,
            "conductor prelude" | "conductor coverage" | "tempo" | "meter" => Family::Conductor,
            "sequence terminal" => Family::Terminal,
            "mute" => Family::SavedMuteEvidence,
            "source accounting" => Family::Accounting,
            "routing" => Family::Routing,
            "event walk" => Family::Decoding,
            "Patch" | "translation" | "context Controller" | "SysEx framing" => Family::Translation,
            "source ordering" => Family::Ordering,
            "conductor serialization" | "Format 1 assembly" => Family::Serialization,
            "label" | "channel" | "track adaptation" | "conductor adaptation" => Family::Adaptation,
            _ => Family::Structure,
        }),
        source_range: None,
        cursor: None,
        reason_code: if reason_code.is_empty() {
            "composite_refusal".into()
        } else {
            reason_code
        },
    }
}
fn general_probe(bytes: &[u8], ordinal: usize, mode: RecoveryInclusion) -> Probe {
    match build_recovery_sequence_manifest(bytes, ordinal, 480, mode)
        .and_then(|m| assemble_recovery_sequence(&m))
    {
        Ok(_) => Probe::passed(),
        Err(e) => manifest_probe(e),
    }
}
struct Rows {
    rows: Vec<Diagnostic>,
    descriptor: Option<usize>,
    pair: Option<usize>,
}
impl Rows {
    fn add(
        &mut self,
        stage: Stage,
        outcome: Outcome,
        family: Family,
        reason: impl Into<String>,
        range: Option<Range<usize>>,
        prerequisites: &[usize],
    ) -> usize {
        let id = self.rows.len();
        self.rows.push(Diagnostic {
            id,
            descriptor_ordinal: self.descriptor,
            pair_ordinal: self.pair,
            stage,
            outcome,
            blocker_family: (outcome == Outcome::Failed).then_some(family),
            blocks_general_recovery: outcome == Outcome::Failed
                && family != Family::SavedMuteEvidence,
            reason_code: reason.into(),
            source_range: range,
            cursor: None,
            prerequisites: prerequisites.to_vec(),
        });
        id
    }
    fn check<T, E: Debug>(
        &mut self,
        stage: Stage,
        family: Family,
        range: Option<Range<usize>>,
        result: &Result<T, E>,
    ) -> usize {
        self.add(
            stage,
            if result.is_ok() {
                Outcome::Passed
            } else {
                Outcome::Failed
            },
            family,
            result
                .as_ref()
                .err()
                .map(code)
                .unwrap_or_else(|| "passed".into()),
            range,
            &[],
        )
    }
    fn skip(&mut self, stage: Stage, cause: usize) -> usize {
        self.add(
            stage,
            Outcome::NotEvaluated,
            Family::Structure,
            "prerequisite_failed",
            None,
            &[cause],
        )
    }
    fn defer(&mut self, id: usize, causes: &[usize]) {
        let row = &mut self.rows[id];
        row.outcome = Outcome::NotEvaluated;
        row.blocker_family = None;
        row.blocks_general_recovery = false;
        row.reason_code = "prerequisite_failed".into();
        row.prerequisites = causes.to_vec();
    }
    fn absent(&mut self, stage: Stage, range: Option<Range<usize>>) -> usize {
        self.add(
            stage,
            Outcome::NotApplicable,
            Family::Structure,
            "not_present",
            range,
            &[],
        )
    }
}

/// Pure API: no filesystem or application authority is consulted.
pub fn census_bytes(bytes: &[u8]) -> ProjectCensus {
    let parsed = crate::sequence_container::parse_project_166(bytes);
    let mut root = Rows {
        rows: vec![],
        descriptor: None,
        pair: None,
    };
    root.check(
        Stage::Structure,
        Family::Structure,
        Some(0..bytes.len()),
        &parsed,
    );
    let mut report = ProjectCensus { schema_version: 1, diagnostics: root.rows, sequences: vec![], metrics: Metrics::default(),
        coverage_limitations: vec!["Descriptor166 only; failed structural parsing exposes no invented sequences".into(),
        "Composite qualifiers stop internally; no resynchronization or exhaustive subcondition diagnosis".into(),
        "Application probe covers authorization and in-memory preparation, not destination/publication failures".into()] };
    let Ok(project) = parsed else {
        report.recount();
        return report;
    };
    let routes =
        crate::bounded_routing::decode_bounded_routing(bytes).expect("same-source parsed project");
    let mutes =
        crate::saved_mute::collect_saved_mute_evidence(bytes).expect("same-source parsed project");
    let patches = crate::bounded_patch_translation::classify_bounded_cc0_patches(bytes)
        .expect("same-source parsed project");
    for (ordinal, sequence) in project.sequences.iter().enumerate() {
        let mut r = Rows {
            rows: vec![],
            descriptor: None,
            pair: None,
        };
        let ownership = match &sequence.track_associations {
            TrackAssociations::Ordinal(b)
                if !b.is_empty()
                    && b.len() == sequence.track_pairs.len()
                    && b.len() == sequence.track_descriptors().len()
                    && b.iter()
                        .enumerate()
                        .all(|(i, b)| b.pair_ordinal == i && b.descriptor_ordinal == i + 2) =>
            {
                Ok(b)
            }
            _ => Err("nonexhaustive_ordinal_binding"),
        };
        let owner = r.add(
            Stage::Ownership,
            if ownership.is_ok() {
                Outcome::Passed
            } else {
                Outcome::Failed
            },
            Family::Ownership,
            if ownership.is_ok() {
                "passed"
            } else {
                "nonexhaustive_ordinal_binding"
            },
            Some(sequence.sequence_range.clone()),
            &[],
        );
        if sequence.prelude_records.is_empty() {
            r.absent(Stage::Prelude, None);
        } else if ownership.is_ok() {
            r.check(
                Stage::Prelude,
                Family::Conductor,
                Some(
                    sequence.prelude_records.first().unwrap().record_range.start
                        ..sequence.prelude_records.last().unwrap().record_range.end,
                ),
                &crate::bounded_prelude166::classify_prelude_coverage(bytes, ordinal),
            );
        } else {
            r.skip(Stage::Prelude, owner);
        }
        if sequence.terminal_record.payload.bytes == [255; 8] {
            r.add(
                Stage::Terminal,
                Outcome::Passed,
                Family::Terminal,
                "legacy_eight_ff",
                Some(sequence.terminal_record.record_range.clone()),
                &[],
            );
        } else if ownership.is_ok() {
            r.check(
                Stage::Terminal,
                Family::Terminal,
                Some(sequence.terminal_record.record_range.clone()),
                &crate::bounded_terminal166::classify_terminal_coverage(bytes, ordinal),
            );
        } else {
            r.skip(Stage::Terminal, owner);
        }
        let tempo = crate::tempo::decode_bounded_initial_tempo(
            bytes,
            crate::tempo::InitialTempoBounds {
                event_range: sequence.initial_tempo_range.clone(),
            },
        );
        let t = r.check(
            Stage::Tempo,
            Family::Conductor,
            Some(sequence.initial_tempo_range.clone()),
            &tempo,
        );
        if tempo.is_ok() {
            let coverage = r.check(
                Stage::TempoCoverage,
                Family::Conductor,
                Some(
                    sequence.tempo_primary.record_range.start
                        ..sequence.tempo_secondary.record_range.end,
                ),
                &crate::bounded_sequence::initial_only(
                    &sequence.tempo_primary,
                    &sequence.tempo_secondary,
                    &bytes[sequence.initial_tempo_range.clone()],
                ),
            );
            if r.rows[coverage].outcome == Outcome::Failed {
                r.rows[coverage].reason_code = "initial_only_envelope_or_secondary_mismatch".into();
            }
        } else {
            r.skip(Stage::TempoCoverage, t);
        }
        let meter = crate::meter::decode_bounded_initial_meter(
            bytes,
            crate::meter::InitialMeterBounds {
                event_range: sequence.initial_meter_range.clone(),
            },
        );
        let m = r.check(
            Stage::Meter,
            Family::Conductor,
            Some(sequence.initial_meter_range.clone()),
            &meter,
        );
        if meter.is_ok() {
            let coverage = r.check(
                Stage::MeterCoverage,
                Family::Conductor,
                Some(
                    sequence.meter_primary.record_range.start
                        ..sequence.meter_secondary.record_range.end,
                ),
                &crate::bounded_sequence::initial_only(
                    &sequence.meter_primary,
                    &sequence.meter_secondary,
                    &bytes[sequence.initial_meter_range.clone()],
                ),
            );
            if r.rows[coverage].outcome == Outcome::Failed {
                r.rows[coverage].reason_code = "initial_only_envelope_or_secondary_mismatch".into();
            }
        } else {
            r.skip(Stage::MeterCoverage, m);
        }
        if let (Ok(tempo), Ok(meter)) = (&tempo, &meter) {
            let result = crate::bounded_sequence::prepare_conductor_export(
                sequence.sequence_name.bytes.bytes,
                tempo,
                meter,
            );
            let id = r.check(
                Stage::ConductorExport,
                Family::Conductor,
                Some(
                    sequence.name_record.record_range.start
                        ..sequence.tempo_secondary.record_range.end,
                ),
                &result,
            );
            if let Err(error) = result {
                r.rows[id].reason_code = if error.stage == "meter" {
                    "fallback_or_invalid_meter".into()
                } else {
                    manifest_probe(error).reason_code
                };
            }
        } else {
            let causes: Vec<_> = [t, m]
                .into_iter()
                .filter(|id| r.rows[*id].outcome == Outcome::Failed)
                .collect();
            r.add(
                Stage::ConductorExport,
                Outcome::NotEvaluated,
                Family::Conductor,
                "prerequisite_failed",
                None,
                &causes,
            );
        }
        for descriptor in sequence.track_descriptors() {
            r.descriptor = Some(descriptor.ordinal);
            r.pair = None;
            let binding = ownership.as_ref().ok().and_then(|b| {
                b.iter()
                    .find(|b| b.descriptor_ordinal == descriptor.ordinal)
            });
            let Some(binding) = binding else {
                for stage in track_stages() {
                    r.skip(stage, owner);
                }
                continue;
            };
            r.pair = Some(binding.pair_ordinal);
            let bounds = sequence.validated_track_event_bounds(binding.pair_ordinal);
            let bound_id = r.check(
                Stage::EventBounds,
                Family::Bounds,
                Some(
                    sequence.track_pairs[binding.pair_ordinal]
                        .primary
                        .record_range
                        .clone(),
                ),
                &bounds,
            );
            let Ok(bounds) = bounds else {
                for stage in track_stages()
                    .into_iter()
                    .filter(|s| *s != Stage::EventBounds)
                {
                    r.skip(stage, bound_id);
                }
                continue;
            };
            let range = Some(bounds.event_range.clone());
            let pair = &sequence.track_pairs[binding.pair_ordinal];
            let declared = u32::from_be_bytes(pair.primary.payload.bytes[2..6].try_into().unwrap());
            let empty_failure = if declared == 0 || bounds.event_range.is_empty() {
                let result = crate::bounded_sequence::qualify_empty_track(sequence, binding);
                let empty = r.check(
                    Stage::EmptyTrack,
                    Family::Accounting,
                    range.clone(),
                    &result,
                );
                if let Err(e) = &result {
                    r.rows[empty].reason_code = e.stage.replace(' ', "_");
                }
                if result.is_ok() {
                    for stage in track_stages()
                        .into_iter()
                        .filter(|s| !matches!(s, Stage::EventBounds | Stage::EmptyTrack))
                    {
                        if stage == Stage::SourceAccounting {
                            r.add(
                                stage,
                                Outcome::Passed,
                                Family::Accounting,
                                "accounted_empty",
                                range.clone(),
                                &[],
                            );
                        } else {
                            r.absent(stage, range.clone());
                        }
                    }
                    continue;
                }
                Some(empty)
            } else {
                r.absent(Stage::EmptyTrack, range.clone());
                None
            };
            let walk = crate::mixed_event::walk_bounded_mixed_events(
                bytes,
                MixedEventBounds {
                    event_range: bounds.event_range.clone(),
                },
                Default::default(),
            );
            let walk_id = r.check(Stage::EventWalk, Family::Decoding, range.clone(), &walk);
            if let Err(e) = &walk {
                r.rows[walk_id].cursor = walk_cursor(e);
            }
            let mute = mutes
                .iter()
                .find(|m| {
                    m.sequence_ordinal == ordinal && m.descriptor_ordinal == descriptor.ordinal
                })
                .unwrap();
            match mute.state {
                SavedMuteState::Off | SavedMuteState::On => {
                    r.add(
                        Stage::SavedMute,
                        Outcome::Passed,
                        Family::SavedMuteEvidence,
                        if mute.state == SavedMuteState::On {
                            "saved_on"
                        } else {
                            "saved_off"
                        },
                        mute.candidate_offset.map(|p| p..p + 1),
                        &[],
                    );
                }
                SavedMuteState::Unknown(SavedMuteUnknown::IncompleteEventWalk) if walk.is_err() => {
                    r.skip(Stage::SavedMute, walk_id);
                }
                SavedMuteState::Unknown(SavedMuteUnknown::EmptyEvents)
                    if empty_failure.is_some() =>
                {
                    r.skip(Stage::SavedMute, empty_failure.unwrap());
                }
                SavedMuteState::Unknown(reason) => {
                    r.add(
                        Stage::SavedMute,
                        Outcome::Failed,
                        Family::SavedMuteEvidence,
                        code(reason),
                        mute.candidate_offset.map(|p| p..p + 1),
                        &[],
                    );
                }
            }
            let route = &routes
                .iter()
                .find(|t| {
                    t.structural_ordinal == ordinal && t.descriptor_ordinal == descriptor.ordinal
                })
                .unwrap()
                .result;
            // Routing's complete-context validation includes a full walk. Report
            // the decoding root once rather than calling it a second routing defect.
            let empty_route_cause = if matches!(
                route,
                Err(crate::bounded_routing::RoutingRefusal::EmptyEventRegion)
            ) {
                empty_failure
            } else {
                None
            };
            let route_id = if let Some(cause) = empty_route_cause {
                r.skip(Stage::Routing, cause)
            } else if matches!(
                route,
                Err(crate::bounded_routing::RoutingRefusal::IncompleteEventWalk(
                    _
                ))
            ) && walk.is_err()
            {
                r.skip(Stage::Routing, walk_id)
            } else {
                r.check(Stage::Routing, Family::Routing, range.clone(), route)
            };
            let Ok(walk) = walk else {
                for stage in [
                    Stage::SourceAccounting,
                    Stage::PatchTranslation,
                    Stage::ControllerTranslation,
                    Stage::ContextNoteTranslation,
                    Stage::ContextControllerTranslation,
                    Stage::SysExTranslation,
                    Stage::Translation,
                    Stage::TrackExport,
                ] {
                    r.skip(stage, walk_id);
                }
                continue;
            };
            let raw_objects = raw_source_objects(&walk);
            let source_count = if raw_objects == declared as usize {
                Ok(())
            } else {
                Err("declared_source_object_count_mismatch")
            };
            let accounting_id = r.add(
                Stage::SourceAccounting,
                if source_count.is_ok() {
                    Outcome::Passed
                } else {
                    Outcome::Failed
                },
                Family::Accounting,
                if source_count.is_ok() {
                    "passed"
                } else {
                    "declared_source_object_count_mismatch"
                },
                range.clone(),
                &[],
            );
            if let Some(empty) = empty_failure {
                if source_count.is_err() {
                    r.defer(empty, &[accounting_id]);
                }
            }
            let Ok(route) = route else {
                for stage in [
                    Stage::PatchTranslation,
                    Stage::ControllerTranslation,
                    Stage::ContextNoteTranslation,
                    Stage::ContextControllerTranslation,
                    Stage::SysExTranslation,
                    Stage::Translation,
                    Stage::TrackExport,
                ] {
                    if family_absent(&walk, stage) {
                        r.absent(stage, range.clone());
                    } else {
                        r.skip(stage, route_id);
                    }
                }
                continue;
            };
            let accounting =
                crate::controller166::qualify_track(bytes, ordinal, binding.pair_ordinal);
            let classified = patches
                .iter()
                .find(|p| {
                    p.structural_ordinal == ordinal && p.descriptor_ordinal == descriptor.ordinal
                })
                .unwrap()
                .result
                .as_ref()
                .expect("same route and walk");
            let start = r.rows.len();
            // The Controller qualifier is also the required full source gate of
            // context-Note/Controller and alternative Patch proofs. A raw count
            // match alone must not turn its refusal into new family failures.
            let controller_id = if family_absent(&walk, Stage::ControllerTranslation) {
                r.absent(Stage::ControllerTranslation, range.clone())
            } else if source_count.is_err() {
                r.skip(Stage::ControllerTranslation, accounting_id)
            } else {
                r.check(
                    Stage::ControllerTranslation,
                    Family::Translation,
                    range.clone(),
                    &accounting,
                )
            };
            let proof_prerequisite = if source_count.is_err() {
                Some(accounting_id)
            } else if accounting.is_err() {
                Some(controller_id)
            } else {
                None
            };
            for stage in [Stage::ContextNoteTranslation, Stage::SysExTranslation] {
                if family_absent(&walk, stage) {
                    r.absent(stage, range.clone());
                    continue;
                }
                let prerequisite = if stage == Stage::ContextNoteTranslation {
                    proof_prerequisite
                } else {
                    (source_count.is_err()).then_some(accounting_id)
                };
                if let Some(cause) = prerequisite {
                    r.skip(stage, cause);
                    continue;
                }
                let result = if stage == Stage::ContextNoteTranslation {
                    crate::context_note166::qualify_track(bytes, ordinal, binding.pair_ordinal)
                        .map(|_| ())
                        .map_err(code)
                } else {
                    crate::sysex166::qualify_track(bytes, ordinal, binding.pair_ordinal, &walk)
                        .map(|_| ())
                        .map_err(code)
                };
                r.add(
                    stage,
                    if result.is_ok() {
                        Outcome::Passed
                    } else {
                        Outcome::Failed
                    },
                    Family::Translation,
                    result.err().unwrap_or_else(|| "passed".into()),
                    range.clone(),
                    &[],
                );
            }
            let mut logical = 0;
            let mut patch_index = 0;
            let mut context_count = 0;
            for item in &walk.items {
                if let Some(cause) = proof_prerequisite {
                    let stage = match item {
                        MixedEventItem::PatchToNote(_) | MixedEventItem::Patch(_) => {
                            Some(Stage::PatchTranslation)
                        }
                        MixedEventItem::Event(e)
                            if matches!(e.event, MixedEventKind::ContextMediatedController(_)) =>
                        {
                            Some(Stage::ContextControllerTranslation)
                        }
                        _ => None,
                    };
                    if let Some(stage) = stage {
                        r.skip(stage, cause);
                        if stage == Stage::PatchTranslation {
                            patch_index += 1;
                        } else {
                            context_count += 1;
                        }
                        logical += item.logical_event_count() as u64;
                        continue;
                    }
                }
                match item {
                    MixedEventItem::PatchToNote(n) => {
                        let evidence = &classified.patches[patch_index];
                        let accepted = evidence.result.is_ok()
                            || patch_fallback(
                                crate::program_only166::qualify_track(
                                    bytes,
                                    ordinal,
                                    binding.pair_ordinal,
                                )
                                .ok()
                                .map(|proof| {
                                    let p = proof.provenance();
                                    p.logical_source_ordinal == logical
                                        && p.composition_range == n.representation_range
                                }),
                                || {
                                    crate::numeric_program166::qualify_current(
                                        bytes,
                                        ordinal,
                                        binding.pair_ordinal,
                                        item,
                                        route,
                                        logical,
                                    )
                                    .is_ok()
                                },
                                || {
                                    crate::bank_patch166_handoff::qualify_current(
                                        bytes,
                                        ordinal,
                                        binding.pair_ordinal,
                                        item,
                                        route,
                                        logical,
                                    )
                                    .is_ok()
                                },
                            );
                        r.add(
                            Stage::PatchTranslation,
                            if accepted {
                                Outcome::Passed
                            } else {
                                Outcome::Failed
                            },
                            Family::Translation,
                            if accepted {
                                "passed".into()
                            } else {
                                code(evidence.result.as_ref().unwrap_err())
                            },
                            Some(n.representation_range.clone()),
                            &[],
                        );
                        patch_index += 1;
                    }
                    MixedEventItem::Patch(p) => {
                        let accepted = crate::terminal_program166::qualify_track(
                            bytes,
                            ordinal,
                            binding.pair_ordinal,
                        )
                        .ok()
                        .is_some_and(|p2| {
                            p2.provenance().logical_source_ordinal == logical
                                && p2.provenance().representation_range
                                    == p.patch.representation_range
                        }) || crate::standalone_bank166::qualify_current(
                            bytes,
                            ordinal,
                            binding.pair_ordinal,
                            item,
                            route,
                            logical,
                        )
                        .is_ok();
                        r.add(
                            Stage::PatchTranslation,
                            if accepted {
                                Outcome::Passed
                            } else {
                                Outcome::Failed
                            },
                            Family::Translation,
                            if accepted {
                                "passed"
                            } else {
                                "unsupported_standalone_patch"
                            },
                            Some(p.patch.representation_range.clone()),
                            &[],
                        );
                        patch_index += 1;
                    }
                    MixedEventItem::Event(e)
                        if matches!(e.event, MixedEventKind::ContextMediatedController(_)) =>
                    {
                        r.check(
                            Stage::ContextControllerTranslation,
                            Family::Translation,
                            range.clone(),
                            &crate::context_controller166::qualify_current(
                                bytes,
                                ordinal,
                                binding.pair_ordinal,
                                item,
                                route,
                                logical,
                            ),
                        );
                        context_count += 1;
                    }
                    _ => {}
                }
                logical += item.logical_event_count() as u64;
            }
            if patch_index == 0 {
                r.absent(Stage::PatchTranslation, range.clone());
            }
            if context_count == 0 {
                r.absent(Stage::ContextControllerTranslation, range.clone());
            }
            let causes: Vec<_> = r.rows[start..]
                .iter()
                .filter(|d| matches!(d.outcome, Outcome::Failed | Outcome::NotEvaluated))
                .map(|d| d.id)
                .chain((source_count.is_err()).then_some(accounting_id))
                .chain(empty_failure)
                .collect();
            if !causes.is_empty() {
                let id = r.add(
                    Stage::Translation,
                    Outcome::NotEvaluated,
                    Family::Translation,
                    "prerequisite_failed",
                    range.clone(),
                    &causes,
                );
                r.skip(Stage::TrackExport, id);
            } else {
                let translated = crate::bounded_sequence::qualify_track_translation(
                    bytes, ordinal, binding, sequence, mute, classified, &walk,
                );
                let id = r.check(
                    Stage::Translation,
                    Family::Translation,
                    range.clone(),
                    &translated,
                );
                if let Err(error) = &translated {
                    r.rows[id].reason_code = manifest_probe(error.clone()).reason_code;
                }
                if let Ok(track) = translated {
                    let result = crate::bounded_sequence::prepare_track_export(&track);
                    let family = result
                        .as_ref()
                        .err()
                        .map(|e| match e.stage {
                            "source ordering" => Family::Ordering,
                            _ => Family::Adaptation,
                        })
                        .unwrap_or(Family::Adaptation);
                    let e = r.check(Stage::TrackExport, family, range.clone(), &result);
                    if let Err(error) = result {
                        r.rows[e].reason_code = manifest_probe(error).reason_code;
                    }
                } else {
                    r.skip(Stage::TrackExport, id);
                }
            }
        }
        // Physical framed pairs remain independently bounded even when their
        // descriptor ownership is unresolved. Never assign them a track label.
        if ownership.is_err() {
            r.descriptor = None;
            for (pair_ordinal, pair) in sequence.track_pairs.iter().enumerate() {
                r.pair = Some(pair_ordinal);
                let bounds = pair.validated_event_bounds();
                let b = r.check(
                    Stage::EventBounds,
                    Family::Bounds,
                    Some(pair.primary.record_range.clone()),
                    &bounds,
                );
                if let Ok(bounds) = bounds {
                    let range = Some(bounds.event_range.clone());
                    let walk = crate::mixed_event::walk_bounded_mixed_events(
                        bytes,
                        MixedEventBounds {
                            event_range: bounds.event_range,
                        },
                        Default::default(),
                    );
                    let id = r.check(Stage::EventWalk, Family::Decoding, range.clone(), &walk);
                    match walk {
                        Ok(walk) => {
                            let declared = u32::from_be_bytes(
                                pair.primary.payload.bytes[2..6].try_into().unwrap(),
                            );
                            let matches = raw_source_objects(&walk) == declared as usize;
                            r.add(
                                Stage::SourceAccounting,
                                if matches {
                                    Outcome::Passed
                                } else {
                                    Outcome::Failed
                                },
                                Family::Accounting,
                                if matches {
                                    "passed"
                                } else {
                                    "declared_source_object_count_mismatch"
                                },
                                range,
                                &[],
                            );
                        }
                        Err(e) => {
                            r.rows[id].cursor = walk_cursor(&e);
                            r.skip(Stage::SourceAccounting, id);
                        }
                    }
                } else {
                    let id = r.skip(Stage::EventWalk, b);
                    r.skip(Stage::SourceAccounting, id);
                }
            }
        }
        r.descriptor = None;
        r.pair = None;
        if let Some(prelude) = r.rows.iter().position(|d| {
            d.stage == Stage::Prelude
                && d.outcome == Outcome::Failed
                && ["Routing", "TrackCoverage"].contains(&d.reason_code.as_str())
        }) {
            let causes: Vec<_> = r
                .rows
                .iter()
                .filter(|d| {
                    matches!(
                        d.stage,
                        Stage::Routing
                            | Stage::EventWalk
                            | Stage::EventBounds
                            | Stage::SourceAccounting
                            | Stage::EmptyTrack
                    ) && d.outcome == Outcome::Failed
                })
                .map(|d| d.id)
                .collect();
            if !causes.is_empty() {
                r.defer(prelude, &causes);
            }
        }
        let include = general_probe(bytes, ordinal, RecoveryInclusion::IncludeMuted);
        let exclude = general_probe(bytes, ordinal, RecoveryInclusion::ExcludeMuted);
        if include.outcome == Outcome::Passed {
            r.add(
                Stage::SequenceAssembly,
                Outcome::Passed,
                Family::Serialization,
                "passed",
                Some(sequence.sequence_range.clone()),
                &[],
            );
        } else {
            let causes: Vec<_> = r
                .rows
                .iter()
                .filter(|d| d.blocks_general_recovery)
                .map(|d| d.id)
                .collect();
            if causes.is_empty() {
                // A composite production refusal does not establish that an
                // independent assembly was attempted or failed.
                let cause = r.add(
                    Stage::ProductionRecovery,
                    Outcome::Failed,
                    include.blocker_family.unwrap_or(Family::Structure),
                    format!("{}/{}", include.stage, include.reason_code),
                    Some(sequence.sequence_range.clone()),
                    &[],
                );
                r.skip(Stage::SequenceAssembly, cause);
            } else {
                r.add(
                    Stage::SequenceAssembly,
                    Outcome::NotEvaluated,
                    Family::Serialization,
                    "prerequisite_failed",
                    Some(sequence.sequence_range.clone()),
                    &causes,
                );
            }
        }
        report.sequences.push(SequenceCensus { ordinal, descriptor_count:sequence.track_descriptors().len(),pair_count:sequence.track_pairs.len(),
            general_include_muted:include,general_exclude_muted:exclude,
            application_default:Probe::skipped("application_probe_not_requested"),application_include_muted:Probe::skipped("application_probe_not_requested"),application_exclude_muted:Probe::skipped("application_probe_not_requested"),
            proven_policy_only:false,diagnostics:r.rows,coverage_limitations:vec![
                "SequenceAssembly is IncludeMuted; whole-sequence assembly is not attempted when manifest prerequisites fail".into(),
                "TrackExport is a composite adaptation/order check; its internal checks retain production order".into(),
                "Family qualifiers may stop at their first internal refusal; ranges are enclosing bounds unless an exact item/cursor is available".into(),
                "Saved-mute evidence failures are not by themselves export-policy blockers".into(),
                "Pair-only rows have no descriptor identity; successful raw walks never repair unresolved ownership".into()] });
    }
    report.recount();
    report
}

// Some(false) is a successful but differently bound program-only proof:
// production refuses it immediately, rather than trying another qualifier.
fn patch_fallback(
    program_only: Option<bool>,
    numeric: impl FnOnce() -> bool,
    bank: impl FnOnce() -> bool,
) -> bool {
    match program_only {
        Some(matches) => matches,
        None => numeric() || bank(),
    }
}

// Same logical-object/context count used by the production accounting qualifier.
fn raw_source_objects(walk: &crate::mixed_event::MixedEventWalk<'_>) -> usize {
    walk.items
        .iter()
        .map(|item| {
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
            item.logical_event_count() + contexts
        })
        .sum()
}

fn family_absent(walk: &crate::mixed_event::MixedEventWalk<'_>, stage: Stage) -> bool {
    match stage {
        Stage::PatchTranslation => !walk
            .items
            .iter()
            .any(|i| matches!(i, MixedEventItem::Patch(_) | MixedEventItem::PatchToNote(_))),
        Stage::ControllerTranslation
        | Stage::ContextNoteTranslation
        | Stage::ContextControllerTranslation
        | Stage::SysExTranslation => !walk.items.iter().any(|i| match i {
            MixedEventItem::Event(e) => matches!(
                (stage, &e.event),
                (Stage::ControllerTranslation, MixedEventKind::Controller(_))
                    | (
                        Stage::ContextNoteTranslation,
                        MixedEventKind::ContextMediatedNote(_)
                    )
                    | (
                        Stage::ContextControllerTranslation,
                        MixedEventKind::ContextMediatedController(_)
                    )
                    | (Stage::SysExTranslation, MixedEventKind::SysEx(_))
            ),
            _ => false,
        }),
        _ => false,
    }
}

fn track_stages() -> [Stage; 13] {
    [
        Stage::EventBounds,
        Stage::EmptyTrack,
        Stage::EventWalk,
        Stage::SourceAccounting,
        Stage::Routing,
        Stage::SavedMute,
        Stage::PatchTranslation,
        Stage::ControllerTranslation,
        Stage::ContextNoteTranslation,
        Stage::ContextControllerTranslation,
        Stage::SysExTranslation,
        Stage::Translation,
        Stage::TrackExport,
    ]
}
fn walk_cursor(e: &MixedEventWalkError) -> Option<usize> {
    use MixedEventWalkError::*;
    match e {
        InvalidEventBounds { .. } => None,
        TimingVlq { cursor, .. }
        | PositionOverflow { cursor, .. }
        | MissingDispatchByte { cursor, .. }
        | DataWithoutActiveState { cursor, .. }
        | UnsupportedStatus { cursor, .. }
        | MalformedSysEx { cursor, .. }
        | UnsupportedSysExSuccessor { cursor, .. }
        | UnsupportedFfTag { cursor, .. }
        | MalformedKnownTag { cursor, .. }
        | MalformedNote { cursor, .. }
        | MalformedController { cursor, .. }
        | MalformedChannelPressure { cursor, .. }
        | MalformedPitchBend { cursor, .. }
        | MalformedPatch { cursor, .. }
        | PatchContextMismatch { cursor, .. }
        | ContextLengthMismatch { cursor, .. }
        | EventPastBound { cursor, .. }
        | HighBitData { cursor, .. }
        | CursorDidNotAdvance { cursor } => Some(*cursor),
    }
}

/// Read approved paths only. The CLI additionally verifies source metadata/xattrs.
/// Errors are deliberately bounded and never contain the supplied path.
pub fn census_path(path: &Path) -> Result<ProjectCensus, &'static str> {
    let bytes = std::fs::read(path).map_err(|_| "source_unavailable")?;
    let mut report = census_bytes(&bytes);
    let mut app = AppService::new();
    let inspected = app
        .inspect_project(InspectProjectRequest {
            contract_version: CONTRACT_VERSION,
            source_path: path.to_str().ok_or("non_utf8_path")?.into(),
            diagnostics_level: DiagnosticsLevel::Full,
        })
        .map_err(|_| "application_inspection_failed")?;
    if inspected.sequences.len() != report.sequences.len() {
        return Err("application_sequence_coverage_mismatch");
    }
    for (s, summary) in report.sequences.iter_mut().zip(&inspected.sequences) {
        let request = ExportSequenceRequest {
            contract_version: CONTRACT_VERSION,
            session_id: inspected.session_id.clone(),
            sequence_id: summary.sequence_id.clone(),
            destination_folder: String::new(),
            filename_stem: String::new(),
            collision_policy: CollisionPolicy::FailIfExists,
            operation_id: None,
        };
        s.application_default = application_probe(&app, &request, None);
        // Only authenticated recovery currently permits explicit inclusion.
        let explicit = summary.bounded_export_capability.as_ref().is_some_and(|c| {
            c.contract_id == "authenticated_xform_recovery"
                || c.contract_id == "authenticated_jurrasic_recovery"
        });
        if explicit {
            s.application_include_muted =
                application_probe(&app, &request, Some(RecoveryInclusionOption::IncludeMuted));
            s.application_exclude_muted =
                application_probe(&app, &request, Some(RecoveryInclusionOption::ExcludeMuted));
        } else {
            for p in [
                &mut s.application_include_muted,
                &mut s.application_exclude_muted,
            ] {
                p.outcome = Outcome::NotApplicable;
                p.reason_code = "authority_does_not_offer_explicit_inclusion".into();
            }
        }
        s.proven_policy_only = s.general_include_muted.outcome == Outcome::Passed
            && s.general_exclude_muted.outcome == Outcome::Passed
            && s.application_default.outcome == Outcome::Failed
            && s.application_default.reason_code == "sequence_not_export_capable";
    }
    if std::fs::read(path).map_err(|_| "source_revalidation_unavailable")? != bytes {
        return Err("source_changed");
    }
    report.recount();
    Ok(report)
}
fn application_probe(
    app: &AppService,
    request: &ExportSequenceRequest,
    mode: Option<RecoveryInclusionOption>,
) -> Probe {
    match app.prepare_export_with_inclusion(request, mode) {
        Ok(_) => Probe::passed(),
        Err(e) => Probe {
            outcome: Outcome::Failed,
            stage: "application_preparation".into(),
            blocker_family: Some(
                if matches!(
                    e.diagnostic_code.as_str(),
                    "sequence_not_export_capable" | "unsupported_recovery_inclusion"
                ) {
                    Family::Authorization
                } else {
                    Family::ApplicationPreparation
                },
            ),
            source_range: None,
            cursor: None,
            reason_code: e.diagnostic_code,
        },
    }
}
impl ProjectCensus {
    fn recount(&mut self) {
        let mut m = Metrics::default();
        for s in &self.sequences {
            m.complete_include_muted +=
                usize::from(s.general_include_muted.outcome == Outcome::Passed);
            m.complete_exclude_muted +=
                usize::from(s.general_exclude_muted.outcome == Outcome::Passed);
            m.application_authorized +=
                usize::from(s.application_default.outcome == Outcome::Passed);
            m.application_not_evaluated +=
                usize::from(s.application_default.outcome == Outcome::NotEvaluated);
            m.proven_policy_only += usize::from(s.proven_policy_only);
            m.sequences_with_decoding_blockers += usize::from(
                s.diagnostics
                    .iter()
                    .any(|d| d.blocker_family == Some(Family::Decoding)),
            );
            m.sequences_with_translation_blockers += usize::from(
                s.diagnostics
                    .iter()
                    .any(|d| d.blocker_family == Some(Family::Translation)),
            );
            for d in &s.diagnostics {
                if let Some(family) = d.blocker_family {
                    let counts = if d.blocks_general_recovery {
                        &mut m.blocker_instances
                    } else {
                        &mut m.evidence_limitations
                    };
                    *counts.entry(family).or_default() += 1;
                }
                if d.outcome == Outcome::NotEvaluated {
                    for id in &d.prerequisites {
                        let cause = &s.diagnostics[*id];
                        *m.not_evaluated_by_cause
                            .entry(format!(
                                "{:?}/{:?}/{}",
                                d.stage, cause.stage, cause.reason_code
                            ))
                            .or_default() += 1;
                    }
                }
            }
        }
        for d in &self.diagnostics {
            if let Some(f) = d.blocker_family {
                *m.blocker_instances.entry(f).or_default() += 1;
            }
        }
        self.metrics = m;
    }
    pub fn summary(&self) -> String {
        let mut out=format!("Sequences: {}; complete include/exclude: {}/{}; application authorized: {}; policy-only: {}\n",self.sequences.len(),self.metrics.complete_include_muted,self.metrics.complete_exclude_muted,self.metrics.application_authorized,self.metrics.proven_policy_only);
        for s in &self.sequences {
            let failures: Vec<_> = s
                .diagnostics
                .iter()
                .filter(|d| d.outcome == Outcome::Failed)
                .map(|d| format!("{:?}:{}", d.stage, d.reason_code))
                .collect();
            let skipped = s
                .diagnostics
                .iter()
                .filter(|d| d.outcome == Outcome::NotEvaluated)
                .count();
            out.push_str(&format!(
                "{:02}: general {:?}/{:?}; app {:?}; failures [{}]; not evaluated {}\n",
                s.ordinal,
                s.general_include_muted.outcome,
                s.general_exclude_muted.outcome,
                s.application_default.outcome,
                failures.join(", "),
                skipped
            ));
        }
        out
    }
}

#[cfg(test)]
#[path = "census_tests.rs"]
mod tests;
