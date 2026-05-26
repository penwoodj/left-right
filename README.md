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
- **Spatial/compounding symbology**: Expressive notation (e.g., `"^` for uppercase, `"^_` for capitalize)
- **Auto-currying**: Partial application happens automatically
- **JSON-like structure**: Both data and programs use the same syntax
- **No functions**: Uses closures (maps with `_<` and `_>`) instead

### Core Operators

Left-Right provides powerful operators for data transformation:

**Loop Operators:**
- **Iterate**: `list $ { _< * 2 }` — transform each element
- **Filter**: `list $? { _< > 3 }` — keep elements matching condition
- **Some/Any**: `list $| { _< > 0 }` — test if any element matches
- **Every/All**: `list $& { _< > 0 }` — test if all elements match
- **Find**: `list $?| { _< > 10 }` — find first matching element
- **Flatmap**: `lists $_` — flatten nested lists
- **UniqueBy**: `list $~` — remove duplicates
- **GroupBy**: `list $>` — group elements by key

**String Operators:**
- **Uppercase**: `"hello" "^` → "HELLO"
- **Lowercase**: `"HELLO" "_` → "hello"
- **Capitalize**: `"hello" "^_` → "Hello"
- **Replace**: `"hello" "~ ["e","a"]` → "hallo"
- **Split**: `"a,b,c" <> ,` → ["a","b","c"]
- **Join**: `["a","b"] >< ,` → "a,b"

**Core Operators:**
- **Assignment**: `config: files@`key``
- **Access**: `data@`key`` or `data@[`key1`, `key2`]` — access nested properties
- **Size**: `list #` — length of list or string
- **Concat**: `[] + item` — concatenate lists/strings
- **Spread**: `... + list` — spread into context
- **AND**: `true & false` → false
- **OR/Default**: `true | false` → true, `value | default` → default if falsy
- **Negate**: `! true` → false
- **ToBoolean**: `value ?` → truthy check
- **ToString**: `5 "` → "5"

**Compound Operators:**
- **IsString**: `value ?"` → true if string
- **IsNumber**: `value ?#` → true if number
- **Contains**: `list ?> item` → true if item in list
- **Throw**: `!!! "error"` — throw error
- **Catch**: `!!!? { tryBlock, catchBlock }` — try/catch
- **Early Return**: `value ?: { returnBlock }` — guard pattern

**Types**: Operator, Map, List, String, Boolean, Number, Undefined

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

```lr
# Iterate over a list — every operator is left-to-right
[1, 2, 3, 4, 5] $ { _< + 10 }
# Output: [11, 12, 13, 14, 15]

# Filter with a condition
[1, 2, 3, 4, 5] $? { _< > 3 }
# Output: [4, 5]

# Define and call a closure (not a function)
double: { _< * 2 }
double 5
# Output: 10

# String interpolation
name: `Alice`,
`hello {name}`
# Output: `hello Alice`

# Chain operations
[1, 2, 3, 4, 5] $? { _< > 2 } $ { _< * 2 }
# Output: [6, 8, 10]
```

### Running Code

```bash
# Execute a file
lr myfile.lr

# Run a single expression
lr -e "[1, 2, 3] $ { _< + 10 }"

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
4. **Clean transpilation**: Targets JS and Rust without hacks

The language is inspired by APL, J, K, BQN, Haskell, Clojure, and lodash/FP — but with its own distinct voice.

### Expression Model

Left-Right uses left-hungry curried operators. All operators are left-associative with no operator precedence. Every expression evaluates strictly left-to-right:

```lr
5 + 3 * 2
# Evaluates as: ((5 + 3) * 2) → 16, not 11
```

This eliminates ambiguity and makes code predictably linear.

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