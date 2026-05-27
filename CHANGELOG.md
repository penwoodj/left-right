# Changelog

## [0.1.0] - 2026-05-27

First official release of the Left-Right programming language.

### Language

- **Point-free, operator-based syntax** — No named variables needed for common operations; values flow through operators
- **Strict left-to-right evaluation** — No operator precedence. `1 + 2 * 3` = `(1 + 2) * 3` = 9
- **Backtick strings** — All strings use backtick delimiters (`like this`). `"` is a reserved operator, not a string delimiter
- **No unary negation** — `-5` is invalid. Use `0 - 5`. The minus operator is diadic only
- **Infix-only closures** — Closures with `_<` require data on the left. `{ _< + 1 } 5` errors. `5 { _< + 1 }` works
- **Polymorphic `+` operator** — Adds numbers, concatenates strings/lists, merges maps, prepends numbers to lists
- **`@` get operator** — Property access via `data@\`key\`` or `list@2`
- **`#` size operator** — Returns length of strings, lists, and maps
- **Auto-currying** — Operator partial application happens automatically
- **Closures** — Monadic (`_<`) and diadic (`_<`, `_>`) closures as first-class values
- **Loop operators** — `$` (map), `$?` (filter), `$|` (some), `$&` (every), `$?|` (find), `$_` (flatmap), `$~` (uniqueBy), `$>` (groupBy), `$%` (sort)
- **String operators** — `"^` (uppercase), `"_` (lowercase), `"^_` (capitalize), `"~` (replace), `<>` (split), `><` (join)
- **String interpolation** — `` `hello {name}` `` with inline expressions
- **Boolean operators** — `&` (AND), `|` (OR/default), `!` (negate), `?` (truthy check)
- **Type checks** — `?"` (isString), `?#` (isNumber), `?><` (contains)
- **Numeric operators** — `+`, `-`, `*`, `/`, `%`, `^`, `>`, `<`, `>=`, `<=`, `=`, `!=`
- **Error handling** — `!!!` (throw), `!!!?` (try/catch), `?:` (early return / guard)
- **Async support** — `///` (make async), `\\\\` (await)
- **Spread operator** — `{ a: 1, +: { a: 3, b: 2 } }` merges maps
- **Import/Export** — Module system with `import` and `export`
- **Ternary operator** — `?` for conditional expressions
- **Context-dependent `|`** — Returns default when left is falsy, boolean OR otherwise

### Compiler & VM

- **10-crate Rust workspace** — lr-common, lr-lexer, lr-ast, lr-parser, lr-diagnostics, lr-bytecode, lr-vm, lr-compiler, lr-runtime, lr-cli
- **Lexer** — Full token recognition including compound tokens (`!!!`, `!!!?`, `$@`, `+:`)
- **Parser** — Recursive descent parser producing typed AST
- **Bytecode compiler** — Compiles AST to bytecode instructions
- **Stack-based VM** — Executes bytecode with GC-managed values
- **Operator corrections** — Removed dead `==`, `&&`, `||` from VM dispatch. Equality is `=` only. AND is `&` only. OR is `|` only
- **Ternary map compilation** — Maps containing ternary expressions compile correctly
- **Reverse closure calls** — Closures can be called with data on either side
- **Expression keys in maps** — Map keys can be expressions without requiring parentheses

### CLI

- **`lr run <file>`** — Execute a `.lr` file
- **`lr repl`** — Interactive REPL with readline support (rustyline)
- **`lr new <name>`** — Create a new Left-Right project
- **`lr build`** — Build project
- **`lr test`** — Run tests
- **`lr watch`** — Watch mode for development

### CI & Distribution

- **GitHub Actions release workflow** — Automated builds on tag push
- **Pre-built binaries** — Linux x86_64, macOS ARM, Windows x86_64

### Documentation

- [Language Specification](docs/specs/left-right-language-specification.md)
- [AST Specification](docs/specs/ast-specification.md)
- [Lexer Specification](docs/specs/lexer-specification.md)
- [Implementation Corrections](docs/specs/implementation-corrections.md)
- [JavaScript Translations](docs/translations/javascript/)
- [Rosetta Code Translations](docs/translations/future-translations/rosettacode/)

### Testing

- 230 tests passing across all crates
