#!/bin/bash

# Deployment Script for cmdr
# This script handles deploying cmdr to various platforms

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
HOMEBREW_TAP="cmdr-project/cmdr"

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

# Deploy to Homebrew
deploy_homebrew() {
    log_info "Deploying to Homebrew..."
    
    if [[ -z "${HOMEBREW_TAP_TOKEN:-}" ]]; then
        log_warning "HOMEBREW_TAP_TOKEN not set, skipping Homebrew deployment"
        return 0
    fi
    
    # Update Homebrew formula
    local version=$(grep '^version = ' Cargo.toml | cut -d'"' -f2)
    local sha256=$(shasum -a 256 "$DIST_DIR/cmdr" | cut -d' ' -f1)
    
    # Update Formula/cmdr.rb
    sed -i.bak "s/version \"[^\"]*\"/version \"$version\"/" Formula/cmdr.rb
    sed -i.bak "s/sha256 \"[^\"]*\"/sha256 \"$sha256\"/" Formula/cmdr.rb
    rm Formula/cmdr.rb.bak
    
    # Commit and push to tap repository
    git add Formula/cmdr.rb
    git commit -m "Update cmdr to version $version"
    git push origin main
    
    log_success "Homebrew deployment completed"
}

# Deploy to GitHub Releases
deploy_github_releases() {
    log_info "Deploying to GitHub Releases..."
    
    if [[ -z "${GITHUB_TOKEN:-}" ]]; then
        log_warning "GITHUB_TOKEN not set, skipping GitHub Releases deployment"
        return 0
    fi
    
    local version=$(grep '^version = ' Cargo.toml | cut -d'"' -f2)
    local tag="v$version"
    
    # Create GitHub release using gh CLI
    if command -v gh >/dev/null 2>&1; then
        gh release create "$tag" \
            --title "Release $version" \
            --notes "Release version $version" \
            "$DIST_DIR"/*.tar.gz \
            "$DIST_DIR"/*.zip
        
        log_success "GitHub release created: $tag"
    else
        log_warning "GitHub CLI (gh) not installed, skipping GitHub Releases"
    fi
}

# Deploy to crates.io
deploy_crates_io() {
    log_info "Deploying to crates.io..."
    
    if [[ -z "${CARGO_REGISTRY_TOKEN:-}" ]]; then
        log_warning "CARGO_REGISTRY_TOKEN not set, skipping crates.io deployment"
        return 0
    fi
    
    # Publish each crate
    for crate in crates/*/; do
        if [[ -f "$crate/Cargo.toml" ]]; then
            local crate_name=$(basename "$crate")
            log_info "Publishing $crate_name..."
            cd "$crate"
            cargo publish --token "$CARGO_REGISTRY_TOKEN"
            cd - >/dev/null
            log_success "Published $crate_name"
        fi
    done
}

# Deploy to Docker Hub
deploy_docker() {
    log_info "Deploying to Docker Hub..."
    
    if [[ -z "${DOCKER_USERNAME:-}" ]] || [[ -z "${DOCKER_PASSWORD:-}" ]]; then
        log_warning "Docker credentials not set, skipping Docker deployment"
        return 0
    fi
    
    local version=$(grep '^version = ' Cargo.toml | cut -d'"' -f2)
    
    # Build Docker image
    docker build -t cmdr:latest -t cmdr:$version .
    
    # Login to Docker Hub
    echo "$DOCKER_PASSWORD" | docker login -u "$DOCKER_USERNAME" --password-stdin
    
    # Push images
    docker push cmdr:latest
    docker push cmdr:$version
    
    log_success "Docker deployment completed"
}

# Main deployment process
main() {
    local target=${1:-all}
    
    log_info "Starting deployment process for target: $target"
    
    # Check if we have release artifacts
    if [[ ! -d "$DIST_DIR" ]]; then
        log_error "Release artifacts not found in $DIST_DIR"
        log_info "Run ./tools/ci/release.sh first to create release artifacts"
        exit 1
    fi
    
    case "$target" in
        homebrew)
            deploy_homebrew
            ;;
        github)
            deploy_github_releases
            ;;
        crates)
            deploy_crates_io
            ;;
        docker)
            deploy_docker
            ;;
        all)
            deploy_homebrew
            deploy_github_releases
            deploy_crates_io
            deploy_docker
            ;;
        *)
            log_error "Unknown deployment target: $target"
            log_info "Available targets: homebrew, github, crates, docker, all"
            exit 1
            ;;
    esac
    
    log_success "Deployment completed successfully!"
}

main "$@" 