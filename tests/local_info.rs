mod common;
use common::{dc_local_info, get_stdout, temp_dc, write_template, write_template_with_details};
use std::fs;

#[test]
fn shows_details_txt_when_present() {
    let (_tmp, base) = temp_dc();
    let local = base.join("templates").join("local");

    write_template_with_details(
        &local,
        "bash",
        "FROM ubuntu:22.04\n",
        Some("my custom details\nline2"),
    );

    let assert = dc_local_info(&base, Some("bash")).assert().success();
    let out = get_stdout(&assert);

    assert!(
        out.contains("Details of the Dockerfile:"),
        "missing header: {out}"
    );
    assert!(
        out.contains("my custom details"),
        "missing details.txt content: {out}"
    );
    assert!(out.contains("line2"), "missing details line2: {out}");
}

#[test]
fn falls_back_to_dockerfile() {
    let (_tmp, base) = temp_dc();
    let local = base.join("templates").join("local");

    write_template_with_details(&local, "bash", "FROM ubuntu:22.04\nRUN echo hi\n", None);

    let assert = dc_local_info(&base, Some("bash")).assert().success();
    let out = get_stdout(&assert);

    assert!(
        out.contains("Dockerfile:"),
        "missing Dockerfile header: {out}"
    );
    assert!(
        out.contains("FROM ubuntu:22.04"),
        "missing Dockerfile content: {out}"
    );
    assert!(out.contains("RUN echo hi"), "missing second line: {out}");
}

#[test]
fn empty_local_prints_message() {
    let (_tmp, base) = temp_dc();
    fs::create_dir_all(base.join("templates").join("local")).unwrap();

    let assert = dc_local_info(&base, Some("bash")).assert().success();
    let out = get_stdout(&assert);

    assert!(
        out.to_lowercase().contains("no templates found"),
        "expected empty message, got: {out}"
    );
}

#[test]
fn missing_local_hints_init() {
    let (_tmp, base) = temp_dc(); // never created

    let assert = dc_local_info(&base, Some("bash")).assert().success();
    let out = get_stdout(&assert);

    assert!(
        out.to_lowercase().contains("dc init"),
        "expected hint to run dc init, got: {out}"
    );
}

#[test]
fn unknown_template_errors() {
    let (_tmp, base) = temp_dc();
    let local = base.join("templates").join("local");

    write_template(&local, "bash");

    let assert = dc_local_info(&base, Some("nonexistent")).assert().failure();
    let stderr = String::from_utf8_lossy(&assert.get_output().stderr).into_owned();
    let stdout = get_stdout(&assert);
    let combined = format!("{stdout}{stderr}");

    assert!(
        combined.to_lowercase().contains("not found"),
        "expected not found error, got stdout: {stdout} stderr: {stderr}"
    );
}
