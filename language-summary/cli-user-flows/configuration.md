# Configuration System

Configure Left-Right behavior through global machine config, project config, and environment variables.

## Quick Start

```bash
# View current config
$ lr --config show

# Edit global config
$ lr --config edit

# Generate default config
$ lr --config init

# Override config inline
$ lr "file.lr" --target node
```

## Config File Locations

### Location Priority (High to Low)

1. **CLI flags** (`--target node`)
2. **Environment variables** (`LR_TARGET`)
3. **Project config** (`lr.toml`, `.lftconfig`)
4. **Global config** (`~/.config/left-right/config.toml`)
5. **Defaults** (hardcoded)

### Global Config Locations

| Platform | Path |
|-----------|-------|
| Linux | `~/.config/left-right/config.toml` |
| macOS | `~/.config/left-right/config.toml` |
| Windows | `%APPDATA%\left-right\config.toml` |
| Legacy | `~/.lrc` (deprecated) |

### Project Config Locations

| File | Priority |
|-------|-----------|
| `lr.toml` | 1 (recommended) |
| `.lftconfig` | 2 (legacy) |
| `LeftRight.toml` | 3 (legacy) |

## Global Config Format

### Default Global Config

`~/.config/left-right/config.toml`:
```toml
[general]
# Default transpilation target
default_target = "rust"  # "rust" or "node"

# Verbosity level (0=quiet, 1=normal, 2=verbose, 3=debug)
verbosity = 1

# Enable colored output
color = true

# Editor for opening files (TUI "open" command)
editor = "vim"

[transpilation]
# Optimization level (0=none, 1=basic, 2=aggressive)
optimization = 1

# Generate source maps (for debugging)
source_maps = false

# Output format (compact, pretty, raw)
output_format = "pretty"

[rust]
# Rust optimization flags (passed to rustc)
rustc_flags = "-O"

# Target architecture
target_triple = ""  # Empty = system default

# Use rustup toolchain
toolchain = "stable"

[node]
# Node.js binary path (empty = from PATH)
node_path = ""

# Node arguments
node_args = ""

[watch]
# Debounce delay in milliseconds
debounce_ms = 100

# Watch directories recursively
recursive = true

# Clear screen before each run
clear_on_run = true

# Show timestamps
show_timestamps = false

# Patterns to ignore
ignore_patterns = [
  "*.swp",
  "*~",
  ".git/*",
  "node_modules/*",
  ".lr-cache/*"
]

[tui]
# History size (entries)
history_limit = 1000

# Autosave interval (seconds)
autosave_interval = 60

# Session file location
session_file = "~/.local/share/left-right/session.lr-session"

# Enable mouse support
mouse_support = true

[semantics]
# Allow custom operators
allow_custom_operators = true

# Enable operator overriding
allow_operator_override = true

# File for custom operator definitions
custom_operators_file = "~/.config/left-right/custom-ops.lr"

# !? operator output format
# Default: "js" (matches JavaScript typeof semantics)
# Options: "js", "rust", "python", "simple"
type_of_format = "js"

# Operator override priority (high to low)
# intra-script: In-file header overrides (#! override: ...)
# folder: Directory-level operators.lr
# project: lr.toml operators_file
# global: custom-ops.lr
# Transpiler defaults: operators-*.lr files
override_priority = ["intra-script", "folder", "project", "global", "default"]

# Transpiler operator definition files
operators_recursive = "~/.config/left-right/operators-recursive.lr"
operators_rust_optimal = "~/.config/left-right/operators-rust-optimal.lr"
operators_js_optimal = "~/.config/left-right/operators-js-optimal.lr"

[cache]
# Enable transpilation cache
enabled = true

# Cache directory
directory = "~/.cache/left-right"

# Cache TTL (seconds)
ttl = 3600
```

## Project Config Format

### Minimal Project Config

`lr.toml`:
```toml
[project]
name = "my-project"
version = "1.0.0"

[target]
# Override global default for this project
default_target = "node"

[entry]
# Main entry point (for `lr` without args)
main = "src/main.lr"
```

### Full Project Config

`lr.toml`:
```toml
[project]
name = "data-pipeline"
version = "1.0.0"
description = "ETL pipeline for log processing"

[target]
default_target = "rust"
rustc_flags = "-O --release"
node_args = "--max-old-space-size=4096"

[entry]
main = "src/main.lr"
test = "src/test.lr"

[semantics]
# Project-specific custom operators
operators_file = "config/operators.lr"

# Override precedence
precedence_file = "config/precedence.lr"

[transpilation]
# Output directory for transpiled code
output_dir = "target/"

# Keep transpiled files
keep_intermediate = true

[watch]
# Project-specific watch config
debounce_ms = 200
ignore_patterns = [
  "target/*",
  "*.rs",
  "*.js"
]

[environment]
# Environment variables passed to executed code
LR_DEBUG = "0"
LR_LOG_LEVEL = "info"

[dependencies]
# External libraries (for file import resolution)
lodash = "node_modules/lodash/fp"
moment = "node_modules/moment"
```

## Environment Variable Overrides

### General Variables

| Variable | Description | Default |
|-----------|-------------|----------|
| `LR_TARGET` | Default transpilation target | rust |
| `LR_CONFIG` | Path to config file | ~/.config/left-right/config.toml |
| `LR_TEMP_DIR` | Temporary directory for builds | /tmp/lr-builds |
| `LR_CACHE_DIR` | Cache directory | ~/.cache/left-right |
| `LR_DEBUG` | Debug mode (0/1) | 0 |
| `LR_LOG_LEVEL` | Logging level (error/warn/info/debug) | info |
| `LR_NO_COLOR` | Disable ANSI colors | 0 |
| `LR_VERBOSE` | Verbose output (0/1) | 0 |

### Target-Specific Variables

```bash
# Rust target overrides
export LR_RUSTC_FLAGS="-O --release"
export LR_RUST_TARGET_TRIPLE="x86_64-unknown-linux-gnu"
export LR_RUST_TOOLCHAIN="nightly"

# Node target overrides
export LR_NODE_PATH="/usr/local/bin/node"
export LR_NODE_ARGS="--max-old-space-size=4096"
```

### Watch Mode Variables

```bash
export LR_WATCH_DEBOUNCE=200
export LR_WATCH_RECURSIVE=1
export LR_WATCH_POLLING=0  # 1=force polling
```

### TUI Variables

```bash
export LR_TUI_HISTORY_LIMIT=2000
export LR_TUI_MOUSE_SUPPORT=1
export LR_TUI_EDITOR="code"
```

## Semantic Customization

### Custom Operators

Create `~/.config/left-right/custom-ops.lr`:

```left-right
{
  // Custom operator: ** for exponentiation
  **:
    precedence: 10,
    associativity: right,
    implementation:
      { base, exp => base ^ exp },

  // Custom operator: ?? for undefined coalescing
  ??:
    precedence: 5,
    associativity: left,
    implementation:
      { value, default => value ?? undefined & default | value },

  // Custom operator: <~ for text interpolation
  <~:
    precedence: 8,
    associativity: right,
    implementation:
      { template, values => interpolate [template, values] }
}
```

Now usable in any Left-Right code:

```left-right
{
  result: 2 ** 8,  // 256
  value: undefined ?? `default`,  // `default`
  message: `Hello, ` <~ [`world!`]  // `Hello, world!`
}
```

### Operator Precedence Override

`config/precedence.lr`:
```left-right
{
  precedence:
    [
      { operator: '@', level: 1 },
      { operator: '**', level: 10 },  // Higher than *
      { operator: '*', level: 3 },
      { operator: '/', level: 3 },
      { operator: '+', level: 2 },
      { operator: '-', level: 2 },
      { operator: '>>', level: 4 },
      { operator: '_>', level: 6 },
      { operator: '_<', level: 6 },
      { operator: '&', level: 1 },
      { operator: '|', level: 1 },
      { operator: '~', level: 7 }
    ]
}
```

### Type Dispatch Override

`config/types.lr`:
```left-right
{
  number:
    {
      // Custom + behavior for numbers
      +: { lhs, rhs => lhs + rhs },
      // Custom comparison
      ==: { lhs, rhs => lhs - rhs < 0.00001 }
    },

  string:
    {
      // Case-insensitive comparison
      ==: { lhs, rhs => lhs >> toLowerCase == rhs >> toLowerCase },
      // Concatenation (default)
      +: { lhs, rhs => lhs + rhs }
    }
}
```

### !? Operator Configuration

The `!?` operator (type inspection) can be configured at different levels:

**Global Config (`~/.config/left-right/config.toml`)**:
```toml
[semantics]
# Match JavaScript type of semantics (default)
type_of_format = "js"

# Or choose alternatives:
# "rust" - match Rust std::any::type_name
# "python" - match Python type() naming
# "simple" - basic type names (number, text, etc.)
```

**Project Config (`lr.toml`)**:
```toml
[semantics]
# Override global for this project
type_of_format = "simple"
```

**Usage Example**:
```left-right
# With type_of_format = "js"
{ type: 42 >> !? }
# Output: "number"

{ type: `hello` >> !? }
# Output: "text"

{ type: [1, 2, 3] >> !? }
# Output: "list"

# With type_of_format = "simple"
{ type: 42 >> !? }
# Output: "number"
```

### Operator Override Priority Configuration

Configure which override levels are active and their order:

**Global Config (`~/.config/left-right/config.toml`)**:
```toml
[semantics]
# Full cascade enabled (default)
override_priority = ["intra-script", "folder", "project", "global", "default"]

# Disable folder overrides
override_priority = ["intra-script", "project", "global", "default"]

# Only use global + transpiler defaults (no overrides)
override_priority = ["global", "default"]
```

**Override Level Details**:

| Level | File Location | Priority | Use Case |
|-------|---------------|----------|----------|
| intra-script | `#! override: <file>` (header) | Highest | File-specific behavior |
| folder | `./operators.lr` | High | Directory-wide operators |
| project | `lr.toml: [semantics].operators_file` | Medium | Project standards |
| global | `~/.config/left-right/custom-ops.lr` | Low | Personal preferences |
| default | `operators-*.lr` | Lowest | Transpiler built-ins |

### Transpiler Operator Files Configuration

Three operator definition files control transpilation behavior:

**Global Config Paths**:
```toml
[semantics]
# Recursive-based operators (default for most cases)
operators_recursive = "~/.config/left-right/operators-recursive.lr"

# Rust-optimized operators (maximal efficiency)
operators_rust_optimal = "~/.config/left-right/operators-rust-optimal.lr"

# JavaScript-optimized operators (maximal efficiency)
operators_js_optimal = "~/.config/left-right/operators-js-optimal.lr"
```

**Project Config Overrides**:
```toml
[semantics]
# Use Rust-optimized for this project
operators_file_override = "rust-optimal"
# Or specify custom path:
operators_file_custom = "./config/my-operators.lr"
```

**Shebang Override in Files**:
```left-right
#! override: operators-rust-optimal.lr

{
  # This file uses Rust-optimized operators
  result: data >> filter [x => x > 0]
}
```

**All Overridable At**:
- **Intra-script level**: Via shebang header (`#! override: <file>`)
- **Folder level**: Via `./operators.lr` file
- **Project level**: Via `lr.toml` configuration
- **Global machine level**: Via `~/.config/left-right/config.toml`

## TUI Configuration Integration

### Reading Config in TUI

```rust
fn load_config(app: &mut TuiApp) {
    let config = read_global_config()?;

    app.config.default_target = config.general.default_target;
    app.config.color = config.general.color;
    app.tui.history_limit = config.tui.history_limit;
    app.tui.mouse_support = config.tui.mouse_support;

    Ok(())
}
```

### Writing Config from TUI

Access via Semantic Editor:

```
Left-Right REPL
> :config edit
# Opens config in editor defined in [general].editor

> :config reload
# Reloads config without restarting

> :config show
# Displays current config

> :config set default_target node
# Sets specific option (temporary)
```

### Session-Only Config Changes

Changes made in TUI affect only current session:

```
Left-Right REPL
> :set target node
# Session target now: node
> { result: 42 }
# Uses Node for this session

> :save config
# Save current session settings to global config
```

## Config File Examples

### Minimal Config (Rust-Optimized)

`~/.config/left-right/config.toml`:
```toml
[general]
default_target = "rust"

[rust]
rustc_flags = "-O --release"

[watch]
debounce_ms = 50
```

### Minimal Config (Node-Optimized)

`~/.config/left-right/config.toml`:
```toml
[general]
default_target = "node"

[node]
node_args = "--max-old-space-size=4096"
```

### Development Config

`lr.toml`:
```toml
[project]
name = "my-app"

[target]
default_target = "node"  # Fast iteration

[transpilation]
output_format = "pretty"
source_maps = true  # Debug support

[watch]
debounce_ms = 100
clear_on_run = false  # Keep history visible
```

### Production Config

`lr.toml`:
```toml
[project]
name = "my-app"

[target]
default_target = "rust"  # Performance

[rust]
rustc_flags = "-O --release --lto"

[transpilation]
output_format = "compact"
source_maps = false
```

## Config Validation

### Checking Config

```bash
$ lr --config validate
✓ Config file syntax: OK
✓ default_target: valid (rust)
✓ watch.debounce_ms: valid (100)
✗ cache.directory: invalid (path does not exist)

# Fix:
mkdir -p ~/.cache/left-right
```

### Auto-Repair

```bash
$ lr --config repair
# Fixes common config issues:
# - Creates missing directories
# - Sets invalid values to defaults
# - Removes deprecated keys
```

## Comparison with Other Config Systems

### vs Cargo.toml

| Feature | Cargo.toml | lr.toml |
|---------|-------------|-----------|
| Syntax | TOML | TOML |
| Sections | `[package]`, `[dependencies]` | `[project]`, `[target]` |
| Dependency management | Yes | Planned |
| Build configuration | Yes | Yes (transpilation) |
| Workspace support | Yes | Planned |

**Similarities**:
- TOML-based
- Hierarchical sections
- Project-level config

**Differences**:
- Left-Right has semantic customization
- Cargo has full dependency management

### vs tsconfig.json

| Feature | tsconfig.json | lr.toml |
|--------------|--------------|-----------|
| Syntax | JSON | TOML |
| Compiler options | Extensive | Focused (transpilation) |
| Path mapping | Yes | Planned |
| Semantic customization | No | Yes |

**Advantages of lr.toml**:
- More readable (TOML vs JSON)
- Built-in semantic customization
- Target-agnostic config

### vs .babelrc

| Feature | .babelrc | lr.toml |
|-----------|-----------|-----------|
| Syntax | JSON/YAML | TOML |
| Plugin system | Extensive | Planned |
| Presets | Yes | No |
| Custom operators | No | Yes |

**Advantages of lr.toml**:
- Operator definitions built-in
- Type dispatch customization
- Precedence override

## Advanced Config Patterns

### Environment-Specific Configs

`lr.toml`:
```toml
[project]
name = "my-app"

[target]
# Select target based on environment
default_target = "${LR_ENV:-rust}"  # rust if LR_ENV not set

[environment.prod]
target = "rust"
rustc_flags = "-O --release"

[environment.dev]
target = "node"
source_maps = true
```

Usage:
```bash
# Production
export LR_ENV=prod
lr "main.lr"

# Development
export LR_ENV=dev
lr "main.lr"
```

### Config Inheritance

`lr.toml`:
```toml
[project]
name = "my-app"

# Base config
extends = "../common-config/lr.toml"

[target]
# Override specific values
default_target = "rust"
```

### Conditional Configs

`lr.toml`:
```toml
[watch]
# Only watch specific directories when in CI
watch_dirs = [
  "${CI:-src}"
]

[transpilation]
# Disable source maps in production
source_maps = "${CI:-false}"
```

## Troubleshooting

### Config Not Loading

```bash
# Check config file exists
ls -la ~/.config/left-right/

# Validate config syntax
lr --config validate

# Reset to defaults
rm ~/.config/left-right/config.toml
lr --config init
```

### Conflicting Configs

```bash
# Check which config is being used
lr --config show

# See all config sources
lr --config show --verbose

# Temporarily disable project config
LR_NO_PROJECT_CONFIG=1 lr "file.lr"
```

### Custom Operators Not Working

```bash
# Check custom operators file
cat ~/.config/left-right/custom-ops.lr

# Validate syntax
lr --config validate

# Check if custom operators enabled
lr --config show | grep allow_custom_operators
```

### Config Changes Not Applied

```bash
# Reload config (if in TUI)
> :config reload

# Restart process
$ lr "file.lr"  # New process loads new config

# Clear cache (if caching issues)
rm -rf ~/.cache/left-right/*
```
