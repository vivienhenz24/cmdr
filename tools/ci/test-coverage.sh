#!/bin/bash

# Test Coverage Script for cmdr
# This script generates and reports test coverage

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
COVERAGE_DIR="target/coverage"
COVERAGE_THRESHOLD=80

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

# Check if grcov is installed
check_grcov() {
    if ! command -v grcov >/dev/null 2>&1; then
        log_error "grcov is not installed"
        log_info "Install with: cargo install grcov"
        exit 1
    fi
}

# Install required components
install_components() {
    log_info "Installing required components..."
    
    # Install grcov if not present
    if ! command -v grcov >/dev/null 2>&1; then
        log_info "Installing grcov..."
        cargo install grcov
    fi
    
    # Install llvm-tools if not present
    if ! rustup component list | grep -q "llvm-tools-preview"; then
        log_info "Installing llvm-tools-preview..."
        rustup component add llvm-tools-preview
    fi
}

# Clean previous coverage data
clean_coverage() {
    log_info "Cleaning previous coverage data..."
    rm -rf "$COVERAGE_DIR"
    mkdir -p "$COVERAGE_DIR"
    
    # Clean target directory to ensure fresh coverage
    cargo clean
}

# Generate coverage data
generate_coverage() {
    log_info "Generating coverage data..."
    
    # Set environment variables for coverage
    export CARGO_INCREMENTAL=0
    export RUSTFLAGS="-Zinstrument-coverage"
    export LLVM_PROFILE_FILE="$COVERAGE_DIR/cargo-test-%p-%m.profraw"
    
    # Run tests with coverage
    cargo test --all-targets
    
    # Generate coverage report
    grcov . \
        --binary-path ./target/debug/ \
        -s . \
        -t html \
        --branch \
        --ignore-not-existing \
        --ignore "/*" \
        --ignore "target/*" \
        --ignore "tools/*" \
        --ignore "examples/*" \
        --ignore "tests/*" \
        -o "$COVERAGE_DIR/html"
    
    # Generate coverage report in lcov format
    grcov . \
        --binary-path ./target/debug/ \
        -s . \
        -t lcov \
        --branch \
        --ignore-not-existing \
        --ignore "/*" \
        --ignore "target/*" \
        --ignore "tools/*" \
        --ignore "examples/*" \
        --ignore "tests/*" \
        -o "$COVERAGE_DIR/lcov.info"
    
    log_success "Coverage data generated"
}

# Calculate coverage percentage
calculate_coverage() {
    log_info "Calculating coverage percentage..."
    
    if [[ -f "$COVERAGE_DIR/lcov.info" ]]; then
        # Use lcov to get coverage percentage
        local coverage=$(lcov --summary "$COVERAGE_DIR/lcov.info" 2>/dev/null | \
            grep "lines......:" | \
            sed 's/.*lines......: \([0-9.]*\)%.*/\1/')
        
        echo "$coverage"
    else
        echo "0"
    fi
}

# Check coverage threshold
check_threshold() {
    local coverage=$1
    local threshold=$2
    
    if (( $(echo "$coverage < $threshold" | bc -l) )); then
        log_error "Coverage $coverage% is below threshold $threshold%"
        return 1
    else
        log_success "Coverage $coverage% meets threshold $threshold%"
        return 0
    fi
}

# Generate coverage report
generate_report() {
    log_info "Generating coverage report..."
    
    local coverage=$(calculate_coverage)
    
    # Create summary report
    cat > "$COVERAGE_DIR/summary.txt" << EOF
Test Coverage Report
===================

Generated: $(date)
Coverage: ${coverage}%
Threshold: ${COVERAGE_THRESHOLD}%

Files covered:
$(find "$COVERAGE_DIR/html" -name "*.html" | wc -l) HTML files
1 LCOV file

View detailed report: $COVERAGE_DIR/html/index.html
EOF
    
    log_success "Coverage report generated"
    echo "Coverage: ${coverage}%"
}

# Upload coverage to codecov (if configured)
upload_coverage() {
    if [[ -n "${CODECOV_TOKEN:-}" ]]; then
        log_info "Uploading coverage to Codecov..."
        
        if command -v codecov >/dev/null 2>&1; then
            codecov -f "$COVERAGE_DIR/lcov.info" -t "$CODECOV_TOKEN"
            log_success "Coverage uploaded to Codecov"
        else
            log_warning "codecov CLI not installed, skipping upload"
        fi
    else
        log_info "CODECOV_TOKEN not set, skipping upload"
    fi
}

# Main coverage process
main() {
    log_info "Starting test coverage analysis..."
    
    # Install required components
    install_components
    
    # Check grcov installation
    check_grcov
    
    # Clean previous coverage
    clean_coverage
    
    # Generate coverage
    generate_coverage
    
    # Generate report
    generate_report
    
    # Check threshold
    local coverage=$(calculate_coverage)
    if ! check_threshold "$coverage" "$COVERAGE_THRESHOLD"; then
        log_warning "Coverage is below threshold, but continuing..."
    fi
    
    # Upload coverage
    upload_coverage
    
    log_success "Coverage analysis completed!"
    log_info "View detailed report: $COVERAGE_DIR/html/index.html"
}

main "$@" 