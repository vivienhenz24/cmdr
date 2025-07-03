# Contributing to cmdr

Thank you for your interest in contributing to cmdr! This document outlines the development setup and contribution guidelines.

## Prerequisites

- **Rust toolchain**: Install Rust stable via [rustup](https://rustup.rs/)
- **C compiler**: Install clang (for FFI bindings to llama.cpp)
- **Git**: Latest version for version control

## Development Setup

1. Fork and clone the repository
2. Run `cargo check` to verify the build
3. Run `cargo test` to ensure all tests pass

## Coding Standards

- **Formatting**: Use `rustfmt` for consistent code formatting
- **Linting**: Run `cargo clippy` to catch common issues
- **Tests**: Write unit tests for new functionality
- **Documentation**: Add doc comments for public APIs

## Branch Policy

- Create feature branches from `master`
- Use descriptive branch names (e.g., `feature/llama-ffi-bindings`)
- Submit pull requests for review
- Ensure CI passes before merging

## Commit Style

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add llama.cpp FFI bindings
fix: resolve memory leak in token processing
docs: update inference backend documentation
test: add unit tests for model loading
```

## Pull Request Process

1. Ensure your code follows the coding standards
2. Update documentation if needed
3. Add tests for new functionality
4. Request review from maintainers
5. Address feedback and ensure CI passes

## Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct v2.0](https://www.contributor-covenant.org/version/2/0/code_of_conduct/). By participating, you are expected to uphold this code.

## Getting Help

- Open an issue for bugs or feature requests
- Join discussions in pull requests
- Check existing documentation in the `docs/` directory

Thank you for contributing to cmdr! 