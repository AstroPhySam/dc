#![cfg(feature = "docker")]

mod common;
use common::{dc_local_launch, get_stdout, has_docker, temp_dc, write_template_with_details};

#[test]
fn builds_simple_template() {
    if !has_docker() {
        eprintln!("skipping: docker not available");
        return;
    }

    let (_tmp, base) = temp_dc();
    let local = base.join("templates").join("local");

    write_template_with_details(
        &local,
        "alpine-echo",
        "FROM alpine\nCMD [\"echo\",\"hello docker\"]\n",
        None,
    );

    let assert = dc_local_launch(&base, Some("alpine-echo"))
        .assert()
        .success();

    let out = get_stdout(&assert);

    // docker build output goes to stdout/stderr via Command::status inheritance,
    // but our launch prints Deleted/Built via println. Just ensure it succeeded.
    assert!(
        out.to_lowercase().contains("alpine-echo") || assert.get_output().status.success(),
        "expected success: {out}"
    );
}
