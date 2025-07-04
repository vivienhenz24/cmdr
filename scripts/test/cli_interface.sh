#!/bin/bash

# CLI Interface Test Suite
# Tests command-line argument parsing, help, version, and error handling

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$(dirname "$SCRIPT_DIR")")"
CMDR_BIN="$PROJECT_ROOT/target/debug/cmdr"

echo "🔧 Testing CLI Interface"
echo "========================"

# Ensure cmdr binary exists
if [[ ! -f "$CMDR_BIN" ]]; then
    echo "❌ cmdr binary not found at $CMDR_BIN"
    echo "   Run 'cargo build --bin cmdr' first"
    exit 1
fi

echo "✅ cmdr binary found at $CMDR_BIN"

# Test 1: Help flag
echo ""
echo "📝 Test 1: Help flag"
echo "Command: $CMDR_BIN --help"
"$CMDR_BIN" --help > /dev/null
if [[ $? -eq 0 ]]; then
    echo "✅ Help flag test passed"
else
    echo "❌ Help flag test failed"
    exit 1
fi

# Test 2: Version flag
echo ""
echo "📝 Test 2: Version flag"
echo "Command: $CMDR_BIN --version"
VERSION_OUTPUT=$("$CMDR_BIN" --version)
echo "Version output: $VERSION_OUTPUT"
if [[ $VERSION_OUTPUT =~ ^cmdr\ [0-9]+\.[0-9]+\.[0-9]+ ]]; then
    echo "✅ Version flag test passed"
else
    echo "❌ Version flag test failed"
    exit 1
fi

# Test 3: Short help flag
echo ""
echo "📝 Test 3: Short help flag"
echo "Command: $CMDR_BIN -h"
"$CMDR_BIN" -h > /dev/null
if [[ $? -eq 0 ]]; then
    echo "✅ Short help flag test passed"
else
    echo "❌ Short help flag test failed"
    exit 1
fi

# Test 4: Short version flag
echo ""
echo "📝 Test 4: Short version flag"
echo "Command: $CMDR_BIN -V"
VERSION_OUTPUT=$("$CMDR_BIN" -V)
echo "Version output: $VERSION_OUTPUT"
if [[ $VERSION_OUTPUT =~ ^cmdr\ [0-9]+\.[0-9]+\.[0-9]+ ]]; then
    echo "✅ Short version flag test passed"
else
    echo "❌ Short version flag test failed"
    exit 1
fi

# Test 5: Command flag (-c)
echo ""
echo "📝 Test 5: Command flag (-c)"
echo "Command: $CMDR_BIN -c 'test command'"
COMMAND_OUTPUT=$("$CMDR_BIN" -c "test command")
if [[ $COMMAND_OUTPUT == "(command execution not yet implemented)" ]]; then
    echo "✅ Command flag test passed"
else
    echo "❌ Command flag test failed"
    echo "Expected: (command execution not yet implemented)"
    echo "Got: $COMMAND_OUTPUT"
    exit 1
fi

# Test 6: Long command flag (--command)
echo ""
echo "📝 Test 6: Long command flag (--command)"
echo "Command: $CMDR_BIN --command 'test command'"
COMMAND_OUTPUT=$("$CMDR_BIN" --command "test command")
if [[ $COMMAND_OUTPUT == "(command execution not yet implemented)" ]]; then
    echo "✅ Long command flag test passed"
else
    echo "❌ Long command flag test failed"
    echo "Expected: (command execution not yet implemented)"
    echo "Got: $COMMAND_OUTPUT"
    exit 1
fi

# Test 7: Config flag (--config)
echo ""
echo "📝 Test 7: Config flag (--config)"
echo "Command: $CMDR_BIN --config test.conf"
CONFIG_OUTPUT=$("$CMDR_BIN" --config test.conf)
if [[ $CONFIG_OUTPUT == *"Configuration file specified: test.conf"* ]]; then
    echo "✅ Config flag test passed"
else
    echo "❌ Config flag test failed"
    echo "Expected output to contain: Configuration file specified: test.conf"
    echo "Got: $CONFIG_OUTPUT"
    exit 1
fi

# Test 8: Error handling - invalid argument combination
echo ""
echo "📝 Test 8: Error handling - invalid argument combination"
echo "Command: $CMDR_BIN -c 'test' extra_arg"
if "$CMDR_BIN" -c "test" extra_arg 2>&1 | grep -q "error: unexpected argument found"; then
    echo "✅ Error handling test passed"
else
    echo "❌ Error handling test failed"
    exit 1
fi

# Test 9: Default behavior (no arguments)
echo ""
echo "📝 Test 9: Default behavior (no arguments)"
echo "Command: $CMDR_BIN"
DEFAULT_OUTPUT=$(echo -e "\x04" | "$CMDR_BIN")
if [[ $DEFAULT_OUTPUT == *"Interactive REPL mode (not yet implemented)"* ]]; then
    echo "✅ Default behavior test passed"
else
    echo "❌ Default behavior test failed"
    echo "Expected output to contain: Interactive REPL mode (not yet implemented)"
    echo "Got: $DEFAULT_OUTPUT"
    exit 1
fi

echo ""
echo "🎉 All CLI interface tests completed successfully!" 