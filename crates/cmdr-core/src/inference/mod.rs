//! LLM inference module
//! 
//! This module provides the interface for different LLM inference engines.

mod engine;

pub use engine::{InferenceEngine, MockInferenceEngine};

/// Error type for inference operations
#[derive(Debug, thiserror::Error)]
pub enum InferenceError {
    #[error("Failed to initialize inference engine: {0}")]
    Initialization(String),
    #[error("Failed to generate response: {0}")]
    Generation(String),
    #[error("Model not loaded: {0}")]
    ModelNotLoaded(String),
    #[error("Invalid parameters: {0}")]
    InvalidParams(String),
}

/// Result type for inference operations
pub type InferenceResult<T> = Result<T, InferenceError>; 