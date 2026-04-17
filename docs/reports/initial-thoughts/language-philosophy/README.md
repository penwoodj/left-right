# Language Philosophy — Design Principles

Documents covering core philosophy, evaluation model, and design principles.

## Contents

- [`00-index.md`](./00-index.md) — Summary of previous thoughts and terminology
- [`Penscript_LeftRight brainstorm.md`](../language-design-comprehensive/Penscript_LeftRight%20brainstorm.md) — Section 1: Language Philosophy & Goals
- [`02-chatgpt-llm-pseudo-compiler-model.md`](../code-transpilation/02-chatgpt-llm-pseudo-compiler-model.md) — Section 7: PenroScript Spec Recall

## Key Topics

### Core Philosophy

#### Problems Optimized For
From brainstorm checklist:
- Data-transform pipelines
- Templateable DSLs
- Config + compute
- Functional composition

#### Ergonomics Priorities
- Readability > brevity
- Left-to-right flow over nesting
- Low ceremony over strictness

### Evaluation Model

#### Left-to-Right Evaluation
From [`PenroScript.md`](../pipe-operators/PenroScript.md):
```javascript
// Every operator evaluates LTR unless grouped
3 + 4 * 2 // (3 + 4) * 2 = 14
```

#### Expression Sequencing
- Multiple top-level expressions in file
- Newline-terminated statements
- Implicit block for file

#### Strict vs Lazy Operators
- Default is strict (eager)
- Some operators may short-circuit
- Lazy evaluation supported where beneficial

### Design Principles

1. **Operators as First-Class** — Can be stored, passed, returned
2. **JSON-like Structure** — Familiar data representation
3. **Deterministic Execution** — Given input, always same output
4. **Transpilation Target** — JavaScript/TypeScript runtime
5. **Point-Free Style** — Enable through chaining
6. **Minimal Ceremony** — No explicit control flow keywords
7. **Type-Dependent Behavior** — Operators adapt to input types
8. **Extensible Syntax** — Custom operators and symbols

### Type System

**Primitives:**
- String (`''`)
- Number (`0`)
- Boolean (`false`)
- Undefined (`undefined`)

**Data Structures:**
- Array (`[1, "a"]`) — Heterogeneous, ordered
- Map (`{a:1, b:2}`) — String-keyed, ordered

**Operators as Values:**
- First-class status
- Can be keys in Maps
- Can be elements in Arrays

### Interop Philosophy

#### JavaScript/TypeScript Integration
- Clean mapping to JS data structures
- Host function calling capability
- Runtime shims for language features
- Source maps for debugging

#### Serialization
- Deterministic JSON/YAML output
- AST interchange format
- Config + compute separation

### Error Handling

#### Default Behavior
- Errors tend to default to `Undefined`
- No explicit exception mechanism
- Optional chaining for missing keys
- Graceful degradation

### Semantics Guarantees

#### Determinism
- No random evaluation
- Consistent iteration order
- Reproducible operations

#### Composability
- Every operation returns value for next
- No global state mutation
- Pure function semantics

## Related Concepts

- **Language Design** — Creating programming languages
- **Evaluation Models** — How code executes
- **Semantics** — Meaning of programs
- **Determinism** — Predictable behavior
- **Functional Programming** — Pure functions, immutability
- **DSL Design** — Domain-specific languages
- **Language Philosophy** — Core principles and goals
- **Ergonomics** — User experience design
