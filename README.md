<div styles="display: flex; font-size: 35px; align-items: center; justify-content: center;"><img src="docs/logo/logo.svg" alt="Left-Right Logo" styles="height: 45px; padding-top: 3px; padding-bottom: 3px; padding-left: 7px;" /> *Left-Right*</div>

> *Previously known as PenRoScript*

---

A point-free, operator-based, hierarchical-data oriented general-purpose scripting language inspired by array-oriented languages.

## Overview

Left-Right (`lr`) is a novel programming language designed for clarity and simplicity. It combines the power of function composition with a syntax that reads naturally left-to-right.

### Core Characteristics

- **Point-free syntax**: No named variables in common cases - values flow through operators
- **Operator-based**: Operators are first-class values that can be stored, passed, and returned
- **Hierarchical-Data Oriented**: Inspired by array-oriented languages, optimized for hierarchical data structures (maps, lists)
- **Simply loosely typed**: Runtime type inference with optional type checking.
  > Only types are `Operators`, `Maps`, `Lists`, `Numbers`, `Strings`, `Boolean`, and `Undefined`
- **Left-to-right evaluation**: Natural reading order, no operator precedence

### Key Features

- **JSON-like structure**: Both data and programs use the similar syntax
- **Operators instead of functions**: Uses closures (maps with `_<` and `_>`) instead
- **Type-dependent operator behavior**: Same operator adapts to input types
- **Auto-currying**: Operator partial application happens automatically
- **Spatial symbology**: Expressive notation (e.g., `"^` for uppercase, `"^_` for capitalize)
- **Backtick strings**: All strings use backtick delimiters (`` `like this` ``) — `"` is a reserved operator, not a string delimiter
- **Compiles to Rust**: Compiled to native Rust

### Core Operators

Left-Right provides powerful operators for data transformation:

**Core Operators:**
- **Assignment**: `{ a: 1, b: a+2}` -> `{ a: 1, b: 3 }`
- **Access/Get**: `data@\`keyString\`` or `data@[\`firstKeyString\`, \`nestedKeyString\`]` — access nested properties
- **Size**: `list #` — length of collection types (*list, map, or string*)
- **Concat**: `[] + item` — concatenate collection types
- **Spread**: `{ a: 1, +: { a:3, b:2 }, b: 4 }` -> `{ a:3, b:4 }` — spread into context

**String Operators:**
- **Uppercase**: `` `hello` "^ `` → `` `HELLO` ``
- **Lowercase**: `` `HELLO` "_ `` → `` `hello` ``
- **Capitalize**: `` `hello` "^_ `` → `` `Hello` ``
- **Replace**: `` `hello` "~ [`e`, `a`] `` → `` `hallo` ``
- **Split**: `` `a,b,c` <> `,` `` → `` [`a`, `b`, `c`] ``
- **Join**: `` [`a`, `b`] >< `,` `` → `` `a,b` ``

**Boolean Operators:**
- **AND**: `true & false` → false
- **OR/Default**: `true | false` → true, `value | default` → default if falsy (0, *empty string*, [], {}, undefined)
- **Negate**: ` true !` → false

**Type Checks:**
- **IsString**: `value ?"` → true if string
- **IsNumber**: `value ?#` → true if number
- **ToBoolean**: `value ?` → truthy check
- **Contains**: `list ?>< item` → true if item in list

**Filter Comparison Operators:**
- **Filter Greater**: `[1,2,3] $?> 2` → `[3]`
- **Filter Less**: `[1,2,3] $?< 2` → `[1]`
- **Filter Plus/Minus**: `[1,2,3] $?+ 2` → `[3]`, `[1,2,3] $?- 2` → `[1]`

**Element-wise Operators:**
- **Add**: `[1,2] $+ 3` → `[4,5]`
- **Subtract**: `[5,6] $- 2` → `[3,4]`
- **Multiply**: `[1,2] $* 3` → `[3,6]`
- **Divide**: `[6,8] $/ 2` → `[3,4]`

**Map-Each-Property:**
- **Pluck**: `[{a:1},{a:2}] $@ \`a\`` → `[1,2]`

**Error Handling:**
- **Throw**: `` `error` !!! `` — throw error
- **Catch**: `` `error` !!! !!!? `caught` `` — try/catch
- **Error Constructor**: `` Error[\`message\`] `` — create error value

**Optional Apply:**
- **`!!`**: `value !!` — apply if truthy, return undefined if falsy

**Guards:**
- **`?:`**: `{ a: 1, a?: { 99 } }` — guard pattern, execute if truthy

**Map Equality:**
- **Equal**: `{ a: 1 } = { a: 1 }` → true
- **Not Equal**: `{ a: 1 } != { a: 2 }` → true

**List Operators:**
- **Iterate**: `list $ { _< * 2 }` — transform each element
- **Filter**: `list $? { _<  > 3 }` — keep elements matching condition
- **Some/Any**: `list $| { _< > 0 }` — test if any element matches
- **Every/All**: `list $& { _< > 0 }` — test if all elements match
- **Find**: `list $?| { _< > 10 }` — find first matching element
- **Flatmap**: `list $_ { _< * 2 }` — map then flatten
- **UniqueBy**: `list $~ { _< }` — remove duplicates by key
- **GroupBy**: `list $> { _< }` — group elements by key function
- **Sort**: `list $%` — sort collection
- **Compact**: `list $?!` — remove undefined/null values

**Control Flow Operators:**

### Defining Operators

Operators are defined as maps with argument placeholders:

**Monadic** (single argument):
```lr
double: { _< * 2 }
```

**Diadic** (two arguments, auto-currying):
```lr
add: { _< + _> }
```

**Multi-argument** (positional via list):
```lr
lookup: { key: _<@0, value: _<@1 }
```

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

### Running Code

```bash
# Execute a file
lr run myfile.lr

# Start the REPL
lr repl
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

**CLI**: `lr` | **Paradigm**: Point-free, Operator-based, Array-oriented