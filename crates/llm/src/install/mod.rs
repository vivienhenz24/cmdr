//! Installation framework for cmdr LLM dependencies
//! 
//! This module handles the automatic installation and setup of
//! Ollama and Llama 3.2 3B model for local LLM inference.

mod ollama;
mod model;
mod system;

pub use ollama::OllamaInstaller;
pub use model::ModelInstaller;
pub use system::SystemChecker;

use std::path::PathBuf;
use thiserror::Error;

/// Error type for installation operations
#[derive(Debug, Error)]
pub enum InstallError {
    #[error("System check failed: {0}")]
    SystemCheck(String),
    #[error("Ollama installation failed: {0}")]
    OllamaInstall(String),
    #[error("Model download failed: {0}")]
    ModelDownload(String),
    #[error("Model verification failed: {0}")]
    ModelVerification(String),
    #[error("Configuration failed: {0}")]
    Configuration(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type for installation operations
pub type InstallResult<T> = Result<T, InstallError>;

/// Installation status
#[derive(Debug, Clone, PartialEq)]
pub enum InstallStatus {
    NotInstalled,
    Installing,
    Installed,
    Failed(String),
}

/// Main installer that coordinates the entire installation process
pub struct LlmInstaller {
    ollama_installer: OllamaInstaller,
    model_installer: ModelInstaller,
    system_checker: SystemChecker,
}

impl LlmInstaller {
    /// Create a new LLM installer
    pub fn new() -> Self {
        Self {
            ollama_installer: OllamaInstaller::new(),
            model_installer: ModelInstaller::new(),
            system_checker: SystemChecker::new(),
        }
    }

    /// Check if the system meets requirements
    pub fn check_system(&self) -> InstallResult<bool> {
        self.system_checker.check_requirements()
    }

    /// Get the installation status of Ollama
    pub fn ollama_status(&self) -> InstallResult<InstallStatus> {
        self.ollama_installer.status()
    }

    /// Get the installation status of Llama 3.2 3B
    pub fn model_status(&self) -> InstallResult<InstallStatus> {
        self.model_installer.status()
    }

    /// Install Ollama if not already installed
    pub fn install_ollama(&mut self) -> InstallResult<()> {
        self.ollama_installer.install()
    }

    /// Install Llama 3.2 3B model
    pub fn install_model(&mut self) -> InstallResult<()> {
        self.model_installer.install()
    }

    /// Perform a complete installation (Ollama + model)
    pub fn install_all(&mut self) -> InstallResult<()> {
        println!("Starting cmdr LLM installation...");
        
        // Check system requirements
        println!("Checking system requirements...");
        if !self.check_system()? {
            return Err(InstallError::SystemCheck("System requirements not met".to_string()));
        }
        println!("✓ System requirements met");

        // Install Ollama
        println!("Installing Ollama...");
        self.install_ollama()?;
        println!("✓ Ollama installed successfully");

        // Install Llama 3.2 3B
        println!("Installing Llama 3.2 3B model...");
        self.install_model()?;
        println!("✓ Llama 3.2 3B model installed successfully");

        println!("🎉 cmdr LLM installation completed successfully!");
        Ok(())
    }

    /// Get the path to the Ollama binary
    pub fn ollama_path(&self) -> Option<PathBuf> {
        self.ollama_installer.binary_path()
    }

    /// Get the model name for Llama 3.2 3B
    pub fn model_name(&self) -> String {
        self.model_installer.model_name()
    }

    /// Get the system checker
    pub fn system_checker(&self) -> &SystemChecker {
        &self.system_checker
    }

    /// Get the Ollama installer
    pub fn ollama_installer(&self) -> &OllamaInstaller {
        &self.ollama_installer
    }
}

impl Default for LlmInstaller {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_installer_creation() {
        let installer = LlmInstaller::new();
        assert_eq!(installer.model_name(), "llama3.2:3b");
    }
} 