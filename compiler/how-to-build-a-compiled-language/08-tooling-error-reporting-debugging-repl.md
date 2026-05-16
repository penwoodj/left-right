# Tooling, Error Reporting, Debugging, and REPL Design

## Report 8: Tooling Infrastructure for the Left-Right Compiler

This report covers the developer tooling infrastructure that turns a compiler into a production-ready language ecosystem. While the core compiler translates source code to output, tooling determines developer experience, productivity, and adoption.

---

## 1. Error Reporting and Diagnostics

Good error messages distinguish great compilers from frustrating ones. Error reporting is not just about displaying what went wrong, but about guiding developers toward the fix.

### Design Principles for Compiler Error Messages

**Clarity over brevity:** An error message should be immediately understandable without referring to documentation. Instead of "type mismatch at line 42", use "expected function but found number at line 42:42".

**Precise location:** Point to exactly where the error occurred, including column position. For operator-heavy languages like Left-Right, column position is critical because errors often occur between operators in dense expressions.

**Explain the "why", not just "what":** Don't just say "undefined identifier x". Say "identifier x is not defined in this context. Did you mean to access it via `imports` from another library, or is it a typo?"

**Provide actionable suggestions:** When possible, suggest fixes. "Did you mean `map` instead of `map_`?" for typos. "Check type with `?#` or `?"` operators" for type-related issues.

**Show context, not just the error:** Display the offending line with surrounding code for context. For multi-line expressions, show the relevant span.

**Error code for documentation:** Each error should have a unique identifier like `E0001` or `LR-001`. This lets developers search docs and discuss errors unambiguously.

### Error Span Highlighting with Source Context

Spans capture precise locations in source code. For Left-Right's point-free, operator-heavy syntax, spans need to handle:

**Operator spans:** In expressions like `list ${...} ?{...}`, errors in operator arguments need to span the operator application. An arity mismatch should highlight from the operator through its argument.

**Nested expression spans:** In `(arr map fn) (arr2 filter pred)`, a type error should span the specific sub-expression causing the problem.

**Multi-line spans:** String interpolation or multi-line operators need spans that cross line boundaries. Display multi-line spans with caret markers on each affected line.

**Macro expansion spans:** When macros expand, errors in expanded code should reference the original macro invocation. Track expansion chains to show "error in expansion of macro X at line Y".

### Error Recovery for Continued Compilation

A compiler that stops at the first error forces developers through compile-fix-compile loops. Error recovery lets one compilation pass report multiple errors.

**Synchronization points:** After encountering an error, skip tokens until reaching a known synchronization point like end of statement, closing parenthesis, or top-level expression boundary.

**Panic mode recovery:** When the parser enters an invalid state, discard tokens until resynchronizing. For Left-Right's expression-based structure, synchronize at expression boundaries (end of line, operator precedence drop).

**AST node marking:** Instead of aborting, insert error marker nodes into the AST. Downstream phases can process these markers to skip error regions while continuing analysis.

**Limit error cascades:** One syntax error shouldn't trigger 50 type errors. When an error is detected, disable subsequent error reporting in that region to avoid cascading failures.

### Warning vs Error Categorization

Not all problems are fatal. Categorize issues appropriately:

**Errors:** Unambiguous problems that prevent compilation. Type mismatches, undefined identifiers, syntax errors.

**Warnings:** Problems that compile but may indicate bugs. Dead code, unreachable code, suspicious patterns.

**Hints:** Suggestions for improvement that don't indicate problems. "Consider using `?{` instead of `${` followed by filtering" for readability.

**Allow warnings as errors:** Provide a flag to treat warnings as errors for strict CI pipelines.

### Suggesting Fixes in Error Messages

Fix suggestions turn errors into learning opportunities:

**Typo detection:** Levenshtein distance on identifiers to suggest "Did you mean `filter`?" for `fliter`.

**Missing identifiers:** When an undefined name matches an available symbol, suggest "Use `imports@[`library`, `symbol`]` to access from library or use direct path access".

**Type error fixes:** For type mismatches, suggest conversion operators: "Use `?\"` to check if string, `?#` to check if number, `toString` or `asString` for conversion".

**Signature suggestions:** When operator arguments don't match, show usage example: "map takes a list and an operator that transforms each element: `list ${(x -> x * 2)}`".

**Automatic fix application:** In editor environments, offer "quick fix" actions via LSP to automatically apply suggested corrections.

### Multi-line Error Spans

Left-Right expressions can span multiple lines, especially with string interpolation:

```
greeting = `
  Hello, {name}
  Your score is: {score}
`
```

If `score` is undefined, the error span should cover `{score}` specifically. Multi-line span display:

```
error[E0001]: undefined variable `score`
  --> example.lr:3:18
   |
 2 |   Hello, {name}
 3 |   Your score is: {score}
   |                  ^^^^^^ undefined variable in string interpolation
```

For chained operator expressions across lines:

```
arr = [1, 2, 3]
result = {arr: arr}
  map (x -> x * 2)
  filter (x -> x > 3)
  sum
```

If `sum` is not imported, highlight line 5 specifically but show context from lines 1-5.

### Production Examples: Rustc and Elm

**Rustc** is widely praised for error messages:
- Color-coded output with error codes (E0382, etc.)
- Suggestions with "help:" sections
- "note:" sections explaining why the error occurs
- Multiple suggestions ranked by likelihood
- Example of good output showing the error span and explanation

**Elm compiler** focuses on beginner-friendly messages:
- Natural language explanations over technical jargon
- ASCII art style diagrams for complex errors
- "I ran into something unexpected" phrasing to reduce anxiety
- Step-by-step guidance for fixing common issues

Left-Right should draw from both: Elm's beginner-friendly explanations combined with Rustc's precision and suggestion system.

---

## 2. Source Maps and Debugging

Debugging transpiled code requires mapping runtime errors back to source code. Source maps bridge this gap.

### Source Map Format and Generation

**Source map format:** Use the standard Source Map Revision 3 format. Maps are JSON files describing how positions in generated code map to positions in original source.

**Mapping data:** For each position in generated code, store:
- Generated line and column
- Original source file name
- Original line and column
- Name (for variables and functions)

**Generation strategy:** During code generation, track the origin of each emitted section. When transpiling Left-Right to JavaScript, emit a mapping entry each time a Left-Right expression produces a chunk of JavaScript.

**Inline vs external:** For development, embed source maps as base64 data URLs in generated output for convenience. For production, emit separate `.map` files to reduce bundle size.

### Mapping Compiled Output Back to LR Source

**Expression-level mapping:** Each Left-Right expression maps to its corresponding JavaScript. For point-free chains like `arr map fn filter pred`, map each operator call to its JavaScript equivalent.

**Operator precedence mapping:** Left-Right's operator precedence might not match JavaScript's. Map grouped sub-expressions correctly, not token-by-token.

**Macro expansion tracking:** When macros expand code, maintain the original source location in the generated mapping. Errors in expanded code point to the macro invocation site.

**Source map integrity:** Include a source map URL comment in generated output:
```javascript
//# sourceMappingURL=example.lr.js.map
```

### Debug Info Formats

**For JavaScript target:** Use standard source maps. Browser DevTools and Node.js both support source maps for debugging transpiled code.

**For native compilation:** Use DWARF (Debugging With Attributed Record Formats), the standard debug information format for compiled languages on Unix-like systems.
- DWARF debug sections in the binary: `.debug_info`, `.debug_line`, `.debug_abbrev`, `.debug_str`
- Map machine instructions back to Left-Right source locations
- Support for value types, scopes, and line information

**Hybrid approaches:** For WebAssembly targets, use WebAssembly custom sections for debug info, compatible with DWARF.

### Debugger Integration

**For JavaScript target:**
- Chrome DevTools automatically loads source maps when referenced
- Set breakpoints in Left-Right source files, they map to running JavaScript
- Debug console shows Left-Right stack frames, not transpiled JavaScript
- Variable inspection shows original Left-Right names, not minified identifiers

**For native target:**
- Generate DWARF info compatible with GDB and LLDB
- GDB: Set breakpoints in `.lr` files if GDB has Left-Right support, otherwise debug at generated Rust/C level
- LLDB: Similar support, better for macOS development
- Print debugging: Map addresses back to source lines in stack traces

**Custom debuggers:** Consider a Left-Right-specific debugger for the REPL that understands the language semantics directly, stepping through operator chains rather than low-level instructions.

### Breakpoint Mapping Across Compilation

Breakpoints set in source code must survive the compilation pipeline:

**AST-level breakpoints:** During parsing, record breakpoint locations (line/column) in metadata.

**Codegen preservation:** When generating code, translate AST-level breakpoint locations to output code locations and embed in source map.

**Multiple targets:** A breakpoint in Left-Right source might map to different locations in JavaScript vs native output. Store breakpoints at the source level and resolve them per-target during debugging.

**Conditional breakpoints:** Support conditional breakpoints like "break when x > 10" by evaluating the condition in the runtime.

---

## 3. REPL Design

A Read-Eval-Print Loop enables rapid experimentation and learning. For compiled languages, REPL design requires special consideration.

### REPL Architecture for Compiled Languages

**Compilation pipeline in REPL:** Each input line goes through the full pipeline: lexing, parsing, type inference, code generation, execution.

**Two approaches:**
1. **JIT compilation:** Compile to native code on-the-fly and execute. Requires embedded compiler in REPL binary.
2. **Interpretation:** Parse and execute directly via an interpreter. Faster startup, easier to implement, but slower execution.

For Left-Right, a hybrid approach works well: parse and type-check once, then either JIT compile or interpret based on expression complexity.

**REPL vs script mode:** REPL sessions maintain state across inputs, while scripts are self-contained. Design the compiler core to support both execution models.

### Incremental Compilation in REPL Context

**Incremental parsing:** Parse new inputs in context of previous state. Maintain a parser state that can be extended.

**Incremental type inference:** Track the type environment across inputs. When new code enters, infer types against the existing environment, update the environment with new bindings.

**Hot reloading:** Support redefining functions and variables. When a user redefines a symbol, update the runtime environment and invalidate dependent code.

**Import caching:** When imports are used in REPL, cache the imported modules to avoid recompilation on every session restart.

### Expression Evaluation vs Statement Execution

Left-Right is expression-based (every .lr file is a single root expression). The REPL should follow this model:

**Expression evaluation:** Each line is an expression whose result is printed. Input: `[1, 2, 3] ${x -> x * 2}` Output: `[2, 4, 6]`.

**Declaration syntax:** Store results by creating maps with keys. To create a binding, use map syntax: `{result: expr}`.

**Multi-line input:** Detect incomplete expressions (unclosed parentheses, unmatched operators) and prompt for continuation lines.

**Implicit printing:** Don't print results for declarations. Print results for expression-only inputs.

### REPL State Management

**Variable bindings:** Left-Right has no standalone variables. REPL evaluates expressions and shows results. Previous results could be referenced by index (`_1`, `_2`) if needed.

**Operator definitions:** Operators (stored in maps) defined in REPL are added to the environment and can reference previously defined symbols.

**Import management:** Track `imports` variable references across the session. `imports` is a runtime map, not a keyword.

**Session persistence:** Optionally support saving/loading REPL sessions to files for reproducibility.

**Scope isolation:** Create sub-maps for nested contexts. Maps defined inside don't leak to outer context unless explicitly merged.

**Error recovery:** When an input errors, don't discard the entire state. Report the error and continue with the previous environment intact.

### Tab Completion for Operators

Left-Right's operator-heavy syntax benefits from smart completion:

**Operator completion:** Complete operator names when typing: `fil<TAB>` -> `filter`. Show usage example: `list ?{predicate}` for filter, `list ${(x -> x * 2)}` for map.

**Chained completion:** When typing operator chains, suggest compatible operators based on type. After `list ${...}`, suggest `?{` for filter, `#`, for count (operators that work on arrays).

**Library completion:** Complete library names: `Array.`<TAB> -> show available operators from array library via `imports` lookup.

**Signature preview:** Show usage examples in completion results to help choose the right operator.

**Fuzzy matching:** Support fuzzy completion for typos: `flt` matches `filter`.

### History and Readline Integration

**Command history:** Store previous inputs in a history file (`~/.lr_history`). Navigate with up/down arrows.

**Search history:** Reverse search with Ctrl+R, matching commands by pattern.

**Readline library:** Use a mature readline implementation like Rust's `rustyline` for:
- Line editing (arrow keys, home/end)
- History navigation
- Multi-line editing
- Custom keybindings

**Keyboard shortcuts:** Support common shortcuts:
- Ctrl+C: Cancel current input (don't exit)
- Ctrl+D: Exit REPL (on empty line)
- Ctrl+L: Clear screen
- Tab: Complete or indent
- Shift+Tab: Dedent

### Production Examples

**Rust evcxr:** Rust's REPL that compiles Rust code on the fly.
- Uses cargo for compilation
- Maintains a crate context across inputs
- Supports external dependencies
- Prints expression results with `Debug` formatting

**Python REPL:** The gold standard for REPL UX.
- Auto-indentation for blocks
- Tab completion for attributes and methods
- `_` variable for last result
- `help()` built-in for documentation
- `pprint` for pretty printing

**Node.js REPL:** JavaScript REPL with ES6 support.
- Multi-line expression editing
- `.load` command to load files
- `.save` command to save session
- `.editor` mode for multi-line input

Left-Right REPL should combine the best: Python's usability with Rust's type system awareness.

---

## 4. Language Server Protocol (LSP)

LSP enables editor integration without writing editor-specific plugins. An LSP implementation provides IDE features across VS Code, Vim, Emacs, and more.

### LSP Protocol Overview

**Client-server model:** Editor (client) sends requests to language server, which responds with results. Server runs as a separate process.

**Key capabilities:**
- Text synchronization (document changes)
- Diagnostics (errors and warnings)
- Hover information
- Go-to-definition
- Completion
- Code actions (quick fixes)
- Symbol search
- Code formatting
- Semantic highlighting

**Server lifecycle:** Start on first file open, stay running for the session, handle multiple documents concurrently.

### Implementing Diagnostics

**Incremental analysis:** As documents change, re-parse and re-check affected regions. Don't re-analyze the entire workspace on every keystroke.

**Debouncing:** Delay analysis by 100-500ms after last keystroke to avoid thrashing on rapid typing.

**Diagnostic caching:** Cache analysis results per document. When a document changes, invalidate cached results and recompute.

**Severity levels:** Map Left-Right's error categorization to LSP severities:
- Error = `Error`
- Warning = `Warning`
- Hint = `Hint`
- Suggestion = `Information`

**Related information:** For errors with context, provide "RelatedInformation" links to point to other locations (e.g., where a value was defined, where a library is referenced).

### Hover Information

**Type hover:** Hovering over an expression shows its inferred type: `arr map (x -> x * 2)` -> hover shows `[Number]` or generic `[a]` depending on type inference.

**Documentation hover:** Hovering over operators shows documentation:
```
map (list operator)

Apply an operator to each element of a list.

Example:
[1, 2, 3] ${x -> x * 2}  // [2, 4, 6]
```

**Signature hover:** For functions, show parameter names and types in hover.

**Cross-reference hover:** Show where a symbol is defined when hovering over a usage.

### Go-to-Definition for Dynamic Languages

Left-Right is dynamically typed, which complicates go-to-definition:

**Static analysis:** Follow symbol references through imports and module scopes even without static types. If `key@data` is used, find where `key` is accessed.

**Protocol tracking:** If the language has protocols (shared operator contracts), go-to-definition should resolve to the protocol implementation when known.

**Multiple candidates:** When a symbol could refer to multiple definitions (e.g., overloaded operators), show all candidates and let the user choose.

**Ambiguity handling:** For truly dynamic references (e.g., computed property access), show "Cannot resolve definition dynamically" instead of failing.

### Symbol Search for Operators

**Workspace symbol search:** Search for operators and definitions across all files. Support patterns like `map*` to find `map`, `mapWithIndex`, etc.

**Document symbol search:** List all symbols in current document. For Left-Right, this includes keys exported via `}@&[...]` pattern at end of file.

**Symbol kind:** Categorize symbols as `Operator`, `Map`, `List`, etc. For operator-heavy syntax, most bindings are operators stored in map context.

**Symbol hierarchy:** Group symbols by library. Show the library access tree to help navigation.

### Code Formatting for Point-Free Style

**Formatting challenges:** Operator-heavy, point-free code needs careful line wrapping and operator alignment.

**Wadler's prettiest printer:** Use the Wadler-Leijen algorithm for pretty printing, which balances line breaks and spacing for readability.

**Operator precedence:** Break lines at major precedence boundaries. For `arr map fn filter pred sum`, prefer breaking after `map` or `filter` rather than in the middle of a function.

**Alignment:** Align similar operators vertically for readability:
```
arr  map fn
     filter pred
     sum
```

**Width budget:** Respect editor line width (typically 80-120 chars) and wrap accordingly.

**Configuration:** Support `.editorconfig` or `.lrrc.json` for user preferences (indent size, line width, operator alignment style).

### Incremental Parsing for Responsive LSP

**Avoid re-parsing everything:** When a document changes, only re-parse the affected regions. Use incremental parsing techniques to reuse previous parse tree where unchanged.

**Tree-sitter integration:** Consider using tree-sitter for parsing, which provides:
- Incremental parsing out of the box
- Syntax tree queries for analysis
- Error recovery for robustness

**Change ranges:** LSP sends "text document change" events with change ranges. Update the AST in-place rather than rebuilding from scratch.

**Background analysis:** Offload expensive analysis (type inference, dead code detection) to background threads to keep the UI responsive.

### Production Examples

**rust-analyzer:** Rust's LSP server, known for speed and correctness.
- Uses salsa for incremental computation
- Fly-checks code in background
- Excellent error recovery
- Support for macros and procedural macros

**TypeScript Language Server:** The reference LSP implementation for TypeScript.
- Incremental type inference
- Cross-file refactoring
- Excellent code navigation
- Built-in compiler integration

Left-Right LSP should aim for rust-analyzer's responsiveness with TypeScript's feature completeness.

---

## 5. Build System and Package Manager

Left-Right needs a build system to manage dependencies, compile projects, and enable iterative development.

### Package.lr Format

`package.lr` defines project metadata with name, version, scripts, and requiredLibraries:

```
name: "my-left-right-project"
version: "1.0.0"
description: "A sample Left-Right project"
author: "Your Name <you@example.com>"
license: "MIT"

scripts:
  build: "src/build.lr"
  test: "src/test.lr"
  dev: "src/dev.lr"

requiredLibraries:
  - `array`
  - `string`
  - `async`
```

**Scripts mapping:** The `scripts` key maps to file paths. `lr build` executes the script at `scripts.build` path.

**Required libraries:** `requiredLibraries` lists external libraries needed by the project. Libraries are loaded at runtime via the `imports` variable.

### Library Resolution

**Library lookup:** The `imports` runtime variable provides access to standard and external libraries. Libraries are resolved based on their name.

**File imports:** Use `files@[`path`]` to access local `.lr` files.

**Circular dependencies:** Handled at runtime via the `imports` variable.

**Lockfile:** Generate a `package.lr.lock` file with resolved library versions for reproducible builds if using external libraries.

### Incremental Build System Design

**Dependency tracking:** Track which files depend on which other files. When a file changes, only recompile affected files.

**File system watching:** For `lr watch` mode, watch for file changes and trigger rebuilds automatically.

**Dirty checking:** Compare file modification times or hashes to determine what changed.

**Build cache:** Cache compilation outputs in `.lr/cache`. Skip recompilation for unchanged files.

**Parallel compilation:** Compile independent files in parallel to speed up builds.

**Topological sorting:** Compile files in dependency order (dependencies before dependents).

### Watch Mode for Development

**Hot reload:** Watch for changes and recompile automatically. For web targets, trigger browser reload via HMR (Hot Module Replacement).

**Debouncing:** Wait 100-500ms after last change before rebuilding to avoid thrashing on rapid saves.

**Error feedback:** Display build errors in real-time without requiring manual rebuilds.

**Selective rebuild:** Only rebuild the file that changed and its dependents, not the entire project.

**Terminal integration:** Use terminal spinner or progress indicator for build status.

### Build Caching Strategies

**Content-addressed cache:** Cache compilation outputs keyed by file content hash. If a file is reverted to a previous version, use cached output.

**Remote caching:** For CI/CD, fetch cache from a remote store to avoid recompiling unchanged code.

**Cache invalidation:** Invalidate cache entries when:
- Source file changes
- Dependency versions change
- Compiler version changes
- Compiler flags change

**Cache pruning:** Periodically clean up old cache entries to prevent disk bloat. Keep most-recently-used entries.

### Integration with Existing Tools

**npm for JavaScript dependencies:** When targeting JavaScript, allow importing npm packages via a compatibility layer. Generate separate `package.json` with dependencies for use by bundlers if needed.

**Cargo for native dependencies:** When compiling to native via Rust, optionally support Cargo dependencies through a `Cargo.toml` bridge.

**Language-agnostic package manager:** Consider building Left-Right's own package registry and manager (similar to Cargo, npm, pip) for language-specific packages.

**Polyglot projects:** Support mixed-language projects where Left-Right code imports libraries from other languages via FFI (Foreign Function Interface).

---

## 6. Testing Infrastructure

A compiler requires comprehensive testing infrastructure to ensure correctness and prevent regressions.

### Testing Framework for Compiler Stages

**Unit tests per stage:** Test each compilation stage independently:
- Lexer: Test tokenization of various inputs
- Parser: Test AST generation from source code
- Type inference: Test type inference and error detection
- Code generator: Test output code generation for various inputs

**Property-based testing:** Use property-based testing (e.g., QuickCheck) for compiler passes. For example: "round-tripping (parse -> print -> parse) should produce the same AST".

**Golden file testing:** For code generation, store expected output files and compare against generated output. Use `cargo test`'s `--ignored` flag or similar for large test suites.

**Error test cases:** Test that the compiler produces the correct errors for invalid inputs. Store error messages in test expectations and update them intentionally when error messages change.

**Stage integration tests:** Test the full pipeline from source to output to catch integration issues between stages.

### Snapshot Testing for Code Generation

**Snapshot testing:** For each test input, generate code and store the output as a "snapshot". On subsequent runs, compare new output against the snapshot. If output changes intentionally, update the snapshot.

**Snapshot files:** Store snapshots alongside test files, typically in `snapshots/` directory. Use descriptive names like `map_operator_codegen.snap`.

**Snapshot review:** When snapshots change, use tools like `insta` or `snapshots` to review diffs and approve changes.

**Platform-specific snapshots:** Different platforms may produce different code. Store platform-specific snapshots or use snapshots only for platform-independent tests.

**Snapshot compression:** For large snapshot files, compress them or store only the essential differences.

### Fuzz Testing the Parser/Lexer

**Fuzzer integration:** Use fuzzers like AFL++ or libFuzzer to find edge cases in the lexer and parser. Fuzzers generate random or mutated inputs to uncover crashes and panics.

**Grammar fuzzing:** Generate random valid inputs based on the grammar to ensure the parser handles all syntactic constructs correctly.

**Error path fuzzing:** Also fuzz invalid inputs to test error recovery. The parser shouldn't crash on malformed input.

**Coverage guidance:** Use coverage reports to ensure fuzzing exercises all parser branches.

**Regression testing:** Save any fuzzed inputs that cause crashes as regression tests. Fix the bug and add the input to the test suite.

### Property-Based Testing for Semantics

**Semantic properties:** Define properties that the compiled code must satisfy. For example:
- "Compiling and executing code should produce the same result as interpreting it"
- "Type inference should never produce false negatives (if code compiles, it should be safe)"
- "Inline expansion should preserve semantics"

**Model checking:** For small inputs, compare the behavior of compiled code against a reference implementation (e.g., an interpreter).

**Random test generation:** Generate random Left-Right programs and test that they compile correctly and produce expected results.

**Equivalence testing:** Generate two equivalent programs (e.g., using different operators that should produce the same result) and test that they compile to equivalent output.

### Benchmarking Infrastructure

**Criterion integration:** Use Criterion (Rust) or similar benchmarking framework to measure compiler performance.

**Benchmark suites:** Create benchmark suites for:
- Parsing large files
- Type checking complex programs
- Code generation for various targets
- Full compilation of real-world projects

**Performance regression testing:** Run benchmarks in CI and fail if performance degrades beyond a threshold.

**Profiling integration:** Generate profiling data (flame graphs, call graphs) for hot paths.

**Memory profiling:** Track memory usage during compilation and detect leaks or excessive allocations.

---

## 7. Formatter and Linter

Code formatters and linters enforce consistency and catch common errors.

### Code Formatting for Operator-Heavy Languages

**Operator precedence formatting:** Format operator chains to reflect precedence. Higher precedence operators stay closer to their operands.

**Whitespace rules:** Define clear rules for whitespace around operators:
- Binary operators: space before and after (`a + b`)
- Unary operators: no space after (`-x`)
- Pipe/composition operators: consistent spacing

**Line breaking:** Break long operator chains at logical boundaries. Prefer breaking before or after major operators rather than in the middle of expressions.

**Indentation:** Use consistent indentation (2 or 4 spaces). Align continued lines under the operator or expression start.

**Comment formatting:** Format comments consistently with the surrounding code.

### Pretty-Printing Algorithms

**Wadler-Leijen algorithm:** The de facto standard for pretty printing. It uses a "doc" algebra with concatenation, line breaks, and indentation operators.

**Layout combinators:** Build combinators like:
- `group(d)`: Try to fit `d` on one line, or break if too wide
- `nest(i, d)`: Indent `d` by `i` spaces
- `line`: Optional line break (break if needed, space otherwise)
- `hardline`: Mandatory line break

**Width awareness:** The algorithm is aware of the page width and chooses layouts accordingly.

**Alt layouts:** Provide multiple layout options (compact vs verbose) and let the pretty printer choose.

**Custom operators:** Define pretty-printing rules for Left-Right's custom operators to handle syntax-specific formatting.

### Linting Rules for the Language

**Unused map keys:** Detect map keys that are defined but never referenced.

**Dead code:** Detect expressions that have no effect (e.g., computing a value but never using it).

**Unused library references:** Detect libraries loaded via `imports` but never referenced.

**Type checks:** Suggest using type-check operators like `?#`, `?\"`, `?[]` for clarity in complex expressions.

**Operator misuse:** Detect patterns that suggest misuse of operators (e.g., using `${` when `?{` is more appropriate).

**Naming conventions:** Enforce consistent naming for map keys and operators.

**Code complexity:** Measure cyclomatic complexity and flag overly complex expressions.

**Anti-patterns:** Detect common anti-patterns for the language (e.g., unnecessary chaining, inefficient constructs).

### Auto-Fix Capabilities

**Lint auto-fix:** When a lint rule has a clear fix, apply it automatically. For example:
- Remove unused map keys
- Remove unused library references
- Apply consistent formatting

**Safe fixes:** Only auto-fix when the fix is unambiguous and safe. Require manual review for ambiguous cases.

**Refactoring suggestions:** Suggest code refactorings like "extract this expression into a function" or "replace this pattern with a more idiomatic alternative".

**Batch fixes:** Support applying multiple fixes at once for efficiency.

**Fix review:** Show a diff of proposed fixes before applying, or apply immediately with undo support.

---

## 8. Implementation Recommendations

Building all this tooling at once is overwhelming. Here's a prioritized roadmap.

### Priority Order for Tooling Development

**Phase 1: Core tooling (MVP)**
1. Error reporting with clear messages
2. Basic REPL for experimentation
3. Simple build system (compile single files)

**Phase 2: Developer experience**
4. Source map generation for JavaScript target
5. Tab completion in REPL
6. Basic LSP diagnostics

**Phase 3: Advanced tooling**
7. Full LSP implementation (hover, go-to-definition, completion)
8. Formatting and linting
9. Comprehensive testing infrastructure

**Phase 4: Ecosystem**
10. Package manager and dependency resolution
11. Incremental builds and caching
12. Advanced debugging (native DWARF support)

### MVP Tooling (What to Build First)

**Minimum Viable Product:**
1. **Error messages:** Focus on clarity and helpfulness first. Implement error spans and suggestions.
2. **REPL:** Build a basic REPL that can parse and evaluate expressions. Start without advanced features like tab completion or history.
3. **Build system:** Implement a simple `lr build` command that compiles entry points. Add `package.lr` for basic configuration.
4. **Testing:** Set up unit tests for each compiler stage. Add a few snapshot tests for code generation.

**Why this order:** Error messages and REPL provide immediate value to users. The build system is needed for practical projects. Testing ensures stability before adding more features.

### LSP Implementation Roadmap

**Step 1: Diagnostics (high value, low complexity)**
- Implement LSP server scaffolding
- Publish diagnostics from the compiler
- Handle text synchronization
- Support basic error/warning/hint severity

**Step 2: Hover and completion (medium value, medium complexity)**
- Implement hover for type information
- Add symbol-based completion for operators
- Show documentation in hover

**Step 3: Navigation (medium value, high complexity)**
- Implement go-to-definition
- Add symbol search
- Handle cross-file references

**Step 4: Formatting and code actions (high value, medium complexity)**
- Implement code formatter
- Add quick fixes for common errors
- Support code actions for refactoring

**Step 5: Optimization and polish (lower value, high complexity)**
- Implement incremental analysis for speed
- Add semantic highlighting
- Optimize for large workspaces

### REPL Architecture Recommendation

**Recommended architecture:**
1. Use a parser that supports incremental parsing (tree-sitter or hand-rolled incremental parser)
2. Build a type checker that maintains an environment across inputs
3. For execution, start with an interpreter for simplicity. Add JIT compilation later if needed.
4. Use a mature readline library (rustyline for Rust, readline for Python) for history and editing
5. Implement tab completion by querying the compiler's symbol table

**Execution model:**
- Parse input into AST
- Type-check in context of current environment
- Either interpret AST or compile to bytecode
- Execute and print result
- Update environment with new bindings

**State management:**
- Store environment in a persistent data structure (e.g., `HashMap<String, Value>`)
- For each input, create a fresh scope but inherit from the parent environment
- Support redefinition by updating the environment in-place
- Maintain a history of inputs for replay and session persistence

**Scalability:**
- Start with a simple in-memory environment
- Add file-backed persistence for session saving
- Consider module system for importing libraries into REPL context
- Support multi-line editing for complex expressions

---

## Conclusion

Tooling is not an afterthought; it is the developer experience. A great language with poor tooling will struggle to gain adoption, while a decent language with excellent tooling can thrive.

For Left-Right, focus on error messages first, then REPL, then LSP, then advanced features. Build incrementally, get user feedback, and iterate. The goal is to make Left-Right not just a language, but a productive and joyful development experience.