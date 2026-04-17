# Watch Mode — Auto-Reload Development

Watch files for changes and automatically transpile and run. Perfect for development workflows.

## Quick Start

```bash
# Watch current directory
$ lr --watch

# Watch specific directory
$ lr --watch src/

# Watch with custom debounce delay
$ lr --watch --debounce 200

# Watch with Node target
$ lr --watch --target node
```

## Watch Flow

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          lr --watch src/                           │
└─────────────────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│  Initial: Transpile & Run all .lr files in directory               │
└─────────────────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │  1. Listen for file system events (notify crate)             │  │
│  │     - File created, modified, deleted                          │  │
│  │     - Directory changes                                        │  │
│  └───────────────────────────────────────────────────────────────────┘  │
│                              │                                      │
│                              ▼                                      │
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │  2. Debounce (default: 100ms)                             │  │
│  │     - Aggregate rapid changes                                   │  │
│  │     - Filter non-.lr files                                    │  │
│  └───────────────────────────────────────────────────────────────────┘  │
│                              │                                      │
│                              ▼                                      │
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │  3. Identify changed files                                    │  │
│  │     - Map events to file paths                                │  │
│  │     - Track dependency graph                                   │  │
│  └───────────────────────────────────────────────────────────────────┘  │
│                              │                                      │
│                              ▼                                      │
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │  4. Re-transpile affected files                              │  │
│  │     - Incremental transpilation if possible                     │  │
│  │     - Full transpilation for major changes                      │  │
│  └───────────────────────────────────────────────────────────────────┘  │
│                              │                                      │
│                              ▼                                      │
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │  5. Execute (if configured)                                  │  │
│  │     - Run transpiled code                                     │  │
│  │     - Capture output/errors                                    │  │
│  └───────────────────────────────────────────────────────────────────┘  │
│                              │                                      │
│                              ▼                                      │
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │  6. Display results                                        │  │
│  │     - Clear or append output                                  │  │
│  │     - Show errors (continue watching)                           │  │
│  └───────────────────────────────────────────────────────────────────┘  │
│                              │                                      │
│                              └───────────┬────────────────────────┘   │
                                          ▼                       │
                          (Loop back to step 1)                    │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Basic Usage

### Watch Current Directory

```bash
$ lr --watch
```

Output:
```
[watch] Watching /home/jon/code/project for .lr files...
[info] Initial scan: 3 files found
[exec] main.lr → { "result": 42 }

File changed: main.lr
[transpile] main.lr → Rust
[exec] main.lr → { "result": 42 }

File changed: utils.lr
[transpile] utils.lr → Rust
[exec] utils.lr → { "value": 10 }

File changed: main.lr
[transpile] main.lr → Rust
[exec] main.lr → { "result": 42 }
```

### Watch Specific Directory

```bash
$ lr --watch src/
```

Only watches `.lr` files in `src/`, not subdirectories (unless recursive).

### Watch with Recursive Scanning

```bash
$ lr --watch --recursive
# Watches src/, src/lib/, src/utils/, etc.
```

## File System Watching (notify Crate)

### Implementation Details

Watch mode uses the Rust `notify` crate for cross-platform file system events:

```rust
use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};

fn start_watch(path: &Path) -> notify::Result<()> {
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_millis(100))?;

    watcher.watch(path, RecursiveMode::Recursive)?;

    for event in rx {
        match event {
            DebouncedEvent::Create(path) => handle_file_created(path),
            DebouncedEvent::Write(path) => handle_file_modified(path),
            DebouncedEvent::Remove(path) => handle_file_removed(path),
            DebouncedEvent::Rename(_, new_path) => handle_file_renamed(new_path),
            _ => {}
        }
    }

    Ok(())
}
```

### Platform Support

| Platform | Backend | Status |
|-----------|----------|--------|
| Linux | inotify | ✅ Fully supported |
| macOS | FSEvents | ✅ Fully supported |
| Windows | ReadDirectoryChangesW | ✅ Fully supported |
| BSD/Linux fallback | polling | ⚠️ Slower, resource-intensive |

### Event Types Handled

- `Create` — New file added
- `Write` — File content changed
- `Remove` — File deleted
- `Rename` — File renamed/moved

Ignored events:
- `Access` — File read (no change)
- `.git/` directory changes
- `node_modules/` directory changes
- Temporary editor files (`*.swp`, `*~`)

## Debouncing Strategy

### What is Debouncing?

Debouncing aggregates rapid file changes into single events. Prevents multiple transpilations from a single save (e.g., atomic write operations in editors).

### Default Debounce: 100ms

```bash
# Default behavior
$ lr --watch
# Waits 100ms after last change before triggering transpilation
```

### Custom Debounce

```bash
# Faster debounce (immediate response)
$ lr --watch --debounce 50

# Slower debounce (avoids rapid rebuilds)
$ lr --watch --debounce 500

# No debounce (instant rebuild)
$ lr --watch --debounce 0
```

### Debounce Behavior Example

```
Editor saves file at: 0ms
Editor saves file at: 50ms
Editor saves file at: 100ms
Editor saves file at: 150ms
Editor saves file at: 200ms
                  ↓
          Last change at 200ms
                  ↓
          + 100ms debounce
                  ↓
          Trigger transpile at 300ms
```

### Smart Debouncing (Planned)

Future versions will detect file save patterns:

```rust
fn smart_debounce(events: Vec<FileEvent>) -> Vec<PathBuf> {
    // Group events by file
    let grouped = group_by_path(events);

    // Only trigger if event pattern matches atomic save
    grouped.into_iter()
        .filter(|(path, events)| is_atomic_save(events))
        .map(|(path, _)| path)
        .collect()
}
```

## Files That Trigger Re-Transpilation

### Triggers

| File Type | Triggers Re-Transpile? | Reason |
|-----------|------------------------|---------|
| `*.lr` | ✅ Yes | Source file changed |
| `lr.toml` | ✅ Yes | Config changed |
| `.lftconfig` | ✅ Yes | Project config changed |
| `*.js` (if Node target) | ❌ No | Not source |
| `*.rs` (if Rust target) | ❌ No | Not source |
| `node_modules/**` | ❌ No | Ignored |
| `.git/**` | ❌ No | Ignored |
| `*.swp`, `*~` | ❌ No | Editor temp files |

### Dependency Tracking

When a file is imported, dependent files are re-transpiled:

```left-right
// main.lr
{
  utils: file['./utils.lr'],
  result: utils.calculate [10, 20, 30]
}
```

When `utils.lr` changes:
```
File changed: utils.lr
[transpile] utils.lr (dependency of main.lr)
[exec] utils.lr → { "calculate": "[fn]" }
[transpile] main.lr
[exec] main.lr → { "result": 60 }
```

### Dependency Graph

```
main.lr
  ├─> utils.lr
  │     └─> math.lr
  └─> config.lr

If math.lr changes:
  math.lr → utils.lr → main.lr (all re-transpiled)
```

## Output Behavior

### Clear Screen Mode (Default)

```bash
$ lr --watch
# Output clears on each run
```

Example:
```
[watch] Watching...
[exec] main.lr → { "result": 42 }

[watch] File changed: main.lr
[exec] main.lr → { "result": 43 }  # Previous output cleared
```

### Append Mode

```bash
$ lr --watch --append
# Output accumulates
```

Example:
```
[watch] Watching...
[exec] main.lr → { "result": 42 }

[watch] File changed: main.lr
[exec] main.lr → { "result": 43 }
[exec] main.lr → { "result": 44 }
[exec] main.lr → { "result": 45 }
# All output retained
```

### Timestamped Output

```bash
$ lr --watch --timestamp
```

Example:
```
[2026-04-15 10:30:15] [exec] main.lr → { "result": 42 }
[2026-04-15 10:30:23] [watch] File changed: main.lr
[2026-04-15 10:30:24] [exec] main.lr → { "result": 43 }
```

## Error Handling During Watch

### Graceful Error Handling

Watch mode continues running even after errors:

```bash
$ lr --watch
[watch] Watching...
[exec] main.lr → { "result": 42 }

File changed: main.lr
[error] Parse error at line 5, column 12: Unexpected token
[watch] Still watching... (waiting for changes)

File changed: main.lr
[exec] main.lr → { "result": 42 }
[info] Previous error fixed, resuming normal execution
```

### Error Types Handled

| Error Type | Behavior |
|------------|-----------|
| Parse error | Show error, continue watching |
| Transpilation error | Show error, continue watching |
| Runtime error | Show error, continue watching |
| Compile error (Rust) | Show error, continue watching |
| File not found | Show warning, continue watching |
| Permission denied | Show error, continue watching |

### Automatic Recovery

```bash
$ lr --watch --auto-recover
# Automatically retries after temporary errors

File changed: main.lr
[error] Temporary lock file present, retrying in 1s...
[exec] main.lr → { "result": 42 }
```

### Fatal Errors

Only these stop watch mode:
- `Ctrl+C` (user interrupt)
- Out of memory
- File system watcher crash

## Integration with TUI Shell

### Watch Mode Inside TUI

Launch watch mode from TUI:

```
Left-Right REPL
> :watch src/
Entering watch mode (Ctrl+C to exit)
[watch] Watching src/...
[exec] src/main.lr → { "result": 42 }
```

Access from TUI with:
- `:watch [path]` command
- Click "Watch" button in toolbar (if mouse enabled)

### Watch Mode with Live AST

View AST as it changes:

```bash
$ lr --watch --ast
```

Output:
```
[watch] Watching...
File changed: main.lr
[exec] main.lr → { "result": 42 }
[ast] Program(File("main.lr"), [
  Object([
    Pair("result", Number(42))
  ])
])

File changed: main.lr
[exec] main.lr → { "result": 43 }
[ast] Program(File("main.lr"), [
  Object([
    Pair("result", Number(43))
  ])
])
```

## Comparison with Similar Tools

### vs nodemon

| Feature | nodemon | lr --watch |
|---------|----------|-------------|
| Target language | Node.js | Left-Right |
| Transpilation | No | Yes (LR → Rust/JS) |
| Dependency tracking | Manual | Automatic |
| Config file | nodemon.json | lr.toml |
| Debounce | Configurable | Configurable |
| Auto-restart | Yes | Yes |

**Advantages of lr --watch**:
- Built-in transpilation
- Multi-target support (Rust/Node)
- Dependency graph tracking
- Language-aware debouncing

### vs cargo-watch

| Feature | cargo-watch | lr --watch |
|-----------|--------------|-------------|
| Target language | Rust | Left-Right |
| Compilation | rustc | rustc (via transpile) |
| Hot reload | Yes | Yes |
| Multi-target | No | Yes (Rust/Node) |

**Advantages of lr --watch**:
- Source language abstraction
- Automatic transpilation
- Cross-platform file watching

### vs wasm-pack watch

| Feature | wasm-pack watch | lr --watch |
|-------------------|------------------|-------------|
| Target | WebAssembly | Rust/Node |
| Use case | Web apps | General |
| Incremental builds | Yes | Planned |

## Advanced Watch Scenarios

### Multi-Project Watch

Watch multiple directories:

```bash
# Terminal 1
$ lr --watch project-a/src/

# Terminal 2
$ lr --watch project-b/src/
```

Or use watch file:

`watch.lr`:
```left-right
{
  project_a: file['project-a/src'],
  project_b: file['project-b/src'],
  watch: [project_a, project_b] >> watch []
}
```

```bash
lr "watch.lr"
```

### Conditional Watch

Only watch specific files:

```bash
$ lr --watch --pattern 'test_*.lr'
# Only watches files matching test_*.lr
```

### Watch with Notifications

```bash
$ lr --watch --notify
# Sends system notification on error/success

# Requires libnotify (Linux) or osascript (macOS)
```

### Profiling Watch Performance

```bash
$ lr --watch --profile
# Shows timing information

[profile] File changed: main.lr
[profile] Debounce: 104ms
[profile] Parse: 12ms
[profile] Transpile: 45ms
[profile] Execute: 2ms
[profile] Total: 163ms
```

## Configuration

### In Config File

`lr.toml`:
```toml
[watch]
debounce_ms = 100
recursive = true
clear_on_run = true
show_timestamps = false
auto_recover = true
ignore_patterns = [
  "*.swp",
  "*~",
  ".git/*",
  "node_modules/*"
]
```

### CLI Flags Override Config

```bash
$ lr --watch --debounce 200  # Overrides config
```

## Troubleshooting

### Watch Not Detecting Changes

```bash
# Check notify backend
$ lr --watch --verbose
[info] Using inotify backend (Linux)

# Try slower debounce
$ lr --watch --debounce 500

# Disable recursion if not needed
$ lr --watch --no-recursive
```

### Too Frequent Rebuilds

```bash
# Increase debounce
$ lr --watch --debounce 500

# Ignore editor temp files explicitly
$ lr --watch --ignore '*.swp' --ignore '*~'
```

### Permission Denied Errors

```bash
# Check file permissions
ls -la project/
chmod 755 project/
chmod 644 project/*.lr
```

### Watch Process Using High CPU

```bash
# Reduce frequency (slower debounce)
$ lr --watch --debounce 1000

# Limit recursion depth
$ lr --watch --max-depth 1

# Use polling (slower but predictable)
$ LR_WATCH_POLLING=1 lr --watch
```
