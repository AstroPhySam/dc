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
    /// Prints a Hello message
    Hello,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Command::Hello) => println!("Hello from 'dc'!"),
        None => {
            let mut cmd = Cli::command();
            let _ = cmd.print_help();
            println!();
        }
    }
}
