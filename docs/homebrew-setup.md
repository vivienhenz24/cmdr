# Homebrew Setup Guide

This document explains how to set up and maintain the Homebrew tap for cmdr.

## Overview

We use a custom Homebrew tap to distribute cmdr via `brew install`. The tap repository is located at `cmdr-project/homebrew-cmdr`.

## Initial Setup

1. **Create the tap repository** (one-time setup):
   ```bash
   ./scripts/setup_homebrew_tap.sh
   ```

2. **Verify the tap works**:
   ```bash
   brew tap cmdr-project/cmdr
   brew install cmdr
   ```

## Releasing a New Version

When you want to release a new version:

1. **Create and push a new tag**:
   ```bash
   git tag v0.2.0
   git push origin v0.2.0
   ```

2. **Update the Homebrew tap**:
   ```bash
   ./scripts/setup_homebrew_tap.sh v0.2.0
   ```

The script will:
- Download the release tarball
- Calculate the SHA256 hash
- Update the formula in the tap repository
- Commit and push the changes

## Manual Formula Updates

If you need to manually update the formula, edit `Formula/cmdr.rb` in the tap repository:

```ruby
class Cmdr < Formula
  desc "A fast, REPL-based command-line interface that translates natural language to shell commands"
  homepage "https://github.com/cmdr-project/cmdr"
  url "https://github.com/cmdr-project/cmdr/archive/refs/tags/v0.2.0.tar.gz"
  sha256 "actual_sha256_hash_here"
  license "MIT OR Apache-2.0"
  head "https://github.com/cmdr-project/cmdr.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--path", "cmdr-cli", "--bin", "cmdr"
    bin.install "target/release/cmdr"
  end

  test do
    system "#{bin}/cmdr", "--help"
  end
end
```

## Testing the Formula

To test the formula locally:

```bash
# Clone the tap repository
git clone https://github.com/cmdr-project/homebrew-cmdr.git
cd homebrew-cmdr

# Install from local formula
brew install ./Formula/cmdr.rb
```

## Troubleshooting

### SHA256 Mismatch
If you get a SHA256 mismatch error:
1. Download the tarball manually
2. Calculate the correct SHA256: `shasum -a 256 cmdr-v0.2.0.tar.gz`
3. Update the formula with the correct hash

### Build Failures
If the build fails in Homebrew:
1. Test locally first: `brew install --build-from-source ./Formula/cmdr.rb`
2. Check that all dependencies are properly specified
3. Ensure the build process works in a clean environment

## Alternative: Homebrew Core

For inclusion in the official Homebrew core repository:
1. Follow the [Homebrew contribution guidelines](https://github.com/Homebrew/homebrew-core/blob/master/CONTRIBUTING.md)
2. Submit a pull request to homebrew-core
3. Wait for review and approval

Note: This requires more stringent requirements and community approval. 