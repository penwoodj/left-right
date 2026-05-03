# Evaluation Model — Left-to-Right Execution

Left-Right language uses a deterministic left-to-right evaluation model that eliminates operator precedence complexities. This document covers evaluation model, currying behavior, directional forms, and expression grouping.

## Top-Level Expression Rule

The top level of every Left-Right file must be a valid LeftRightExpression. This ensures that the entire file can be parsed and evaluated as a single expression.

**Valid Top-Level Expressions:**
- A single value (number, text, undefined)
- A map/object: `{ key: value, ... }`
- A list: `[ value1, value2, ... ]`
- An operator application: `value operator otherValue`
- A map with operators: `{ step1: {...}, result: step1.operator }`

**Invalid Top-Level:**
- Multiple expressions at root level (must wrap in map)
- Comments outside expressions

**Example:**
```penroscript
// Valid: single map expression
{
  greeting: `Hello, World!`,
  greeting
}

// Valid: single value
42

// Valid: operator chain
[1, 2, 3] ${ _< * 2 } #  // [6]
```

## Fundamental Principle: LTR Evaluation

### Strict Left-to-Right Order

Every operator in Left-Right evaluates left to right unless explicitly grouped with parentheses. This is the core evaluation model of the language.

```javascript
// Mathematical operations evaluate LTR
3 + 4 * 2 // Result: 14 (not 11)
// Evaluated as: (3 + 4) * 2

// Complex operations
10 - 3 + 2 // Result: 9
// Evaluated as: ((10 - 3) + 2)
```

**Key Principles:**
- No implicit operator precedence
- Sequential execution follows reading direction
- Eliminates precedence bugs from traditional languages
- Simplifies parsing and mental model

### Why No Precedence?

Traditional languages use complex precedence tables that developers must memorize. Left-Right eliminates this cognitive load:

| Traditional (C-family) | Left-Right |
|------------------------|-------------|
| `3 + 4 * 2` → 11 | `3 + 4 * 2` → 14 |
| Memorize precedence rules | Just read left to right |
| Parentheses required frequently | Parentheses only for intentional grouping |

**Benefits:**
- Simpler parsing
- Reduced cognitive load
- Matches natural reading order
- Eliminates precedence-based bugs
- Predictable evaluation at all times

## Left-Hungry Currying

### Default Behavior for Dyadic Operators

Dyadic (two-argument) operators are left-hungry by default. When one argument is statically provided, the operator becomes a monadic function waiting for the remaining argument.

```javascript
// Static value on left creates right-hungry function
{ _< + 1 } // Expects right argument
// Equivalent to: (x) => x + 1

// Static value on right creates left-hungry function
{ 5 + _> } // Expects left argument
// Equivalent to: (x) => 5 + x
```

### Currying in Practice

Curried operators can be applied to single values or collections:

```javascript
// Apply to single value
5 { _< + 1 } // Result: 6

// Apply to list (map operation)
[1, 2, 3] ${ _< + 1 } // Result: [2, 3, 4]
```

### Operator Definitions with Currying

Functions can define curried operators for partial application:

```javascript
// Define a function using curried operator
rangeFilter: { _< $<[>2, <5] $+ }
// rangeFilter is now callable with left argument
```

## Directional Forms: _< and _>

### Left Section (_<)

The left section (`_<`) binds the left argument of an operator, creating a function that expects the right argument.

```javascript
// Bind left argument
{ _< + threshold }
// Equivalent to: (x) => x + threshold

// Usage
items ${ _< + 10 } // Add 10 to each item
```

### Right Section (_>)

The right section (`_>`) binds the right argument of an operator, creating a function that expects the left argument.

```javascript
// Bind right argument
{ _> - threshold }
// Equivalent to: (x) => threshold - x

// Usage
100 { _> - 10 } // Result: 90
```

### Nesting Directional Sections

Directional sections can be composed:

```javascript
// Compose directional operators
{ _< + { _< * 2 } }
// Multiply by 2, then add

// Step-by-step:
// 1. { _< * 2 } creates function: (x) => x * 2
// 2. { _< + ... } creates function: (x) => x + (result of inner)
// 3. Overall: (x) => x + (x * 2)
```

### Reversing Currying Direction

Default left-hungry currying can be explicitly reversed:

```javascript
// Left-hungry (default)
{ _< + 1 } // Binds left, expects right

// Right-hungry (reversed)
{ _> + 1 } // Binds right, expects left
// Equivalent to: { x: _, 1 + x }
```

## Expression Grouping

### Parentheses for Explicit Grouping

Parentheses provide the only mechanism for overriding LTR evaluation order:

```javascript
// Force addition before multiplication
(3 + 4) * 2 // Result: 14
// Without parentheses: 3 + 4 * 2 = 14 anyway (same due to LTR)

// Force multiplication in middle
3 + (4 * 2) // Result: 11
// LTR default: (3 + 4) * 2 = 14
```

### Grouping in Context

Grouping is particularly important when mixing operations:

```javascript
// Complex grouping
(10 - 3) * (2 + 1) // Result: 21
// Evaluated step-by-step:
// 1. (10 - 3) = 7
// 2. (2 + 1) = 3
// 3. 7 * 3 = 21
```

## LTR Evaluation Walkthrough: ?= Chain

The `?` type check operator chained with `=` demonstrates full LTR evaluation:

**Example:**
```penroscript
`hello` ? = `text`
```

**Step-by-Step Evaluation:**
1. `` `hello` `` evaluates → `hello` (text value)
2. `?` applied to `hello` → outputs `text` (type name)
3. `=` compares `text` = `text` → `true`

**Full LTR Flow:**
```
`hello`   → evaluates to text value
  ?      → outputs type name: "text"
    =      → compares to right side
      `text` → text literal
```

**Result:** `true`

This shows that Left-Right evaluates strictly left-to-right, with each operation's result feeding into the next.

## Map Body Evaluation

### Sequential Key Evaluation

Map bodies (operators and JSON objects) evaluate keys sequentially from top to bottom. Each key can reference previously defined keys.

```javascript
// Sequential evaluation with key references
{
  base: 10,
  offset: 5,
  total: base + offset
}
// base = 10
// offset = 5
// total = 10 + 5 = 15
```

### Last Expression as Return Value

In map bodies, the last expression evaluated becomes the return value unless a `return` key explicitly specifies otherwise.

```javascript
// Implicit return (last expression)
{
  a: 1,
  b: 2,
  a + b // Returns: 3
}

// Explicit return
{
  a: 1,
  b: 2,
  return: a * b // Returns: 2
}
```

## Operator Blocks vs JSON Objects

### Distinguishing Forms

Left-Right has two distinct uses of brace syntax:

#### JSON Objects

When braces contain only `key: value` pairs, they create JSON objects:

```javascript
// JSON object (data structure)
{
  name: `Alice`,
  age: 30,
  active: true
}
```

#### Operator Blocks

When braces end with a non-key-value expression or contain `_<` or `_>` placeholders, they create operators (functions):

```javascript
// Operator block (function)
{
  param1: _<@0,
  param2: _<@1,
  param1 + param2 // Returns sum
}

// Operator block with directional form
{
  _< + 1 // Returns function
}
```

### Detection Rules: Map as Operator

A map `{}` becomes an operator (function) when EITHER:
- (a) Last expression after final `,` has no `:` assignment, OR
- (b) `{}` contains `_<` or `_>`

Otherwise `{}` evaluates at runtime as a Map (data structure).

```javascript
// Operator: last expression has no : assignment
{
  a: 1,
  b: 2,
  a + b
}

// Operator: contains _< directional marker
{
  input: _<@0,
  input + 1
}

// Map: all key-value pairs (last has : assignment)
{
  a: 1,
  b: 2,
  sum: 3
}

// Map example with operator application
[`param1content`, `param2content`] {
  param1: _<@0,
  param2: _<@1,
  param1 + param2
}
// Evaluates to: `param1contentparam2content`
```

### Detection Rules: String as Operator

An interpolated string becomes an operator when any `{ }` contains `_<` or `_>`. Otherwise the string evaluates as Text.

```javascript
// Text: no _< or _> in interpolation
`foo {var_in_scope}`
// Executes at runtime, produces text value

// Operator: contains _< in interpolation
`foo {_< + `bar`}`
// Returns unexecuted operator

// Operator application example
[`param1content`, `param2content`] {
  param1: _<@0,
  param2: _<@1,
  `Result: {param1 + param2}`
}
// Evaluates to: `Result: param1contentparam2content`
```

## Comparison with Traditional Evaluation

### Traditional Precedence-Based Evaluation

Most languages (C, Java, JavaScript, Python) use complex precedence tables:

| Precedence Level | Operators |
|-----------------|------------|
| 1 (highest) | `()`, `[]`, `.` |
| 2 | `!`, `~`, unary `+`, unary `-` |
| 3 | `*`, `/`, `%` |
| 4 | `+`, `-` |
| 5 | `<<`, `>>` |
| 6 | `<`, `<=`, `>`, `>=` |
| 7 | `==`, `!=`, `===`, `!==` |
| 8 | `&` |
| 9 | `^` |
| 10 | `|` |
| 11 (lowest) | `&&`, `||` |

### Flat Precedence in Left-Right

Left-Right uses flat precedence — all operators have equal precedence:

| Evaluation | Traditional (JavaScript) | Left-Right |
|-----------|------------------------|-------------|
| `3 + 4 * 2` | `3 + (4 * 2) = 11` | `(3 + 4) * 2 = 14` |
| `10 - 3 + 2` | `(10 - 3) + 2 = 9` | `((10 - 3) + 2) = 9` |
| `5 * 2 - 1` | `(5 * 2) - 1 = 9` | `((5 * 2) - 1) = 9` |

**Key Difference:** Left-Right evaluates exactly as written, left to right. No mental translation required.

## Relationship to Concatenative Languages

### Stack-Based Evaluation

Left-Right shares concepts with concatenative languages like Forth, Factor, and Joy:

**Concatenative Pattern:**
```forth
\ Forth-style stack evaluation
10 3 + 2 *  \ Push 10, push 3, add, push 2, multiply
```

**Left-Right Equivalent:**
```javascript
10 + 3 * 2 \ LTR evaluation produces same result
```

### Similarities

1. **Implicit Parameter Passing**
   - Concatenative: Values on stack consumed by operations
   - Left-Right: Values flow through operators

2. **Composition by Concatenation**
   - Concatenative: Operations written sequentially
   - Left-Right: Operators chained left to right

3. **No Named Parameters in Pipelines**
   - Concatenative: Stack manipulation only
   - Left-Right: Directional sections for implicit binding

### Key Differences

| Feature | Concatenative | Left-Right |
|----------|---------------|-------------|
| Stack | Explicit stack data structure | Implicit data flow |
| Scope | Limited scoping | Map-based named scope |
| Data Structures | Stack lists | Rich type system (Map, List, etc.) |
| Composition | Stack shuffling | Operator chaining |

### First-Class Operators

Both paradigms treat operators as first-class values:

```javascript
// Left-Right: Operators as values
addOne: { _< + 1 } // Stored function

// Concatenative: Words as values
: add-one 1 + ;
```

## Operator Syntax, Parsing, and Semantics

This section synthesizes how operators are defined syntactically, parsed by the compiler, and their execution semantics.

### Operator Syntax: Definition Rules

An operator in Left-Right is **always a Map** with specific structural characteristics that distinguish it from ordinary data maps:

```
[left-right]
// Operator syntax patterns
{ _<, expression }         // Left-hungry unary operator
{ expression, _> }          // Right-hungry unary operator  
{ _<, expression, _> }       // Binary operator (both arguments)
{ expression }                // Map that IS an operator (single element, no : key)
{ step1: {...}, result: step1.transform }  // Multi-step operator definition
```

**Operator Definition Criteria:**
1. **Map structure** — All operators are Maps at their core
2. **Directional placeholder** — Must contain `_<` and/or `_>` to be diatic
3. **Expression body** — Last or only element must be an expression (not a `:`-keyed assignment)
4. **No `:` in final element** — If the last element has no `:` key, the entire Map is an operator

```
[left-right]
// This is a map (NOT an operator) — final element has : key
{ a: 1, b: 2, result: a + b }

// This IS an operator — final element is expression (no : key)
{ a: 1, b: 2, a + b }  // When applied, a+b evaluates with context
```

### Operator Parsing: How the Compiler Recognizes Operators

The Left-Right parser identifies operators through **structural analysis** of Maps:

**Parsing Algorithm:**
1. **Read as Map** — Initially parse any `{ ... }` as a Map
2. **Check final element** — Does the last element have a `:` key separator?
   - **YES** → This is a data Map (stores values)
   - **NO** → This is an operator (defines behavior)
3. **Detect directional placeholders** — Scan for `_<` and `_>` in the Map:
   - **Found `_<`** → Left-hungry operator (curries left argument)
   - **Found `_>`** → Right-hungry operator (curries right argument)
   - **Both found** → Binary operator (requires both sides)
4. **Build operator signature** — Determine arity and argument binding from placeholders

```
[left-right]
// Parse: Map → Check final element → No : → Contains _< → Left-hungry operator
double: { _<, value * 2 }
// Parsed as: Unary operator expecting left argument

// Parse: Map → Check final element → No : → Contains _> → Right-hungry operator  
halve: { _>, value / 2 }
// Parsed as: Unary operator expecting right argument

// Parse: Map → Check final element → No : → Contains _< and _> → Binary operator
add: { _<, left + right, _> }
// Parsed as: Binary operator expecting left and right arguments
```

### Operator Semantics: Type-Dependent Execution

Once parsed and applied, operators execute with **type-dependent semantics**:

**Semantics Rules:**
1. **Identity elements disappear** — `0` for addition, `` (empty) for concatenation, `[]` (empty list) wraps
2. **Type coercion** — Operands coerce to compatible types based on operator rules
3. **Dynamic dispatch** — Same operator symbol uses different implementations per type combination
4. **Truthy/falsy logic** — Boolean type exists; non-Boolean values evaluate as truthy or falsy contextually

```
[left-right]
// + operator semantics table (full hierarchy in 13-operator-overloading-hierarchy.md)
Text + Text    → Concatenation (`Hello ` + ` World` → `Hello World`)
Number + Number → Addition (`3 + 4` → `7`)
List + List    → Concatenation (`[1,2] + [3,4]` → `[1,2,3,4]`)

// Identity behavior
1 + undefined → 1          // undefined disappears (0 identity for +)
`text` + `` → `text`         // empty text disappears (identity for concatenation)
undefined + [] → []            // undefined disappears, list wraps
```

**Operator Application Sequence:**
```
1. **Parse operator definition** (structural analysis of Map)
2. **Bind directional arguments** (apply left-hungry to left value, right-hungry to right)
3. **Type-identify operands** (runtime type check)
4. **Select semantic implementation** (dispatch to appropriate behavior)
5. **Execute** (compute result)
6. **Return** (result feeds into next LTR operation)
```

### Common Parsing Patterns

**Whitespace Insensitivity:**
```
[left-right]
// All equivalent — whitespace around operators is optional
value operator value
value operator value
value operator    value
```

**Implicit vs Explicit Currying:**
```
[left-right]
// Implicit: binary form expects both sides
{ _<, left + right, _> }

// Explicit: can call with single side for partial application
increment: { _<, + 1 }
5 increment  // Applies left=5, right=1 bound

// Right-hungry partial application
decrement: { _>, - 1 }
decrement 10  // Applies left=undefined, right=10
```

## Design Principles Summary

1. **Deterministic LTR Evaluation** — Same order, same result, always
2. **No Implicit Precedence** — Explicit grouping only
3. **Left-Hungry Currying** — Dyadic operators curry by default
4. **Directional Forms** — `_</_>` control argument binding
5. **Sequential Map Evaluation** — Keys evaluate top to bottom
6. **Clear Block Distinction** — Operators vs JSON objects

## Related Concepts

- **Operator Precedence** — Traditional evaluation order
- **Flat Precedence** — All operators equal
- **Currying** — Multi-arg to single-arg chains
- **Partial Application** — Fixing some arguments
- **Directional Sections** — Binding arguments (Haskell-style)
- **Concatenative Languages** — Stack-based composition
- **Point-Free Programming** — No intermediate variable names
- **Determinism** — Reproducible execution
