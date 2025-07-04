cmdr is a fast, REPL-based command-line interface written in Rust that lets users interact with their shell using natural language. It intercepts user input, using a local llm to translate requests into shell commands, then executes them transparently.

## Installation

### Homebrew (macOS and Linux)

```bash
brew tap cmdr-project/cmdr
brew install cmdr
```

### From Source

```bash
git clone https://github.com/cmdr-project/cmdr.git
cd cmdr
cargo install --path cmdr-cli
```

### Pre-built Binaries

Download the latest release from [GitHub Releases](https://github.com/cmdr-project/cmdr/releases).

## Usage

```bash
cmdr
```

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

