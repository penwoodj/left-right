# Lisp Macros — Homoiconicity

Documents covering Lisp macros, homoiconicity, and code-as-data patterns.

## Contents

- [`09-chatgpt-functional-programming.md`](../functional-programming/09-chatgpt-functional-programming.md) — May contain Lisp/macro discussions

## Key Topics

### Homoiconicity

**Definition:**
- Code and data share same representation
- Program is data structure
- Metaprogramming operates on code as data

**Benefits:**
- **Uniform Syntax** — No separate "code" vs "data" syntax
- **Powerful Macros** — Transform code using code
- **Simplified Parsing** — Data structures are already parsed
- **Programmable Syntax** — Extend language through data manipulation

### Lisp Macros

#### Macro Definition
- Compile-time code transformation
- Pattern matching on code structure
- Hygienic variable capture
- Syntax expansion

#### Macro Use Cases
- **Domain-Specific Languages** — Create DSLs within Lisp
- **Code Generation** — Produce boilerplate programmatically
- **Optimization** — Rewrite patterns for performance
- **Control Flow Abstractions** — New iteration/conditional constructs

### Code-as-Data Patterns

#### JSON-as-Program-Structure
From PenroScript design:
- JSON-like syntax for programs
- Maps represent operators and data
- No separate "code" token vs "data" token
- Structure itself is the program

#### Program Representation
- **AST** — Abstract Syntax Tree as data
- **S-expressions** — Symbolic expressions
- **Quoted Code** — Treated as data, not evaluated
- **Evaluated Code** — Treated as code, executed

### Metaprogramming

#### Program Self-Modification
- Reflection and metaprogramming
- Dynamic code generation
- Runtime code manipulation
- Self-interpreting programs

#### Macro Hygiene
- Variable capture rules
- Preventing name collisions
- Predictable macro expansion
- Local scope isolation

### PenroScript Parallels

While not homoiconic, PenroScript shares concepts:
- **JSON-like Structure** — Programs as data structures
- **Operator Definition** — Maps that behave like functions
- **Template as Data** — Strings as operators
- **No Explicit Syntax** — Data structures represent code

**Differences:**
- PenroScript: Data and code both JSON-like
- Lisp: Explicit distinction (quote vs eval)
- PenroScript: Left-to-right evaluation
- Lisp: Nested evaluation with macro expansion

### Metaprogramming Techniques

#### Pattern Matching
- Destructuring on structure
- Guard clauses for conditions
- Recursive pattern matching
- Macro pattern matching

#### Code Generation
- Template-based generation
- Quasiquotation for structure capture
- Unquoting for evaluation
- Symbol manipulation

## Design Principles

1. **Homoiconicity** — Code and data share representation
2. **Macro Power** — Compile-time transformations
3. **Hygienic Expansion** — Safe variable capture
4. **Programmable Syntax** — Extend through data manipulation
5. **Clear Boundaries** — Explicit code vs data distinction
6. **Composability** — Macros compose like functions
7. **Predictable Behavior** — No hidden side effects in macros
8. **Debugging Support** — Understand macro expansion

## Related Concepts

- **Homoiconicity** — Code and data same structure
- **Macros** — Compile-time code transformation
- **Metaprogramming** — Programs manipulating programs
- **Code as Data** — Programs represented as data structures
- **S-expressions** — Symbolic expressions in Lisp
- **Quasiquotation** — Capture code as data
- **AST Manipulation** — Operate on syntax tree
- **Language-Oriented Programming** — DSLs within general languages
- **Metacircular Evaluator** — Self-interpreting programs
