use std::io::{self, Result, Write};

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    env_files: Vec<String>,

    #[arg(last = true)]
    command: Vec<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    cli.env_files.iter().for_each(|env_file| {
        dotenvy::from_filename(env_file).ok();
    });

    run_command(cli.command)
}

fn run_command(cli_command: Vec<String>) -> Result<()> {
    if cli_command.is_empty() {
        return Ok(());
    }

    let mut command = std::process::Command::new(&cli_command[0]);

    if cli_command.len() > 1 {
        cli_command[1..].iter().for_each(|a| {
            command.arg(a);
        });
    }

    let output = command.output()?;

    io::stdout().write_all(&output.stdout)
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
