mod common;
use common::{dc_local_launch, get_stdout, temp_dc, write_template};
use std::fs;

#[test]
fn missing_local_hints_init() {
    let (_tmp, base) = temp_dc();

    let assert = dc_local_launch(&base, Some("bash")).assert().success();
    let out = get_stdout(&assert);

    assert!(
        out.to_lowercase().contains("dc init"),
        "expected hint: {out}"
    );
}

#[test]
fn empty_local_prints_message() {
    let (_tmp, base) = temp_dc();

    fs::create_dir_all(base.join("templates").join("local")).unwrap();

    let assert = dc_local_launch(&base, Some("bash")).assert().success();
    let out = get_stdout(&assert);

    assert!(
        out.to_lowercase().contains("no templates found"),
        "expected empty message: {out}"
    );
}

#[test]
fn unknown_template_errors() {
    let (_tmp, base) = temp_dc();
    let local = base.join("templates").join("local");

    write_template(&local, "bash");

    let assert = dc_local_launch(&base, Some("nonexistent"))
        .assert()
        .failure();

    let stderr = String::from_utf8_lossy(&assert.get_output().stderr).into_owned();
    let stdout = get_stdout(&assert);
    let combined = format!("{stdout}{stderr}");

    assert!(
        combined.to_lowercase().contains("not found"),
        "expected not found: {combined}"
    );
}
