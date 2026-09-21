//! Private, opt-in, read-only research observations. No recovery profile is created.

use crate::app_contract::{
    ResearchField, ResearchGroup, ResearchNote, ResearchObservation, ResearchTrack,
};
use crate::sequence_container::{parse_root_record_stream, FramedRecord};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::fs;
use std::ops::Range;
use std::path::Path;

const AUTH_ENV: &str = "PHOENIX_PROLOGUE_RESEARCH_AUTH_FILE";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Bounds {
    start: usize,
    end: usize,
}

impl Bounds {
    fn range(&self) -> Range<usize> {
        self.start..self.end
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GroupSpec {
    record: Bounds,
    name: String,
    name_bytes: Bounds,
    descriptor_count: usize,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TrackSpec {
    label: String,
    descriptor_label: Bounds,
    second_descriptor_label: Bounds,
    primary: Option<Bounds>,
    secondary: Option<Bounds>,
    count: Option<usize>,
    tuple_sha256: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FieldSpec {
    bytes: Bounds,
    value: u32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Authorization {
    schema_version: u32,
    source_sha256: String,
    source_size: usize,
    groups: [GroupSpec; 2],
    tracks: [TrackSpec; 3],
    correlated_fields: Vec<FieldSpec>,
}

/// Absence, invalid authorization, or any structural mismatch returns no observation.
pub(crate) fn inspect_if_authorized(bytes: &[u8], sha256: &str) -> Option<ResearchObservation> {
    let path = std::env::var_os(AUTH_ENV)?;
    let path = Path::new(&path);
    if !path.is_absolute() {
        return None;
    }
    let metadata = fs::symlink_metadata(path).ok()?;
    if !metadata.file_type().is_file() || metadata.len() > 65_536 {
        return None;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::{MetadataExt, PermissionsExt};
        if metadata.uid() != unsafe { libc_geteuid() } || metadata.permissions().mode() & 0o077 != 0
        {
            return None;
        }
    }
    let auth: Authorization = serde_json::from_slice(&fs::read(path).ok()?).ok()?;
    extract(bytes, sha256, &auth)
}

#[cfg(unix)]
unsafe extern "C" {
    #[link_name = "geteuid"]
    fn libc_geteuid() -> u32;
}

fn record<'a>(
    records: &'a [FramedRecord<'a>],
    bounds: &Bounds,
    kind: u8,
) -> Option<&'a FramedRecord<'a>> {
    records
        .iter()
        .find(|r| r.record_range == bounds.range() && r.record_type.value == kind)
}

fn checked_bytes<'a>(bytes: &'a [u8], bounds: &Bounds) -> Option<&'a [u8]> {
    if bounds.start >= bounds.end {
        return None;
    }
    bytes.get(bounds.range())
}

fn extract(bytes: &[u8], sha256: &str, auth: &Authorization) -> Option<ResearchObservation> {
    if auth.schema_version != 1
        || auth.source_size != bytes.len()
        || auth.source_sha256.len() != 64
        || !auth
            .source_sha256
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
        || auth.source_sha256 != sha256
    {
        return None;
    }
    let root = parse_root_record_stream(bytes).ok()?;
    let mut groups = Vec::new();
    for group in &auth.groups {
        let object = record(&root.records, &group.record, 1)?;
        let count = group.descriptor_count;
        if count < 3
            || object.record_range.len() != 75_usize.checked_add(120_usize.checked_mul(count)?)?
            || usize::from(*bytes.get(object.record_range.start + 5)?) != count
        {
            return None;
        }
        if group.name_bytes.start < object.record_range.start
            || group.name_bytes.end > object.record_range.end
            || checked_bytes(bytes, &group.name_bytes)? != group.name.as_bytes()
        {
            return None;
        }
        groups.push(ResearchGroup {
            name: group.name.clone(),
            record_start: group.record.start as u64,
            record_end: group.record.end as u64,
        });
    }
    if groups[0].name != groups[1].name || groups[0].record_end >= groups[1].record_start {
        return None;
    }
    let mut tracks = Vec::new();
    for (index, track) in auth.tracks.iter().enumerate() {
        if track.label.is_empty()
            || track.descriptor_label.start < auth.groups[0].record.start
            || track.descriptor_label.end > auth.groups[0].record.end
            || checked_bytes(bytes, &track.descriptor_label)? != track.label.as_bytes()
            || track.second_descriptor_label.start < auth.groups[1].record.start
            || track.second_descriptor_label.end > auth.groups[1].record.end
            || checked_bytes(bytes, &track.second_descriptor_label)? != track.label.as_bytes()
        {
            return None;
        }
        let (primary_start, primary_end, tuple_start, tuple_end, notes, limitation) = match (
            &track.primary,
            &track.secondary,
            track.count,
        ) {
            (Some(primary), Some(secondary), Some(count)) if index < 2 && count > 0 => {
                let p = record(&root.records, primary, 2)?;
                let s = record(&root.records, secondary, 0x29)?;
                if p.record_range.end != s.record_range.start
                    || primary.start < auth.groups[0].record.end
                    || secondary.end > auth.groups[1].record.start
                {
                    return None;
                }
                let payload = p.payload.bytes;
                let tuple_len = count.checked_mul(5)?;
                let expected_len = 16_usize.checked_add(tuple_len)?.checked_add(6)?;
                if payload.len() != expected_len
                    || payload.get(5).copied()? as usize != count
                    || payload[15] != 0x90
                    || payload[16 + tuple_len] != 0xff
                    || payload[19 + tuple_len..22 + tuple_len] != [0xff, 0x2f, 0x00]
                {
                    return None;
                }
                let start = p.payload.range.start + 16;
                let tuple_bytes = &payload[16..16 + tuple_len];
                if track.tuple_sha256.as_deref() != Some(hash_bytes(tuple_bytes).as_str()) {
                    return None;
                }
                let mut onset = u32::from(payload[14]);
                let mut notes = Vec::with_capacity(count);
                for ordinal in 0..count {
                    let offset = 16 + ordinal * 5;
                    let chunk = &payload[offset..offset + 5];
                    if chunk[..4].iter().any(|b| *b > 127)
                        || (ordinal + 1 < count && chunk[4] > 127)
                        || (ordinal + 1 == count && chunk[4] != 0xff)
                    {
                        return None;
                    }
                    notes.push(ResearchNote {
                        source_start: (start + ordinal * 5) as u64,
                        onset_tick: onset,
                        pitch: chunk[0],
                        attack_velocity: chunk[1],
                        release_velocity: chunk[2],
                        duration_ticks: chunk[3],
                    });
                    if ordinal + 1 < count {
                        onset = onset.checked_add(u32::from(chunk[4]))?;
                    }
                }
                (
                    Some(primary.start as u64),
                    Some(primary.end as u64),
                    Some(start as u64),
                    Some((start + tuple_len) as u64),
                    notes,
                    None,
                )
            }
            (None, None, None) if index == 2 && track.tuple_sha256.is_none() => (
                None,
                None,
                None,
                None,
                Vec::new(),
                Some(
                    "Descriptor present; no local note pair established in this representation."
                        .into(),
                ),
            ),
            _ => return None,
        };
        tracks.push(ResearchTrack {
            label: track.label.clone(),
            descriptor_label_start: track.descriptor_label.start as u64,
            descriptor_label_end: track.descriptor_label.end as u64,
            primary_start,
            primary_end,
            tuple_start,
            tuple_end,
            notes,
            limitation,
        });
    }
    let mut correlated_fields = Vec::new();
    if auth.correlated_fields.len() != 4 {
        return None;
    }
    for field in &auth.correlated_fields {
        let raw = checked_bytes(bytes, &field.bytes)?;
        if !matches!(raw.len(), 2 | 4) {
            return None;
        }
        let value = raw.iter().fold(0_u32, |v, b| (v << 8) | u32::from(*b));
        if value != field.value || !matches!(value, 10 | 1920) {
            return None;
        }
        correlated_fields.push(ResearchField {
            start: field.bytes.start as u64,
            end: field.bytes.end as u64,
            value,
            interpretation: if value == 1920 {
                "correlates with observed track period; control semantics unresolved".into()
            } else {
                "correlates with observed ten-bar length; control semantics unresolved".into()
            },
        });
    }
    if correlated_fields.iter().filter(|f| f.value == 10).count() != 2
        || correlated_fields.iter().filter(|f| f.value == 1920).count() != 2
    {
        return None;
    }
    Some(ResearchObservation {
        label: "Authenticated private research observation; not a recovered Sequence".into(),
        representation: "bounded 120-byte source representation".into(),
        groups,
        tracks,
        correlated_fields,
        unresolved: vec![
            "Sequence–Segment ownership and references".into(),
            "selection, repetition, and endpoint rules".into(),
            "serialized mute state".into(),
            "third musical track's original-file note storage".into(),
            "MIDI export completeness".into(),
        ],
    })
}

fn hash_bytes(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};

    fn push(bytes: &mut Vec<u8>, kind: u8, payload: &[u8]) -> Bounds {
        let start = bytes.len();
        bytes.push(kind);
        bytes.extend_from_slice(&(payload.len() as u32).to_be_bytes());
        bytes.extend_from_slice(payload);
        Bounds {
            start,
            end: bytes.len(),
        }
    }

    fn fixture() -> (Vec<u8>, Authorization) {
        let mut bytes = vec![0; 8];
        let mut object = vec![0; 430];
        object[0] = 3;
        object[20..24].copy_from_slice(b"Demo");
        object[40] = b'A';
        object[70] = b'B';
        object[100] = b'C';
        object[150..152].copy_from_slice(&10_u16.to_be_bytes());
        object[152..156].copy_from_slice(&1920_u32.to_be_bytes());
        object[156..158].copy_from_slice(&10_u16.to_be_bytes());
        object[158..162].copy_from_slice(&1920_u32.to_be_bytes());
        let g1 = push(&mut bytes, 1, &object);
        let mut tracks = Vec::new();
        for (index, label) in ["A", "B"].into_iter().enumerate() {
            let mut payload = vec![0; 32];
            payload[5] = 2;
            payload[15] = 0x90;
            payload[16..26].copy_from_slice(&[60, 100, 64, 5, 10, 61, 100, 64, 6, 255]);
            payload[26..32].copy_from_slice(&[255, 1, 2, 255, 47, 0]);
            let primary = push(&mut bytes, 2, &payload);
            let secondary = push(&mut bytes, 0x29, &[0]);
            tracks.push(TrackSpec {
                label: label.into(),
                descriptor_label: Bounds {
                    start: g1.start + 5 + 40 + index * 30,
                    end: g1.start + 5 + 41 + index * 30,
                },
                second_descriptor_label: Bounds { start: 0, end: 1 },
                primary: Some(primary),
                secondary: Some(secondary),
                count: Some(2),
                tuple_sha256: Some(hash_bytes(&payload[16..26])),
            });
        }
        let g2 = push(&mut bytes, 1, &object);
        tracks.push(TrackSpec {
            label: "C".into(),
            descriptor_label: Bounds {
                start: g1.start + 5 + 100,
                end: g1.start + 5 + 101,
            },
            second_descriptor_label: Bounds { start: 0, end: 1 },
            primary: None,
            secondary: None,
            count: None,
            tuple_sha256: None,
        });
        let fields = [
            (150, 152, 10),
            (152, 156, 1920),
            (156, 158, 10),
            (158, 162, 1920),
        ]
        .into_iter()
        .map(|(start, end, value)| FieldSpec {
            bytes: Bounds {
                start: g1.start + 5 + start,
                end: g1.start + 5 + end,
            },
            value,
        })
        .collect();
        for (index, track) in tracks.iter_mut().enumerate() {
            let offset = [40, 70, 100][index];
            track.second_descriptor_label = Bounds {
                start: g2.start + 5 + offset,
                end: g2.start + 5 + offset + 1,
            };
        }
        let auth = Authorization {
            schema_version: 1,
            source_sha256: hash(&bytes),
            source_size: bytes.len(),
            groups: [
                GroupSpec {
                    record: g1,
                    name: "Demo".into(),
                    name_bytes: Bounds {
                        start: 8 + 5 + 20,
                        end: 8 + 5 + 24,
                    },
                    descriptor_count: 3,
                },
                GroupSpec {
                    name_bytes: Bounds {
                        start: g2.start + 5 + 20,
                        end: g2.start + 5 + 24,
                    },
                    record: g2,
                    name: "Demo".into(),
                    descriptor_count: 3,
                },
            ],
            tracks: tracks.try_into().ok().unwrap(),
            correlated_fields: fields,
        };
        (bytes, auth)
    }

    fn hash(bytes: &[u8]) -> String {
        format!("{:x}", Sha256::digest(bytes))
    }

    #[test]
    fn bounded_observation_and_fail_closed_mutations() {
        let (bytes, mut auth) = fixture();
        let result = extract(&bytes, &hash(&bytes), &auth).unwrap();
        assert_eq!(result.tracks[0].notes.len(), 2);
        assert_eq!(result.tracks[0].notes[1].onset_tick, 10);
        assert!(result.tracks[2].notes.is_empty());
        auth.source_sha256 = "0".repeat(64);
        assert!(extract(&bytes, &hash(&bytes), &auth).is_none());
        auth.source_sha256 = hash(&bytes);
        for offset in [
            auth.tracks[0].descriptor_label.start,
            auth.tracks[0].primary.as_ref().unwrap().start,
            auth.tracks[0].primary.as_ref().unwrap().start + 5 + 5,
            auth.tracks[0].primary.as_ref().unwrap().start + 5 + 16 + 4,
            auth.tracks[0].primary.as_ref().unwrap().end - 1,
        ] {
            let mut changed = bytes.clone();
            changed[offset] ^= 1;
            auth.source_sha256 = hash(&changed);
            assert!(
                extract(&changed, &auth.source_sha256, &auth).is_none(),
                "offset {offset}"
            );
        }
        let mut truncated = bytes.clone();
        truncated.pop();
        auth.source_size = truncated.len();
        auth.source_sha256 = hash(&truncated);
        assert!(extract(&truncated, &auth.source_sha256, &auth).is_none());
    }

    #[test]
    fn adversarial_authorization_bounds_refuse_without_panicking() {
        let (bytes, mut auth) = fixture();
        let digest = hash(&bytes);
        auth.tracks[0].count = Some(usize::MAX);
        assert!(extract(&bytes, &digest, &auth).is_none());
        auth.tracks[0].count = Some(2);
        auth.groups[0].record.start = usize::MAX;
        assert!(extract(&bytes, &digest, &auth).is_none());
        auth.groups[0].record.start = 8;
        auth.correlated_fields[0].bytes = Bounds {
            start: usize::MAX - 1,
            end: usize::MAX,
        };
        assert!(extract(&bytes, &digest, &auth).is_none());
    }
}
