//! Basic usage example for cmdr
//! 
//! This example demonstrates how to use the cmdr core library
//! to translate natural language to shell commands.

use cmdr_core::{
    TranslationEngine, MockInferenceEngine, ShellExecutor,
    NaturalLanguageRequest, ShellCommand
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("cmdr Basic Usage Example");
    println!("=======================\n");
    
    // Initialize the components
    let mut inference_engine = MockInferenceEngine::new();
    inference_engine.initialize()?;
    inference_engine.load_model("mock-model")?;
    
    let mut translation_engine = TranslationEngine::new(inference_engine);
    let mut shell_executor = ShellExecutor::default();
    
    // Example natural language requests
    let requests = vec![
        "show me all Python files in the current directory",
        "create a backup of my documents folder",
        "find files modified in the last 24 hours",
    ];
    
    for request_text in requests {
        println!("Request: {}", request_text);
        
        let request = NaturalLanguageRequest {
            text: request_text.to_string(),
            context: None,
        };
        
        // Translate the request
        let command = translation_engine.translate(request).await?;
        println!("Translated command: {}", command.command);
        
        // Execute the command (disabled for safety in this example)
        shell_executor.set_auto_execute(false);
        let result = shell_executor.execute(&command)?;
        
        if result.success {
            println!("Output: {}", result.output);
        } else {
            println!("Error: {}", result.error.unwrap_or_else(|| "Unknown error".to_string()));
        }
        
        println!("---\n");
    }
    
    Ok(())
} 