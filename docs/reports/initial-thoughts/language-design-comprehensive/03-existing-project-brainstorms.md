# Existing Project Brainstorm Documents

Reference index to brainstorm documents already in the `~/code/left-right/` repository root.

---

## PenroScript.md

**Location**: `~/code/left-right/PenroScript.md`
**Lines**: 89

### Summary
Shows TypeScript/JavaScript to PenroScript syntax comparison with two complete code examples demonstrating the language's left-to-right evaluation, operator design, and functional composition patterns.

### Key Content
- **getEntityTypes function**: TS/JS comparison showing filtering entities by type using `toLower`, `map`, `filter`, `some`
- **Anonymous function with Lodash FP**: Threat classification pipeline with flow, map, filter, capitalize, uniq, join
- **PenroScript operators demonstrated**:
  - `_>` - right-directional apply operator
  - `<` - filter operator
  - `$?{ }` - predicate-driven filter
  - `~` - map transformation
  - `~` - uniqueness/identity
  - `#` - size/count operator
  - `?|` - logical OR/conditional
  - `^` - string to uppercase
  - `^_` - string capitalize
  - `+` - array concatenation
- **Types listed**: Operator, Hashmap, Array, String, Boolean, Number, Undefined
- **Language design principles**:
  - Diatic operators are left-hungry curried by default, can be reversed
  - Left-to-right evaluation, parenthetical grouping
  - `{ ... _< ... }` or `{ ... _> ... }` is operator, `{ ... key: value }` is JSON object
  - All strings are template literals from interface standpoint
  - Core language operators are input type dependent
  - Operator symbols can be overridden and extended
  - Symbology is spatial: `'asdf' "^` is toUpperCase, `'asdf" "^_` is capitalize

---

## Penscript_LeftRight brainstorm.md

**Location**: `~/code/left-right/Penscript_LeftRight brainstorm.md`
**Lines**: 246

### Summary
A comprehensive 25-category specification checklist for defining PenroScript language design decisions, covering everything from philosophy to migration strategy.

### Categories Covered
1. **Language Philosophy & Goals** - Problems optimized for, ergonomics priorities, runtimes, target audience
2. **Core Evaluation Model** - LTR evaluation, strict/lazy operators, expression sequencing, grouping rules, precedence
3. **Types & Values** - Complete type set, Undefined vs Null, truthiness, Maps, Arrays, Operators as values
4. **Variables, Names, and Assignment** - Declaration, immutability, reassignment, scope, name resolution
5. **Functions & Calls** - Definition, arity, calling syntax, partial application, methods
6. **Operators (Design & Extensibility)** - Left hungry definition, associativity, precedence table, overloading, custom operators
7. **The `_/` Directional Forms** - `_</_>` grammar, callable values vs macros, nesting, pipeline interaction
8. **Collections & Paths** - `@` get operator syntax, missing key behavior, set/update, deep merge
9. **Conditionals, Patterning & Control Flow** - Conditionals, looping, pattern matching, error handling
10. **Strings & Templates** - Interpolation syntax, `_</_>` as templating operators, escaping, multiline/raw strings
11. **Standard Library (HOO & Data Ops)** - HOFs (map, filter, some, every, reduce, size, uniq, join, flow), composition/pipe, equality/comparison, string ops, collection builders
12. **Modules, Files, & Imports** - File as module, import syntax, circular imports, execution model
13. **Interop (TS/JS & JSON)** - JS object interop, host function calls, type marshalling, JSON/YAML serialization
14. **Comments, Whitespace, and Layout** - Comment syntax, significant whitespace, indentation vs braces
15. **Errors, Diagnostics, and Types (Static vs Dynamic)** - Static typing, runtime errors, lints, contracts
16. **Macros & Metaprogramming** - Doc-blocks as executable meta, AST transformation, symbol resolution
17. **Performance & Semantics Guarantees** - Determinism, tail-call, Big-O, memory model
18. **Concurrency & Effects** - Concurrency primitives, IO model, effects representation
19. **Tooling & Packaging** - CLI commands, formatter, package manager
20. **Syntax Reference & Grammar** - EBNF grammar, operator tokenization, reserved words
21. **Examples & Canonical Patterns** - Hello World, data-transforms, templating, pipelines, operator overrides
22. **Backwards/Forwards Compatibility** - Stability policy, versioning, feature flags
23. **Security & Sandboxing** - Untrusted code safety, resource limits, capability objects
24. **Testing & Documentation Conventions** - Doctest, snapshot tests, error golden tests
25. **Migration & Transpilation** - TS/JS → PenroScript patterns, PenroScript → JS/TS guarantees, AST interchange

---

## Map Programming Language Syntax Brainstorming.txt

**Location**: `~/code/left-right/Map Programming Language Syntax Brainstorming.txt`
**Lines**: 1753

### Summary
Massive brainstorming document for a functional programming language with JSON-like syntax, covering file/module system, operator design, function composition, and real-world integration examples (primarily ServiceNow). The language emphasizes left-to-right evaluation, deterministic execution, and a tree-based documentation model.

### Major Topics
- **File/Package System**: Files as packages, tree structure documentation, self-documenting codebases
- **Types & Values**: Primitives, JSON/objects/arrays with guaranteed order, equality semantics (= non-ordered, == ordered)
- **Function Design**: Array/object destructuring parameters, curried functions, application operators
- **Operators**: `>>` (append), `<<` (prepend), `_>` (spread), `=>` (arrow functions), `&` (AND), `|` (OR), `^` (double-negative/!!), logical operators evaluated LTR
- **List Functions**: Implicit function bodies with `$0`, `$1` syntax
- **String Operations**: Template literals, substring/includes, regex support
- **Real-World Integration**: ServiceNow integration examples, lodash/fp interop

### Code Examples
- **ServiceNow Integration** (`exampleFile`):
  - `getTableQueryDataSummaryTags` - asset count summary tag
  - `getTotalAssetSummaryTag` - knowledge base document count
  - `getTotalKbDocsSummaryTag` - complex pipeline with flatMap, get, concat, map, uniq, slice, conditional logic
- **Lodash FP Integration**:
  - Imports: `map`, `flatMap`, `flow`, `get`, `getOr`, `compact`, `__`, `uniq`, `size`, `capitalize`, `concat`, `slice`
  - Application examples with array/object spread syntax
- **Conditional Operators**:
  - `?` for ternary-like behavior using `&` and `|`
  - Pattern: `predicate & truthyValue | falseyValue`
- **Function Composition**:
  - Pipeline examples: `result >> getOr['tableQueryData', []] >> flatMap[[...] ==> {...}] >> compact >> uniq`
  - Nested destructuring and default values
