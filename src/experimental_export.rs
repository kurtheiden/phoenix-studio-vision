//! Opt-in conditional local Note rendering, never complete-recovery authority.
use crate::authority120::{prove_project120, TrackRoute120};
use crate::midi_export::{
    self, ChannelAssignment, ChannelAssignmentProvenance, DecodedExportEvent, MeterPolicy,
    PatchPolicy, TimingPolicy,
};
use crate::mixed_event::{MixedEventItem, MixedEventKind};
use crate::smf;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::ops::Range;

pub const LABEL: &str = "EXPERIMENTAL / INCOMPLETE / NOT VERIFIED";
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ExperimentalTrackReport {
    pub ordinal: usize,
    pub name: String,
    pub notes: usize,
    pub channel: Option<u8>,
    pub pair_ranges: Option<[Range<usize>; 2]>,
    pub saved_mute: String,
    pub inclusion: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ExperimentalRegion {
    pub range: Range<usize>,
    pub record_type: u8,
    pub role: String,
    pub sha256: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ExperimentalReport {
    pub label: String,
    pub source_sha256: String,
    pub sequence_ordinal: usize,
    pub sequence_range: Range<usize>,
    pub source_name: String,
    pub verified_refusal: String,
    pub ppqn: u16,
    pub observed_initial_mpqn: u32,
    pub observed_source_meter: (u8, u8, u8, u8),
    pub tracks: Vec<ExperimentalTrackReport>,
    pub included_notes: usize,
    pub omitted_known_notes: usize,
    /// None means UNKNOWN, not zero. No complete map/context claim is made.
    pub unresolved_semantic_event_count: Option<usize>,
    pub regions: Vec<ExperimentalRegion>,
    pub limitations: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_scope166: Option<crate::experimental166::SelectedScope166>,
}
/// Immutable plan; callers cannot supply events, routing, identity or omissions.
#[derive(Debug)]
pub struct ExperimentalPlan {
    pub(crate) session_id: crate::app_contract::SessionId,
    pub(crate) sequence_id: crate::app_contract::SequenceId,
    pub(crate) report: ExperimentalReport,
    pub(crate) midi: Vec<u8>,
    pub(crate) selection166: Option<Vec<crate::experimental166::TrackIdentity166>>,
}
impl ExperimentalPlan {
    pub fn report(&self) -> &ExperimentalReport {
        &self.report
    }
}
#[derive(Debug)]
pub struct ExperimentalExportResponse {
    pub label: &'static str,
    pub output_path: String,
    pub limitations_path: String,
    pub report: ExperimentalReport,
    pub publication_warnings: Vec<crate::app_contract::Warning>,
}
#[derive(Debug, PartialEq, Eq)]
pub struct ExperimentalExportError(pub String);
impl std::fmt::Display for ExperimentalExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Experimental export refused: {}", self.0)
    }
}
impl std::error::Error for ExperimentalExportError {}
fn err(e: impl std::fmt::Debug) -> ExperimentalExportError {
    ExperimentalExportError(format!("{e:?}"))
}

pub(crate) fn build(
    source: &[u8],
    ordinal: usize,
) -> Result<(ExperimentalReport, Vec<u8>), ExperimentalExportError> {
    let proofs = prove_project120(source).map_err(err)?;
    let proof = proofs
        .into_iter()
        .find(|p| p.ordinal == ordinal)
        .ok_or_else(|| err("unknown object"))?
        .result
        .map_err(err)?;
    let bridge = crate::semantic_layout120::associate_project_120(source).map_err(err)?;
    let association = bridge
        .candidates
        .into_iter()
        .nth(ordinal)
        .ok_or_else(|| err("unknown association"))?
        .association
        .map_err(err)?;
    if association.sequence_range != proof.range() || association.source_name.bytes != proof.name()
    {
        return Err(err("identity"));
    }
    // The known padded primary grammar identifies exactly ONE initial event.
    // Its unknown secondary/context applicability is reported, never accepted
    // as a complete conductor by this path.
    for (i, special) in association.leading_special.iter().enumerate() {
        let p = special.pair.primary.payload.bytes;
        let n = if i == 0 { 8 } else { 7 };
        let tail = 14 + n;
        if p.len() != 38
            || p[..2] != [0, 1]
            || p[2..6] != 1u32.to_be_bytes()
            || p[6..10] != [0; 4]
            || p[tail..tail + 7] != [135, 255, 255, 127, 255, 47, 0]
            || p[tail + 7..].iter().any(|x| *x != 0)
        {
            return Err(err("unsafe initial conductor position/count/envelope"));
        }
    }
    let m = &association.leading_special[0].pair.primary.payload.range;
    let t = &association.leading_special[1].pair.primary.payload.range;
    let meter = crate::meter::decode_bounded_initial_meter(
        source,
        crate::meter::InitialMeterBounds {
            event_range: m.start + 14..m.start + 22,
        },
    )
    .map_err(err)?;
    let tempo = crate::tempo::decode_bounded_initial_tempo(
        source,
        crate::tempo::InitialTempoBounds {
            event_range: t.start + 14..t.start + 21,
        },
    )
    .map_err(err)?;
    let meter_values = (
        meter.numerator.value,
        meter.denominator_exponent.value,
        meter.third_payload.value,
        meter.fourth_payload.value,
    );
    let title = format!("{LABEL}: {}", String::from_utf8_lossy(proof.name()));
    let conductor = midi_export::adapt_conductor(
        title.as_bytes(),
        tempo.mpqn(),
        meter_values,
        TimingPolicy::Identity480,
        MeterPolicy::KnownHistoricalOnly,
    )
    .map_err(err)?;
    let mut chunks = vec![smf::serialize_conductor_track(
        &conductor.sequence_name,
        conductor.tempo_mpqn,
        conductor.time_signature,
    )
    .map_err(err)?];
    let mut expected = vec![crate::readback120::conductor_expected(&conductor, None)];
    let mut tracks = Vec::new();
    for track in proof.tracks() {
        let mut events = Vec::new();
        if let Some(walk) = track.walk() {
            if walk.event_range != walk.consumed_range {
                return Err(err("incomplete consumption"));
            }
            for (i, item) in walk.items.iter().enumerate() {
                let MixedEventItem::Event(event) = item else {
                    return Err(err("unsupported included Patch/context"));
                };
                let MixedEventKind::Note(note) = &event.event else {
                    return Err(err("unsupported included musical event"));
                };
                events.push(DecodedExportEvent::from_note(
                    event.position,
                    i as u64,
                    note,
                ));
            }
        }
        let channel = match track.route() {
            TrackRoute120::NoEvents if events.is_empty() => None,
            TrackRoute120::Single(route) => Some(route.channel),
            _ => return Err(err("unresolved or unsupported route")),
        };
        let assignment = channel
            .map(|c| {
                smf::MidiChannel::new(c).map(|channel| ChannelAssignment {
                    channel,
                    provenance: ChannelAssignmentProvenance::ParsedRouting,
                })
            })
            .transpose()
            .map_err(err)?;
        let (scheduled, provenance) = if events.is_empty() {
            (Vec::new(), Vec::new())
        } else {
            let adapted = midi_export::adapt_track(
                &events,
                assignment,
                TimingPolicy::Identity480,
                PatchPolicy::StrictKnownOnly,
            )
            .map_err(err)?;
            if !adapted.warnings.is_empty() || !adapted.untranslated_metadata.is_empty() {
                return Err(err("unreported translation"));
            }
            (adapted.scheduled_events, adapted.note_provenance)
        };
        let name = midi_export::adapt_text(track.name()).map_err(err)?;
        chunks.push(
            smf::serialize_named_musical_track_with_note_provenance(&name, &scheduled, &provenance)
                .map_err(err)?,
        );
        expected.push(crate::readback120::musical_expected(&name, &scheduled));
        tracks.push(ExperimentalTrackReport {
            ordinal: track.ordinal(),
            name: String::from_utf8_lossy(track.name()).into_owned(),
            notes: events.len(),
            channel,
            pair_ranges: track.pair_ranges().cloned(),
            saved_mute: "Unknown: no saved-mute policy established by this 120 proof".into(),
            inclusion: "All proved ordinary Tracks explicitly selected; no automatic mute omission"
                .into(),
        });
    }
    let included_notes = tracks.iter().map(|t| t.notes).sum();
    if included_notes == 0 {
        return Err(err("no playable proven Notes"));
    }
    let root = crate::sequence_container::parse_root_record_stream(source).map_err(err)?;
    let mut cursor = proof.range().start;
    let mut regions = Vec::new();
    for record in root.records.iter().filter(|r| {
        r.record_range.start >= proof.range().start && r.record_range.end <= proof.range().end
    }) {
        if record.record_range.start != cursor {
            return Err(err("unreported sequence bytes"));
        }
        cursor = record.record_range.end;
        let ordinary = tracks.iter().any(|t| {
            t.pair_ranges
                .as_ref()
                .is_some_and(|p| p.contains(&record.record_range))
        });
        regions.push(ExperimentalRegion{range:record.record_range.clone(),record_type:record.record_type.value,role:if ordinary {"Proved Note pair; original framing/context retained as source provenance"} else {"Project/conductor/terminal region: NOT rendered as source semantics; applicability unresolved"}.into(),sha256:format!("{:x}",Sha256::digest(&source[record.record_range.clone()]))});
    }
    if cursor != proof.range().end {
        return Err(err("unreported terminal bytes"));
    }
    let verified_refusal = crate::recovery120::recover_project120(source)
        .map_err(err)?
        .into_iter()
        .find(|c| c.ordinal == ordinal)
        .ok_or_else(|| err("missing verified object"))?
        .result
        .err()
        .ok_or_else(|| err("already verified; use normal export"))?;
    let report=ExperimentalReport{label:LABEL.into(),source_sha256:format!("{:x}",Sha256::digest(source)),sequence_ordinal:ordinal,sequence_range:proof.range(),source_name:String::from_utf8_lossy(proof.name()).into_owned(),verified_refusal:format!("{verified_refusal:?}"),ppqn:conductor.ppqn,observed_initial_mpqn:tempo.mpqn(),observed_source_meter:meter_values,tracks,included_notes,omitted_known_notes:0,unresolved_semantic_event_count:None,regions,selected_scope166:None,limitations:vec![
        "Conditional local-timeline rendering, NOT a complete or verified recovery.".into(),
        "Only observed initial tempo/meter values used; complete map, applicability and synchronization are UNRESOLVED. No default tempo invented.".into(),
        "Conductor wrappers, terminal duration and other project metadata are not translated. Every framed region is identified by range/hash; unknown semantic event count is UNKNOWN.".into(),
        "All independently proved ordinary Notes included; original mute state Unknown and historical inclusion not reproduced. Physical instrument/timbre is not guaranteed by routed MIDI channel.".into(),
        "End-of-Track closes the rendered last release, not the original sequence duration. Labels may be stripped by another application.".into()]};
    let json = serde_json::to_vec(&report).map_err(err)?;
    chunks[0] = smf::serialize_conductor_track_with_text(
        &conductor.sequence_name,
        conductor.tempo_mpqn,
        conductor.time_signature,
        &json,
    )
    .map_err(err)?;
    let eot = expected[0].len() - 1;
    expected[0].insert(eot, (0, 1, json));
    let midi = smf::serialize_format1(conductor.ppqn, &chunks).map_err(err)?;
    crate::readback120::verify(&midi, conductor.ppqn, &expected).map_err(err)?;
    Ok((report, midi))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture(events: &[u8], count: u32) -> Vec<u8> {
        let original = crate::recovery120::tests::fixture(events, count);
        let root = crate::sequence_container::parse_root_record_stream(&original).unwrap();
        let mut out = original[..8].to_vec();
        let mut changed = false;
        for record in &root.records {
            let mut payload = record.payload.bytes.to_vec();
            if !changed && record.record_type.value == 2 {
                // Synthetic padded initial-only Meter, with unresolved secondary
                // count/copy: deliberately not eligible for verified recovery.
                payload[2..6].copy_from_slice(&1u32.to_be_bytes());
                payload.splice(14..20, []);
                payload.resize(38, 0);
                changed = true;
            }
            if record.record_type.value == 2
                && payload.len() == 28
                && payload[14..17] == [0, 255, 81]
            {
                payload.resize(38, 0);
            }
            out.push(record.record_type.value);
            out.extend((payload.len() as u32).to_be_bytes());
            out.extend(payload);
        }
        out
    }
    #[test]
    fn portable_notes_have_exact_consumption_timing_and_complete_region_ledger() {
        let note = crate::recovery120::tests::note();
        let b = fixture(&note, 1);
        let (report, midi) = build(&b, 0).unwrap();
        assert_eq!(report.included_notes, 1);
        assert_eq!(report.tracks[0].channel, Some(5));
        assert_eq!(report.observed_initial_mpqn, 500000);
        assert_eq!(report.observed_source_meter, (3, 2, 8, 8));
        assert!(midi
            .windows(8)
            .any(|w| w == [0x94, 60, 100, 0x81, 0x70, 0x84, 60, 64]));
        assert_eq!(
            report.regions.first().unwrap().range.start,
            report.sequence_range.start
        );
        assert_eq!(
            report.regions.last().unwrap().range.end,
            report.sequence_range.end
        );
        assert!(report
            .regions
            .windows(2)
            .all(|w| w[0].range.end == w[1].range.start));
        assert!(crate::recovery120::recover_project120(&b).unwrap()[0]
            .result
            .is_err());
    }
    #[test]
    fn unsupported_program_and_all_incomplete_note_prefixes_refuse() {
        assert!(build(&fixture(&[0, 0xc0, 23], 1), 0)
            .unwrap_err()
            .0
            .contains("unsupported included musical event"));
        let note = crate::recovery120::tests::note();
        for n in 0..note.len() {
            assert!(build(&fixture(&note[..n], 1), 0).is_err(), "prefix {n}");
        }
        assert!(build(&fixture(&note, 2), 0).is_err());
    }
    #[test]
    fn non_equivalent_same_tick_same_pitch_notes_refuse() {
        let first = crate::recovery120::tests::note();
        let mut second = first.clone();
        second[3] = 99;
        let bytes = [first, second].concat();
        assert!(build(&fixture(&bytes, 2), 0)
            .unwrap_err()
            .0
            .contains("AmbiguousSourceOrder"));
    }
    #[test]
    fn independently_paired_empty_track_is_retained_without_inventing_channel() {
        let original = fixture(&crate::recovery120::tests::note(), 1);
        let root = crate::sequence_container::parse_root_record_stream(&original).unwrap();
        let mut out = original[..8].to_vec();
        for record in &root.records {
            let mut payload = record.payload.bytes.to_vec();
            if record.record_type.value == 1 {
                payload[0] = 4;
                payload.extend([0; 120]);
                let label = 8 + 121 + 360 - record.payload.range.start;
                payload[label..label + 5].copy_from_slice(b"Empty");
                payload[label - 39..label - 31].copy_from_slice(&[0x80, 0, 4, 0, 0, 4, 1, 0]);
            }
            if record.record_type.value == 0 {
                let mut empty = vec![0; 14];
                empty.extend([255, 0, 0, 0, 255, 47, 0]);
                for (tag, p) in [(2, empty), (41, vec![0; 23])] {
                    out.push(tag);
                    out.extend((p.len() as u32).to_be_bytes());
                    out.extend(p);
                }
            }
            out.push(record.record_type.value);
            out.extend((payload.len() as u32).to_be_bytes());
            out.extend(payload);
        }
        let (report, _) = build(&out, 0).unwrap();
        assert_eq!(report.tracks.len(), 2);
        assert_eq!(report.tracks[1].notes, 0);
        assert_eq!(report.tracks[1].channel, None);
    }
    #[test]
    fn unsafe_initial_position_unknown_channel_and_ownership_refuse() {
        let b = fixture(&crate::recovery120::tests::note(), 1);
        let root = crate::sequence_container::parse_root_record_stream(&b).unwrap();
        let meter = root
            .records
            .iter()
            .find(|r| r.record_type.value == 2)
            .unwrap();
        let mut bad = b.clone();
        bad[meter.payload.range.start + 14] = 1;
        assert!(build(&bad, 0).is_err());
        let mut bad = b.clone();
        for r in &root.records {
            if r.record_type.value == 16 {
                bad[r.payload.range.start + 27] = 255;
            }
        }
        assert!(build(&bad, 0).is_err());
        let tempo = root
            .records
            .iter()
            .filter(|r| r.record_type.value == 2)
            .nth(1)
            .unwrap();
        let mut bad = b.clone();
        bad[tempo.payload.range.start + 18..tempo.payload.range.start + 21].fill(0);
        assert!(build(&bad, 0).is_err());
        let mut bad = b.clone();
        bad[root.records[0].payload.range.start] = 4;
        assert!(build(&bad, 0).is_err());
    }
}
