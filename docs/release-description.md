# Left-Right v0.0.1 — First Public Release

**Left-Right** is a point-free, operator-based, hierarchical-data oriented scripting language. It combines function composition with a syntax that reads naturally left-to-right, inspired by array-oriented languages like APL, J, K, and BQN.

> *Previously known as PenRoScript*

---

## What's Inside

### Language Core

- **Point-free syntax** — no named variables needed for common operations; values flow through operators
- **Strict left-to-right evaluation** — zero operator precedence: `1 + 2 * 3` = `(1 + 2) * 3` = 9
- **Backtick strings only** — `"` is an operator, not a string delimiter
- **Auto-currying** — operator partial application happens automatically
- **7 types**: Operators, Maps, Lists, Numbers, Strings, Boolean, Undefined

### Operators

- `@` get — property access (`data@\`key\``, `list@2`)
- `#` size — length of collections
- `+` polymorphic add — numbers, string concat, list concat, map merge, number+list prepend
- `$`, `$?`, `$|`, `$&`, `$_`, `$~`, `$>`, `$%` — map, filter, some, every, flatmap, uniqueBy, groupBy, sort
- `"^`, `"_`, `"^_`, `"~`, `<>`, `><` — uppercase, lowercase, capitalize, replace, split, join
- `&`, `|`, `!`, `?` — AND, OR/default, negate, truthy
- `!!!`, `!!!?`, `?:` — throw, try/catch, early return
- `///`, `\\\` — async, await
- Standard arithmetic: `+`, `-`, `*`, `/`, `%`, `^`, `>`, `<`, `>=`, `<=`, `=`, `!=`

### Closures

- Monadic (`{ _< * 2 }` or `{ 2 * _> }`) and diadic (`{ _< + _> }`) closures as first-class values
- Infix-only — closures with `_<` require data on the left
- String interpolation — `` `hello {name}` ``

### Compiler & VM

- 10-crate Rust workspace: lexer, parser, AST, bytecode compiler, stack-based VM with GC
- Compiles Left-Right source to bytecode, executes via virtual machine

### CLI (`lr`)

- `lr run <file>` — execute a file
- `lr repl` — interactive REPL with readline support
- `lr new`, `lr build`, `lr test`, `lr watch`

---

## Design Rules

1. **No unary negation** — `-5` is invalid. Use `0 - 5`
2. **Data-first** — data appears left, operators right
3. **Infix-only closures** — `5 { _< + 1 }` works, `{ _< + 1 } 5` errors
4. **Polymorphic `+`** — adapts to input types
5. **No operator precedence** — strict left-to-right
6. **Key-value pairs only in `{}`** — `:` assignment only valid inside map literals

---

## Documentation

- [Language Spec](./specs/left-right-language-specification.md)
- [AST Spec](./specs/ast-specification.md)
- [Lexer Spec](./specs/lexer-specification.md)
- [Implementation Corrections](./specs/implementation-corrections.md)
- [JavaScript Translations](./translations/javascript/)

---

## Binaries

| Platform | Architecture        | Download                  |
| -------- | ------------------- | ------------------------- |
| Linux    | x86_64              | `lr-x86_64-linux.tar.gz`  |
| macOS    | ARM (Apple Silicon) | `lr-aarch64-macos.tar.gz` |
| Windows  | x86_64              | `lr-x86_64-windows.exe`   |

Or build from source:

```bash
cd compiler && cargo build --release
```

---

## Quick Start

```lr
# Map over a list
[1, 2, 3, 4, 5] $ { _< + 10 }
# → [11, 12, 13, 14, 15]

# Filter
[1, 2, 3, 4, 5] $? { _< > 3 }
# → [4, 5]

# Chain
[1, 2, 3, 4, 5] $? { _< > 2 } $ { _< * 2 }
# → [6, 8, 10]

# Closures defined inside maps
{
  double: { _< * 2 },
  7 double
}
# → 14
```

---

**Full Changelog**: https://github.com/penwoodj/left-right/commits/v0.0.1
