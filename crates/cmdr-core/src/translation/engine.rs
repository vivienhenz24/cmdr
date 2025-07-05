//! Translation engine implementation
//!
//! This module implements the core translation logic for converting
//! natural language to shell commands.

use super::TranslationResult;
use crate::inference::InferenceEngine;
use crate::types::{NaturalLanguageRequest, ShellCommand};

/// Translation engine for converting natural language to shell commands
pub struct TranslationEngine<E> {
    inference_engine: E,
    prompt_template: String,
}

impl<E: InferenceEngine> TranslationEngine<E> {
    /// Create a new translation engine
    pub fn new(inference_engine: E) -> Self {
        Self {
            inference_engine,
            prompt_template: Self::default_prompt_template(),
        }
    }

    /// Set a custom prompt template
    pub fn with_prompt_template(mut self, template: String) -> Self {
        self.prompt_template = template;
        self
    }

    /// Translate a natural language request to a shell command
    pub async fn translate(
        &mut self,
        request: NaturalLanguageRequest,
    ) -> TranslationResult<ShellCommand> {
        let prompt = self.build_prompt(&request);
        let response = self.inference_engine.generate(&prompt).await?;

        // TODO: Parse the response to extract the command and confidence
        let command = self.parse_response(&response)?;

        Ok(command)
    }

    /// Build the prompt for the inference engine
    fn build_prompt(&self, request: &NaturalLanguageRequest) -> String {
        format!(
            "{}\n\nUser request: {}\n\nShell command:",
            self.prompt_template, request.text
        )
    }

    /// Parse the inference engine response
    fn parse_response(&self, response: &str) -> TranslationResult<ShellCommand> {
        // TODO: Implement proper response parsing
        // For now, just return the response as-is
        Ok(ShellCommand {
            command: response.trim().to_string(),
            explanation: None,
            confidence: 0.8,
        })
    }

    /// Get the default prompt template
    fn default_prompt_template() -> String {
        r#"You are a helpful assistant that translates natural language requests into shell commands.

Your task is to:
1. Understand the user's intent
2. Generate the appropriate shell command
3. Provide a brief explanation if needed

Rules:
- Only output the shell command, no additional text
- Use standard Unix/Linux commands
- Be safe and avoid destructive operations
- Prefer simple, readable commands"#.to_string()
    }
}
