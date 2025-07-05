class Cmdr < Formula
  desc "A fast, REPL-based command-line interface that translates natural language to shell commands"
  homepage "https://github.com/vivienhenz24/cmdr"
  url "https://github.com/vivienhenz24/cmdr/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "PLACEHOLDER_SHA256"
  license "MIT OR Apache-2.0"
  head "https://github.com/vivienhenz24/cmdr.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--path", "cmdr-cli", "--bin", "cmdr"
    bin.install "target/release/cmdr"
  end

  test do
    system "#{bin}/cmdr", "--help"
  end
end 