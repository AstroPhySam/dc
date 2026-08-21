mod common;
use common::{dc_local_list, get_stdout, temp_dc, write_template};
use predicates::prelude::*;
use std::fs;

#[test]
fn lists_nested_templates_sorted() {
    let (_tmp, base) = temp_dc();
    let local = base.join("templates").join("local");
    write_template(&local, "rust");
    write_template(&local, "bash");
    write_template(&local, "python/basic");

    let assert = dc_local_list(&base)
        .assert()
        .success()
        .stdout(predicate::str::contains("Available local templates"));

    let out = get_stdout(&assert);
    assert!(out.contains("bash"), "missing bash in {out}");
    assert!(out.contains("rust"), "missing rust in {out}");
    assert!(
        out.contains("python/basic"),
        "missing python/basic in {out}"
    );
    assert!(
        !out.contains("\n  python\n"),
        "should not list intermediate dir without Dockerfile: {out}"
    );

    // sorted order check
    let pos_bash = out.find("bash").unwrap();
    let pos_python = out.find("python/basic").unwrap();
    let pos_rust = out.find("rust").unwrap();
    assert!(
        pos_bash < pos_python && pos_python < pos_rust,
        "not sorted: {out}"
    );
}

#[test]
fn ignores_dirs_without_dockerfile() {
    let (_tmp, base) = temp_dc();
    let local = base.join("templates").join("local");
    fs::create_dir_all(local.join("empty")).unwrap();
    fs::write(local.join("empty").join("README.md"), "hi\n").unwrap();
    write_template(&local, "has");

    let assert = dc_local_list(&base).assert().success();
    let out = get_stdout(&assert);
    assert!(out.contains("has"), "{out}");
    assert!(
        !out.contains("empty"),
        "dir without Dockerfile leaked: {out}"
    );
}

#[test]
fn empty_local_prints_message() {
    let (_tmp, base) = temp_dc();
    fs::create_dir_all(base.join("templates").join("local")).unwrap();

    let assert = dc_local_list(&base).assert().success();
    let out = get_stdout(&assert);
    assert!(
        out.to_lowercase().contains("no templates found"),
        "expected empty message, got: {out}"
    );
}

#[test]
fn missing_local_hints_init() {
    let (_tmp, base) = temp_dc(); // never created

    let assert = dc_local_list(&base).assert().success();
    let out = get_stdout(&assert);
    assert!(
        out.to_lowercase().contains("dc init"),
        "expected hint to run dc init, got: {out}"
    );
}
