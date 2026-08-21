mod common;
use common::{dc_init, get_stdout, temp_dc};
use predicates::prelude::*;
use std::fs;

#[test]
fn init_creates_all_directories() {
    let (_tmp, base) = temp_dc();

    let assert = dc_init(&base)
        .assert()
        .success()
        .stdout(predicate::str::contains("Initializing DC"));

    let out = get_stdout(&assert);
    assert_eq!(out.matches("# created").count(), 4);
    assert_eq!(out.matches("# already exists").count(), 0);

    assert!(base.join("templates").is_dir());
    assert!(base.join("templates/local").is_dir());
    assert!(base.join("templates/remote").is_dir());
}

#[test]
fn init_is_idempotent() {
    let (_tmp, base) = temp_dc();

    dc_init(&base).assert().success();

    let assert = dc_init(&base)
        .assert()
        .success()
        .stdout(predicate::str::contains("Initializing DC"));

    let out = get_stdout(&assert);
    assert_eq!(out.matches("# already exists").count(), 4);
    assert_eq!(out.matches("# created").count(), 0);

    assert!(base.join("templates/local").is_dir());
    assert!(base.join("templates/remote").is_dir());
}

#[test]
fn init_preserves_existing_templates() {
    let (_tmp, base) = temp_dc();
    let local = base.join("templates/local");

    fs::create_dir_all(&local).unwrap();
    fs::write(local.join("Dockerfile"), "FROM alpine\n").unwrap();

    let assert = dc_init(&base).assert().success();
    let out = get_stdout(&assert);
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
    let (_tmp, base) = temp_dc();

    fs::create_dir_all(&base).unwrap();

    let assert = dc_init(&base).assert().success();
    let out = get_stdout(&assert);

    let base_line = out.lines().find(|l| l.contains("~/.dc")).unwrap();
    assert!(base_line.contains("# already exists"));

    let local_line = out.lines().find(|l| l.contains("local/")).unwrap();
    assert!(local_line.contains("# created"));

    assert!(base.join("templates/local").is_dir());
    assert!(base.join("templates/remote").is_dir());
}
