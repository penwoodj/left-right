# Language Design — Comprehensive Specification

Complete specification checklists and detailed design decisions for PenroScript language.

## Contents

- [`01-chatgpt-designing-a-programming-language.md`](./01-chatgpt-designing-a-programming-language.md) — 1609 lines of comprehensive design decisions
- [`Penscript_LeftRight brainstorm.md`](./Penscript_LeftRight%20brainstorm.md) — 246 lines of 25-category checklist
- [`03-existing-project-brainstorms.md`](./03-existing-project-brainstorms.md) — Reference index to existing docs

## Key Topics

### 25-Category Specification Checklist

From [`Penscript_LeftRight brainstorm.md`](./Penscript_LeftRight%20brainstorm.md):

1. **Language Philosophy & Goals** — Problems optimized for, ergonomics priorities
2. **Core Evaluation Model** — LTR evaluation, strict/lazy operators, expression sequencing
3. **Types & Values** — Complete type set, Undefined vs Null, truthiness
4. **Variables, Names, and Assignment** — Declaration, immutability, scope, name resolution
5. **Functions & Calls** — Definition, arity, calling syntax, partial application
6. **Operators (Design & Extensibility)** — Left hungry definition, associativity, precedence table
7. **The `_/` Directional Forms** — `_</_>` grammar, callable values vs macros
8. **Collections & Paths** — `@` get operator syntax, missing key behavior, set/update
9. **Conditionals, Patterning & Control Flow** — Conditionals, looping, pattern matching
10. **Strings & Templates** — Interpolation syntax, `_</_>` as templating operators
11. **Standard Library (HOO & Data Ops)** — HOFs, composition/pipe, equality/comparison
12. **Modules, Files, & Imports** — File as module, import syntax, circular imports
13. **Interop (TS/JS & JSON)** — JS object interop, host function calls, type marshalling
14. **Comments, Whitespace, and Layout** — Comment syntax, significant whitespace, indentation
15. **Errors, Diagnostics, and Types (Static vs Dynamic)** — Static typing, runtime errors, lints
16. **Macros & Metaprogramming** — Doc-blocks as executable meta, AST transformation
17. **Performance & Semantics Guarantees** — Determinism, tail-call, Big-O, memory model
18. **Concurrency & Effects** — Concurrency primitives, IO model, effects representation
19. **Tooling & Packaging** — CLI commands, formatter, package manager
20. **Syntax Reference & Grammar** — EBNF grammar, operator tokenization, reserved words
21. **Examples & Canonical Patterns** — Hello World, data-transforms, templating, pipelines
22. **Backwards/Forwards Compatibility** — Stability policy, versioning, feature flags
23. **Security & Sandboxing** — Untrusted code safety, resource limits, capability objects
24. **Testing & Documentation Conventions** — Doctest, snapshot tests, error golden tests
25. **Migration & Transpilation** — TS/JS → PenroScript patterns, PenroScript → JS/TS guarantees

### Design Decisions & Open Questions

From [`01-chatgpt-designing-a-programming-language.md`](./01-chatgpt-designing-a-programming-language.md):

#### TODO Items Identified
1. **Conditionals** — Think through way to do conditionals with just boolean variables
2. **uniqueWith** — Figure out way of doing uniqueWith without running into threatClassifications problem
3. **And Ternaries** — Figure out way of doing And Ternaries cleaner
4. **Currying** — Diadic functions if given a static type to either side return a monadic function
5. **$>< operator** — Syntax clarification for nested conditions

#### Key Design Principles
1. **Point-Free Style** — Language enables point-free programming by chaining operators
2. **JSON-like Structure** — Uses Map/Map-like structures for both data and operators
3. **Implicit Left Argument** — Single-expression operators can assume `_>` without explicit declaration
4. **Variable Scope** — Keys in Maps/Operators become accessible as variables to subsequent keys
5. **String Interpolation** — Curly braces in strings enable both interpolation and operator creation
6. **Boolean Keys in Operators** — Boolean expressions as keys enable conditional execution
7. **Operator Distinguishing** — Last item is expression → unexecuted operator; contains `_<` or `_>` → unexecuted operator
8. **Static Type Currying** — Operators with static values on one side return monadic functions
9. **Type System** — Four primitive types and two data structures

### Operator Table

From [`01-chatgpt-designing-a-programming-language.md`](./01-chatgpt-designing-a-programming-language.md#operators-table):

**Math:**
- `+` — add/concat/combine maps
- `-` — subtract/remove from list
- `%` — divide
- `*` — multiply/repeat list
- `**` — exponent
- `%%` — modulus/remainder

**Boolean:**
- `<` / `<=` / `>` / `>=` — comparisons
- `=` — equality
- `!` — not
- `&` — and
- `|` — or

**List/String:**
- `>.<` — includes/contains
- `<.` — startsWith
- `>.` — endsWith
- `?|` — some
- `$&` — all

**Getters:**
- `#` — size
- `@` — get
- `@+` — pick
- `@-` — omit
- `@0` — first/head
- `@-1` — last
- `@~` — tail
- `@\` — slice
- `@>` — values
- `@<` — keys

**String:**
- `><` — join
- `<>` — split
- `>"<` — replace
- `<"` — trim
- `^` — toUpper
- `_"` — toLower
- `^_` — capitalize

**Lists & Objects:**
- `$` — map
- `_` — flatten
- `$_` — flatMap
- `$+` — reduce
- `$><` — group
- `$?` — filter
- `$?.` — find
- `$?{_<}` — compact
- `$#` — chunk
- `$<` — applyToEachLeft
- `$>` — applyToEachRight
- `~` — unique
- `$~` — uniqWith
- `~~` — reverse
- `~?` — orderBy/sort
- `??` — shuffle

**Type Conversion:**
- `^"` — toString
- `^?` — toBoolean
- `^#` — toNumber
- `^]` — toList
- `^}` — listsToMap

### Code Examples

#### getResultForThisEntity
```javascript
// TypeScript/Lodash FP
const getResultForThisEntity = (
  entity: Entity,
  results: any[],
  onlyReturnUniqueResults: boolean = false
): any =>
  flow(
    filter(flow(get('resultId'), eq(entity.value))),
    flatMap(get('result')),
    onlyReturnUniqueResults ? uniqWith(isEqual) : identity
  )(results);
```

```javascript
// PenroScript
{ entity: _<@0, results: _<@1, onlyReturnUniqueResults: _<@2 | false,
  results
    $?{ @'resultId' = entity@'value' }
    $_{ @'result' }
    { onlyReturnUniqueResults ^?: _<~, _< }
}
```

#### Threat Analysis
```javascript
// JavaScript with Lodash FP
({ threats }) => {
  const maliciousThreatsCount = flow(
      filter((threat) => get(`['AI Confidence Level'].value`, threat) === 'malicious'),
      size
    )(threats);

  const threatClassifications = flow(
    map(flow(get(`['Classification'].value`), capitalize)),
    uniq,
    join(', '),
    (threatClassifications) =>
      threatClassifications && `Threat Classifications: ${threatClassifications}`
  )(threats);

  return []
    .concat(maliciousThreatsCount)
    .concat(threatClassifications)
}
```

```javascript
// PenroScript
{ threats: _<@[0,'threats'],
  maliciousThreatsCount: threats
    $?{ @['AI Confidence Level', 'value'] = 'malicious' }
    #,
  threatClassifications: threats
    ${ @['AI Confidence Level', 'value'] "^_}
    ~
    >< ', '
    { threatClassifications: _<,
      threatClassifications & 'Threat Classifications: {threatClassifications}'
    },

  [] + maliciousThreatsCount + threatClassifications
}
```

## Design Philosophy

1. **Terse DSL** — Optimized for data transformation and templating
2. **Left-to-Right** — Fundamental evaluation model
3. **Operator-Centric** — Operators are first-class citizens
4. **Deterministic** — No hidden nondeterminism
5. **JSON-like** — Familiar structure for data and programs
6. **Transpilation Target** — JavaScript/TypeScript
7. **No Explicit Control Flow** — Combinator-driven instead of if/for
8. **Directional Sections** — `_</_>` for evaluation order control

## Related Concepts

- **Language Specification** — Formal language definition
- **Design Checklist** — Comprehensive decision tracking
- **Operator Design** — Creating effective operators
- **Type System Design** — Defining language types
- **Syntax Design** — Language grammar and tokenization
- **Semantics** — Meaning of language constructs
- **Evaluation Strategy** — How programs execute
