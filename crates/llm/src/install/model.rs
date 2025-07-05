//! Model installer module
//! 
//! This module handles the installation and management of
//! Llama 3.2 3B model via Ollama.

use super::{InstallError, InstallResult, InstallStatus};
use std::process::{Command, Stdio};

/// Model installer that handles downloading and managing models
pub struct ModelInstaller {
    model_name: String,
    ollama_binary: Option<String>,
}

impl ModelInstaller {
    /// Create a new model installer
    pub fn new() -> Self {
        Self {
            model_name: "llama3.2:3b".to_string(),
            ollama_binary: Self::find_ollama_binary(),
        }
    }

    /// Check if the model is already installed
    pub fn is_installed(&self) -> bool {
        if let Some(ollama_path) = &self.ollama_binary {
            let output = Command::new(ollama_path)
                .args(["list"])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output();

            if let Ok(output) = output {
                if output.status.success() {
                    let output_str = String::from_utf8_lossy(&output.stdout);
                    return output_str.contains(&self.model_name);
                }
            }
        }
        false
    }

    /// Get the installation status
    pub fn status(&self) -> InstallResult<InstallStatus> {
        if self.is_installed() {
            Ok(InstallStatus::Installed)
        } else {
            Ok(InstallStatus::NotInstalled)
        }
    }

    /// Install the Llama 3.2 3B model
    pub fn install(&self) -> InstallResult<()> {
        if self.is_installed() {
            println!("Model {} is already installed", self.model_name);
            return Ok(());
        }

        let ollama_path = self.ollama_binary.as_ref()
            .ok_or_else(|| InstallError::ModelDownload("Ollama not found".to_string()))?;

        println!("Installing {} model (this may take several minutes)...", self.model_name);
        println!("Download size: ~2.1 GB");

        // Start the model download
        let mut child = Command::new(ollama_path)
            .args(["pull", &self.model_name])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| InstallError::ModelDownload(format!("Failed to start model download: {}", e)))?;

        // Wait for the download to complete
        let status = child.wait()
            .map_err(|e| InstallError::ModelDownload(format!("Failed to wait for download: {}", e)))?;

        if !status.success() {
            return Err(InstallError::ModelDownload("Model download failed".to_string()));
        }

        // Verify the installation
        if !self.is_installed() {
            return Err(InstallError::ModelVerification("Model download completed but verification failed".to_string()));
        }

        println!("✓ Model {} installed successfully", self.model_name);
        Ok(())
    }

    /// Get the model name
    pub fn model_name(&self) -> String {
        self.model_name.clone()
    }

    /// Set a custom model name
    pub fn with_model_name(mut self, name: String) -> Self {
        self.model_name = name;
        self
    }

    /// Test the model with a simple prompt
    pub fn test_model(&self) -> InstallResult<String> {
        let ollama_path = self.ollama_binary.as_ref()
            .ok_or_else(|| InstallError::ModelVerification("Ollama not found".to_string()))?;

        if !self.is_installed() {
            return Err(InstallError::ModelVerification("Model not installed".to_string()));
        }

        let output = Command::new(ollama_path)
            .args(["run", &self.model_name, "Hello, this is a test."])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| InstallError::ModelVerification(format!("Failed to test model: {}", e)))?;

        if output.status.success() {
            let response = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Ok(response)
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(InstallError::ModelVerification(format!("Model test failed: {}", error)))
        }
    }

    /// Get model information
    pub fn model_info(&self) -> InstallResult<String> {
        let ollama_path = self.ollama_binary.as_ref()
            .ok_or_else(|| InstallError::ModelVerification("Ollama not found".to_string()))?;

        let output = Command::new(ollama_path)
            .args(["show", &self.model_name])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| InstallError::ModelVerification(format!("Failed to get model info: {}", e)))?;

        if output.status.success() {
            let info = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Ok(info)
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(InstallError::ModelVerification(format!("Failed to get model info: {}", error)))
        }
    }

    /// Remove the model
    pub fn remove(&self) -> InstallResult<()> {
        if !self.is_installed() {
            println!("Model {} is not installed", self.model_name);
            return Ok(());
        }

        let ollama_path = self.ollama_binary.as_ref()
            .ok_or_else(|| InstallError::ModelDownload("Ollama not found".to_string()))?;

        println!("Removing model {}...", self.model_name);

        let output = Command::new(ollama_path)
            .args(["rm", &self.model_name])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| InstallError::ModelDownload(format!("Failed to remove model: {}", e)))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(InstallError::ModelDownload(format!("Failed to remove model: {}", error)));
        }

        println!("✓ Model {} removed successfully", self.model_name);
        Ok(())
    }

    /// Find the Ollama binary
    fn find_ollama_binary() -> Option<String> {
        // Check common installation paths
        let common_paths = [
            "/usr/local/bin/ollama",
            "/opt/homebrew/bin/ollama",
            "/usr/bin/ollama",
        ];

        for path in &common_paths {
            if std::path::Path::new(path).exists() {
                return Some(path.to_string());
            }
        }

        // Check PATH environment variable
        if let Ok(path_var) = std::env::var("PATH") {
            for path in path_var.split(':') {
                let ollama_path = std::path::Path::new(path).join("ollama");
                if ollama_path.exists() {
                    return Some(ollama_path.to_string_lossy().to_string());
                }
            }
        }

        None
    }

    /// Get available models
    pub fn list_models(&self) -> InstallResult<String> {
        let ollama_path = self.ollama_binary.as_ref()
            .ok_or_else(|| InstallError::ModelVerification("Ollama not found".to_string()))?;

        let output = Command::new(ollama_path)
            .args(["list"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| InstallError::ModelVerification(format!("Failed to list models: {}", e)))?;

        if output.status.success() {
            let models = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Ok(models)
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(InstallError::ModelVerification(format!("Failed to list models: {}", error)))
        }
    }
}

impl Default for ModelInstaller {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_installer_creation() {
        let installer = ModelInstaller::new();
        assert_eq!(installer.model_name(), "llama3.2:3b");
    }

    #[test]
    fn test_custom_model_name() {
        let installer = ModelInstaller::new().with_model_name("test:model".to_string());
        assert_eq!(installer.model_name(), "test:model");
    }
} 