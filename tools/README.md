# cmdr Development Tools

This directory contains comprehensive development tools for the cmdr project, designed to scale with the project's growth.

## Directory Structure

```
tools/
├── benchmark/          # Performance benchmarking tools
├── fuzz/              # Fuzzing and stress testing tools
├── ci/                # CI/CD and deployment scripts
└── README.md          # This file
```

## Benchmark Tools

### Overview
The benchmarking tools provide comprehensive performance measurement capabilities for cmdr components.

### Features
- **Performance Measurement**: Measure throughput and latency of core operations
- **Comparison**: Compare performance between different versions or configurations
- **JSON Reports**: Generate detailed performance reports in JSON format
- **Extensible**: Easy to add new benchmarks for different components

### Usage
```bash
# Run benchmarks
cd tools/benchmark
cargo run --release -- --iterations 1000 --output ../../benchmark-results.json

# Compare with previous results
cargo run --release -- --compare previous-results.json
```

### Adding New Benchmarks
1. Add benchmark code to `tools/benchmark/src/main.rs`
2. Use the `BenchmarkRunner` to measure performance
3. Run with appropriate iteration counts for statistical significance

## Fuzzing Tools

### Overview
The fuzzing tools help find edge cases and bugs through automated testing with random inputs.

### Features
- **Random Input Generation**: Generate random test data
- **Crash Detection**: Automatically detect panics and crashes
- **Timeout Protection**: Prevent infinite loops during fuzzing
- **Detailed Reports**: Generate comprehensive fuzzing reports

### Usage
```bash
# Run fuzz tests
cd tools/fuzz
cargo run --release -- --iterations 10000 --timeout 300
```

### Adding New Fuzz Tests
1. Define a function that takes `&[u8]` input
2. Add it to the fuzzer in `tools/fuzz/src/main.rs`
3. Run with appropriate iteration counts

## CI/CD Tools

### Build Script (`tools/ci/build.sh`)
Comprehensive build and quality check script.

**Features:**
- Rust version checking
- Code quality checks (cargo check, clippy)
- Test execution
- Benchmark execution
- Documentation generation
- Security audit

**Usage:**
```bash
./tools/ci/build.sh
```

### Release Script (`tools/ci/release.sh`)
Automated release creation with proper versioning.

**Features:**
- Version management
- Git tag creation
- Release artifact generation
- Pre-release validation

**Usage:**
```bash
./tools/ci/release.sh 1.0.0
```

### Deployment Script (`tools/ci/deploy.sh`)
Multi-platform deployment automation.

**Supported Platforms:**
- Homebrew (macOS/Linux)
- GitHub Releases
- crates.io
- Docker Hub

**Usage:**
```bash
# Deploy to all platforms
./tools/ci/deploy.sh all

# Deploy to specific platform
./tools/ci/deploy.sh homebrew
./tools/ci/deploy.sh github
./tools/ci/deploy.sh crates
./tools/ci/deploy.sh docker
```

### Test Coverage Script (`tools/ci/test-coverage.sh`)
Generate and analyze test coverage reports.

**Features:**
- HTML coverage reports
- LCOV format output
- Coverage threshold checking
- Codecov integration

**Usage:**
```bash
./tools/ci/test-coverage.sh
```

## Environment Variables

The CI/CD scripts use the following environment variables:

### Deployment
- `HOMEBREW_TAP_TOKEN`: Token for Homebrew tap repository
- `GITHUB_TOKEN`: GitHub API token for releases
- `CARGO_REGISTRY_TOKEN`: Token for crates.io publishing
- `DOCKER_USERNAME`: Docker Hub username
- `DOCKER_PASSWORD`: Docker Hub password

### Coverage
- `CODECOV_TOKEN`: Codecov upload token

## Integration with CI/CD

### GitHub Actions
Add to `.github/workflows/ci.yml`:
```yaml
- name: Run CI checks
  run: ./tools/ci/build.sh

- name: Generate coverage
  run: ./tools/ci/test-coverage.sh

- name: Run benchmarks
  run: |
    cd tools/benchmark
    cargo run --release -- --iterations 100
```

### Pre-commit Hooks
The tools integrate with the pre-commit hooks in `scripts/git-hooks/` to ensure code quality.

## Extending the Tools

### Adding New Tools
1. Create a new directory under `tools/`
2. Add a `Cargo.toml` for Rust tools or shell scripts
3. Document the tool in this README
4. Add integration to CI scripts if appropriate

### Best Practices
- Use consistent error handling and logging
- Provide clear documentation and usage examples
- Make tools configurable through command-line arguments
- Ensure tools work in CI environments
- Add appropriate tests for tool functionality

## Troubleshooting

### Common Issues

**Benchmark tool fails to compile:**
- Ensure all dependencies are installed
- Check that cmdr-core is properly linked

**Fuzzing tool crashes:**
- Reduce iteration count
- Increase timeout value
- Check for memory issues

**CI scripts fail:**
- Verify environment variables are set
- Check file permissions on scripts
- Ensure required tools are installed

### Getting Help
- Check the individual tool documentation
- Review the CI logs for specific error messages
- Ensure all dependencies are properly installed 