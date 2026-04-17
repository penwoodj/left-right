# DSL Design — Domain-Specific Language Patterns

Documents covering domain-specific language (DSL) design principles and implementation patterns.

## Contents

- [`Penscript_LeftRight brainstorm.md`](../language-design-comprehensive/Penscript_LeftRight%20brainstorm.md) - 25-category specification checklist

## Key Topics

### DSL Design Principles (from brainstorm checklist)

#### 1. Language Philosophy & Goals
- Problems optimized for (data-transform pipelines, templateable DSLs, config + compute)
- Ergonomics priorities (readability vs brevity)
- Left-to-right flow over nesting
- Low ceremony over strictness

#### 2. Core Evaluation Model
- Every operator obeys left-to-right evaluation unless grouped
- Strict vs lazy operators
- Expression return value rules
- Sequencing multiple expressions
- Precedence interaction with operator direction

#### 3. Types & Values
- Complete type set (Operator, Hashmap, Array, String, Boolean, Number, Undefined)
- Distinction between Undefined vs Null
- Truthiness rules
- Ordered Maps/Arrays
- Operators as first-class values

#### 11. Standard Library (HOO & Data Ops)
- Canonical names and signatures for core HOFs
- Composition / pipe operators
- Equality & comparison semantics
- String operators
- Collection builders

### DSL Patterns

#### Functional Computation DSL
- Map/filter/some/reduce for iteration
- No explicit if/for loops
- Combinator-driven control flow

#### Template/Config DSL
- String interpolation
- JSON-like structure for data
- Dynamic key access via `@` operator

#### Data Transformation DSL
- Pipeline-based operations
- Left-to-right evaluation
- Point-free function composition

### DSL Implementation Considerations

#### Interop with Host Languages
- JavaScript/TypeScript as transpilation target
- Type marshalling (Map ⇄ Object, Array, Undefined/Null)
- Host function calling capability

#### Deterministic Serialization
- JSON/YAML compatibility
- AST interchange format
- Source maps for debugging

## Related Concepts

- **Domain-Specific Languages** — Tailored syntax for specific problem domains
- **Embedded DSLs** — DSLs within general-purpose languages
- **External DSLs** — Standalone language implementations
- **DSL Design Patterns** — Common patterns in DSL creation
- **Template Engines** — Specialized DSLs for string generation
- **Configuration Languages** — DSLs for system configuration
