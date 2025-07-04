#!/bin/bash

# REPL Behavior Test Suite
# Tests interactive REPL functionality including Ctrl-C, Ctrl-D, and command processing

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$(dirname "$SCRIPT_DIR")")"
CMDR_BIN="$PROJECT_ROOT/target/debug/cmdr"

echo "🧪 Testing REPL Behavior"
echo "========================"

# Ensure cmdr binary exists
if [[ ! -f "$CMDR_BIN" ]]; then
    echo "❌ cmdr binary not found at $CMDR_BIN"
    echo "   Run 'cargo build --bin cmdr' first"
    exit 1
fi

echo "✅ cmdr binary found at $CMDR_BIN"

# Test 1: Basic command processing
echo ""
echo "📝 Test 1: Basic command processing"
echo "Input: echo -e 'test command\\nsecond command\\n' | $CMDR_BIN"
echo -e "test command\nsecond command\n" | "$CMDR_BIN"
echo "✅ Basic command processing test completed"

# Test 2: Ctrl-D exit behavior
echo ""
echo "📝 Test 2: Ctrl-D exit behavior"
echo "Input: echo -e 'test command\\n\x04' | $CMDR_BIN"
echo -e "test command\n\x04" | "$CMDR_BIN"
EXIT_CODE=$?
if [[ $EXIT_CODE -eq 0 ]]; then
    echo "✅ Ctrl-D exit test passed (exit code: $EXIT_CODE)"
else
    echo "❌ Ctrl-D exit test failed (exit code: $EXIT_CODE)"
    exit 1
fi

# Test 3: Ctrl-C behavior (interrupt handling)
echo ""
echo "📝 Test 3: Ctrl-C behavior"
echo "Input: echo -e 'test command\\n\x03\\nsecond command\\n' | $CMDR_BIN"
echo -e "test command\n\x03\nsecond command\n" | "$CMDR_BIN"
echo "✅ Ctrl-C behavior test completed"

# Test 4: Empty line handling
echo ""
echo "📝 Test 4: Empty line handling"
echo "Input: echo -e 'command1\\n\\ncommand2\\n' | $CMDR_BIN"
echo -e "command1\n\ncommand2\n" | "$CMDR_BIN"
echo "✅ Empty line handling test completed"

# Test 5: Multiple commands with mixed input
echo ""
echo "📝 Test 5: Multiple commands with mixed input"
echo "Input: echo -e 'first\\nsecond\\nthird\\n\x04' | $CMDR_BIN"
echo -e "first\nsecond\nthird\n\x04" | "$CMDR_BIN"
echo "✅ Multiple commands test completed"

echo ""
echo "🎉 All REPL behavior tests completed successfully!" 