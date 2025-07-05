//! Command parsing utilities
//! 
//! This module provides utilities for parsing and validating shell commands.

use super::{ShellError, ShellResult};

/// Command parser for shell commands
pub struct CommandParser;

impl CommandParser {
    /// Parse a command string into its components
    pub fn parse(command: &str) -> ShellResult<Vec<String>> {
        let trimmed = command.trim();
        if trimmed.is_empty() {
            return Err(ShellError::InvalidCommand("Empty command".to_string()));
        }
        
        // Simple space-based splitting for now
        // TODO: Implement proper shell command parsing
        let parts: Vec<String> = trimmed
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        
        Ok(parts)
    }
    
    /// Check if a command is potentially dangerous
    pub fn is_dangerous(command: &str) -> bool {
        let dangerous_patterns = [
            "rm -rf /",
            "rm -rf /*",
            "dd if=/dev/zero",
            "mkfs",
            "fdisk",
            "format",
        ];
        
        let lower_command = command.to_lowercase();
        dangerous_patterns.iter().any(|pattern| lower_command.contains(pattern))
    }
    
    /// Validate command safety
    pub fn validate_safety(command: &str) -> ShellResult<()> {
        if Self::is_dangerous(command) {
            return Err(ShellError::InvalidCommand(
                "Command appears to be potentially dangerous".to_string()
            ));
        }
        Ok(())
    }
} 