# Scripts Directory

This directory contains organized test scripts and utilities for the cmdr project.

## Directory Structure

```
scripts/
├── README.md              # This file
├── run_tests.sh           # Master test runner
├── setup-git-hooks.sh     # Git hooks setup script
├── git-hooks/             # Git hooks templates
│   ├── pre-commit         # Check-only pre-commit hook
│   └── pre-commit-auto-format  # Auto-format pre-commit hook
└── test/                  # Test suites
    ├── build_verification.sh    # Build and dependency tests
    ├── cli_interface.sh         # CLI argument parsing tests
    ├── repl_behavior.sh         # REPL interactive behavior tests
    └── file_descriptors.sh      # File descriptor leak tests
```

## Git Hooks Setup

To prevent formatting and code quality issues, you can install pre-commit hooks:

```bash
./scripts/setup-git-hooks.sh
```

This will prompt you to choose between:
1. **Check-only hook** (recommended) - Validates formatting without changing files
2. **Auto-format hook** - Automatically formats code before commit

The hooks will run:
- `cargo fmt --check` - Ensures code is properly formatted
- `cargo check` - Validates compilation
- `cargo clippy` - Checks for code quality issues

If any check fails, the commit will be aborted.

## Test Categories

### 1. Build Verification (`build_verification.sh`)
Tests the build process and project configuration:
- Clean and debug/release builds
- Binary size verification
- Version information with Git integration
- Cargo check and test execution
- Dependency tree verification
- Build script functionality

### 2. CLI Interface (`cli_interface.sh`)
Tests command-line argument parsing and behavior:
- Help and version flags (short and long forms)
- Command flag (-c/--command) functionality
- Config flag (--config) placeholder behavior
- Error handling for invalid argument combinations
- Default behavior (no arguments)

### 3. REPL Behavior (`repl_behavior.sh`)
Tests interactive REPL functionality:
- Basic command processing
- Ctrl-D exit behavior
- Ctrl-C interrupt handling
- Empty line handling
- Multiple command processing

### 4. File Descriptors (`file_descriptors.sh`)
Tests process termination and resource cleanup:
- Normal exit via Ctrl-D
- Fast-path exit via -c flag
- Help and version flag exits
- Error condition exits
- Long-running process termination
- File descriptor leak detection

## Usage

### Run All Tests
```bash
./scripts/run_tests.sh
```

### Run Specific Test Categories
```bash
./scripts/run_tests.sh build cli          # Run only build and CLI tests
./scripts/run_tests.sh repl               # Run only REPL tests
./scripts/run_tests.sh fds                # Run only file descriptor tests
```

### List Available Tests
```bash
./scripts/run_tests.sh --list
```

### Get Help
```bash
./scripts/run_tests.sh --help
```

## Prerequisites

Before running tests, ensure:
1. The project builds successfully: `cargo build --bin cmdr`
2. You have the necessary system tools (lsof, stat, etc.)
3. You're running from the project root directory

## Test Output

Tests use colored output for easy identification:
- 🔨 Build verification tests
- 🔧 CLI interface tests  
- 🧪 REPL behavior tests
- 🔍 File descriptor tests

Each test category provides detailed output showing:
- What is being tested
- Expected vs actual results
- Pass/fail status with emojis
- Summary of all test results

## Adding New Tests

To add a new test category:

1. Create a new script in `scripts/test/` with the naming convention `category_name.sh`
2. Make the script executable: `chmod +x scripts/test/category_name.sh`
3. Follow the existing pattern:
   - Use `set -e` for error handling
   - Include proper script header with description
   - Use the project root detection pattern
   - Provide clear test descriptions and status messages
   - Return appropriate exit codes

4. The master test runner will automatically pick up new test scripts

## Continuous Integration

These scripts are designed to be run in CI environments:
- All scripts use relative paths and detect project root
- Tests are independent and can be run in isolation
- Clear exit codes for CI integration
- Comprehensive error reporting

## Troubleshooting

### Common Issues

**Binary not found**: Run `cargo build --bin cmdr` first

**Permission denied**: Make scripts executable with `chmod +x scripts/test/*.sh`

**File descriptor tests fail**: Ensure you have `lsof` installed and proper permissions

**Tests hang**: Some tests may wait for user input; use Ctrl-C to interrupt

### Debug Mode

For verbose output, use the `-v` flag:
```bash
./scripts/run_tests.sh -v
``` 