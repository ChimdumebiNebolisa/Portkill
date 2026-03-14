use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PortkillError {
    #[error("no process is using port {0}")]
    NoProcessOnPort(u16),

    #[allow(dead_code)]
    #[error("required command not found: {command}. {hint}")]
    CommandNotFound { command: String, hint: String },

    #[error("failed to run '{command}': {source}")]
    CommandFailed {
        command: String,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to kill PID {pid}: {message}")]
    KillFailed { pid: u32, message: String },

    #[allow(dead_code)]
    #[error("failed to parse output: {0}")]
    ParseError(String),

    #[error("some processes could not be killed: {0}")]
    PartialKillFailure(String),
}

pub type Result<T> = std::result::Result<T, PortkillError>;

#[allow(dead_code)]
pub fn command_not_found(command: &str, hint: &str) -> PortkillError {
    PortkillError::CommandNotFound {
        command: command.to_string(),
        hint: hint.to_string(),
    }
}

pub fn check_command_output(cmd: &mut Command, command_name: &str) -> Result<std::process::Output> {
    cmd.output().map_err(|e| PortkillError::CommandFailed {
        command: command_name.to_string(),
        source: e,
    })
}
