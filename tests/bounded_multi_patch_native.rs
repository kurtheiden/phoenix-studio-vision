//! Authenticated fixture ranges are test evidence, never production predicates.
use phoenix::{
    bounded_multi_patch::decode_bounded_multi_patch_track,
    smf::{
        serialize_channel_message, serialize_named_musical_track_with_ordering,
        MusicalTrackOrdering,
    },
};
use sha2::{Digest, Sha256};
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
    Track { name, messages }
}
fn smf(b: &[u8]) -> Vec<Track> {
    assert_eq!(
        &b[..14],
        &[b'M', b'T', b'h', b'd', 0, 0, 0, 6, 0, 1, 0, 8, 1, 224]
    );
    let mut pos = 14;
    let mut out = Vec::new();
    for _ in 0..8 {
        assert_eq!(&b[pos..pos + 4], b"MTrk");
        let len = u32::from_be_bytes(b[pos + 4..pos + 8].try_into().unwrap()) as usize;
        out.push(track(&b[pos + 8..pos + 8 + len]));
        pos += 8 + len;
    }
    assert_eq!(pos, b.len());
    out
}
fn bank_pc(m: &Message) -> bool {
    m.bytes[0] >> 4 == 12 || (m.bytes[0] >> 4 == 11 && matches!(m.bytes[1], 0 | 32))
}

#[test]
fn authenticated_standalone_multi_banks_match_untouched_native_patch_initializations() {
    let source = std::fs::read("/Users/kurtheiden/Documents/Phoenix Research/Authentic Studio Vision Projects/SCHOOL PROJECTS").expect("authentic source required");
    let native = std::fs::read("/Users/kurtheiden/Documents/Phoenix Research/Studio Vision MIDI Exports/Project 001/Sequence H").expect("owner-attested native reference required");
    assert_eq!(source.len(), 343875);
    assert_eq!(
        format!("{:x}", Sha256::digest(&source)),
        "bfd4fa1208e2cd884ec51ccfb1131d5c02723d7c90acafc98bd89597e1a20331"
    );
    assert_eq!(native.len(), 2641);
    assert_eq!(
        format!("{:x}", Sha256::digest(&native)),
        "78b27be7e35408957600349cce1ba0d0135561704e172f83e735bedd0c2e4b0d"
    );
    let native = smf(&native);
    assert_eq!(native[0].name.as_deref(), Some(b"Sequence H".as_slice()));
    let expected: Vec<_> = native[7].messages.iter().filter(|m| bank_pc(m)).collect();
    assert_eq!(native[7].name.as_deref(), Some(b"Track 2".as_slice()));
    assert_eq!(expected.len(), 8);
    assert!(!native
        .iter()
        .flat_map(|t| &t.messages)
        .any(|m| m.bytes[0] >> 4 == 11 && m.bytes[1] == 32));
    let decoded =
        decode_bounded_multi_patch_track(&source, 245193..245226, 248125..248243).unwrap();
    let adapted = decoded.adapt().unwrap();
    assert_eq!(adapted.counts.bank_select_msb, 4);
    assert_eq!(adapted.counts.bank_select_lsb, 0);
    assert_eq!(adapted.counts.program_changes, 4);
    let serialized = serialize_named_musical_track_with_ordering(
        b"Track 2",
        &adapted.scheduled_events,
        MusicalTrackOrdering::SourceOrder,
    )
    .unwrap();
    let bytes = serialized.as_bytes();
    assert_eq!(&bytes[..4], b"MTrk");
    let generated = track(&bytes[8..]);
    assert_eq!(generated.messages.iter().collect::<Vec<_>>(), expected);
    assert_eq!(
        adapted
            .scheduled_events
            .iter()
            .map(|e| Message {
                tick: e.absolute_tick,
                bytes: serialize_channel_message(&e.message)
            })
            .collect::<Vec<_>>(),
        generated.messages
    );
}
