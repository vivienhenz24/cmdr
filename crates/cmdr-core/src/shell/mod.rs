//! Shell command execution module
//! 
//! This module handles the execution of shell commands.

mod executor;
mod parser;

pub use executor::ShellExecutor;
pub use parser::CommandParser;


/// Error type for shell operations
#[derive(Debug, thiserror::Error)]
pub enum ShellError {
    #[error("Failed to execute command: {0}")]
    Execution(String),
    #[error("Command not found: {0}")]
    CommandNotFound(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("Invalid command: {0}")]
    InvalidCommand(String),
}

/// Result type for shell operations
pub type ShellResult<T> = Result<T, ShellError>; 