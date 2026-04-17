# Operator Overloading Hierarchy — Left-Right

## Overview

Left-Right implements a sophisticated type-dependent operator overloading system. Operators adapt their behavior based on input types, enabling polymorphic operations without explicit type overloads. This document comprehensively maps operator behavior across all type combinations, identity elements, coercion hierarchies, and the operator overriding system.

---

## Key Principles

### 1. Type-Dependent Dispatch

Every operator in Left-Right is polymorphic. The same operator changes meaning based on input types:

```penroscript
// + operator demonstrates type-dependent behavior
{
  // Numbers: arithmetic addition
  numeric: 1 + 2,              // 3

  // Text: concatenation
  text: `hello` + `world`,     // `helloworld`

  // Lists: concatenation
  lists: [1, 2] + [3, 4],    // [1, 2, 3, 4]

  // Maps: merge
  maps: {a: 1} + {b: 2}      // {a: 1, b: 2}
}
```

### 2. Identity Element Behavior

Identity elements disappear when combined with non-identity values:

```penroscript
{
  // Numbers: undefined is identity
  n1: 1 + undefined,           // 1
  n2: undefined + 5,           // 5

  // Text: empty backtick string is identity (converts types)
  s1: `hello` + ``,           // `hello`
  s2: 1 + ``,                 // `1` (number coerced to text)

  // Lists: [] is identity
  a1: [] + 0,                 // [0]
  a2: [1, 2] + [],           // [1, 2]

  // Maps: {} is identity
  m1: {} + {a: 1},           // {a: 1}
  m2: {b: 2} + {}            // {b: 2}
}
```

### 3. Comparison to JavaScript

Left-Right's type coercion improves on JavaScript's surprising behavior:

| Operation | JavaScript | Left-Right | Why |
|-----------|------------|-------------|-----|
| `1 + undefined` | `NaN` | `1` | Identity disappears |
| `` `5` `` + 3 | `` `53` `` | `` `53` `` | Text wins (both) |
| `[] + []` | `` `` `` (empty text!) | `[]` (list concat) | Proper list semantics |
| `[] + 0` | `` `0` `` (coerced to text) | `[0]` (list wrap) | List type preserved |

---

## 1. Identity Elements

### Definition

An identity element `e` for operator `op` satisfies: `x op e = x` and `e op x = x` for all `x` in the domain.

### Identity Elements by Type

| Type | Primary Identity | Secondary Identity | Behavior |
|------|-----------------|-------------------|----------|
| **Number** | `0` | `undefined` | `n + 0 = n`, `n + undefined = n` |
| **Text** | `` (empty backtick string) | None | `s + `` = s`, `` + n = `` + n (coerces) |
| **List** | `[]` | `undefined` | `[a] + [] = [a]`, `[a] + undefined = [a]` |
| **Map** | `{}` | `undefined` | `{k:v} + {} = {k:v}`, `{k:v} + undefined = {k:v}` |

### Identity Disappearing Rules

When identity is added to non-identity, returns non-identity:

```penroscript
// Number identity behavior
{
  n1: 42 + 0,              // 42 (0 disappears)
  n2: 42 + undefined,       // 42 (undefined disappears)
  n3: undefined + 42,        // 42 (order doesn't matter)

  // Exception: both undefined
  n4: undefined + undefined,  // undefined (both identities)
}

// Text identity behavior
{
  s1: `hello` + ``,         // `hello` (`` disappears)
  s2: `` + `world`,         // `world` (order doesn't matter)
  s3: 1 + ``,               // `1` (number coerced, then `` disappears)
  s4: `` + 1,               // `1` (`` + 1 = `1`, no disappear)
}

// List identity behavior
{
  a1: [1, 2, 3] + [],      // [1, 2, 3] ([] disappears)
  a2: [] + [4, 5],         // [4, 5] (order preserved)
  a3: [] + undefined,        // [] (undefined disappears in list context)

  // List + non-list wraps non-list
  a4: [1] + 2,            // [1, 2] (2 wrapped, not [])
  a5: [] + 0,              // [0] (0 wrapped into list)
}

// Map identity behavior
{
  m1: {a: 1} + {},         // {a: 1} ({} disappears)
  m2: {} + {b: 2},         // {b: 2} (order preserved)
  m3: {a: 1} + undefined,  // {a: 1} (undefined disappears)
}
```

---

## 2. Cross-Type `+` Operator Behavior (Full Matrix)

### Complete Type Combination Matrix

This matrix documents EVERY combination of the `+` operator across all Left-Right types:

| Left Type | Right Type | Result Type | Behavior | Example |
|-----------|------------|-------------|----------|----------|
| **Number** | **Number** | Number | Arithmetic addition | `1 + 2` → `3` |
| **Number** | **Text** | Text | Number coerced to text, concat | `1 + `a`` → `` `1a` `` |
| **Number** | **List** | List | Number wrapped, concat | `1 + [2]` → `[1, 2]` |
| **Number** | **Map** | Map | Number added as key '0'? **UNDEFINED** | — |
| **Number** | **undefined** | Number | undefined disappears | `1 + undefined` → `1` |
| **Text** | **Number** | Text | Number coerced to text, concat | `` `a` `` + 1 → `` `a1` `` |
| **Text** | **Text** | Text | Concatenation | `` `a` `` + `` `b` `` → `` `ab` `` |
| **Text** | **List** | List | Text wrapped, concat | `` `a` `` + [1] → `[`a`, 1]` |
| **Text** | **Map** | Map | Text added as key? **UNDEFINED** | — |
| **Text** | **undefined** | Text | undefined disappears | `` `hello` `` + undefined → `` `hello` `` |
| **List** | **Number** | List | Number wrapped, concat | `[1] + 2` → `[1, 2]` |
| **List** | **Text** | List | Text wrapped, concat | `[1] + `a`` → `[1, `a`]` |
| **List** | **List** | List | Concatenation | `[1] + [2]` → `[1, 2]` |
| **List** | **Map** | **UNDEFINED** | Invalid combination | — |
| **List** | **undefined** | List | undefined disappears | `[1, 2] + undefined` → `[1, 2]` |
| **Map** | **Number** | **UNDEFINED** | Invalid combination | — |
| **Map** | **Text** | **UNDEFINED** | Invalid combination | — |
| **Map** | **List** | **UNDEFINED** | Invalid combination | — |
| **Map** | **Map** | Map | Merge (right overrides on collision) | `{a:1} + {b:2}` → `{a:1,b:2}` |
| **Map** | **undefined** | Map | undefined disappears | `{a:1} + undefined` → `{a:1}` |
| **undefined** | **Number** | Number | undefined disappears | `undefined + 1` → `1` |
| **undefined** | **Text** | Text | undefined disappears | `undefined + `a`` → `` `a` `` |
| **undefined** | **List** | List | undefined disappears | `undefined + [1]` → `[1]` |
| **undefined** | **Map** | Map | undefined disappears | `undefined + {a:1}` → `{a:1}` |
| **undefined** | **undefined** | undefined | Both remain | `undefined + undefined` → `undefined` |

### List + Non-List Behavior

When adding a list to a non-list value:

```penroscript
{
  // Number is wrapped
  a1: [] + 0,              // [0]

  // Text is wrapped
  a2: [] + `hello`,        // [`hello`]

  // Undefined disappears
  a4: [] + undefined,       // []

  // List + number with existing elements
  a5: [1, 2] + 3,         // [1, 2, 3]

  // Non-list + list also wraps left
  a6: 0 + [1, 2],         // [0, 1, 2]
}
```

### Map Merge Semantics

When adding two maps, the right-side map's values override on key collision:

```penroscript
{
  // Simple merge
  m1: {a: 1, b: 2} + {c: 3, d: 4},
  // Result: {a: 1, b: 2, c: 3, d: 4}

  // Collision: right side wins
  m2: {a: 1, b: 2} + {b: 99, c: 3},
  // Result: {a: 1, b: 99, c: 3}  // b:2 overridden by b:99

  // Chained merges (last wins)
  m3: {a: 1} + {a: 2} + {a: 3},
  // Result: {a: 3}
}
```

### Comparison with JavaScript `+` Operator

| Left-Right | JavaScript | Difference |
|-----------|------------|-------------|
| `[1, 2] + [3, 4]` → `[1, 2, 3, 4]` | `[1, 2] + [3, 4]` → `'1,23,4'` | List concat vs. text coercion |
| `{} + {a: 1}` → `{a: 1}` | `{} + {a: 1}` → `'[object Object][object Object]'` | Map merge vs. string coercion |
| `1 + undefined` → `1` | `1 + undefined` → `NaN` | Identity disappear vs. NaN |
| `[] + 0` → `[0]` | `[] + 0` → `'0'` | List wrap vs. text coercion |

---

## 3. Type Coercion Hierarchy (for `+` Operator)

### Resolution Order

When types differ, the `+` operator follows this deterministic coercion hierarchy:

  1. **List** — If either side is List → List result
  2. **Text** — If either side is Text → Text result
3. **Map** — If either side is Map → Map result (only Map+Map valid)
4. **Number** — Otherwise → Number result
5. **Exception**: `undefined` disappears unless both sides are `undefined`

### Visual Coercion Tree

```
+ Operator Type Resolution
├── Either List?
│   └── YES → List (wrap non-list, concat)
│   └── NO
│       └── Either Text?
│           └── YES → Text (coerce numbers, concat)
│           └── NO
│               └── Either Map?
│                   └── YES → Map (only if both are Maps)
│                   └── NO
│                       └── Number
└── Special case: undefined disappears unless both sides undefined
```

### Coercion Examples

```penroscript
// List dominates
{
  // Number + List → List
  c1: 1 + [2],              // [1, 2]

  // Text + List → List
  c2: `a` + [1],           // [`a`, 1]

  // List + Anything → List
  c4: [1] + 2,             // [1, 2]
  c5: [1] + `a`,           // [1, `a`]
}

// Text dominates (when no List)
{
  // Number + Text → Text
  s1: 1 + `a`,             // `1a`

  // Text + Anything (no List) → Text
  s3: `hello` + 1,         // `hello1`
}

// Map only works with Map
{
  // Map + Map → Map
  m1: {a: 1} + {b: 2},    // {a: 1, b: 2}

  // Map + non-Map → UNDEFINED
  m2: {a: 1} + 1,         // UNDEFINED (invalid)
  m3: {a: 1} + `a`,        // UNDEFINED (invalid)
}
```

### Why This Hierarchy?

The hierarchy prioritizes **structure preservation**:

 1. **List first**: Lists are structural containers — preserve them
  2. **Text second**: Texts are sequential data — preserve them
3. **Map third**: Maps are key-value structures — preserve them
4. **Number last**: Numbers are primitive scalars — coerce to last

This order matches user intuition: "I have a list, I want to add to the list" not "I have a list, I want to convert it to a string."

---

## 4. Other Operators' Type Behavior

### Comparison Operators

#### `=` — Unordered Equality (Loose, Type-Coercing)

```penroscript
{
  // Primitive equality with type coercion
  e1: 1 = 1,              // true
  e2: 1 = '1',            // true (number coerced to string)
  e3: '5' = 5,            // true (string coerced to number)

  // Truthy/falsy coercion
  e4: true = 1,            // true
  e5: false = 0,           // true

  // Collection equality (unordered)
  e6: [1, 2, 3] = [3, 2, 1],  // true (order doesn't matter)
  e7: {a:1, b:2} = {b:2, a:1},  // true (key order doesn't matter)

  // Nested collections
  e8: [[1, 2], [3]] = [[3], [2, 1]],  // true
  e9: {a: {x:1}} = {a: {x:1}},       // true

  // undefined behavior
  e10: undefined = undefined,  // true
  e11: null = undefined,       // true (null coerces to undefined)
}
```

**Comparison to JavaScript `==`**:
- Left-Right `=` treats collections as unordered
- JavaScript `==` doesn't deep-compare arrays/maps at all
- Left-Right coerces undefined/null together

#### `==` — Ordered Equality (Strict Type)

```penroscript
{
  // Strict type equality
  e1: 1 == 1,             // true
  e2: 1 == '1',           // false (different types!)
  e3: 0 == false,          // false (different types!)

  // Ordered collection equality
  e4: [1, 2, 3] == [1, 2, 3],     // true (same order)
  e5: [1, 2, 3] == [3, 2, 1],     // false (different order!)

  e6: {a:1, b:2} == {a:1, b:2},   // true (same key order)
  e7: {a:1, b:2} == {b:2, a:1},   // false (different key order!)

  // Deep comparisons
  e8: [[1, 2], [3]] == [[1, 2], [3]],    // true
  e9: {a: {x:1}} == {a: {x:1}},        // true
}
```

**Comparison to JavaScript `===`**:
- Left-Right `==` performs deep equality
- JavaScript `===` is shallow (objects/arrays compare by reference)
- Left-Right `==` cares about collection order

### Arithmetic Operators

#### `-` — Subtract (Numbers Only)

```penroscript
{
  // Number subtraction
  s1: 10 - 3,             // 7
  s2: 5 - 8,              // -3

  // List - List (remove elements)
  s3: [1, 2, 3, 4] - [2, 4],     // [1, 3]
  s4: [`a`, `b`, `c`] - [`b`],      // [`a`, `c`]

  // Invalid types → undefined
  s5: `hello` - `world`,    // undefined
  s6: {a:1} - {b:2},       // undefined
  s7: 10 - `a`,             // undefined
}
```

#### `*` — Multiply/Repeat

```penroscript
{
  // Number * Number
  m1: 3 * 4,               // 12
  m2: 2.5 * 2,             // 5

  // Text * Number (repeat text)
  m3: `ab` * 3,            // `ababab`
  m4: `x` * 5,             // `xxxxx`

  // List * Number (repeat list)
  m5: [1, 2] * 3,          // [1, 2, 1, 2, 1, 2]
  m6: [`a`, `b`] * 2,      // [`a`, `b`, `a`, `b`]

  // Invalid combinations → undefined
  m7: `a` * `b`,           // undefined
  m8: [1] * [2],           // undefined
}
```

#### `%` — Divide

```penroscript
{
  d1: 10 % 2,              // 5
  d2: 100 % 4,            // 25
  d3: 1 % 3,              // 0.333...

  // Division by zero
  d4: 10 % 0,             // undefined (graceful degradation)

  // Invalid types → undefined
  d5: 'hello' % 2,        // undefined
  d6: [1] % 2,           // undefined
}
```

#### `%%` — Modulus

```penroscript
{
  mod1: 10 %% 3,           // 1
  mod2: 17 %% 5,           // 2
  mod3: 100 %% 7,          // 2

  // Division by zero
  mod4: 10 %% 0,          // undefined

  // Invalid types → undefined
  mod5: 'a' %% 2,         // undefined
}
```

### Collection Operators

#### `#` — Count (Works on All Collections)

```penroscript
{
  // Text length
  c1: #`hello`,            // 5
  c2: #``,                 // 0

  // List length
  c3: #[1, 2, 3, 4],     // 4
  c4: #[],                // 0

  // Map key count
  c5: #{a:1, b:2, c:3},  // 3
  c6: #{},                     // 0

  // Nested collections (counts top-level only)
  c7: #[[1, 2], [3, 4]],    // 2 (not 4)
  c8: #{a: {x:1, y:2}},     // 1 (not 2)

  // Invalid types → undefined
  c9: #42,                  // undefined
  c10: #true,                // undefined
}
```

#### `@` — Path Access (Maps and Lists)

```penroscript
{
  data: {
    user: {
      name: `Alice`,
      emails: [`alice@work.com`, `alice@home.com`]
    },
    items: [10, 20, 30, 40, 50]
  },

  // Map path access
  name: data @['user', 'name'],           // `Alice`
  email: data @['user', 'emails', 0],     // `alice@work.com`

  // List indexing
  first: data @['items', 0],            // 10
  last: data @['items', -1],            // 50
  slice: data @['items', 1:3],         // [20, 30]

  // Missing keys → undefined
  missing: data @['user', 'age'],      // undefined
  outOfBounds: data @['items', 100],    // undefined

  // Invalid operations → undefined
  invalid: 42 @['key'],                // undefined
}
```

#### `~` — Unique (Lists and Text)

```penroscript
{
  // List unique
  u1: ~[1, 2, 1, 3, 2, 4],          // [1, 2, 3, 4]
  u2: ~[`a`, `b`, `a`, `c`, `b`],      // [`a`, `b`, `c`]

  // Text unique (remove duplicate characters)
  u3: ~`aabbccdd`,                      // `abcd`
  u4: ~`hello world`,                    // `helo wrd`

  // Empty inputs
  u5: ~[],                             // []
  u6: ~``,                              // ``

  // Nested lists (flattens first, then unique)
  u7: ~[[1, 2], [2, 3], [1, 3]],      // [1, 2, 3]

  // Invalid types → undefined
  u8: ~42,                               // undefined
  u9: ~{a:1},                           // undefined
}
```

#### `><` — Join (Lists and Text)

```penroscript
{
  // List join with separator
  j1: [`hello`, `world`, `from`, `left-right`] >< ' ',
  // `hello world from left-right`

  j2: [`a`, `b`, `c`] > `` ,          // `abc` (empty separator)
  j3: [1, 2, 3] > ` `-`,              // `1-2-3`

  // Text concatenation
  j4: `hello` > ` `  > `world`,      // `hello world`

  // Nested lists
  j5: [[1, 2], [3, 4]] > ` `-`,       // `1,2-3,4` (lists stringified first)

  // Empty list
  j6: [] > ` `,                      // ``

  // Invalid types → undefined
  j7: 42 > ` -,                      // undefined
  j8: {a:1} > ` ,,                  // undefined
}
```

### List Operators

#### `?|` — Some/Any

Returns `true` if ANY element satisfies predicate:

```penroscript
{
  numbers: [1, 2, 3, 4, 5],

  // Has even?
  hasEven: numbers ?| { _< / 2 = 0 },
  // true (2 and 4 are even)

  // Has negative?
  hasNegative: numbers ?| { _< < 0 },
  // false (no negatives)

  // Empty array
  emptyHas: [] ?| { true },
  // false (empty arrays are always false)

  // With strings
  words: ['hello', 'world', 'test'],
  hasLong: words ?| { #_< > 4 },
  // true ('hello' and 'world' are length 5)

  // Condition reversal required
  // To find if "every element is <= 0", need to check if "any element is NOT > 0"
  allNonPositive: numbers ?| { !(_< > 0) },
  // false (some are > 0)
}
```

#### `?|!` — Every

Returns `true` if ALL elements satisfy predicate. **No condition reversal needed!**

```penroscript
{
  numbers: [1, 2, 3, 4, 5],

  // Every element <= 5?
  allLe5: numbers ?|! { _< <= 5 },
  // true (all 1,2,3,4,5 are <= 5)

  // Every element is positive?
  allPositive: numbers ?|! { _< > 0 },
  // true (all are positive)

  // Every element is even? (NOT true)
  allEven: numbers ?|! { _< / 2 = 0 },
  // false (1, 3, 5 are not even)

  // Empty array (vacuously true)
  emptyAll: [] ?|! { true },
  // true (empty arrays satisfy every condition vacuously)

  // Comparison to ?| (requires reversal)
  // ?|! is cleaner:
  allNonPositive: numbers ?|! { _< <= 0 },
  // Equivalent to:
  // anyPositive: numbers ?| { _< > 0 }, then allNonPositive: !anyPositive
}
```

**Key Difference from `?|`**:

| Operator | Meaning | Example |
|----------|----------|----------|
| `?|` | "Is there ANY element where X is true?" | `[1,2,3] ?| { _< = 2 }` → `true` |
| `?|!` | "Is EVERY element where X is true?" | `[1,2,3] ?|! { _< < 4 }` → `true` |

**Comparison to JavaScript**:

```javascript
// JavaScript (Array.some)
[1, 2, 3].some(x => x > 2);  // true

// Left-Right (?|)
[1, 2, 3] ?| { _< > 2 };    // true

// JavaScript (Array.every)
[1, 2, 3].every(x => x < 4);  // true

// Left-Right (?|!)
[1, 2, 3] ?|! { _< < 4 };    // true

// The ! suffix means "universal quantifier" (no reversal needed)
```

---

## 5. `!?` Operator Deep Dive

### Overview

The `!?` operator serves two purposes:
1. **Type checking**: Returns the type of a value
2. **Conditional**: Ternary-like behavior with equality check

### Default Type Checking

```penroscript
{
  // Check types of values
  t1: `hello` !?,           // 'text' (default JS-style typeof)
  t2: 42 !?,               // 'number'
  t3: true !?,              // (truthy value, no Boolean type)
  t4: undefined !?,          // 'undefined'
  t5: [1, 2] !?,           // 'list'
  t6: {a: 1} !?,          // 'map'

  // Nested values
  nested: { x: { y: `test` } },
  typeOfY: nested @['x', 'y'] !?,
  // 'text'
}
```

### Type Check with Equality

```penroscript
{
  // Check if value equals specific type
  // Format: value !?= expectedType

  isText1: `hello` !?= 'text',      // true
  isText2: 42 !?= 'text',           // false

  isNumber: 42 !?= 'number',            // true

  isList: [1, 2] !?= 'list',          // true
  isNotList: 42 !?= 'list',          // false

  isMap: {a: 1} !?= 'map',            // true

  isUndefined: undefined !?= 'undefined', // true
}
```

**Note**: The equality check uses the `=` operator (unordered, loose equality), so:
```penroscript
{
  // These are equivalent
  check1: `hello` !?= 'text',
  check2: `hello` !? = 'text',

  // Both evaluate as: `hello` !? → 'text', then 'text' = 'text' → true
}
```

### Evaluation Order Walkthrough

Let's trace `` `hello` !?= 'text' `` step-by-step:

```
1. Evaluate left operand: `hello`
    → Value: `hello`

2. Apply !? to `hello`
    → Operation: Get type of `hello`
    → Result: 'text'

3. Pull right operand: 'text'
    → Value: 'text'

4. Apply = (unordered equality)
    → Compare 'text' = 'text'
    → Result: true

Final result: true
```

### Complex Expression Walkthrough

```penroscript
{
  value: 42,

  // Complex nested expression
  result: (value + 1) !? > 40,

  // Evaluation order:
  // 1. (value + 1) → 43
  // 2. 43 !? → 'number'
  // 3. Pull '40'
  // 4. 'number' > 40 → ???

  // This is likely a type error - comparing 'number' text to number
  // Better to check type first, then compare:
  result2: (value !?= 'number') & (value > 40),
  // true AND true → true
}
```

### Configurable Type Names

The `!?` operator is **configurable** at multiple levels:

```penroscript
// Global/project/file-level configuration
{
  // Configuration example (pseudo-syntax)
  config: {
    typeNames: {
      'text': 'str',
      'number': 'num',
      'list': 'seq',
      'map': 'hash',
    }
  },

  // After configuration
  nameType: `hello` !?,          // 'str' (instead of 'text')
  ageType: 42 !?,                // 'num' (instead of 'number')

  // Equality checks adapt to configured names
  isText: `hello` !?= 'text',    // true
  isNum: 42 !?= 'num',          // true
  isTextOld: `hello` !?= 'text',  // false (name changed)
}
```

### Configuration Priority

1. **Intra-script** — Highest priority, local to file
2. **Folder** — Applies to all files in directory
3. **Project** — Project-wide configuration
4. **Global machine** — System-wide defaults

```penroscript
// Example priority
// Global: 'string', 'number'

// Folder: 'text', 'num'

// Project: 'str', 'n'

// File (intra-script): 'STRING', 'NUMBER'

{
  // This file uses 'STRING'
  myType: 'hello' !?,           // 'STRING'

  // Other files in folder use 'text'
  // Other projects use 'str'
  // Global uses 'string'
}
```

### Comparison to JavaScript `typeof`

| Left-Right | JavaScript | Difference |
|-----------|------------|-------------|
| `[] !?` → `'list'` | `typeof []` → `'object'` | Better type discrimination |
| `{a:1} !?` → `'map'` | `typeof {a:1}` → `'object'` | Maps distinguished |
| `undefined !?` → `'undefined'` | `typeof undefined` → `'undefined'` | Same |
| `null !?` → `'undefined'` | `typeof null` → `'object'` | null coerces to undefined |
| `` `hello` `` !? → `'text'` | `typeof 'hello'` → `'string'` | Text instead of String |
| `42 !?` → `'number'` | `typeof 42` → `'number'` | Same |

### Practical Use Cases

```penroscript
// 1. Type guards in data processing
{
  data: [1, `hello`, 42, `world`, true],

  // Process only text
  text: data $? { _< !?= 'text' },
  // [`hello`, `world`]

  // Process only numbers
  numbers: data $? { _< !?= 'number' },
  // [1, 42]
}

// 2. Type checking before operations
{
  value: getUserInput(),  // Could be anything

  // Safe addition (only if number)
  safeAdd: value !?= 'number' ? value + 10 : undefined,
  // If value is `hello`, returns undefined instead of error
}

// 3. Runtime type validation
{
  validateUser: { user: _<,
    // Check required fields
    hasName: user @['name'] !?= 'text',
    hasAge: user @['age'] !?= 'number',

    // Return validation result
    result: hasName & hasAge
  }
}
```

---

## 6. Operator Overriding System

### Overview

Left-Right allows **full operator overriding** at multiple scope levels. Override behavior is top-down dominant in map/object/hashmap contexts.

### Override Levels

```
Override Priority (highest → lowest):
1. Intra-script (local to file)
2. Folder (directory-level)
3. Project (project-wide)
4. Global machine (system-wide)
```

### Basic Override Syntax

```penroscript
{
  // Override + operator for this scope
  +: { _<, _> },
    customLogic: 'Custom addition behavior',
    result: _< + _>  // Original behavior preserved or modified
  }
}
```

### Partial Overrides with Type Checks

You can override operators for specific type combinations using `!?` type check in override pattern:

```penroscript
{
  // Example from specification
  +: {
    // When both sides are text type
    _<!?=`text`|_>!?=`text`:
      `adding text fun`,

    // Default behavior for all other types
    _<_>+_>
  },

  // Usage
  result1: `hello` + `world`,
  // `adding text fun` (custom override)

  result2: 1 + 2,
  // 3 (default behavior, not overridden)
}
```

### Control Flow in Overrides

The override syntax uses a control-flow-like pattern:

```penroscript
{
  +: {
    // Pattern: condition: result

    // Condition 1: both are text
    _<!?=`text`|_>!?=`text`:
      `concatenating: {_<} and {_>}`,

    // Condition 2: both are lists
    _<!?=`list`|_>!?=`list`:
      _< $_> + _>,  // Flatten then concat

    // Condition 3: left is undefined (disappears)
    _<!?=`undefined`:
      _>,

    // Condition 4: right is undefined (disappears)
    _>!?=`undefined`:
      _<,

    // Default: normal behavior
    _<_>+_>
  },

  // Test
  s1: `hello` + `world`,
  // `concatenating: hello and world`

  s2: [1, 2] + [3, [4]],
  // [1, 2, 3, 4] (flattened then concatenated)

  s3: 1 + undefined,
  // 1 (undefined disappears)

  s4: 5 + 5,
  // 10 (default behavior)
}
```

### Override Syntax Breakdown

```
operator: {
  condition1 | condition2: result,
  condition3: result,
  defaultBehavior
}
```

- **Pattern matching**: Conditions are checked in order, first match wins
- **`|` separator**: Logical OR between conditions
- **`,` separator**: Logical AND between conditions
- **Default behavior**: Unmatched pattern falls through to default

### Type Check Patterns

```penroscript
{
  // Single type check
  _<!?=`string`: result,

  // Multiple type checks (OR)
  _<!?=`string`|_>!?=`number`: result,

  // Multiple type checks (AND)
  _<!?=`string`&_>!?=`string`: result,

  // Negated type check
  _<!?!=`undefined`: result,
}
```

### Override Scope Dominance

#### Intra-Script Override

```penroscript
// File: override.lr
{
  +: {
    _<!?=`text`|_>!?=`text`:
      `custom concat: {_<} + {_>}`,
    _<_>+_>
  },

  local: `a` + `b`,
  // `custom concat: a + b` (override applies)
}

// Override only applies to expressions within this file
// Other files use default behavior
```

#### Folder-Level Override

```penroscript
// File: .opencode/config/folder-override.lr
{
  // All .lr files in this directory inherit this
  +: {
    _<!?=`list`|_>!?=`list`:
      ~(_< $_> + _>),  // Unique concat
    _<_>+_>
  }
}

// File: folder1/script.lr
{
  // Uses folder override
  arr: [1, 1] + [2, 2],
  // [1, 2] (unique concat from folder override)
}
```

#### Project-Level Override

```penroscript
// File: .opencode/config/project-override.lr
{
  // All files in project use this
  -: {
    _<!?=`number`&_>!?=`number`:
      Math.abs(_< - _>),  // Always return positive difference
    _<_>-_>
  }
}

// Any file in project:
{
  diff: 3 - 10,
  // 7 (absolute difference, not -7)
}
```

#### Global Override

```penroscript
// System-wide configuration
// ~/.config/left-right/overrides.lr
{
  +: {
    _<!?=`undefined`:
      0,  // Treat undefined as 0 for addition
    _<!?=`undefined`|_>!?=`undefined`:
      _< + _>,  // Handle right-side undefined
    _<_>+_>
  }
}

// All projects on machine use this
{
  result: 5 + undefined,
  // 5 (not undefined - global override converts undefined to 0)
}
```

### Nested Overrides

Overrides can be nested for complex behavior:

```penroscript
{
  +: {
    // Override for text
    _<!?=`text`|_>!?=`text`:
      {
        // Nested override for text length > 5
        #_< > 5 | #_> > 5:
          _< @\(0:3) + `...` + _> @\(0:3),

        // Normal text concat
        _: _< + _>
      },

    // Override for lists
    _<!?=`list`|_>!?=`list`:
      _< ~_>,  // Concat with deduplication

    // Default behavior
    _<_>+_>
  },

  // Test
  long: `hellos` + `worlds`,
  // `hel...wor...` (truncated)

  dup: [1, 2] + [2, 3],
  // [1, 2, 3] (deduplicated)

  short: `hi` + `bye`,
  // `hibye` (normal concat)
}
```

### Disabling Overrides

You can explicitly disable overrides at any level:

```penroscript
{
  // Explicitly use default + operator
  defaultPlus: +_default{ _< + _> },

  // Usage
  result: defaultPlus['a', 'b'],
  // 'ab' (bypasses all overrides)
}
```

### Operator Aliases

You can create aliases for operators:

```penroscript
{
  // Define alias for addition with logging
  addWithLog: {
    left: _<,
    right: _>,
    sum: left + right,
    log: `Adding {left} + {right} = {sum}`,
    sum
  },

  // Use alias
  result: 5 addWithLog 3,
  // Logs: Adding 5 + 3 = 8
  // Returns: 8
}
```

---

## 7. The `?|!` Every Operator

### Comparison to `?|` Some Operator

| Operator | Semantics | Condition Reversal? | Example |
|----------|-----------|-------------------|----------|
| `?|` | "Is there ANY element where X?" | YES - to find "all" you need "any NOT X" | `[1,2,3] ?| { _< > 2 }` → `true` |
| `?|!` | "Does EVERY element satisfy X?" | NO - condition is direct | `[1,2,3] ?|! { _< > 0 }` → `true` |

### Syntax

```penroscript
collection ?|! { predicate }
```

### Examples

```penroscript
// All elements are positive?
{
  numbers: [1, 2, 3, 4, 5],
  allPositive: numbers ?|! { _< > 0 },
  // true (all > 0)

  // Empty list (vacuously true)
  emptyAll: [] ?|! { _< > 0 },
  // true (empty lists satisfy every condition)
}

// All elements are <= 10?
{
  data: [5, 7, 9, 10],
  allLe10: data ?|! { _< <= 10 },
  // true (all <= 10)

  data2: [5, 7, 9, 11],
  allLe10_2: data2 ?|! { _< <= 10 },
  // false (11 is not <= 10)
}

// All text values have length > 3?
{
  words: [`hello`, `world`, `test`],
  allLong: words ?|! { #_< > 3 },
  // true (`test` has length 4)

  words2: [`hi`, `hello`],
  allLong2: words2 ?|! { #_< > 3 },
  // false (`hi` has length 2)
}

// All user objects are active?
{
  users: [
    { name: `Alice`, active: true },
    { name: `Bob`, active: true },
    { name: `Charlie`, active: true }
  ],
  allActive: users ?|! { @['active'] },
  // true (all active: true)

  users2: [
    { name: `Alice`, active: true },
    { name: `Bob`, active: false }
  ],
  allActive2: users2 ?|! { @['active'] },
  // false (Bob is not active)
}
```

### Comparison to Using `?|` with Reversal

```penroscript
{
  numbers: [1, 2, 3, 4, 5],

  // Using ?|! (cleaner)
  allPositive1: numbers ?|! { _< > 0 },
  // true

  // Using ?| with reversal (verbose)
  allPositive2: !(numbers ?| { _< <= 0 }),
  // true (equivalent to: NOT(any element <= 0))

  // More complex example
  // All elements are between 0 and 10?
  allInRange1: numbers ?|! { _< >= 0 & _< <= 10 },
  // true

  // With ?| reversal (harder to read)
  allInRange2: !(numbers ?| { _< < 0 | _< > 10 }),
  // true (equivalent: NOT(any element < 0 OR > 10))
}
```

### Practical Use Cases

```penroscript
// 1. Form validation (all fields present)
{
  formData: { name: `Alice`, email: `alice@test.com`, age: 30 },
  required: [`name`, `email`, `age`],

  allPresent: required ?|! { field: _<,
    formData @field !?= 'undefined'
  },
  // true (all required fields present)
}

// 2. Data quality checks (all records valid)
{
  records: [
    { id: 1, valid: true },
    { id: 2, valid: true },
    { id: 3, valid: true }
  ],

  allValid: records ?|! { @['valid'] },
  // true

  // Partially valid
  records2: [
    { id: 1, valid: true },
    { id: 2, valid: false }
  ],
  allValid2: records2 ?|! { @['valid'] },
  // false
}

// 3. Permission checks (all users have permission)
{
  users: [`alice`, `bob`, `charlie`],
  requiredPermission: `read`,

  allCanRead: users ?|! { user: _<,
    user @['permissions'] ?| { _< = requiredPermission }
  },
  // true (all users have 'read' permission)
}

// 4. Test suite results (all tests pass)
{
  tests: [
    { name: `test1`, passed: true },
    { name: `test2`, passed: true },
    { name: `test3`, passed: true }
  ],

  allPass: tests ?|! { @['passed'] },
  // true

  // Some failed
  tests2: [
    { name: `test1`, passed: true },
    { name: `test2`, passed: false }
  ],
  allPass2: tests2 ?|! { @['passed'] },
  // false
}
```

### Comparison to JavaScript

```javascript
// JavaScript: Array.every
const numbers = [1, 2, 3, 4, 5];
const allPositive = numbers.every(x => x > 0);  // true

// Left-Right: ?|!
const numbers = [1, 2, 3, 4, 5];
const allPositive = numbers ?|! { _< > 0 };    // true

// JavaScript: Condition reversal for negation
const noneNegative = numbers.every(x => !(x < 0));  // true

// Left-Right: Direct condition with ?|!
const noneNegative = numbers ?|! { _< >= 0 };    // true
```

### Edge Cases

```penroscript
{
  // Empty list (vacuously true)
  empty: [] ?|! { false },
  // true (no elements to violate condition)

  // List with single element
  single: [42] ?|! { _< = 42 },
  // true

  // Nested lists
  nested: [[1, 2], [3, 4]],
  // Check that all inner lists have length 2?
  allLength2: nested ?|! { #_< = 2 },
  // true

  // Type coercion in predicate
  mixed: [1, `2`, 3],
  // All elements are truthy?
  allTruthy: mixed ?|! { _< },
  // true

  // Mixed types in predicate
  mixed2: [1, 2, `hello`, 3],
  // All elements are either numbers or text?
  allValidTypes: mixed2 ?|! { _<!?=`number`|_<!?=`text` },
  // true
}
```

---

## 8. Summary of Operator Overloading Benefits

### Advantages Over JavaScript

1. **Predictable Type Coercion**
   - Left-Right: Clear hierarchy (List > Text > Map > Number)
   - JavaScript: Complex, surprising rules

2. **Identity Element Handling**
   - Left-Right: `undefined` disappears naturally
   - JavaScript: `1 + undefined = NaN`

3. **Collection Semantics**
   - Left-Right: Lists concatenate, maps merge
   - JavaScript: Lists coerce to strings!

4. **Type-Dependent Dispatch**
   - Left-Right: Single operator, multiple meanings
   - JavaScript: Multiple operators, fewer types

5. **Override Capability**
   - Left-Right: Customize operator behavior per scope
   - JavaScript: No operator overriding

### Comparison Table

| Operation | JavaScript | Left-Right | Benefit |
|-----------|------------|-------------|----------|
| `[1, 2] + [3, 4]` | `'1,23,4'` | `[1, 2, 3, 4]` | Proper list concat |
| `{a:1} + {b:2}` | `'[object Object][object Object]'` | `{a: 1, b: 2}` | Map merge |
| `1 + undefined` | `NaN` | `1` | Identity handling |
| `` `5` `` + 3 | `'53'` | `` `53` `` | Same (text wins) |
| `[1] + 2` | `'12'` | `[1, 2]` | List preservation |

### Design Philosophy Summary

1. **Structure Preservation**: Collections stay collections
2. **Type Safety**: Clear rules, no surprises
3. **Extensibility**: Override operators at any level
4. **Readability**: Type-dependent operators reduce boilerplate
5. **Determinism**: Same types, same result, always

---

## Related Documentation

- [Type System](./02-type-system.md) — Primitive types and type inference
- [Operator Reference](./03-operator-reference.md) — Comprehensive operator catalog
- [Evaluation Model](./04-evaluation-model.md) — LTR evaluation and currying
- [Design Philosophy](./01-design-philosophy.md) — Language design principles
- [Language Overview](./00-language-overview.md) — Complete language overview
- [Master Index](./README.md) — Complete documentation suite
