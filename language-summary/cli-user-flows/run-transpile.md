# Run & Transpile Commands

Execute Left-Right files with transpilation to Rust (default) or Node. The full pipeline: parse → transpile → compile → execute.

## Quick Start

```bash
# Execute with Rust (default)
lr "path/to/file.lr"

# Execute with Node
lr "path/to/file.lr" --target node

# Transpile only, don't execute
lr "path/to/file.lr" --output output.rs --no-execute

# Show diagnostics but don't run
lr "path/to/file.lr" --dry-run
```

## Execution Pipeline

```
┌─────────────┐
│  .lr file   │
└──────┬──────┘
       │
       ▼
┌─────────────────────────────────┐
│  1. Parse (Lexer + Parser)  │
│  → AST                       │
└──────┬────────────────────────┘
       │
       ▼
┌─────────────────────────────────┐
│  2. Semantic Analysis        │
│  → Type checking             │
│  → Scope resolution          │
└──────┬────────────────────────┘
       │
       ▼
┌─────────────────────────────────┐
│  3. Load Operator Files    │
│  → operators-recursive.lr   │
│  → operators-rust-optimal.lr │
│  → operators-js-optimal.lr   │
│  → Override cascade (intra  │
│    > folder > project >      │
│    global)                   │
└──────┬────────────────────────┘
       │
       ▼
┌─────────────────────────────────┐
│  4. Transpile               │
│  → Rust OR JS code           │
└──────┬────────────────────────┘
       │
       ├──────────────┬──────────────┐
       ▼              ▼              ▼
┌─────────────┐ ┌─────────────┐ ┌─────────────┐
│  Rust path  │ │  Node path  │ │ No-execute  │
└──────┬──────┘ └──────┬──────┘ └──────┬──────┘
       │                 │                 │
       ▼                 ▼                 ▼
┌─────────────┐ ┌─────────────┐ ┌─────────────┐
│  5. Compile  │ │  5. Direct  │ │  5. Output  │
│  rustc       │ │  node exec   │ │  transpiled  │
└──────┬──────┘ └──────┬──────┘ └──────┬──────┘
       │                 │                 │
       ▼                 ▼                 │
┌─────────────────────────────────┐         │
│  6. Execute                   │         │
│  → Binary / Node process       │         │
└──────┬────────────────────────┘         │
       │                                  │
       └──────────────────────────────────────┘
                  │
                  ▼
           ┌─────────────┐
           │  7. Output  │
           └─────────────┘
```

## Operator Override Cascade

During execution (step 3), operators are resolved through a priority cascade:

```
┌─────────────────────────────────────────────────────────────┐
│  Operator Override Priority (high to low)                   │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  1. Intra-script override (in file header)     │
│     → Highest priority, most specific                            │
│                                                              │
│  2. Folder override (./operators.lr)         │
│     → Applies to all files in directory                        │
│                                                              │
│  3. Project override (lr.toml operators.lr)                  │
│     → Project-wide operator definitions                        │
│                                                              │
│  4. Global override (~/.config/left-right/custom-ops.lr)     │
│     → Default for all Left-Right code                         │
│                                                              │
│  5. Transpiler defaults (operators-*.lr)                      │
│     → Fallback if no override present                         │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### Override Resolution

For each operator used in code, the transpiler checks each level:

```bash
$ lr "my-script.lr" --verbose
[INFO] Parsing my-script.lr...
[INFO] AST generated: 15 nodes
[INFO] Resolving operators...
[INFO]   Operator `+`: intra-script override found
[INFO]   Operator `>>`: folder override found
[INFO]   Operator `@`: using project override
[INFO]   Operator `filter`: using global override
[INFO]   Operator `reduce`: using operators-recursive.lr
[INFO] Transpiling to Rust...
```

### Override Example

`script.lr` with intra-script override:
```left-right
#! override: operators-rust-optimal.lr

{
  result: data >> filter [x => x > 5]
}
```

Execution checks overrides in order:
1. Intra-script: Uses `operators-rust-optimal.lr` (shebang override)
2. Folder: Skipped (intra-script present)
3. Project: Skipped (intra-script present)
4. Global: Skipped (intra-script present)
5. Default: Skipped (override found)

## Basic Execution

### Simple File

`example.lr`:
```left-right
{
  greeting: `Hello, world!`,
  result: greeting >> ` ` >> `Welcome to Left-Right!`
}
```

Execute:
```bash
$ lr "example.lr"
{ greeting: "Hello, world!", result: "Hello, world! Welcome to Left-Right!" }
```

### With Imports

`main.lr`:
```left-right
{
  utils: file[`./utils.lr`],
  message: utils.greeting >> ` ` >> `This is imported!`
}
```

`utils.lr`:
```left-right
{
  greeting: `Hello from utils`
}
```

Execute:
```bash
$ lr "main.lr"
{ utils: { greeting: `Hello from utils` }, message: `Hello from utils This is imported!` }
```

## Rust Target (Default)

### Pipeline Steps

1. **Parse** — Left-Right AST
2. **Transpile** — Generate Rust code
3. **Compile** — Invoke `rustc` (temp binary)
4. **Execute** — Run binary, capture stdout
5. **Cleanup** — Remove temp files

### Transpiled Rust Example

Input (`sum.lr`):
```left-right
{
  numbers: [1, 2, 3, 4, 5],
  sum: numbers >> reduce [+]
}
```

Transpiled Rust:
```rust
fn main() {
    let numbers = vec
![1, 2, 3, 4, 5];
    let sum = numbers.into_iter().fold(0, |acc, x| acc + x);
    println
!("{}", serde_json::to_string(&serde_json::json
!({ numbers, sum })).unwrap())
}
```

### Execution Flow

```bash
$ lr "sum.lr" --verbose
[INFO] Parsing sum.lr...
[INFO] AST generated: 12 nodes
[INFO] Transpiling to Rust...
[INFO] Generated: /tmp/lr_sum_abc123.rs
[INFO] Compiling with rustc...
[INFO] Binary: /tmp/lr_sum_abc123
[INFO] Executing...
{ "numbers": [1, 2, 3, 4, 5], "sum": 15 }
[INFO] Exit code: 0
[INFO] Cleanup: removing temp files
```

### Performance Characteristics

| Operation | Time (relative) | Notes |
|-----------|-----------------|-------|
| Parse | ~1x | Fast, linear in source size |
| Transpile | ~2x | AST → Rust traversal |
| Compile | ~10-50x | rustc is the bottleneck |
| Execute | ~0.1x | Rust is very fast |
| Total | ~13-53x | Compiling dominates |

**Optimization**: For repeated execution, use `--output` to save transpiled code, then compile manually.

```bash
# Transpile once
lr "heavy-computation.lr" --output computation.rs

# Compile optimized
rustc -O computation.rs -o computation

# Run many times fast
./computation
```

## Node Target

### Pipeline Steps

1. **Parse** — Left-Right AST
2. **Transpile** — Generate JavaScript code
3. **Execute** — Invoke `node` with transpiled code
4. **Output** — Capture stdout

### Transpiled JavaScript Example

Input (`sum.lr`):
```left-right
{
  numbers: [1, 2, 3, 4, 5],
  sum: numbers >> reduce [+]
}
```

Transpiled JavaScript:
```javascript
const numbers = [1, 2, 3, 4, 5];
const sum = numbers.reduce((acc, x) => acc + x, 0);
console.log(JSON.stringify({ numbers, sum }));
```

### Execution Flow

```bash
$ lr "sum.lr" --target node --verbose
[INFO] Parsing sum.lr...
[INFO] AST generated: 12 nodes
[INFO] Transpiling to JavaScript...
[INFO] Generated: /tmp/lr_sum_abc123.js
[INFO] Executing with node...
{ "numbers": [1, 2, 3, 4, 5], "sum": 15 }
[INFO] Exit code: 0
[INFO] Cleanup: removing temp files
```

### Performance Characteristics

| Operation | Time (relative) | Notes |
|-----------|-----------------|-------|
| Parse | ~1x | Fast, linear |
| Transpile | ~2x | AST → JS traversal |
| Execute | ~10x | Node is slower than Rust |
| Total | ~13x | No compile step |

**When to use Node target**:
- Faster iteration (no compilation)
- Easier debugging with `node --inspect`
- Compatibility with existing JS tooling
- Prototyping

## Error Reporting

### Parse Errors

`invalid.lr`:
```left-right
{
  missing_closing_brace: [1, 2, 3
}
```

```bash
$ lr "invalid.lr"
error: Expected '}' at line 2, column 24
  |
2 |   missing_closing_brace: [1, 2, 3
  |                        ^^^^^^^^^^^^
```

### Transpilation Errors

`invalid-operator.lr`:
```left-right
{
  result: 5 **@ 3  # @ cannot follow **
}
```

```bash
$ lr "invalid-operator.lr"
error: Invalid operator sequence: '**@' at line 1, column 12
  |
1 |   result: 5 **@ 3
  |            ^^^^
```

### Runtime Errors

`division-zero.lr`:
```left-right
{
  result: 10 / 0
}
```

```bash
$ lr "division-zero.lr"
error: Runtime error: Division by zero at line 1, column 12
  |
1 |   result: 10 / 0
  |            ^^^
Stack trace:
  - main() at line 1
  - / operator handler
```

### Error Codes

| Code | Meaning |
|------|---------|
| E001 | Syntax error |
| E002 | Unexpected token |
| E003 | Invalid operator sequence |
| T001 | Type mismatch |
| T002 | Operator not defined for type |
| R001 | Division by zero |
| R002 | Index out of bounds |
| R003 | Stack overflow |

## Output Format Options

### Default Output

```bash
$ lr "data.lr"
{ "numbers": [1, 2, 3], "doubled": [2, 4, 6] }
```

### Pretty Output

```bash
$ lr "data.lr" --pretty
{
  "numbers": [1, 2, 3],
  "doubled": [2, 4, 6]
}
```

### Compact Output

```bash
$ lr "data.lr" --compact
{"numbers":[1,2,3],"doubled":[2,4,6]}
```

### Output as Value Only

```bash
$ lr "value.lr" --raw
42
```

`value.lr`:
```left-right
{ value: 42 }
```

### Output to File

```bash
$ lr "data.lr" --output results.json
$ cat results.json
{ "numbers": [1, 2, 3], "doubled": [2, 4, 6] }
```

## Exit Codes

| Code | Meaning | Example Cause |
|------|---------|---------------|
| 0 | Success | Program executed normally |
| 1 | Parse error | Syntax error, unexpected token |
| 2 | Transpilation error | Invalid operator, type error |
| 3 | Runtime error | Division by zero, index error |
| 4 | File not found | Path doesn't exist |
| 5 | Invalid configuration | Config parse error |
| 6 | Compile error (Rust) | rustc failed |
| 7 | Node not found | --target node but node unavailable |
| 8 | Rust not found | Default target but rustc unavailable |
| 128 | Interrupted | User pressed Ctrl+C |

### Checking Exit Codes

```bash
lr "script.lr"
echo $?

# Check in scripts
if lr "script.lr"; then
  echo "Success"
else
  case $? in
    1) echo "Parse error" ;;
    2) echo "Transpile error" ;;
    3) echo "Runtime error" ;;
  esac
fi
```

## Environment Variable Overrides

```bash
# Override default target
export LR_TARGET=node
lr "file.lr"  # Uses Node instead of Rust

# Override config path
export LR_CONFIG=/custom/config.toml
lr "file.lr"  # Uses custom config

# Enable debug logging
export LR_DEBUG=1
lr "file.lr"  # Shows detailed logs

# Set temp directory
export LR_TEMP_DIR=/tmp/lr-builds
lr "file.lr"  # Uses custom temp dir

# Disable colors
export LR_NO_COLOR=1
lr "file.lr"  # No ANSI colors
```

### Environment Variable Precedence

1. CLI flags (`--target node`)
2. Environment variables (`LR_TARGET`)
3. Config file (`default_target`)

## Per-Run Target Specification

### Using CLI Flag

```bash
# Rust (default)
lr "file.lr"

# Node
lr "file.lr" --target node

# Node (short form)
lr "file.lr" -t node
```

### Using File Shebang

`script.lr`:
```left-right
#! lr --target node

{
  message: `Running with Node target`
}
```

```bash
chmod +x script.lr
./script.lr  # Uses Node due to shebang
```

### Using Project Config

`lr.toml`:
```toml
[project]
target = "node"  # Override global config for this project
```

```bash
# In directory with lr.toml
lr "file.lr"  # Uses Node (from project config)
```

## Transpile-Only Mode

### Generate Rust Code

```bash
lr "script.lr" --output script.rs --no-execute
# Creates script.rs without running
```

### Generate JavaScript Code

```bash
lr "script.lr" --target node --output script.js --no-execute
# Creates script.js without running
```

### Dry Run

```bash
lr "script.lr" --dry-run
# Shows what would happen without doing it

# Output example:
# Target: Rust
# Parse: 12 nodes, 45 tokens
# Transpile: 234 lines of Rust
# Compile: Would run: rustc /tmp/lr_xxx.rs -o /tmp/lr_xxx
# Execute: Would run: /tmp/lr_xxx
```

## Advanced Usage

### Chaining with Shell Pipes

```bash
# Transpile to Rust, pipe to grep
lr "data.lr" --output - --no-execute | grep "fn main"

# Transpile to JS, pipe to uglify
lr "data.lr" --target node --output - --no-execute | uglifyjs -c
```

### Multiple Files

```bash
# Execute all .lr files
for f in *.lr; do lr "$f"; done

# Execute only changed files (using git)
git diff --name-only | grep '\.lr$' | xargs -I {} lr {}
```

### Conditional Execution

```bash
# Only run if parse succeeds
if lr "validate.lr" --check-only; then
  lr "build.lr"
else
  echo "Validation failed"
fi
```

### Performance Profiling

```bash
# Rust target with release optimization
lr "heavy.lr" --target rust --release

# Node target with V8 profiling
lr "heavy.lr" --target node --profile

# Time execution
time lr "script.lr"
```

## Comparison: Rust vs Node Targets

| Aspect | Rust | Node |
|---------|-------|------|
| Execution speed | 10-100x faster | Slower |
| Cold start | ~2s (compile) | ~0.1s |
| Warm start | ~0.01s | ~0.1s |
| Debugging | gdb/lldb | node --inspect |
| Binary size | ~2-5MB | N/A (interpreted) |
| Portability | Platform-specific | Cross-platform |
| Ecosystem | crates.io | npm |
| Use case | Production, performance | Prototyping, debugging |

## Troubleshooting

### "rustc not found"

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### "node not found" with `--target node`

```bash
# Install Node.js (using nvm)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install --lts
```

### Compilation Errors (Rust)

```bash
# View detailed rustc output
lr "file.lr" --target rust --verbose

# Try with debug symbols
lr "file.lr" --target rust --debug
```

### Permission Denied on Output

```bash
# Specify output directory explicitly
lr "file.lr" --output ./output/

# Fix permissions
chmod 755 output/
```
