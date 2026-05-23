# Left-Right

<div align="center">
  <img src="docs/logo/logo.svg" alt="Left-Right Logo" width="256" />
</div>

**Left-Right was previously known as PenRoScript**

---

A point-free, operator-based, hierarchical-data oriented general-purpose programming language inspired by array-oriented languages. Code as data, taken literally.

## Overview

Left-Right (`lr`) is a novel programming language designed for clarity and expressivity in data transformation tasks. It combines the power of functional composition with a syntax that reads naturally left-to-right.

### Core Characteristics

- **Point-free syntax**: No named variables in common cases — values flow through operators
- **Operator-based**: Operators are first-class values that can be stored, passed, and returned
- **Hierarchical-Data Oriented**: Inspired by array-oriented languages, optimized for hierarchical data structures (maps, lists)
- **Loosely typed**: Runtime type inference with optional type checking
- **Left-to-right evaluation**: Natural reading order, no operator precedence

### Key Features

- **Transpiles to JavaScript and Rust**: Run in Node.js or compiled to native Rust
- **Type-dependent operator behavior**: Same operator adapts to input types
- **Spatial/compounding symbology**: Expressive notation (e.g., `^` for uppercase, `^_` for capitalize)
- **Auto-currying**: Partial application happens automatically
- **JSON-like structure**: Both data and programs use the same syntax

### Core Operators

Left-Right provides powerful operators for data transformation:

- **Filter**: `list ?{ condition }` — keep elements matching condition
- **Map**: `list ${ operation }` — transform each element
- **Some/Any**: `list ?|{ condition }` — test if any element matches
- **Includes**: `list >< item` — check if item exists in list
- **Join**: `list >< separator` — join list elements with separator
- **Unique**: `~list` — remove duplicates
- **Count**: `list ?{ condition } #,` — count filtered elements
- **Type Check**: `value ?= \`type\`: trueCase, falseCase` — ternary type check
- **Path Access**: `data @\`key\`` or `data@[\`key1\`, \`key2\`]` — access nested properties (array path idiomatic)
- **String Ops**: `text "^` (uppercase), `text "^_` (capitalize)
- **Conditional Append**: `value & 'template {var}'` — append if truthy
- **Curry Reversal**: `_` suffix on operator (e.g., `text "^_` vs `text "^`)
- **List Concat**: `[] + item1 + item2` — concatenate lists

**Types**: Operator, Map, List, Text, Number, Boolean, Undefined

**Right Notation**: `{` opens operator context, `_<` left input (first element), `_>` right input (second element), `}` closes

### The Logo

The logo `{_<_>}` reads as "left right" in Left-Right notation:

- `{` — opens the left context
- `_<` — the "left" input operator (takes first element)
- `_>` — the "right" input operator (takes second element)
- `}` — closes the right context

So this is an operator that literally reads Left Right. This simple pattern captures the essence of the language: directional flow through operators.

## Getting Started

### Installation

Download a pre-built binary from the [latest release](https://github.com/penwoodj/left-right/releases).

```bash
# Linux (x86_64)
curl -fsSL https://github.com/penwoodj/left-right/releases/latest/download/lr-x86_64-linux.tar.gz | tar xz

# macOS (Apple Silicon)
curl -fsSL https://github.com/penwoodj/left-right/releases/latest/download/lr-aarch64-macos.tar.gz | tar xz

# Windows (x86_64)
# Download lr-x86_64-windows.exe from the releases page
```

Or build from source:

```bash
cd compiler
cargo build --release
# Binary at compiler/target/release/lr
```

### Hello World

```\lr
# Map over a list — every operator is left-to-right
[1, 2, 3, 4, 5] $ _< + 10
# Output: [11, 12, 13, 14, 15]

# Filter with a condition
[1, 2, 3, 4, 5] $? _< > 3
# Output: [4, 5]

# Define and call a closure
double: {_< * 2},
double 5
# Output: 10

# String interpolation
name: `Alice`,
`hello {name}`
# Output: `hello Alice`
```

### Running Code

```bash
# Execute a file
lr myfile.lr

# Run a single expression
lr -e "[1, 2, 3] $ _< + 10"

# Start the REPL
lr
```

## Documentation

- [Language Specification](./docs/specs/left-right-language-specification.md) — Complete language specification
- [AST Specification](./docs/specs/ast-specification.md) — AST node definitions and grammar
- [Lexer Specification](./docs/specs/lexer-specification.md) — Token types and recognition rules
- [Brainstorm Documents](./docs/brainstorms/) — Original language design documents
- [Implementation Plans](./docs/plans/) — Detailed plans for lexer/AST, compiler/VM, and CLI
- [Translations](./docs/translations/) — JavaScript-to-Left-Right translation examples

## Design Philosophy

Left-Right prioritizes:

1. **Readability over brevity**: Code should read like a story
2. **Low ceremony over strictness**: Minimal boilerplate
3. **Deterministic execution**: Clear semantics, no surprises
4. **Clean transpilation**: Targets JS/TS and Rust without hacks

The language is inspired by APL, J, K, BQN, Haskell, Clojure, and lodash/FP — but with its own distinct voice.

## Project Status

**Current Status**: Alpha — core compiler, VM, and CLI implemented. Run the REPL or execute `.lr` files.

The compiler is written in Rust. It lexes, parses, compiles to bytecode, and executes via a stack-based VM with GC-managed values.

## Contributing

We welcome contributions! Areas of focus:
- Transpiler implementation
- Standard library
- Language tooling (linter, formatter, IDE support)
- Documentation improvements
- Example programs

## License

MIT License — see LICENSE file for details.

---

**CLI**: `lr` | **Paradigm**: Point-free, Operator-based, Hierarchical-Data Oriented
