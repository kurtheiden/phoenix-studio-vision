use crate as phoenix;
use crate::mixed_event::{
    walk_bounded_mixed_events, MixedEventBounds, MixedEventItem, MixedEventKind,
};
use crate::reference_smf;

#[test]
#[ignore = "requires independent authentic dominion with samples and Reservist Revolt MIDI"]
fn authentic_reservist_validates_all_three_bridges_and_complete_stream() {
    use phoenix::semantic_layout120::associate_semantic_project_120;
    use phoenix::sequence_container::TrackRecordPair;
    use sha2::{Digest, Sha256};
    let source = std::fs::read(std::env::var("PHOENIX_DOMINION_SAMPLES_SOURCE").unwrap()).unwrap();
    assert_eq!(
        format!("{:x}", Sha256::digest(&source)),
        "4ec77addb3b39dfb5791d853cbb6225df9da30a95431633137e02a6811a8b34b"
    );
    let project = associate_semantic_project_120(&source).unwrap();
    let sequence = project
        .candidates
        .iter()
        .filter_map(|c| c.association.as_ref().ok())
        .find(|s| s.source_name.bytes == b"Reservist Revolt")
        .unwrap();
    let t = sequence
        .ordinary
        .iter()
        .find(|t| t.slot.label.bytes == b"Track 6")
        .unwrap();
    assert_eq!(t.slot.ordinal, 7);
    assert_eq!(t.pair.primary.record_range, 26850..28186);
    let bounds = TrackRecordPair {
        pair_ordinal: t.local_pair_ordinal,
        primary: t.pair.primary.clone(),
        secondary: t.pair.secondary.clone(),
        candidate_event_start: t.pair.primary.payload.range.start + 14,
        event_containing_range: t.pair.primary.payload.range.start + 14
            ..t.pair.primary.payload.range.end,
    }
    .validated_event_bounds()
    .unwrap();
    assert_eq!(bounds.event_range, 26869..28179);
    let w = walk_bounded_mixed_events(
        &source,
        MixedEventBounds {
            event_range: bounds.event_range,
        },
        Default::default(),
    )
    .unwrap();
    let root = phoenix::sequence_container::parse_root_record_stream(&source).unwrap();
    let proof = phoenix::authority120::prove_track(&source, &root.records, sequence, t).unwrap();
    assert_eq!(proof.context_records(), 9);
    assert_eq!(proof.declared(), Some(311));
    assert_eq!(proof.walk().unwrap(), &w);
    let phoenix::authority120::TrackRoute120::Single(route) = proof.route() else {
        panic!()
    };
    assert_eq!(route.channel, 9);
    assert_eq!(route.contexts.len(), 10); // Patch plus nine FF60 payloads
    let mut notes = Vec::new();
    let mut cc = Vec::new();
    let mut bridges = Vec::new();
    for item in &w.items {
        let MixedEventItem::Event(e) = item else {
            continue;
        };
        match &e.event {
            MixedEventKind::Note(n) => notes.push((
                e.position,
                n.pitch.value,
                n.attack_velocity.value,
                n.duration.value,
            )),
            MixedEventKind::ContextMediatedNote(n) => notes.push((
                e.position,
                n.note.pitch.value,
                n.note.attack_velocity.value,
                n.note.duration.value,
            )),
            MixedEventKind::MidiController(c) => {
                cc.push((
                    e.position,
                    0xb8,
                    vec![c.controller_number.value, c.controller_value.value],
                ));
                if let Some(ctx) = &c.context {
                    bridges.push((
                        ctx.leading_timing.range.start,
                        ctx.leading_timing.value,
                        c.timing_delta.value,
                        e.position,
                    ));
                }
            }
            _ => panic!("unexpected family"),
        }
    }
    assert_eq!((notes.len(), cc.len()), (61, 240));
    assert_eq!(
        bridges,
        [
            (27320, 1192, 98, 93209),
            (27588, 13, 2, 97172),
            (27796, 443, 127, 101356)
        ]
    );
    for (key, hash) in [
        (
            "PHOENIX_RESERVIST_NATIVE_MIDI",
            "630ae0844e98180e513e76f16fc5532f0e2e43520362724fc13d502c8624521c",
        ),
        (
            "PHOENIX_RESERVIST_MULTITRACK_MIDI",
            "9c1b9b5383acbcfbc56f5b059de17702717813f5d5b6efb93ba8bae0ccabc0a6",
        ),
    ] {
        let reference = std::fs::read(std::env::var(key).unwrap()).unwrap();
        assert_eq!(format!("{:x}", Sha256::digest(&reference)), hash);
        let tracks = reference_smf::midi_tracks(&reference);
        let track = tracks
            .iter()
            .find(|es| es.iter().filter(|e| e.1 == 0x98 && e.2[1] > 0).count() == 61)
            .unwrap();
        let native_cc: Vec<_> = track
            .iter()
            .filter(|e| e.1 == 0xb8 && !(e.0 == 61440 && e.2 == [0, 80]))
            .cloned()
            .collect();
        assert_eq!(cc, native_cc); // exclude the separately represented Patch bank
        let ons: Vec<_> = track
            .iter()
            .filter(|e| e.1 == 0x98 && e.2[1] > 0)
            .map(|e| (e.0, e.2[0], e.2[1]))
            .collect();
        assert_eq!(
            notes.iter().map(|n| (n.0, n.1, n.2)).collect::<Vec<_>>(),
            ons
        );
        for (tick, pitch, _, duration) in &notes {
            let ending = track
                .iter()
                .find(|e| {
                    e.0 > *tick
                        && e.2.first() == Some(pitch)
                        && (e.1 == 0x88 || (e.1 == 0x98 && e.2[1] == 0))
                })
                .unwrap();
            assert_eq!(ending.0 - tick, *duration);
        }
    }
}
