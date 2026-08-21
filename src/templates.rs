use crate::config;
use anyhow::Result;
use std::path::{Path, PathBuf};

pub fn run_list() -> Result<()> {
    let local = config::local_dir()?;
    if !local.exists() {
        println!(
            "No templates directory at {}, run `dc init` first.",
            local.display()
        );
        return Ok(());
    }

    let tpls = find_templates(&local)?;
    if tpls.is_empty() {
        println!("No templates found in {}.", local.display());
    } else {
        println!("Available local templates:");
        for t in tpls {
            let display = t.display().to_string().replace('\\', "/");
            println!("└──> {}", display);
        }
    }
    Ok(())
}

pub fn find_templates(root: &Path) -> Result<Vec<PathBuf>> {
    if !root.exists() {
        return Ok(vec![]);
    }

    let mut out = Vec::new();
    walk(root, root, &mut out)?;
    out.sort_by(|a, b| {
        a.to_string_lossy()
            .replace('\\', "/")
            .cmp(&b.to_string_lossy().replace('\\', "/"))
    });
    Ok(out)
}

fn walk(dir: &Path, root: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    if dir.join("Dockerfile").is_file() {
        out.push(dir.strip_prefix(root).unwrap().to_path_buf());
    }

    for e in std::fs::read_dir(dir)? {
        let p = e?.path();
        if p.is_dir() {
            walk(&p, root, out)?;
        }
    }
    Ok(())
}
