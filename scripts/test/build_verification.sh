#!/bin/bash

# Build Verification Test Suite
# Tests that the project builds correctly and has proper version information

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$(dirname "$SCRIPT_DIR")")"

echo "🔨 Testing Build Verification"
echo "============================="

cd "$PROJECT_ROOT"

# Test 1: Clean build
echo ""
echo "📝 Test 1: Clean build"
echo "Running: cargo clean"
cargo clean
echo "✅ Clean completed"

echo ""
echo "📝 Test 2: Debug build"
echo "Running: cargo build --bin cmdr"
cargo build --bin cmdr
if [[ -f "target/debug/cmdr" ]]; then
    echo "✅ Debug build successful"
else
    echo "❌ Debug build failed"
    exit 1
fi

echo ""
echo "📝 Test 3: Release build"
echo "Running: cargo build --release --bin cmdr"
cargo build --release --bin cmdr
if [[ -f "target/release/cmdr" ]]; then
    echo "✅ Release build successful"
else
    echo "❌ Release build failed"
    exit 1
fi

# Test 4: Check binary sizes
echo ""
echo "📝 Test 4: Binary size verification"
DEBUG_SIZE=$(stat -f%z target/debug/cmdr 2>/dev/null || stat -c%s target/debug/cmdr 2>/dev/null || echo "unknown")
RELEASE_SIZE=$(stat -f%z target/release/cmdr 2>/dev/null || stat -c%s target/release/cmdr 2>/dev/null || echo "unknown")

echo "Debug binary size: $DEBUG_SIZE bytes"
echo "Release binary size: $RELEASE_SIZE bytes"

# Check if release binary is smaller than debug (should be)
if [[ "$RELEASE_SIZE" != "unknown" && "$DEBUG_SIZE" != "unknown" ]]; then
    if [[ $RELEASE_SIZE -lt $DEBUG_SIZE ]]; then
        echo "✅ Release binary is smaller than debug binary"
    else
        echo "⚠️  Release binary is not smaller than debug binary"
    fi
fi

# Test 5: Version information
echo ""
echo "📝 Test 5: Version information"
VERSION_OUTPUT=$(target/debug/cmdr --version)
echo "Version output: $VERSION_OUTPUT"

# Check if version contains expected components
if [[ $VERSION_OUTPUT =~ ^cmdr\ [0-9]+\.[0-9]+\.[0-9]+ ]]; then
    echo "✅ Version format is correct"
else
    echo "❌ Version format is incorrect"
    exit 1
fi

# Check if version contains Git information (should contain commit hash)
if [[ $VERSION_OUTPUT =~ -[a-f0-9]+ ]]; then
    echo "✅ Version contains Git commit hash"
else
    echo "⚠️  Version does not contain Git commit hash"
fi

# Test 6: Cargo check
echo ""
echo "📝 Test 6: Cargo check"
echo "Running: cargo check"
cargo check
echo "✅ Cargo check passed"

# Test 7: Cargo test
echo ""
echo "📝 Test 7: Cargo test"
echo "Running: cargo test"
cargo test
echo "✅ Cargo test passed"

# Test 8: Dependencies tree
echo ""
echo "📝 Test 8: Dependencies verification"
echo "Running: cargo tree -e no-dev | grep clap"
CLAP_DEPS=$(cargo tree -e no-dev | grep clap || echo "No clap dependencies found")
echo "Clap dependencies:"
echo "$CLAP_DEPS"

# Verify essential clap dependencies are present
if echo "$CLAP_DEPS" | grep -q "clap v4.5.40"; then
    echo "✅ Main clap dependency found"
else
    echo "❌ Main clap dependency not found"
    exit 1
fi

if echo "$CLAP_DEPS" | grep -q "clap_builder"; then
    echo "✅ clap_builder dependency found"
else
    echo "❌ clap_builder dependency not found"
    exit 1
fi

if echo "$CLAP_DEPS" | grep -q "rustyline"; then
    echo "✅ rustyline dependency found"
else
    echo "❌ rustyline dependency not found"
    exit 1
fi

# Test 9: Build script verification
echo ""
echo "📝 Test 9: Build script verification"
if [[ -f "cmdr-cli/build.rs" ]]; then
    echo "✅ Build script exists"
    
    # Check if build script contains Git version logic
    if grep -q "git rev-parse" cmdr-cli/build.rs; then
        echo "✅ Build script contains Git version logic"
    else
        echo "❌ Build script missing Git version logic"
        exit 1
    fi
else
    echo "❌ Build script not found"
    exit 1
fi

echo ""
echo "🎉 All build verification tests completed successfully!" 