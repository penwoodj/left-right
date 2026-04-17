# Functional Programming — Code Golf & Terse Syntax

Documents demonstrating functional programming patterns, point-free style, and terse syntax design.

## Contents

- [`PenroScript.md`](../pipe-operators/PenroScript.md) - Complete TypeScript → PenroScript translations
- [`Map Programming Language Syntax Brainstorming.txt`](../data-pipeline-etl/Map%20Programming%20Language%20Syntax%20Brainstorming.txt) - Functional composition with real-world examples

## Key Topics

### Point-Free Programming
From [`PenroScript.md`](../pipe-operators/PenroScript.md):
- No variable declarations for intermediate results
- Function composition through operator chaining
- Left-to-right evaluation eliminates nested parentheses

### Higher-Order Functions (HOFs)
From [`01-chatgpt-designing-a-programming-language.md`](../language-design-comprehensive/01-chatgpt-designing-a-programming-language.md):

**Standard Library Functions:**
- `map` — Transform collections
- `filter` — Predicate-based selection
- `some` / `all` / `every` — Boolean aggregation
- `reduce` — Accumulation
- `uniq` — Deduplication
- `join` — String concatenation
- `flow` — Function composition

### Code Golf Patterns
From operator table:
- Single-character operators (`+`, `-`, `*`, `/`, `%`, `**`, `%%`)
- Symbolic predicate notation (`>2`, `<5` for "greater than 2", "less than 5")
- Prefix notation (`~?` for filter, `$_` for flatMap)
- Minimal brackets (implicit left argument in `{ $<[>2, <5] $+`& }`)

### Functional Composition Examples

#### Pipeline Composition
```javascript
// Lodash FP style
const getResult = flow(
  filter(pred1),
  map(transform),
  reduce(combiner)
)(data);
```

```javascript
// PenroScript style
data $pred1 $transform $combiner
```

#### Implicit Parameters
```javascript
// Explicit parameters
const add = (a, b) => a + b;
```

```javascript
// Left-hungry currying (default)
{ _< + 1 } // Curried: adds 1 to left argument
```

### Terse Syntax Techniques

1. **Symbolic Predicates** — `>2` instead of `(x) => x > 2`
2. **Directional Sections** — `_>`/`_<` for partial application
3. **Implicit Left Argument** — Omit `_>` in single-argument operators
4. **Operator Overloading** — Same symbol, different behavior by type
5. **Flat Precedence** — Left-to-right eliminates precedence concerns

### Real-World Integration
From [`Map Programming Language Syntax Brainstorming.txt`](../data-pipeline-etl/Map%20Programming%20Language%20Syntax%20Brainstorming.txt):

**ServiceNow Integration:**
```javascript
// Lodash FP import
import { map, flatMap, flow, get } from 'lodash/fp';

// Pipeline application
result >> getOr['tableQueryData', []] >> flatMap[[...] ==> {...}] >> compact >> uniq
```

**Conditional Operators:**
```javascript
// Boolean-driven selection
predicate & truthyValue | falseyValue
```

## Related Concepts

- **Code Golf** — Minimizing syntax length
- **Point-Free Programming** — Composing functions without naming intermediate results
- **Tacit Programming** — Implicit arguments through currying
- **Concatenative Languages** — Stack-based composition
- **Functional Programming** — Pure functions, immutability, composition
- **Information Density** — Expressiveness per character
