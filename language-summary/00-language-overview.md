# Language Overview — Left-Right

## Name Evolution

The language has evolved through three names, reflecting design refinement and philosophical shifts:

1. **Penscript** — Earliest name, reflecting initial focus on string-based scripting
2. **PenroScript** — Intermediate name (used in early ChatGPT conversations), emphasizing operator-centric approach
3. **Left-Right** — Current name, directly reflecting the fundamental left-to-right evaluation model

The transition to "Left-Right" explicitly names the core execution paradigm, making evaluation order immediately clear from the language name itself.

---

## Core Paradigm

Left-Right is built on five interconnected principles:

### Point-Free Style

Functions are chained without explicit argument passing through operator composition. Arguments flow implicitly through the chain, reducing cognitive load by eliminating intermediate variable assignments.

**Example:**
```penroscript
// Instead of: const x = doThis(input); const y = doThat(x); doFinal(y)
input $doThis $doThat doFinal
```

### Operator-Based Syntax

Operators are first-class values that can be:
- Stored in maps
- Passed as arguments
- Returned from functions
- Composed and chained

This contrasts with languages where operators are purely syntactic sugar for function calls.

### Inspired by List-Oriented Languages

Collections (lists and maps) are first-class citizens. Operations are designed for bulk data transformation rather than single-value processing. Mapping, filtering, reducing, and other collection operations are central to the language.

### Loosely Typed

No type declarations required. Runtime type inference determines operator behavior. The same operator changes meaning based on input type (type-dependent behavior).

**Example:**
```penroscript
// + means concatenation for text, addition for numbers, merging for lists
1 + 2           // 3 (number addition)
`hello` + `world`  // `helloworld` (text concatenation)
[1,2] + [3,4]  // [1,2,3,4] (list concatenation)
```

### Left-to-Right Evaluation

Every operator evaluates left to right unless grouped by parentheses. This eliminates precedence confusion and makes execution order explicit and readable.

**Example:**
```penroscript
3 + 4 * 2  // (3 + 4) * 2 = 14, NOT 3 + (4 * 2) = 11
```

---

## Syntax Structure

### JSON-Like Structure

The language uses familiar JSON-like syntax for both data and program expressions:

- **Maps:** `{ key: value, key2: value2 }`
- **Lists:** `[ value1, value2, value3 ]`
- **Text:** `` `text` `` (backticks ONLY — single/double quotes reserved for operator names)
- **Numbers:** `42` or `3.14`
- **Undefined:** `undefined` (truthy/falsy values like 0, empty list, empty map are falsy)

### Operators as Maps

Functions/operators have the same syntax as maps but are distinguished by:
1. Last item is an expression (unexecuted operator returning result)
2. Presence of `_</_>` directional placeholders (unexecuted operator with arguments)

**Example:**
```penroscript
// Map data structure
{ a: 1, b: 2 }

// Unexecuted operator (last item is expression)
{ a: 1, b: 2, a+b }

// Unexecuted operator with directional placeholders
{ _<, b: a+1 }
```

### Text Interpolation

Text supports interpolation using curly braces:
```penroscript
// Text with variable
`Hello {name}`

// Text becomes operator when _</_> used
`Thanks {_<}`
```

---

## Transpilation Targets

### JavaScript/TypeScript

Primary target with full feature parity. Transpiler generates clean, idiomatic JS/TS code with:
- Proper type annotations (for TS target)
- Efficient data structure mapping
- Source maps for debugging

### Rust

Secondary target providing:
- Type safety at compile time
- Performance optimizations
- Native execution without runtime overhead

Both targets maintain deterministic semantics and operator behavior from Left-Right source.

---

## Key Differentiators

### From APL/J/K

| Aspect | APL/J/K | Left-Right |
|---------|-------------|-------------|
| Syntax | Special characters, steep learning curve | ASCII-friendly, JSON-like syntax |
| Paradigm | Right-to-left evaluation | Left-to-right evaluation |
| Data Types | Primitive-heavy | JSON-like types familiar to web devs |
| Learning Curve | High due to special characters | Low due to familiarity |

### From PureScript/Elm

| Aspect | PureScript/Elm | Left-Right |
|---------|----------------|-------------|
| Type System | Static, strong, typed | Loose, runtime inference |
| Type Declarations | Required | Never required |
| Paradigm | Pure functional | Functional with pragmatic trade-offs |
| Learning Curve | Moderate | Low for JS developers |

### From Clojure

| Aspect | Clojure | Left-Right |
|---------|-----------|-------------|
| Syntax | Lisp-based parentheses | JSON-like maps/arrays |
| Evaluation | Left-to-right | Left-to-right |
| Data Immutability | Strict immutability | Immutable by convention, mutable options available |
| Operator First-Class | Functions are first-class | Operators are first-class |

### Unique Innovations

1. **Operators as First-Class Values** — Can be stored, passed, returned, manipulated like any other value
2. **Spatial/Compounding Symbology** — Related operators compound symbols (e.g., `^` = uppercase, `^_` = capitalize, `"` = toLower)
3. **Type-Dependent Behavior** — Same operator adapts behavior based on input type
4. **Left-Hungry Auto-Currying** — Operators automatically curry from left when partial application occurs
5. **JSON-Like Program Structure** — Programs look like data, reducing cognitive dissonance between code and configuration

---

## Design Goals

From the brainstorm documents and design specifications:

### Ergonomics Priorities

1. **Readability > Brevity** — Prefer clear, understandable code over extreme terseness
2. **Left-to-Right Flow Over Nesting** — Linear chains reduce cognitive load
3. **Low Ceremony Over Strictness** — Minimal syntax, no type declarations, implicit features where clear

### Problems Optimized For

1. **Data-Transform Pipelines** — Chaining operations for ETL workflows
2. **Templateable General-Purpose Scripting** — Text interpolation and operator creation in templates
3. **Config + Compute** — Configuration files that contain executable logic
4. **Functional Composition** — Point-free style enabled through operator chaining

### Execution Model

1. **Deterministic** — Given input, always produces same output
2. **Left-to-Right** — Consistent evaluation order, no precedence ambiguity
3. **Expressive** — Rich operator set with type-dependent behavior
4. **Composable** — Every operation returns value for next operation

### Semantics Guarantees

1. **No Hidden Nondeterminism** — Random evaluation or iteration order
2. **Consistent Iteration Order** — Maps and arrays maintain predictable order
3. **Reproducible Operations** — Same code, same input → same output
4. **Pure Function Semantics** — No global state mutation by default

### JavaScript Familiarity

The language is designed so JavaScript engineers find semantics intuitive:

- **Text syntax**: Backticks match JS template literals
- **Comparison operators**: `==` for strict type checking (like JS `===`), `=` for loose equality (like JS `==`)
- **Loose typing**: Runtime type inference similar to dynamic typing in JS
- **JSON structure**: Data structures identical to JavaScript objects/arrays
- **List methods**: Familiar collection operations (map, filter, reduce) available

While having power and brevity of languages inspired by array-oriented languages (APL/J/K), the semantics feel natural to JS developers.

---

## The "Hashmap-Array of Intent" Concept

A core philosophical insight: programs are structured as hashmaps (maps) where keys represent operations and values represent data flow, creating an "array of intent."

### Intent Flow

Keys are evaluated top-to-bottom, with each key's value becoming accessible to subsequent keys:

```penroscript
{
  step1: { a: 1, b: 2 },
  step2: step1.a + step1.b,  // Accesses step1 keys
  step3: step2 * 10
}
```

This creates a linear narrative of computation where each key describes an operation's intent.

### Operators as Intent

When an operator is defined, it captures computational intent as a map structure:

```penroscript
// Operator to count malicious threats
countMalicious: { threats: _<,
  threats $?{ @['AI Confidence Level', 'value'] = 'malicious' } #
}
```

The map structure clearly documents:
- Input (`threats`)
- Operation (filter by malicious confidence)
- Output transformation (count)

This self-documenting nature makes code "look like data" while "acting like code."

---

## Kieran Brown's Feedback

In design discussions, Kieran Brown noted several characteristics:

### Algol-Family Flavor

The language exhibits Algol-family characteristics:
- Block-structured execution
- Clear operator precedence
- Familiar expression syntax
- Statement-based structure (key evaluation order)

### Modula-2/Oberon Comparison

Similarities to Modula-2 and Oberon:
- Module-like structure (files as modules/packages)
- Clear separation between data and operations
- Type safety through runtime inference
- Explicit import/export semantics

These connections ground Left-Right in established language family traditions while innovating in operator-first design and left-to-right evaluation.

---

## Current Status

**Implementation Phase:** Transpiler Development

The language specification is comprehensive with:
- Complete type system defined
- Full operator set specified
- Evaluation model clarified
- Design principles documented
- Transpilation architecture designed

**Next Steps:**
- Complete Rust-based transpiler implementation
- Build standard library with reference implementations
- Create comprehensive test suite
- Develop tooling (CLI, formatter, linter)

---

## Quick Start

### Hello World

```penroscript
{
  greeting: `Hello, World!`,
  greeting
}
```

### Simple Pipeline

```penroscript
// Filter, transform, and count
{
  data: [1, 2, 3, 4, 5],
  result: data
    $?{ _< > 2 }        // Filter: greater than 2
    ${ _< * 2 }          // Map: multiply by 2
    #                    // Count
}
```

### Text Template

```penroscript
{
  name: `Alice`,
  greeting: `Hello, {name}!`,
  greeting
}
```

---

## Related Documentation

- [Design Philosophy](./01-design-philosophy.md) — Deep dive into language philosophy and design principles
- [Type System](./02-type-system.md) — Complete type system documentation
- [Operator Reference](./03-operator-reference.md) — Comprehensive operator reference
- [Master Index](./README.md) — Complete documentation suite index
