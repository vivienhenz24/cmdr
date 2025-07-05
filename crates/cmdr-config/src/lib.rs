//! Configuration management for cmdr
//! 
//! This crate handles loading, parsing, and managing configuration
//! for the cmdr application.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main configuration structure for cmdr
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// Inference engine configuration
    pub inference: InferenceConfig,
    /// Shell execution configuration
    pub shell: ShellConfig,
    /// UI/REPL configuration
    pub ui: UiConfig,
}

/// Inference engine configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceConfig {
    /// Path to the model file
    pub model_path: Option<PathBuf>,
    /// Number of threads to use for inference
    pub threads: Option<u32>,
    /// Context size for the model
    pub context_size: Option<u32>,
    /// Temperature for generation
    pub temperature: Option<f32>,
}

/// Shell execution configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellConfig {
    /// Whether to execute commands automatically
    pub auto_execute: bool,
    /// Whether to show the translated command before execution
    pub show_translation: bool,
    /// Default shell to use
    pub default_shell: String,
}

/// UI/REPL configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// Prompt string for the REPL
    pub prompt: String,
    /// Whether to enable syntax highlighting
    pub syntax_highlighting: bool,
    /// History file path
    pub history_file: Option<PathBuf>,
}

impl Default for InferenceConfig {
    fn default() -> Self {
        Self {
            model_path: None,
            threads: Some(num_cpus::get() as u32),
            context_size: Some(2048),
            temperature: Some(0.7),
        }
    }
}

impl Default for ShellConfig {
    fn default() -> Self {
        Self {
            auto_execute: true,
            show_translation: true,
            default_shell: "bash".to_string(),
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            prompt: "[cmdr] ".to_string(),
            syntax_highlighting: true,
            history_file: Some(PathBuf::from("~/.cmdr_history")),
        }
    }
}

/// Load configuration from a file
pub fn load_config(path: &PathBuf) -> anyhow::Result<Config> {
    let content = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

/// Save configuration to a file
pub fn save_config(config: &Config, path: &PathBuf) -> anyhow::Result<()> {
    let content = toml::to_string_pretty(config)?;
    std::fs::write(path, content)?;
    Ok(())
} 