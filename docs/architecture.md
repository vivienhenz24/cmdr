# cmdr Architecture

This document provides a high-level overview of the cmdr architecture and design decisions.

## Overview

cmdr is designed as a modular, extensible system for translating natural language to shell commands using local LLM inference. The architecture follows Rust best practices with clear separation of concerns and trait-based abstractions.

## Core Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   cmdr-cli      │    │   cmdr-core     │    │   llm           │
│   (CLI Binary)  │◄──►│   (Core Logic)  │◄──►│   (LLM Bindings)│
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│ cmdr-config     │    │ cmdr-plugin     │    │ External LLMs   │
│ (Configuration) │    │ (Plugin System) │    │ (llama.cpp, etc)│
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## Crate Responsibilities

### cmdr-cli
- **Purpose**: Command-line interface and user interaction
- **Responsibilities**:
  - Parse command-line arguments
  - Provide REPL interface
  - Handle user input/output
  - Coordinate between components

### cmdr-core
- **Purpose**: Core business logic and abstractions
- **Responsibilities**:
  - Natural language translation
  - Shell command execution
  - Inference engine abstractions
  - Common types and error handling

### llm
- **Purpose**: LLM inference engine bindings
- **Responsibilities**:
  - llama.cpp FFI bindings
  - Model loading and management
  - Inference execution
  - Memory management

### cmdr-config
- **Purpose**: Configuration management
- **Responsibilities**:
  - Load/save configuration files
  - Validate configuration
  - Provide default settings
  - Configuration schema management

### cmdr-plugin
- **Purpose**: Plugin system for extensibility
- **Responsibilities**:
  - Dynamic library loading
  - Plugin lifecycle management
  - Plugin interface definitions
  - Plugin discovery and registration

## Key Design Patterns

### Trait-Based Abstractions

The system uses Rust traits to provide flexible, testable abstractions:

```rust
#[async_trait]
pub trait InferenceEngine: Send + Sync {
    async fn generate(&mut self, prompt: &str) -> InferenceResult<String>;
    fn load_model(&mut self, model_path: &str) -> InferenceResult<()>;
    // ...
}
```

### Error Handling

Comprehensive error handling using `thiserror`:

```rust
#[derive(Debug, thiserror::Error)]
pub enum TranslationError {
    #[error("Failed to generate translation: {0}")]
    Generation(String),
    #[error("Inference engine error: {0}")]
    Inference(#[from] InferenceError),
}
```

### Async/Await Support

Full async/await support for non-blocking operations:

```rust
pub async fn translate(&mut self, request: NaturalLanguageRequest) -> TranslationResult<ShellCommand> {
    let response = self.inference_engine.generate(&prompt).await?;
    // ...
}
```

## Data Flow

1. **User Input**: Natural language request via CLI
2. **Translation**: Core engine translates to shell command
3. **Validation**: Command safety and syntax validation
4. **Execution**: Shell command execution (if enabled)
5. **Output**: Results displayed to user

## Extension Points

### Inference Engines
New LLM backends can be added by implementing the `InferenceEngine` trait.

### Plugins
Custom functionality can be added through the plugin system.

### Configuration
New configuration options can be added to the configuration schema.

## Security Considerations

- **Command Validation**: All commands are validated for safety
- **Sandboxing**: Commands are executed in the user's shell environment
- **No Network**: Local-only inference by default
- **Permission Model**: Respects existing file permissions

## Performance Considerations

- **Async Operations**: Non-blocking I/O for better responsiveness
- **Memory Management**: Efficient memory usage for large models
- **Caching**: Model and response caching where appropriate
- **Parallelization**: Multi-threaded inference support

## Future Extensibility

The architecture is designed to support:

- Multiple inference backends
- Custom prompt templates
- Advanced shell integration
- Cloud-based inference (optional)
- Advanced plugin capabilities
- GUI interfaces 