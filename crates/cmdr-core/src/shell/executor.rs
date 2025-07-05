//! Shell command executor implementation
//! 
//! This module implements the execution of shell commands.

use std::process::{Command, Stdio};
use crate::types::{ShellCommand, ExecutionResult};
use super::{ShellError, ShellResult};

/// Shell command executor
pub struct ShellExecutor {
    shell: String,
    auto_execute: bool,
}

impl ShellExecutor {
    /// Create a new shell executor
    pub fn new(shell: String) -> Self {
        Self {
            shell,
            auto_execute: true,
        }
    }
    
    /// Set whether to auto-execute commands
    pub fn set_auto_execute(&mut self, auto_execute: bool) {
        self.auto_execute = auto_execute;
    }
    
    /// Execute a shell command
    pub fn execute(&self, command: &ShellCommand) -> ShellResult<ExecutionResult> {
        if !self.auto_execute {
            return Ok(ExecutionResult {
                success: false,
                output: "Auto-execution disabled".to_string(),
                error: None,
                exit_code: 0,
            });
        }
        
        let output = Command::new(&self.shell)
            .arg("-c")
            .arg(&command.command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| ShellError::Execution(e.to_string()))?;
        
        let success = output.status.success();
        let output_str = String::from_utf8_lossy(&output.stdout).to_string();
        let error_str = String::from_utf8_lossy(&output.stderr).to_string();
        
        Ok(ExecutionResult {
            success,
            output: output_str,
            error: if error_str.is_empty() { None } else { Some(error_str) },
            exit_code: output.status.code().unwrap_or(-1),
        })
    }
    
    /// Validate a command without executing it
    pub fn validate(&self, command: &str) -> ShellResult<()> {
        // TODO: Implement command validation
        // For now, just check if the command is not empty
        if command.trim().is_empty() {
            return Err(ShellError::InvalidCommand("Empty command".to_string()));
        }
        Ok(())
    }
}

impl Default for ShellExecutor {
    fn default() -> Self {
        Self::new("bash".to_string())
    }
} 