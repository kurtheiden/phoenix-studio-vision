use std::fs;

use phoenix::sequence_container::{
    parse_project_166, parse_root_record_stream, Project166Error, TrackAssociations,
};

const BASELINE: &str = "/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline";
const OLDER_SAMPLE: &str = "samples/newest STUFF";

#[test]
fn experiment_007_walks_from_root_to_all_eighteen_sequences() {
    let bytes = fs::read(BASELINE).expect("authentic Experiment 007 fixture should be available");
    let project = parse_project_166(&bytes).unwrap();

    assert_eq!(project.root_header.range, 0x0000_0000..0x0000_0008);
    assert_eq!(
        project.root_header.raw.bytes,
        &[0x00, 0x2d, 0x00, 0x93, 0x00, 0xac, 0x00, 0xa6]
    );
    assert_eq!(project.records.len(), 527);
    assert_eq!(project.consumed_range, 0..0x0003_3a0c);
    assert_eq!(project.consumed_range.end, bytes.len());
    assert_eq!(project.sequences.len(), 18);
    assert_eq!(project.sequences[0].sequence_range.start, 0x0000_6abc);

    let final_record = project.records.last().unwrap();
    assert_eq!(final_record.record_type.value, 0x05);
    assert_eq!(final_record.payload_length, 0);
    assert_eq!(final_record.record_range, 0x0003_3a07..0x0003_3a0c);
}

#[test]
fn experiment_007_derives_bells_and_ode_bounds_without_production_offsets() {
    let bytes = fs::read(BASELINE).expect("authentic Experiment 007 fixture should be available");
    let project = parse_project_166(&bytes).unwrap();

    let bells = &project.sequences[1];
    assert_eq!(bells.sequence_name.as_utf8(), Some("Bells for her"));
    assert_eq!(bells.initial_meter_range, 0x0000_eb80..0x0000_eb88);
    assert_eq!(bells.initial_tempo_range, 0x0000_ebd8..0x0000_ebdf);
    assert_eq!(
        bells.track_pairs[8].primary.record_range,
        0x0001_43b5..0x0001_495e
    );
    assert_eq!(
        bells.track_pairs[13].primary.record_range,
        0x0001_4e13..0x0001_5edb
    );

    let ode = &project.sequences[14];
    assert_eq!(ode.sequence_name.as_utf8(), Some("Ode to Clarke"));
    assert_eq!(ode.sequence_range, 0x0002_ef6f..0x0003_202c);
    assert_eq!(ode.initial_meter_range, 0x0002_f784..0x0002_f78c);
    assert_eq!(ode.initial_tempo_range, 0x0002_f7dc..0x0002_f7e3);
    assert_eq!(
        ode.track_pairs[0].primary.record_range,
        0x0002_f820..0x0002_fa7a
    );
    assert_eq!(
        ode.track_pairs[8].primary.record_range,
        0x0003_1bf5..0x0003_1fa3
    );
}

#[test]
fn experiment_007_descriptor166_track_bounds_validate_for_ordinal_pairs() {
    let bytes = fs::read(BASELINE).expect("authentic Experiment 007 fixture should be available");
    let project = parse_project_166(&bytes).unwrap();
    let mut checked = 0;
    for sequence in &project.sequences {
        if !matches!(sequence.track_associations, TrackAssociations::Ordinal(_)) {
            continue;
        }
        for pair in &sequence.track_pairs {
            let bounds = sequence
                .validated_track_event_bounds(pair.pair_ordinal)
                .expect("ordinal Descriptor166 track should have validated bounds");
            assert_eq!(bounds.event_range.start, pair.candidate_event_start);
            assert_eq!(bounds.event_range.end, bounds.tail_range.start);
            assert_eq!(bounds.tail_range.end, bounds.payload_range.end);
            assert!(bounds.event_range.start >= pair.event_containing_range.start);
            assert!(bounds.event_range.end <= pair.event_containing_range.end);
            checked += 1;
        }
    }
    assert!(checked > 0);
}

#[test]
fn experiment_007_preserves_sequence_i_mismatch_without_guessing() {
    let bytes = fs::read(BASELINE).expect("authentic Experiment 007 fixture should be available");
    let project = parse_project_166(&bytes).unwrap();
    let sequence_i = &project.sequences[8];

    assert_eq!(sequence_i.sequence_name.as_utf8(), Some("Sequence I"));
    assert_eq!(sequence_i.track_descriptors().len(), 11);
    assert_eq!(sequence_i.track_pairs.len(), 10);
    assert!(matches!(
        sequence_i.track_associations,
        TrackAssociations::Unresolved {
            descriptor_count: 11,
            pair_count: 10
        }
    ));
    assert_eq!(sequence_i.descriptors.len(), 13);
}

#[test]
fn older_sample_is_generically_framed_but_not_semantically_reinterpreted() {
    let bytes = fs::read(OLDER_SAMPLE).expect("older authentic fixture should be available");
    let root = parse_root_record_stream(&bytes).unwrap();
    assert_eq!(root.root_header.range, 0..8);
    assert_eq!(
        root.root_header.raw.bytes,
        &[0x00, 0x1e, 0x00, 0x93, 0x00, 0x46, 0x00, 0x78]
    );
    assert_eq!(root.records.len(), 495);
    assert_eq!(root.consumed_range.end, bytes.len());

    assert!(matches!(
        parse_project_166(&bytes),
        Err(Project166Error::MalformedSequenceCandidate {
            candidate_range,
            ..
        }) if candidate_range.start == 0x0000_27fc
    ));
}
