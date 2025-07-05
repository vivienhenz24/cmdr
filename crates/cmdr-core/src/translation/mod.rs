//! Natural language translation module
//! 
//! This module handles translating natural language requests into shell commands.

mod engine;
mod prompt;

pub use engine::TranslationEngine;
pub use prompt::PromptTemplate;


/// Error type for translation operations
#[derive(Debug, thiserror::Error)]
pub enum TranslationError {
    #[error("Failed to generate translation: {0}")]
    Generation(String),
    #[error("Invalid prompt template: {0}")]
    InvalidPrompt(String),
    #[error("Inference engine error: {0}")]
    Inference(#[from] crate::inference::InferenceError),
}

/// Result type for translation operations
pub type TranslationResult<T> = Result<T, TranslationError>; 