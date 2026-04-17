# Auto-Currying — Partial Application

Documents covering auto-currying mechanisms and partial application patterns.

## Key Topics

### Left-Hungry Currying
From [`PenroScript.md`](../pipe-operators/PenroScript.md):

**Default Behavior:**
```javascript
// Diatic operators left-hungry by default
{ _< + 1 } // Expects left argument, returns function
```

**Usage:**
```javascript
// Apply to single value
5 { _< + 1 } // Result: 6

// Apply to multiple values
[1, 2, 3] ${ _< + 1 } // Result: [2, 3, 4]
```

### Currying by Type

From design notes:
```javascript
// Static value on one side returns monadic function
{ _< + 1 } // Curried: waits for right argument
{ 1 + _> } // Curried: waits for left argument
```

### Partial Application

#### Single Argument
```javascript
// Partially apply operator
addOne: { _< + 1 }
// Now addOne is a function
```

#### Multiple Arguments
```javascript
// Apply to list (partial application)
items ${ _< + 10 }
```

### Auto-Currying Examples

#### Operator as Function
```javascript
// Operator definition with implicit left
rangeFilter: { _< $<[>2, <5] $+`& }
// rangeFilter is now a callable function
```

#### Pipeline Partial
```javascript
// Stage in pipeline is partial function
data
  $filter // Partial: waits for predicate
  $transform // Partial: waits for transformation
```

### Directional Currying

#### Left Section
```javascript
// Bind left argument
{ _< - threshold }
// Equivalent to: (x) => x - threshold
```

#### Right Section
```javascript
// Bind right argument
{ _> - threshold }
// Equivalent to: (x) => threshold - x
```

### Reversing Currying

From design notes:
```javascript
// Can be reversed from left-hungry
{ _> + 1 } // Right-hungry
// Equivalent to: { x: _, 1 + x }
```

## Design Principles

1. **Default Currying** — Diatic operators curry automatically
2. **Explicit Arguments** — Override default with directional sections
3. **Composable** — Curried functions can be composed
4. **No Explicit Currying Syntax** — Happens implicitly
5. **Predictable** — Currying follows clear rules

## Related Concepts

- **Currying** — Converting multi-arg functions to single-arg chains
- **Partial Application** — Fixing some arguments, leaving others
- **Auto-Currying** — Implicit currying by default
- **Function Composition** — Combining partial applications
- **Haskell Currying** — All functions curried by default
- **ML Family** — Currying semantics
- **Section Syntax** — Creating partial functions (Haskell sections)
