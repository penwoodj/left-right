# Left-Right by Example

Complete example library for Left-Right language (CLI: `lr`). Point-free, operator-based, hierarchical-data oriented syntax inspired by array-oriented languages with JSON-like structure that transpiles to JavaScript and Rust.

## Format

Each example shows:
- **Left-Right source** — The native syntax
- **JavaScript output** — Transpiled to modern JS
- **Rust output** — Transpiled to Rust
- **Explanation** — Key concepts demonstrated

## Examples by Difficulty

### Basic
- [Basic Operations](./basic-operations.md) — Arithmetic, text, templates, types
- [Collections](./collections.md) — Maps, lists, paths, filtering, mapping

### Intermediate
- [Pipelines](./pipelines.md) — Multi-stage transforms, composition, LTR evaluation
- [Functions](./functions.md) — Lambdas, currying, point-free chains
- [Conditionals](./conditionals.md) — Type checks, predicates, equality, guards

### Advanced
- [Real-World](./real-world.md) — ServiceNow integration, ETL patterns, complex transforms
- [Interop](./interop.md) — JS/Rust integration, module system, cross-language projects

## Core Operators Reference

| Symbol | Purpose | Example |
|---------|----------|----------|
| `_< _>` | Lambda boundaries (left/right args) | `{ _< + 1 }` |
| `@` | Path access | `obj@`key``, `arr@0` |
| `$` | Apply/map | `arr${ _< * 2 }` |
| `?{` | Filter | `arr?{ _< > 5 }` |
| `$#` | Size/length | `arr$#` |
| `#` | Count | `arr#` |
| `~` | Unique | `arr~` |
| `><` | Join | `arr><`, `` |
| `?|` | Some/any | `arr?|{ _< > 0 }` |
| `?|!` | Every | `arr?|!{ _< < 0 }` |
| `^` | Uppercase | `` `hello`^ `` |
| `^_` | Capitalize | `` `hello`^_ `` |
| `"` | Lowercase | `` `HELLO`" `` |
| `?` | Type check | `value?` |
| `=` | Unordered equality | `[1,2] = [2,1]` |
| `==` | Ordered equality | `[1,2] == [1,2]` |
| `+` | Concatenation | `[] + [1]` |
| `&` | String concatenation | `` `Hello ` & `world` `` |
| `>>` | Forward composition | `f >> g` |
| `<<` | Backward composition | `g << f` |

## Quick Start

```left-right
// Basic arithmetic
5 + 3 * 2

// Text operations
`hello`^_ & ` world`

// List mapping
[1,2,3]${ _< * 2 }

// Path access
{ name: `Alice`, age: 30 }@`name`
```

## Language Features

- **Point-free** — No named variables for intermediate results
- **Operator-based** — First-class operators as primary syntax
- **Hierarchical-Data Oriented** — Inspired by array-oriented languages, optimized for hierarchical data structures
- **Loosely typed** — Type-dependent operator behavior
- **JSON-like** — Familiar data structures
- **LTR evaluation** — Left-to-right execution order
- **Auto-currying** — Functions are curried by default

## Transpilation

Left-Right transpiles to both JavaScript and Rust with:
- Deterministic output
- Source map support
- Type inference hints
- Zero runtime overhead

Run: `lr build file.lr --target js` or `lr build file.lr --target rust`
