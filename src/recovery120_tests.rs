use super::*;
use crate::observed_layout120::tests::synthetic_marker;
fn record(b: &mut Vec<u8>, ty: u8, p: &[u8]) {
    b.push(ty);
    b.extend((p.len() as u32).to_be_bytes());
    b.extend(p);
}
fn fixture(events: &[u8], raw: u32) -> Vec<u8> {
    let mut b = synthetic_marker(3, b"Portable", [0xfe, 0xff]);
    b.truncate(8 + 75 + 120 * 3);
    for (i, l) in [b"Meter Track".as_slice(), b"Tempo Track", b"Part"]
        .iter()
        .enumerate()
    {
        let at = 8 + 121 + 120 * i;
        b[at..at + l.len()].copy_from_slice(l);
        b[at - 39..at - 31].copy_from_slice(&[0x80, 0, 4, 0, 0, 4, 1, 0]);
    }
    let label = 8 + 121 + 240;
    b[label - 33..label].fill(0xff);
    b[label - 33..label - 30].copy_from_slice(&[1, 0, 1]);
    record(&mut b, 7, &[0; 10]);
    let meter = vec![0, 255, 88, 4, 3, 2, 8, 8];
    let tempo = vec![0, 255, 81, 3, 7, 161, 32];
    for (count, events, last) in [
        (
            2u32,
            [vec![0, 255, 89, 2, 253, 1], meter.clone()].concat(),
            meter,
        ),
        (1, tempo.clone(), tempo),
    ] {
        let mut p = vec![0, 1];
        p.extend(count.to_be_bytes());
        p.extend([0; 4]);
        p.extend([9, 8, 7, 6]);
        p.extend(&events);
        p.extend([135, 255, 255, 127, 255, 47, 0]);
        let mut value = vec![last[2]];
        value.extend(&last[4..]);
        let mut s = vec![0, 1, 0, 0, 0, (18 + value.len()) as u8];
        s.extend(count.to_be_bytes());
        s.extend(((events.len() + 4) as u32).to_be_bytes());
        s.extend([0, 255, 255, 255, 47, 255, 0, 0, 0, 0]);
        s.extend(value);
        record(&mut b, 2, &p);
        record(&mut b, 41, &s);
    }
    let mut p = vec![0; 14];
    p[2..6].copy_from_slice(&raw.to_be_bytes());
    p.extend(events);
    p.extend([255, 0, 0, 0, 255, 47, 0]);
    record(&mut b, 2, &p);
    record(&mut b, 41, &[0; 23]);
    record(&mut b, 0, &[255, 254, 255, 254, 255]);
    for i in 0..2 {
        let mut p = vec![0; 36];
        p[25] = i;
        p[26] = 7;
        p[27] = 4;
        record(&mut b, 16, &p);
    }
    let mut p = vec![0; 40];
    p[0] = 3;
    p[1..4].copy_from_slice(b"Box");
    p[33] = 7;
    record(&mut b, 42, &p);
    b
}
fn note() -> Vec<u8> {
    vec![0, 0x90, 60, 100, 64, 0x81, 0x70]
}
fn result(b: &[u8]) -> Result<Recovery120<'_>, Recovery120Refusal> {
    recover_project120(b).unwrap().remove(0).result
}
#[test]
fn complete_portable_recovery_retains_proof_key_terminal_and_identity() {
    let b = fixture(&note(), 1);
    let r = result(&b).unwrap();
    assert_eq!(r.proof().source(), b);
    assert_eq!(r.candidate_ordinal(), 0);
    assert_eq!(r.name(), b"Portable");
    assert_eq!(r.counts().notes, 1);
    assert_eq!(r.counts().generated_note_offs, 1);
    assert_eq!(r.tracks()[0].channel_assignment.unwrap().channel.get(), 5);
    assert_eq!(r.conductor().key().unwrap().accidentals, -3);
    assert_eq!(
        r.terminal().unwrap().source_bytes,
        vec![255, 254, 255, 254, 255]
    );
    assert!(r.mechanically_verified());
    assert_eq!(r.midi(), result(&b).unwrap().midi());
    assert_eq!(r.source_sha256(), format!("{:x}", Sha256::digest(&b)));
}
#[test]
fn accounting_and_unknown_music_refuse_before_midi() {
    for raw in [0, 2] {
        assert_eq!(
            result(&fixture(&note(), raw)).unwrap_err().stage,
            "authority"
        );
    }
    assert!(result(&fixture(&[0, 255, 64, 0], 1)).is_err());
}
#[test]
fn empty_paired_track_is_preserved_without_a_route() {
    let b = fixture(&[], 0);
    let r = result(&b).unwrap();
    assert_eq!(r.tracks().len(), 1);
    assert!(r.tracks()[0].channel_assignment.is_none());
    assert!(r.tracks()[0].handoff.is_empty());
    assert!(r.mechanically_verified());
}
#[test]
fn conductor_terminal_and_non_equivalent_source_order_stay_blocking() {
    let mut b = fixture(&note(), 1);
    let root = crate::sequence_container::parse_root_record_stream(&b).unwrap();
    let at = root
        .records
        .iter()
        .find(|r| r.record_type.value == 2)
        .unwrap()
        .payload
        .range
        .start;
    b[at + 1] = 0x7f;
    assert_eq!(result(&b).unwrap_err().stage, "conductor");
    let mut b = fixture(&note(), 1);
    let root = crate::sequence_container::parse_root_record_stream(&b).unwrap();
    let at = root
        .records
        .iter()
        .find(|r| r.record_type.value == 0)
        .unwrap()
        .payload
        .range
        .start;
    b[at] = 1;
    assert_eq!(result(&b).unwrap_err().stage, "terminal");
    let events = [vec![0, 0x90, 60, 100, 64, 10], vec![0, 0x90, 60, 90, 64, 8]].concat();
    assert_eq!(
        result(&fixture(&events, 2)).unwrap_err().stage,
        "source order/serialization"
    );
}
#[test]
fn narrow_existing_program_only_predicate_does_not_admit_banks_or_lookalikes() {
    assert_eq!(
        program_only(7, &[0, 0, 7, 0xf8, 7], &[2, b'G', b'7', 4, 255, 255, 255]),
        Ok(PatchTranslation::ProgramOnlyConfirmed)
    );
    for tail in [[4, 255, 80, 255], [4, 255, 81, 255], [4, 0, 255, 255]] {
        let mut post = vec![2, b'G', b'7'];
        post.extend(tail);
        assert!(program_only(7, &[0, 0, 7, 0xf8, 7], &post).is_err());
    }
    assert!(program_only(7, &[0, 0, 7, 0xf8, 8], &[2, b'G', b'7', 4, 255, 255, 255]).is_err());
    assert!(program_only(7, &[0, 0, 7, 0, 7], &[2, b'G', b'7', 4, 255, 255, 255]).is_err());
    assert!(program_only(7, &[0, 0, 7, 0xf8, 7], &[2, b'I', b'7', 4, 255, 255, 255]).is_err());
}
#[test]
fn independent_readback_detects_truncation_corruption_and_extra_events() {
    let b = fixture(&note(), 1);
    let r = result(&b).unwrap();
    let c = r.conductor().adapt(r.name()).unwrap();
    let expected = vec![
        crate::readback120::conductor_expected(&c, Some((-3, true))),
        crate::readback120::musical_expected(&r.tracks()[0].name, &r.tracks()[0].scheduled),
    ];
    for end in 0..r.midi().len() {
        assert!(crate::readback120::verify(&r.midi()[..end], 480, &expected).is_err());
    }
    for i in 0..r.midi().len() {
        let mut bytes = r.midi().to_vec();
        bytes[i] ^= 1;
        assert!(
            crate::readback120::verify(&bytes, 480, &expected).is_err(),
            "byte {i}"
        );
    }
    let mut bytes = r.midi().to_vec();
    bytes.push(0);
    assert!(crate::readback120::verify(&bytes, 480, &expected).is_err());
}
#[test]
fn authentic_external_classification_and_eight_byte_identical_artifacts_when_available() {
    let source="/Users/kurtheiden/Documents/Phoenix Research/Authentic Studio Vision Projects/SCHOOL PROJECTS";
    let Ok(b) = std::fs::read(source) else {
        eprintln!("external authentic source unavailable");
        return;
    };
    assert_eq!(b.len(), 343875);
    assert_eq!(
        format!("{:x}", Sha256::digest(&b)),
        "bfd4fa1208e2cd884ec51ccfb1131d5c02723d7c90acafc98bd89597e1a20331"
    );
    let filenames = [
        (0, "DANCING"),
        (1, "NOTHING"),
        (4, "Nothing FINAL"),
        (7, "Sequence H"),
        (8, "Unnamed Sequence 09"),
        (9, "Wind ensemble"),
        (11, "Tuba thing"),
        (12, "Sequence M"),
    ];
    let rows = recover_project120(&b).unwrap();
    assert_eq!(rows.len(), 15);
    let mut good = 0;
    for row in rows {
        eprintln!(
            "candidate {} recovery {:?}",
            row.ordinal + 1,
            row.result
                .as_ref()
                .map(|r| (r.midi().len(), r.counts().notes))
        );
        if let Some((_, name)) = filenames.iter().find(|(i, _)| *i == row.ordinal) {
            let r = row.result.unwrap();
            assert!(r.mechanically_verified());
            let path=format!("/Users/kurtheiden/Documents/Phoenix Research/Experimental MIDI Recovery/SCHOOL PROJECTS/SCHOOL - {name}.mid");
            let existing = std::fs::read(path).expect("validated persistent artifact");
            assert_eq!(r.midi(), existing, "artifact {name}");
            good += 1;
        } else {
            assert!(row.result.is_err());
        }
    }
    assert_eq!(good, 8);
}
