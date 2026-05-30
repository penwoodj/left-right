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

## Test Coverage Status (as of commit b674994)

**Branch**: `feat/guards-and-optional-apply`

### Test Counts

| Layer | Count | Runner |
|-------|-------|--------|
| Rust unit/e2e | 360 (2 ignored) | `cargo test` in `compiler/` |
| CLI integration | 66 | `lr test` from `crates/lr-cli/` |
| Live system | 103 | `compiler/tests/live_runner.sh` |
| **Total** | **529** | |

### Fully Verified Features (all 3 layers)

- **Arithmetic**: `+`, `-`, `*`, `/`, `%`, `^`
- **Comparison**: `==`, `=`, `!=`, `<`, `>`, `<=`, `>=`
- **Boolean logic**: `&` (AND), `|` (OR/default), `!` (negation)
- **String ops**: `+` (concat), `^` (uppercase), `_` (lowercase), `^_` (capitalize), `~` (replace), `<>` (split), `><` (join), `==`/`!=` (equality), `+` with Number/Boolean
- **List ops**: `@` (index), `#` (size), `+` (concat/append/prepend), `><` (join), `?><` (contains), `==` (equality)
- **Loop ops**: `$` (map), `$?` (filter), `$_` (flatmap), `$|` (some), `$&` (every), `$?|` (find), `$~` (uniqueBy), `$>` (groupBy), `$%` (sort), `$?!` (compact), `$@` (pluck), `$"` (eachToString)
- **Element-wise**: `$+`, `$-`, `$*`, `$/`, `$%`
- **Filter comparisons**: `$?>`, `$?<`, `$?>=`, `$?<=`, `$?+`, `$?-`
- **Map ops**: `@` (get), `-` (remove), `+` (merge), `#` (size), `==`/`!=` (equality), `@` with bracket path
- **Type checks**: `?"` (isString), `?#` (isNumber)
- **Error**: `!!!` (throw), `!!!?` (catch), `Error[expr]` (constructor), `Error@message`
- **Closures**: monadic `{ _< }`, diadic `{ _< + _> }`, nested, chained
- **Control**: `?:` (guards), `!!` (optional apply), `|` (default), `?` (ternary)
- **Spread**: `+:` map merge with override
- **Other**: partial application `[args] func`, template interpolation `{var}`, map binding `{a:1, b:a+1}`, program maps, bracket path access `@[key1, key2]`

### Features NOT Fully End-to-End Verified

These are IMPLEMENTED but missing coverage in one or more test layers:

#### Missing from Rust Unit/E2e Tests (compiler.rs / vm.rs)

1. **`?:else` ternary else clause** — VM dispatch at vm.rs ~line 1007. No e2e test.
2. **Boolean `==`/`!=`/`=` equality** — PartialOperator creation for Boolean+String at vm.rs line 581. No dedicated test.
3. **`!` negation on all types** — Verified on Boolean, Number, String. Missing: List negation as PartialOperator creation, Map negation as PartialOperator creation.
4. **`!!` optional apply on all types** — Verified on Number/String. Missing: Boolean, List, Map optional apply as PartialOperator creation.
5. **Map `@` with numeric index** — vm.rs line 808 returns `[key, value]` pair. Has live test but no Rust e2e test.
6. **List `@` with list of indices** (nested path) — vm.rs line 823. Has live test but no Rust e2e test.
7. **Error `@` property access** — vm.rs line 1072. Has live test but no Rust e2e test.

#### Missing from CLI Integration Tests (crates/lr-cli/tests/*.lr)

1. **`?` ternary with non-Boolean left** — Only tested with Boolean. No test for Number/String/List ternary.
2. **`!!` optional apply on String/List/Map/Boolean** — Only tested with Number.
3. **`$?>`/`$?<`/`$?>=`/`$?<=` filter with Number arg** — Only `$?+`/`$?-` variants tested. `$?>`/`$?<`/`$?>=`/`$?<=` with direct Number missing.
4. **`?` as PartialOperator on Number/String/List/Map** — Ternary variant tested, but `?` as truthy-check PartialOperator not tested.
5. **`|` default on String/List/Map** — Only tested with Number.
6. **`!"`/`?#` as operator dispatch** — Tested as prefix, not as data-first operator form.
7. **`_` list concat variant** — `_` is concat alias for `+` on lists. No dedicated test.
8. **`<>`/`><` on List** — These create PartialOperators for List. No dedicated test for list split/join via these ops.
9. **Map `@` with numeric index** — Returns `[key, value]` pair at index. No CLI test.

#### Missing from Live System Tests (compiler/tests/live/*.lr)

1. **`?:else` ternary else clause** — Not tested at all in any layer.
2. **`!!` optional apply on String, List, Map, Boolean** — Only Number tested.
3. **`?` ternary with String, List, Map** — Only Boolean/Number tested.
4. **`|` default with String, List, Map** — Only Number/zero tested.
5. **`!` negation operator form** — Only truthiness-tested, not `value !` operator form on all types.
6. **Boolean `&`/`|` with non-Boolean values** — Number AND/OR works, but String/List/Map not tested.
7. **`?"`/`?#` as data-first operator** — Only prefix form tested.

### Unimplemented Features (NOT testable)

These are spec features with no runtime implementation:

| Feature | Status | Infrastructure |
|---------|--------|---------------|
| `///` async / `\\\` await | Stub opcode only | MakeAsync (120), Await (121) return UnimplementedOpcode |
| Import/Export | Stub opcode only | Import (140), Export (141) — VM has TODO stubs |
| `imports@` / `files@` | Parser recognizes pattern | No module loading runtime |
| `}@&[...]` export | Parser recognizes pattern | No export runtime |
| Method calls `obj method [args]` | No infrastructure | No parser AST, no VM dispatch |
| Constructor `Type[args]` | Partial (Error[] works) | No general constructor dispatch |
| JSON parse `/json` | No infrastructure | No JSON parser in VM |
| `@&` pick/destructure | No infrastructure | No VM dispatch |
| Named destructuring `_<@\`prop\`` | No infrastructure | Compiler only handles `_<@0`/`_<@1` |
| List `-` removal | No infrastructure | `-` only dispatches on Map |
| String `#` (length) | Not implemented | `#` only dispatches on List and Map |
| `?:` early return | Guards work for truthiness | Early return from program not implemented |

### Test Infrastructure

- **Rust tests**: `cargo test` from `compiler/` — runs all crate tests
- **CLI tests**: `cd compiler/crates/lr-cli && cargo run -- test` — runs .lr files from `tests/` dir
- **Live tests**: `compiler/tests/live_runner.sh` — runs `lr run` on each .lr file, compares stdout against `.lr.expected` files
