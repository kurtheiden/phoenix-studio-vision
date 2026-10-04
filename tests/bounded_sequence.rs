use phoenix::bounded_sequence::*;
use phoenix::sequence_container::parse_project_166;
use sha2::{Digest, Sha256};

const PROJECT: &str = "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline";
const MIDI: &str = "/Users/kurtheiden/Documents/Phoenix Research/Studio Vision MIDI Exports/Project 001/Seq R Test";
const NOTE: &[u8] = &[0, 0x90, 60, 127, 113, 102];

fn record(b: &mut Vec<u8>, tag: u8, p: &[u8]) {
    b.push(tag);
    b.extend((p.len() as u32).to_be_bytes());
    b.extend(p);
}
fn synthetic(events: &[u8]) -> Vec<u8> {
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
fn accepted(b: &[u8]) -> BoundedSequenceManifest {
    build_bounded_sequence_manifest(b, 0, 480).unwrap()
}
fn refused(b: &[u8]) -> ManifestRefusal {
    assert_application_refusal(b);
    build_bounded_sequence_manifest(b, 0, 480).unwrap_err()
}
#[test]
fn synthetic_note_and_patch_manifest_without_corpus_identity() {
    let mut events = NOTE.to_vec();
    events.extend(patch(16));
    // Separate same-pitch notes to avoid deliberately ambiguous simultaneous attacks.
    let n = events.len();
    events[n - 6] = 120;
    let b = synthetic(&events);
    let m = accepted(&b);
    assert_eq!(m.name(), b"New!");
    assert_eq!(m.ppqn(), 480);
    assert_eq!(m.tracks().len(), 1);
    assert_eq!(m.tracks()[0].events().len(), 3);
    assert_eq!(m.tracks()[0].routing().midi_channel, 4);
    assert_eq!(
        &assemble_bounded_sequence(&m).unwrap()[8..14],
        &[0, 1, 0, 2, 1, 224]
    );
}
#[test]
fn arbitrary_sequence_track_device_and_patch_names_accept() {
    let b = synthetic(&patch(35));
    let mut changed = b.clone();
    let p = parse_project_166(&b).unwrap();
    let s = &p.sequences[0];
    changed[s.sequence_name.bytes.range.clone()].copy_from_slice(b"Else");
    let label = s.track_descriptors()[0].label.as_ref().unwrap();
    changed[label.range.clone()].copy_from_slice(b"Fresh");
    accepted(&changed);
    assert_ne!(Sha256::digest(&b), Sha256::digest(&changed));
}
#[test]
fn nonaccepted_division_refuses() {
    for ppqn in [0, 96, 960, 0xe728] {
        assert_eq!(
            build_bounded_sequence_manifest(&synthetic(NOTE), 0, ppqn)
                .unwrap_err()
                .stage,
            "division"
        );
    }
}
#[test]
fn unresolved_pair_ownership_refuses() {
    let b = synthetic(NOTE);
    let p = parse_project_166(&b).unwrap();
    let pair = &p.sequences[0].track_pairs[0];
    let mut changed = b.clone();
    changed.drain(pair.primary.record_range.start..pair.secondary.record_range.end);
    assert_eq!(refused(&changed).stage, "ownership");
}
#[test]
fn blank_label_refuses() {
    let mut b = synthetic(NOTE);
    let p = parse_project_166(&b).unwrap();
    let range = p.sequences[0].track_descriptors()[0]
        .label
        .as_ref()
        .unwrap()
        .range
        .clone();
    b[range].fill(b' ');
    assert_eq!(refused(&b).stage, "mute");
}
#[test]
fn empty_track_refuses() {
    refused(&synthetic(&[]));
}
#[test]
fn incomplete_walk_refuses() {
    refused(&synthetic(&NOTE[..5]));
}
#[test]
fn mute_on_refuses() {
    mute_mutation(136);
}
#[test]
fn unknown_mute_refuses() {
    mute_mutation(129);
}
fn mute_mutation(value: u8) {
    let mut b = synthetic(NOTE);
    let p = parse_project_166(&b).unwrap();
    let offset = p.sequences[0].track_descriptors()[0].label_start - 39;
    b[offset] = value;
    assert_eq!(refused(&b).stage, "mute");
}
#[test]
fn invalid_routing_refuses() {
    let mut b = synthetic(NOTE);
    b[49 + 5 + 27] = 16;
    assert_eq!(refused(&b).stage, "routing");
}
#[test]
fn unsupported_patch_refuses() {
    assert_eq!(refused(&synthetic(&patch(17))).stage, "Patch");
}
#[test]
fn ambiguous_source_order_refuses() {
    let mut notes = NOTE.to_vec();
    let mut unequal = NOTE.to_vec();
    unequal[3] -= 1; // Same pitch/start, unequal attack: still genuinely ambiguous.
    notes.extend(unequal);
    assert_eq!(refused(&synthetic(&notes)).stage, "source ordering");
}
#[test]
fn bounded_sequence_retains_equivalent_duplicate_notes() {
    let mut notes = NOTE.to_vec();
    notes.extend(NOTE);
    let manifest = accepted(&synthetic(&notes));
    assert_eq!(manifest.tracks()[0].events().len(), 2);
    let midi = assemble_bounded_sequence(&manifest).unwrap();
    assert_eq!(midi.windows(3).filter(|w| *w == [0x93, 60, 127]).count(), 2);
    assert_eq!(midi.windows(3).filter(|w| w[..2] == [0x83, 60]).count(), 2);
}
#[test]
fn unsupported_conductor_prelude_refuses() {
    let mut b = synthetic(NOTE);
    let pos = parse_project_166(&b).unwrap().sequences[0]
        .meter_primary
        .record_range
        .start;
    b.splice(pos..pos, [9, 0, 0, 0, 0]);
    assert_eq!(refused(&b).stage, "conductor prelude");
}
fn conductor_mutation(tempo: bool, delta: usize, value: u8, copy: bool) -> Vec<u8> {
    let mut b = synthetic(NOTE);
    let p = parse_project_166(&b).unwrap();
    let s = &p.sequences[0];
    let start = if tempo {
        s.initial_tempo_range.start
    } else {
        s.initial_meter_range.start
    };
    let secondary = if tempo {
        s.tempo_secondary.payload.range.start
    } else {
        s.meter_secondary.payload.range.start
    };
    b[start + delta] = value;
    if copy {
        b[secondary + 33 + delta - 4] = value;
    }
    b
}
#[test]
fn invalid_initial_tempo_refuses() {
    refused(&conductor_mutation(true, 2, 80, false));
}
#[test]
fn zero_initial_tempo_refuses() {
    let mut b = synthetic(NOTE);
    let p = parse_project_166(&b).unwrap();
    let s = &p.sequences[0];
    let r = s.initial_tempo_range.start;
    let q = s.tempo_secondary.payload.range.start;
    b[r + 4..r + 7].fill(0);
    b[q + 33..q + 36].fill(0);
    assert_eq!(refused(&b).stage, "conductor adaptation");
}
#[test]
fn invalid_initial_meter_refuses() {
    refused(&conductor_mutation(false, 2, 80, false));
}
#[test]
fn unknown_meter_mapping_refuses() {
    refused(&conductor_mutation(false, 6, 7, true));
}
#[test]
fn meter_fallback_refuses() {
    assert_eq!(
        refused(&conductor_mutation(false, 7, 128, true)).stage,
        "meter"
    );
}
#[test]
fn contradictory_secondary_refuses() {
    assert_eq!(
        refused(&conductor_mutation(true, 6, 121, false)).stage,
        "conductor coverage"
    );
}
fn extra_content(tempo: bool) {
    let mut b = synthetic(NOTE);
    let p = parse_project_166(&b).unwrap();
    let s = &p.sequences[0];
    let primary = if tempo {
        &s.tempo_primary
    } else {
        &s.meter_primary
    };
    let event = if tempo {
        s.initial_tempo_range.clone()
    } else {
        s.initial_meter_range.clone()
    };
    let addition = b[event].to_vec();
    let pos = primary.payload.range.end;
    let len = primary.payload_length + addition.len() as u32;
    let header = primary.length_bytes.range.clone();
    b[header].copy_from_slice(&len.to_be_bytes());
    b.splice(pos..pos, addition);
    assert_eq!(refused(&b).stage, "conductor coverage");
}
#[test]
fn additional_tempo_refuses() {
    extra_content(true);
}
#[test]
fn additional_meter_refuses() {
    extra_content(false);
}
#[test]
fn missing_initial_tempo_refuses() {
    refused(&conductor_mutation(true, 1, 0, false));
}
#[test]
fn missing_initial_meter_refuses() {
    refused(&conductor_mutation(false, 1, 0, false));
}
#[test]
fn unexpected_sequence_object_refuses() {
    let mut b = synthetic(NOTE);
    let pos = parse_project_166(&b).unwrap().sequences[0]
        .terminal_record
        .record_range
        .start;
    b.splice(pos..pos, [9, 0, 0, 0, 1, 0]);
    assert_eq!(refused(&b).stage, "structure");
}
// Strict small SMF reader for direct native-vs-generated message comparison.
// Retains order and resolved status bytes; does not infer bank semantics.
#[derive(Debug, PartialEq, Eq)]
struct Message {
    tick: u32,
    bytes: Vec<u8>,
}
struct Track {
    name: Option<Vec<u8>>,
    messages: Vec<Message>,
    meta: Vec<(u32, u8, Vec<u8>)>,
}
fn vlq(b: &[u8], pos: &mut usize) -> u32 {
    let mut v = 0;
    for _ in 0..4 {
        let x = b[*pos];
        *pos += 1;
        v = (v << 7) | u32::from(x & 127);
        if x < 128 {
            return v;
        }
    }
    panic!("invalid VLQ")
}
fn track(b: &[u8]) -> Track {
    let (mut pos, mut tick, mut status, mut name, mut messages, mut eot) =
        (0, 0, None, None, Vec::new(), 0);
    let mut meta = Vec::new();
    while pos < b.len() {
        tick += vlq(b, &mut pos);
        let first = b[pos];
        let s = if first >= 128 {
            pos += 1;
            first
        } else {
            status.expect("running status")
        };
        match s {
            255 => {
                status = None;
                let kind = b[pos];
                pos += 1;
                let len = vlq(b, &mut pos) as usize;
                let data = &b[pos..pos + len];
                pos += len;
                if matches!(kind, 3 | 81 | 88) {
                    meta.push((tick, kind, data.to_vec()));
                }
                if kind == 3 {
                    name = Some(data.to_vec());
                }
                if kind == 47 {
                    assert_eq!(len, 0);
                    assert_eq!(pos, b.len());
                    eot += 1;
                }
            }
            240 | 247 => {
                status = None;
                let len = vlq(b, &mut pos) as usize;
                pos += len;
                assert!(pos <= b.len());
            }
            _ => {
                assert!((128..240).contains(&s));
                status = Some(s);
                let len = if matches!(s >> 4, 12 | 13) { 1 } else { 2 };
                let data = &b[pos..pos + len];
                assert!(data.iter().all(|x| *x < 128));
                pos += len;
                let mut bytes = vec![s];
                bytes.extend(data);
                messages.push(Message { tick, bytes });
            }
        }
    }
    assert_eq!(eot, 1);
    Track {
        name,
        messages,
        meta,
    }
}
fn smf(b: &[u8]) -> Vec<Track> {
    assert_eq!(
        &b[..14],
        &[b'M', b'T', b'h', b'd', 0, 0, 0, 6, 0, 1, 0, 3, 1, 224]
    );
    let mut pos = 14;
    let mut out = Vec::new();
    for _ in 0..3 {
        assert_eq!(&b[pos..pos + 4], b"MTrk");
        let len = u32::from_be_bytes(b[pos + 4..pos + 8].try_into().unwrap()) as usize;
        out.push(track(&b[pos + 8..pos + 8 + len]));
        pos += 8 + len;
    }
    assert_eq!(pos, b.len());
    out
}

#[test]
fn authenticated_native_sequence_reconciliation() {
    let b = std::fs::read(PROJECT).unwrap();
    assert_eq!(b.len(), 211468);
    assert_eq!(
        format!("{:x}", Sha256::digest(&b)),
        "e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132"
    );
    let native = std::fs::read(MIDI).unwrap();
    assert_eq!(native.len(), 1025);
    assert_eq!(
        format!("{:x}", Sha256::digest(&native)),
        "a97c0c3e99e95bc0e97941c8823e90f81758c94112c7dd1a64ae707250eca887"
    );
    let m = build_bounded_sequence_manifest(&b, 17, 480).unwrap();
    assert_eq!(m.tracks().len(), 2);
    assert_eq!(m.conductor().tempo().mpqn(), 472440);
    let (output, receipt) = application_export(&b, 17);
    assert_eq!(output, assemble_bounded_sequence(&m).unwrap());
    assert!(receipt.compatibility_profile.is_none());
    assert!(receipt.bounded_export_capability.is_some());
    assert_eq!(receipt.counts.notes, 99);
    assert_eq!(receipt.counts.generated_note_offs, 99);
    assert_eq!(receipt.counts.bank_select_msb, 2);
    assert_eq!(receipt.counts.bank_select_lsb, 0);
    assert_eq!(receipt.counts.programs, 2);
    assert_eq!(receipt.counts.controllers, 0);
    assert_eq!(receipt.untranslated_metadata_count, 0);
    assert!(receipt.warnings.is_empty());
    let generated = smf(&output);
    let mut native = smf(&native);
    assert_eq!(generated[0].meta, native[0].meta);
    let mut normalized = 0;
    let mut counts = [0; 5];
    for i in 1..3 {
        assert_eq!(generated[i].name, native[i].name);
        assert_eq!(generated[i].messages.len(), [76, 126][i - 1]);
        for (actual, expected) in generated[i].messages.iter().zip(&mut native[i].messages) {
            assert_eq!(actual.bytes[0] & 15, (i + 1) as u8);
            if expected.bytes[0] >> 4 == 9 && expected.bytes[2] == 0 {
                assert_eq!(actual.tick, expected.tick);
                assert_eq!(actual.bytes[0], 0x80 | (expected.bytes[0] & 15));
                assert_eq!(actual.bytes[1], expected.bytes[1]);
                expected.bytes = actual.bytes.clone();
                normalized += 1;
            }
            assert_eq!(actual, expected);
            match actual.bytes[0] >> 4 {
                9 => counts[0] += 1,
                11 => match actual.bytes[1] {
                    0 => counts[1] += 1,
                    32 => counts[3] += 1,
                    _ => counts[4] += 1,
                },
                12 => counts[2] += 1,
                8 => {}
                _ => panic!("unexpected channel family"),
            }
        }
        assert_eq!(generated[i].messages, native[i].messages);
    }
    assert_eq!(counts, [99, 2, 2, 0, 0]);
    assert_eq!(normalized, 1);
    assert_eq!(
        generated[2]
            .messages
            .iter()
            .filter(|m| m.tick == 0)
            .map(|m| m.bytes.clone())
            .collect::<Vec<_>>(),
        vec![
            vec![0x93, 0x3c, 0x7f],
            vec![0xb3, 0, 0x50],
            vec![0xc3, 0x10]
        ]
    );
}

#[test]
fn parsed_controller_has_no_translation_in_this_contract() {
    let b = synthetic(&[0, 255, 65, 5, 0, 1, 0, 7, 64]);
    assert_eq!(refused(&b).stage, "translation");
}
#[test]
fn complete_conductor_envelope_is_guarded() {
    let b = synthetic(NOTE);
    let p = parse_project_166(&b).unwrap();
    let s = &p.sequences[0];
    for (primary, secondary, event) in [
        (&s.meter_primary, &s.meter_secondary, &s.initial_meter_range),
        (&s.tempo_primary, &s.tempo_secondary, &s.initial_tempo_range),
    ] {
        for offset in primary
            .payload
            .range
            .clone()
            .chain(secondary.payload.range.clone())
        {
            if (primary.payload.range.start + 10..primary.payload.range.start + 14)
                .contains(&offset)
                || event.contains(&offset)
            {
                continue;
            }
            let mut changed = b.clone();
            changed[offset] ^= 1;
            assert_eq!(
                refused(&changed).stage,
                "conductor coverage",
                "offset {offset}"
            );
        }
    }
}
#[test]
fn one_unacceptable_track_refuses_the_whole_native_sequence() {
    let mut b = std::fs::read(PROJECT).unwrap();
    let p = parse_project_166(&b).unwrap();
    let offset = p.sequences[17].track_descriptors()[1].label_start - 39;
    b[offset] = 136;
    assert_eq!(
        build_bounded_sequence_manifest(&b, 17, 480)
            .unwrap_err()
            .stage,
        "mute"
    );
}
#[test]
fn authorization_firewall_and_exact_profile_precedence_remain() {
    use phoenix::app_contract::*;
    use phoenix::app_service::AppService;
    let b = std::fs::read(PROJECT).unwrap();
    let mut service = AppService::new();
    let response = service
        .inspect_project(InspectProjectRequest {
            contract_version: CONTRACT_VERSION,
            source_path: PROJECT.into(),
            diagnostics_level: DiagnosticsLevel::Summary,
        })
        .unwrap();
    let ready = response
        .sequences
        .iter()
        .filter(|s| s.readiness == Readiness::Ready)
        .map(|s| s.display_name.as_str())
        .collect::<Vec<_>>();
    for name in [
        "Bells for her",
        "Girl-U-Want",
        "Sequence K",
        "Ode to Clarke",
        "Over the Top",
        "Sequence Q",
    ] {
        assert!(ready.contains(&name));
        let row = response
            .sequences
            .iter()
            .find(|s| s.display_name == name)
            .unwrap();
        assert!(row.export_capability.is_some());
        assert!(row.bounded_export_capability.is_none());
    }
    let r = response
        .sequences
        .iter()
        .find(|s| s.display_name == "Sequence R")
        .unwrap();
    let before = service
        .assessment_for_sequence(&response.session_id, &r.sequence_id)
        .unwrap();
    let manifest = build_bounded_sequence_manifest(&b, 17, 480).unwrap();
    assemble_bounded_sequence(&manifest).unwrap();
    assert_eq!(r.readiness, Readiness::Ready);
    assert!(r.bounded_export_capability.is_some());
    assert!(r.export_capability.is_none());
    assert!(!before.has_resolved_policy);
    assert!(before.capability.is_none());
    // A manifest is not accepted by any application API; the session authority
    // is independently established by inspection and refreshed by export.
    let (_, receipt) = application_export(&b, 17);
    assert!(receipt.compatibility_profile.is_none());
    let after = service
        .assessment_for_sequence(&response.session_id, &r.sequence_id)
        .unwrap();
    assert!(!after.has_resolved_policy);
    assert!(after.capability.is_none());
}

fn application_export(
    bytes: &[u8],
    ordinal: usize,
) -> (Vec<u8>, phoenix::app_contract::ExportSequenceResponse) {
    use phoenix::app_contract::*;
    let directory = TestDirectory::new();
    let source = directory.path().join("unrelated-project");
    std::fs::write(&source, bytes).unwrap();
    let mut service = phoenix::app_service::AppService::new();
    let inspected = service
        .inspect_project(InspectProjectRequest {
            contract_version: CONTRACT_VERSION,
            source_path: source.to_str().unwrap().into(),
            diagnostics_level: DiagnosticsLevel::Full,
        })
        .unwrap();
    let sequence = &inspected.sequences[ordinal];
    assert_eq!(sequence.readiness, Readiness::Ready);
    let receipt = service
        .export_sequence(ExportSequenceRequest {
            contract_version: CONTRACT_VERSION,
            session_id: inspected.session_id,
            sequence_id: sequence.sequence_id.clone(),
            destination_folder: directory.path().to_str().unwrap().into(),
            filename_stem: "output".into(),
            collision_policy: CollisionPolicy::FailIfExists,
            operation_id: None,
        })
        .unwrap();
    (std::fs::read(&receipt.output_path).unwrap(), receipt)
}

#[test]
fn unrelated_bounded_application_exports_reinspected_changed_identity() {
    let b = synthetic(&patch(35));
    let (first, receipt) = application_export(&b, 0);
    assert!(receipt.compatibility_profile.is_none());
    let mut changed = b.clone();
    let p = parse_project_166(&b).unwrap();
    changed[p.sequences[0].sequence_name.bytes.range.clone()].copy_from_slice(b"Else");
    changed[p.sequences[0].track_descriptors()[0]
        .label
        .as_ref()
        .unwrap()
        .range
        .clone()]
    .copy_from_slice(b"Fresh");
    changed[96..100].copy_from_slice(b"Name");
    let (second, receipt) = application_export(&changed, 0);
    assert!(receipt.bounded_export_capability.is_some());
    assert_ne!(first, second);
    assert_ne!(Sha256::digest(&b), Sha256::digest(&changed));
}

struct TestDirectory(std::path::PathBuf);
impl TestDirectory {
    fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!(
            "phoenix-bounded-{}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn path(&self) -> &std::path::Path {
        &self.0
    }
}
impl Drop for TestDirectory {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.0).unwrap();
    }
}

fn assert_application_refusal(bytes: &[u8]) {
    use phoenix::app_contract::*;
    let directory = TestDirectory::new();
    let source = directory.path().join("unsupported");
    std::fs::write(&source, bytes).unwrap();
    let mut service = phoenix::app_service::AppService::new();
    let inspected = service
        .inspect_project(InspectProjectRequest {
            contract_version: CONTRACT_VERSION,
            source_path: source.to_str().unwrap().into(),
            diagnostics_level: DiagnosticsLevel::Full,
        })
        .unwrap();
    for sequence in &inspected.sequences {
        assert_ne!(sequence.readiness, Readiness::Ready);
        assert!(sequence.export_capability.is_none());
        assert!(sequence.bounded_export_capability.is_none());
        let error = service
            .export_sequence(ExportSequenceRequest {
                contract_version: CONTRACT_VERSION,
                session_id: inspected.session_id.clone(),
                sequence_id: sequence.sequence_id.clone(),
                destination_folder: directory.path().to_str().unwrap().into(),
                filename_stem: "refused".into(),
                collision_policy: CollisionPolicy::FailIfExists,
                operation_id: None,
            })
            .unwrap_err();
        assert_eq!(error.diagnostic_code, "sequence_not_export_capable");
    }
    assert_eq!(std::fs::read_dir(directory.path()).unwrap().count(), 1);
}

#[test]
fn invalid_selected_sequence_is_not_replaced_by_valid_sibling() {
    use phoenix::app_contract::*;
    let valid = synthetic(NOTE);
    let mut bytes = synthetic(&[]);
    let parsed = parse_project_166(&valid).unwrap();
    bytes.extend_from_slice(&valid[parsed.sequences[0].sequence_range.clone()]);
    let directory = TestDirectory::new();
    let path = directory.path().join("siblings");
    std::fs::write(&path, &bytes).unwrap();
    let mut service = phoenix::app_service::AppService::new();
    let response = service
        .inspect_project(InspectProjectRequest {
            contract_version: CONTRACT_VERSION,
            source_path: path.to_str().unwrap().into(),
            diagnostics_level: DiagnosticsLevel::Full,
        })
        .unwrap();
    assert_eq!(response.sequences.len(), 2);
    assert_ne!(response.sequences[0].readiness, Readiness::Ready);
    assert_eq!(response.sequences[1].readiness, Readiness::Ready);
    let error = service
        .export_sequence(ExportSequenceRequest {
            contract_version: CONTRACT_VERSION,
            session_id: response.session_id,
            sequence_id: response.sequences[0].sequence_id.clone(),
            destination_folder: directory.path().to_str().unwrap().into(),
            filename_stem: "refused".into(),
            collision_policy: CollisionPolicy::FailIfExists,
            operation_id: None,
        })
        .unwrap_err();
    assert_eq!(error.diagnostic_code, "sequence_not_export_capable");
    assert_eq!(std::fs::read_dir(directory.path()).unwrap().count(), 1);
    application_export(&bytes, 1);
}
