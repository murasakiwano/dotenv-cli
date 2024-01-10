use std::io::Result;

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

    let code = run_command(cli.command)?;

    std::process::exit(code);
}

fn run_command(cli_command: Vec<String>) -> Result<i32> {
    if cli_command.is_empty() {
        return Ok(0);
    }

    let mut command = std::process::Command::new(&cli_command[0]);

    if cli_command.len() > 1 {
        cli_command[1..].iter().for_each(|a| {
            command.arg(a);
        });
    }

    let mut child = command.spawn()?;
    let exit_status = child.wait()?;

    let code = exit_status.code().unwrap_or(-1);
    println!("\nChild process exited with code {:?}", code);

    Ok(code)
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
