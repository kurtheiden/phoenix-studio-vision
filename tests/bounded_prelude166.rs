use phoenix::bounded_prelude166::{classify_prelude_coverage as classify, PreludeRefusal};
use phoenix::bounded_sequence::{
    assemble_bounded_sequence, build_bounded_sequence_manifest as manifest,
};
use phoenix::sequence_container::parse_project_166;
use sha2::{Digest, Sha256};
const BASE: &str = "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/";
const NOTE: &[u8] = &[0, 144, 60, 100, 64, 12, 0, 144, 50, 100, 64, 12];
fn record(b: &mut Vec<u8>, t: u8, p: &[u8]) {
    b.push(t);
    b.extend((p.len() as u32).to_be_bytes());
    b.extend(p);
}
fn fixture(tracks: usize) -> Vec<u8> {
    let mut b = vec![0; 8];
    for i in 0..2 {
        let mut p = vec![0; 36];
        p[25] = i;
        p[26] = 7;
        p[27] = 3;
        record(&mut b, 16, &p);
    }
    let mut device = vec![0; 40];
    device[0] = 4;
    device[1..5].copy_from_slice(b"Test");
    device[33] = 7;
    record(&mut b, 42, &device);
    let start = b.len();
    let mut p = vec![0; (tracks + 2) * 166 + 172];
    p[0] = (tracks + 2) as u8;
    record(&mut b, 1, &p);
    for i in 0..tracks {
        let l = start + 208 + (i + 2) * 166 + 15;
        b[l..l + 5].copy_from_slice(b"Other");
        b[l - 39..l - 31].copy_from_slice(&[128, 0, 4, 0, 0, 4, 1, 0]);
        b[l - 33] = 1;
        b[l - 31] = 1;
        b[l - 30..l].fill(255);
    }
    let mut name = vec![0; 11];
    name.extend([4, b'T', b'e', b's', b't']);
    record(&mut b, 7, &name);
    conductor_pair(&mut b, &[0, 255, 88, 4, 4, 2, 8, 8]);
    conductor_pair(&mut b, &[0, 255, 81, 3, 7, 53, 120]);
    for _ in 0..tracks {
        let mut p = vec![0; 14];
        p[2..6].copy_from_slice(&2u32.to_be_bytes());
        p.extend(NOTE);
        p.extend([255, 0, 0, 0, 255, 47, 0]);
        record(&mut b, 2, &p);
        record(&mut b, 41, &[]);
    }
    record(&mut b, 0, &[255; 8]);
    b
}
fn payload() -> Vec<u8> {
    vec![0, 1, 0, 2, 0, 1, 60, 15, 0, 1, 50, 15]
}
fn insert(b: &mut Vec<u8>, payload: &[u8]) {
    let at = parse_project_166(b).unwrap().sequences[0]
        .meter_primary
        .record_range
        .start;
    let mut r = vec![];
    record(&mut r, 9, payload);
    b.splice(at..at, r);
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
fn exact_coverage_retains_bytes_and_adds_no_midi_events() {
    for word in [1, 2] {
        let bare = fixture(1);
        assert!(classify(&bare, 0).unwrap().is_empty());
        let before = assemble_bounded_sequence(&manifest(&bare, 0, 480).unwrap()).unwrap();
        let mut b = bare;
        let mut p = payload();
        p[1] = word;
        insert(&mut b, &p);
        let proof = classify(&b, 0).unwrap();
        assert_eq!(proof.len(), 1);
        let q = &proof[0];
        assert_eq!(q.payload(), p);
        assert_eq!(&b[q.payload_range()], p);
        assert_eq!(q.record_range().end - q.record_range().start, p.len() + 5);
        assert_eq!(q.routing().pair_ordinal, 0);
        assert_eq!(q.routing().instrument_candidate, 1);
        assert_eq!(q.pitches().iter().copied().collect::<Vec<_>>(), [50, 60]);
        let m = manifest(&b, 0, 480).unwrap();
        assert_eq!(m.conductor().preludes(), proof);
        assert_eq!(assemble_bounded_sequence(&m).unwrap(), before);
    }
}
#[test]
fn header_count_key_flag_and_pitch_mutations_refuse() {
    for (at, v) in [
        (0, 1),
        (1, 3),
        (2, 1),
        (3, 0),
        (3, 1),
        (3, 3),
        (4, 1),
        (5, 0),
        (9, 2),
        (6, 128),
        (7, 14),
        (10, 60),
        (10, 61),
        (10, 49),
    ] {
        let mut p = payload();
        p[at] = v;
        let mut b = fixture(1);
        insert(&mut b, &p);
        assert!(classify(&b, 0).is_err(), "{at}/{v}");
        assert_eq!(manifest(&b, 0, 480).unwrap_err().stage, "conductor prelude");
    }
    let mut p = payload();
    p.push(0);
    let mut b = fixture(1);
    insert(&mut b, &p);
    assert_eq!(classify(&b, 0).unwrap_err(), PreludeRefusal::Count);
    let mut b = fixture(1);
    insert(&mut b, &[0, 1, 0, 0]);
    assert_eq!(classify(&b, 0).unwrap_err(), PreludeRefusal::Count);
}
#[test]
fn missing_surplus_and_ambiguous_tracks_refuse() {
    for p in [
        vec![0, 1, 0, 1, 0, 1, 60, 15],
        vec![0, 1, 0, 3, 0, 1, 60, 15, 0, 1, 55, 15, 0, 1, 50, 15],
    ] {
        let mut b = fixture(1);
        insert(&mut b, &p);
        assert_eq!(
            classify(&b, 0).unwrap_err(),
            PreludeRefusal::UnmatchedOrAmbiguous
        );
    }
    let mut b = fixture(2);
    insert(&mut b, &payload());
    assert_eq!(
        classify(&b, 0).unwrap_err(),
        PreludeRefusal::UnmatchedOrAmbiguous
    );
    let mut b = fixture(1);
    insert(&mut b, &payload());
    insert(&mut b, &payload());
    assert_eq!(classify(&b, 0).unwrap_err(), PreludeRefusal::TrackOrder);
}
#[test]
fn ownership_counts_routing_and_conductor_guards_remain() {
    let mut b = fixture(1);
    insert(&mut b, &payload());
    let p = parse_project_166(&b).unwrap();
    let s = &p.sequences[0];
    let mut bad = b.clone();
    let a = s.track_pairs[0].primary.payload.range.start + 2;
    bad[a..a + 4].copy_from_slice(&3u32.to_be_bytes());
    assert_eq!(
        classify(&bad, 0).unwrap_err(),
        PreludeRefusal::TrackCoverage
    );
    let mut bad = b.clone();
    bad[s.descriptors[2].label_start - 31] = 0;
    assert_eq!(classify(&bad, 0).unwrap_err(), PreludeRefusal::Routing);
    let mut bad = b.clone();
    bad[s.terminal_record.payload.range.start] = 0;
    assert_eq!(
        manifest(&bad, 0, 480).unwrap_err().stage,
        "sequence terminal"
    );
    let mut bad = b.clone();
    bad[s.tempo_secondary.payload.range.end - 4] ^= 1;
    assert!(classify(&bad, 0).is_ok());
    assert_eq!(
        manifest(&bad, 0, 480).unwrap_err().stage,
        "conductor coverage"
    );
    let mut bad = b.clone();
    bad[s.descriptors[2].label_start - 39] = 136;
    assert!(classify(&bad, 0).is_ok());
    assert_eq!(manifest(&bad, 0, 480).unwrap_err().stage, "mute");
    let mut bad = b.clone();
    let terminal = s.terminal_record.record_range.start;
    let mut extra = vec![];
    record(&mut extra, 2, s.track_pairs[0].primary.payload.bytes);
    record(&mut extra, 41, &[]);
    bad.splice(terminal..terminal, extra);
    assert_eq!(classify(&bad, 0).unwrap_err(), PreludeRefusal::Ownership);
}
fn read(relative: &str, hash: &str) -> Vec<u8> {
    let b = std::fs::read(format!("{BASE}{relative}"))
        .expect("authenticated controlled specimen required");
    assert_eq!(format!("{:x}", Sha256::digest(&b)), hash);
    b
}
#[test]
fn authenticated_seven_records_and_controlled_pitch_interventions() {
    let b = read(
        "Experiment 007 - Untouched Baseline/newest STUFF baseline",
        "e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132",
    );
    for (seq, pairs) in [
        (0, vec![2, 6]),
        (1, vec![2, 7, 11]),
        (5, vec![1]),
        (14, vec![8]),
    ] {
        let qs = classify(&b, seq).unwrap();
        assert_eq!(
            qs.iter()
                .map(|q| q.routing().pair_ordinal)
                .collect::<Vec<_>>(),
            pairs
        );
    }
    for (path, hash, pitches) in [
        (
            "Experiment 008 - Native Duplicate Edit Save/newest STUFF Track A native",
            "ccc53da15cdd9044ee475e1a1ffd92be6301f70a0a64f3718df361aa6c412ffb",
            vec![36, 38],
        ),
        (
            "Experiment 009 - Single MIDI Note Pitch Change/newest STUFF baseline copy",
            "2b08f822c65bf21b1eeda8f509e6b9c162414f210bb2fbf037650a3541c8ce87",
            vec![37, 38],
        ),
        (
            "Experiment 010 - Single MIDI Note Down One Semitone/newest STUFF baseline EXP10",
            "146d4a9da37aed1c1d6085ead4af9c57ad6d51e4e021001ae442f5d1fc9d2a42",
            vec![35, 38],
        ),
    ] {
        let b = read(path, hash);
        let qs = classify(&b, 14).unwrap();
        assert_eq!(qs[0].pitches().iter().copied().collect::<Vec<_>>(), pitches);
    }
    let b = read(
        "Experiment 021 - Next Event Pitch Down One/newest STUFF baseline EXP21",
        "5aa2c305d502afafcd56071740170e64d496f89948313e1f9c03b93e015a7da2",
    );
    assert!(classify(&b, 14).is_err());
}
#[test]
fn experiment073_preludes_and_terminals_pass_but_authority_stays_blocked() {
    for (name, hash, header) in [
        (
            "CCA073 CONTROL",
            "e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132",
            1,
        ),
        (
            "CCA073 EDIT",
            "3f12c7bb9371d76dd4d4da081ccf347101cd5d4f187fd039a9562665d87d0c87",
            2,
        ),
    ] {
        let path = format!("/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/{name}");
        let b = std::fs::read(&path).expect("returned source required");
        assert_eq!(format!("{:x}", Sha256::digest(&b)), hash);
        let q = classify(&b, 0).unwrap();
        assert_eq!(q.len(), 2);
        assert_eq!(q[0].payload()[1], header);
        assert_eq!(
            q.iter().map(|p| p.pitches().len()).collect::<Vec<_>>(),
            [14, 10]
        );
        assert_eq!(
            manifest(&b, 0, 480).unwrap_err().stage,
            if header == 1 {
                "mute"
            } else {
                "conductor coverage"
            }
        );
        let mut app = phoenix::app_service::AppService::new();
        let response = app
            .inspect_project(phoenix::app_contract::InspectProjectRequest {
                contract_version: phoenix::app_contract::CONTRACT_VERSION,
                source_path: path.clone(),
                diagnostics_level: phoenix::app_contract::DiagnosticsLevel::Summary,
            })
            .unwrap();
        let row = response
            .sequences
            .iter()
            .find(|s| s.display_name == "xForm")
            .unwrap();
        assert_eq!(
            row.readiness,
            phoenix::app_contract::Readiness::PartiallySupported
        );
        assert!(row.export_capability.is_none());
        assert!(row.bounded_export_capability.is_none());
        assert_eq!(
            format!("{:x}", Sha256::digest(std::fs::read(path).unwrap())),
            hash
        );
    }
}

#[test]
fn reversed_unique_track_order_and_misplaced_records_refuse() {
    let mut bare = fixture(2);
    let p = parse_project_166(&bare).unwrap();
    let at = p.sequences[0]
        .validated_track_event_bounds(1)
        .unwrap()
        .event_range
        .start
        + 2;
    bare[at] = 61;
    let a = payload();
    let mut b = a.clone();
    b[6] = 61;
    let mut forward = bare.clone();
    insert(&mut forward, &a);
    insert(&mut forward, &b);
    assert_eq!(classify(&forward, 0).unwrap().len(), 2);
    let mut reversed = bare;
    insert(&mut reversed, &b);
    insert(&mut reversed, &a);
    assert_eq!(
        classify(&reversed, 0).unwrap_err(),
        PreludeRefusal::TrackOrder
    );
    let mut bad = fixture(1);
    let p = parse_project_166(&bad).unwrap();
    let at = p.sequences[0].tempo_secondary.record_range.end;
    let mut r = vec![];
    record(&mut r, 9, &payload());
    bad.splice(at..at, r);
    assert_eq!(classify(&bad, 0).unwrap_err(), PreludeRefusal::Structure);
}

#[path = "support/reference_smf.rs"]
mod reference_smf;
#[test]
fn seven_correlated_records_match_pinned_native_pitch_sets() {
    use std::collections::BTreeSet;
    let b = read(
        "Experiment 007 - Untouched Baseline/newest STUFF baseline",
        "e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132",
    );
    let mut count = 0;
    for (seq,path,hash) in [
      (0,"/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/MIDI Exports from SVP/xForm Multitrack","872272ce43c389474fdb408416ad2406fa9a63de5e1d8bb94e0f6bbced7a1241"),
      (1,"/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/newest STUFF - Bells for her - provenance multitrack","ffbdbb6be208a2d607c9b0c55a12b72226a18d43b9494c2b46b058d4568fc2c3"),
      (5,"/Users/kurtheiden/Documents/Phoenix Research/Studio Vision MIDI Exports/Project 001/Girl-U-Want - SVP ref","6a5c2b30a2b39c9acdf33f6cc7eebfd742f9022d6f190dacb8d0f8de593944e0"),
      (14,"/Users/kurtheiden/Documents/Phoenix Research/Studio Vision MIDI Exports/Project 001/Ode to Clarke Multi All","4f63b34ef92204d4bc5eeb78dbbe7b94d005c1f9ceb57ea0f9809533ad590f29"),
    ] {
      let m=std::fs::read(path).expect("pinned native reference required");assert_eq!(format!("{:x}",Sha256::digest(&m)),hash);
      let tracks=reference_smf::midi_tracks(&m);
      for q in classify(&b,seq).unwrap() {
        let matches:Vec<_>=tracks.iter().filter(|track|track.iter().any(|(_,s,d)|*s==3&&*d==q.routing().label_bytes)).collect();
        assert_eq!(matches.len(),1);
        let notes:Vec<_>=matches[0].iter().filter(|(_,s,d)|*s>>4==9&&d[1]!=0).collect();
        let pitches:BTreeSet<_>=notes.iter().map(|(_,_,d)|d[0]).collect();
        let channels:BTreeSet<_>=notes.iter().map(|(_,s,_)|1+(*s&15)).collect();
        assert_eq!(&pitches,q.pitches());assert_eq!(channels,BTreeSet::from([q.routing().midi_channel]));
        assert_eq!(q.record_bytes(),&b[q.record_range()]);count+=1;
      }
      assert_eq!(format!("{:x}",Sha256::digest(std::fs::read(path).unwrap())),hash);
    }
    assert_eq!(count, 7);
}
#[test]
fn surplus_counterexample_refuses_even_if_entries_are_sorted() {
    let mut b = read(
        "Experiment 021 - Next Event Pitch Down One/newest STUFF baseline EXP21",
        "5aa2c305d502afafcd56071740170e64d496f89948313e1f9c03b93e015a7da2",
    );
    let p = parse_project_166(&b).unwrap();
    let r = &p.sequences[14].prelude_records[0];
    let at = r.payload.range.start + 4;
    let end = r.payload.range.end;
    let mut entries: Vec<_> = b[at..end].chunks_exact(4).map(|e| e.to_vec()).collect();
    assert_eq!(entries.len(), 16);
    entries.sort_by_key(|e| std::cmp::Reverse(e[2]));
    b[at..end].copy_from_slice(&entries.concat());
    assert_eq!(
        classify(&b, 14).unwrap_err(),
        PreludeRefusal::UnmatchedOrAmbiguous
    );
}
