#!/bin/bash

# Master Test Runner for cmdr
# Runs all test suites or specific test categories

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEST_DIR="$SCRIPT_DIR/test"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

# Function to print usage
print_usage() {
    echo "Usage: $0 [OPTIONS] [TEST_CATEGORIES...]"
    echo ""
    echo "Options:"
    echo "  -h, --help     Show this help message"
    echo "  -v, --verbose  Enable verbose output"
    echo "  --list         List available test categories"
    echo ""
    echo "Test Categories:"
    echo "  all           Run all tests (default)"
    echo "  build         Build verification tests"
    echo "  cli           CLI interface tests"
    echo "  repl          REPL behavior tests"
    echo "  fds           File descriptor leak tests"
    echo ""
    echo "Examples:"
    echo "  $0                    # Run all tests"
    echo "  $0 build cli          # Run only build and CLI tests"
    echo "  $0 --list             # List available test categories"
}

# Function to make test scripts executable
make_tests_executable() {
    print_status "$BLUE" "Making test scripts executable..."
    chmod +x "$TEST_DIR"/*.sh
}

# Function to run a test suite
run_test_suite() {
    local test_name=$1
    local test_script="$TEST_DIR/${test_name}.sh"
    
    if [[ ! -f "$test_script" ]]; then
        print_status "$RED" "❌ Test script not found: $test_script"
        return 1
    fi
    
    print_status "$BLUE" "🚀 Running $test_name tests..."
    print_status "$BLUE" "=================================="
    
    if "$test_script"; then
        print_status "$GREEN" "✅ $test_name tests passed"
        return 0
    else
        print_status "$RED" "❌ $test_name tests failed"
        return 1
    fi
}

# Function to list available tests
list_tests() {
    print_status "$BLUE" "Available test categories:"
    echo ""
    for script in "$TEST_DIR"/*.sh; do
        if [[ -f "$script" ]]; then
            local test_name=$(basename "$script" .sh)
            echo "  $test_name"
        fi
    done
    echo ""
}

# Parse command line arguments
VERBOSE=false
TEST_CATEGORIES=()

while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            print_usage
            exit 0
            ;;
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        --list)
            list_tests
            exit 0
            ;;
        -*)
            print_status "$RED" "Unknown option: $1"
            print_usage
            exit 1
            ;;
        *)
            TEST_CATEGORIES+=("$1")
            shift
            ;;
    esac
done

# If no test categories specified, run all tests
if [[ ${#TEST_CATEGORIES[@]} -eq 0 ]]; then
    TEST_CATEGORIES=("all")
fi

# Check if test directory exists
if [[ ! -d "$TEST_DIR" ]]; then
    print_status "$RED" "❌ Test directory not found: $TEST_DIR"
    exit 1
fi

# Make test scripts executable
make_tests_executable

# Run tests
FAILED_TESTS=()
PASSED_TESTS=()

if [[ " ${TEST_CATEGORIES[@]} " =~ " all " ]]; then
    # Run all test suites
    for script in "$TEST_DIR"/*.sh; do
        if [[ -f "$script" ]]; then
            test_name=$(basename "$script" .sh)
            if run_test_suite "$test_name"; then
                PASSED_TESTS+=("$test_name")
            else
                FAILED_TESTS+=("$test_name")
            fi
            echo ""
        fi
    done
else
    # Run specific test suites
    for category in "${TEST_CATEGORIES[@]}"; do
        if run_test_suite "$category"; then
            PASSED_TESTS+=("$category")
        else
            FAILED_TESTS+=("$category")
        fi
        echo ""
    done
fi

# Print summary
echo ""
print_status "$BLUE" "📊 Test Summary"
print_status "$BLUE" "==============="

if [[ ${#PASSED_TESTS[@]} -gt 0 ]]; then
    print_status "$GREEN" "✅ Passed tests: ${PASSED_TESTS[*]}"
fi

if [[ ${#FAILED_TESTS[@]} -gt 0 ]]; then
    print_status "$RED" "❌ Failed tests: ${FAILED_TESTS[*]}"
fi

echo ""
if [[ ${#FAILED_TESTS[@]} -eq 0 ]]; then
    print_status "$GREEN" "🎉 All tests passed!"
    exit 0
else
    print_status "$RED" "💥 Some tests failed!"
    exit 1
fi 