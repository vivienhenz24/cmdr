//! Prompt template management
//! 
//! This module handles prompt templates for different types of requests.

use std::collections::HashMap;

/// A template for generating prompts
pub struct PromptTemplate {
    template: String,
    variables: HashMap<String, String>,
}

impl PromptTemplate {
    /// Create a new prompt template
    pub fn new(template: String) -> Self {
        Self {
            template,
            variables: HashMap::new(),
        }
    }
    
    /// Set a variable in the template
    pub fn set_variable(&mut self, name: &str, value: String) {
        self.variables.insert(name.to_string(), value);
    }
    
    /// Render the template with the current variables
    pub fn render(&self) -> String {
        let mut result = self.template.clone();
        
        for (name, value) in &self.variables {
            let placeholder = format!("{{{{{}}}}}", name);
            result = result.replace(&placeholder, value);
        }
        
        result
    }
}

impl Default for PromptTemplate {
    fn default() -> Self {
        Self::new(Self::default_template())
    }
}

impl PromptTemplate {
    /// Get the default template
    fn default_template() -> String {
        r#"You are a helpful assistant that translates natural language requests into shell commands.

Your task is to:
1. Understand the user's intent
2. Generate the appropriate shell command
3. Provide a brief explanation if needed

Rules:
- Only output the shell command, no additional text
- Use standard Unix/Linux commands
- Be safe and avoid destructive operations
- Prefer simple, readable commands

User request: {request}

Shell command:"#.to_string()
    }
} 