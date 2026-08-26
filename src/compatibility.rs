//! Generic, Core-only compatibility-profile evidence and matching.
//!
//! This module contains no concrete project profile. It validates declarative
//! policy against owned structural evidence; AppService wiring and export are
//! intentionally separate follow-up steps.

use crate::app_contract::ProfileCapability;
use std::collections::HashSet;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProfileId(String);

impl ProfileId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn capability(
        &self,
        version: ProfileVersion,
        display_label: impl Into<String>,
    ) -> ProfileCapability {
        ProfileCapability {
            profile_id: self.0.clone(),
            profile_version: version.get(),
            display_label: display_label.into(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProfileVersion(u32);

impl ProfileVersion {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParserProfileId(String);

impl ParserProfileId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByteRange {
    start: u64,
    end_exclusive: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidByteRange;

impl ByteRange {
    pub const fn new(start: u64, end_exclusive: u64) -> Result<Self, InvalidByteRange> {
        if start <= end_exclusive {
            Ok(Self {
                start,
                end_exclusive,
            })
        } else {
            Err(InvalidByteRange)
        }
    }

    pub const fn start(self) -> u64 {
        self.start
    }

    pub const fn end_exclusive(self) -> u64 {
        self.end_exclusive
    }

    pub const fn length(self) -> u64 {
        self.end_exclusive - self.start
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EvidenceEventFamily {
    Note,
    Controller,
    ProgramChange,
    ChannelPressure,
    PitchBend,
    Tempo,
    Meter,
    Patch,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PatchEvidence {
    pub source_ordinal: u32,
    pub source_range: ByteRange,
    pub decoded_program: u8,
    pub decoded_bank_msb: Option<u8>,
    pub decoded_bank_lsb: Option<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackEvidence {
    pub descriptor_ordinal: u32,
    pub descriptor_range: ByteRange,
    pub pair_ordinal: u32,
    pub primary_range: ByteRange,
    pub exact_event_range: Option<ByteRange>,
    pub label_bytes: Vec<u8>,
    pub decoded_event_families: Vec<EvidenceEventFamily>,
    pub decoded_event_count: u64,
    pub patch_evidence: Vec<PatchEvidence>,
    /// Optional observed routing evidence. `None` means routing is supplied by
    /// the authenticated profile rather than discovered by the parser.
    pub observed_channel: Option<u8>,
    pub evidence_complete: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SequenceEvidence {
    pub structural_ordinal: u32,
    pub sequence_range: ByteRange,
    pub name_bytes: Vec<u8>,
    pub name_range: ByteRange,
    pub descriptor_count: u32,
    pub pair_count: u32,
    pub tracks: Vec<TrackEvidence>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProfileEvidence {
    pub source_sha256: String,
    pub source_byte_size: u64,
    pub parser_profile: ParserProfileId,
    pub sequences: Vec<SequenceEvidence>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectExpectation {
    pub source_sha256: String,
    pub source_byte_size: u64,
    pub parser_profile: ParserProfileId,
    pub sequence_count: u32,
}

impl ProjectExpectation {
    pub fn new(
        source_sha256: impl Into<String>,
        source_byte_size: u64,
        parser_profile: ParserProfileId,
        sequence_count: u32,
    ) -> Result<Self, ProfileDefinitionError> {
        let source_sha256 = source_sha256.into();
        if source_sha256.len() != 64 || !source_sha256.bytes().all(|byte| byte.is_ascii_hexdigit())
        {
            return Err(ProfileDefinitionError::InvalidSha256);
        }
        Ok(Self {
            source_sha256,
            source_byte_size,
            parser_profile,
            sequence_count,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TrackKey {
    pub descriptor_ordinal: u32,
    pub pair_ordinal: u32,
}

impl TrackKey {
    pub const fn new(descriptor_ordinal: u32, pair_ordinal: u32) -> Self {
        Self {
            descriptor_ordinal,
            pair_ordinal,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackChannelPolicy {
    pub key: TrackKey,
    pub midi_channel: u8,
}

impl TrackChannelPolicy {
    pub fn new(key: TrackKey, midi_channel: u8) -> Result<Self, ProfileDefinitionError> {
        if (1..=16).contains(&midi_channel) {
            Ok(Self { key, midi_channel })
        } else {
            Err(ProfileDefinitionError::InvalidMidiChannel(midi_channel))
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PatchTranslationPolicy {
    ProgramOnly { program: u8 },
    BankSelectMsbAndProgram { msb: u8, program: u8 },
    BankSelectAndProgram { msb: u8, lsb: u8, program: u8 },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PatchExpectation {
    pub source_ordinal: u32,
    pub source_range: ByteRange,
    pub decoded_program: u8,
    pub decoded_bank_msb: Option<u8>,
    pub decoded_bank_lsb: Option<u8>,
    pub translation: PatchTranslationPolicy,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OmittedPatchExpectation {
    pub source_ordinal: u32,
    pub source_range: ByteRange,
    pub decoded_program: u8,
    pub decoded_bank_msb: Option<u8>,
    pub decoded_bank_lsb: Option<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IncludedTrackOutputExpectation {
    pub channel_policy: TrackChannelPolicy,
    pub patch_expectations: Vec<PatchExpectation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuthenticatedTrackOmissionExpectation {
    AuthenticatedNonempty {
        decoded_event_count: u64,
        decoded_event_families: Vec<EvidenceEventFamily>,
        patch_expectations: Vec<OmittedPatchExpectation>,
    },
    StructuralEmpty,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrackOutputDispositionExpectation {
    Included(IncludedTrackOutputExpectation),
    Omitted(AuthenticatedTrackOmissionExpectation),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackExpectation {
    pub key: TrackKey,
    pub descriptor_range: ByteRange,
    pub primary_range: ByteRange,
    pub exact_event_range: Option<ByteRange>,
    pub expected_label_bytes: Option<Vec<u8>>,
    pub output: TrackOutputDispositionExpectation,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SequenceExpectation {
    pub structural_ordinal: u32,
    pub sequence_range: ByteRange,
    pub expected_name_bytes: Vec<u8>,
    pub name_range: ByteRange,
    pub descriptor_count: u32,
    pub pair_count: u32,
    pub track_expectations: Vec<TrackExpectation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompatibilityProfile {
    pub id: ProfileId,
    pub version: ProfileVersion,
    pub display_label: String,
    pub project: ProjectExpectation,
    pub sequences: Vec<SequenceExpectation>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedTrackManifestEntry {
    pub key: TrackKey,
    pub output: ResolvedTrackOutputDisposition,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResolvedTrackOutputDisposition {
    Included {
        midi_channel: u8,
        patches: Vec<PatchTranslationPolicy>,
    },
    OmittedAuthenticatedNonempty {
        decoded_event_count: u64,
        decoded_event_families: Vec<EvidenceEventFamily>,
        patches: Vec<OmittedPatchExpectation>,
    },
    OmittedStructuralEmpty,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedSequenceIdentity {
    pub structural_ordinal: u32,
    pub sequence_range: ByteRange,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedProfilePolicy {
    pub profile_id: ProfileId,
    pub profile_version: ProfileVersion,
    pub sequence: ResolvedSequenceIdentity,
    pub track_manifest: Vec<ResolvedTrackManifestEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProfileMatch {
    NoMatch,
    Matched {
        capability: ProfileCapability,
        resolved_policy: ResolvedProfilePolicy,
    },
    Rejected {
        profile_id: ProfileId,
        reason: ProfileMismatchReason,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProfileMismatchReason {
    HashMismatch,
    SizeMismatch,
    ParserProfileMismatch,
    SequenceIdentityMismatch,
    TrackManifestMismatch,
    ChannelPolicyMismatch,
    PatchPolicyMismatch,
    AmbiguousMatch,
}

impl ProfileMismatchReason {
    pub const fn diagnostic_code(self) -> &'static str {
        match self {
            Self::HashMismatch => "profile_hash_mismatch",
            Self::SizeMismatch => "profile_size_mismatch",
            Self::ParserProfileMismatch => "profile_parser_profile_mismatch",
            Self::SequenceIdentityMismatch => "profile_sequence_identity_mismatch",
            Self::TrackManifestMismatch => "profile_track_manifest_mismatch",
            Self::ChannelPolicyMismatch => "profile_channel_policy_mismatch",
            Self::PatchPolicyMismatch => "profile_patch_policy_mismatch",
            Self::AmbiguousMatch => "profile_ambiguous_match",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProfileDefinitionError {
    InvalidSha256,
    InvalidMidiChannel(u8),
    DuplicateProfile,
    DuplicateSequenceOrdinal(u32),
    DuplicateTrackKey(TrackKey),
    DuplicatePatchOrdinal {
        track: TrackKey,
        source_ordinal: u32,
    },
    PatchPolicyMismatch {
        track: TrackKey,
        source_ordinal: u32,
    },
    InvalidOmissionEvidence(TrackKey),
    IncompleteTrackEvidence(TrackKey),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RegistryMatchError {
    AmbiguousProfiles(Vec<ProfileId>),
}

/// An immutable, deterministically ordered collection of compiled profiles.
#[derive(Clone, Debug, Default)]
pub struct CompatibilityRegistry {
    profiles: Vec<CompatibilityProfile>,
}

impl CompatibilityRegistry {
    pub fn new(mut profiles: Vec<CompatibilityProfile>) -> Result<Self, ProfileDefinitionError> {
        let mut identities = HashSet::new();
        for profile in &profiles {
            if !identities.insert((profile.id.clone(), profile.version)) {
                return Err(ProfileDefinitionError::DuplicateProfile);
            }
            validate_profile(profile)?;
        }
        profiles.sort_by(|left, right| {
            left.id
                .cmp(&right.id)
                .then(left.version.cmp(&right.version))
        });
        Ok(Self { profiles })
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn profiles(&self) -> &[CompatibilityProfile] {
        &self.profiles
    }

    pub fn assess(
        &self,
        evidence: &ProfileEvidence,
        selected_sequence_ordinal: u32,
    ) -> Result<ProfileMatch, RegistryMatchError> {
        let mut matches = Vec::new();
        let mut rejection = None;
        for profile in &self.profiles {
            match assess_profile(profile, evidence, selected_sequence_ordinal) {
                ProfileMatch::Matched { .. } => matches.push(profile),
                ProfileMatch::Rejected { reason, .. } => rejection = Some((profile, reason)),
                ProfileMatch::NoMatch => {}
            }
        }
        if matches.len() > 1 {
            return Err(RegistryMatchError::AmbiguousProfiles(
                matches.iter().map(|profile| profile.id.clone()).collect(),
            ));
        }
        if let Some(profile) = matches.first() {
            return Ok(assess_profile(profile, evidence, selected_sequence_ordinal));
        }
        if let Some((profile, reason)) = rejection {
            return Ok(ProfileMatch::Rejected {
                profile_id: profile.id.clone(),
                reason,
            });
        }
        Ok(ProfileMatch::NoMatch)
    }
}

fn validate_profile(profile: &CompatibilityProfile) -> Result<(), ProfileDefinitionError> {
    let mut sequence_ordinals = HashSet::new();
    for sequence in &profile.sequences {
        if !sequence_ordinals.insert(sequence.structural_ordinal) {
            return Err(ProfileDefinitionError::DuplicateSequenceOrdinal(
                sequence.structural_ordinal,
            ));
        }
        let mut track_keys = HashSet::new();
        for track in &sequence.track_expectations {
            if !track_keys.insert(track.key.clone()) {
                return Err(ProfileDefinitionError::DuplicateTrackKey(track.key.clone()));
            }
            let Some(event_range) = track.exact_event_range else {
                return Err(ProfileDefinitionError::IncompleteTrackEvidence(
                    track.key.clone(),
                ));
            };
            match &track.output {
                TrackOutputDispositionExpectation::Included(included) => {
                    if included.channel_policy.key != track.key {
                        return Err(ProfileDefinitionError::DuplicateTrackKey(track.key.clone()));
                    }
                    let mut patch_ordinals = HashSet::new();
                    for patch in &included.patch_expectations {
                        if !patch_ordinals.insert(patch.source_ordinal) {
                            return Err(ProfileDefinitionError::DuplicatePatchOrdinal {
                                track: track.key.clone(),
                                source_ordinal: patch.source_ordinal,
                            });
                        }
                        if !patch_matches_translation(patch) {
                            return Err(ProfileDefinitionError::PatchPolicyMismatch {
                                track: track.key.clone(),
                                source_ordinal: patch.source_ordinal,
                            });
                        }
                    }
                }
                TrackOutputDispositionExpectation::Omitted(
                    AuthenticatedTrackOmissionExpectation::AuthenticatedNonempty {
                        decoded_event_count,
                        decoded_event_families,
                        patch_expectations,
                    },
                ) => {
                    if *decoded_event_count == 0
                        || event_range.length() == 0
                        || !families_are_canonical_nonempty(decoded_event_families)
                        || *decoded_event_count < decoded_event_families.len() as u64
                        || *decoded_event_count < patch_expectations.len() as u64
                        || decoded_event_families.contains(&EvidenceEventFamily::Patch)
                            != !patch_expectations.is_empty()
                        || patch_expectations
                            .iter()
                            .any(|patch| patch.source_range.length() == 0)
                    {
                        return Err(ProfileDefinitionError::InvalidOmissionEvidence(
                            track.key.clone(),
                        ));
                    }
                    if let Some(source_ordinal) =
                        duplicate_omitted_patch_ordinal(patch_expectations)
                    {
                        return Err(ProfileDefinitionError::DuplicatePatchOrdinal {
                            track: track.key.clone(),
                            source_ordinal,
                        });
                    }
                }
                TrackOutputDispositionExpectation::Omitted(
                    AuthenticatedTrackOmissionExpectation::StructuralEmpty,
                ) => {
                    if event_range.length() != 0 {
                        return Err(ProfileDefinitionError::InvalidOmissionEvidence(
                            track.key.clone(),
                        ));
                    }
                }
            }
        }
    }
    Ok(())
}

fn families_are_canonical_nonempty(families: &[EvidenceEventFamily]) -> bool {
    !families.is_empty()
        && families
            .iter()
            .map(|family| canonical_family_index(*family))
            .collect::<Option<Vec<_>>>()
            .is_some_and(|indices| indices.windows(2).all(|pair| pair[0] < pair[1]))
}

fn canonical_family_index(family: EvidenceEventFamily) -> Option<u8> {
    match family {
        EvidenceEventFamily::Patch => Some(0),
        EvidenceEventFamily::Note => Some(1),
        EvidenceEventFamily::Controller => Some(2),
        EvidenceEventFamily::ChannelPressure => Some(3),
        EvidenceEventFamily::PitchBend => Some(4),
        EvidenceEventFamily::ProgramChange
        | EvidenceEventFamily::Tempo
        | EvidenceEventFamily::Meter => None,
    }
}

fn duplicate_omitted_patch_ordinal(patches: &[OmittedPatchExpectation]) -> Option<u32> {
    let mut ordinals = HashSet::new();
    patches
        .iter()
        .find_map(|patch| (!ordinals.insert(patch.source_ordinal)).then_some(patch.source_ordinal))
}

fn patch_matches_translation(patch: &PatchExpectation) -> bool {
    match patch.translation {
        PatchTranslationPolicy::ProgramOnly { program } => {
            patch.decoded_program == program
                && patch.decoded_bank_msb.is_none()
                && patch.decoded_bank_lsb.is_none()
        }
        PatchTranslationPolicy::BankSelectMsbAndProgram { msb, program } => {
            patch.decoded_program == program
                && patch.decoded_bank_msb.map_or(true, |value| value == msb)
                && patch.decoded_bank_lsb.is_none()
        }
        PatchTranslationPolicy::BankSelectAndProgram { msb, lsb, program } => {
            patch.decoded_program == program
                && patch.decoded_bank_msb.map_or(true, |value| value == msb)
                && patch.decoded_bank_lsb.map_or(true, |value| value == lsb)
        }
    }
}

fn assess_profile(
    profile: &CompatibilityProfile,
    evidence: &ProfileEvidence,
    selected_sequence_ordinal: u32,
) -> ProfileMatch {
    if evidence.source_sha256 != profile.project.source_sha256 {
        return ProfileMatch::NoMatch;
    }
    if evidence.source_byte_size != profile.project.source_byte_size {
        return ProfileMatch::NoMatch;
    }
    if evidence.parser_profile != profile.project.parser_profile {
        return ProfileMatch::NoMatch;
    }
    if evidence.sequences.len() != profile.project.sequence_count as usize {
        return rejected(profile, ProfileMismatchReason::SequenceIdentityMismatch);
    }
    let Some(expected) = profile
        .sequences
        .iter()
        .find(|sequence| sequence.structural_ordinal == selected_sequence_ordinal)
    else {
        return rejected(profile, ProfileMismatchReason::SequenceIdentityMismatch);
    };
    let Some(observed) = evidence
        .sequences
        .iter()
        .find(|sequence| sequence.structural_ordinal == selected_sequence_ordinal)
    else {
        return rejected(profile, ProfileMismatchReason::SequenceIdentityMismatch);
    };
    if expected.sequence_range != observed.sequence_range
        || expected.expected_name_bytes != observed.name_bytes
        || expected.name_range != observed.name_range
        || expected.descriptor_count != observed.descriptor_count
        || expected.pair_count != observed.pair_count
    {
        return rejected(profile, ProfileMismatchReason::SequenceIdentityMismatch);
    }
    if expected.track_expectations.len() != observed.tracks.len() {
        return rejected(profile, ProfileMismatchReason::TrackManifestMismatch);
    }
    let mut observed_keys = HashSet::new();
    for track in &observed.tracks {
        let key = TrackKey::new(track.descriptor_ordinal, track.pair_ordinal);
        if !observed_keys.insert(key) {
            return rejected(profile, ProfileMismatchReason::TrackManifestMismatch);
        }
    }
    let mut resolved_manifest = Vec::with_capacity(expected.track_expectations.len());
    for observed_track in &observed.tracks {
        let Some(expectation) = expected.track_expectations.iter().find(|expectation| {
            observed_track.descriptor_ordinal == expectation.key.descriptor_ordinal
                && observed_track.pair_ordinal == expectation.key.pair_ordinal
        }) else {
            return rejected(profile, ProfileMismatchReason::TrackManifestMismatch);
        };
        if expectation.descriptor_range != observed_track.descriptor_range
            || expectation.primary_range != observed_track.primary_range
            || expectation.exact_event_range != observed_track.exact_event_range
            || expectation
                .expected_label_bytes
                .as_ref()
                .is_some_and(|label| label != &observed_track.label_bytes)
        {
            return rejected(profile, ProfileMismatchReason::TrackManifestMismatch);
        }
        if observed_track.exact_event_range.is_none() {
            return rejected(profile, ProfileMismatchReason::TrackManifestMismatch);
        }
        let output = match &expectation.output {
            TrackOutputDispositionExpectation::Included(included) => {
                if observed_track
                    .observed_channel
                    .is_some_and(|channel| channel != included.channel_policy.midi_channel)
                {
                    return rejected(profile, ProfileMismatchReason::ChannelPolicyMismatch);
                }
                if included.patch_expectations.len() != observed_track.patch_evidence.len() {
                    return rejected(profile, ProfileMismatchReason::PatchPolicyMismatch);
                }
                let mut resolved_patches = Vec::with_capacity(included.patch_expectations.len());
                for patch_expectation in &included.patch_expectations {
                    let Some(observed_patch) = observed_track
                        .patch_evidence
                        .iter()
                        .find(|patch| patch.source_ordinal == patch_expectation.source_ordinal)
                    else {
                        return rejected(profile, ProfileMismatchReason::PatchPolicyMismatch);
                    };
                    if !patch_evidence_matches_expectation(observed_patch, patch_expectation) {
                        return rejected(profile, ProfileMismatchReason::PatchPolicyMismatch);
                    }
                    resolved_patches.push(patch_expectation.translation.clone());
                }
                ResolvedTrackOutputDisposition::Included {
                    midi_channel: included.channel_policy.midi_channel,
                    patches: resolved_patches,
                }
            }
            TrackOutputDispositionExpectation::Omitted(
                AuthenticatedTrackOmissionExpectation::AuthenticatedNonempty {
                    decoded_event_count,
                    decoded_event_families,
                    patch_expectations,
                },
            ) => {
                if observed_track.decoded_event_count != *decoded_event_count
                    || observed_track.decoded_event_families != *decoded_event_families
                    || patch_expectations.len() != observed_track.patch_evidence.len()
                    || !patch_expectations.iter().all(|expected_patch| {
                        observed_track.patch_evidence.iter().any(|observed_patch| {
                            omitted_patch_evidence_matches(observed_patch, expected_patch)
                        })
                    })
                {
                    return rejected(profile, ProfileMismatchReason::TrackManifestMismatch);
                }
                let mut patches = patch_expectations.clone();
                patches.sort_by_key(|patch| patch.source_ordinal);
                ResolvedTrackOutputDisposition::OmittedAuthenticatedNonempty {
                    decoded_event_count: *decoded_event_count,
                    decoded_event_families: decoded_event_families.clone(),
                    patches,
                }
            }
            TrackOutputDispositionExpectation::Omitted(
                AuthenticatedTrackOmissionExpectation::StructuralEmpty,
            ) => {
                if expectation
                    .exact_event_range
                    .map_or(true, |range| range.length() != 0)
                    || observed_track.decoded_event_count != 0
                    || !observed_track.decoded_event_families.is_empty()
                    || !observed_track.patch_evidence.is_empty()
                {
                    return rejected(profile, ProfileMismatchReason::TrackManifestMismatch);
                }
                ResolvedTrackOutputDisposition::OmittedStructuralEmpty
            }
        };
        resolved_manifest.push(ResolvedTrackManifestEntry {
            key: expectation.key.clone(),
            output,
        });
    }
    ProfileMatch::Matched {
        capability: profile
            .id
            .capability(profile.version, profile.display_label.clone()),
        resolved_policy: ResolvedProfilePolicy {
            profile_id: profile.id.clone(),
            profile_version: profile.version,
            sequence: ResolvedSequenceIdentity {
                structural_ordinal: observed.structural_ordinal,
                sequence_range: observed.sequence_range,
            },
            track_manifest: resolved_manifest,
        },
    }
}

fn patch_evidence_matches_expectation(
    observed: &PatchEvidence,
    expected: &PatchExpectation,
) -> bool {
    observed.source_range == expected.source_range
        && observed.decoded_program == expected.decoded_program
        && observed.decoded_bank_msb == expected.decoded_bank_msb
        && observed.decoded_bank_lsb == expected.decoded_bank_lsb
}

fn omitted_patch_evidence_matches(
    observed: &PatchEvidence,
    expected: &OmittedPatchExpectation,
) -> bool {
    observed.source_ordinal == expected.source_ordinal
        && observed.source_range == expected.source_range
        && observed.decoded_program == expected.decoded_program
        && observed.decoded_bank_msb == expected.decoded_bank_msb
        && observed.decoded_bank_lsb == expected.decoded_bank_lsb
}

fn rejected(profile: &CompatibilityProfile, reason: ProfileMismatchReason) -> ProfileMatch {
    ProfileMatch::Rejected {
        profile_id: profile.id.clone(),
        reason,
    }
}

impl fmt::Display for InvalidByteRange {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("byte range start exceeds end")
    }
}

impl std::error::Error for InvalidByteRange {}
