//! Only synthetic temporary inputs; never searches for authentic projects.
use std::{
    fs,
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
};
static NEXT: AtomicU64 = AtomicU64::new(0);
fn run(args: &[&std::ffi::OsStr]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_phoenix-census"))
        .args(args)
        .output()
        .unwrap()
}
#[test]
fn explicit_allowlist_missing_source_and_no_recursive_discovery() {
    let root = std::env::temp_dir().join(format!(
        "phoenix-census-cli-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir(&root).unwrap();
    let output = root.join("report.json");
    let missing = root.join("PRIVATE-SENTINEL");
    let r = run(&["--output".as_ref(), output.as_os_str(), missing.as_os_str()]);
    assert!(!r.status.success());
    assert!(r.stdout.is_empty());
    assert!(!String::from_utf8_lossy(&r.stderr).contains("PRIVATE-SENTINEL"));
    assert!(!output.exists()); // Failure before output creation.
    let r = run(&["--output".as_ref(), output.as_os_str(), root.as_os_str()]);
    assert!(!r.status.success());
    assert!(String::from_utf8_lossy(&r.stderr).contains("source_not_regular_file"));
    let allowlist = root.join("allowlist.json");
    fs::write(&allowlist, b"[]").unwrap();
    let r = run(&[
        "--allowlist".as_ref(),
        allowlist.as_os_str(),
        missing.as_os_str(),
    ]);
    assert!(!r.status.success());
    assert!(
        String::from_utf8_lossy(&r.stderr).contains("use_either_explicit_files_or_one_allowlist")
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn successful_cli_is_deterministic_private_and_never_overwrites() {
    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;
    let root = std::env::temp_dir().join(format!(
        "phoenix-census-cli-success-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir(&root).unwrap();
    #[cfg(unix)]
    fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).unwrap();
    let source = root.join("PRIVATE-SOURCE");
    let original = [0u8; 3];
    fs::write(&source, original).unwrap();
    let allowlist = root.join("approved.json");
    fs::write(
        &allowlist,
        serde_json::to_vec(&serde_json::json!([{"id":"p000","path":source}])).unwrap(),
    )
    .unwrap();
    let a = root.join("a.json");
    let b = root.join("b.json");
    for output in [&a, &b] {
        let result = run(&[
            "--allowlist".as_ref(),
            allowlist.as_os_str(),
            "--output".as_ref(),
            output.as_os_str(),
        ]);
        assert!(result.status.success(), "{:?}", result.stderr);
        assert!(result.stdout.is_empty());
        #[cfg(unix)]
        assert_eq!(
            fs::metadata(output).unwrap().permissions().mode() & 0o777,
            0o600
        );
    }
    let bytes = fs::read(&a).unwrap();
    assert_eq!(bytes, fs::read(&b).unwrap());
    // Successful census execution must still report this synthetic structural
    // refusal honestly, rather than inventing sequence or recovery coverage.
    let report: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(
        report["projects"][0]["report"]["diagnostics"][0]["outcome"],
        "Failed"
    );
    assert_eq!(
        report["projects"][0]["report"]["sequences"],
        serde_json::json!([])
    );
    assert!(!String::from_utf8_lossy(&bytes).contains("PRIVATE-SOURCE"));
    let result = run(&["--output".as_ref(), a.as_os_str(), source.as_os_str()]);
    assert!(!result.status.success());
    assert_eq!(fs::read(&a).unwrap(), bytes);
    assert_eq!(fs::read(&source).unwrap(), original);
    fs::remove_dir_all(root).unwrap();
}
