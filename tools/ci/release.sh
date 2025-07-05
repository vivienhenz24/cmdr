#!/bin/bash

# Release Script for cmdr
# This script handles creating releases with proper versioning

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
RELEASE_DIR="target/release"
DIST_DIR="dist"

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

# Get current version from Cargo.toml
get_version() {
    grep '^version = ' Cargo.toml | cut -d'"' -f2
}

# Check if we're on a clean git state
check_git_clean() {
    if [[ -n $(git status --porcelain) ]]; then
        log_error "Git working directory is not clean. Please commit or stash changes."
        exit 1
    fi
}

# Check if we're on the main branch
check_branch() {
    current_branch=$(git branch --show-current)
    if [[ "$current_branch" != "main" && "$current_branch" != "master" ]]; then
        log_error "Not on main/master branch. Current branch: $current_branch"
        exit 1
    fi
}

# Build release artifacts
build_release() {
    log_info "Building release artifacts..."
    
    # Clean and build
    cargo clean
    cargo build --release
    
    # Create distribution directory
    rm -rf "$DIST_DIR"
    mkdir -p "$DIST_DIR"
    
    # Copy binary
    cp "$RELEASE_DIR/cmdr" "$DIST_DIR/"
    
    # Create platform-specific archives
    local version=$(get_version)
    local platform=$(uname -s | tr '[:upper:]' '[:lower:]')
    local arch=$(uname -m)
    
    # Create tar.gz archive
    tar -czf "$DIST_DIR/cmdr-${version}-${platform}-${arch}.tar.gz" -C "$DIST_DIR" cmdr
    
    # Create zip archive (if zip is available)
    if command -v zip >/dev/null 2>&1; then
        cd "$DIST_DIR"
        zip "cmdr-${version}-${platform}-${arch}.zip" cmdr
        cd - >/dev/null
    fi
    
    log_success "Release artifacts created in $DIST_DIR/"
}

# Update version in Cargo.toml
update_version() {
    local new_version=$1
    local current_version=$(get_version)
    
    log_info "Updating version from $current_version to $new_version"
    
    # Update Cargo.toml
    sed -i.bak "s/^version = \"$current_version\"/version = \"$new_version\"/" Cargo.toml
    rm Cargo.toml.bak
    
    # Update Cargo.lock
    cargo update
    
    log_success "Version updated to $new_version"
}

# Create git tag
create_tag() {
    local version=$1
    local tag="v$version"
    
    log_info "Creating git tag: $tag"
    git add Cargo.toml Cargo.lock
    git commit -m "Release version $version"
    git tag -a "$tag" -m "Release version $version"
    
    log_success "Git tag $tag created"
}

# Main release process
main() {
    if [[ $# -ne 1 ]]; then
        log_error "Usage: $0 <new_version>"
        log_info "Example: $0 1.0.0"
        exit 1
    fi
    
    local new_version=$1
    
    log_info "Starting release process for version $new_version"
    
    # Pre-release checks
    check_git_clean
    check_branch
    
    # Run CI checks
    log_info "Running CI checks..."
    ./tools/ci/build.sh
    
    # Run tests without native-llama feature
    log_info "Running tests..."
    cargo test --no-default-features
    
    # Update version
    update_version "$new_version"
    
    # Build release artifacts
    build_release
    
    # Create git tag
    create_tag "$new_version"
    
    log_success "Release $new_version is ready!"
    log_info "Next steps:"
    log_info "1. Review the changes: git log --oneline -5"
    log_info "2. Push the tag: git push origin v$new_version"
    log_info "3. Push the changes: git push origin main"
    log_info "4. Create a GitHub release with the artifacts from $DIST_DIR/"
}

main "$@" 