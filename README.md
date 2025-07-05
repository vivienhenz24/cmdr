cmdr is a fast, REPL-based command-line interface written in Rust that lets users interact with their shell using natural language. It intercepts user input, using a local llm to translate requests into shell commands, then executes them transparently.

## Installation

### Homebrew (macOS and Linux)

```bash
brew tap vivienhenz24/cmdr
brew install cmdr
```

### From Source

```bash
git clone https://github.com/vivienhenz24/cmdr.git
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


<pre>
git commit -m"<b><a href="#types">&lt;type&gt;</a></b></font>(<b><a href="#scopes">&lt;optional scope&gt;</a></b>): <b><a href="#description">&lt;description&gt;</a></b>" \
  -m"<b><a href="#body">&lt;optional body&gt;</a></b>" \
  -m"<b><a href="#footer">&lt;optional footer&gt;</a></b>"
</pre>

> [!TIP]
> Take a look at **[git-conventional-commits](https://github.com/qoomon/git-conventional-commits)** ; a CLI util to ensure these conventions, determine version and generate changelogs.

## Commit Message Formats

### General Commit
<pre>
<b><a href="#types">&lt;type&gt;</a></b></font>(<b><a href="#scopes">&lt;optional scope&gt;</a></b>): <b><a href="#description">&lt;description&gt;</a></b>
<sub>empty line as separator</sub>
<b><a href="#body">&lt;optional body&gt;</a></b>
<sub>empty line as separator</sub>
<b><a href="#footer">&lt;optional footer&gt;</a></b>
</pre>

### Initial Commit 
```
chore: init
```

### Merge Commit
<pre>
Merge branch '<b>&lt;branch name&gt;</b>'
</pre>
<sup>Follows default git merge message</sup>

### Revert Commit
<pre>
Revert "<b>&lt;reverted commit subject line&gt;</b>"
</pre>
<sup>Follows default git revert message</sup>


### Types
- Changes relevant to the API or UI:
    - `feat` Commits that add, adjust or remove a new feature to the API or UI
    - `fix` Commits that fix an API or UI bug of a preceded `feat` commit
- `refactor` Commits that rewrite or restructure code without altering API or UI behavior
    - `perf` Commits are special type of `refactor` commits that specifically improve performance
- `style` Commits that address code style (e.g., white-space, formatting, missing semi-colons) and do not affect application behavior
- `test` Commits that add missing tests or correct existing ones
- `docs` Commits that exclusively affect documentation
- `build` Commits that affect build-related components such as build tools, dependencies, project version, CI/CD pipelines, ...
- `ops` Commits that affect operational components like infrastructure, deployment, backup, recovery procedures, ...
- `chore` Miscellaneous commits e.g. modifying `.gitignore`, ...

