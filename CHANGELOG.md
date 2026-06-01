# Changelog

## [Unreleased]

### Added
- **JS Transpiler** (`lr transpile`): AST-to-JavaScript code generation via `lr-codegen-js` crate. Supports closures, programs, operators, async/await, parallel map.
- **`$|||` parallel map operator**: Multi-threaded execution via `std::thread::scope` in compiled VM. Transpiles to `Promise.all()` in JS output.
- **`$?|` find operator**: Find first matching element in a list.
- **`///` async operator**: Mark expression as async (stub execution in VM).
- **`\\\` await operator**: Await async result (stub execution in VM).
- **`!!!?` catch expression**: Try/catch error handling with `!!!?` syntax.
- **`Error[expr]` constructor**: Error value creation.
- **`Type[expr]` generic constructor**: Generic type constructor returning map with `_type` key.
- **`$~` uniqueBy**: Remove duplicates by key function.
- **`$>` groupBy**: Group elements by key function.
- **`$%` sort**: Sort collection.
- **`$?!` compact**: Remove undefined/null values.
- **`$@` pluck**: Map-each-property.
- **`$" eachToString**: Convert each element to string.
- **Element-wise operators**: `$+`, `$-`, `$*`, `$/`, `$%` for element-wise list operations.
- **Filter comparison operators**: `$?>`, `$?<`, `$?>=`, `$?<=`, `$?+`, `$?-`.
- **`<>` split** and **`><` join**: String split and join operators.
- **`~` replace**: String replace operator.
- **`^` uppercase**, **`_` lowercase**, **`^_` capitalize**: String case operators.
- **`<>` split**, **`><` join**: String split and join.
- **Template interpolation**: `{var}` inside backtick strings.
- **`?:` guards**: Conditional execution in closures and programs.
- **`!!` optional apply**: Apply if truthy, return undefined if falsy.
- **`|` default operator**: Return default value when left is falsy.
- **`?` ternary**: Ternary conditional operator.
- **`+:` spread**: Map merge with override.
- **`_<@`prop`` destructuring**: Named argument destructuring.
- **Map binding**: `{a:1, b:a+1}` sequential binding in maps.
- **Program maps**: Maps with sequential bindings and implicit return.
- **Bracket path access**: `@[key1, key2]` nested property access.
- **`files@`path`` imports**: Local .lr file loading.
- **CLI `lr transpile`**: Transpile .lr files to JavaScript.
- **CLI `lr watch`**: File watching with auto-reexecution.
- **CLI `lr build`**: Project build command.
- **CLI `lr test`**: Test runner.

### Fixed
- **Operator corrections**: Removed dead `==`, `&&`, `||` from VM dispatch. Equality is `=` only. AND is `&` only. OR is `|` only.
- **`!=` operator**: Confirmed valid as not-equals operator.
- **Context-dependent `|`**: `|` now returns default value when left is falsy, boolean OR otherwise.
- **Compound token recognition**: Lexer recognizes `!!!`, `!!!?`, `$@`, `+:` as single tokens.

### Removed
- **v0.0.1 release deleted**: Contained incorrect operator descriptions.

### Test Coverage
- 715 tests across 3 layers: Rust unit/e2e (427), CLI integration (114), Live system (174).
- All implemented features have full 3-layer test coverage.

### Architecture
- 12-crate Rust workspace: lr-lexer, lr-parser, lr-ast, lr-bytecode, lr-compiler, lr-vm, lr-diagnostics, lr-codegen-js, lr-cli, and 3 test helper crates.