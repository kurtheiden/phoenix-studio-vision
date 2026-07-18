use std::fmt;
use std::path::Path;

const STUDIO_VISION_TYPE: FourCc = FourCc(*b"MID2");
const STUDIO_VISION_CREATOR: FourCc = FourCc(*b"MIDA");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct FourCc([u8; 4]);

impl fmt::Display for FourCc {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("'")?;
        for byte in self.0 {
            if byte.is_ascii_graphic() && byte != b'\'' && byte != b'\\' {
                formatter.write_str(&char::from(byte).to_string())?;
            } else {
                write!(formatter, "\\x{byte:02x}")?;
            }
        }
        formatter.write_str("'")
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum CodeObservation {
    Observed(FourCc),
    Absent,
    #[allow(dead_code)] // Constructed by the non-macOS adapter and injected tests.
    Unsupported,
    ReadError(String),
    Malformed {
        finder_info_length: usize,
    },
}

impl fmt::Display for CodeObservation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Observed(code) => write!(formatter, "observed {code}"),
            Self::Absent => formatter.write_str("absent (com.apple.FinderInfo not present)"),
            Self::Unsupported => formatter.write_str("unsupported on this platform"),
            Self::ReadError(error) => write!(formatter, "read error: {error}"),
            Self::Malformed { finder_info_length } => write!(
                formatter,
                "malformed com.apple.FinderInfo ({finder_info_length} bytes; at least 8 required)"
            ),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct FinderMetadata {
    pub(crate) file_type: CodeObservation,
    pub(crate) creator: CodeObservation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)] // The complete public-facing scale is reserved for future independent evidence.
pub(crate) enum Confidence {
    VeryHigh,
    High,
    Medium,
    Low,
    Unknown,
}

impl fmt::Display for Confidence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::VeryHigh => "Very High",
            Self::High => "High",
            Self::Medium => "Medium",
            Self::Low => "Low",
            Self::Unknown => "Unknown",
        })
    }
}

pub(crate) struct Identification {
    pub(crate) metadata: FinderMetadata,
    pub(crate) evidence: Vec<String>,
    pub(crate) confidence: Confidence,
}

pub(crate) fn identify(metadata: FinderMetadata) -> Identification {
    use CodeObservation::{Absent, Observed, Unsupported};

    let type_matches = metadata.file_type == Observed(STUDIO_VISION_TYPE);
    let creator_matches = metadata.creator == Observed(STUDIO_VISION_CREATOR);
    let type_unavailable = matches!(metadata.file_type, Absent | Unsupported);
    let creator_unavailable = matches!(metadata.creator, Absent | Unsupported);
    let type_conflicts = matches!(metadata.file_type, Observed(code) if code != STUDIO_VISION_TYPE);
    let creator_conflicts =
        matches!(metadata.creator, Observed(code) if code != STUDIO_VISION_CREATOR);

    let (confidence, evidence) = if type_matches && creator_matches {
        (
            Confidence::High,
            vec!["The observed Finder type 'MID2' and creator 'MIDA' pair is strong observed evidence associated with Studio Vision; it is not a content signature or structural validation.".into()],
        )
    } else if type_matches && creator_unavailable {
        (
            Confidence::Low,
            vec!["Finder type 'MID2' matches observed Studio Vision metadata, but the creator code is unavailable; this incomplete signal is provisional.".into()],
        )
    } else if creator_matches && type_unavailable {
        (
            Confidence::Low,
            vec!["Finder creator 'MIDA' matches observed Studio Vision metadata, but the type code is unavailable; this incomplete signal is provisional.".into()],
        )
    } else if (type_matches && creator_conflicts) || (creator_matches && type_conflicts) {
        (
            Confidence::Unknown,
            vec!["One Finder code matches observed Studio Vision metadata, but the other observed code conflicts; conflicting metadata is not identification evidence.".into()],
        )
    } else {
        (Confidence::Unknown, vec![evidence_for_unknown(&metadata)])
    };

    Identification {
        metadata,
        evidence,
        confidence,
    }
}

fn evidence_for_unknown(metadata: &FinderMetadata) -> String {
    if matches!(metadata.file_type, CodeObservation::ReadError(_))
        || matches!(metadata.creator, CodeObservation::ReadError(_))
    {
        "Finder metadata could not be read, so no identification conclusion can be supported."
            .into()
    } else if matches!(metadata.file_type, CodeObservation::Malformed { .. })
        || matches!(metadata.creator, CodeObservation::Malformed { .. })
    {
        "Finder metadata is malformed, so its type and creator codes cannot be used as evidence."
            .into()
    } else if matches!(metadata.file_type, CodeObservation::Unsupported)
        && matches!(metadata.creator, CodeObservation::Unsupported)
    {
        "Finder metadata acquisition is unsupported on this platform; absence of evidence is not evidence against Studio Vision."
            .into()
    } else if matches!(metadata.file_type, CodeObservation::Absent)
        && matches!(metadata.creator, CodeObservation::Absent)
    {
        "No Finder metadata was found; its absence is not evidence against Studio Vision.".into()
    } else {
        "The observed Finder metadata does not provide supported Studio Vision identification evidence."
            .into()
    }
}

#[cfg(any(test, target_os = "macos"))]
fn decode_finder_info(bytes: &[u8]) -> FinderMetadata {
    if bytes.len() < 8 {
        let malformed = CodeObservation::Malformed {
            finder_info_length: bytes.len(),
        };
        return FinderMetadata {
            file_type: malformed.clone(),
            creator: malformed,
        };
    }
    FinderMetadata {
        file_type: CodeObservation::Observed(FourCc(bytes[0..4].try_into().unwrap())),
        creator: CodeObservation::Observed(FourCc(bytes[4..8].try_into().unwrap())),
    }
}

#[cfg(not(target_os = "macos"))]
pub(crate) fn read_finder_metadata(_path: &Path) -> FinderMetadata {
    FinderMetadata {
        file_type: CodeObservation::Unsupported,
        creator: CodeObservation::Unsupported,
    }
}

#[cfg(target_os = "macos")]
pub(crate) fn read_finder_metadata(path: &Path) -> FinderMetadata {
    match macos::read_finder_info(path) {
        Ok(Some(bytes)) => decode_finder_info(&bytes),
        Ok(None) => FinderMetadata {
            file_type: CodeObservation::Absent,
            creator: CodeObservation::Absent,
        },
        Err(error) => {
            let observation = CodeObservation::ReadError(error);
            FinderMetadata {
                file_type: observation.clone(),
                creator: observation,
            }
        }
    }
}

#[cfg(target_os = "macos")]
mod macos {
    use std::ffi::{c_char, c_int, c_void, CString};
    use std::io;
    use std::os::unix::ffi::OsStrExt;
    use std::path::Path;
    use std::ptr;

    const FINDER_INFO: &[u8] = b"com.apple.FinderInfo\0";
    const ENOATTR: i32 = 93;

    extern "C" {
        fn getxattr(
            path: *const c_char,
            name: *const c_char,
            value: *mut c_void,
            size: usize,
            position: u32,
            options: c_int,
        ) -> isize;
    }

    pub(super) fn read_finder_info(path: &Path) -> Result<Option<Vec<u8>>, String> {
        let path = CString::new(path.as_os_str().as_bytes())
            .map_err(|_| "path contains an interior NUL byte".to_owned())?;
        let name = FINDER_INFO.as_ptr().cast::<c_char>();

        // SAFETY: Both C strings are NUL-terminated, the value pointer is null for this
        // size query, and getxattr does not mutate the selected file.
        let size = unsafe { getxattr(path.as_ptr(), name, ptr::null_mut(), 0, 0, 0) };
        if size < 0 {
            let error = io::Error::last_os_error();
            return if error.raw_os_error() == Some(ENOATTR) {
                Ok(None)
            } else {
                Err(error.to_string())
            };
        }

        let mut bytes = vec![0_u8; size as usize];
        // SAFETY: The buffer is valid for `bytes.len()` writable bytes and both C
        // strings remain alive for the duration of this read-only metadata call.
        let read = unsafe {
            getxattr(
                path.as_ptr(),
                name,
                bytes.as_mut_ptr().cast::<c_void>(),
                bytes.len(),
                0,
                0,
            )
        };
        if read < 0 {
            return Err(io::Error::last_os_error().to_string());
        }
        bytes.truncate(read as usize);
        Ok(Some(bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn observed(file_type: &[u8; 4], creator: &[u8; 4]) -> FinderMetadata {
        FinderMetadata {
            file_type: CodeObservation::Observed(FourCc(*file_type)),
            creator: CodeObservation::Observed(FourCc(*creator)),
        }
    }

    #[test]
    fn matching_pair_has_high_confidence() {
        assert_eq!(
            identify(observed(b"MID2", b"MIDA")).confidence,
            Confidence::High
        );
    }

    #[test]
    fn type_only_has_low_confidence() {
        let metadata = FinderMetadata {
            file_type: CodeObservation::Observed(FourCc(*b"MID2")),
            creator: CodeObservation::Absent,
        };
        assert_eq!(identify(metadata).confidence, Confidence::Low);
    }

    #[test]
    fn creator_only_has_low_confidence() {
        let metadata = FinderMetadata {
            file_type: CodeObservation::Unsupported,
            creator: CodeObservation::Observed(FourCc(*b"MIDA")),
        };
        assert_eq!(identify(metadata).confidence, Confidence::Low);
    }

    #[test]
    fn matching_type_and_conflicting_creator_are_unknown() {
        assert_eq!(
            identify(observed(b"MID2", b"NOPE")).confidence,
            Confidence::Unknown
        );
    }

    #[test]
    fn matching_creator_and_conflicting_type_are_unknown() {
        assert_eq!(
            identify(observed(b"NOPE", b"MIDA")).confidence,
            Confidence::Unknown
        );
    }

    #[test]
    fn absent_and_unsupported_metadata_are_unknown() {
        for observation in [CodeObservation::Absent, CodeObservation::Unsupported] {
            let metadata = FinderMetadata {
                file_type: observation.clone(),
                creator: observation,
            };
            assert_eq!(identify(metadata).confidence, Confidence::Unknown);
        }
    }

    #[test]
    fn read_errors_are_explicit_and_unknown() {
        let metadata = FinderMetadata {
            file_type: CodeObservation::ReadError("denied".into()),
            creator: CodeObservation::ReadError("denied".into()),
        };
        let result = identify(metadata);
        assert_eq!(result.confidence, Confidence::Unknown);
        assert!(result
            .metadata
            .file_type
            .to_string()
            .contains("read error: denied"));
    }

    #[test]
    fn a_match_plus_read_error_is_explicit_and_unknown() {
        let metadata = FinderMetadata {
            file_type: CodeObservation::Observed(FourCc(*b"MID2")),
            creator: CodeObservation::ReadError("denied".into()),
        };
        let result = identify(metadata);
        assert_eq!(result.confidence, Confidence::Unknown);
        assert!(result.evidence[0].contains("could not be read"));
    }

    #[test]
    fn short_finder_info_is_malformed_and_unknown() {
        let result = identify(decode_finder_info(b"MID2MID"));
        assert_eq!(result.confidence, Confidence::Unknown);
        assert!(result.metadata.file_type.to_string().contains("malformed"));
    }

    #[test]
    fn finder_info_decodes_first_eight_bytes() {
        assert_eq!(
            decode_finder_info(b"MID2MIDAextra"),
            observed(b"MID2", b"MIDA")
        );
    }

    #[test]
    fn arbitrary_codes_render_without_control_characters() {
        assert_eq!(
            FourCc([0, b'A', b'\\', b'\'']).to_string(),
            "'\\x00A\\x5c\\x27'"
        );
    }

    #[test]
    fn classifier_has_no_filename_input() {
        let result = identify(observed(b"MID2", b"MIDA"));
        assert_eq!(result.confidence, Confidence::High);
    }

    #[test]
    fn unused_confidence_levels_remain_displayable() {
        assert_eq!(Confidence::VeryHigh.to_string(), "Very High");
        assert_eq!(Confidence::Medium.to_string(), "Medium");
    }
}
