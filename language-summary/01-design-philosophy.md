# Design Philosophy — Left-Right

## LTR as Fundamental Principle

Left-to-right evaluation is not a syntax detail but the foundational execution model. Every operator evaluates left to right unless explicitly grouped by parentheses.

**Why LTR Matters:**
- Eliminates precedence confusion (no need to memorize operator precedence tables)
- Matches natural reading direction for left-to-right languages
- Creates linear, predictable execution flow
- Reduces cognitive overhead when parsing complex expressions

**Example:**
```penroscript
// Clear execution: (3 + 4) * 2 = 14
3 + 4 * 2

// No ambiguity about order of operations
```

**Contrast with RTL:**
Languages like APL/J evaluate right-to-left, which requires mental reversal when writing code. LTR aligns execution order with code layout, reducing context switching between reading and execution.

---

## Operators as First-Class Values

Operators are not syntax sugar for function calls—they are values themselves. Like text, numbers, or lists, operators can be:

- **Stored** in maps as values
- **Passed** as function arguments
- **Returned** from function calls
- **Composed** with other operators
- **Assigned** to variables

**Example:**
```penroscript
{
  // Store operator as value
  adder: +,

  // Pass operator as argument
  apply: { op: _<, a: _> },
  apply.op[a],  // Apply + to a

  // Return operator from function
  makeAdder: { x: _<, { _<, x + _> } }
}
```

This enables **higher-order operators** and **metaprogramming** capabilities unavailable in languages where operators are purely syntactic.

---

## Terse General-Purpose Scripting for Data Transformation

Left-Right is optimized as a general-purpose scripting language for:
- Data extraction and transformation
- Template generation and string manipulation
- Collection filtering and aggregation
- ETL (Extract-Transform-Load) pipelines

**Language Characteristics:**
- Minimal ceremony (no type declarations, explicit control flow)
- Operator chaining replaces verbose function calls
- JSON-like structure makes data transformations visible
- Text interpolation enables dynamic templates

**Example:**
```penroscript
// ETL pipeline: filter, map, join
{
  threats: [...]  // Input data
  result: threats
    ?{ @[] = 'high' }  // Filter
    ${ @[] "^_ }          // Transform
    ~                                      // Unique
    >< ', '                              // Join
}
```

The same operation in conventional languages requires multiple intermediate variables and explicit flow control.

---

## Determinism as Core Value

**Every operation is deterministic:**
- Given identical inputs, produces identical outputs
- No random evaluation or nondeterministic iteration order
- Consistent map key and list index ordering
- Reproducible behavior across runs

**Why Determinism Matters:**
- Predictable debugging (same bug, same behavior)
- Testable (fixtures produce stable results)
- Cacheable (pure functions enable memoization)
- Parallelizable (no shared state or race conditions)

**Enforced Determinism:**
- Collection iteration order is guaranteed (not hash-based)
- Operator behavior is type-dependent but consistent
- No implicit global state mutation
- Optional chaining rather than throwing errors

**Example:**
```penroscript
// Always produces same result for same input
{ data: _<,
  result: data
    ${ _< * 2 }
    ~
    #,
}
```

---

## JSON-Like Readability

The language uses JSON-like structure so programs "look like data, act like code."

**Benefits:**
- Familiar syntax for web developers (JS/TS, JSON, YAML)
- Easy to distinguish between data structures and operations
- Self-documenting (map keys describe operations)
- Low cognitive dissonance (configuration looks like configuration, code looks like code)

**Example:**
```penroscript
// Looks like JSON data
{
  step1: { extract: `user` },
  step2: step1.extract @[],
  step3: step2 ?/ `@company.com`,
  result: step3
}
```

This structure makes code immediately readable to developers familiar with JSON while containing executable logic.

---

## Backtick-Only Strings

A foundational design decision: strings use ONLY backticks, reserving single and double quotes for operator names.

**Why This Matters:**
- **Operator names as characters**: `"` (toLower), `"`_ (lowercase-first), `"^` (mixed case)
- **No ambiguity**: Backtick ` is reserved exclusively for string delimiters
- **Visual clarity**: Distinguishes string literals from operator symbols immediately
- **JS familiarity**: Matches JavaScript template literal syntax

**Examples:**
```penroscript
// Text literals use backticks ONLY
`hello`         // Valid text
`world`         // Valid text

// Quote characters are operators
`hello` "        // Convert to lowercase: "hello"
`world` ^        // Convert to uppercase: "WORLD"
```

Point-free style eliminates explicit variable names by chaining operations directly:

```penroscript
// Traditional style (with variables)
{
  x: 10,
  y: x * 2,
  z: y + 5,
  z
}

// Point-free style (no intermediate variables)
10 * 2 + 5
```

**Benefits:**
- Reduced cognitive load (fewer names to track)
- Linear flow (each operation feeds next)
- Concise without sacrificing readability
- Focuses on transformation, not variable management

**When to Use:**
- Simple transformations with clear data flow
- Chaining standard operations
- Pipeline-style processing

**When Not to Use:**
- Complex branching logic requiring named conditions
- Performance-critical sections where intermediate results benefit from explicit names
- Operations with multiple inputs requiring clear argument names

---

## No Explicit Control Flow

The language eliminates traditional control flow keywords (`if`, `for`, `while`, `switch`) in favor of:
- **Conditional operators** (`?|`, `$?`, `!`)
- **Collection operations** (`$`, `$_`, `$?`, `$+`)
- **Logical operators** (`&`, `|`, `!`)

**Rationale:**
- Every operation returns a value (no void expressions)
- Control flow emerges from expression evaluation
- Reduces syntactic surface area
- Encourages composable, pure functions

**Example:**
```penroscript
// Conditional without if keyword
{
  value: _<,
  result: value
    ?/ 0               // Divisible by 2?
    & `even`             // Yes: return `even`
    | `odd`             // No: return `odd`
}
```

This approach treats conditional logic as data transformation rather than control flow.

---

## Spatial/Compounding Symbology

Operators use spatial relationships between symbols to encode related operations:

### Text Case Operators

| Operator | Name | Example | Result |
|-----------|------|----------|----------|
| `^` | toUpper | `` `hello` ^ `` | `` `HELLO` `` |
| `^_` | capitalize | `` `hello` ^_ `` | `` `Hello` `` |
| `"` | toLower | `` `HELLO` " `` | `` `hello` `` |

**Note:** The backtick `` ` `` is the ONLY string syntax in Left-Right. Single and double quotes are reserved for operator names (e.g., `"` is the toLower operator, `"_" is the lowercase-first operator). This is a foundational design decision.

### Spatial Relationships

- `^` (caret) alone = uppercase (raise all characters)
- `^_` (caret + underscore) = capitalize (raise first character only)
- `"` (quote) alone = lowercase (lower all characters)

This **compounding** creates a symbology system where operators are:
- Memorizable through spatial patterns
- Extensible through combining symbols
- Typeable with standard keyboard characters
- Related visually to their function

**Additional Examples:**
- `><` (arrows outward) = join (combine collections)
- `<>` (arrows inward) = split (separate collection)
- `~` (tilde, often "not" or "approximate") = unique (remove duplicates)

---

## Type-Dependent Operator Behavior

The same operator changes meaning based on input type, enabling polymorphic behavior without type declarations.

**Example: `+` Operator**
```penroscript
// Numbers: addition
1 + 2              // 3

// Text: concatenation
`hello` + `world`  // `helloworld`

// Lists: concatenation
[1,2] + [3,4]     // [1,2,3,4]

// Maps: merge
{a:1} + {b:2}     // {a:1, b:2}
```

**Example: `@` Path Access**
```penroscript
// Text: property access
obj @[]      // obj.name

// List: index access
arr @[0]            // arr[0]

// Map: nested path access
data @[, , ]  // data.user.profile.email
```

This design:
- Reduces operator count (one symbol, multiple uses)
- Matches natural language intuition (same word, different meanings by context)
- Eliminates need for explicit type conversion
- Enables intuitive API design

---

## Left-Hungry Auto-Currying

Operators automatically curry from the left when applied to insufficient arguments:

```penroscript
// Binary operator applied to one argument becomes unary
{
  add2: + 2,        // + applied to 2, returns function waiting for right arg
  result: add2 3        // Equivalent to 2 + 3 = 5
}

// Directional sections
{
  // _< placeholder for left argument
  greaterThan10: _< > 10,

  // _> placeholder for right argument
  addTo10: _> + 10,

  greaterThan10 15,  // 15 > 10 = true
  addTo10 5,          // 5 + 10 = 15
}
```

**Benefits:**
- Partial application without explicit wrapper functions
- Natural left-to-right currying behavior
- Cleaner pipelines (operators curry as they flow)
- Reduced function boilerplate

---

## Inspiration Sources

### APL/J/K Influence

**From APL/J/K:**
- Inspired by array-oriented programming paradigm
- Concise operators for data transformation
- Type-dependent operator behavior
- Functional composition through chaining

**Adapted for Left-Right:**
- ASCII-friendly syntax (replacing special characters)
- Left-to-right evaluation (reversing APL's RTL)
- JSON-like structure (replacing matrix syntax)
- Web developer familiarity (JS/TS target)

### Haskell Influence

**From Haskell:**
- Point-free style through operator composition
- Currying and partial application
- Higher-order functions
- Pure function semantics

**Adapted for Left-Right:**
- Pragmatic trade-offs (loose typing, mutable options)
- Operators as first-class values (Haskell functions are first-class)
- JSON-like data structures (Haskell uses algebraic data types)

### Clojure Influence

**From Clojure:**
- Data-oriented programming
- Immutable-first design
- Rich collection operations
- Functional composition

**Adapted for Left-Right:**
- JSON-like syntax (replacing Lisp parentheses)
- Operators as first-class (Clojure functions are first-class)
- LTR evaluation (Clojure is also LTR)

### Lodash/FP Influence

**From lodash/FP:**
- Practical functional utilities
- Data transformation pipelines
- Collection iteration patterns
- Point-free style patterns

**Adapted for Left-Right:**
- Native operator syntax (replacing function calls)
- Built-in currying (lodash requires explicit `_.partial`)
- Type inference (lodash requires type checking)

---

## Semiotics Connection

The language draws from semiotic principles of how symbols convey meaning:

### Signifier and Signified

- **Signifier (Symbol):** The visual operator token (e.g., `^`, `+`, `@`)
- **Signified (Concept):** The operation or transformation the symbol represents

In Left-Right, the relationship is:
- **Iconic:** Symbol visually relates to operation (e.g., `^` as "up" = uppercase)
- **Indexical:** Symbol points to operation through convention (e.g., `+` as addition)
- **Symbolic:** Symbol arbitrary but consistent (e.g., `$` as map)

### Spatial Symbology

Operators use **spatial relationships** to encode meaning:
- Position relative to other symbols
- Directionality (arrows, carets)
- Compound symbols (base + modifier)

This creates a **learnable symbology** where new operators can be derived from existing patterns.

**Example:**
- Base: `^` (up/uppercase)
- Modifier: `_` (lower/first)
- Compound: `^_` (uppercase first = capitalize)

### Iconicity in Design

The language prioritizes **iconic symbols** where the visual representation relates to the operation:
- `><` (arrows outward) = join/combine
- `<>` (arrows inward) = split/separate
- `@` (at/location) = get/access
- `#` (number/count) = size

This reduces learning time and increases retention.

---

## JavaScript Familiarity Goal

The language is designed so JavaScript engineers find semantics intuitive, bridging the power of languages inspired by array-oriented languages (APL/J/K) with the familiarity of web development.

**Semantic Parallels:**
- **Text syntax**: Backticks match JS template literals exactly
- **Comparison**: `==` for strict type checking (like JS `===`), `=` for loose equality (like JS `==`)
- **Loose typing**: Runtime type inference similar to dynamic typing in JS
- **Data structures**: Maps/lists identical to JavaScript objects/arrays
- **Collection operations**: Map, filter, reduce patterns familiar to JS developers

**Design Rationale:**
- Reduces learning curve for the largest programming language community
- Enables smooth migration from JS/TS pipelines to Left-Right
- Maintains power and brevity of paradigms inspired by array-oriented languages
- Targets both JS and Rust transpilation natively

---

## Cognitive Load Analysis

LTR evaluation and JSON-like structure reduce cognitive overhead through:

### Linear Execution Flow

```penroscript
// Cognitive load: O(n) - linear reading
input $filter $map $reduce
```

Each operation follows the previous in reading order, matching cognitive processing.

### Consistent Syntax

All operations use the same structural patterns:
- Map/object syntax for data and operators
- List syntax for collections
- Same operator rules across all types

This reduces **syntactic switching cost** when moving between data types.

### Minimal Context Switching

```penroscript
// Low context switching: all operations visible at once
{
  step1: data $filter,
  step2: step1 $map,
  step3: step2 $join
}
```

Compare to nested syntax where operations are hidden inside parentheses or blocks.

### Reduced Working Memory

Point-free style eliminates need to track intermediate variables:
```penroscript
// Without tracking: x, y, z, result
input $filter $map $join
```

Cognitive research shows that reducing the number of mental variables lowers error rates and improves comprehension.

---

## The Aesthetic Dimension

The designer frames Left-Right as an **art practice**, where:

### Code as Aesthetic Object

Programs are not just functional artifacts but aesthetic objects with:
- Visual elegance through consistent syntax
- Rhythmic flow through linear evaluation
- Symbolic beauty through spatial relationships
- Minimalist philosophy through operator-first design

### Beauty in Simplicity

The aesthetic value emerges from:
- **Economy of expression:** Maximum meaning with minimum symbols
- **Predictable patterns:** Operators behave consistently
- **Harmonious integration:** All elements work together coherently

### Art Practice Aspects

1. **Intentionality:** Each symbol choice is deliberate
2. **Craftsmanship:** Refining operator set and syntax
3. **Expressiveness:** Enabling beautiful solutions to complex problems
4. **Evolution:** Language evolves through iterative refinement

This aesthetic dimension distinguishes Left-Right from purely utilitarian languages, positioning it as a creative medium for expression.

---

## Related Documentation

- [Language Overview](./00-language-overview.md) — Complete language overview and history
- [Type System](./02-type-system.md) — Type system documentation
- [Operator Reference](./03-operator-reference.md) — Comprehensive operator catalog
- [Master Index](./README.md) — Complete documentation suite
