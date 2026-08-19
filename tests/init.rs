use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

fn dc_in(base: &Path) -> Command {
    let mut cmd = Command::cargo_bin("dc").unwrap();
    cmd.env("DC_HOME", base).arg("init");
    cmd
}

fn stdout(assert: &assert_cmd::assert::Assert) -> String {
    String::from_utf8_lossy(&assert.get_output().stdout).into_owned()
}

#[test]
fn init_creates_all_directories() {
    let tmp = tempdir().unwrap();
    let base = tmp.path().join("dc_home");

    let assert = dc_in(&base)
        .assert()
        .success()
        .stdout(predicate::str::contains("Initializing DC"));

    let out = stdout(&assert);
    assert_eq!(out.matches("# created").count(), 4);
    assert_eq!(out.matches("# already exists").count(), 0);

    assert!(base.join("templates").is_dir());
    assert!(base.join("templates/local").is_dir());
    assert!(base.join("templates/remote").is_dir());
}

#[test]
fn init_is_idempotent() {
    let tmp = tempdir().unwrap();
    let base = tmp.path().join("dc_home");

    dc_in(&base).assert().success();

    let assert = dc_in(&base)
        .assert()
        .success()
        .stdout(predicate::str::contains("Initializing DC"));

    let out = stdout(&assert);
    assert_eq!(out.matches("# already exists").count(), 4);
    assert_eq!(out.matches("# created").count(), 0);

    assert!(base.join("templates/local").is_dir());
    assert!(base.join("templates/remote").is_dir());
}

#[test]
fn init_preserves_existing_templates() {
    let tmp = tempdir().unwrap();
    let base = tmp.path().join("dc_home");
    let local = base.join("templates/local");

    fs::create_dir_all(&local).unwrap();
    fs::write(local.join("Dockerfile"), "FROM alpine\n").unwrap();

    let assert = dc_in(&base).assert().success();
    let out = stdout(&assert);
    let local_line = out.lines().find(|l| l.contains("local/")).unwrap();

    assert!(
        local_line.contains("# already exists"),
        "expected preserved local: {local_line}"
    );

    let dockerfile = fs::read_to_string(local.join("Dockerfile")).unwrap();
    assert_eq!(dockerfile, "FROM alpine\n");
}

#[test]
fn init_mixed_state() {
    let tmp = tempdir().unwrap();
    let base = tmp.path().join("dc_home");

    fs::create_dir_all(&base).unwrap();

    let assert = dc_in(&base).assert().success();
    let out = stdout(&assert);

    let base_line = out.lines().find(|l| l.contains("~/.dc")).unwrap();
    assert!(base_line.contains("# already exists"));

    let local_line = out.lines().find(|l| l.contains("local/")).unwrap();
    assert!(local_line.contains("# created"));

    assert!(base.join("templates/local").is_dir());
    assert!(base.join("templates/remote").is_dir());
}
