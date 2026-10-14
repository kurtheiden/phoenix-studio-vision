use phoenix::{app_contract::*, app_service::AppService};
#[test]
#[ignore = "requires authentic original COMIC BOOK"]
fn authentic_partial_sweeper_source_counts_and_publication_safety() {
    use sha2::{Digest, Sha256};
    let path = std::env::var("PHOENIX_COMIC_ORIGINAL_SOURCE").unwrap();
    let bytes = std::fs::read(&path).unwrap();
    assert_eq!(bytes.len(), 452284);
    assert_eq!(
        format!("{:x}", Sha256::digest(&bytes)),
        "22a9e2589098b0d475f6e288eeebc9303bed644aa8ce09f9971a0060c9160486"
    );
    let mut service = AppService::new();
    let response = service
        .inspect_project(InspectProjectRequest {
            contract_version: CONTRACT_VERSION,
            source_path: path.clone(),
            diagnostics_level: DiagnosticsLevel::Full,
        })
        .unwrap();
    let seq = &response.sequences[5];
    assert_eq!(seq.display_name, "sweeper");
    assert!(seq.bounded_export_capability.is_none());
    assert!(service
        .preview_experimental_sequence(&response.session_id, &seq.sequence_id)
        .is_err());
    let plan = service
        .preview_experimental_partial120(&response.session_id, &seq.sequence_id)
        .unwrap();
    println!("{}", serde_json::to_string_pretty(plan.report()).unwrap());
    assert_eq!(plan.report().tracks.len(), 24);
    let a = plan.report().partial120.as_ref().unwrap();
    assert_eq!(a.exported.tempo, 26);
    assert_eq!(a.exported.meter, 1);
    assert_eq!(a.exported.notes, 3921);
    assert_eq!(a.exported.controllers, 5870);
    assert_eq!(a.exported.patches, 15);
    assert_eq!(a.exported.pitch_bend, 373);
    assert_eq!(a.exported.sysex, 4);
    assert_eq!(a.unresolved_source_objects, 6629);
    let multi = a.tracks.iter().find(|t| t.ordinal == 9).unwrap();
    assert_eq!(multi.declared_source_objects, 46);
    assert_eq!(
        (
            multi.exported.sysex,
            multi.exported.patches,
            multi.exported.controllers
        ),
        (1, 15, 30)
    );
    assert_eq!(multi.unresolved_source_objects, 0);
    assert!(a.unresolved_source_objects > 0);
    let bass = a.tracks.iter().find(|t| t.ordinal == 4).unwrap();
    assert_eq!(bass.declared_source_objects, 741);
    assert_eq!(
        (
            bass.exported.notes,
            bass.exported.controllers,
            bass.exported.pitch_bend
        ),
        (461, 66, 213)
    );
    assert_eq!(bass.contexts, Some(1));
    assert_eq!(bass.unresolved_source_objects, 0);
    assert_eq!(
        bass.bounded_prefix.as_ref().unwrap().accounted_source_range,
        39955..43445
    );
    let square = a.tracks.iter().find(|t| t.ordinal == 7).unwrap();
    assert_eq!(
        (square.exported.notes, square.exported.controllers),
        (64, 763)
    );
    assert_eq!(square.unresolved_source_objects, 5269);
    assert_eq!(square.contexts, Some(5));
    assert_eq!(square.ff60_controller_bridges.len(), 3);
    assert_eq!(
        square
            .bounded_prefix
            .as_ref()
            .unwrap()
            .accounted_source_objects,
        832
    );
    assert_eq!(
        square
            .bounded_prefix
            .as_ref()
            .unwrap()
            .accounted_source_range,
        43854..46644
    );
    for bridge in &square.ff60_controller_bridges {
        assert_eq!(bridge.context_bytes, bytes[bridge.context_range.clone()]);
        assert_eq!(
            bridge.leading_timing_bytes,
            bytes[bridge.leading_timing_range.clone()]
        );
        assert_eq!(
            bridge.successor_timing_bytes,
            bytes[bridge.successor_timing_range.clone()]
        );
        assert_eq!(bridge.context_bytes[..6], [0xff, 0x60, 7, 0x57, 0x7f, 0]);
    }
    assert_eq!(
        &bytes[46644..46655],
        &[2, 0xff, 0x60, 6, 0x57, 0x7f, 0, 0x7c, 0x7f, 0x21, 1]
    );
    assert_eq!(
        square
            .bounded_prefix
            .as_ref()
            .unwrap()
            .unresolved_source_range,
        46644..64556
    );
    let pipe = a.tracks.iter().find(|t| t.ordinal == 10).unwrap();
    assert_eq!(
        (
            pipe.exported.notes,
            pipe.exported.controllers,
            pipe.exported.pitch_bend
        ),
        (151, 993, 22)
    );
    assert_eq!(pipe.contexts, Some(11));
    assert_eq!(pipe.unresolved_source_objects, 0);
    assert_eq!(
        pipe.bounded_prefix.as_ref().unwrap().accounted_source_range,
        70938..75072
    );
    assert_eq!(
        pipe.bounded_prefix
            .as_ref()
            .unwrap()
            .accounted_source_objects,
        1177
    );
    let piano = a.tracks.iter().find(|t| t.ordinal == 22).unwrap();
    assert_eq!(
        (piano.exported.notes, piano.exported.controllers),
        (127, 20)
    );
    assert_eq!(piano.contexts, Some(6));
    assert_eq!(piano.unresolved_source_objects, 1);
    assert_eq!(
        piano
            .bounded_prefix
            .as_ref()
            .unwrap()
            .accounted_source_range,
        104090..105015
    );
    assert_eq!(
        piano
            .bounded_prefix
            .as_ref()
            .unwrap()
            .unresolved_source_range,
        105015..105025
    );
    let sweep = a.tracks.iter().find(|t| t.ordinal == 17).unwrap();
    assert_eq!((sweep.exported.notes, sweep.exported.controllers), (0, 66));
    assert_eq!(sweep.unresolved_source_objects, 677);
    assert_eq!(
        sweep
            .bounded_prefix
            .as_ref()
            .unwrap()
            .accounted_source_range,
        98334..98578
    );
    let chord = a.tracks.iter().find(|t| t.ordinal == 11).unwrap();
    assert_eq!(chord.declared_source_objects, 2025);
    assert_eq!(chord.exported.notes, 55);
    assert_eq!(chord.exported.controllers, 1287);
    assert_eq!(chord.contexts, Some(2));
    assert_eq!(chord.unresolved_source_objects, 681);
    let prefix = chord.bounded_prefix.as_ref().unwrap();
    assert_eq!(prefix.accounted_source_range, 75359..79617);
    assert_eq!(prefix.unresolved_source_range, 79617..81802);
    assert_eq!(prefix.accounted_source_objects, 1344);
    assert_eq!(prefix.native_displayed_track_events, Some(2020));
    // Entire-body strict decoder still refuses the unsupported context.
    assert!(phoenix::mixed_event::walk_bounded_mixed_events(
        &bytes,
        phoenix::mixed_event::MixedEventBounds {
            event_range: 75359..81802
        },
        Default::default()
    )
    .is_err());
    let folder = std::path::PathBuf::from(
        std::env::var("PHOENIX_SWEEPER_LISTENING_DIR")
            .expect("supply a fresh persistent audition directory"),
    );
    std::fs::create_dir(&folder).unwrap();
    let req = ExportSequenceRequest {
        contract_version: CONTRACT_VERSION,
        session_id: response.session_id.clone(),
        sequence_id: seq.sequence_id.clone(),
        destination_folder: folder.to_string_lossy().into_owned(),
        filename_stem: "sweeper CCA-064".into(),
        collision_policy: CollisionPolicy::FailIfExists,
        operation_id: None,
    };
    assert!(service
        .export_experimental_sequence(req.clone(), &plan, false)
        .is_err());
    assert_eq!(std::fs::read_dir(&folder).unwrap().count(), 0);
    let out = service
        .export_experimental_sequence(req.clone(), &plan, true)
        .unwrap();
    println!("OUTPUT {}", out.output_path);
    println!("SIDECAR {}", out.limitations_path);
    let midi = std::fs::read(&out.output_path).unwrap();
    assert_eq!(&midi[..4], b"MThd");
    assert_eq!(u16::from_be_bytes([midi[10], midi[11]]), 25);
    assert!(midi.windows(12).any(|w| w == b"EXPERIMENTAL"));
    assert!(midi
        .windows(12)
        .any(|w| w == [0xf0, 10, 0x41, 0x10, 0x42, 0x12, 0x40, 0, 0x7f, 0, 0x41, 0xf7]));
    let parsed = midi_tracks(&midi);
    // Prior-checkpoint comparisons below exclude only the 12 newly recovered
    // Seashore objects and restore that checkpoint's EOT tick.
    let mut prior_parsed = parsed.clone();
    prior_parsed[16] = vec![parsed[16][0].clone()];
    prior_parsed[16].extend(
        parsed[16]
            .iter()
            .filter(|e| e.1 >> 4 == 11)
            .take(57)
            .cloned(),
    );
    prior_parsed[16].push((0, 0x2f, vec![]));
    let baseline062_path = std::env::var("PHOENIX_SWEEPER_CCA062_MIDI").unwrap();
    let baseline062_bytes = std::fs::read(&baseline062_path).unwrap();
    assert_eq!(
        format!("{:x}", Sha256::digest(&baseline062_bytes)),
        "3011dc312e4fd8102d425a711d05fc54bd18ca8c7529912e076188f132331ac0"
    );
    let baseline062 = midi_tracks(&baseline062_bytes);
    assert_eq!(baseline062, prior_parsed);
    let baseline062_report: serde_json::Value = serde_json::from_slice(
        &std::fs::read(std::path::Path::new(&baseline062_path).with_extension("limitations.json"))
            .unwrap(),
    )
    .unwrap();
    let report064 = serde_json::to_value(plan.report()).unwrap();
    for (before, after) in baseline062_report["partial120"]["tracks"]
        .as_array()
        .unwrap()
        .iter()
        .zip(report064["partial120"]["tracks"].as_array().unwrap())
    {
        if before["ordinal"] != 17 {
            assert_eq!(before, after, "CCA-062 Track ledger changed");
        }
    }
    let new_seashore: Vec<_> = parsed[16]
        .iter()
        .filter(|e| e.1 == 0xf0 || e.1 >> 4 == 11)
        .cloned()
        .collect();
    assert_eq!(new_seashore.len(), 69);
    assert_eq!(sweep.sysex_framing.len(), 3);
    for (p, range, payload_range, ordinal) in [
        (&sweep.sysex_framing[0], 98506..98518, 98510..98518, 57),
        (&sweep.sysex_framing[1], 98531..98545, 98535..98545, 62),
        (&sweep.sysex_framing[2], 98555..98569, 98559..98569, 66),
    ] {
        assert_eq!(p.representation_range, range);
        assert_eq!(p.payload_range, payload_range);
        assert_eq!(p.logical_source_ordinal, ordinal);
        assert_eq!(p.source_bytes, bytes[p.representation_range.clone()]);
        assert_eq!(p.midi_payload, bytes[p.payload_range.clone()]);
        assert_eq!(p.framing_value, 0);
        assert_eq!(p.absolute_tick, 0);
    }
    assert_eq!(
        sweep
            .bounded_prefix
            .as_ref()
            .unwrap()
            .unresolved_source_range,
        98578..100653
    );
    assert_eq!(
        sweep
            .bounded_prefix
            .as_ref()
            .unwrap()
            .accounted_source_objects,
        69
    );
    assert_eq!(sweep.exported.sysex, 3);

    let bass = &parsed[3];
    let notes: Vec<_> = bass.iter().filter(|e| e.1 == 0x91).collect();
    assert_eq!(notes.len(), 461);
    assert_eq!(*notes[0], (7680, 0x91, vec![30, 127]));
    assert_eq!(*notes.last().unwrap(), &(123840, 0x91, vec![46, 85]));
    assert!(bass.contains(&(7758, 0x81, vec![30, 61])));
    assert_eq!(bass.iter().filter(|e| e.1 == 0xe1).count(), 213);
    let pan: Vec<_> = bass.iter().filter(|e| e.1 == 0xb1).collect();
    assert_eq!(pan.len(), 66);
    assert!(pan.iter().all(|e| e.2[0] == 10));
    assert_eq!(*pan[0], (6720, 0xb1, vec![10, 62]));
    let square = &parsed[6];
    assert!(square.contains(&(3840, 0x93, vec![42, 127])));
    assert!(square.contains(&(3881, 0x83, vec![42, 102])));
    assert_eq!(square.iter().filter(|e| e.1 >> 4 == 9).count(), 64);
    assert_eq!(square.iter().filter(|e| e.1 == 0xb3).count(), 763);
    let chord = &parsed[10]; // source ordinal 11, not Logic Track 7
    let ons: Vec<_> = chord.iter().filter(|e| e.1 == 0x95).collect();
    assert_eq!(ons.len(), 55);
    assert_eq!(
        ons[..3],
        [
            &(3840, 0x95, vec![58, 77]),
            &(3842, 0x95, vec![61, 77]),
            &(3843, 0x95, vec![66, 77])
        ]
    );
    // Independently read back exact source duration and release velocity.
    for event in [
        (7431, 0x85, vec![58, 29]),
        (7418, 0x85, vec![61, 43]),
        (7422, 0x85, vec![66, 40]),
    ] {
        assert!(chord.contains(&event));
    }
    assert_eq!(ons.last().unwrap().0, 55674);
    assert_eq!(ons.iter().map(|e| e.2[0]).min(), Some(42));
    assert_eq!(ons.iter().map(|e| e.2[0]).max(), Some(66));
    assert!(chord
        .iter()
        .filter(|e| e.1 >> 4 == 9 || e.1 >> 4 == 8)
        .all(|e| e.1 & 15 == 5));
    let cc: Vec<_> = chord.iter().filter(|e| e.1 == 0xb5).collect();
    assert_eq!(cc.len(), 1287);
    for (number, count) in [(10, 61), (99, 1), (98, 1), (6, 1224)] {
        assert_eq!(cc.iter().filter(|e| e.2[0] == number).count(), count);
    }
    assert_eq!(*cc[0], (2880, 0xb5, vec![10, 60]));
    assert_eq!(*cc[1], (2892, 0xb5, vec![10, 59]));
    let selectors: Vec<_> = cc
        .iter()
        .filter(|e| e.2[0] == 99 || e.2[0] == 98)
        .map(|e| (e.0, e.2.clone()))
        .collect();
    assert_eq!(selectors, [(3840, vec![99, 1]), (3840, vec![98, 32])]);
    // Compare the independent CCA-057 artifact, not a fresh decoder-generated baseline.
    let previous = std::fs::read(
        std::env::var("PHOENIX_SWEEPER_CCA057_MIDI").expect("existing CCA-057 MIDI path"),
    )
    .unwrap();
    assert_eq!(
        format!("{:x}", Sha256::digest(&previous)),
        "d08761ce16074b2d345338d8a2d45ca01099956bc052f09396218c539093551c"
    );
    let old = midi_tracks(&previous);
    assert_eq!(old.len(), parsed.len());
    for (i, (before, after)) in old.iter().zip(&prior_parsed).enumerate() {
        if ![6, 7, 11, 12, 15].contains(&i) {
            assert_eq!(
                normalize_cc(before),
                normalize_cc(after),
                "prior Track {i} changed beyond Controller targets"
            );
        }
    }
    // All prior Pipe channel events, including releases, remain an exact prefix.
    let channel_events = |track: &Vec<(u32, u8, Vec<u8>)>| {
        track
            .iter()
            .filter(|e| (0x80..0xf0).contains(&e.1))
            .map(|e| {
                let mut e = e.clone();
                if e.1 >> 4 == 11 {
                    e.1 = 0xb0;
                }
                e
            })
            .collect::<Vec<_>>()
    };
    let native = midi_tracks(
        &std::fs::read(
            std::env::var("PHOENIX_SWEEPER_NATIVE_MIDI")
                .expect("authentic native Sweeper reference"),
        )
        .unwrap(),
    );
    assert_eq!(
        new_seashore,
        native[13]
            .iter()
            .filter(|e| e.1 == 0xf0 || e.1 >> 4 == 11)
            .take(69)
            .cloned()
            .collect::<Vec<_>>()
    );
    let baseline058 = std::fs::read(std::env::var("PHOENIX_SWEEPER_CCA058_MIDI").unwrap()).unwrap();
    assert_eq!(
        format!("{:x}", Sha256::digest(&baseline058)),
        "302a003fb6941d108425ee23b8d248f3bac06ed6e7eded8932dc38b89bdc7305"
    );
    let baseline058 = midi_tracks(&baseline058);
    for (i, (before, after)) in baseline058.iter().zip(&prior_parsed).enumerate() {
        if i != 6 {
            assert_eq!(
                normalize_cc(before),
                normalize_cc(after),
                "CCA-058 track {i} changed beyond Controller targets"
            );
        }
    }
    // Old Square events remain present and ordered; its old Note Off now falls
    // among the newly recovered Controllers rather than at the old track end.
    let old_square = channel_events(&baseline058[6]);
    let new_square = channel_events(&parsed[6]);
    let mut remaining = new_square.as_slice();
    for event in old_square {
        let at = remaining
            .iter()
            .position(|e| e == &event)
            .expect("lost prior Square event");
        remaining = &remaining[at + 1..];
    }
    let square_ons: Vec<_> = parsed[6]
        .iter()
        .filter(|e| e.1 == 0x93 && e.2[1] > 0)
        .cloned()
        .collect();
    let native_square_ons: Vec<_> = native[3]
        .iter()
        .filter(|e| e.1 == 0x93 && e.2[1] > 0)
        .take(64)
        .cloned()
        .collect();
    assert_eq!(square_ons, native_square_ons);
    let square_cc: Vec<_> = parsed[6]
        .iter()
        .filter(|e| e.1 == 0xb3)
        .map(|e| (e.0, e.2.clone()))
        .collect();
    let native_square_cc: Vec<_> = native[3]
        .iter()
        .filter(|e| e.1 == 0xb3)
        .take(763)
        .map(|e| (e.0, e.2.clone()))
        .collect();
    assert_eq!(square_cc, native_square_cc); // resolved channel is separately validated below
                                             // CCA-062 changes only destination bytes of the CCA-059 Controllers.
    let baseline059_path = std::env::var("PHOENIX_SWEEPER_CCA059_MIDI").unwrap();
    let baseline059_bytes = std::fs::read(&baseline059_path).unwrap();
    assert_eq!(
        format!("{:x}", Sha256::digest(&baseline059_bytes)),
        "cb3e7386137f58d4c7f27add78bd3175fcf1849067bd3567872031c3b5fdac32"
    );
    let baseline059 = midi_tracks(&baseline059_bytes);
    let baseline_report: serde_json::Value = serde_json::from_slice(
        &std::fs::read(std::path::Path::new(&baseline059_path).with_extension("limitations.json"))
            .unwrap(),
    )
    .unwrap();
    let now_report = serde_json::to_value(plan.report()).unwrap();
    let mut changed_targets = 0;
    for (index, (before, after)) in baseline059.iter().zip(&prior_parsed).enumerate() {
        assert_eq!(
            before.len(),
            after.len(),
            "event coverage changed in chunk {index}"
        );
        let ledger = a.tracks.iter().find(|t| t.ordinal == index + 1);
        let mut targets = ledger
            .map(|t| t.controller_targets.as_slice())
            .unwrap_or(&[])
            .iter()
            .take(if index == 16 { 57 } else { usize::MAX });
        for (before, after) in before.iter().zip(after) {
            if before.1 >> 4 == 11 {
                let p = targets
                    .next()
                    .expect("missing Controller resolution provenance");
                assert_eq!((before.0, &before.2), (after.0, &after.2));
                assert_eq!(before.1, p.encoded_status);
                assert_eq!(after.1, 0xb0 + p.resolved_channel - 1);
                assert_eq!(p.position, after.0);
                assert_eq!(p.selector, p.encoded_status & 15);
                assert_eq!(bytes[p.status_offset], p.encoded_status);
                assert_eq!(bytes[p.controller_number_offset], p.controller_number);
                assert_eq!(bytes[p.controller_value_offset], p.controller_value);
                assert_eq!(bytes[p.instrument_identity_offset], p.instrument_row);
                assert_eq!(bytes[p.channel_offset] + 1, p.resolved_channel);
                assert_eq!(
                    bytes[p.list_entry_range.start..p.list_entry_range.end],
                    [0, p.instrument_row]
                );
                assert_ne!(before.1, after.1);
                changed_targets += 1;
            } else {
                assert_eq!(before, after, "non-Controller changed in chunk {index}");
            }
        }
        assert!(targets.next().is_none());
    }
    assert_eq!(changed_targets, 5861);
    for (before, after) in baseline_report["partial120"]["tracks"]
        .as_array()
        .unwrap()
        .iter()
        .zip(now_report["partial120"]["tracks"].as_array().unwrap())
    {
        if before["ordinal"] == 17 {
            continue;
        } // exact CCA-062 comparison above, plus new provenance
        for field in [
            "ordinal",
            "primary_range",
            "declared_source_objects",
            "exported",
            "contexts",
            "unresolved_source_objects",
            "refusal",
            "bounded_prefix",
            "ff60_controller_bridges",
            "instrument_pitch_provenance",
        ] {
            assert_eq!(
                before[field], after[field],
                "CCA-059 ledger/provenance changed: {field}"
            );
        }
    }
    // Source ownership identifies counterparts; native events supply test evidence only.
    for (source_ordinal, native_index) in [
        (3, 1),
        (4, 2),
        (7, 3),
        (9, 5),
        (10, 6),
        (11, 7),
        (14, 10),
        (15, 11),
        (17, 13),
        (20, 16),
        (22, 18),
        (24, 19),
    ] {
        let actual: Vec<_> = parsed[source_ordinal - 1]
            .iter()
            .filter(|e| e.1 >> 4 == 11)
            .cloned()
            .collect();
        let expected: Vec<_> = native[native_index]
            .iter()
            .filter(|e| e.1 >> 4 == 11)
            .take(actual.len())
            .cloned()
            .collect();
        assert_eq!(
            actual, expected,
            "native Controller mismatch in source Track {source_ordinal}"
        );
    }
    let native_notes = native
        .iter()
        .flatten()
        .filter(|e| e.1 >> 4 == 9 && e.2[1] > 0)
        .count();
    let native_controllers = native.iter().flatten().filter(|e| e.1 >> 4 == 11).count();
    assert_eq!(native_notes - a.exported.notes as usize, 492);
    assert_eq!(native_controllers - a.exported.controllers as usize, 6103);
    let mut changed = 0;
    for (phoenix_index, native_index, ordinal, expected) in [
        (7, 4, 8, 950),
        (11, 8, 12, 328),
        (12, 9, 13, 394),
        (15, 12, 16, 622),
    ] {
        assert_eq!(old[phoenix_index].len(), parsed[phoenix_index].len());
        for (before, after) in old[phoenix_index].iter().zip(&parsed[phoenix_index]) {
            let mut want = before.clone();
            if want.1 >> 4 == 9 || want.1 >> 4 == 8 {
                want.2[0] -= 12;
            }
            assert_eq!(*after, want, "unexpected event change in {ordinal}");
        }
        let note_ons = |track: &Vec<(u32, u8, Vec<u8>)>| {
            track
                .iter()
                .filter(|e| e.1 >> 4 == 9 && e.2[1] != 0)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(
            note_ons(&parsed[phoenix_index]),
            note_ons(&native[native_index])
        );
        let provenance = a
            .tracks
            .iter()
            .find(|t| t.ordinal == ordinal)
            .unwrap()
            .instrument_pitch_provenance
            .as_ref()
            .unwrap();
        assert_eq!(provenance.instrument.encoded_value, 0xf4);
        assert_eq!(provenance.instrument.semitones, -12);
        assert_eq!(provenance.changed_notes.len(), expected);
        for note in &provenance.changed_notes {
            assert_eq!(note.original_pitch, note.exported_pitch + 12);
            assert_eq!(note.applied_semitones, -12);
        }
        changed += expected;
    }
    assert_eq!(changed, 2294);
    assert_eq!(
        a.tracks
            .iter()
            .filter_map(|t| t.instrument_pitch_provenance.as_ref())
            .map(|p| p.changed_notes.len())
            .sum::<usize>(),
        2294
    );
    let old_pipe = channel_events(&old[9]);
    let new_pipe = channel_events(&parsed[9]);
    assert_eq!(&new_pipe[..old_pipe.len()], old_pipe);
    for event in [
        (121656, 0x94, vec![73, 102]),
        (121698, 0x84, vec![73, 102]),
        (121796, 0x94, vec![73, 98]),
        (122029, 0x84, vec![73, 69]),
        (122033, 0xb4, vec![1, 4]),
        (122037, 0x94, vec![75, 96]),
        (122086, 0x84, vec![75, 102]),
        (122040, 0xb4, vec![1, 21]),
        (122044, 0xb4, vec![1, 46]),
        (122051, 0xb4, vec![1, 70]),
        (122061, 0xb4, vec![1, 90]),
    ] {
        assert!(parsed[9].contains(&event), "native Pipe witness {event:?}");
    }
    assert!(!parsed[9].contains(&(121801, 0x94, vec![75, 96])));
    assert_eq!(parsed[9].iter().filter(|e| e.1 == 0x94).count(), 151);
    assert_eq!(parsed[9].iter().filter(|e| e.1 == 0xb4).count(), 993);
    assert!(new_pipe
        .iter()
        .filter(|e| e.1 >> 4 == 9 || e.1 >> 4 == 8 || e.1 >> 4 == 14)
        .all(|e| e.1 & 15 == 4));
    let piano_events = &parsed[21];
    let piano_notes: Vec<_> = piano_events.iter().filter(|e| e.1 >> 4 == 9).collect();
    assert_eq!(piano_notes.len(), 127);
    assert!(piano_notes.iter().all(|e| e.1 == 0x9c));
    assert!(piano_notes.iter().any(|e| (128640..130560).contains(&e.0))); // measure 68, 4/4
    assert_eq!(parsed[16].iter().filter(|e| e.1 >> 4 == 11).count(), 66);
    assert!(parsed[16]
        .iter()
        .filter(|e| e.1 >> 4 == 11)
        .take(64)
        .all(|e| e.0 == 0 && e.1 == 0xb6));
    let setup: Vec<_> = parsed[13].iter().filter(|e| e.1 >> 4 == 11).collect();
    assert_eq!(setup.len(), 14);
    assert!(setup.iter().all(|e| e.0 == 0));
    for (i, lsb) in [102, 100, 33, 32].into_iter().enumerate() {
        assert_eq!(setup[i * 3].2, [99, 1]);
        assert_eq!(setup[i * 3 + 1].2, [98, lsb]);
        assert_eq!(setup[i * 3 + 2].2[0], 6);
    }
    assert_eq!(setup[12].2, [91, 95]);
    // Authentic serialized value is 33; owner transcription says 35.
    // Preserve bytes rather than manufacture agreement.
    assert_eq!(setup[13].2, [93, 33]);
    let multi = &parsed[8];
    assert_eq!(
        multi[1],
        (
            0,
            0xf0,
            vec![0x41, 0x10, 0x42, 0x12, 0x40, 0, 0x7f, 0, 0x41, 0xf7]
        )
    );
    assert_eq!(multi[2], (70, 0xc6, vec![122]));
    let pcs = multi
        .iter()
        .filter(|e| e.1 >> 4 == 12)
        .map(|e| (e.0, e.2[0]))
        .collect::<Vec<_>>();
    assert_eq!(
        pcs,
        [
            (70, 122),
            (75, 26),
            (80, 81),
            (85, 10),
            (90, 80),
            (95, 99),
            (101, 79),
            (450, 90),
            (1920, 102),
            (2480, 6),
            (2485, 0),
            (2490, 92),
            (56640, 87),
            (107520, 87),
            (107520, 46)
        ]
    );
    // CCA-042 independently recorded per-channel selector/data tuple positions.
    for (tick, status, number, value) in [
        (320, 0xb2, 99, 1),
        (325, 0xb2, 98, 102),
        (330, 0xb2, 6, 77),
        (385, 0xb1, 99, 1),
        (390, 0xb1, 98, 102),
        (395, 0xb1, 6, 80),
    ] {
        assert!(multi.contains(&(tick, status, vec![number, value])));
    }
    assert_eq!(parsed[0].iter().filter(|e| e.1 == 81).count(), 26);
    let last = parsed[0].iter().rfind(|e| e.1 == 81).unwrap();
    assert_eq!(last, &(144001, 81, vec![14, 78, 28]));
    assert!(service
        .export_experimental_sequence(req, &plan, true)
        .is_err());
    assert_eq!(std::fs::read(&out.output_path).unwrap(), midi);
    assert_eq!(std::fs::read(&path).unwrap(), bytes);
}

#[path = "support/reference_smf.rs"]
mod reference_smf;
use reference_smf::midi_tracks;

fn normalize_cc(track: &[(u32, u8, Vec<u8>)]) -> Vec<(u32, u8, Vec<u8>)> {
    track
        .iter()
        .map(|e| {
            let mut e = e.clone();
            if e.1 >> 4 == 11 {
                e.1 = 0xb0;
            }
            e
        })
        .collect()
}
