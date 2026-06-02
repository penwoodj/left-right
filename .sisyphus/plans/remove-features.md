# Plan: Remove Non-Canonical Features

## Objective
Remove features that don't belong in the Left-Right language per user directive:
1. `$?+` and `$?-` filter comparison operators (not real ops)
2. `!!` optional apply operator (doesn't exist in language)
3. `Error[expr]` constructor syntax (only `!!!` throw exists)
4. `Type[expr]` constructor syntax (no class system)

Also fix:
5. `@&` pick — current transpiler emits complex `Object.fromEntries` helper; should emit simpler pick

## Language Philosophy (confirmed by user)
- **Types**: Operators, Maps, Lists, Strings, Numbers, Booleans, Undefined — ONLY these
- **Strings**: Only backtick `` ` `` strings exist
- **Comments**: Only `` ``` `` at end of line (no block comments)
- **Error handling**: `!!!` throws whatever is on the left into a map with stack trace + `error` key
- **No class/type system**: Error[] and Type[] constructors don't exist
- **JS interop**: nulls → undefined, numbers → number, strings → string, arrays → lists, objects → maps, functions → operators (preserve names and import/export structure)

---

## Phase 2: Remove `$?+` and `$?-`

### Files to modify:

**Transpiler** (`compiler/crates/lr-codegen-js/src/lib.rs`):
- Remove `$?+` and `$?-` from `gen_dollar_operator()` filter comparison block (lines ~530-537)
- Remove tests `test_filter_plus_fixed` and `test_filter_minus_fixed`

**VM** (`compiler/crates/lr-vm/src/vm.rs`):
- Remove `$?+` and `$?-` from partial_operator match (line ~626)
- Remove Number comparison logic for these ops (lines ~1161-1162)

**Compiler tests** (`compiler/crates/lr-compiler/src/compiler.rs`):
- Remove tests at lines 2490, 2496

**CLI tests** (`compiler/crates/lr-cli/tests/`):
- DELETE: `filter_plus.lr`
- DELETE: `filter_minus.lr`

**Live tests** (`compiler/tests/live/`):
- DELETE: `65_filter_plus.lr` and `65_filter_plus.lr.expected`
- DELETE: `66_filter_minus.lr` and `66_filter_minus.lr.expected`

---

## Phase 3: Remove `!!` Optional Apply

### Files to modify:

**Transpiler** (`compiler/crates/lr-codegen-js/src/lib.rs`):
- Remove `OptionalApply` variant from `OperatorPattern` enum
- Remove detection at line 373: `} else if op == "!!" {`
- Remove generation in `gen_operator_pattern` for OptionalApply
- Remove test `test_optional_apply`

**VM** (`compiler/crates/lr-vm/src/vm.rs`):
- Remove `"!!"` from partial_operator match at line 212
- Remove closure execution logic at lines 503-508
- Remove `"!!"` from partial_operator creation in every type branch: lines 571, 605, 671, 690, 701, 1238

**Lexer** (`compiler/crates/lr-lexer/src/lexer.rs`):
- Remove `!!` tokenization at line 291 (keep `!!!` and `!!!?`)
- Remove lexer test for `!!`

**Compiler tests** (`compiler/crates/lr-compiler/src/compiler.rs`):
- Remove 4 tests at lines 1719-1741

**CLI tests** (`compiler/crates/lr-cli/tests/`):
- DELETE: `optional_apply.lr`
- DELETE: `optional_apply_list.lr`
- DELETE: `optional_apply_map.lr`
- DELETE: `optional_apply_string_trivial.lr`

**Live tests** (`compiler/tests/live/`):
- DELETE: 9 files: 105, 113, 115, 116, 117, 129, 130, 131, 132 (all `*_optional_*` files)

---

## Phase 4: Remove `Error[expr]` Constructor

### Files to modify:

**Transpiler** (`compiler/crates/lr-codegen-js/src/lib.rs`):
- Remove `Constructor` enum variant
- Remove Error[] detection in `detect_operator_pattern` (lines 389-400, 443-454)
- Remove Constructor generation (lines 321-337)
- Remove test `test_error_constructor`

**VM** (`compiler/crates/lr-vm/src/vm.rs`):
- Remove Error[] case from Opcode::Call handler (lines 309-317)

**VM** (`compiler/crates/lr-vm/src/value.rs`):
- KEEP `Error` enum variant and `ErrorData` (still used by `!!!` throw)
- KEEP `Value::error()` constructor (still used internally)

**Compiler tests** (`compiler/crates/lr-compiler/src/compiler.rs`):
- Remove test `test_error_constructor` (line 2093-2100)
- Keep error message access tests (still valid for `!!!` thrown errors)

**CLI tests** (`compiler/crates/lr-cli/tests/`):
- DELETE: `error_constructor.lr`
- KEEP: `error_message.lr` (tests `Error@message` which may still work with `!!!` thrown errors)

---

## Phase 5: Remove `Type[expr]` Constructor

### Files to modify:

**Transpiler** (`compiler/crates/lr-codegen-js/src/lib.rs`):
- Remove Type[] case from Constructor handling
- Remove test `test_type_constructor`

**VM** (`compiler/crates/lr-vm/src/vm.rs`):
- Remove generic Type[] case from Opcode::Call handler (lines 319-341)

---

## Phase 6: Fix `@&` Pick

### Files to modify:

**Transpiler** (`compiler/crates/lr-codegen-js/src/lib.rs`):
- Change Pick output from complex `Object.fromEntries` helper to simpler:
  ```js
  // Option A: Manual pick using reduce
  keys.reduce((acc, k) => (k in obj && (acc[k] = obj[k]), acc), {})
  
  // Option B: lodash-style pick (if lodash available)
  _.pick(obj, keys)
  ```
- For now, keep the working implementation — it's functionally correct
- User confirmed: `{a:1, b:2} @& ["a"]` → `{a:1}` (pick only matching keys)

---

## Phase 7: Update Documentation

### AGENTS.md:
- Remove `$?+`, `$?-`, `!!`, `Error[]`, `Type[]` from operator tables
- Update test counts after removal
- Add language philosophy section (types, strings, comments, JS interop)

### docs/specs/:
- Update implementation-corrections.md if needed

---

## Phase 8: Verification

After all changes:
1. `cargo build` — must compile
2. `cargo test` — all tests pass (will be fewer tests)
3. `cd compiler/crates/lr-cli && cargo run -- test` — CLI tests pass
4. `compiler/tests/live_runner.sh` — live tests pass

---

## Phase 9: Commit + Push

Commit incrementally after each phase for safety.

---

## Phase 10: Operator Q&A Walkthrough

After all removals, walk through each remaining operator with user in batches of 5-8, asking multiple choice questions about behavior with different types.
