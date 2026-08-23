mod config;
mod init;
mod templates;
use clap::{CommandFactory, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "dc",
    version,
    about = "dc: A CLI that manages Dev Environment Dockerfile Collections"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Initializes DC (creates the templates directories)
    Init,
    /// Commands for local templates management
    #[command(subcommand)]
    Local(LocalCommand),
}

#[derive(Subcommand)]
enum LocalCommand {
    /// List local templates
    List,
    /// Show details of a local template (single-select)
    Info {
        /// Template to show (hidden, for testing — bypasses interactive prompt)
        #[arg(long, hide = true)]
        template: Option<String>,
    },
    /// Delete local templates (multiselect)
    Delete {
        /// Templates to delete (hidden, for testing — bypasses interactive prompt)
        #[arg(long, hide = true)]
        template: Vec<String>,
        /// Skip confirmation (hidden, for testing)
        #[arg(long, hide = true, default_value_t = false)]
        yes: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Command::Init) => init::run(),
        Some(Command::Local(LocalCommand::List)) => templates::run_list(),
        Some(Command::Local(LocalCommand::Info { template })) => templates::run_info_with(template),
        Some(Command::Local(LocalCommand::Delete { template, yes })) => {
            templates::run_delete_with(template, yes)
        }
        None => {
            let mut cmd = Cli::command();
            let _ = cmd.print_help();
            println!();
            Ok(())
        }
    }
}
