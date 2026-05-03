# Type System — Left-Right

## Overview

Left-Right uses a **loosely typed, dynamically inferred** type system with no type declarations. Operators adapt their behavior based on input types, enabling polymorphic operations without explicit type annotations.

---

## Primitive Types

### Text

Text values delimited by backticks ONLY. Single and double quotes are reserved for operator names.

**Syntax:**
```penroscript
`hello world`
`backticks only - quotes reserved for operators`
```

**Features:**
- Multi-line text supported
- Template literals with `{variable}` interpolation
- Escape sequences: `\n`, `\t`, `\`` (backtick)
- Operator-enabled when using `_</_>` in templates

**Examples:**
```penroscript
// Simple text
greeting: `Hello, World!`

// Interpolation
name: `Alice`,
message: `Hello, {name}!`

// Text as operator (becomes uppercase function)
`hello` ^  // Returns `HELLO`
```

### Number

Numeric values for arithmetic operations.

**Syntax:**
```penroscript
42
3.14
-10
```

**Features:**
- Integer and floating-point support
- Scientific notation support
- Standard arithmetic operators: `+`, `-`, `*`, `%` (divide), `%%` (modulus)

**Examples:**
```penroscript
// Basic arithmetic
sum: 10 + 20
product: 3 * 4
quotient: 100 % 4
remainder: 17 %% 5
```

### Boolean

Boolean values representing truth and falsehood.

**Syntax:**
```penroscript
true
false
```

**Features:**
- Two values: `true` and `false`
- Used in conditional logic and predicate operators
- Results of comparison operators (`<`, `<=`, `>`, `>=`, `=`)
- Logical operators: `!` (not), `&` (and), `|` (or)

**Examples:**
```penroscript
// Boolean literals
isValid: true,
isDisabled: false,

// Comparison returns boolean
isGreater: 10 > 5,     // true
isEqual: 3 = 3,       // true

// Logical operations
combined: true & false,   // false
either: true | false,     // true
negated: !true,           // false
```

### Truthy/Falsy Values

Values are truthy or falsy based on context.

**Falsy Values:**
- `undefined`
- `false` (boolean false)
- `0` (zero)
- `` `` (empty text)
- `[]` (empty list)
- `{}` (empty map)

**Truthy Values:**
- `true` (boolean true)
- Non-zero numbers
- Non-empty text
- Non-empty lists
- Non-empty maps

**Features:**
- Logical operators: `!` (not), `&` (and), `|` (or)
- Comparison operators: `<`, `<=`, `>`, `>=`, `=` (equality)
- Type-dependent behavior (operators work on all types with truthy/falsy coercion)

**Examples:**
```penroscript
// Logical operators
isValid: value > 0 & value < 100

// Truthy/falsy coercion
truthyValue: 42 ?/ `truthy`    // Returns `truthy`
falsyValue: 0 ?/ `truthy`      // Returns `falsy`

// Empty collections are falsy
hasItems: [1, 2, 3] & `has items`  // `has items`
isEmpty: [] | `is empty`             // `is empty`
```

### Undefined

Represents absence of value or error state.

**Syntax:**
```penroscript
undefined
```

**Features:**
- Default for missing keys and failed operations
- JSON `null` values convert to `undefined` by default (configurable globally to convert to `null` text string instead)
- Optional chaining returns undefined for missing paths
- Error handling uses undefined instead of exceptions

**Examples:**
```penroscript
// Missing key returns undefined
obj: { a: 1 },
missingValue: obj @b  // Returns undefined

// Failed operation defaults to undefined
result: {} @['nested', 'path']  // Returns undefined
```

---

## Collection Types

### List

Ordered, heterogeneous collections of values.

**Syntax:**
```penroscript
[1, 2, 3]
[`a`, 42, true, {nested: `value`}]
```

**Features:**
- Ordered (preserves insertion order)
- Heterogeneous (mixed types allowed)
- Zero-based indexing with `@` operator
- Rich collection operators: `${` (map), `?{` (filter), `$+` (reduce)

**Examples:**
```penroscript
// Basic list
numbers: [1, 2, 3, 4, 5]

// Heterogeneous list
mixed: [`text`, 42, true, {key: `value`}]

// Indexing
first: numbers @[0]      // 1
last: numbers @[-1]       // 5
slice: numbers @[1:3]     // [2, 3, 4]
```

### Map

Text-keyed collections of key-value pairs.

**Syntax:**
```penroscript
{ a: 1, b: 2, c: 3 }
```

**Features:**
- Text keys only
- Ordered (preserves key insertion order)
- JSON-like syntax
- Path access with `@` operator
- Equality operators: `=` (unordered) and `==` (ordered)
- Stringifies to minified JSON-like text when interpolated

**Examples:**
```penroscript
// Basic map
config: {
  host: `localhost`,
  port: 8080,
  debug: true
}

// Path access
host: config @`host`
nestedValue: data@[`user`, `profile`, `email`]

// Map operators
keys: config @<    // [`host`, `port`, `debug`]
values: config @>   // [`localhost`, 8080, true]

// Map-to-text stringification (interpolation)
data: {a:1, b:2},
result: `Values: {data}`,        // `Values: {a:1,b:2}` (minified)
nested: {data:{a:1,b:2}, `Result: {data}`}  // `Result: {a:1,b:2}`
```

---

## Operators as First-Class Type

Operators are values that can be:

- **Stored** in maps and arrays
- **Passed** as function arguments
- **Returned** from functions
- **Assigned** to variables

**Examples:**
```penroscript
// Store operator as value
{
  adder: +,
  doubler: * 2,
  upper: ^,
}

// Pass operator as argument
apply: { op: _<, value: _> },
result: apply.op[value]  // Applies the passed operator

// Return operator from function
makeTransformer: { op: _<,
  { _<, op }
},
toUpper: makeTransformer[^],  // Returns function that uppercases
result: toUpper`hello`   // `HELLO`
```

This enables **higher-order programming** and **operator composition** patterns unavailable in languages where operators are syntactic sugar.

---

## Loosely Typed

No type declarations required. Runtime type inference determines behavior.

**Benefits:**
- No ceremony (just write code)
- Flexible APIs (accept multiple types)
- Rapid prototyping
- Polymorphic operators

**Trade-offs:**
- No compile-time type checking
- Runtime type errors possible
- Requires careful operator design

**Example:**
```penroscript
// Type inferred from value
number: 42,
text: `hello`,
flag: true,

// Type inferred from operator behavior
sum: number + number,      // + operates on numbers → returns number
combined: text + text,      // + operates on text → returns text
filtered: data ?{ ... }    // $? operates on lists → returns list
```

---

## Type-Dependent Operator Behavior

The same operator changes meaning based on input types, enabling polymorphic behavior without type overloads.

### `+` Operator Behavior

| Input Type | Behavior | Example |
|------------|----------|----------|
| Number + Number | Addition | `1 + 2` → `3` |
| Text + Text | Concatenation | `` `a` + `b` `` → `` `ab` `` |
| List + List | Concatenation | `[1] + [2]` → `[1,2]` |
| Map + Map | Merge | `{a:1} + {b:2}` → `{a:1, b:2}` |

**Identity Elements for +:**
- `undefined` is identity for numbers: `1 + undefined` → `1`
- `undefined` is identity for text: `` `hello` + undefined `` → `` `hello` ``
- `undefined` is identity for maps: `{a:1} + undefined` → `{a:1}`
- `undefined` APPENDS to lists: `[1,2] + undefined` → `[1,2,undefined]`
- Non-identity from different set: value disappears
- Text concat when either side is text
- List concat/append when either side is list

### `@` Path Access Behavior

| Input Type | Behavior | Example |
|------------|----------|----------|
| Text | Property access | `obj @['name']` → `obj.name` |
| List | Index access | `arr @[0]` → `arr[0]` |
| Number | Index access (numeric path) | `arr @0` → `arr[0]` |
| List of text | Nested path access | `data @['user', 'profile', 'email']` → `data.user.profile.email` |

### `#` Size Operator Behavior

| Input Type | Behavior | Example |
|------------|----------|----------|
| Text | Character count | `` #`hello` `` → `5` |
| List | Element count | `#[1,2,3]` → `3` |
| Map | Key count | `#{a:1, b:2}` → `2` |

### `~` Unique Operator Behavior

| Input Type | Behavior | Example |
|------------|----------|----------|
| List | Remove duplicates | `~[1,2,1,3,2]` → `[1,2,3]` |
| Text | Remove duplicate characters | `` ~`aabbcc` `` → `` `abc` `` |

### Comparison Operators

| Operator | Name | Behavior | Example |
|----------|------|----------|----------|
| `=` | Loose equality | Unordered set comparison, type coercion | `0 = `0`` → `true` |
| `==` | Strict equality | Ordered comparison, strict type checking | `0 == `0`` → `false` |

**Key Differences:**
- `=` is loose equality (like JS `==`) - performs type coercion
- `==` is strict type comparison (like JS `===`) - requires same type
- `=` is unordered for maps/arrays (set comparison)
- `==` is ordered for maps/arrays (preserves key/index order)

---

## ? Type Checking Operator

The `?` operator returns the type of a value, similar to JavaScript's `typeof` operator.

**Default Output:**
`` `hello` ? `` → `text`
42 ? → `number`
true ? → `boolean`
false ? → `boolean`
undefined ? → `undefined`
[1,2,3] ? → `list`
{a:1} ? → `map`
{ _< + _> } ? → `operator`
```

**Configurability:**
The `?` operator output is configurable:
- **Global:** Default output matches JS `typeof`
- **Per-project:** Override type names for specific projects
- **Per-file:** Configure type names for individual files

**LTR Evaluation:**
`` `hello` ? = `text` `` - Full LTR: `` `hello` `` evaluates first, `?` outputs `text`, `=` compares

---

## Error Handling

Errors default to `undefined` rather than throwing exceptions.

**Design Philosophy:**
- Graceful degradation over crashing
- Optional chaining with undefined propagation
- Consistent error representation
- No try/catch required

**Error Sources:**
1. **Missing keys** in map path access
2. **Index out of bounds** in list access
3. **Type mismatches** in operator application
4. **Division by zero**
5. **Invalid operations** (e.g., `undefined #`)

**Examples:**
```penroscript
// Missing key returns undefined
obj: { a: 1 },
missing: obj @['nonexistent']  // undefined

// Out of bounds returns undefined
arr: [1, 2, 3],
missing: arr @[10]  // undefined

// Division by zero returns undefined
result: 10 % 0  // undefined

// Optional chaining handles errors gracefully
email: user@[`profile`, `email`]  // undefined if profile or email missing
```

**Comparison to Exception-Based Error Handling:**
- **Benefits:** No unexpected crashes, explicit error handling, functional purity
- **Trade-offs:** Silent failures, harder to debug, no stack traces

---

## Comparison with APL/J Type Systems

| Aspect | APL/J | Left-Right |
|---------|----------|-------------|
| **Type System** | Dynamic, rank-based | Dynamic, type-dependent |
| **Inspiration** | Array-oriented languages | Inspired by array-oriented languages |
| **Nested Lists** | Deep nesting common | JSON-like map/list mix |
| **Scalar Extension** | Automatic | Manual (implicit coercion) |
| **Empty Values** | Various (0, '', etc.) | Unified (`undefined`) |

**Key Differences:**
1. **Syntax:** Left-Right uses JSON-like structure vs. APL's special characters
2. **Evaluation:** LTR vs. RTL (APL/J)
3. **Data Structures:** Maps and lists separate vs. APL's array-only approach
4. **Operator Design:** ASCII-friendly vs. APL's Unicode operators
5. **Type Inference:** Explicit operator behavior vs. APL's rank-based dispatch

---

## Transpilation Target Interaction

### JavaScript/TypeScript

**Type Mapping:**
| Left-Right | JavaScript/TypeScript |
|-------------|---------------------|
| `undefined` | `undefined` |
| `true`/`false` | `true`/`false` |
| `` `text` `` | `'text'` |
| `42` | `42` |
| `[1,2,3]` | `[1,2,3]` |
| `{a:1}` | `{a:1}` |

**Runtime Semantics:**
- `==` (strict type checking) → `===` in JS
- `=` (loose equality, unordered) → `==` in JS
- Optional chaining (`?.` operator equivalent)
- Array spread for map merging
- Array methods for collection operations

**Example Transpilation:**
```penroscript
// Left-Right
{
  result: data
    ?{ @active = true }
    ${ @value * 2 }
    ~
}

// Generated JavaScript
const result = pipe(
  data,
  filter(item => item.active === true),
  map(item => item.value * 2),
  uniq
);
```

### Rust

**Type Mapping:**
| Left-Right | Rust |
|-------------|-------|
| `undefined` | `Option<T>` or explicit `None` |
| `true`/`false` | `bool` |
| `` `text` `` | `&str` |
| `42` | appropriate numeric type (`i32`, `f64`) |
| `[1,2,3]` | `Vec<T>` |
| `{a:1}` | `HashMap<String, T>` or `BTreeMap` |

**Runtime Semantics:**
- Strong static typing with inference
- `Result<T, E>` for error handling
- Iterators for collection operations
- Trait-based operator behavior

**Example Transpilation:**
```penroscript
// Left-Right
{
  result: data
    ?{ @active = true }
    ${ @value * 2 }
    ~
}

// Generated Rust (conceptual)
let result = data
  .iter()
  .filter(|item| item.active)
  .map(|item| item.value * 2)
  .collect::<HashSet<_>>()
  .into_iter()
  .collect();
```

---

## Type Inference Examples

### Implicit Inference

The language infers types from context:

```penroscript
// Type inferred from value
number: 42,
text: `hello`,
flag: true,

// Type inferred from operator behavior
sum: number + number,      // + operates on numbers → returns number
combined: text + text,      // + operates on text → returns text
filtered: data ?{ ... }    // $? operates on lists → returns list
```

### Collection Type Inference

Collection operators infer element types from input:

```penroscript
// Map preserves element types
{
  data: [1, 2, 3],
  doubled: data ${ _< * 2 }  // [2, 4, 6]
}

// Filter preserves type
{
  data: [1, 2, 3, 4, 5],
  evens: data ?{ _< / 2 = 0 }  // [2, 4]
}
```

---

## Best Practices

### Leverage Type-Dependent Behavior

Use polymorphic operators for cleaner code:

```penroscript
// Good: Single operator, multiple types
combine: left + right

// Avoid: Separate operators per type
combine: typeSwitch[left, right] { ... }
```

### Handle Undefined Gracefully

Design operators to handle `undefined` input:

```penroscript
// Good: Explicit undefined handling
{
  result: input ?/ defaultValue,
  output: result ?? defaultValue
}
```

### Use Type Coercion

Use implicit coercion for flexible APIs:

```penroscript
// Truthy/falsy coercion for conditional logic
{
  result: value & 'truthy',
  isActive: config @['enabled'] ?? false
}
```

---

## Related Documentation

- [Design Philosophy](./01-design-philosophy.md) — Language design principles and rationale
- [Language Overview](./00-language-overview.md) — Complete language overview
- [Operator Reference](./03-operator-reference.md) — Comprehensive operator catalog with type behavior
- [Master Index](./README.md) — Complete documentation suite
