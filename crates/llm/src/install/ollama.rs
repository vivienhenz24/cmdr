//! Ollama installer module
//!
//! This module handles the installation and management of Ollama
//! for local LLM inference.

use super::{InstallError, InstallResult, InstallStatus};
use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Ollama installer that handles installation and management
pub struct OllamaInstaller {
    binary_path: Option<PathBuf>,
}

impl OllamaInstaller {
    /// Create a new Ollama installer
    pub fn new() -> Self {
        Self {
            binary_path: Self::find_ollama_binary(),
        }
    }

    /// Check if Ollama is already installed
    pub fn is_installed(&self) -> bool {
        self.binary_path.is_some() && self.test_ollama()
    }

    /// Get the installation status
    pub fn status(&self) -> InstallResult<InstallStatus> {
        if self.is_installed() {
            Ok(InstallStatus::Installed)
        } else {
            Ok(InstallStatus::NotInstalled)
        }
    }

    /// Install Ollama
    pub fn install(&mut self) -> InstallResult<()> {
        if self.is_installed() {
            println!("Ollama is already installed");
            return Ok(());
        }

        println!("Installing Ollama...");

        // Use the official Ollama install script
        let install_script = "https://ollama.ai/install.sh";

        let output = Command::new("curl")
            .args(["-fsSL", install_script])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| {
                InstallError::OllamaInstall(format!("Failed to download install script: {}", e))
            })?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(InstallError::OllamaInstall(format!(
                "Failed to download install script: {}",
                error
            )));
        }

        // Execute the install script
        let output = Command::new("sh")
            .arg("-c")
            .arg("curl -fsSL https://ollama.ai/install.sh | sh")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| {
                InstallError::OllamaInstall(format!("Failed to execute install script: {}", e))
            })?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(InstallError::OllamaInstall(format!(
                "Installation failed: {}",
                error
            )));
        }

        // Update binary path after installation
        self.binary_path = Self::find_ollama_binary();

        if self.binary_path.is_none() {
            return Err(InstallError::OllamaInstall(
                "Ollama installed but binary not found in PATH".to_string(),
            ));
        }

        // Start Ollama service
        self.start_service()?;

        Ok(())
    }

    /// Start the Ollama service
    pub fn start_service(&self) -> InstallResult<()> {
        if let Some(binary_path) = &self.binary_path {
            let _output = Command::new(binary_path)
                .arg("serve")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .map_err(|e| {
                    InstallError::OllamaInstall(format!("Failed to start Ollama service: {}", e))
                })?;

            // Give it a moment to start
            std::thread::sleep(std::time::Duration::from_secs(2));

            Ok(())
        } else {
            Err(InstallError::OllamaInstall(
                "Ollama binary not found".to_string(),
            ))
        }
    }

    /// Get the path to the Ollama binary
    pub fn binary_path(&self) -> Option<PathBuf> {
        self.binary_path.clone()
    }

    /// Test if Ollama is working
    fn test_ollama(&self) -> bool {
        if let Some(binary_path) = &self.binary_path {
            let _output = Command::new(binary_path)
                .arg("--version")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .output();

            _output.map(|o| o.status.success()).unwrap_or(false)
        } else {
            false
        }
    }

    /// Find the Ollama binary in the system PATH
    fn find_ollama_binary() -> Option<PathBuf> {
        // Check common installation paths
        let common_paths = [
            "/usr/local/bin/ollama",
            "/opt/homebrew/bin/ollama",
            "/usr/bin/ollama",
        ];

        for path in &common_paths {
            if std::path::Path::new(path).exists() {
                return Some(PathBuf::from(path));
            }
        }

        // Check PATH environment variable
        if let Ok(path_var) = env::var("PATH") {
            for path in path_var.split(':') {
                let ollama_path = PathBuf::from(path).join("ollama");
                if ollama_path.exists() {
                    return Some(ollama_path);
                }
            }
        }

        None
    }

    /// Get Ollama version
    pub fn version(&self) -> InstallResult<String> {
        if let Some(binary_path) = &self.binary_path {
            let output = Command::new(binary_path)
                .arg("--version")
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .map_err(|e| {
                    InstallError::OllamaInstall(format!("Failed to get version: {}", e))
                })?;

            if output.status.success() {
                let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                Ok(version)
            } else {
                Err(InstallError::OllamaInstall(
                    "Failed to get Ollama version".to_string(),
                ))
            }
        } else {
            Err(InstallError::OllamaInstall(
                "Ollama not installed".to_string(),
            ))
        }
    }
}

impl Default for OllamaInstaller {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_installer_creation() {
        // This test will pass regardless of whether Ollama is installed
    }
}
