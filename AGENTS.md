# Left-Right Language Project — Agent Rules

## Protected Source Files (NEVER EDIT)

These 3 files are the user's original language design documents. They must NEVER be modified by any agent, tool, or automated process:

1. `Map Programming Language Syntax Brainstorming.txt` — Primary brainstorm document (1753 lines)
2. `PenroScript.md` — PenroScript code examples with operator notes (89 lines)
3. `Penscript_LeftRight brainstorm.md` — 25-category specification checklist (246 lines)

**Enforcement**: Do not use Edit, Write, or any other tool to modify these files. Only Read is permitted.

## Brainstorm History (NEVER EDIT)

The `docs/brainstorm-history/` directory contains the complete historical record of the language design process. This folder is **permanently frozen** — it must NEVER be modified, renamed, or deleted by any agent, tool, or automated process.

**Contents include**: Original brainstorm documents, early language experiments, design evolution artifacts.

**Enforcement**: Only Read is permitted on files in `docs/brainstorm-history/`. No edits, moves, or deletions.

## DO NOT EDIT Files (NEVER EDIT)

Any file containing the text `DO NOT EDIT` (case-sensitive) anywhere in its content must NEVER be modified by any agent, tool, or automated process. This applies across the entire workspace, not just to specific directories.

**Enforcement**: Before any edit, grep the target file for `DO NOT EDIT`. If found, abort the edit immediately. Only Read is permitted on such files.

## Project Structure

- `/docs/specs/` — Language specifications (read-only reference)
- `/docs/brainstorm-history/` — Historical brainstorm documents (frozen, never modify)
- `/docs/translations/` — JavaScript-to-Left-Right translations
- `/docs/reports/initial-thoughts/` — User's research notes (read-only reference)
- Top-level `.txt` and `.md` files — Protected source documents

## Language Context

- **Name**: Left-Right (evolved from Penscript → PenroScript → Left-Right)
- **CLI command**: `lr`
- **Paradigm**: Point-free, operator-based, array-oriented, loosely typed
- **Targets**: Transpiles to both JavaScript and Rust
- **Transpiler**: Written in Rust

## Design Rules (MUST follow)

**Source of truth**: `docs/translations/javascript/` — 4 files (2 .lr + 2 .js). When in doubt, check these. Full rules at `docs/specs/implementation-corrections.md`.

1. **No unary negation** — `-5` is invalid. Use `0 - 5`. `-` is diadic only.
2. **`_<` = value from the left** — Closures with `_<` must have data to their left. `{ _< + 1 } 5` errors. `5 { _< + 1 }` works.
3. **Data-first** — Data appears left, operators right. `entities removePrivateIps` not `removePrivateIps(entities)`.
4. **Strict left-to-right** — No operator precedence. `1 + 2 * 3` = `(1 + 2) * 3` = 9.
5. **`+` is polymorphic** — Adds numbers, concats strings/lists, merges maps. No `++` operator. Either operand being string triggers toString coercion.
6. **`@` is get** — Property access requires `@`: `options@`key`` or `{ a: 1 } @ `a``. No bare key access.
7. **Program maps data-first** — `{ double: { _< * 2 }, 7 double }` not `{ double: { _< * 2 }, double 7 }`.

## Test Coverage Status (as of files@ local imports)

**Branch**: `main`

### Test Counts

| Layer | Count | Runner |
|-------|-------|--------|
| Rust unit/e2e | 427 (2 ignored) | `cargo test` in `compiler/` |
| CLI integration | 114 | `lr test` from `crates/lr-cli/` |
| Live system | 174 | `compiler/tests/live_runner.sh` |
| **Total** | **715** | |

### Fully Verified Features (all 3 layers)

- **Arithmetic**: `+`, `-`, `*`, `/`, `%`, `^`
- **Comparison**: `==`, `=`, `!=`, `<`, `>`, `<=`, `>=`
- **Boolean logic**: `&` (AND), `|` (OR/default), `!` (negation)
- **String ops**: `+` (concat), `^` (uppercase), `_` (lowercase), `^_` (capitalize), `~` (replace), `<>` (split), `><` (join), `==`/`!=` (equality), `#` (length), `-` (remove substring), `+` with Number/Boolean/undefined/List
- **List ops**: `@` (index), `#` (size), `+` (concat/append/prepend), `_` (concat alias), `-` (remove elements), `><`/`<>` (join with separator), `?><` (contains), `==` (equality)
- **Loop ops**: `$` (map), `$?` (filter), `$_` (flatmap), `$|` (some), `$&` (every), `$?|` (find), `$~` (uniqueBy), `$>` (groupBy), `$%` (sort), `$?!` (compact), `$@` (pluck), `$"` (eachToString), `$|||` (parallel map — multi-threaded via `std::thread::scope`)
- **Element-wise**: `$+`, `$-`, `$*`, `$/`, `$%`
- **Filter comparisons**: `$?>`, `$?<`, `$?>=`, `$?<=`, `$?+`, `$?-`
- **Map ops**: `@` (get), `-` (remove), `+` (merge), `#` (size), `==`/`!=` (equality), `@` with bracket path, `@&` (pick), `|` (default), property access by name
- **Type checks**: `?"` (isString), `?#` (isNumber)
- **Error**: `!!!` (throw), `!!!?` (catch), `Error[expr]` (constructor), `Error@message`
- **Constructors**: `Error[expr]` (Error constructor), `Type[expr]` (generic constructor — returns map with `_type` key)
- **Closures**: monadic `{ _< }`, diadic `{ _< + _> }`, nested, chained
- **Control**: `?:` (guards in closures and programs), `!!` (optional apply), `|` (default), `?` (ternary)
- **Spread**: `+:` map merge with override
- **Destructuring**: `_<@`prop`` named arg destructuring
- **Async/Await**: `///` (make async), `\\\` (await) — synchronous stub, pass-through execution
- **Other**: partial application `[args] func`, template interpolation `{var}`, map binding `{a:1, b:a+1}`, program maps, bracket path access `@[key1, key2]`
- **Imports**: `files@`path`` (local .lr file loading), `files@`path`@`key`` (get from module), `files@`path`@&[...]` (pick from module). Data values only — imported closures cannot be called (body_start references imported chunk's bytecode, not accessible from outer execution context).
- **$||| parallel map**: Multi-threaded via `std::thread::scope` in compiled VM. Transpiles to `await Promise.all(arr.map(fn))` in JS output.

### JS Transpiler (lr-codegen-js)

Transpiles Left-Right AST to JavaScript. CLI: `lr transpile <file>`.

| LR Construct | JS Output |
|---|---|
| `5 + 3` | `5 + 3` |
| `arr $ { _< * 2 }` | `arr.map(x => x * 2)` |
| `arr $? { _< > 2 }` | `arr.filter(x => x > 2)` |
| `arr $||| { _< * 2 }` | `Promise.all(arr.map(x => x * 2))` |
| `obj@`key`` | `obj["key"]` |
| `arr #` | `(arr).length` |
| `data func` | `func(data)` |
| `{ _< + 1 } 5` | `(x => x + 1)(5)` |
| `{ a: 1, b: a + 1, b }` | `(() => { const a = 1; const b = a + 1; return b; })()` |
| `42 !!!` | `throw 42` |
| `expr !!!? { handler }` | `try { expr } catch(__e) { handler }` |
| `expr ///` | `(async () => expr)()` |
| `expr \\\` | `await expr` |
| `{ a: 1, +: other }` | `{a: 1, ...other}` |

21 unit tests in `lr-codegen-js`. Crate at `compiler/crates/lr-codegen-js/`.

### Features NOT Fully End-to-End Verified

These are IMPLEMENTED but missing coverage in one or more test layers:

No gaps — all implemented features have full 3-layer test coverage.

### Unimplemented Features (NOT testable)

These are spec features with no runtime implementation:

| Feature | Status | Infrastructure |
|---------|--------|---------------|
| `imports@` | Parser recognizes pattern | No npm module loading runtime |
| `}@&[...]` export | Works via `@&` pick operator | No special export opcode needed |
| Imported closures | Module loads but closures uncallable | `body_start` references imported chunk's bytecode; VM passes outer `code` to `run_closure_body` |

### Test Infrastructure

- **Rust tests**: `cargo test` from `compiler/` — runs all crate tests
- **CLI tests**: `cd compiler/crates/lr-cli && cargo run -- test` — runs .lr files from `tests/` dir
- **Live tests**: `compiler/tests/live_runner.sh` — runs `lr run` on each .lr file, compares stdout against `.lr.expected` files
