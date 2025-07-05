class Cmdr < Formula
  desc "A fast, REPL-based command-line interface that translates natural language to shell commands"
  homepage "https://github.com/vivienhenz24/cmdr"
  url "https://github.com/vivienhenz24/cmdr/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "PLACEHOLDER_SHA256"
  license "MIT OR Apache-2.0"
  head "https://github.com/vivienhenz24/cmdr.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "build", "--release", "--bin", "cmdr"
    bin.install "target/release/cmdr"
  end

  def post_install
    # Run the LLM installation after cmdr is installed
    system "#{bin}/cmdr", "install", "--skip-checks"
  rescue => e
    opoo "Failed to install LLM dependencies: #{e.message}"
    opoo "You can manually install them later with: #{bin}/cmdr install"
  end

  test do
    system "#{bin}/cmdr", "--help"
  end
end 