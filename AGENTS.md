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

## Test Coverage Status (as of async/await implementation)

**Branch**: `feat/guards-and-optional-apply`

### Test Counts

| Layer | Count | Runner |
|-------|-------|--------|
| Rust unit/e2e | 400 (2 ignored) | `cargo test` in `compiler/` |
| CLI integration | 106 | `lr test` from `crates/lr-cli/` |
| Live system | 164 | `compiler/tests/live_runner.sh` |
| **Total** | **670** | |

### Fully Verified Features (all 3 layers)

- **Arithmetic**: `+`, `-`, `*`, `/`, `%`, `^`
- **Comparison**: `==`, `=`, `!=`, `<`, `>`, `<=`, `>=`
- **Boolean logic**: `&` (AND), `|` (OR/default), `!` (negation)
- **String ops**: `+` (concat), `^` (uppercase), `_` (lowercase), `^_` (capitalize), `~` (replace), `<>` (split), `><` (join), `==`/`!=` (equality), `#` (length), `-` (remove substring), `+` with Number/Boolean/undefined/List
- **List ops**: `@` (index), `#` (size), `+` (concat/append/prepend), `-` (remove elements), `><`/`<>` (join with separator), `?><` (contains), `==` (equality)
- **Loop ops**: `$` (map), `$?` (filter), `$_` (flatmap), `$|` (some), `$&` (every), `$?|` (find), `$~` (uniqueBy), `$>` (groupBy), `$%` (sort), `$?!` (compact), `$@` (pluck), `$"` (eachToString)
- **Element-wise**: `$+`, `$-`, `$*`, `$/`, `$%`
- **Filter comparisons**: `$?>`, `$?<`, `$?>=`, `$?<=`, `$?+`, `$?-`
- **Map ops**: `@` (get), `-` (remove), `+` (merge), `#` (size), `==`/`!=` (equality), `@` with bracket path, `@&` (pick), `|` (default)
- **Type checks**: `?"` (isString), `?#` (isNumber)
- **Error**: `!!!` (throw), `!!!?` (catch), `Error[expr]` (constructor), `Error@message`
- **Closures**: monadic `{ _< }`, diadic `{ _< + _> }`, nested, chained
- **Control**: `?:` (guards), `!!` (optional apply), `|` (default), `?` (ternary)
- **Spread**: `+:` map merge with override
- **Destructuring**: `_<@`prop`` named arg destructuring
- **Async/Await**: `///` (make async), `\\\` (await) — synchronous stub, pass-through execution
- **Other**: partial application `[args] func`, template interpolation `{var}`, map binding `{a:1, b:a+1}`, program maps, bracket path access `@[key1, key2]`

### Features NOT Fully End-to-End Verified

These are IMPLEMENTED but missing coverage in one or more test layers:

#### Missing from Rust Unit/E2e Tests (compiler.rs / vm.rs)

1. **`?:else` ternary else clause** — VM dispatch exists (`(_, _, "?:else") => right`) but no LR syntax generates it. Test confirms it doesn't crash, but can't assert behavior.

#### Missing from CLI Integration Tests (crates/lr-cli/tests/*.lr)

1. **`?` as PartialOperator on Number/String/List/Map** — Ternary variant tested, but `?` as truthy-check PartialOperator not tested.
2. **`_` list concat variant** — Runtime error: "Cannot apply partial operator _ to list". `_` is NOT a concat alias for `+` on lists — spec mismatch.

#### Missing from Live System Tests (compiler/tests/live/*.lr)

None — all implementable features have live test coverage.

### Unimplemented Features (NOT testable)

These are spec features with no runtime implementation:

| Feature | Status | Infrastructure |
|---------|--------|---------------|
| Import/Export | Stub opcode only | Import (140), Export (141) — VM has TODO stubs |
| `imports@` / `files@` | Parser recognizes pattern | No module loading runtime |
| `}@&[...]` export | Parser recognizes pattern | No export runtime |
| Method calls `obj method [args]` | No infrastructure | No parser AST, no VM dispatch |
| Constructor `Type[args]` | Partial (Error[] works) | No general constructor dispatch |
| JSON parse `/json` | Implemented | serde_json in lr-vm, VM String operator dispatch, 4 unit tests. No CLI/live tests — LR string literals can't contain `"` (triggers interpolation). Works with runtime strings only (e.g., HTTP response bodies). |
| `?:` early return | Guards work for truthiness | Early return from program not implemented |

### Test Infrastructure

- **Rust tests**: `cargo test` from `compiler/` — runs all crate tests
- **CLI tests**: `cd compiler/crates/lr-cli && cargo run -- test` — runs .lr files from `tests/` dir
- **Live tests**: `compiler/tests/live_runner.sh` — runs `lr run` on each .lr file, compares stdout against `.lr.expected` files
