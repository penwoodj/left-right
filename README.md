# Left-Right

<div align="center">
  <img src="logo.svg" alt="Left-Right Logo" width="256" />
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
- **Type Check**: `value ?= 'type': trueCase, falseCase` — ternary type check
- **Path Access**: `data @['key']` — access nested properties (array path idiomatic)
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

```bash
# From source (Rust)
cargo install left-right

# Or download pre-built binary
curl -fsSL https://raw.githubusercontent.com/user/left-right/main/install.sh | sh
```

### Hello World

```javascript
// Simple pipeline: take first element, transform it
[{ name: `Alice` }, { name: `Bob` }]  <  .name
// Output: `Alice`

// Map over list with operator
[1, 2, 3, 4, 5]  + 10
// Output: [11, 12, 13, 14, 15]
```

### Running Code

```bash
# Execute a file
lr myfile.lr

# Transpile to JavaScript
lr --target js myfile.lr --output myfile.js

# Transpile to Rust
lr --target rust myfile.lr --output myfile.rs

# Watch mode for development
lr --watch myfile.lr
```

## Documentation

Comprehensive documentation is available in the [`language-summary/`](./language-summary/) directory:

- [Language Overview](./language-summary/00-language-overview.md) — Complete language overview and philosophy
- [Operator Reference](./language-summary/03-operator-reference.md) — All operators with examples
- [Type System](./language-summary/02-type-system.md) — Primitive types, collections, type inference
- [Evaluation Model](./language-summary/04-evaluation-model.md) — LTR evaluation, currying, composition
- [Examples](./language-summary/example-io/) — Practical examples by difficulty level

## Design Philosophy

Left-Right prioritizes:

1. **Readability over brevity**: Code should read like a story
2. **Low ceremony over strictness**: Minimal boilerplate
3. **Deterministic execution**: Clear semantics, no surprises
4. **Clean transpilation**: Targets JS/TS and Rust without hacks

The language is inspired by APL, J, K, BQN, Haskell, Clojure, and lodash/FP — but with its own distinct voice.

## Project Status

**Current Status**: Design phase — comprehensive specification complete, implementation in progress.

The transpiler is written in Rust and generates JavaScript/TypeScript and Rust code from Left-Right source.

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
