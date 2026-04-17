# TUI Shell — Interactive Left-Right REPL

Running `lr` without arguments launches an interactive Terminal User Interface (TUI) shell — a REPL with semantic editing capabilities powered by Ink (React-like Rust TUI framework).

## Quick Start

```bash
# Launch TUI shell
$ lr
```

## TUI Layout

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  Left-Right REPL  v0.1.0  │  Target: Rust  │  Config: ~/.lrconfig  │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌─────────────────┐  ┌─────────────────────────────────────────────┐ │
│  │   Input Panel   │  │         Output / Results Area            │ │
│  │                 │  │                                         │ │
│  │ > {            │  │  {                                     │ │
│  │     data: [1,2,│  │    result: [3, 4, 5]                 │ │
│  │     3],          │  │  }                                     │ │
│  │     doubled:     │  │                                         │ │
│  │       data >>    │  │  [Ready - Type expression or command]      │ │
│  │       map [x =>  │  │                                         │ │
│  │         x * 2]  │  │                                         │ │
│  │   }             │  │                                         │ │
│  │                 │  │                                         │ │
│  │                ↵│  │                                         │ │
│  └─────────────────┘  └─────────────────────────────────────────────┘ │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────────────┐ │
│  │  Semantic Editor  │  History  │  AST View  │  Help  │
│  └─────────────────────────────────────────────────────────────────────┘ │
│  ↑↓: History  │  Tab: Auto-complete  │  Ctrl+R: Search  │  F1: Help │
└─────────────────────────────────────────────────────────────────────────────┘
```

## Components

### 1. Input Panel (Left)

Primary REPL area for typing Left-Right expressions.

Features:
- **Multi-line editing** — Press `Enter` to submit, `Shift+Enter` for new line
- **Syntax highlighting** — Real-time highlighting of operators, strings, numbers
- **Auto-indentation** — Smart indentation for nested structures
- **Bracket matching** — Highlight matching `{}`, `[]`, `()`, `<>` pairs
- **Error squiggles** — Underline syntax errors as you type

### 2. Output Area (Right)

Displays evaluation results, errors, and diagnostics.

Display modes:
- **Results** — Normal evaluation output
- **Errors** — Parse/transpile/runtime errors with line/column info
- **Diagnostics** — Warnings, type hints, suggestions
- **Intermediate** — Show AST, transpiled code (toggle with `Ctrl+T`)

### 3. Semantic Editor (Bottom Tab)

Edit language semantics live — modify operators, define new ones, adjust precedence.

Access with: `Ctrl+E` or click "Semantic Editor" tab (if mouse supported).

### 4. History Panel (Bottom Tab)

Command history with search and filtering.

Access with: `Ctrl+H` or click "History" tab.

### 5. AST View (Bottom Tab)

Visualize abstract syntax tree of current expression.

Access with: `Ctrl+A` or click "AST View" tab.

## Keyboard Shortcuts

### REPL Navigation

| Shortcut | Action |
|----------|--------|
| `↑` / `Ctrl+P` | Previous history entry |
| `↓` / `Ctrl+N` | Next history entry |
| `Ctrl+R` | Reverse search history |
| `Tab` | Auto-complete identifier/operator |
| `Shift+Tab` | Navigate between panels |
| `Ctrl+L` | Clear input (preserves output) |
| `Ctrl+K` | Clear to end of line |
| `Ctrl+U` | Clear to beginning of line |
| `Enter` | Submit expression |
| `Shift+Enter` | New line (don't submit) |

### Output Management

| Shortcut | Action |
|----------|--------|
| `Ctrl+C` | Cancel current evaluation |
| `Ctrl+O` | Toggle output mode (compact/verbose) |
| `Ctrl+T` | Toggle intermediate view (AST/transpiled) |
| `Ctrl+S` | Save output to file |
| `Ctrl+Shift+S` | Save session transcript |

### Semantic Editor

| Shortcut | Action |
|----------|--------|
| `Ctrl+E` | Open semantic editor tab |
| `Ctrl+Shift+E` | Export semantic config to file |
| `Ctrl+I` | Import semantic config from file |
| `Ctrl+R` (in editor) | Reset semantics to defaults |

### Session Management

| Shortcut | Action |
|----------|--------|
| `Ctrl+X` | Export session to `.lr-session` file |
| `Ctrl+Q` | Quit TUI (prompts to save) |
| `Ctrl+D` | Quit (no prompt) |
| `Ctrl+Z` | Suspend (background job in shell) |

## Clickable Interface Support

Ink supports basic mouse interaction through the `ink` library's event system.

### Supported Mouse Actions

| Action | Function |
|---------|----------|
| Left-click on tab | Switch to that tab panel |
| Left-click on history item | Load into input panel |
| Left-click on AST node | Jump to corresponding source location |
| Scroll wheel | Scroll output/history panels |

### Limitations

- Text selection/copy may vary by terminal emulator
- Right-click context menus not available (terminal limitation)
- Drag-and-drop not supported

## Semantic Editor

Edit Left-Right language semantics without restarting the shell.

### What Can You Edit?

1. **Operator definitions** — Change how `+`, `-`, `@`, etc. behave
2. **Precedence table** — Reorder operator precedence
3. **Type dispatch** — Add or modify type-specific behavior
4. **Custom operators** — Define new symbols with behavior
5. **Directional sections** — Adjust `_>` and `_<` behavior
6. **Operator overrides** — Set override levels (intra-script > folder > project > global)

### Semantic Editor UI

```
┌─────────────────────────────────────────────────────────────────────┐
│  Semantic Editor                                    [Reset] [Save] │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌─────────────────────────┐  ┌───────────────────────────────┐    │
│  │  Operator Table         │  │  Precedence Table           │    │
│  │                         │  │                             │    │
│  │  +  (add)             │  │  1. @ (get)               │    │
│  │  -  (subtract)         │  │  2. +, - (math)          │    │
│  │  *  (multiply)         │  │  3. *, / (multiply/divide) │    │
│  │  /  (divide)           │  │  4. >> (pipe)             │    │
│  │  >> (pipe)            │  │  5. _>, _< (directional)  │    │
│  │  _> (right-section)   │  │  6. &, | (logical)         │    │
│  │  _< (left-section)     │  │  7. ~ (not)               │    │
│  │                         │  │                             │    │
│  │  [Add New Operator]     │  │  [Reorder]                 │    │
│  └─────────────────────────┘  └───────────────────────────────┘    │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────────────┐ │
│  │  Type Dispatch (Number)                                    │ │
│  │                                                             │ │
│  │  +: { lhs, rhs => lhs + rhs }                               │ │
│  │  -: { lhs, rhs => lhs - rhs }                               │ │
│  │  *: { lhs, rhs => lhs * rhs }                               │ │
│  │                                                             │ │
│  └─────────────────────────────────────────────────────────────────────┘ │
│                                                                     │
│  Tab: Operators  │  Tab: Precedence  │  Tab: Types  │  Tab: Custom │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Example: Adding Custom Operator

```left-right
# In Semantic Editor:
# Define new operator `**` for exponentiation
**:
  precedence: 10  # Higher than *, /
  associativity: right
  implementation:
    { base, exp => base ^ exp }  # Uses Rust's pow
```

Now usable in REPL:

```left-right
> { result: 2 ** 8 }
# result: 256
```

## Session Management

### Saving Sessions

```bash
# Save current session
Ctrl+X  # Prompts for filename
Enter: my-session.lr-session

# Load session later
lr --load my-session.lr-session
```

### Session File Format

`.lr-session` files contain:

```toml
[metadata]
created = "2026-04-15T10:30:00Z"
version = "0.1.0"
target = "rust"

[history]
[[entry]]
timestamp = "2026-04-15T10:30:15Z"
expression = "{ data: [1,2,3] }"
result = "{ data: [1, 2, 3] }"

[[entry]]
timestamp = "2026-04-15T10:30:45Z"
expression = "data >> map [x => x * 2]"
result = "[2, 4, 6]"

[semantics]
# Custom operator definitions
custom_operators = ["**", "??"]
```

### Session Persistence

Automatic saving options (configurable):

```toml
# ~/.config/left-right/config.toml
[tui]
autosave = true
autosave_interval = 60  # seconds
session_file = "~/.local/share/left-right/session.lr-session"
```

## History and Autocomplete

### History Search

```bash
# Reverse search (Ctrl+R)
(reverse-search): map

# Shows:
# 3: data >> map [x => x * 2]
# 2: list >> map [item => item.name]
# 1: results >> map [r => r.value]

# Press Enter to select, or continue typing
(reverse-search): data >> map filter
# Refines to entry 3
```

### Autocomplete

Press `Tab` to trigger autocomplete at cursor:

```left-right
> data >> m<Tab>
# Suggests:
# map
# mapWithIndex
# max
# min
# merge
```

Context-aware suggestions:
- Operators starting with `@`: `@`, `@@`, `@|`, etc.
- Collection methods after `>>`: `map`, `filter`, `reduce`, etc.
- File paths after `file`: file paths in project

## Comparison with Other REPLs

### vs Node REPL

| Feature | Node REPL | Left-Right TUI |
|---------|------------|-----------------|
| Multi-line edit | Limited | Full support |
| Semantic editing | No | Yes |
| AST inspection | No | Yes |
| History search | Basic | Advanced |
| Mouse support | No | Yes (Ink) |
| Configurable semantics | No | Yes |

### vs Python REPL

| Feature | Python REPL | Left-Right TUI |
|-------------|--------------|-----------------|
| Multi-line edit | Yes | Yes |
| Semantic editing | No | Yes |
| Operator precedence change | No | Yes |
| History search | Yes | Yes |
| Visual AST | No | Yes |

### vs GHCi (Haskell)

| Feature | GHCi | Left-Right TUI |
|-------------|-------|-----------------|
| Type inference | Yes | Planned |
| Multi-line edit | Yes | Yes |
| :kind/:type | Yes | Similar with `Ctrl+T` |
| Module loading | Yes | Yes (file imports) |
| Custom operators | No | Yes |

## Ink-Based Implementation

### Architecture

```rust
// Main TUI structure (Ink components)
use ink::components::*;
use crossterm::event::{read, Event, KeyCode, KeyEvent};

struct TuiApp {
    input: InputPanel,
    output: OutputPanel,
    current_tab: Tab,  // SemanticEditor | History | AstView
    semantics: SemanticsStore,
    history: HistoryBuffer,
}

enum Tab {
    SemanticEditor,
    History,
    AstView,
}

impl Component for TuiApp {
    fn render(&self) -> Element {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40),  // Input
                Constraint::Percentage(60),  // Output
            ])
            .split(vec![
                Box::new(self.input.render()),
                Box::new(self.output.render()),
            ])
            .push(self.current_tab.render())
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(KeyEvent::Char(c)) => self.input.type_char(c),
            Event::Key(KeyEvent::Enter) => self.evaluate(),
            Event::Key(KeyEvent::Ctrl('r')) => self.search_history(),
            // ... more event handlers
        }
    }
}
```

### Async Evaluation

Transpilation and execution don't block the UI:

```rust
async fn evaluate_expression(app: &mut TuiApp, expr: String) {
    // Parse (non-blocking)
    let ast = parse_async(&expr).await?;

    // Transpile (non-blocking)
    let transpiled = transpile_async(ast.clone()).await?;

    // Execute in target runtime
    let result = match app.config.target {
        Target::Rust => execute_rust(transpiled).await?,
        Target::Node => execute_node(transpiled).await?,
    };

    // Update UI (thread-safe)
    app.output.push(result);
    app.history.add(expr, result);
}
```

### Terminal Compatibility

Ink supports:
- Linux: xterm, gnome-terminal, kitty, alacritty
- macOS: Terminal.app, iTerm2, Warp
- Windows: Windows Terminal, PowerShell

Tested terminals:
```bash
# Test terminal capabilities
lr --test-terminal
```

## Advanced Features

### Multi-Session Support

Open multiple TUI instances:

```bash
# Terminal 1: Data experimentation
lr --session data

# Terminal 2: Operator design
lr --session semantics
```

Session isolation:
- Separate history buffers
- Independent semantic edits
- No shared state

### Remote Collaboration (Planned)

```bash
# Host shared session
lr --share --session collab

# Join shared session
lr --join collab@host:port
```

### Plugin System (Planned)

```rust
// TUI plugin API
trait TuiPlugin {
    fn name(&self) -> String;
    fn on_eval(&mut self, expr: &str, result: &Value);
    fn render_panel(&self) -> Option<Element>;
}
```

Example plugins:
- Data visualization (charts/graphs)
- Profiler overlay
- Git integration panel

## Performance

Startup time: < 100ms
Render frame: ~16ms (60 FPS)
Memory: < 50MB baseline

Optimization tips:
- Limit history size (`tui.history_limit = 1000`)
- Disable AST rendering for large outputs
- Use `--no-color` on slow terminals

## Troubleshooting

### TUI Rendering Issues

```bash
# Force 256-color mode
lr --force-color

# Disable mouse support
lr --no-mouse

# Fallback to simple mode
lr --simple
```

### Frozen TUI

```bash
# Kill gracefully
Ctrl+C  # Once to cancel eval, twice to quit

# Force kill if frozen
pkill -9 lr
```

### History Not Persisting

```bash
# Check permissions
ls -l ~/.local/share/left-right/

# Fix permissions
chmod 755 ~/.local/share/left-right/
```
