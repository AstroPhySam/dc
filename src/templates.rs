use crate::config;
use anyhow::Result;
use inquire::{Confirm, MultiSelect, Select};
use std::fs;
use std::io::IsTerminal;
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
        if !std::io::stdin().is_terminal() {
            anyhow::bail!("no template specified and no terminal available for selection");
        }
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

pub fn run_delete_with(template: Vec<String>, yes: bool) -> Result<()> {
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

    let selected: Vec<PathBuf> = if !template.is_empty() {
        let mut out = Vec::new();
        for name in template {
            let normalized = name.replace('\\', "/");
            let found = tpls
                .iter()
                .find(|p| p.display().to_string().replace('\\', "/") == normalized)
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("template '{}' not found", name))?;

            out.push(found);
        }
        out
    } else {
        if !std::io::stdin().is_terminal() {
            println!("No template was deleted.");
            return Ok(());
        }

        let items: Vec<String> = tpls
            .iter()
            .map(|p| p.display().to_string().replace('\\', "/"))
            .collect();

        let ans = MultiSelect::new("Select templates to delete:", items.clone())
            .prompt()
            .unwrap_or_default();

        if ans.is_empty() {
            println!("No template was deleted.");
            return Ok(());
        }

        ans.iter()
            .map(|s| {
                let idx = items.iter().position(|x| x == s).unwrap();
                tpls[idx].clone()
            })
            .collect()
    };

    if selected.is_empty() {
        println!("No template was deleted.");
        return Ok(());
    }

    let confirmed = if yes {
        true
    } else if !std::io::stdin().is_terminal() {
        false
    } else {
        Confirm::new("Are you sure?")
            .with_default(false)
            .prompt()
            .unwrap_or(false)
    };

    if !confirmed {
        println!("No template was deleted.");
        return Ok(());
    }

    for rel in selected {
        let dir = local.join(&rel);
        fs::remove_dir_all(&dir)?;
        let display = rel.display().to_string().replace('\\', "/");
        println!("Deleted '{}'", display);
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
