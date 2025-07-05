# cmdr LLM Installation Framework

This directory contains the installation framework for cmdr's LLM dependencies. The framework automatically installs and configures Ollama and the Llama 3.2 3B model for local LLM inference.

## Overview

The installation framework consists of three main components:

1. **OllamaInstaller** - Handles installation and management of Ollama
2. **ModelInstaller** - Manages downloading and setup of Llama 3.2 3B
3. **SystemChecker** - Validates system requirements

## Components

### OllamaInstaller (`ollama.rs`)

The `OllamaInstaller` handles:
- Detecting if Ollama is already installed
- Installing Ollama using the official install script
- Starting the Ollama service
- Version checking and binary path management

**Features:**
- Automatic detection in common installation paths
- PATH environment variable scanning
- Service management
- Version verification

### ModelInstaller (`model.rs`)

The `ModelInstaller` manages:
- Downloading Llama 3.2 3B via Ollama
- Model verification and testing
- Model information retrieval
- Model removal capabilities

**Features:**
- Automatic model download via `ollama pull`
- Progress tracking during download
- Model verification after installation
- Test generation to validate functionality

### SystemChecker (`system.rs`)

The `SystemChecker` validates:
- Platform compatibility (macOS, Linux)
- Architecture support (x86_64, aarch64, arm64)
- Memory requirements (minimum 4GB)
- Disk space availability (minimum 3GB)
- Network connectivity

**Features:**
- Cross-platform system information gathering
- Memory and disk space checking
- Network connectivity testing
- Detailed system information reporting

## Usage

### CLI Commands

```bash
# Install LLM dependencies
cmdr install

# Install with custom options
cmdr install --skip-checks --force

# Check installation status
cmdr status
```

### Programmatic Usage

```rust
use llm::LlmInstaller;

// Create installer
let mut installer = LlmInstaller::new();

// Check system requirements
if installer.check_system()? {
    // Install Ollama and model
    installer.install_all()?;
}
```

## Installation Process

1. **System Check**: Validates platform, architecture, memory, disk space, and network
2. **Ollama Installation**: Downloads and installs Ollama using the official script
3. **Service Start**: Starts the Ollama service in the background
4. **Model Download**: Downloads Llama 3.2 3B (~2.1GB) via Ollama
5. **Verification**: Tests the model with a simple prompt

## System Requirements

- **Platform**: macOS or Linux
- **Architecture**: x86_64, aarch64, or arm64
- **Memory**: Minimum 4GB RAM
- **Disk Space**: Minimum 3GB available
- **Network**: Internet connection for downloads

## Error Handling

The framework provides comprehensive error handling with specific error types:

- `InstallError::SystemCheck` - System requirement failures
- `InstallError::OllamaInstall` - Ollama installation issues
- `InstallError::ModelDownload` - Model download problems
- `InstallError::ModelVerification` - Model verification failures
- `InstallError::Configuration` - Configuration issues

## Integration with Homebrew

When users install cmdr via Homebrew, the installation framework automatically runs during the `post_install` phase:

```ruby
def post_install
  system "#{bin}/cmdr", "install", "--skip-checks"
rescue => e
  opoo "Failed to install LLM dependencies: #{e.message}"
  opoo "You can manually install them later with: #{bin}/cmdr install"
end
```

This ensures that users get a fully functional cmdr installation with local LLM capabilities out of the box.

## Troubleshooting

### Common Issues

1. **Insufficient Memory**: Ensure at least 4GB RAM is available
2. **Network Issues**: Check internet connectivity and firewall settings
3. **Permission Errors**: Ensure write access to installation directories
4. **Model Download Failures**: Check disk space and network stability

### Manual Installation

If automatic installation fails, users can manually install dependencies:

```bash
# Install Ollama manually
curl -fsSL https://ollama.ai/install.sh | sh

# Install Llama 3.2 3B manually
ollama pull llama3.2:3b

# Verify installation
cmdr status
```

## Development

### Adding New Models

To add support for additional models, modify the `ModelInstaller`:

```rust
let installer = ModelInstaller::new()
    .with_model_name("llama3.2:7b".to_string());
```

### Custom System Requirements

To modify system requirements, update the `SystemRequirements` struct:

```rust
let requirements = SystemRequirements {
    min_memory_gb: 8,  // Increase memory requirement
    min_disk_space_gb: 5,  // Increase disk space requirement
    // ... other fields
};
```

## Testing

Run the installation framework tests:

```bash
cargo test -p llm
```

The tests include:
- Installer creation and configuration
- System requirement validation
- Error handling scenarios
- Cross-platform compatibility checks 