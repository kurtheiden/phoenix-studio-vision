use phoenix::app_contract::ProfileCapability;
use phoenix::compatibility::*;

fn range(start: u64, end: u64) -> ByteRange {
    ByteRange::new(start, end).expect("valid range")
}

fn patch(program: u8, bank: Option<(u8, u8)>) -> PatchEvidence {
    PatchEvidence {
        source_ordinal: 4,
        source_range: range(40, 48),
        decoded_program: program,
        decoded_bank_msb: bank.map(|value| value.0),
        decoded_bank_lsb: bank.map(|value| value.1),
    }
}

fn track(observed_channel: Option<u8>, patches: Vec<PatchEvidence>) -> TrackEvidence {
    TrackEvidence {
        descriptor_ordinal: 2,
        descriptor_range: range(100, 200),
        pair_ordinal: 0,
        primary_range: range(300, 400),
        exact_event_range: range(320, 390),
        label_bytes: b"Piano".to_vec(),
        decoded_event_families: vec![EvidenceEventFamily::Note],
        decoded_event_count: 2,
        patch_evidence: patches,
        observed_channel,
    }
}

fn evidence_with(track_value: TrackEvidence) -> ProfileEvidence {
    ProfileEvidence {
        source_sha256: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into(),
        source_byte_size: 500,
        parser_profile: ParserProfileId::new("synthetic-profile"),
        sequences: vec![SequenceEvidence {
            structural_ordinal: 0,
            sequence_range: range(10, 900),
            name_bytes: b"Piano Song".to_vec(),
            name_range: range(20, 30),
            descriptor_count: 3,
            pair_count: 1,
            tracks: vec![track_value],
        }],
    }
}

fn profile_with(track_expectation: TrackExpectation) -> CompatibilityProfile {
    CompatibilityProfile {
        id: ProfileId::new("synthetic-profile"),
        version: ProfileVersion::new(1),
        display_label: "Synthetic validated profile".into(),
        project: ProjectExpectation::new(
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            500,
            ParserProfileId::new("synthetic-profile"),
            1,
        )
        .expect("valid project expectation"),
        sequences: vec![SequenceExpectation {
            structural_ordinal: 0,
            sequence_range: range(10, 900),
            expected_name_bytes: b"Piano Song".to_vec(),
            name_range: range(20, 30),
            descriptor_count: 3,
            pair_count: 1,
            track_expectations: vec![track_expectation],
        }],
    }
}

fn expectation(patches: Vec<PatchExpectation>) -> TrackExpectation {
    let key = TrackKey::new(2, 0);
    TrackExpectation {
        key: key.clone(),
        descriptor_range: range(100, 200),
        primary_range: range(300, 400),
        exact_event_range: range(320, 390),
        expected_label_bytes: Some(b"Piano".to_vec()),
        channel_policy: TrackChannelPolicy::new(key, 1).expect("valid channel"),
        patch_expectations: patches,
    }
}

fn exact_registry(patches: Vec<PatchExpectation>) -> CompatibilityRegistry {
    CompatibilityRegistry::new(vec![profile_with(expectation(patches))]).expect("valid registry")
}

fn program_patch() -> PatchExpectation {
    PatchExpectation {
        source_ordinal: 4,
        source_range: range(40, 48),
        decoded_program: 7,
        decoded_bank_msb: None,
        decoded_bank_lsb: None,
        translation: PatchTranslationPolicy::ProgramOnly { program: 7 },
    }
}

#[test]
fn empty_registry_is_no_match() {
    let result = CompatibilityRegistry::empty()
        .assess(&evidence_with(track(None, vec![])), 0)
        .expect("assessment");
    assert!(matches!(result, ProfileMatch::NoMatch));
}

#[test]
fn exact_match_returns_capability_and_resolved_policy() {
    let result = exact_registry(vec![])
        .assess(&evidence_with(track(None, vec![])), 0)
        .expect("assessment");
    let ProfileMatch::Matched {
        capability,
        resolved_policy,
    } = result
    else {
        panic!("expected exact match");
    };
    assert_eq!(
        capability,
        ProfileCapability {
            profile_id: "synthetic-profile".into(),
            profile_version: 1,
            display_label: "Synthetic validated profile".into(),
        }
    );
    assert_eq!(resolved_policy.tracks[0].midi_channel, 1);
}

#[test]
fn provenance_mismatches_are_no_match() {
    let registry = exact_registry(vec![]);
    let mut evidence = evidence_with(track(None, vec![]));
    evidence.source_sha256.replace_range(..1, "b");
    assert!(matches!(
        registry.assess(&evidence, 0).expect("assessment"),
        ProfileMatch::NoMatch
    ));
    evidence.source_sha256 =
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into();
    evidence.source_byte_size += 1;
    assert!(matches!(
        registry.assess(&evidence, 0).expect("assessment"),
        ProfileMatch::NoMatch
    ));
    evidence.source_byte_size = 500;
    evidence.parser_profile = ParserProfileId::new("other");
    assert!(matches!(
        registry.assess(&evidence, 0).expect("assessment"),
        ProfileMatch::NoMatch
    ));
}

#[test]
fn sequence_identity_mismatches_are_rejected() {
    let registry = exact_registry(vec![]);
    let mut evidence = evidence_with(track(None, vec![]));
    evidence.sequences[0].sequence_range = range(11, 900);
    assert!(matches!(
        registry.assess(&evidence, 0).expect("assessment"),
        ProfileMatch::Rejected {
            reason: ProfileMismatchReason::SequenceIdentityMismatch,
            ..
        }
    ));
    evidence.sequences[0].sequence_range = range(10, 900);
    evidence.sequences[0].name_bytes = b"Same label is not identity".to_vec();
    assert!(matches!(
        registry.assess(&evidence, 0).expect("assessment"),
        ProfileMatch::Rejected {
            reason: ProfileMismatchReason::SequenceIdentityMismatch,
            ..
        }
    ));
}

#[test]
fn track_manifest_is_exact_and_labels_are_not_identity() {
    let registry = exact_registry(vec![]);
    let mut evidence = evidence_with(track(None, vec![]));
    evidence.sequences[0].tracks[0].descriptor_range = range(101, 200);
    assert!(matches!(
        registry.assess(&evidence, 0).expect("assessment"),
        ProfileMatch::Rejected {
            reason: ProfileMismatchReason::TrackManifestMismatch,
            ..
        }
    ));
    evidence.sequences[0].tracks[0].descriptor_range = range(100, 200);
    evidence.sequences[0].tracks[0].label_bytes = b"Piano".to_vec();
    evidence.sequences[0].tracks[0].pair_ordinal = 1;
    assert!(matches!(
        registry.assess(&evidence, 0).expect("assessment"),
        ProfileMatch::Rejected {
            reason: ProfileMismatchReason::TrackManifestMismatch,
            ..
        }
    ));
}

#[test]
fn missing_and_extra_tracks_are_rejected() {
    let registry = exact_registry(vec![]);
    let mut evidence = evidence_with(track(None, vec![]));
    evidence.sequences[0].tracks.clear();
    assert!(matches!(
        registry.assess(&evidence, 0).expect("assessment"),
        ProfileMatch::Rejected {
            reason: ProfileMismatchReason::TrackManifestMismatch,
            ..
        }
    ));
    evidence.sequences[0].tracks.push(track(None, vec![]));
    evidence.sequences[0].tracks.push(track(None, vec![]));
    assert!(matches!(
        registry.assess(&evidence, 0).expect("assessment"),
        ProfileMatch::Rejected {
            reason: ProfileMismatchReason::TrackManifestMismatch,
            ..
        }
    ));
}

#[test]
fn channel_policy_has_no_default_and_observed_mismatch_rejects() {
    assert!(TrackChannelPolicy::new(TrackKey::new(2, 0), 0).is_err());
    assert!(TrackChannelPolicy::new(TrackKey::new(2, 0), 17).is_err());
    let registry = exact_registry(vec![]);
    let evidence = evidence_with(track(Some(2), vec![]));
    assert!(matches!(
        registry.assess(&evidence, 0).expect("assessment"),
        ProfileMatch::Rejected {
            reason: ProfileMismatchReason::ChannelPolicyMismatch,
            ..
        }
    ));
}

#[test]
fn program_and_banked_patch_policies_match_exactly() {
    let program_registry = exact_registry(vec![program_patch()]);
    let program_evidence = evidence_with(track(None, vec![patch(7, None)]));
    let result = program_registry
        .assess(&program_evidence, 0)
        .expect("assessment");
    assert!(matches!(result, ProfileMatch::Matched { .. }));

    let banked = PatchExpectation {
        source_ordinal: 4,
        source_range: range(40, 48),
        decoded_program: 7,
        decoded_bank_msb: Some(81),
        decoded_bank_lsb: Some(2),
        translation: PatchTranslationPolicy::BankSelectAndProgram {
            msb: 81,
            lsb: 2,
            program: 7,
        },
    };
    let result = exact_registry(vec![banked])
        .assess(
            &evidence_with(track(None, vec![patch(7, Some((81, 2)))])),
            0,
        )
        .expect("assessment");
    assert!(matches!(result, ProfileMatch::Matched { .. }));
}

#[test]
fn wrong_patch_identity_or_missing_patch_rejects() {
    let registry = exact_registry(vec![program_patch()]);
    let mut evidence = evidence_with(track(None, vec![patch(8, None)]));
    assert!(matches!(
        registry.assess(&evidence, 0).expect("assessment"),
        ProfileMatch::Rejected {
            reason: ProfileMismatchReason::PatchPolicyMismatch,
            ..
        }
    ));
    evidence.sequences[0].tracks[0].patch_evidence.clear();
    assert!(matches!(
        registry.assess(&evidence, 0).expect("assessment"),
        ProfileMatch::Rejected {
            reason: ProfileMismatchReason::PatchPolicyMismatch,
            ..
        }
    ));
}

#[test]
fn definitions_reject_invalid_hash_duplicates_and_inconsistent_patch_policy() {
    assert!(ProjectExpectation::new("not-a-hash", 1, ParserProfileId::new("p"), 1).is_err());
    let profile = profile_with(expectation(vec![]));
    assert!(matches!(
        CompatibilityRegistry::new(vec![profile.clone(), profile]),
        Err(ProfileDefinitionError::DuplicateProfile)
    ));
    let invalid = profile_with(expectation(vec![PatchExpectation {
        source_ordinal: 1,
        source_range: range(1, 2),
        decoded_program: 4,
        decoded_bank_msb: None,
        decoded_bank_lsb: None,
        translation: PatchTranslationPolicy::ProgramOnly { program: 5 },
    }]));
    assert!(matches!(
        CompatibilityRegistry::new(vec![invalid.clone()]),
        Err(ProfileDefinitionError::PatchPolicyMismatch { .. })
    ));
    let mut duplicate_profile = profile_with(expectation(vec![]));
    let duplicate_track = duplicate_profile.sequences[0].track_expectations[0].clone();
    duplicate_profile.sequences[0]
        .track_expectations
        .push(duplicate_track);
    assert!(matches!(
        CompatibilityRegistry::new(vec![duplicate_profile]),
        Err(ProfileDefinitionError::DuplicateTrackKey(_))
    ));
}

#[test]
fn duplicate_profiles_that_match_are_ambiguous_not_first_wins() {
    let mut first = profile_with(expectation(vec![]));
    let mut second = first.clone();
    second.id = ProfileId::new("another-profile");
    first.display_label = "first".into();
    let registry = CompatibilityRegistry::new(vec![first, second]).expect("valid registry");
    let error = registry
        .assess(&evidence_with(track(None, vec![])), 0)
        .expect_err("ambiguity must fail");
    assert!(matches!(error, RegistryMatchError::AmbiguousProfiles(_)));
}

#[test]
fn range_is_half_open_and_fixed_width() {
    assert!(ByteRange::new(9, 8).is_err());
    let value = ByteRange::new(u64::MAX - 1, u64::MAX).expect("boundary range");
    assert_eq!(value.length(), 1);
    assert_eq!(value.start(), u64::MAX - 1);
    assert_eq!(value.end_exclusive(), u64::MAX);
}

#[test]
fn evidence_and_profiles_are_owned() {
    fn make_values() -> (ProfileEvidence, CompatibilityProfile) {
        let evidence = evidence_with(track(None, vec![]));
        let profile = profile_with(expectation(vec![]));
        (evidence, profile)
    }
    let (evidence, profile) = make_values();
    assert_eq!(evidence.sequences[0].name_bytes, b"Piano Song");
    assert_eq!(profile.id.as_str(), "synthetic-profile");
}
