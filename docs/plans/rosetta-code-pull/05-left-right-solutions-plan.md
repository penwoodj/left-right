# Left-Right Solutions Plan

## Overview

This document outlines how to use the Rosetta Code corpus to validate and improve Left-Right language design. It identifies which tasks to solve first, how to approach them, and what language features each category exercises.

## Approach

### By-Example Documentation

Following Kieran Brown's suggestion, we'll build documentation through examples. Each Rosetta Code task solved in Left-Right becomes a documented pattern showing:

1. Problem statement
2. Left-Right solution
3. Explanation of operator usage
4. Comparison with other languages
5. Design insights (what works, what doesn't)

These examples form the basis of:
- Tutorial documentation
- Language spec examples
- Compiler test suite
- Marketing materials (showing expressiveness)

### Iterative Validation

Solve tasks in priority order. After each batch:

1. **Does it compile?** Validate transpiler correctness
2. **Is it readable?** Test operator syntax ergonomics
3. **Is it concise?** Compare line counts with other languages
4. **Is it missing features?** Identify gaps in the language

Use insights to refine Left-Right spec and transpiler before solving more tasks.

### Validation Metrics

For each solved task, track:

| Metric | Target | How to Measure |
|---------|--------|----------------|
| Line count | ≤ median of other languages | Count non-blank, non-comment lines |
| Concept count | Low (≤ 5 concepts) | Count distinct operators/operations |
| Readability score | High | Human rating 1-10 |
| Compilation success | 100% | Run transpiler |
| Runtime correctness | 100% | Execute transpiled code |

## Phase 1: Core Features (Must Validate First)

### Category: Control Flow

**Why Tests**: Left-Right's `&` and `|` operators are unusual. Need to validate they work as intended for all conditional patterns.

**Tasks to Solve** (Priority: CRITICAL)

#### 1. Conditional Structures

**Rosetta Code**: `/wiki/Conditional_structures`

**Problem**: Show if/else, ternary, and switch patterns.

**Left-Right Approach**:

```left-right
// Traditional if/else
condition & trueValue | falseValue

// Nested conditions
condition1 & value1 | condition2 & value2 | defaultValue

// Ternary equivalent
number > 10 & 'large' | number > 5 & 'medium' | 'small'
```

**Validation Points**:

- Does left-to-right evaluation of `&` and `|` work as expected?
- Can you chain multiple conditions clearly?
- Is precedence intuitive?
- How does it compare to Python's ternary or JavaScript's conditional operator?

**Expected Line Count**: 1-3 (vs Python 3-5, JavaScript 3-5)

**Success If**: Can express all Rosetta Code conditional examples with ≤ 3 lines.

#### 2. Loops

**Rosetta Code**: `/wiki/Loops`

**Problem**: Show for loop, while loop, and iteration patterns.

**Left-Right Approach**:

```left-right
// For loop over array
1..10 #| { _< @0 * 2 }

// While loop with predicate
{ counter: 0 } ?| { counter @> 10 | counter < @ { counter: counter + 1 } }

// Reduce loop (sum)
0..10 #| { _< _ + @0 } $0 @> +
```

**Validation Points**:

- Does `#|` (filter/map) iterate correctly?
- Can you express both eager and lazy iteration?
- Is recursion via `^^` tail-call optimized?
- How does loop syntax compare to Python's `for x in range(10)`?

**Expected Line Count**: 1-3 per loop type

**Success If**: Can express all Rosetta Code loop patterns cleanly.

#### 3. Break/Continue

**Rosetta Code**: `/wiki/Break`, `/wiki/Continue`

**Problem**: Show early exit from loops and skipping iterations.

**Left-Right Approach** (MAY NEED NEW SYNTAX):

```left-right
// Current approach (if no break/continue operators)
1..10 #| {
  num: _< @0
  num % 3 == 0 & [] | num == 5 & [] | num
} $> filter // Filter instead of continue

// Proposed: add break/continue operators
1..10 #| {
  num: _< @0
  num % 3 == 0 && { continue: true }
  num == 5 && { break: true }
  num
}
```

**Validation Points**:

- Can filter be used instead of continue effectively?
- Should we add explicit break/continue operators?
- How does this compare to Python's `continue`/`break`?

**Decision Point**: If filtering is awkward, add break/continue to language spec.

### Category: Arithmetic & Math

**Why Tests**: Validates operator direction (`_<` vs `_>`), precedence, and basic types.

**Tasks to Solve** (Priority: HIGH)

#### 4. Arithmetic Operations

**Rosetta Code**: `/wiki/Arithmetic_operations`

**Problem**: Demonstrate addition, subtraction, multiplication, division, modulo.

**Left-Right Approach**:

```left-right
// Basic operations
a + b
a - b
a * b
a / b
a % b

// Operator direction test
5 _< + 2  // left section: (+ 2)
5 _> * 2  // right section: (5 *)

// Chained operations
10 - 3 * 2 + 1  // Left to right: ((10 - 3) * 2) + 1
```

**Validation Points**:

- Does left-to-right evaluation feel natural?
- Are `_</_>` sections useful for currying?
- How does precedence (or lack thereof) affect readability?
- Should we add parentheses for grouping?

**Expected Line Count**: 1 per operation

**Success If**: Can express arithmetic as clearly as Python/JavaScript.

#### 5. Factorial

**Rosetta Code**: `/wiki/Factorial`

**Problem**: Calculate factorial of n (n!).

**Left-Right Approach**:

```left-right
// Recursive (uses ^^ for self-reference)
{ n: _< } ?| {
  n <= 1 & 1 | n * ^^[n - 1]
}

// Iterative using reduce
{ n: _< }
1..n #| { _< @0 + 1 } $> *

// Point-free using range and product
1..n $> *
```

**Validation Points**:

- Does `^^` (recursive self-reference) work as intended?
- Is the iterative version clearer?
- Can you express factorial point-free elegantly?
- How does this compare to Python's `math.factorial`?

**Expected Line Count**: 1-3 (vs Python 1-3, Haskell 1-2)

**Success If**: Can express factorial concisely with clear intent.

#### 6. Fibonacci Sequence

**Rosetta Code**: `/wiki/Fibonacci_sequence`

**Problem**: Generate first n Fibonacci numbers.

**Left-Right Approach**:

```left-right
// Iterative
{ n: _< }
{ a: 0, b: 1, result: [a] }
1..n #| {
  { a: b, b: a + b, result: result + [b] }
} @> result

// Using higher-order operations
{ n: _< }
[0, 1] $> {
  [prev, current]
  prev + current
} ?| { _< $0 } $0 @< 1
```

**Validation Points**:

- Can you express stateful iteration clearly?
- Is the array-oriented approach concise?
- Do you need imperative variables for state?
- How does this compare to Python's tuple unpacking?

**Expected Line Count**: 2-4 (vs Python 3-6, Haskell 1-2)

**Success If**: Can express Fibonacci without boilerplate.

### Category: String Processing

**Why Tests**: Left-Right treats strings as operators with `_</_>` interpolation. Critical validation.

**Tasks to Solve** (Priority: HIGH)

#### 7. String Interpolation

**Rosetta Code**: `/wiki/String_interpolation`

**Problem**: Insert variables into strings.

**Left-Right Approach**:

```left-right
// Simple interpolation
name: 'John'
greeting: 'Hello, {name}!'

// Using _< for inline interpolation
age: 30
'You are {age} years old'

// Complex expressions
items: 5
'You have {items % 10 == 1 & "item" | "items"}'

// Multiline string with operator
template: '
Hello {name},
You have {items} {items % 10 == 1 & "item" | "items"}.
Total: ${items * 10}.
'
```

**Validation Points**:

- Is `{var}` interpolation syntax clear?
- Can you embed expressions naturally?
- Are multiline strings easy to write?
- How does this compare to JavaScript's `${var}` or Python's f-strings?

**Expected Line Count**: 1-2 (vs Python 1, JavaScript 1)

**Success If**: String interpolation feels more natural than template literals.

#### 8. Reverse a String

**Rosetta Code**: `/wiki/Reverse_a_string`

**Problem**: Reverse a string character by character.

**Left-Right Approach**:

```left-right
// Convert to array, reverse, join
'hello' #> ~
// or
'hello' #> $> @< 0

// Point-free
#> $> @< 0
```

**Validation Points**:

- Does `~` (reverse) operator work on strings?
- Can you convert string to array easily?
- Is the point-free version readable?
- How does this compare to Python's `'hello'[::-1]`?

**Expected Line Count**: 1 (vs Python 1, JavaScript 2)

**Success If**: String reversal is trivial in Left-Right.

#### 9. String Length

**Rosetta Code**: `/wiki/String_length`

**Problem**: Get character count of a string.

**Left-Right Approach**:

```left-right
// Using # (tally/length) operator
'hello' #
// or
'hello' $> #

// Point-free
#
```

**Validation Points**:

- Does `#` work on strings intuitively?
- Is it clear this counts characters, not bytes?
- Should there be separate operators for `len()` and `size()`?
- How does this compare to Python's `len(s)` or JavaScript's `s.length`?

**Expected Line Count**: 1 (vs Python 1, JavaScript 1)

**Success If**: Getting string length is obvious.

### Category: Collections

**Why Tests**: Arrays and maps are core to Left-Right. Need to validate syntax and operators.

**Tasks to Solve** (Priority: HIGH)

#### 10. Arrays

**Rosetta Code**: `/wiki/Arrays`

**Problem**: Demonstrate array creation, indexing, slicing, and operations.

**Left-Right Approach**:

```left-right
// Array literal
numbers: [1, 2, 3, 4, 5]

// Indexing
numbers @ 2  // 3

// Slicing
numbers @ [1..3]  // [2, 3, 4]

// Range
1..10  // [1, 2, 3, ..., 10]

// Concatenation
[1, 2] + [3, 4]  // [1, 2, 3, 4]

// Mapping
numbers #| { _< @0 * 2 }  // [2, 4, 6, 8, 10]

// Filtering
numbers #| { _< @0 % 2 == 0 }  // [2, 4]

// Reducing
numbers #| { _< @0 + @1 } $0  // sum
// or
numbers $> +
```

**Validation Points**:

- Is array literal syntax `[]` intuitive?
- Does `@` for indexing feel natural?
- Are slicing operations clear?
- Do map/filter/reduce operators work as expected?
- Is array concatenation with `+` intuitive?

**Expected Line Count**: 1-2 per operation

**Success If**: Array operations feel more ergonomic than JavaScript.

#### 11. Associative Arrays (Maps)

**Rosetta Code**: `/wiki/Associative_arrays`

**Problem**: Demonstrate map creation, access, update, and iteration.

**Left-Right Approach**:

```left-right
// Map literal
person: { name: 'John', age: 30, city: 'NYC' }

// Access by key
person @ 'name'  // 'John'
// or
person @ name  // if name is a variable

// Nested access
person @ ['city', 0]  // 'N' (first char of 'NYC')

// Update/add
person + { age: 31 }

// Keys
person @> $0  // ['name', 'age', 'city']

// Values
person @> $1  // ['John', 30, 'NYC']

// Iteration
person @> #| { _< @0 @> ':' + ' ' + @1 @> '\n' }
```

**Validation Points**:

- Is `{ key: value }` syntax consistent and clear?
- Does `@` for map access work well?
- Can you distinguish between string keys and variable keys?
- Are `@>` and `@<` for keys/values intuitive?
- Is map concatenation with `+` useful?

**Expected Line Count**: 1-2 per operation

**Success If**: Map operations are more concise than JavaScript objects.

#### 12. Stack Operations

**Rosetta Code**: `/wiki/Stacks`

**Problem**: Implement LIFO push/pop operations.

**Left-Right Approach**:

```left-right
// Push
stack: []
stack + [1]  // push 1
stack + [2]  // push 2

// Pop
stack @ [0..-2]  // all but last
stack @ -1  // last element

// Peek
stack @ -1

// Is empty
stack # == 0 & true | false
```

**Validation Points**:

- Are array slices with negative indices intuitive?
- Can you express stack operations clearly?
- Should we add dedicated push/pop operators?
- How does this compare to Python's `list.append()`/`list.pop()`?

**Expected Line Count**: 1-2 per operation

**Decision Point**: If stack operations are awkward, consider dedicated operators.

## Phase 2: Patterns and Algorithms (Important Expressiveness)

### Category: Recursion

**Why Tests**: Validates function syntax, self-reference (`^^`), and tail-call optimization.

**Tasks to Solve** (Priority: MEDIUM)

#### 13. Ackermann Function

**Rosetta Code**: `/wiki/Ackermann_function`

**Problem**: Implement Ackermann function (deep recursion test).

**Left-Right Approach**:

```left-right
{ m: _<, n: _< } ?| {
  m == 0 & n + 1 |
  m > 0 & n == 0 & ^^[m - 1, 1] |
  ^^[m - 1, ^^[m, n - 1]]
}
```

**Validation Points**:

- Can you express mutual recursion clearly?
- Does tail-call optimization handle this?
- How does this compare to Haskell's pattern matching?

**Expected Line Count**: 4-6 (vs Python 6-8, Haskell 4-5)

**Success If**: Recursive function syntax is clean.

### Category: Algorithms

**Why Tests**: Validates higher-order functions and algorithmic expressiveness.

**Tasks to Solve** (Priority: MEDIUM)

#### 14. Sorting

**Rosetta Code**: `/wiki/Sorting_algorithms`

**Problem**: Implement bubble sort, quick sort, or merge sort.

**Left-Right Approach** (Bubble Sort):

```left-right
{ arr: _< }
{ sorted: arr }
{ len: arr # }
{ swapped: true }
swapped ?| {
  0..(len - 2) #| {
    i: _< @0
    i + 1
    sorted @ i > sorted @ [i + 1] ?| {
      { sorted: sorted @ [i] + [sorted @ [i + 1]] + sorted @ [i + 2..], swapped: true }
    } | sorted
  } @> sorted
} @> sorted
```

**Validation Points**:

- Can you express algorithms without excessive boilerplate?
- Is array updating clear?
- How does this compare to Python's sorting?

**Expected Line Count**: 8-12 (vs Python 6-10, Haskell 5-8)

**Note**: Left-Right should rely on stdlib sort in practice. This tests algorithm expression only.

#### 15. Binary Search

**Rosetta Code**: `/wiki/Binary_search`

**Problem**: Search sorted array using divide-and-conquer.

**Left-Right Approach**:

```left-right
{ arr: _<, target: _< }
{ lo: 0, hi: arr # - 1 }
lo <= hi ?| {
  { mid: (lo + hi) / 2 }
  arr @ mid == target & mid |
  arr @ mid < target & { lo: mid + 1 } |
  { hi: mid - 1 }
} | -1
```

**Validation Points**:

- Can you express divide-and-conquer clearly?
- Is the loop structure intuitive?
- How does this compare to Python's `bisect` module?

**Expected Line Count**: 6-8 (vs Python 5-7)

**Success If**: Algorithm is readable and correct.

## Phase 3: Library and Practicality (Defer If Needed)

### Category: File I/O

**Why Tests**: Validates that Left-Right is practical for real-world tasks.

**Tasks to Solve** (Priority: LOW - depends on target runtime)

#### 16. Read a File

**Rosetta Code**: `/wiki/Read_a_file`

**Problem**: Read entire file content into string.

**Left-Right Approach** (MAY NEED STD FUNCTIONS):

```left-right
// Assuming std function read()
read('/path/to/file.txt')

// Or as operator
'/path/to/file.txt' @> read
```

**Validation Points**:

- Should file I/O be operator-based or function-based?
- How do you handle errors?
- What's the return type (string or array of lines)?

**Decision Point**: Design file I/O based on target language (JS/Rust) conventions.

### Category: Type System

**Why Tests**: Validates loose typing and type coercion behavior.

**Tasks to Solve** (Priority: LOW)

#### 17. Type Casting

**Rosetta Code**: `/wiki/Type_casting`

**Problem**: Convert between types (string to number, etc.).

**Left-Right Approach**:

```left-right
// String to number
'123' @> number

// Number to string
123 @> string

// Boolean to number
true @> number  // 1
false @> number  // 0

// Implicit coercion in operations
'5' + 3  // Should this be 8 or '53'?
```

**Validation Points**:

- Should type coercion be explicit or implicit?
- What are the coercion rules?
- How does this compare to JavaScript's loose typing?

**Decision Point**: Define explicit type conversion operators if implicit coercion is error-prone.

## Priority Order Summary

### Immediate (Week 1-2)

1. Control Flow (3 tasks) - Validate core operators
2. Arithmetic & Math (3 tasks) - Validate operator syntax
3. String Processing (3 tasks) - Validate template operators
4. Collections (3 tasks) - Validate array/map syntax

### Important (Week 3-4)

5. Recursion (1-2 tasks) - Validate function syntax
6. Algorithms (2 tasks) - Validate expressiveness
7. Pattern Matching (if needed) - Validate destructuring

### Defer Until Core Validated

8. File I/O - Depends on target runtime design
9. Type System - Depends on coercion decisions
10. Other categories - After core features validated

## Success Criteria

### Per Category

A category is validated when:

- [ ] All solved tasks compile successfully
- [ ] Solutions are ≤ median line count of other languages
- [ ] At least one example demonstrates unique Left-Right strength
- [ ] Documentation written explaining patterns

### Per Language Feature

A feature is validated when:

- [ ] 5+ tasks use it successfully
- [ ] No workarounds or hacks required
- [ ] Solutions feel natural, not forced
- [ ] Comparison with other languages shows strengths

### Overall Project

Rosetta Code validation succeeds when:

- [ ] 50+ tasks solved across high-priority categories
- [ ] 10+ language design improvements identified
- [ ] By-example documentation generated
- [ ] Compiler/transpiler bugs found and fixed
- [ ] Left-Right solutions contributed to Rosetta Code

## Integration with Left-Right Toolchain

### Testing

After solving each task:

```bash
# Transpile to JavaScript
lr transpile examples/factorial.lr > factorial.js

# Execute
node factorial.js

# Transpile to Rust
lr transpile examples/factorial.lr > factorial.rs

# Compile and run
rustc factorial.rs && ./factorial
```

### Documentation Generation

Auto-generate docs from solved examples:

```bash
# Generate tutorial
lr docs generate --source examples/ --output docs/

# Generate spec examples
lr docs spec --source examples/ --output spec/
```

### Compiler Test Suite

Use Rosetta Code examples as test cases:

```left-right
// test suite format
{ name: 'Factorial', input: 5, expected: 120 }
{ name: 'Fibonacci', input: 10, expected: [0, 1, 1, 2, 3, 5, 8, 13, 21, 34] }
```

---

**Next**: After scraping is complete, start solving Phase 1 tasks.
