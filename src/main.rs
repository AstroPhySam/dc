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
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Command::Init) => init::run(),
        Some(Command::Local(LocalCommand::List)) => templates::run_list(),
        None => {
            let mut cmd = Cli::command();
            let _ = cmd.print_help();
            println!();
            Ok(())
        }
    }
}
