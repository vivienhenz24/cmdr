//! Inference engine trait definition
//!
//! This module defines the trait that all inference engines must implement.

use super::{InferenceError, InferenceResult};
use async_trait::async_trait;

/// Trait for LLM inference engines
#[async_trait]
pub trait InferenceEngine: Send + Sync {
    /// Initialize the inference engine
    fn initialize(&mut self) -> InferenceResult<()>;

    /// Load a model from a file
    fn load_model(&mut self, model_path: &str) -> InferenceResult<()>;

    /// Generate a response for the given prompt
    async fn generate(&mut self, prompt: &str) -> InferenceResult<String>;

    /// Set generation parameters
    fn set_parameters(&mut self, temperature: f32, max_tokens: usize) -> InferenceResult<()>;

    /// Check if a model is loaded
    fn is_model_loaded(&self) -> bool;
}

/// Mock inference engine for testing
pub struct MockInferenceEngine {
    pub model_loaded: bool,
}

impl MockInferenceEngine {
    /// Create a new mock inference engine
    pub fn new() -> Self {
        Self {
            model_loaded: false,
        }
    }
}

impl Default for MockInferenceEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl InferenceEngine for MockInferenceEngine {
    fn initialize(&mut self) -> InferenceResult<()> {
        Ok(())
    }

    fn load_model(&mut self, _model_path: &str) -> InferenceResult<()> {
        self.model_loaded = true;
        Ok(())
    }

    async fn generate(&mut self, prompt: &str) -> InferenceResult<String> {
        if !self.model_loaded {
            return Err(InferenceError::ModelNotLoaded(
                "No model loaded".to_string(),
            ));
        }

        // Return a mock response based on the prompt
        Ok(format!("Mock response to: {prompt}"))
    }

    fn set_parameters(&mut self, _temperature: f32, _max_tokens: usize) -> InferenceResult<()> {
        Ok(())
    }

    fn is_model_loaded(&self) -> bool {
        self.model_loaded
    }
}
