use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

fn temporary_path(name: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "phoenix-test-{}-{unique}-{name}",
        std::process::id()
    ))
}

fn run(arguments: &[&std::ffi::OsStr]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_phoenix"))
        .args(arguments)
        .output()
        .expect("phoenix should run")
}

#[test]
fn inspects_a_binary_file_without_an_extension() {
    let path = temporary_path("sample");
    fs::write(&path, b"abc\0\xff").expect("fixture should be written");

    let output = run(&[path.as_os_str()]);
    let stdout = String::from_utf8(output.stdout).expect("stdout should be UTF-8");

    assert!(output.status.success());
    assert!(stdout.contains("Size: 5 bytes"));
    assert!(stdout
        .contains("SHA-256: 77cb6bea091ff250af304a09024b0c526be6a21014a91ab56e788c63a69e811f"));
    assert!(stdout.contains("61 62 63 00 ff"));
    assert_eq!(
        fs::read(&path).expect("fixture should remain readable"),
        b"abc\0\xff"
    );

    fs::remove_file(path).expect("fixture should be removed");
}

#[test]
fn rejects_an_empty_file() {
    let path = temporary_path("empty");
    fs::write(&path, []).expect("fixture should be written");

    let output = run(&[path.as_os_str()]);
    let stderr = String::from_utf8(output.stderr).expect("stderr should be UTF-8");

    assert!(!output.status.success());
    assert!(stderr.contains("is empty"));

    fs::remove_file(path).expect("fixture should be removed");
}

#[test]
fn rejects_a_missing_file() {
    let path = temporary_path("missing");
    let output = run(&[path.as_os_str()]);
    let stderr = String::from_utf8(output.stderr).expect("stderr should be UTF-8");

    assert!(!output.status.success());
    assert!(stderr.contains("cannot open"));
}

#[test]
fn requires_exactly_one_path() {
    let no_arguments = run(&[]);
    assert!(!no_arguments.status.success());
    assert!(String::from_utf8(no_arguments.stderr)
        .expect("stderr should be UTF-8")
        .contains("missing file path"));

    let two_arguments = run(&["one".as_ref(), "two".as_ref()]);
    assert!(!two_arguments.status.success());
    assert!(String::from_utf8(two_arguments.stderr)
        .expect("stderr should be UTF-8")
        .contains("expected exactly one file path"));
}
