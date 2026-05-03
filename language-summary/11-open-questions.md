# Open Questions — Unresolved Design Decisions

Left-Right language design has several open questions and areas requiring further exploration. This document tracks TODO items, unresolved decisions, and areas needing specification.

## Resolved Questions

### Text Syntax (RESOLVED)

**Status:** ✅ Resolved — Backtick-only syntax

**Decision:**
- `` `text` `` is ONLY text syntax in Left-Right
- No single quotes or double quotes for text — `"` and `'` are reserved for operator names
- NO operator may contain backtick `` ` `` character
- All text literals must use backticks

**Examples:**
```javascript
// Correct
name: `Alice`
path: `./utils.lr`

// Incorrect (these are for operator names)
name: "Alice"
path: './utils.lr'
```

**Rationale:**
- Clear distinction between strings and operators
- Eliminates ambiguity in parsing
- Supports operator names with quotes
- Unique, recognizable syntax

### Equality Operators (RESOLVED)

**Status:** ✅ Resolved — `==` vs `=` distinction

**Decision:**
- `=` is loose equality (like JS `==`) — type coercion allowed, unordered
- `==` is strict equality (like JS `===`) — type must match, ordered
- `0 = `0`` yields true (loose, type coerces)
- `0 == `0`` yields false (strict, type mismatch)

**Examples:**
```javascript
// Loose equality (=)
0 = `0`     // true  (number coerced to text)
0 = 0       // true
`0` = `0`   // true

// Strict equality (==)
0 == 0      // true
`0` == `0`  // true
0 == `0`    // false (type mismatch)

// Unordered comparison
{ a: 1, b: 2 } = { b: 2, a: 1 }  // true (same keys/values)
{ a: 1, b: 2 } == { b: 2, a: 1 } // false (order differs)
```

**Rationale:**
- Familiar to JS developers
- Clear distinction between loose and strict equality
- Type-safe comparisons when needed
- Flexible comparisons for data pipelines

## Conditional Syntax

### Ternary Expression

**Status:** TODO — Syntax not finalized

**Question:** How to express conditional expressions without explicit if/else keywords?

**Proposed Options:**

#### Option 1: Logical & Operators
```javascript
// Use & and | as ternary
condition & `truthy value` | `falsy value`

// Example
status ?& `Active` :| `Inactive`
```

**Pros:**
- Uses existing operators
- Consistent with LTR evaluation
- No new syntax

**Cons:**
- Ambiguous precedence without grouping
- May require parentheses for clarity

#### Option 2: Conditional Blocks
```javascript
// Logical keys as conditional
{
  condition: true,
  return: {
    condition ? `Value when true` : `Value when false`
  }
}
```

**Pros:**
- Clear conditional structure
- Familiar pattern from other languages

**Cons:**
- Verbose
- Requires map syntax

#### Option 3: Conditional Operator
```javascript
// New ternary operator
condition ? `if true` : `if false`
```

**Pros:**
- Familiar to most developers
- Concise

**Cons:**
- Adds new operator to language
- Requires precedence clarification

**Open Issue:** Need to decide which approach best fits Left-Right philosophy.

## UniqueWith Implementation

**Status:** TODO — Semantic not finalized

**Question:** How to implement `uniqueWith` without the "threatClassifications problem"?

**Problem Statement:**
The current `~?` (orderBy) operator sorts and `~` (unique) deduplicates. However, `uniqueWith` needs to deduplicate by a custom key while preserving order and avoiding incorrect semantic behavior.

**Proposed Approaches:**

#### Option 1: Unique by Path
```javascript
// Unique by specific path
items ~?@`id`
// Deduplicate by `id` field, preserve order
```

**Pros:**
- Simple syntax
- Clear intent

**Cons:**
- May not handle deep object comparisons
- Performance concerns with deep paths

#### Option 2: Custom Comparer
```javascript
// Unique with custom comparison function
items ~? {
  a: _<@0,
  b: _>@1,
  a@`id` = b@`id`
}
```

**Pros:**
- Flexible comparison
- Handles complex equality logic

**Cons:**
- More verbose
- Potential performance overhead

#### Option 3: Hash-Based Unique
```javascript
// Compute hash for deduplication
items ~@{
  item: _<,
  computeHash: item@`id`
}
```

**Pros:**
- Efficient for large collections
- Handles complex objects

**Cons:**
- Hash collision potential
- More complex syntax

**Open Issue:** Need to define `uniqueWith` semantics and syntax clearly.

## Currying Edge Cases

**Status:** TODO — Edge cases not fully specified

**Question:** How does currying behave in complex scenarios?

### Edge Case 1: Multi-Argument Currying

```javascript
// Currying of n-ary operators
{ _< + _> + _<@0 }

// Questions:
// 1. Which argument gets bound first?
// 2. How to unbind?
// 3. Interaction with partial application?
```

### Edge Case 2: Nested Directional Forms

```javascript
// Nested currying
{ _< + { _< * 2 } }

// Questions:
// 1. Evaluation order?
// 2. Type inference?
// 3. Interaction with type checking?
```

### Edge Case 3: Operator Overriding

```javascript
// Currying of overridden operators
customAdd: { _< + _> }

// Questions:
// 1. How does original operator behave?
// 2. Can original be called explicitly?
// 3. Shadowing rules?
```

**Open Issue:** Need to specify currying rules for complex scenarios.

## Async/Await Patterns

**Status:** TODO — No async model defined

**Question:** How to handle asynchronous operations in Left-Right?

**Proposed Approaches:**

#### Option 1: Promise-Based
```javascript
// Use JavaScript promises
{
  fetchData: async _<@0,
  data: await fetchData,
  process: data
}
```

**Pros:**
- Leverages existing JS async model
- Familiar to JavaScript developers

**Cons:**
- Doesn't translate well to Rust target
- Requires explicit async syntax

#### Option 2: Explicit Async Markers
```javascript
// New async marker
async {
  data: fetchAPI,
  result: await data
}
```

**Pros:**
- Clear async semantics
- Can transpile differently per target

**Cons:**
- Adds new keyword
- Requires type system changes

#### Option 3: Implicit Async
```javascript
// All operations implicitly async
{
  data: fetchAPI,
  result: data
}
// Transpiled to async/await in JS, tokio in Rust
```

**Pros:**
- Minimal syntax
- Fits loosely-typed philosophy
- Natural async handling

**Cons:**
- May confuse synchronous vs asynchronous
- Performance implications for sync operations

**Open Issue:** Need to design async model that works for both JS and Rust targets.

## Macro System Design

**Status:** TODO — No macro system defined

**Question:** Should Left-Right have explicit macros? If so, how should they work?

**Proposed Approaches:**

#### Option 1: Doc-Block Macros

```javascript
/**
 * Macro: debug
 * Usage: { debug expression
 */
// Macro expands to:
{
  value: expression,
  console: {
    log: `Debug: {value}`
  },
  return: value
}
```

**Pros:**
- Familiar to Rust developers
- Clear documentation
- Simple invocation

**Cons:**
- Limited power
- Hard to debug

#### Option 2: Expression Macros

```javascript
// Define macro
macro! forEach(items, body) {
  items ${ body }
}

// Use macro
forEach([1, 2, 3], { _< * 2 })
// Expands to: [1, 2, 3] ${ _< * 2 }
```

**Pros:**
- Powerful macro system
- Familiar to Lisp/Clojure developers

**Cons:**
- Complex to implement
- May break hygiene
- Hard to understand for newcomers

#### Option 3: AST Transform Macros

```javascript
// Macro receives AST, returns AST
macro! transform(ast) {
  // Modify AST programmatically
  ast.map_nodes(|node| {
    match node {
        Node::Number(n) => Node::Number(n * 2),
        _ => node
    }
  })
}
```

**Pros:**
- Most powerful
- Full AST manipulation
- Compile-time optimization

**Cons:**
- Very complex
- Hard to debug
- Steep learning curve

**Open Issue:** Need to decide if macros are needed, and if so, what level of power.

## Concurrency Model

**Status:** TODO — No concurrency primitives defined

**Question:** How to handle concurrent operations?

**Proposed Approaches:**

#### Option 1: Parallel Collections

```javascript
// Parallel map operation
items $p { _< * 2 }
// Process items concurrently
```

**Pros:**
- Natural extension of `$` operator
- Simple syntax

**Cons:**
- Requires runtime support
- May be non-deterministic order

#### Option 2: Promise.all Pattern

```javascript
// Explicit concurrency
{
  results: [fetch1, fetch2, fetch3] $parallel,
  process: results
}
```

**Pros:**
- Leverages existing concurrency primitives
- Predictable behavior

**Cons:**
- Verbose
- Target-specific behavior

#### Option 3: Rust Tokio Integration

```javascript
// Transpile to Rust tokio
async {
  handles: [
    task1(),
    task2(),
    task3()
  ] $join_all,
  process: handles
}
```

**Pros:**
- Native Rust performance
- Proper async/await in Rust

**Cons:**
- Different semantics for JS target
- Complex transpilation logic

**Open Issue:** Need to design concurrency model that works across both targets.

## Type Annotation Opt-In

**Status:** TODO — No type system specified

**Question:** Should Left-Right support optional type annotations?

**Proposed Approaches:**

#### Option 1: No Type System

```javascript
// Purely dynamic
{
  input: _<@0,
  output: input * 2
}
```

**Pros:**
- Simplest approach
- Fits loosely-typed philosophy
- No syntax overhead

**Cons:**
- No compile-time checking
- Runtime errors only
- Tooling limitations

#### Option 2: Optional Annotations

```javascript
// Type annotations are optional
{
  input: _<@0: number,
  output: input * 2
}
```

**Pros:**
- Documentation value
- Gradual typing support
- Can enable strict mode

**Cons:**
- Complex to implement
- May confuse users
- Maintenance burden

#### Option 3: Contract Checking

```javascript
// Runtime contracts
{
  input: _<@0,
  assert: input?.number,
  output: input * 2
}
```

**Pros:**
- Leverages existing `?` operator
- No new syntax
- Runtime validation

**Cons:**
- Runtime overhead
- No compile-time benefits
- Limited expressiveness

**Open Issue:** Need to decide on type system approach.

## Testing Framework

**Status:** TODO — No testing framework defined

**Question:** What should the Left-Right testing framework look like?

**Proposed Approaches:**

#### Option 1: Doctest Style

```javascript
/**
 * Example: 5 + 3
 * Result: 8
 */
{
  add: { a: _<@0, b: _>@1, a + b }
}
```

**Pros:**
- Documentation and tests together
- Simple syntax
- Familiar to Rust/Python developers

**Cons:**
- Limited test coverage
- Hard to test edge cases
- No test isolation

#### Option 2: Assert-Based

```javascript
// Explicit test file
{
  testName: `Add two numbers`,

  input: {
    a: 5,
    b: 3
  },

  expected: 8,

  actual: add[input.a, input.b],

  assert: actual = expected
}
```

**Pros:**
- Clear test structure
- Easy to debug
- Comprehensive testing

**Cons:**
- Verbose
- Requires test syntax
- More boilerplate

#### Option 3: Property-Based

```javascript
// Property-based testing
{
  testProperties: `Associative addition`,

  property: { a: _, b: _, c: _,
    add[a, b] + c = add[a, add[b, c]]
  },

  verify: { _<, generate: generateInputs, check: property }
}
```

**Pros:**
- Catches edge cases
- Tests properties not examples
- Powerful testing approach

**Cons:**
- Complex to understand
- Hard to implement
- May overkill for simple cases

**Open Issue:** Need to design testing framework that fits Left-Right philosophy.

## Security Model

**Status:** TODO — No security model defined

**Question:** How to handle untrusted code execution and sandboxing?

**Proposed Approaches:**

#### Option 1: Capability-Based

```javascript
// Explicit capabilities
{
  input: _<@0,
  filesystem: file[`fs`], // Capability object
  result: filesystem.readFile[input]
}
```

**Pros:**
- Explicit resource access
- Fine-grained control
- Audit trail

**Cons:**
- Complex to implement
- Requires runtime support
- May break existing code

#### Option 2: Sandbox Mode

```bash
# Run in sandbox
lr --sandbox script.lr

# Limit capabilities
lr --allow=network,deny=filesystem script.lr
```

**Pros:**
- Clear security boundaries
- CLI-level control
- Easy to understand

**Cons:**
- All-or-nothing approach
- May be too restrictive
- Hard to configure correctly

#### Option 3: No Security Model

```javascript
// Trust all code
{
  // Full access to everything
  result: file[`sensitive-data`]
}
```

**Pros:**
- Simplest approach
- No implementation complexity
- Maximum flexibility

**Cons:**
- Security risk
- Not suitable for untrusted code
- Compliance issues

**Open Issue:** Need to decide on security approach for production use.

## Migration from PenroScript to Left-Right

**Status:** TODO — Migration path not defined

**Question:** How to handle migration from PenroScript naming/semantics?

**Key Considerations:**

1. **Name Change:**
   - PenroScript → Left-Right
   - Update documentation and examples
   - Handle backward compatibility

2. **Syntax Changes:**
   - Identify breaking changes
   - Provide migration guide
   - Deprecation period

3. **Operator Semantics:**
   - Document changed operators
   - Update examples
   - Add migration warnings

4. **File Extensions:**
   - Old: `.prsc` (PenroScript)
   - New: `.lr` (Left-Right)
   - Support both during transition

**Proposed Migration Strategy:**

#### Phase 1: Deprecation

```bash
# PenroScript files still work
lr --legacy script.prsc

# Warning issued
# "PenroScript syntax is deprecated, migrate to Left-Right"
```

#### Phase 2: Automatic Migration

```bash
# Auto-migrate tool
lr --migrate script.prsc script.lr

# Generates migration report
# "Migrated 5 files with 3 warnings"
```

#### Phase 3: Final Removal

```bash
# PenroScript support removed
# Left-Right only
lr script.lr
```

**Open Issue:** Need to define detailed migration plan and timeline.

## Grammar Formalization

**Status:** TODO — No formal grammar specified

**Question:** Should Left-Right have a formal EBNF grammar?

**Proposed Approaches:**

#### Option 1: PEG Grammar

```peg
// Parsing Expression Grammar
Program <- MapBlock+
MapBlock <- "{" KeyValuePair* (Expression | "return" ":" Expression)? "}"
KeyValuePair <- Identifier ":" Expression
Expression <- Term Operator*
```

**Pros:**
- PEG parsing libraries available
- Good error reporting
- Clear specification

**Cons:**
- May not match LR(1) constraints
- Limited backtracking
- Can be complex

#### Option 2: EBNF Grammar

```ebnf
// Extended Backus-Naur Form
<program> ::= "{" { <key-value-pair> } [ <expression> ] "}"
<key-value-pair> ::= <identifier> ":" <expression>
<expression> ::= <term> { <operator> <term> }
```

**Pros:**
- Widely understood
- Standard specification
- Tooling available

**Cons:**
- Ambiguity handling
- Error recovery unclear
- May not capture all nuances

#### Option 3: Hand-Written Parser

```rust
// Recursive descent parser
fn parse_program(input: &str) -> Result<Program, ParseError> {
    let mut parser = Parser::new(input);
    parser.parse_block()
}

// Custom error handling
parser.on_error(|error| {
    ParseError::new(error)
})
```

**Pros:**
- Full control over parsing
- Best error messages
- Can handle edge cases

**Cons:**
- More complex to maintain
- May have bugs
- Hard to specify

**Open Issue:** Need to decide on grammar formalization approach.

## Additional TODOs from Brainstorm Checklist

From the 25-category specification, additional unresolved items:

### Operator Priority

```javascript
// Operator precedence in expressions
3 + 4 * 2 // Ambiguous behavior with mixed types
```

**Issue:** How to handle operator precedence for type-dependent operators?

### Logical Keys in Operators

```javascript
// Logical expressions as map keys
{
  onlyUnique: _<@2,
  result: items ${ { onlyUnique ^?: _<~, _< } }
}
```

**Issue:** What are exact semantics of logical keys as conditionals?

### Function Application Syntax

```javascript
// Multiple function call syntaxes
func[arg1, arg2]
arg1 >> func
arg2 << func
arg1 _> func
```

**Issue:** Which syntax(es) should be standard?

### Nested @ Operator

```javascript
// Deep path access
obj@[`nested`, `path`, `to`, `value`]

// Mixed with other operators
obj@[`nested`] + 1
```

**Issue:** How does `@` interact with other operators in nested contexts?

## New Open Questions

### Operator Overriding Edge Cases

**Status:** TODO — Override interaction not fully specified

**Question:** How do partial overrides interact with complex operator semantics?

#### Edge Case 1: Override vs. Partial Override

```javascript
{
  // Folder-level override
  +: { a: _, b: _, a + b * 2 },

  script: {
    // Intra-script partial override
    +: {
      _<?=`text`: a + b * 10,
      a + b * 3
    },

    result: `hello` + 5  // Which override applies?
  }
}
```

**Questions:**
- Do partial overrides merge or replace full overrides?
- How to resolve conflicting case patterns?
- What's the precedence for case pattern matching?

#### Edge Case 2: Recursive Operator Dependencies

```javascript
{
  // Override operators that reference each other
  +: { a: _, b: _, a - (-b) },
  -: { a: _, b: _, a + (-b) },

  // Circular dependency?
  result: 5 + 3
}
```

**Questions:**
- How to handle circular operator dependencies?
- Detection and error reporting for invalid overrides?
- Fallback behavior when override is invalid?

#### Edge Case 3: Operator Identity Override

```javascript
{
  // Override identity element for +
  +: { identity: 0, ... },
  // But what about array concat identity?
  // [] + [1, 2] → should be [1, 2]
  // But + identity is 0, so...?
}
```

**Questions:**
- Can override specify identity elements per type?
- How to handle type-dependent identity elements?
- Fallback to default when type not specified?

#### Edge Case 4: Cross-Module Override Conflicts

```javascript
// File: lib/math.lr
{
  +: { a: _, b: _, a * b }  // Override +
}

// File: main.lr
{
  math: file[`./lib/math.lr`],

  // Does this override apply to math module?
  +: { a: _, b: _, a + b + 100 },

  result: math.add[5, 3]  // Which behavior?
}
```

**Questions:**
- Do module-level overrides respect importing scope's overrides?
- How to explicitly use original operator?
- Can modules be isolated from overrides?

**Open Issue:** Need to specify override interaction semantics and conflict resolution.

### Identity Element Edge Cases with Maps

**Status:** TODO — Identity behavior not fully specified

**Question:** How do identity elements interact with Map types and complex nested structures?

#### Edge Case 1: Map Identity for +

```javascript
// What is the identity element for Map addition?
result: {} + { a: 1 }
// Should result be:
// Option A: { a: 1 } (empty map as identity)
// Option B: Error (no defined identity)
// Option C: Depends on override settings

// What about merging?
result1: { a: 1 } + { b: 2 }
// Should result be:
// Option A: { a: 1, b: 2 } (merge)
// Option B: Error (conflicting operation)
```

**Questions:**
- Should empty map `{}` be identity for `+`?
- What's the semantics of `+` on maps?
- Should `+` merge maps or be undefined?

#### Edge Case 2: Nested Identity with undefined

```javascript
// Identity disappears in maps
result: {
  base: 1,
  value: base + undefined  // 1
}
// But what about in nested context?

result2: {
  data: {
    base: 1,
    value: data.base + undefined  // ?
  }
}
// Should this be 1 or undefined?
```

**Questions:**
- How does `undefined` identity work in nested map evaluation?
- Is map evaluation depth-first or sequential?
- Does `undefined` propagation differ in nested contexts?

#### Edge Case 3: Array Identity with Map Keys

```javascript
// Array wraps values
result: [] + 5  // [5]

// But what about in map keys?
result2: {
  key: [] + 5  // [5]?
  // Then access:
  value: result2@[result2.key]
  // Does this work with array as key?
}
```

**Questions:**
- Can arrays be used as map keys after `+` wrapping?
- How does key hashing work with identity-wrapped arrays?
- Should identity behavior differ in key vs value contexts?

#### Edge Case 4: Type Conversion Identity

```javascript
// `` converts types
result1: 1 + ``  // `1`
result2: true + ``  // `true`

// But what about mixed operations?
result3: { a: 1 } + ``
// Should this be:
// Option A: Error (map can't be stringified)
// Option B: `{a: 1}` (no conversion)
// Option C: `` + `{a: 1}` (string conversion depends on order)

// What about empty string identity?
result4: 1 + `` + 2
// Is this: `(1 + ``) + 2` → `` + 2 → ``?
// Or: `1 + (`` + 2)` → 1 + 2 → 3?
```

**Questions:**
- Does `` identity work for all types?
- What's the precedence for identity conversion?
- Is identity conversion associative?

#### Edge Case 5: Operator Override for Map Identity

```javascript
{
  // Override + identity for maps
  +: {
    identity: {},
    map_merge: { a: _, b: _, a + b }  // Deep merge
  },

  result: { a: 1, nested: { x: 10 } } + { b: 2, nested: { y: 20 } }
  // Should result be:
  // { a: 1, b: 2, nested: { x: 10, y: 20 } }?
}
```

**Questions:**
- Can overrides define custom identity semantics?
- How to specify identity per type in override?
- Should identity be configurable or fixed by spec?

**Open Issue:** Need to define identity element semantics for complex types and nested structures.

## Design Prioritization

**High Priority:**
1. Conditional syntax — Fundamental control flow
2. Currying edge cases — Core language behavior
3. Async/await patterns — Essential for modern programming

**Medium Priority:**
4. Testing framework — Needed for language adoption
5. Type annotation opt-in — Gradual typing support
6. Security model — Production readiness

**Low Priority:**
7. Macro system — Can defer until v1.x
8. Grammar formalization — Can use informal spec initially
9. Migration strategy — Can address during beta

## Related Concepts

- **Language Specification** — Formal language definition
- **Design Space** — Exploring possible solutions
- **Trade-off Analysis** — Weighing options
- **Language Evolution** — Iterative design improvement
- **Backward Compatibility** — Preserving existing code
- **Deprecation** — Phasing out old features
- **Future Compatibility** — Preparing for extensions
