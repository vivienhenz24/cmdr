//! llama.cpp FFI bindings for cmdr
//! 
//! This crate provides safe Rust bindings to the llama.cpp C API
//! for local LLM inference in cmdr.

pub mod install;

#[cfg(feature = "native-llama")]
mod bindings;

#[cfg(feature = "native-llama")]
pub use bindings::*;

#[cfg(not(feature = "native-llama"))]
pub mod llama_stub {
    // Provide dummy exports if needed
}

pub use install::LlmInstaller;

/// Error type for LLM operations
#[derive(Debug, thiserror::Error)]
pub enum LlmError {
    #[error("Failed to load model: {0}")]
    ModelLoad(String),
    #[error("Failed to initialize context: {0}")]
    ContextInit(String),
    #[error("Failed to generate response: {0}")]
    Generation(String),
    #[error("Invalid parameters: {0}")]
    InvalidParams(String),
}

/// Result type for LLM operations
pub type LlmResult<T> = Result<T, LlmError>;

/// LLM context for managing model state
pub struct LlmContext {
    // TODO: Implement llama.cpp context management
}

impl LlmContext {
    /// Create a new LLM context
    pub fn new() -> LlmResult<Self> {
        // TODO: Initialize llama.cpp context
        Ok(Self {})
    }

    /// Generate a response for the given prompt
    pub fn generate(&mut self, _prompt: &str) -> LlmResult<String> {
        // TODO: Implement generation using llama.cpp
        Ok("Generated response".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_creation() {
        let _context = LlmContext::new().expect("Failed to create context");
    }
} 