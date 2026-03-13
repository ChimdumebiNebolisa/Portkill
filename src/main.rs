mod cli;
mod error;
mod platform;

use std::process::ExitCode;

use clap::Parser;
use dialoguer::Confirm;

use cli::Cli;
use error::{PortkillError, Result};
use platform::{current_platform, get_unique_processes, Platform};

fn run() -> Result<()> {
    let cli = Cli::parse();
    let platform = current_platform();
    let processes = platform.find_processes_on_port(cli.port)?;
    let processes = get_unique_processes(processes);

    for p in &processes {
        let name = p.name.as_deref().unwrap_or("(unknown)");
        if cli.dry_run {
            println!("{} {} (would kill)", p.pid, name);
        } else {
            println!("{} {}", p.pid, name);
        }
    }

    if cli.dry_run {
        println!("Dry run: no processes were killed.");
        return Ok(());
    }

    if cli.should_prompt() {
        let confirm = Confirm::new()
            .with_prompt("Kill these processes?")
            .default(false)
            .interact()
            .map_err(|e| PortkillError::CommandFailed {
                command: "confirmation prompt".to_string(),
                source: e,
            })?;
        if !confirm {
            println!("Aborted.");
            return Ok(());
        }
    }

    let force = cli.force;
    let mut failed = Vec::new();
    for p in &processes {
        if let Err(e) = platform.kill_process(p.pid, force) {
            eprintln!("Failed to kill {}: {}", p.pid, e);
            failed.push(p.pid);
        }
    }

    if failed.is_empty() {
        println!("Killed {} process(es).", processes.len());
        Ok(())
    } else if failed.len() == processes.len() {
        Err(PortkillError::KillFailed {
            pid: failed[0],
            message: "all kill attempts failed".to_string(),
        })
    } else {
        Err(PortkillError::PartialKillFailure(format!(
            "failed PIDs: {}",
            failed
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )))
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("portkill: {}", e);
            ExitCode::FAILURE
        }
    }
}
