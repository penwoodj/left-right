# Left-Right CLI Overview

Left-Right (`lr`) command-line interface for running, transpiling, and experimenting with the Left-Right programming language.

## Quick Start

```bash
# Run interactive TUI shell
lr

# Execute a Left-Right file (default: Rust target)
lr "path/to/file.lr"

# Run with Node target
lr "path/to/file.lr" --target node

# Watch a directory for changes
lr --watch src/
```

## CLI Commands

| Command | Description |
|---------|-------------|
| `lr` | Open interactive TUI shell (REPL + semantic editor) |
| `lr "path-to-file"` | Parse, transpile, and execute a single file |
| `lr --watch "path"` | Watch directory, auto-transpile on file changes |
| `lr --help` | Show command-line help and options |
| `lr --version` | Show version information |

## Installation

### From Source

```bash
# Clone repository
git clone https://github.com/your-org/left-right.git
cd left-right

# Install using Cargo (Rust)
cargo install --path .

# Verify installation
lr --version
```

### Package Managers (Future)

```bash
# Homebrew (macOS/Linux)
brew install left-right-lang/lr

# NPM (JS wrapper)
npm install -g @left-right/cli
```

## First-Time Experience

### 1. Initial Configuration

First run prompts for basic setup:

```bash
$ lr
╔══════════════════════════════════════════════════════════╗
║           Left-Right Language - First-Time Setup                ║
╚══════════════════════════════════════════════════════════╝

Welcome to Left-Right!

Default transpilation target:
  [1] Rust (compiled, faster execution)
  [2] Node (interpreted, easier debugging)

Select target [1-2]: 1

Configuration written to: ~/.config/left-right/config.toml
```

### 2. Create Your First File

`hello-world.lr`:
```left-right
{
  greeting: `Hello, world!`,
  message: greeting >> ` ` >> `Welcome to Left-Right!`
}
```

### 3. Run It

```bash
# Using Rust (default)
lr "hello-world.lr"
# Output: Hello, world! Welcome to Left-Right!

# Using Node explicitly
lr "hello-world.lr" --target node
# Output: Hello, world! Welcome to Left-Right!
```

## Workflow Overview

### Development Flow

```
┌─────────────┐
│  Write      │
│  .lr files  │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  Transpile  │  (Left-Right → Rust or JS)
└──────┬──────┘
       │
       ├─────────┬─────────┐
       │         │         │
       ▼         ▼         ▼
┌─────────┐ ┌─────────┐ ┌─────────┐
│   Rust  │ │  Node   │ │  Watch  │
│  rustc  │ │   node  │ │  Loop   │
└────┬────┘ └────┬────┘ └────┬────┘
     │            │            │
     └────────────┴────────────┘
                  │
                  ▼
           ┌─────────────┐
           │  Execute    │
           │  Output     │
           └─────────────┘
```

### Interactive vs Batch

**Interactive (TUI):**
- Experiment with syntax
- Test snippets
- Edit language semantics live
- Inspect AST/intermediate representations
- Build muscle memory

**Batch (File execution):**
- Run production code
- CI/CD pipelines
- Build scripts
- One-off data transformations

## CLI Options Reference

### Global Options

| Option | Short | Description | Default |
|---------|--------|-------------|----------|
| `--target` | `-t` | Transpilation target (rust/node) | rust |
| `--config` | `-c` | Path to config file | ~/.config/left-right/config.toml |
| `--verbose` | `-v` | Enable verbose logging | false |
| `--output` | `-o` | Output file path (for transpile-only) | stdout |
| `--watch` | `-w` | Watch mode (directory or file) | false |
| `--debounce` | `-d` | Debounce delay for watch (ms) | 100 |

### Output Options

```bash
# Print transpiled code only (don't execute)
lr "file.lr" --output /dev/stdout --no-execute

# Save transpiled Rust to file
lr "file.lr" --target rust --output output.rs

# Show detailed parse/transpile diagnostics
lr "file.lr" --verbose
```

### Environment Variables

```bash
# Override default target
LR_TARGET=node lr "file.lr"

# Specify config file location
LR_CONFIG=/custom/config.toml lr

# Enable debug logging
LR_DEBUG=1 lr --watch src/
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Syntax/parse error |
| 2 | Transpilation error |
| 3 | Runtime error |
| 4 | File not found |
| 5 | Invalid configuration |
| 128 | Interrupted by user (Ctrl+C) |

## Next Steps

- **[TUI Shell](./tui-shell.md)** — Interactive REPL and semantic editor
- **[Run & Transpile](./run-transpile.md)** — File execution and transpilation details
- **[Watch Mode](./watch-mode.md)** — Auto-reload development workflow
- **[Configuration](./configuration.md)** — Config files and semantic customization

## Troubleshooting

### "rustc not found" Error

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### "node not found" Error

```bash
# Install Node.js (using nvm)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install --lts
```

### Config File Issues

```bash
# Reset to defaults
rm ~/.config/left-right/config.toml
lr  # Will recreate with defaults
```

## Examples Directory

After installation, explore example files:

```bash
# Clone example repo (if available)
git clone https://github.com/your-org/left-right-examples.git
cd left-right-examples

# Run all examples
for f in *.lr; do lr "$f"; done
```

## Performance Tips

- **Rust target** ~10-100x faster than Node for CPU-bound work
- **Watch mode** only re-transpiles changed files
- **Transpile once, execute many** for hot loops
- Use `--output` flag to cache transpiled code

## Community

- GitHub: https://github.com/your-org/left-right
- Discord: https://discord.gg/left-right-lang
- Documentation: https://docs.left-right-lang.org
