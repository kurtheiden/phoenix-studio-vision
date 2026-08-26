//! Compiled-in, provenance-locked compatibility profiles.
//!
//! Target-specific policy belongs here, outside generic parser, evidence, and
//! application-service modules. The built-in profile is an authenticated
//! research policy, not general Studio Vision support.

use crate::compatibility::{
    AuthenticatedTrackOmissionExpectation, ByteRange, CompatibilityProfile, CompatibilityRegistry,
    IncludedTrackOutputExpectation, OmittedPatchExpectation, ParserProfileId, PatchExpectation,
    PatchTranslationPolicy, ProfileDefinitionError, ProfileId, ProfileVersion, ProjectExpectation,
    SequenceExpectation, TrackChannelPolicy, TrackExpectation, TrackKey,
    TrackOutputDispositionExpectation,
};

const ODE_PROFILE_ID: &str = "studio_vision_ode_to_clarke_v1";
const ODE_DISPLAY_LABEL: &str = "Validated research profile — Ode to Clarke";
const ODE_SOURCE_SHA256: &str = "e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132";
const ODE_SOURCE_SIZE: u64 = 211_468;
const BELLS_PROFILE_ID: &str = "studio_vision_bells_for_her_v1";
const BELLS_DISPLAY_LABEL: &str = "Validated research profile — Bells for her";
const BELLS_SOURCE_SHA256: &str = ODE_SOURCE_SHA256;
const BELLS_SOURCE_SIZE: u64 = ODE_SOURCE_SIZE;

fn range(start: u64, end: u64) -> ByteRange {
    ByteRange::new(start, end).expect("built-in profile range is valid")
}

fn patch(
    source_range: (u64, u64),
    program: u8,
    translation: PatchTranslationPolicy,
) -> PatchExpectation {
    patch_with_ordinal(source_range, 0, program, translation)
}

fn patch_with_ordinal(
    source_range: (u64, u64),
    source_ordinal: u32,
    program: u8,
    translation: PatchTranslationPolicy,
) -> PatchExpectation {
    PatchExpectation {
        source_ordinal,
        source_range: range(source_range.0, source_range.1),
        decoded_program: program,
        decoded_bank_msb: None,
        decoded_bank_lsb: None,
        translation,
    }
}

fn omitted_patch(
    source_range: (u64, u64),
    source_ordinal: u32,
    program: u8,
) -> OmittedPatchExpectation {
    OmittedPatchExpectation {
        source_ordinal,
        source_range: range(source_range.0, source_range.1),
        decoded_program: program,
        decoded_bank_msb: None,
        decoded_bank_lsb: None,
    }
}

#[allow(clippy::too_many_arguments)]
fn track(
    descriptor_ordinal: u32,
    descriptor_range: (u64, u64),
    pair_ordinal: u32,
    primary_range: (u64, u64),
    event_range: (u64, u64),
    label: &'static [u8],
    channel: u8,
    patches: Vec<PatchExpectation>,
) -> Result<TrackExpectation, ProfileDefinitionError> {
    let key = TrackKey::new(descriptor_ordinal, pair_ordinal);
    Ok(TrackExpectation {
        key: key.clone(),
        descriptor_range: range(descriptor_range.0, descriptor_range.1),
        primary_range: range(primary_range.0, primary_range.1),
        exact_event_range: Some(range(event_range.0, event_range.1)),
        expected_label_bytes: Some(label.to_vec()),
        output: TrackOutputDispositionExpectation::Included(IncludedTrackOutputExpectation {
            channel_policy: TrackChannelPolicy::new(key, channel)?,
            patch_expectations: patches,
        }),
    })
}

#[allow(clippy::too_many_arguments)]
fn omitted_nonempty_track(
    descriptor_ordinal: u32,
    descriptor_range: (u64, u64),
    pair_ordinal: u32,
    primary_range: (u64, u64),
    event_range: (u64, u64),
    label: &'static [u8],
    decoded_event_count: u64,
    decoded_event_families: Vec<crate::compatibility::EvidenceEventFamily>,
    patch_expectations: Vec<OmittedPatchExpectation>,
) -> TrackExpectation {
    TrackExpectation {
        key: TrackKey::new(descriptor_ordinal, pair_ordinal),
        descriptor_range: range(descriptor_range.0, descriptor_range.1),
        primary_range: range(primary_range.0, primary_range.1),
        exact_event_range: Some(range(event_range.0, event_range.1)),
        expected_label_bytes: Some(label.to_vec()),
        output: TrackOutputDispositionExpectation::Omitted(
            AuthenticatedTrackOmissionExpectation::AuthenticatedNonempty {
                decoded_event_count,
                decoded_event_families,
                patch_expectations,
            },
        ),
    }
}

fn omitted_empty_track(
    descriptor_ordinal: u32,
    descriptor_range: (u64, u64),
    pair_ordinal: u32,
    primary_range: (u64, u64),
    event_range: (u64, u64),
    label: &'static [u8],
) -> TrackExpectation {
    TrackExpectation {
        key: TrackKey::new(descriptor_ordinal, pair_ordinal),
        descriptor_range: range(descriptor_range.0, descriptor_range.1),
        primary_range: range(primary_range.0, primary_range.1),
        exact_event_range: Some(range(event_range.0, event_range.1)),
        expected_label_bytes: Some(label.to_vec()),
        output: TrackOutputDispositionExpectation::Omitted(
            AuthenticatedTrackOmissionExpectation::StructuralEmpty,
        ),
    }
}

/// The authenticated Ode target sequence profile, isolated from generic Core.
pub fn ode_to_clarke_profile() -> Result<CompatibilityProfile, ProfileDefinitionError> {
    let tracks = vec![
        track(
            2,
            (0x02f18b, 0x02f231),
            0,
            (0x02f820, 0x02fa7a),
            (0x02f833, 0x02fa73),
            b"Track 1",
            1,
            vec![patch(
                (0x02f833, 0x02f853),
                61,
                PatchTranslationPolicy::ProgramOnly { program: 61 },
            )],
        )?,
        track(
            3,
            (0x02f231, 0x02f2d7),
            1,
            (0x02fb42, 0x0300df),
            (0x02fb55, 0x0300d8),
            b"Track 2",
            2,
            vec![patch(
                (0x02fb55, 0x02fb75),
                37,
                PatchTranslationPolicy::BankSelectAndProgram {
                    msb: 81,
                    lsb: 1,
                    program: 37,
                },
            )],
        )?,
        track(
            4,
            (0x02f2d7, 0x02f37d),
            2,
            (0x0301b7, 0x03097d),
            (0x0301ca, 0x030976),
            b"sys100loops",
            10,
            vec![],
        )?,
        track(
            5,
            (0x02f37d, 0x02f423),
            3,
            (0x030a17, 0x030e9f),
            (0x030a2a, 0x030e98),
            b"Track 4",
            10,
            vec![],
        )?,
        track(
            6,
            (0x02f423, 0x02f4c9),
            4,
            (0x030f31, 0x03125b),
            (0x030f44, 0x031254),
            b"Track 5",
            10,
            vec![],
        )?,
        track(
            7,
            (0x02f4c9, 0x02f56f),
            5,
            (0x0312ed, 0x03156b),
            (0x031300, 0x031564),
            b"Track 3",
            1,
            vec![patch(
                (0x031300, 0x03131b),
                29,
                PatchTranslationPolicy::BankSelectAndProgram {
                    msb: 81,
                    lsb: 2,
                    program: 29,
                },
            )],
        )?,
        track(
            8,
            (0x02f56f, 0x02f615),
            6,
            (0x03165b, 0x031805),
            (0x03166e, 0x0317fe),
            b"Track 6",
            10,
            vec![],
        )?,
        track(
            9,
            (0x02f615, 0x02f6bb),
            7,
            (0x031873, 0x031b05),
            (0x031886, 0x031afe),
            b"Track 3 #2",
            15,
            vec![patch(
                (0x031886, 0x0318b5),
                23,
                PatchTranslationPolicy::ProgramOnly { program: 23 },
            )],
        )?,
        track(
            10,
            (0x02f6bb, 0x02f761),
            8,
            (0x031bf5, 0x031fa3),
            (0x031c08, 0x031f9c),
            b"Track 7",
            10,
            vec![],
        )?,
    ];
    Ok(CompatibilityProfile {
        id: ProfileId::new(ODE_PROFILE_ID),
        version: ProfileVersion::new(1),
        display_label: ODE_DISPLAY_LABEL.into(),
        project: ProjectExpectation::new(
            ODE_SOURCE_SHA256,
            ODE_SOURCE_SIZE,
            ParserProfileId::new("descriptor166"),
            18,
        )?,
        sequences: vec![SequenceExpectation {
            structural_ordinal: 14,
            sequence_range: range(0x02ef6f, 0x03202c),
            expected_name_bytes: b"Ode to Clarke".to_vec(),
            name_range: range(0x02f753, 0x02f760),
            descriptor_count: 11,
            pair_count: 9,
            track_expectations: tracks,
        }],
    })
}

/// The authenticated Bells for her target sequence, including all structural
/// rows and explicit output dispositions established by the reference export.
pub fn bells_for_her_profile() -> Result<CompatibilityProfile, ProfileDefinitionError> {
    use crate::compatibility::EvidenceEventFamily::{Note, Patch};
    let tracks = vec![
        track(
            2,
            (0x00e21b, 0x00e2c1),
            0,
            (0x00ec1c, 0x010543),
            (0x00ec2f, 0x01053c),
            b"Track 1",
            1,
            vec![patch(
                (0x00ec2f, 0x00ec53),
                16,
                PatchTranslationPolicy::ProgramOnly { program: 16 },
            )],
        )?,
        omitted_nonempty_track(
            3,
            (0x00e2c1, 0x00e367),
            1,
            (0x0106cf, 0x010946),
            (0x0106e2, 0x01093f),
            b"Track 2",
            83,
            vec![Patch, Note],
            vec![omitted_patch((0x0106e2, 0x010705), 0, 73)],
        ),
        track(
            4,
            (0x00e367, 0x00e40d),
            2,
            (0x010a3a, 0x0110cf),
            (0x010a4d, 0x0110c8),
            b"Track 3",
            16,
            vec![patch(
                (0x010a4d, 0x010a6d),
                25,
                PatchTranslationPolicy::BankSelectMsbAndProgram {
                    msb: 81,
                    program: 25,
                },
            )],
        )?,
        track(
            5,
            (0x00e40d, 0x00e4b3),
            3,
            (0x011208, 0x011931),
            (0x01121b, 0x01192a),
            b"Track 4",
            2,
            vec![patch(
                (0x01121b, 0x01123a),
                34,
                PatchTranslationPolicy::BankSelectAndProgram {
                    msb: 81,
                    lsb: 1,
                    program: 34,
                },
            )],
        )?,
        track(
            6,
            (0x00e4b3, 0x00e559),
            4,
            (0x011a8a, 0x011d28),
            (0x011a9d, 0x011d21),
            b"Track 5",
            3,
            vec![patch(
                (0x011a9d, 0x011abc),
                70,
                PatchTranslationPolicy::BankSelectAndProgram {
                    msb: 80,
                    lsb: 0,
                    program: 70,
                },
            )],
        )?,
        track(
            7,
            (0x00e559, 0x00e5ff),
            5,
            (0x011e99, 0x0123e4),
            (0x011eac, 0x0123dd),
            b"Track 6",
            1,
            vec![patch_with_ordinal(
                (0x011eb6, 0x011ee4),
                1,
                35,
                PatchTranslationPolicy::BankSelectMsbAndProgram {
                    msb: 81,
                    program: 35,
                },
            )],
        )?,
        omitted_nonempty_track(
            8,
            (0x00e5ff, 0x00e6a5),
            6,
            (0x0124cb, 0x012974),
            (0x0124de, 0x01296d),
            b"Track 7",
            165,
            vec![Patch, Note],
            vec![omitted_patch((0x0124de, 0x012503), 0, 58)],
        ),
        track(
            9,
            (0x00e6a5, 0x00e74b),
            7,
            (0x012a87, 0x014261),
            (0x012a9a, 0x01425a),
            b"Track 8",
            16,
            vec![patch(
                (0x012a9a, 0x012abe),
                25,
                PatchTranslationPolicy::BankSelectMsbAndProgram {
                    msb: 81,
                    program: 25,
                },
            )],
        )?,
        track(
            10,
            (0x00e74b, 0x00e7f1),
            8,
            (0x0143b5, 0x01495e),
            (0x0143c8, 0x014957),
            b"Track 9",
            12,
            vec![patch_with_ordinal(
                (0x0143d1, 0x0143fd),
                1,
                122,
                PatchTranslationPolicy::BankSelectMsbAndProgram {
                    msb: 81,
                    program: 122,
                },
            )],
        )?,
        omitted_empty_track(
            11,
            (0x00e7f1, 0x00e897),
            9,
            (0x014a4c, 0x014a66),
            (0x014a5f, 0x014a5f),
            b"Track 10",
        ),
        track(
            12,
            (0x00e897, 0x00e93d),
            10,
            (0x014a8a, 0x014c99),
            (0x014a9d, 0x014c92),
            b"Track 11",
            8,
            vec![patch(
                (0x014a9d, 0x014abf),
                12,
                PatchTranslationPolicy::BankSelectAndProgram {
                    msb: 80,
                    lsb: 0,
                    program: 12,
                },
            )],
        )?,
        track(
            13,
            (0x00e93d, 0x00e9e3),
            11,
            (0x014d3f, 0x014d8f),
            (0x014d52, 0x014d88),
            b"Track 12",
            10,
            vec![],
        )?,
        omitted_empty_track(
            14,
            (0x00e9e3, 0x00ea89),
            12,
            (0x014dd5, 0x014def),
            (0x014de8, 0x014de8),
            b"Track 13",
        ),
        track(
            15,
            (0x00ea89, 0x00eb2f),
            13,
            (0x014e13, 0x015edb),
            (0x014e26, 0x015ed4),
            b"Track 14",
            15,
            vec![],
        )?,
    ];
    Ok(CompatibilityProfile {
        id: ProfileId::new(BELLS_PROFILE_ID),
        version: ProfileVersion::new(1),
        display_label: BELLS_DISPLAY_LABEL.into(),
        project: ProjectExpectation::new(
            BELLS_SOURCE_SHA256,
            BELLS_SOURCE_SIZE,
            ParserProfileId::new("descriptor166"),
            18,
        )?,
        sequences: vec![SequenceExpectation {
            structural_ordinal: 1,
            sequence_range: range(0x00dfff, 0x01603b),
            expected_name_bytes: b"Bells for her".to_vec(),
            name_range: range(0x00eb21, 0x00eb2e),
            descriptor_count: 16,
            pair_count: 14,
            track_expectations: tracks,
        }],
    })
}

/// Constructs the immutable registry of compiled-in research profiles.
pub fn built_in_compatibility_registry() -> Result<CompatibilityRegistry, ProfileDefinitionError> {
    CompatibilityRegistry::new(vec![ode_to_clarke_profile()?, bells_for_her_profile()?])
}
