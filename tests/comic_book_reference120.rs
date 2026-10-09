use phoenix::recovery120::recover_project120;
use sha2::{Digest, Sha256};
const SOURCE: &str = "/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/MY OLD MUSIC/Archive folder/COMIC BOOK";
const REFERENCE: &str = "/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/pick up the pace";
#[derive(Clone, Debug, PartialEq, Eq)]
struct Event {
    tick: u32,
    status: u8,
    tag: u8,
    data: Vec<u8>,
}
fn vlq(b: &[u8], p: &mut usize) -> u32 {
    let mut value = 0;
    for _ in 0..4 {
        let x = b[*p];
        *p += 1;
        value = (value << 7) | u32::from(x & 127);
        if x < 128 {
            return value;
        }
    }
    panic!("overlong VLQ");
}
fn parse(b: &[u8]) -> Vec<Vec<Event>> {
    assert_eq!(
        &b[..14],
        &[77, 84, 104, 100, 0, 0, 0, 6, 0, 1, 0, 2, 1, 224]
    );
    let mut p = 14;
    let mut tracks = Vec::new();
    for _ in 0..2 {
        assert_eq!(&b[p..p + 4], b"MTrk");
        let n = u32::from_be_bytes(b[p + 4..p + 8].try_into().unwrap()) as usize;
        p += 8;
        let end = p + n;
        let mut tick = 0;
        let mut running = 0;
        let mut events = Vec::new();
        while p < end {
            tick += vlq(b, &mut p);
            let status = if b[p] >= 128 {
                let x = b[p];
                p += 1;
                x
            } else {
                assert!(running >= 128);
                running
            };
            let (tag, data) = if status == 255 {
                running = 0;
                let tag = b[p];
                p += 1;
                let n = vlq(b, &mut p) as usize;
                let data = b[p..p + n].to_vec();
                p += n;
                (tag, data)
            } else {
                assert!((128..240).contains(&status));
                running = status;
                let n = if matches!(status & 240, 192 | 208) {
                    1
                } else {
                    2
                };
                let data = b[p..p + n].to_vec();
                assert!(data.iter().all(|x| *x < 128));
                p += n;
                (0, data)
            };
            events.push(Event {
                tick,
                status,
                tag,
                data,
            });
        }
        assert_eq!(p, end);
        assert_eq!(events.last().unwrap().tag, 47);
        tracks.push(events);
    }
    assert_eq!(p, b.len());
    tracks
}
#[test]
fn exact_archive_reference_matches_complete_musical_timeline() {
    let source = std::fs::read(SOURCE).unwrap();
    let reference = std::fs::read(REFERENCE).unwrap();
    assert_eq!(source.len(), 406294);
    assert_eq!(
        format!("{:x}", Sha256::digest(&source)),
        "40217bf534e7ac7f01ccab65d5d6f3fc873f08c134095410da2cbf5cef88ac70"
    );
    assert_eq!(reference.len(), 592);
    assert_eq!(
        format!("{:x}", Sha256::digest(&reference)),
        "c268a5b58556b17a0376e585fa11f5d671d99a4c655d6c8f432170b552944557"
    );
    let candidates = recover_project120(&source).unwrap();
    let recovered = candidates[0].result.as_ref().unwrap();
    assert_eq!(recovered.name(), b"pick up the pace");
    assert!(recovered.mechanically_verified());
    assert_eq!(recovered.tracks().len(), 1);
    assert_eq!(recovered.counts().notes, 64);
    assert_eq!(
        recovered.terminal().unwrap().source_bytes,
        [0, 0, 0, 1, 255, 255]
    );
    let native = parse(&reference);
    let generated = parse(recovered.midi());
    let messages = |events: &[Event]| {
        events
            .iter()
            .filter(|e| e.status != 255)
            .cloned()
            .collect::<Vec<_>>()
    };
    // Compare every attack and explicit release, including channel, velocity and source order.
    assert_eq!(messages(&generated[1]), messages(&native[1]));
    assert_eq!(messages(&generated[1]).len(), 128);
    assert!(messages(&generated[1]).iter().all(|e| e.status & 15 == 11));
    for tag in [3, 81, 88] {
        assert_eq!(
            generated[0].iter().find(|e| e.tag == tag),
            native[0].iter().find(|e| e.tag == tag)
        );
    }
    assert_eq!(
        native[0].iter().find(|e| e.tag == 84).unwrap().data,
        [96, 0, 0, 0, 0]
    );
    assert_eq!(
        native[1].iter().find(|e| e.tag == 4).unwrap().data,
        b"JV-880 Vintage"
    );
    assert_eq!(native[0].last().unwrap().tick, 19200);
    assert_eq!(native[1].last().unwrap().tick, 19200);
    assert_eq!(generated[0].last().unwrap().tick, 0);
    assert_eq!(generated[1].last().unwrap().tick, 18333);
    // Only established optional native metadata and trailing EOT padding differ.
    for i in 0..2 {
        let essential = |events: &[Event]| {
            events
                .iter()
                .filter(|e| !(e.status == 255 && [4, 47, 84].contains(&e.tag)))
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(essential(&generated[i]), essential(&native[i]));
    }
}
#[test]
fn unqualified_headers_and_changed_source_remain_refused() {
    let source = std::fs::read(SOURCE).unwrap();
    assert!(phoenix::bounded_conductor::decode_compact_conductor(
        &source,
        [9545..9588, 9588..9622, 9622..9665, 9665..9698]
    )
    .is_err());
    assert!(phoenix::bounded_terminal::assess_terminal_coverage(
        &source,
        9095..10134,
        &[
            9545..9588,
            9588..9622,
            9622..9665,
            9665..9698,
            9698..10079,
            10079..10123
        ]
    )
    .is_err());
    for offset in [9551, 9628, 1519, 10131, 9717] {
        let mut changed = source.clone();
        changed[offset] ^= 1;
        assert!(
            recover_project120(&changed)
                .map(|c| c[0].result.is_err())
                .unwrap_or(true),
            "offset {offset}"
        );
    }
}
#[test]
fn existing_service_authorizes_only_after_complete_recovery() {
    use phoenix::app_contract::*;
    let response = phoenix::app_service::AppService::new()
        .inspect_project(InspectProjectRequest {
            contract_version: CONTRACT_VERSION,
            source_path: SOURCE.into(),
            diagnostics_level: DiagnosticsLevel::Summary,
        })
        .unwrap();
    assert_eq!(response.sequences[0].readiness, Readiness::Ready);
    assert!(response.sequences[0].bounded_export_capability.is_some());
    assert!(response.sequences[0].export_capability.is_none());
}
