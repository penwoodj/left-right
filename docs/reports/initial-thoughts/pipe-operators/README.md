# Pipe Operators — Left-to-Right Data Flow

Documents demonstrating pipe operators, left-to-right evaluation, and pipeline patterns.

## Contents

- [`PenroScript.md`](./PenroScript.md) — Complete examples of LTR evaluation
- [`Penscript_LeftRight brainstorm.md`](../language-design-comprehensive/Penscript_LeftRight%20brainstorm.md) — Section 7: The `_/` Directional Forms

## Key Topics

### Left-to-Right Evaluation
From [`PenroScript.md`](./PenroScript.md):

**Core Principle:**
```javascript
// Evaluate left to right, no precedence
threats $?{ condition } ${ transform } # count
```

**No Operator Precedence:**
- Parentheses for grouping only
- No implicit precedence levels
- Every operator follows LTR order

### Pipeline Operators

#### Standard Library Pipes
From operator table:
- `map` (`$`) — Transform each element
- `filter` (`$?`) — Predicate-based selection
- `flatMap` (`$_`) — Transform and flatten
- `reduce` (`$+`) — Accumulate values

#### Function Composition
```javascript
// Lodash FP style (right-to-left nesting)
flow(
  filter(pred),
  map(transform)
)(data);
```

```javascript
// PenroScript style (left-to-right)
data $pred $transform
```

### Directional Sections
From brainstorm checklist:

**Left Section (`_<`):**
```javascript
// Binds left argument
{ _< + 1 } // Equivalent to { x: _, x + 1 }
```

**Right Section (`_>`):**
```javascript
// Binds right argument
{ _> + 1 } // Equivalent to { x: _, 1 + x }
```

**Nesting Sections:**
```javascript
// Compose directional operators
{ _< + { _< * 2 } } // Multiply by 2, then add
```

### Pipe Examples

#### Simple Pipeline
```javascript
// Count malicious threats
threats $?{ @['AI Confidence Level', 'value'] = 'malicious' } #
```

#### Complex Pipeline
```javascript
// Process table data with multiple transforms
tableQueryData
  $_{ @'result' }          // Extract and flatten
  ${ @0 ~~ capitalize }     // Capitalize
  ~                       // Unique
  >< ', '                  // Join with commas
```

#### Conditional Pipeline
```javascript
// Boolean branching in pipeline
{ onlyUnique: _<@2 }
results
  ${ @'result' }
  { onlyUnique ^?: _<~, _< } // Unique if flag set
```

### Execution Walkthroughs
From [`01-chatgpt-designing-a-programming-language.md`](../language-design-comprehensive/01-chatgpt-designing-a-programming-language.md):

**Example: `3 $<[>2, <5] $+`&`**
1. Start with `3`
2. Apply predicates `[>2, <5]` → `[true, true]`
3. Reduce with AND (`&`) → `true`

**Example: `10 $<[>2, <5] $+`|`**
1. Start with `10`
2. Apply predicates → `[true, false]`
3. Reduce with OR (`|`) → `true`

## Design Principles

1. **LTR Evaluation** — No precedence, sequential execution
2. **Explicit Grouping** — Parentheses only
3. **Composable** — Every operation returns value for next
4. **Point-Free** — No intermediate variable names
5. **Predictable** — Given input, always same output

## Related Concepts

- **Pipes** — Unix `|` operator pattern
- **Pipeline Pattern** — Composable transformations
- **Elixir `|>`** — Pipe operator
- **F# `|>`** — Forward pipe
- **JavaScript Pipeline Proposal** — `|>` operator
- **Functional Composition** — Combining functions
- **Monad Bind** — Sequential operations with context
