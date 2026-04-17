# Operator Precedence — Theory & Flat Precedence

Documents covering operator precedence theory, evaluation order, and grouping rules.

## Contents

- [`Penscript_LeftRight brainstorm.md`](../language-design-comprehensive/Penscript_LeftRight%20brainstorm.md) — Section 6: Operators (Design & Extensibility)
- [`01-chatgpt-designing-a-programming-language.md`](../language-design-comprehensive/01-chatgpt-designing-a-programming-language.md) — Operator table and precedence discussion

## Key Topics

### Flat Precedence Design
From [`PenroScript.md`](../pipe-operators/PenroScript.md):

**Core Principle:**
```javascript
// No precedence, left-to-right evaluation only
3 + 4 * 2 // Evaluated as (3 + 4) * 2 = 14
// NOT: 3 + (4 * 2) = 11
```

**Grouping:**
```javascript
// Parentheses for explicit grouping
(3 + 4) * 2 // Force addition first
```

### Precedence Considerations
From brainstorm checklist:

**Why Ditch Precedence?**
- Simplifies parsing
- Reduces cognitive load (no precedence table to memorize)
- Matches left-to-right reading order
- Eliminates precedence bugs

**Alternative: Uniform Precedence**
- Some languages use uniform precedence (all operators equal)
- Others use no precedence (PenroScript approach)
- Flat precedence is simplest model

### Grouping Rules

#### Parentheses
```javascript
// Standard grouping
(result + offset) * multiplier
```

#### Braces for Blocks
```javascript
// Map/operator blocks
{
  a: 1,
  b: a + 1
}
```

#### Array Brackets
```javascript
// Indexing and grouping
items[@index]
```

### Operator Direction vs Precedence

**Directional Sections:**
```javascript
// `_<` and `_>` affect argument binding, not precedence
{ _< + 1 } // Left argument bound, not precedence control
```

**Evaluation Order:**
```javascript
// Still LTR even with directional sections
data $filter $transform // Filter first, then transform
```

### Edge Cases

#### Mixed Operations
```javascript
// Without precedence
3 + 4 * 2 - 1 // Evaluated LTR: ((3 + 4) * 2) - 1
```

#### Function Application
```javascript
// Application binds tighter than operators
func[arg1, arg2] + 1 // Apply func first, then add 1
```

### Precedence in Other Languages

**No Precedence:**
- **APL** — Right-to-left, no precedence
- **Forth** — Stack-based, no precedence
- **Smalltalk** — Uniform precedence

**Explicit Precedence:**
- **C-family** — Complex precedence tables
- **Java** — Well-defined precedence
- **Python** — Explicit precedence

**Uniform Precedence:**
- **Pascal** — Uniform precedence
- **Ada** — Uniform precedence
- **ML** — Uniform precedence

## Design Principles

1. **No Precedence** — Simplicity over traditional rules
2. **Explicit Grouping** — Parentheses only for overriding order
3. **LTR Evaluation** — Matches reading direction
4. **Predictable** — No hidden precedence rules
5. **Minimal Confusion** — Reduces cognitive load

## Related Concepts

- **Operator Precedence** — Evaluation order of mixed operators
- **Associativity** — Left vs right grouping
- **Parsing Theory** — How precedence is implemented
- **Flat Precedence** — All operators equal or none
- **Uniform Precedence** — All operators have same precedence
- **LR Parsing** — Left-to-right parsing without lookahead
