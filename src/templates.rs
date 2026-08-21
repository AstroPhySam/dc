use crate::config;
use anyhow::Result;
use inquire::Select;
use std::fs;
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

pub fn run_info_with(template: Option<String>) -> Result<()> {
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
        return Ok(());
    }

    let rel = if let Some(name) = template {
        let normalized = name.replace('\\', "/");
        tpls.iter()
            .find(|p| p.display().to_string().replace('\\', "/") == normalized)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("template '{}' not found", name))?
    } else {
        let items: Vec<String> = tpls
            .iter()
            .map(|p| p.display().to_string().replace('\\', "/"))
            .collect();

        let ans = Select::new("Select a template:", items.clone()).prompt()?;
        let idx = items.iter().position(|s| s == &ans).unwrap();
        tpls[idx].clone()
    };

    let dir = local.join(&rel);
    print_details(&dir)
}

fn print_details(dir: &Path) -> Result<()> {
    let details = dir.join("details.txt");
    if details.is_file() {
        let content = fs::read_to_string(details)?;
        println!("\nDetails of the Dockerfile: \n{content}");
    } else {
        let dockerfile = dir.join("Dockerfile");
        let content = fs::read_to_string(dockerfile)?;
        println!("\nDockerfile: \n{content}");
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

    for e in fs::read_dir(dir)? {
        let p = e?.path();
        if p.is_dir() {
            walk(&p, root, out)?;
        }
    }
    Ok(())
}
