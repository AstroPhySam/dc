#![allow(dead_code)]

use assert_cmd::Command;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::{TempDir, tempdir};

/// Create a temp directory and return `(TempDir, dc_home_base)` where
/// `dc_home_base = tmp.path().join("dc_home")`. Keeps `DC_HOME` isolated.
pub fn temp_dc() -> (TempDir, PathBuf) {
    let tmp = tempdir().expect("create tempdir");
    let base = tmp.path().join("dc_home");
    (tmp, base)
}

/// Base `dc` command with `DC_HOME` set, no subcommand.
pub fn dc(base: &Path) -> Command {
    let mut cmd = Command::cargo_bin("dc").expect("cargo_bin dc");
    cmd.env("DC_HOME", base);
    cmd
}

/// `dc init` command.
pub fn dc_init(base: &Path) -> Command {
    let mut cmd = dc(base);
    cmd.arg("init");
    cmd
}

/// `dc local list` command.
pub fn dc_local_list(base: &Path) -> Command {
    let mut cmd = dc(base);
    cmd.args(["local", "list"]);
    cmd
}

/// Create a template at `local/<rel>/Dockerfile`.
pub fn write_template(local: &Path, rel: &str) {
    let dir = local.join(rel);
    fs::create_dir_all(&dir).expect("create template dir");
    fs::write(dir.join("Dockerfile"), "FROM alpine\n").expect("write Dockerfile");
}

/// Extract stdout as String from an `assert_cmd` Assert.
pub fn get_stdout(assert: &assert_cmd::assert::Assert) -> String {
    String::from_utf8_lossy(&assert.get_output().stdout).into_owned()
}
