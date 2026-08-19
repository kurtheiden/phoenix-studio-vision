//! Compiled-in, provenance-locked compatibility profiles.
//!
//! Target-specific policy belongs here, outside generic parser, evidence, and
//! application-service modules. The built-in profile is an authenticated
//! research policy, not general Studio Vision support.

use crate::compatibility::{
    ByteRange, CompatibilityProfile, CompatibilityRegistry, ParserProfileId, PatchExpectation,
    PatchTranslationPolicy, ProfileDefinitionError, ProfileId, ProfileVersion, ProjectExpectation,
    SequenceExpectation, TrackChannelPolicy, TrackExpectation, TrackKey,
};

const ODE_PROFILE_ID: &str = "studio_vision_ode_to_clarke_v1";
const ODE_DISPLAY_LABEL: &str = "Validated research profile — Ode to Clarke";
const ODE_SOURCE_SHA256: &str = "e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132";
const ODE_SOURCE_SIZE: u64 = 211_468;

fn range(start: u64, end: u64) -> ByteRange {
    ByteRange::new(start, end).expect("built-in profile range is valid")
}

fn patch(
    source_range: (u64, u64),
    program: u8,
    translation: PatchTranslationPolicy,
) -> PatchExpectation {
    PatchExpectation {
        source_ordinal: 0,
        source_range: range(source_range.0, source_range.1),
        decoded_program: program,
        decoded_bank_msb: None,
        decoded_bank_lsb: None,
        translation,
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
        channel_policy: TrackChannelPolicy::new(key, channel)?,
        patch_expectations: patches,
    })
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

/// Constructs the immutable registry of compiled-in research profiles.
pub fn built_in_compatibility_registry() -> Result<CompatibilityRegistry, ProfileDefinitionError> {
    CompatibilityRegistry::new(vec![ode_to_clarke_profile()?])
}
