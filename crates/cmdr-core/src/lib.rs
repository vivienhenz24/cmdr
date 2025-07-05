//! Core library for cmdr - natural language to shell command translation
//!
//! This crate provides the core functionality for translating natural language
//! requests into shell commands using local LLM inference.

pub mod inference;
pub mod shell;
pub mod translation;

pub use inference::{InferenceEngine, MockInferenceEngine};
pub use shell::ShellExecutor;
pub use translation::TranslationEngine;

/// Common types used throughout the cmdr ecosystem
pub mod types {
    use serde::{Deserialize, Serialize};

    /// A natural language request from the user
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct NaturalLanguageRequest {
        pub text: String,
        pub context: Option<String>,
    }

    /// A translated shell command
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ShellCommand {
        pub command: String,
        pub explanation: Option<String>,
        pub confidence: f32,
    }

    /// Result of command execution
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ExecutionResult {
        pub success: bool,
        pub output: String,
        pub error: Option<String>,
        pub exit_code: i32,
    }
}

pub use types::*;

pub fn hello_world() {
    println!("Hello from cmdr-core!");

    // TODO: wire up llama.cpp FFI bindings in Phase 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        hello_world();
    }
}
