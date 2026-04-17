# Concatenative Languages — Stack-Based Composition

Documents related to stack-based languages and concatenative composition patterns.

## Key Topics

### Stack-Based Evaluation
From language design:
- No explicit variables or named bindings
- Values pushed to stack, operations consume from stack
- Composition through concatenation

### Concatenative Patterns

#### Operator Chaining
From [`PenroScript.md`](../pipe-operators/PenroScript.md):
```javascript
// Left-to-right operator pipeline
threats $?{ @['AI Confidence Level', 'value'] = 'malicious' } #,
```

#### Implicit Parameter Passing
```javascript
// No explicit argument names
{ _< + 1 } // Adds 1 to whatever is on the left
```

#### Curried Functions as Values
```javascript
// Functions are first-class values
{ _< $<[>2, <5] $+`& } // Callable operator definition
```

### Related Concepts

- **Forth** — Stack-based language
- **Factor** — Modern concatenative language
- **PostScript** — Page description language
- **Joy** — Functional concatenative language
- **Stack Machines** — Theoretical model
- **Concatenative Combinators** — Composition by concatenation
- **No Variables** — Pure stack manipulation

### PenroScript Alignment

While not purely concatenative, PenroScript incorporates:
- **Left-to-right evaluation** (similar to stack order)
- **Implicit arguments** (stack-like consumption)
- **Operator composition** (concatenative-style chaining)
- **First-class operators** (functions as values)

Note: PenroScript maintains named scopes via Map/JSON structure, unlike pure concatenative languages.
