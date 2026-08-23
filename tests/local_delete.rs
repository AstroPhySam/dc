mod common;
use common::{dc_local_delete, get_stdout, temp_dc, write_template};
use std::fs;

#[test]
fn deletes_multiple_with_yes() {
    let (_tmp, base) = temp_dc();
    let local = base.join("templates").join("local");

    write_template(&local, "bash");
    write_template(&local, "rust");
    write_template(&local, "python/basic");

    let assert = dc_local_delete(&base, &["bash", "rust"], true)
        .assert()
        .success();

    let out = get_stdout(&assert);

    assert!(out.contains("Deleted 'bash'"), "missing bash delete: {out}");
    assert!(out.contains("Deleted 'rust'"), "missing rust delete: {out}");
    assert!(!local.join("bash").exists(), "bash not deleted");
    assert!(!local.join("rust").exists(), "rust not deleted");
    assert!(
        local.join("python/basic").exists(),
        "python/basic should remain"
    );
}

#[test]
fn aborts_on_no() {
    let (_tmp, base) = temp_dc();
    let local = base.join("templates").join("local");

    write_template(&local, "bash");

    // Without --yes, Confirm defaults to No. In non-TTY test env, prompt
    // falls back to false (see templates.rs unwrap_or(false)), so delete aborts.
    let assert = dc_local_delete(&base, &["bash"], false).assert().success();
    let out = get_stdout(&assert);

    assert!(
        out.contains("No template was deleted"),
        "expected abort message: {out}"
    );
    assert!(
        local.join("bash").exists(),
        "bash should not be deleted on abort"
    );
}

#[test]
fn aborts_when_no_selection() {
    let (_tmp, base) = temp_dc();
    let local = base.join("templates").join("local");

    write_template(&local, "bash");

    // No --template args triggers MultiSelect; in non-TTY it falls back to empty
    let assert = dc_local_delete(&base, &[], false).assert().success();
    let out = get_stdout(&assert);

    assert!(
        out.contains("No template was deleted"),
        "expected abort on empty selection: {out}"
    );
    assert!(
        local.join("bash").exists(),
        "bash should remain after empty"
    );
}

#[test]
fn empty_local_prints_message() {
    let (_tmp, base) = temp_dc();
    fs::create_dir_all(base.join("templates").join("local")).unwrap();

    let assert = dc_local_delete(&base, &["bash"], true).assert().success();
    let out = get_stdout(&assert);

    assert!(
        out.to_lowercase().contains("no templates found"),
        "expected empty message: {out}"
    );
}

#[test]
fn missing_local_hints_init() {
    let (_tmp, base) = temp_dc();

    let assert = dc_local_delete(&base, &["bash"], true).assert().success();
    let out = get_stdout(&assert);

    assert!(
        out.to_lowercase().contains("dc init"),
        "expected hint: {out}"
    );
}

#[test]
fn unknown_template_errors() {
    let (_tmp, base) = temp_dc();
    let local = base.join("templates").join("local");

    write_template(&local, "bash");

    let assert = dc_local_delete(&base, &["nonexistent"], true)
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

#[test]
fn preserves_siblings() {
    let (_tmp, base) = temp_dc();
    let local = base.join("templates").join("local");

    write_template(&local, "python/basic");
    write_template(&local, "python/other");
    write_template(&local, "bash");

    dc_local_delete(&base, &["python/basic"], true)
        .assert()
        .success();

    assert!(
        !local.join("python/basic").exists(),
        "python/basic not deleted"
    );
    assert!(
        local.join("python/other").exists(),
        "python/other should remain"
    );
    assert!(local.join("bash").exists(), "bash should remain");
}
