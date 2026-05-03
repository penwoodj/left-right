# Functions and Scope — First-Class Operators

Left-Right treats functions as operator blocks with first-class status. This document covers function definition, parameter binding, scope model, point-free style, and composition patterns.

## Operator Function Syntax

### Function Definition

Functions are defined as operator blocks using brace syntax with parameter declarations:

```javascript
// Basic function with two parameters
{
  param1: _<@0,
  param2: _<@1,
  param1 + param2 // Return value
}
```

### Parameter Boundary Markers

#### _< — Left Argument Marker

The `_<` placeholder marks the position of the left argument in operator application:

```javascript
// Function expecting left argument
{
  input: _<@0,
  input * 2
}

// Application
value { _< * 2 } // Result: value * 2
```

#### _> — Right Argument Marker

The `_> ` placeholder marks the position of the right argument:

```javascript
// Function expecting right argument
{
  input: _>@0,
  input * 2
}

// Application
{ 10 * _> } value // Result: value * 10
```

### Multiple Parameter Syntax

Multiple parameters are declared using indexed parameter access:

```javascript
// Three-parameter function
{
  first: _<@0,
  second: _<@1,
  third: _<@2,
  first + second + third // Return sum
}
```

## Anonymous Functions and Lambdas

### First-Class Function Values

All functions are anonymous by default and can be assigned, passed, and returned:

```javascript
// Assign to variable
addOne: { input: _<@0, input + 1 }

// Pass as argument
items ${ _< + 1 } // Map using lambda

// Return from function
{
  threshold: 10,
  return: { _< > threshold } // Returns predicate function
}
```

### Lambda as Value

Functions are values that can be stored in maps:

```javascript
// Collection of functions
{
  add: { _<@0, _>@1, _< + _> },
  multiply: { _<@0, _>@1, _< * _> },
  square: { _<@0, _< * _< }
}
```

## Point-Free Style

### Implicit Left Argument

Single-expression operators can assume implicit left argument without explicit parameter declaration:

```javascript
// Point-free: no explicit parameter
items ${ _< * 2 }
// Equivalent to: items ${ { input: _<@0, input * 2 } }

// Multiple point-free stages
data ?{ _< > 5 } ${ _< * 10 } # size
// Filter, map, count
```

### Point-Free Benefits

Point-free style eliminates intermediate variable names:

```javascript
// Traditional style (JavaScript)
const result = items
  .filter(item => item > 5)
  .map(item => item * 10)
  .length;

// Point-free style (Left-Right)
items ?{ _< > 5 } ${ _< * 10 } #
```

**Advantages:**
- No variable naming overhead
- Clearer data flow
- Composable operations
- Reduced boilerplate

## Scope Model

### Sequential Binding in Map Bodies

Map bodies evaluate keys sequentially, allowing later keys to reference earlier keys:

```javascript
// Sequential scope
{
  base: 10,
  offset: 5,
  multiplier: 2,
  result: (base + offset) * multiplier
}
// Evaluation order:
// 1. base = 10
// 2. offset = 5
// 3. multiplier = 2
// 4. result = (10 + 5) * 2 = 30
```

### Nested Scope

Nested blocks create new lexical scopes:

```javascript
{
  outer: 10,
  result: {
    inner: 5,
    outer + inner
  }
}
// outer accessible in inner block
// inner not accessible outside
```

### Scope Through Function Composition

Composition preserves scope visibility:

```javascript
// Outer scope accessible in pipeline
{
  threshold: 5,
  result: items
    ?{ _< > threshold }
    ${ _< * 2 }
}
```

## Parameter Destructuring

### Array Destructuring

Parameters can destructure arrays using indexed access:

```javascript
// Destructure array input
{
  first: _<@0,
  second: _<@1,
  rest: _<@[2, '~'],
  first + second + #rest
}
```

### Path Destructuring

Deep paths can be destructured using the `@` operator:

```javascript
// Destructure nested structure
{
  value: _<@[, , ],
  key: _<@[, , ],
  value + key
}
```

### Conditional Destructuring

Destructuring with conditionals:

```javascript
// Destructure with fallback
{
  value: _<@0,
  fallback: _<@1 || 0,
  value ?? fallback
}
```

## Function Composition

### Composition Operators

#### >> — Left-to-Right Composition

The `>>` operator composes functions left to right:

```javascript
// Compose two functions
items
  >> ${ _< * 2 }
  >> ${ _< + 10 }
// Equivalent to: map(x => (x * 2) + 10)
```

#### << — Right-to-Left Composition

The `<<` operator composes functions right to left (traditional functional style):

```javascript
// Compose in reverse order
items << { _< + 10 } << { _< * 2 }
// Equivalent to: map(x => (x * 2) + 10)
```

### Composition Patterns

#### Single Operation

```javascript
// Apply one transformation
data ${ _< * 2 }
```

#### Multi-Stage Pipeline

```javascript
// Chain multiple transformations
data
  ?{ _< > 5 }        // Filter
  ${ _< * 10 }        // Transform
  ~                    // Unique
  #                    // Count
```

#### Complex Composition

```javascript
// Compose custom functions
{
  increment: { _<@0, _< + 1 },
  double: { _<@0, _< * 2 },
  addThenDouble: _< >> increment >> double
}
```

## Auto-Currying Behavior

### Implicit Currying

Dyadic operators automatically curry when one argument is provided:

```javascript
// Static value creates curried function
{ _< + 1 } // Returns function expecting right argument

// Partial application to collection
[1, 2, 3] ${ _< + 1 }
// Equivalent to: [1, 2, 3].map(x => x + 1)
```

### Currying in Pipelines

Pipeline stages can be partially applied:

```javascript
// Stage 1: Partial filter
data
  ?{ _< > 5 }        // Filter with predicate

// Stage 2: Partial map
  ${ _< * 10 }        // Transform with function

// Stage 3: Partial aggregation
  $+ &                  // Reduce with AND
```

### Directional Currying Control

Override default left-hungry behavior:

```javascript
// Default: left-hungry
{ _< + 10 } // Bind left

// Reversed: right-hungry
{ _> + 10 } // Bind right
```

## Comparison with Functional Patterns

### Haskell Point-Free

**Haskell:**
```haskell
-- Point-free composition
map (*2) . filter (>5)
```

**Left-Right:**
```javascript
// Point-free pipeline
data ?{ _< > 5 } ${ _< * 2 }
```

### Clojure Partial Application

**Clojure:**
```clojure
;; Partial application
(def add-one (partial + 1))
```

**Left-Right:**
```javascript
// Implicit currying
{ _< + 1 }
```

### Lodash/FP Flow

**JavaScript with Lodash FP:**
```javascript
const process = flow(
  filter(x => x > 5),
  map(x => x * 2),
  uniq
);

process(items);
```

**Left-Right:**
```javascript
// Native pipeline
items
  ?{ _< > 5 }
  ${ _< * 2 }
  ~
```

### Ramda Pipe

**Ramda:**
```javascript
const process = R.pipe(
  R.filter(R.gt(5)),
  R.map(R.multiply(2)),
  R.uniq
);
```

**Left-Right:**
```javascript
// Built-in pipeline syntax
items ?{ _< > 5 } ${ _< * 2 } ~
```

## Function as Data

### Storing Functions

Functions can be keys in maps:

```javascript
{
  add: { _<@0, _>@1, _< + _> },
  subtract: { _<@0, _>@1, _< - _> },
  operation: { _<@0, _>@1, operations@_<@0@_>@1 }
}
```

### Conditional Function Selection

```javascript
{
  operation: _<@0,
  threshold: 10,
  result: { _< > threshold } ? { _< + 10 } : { _< * 2 }
}
```

## Operator Overriding Semantics

Left-Right supports overriding default operator behavior at multiple levels, enabling customization while maintaining language consistency.

### Override Levels (Top-Down Dominant)

Operator overrides cascade from most specific to least specific:

1. **Intra-script override** — Most specific, dominant within file
2. **Folder override** — Applies to all .lr files in directory
3. **Project override** — Applies to entire project
4. **Global machine override** — Least specific, system-wide default

**Hierarchy:**
```
intra-script > folder > project > global
```

### Full Override Semantics

Full override replaces default operator behavior entirely:

**Example:**
```penroscript
// Intra-script override for + operator
+: {
  left: _<@0,
  right: _<@1,
  // Custom implementation
  left + right + 10  // Always add 10
}

// Now all + calls in this file use custom behavior
5 + 3  // 18 (not 8)
```

### Partial Overrides with Type Checks

Partial overrides can specify behavior for specific types:

**Example:**
```penroscript
// Override + for text only
+: {
  left: _<@0,
  right: _<@1,
  left ? = `text` & right ? = `text`
    ? left >< right  // Text concatenation with separator
    : left + right   // Default for other types
}

`hello` + `world`  // `hello><world` (custom)
5 + 3                // 8 (default)
```

### Folder-Level Override

Override operators for all files in a directory:

**File Structure:**
```
project/
  folder/
    .lr-operators
    file1.lr
    file2.lr
```

**`.lr-operators` file:**
```penroscript
// Folder-level override
$: {
  collection: _<@0,
  collection # + 1  // Custom + operator
}
```

### Project-Level Override

Override operators for entire project:

**File:** `project/.lr-operators` (root)
```penroscript
// Project-level override
@: {
  path: _<@0,
  // Custom path access with logging
  path @- @[]
}
```

### Global Machine Override

System-wide operator definitions:

**File:** `~/.left-right/operators.lr`
```penroscript
// Global machine override
!: {
  value: _<@0,
  !value  // Custom NOT behavior
}
```

### Override Resolution

When multiple overrides exist, the most specific wins:

**Example:**
```penroscript
// File: project/folder/file.lr

// 1. Global override exists for +
// 2. Project override exists for + (for numbers)
// 3. Folder override exists for + (for strings)
// 4. Intra-script override exists for +

// Resolution: Intra-script override WINS (most specific)
5 + 3  // Uses intra-script implementation
```

### Type Check Syntax

Partial overrides use type checking for conditional behavior:

```penroscript
operatorName: {
  param1: _<@0,
  param2: _<@1,
  param1 ? = `number` & param2 ? = `number`
    ? param1 + param2
    : param1 >< param2  // Different behavior for non-numbers
}
```

### Best Practices

1. **Be explicit about types** — Use `?` operator for type checks in partial overrides
2. **Document overrides** — Add comments explaining why override exists
3. **Test thoroughly** — Overrides affect all operator calls in scope
4. **Use narrow scope** — Prefer folder/file over global when possible
5. **Preserve semantics** — Maintain core operator intent even when customizing

---

## Advanced Patterns

### Higher-Order Functions

Functions that accept or return functions:

```javascript
// Function returning function
{
  multiplier: _<@0,
  return: { _<@0, _< * multiplier }
}

// Function accepting function
{
  items: _<@0,
  transform: _<@1,
  items $transform
}
```

### Predicate Functions

Truthy/falsy-returning functions for filtering:

```javascript
{
  threshold: 5,
  isAboveThreshold: { _< > threshold },
  items ?{ isAboveThreshold }
}
```

### Aggregator Functions

Functions for reduction:

```javascript
{
  sum: {
    items: _<@0,
    items $+ |      // Reduce with OR (initial 0)
  },
  product: {
    items: _<@0,
    items $+ &      // Reduce with AND (initial 1)
  }
}
```

## Design Principles

1. **First-Class Functions** — Functions are values
2. **Anonymous by Default** — No function names required
3. **Implicit Parameters** — Point-free style enabled
4. **Sequential Scope** — Keys reference earlier bindings
5. **Auto-Currying** — Dyadic operators curry automatically
6. **Composable** — Functions chain via pipelines
7. **Destructuring** — Array and path unpacking

## Related Concepts

- **First-Class Functions** — Functions as values
- **Higher-Order Functions** — Functions accepting/returning functions
- **Point-Free Style** — No named parameters
- **Currying** — Multi-arg to single-arg chains
- **Partial Application** — Fixing some arguments
- **Function Composition** — Combining functions
- **Lexical Scope** — Variable visibility
- **Destructuring** — Pattern matching on parameters
- **Lambda Calculus** — Theoretical foundation
