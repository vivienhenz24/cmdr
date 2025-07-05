# cmdr Documentation

Welcome to the cmdr documentation! This directory contains comprehensive documentation for the cmdr project.

## Documentation Structure

### User Documentation
- **[User Guide](user-guide.md)** - Complete guide for using cmdr
- **[Homebrew Setup](homebrew-setup.md)** - Installation instructions for Homebrew users

### Developer Documentation
- **[Developer Guide](developer-guide.md)** - Guide for contributors and developers
- **[Architecture](architecture.md)** - High-level architecture overview
- **[Inference Backend](inference-backend.md)** - Details about LLM inference engines

### API Documentation
- **[API Reference](api/)** - Detailed API documentation for all crates

## Quick Start

1. **Installation**: See [Homebrew Setup](homebrew-setup.md) for installation instructions
2. **Basic Usage**: See [User Guide](user-guide.md) for getting started
3. **Development**: See [Developer Guide](developer-guide.md) for contributing

## Project Structure

```
cmdr/
├── crates/           # Rust crates
│   ├── cmdr-cli/     # Command-line interface
│   ├── cmdr-core/    # Core library
│   ├── llm/          # llama.cpp bindings
│   ├── cmdr-config/  # Configuration management
│   └── cmdr-plugin/  # Plugin system
├── docs/             # This directory
├── examples/         # Example code
└── tools/            # Development tools
```

## Contributing

Please read the [Contributing Guide](../CONTRIBUTING.md) and [Developer Guide](developer-guide.md) before contributing to the project. 