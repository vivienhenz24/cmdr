#!/bin/bash

# CI/CD Build Script for cmdr
# This script handles building, testing, and quality checks

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
RUST_VERSION="1.70.0"
CARGO_FLAGS="--all-targets"
CLIPPY_FLAGS="--all-targets -- -D warnings"

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the right directory
if [[ ! -f "Cargo.toml" ]]; then
    log_error "Not in cmdr project root directory"
    exit 1
fi

# Check Rust version
log_info "Checking Rust version..."
rust_version=$(rustc --version | cut -d' ' -f2)
if [[ "$rust_version" != "$RUST_VERSION" ]]; then
    log_warning "Rust version mismatch. Expected: $RUST_VERSION, Got: $rust_version"
    log_info "Consider using rustup to install the correct version"
fi

# Clean previous builds
log_info "Cleaning previous builds..."
cargo clean

# Update dependencies
log_info "Updating dependencies..."
cargo update

# Check code
log_info "Running cargo check..."
cargo check $CARGO_FLAGS
log_success "Code check passed"

# Run clippy
log_info "Running cargo clippy..."
cargo clippy $CLIPPY_FLAGS
log_success "Clippy check passed"

# Run tests
log_info "Running tests..."
cargo test $CARGO_FLAGS
log_success "Tests passed"

# Build release version
log_info "Building release version..."
cargo build --release
log_success "Release build completed"

# Run benchmarks (if available)
if [[ -d "tools/benchmark" ]]; then
    log_info "Running benchmarks..."
    cd tools/benchmark
    cargo run --release -- --iterations 100 --output ../../benchmark-results.json
    cd ../..
    log_success "Benchmarks completed"
fi

# Run fuzzing (if available)
if [[ -d "tools/fuzz" ]]; then
    log_info "Running fuzz tests..."
    cd tools/fuzz
    cargo run --release -- --iterations 1000 --timeout 60
    cd ../..
    log_success "Fuzz tests completed"
fi

# Check binary size
log_info "Checking binary size..."
binary_size=$(stat -f%z target/release/cmdr 2>/dev/null || stat -c%s target/release/cmdr 2>/dev/null || echo "unknown")
log_info "Binary size: $binary_size bytes"

# Generate documentation
log_info "Generating documentation..."
cargo doc --no-deps
log_success "Documentation generated"

# Run security audit
log_info "Running security audit..."
cargo audit || log_warning "Security audit found issues (check output above)"

log_success "All CI checks completed successfully!" 