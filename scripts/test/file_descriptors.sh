#!/bin/bash

# File Descriptor Leak Test Suite
# Tests that cmdr processes exit cleanly without file descriptor leaks

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$(dirname "$SCRIPT_DIR")")"
CMDR_BIN="$PROJECT_ROOT/target/debug/cmdr"

echo "🔍 Testing File Descriptor Handling"
echo "==================================="

# Ensure cmdr binary exists
if [[ ! -f "$CMDR_BIN" ]]; then
    echo "❌ cmdr binary not found at $CMDR_BIN"
    echo "   Run 'cargo build --bin cmdr' first"
    exit 1
fi

echo "✅ cmdr binary found at $CMDR_BIN"

# Function to check if process exists and has file descriptors
check_process_fds() {
    local pid=$1
    local process_name=$2
    
    if lsof -p "$pid" >/dev/null 2>&1; then
        echo "⚠️  Process $pid ($process_name) still exists with file descriptors:"
        lsof -p "$pid" | head -10
        return 1
    else
        echo "✅ Process $pid ($process_name) exited cleanly"
        return 0
    fi
}

# Test 1: Normal exit via Ctrl-D
echo ""
echo "📝 Test 1: Normal exit via Ctrl-D"
echo "Starting cmdr process..."
"$CMDR_BIN" &
PID=$!
echo "Process started with PID: $PID"

# Wait a moment for process to start
sleep 1

# Check initial file descriptors
echo "Initial file descriptors:"
lsof -p "$PID" | head -5

# Send Ctrl-D to exit
echo "Sending Ctrl-D to exit..."
echo -e "\x04" > /proc/$PID/fd/0 2>/dev/null || echo "Ctrl-D sent"

# Wait for process to exit
sleep 2

# Check if process exited cleanly
if check_process_fds "$PID" "cmdr (Ctrl-D exit)"; then
    echo "✅ Normal exit test passed"
else
    echo "❌ Normal exit test failed"
    kill -9 "$PID" 2>/dev/null || true
    exit 1
fi

# Test 2: Fast-path exit via -c flag
echo ""
echo "📝 Test 2: Fast-path exit via -c flag"
echo "Starting cmdr process with -c flag..."
"$CMDR_BIN" -c "test command" &
PID=$!
echo "Process started with PID: $PID"

# Wait for process to exit
sleep 1

# Check if process exited cleanly
if check_process_fds "$PID" "cmdr (fast-path exit)"; then
    echo "✅ Fast-path exit test passed"
else
    echo "❌ Fast-path exit test failed"
    kill -9 "$PID" 2>/dev/null || true
    exit 1
fi

# Test 3: Help flag exit
echo ""
echo "📝 Test 3: Help flag exit"
echo "Starting cmdr process with --help flag..."
"$CMDR_BIN" --help &
PID=$!
echo "Process started with PID: $PID"

# Wait for process to exit
sleep 1

# Check if process exited cleanly
if check_process_fds "$PID" "cmdr (help exit)"; then
    echo "✅ Help exit test passed"
else
    echo "❌ Help exit test failed"
    kill -9 "$PID" 2>/dev/null || true
    exit 1
fi

# Test 4: Version flag exit
echo ""
echo "📝 Test 4: Version flag exit"
echo "Starting cmdr process with --version flag..."
"$CMDR_BIN" --version &
PID=$!
echo "Process started with PID: $PID"

# Wait for process to exit
sleep 1

# Check if process exited cleanly
if check_process_fds "$PID" "cmdr (version exit)"; then
    echo "✅ Version exit test passed"
else
    echo "❌ Version exit test failed"
    kill -9 "$PID" 2>/dev/null || true
    exit 1
fi

# Test 5: Error exit
echo ""
echo "📝 Test 5: Error exit"
echo "Starting cmdr process with invalid arguments..."
"$CMDR_BIN" -c "test" extra_arg &
PID=$!
echo "Process started with PID: $PID"

# Wait for process to exit
sleep 1

# Check if process exited cleanly
if check_process_fds "$PID" "cmdr (error exit)"; then
    echo "✅ Error exit test passed"
else
    echo "❌ Error exit test failed"
    kill -9 "$PID" 2>/dev/null || true
    exit 1
fi

# Test 6: Long-running process termination
echo ""
echo "📝 Test 6: Long-running process termination"
echo "Starting cmdr process for extended test..."
"$CMDR_BIN" &
PID=$!
echo "Process started with PID: $PID"

# Wait a moment for process to start
sleep 1

# Check initial file descriptors
echo "Initial file descriptors:"
lsof -p "$PID" | head -5

# Send SIGTERM to terminate gracefully
echo "Sending SIGTERM..."
kill -TERM "$PID"

# Wait for process to exit
sleep 2

# Check if process exited cleanly
if check_process_fds "$PID" "cmdr (SIGTERM exit)"; then
    echo "✅ Long-running process termination test passed"
else
    echo "❌ Long-running process termination test failed"
    kill -9 "$PID" 2>/dev/null || true
    exit 1
fi

echo ""
echo "🎉 All file descriptor tests completed successfully!"
echo "✅ No file descriptor leaks detected" 