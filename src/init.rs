use crate::config;
use anyhow::Result;

pub fn run() -> Result<()> {
    let dc = config::dc_dir()?;
    println!("Initializing DC at {}", dc.display());
    println!();

    let dirs = [
        ("~/.dc", dc.clone()),
        ("   └── templates/", config::templates_dir()?),
        ("       ├── local/", config::local_dir()?),
        ("       └── remote/", config::remote_dir()?),
    ];

    for (label, dir) in dirs {
        if dir.exists() {
            println!("  {:<20} # already exists", label);
        } else {
            std::fs::create_dir_all(&dir)?;
            println!("  {:<20} # created", label);
        }
    }
    Ok(())
}
