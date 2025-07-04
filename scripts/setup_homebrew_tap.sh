#!/bin/bash

# Script to set up Homebrew tap for cmdr
set -e

VERSION=${1:-$(git describe --tags --abbrev=0)}
TAP_NAME="cmdr-project/homebrew-cmdr"

echo "Setting up Homebrew tap for version: $VERSION"

# Create tap repository if it doesn't exist
if ! gh repo view "$TAP_NAME" >/dev/null 2>&1; then
    echo "Creating tap repository: $TAP_NAME"
    gh repo create "$TAP_NAME" --public --description "Homebrew tap for cmdr"
fi

# Clone the tap repository
TAP_DIR="/tmp/homebrew-cmdr"
rm -rf "$TAP_DIR"
git clone "https://github.com/$TAP_NAME.git" "$TAP_DIR"

# Download the release tarball and calculate SHA256
TARBALL_URL="https://github.com/cmdr-project/cmdr/archive/refs/tags/$VERSION.tar.gz"
TARBALL_PATH="/tmp/cmdr-$VERSION.tar.gz"

echo "Downloading tarball: $TARBALL_URL"
curl -L -o "$TARBALL_PATH" "$TARBALL_URL"

SHA256=$(shasum -a 256 "$TARBALL_PATH" | cut -d' ' -f1)
echo "SHA256: $SHA256"

# Update the formula
FORMULA_PATH="$TAP_DIR/Formula/cmdr.rb"
mkdir -p "$(dirname "$FORMULA_PATH")"

cat > "$FORMULA_PATH" << EOF
class Cmdr < Formula
  desc "A fast, REPL-based command-line interface that translates natural language to shell commands"
  homepage "https://github.com/cmdr-project/cmdr"
  url "https://github.com/cmdr-project/cmdr/archive/refs/tags/$VERSION.tar.gz"
  sha256 "$SHA256"
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
EOF

# Commit and push changes
cd "$TAP_DIR"
git add .
git commit -m "Update cmdr to $VERSION"
git push origin main

echo "Homebrew tap updated successfully!"
echo "Users can now install cmdr with:"
echo "  brew tap cmdr-project/cmdr"
echo "  brew install cmdr" 