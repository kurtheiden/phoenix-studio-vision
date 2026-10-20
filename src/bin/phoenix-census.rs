//! Explicit approved files only. No discovery, source writes, or MIDI publication.
//! Usage: phoenix-census --output PRIVATE_NEW_JSON FILE...
//! Or: --allowlist PRIVATE_JSON --output PRIVATE_NEW_JSON
//! Allowlist: [{"id":"p000","path":"/private/approved-copy"}]. IDs must be opaque.
//! Reports are created with mode 0600; use an existing private output directory.
use phoenix::census::{census_path, Metrics, ProjectCensus};
use serde::{Deserialize, Serialize};
#[cfg(unix)]
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Approved {
    id: String,
    path: PathBuf,
}
#[derive(Serialize)]
struct Entry {
    project_id: String,
    source_integrity: &'static str,
    report: ProjectCensus,
}
#[derive(Serialize)]
struct Report {
    schema_version: u32,
    projects: Vec<Entry>,
    metrics: Metrics,
}
#[derive(PartialEq, Eq)]
struct Evidence {
    bytes: Vec<u8>,
    metadata: Vec<u64>,
    attributes: Vec<u8>,
}
fn evidence(path: &Path) -> Result<Evidence, &'static str> {
    let m = fs::metadata(path).map_err(|_| "source_metadata_unavailable")?;
    if !m.is_file() {
        return Err("source_not_regular_file");
    }
    #[cfg(unix)]
    let metadata = vec![
        m.len(),
        m.mode() as u64,
        m.uid() as u64,
        m.gid() as u64,
        m.ino(),
        m.dev(),
        m.mtime() as u64,
        m.mtime_nsec() as u64,
        m.ctime() as u64,
        m.ctime_nsec() as u64,
    ];
    #[cfg(not(unix))]
    let metadata = vec![m.len()];
    #[cfg(target_os = "macos")]
    let attributes = {
        let out = std::process::Command::new("/usr/bin/xattr")
            .arg("-lx")
            .arg(path)
            .output()
            .map_err(|_| "extended_attributes_unavailable")?;
        if !out.status.success() {
            return Err("extended_attributes_unavailable");
        }
        out.stdout
    };
    #[cfg(not(target_os = "macos"))]
    let attributes = Vec::new();
    Ok(Evidence {
        bytes: fs::read(path).map_err(|_| "source_unavailable")?,
        metadata,
        attributes,
    })
}
fn run() -> Result<(), &'static str> {
    let mut args = std::env::args_os().skip(1);
    let mut approved = Vec::new();
    let mut output = None;
    let mut allowlist_used = false;
    while let Some(a) = args.next() {
        if a == "--allowlist" {
            let path = args.next().ok_or("missing_allowlist")?;
            if !approved.is_empty() || allowlist_used {
                return Err("use_either_explicit_files_or_one_allowlist");
            }
            allowlist_used = true;
            approved =
                serde_json::from_slice(&fs::read(path).map_err(|_| "allowlist_unavailable")?)
                    .map_err(|_| "invalid_allowlist")?;
        } else if a == "--output" {
            if output.is_some() {
                return Err("duplicate_output");
            }
            output = Some(PathBuf::from(args.next().ok_or("missing_output")?));
        } else if a.to_string_lossy().starts_with("--") {
            return Err("unknown_option");
        } else {
            if allowlist_used {
                return Err("use_either_explicit_files_or_one_allowlist");
            }
            approved.push(Approved {
                id: format!("p{:03}", approved.len()),
                path: PathBuf::from(a),
            });
        }
    }
    if approved.is_empty() {
        return Err("usage: phoenix-census --output PRIVATE_NEW_JSON FILE... | --allowlist PRIVATE_JSON --output PRIVATE_NEW_JSON");
    }
    let mut ids = std::collections::BTreeSet::new();
    let mut paths = std::collections::BTreeSet::new();
    for a in &approved {
        if a.id.is_empty()
            || a.id.len() > 32
            || !a
                .id
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
            || !ids.insert(a.id.clone())
        {
            return Err("invalid_or_duplicate_opaque_id");
        }
        if !paths.insert(fs::canonicalize(&a.path).map_err(|_| "source_unavailable")?) {
            return Err("duplicate_source_path");
        }
    }
    approved.sort_by(|a, b| a.id.cmp(&b.id));
    let output = output.ok_or("private_output_required")?;
    // create_new prevents overwriting any source or existing report. Restrictive
    // permissions apply from creation; parent privacy remains operator-controlled.
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    options.mode(0o600);
    let mut file = options
        .open(output)
        .map_err(|_| "new_private_output_unavailable")?;
    let mut projects = Vec::new();
    let mut metrics = Metrics::default();
    for a in approved {
        let before = evidence(&a.path)?;
        let report = census_path(&a.path)?;
        let after = evidence(&a.path)?;
        if before != after {
            return Err("source_integrity_changed");
        }
        eprint!("{}\n{}", a.id, report.summary());
        metrics += &report.metrics;
        projects.push(Entry {
            project_id: a.id,
            source_integrity: if cfg!(target_os = "macos") {
                "data_metadata_extended_attributes_unchanged"
            } else {
                "data_unix_metadata_unchanged_extended_attributes_not_checked"
            },
            report,
        });
    }
    serde_json::to_writer_pretty(
        &mut file,
        &Report {
            schema_version: 1,
            projects,
            metrics,
        },
    )
    .map_err(|_| "report_write_failed")?;
    file.write_all(b"\n").map_err(|_| "report_write_failed")?;
    file.sync_all().map_err(|_| "report_sync_failed")?;
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("census failed: {e}");
        std::process::exit(1);
    }
}
