use crate::config;
use anyhow::{Context, Result};
use inquire::Select;
use std::io::IsTerminal;
use std::process::Command;

pub fn run_local_launch(template: Option<String>) -> Result<()> {
    let local = config::local_dir()?;
    if !local.exists() {
        println!(
            "No templates directory at {}, run `dc init` first.",
            local.display()
        );
        return Ok(());
    }

    let tpls = crate::templates::find_templates(&local)?;
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
    let slug = format!("local/{}", rel.display().to_string().replace('\\', "/")).replace('/', "-");
    let image = format!("dc/{}", slug);
    let container = format!("dc-{}", slug);

    let status = Command::new("docker")
        .args(["build", "-t", &image, &dir.display().to_string()])
        .status()
        .context("failed to run docker build — is Docker installed?")?;

    if !status.success() {
        anyhow::bail!("docker build failed for '{}'", rel.display());
    }

    let cwd = std::env::current_dir()?;
    let base = cwd
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("workspace")
        .to_string();

    let cwd_str = cwd.display().to_string();
    let vol = format!("{}:/{}", cwd_str, base);
    let workdir = format!("/{}", base);

    let is_tty = std::io::stdin().is_terminal() && std::io::stdout().is_terminal();
    let mut run_args = vec![
        "run", "--rm", "--name", &container, "-v", &vol, "-w", &workdir,
    ];

    if is_tty {
        run_args.insert(1, "-it");
    }

    let status = Command::new("docker")
        .args(run_args.iter().chain(std::iter::once(&image.as_str())))
        .status()
        .context("failed to run docker")?;

    if !status.success() {
        anyhow::bail!("docker run exited with {}", status);
    }
    Ok(())
}
