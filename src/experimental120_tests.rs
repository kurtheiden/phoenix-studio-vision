use super::*;
use crate::semantic_layout120::{associate_semantic_project_120, AssociationRefusal};

enum Track<'a> {
    Paired(&'a [u8], u32),
    PrimaryOnly,
    NoData,
}

fn record(bytes: &mut Vec<u8>, kind: u8, payload: &[u8]) {
    bytes.push(kind);
    bytes.extend((payload.len() as u32).to_be_bytes());
    bytes.extend(payload);
}

fn fixture(tracks: &[(&[u8], Track<'_>)]) -> Vec<u8> {
    let original = crate::recovery120::tests::fixture(&crate::recovery120::tests::note(), 1);
    let root = parse_root_record_stream(&original).unwrap();
    let count = tracks.len() + 2;
    let mut bytes = crate::observed_layout120::tests::synthetic_marker(
        count as u8,
        b"Proven tracks",
        [0xfe, 0xff],
    );
    bytes.truncate(8 + 75 + 120 * count);
    for (i, label) in [b"Meter Track".as_slice(), b"Tempo Track"]
        .into_iter()
        .chain(tracks.iter().map(|t| t.0))
        .enumerate()
    {
        let at = 8 + 121 + 120 * i;
        bytes[at..at + label.len()].copy_from_slice(label);
        bytes[at - 39..at - 31].copy_from_slice(&[0x80, 0, 4, 0, 0, 4, 1, 0]);
        if i >= 2 {
            bytes[at - 33..at].fill(255);
            bytes[at - 33..at - 30].copy_from_slice(&[1, 0, 1]);
            if matches!(tracks[i - 2].1, Track::NoData) {
                bytes[at - 39..at - 31].copy_from_slice(&[0, 0, 4, 0, 0, 4, 0, 0]);
            }
        }
    }
    record(&mut bytes, 7, &[0; 10]);
    // Existing partial conductor contract: an event-only padded Meter envelope.
    // The original compact secondary deliberately remains unresolved; this
    // fixture cannot acquire complete recovery authority through this change.
    let mut meter = vec![0; 38];
    meter[..2].copy_from_slice(&[0, 1]);
    meter[2..6].copy_from_slice(&1u32.to_be_bytes());
    meter[14..22].copy_from_slice(&[0, 255, 88, 4, 3, 2, 8, 8]);
    meter[22..29].copy_from_slice(&[135, 255, 255, 127, 255, 47, 0]);
    record(&mut bytes, 2, &meter);
    for index in [3, 4, 5] {
        bytes.extend(&original[root.records[index].record_range.clone()]);
    }
    for (_, track) in tracks {
        let (events, declared, paired) = match track {
            Track::NoData => continue,
            Track::Paired(events, declared) => (*events, *declared, true),
            Track::PrimaryOnly => (&[0, 255, 32, 0][..], 1, false),
        };
        let mut primary = vec![0; 14];
        primary[..2].copy_from_slice(&[0, 1]);
        primary[2..6].copy_from_slice(&declared.to_be_bytes());
        primary.extend(events);
        primary.extend([255, 0, 0, 0, 255, 47, 0]);
        record(&mut bytes, 2, &primary);
        if paired {
            record(&mut bytes, 41, &[0; 23]);
        }
    }
    // Retain sequence terminal and independently framed routing/device records.
    for framed in &root.records[8..] {
        bytes.extend(&original[framed.record_range.clone()]);
    }
    bytes
}

pub(crate) fn publication_fixture() -> Vec<u8> {
    fixture(&[
        (b"Unresolved", Track::Paired(&[0, 255, 32, 0], 1)),
        (b"Never used", Track::NoData),
        (b"Empty paired", Track::Paired(&[], 0)),
        (
            b"Playable",
            Track::Paired(&crate::recovery120::tests::note(), 1),
        ),
        (b"", Track::NoData),
    ])
}

fn musical_chunks(midi: &[u8]) -> Vec<&[u8]> {
    let mut at = 14;
    let mut chunks = vec![];
    while at < midi.len() {
        assert_eq!(&midi[at..at + 4], b"MTrk");
        let len = u32::from_be_bytes(midi[at + 4..at + 8].try_into().unwrap()) as usize;
        chunks.push(&midi[at + 8..at + 8 + len]);
        at += 8 + len;
    }
    assert_eq!(at, midi.len());
    chunks.remove(0);
    chunks
}

#[test]
fn fully_paired_partial_keeps_failed_and_empty_tracks_in_source_order() {
    let bytes = publication_fixture();
    let bridge = associate_semantic_project_120(&bytes).unwrap();
    let association = bridge.candidates[0].association.as_ref().unwrap();
    assert!(association.primary_only.is_empty());
    assert_eq!(association.ordinary.len(), 3);
    assert_eq!(association.no_event_data.len(), 2);
    let (report, midi) = build(&bytes, 0).unwrap();
    let ledger = report.partial120.as_ref().unwrap();
    assert_eq!(ledger.exported.notes, 1);
    assert_eq!(ledger.unresolved_source_objects, 1);
    assert_eq!(
        report.tracks.iter().map(|t| t.ordinal).collect::<Vec<_>>(),
        [2, 3, 4, 5, 6]
    );
    assert_eq!(
        ledger.tracks.iter().map(|t| t.ordinal).collect::<Vec<_>>(),
        [2, 3, 4, 5, 6]
    );
    assert!(report.tracks[0]
        .inclusion
        .contains("UNRESOLVED whole Track"));
    assert_eq!(ledger.tracks[0].unresolved_source_objects, 1);
    assert!(ledger.tracks[0].refusal.is_some());
    assert_eq!(report.tracks[3].channel, Some(5));
    assert_eq!(ledger.tracks[3].declared_source_objects, 1);
    let chunks = musical_chunks(&midi);
    assert_eq!(chunks.len(), 5);
    for (index, track) in report.tracks.iter().enumerate() {
        let ledger = &ledger.tracks[index];
        assert_eq!(chunks[index].last(), Some(&0)); // EOT
        if index != 3 {
            assert_eq!(track.notes, 0);
            assert!(track.channel.is_none());
        }
        if matches!(index, 1 | 4) {
            let provenance = ledger.no_event_data.as_ref().unwrap();
            let source = &association.no_event_data[usize::from(index == 4)];
            assert_eq!(provenance.descriptor_range, source.slot.view_range);
            assert_eq!(provenance.label_range, source.slot.label.range);
            assert_eq!(
                provenance.label_bytes,
                bytes[provenance.label_range.clone()]
            );
            assert_eq!(provenance.state_context_bytes, [0, 0, 4, 0, 0, 4, 0, 0]);
            assert_eq!(
                provenance.state_context_bytes,
                bytes[provenance.state_context_range.clone()]
            );
            assert!(ledger.primary_range.is_none());
            assert!(track.pair_ranges.is_none());
            assert_eq!(ledger.declared_source_objects, 0);
            assert_eq!(ledger.unresolved_source_objects, 0);
            let name = track.name.as_bytes();
            let mut exact_chunk = vec![0, 0xff, 3, name.len() as u8];
            exact_chunk.extend(name);
            exact_chunk.extend([0, 0xff, 0x2f, 0]);
            assert_eq!(chunks[index], exact_chunk);
        } else {
            assert!(ledger.primary_range.is_some());
            assert!(ledger.no_event_data.is_none());
            assert!(track.pair_ranges.is_some());
        }
    }
    // An empty paired Track retains exactly its own physical pair.
    assert_eq!(
        report.tracks[2].pair_ranges.as_ref().unwrap()[0],
        association.ordinary[1].pair.primary.record_range
    );
    assert_eq!(ledger.tracks[2].contexts, Some(0));
    assert!(ledger.tracks[2].refusal.is_none());
    assert_eq!(build(&bytes, 0).unwrap().1, midi);
    assert!(crate::recovery120::recover_project120(&bytes).unwrap()[0]
        .result
        .is_err());
}

#[test]
fn one_primary_only_can_coexist_with_paired_and_exact_no_data() {
    let bytes = fixture(&[
        (b"No data", Track::NoData),
        (b"Primary only", Track::PrimaryOnly),
        (
            b"Playable",
            Track::Paired(&crate::recovery120::tests::note(), 1),
        ),
        (b"Trailing", Track::NoData),
    ]);
    let (report, midi) = build(&bytes, 0).unwrap();
    assert_eq!(report.included_notes, 1);
    assert_eq!(report.tracks.len(), 4);
    assert!(report.tracks[1].pair_ranges.is_none());
    let ledger = report.partial120.unwrap();
    assert!(ledger.tracks[1].primary_range.is_some());
    assert!(ledger.tracks[1].no_event_data.is_none());
    assert_eq!(ledger.tracks[1].unresolved_source_objects, 1);
    assert_eq!(musical_chunks(&midi).len(), 4);
}

#[test]
fn near_zero_state_and_blank_uninitialized_shortcuts_do_not_skip_bodies() {
    let original = publication_fixture();
    let at = 8 + 121 + 120 * 3 - 39;
    for byte in 0..8 {
        let mut bytes = original.clone();
        bytes[at + byte] ^= 1;
        assert!(
            associate_semantic_project_120(&bytes).unwrap().candidates[0]
                .association
                .is_err()
        );
        assert!(build(&bytes, 0).is_err(), "near-state byte {byte}");
    }
    // Turning a descriptor that owns a real pair into exact ZERO-STATE creates
    // a surplus, rather than permission to drop or reassign that pair.
    let mut bytes = original;
    let at = 8 + 121 + 120 * 4 - 39;
    bytes[at..at + 8].copy_from_slice(&[0, 0, 4, 0, 0, 4, 0, 0]);
    assert!(matches!(
        associate_semantic_project_120(&bytes).unwrap().candidates[0].association,
        Err(AssociationRefusal::Cardinality { .. })
    ));
    assert!(build(&bytes, 0).is_err());
    // A blank label and absent Instrument list with a nonqualifying state
    // still require their own physical body.
    let mut blank = fixture(&[
        (b"", Track::NoData),
        (
            b"Playable",
            Track::Paired(&crate::recovery120::tests::note(), 1),
        ),
    ]);
    blank[8 + 121 + 120 * 2 - 39] = 0x80;
    assert!(matches!(
        associate_semantic_project_120(&blank).unwrap().candidates[0].association,
        Err(AssociationRefusal::Cardinality { .. })
    ));
    assert!(build(&blank, 0).is_err());
}

#[test]
fn shortage_surplus_multiple_primary_only_and_foreign_adjacency_still_refuse() {
    let bytes = publication_fixture();
    let root = parse_root_record_stream(&bytes).unwrap();
    for remove in [true, false] {
        let range = root.records[6].record_range.start..root.records[7].record_range.end;
        let mut mutated = bytes.clone();
        let replacement = if remove {
            vec![]
        } else {
            bytes[range.clone()].to_vec()
        };
        if remove {
            mutated.drain(range);
        } else {
            mutated.splice(range.start..range.start, replacement);
        }
        assert!(matches!(
            associate_semantic_project_120(&mutated).unwrap().candidates[0].association,
            Err(AssociationRefusal::Cardinality { .. })
        ));
        assert!(build(&mutated, 0).is_err());
    }
    // A missing secondary is supported only by the existing primary-only
    // envelope, never by a nonzero context field.
    let mut missing_secondary = bytes.clone();
    missing_secondary[root.records[6].payload.range.start + 10] = 1;
    missing_secondary.drain(root.records[7].record_range.clone());
    assert!(build(&missing_secondary, 0).is_err());
    let multiple = fixture(&[
        (b"First", Track::PrimaryOnly),
        (b"Second", Track::PrimaryOnly),
        (
            b"Playable",
            Track::Paired(&crate::recovery120::tests::note(), 1),
        ),
    ]);
    assert!(matches!(
        associate_semantic_project_120(&multiple)
            .unwrap()
            .candidates[0]
            .association,
        Err(AssociationRefusal::IncompleteNeighborhood)
    ));
    assert!(build(&multiple, 0).is_err());
    let mut interleaved = bytes.clone();
    interleaved[root.records[7].record_range.start] = 0x10;
    assert!(build(&interleaved, 0).is_err());
}

#[test]
fn paired_prefix_stops_at_refused_object_and_keeps_routing_and_counts() {
    let note = crate::recovery120::tests::note();
    let mut events = note.clone();
    events.extend([0, 0xf0, 8]); // Unsupported length: decoder establishes object start.
    events.extend(&note); // Valid-looking later music must never be scanned/resumed.
    let bytes = fixture(&[(b"Prefix", Track::Paired(&events, 3))]);
    let (report, _) = build(&bytes, 0).unwrap();
    let ledger = report.partial120.unwrap();
    assert_eq!(ledger.exported.notes, 1);
    assert_eq!(ledger.unresolved_source_objects, 2);
    let prefix = ledger.tracks[0].bounded_prefix.as_ref().unwrap();
    assert_eq!(prefix.accounted_source_objects, 1);
    assert_eq!(
        prefix.accounted_source_range.end,
        prefix.accounted_source_range.start + note.len()
    );
    assert_eq!(
        prefix.unresolved_source_range.start,
        prefix.accounted_source_range.end
    );
    assert_eq!(
        prefix.unresolved_source_range.len(),
        events.len() - note.len()
    );
    assert_eq!(report.tracks[0].channel, Some(5));
    // A prefix cannot claim a count which leaves no source objects for suffix.
    for declared in [0, 1] {
        assert!(build(
            &fixture(&[(b"Prefix", Track::Paired(&events, declared))]),
            0
        )
        .is_err());
    }
    // Unknown tags have no new prefix authority, even after a valid Note.
    let mut unknown = note.clone();
    unknown.extend([0, 255, 32, 0]);
    unknown.extend(&note);
    assert!(build(&fixture(&[(b"Unknown", Track::Paired(&unknown, 3))]), 0).is_err());
    // Every truncated initial Note stays unresolved; no playable Note invented.
    for end in 0..note.len() {
        assert!(build(
            &fixture(&[(b"Truncated", Track::Paired(&note[..end], 1))]),
            0
        )
        .is_err());
    }
    // Missing routing/device evidence cannot qualify either full music or prefix.
    let root = parse_root_record_stream(&bytes).unwrap();
    let row = root
        .records
        .iter()
        .find(|r| r.record_type.value == 16)
        .unwrap();
    let mut bad_route = bytes.clone();
    bad_route[row.payload.range.start + 25] ^= 1;
    assert!(build(&bad_route, 0).is_err());
    // Corrupt event terminal cannot be bypassed by prefix recovery.
    let primary = &root.records[6];
    let mut bad_bound = bytes.clone();
    bad_bound[primary.payload.range.end - 1] ^= 1;
    assert!(build(&bad_bound, 0).is_err());
}

#[test]
fn conductor_counts_timing_and_playable_note_requirements_remain() {
    let bytes = publication_fixture();
    let root = parse_root_record_stream(&bytes).unwrap();
    for offset in [
        root.records[2].payload.range.start + 5,
        root.records[2].payload.range.start + 14,
        root.records[4].payload.range.start + 5,
        root.records[4].payload.range.start + 14,
    ] {
        let mut changed = bytes.clone();
        changed[offset] ^= 1;
        assert!(build(&changed, 0).is_err());
    }
    assert!(build(
        &fixture(&[
            (b"Only no data", Track::NoData),
            (b"Empty paired", Track::Paired(&[], 0))
        ]),
        0
    )
    .is_err());
    let complete = crate::recovery120::tests::fixture(&crate::recovery120::tests::note(), 1);
    assert!(build(&complete, 0)
        .unwrap_err()
        .0
        .contains("already verified"));
}
