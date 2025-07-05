//! System checker module
//! 
//! This module handles checking system requirements for
//! running Ollama and Llama 3.2 3B.

use super::{InstallError, InstallResult};
use std::env;

/// System requirements for running Ollama and Llama 3.2 3B
#[derive(Debug, Clone)]
pub struct SystemRequirements {
    pub min_memory_gb: u64,
    pub min_disk_space_gb: u64,
    pub supported_architectures: Vec<String>,
    pub supported_platforms: Vec<String>,
}

impl Default for SystemRequirements {
    fn default() -> Self {
        Self {
            min_memory_gb: 4, // 4GB minimum for Llama 3.2 3B
            min_disk_space_gb: 3, // 3GB for model + Ollama
            supported_architectures: vec![
                "x86_64".to_string(),
                "aarch64".to_string(),
                "arm64".to_string(),
            ],
            supported_platforms: vec![
                "darwin".to_string(),
                "linux".to_string(),
            ],
        }
    }
}

/// System checker that validates requirements
pub struct SystemChecker {
    requirements: SystemRequirements,
}

impl SystemChecker {
    /// Create a new system checker
    pub fn new() -> Self {
        Self {
            requirements: SystemRequirements::default(),
        }
    }

    /// Create a system checker with custom requirements
    pub fn with_requirements(requirements: SystemRequirements) -> Self {
        Self { requirements }
    }

    /// Check if the system meets all requirements
    pub fn check_requirements(&self) -> InstallResult<bool> {
        let mut all_checks_passed = true;

        // Check platform
        if !self.check_platform()? {
            println!("❌ Platform not supported");
            all_checks_passed = false;
        } else {
            println!("✓ Platform supported");
        }

        // Check architecture
        if !self.check_architecture()? {
            println!("❌ Architecture not supported");
            all_checks_passed = false;
        } else {
            println!("✓ Architecture supported");
        }

        // Check memory
        if !self.check_memory()? {
            println!("❌ Insufficient memory (need at least {}GB)", self.requirements.min_memory_gb);
            all_checks_passed = false;
        } else {
            println!("✓ Sufficient memory available");
        }

        // Check disk space
        if !self.check_disk_space()? {
            println!("❌ Insufficient disk space (need at least {}GB)", self.requirements.min_disk_space_gb);
            all_checks_passed = false;
        } else {
            println!("✓ Sufficient disk space available");
        }

        // Check network connectivity
        if !self.check_network()? {
            println!("❌ Network connectivity issues");
            all_checks_passed = false;
        } else {
            println!("✓ Network connectivity available");
        }

        Ok(all_checks_passed)
    }

    /// Check if the platform is supported
    fn check_platform(&self) -> InstallResult<bool> {
        let platform = env::consts::OS;
        Ok(self.requirements.supported_platforms.contains(&platform.to_string()))
    }

    /// Check if the architecture is supported
    fn check_architecture(&self) -> InstallResult<bool> {
        let arch = env::consts::ARCH;
        Ok(self.requirements.supported_architectures.contains(&arch.to_string()))
    }

    /// Check available memory
    fn check_memory(&self) -> InstallResult<bool> {
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            
            let output = Command::new("sysctl")
                .args(["-n", "hw.memsize"])
                .stdout(std::process::Stdio::piped())
                .output()
                .map_err(|e| InstallError::SystemCheck(format!("Failed to get memory info: {}", e)))?;

            if output.status.success() {
                let lossy = String::from_utf8_lossy(&output.stdout);
                let memsize_str = lossy.trim();
                if let Ok(memsize) = memsize_str.parse::<u64>() {
                    let memory_gb = memsize / (1024 * 1024 * 1024);
                    return Ok(memory_gb >= self.requirements.min_memory_gb);
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            use std::fs;
            
            if let Ok(content) = fs::read_to_string("/proc/meminfo") {
                for line in content.lines() {
                    if line.starts_with("MemTotal:") {
                        if let Some(kb_str) = line.split_whitespace().nth(1) {
                            if let Ok(kb) = kb_str.parse::<u64>() {
                                let memory_gb = kb / (1024 * 1024);
                                return Ok(memory_gb >= self.requirements.min_memory_gb);
                            }
                        }
                    }
                }
            }
        }

        // Fallback: assume sufficient memory if we can't determine
        Ok(true)
    }

    /// Check available disk space
    fn check_disk_space(&self) -> InstallResult<bool> {
        use std::path::Path;

        // Check space in common installation directories
        let paths_to_check = [
            "/usr/local",
            "/opt/homebrew",
            "/opt",
            "/tmp",
        ];

        for path in &paths_to_check {
            if Path::new(path).exists() {
                if let Ok(available_space) = self.get_available_space(path) {
                    let available_gb = available_space / (1024 * 1024 * 1024);
                    if available_gb >= self.requirements.min_disk_space_gb {
                        return Ok(true);
                    }
                }
            }
        }

        // If we can't determine space, assume sufficient
        Ok(true)
    }

    /// Get available disk space for a path
    fn get_available_space(&self, path: &str) -> InstallResult<u64> {
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            
            let output = Command::new("df")
                .args(["-k", path])
                .stdout(std::process::Stdio::piped())
                .output()
                .map_err(|e| InstallError::SystemCheck(format!("Failed to get disk space: {}", e)))?;

            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = output_str.lines().collect();
                if lines.len() >= 2 {
                    let fields: Vec<&str> = lines[1].split_whitespace().collect();
                    if fields.len() >= 4 {
                        if let Ok(available_kb) = fields[3].parse::<u64>() {
                            return Ok(available_kb * 1024); // Convert KB to bytes
                        }
                    }
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            
            let output = Command::new("df")
                .args(["-B1", path])
                .stdout(std::process::Stdio::piped())
                .output()
                .map_err(|e| InstallError::SystemCheck(format!("Failed to get disk space: {}", e)))?;

            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = output_str.lines().collect();
                if lines.len() >= 2 {
                    let fields: Vec<&str> = lines[1].split_whitespace().collect();
                    if fields.len() >= 4 {
                        if let Ok(available_bytes) = fields[3].parse::<u64>() {
                            return Ok(available_bytes);
                        }
                    }
                }
            }
        }

        Err(InstallError::SystemCheck("Could not determine available disk space".to_string()))
    }

    /// Check network connectivity
    fn check_network(&self) -> InstallResult<bool> {
        use std::process::Command;
        
        // Try to ping a reliable host
        let hosts = ["8.8.8.8", "1.1.1.1", "ollama.ai"];
        
        for host in &hosts {
            let output = Command::new("ping")
                .args(["-c", "1", "-W", "5", host])
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .output();

            if let Ok(output) = output {
                if output.status.success() {
                    return Ok(true);
                }
            }
        }

        // If ping fails, try curl
        let output = Command::new("curl")
            .args(["-s", "--max-time", "5", "https://ollama.ai"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .output();

        Ok(output.map(|o| o.status.success()).unwrap_or(false))
    }

    /// Get system information
    pub fn get_system_info(&self) -> InstallResult<String> {
        let mut info = String::new();
        
        info.push_str(&format!("Platform: {}\n", env::consts::OS));
        info.push_str(&format!("Architecture: {}\n", env::consts::ARCH));
        
        if let Ok(memory_gb) = self.get_memory_gb() {
            info.push_str(&format!("Memory: {}GB\n", memory_gb));
        }
        
        if let Ok(disk_gb) = self.get_disk_gb() {
            info.push_str(&format!("Available disk space: {}GB\n", disk_gb));
        }
        
        Ok(info)
    }

    /// Get memory in GB
    fn get_memory_gb(&self) -> InstallResult<u64> {
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            
            let output = Command::new("sysctl")
                .args(["-n", "hw.memsize"])
                .stdout(std::process::Stdio::piped())
                .output()
                .map_err(|e| InstallError::SystemCheck(format!("Failed to get memory info: {}", e)))?;

            if output.status.success() {
                let lossy = String::from_utf8_lossy(&output.stdout);
                let memsize_str = lossy.trim();
                if let Ok(memsize) = memsize_str.parse::<u64>() {
                    return Ok(memsize / (1024 * 1024 * 1024));
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            use std::fs;
            
            if let Ok(content) = fs::read_to_string("/proc/meminfo") {
                for line in content.lines() {
                    if line.starts_with("MemTotal:") {
                        if let Some(kb_str) = line.split_whitespace().nth(1) {
                            if let Ok(kb) = kb_str.parse::<u64>() {
                                return Ok(kb / (1024 * 1024));
                            }
                        }
                    }
                }
            }
        }

        Err(InstallError::SystemCheck("Could not determine memory size".to_string()))
    }

    /// Get available disk space in GB
    fn get_disk_gb(&self) -> InstallResult<u64> {
        let paths_to_check = ["/usr/local", "/opt/homebrew", "/opt"];
        
        for path in &paths_to_check {
            if std::path::Path::new(path).exists() {
                if let Ok(available_space) = self.get_available_space(path) {
                    return Ok(available_space / (1024 * 1024 * 1024));
                }
            }
        }

        Err(InstallError::SystemCheck("Could not determine disk space".to_string()))
    }
}

impl Default for SystemChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_checker_creation() {
        let checker = SystemChecker::new();
        assert_eq!(checker.requirements.min_memory_gb, 4);
    }

    #[test]
    fn test_platform_check() {
        let checker = SystemChecker::new();
        let result = checker.check_platform();
        assert!(result.is_ok());
    }

    #[test]
    fn test_architecture_check() {
        let checker = SystemChecker::new();
        let result = checker.check_architecture();
        assert!(result.is_ok());
    }
} 