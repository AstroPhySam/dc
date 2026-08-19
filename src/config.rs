use anyhow::{Context, Result};
use std::path::PathBuf;

pub fn dc_dir() -> Result<PathBuf> {
    if let Ok(home) = std::env::var("DC_HOME") {
        return Ok(PathBuf::from(home));
    }
    let home = dirs::home_dir().context("could not locate home directory")?;
    Ok(home.join(".dc"))
}

pub fn templates_dir() -> Result<PathBuf> {
    Ok(dc_dir()?.join("templates"))
}

pub fn local_dir() -> Result<PathBuf> {
    Ok(templates_dir()?.join("local"))
}

pub fn remote_dir() -> Result<PathBuf> {
    Ok(templates_dir()?.join("remote"))
}
