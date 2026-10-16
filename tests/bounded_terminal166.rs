use phoenix::bounded_sequence::{
    assemble_bounded_sequence, build_bounded_sequence_manifest as manifest,
};
use phoenix::bounded_terminal166::classify_terminal_coverage as classify;
use phoenix::sequence_container::parse_project_166;
use sha2::{Digest, Sha256};
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
        p[1] = 1;
        p[2..6].copy_from_slice(&2u32.to_be_bytes());
        p.extend(NOTE);
        p.extend([255, 0, 0, 0, 255, 47, 0]);
        record(&mut b, 2, &p);
        record(&mut b, 41, &[]);
    }
    record(&mut b, 0, &[255; 8]);
    b
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
fn terminal(b: &mut Vec<u8>, ordinal: usize, words: &[u16]) {
    let range = parse_project_166(b).unwrap().sequences[ordinal]
        .terminal_record
        .record_range
        .clone();
    let mut r = Vec::new();
    let payload: Vec<_> = words.iter().flat_map(|w| w.to_be_bytes()).collect();
    record(&mut r, 0, &payload);
    b.splice(range, r);
}
fn headers(b: &mut [u8], ordinal: usize, values: &[u16]) {
    let p = parse_project_166(b).unwrap();
    let s = &p.sequences[ordinal];
    let ranges: Vec<_> = std::iter::once(&s.meter_primary)
        .chain(std::iter::once(&s.tempo_primary))
        .chain(s.track_pairs.iter().map(|p| &p.primary))
        .map(|p| p.payload.range.start)
        .collect();
    assert_eq!(ranges.len(), values.len());
    for (at, word) in ranges.into_iter().zip(values) {
        b[at..at + 2].copy_from_slice(&word.to_be_bytes());
    }
}
#[test]
fn exact_provenance_and_unchanged_midi_legacy_path() {
    let old = fixture(1);
    let before = assemble_bounded_sequence(&manifest(&old, 0, 480).unwrap()).unwrap();
    assert!(manifest(&old, 0, 480)
        .unwrap()
        .terminal_coverage()
        .is_none());
    let mut b = old;
    terminal(&mut b, 0, &[0xffff; 3]);
    let q = classify(&b, 0).unwrap();
    assert_eq!(q.slots().len(), 3);
    assert_eq!(&b[q.record_range()], q.record_bytes());
    assert_eq!(&b[q.payload_range()], q.payload());
    for (i, s) in q.slots().iter().enumerate() {
        assert_eq!(s.descriptor_ordinal, i);
        assert_eq!(s.word, 0xffff);
        assert_eq!(s.primary_header, 1);
        assert_eq!(&b[s.primary_header_range.clone()], &[0, 1]);
        assert_eq!(&b[s.word_range.clone()], &[255, 255]);
        assert!(s.project_rank.is_none());
    }
    let m = manifest(&b, 0, 480).unwrap();
    assert!(m.terminal_coverage().is_some());
    assert_eq!(before, assemble_bounded_sequence(&m).unwrap());
}
#[test]
fn project_wide_ranks_skip_header_one_and_ignore_other_terminals() {
    let mut b = fixture(1);
    let range = parse_project_166(&b).unwrap().sequences[0]
        .sequence_range
        .clone();
    let second = b[range].to_vec();
    b.extend(second);
    headers(&mut b, 0, &[2, 1, 2]);
    headers(&mut b, 1, &[2, 2, 1]);
    terminal(&mut b, 0, &[0, 0xffff, 1]);
    terminal(&mut b, 1, &[2, 3, 0xffff]);
    assert_eq!(
        classify(&b, 1)
            .unwrap()
            .slots()
            .iter()
            .map(|s| s.word)
            .collect::<Vec<_>>(),
        [2, 3, 0xffff]
    );
    // Other terminal contents never confer rank authority.
    terminal(&mut b, 0, &[99, 0xfffe, 42]);
    assert!(classify(&b, 1).is_ok());
    assert!(classify(&b, 0).is_err());
    headers(&mut b, 0, &[3, 1, 2]);
    assert!(classify(&b, 1).is_err());
}
#[test]
fn malformed_slots_headers_order_and_lengths_refuse() {
    for words in [
        vec![0, 1, 2],
        vec![0xffff, 0xffff],
        vec![0xffff; 4],
        vec![0xffff, 0xfffe, 0xffff],
        vec![0xffff, 0, 0xffff],
    ] {
        let mut b = fixture(1);
        terminal(&mut b, 0, &words);
        assert!(classify(&b, 0).is_err());
    }
    for words in [
        [0, 0, 2],
        [0, 2, 1],
        [0, 1, 3],
        [0, 256, 2],
        [0, 1, 0xfffe],
        [0, 1, 0xffff],
    ] {
        let mut b = fixture(1);
        headers(&mut b, 0, &[2; 3]);
        terminal(&mut b, 0, &words);
        assert!(classify(&b, 0).is_err());
    }
    for h in [0, 3, 0xfffe, 0xffff] {
        let mut b = fixture(1);
        headers(&mut b, 0, &[1, 1, h]);
        terminal(&mut b, 0, &[0xffff; 3]);
        assert!(classify(&b, 0).is_err());
    }
    let mut b = fixture(1);
    terminal(&mut b, 0, &[0xffff; 3]);
    let at = parse_project_166(&b).unwrap().sequences[0]
        .terminal_record
        .record_range
        .start;
    b[at + 1..at + 5].copy_from_slice(&7u32.to_be_bytes());
    b.push(255);
    assert!(classify(&b, 0).is_err());
    let mut b = fixture(1);
    terminal(&mut b, 0, &[0xffff; 3]);
    b.extend([1, 2, 3]);
    assert!(classify(&b, 0).is_err());
    let mut b = fixture(1);
    let p = parse_project_166(&b).unwrap();
    let at = p.sequences[0].descriptor_count.offset;
    b[at] += 1;
    assert!(classify(&b, 0).is_err());
}
#[test]
fn orphan_primaries_and_extra_owned_pairs_refuse() {
    let mut b = fixture(1);
    terminal(&mut b, 0, &[0xffff; 3]);
    let mut orphan = Vec::new();
    record(&mut orphan, 2, &[0, 2]);
    b.splice(8..8, orphan);
    assert!(classify(&b, 0).unwrap_err().contains("unowned"));
    let mut b = fixture(1);
    terminal(&mut b, 0, &[0xffff; 3]);
    let p = parse_project_166(&b).unwrap();
    let pair = &p.sequences[0].track_pairs[0];
    let extra = b[pair.primary.record_range.start..pair.secondary.record_range.end].to_vec();
    let at = p.sequences[0].terminal_record.record_range.start;
    b.splice(at..at, extra);
    assert!(classify(&b, 0).unwrap_err().contains("ownership"));
}
fn authenticated(path: &str, hash: &str) -> Vec<u8> {
    let b = std::fs::read(path).expect("authenticated terminal fixture required");
    assert_eq!(format!("{:x}", Sha256::digest(&b)), hash);
    b
}
#[test]
fn authenticated_thirteen_controlled_saves_and_contrasting_sequences() {
    let root = "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/";
    let shared = "/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/";
    let cases=[
("Experiment 007 - Untouched Baseline/newest STUFF baseline","e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132"),
("Experiment 004 - Change one tempo value/newest STUFF tempo to 130","ba1a99a058f7c6ad50f5a559b518e0f72bfa99660bc199e5dbfd8f31e46ee633"),
("Experiment 008 - Native Duplicate Edit Save/newest STUFF Track A native","ccc53da15cdd9044ee475e1a1ffd92be6301f70a0a64f3718df361aa6c412ffb"),
("Experiment 009 - Single MIDI Note Pitch Change/newest STUFF baseline copy","2b08f822c65bf21b1eeda8f509e6b9c162414f210bb2fbf037650a3541c8ce87"),
("Experiment 010 - Single MIDI Note Down One Semitone/newest STUFF baseline EXP10","146d4a9da37aed1c1d6085ead4af9c57ad6d51e4e021001ae442f5d1fc9d2a42"),
("Experiment 030 - Change initial Meter from 4-4 to 7-8/newest STUFF meter 7-8 EXP30","7679d6628069e4538efbd7c227b4fa00d660e7bb77da5436332121ade063f5ee"),
("Experiment 031 - Move Track 3 #2 first Note from 6-1-3 to 6-1-4/newest STUFF baseline EXP31","e1650db0f0b0ebfaefa318707824d3d6ead766c791dd502b48209380266954b7"),
("Experiment 033 - Track 2 Instrument JV-1080-2 to JV-1080-3/Returned from MacOS9/EXP33 CTRL","b1c82a7085f98f054f7320e2800aba523447c3c987c1aa983865518b7a7dd368"),
("Experiment 033 - Track 2 Instrument JV-1080-2 to JV-1080-3/Returned from MacOS9/EXP33 EDIT","91eba17f0e23c39ee480e7b2c2ae585d2b23198bf69cf10b5f5fc9b91bd701d7"),
("Experiment 034 - Track 2 Instrument JV-1080-2 to Juno-106/Returned from MacOS9/EXP34 CTRL","0f811a95b782fc5af3339a18d107f373d337913bdceb7cdf2109878b68603288"),
("Experiment 034 - Track 2 Instrument JV-1080-2 to Juno-106/Returned from MacOS9/EXP34 EDIT","6c87431d619b3934d8deb365c6e7cc8273e9a980f1422307fdfab5ae944091c0"),
];
    let mut all: Vec<_> = cases
        .iter()
        .map(|(p, h)| (format!("{root}{p}"), *h))
        .collect();
    all.extend([
        (format!("{shared}CCA073 CONTROL"), cases[0].1),
        (
            format!("{shared}CCA073 EDIT"),
            "3f12c7bb9371d76dd4d4da081ccf347101cd5d4f187fd039a9562665d87d0c87",
        ),
    ]);
    for (path, hash) in all {
        let b = authenticated(&path, hash);
        let p = parse_project_166(&b).unwrap();
        assert_eq!(
            p.sequences
                .iter()
                .map(|s| s.track_pairs.len() + 2)
                .sum::<usize>(),
            168
        );
        for (ordinal, s) in p.sequences.iter().enumerate() {
            let result = classify(&b, ordinal);
            if s.sequence_name.as_utf8() == Some("Sequence I") {
                assert!(result.is_err());
            } else if matches!(
                s.track_associations,
                phoenix::sequence_container::TrackAssociations::Ordinal(_)
            ) {
                let q = result.unwrap_or_else(|e| panic!("{path} sequence {ordinal}: {e}"));
                assert_eq!(q.slots().len(), s.descriptors.len());
            } else {
                assert!(result.is_err());
            }
        }
        assert!(classify(&b, 0).is_ok());
    }
}
#[test]
fn authenticated_xform_next_gates_and_legacy_r() {
    let root = "/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/";
    for (name, hash, expected) in [
        (
            "CCA073 CONTROL",
            "e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132",
            "Patch",
        ),
        (
            "CCA073 EDIT",
            "3f12c7bb9371d76dd4d4da081ccf347101cd5d4f187fd039a9562665d87d0c87",
            "conductor coverage",
        ),
    ] {
        let b = authenticated(&format!("{root}{name}"), hash);
        assert_eq!(classify(&b, 0).unwrap().slots().len(), 15);
        let e = manifest(&b, 0, 480).unwrap_err();
        eprintln!("{name}: {e}");
        assert_eq!(e.stage, expected);
        if name.ends_with("CONTROL") {
            let p = parse_project_166(&b).unwrap();
            for wanted in ["Sequence Q", "Sequence R", "Bells for her", "Ode to Clarke"] {
                let ordinal = p
                    .sequences
                    .iter()
                    .position(|s| s.sequence_name.as_utf8() == Some(wanted))
                    .unwrap();
                assert!(classify(&b, ordinal).is_ok());
                if wanted == "Sequence R" {
                    assert!(manifest(&b, ordinal, 480)
                        .unwrap()
                        .terminal_coverage()
                        .is_none());
                }
            }
        }
    }
}
