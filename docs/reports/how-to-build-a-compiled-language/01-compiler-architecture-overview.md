# Compiler Architecture Overview

## Executive Summary

Research on modern compiler architecture for Left-Right (LR) — point-free, operator-based, array-oriented, loosely typed language with zero-precedence left-to-right curried evaluation. Analysis covers multi-pass vs single-pass patterns, APL-family compilation strategies, and complete pipeline design from source to target code. Recommendations: 4-stage multi-pass architecture with HIR/MIR/LIR tiers, incremental compilation with caching, optimization passes for array fusion and currying.

**Key Decisions:**
- Multi-pass architecture (4 passes: parsing → IR → optimization → codegen)
- Three-tier IR system (HIR → MIR → LIR)
- AST and IR caching for incremental compilation
- Left-associative parser for zero precedence

---

## 1. Modern Compiler Architecture Patterns

### Multi-pass vs Single-pass Compilers

#### Single-pass Compilers

**Characteristics:**
- Parse and generate code in one traversal
- Minimal memory footprint
- Fast compilation time
- Limited optimization opportunities

**Architecture:**
```
Source → Lexer → Parser (with codegen) → Target Code
```

**Use Cases:**
- Pascal (classic)
- Simple expression calculators
- Bootstrapping compilers

**Pros:**
- Simple to implement
- Low memory usage
- Fast for small projects

**Cons:**
- Cannot do forward reference resolution
- Limited optimization (no whole-program analysis)
- Poor error reporting (context lost)

**When NOT to use for LR:**
- LR's zero-precedence parsing requires lookahead
- Map-based functions need forward reference resolution
- Performance optimization demands multi-stage analysis

#### Multi-pass Compilers

**Characteristics:**
- Separate compilation phases
- Intermediate Representations (IRs) between phases
- Rich optimization opportunities
- Better error recovery and reporting

**Architecture:**
```
Source → Lexer → Parser → Semantic Analysis → HIR → MIR → LIR → CodeGen → Target Code
                          ↑               ↓                ↑
                       Type Inference   Optimization   Register Allocation
```

**Use Cases:**
- GCC, Clang, LLVM
- Modern JavaScript (V8, SpiderMonkey)
- Rust (rustc)

**Pros:**
- Forward reference resolution
- Sophisticated optimization passes
- Better error messages with full context
- Easier to add new targets (share IR)

**Cons:**
- Higher memory usage
- Slower compilation
- More complex architecture

**Why Multi-pass for LR:**

1. **Semantic Requirements:**
   - LR maps may reference functions defined later
   - Type inference benefits from whole-program view
   - Curried application needs resolution

2. **Optimization Requirements:**
   - Array fusion (a b c → fused operation)
   - Dead code elimination in curried chains
   - Inlining of small maps

3. **Target Independence:**
   - Single IR for both JavaScript and Rust codegen
   - Shared optimization passes
   - Easier to add future targets (C, WASM)

**Decision: Multi-pass with 4-stage architecture**

### Frontend → IR → Optimization → Code Generation Pipeline

#### Standard Compiler Pipeline

**Stage 1: Frontend (Analysis)**

```
┌─────────────────────────────────────────────────┐
│  FRONTEND                                      │
│  ┌──────────┐  ┌──────────┐  ┌──────────────┐   │
│  │ Lexer    │→ │ Parser   │→ │ Semantic     │   │
│  │ Tokens   │  │ AST      │  │ Analysis     │   │
│  └──────────┘  └──────────┘  │ - Type check │   │
│                              │ - Name bind  │   │
│                              │ - Scope res  │   │
│                              └──────────────┘   │
└─────────────────────────────────────────────────┘
```

**Lexer (Tokenization):**
- Input: Raw source text
- Output: Stream of tokens
- Responsibilities:
  - Identify operators (identifiers in LR)
  - Recognize literals (strings, numbers)
  - Track location for error messages
  - Handle string interpolation

**Parser (AST Generation):**
- Input: Token stream
- Output: Abstract Syntax Tree
- Responsibilities:
  - Enforce syntax rules
  - Build hierarchical structure
  - Operator chaining (LR: left-to-right, zero precedence)
  - Handle curried application syntax

**Semantic Analysis:**
- Input: AST
- Output: Decorated AST + Symbol Table
- Responsibilities:
  - Type inference/checking
  - Name resolution
  - Scope analysis
  - Map literal validation

**Stage 2: Middle-end (Optimization)**

```
┌─────────────────────────────────────────────────┐
│  MIDDLE-END                                     │
│  ┌──────────────┐  ┌──────────────┐            │
│  │ High-Level   │→ │ Optimization │→ Low-Level │
│  │ IR (HIR)     │  │ Passes       │  │ IR (LIR) │
│  └──────────────┘  │ - Const prop │  └─────────┘
│                    │ - Dead code  │
│                    │ - Inlining   │
│                    │ - Array fus. │
│                    └──────────────┘
└─────────────────────────────────────────────────┘
```

**High-Level IR (HIR):**
- Language-agnostic but high-level
- Preserves original structure
- Enables targeted optimizations
- Example: SSA form, abstract operations

**Optimization Passes:**
- Constant propagation
- Dead code elimination
- Common subexpression elimination
- Loop invariants
- Inlining
- Array fusion (critical for LR)
- Currying optimization

**Low-Level IR (LIR):**
- Target-oriented but still abstract
- Closer to machine model
- Virtual registers
- Control flow explicit

**Stage 3: Backend (Code Generation)**

```
┌─────────────────────────────────────────────────┐
│  BACKEND                                        │
│  ┌──────────────┐  ┌──────────────┐            │
│  │ Register     │→ │ Instruction  │→ Target    │
│  │ Allocation   │  │ Selection    │  │ Code     │
│  └──────────────┘  └──────────────┘  └─────────┘
└─────────────────────────────────────────────────┘
```

**Register Allocation:**
- Map virtual registers to physical resources
- Handle spilling
- Consider calling conventions

**Instruction Selection:**
- Choose optimal target instructions
- Pattern matching on LIR
- Peephole optimization

**Target Code Emission:**
- Generate actual code
- Emit debugging information
- Source maps

### JIT vs AOT Compilation

#### Ahead-Of-Time (AOT) Compilation

**Process:**
```
Source → Compile → Binary → Execute
```

**Characteristics:**
- Full optimization before execution
- No runtime compilation overhead
- Larger binary size
- Slower startup

**Examples:**
- C/C++ (gcc, clang)
- Go
- Rust
- C# (with NGEN)

**Pros:**
- Maximum optimization time budget
- No runtime pauses
- Better deployment predictability

**Cons:**
- No profile-guided optimization (PGO) without training runs
- Can't specialize based on runtime data
- Slower iteration in development

#### Just-In-Time (JIT) Compilation

**Process:**
```
Source → Compile (quick) → Execute
             ↓
         Runtime Profiling
             ↓
         Recompile (optimized)
```

**Characteristics:**
- Compile during execution
- Use runtime profiling for optimization
- Smaller initial code
- Compilation pauses during execution

**Examples:**
- JavaScript (V8, SpiderMonkey)
- Java (HotSpot)
- LuaJIT
- PyPy

**Pros:**
- Profile-guided optimization built-in
- Runtime specialization
- Faster startup (tiered JIT)

**Cons:**
- Compilation pauses
- More complex runtime
- Harder to debug

#### LR Compiler Strategy

**Decision: AOT with Incremental Recompilation**

**Rationale:**

1. **Transpilation Requirement:**
   - Target languages (JS, Rust) don't support LR runtime JIT
   - Must emit source-level code, not machine code

2. **Performance Goals:**
   - Maximize generated code speed
   - No runtime compilation pauses
   - Predictable execution

3. **Development Experience:**
   - Use watch mode for incremental compilation
   - Profile data collection for optimization
   - Separate "debug" and "release" builds

**Architecture:**
```
Dev Mode:          Source → Quick Compile → Target (no opt)
                          ↓
                    Incremental Updates

Release Mode:      Source → Full Opt → Target
                          ↓
                    Profile-Guided (optional)
```

### Incremental Compilation

#### What is Incremental Compilation?

Selective recompilation of only changed parts of a codebase, avoiding full rebuilds.

#### Incremental Compilation Strategies

**1. Fine-Grained Dependency Tracking**

Track dependencies at expression level:

```
File A:
  foo = { x: x + 1 }
  bar = foo 2

File B:
  baz = A.bar

Dependency Graph:
  A.foo → A.bar → B.baz
```

When `A.foo` changes, recompile `A.foo`, `A.bar`, `B.baz`.

**2. AST Caching**

Cache parsed ASTs:
```
Source File
    ↓
[ Cached AST? ] --No→ Parse → Cache
    ↓ Yes
Modified? --Yes→ Parse → Update Cache
    ↓ No
Use Cached AST
```

**3. IR Caching**

Cache optimized IR:
```
AST → HIR [Cache] → MIR [Cache] → LIR
```

When upstream changes, recompute from that stage.

**4. Memoization**

Memoize expensive operations:
```
typeInference(expr) {
  if (cache.has(expr.hash)) return cache.get(expr.hash);
  result = computeType(expr);
  cache.set(expr.hash, result);
  return result;
}
```

#### LR Incremental Strategy

**Caching Layers:**

1. **AST Cache:**
   - Cache per file
   - Invalidation: file modification time
   - Store: memory + disk (for IDE support)

2. **HIR Cache:**
   - Cache per module
   - Invalidation: AST changes
   - Reuse across target backends

3. **Optimization Cache:**
   - Cache optimization results
   - Invalidation: upstream changes or config changes

**Dependency Tracking:**

```
Module Dependency Graph:
  main.lr → utils.lr → math.lr
           ↓
          data.lr
```

**Incremental Recompile Process:**

1. Detect changed file (watch mode or manual)
2. Load cached AST (if valid)
3. Parse only if invalid
4. Re-semantic-analysis module
5. Re-optimize module
6. Re-codegen changed files
7. Update dependency graph

**Configuration:**
```toml
[compiler]
incremental = true
cache_dir = ".lr-cache"
watch_mode = false

[cache]
ast = true
hir = true
mir = true
max_cache_size = "500MB"
```

---

## 2. Architecture for Operator-Based Languages

### APL-Family Language Compilation Strategies

#### APL Compilation Overview

**Key Characteristics:**
- Right-to-left evaluation (LR: left-to-right)
- High-dimensional arrays
- Monadic and dyadic operators
- Tacit (point-free) programming
- No explicit loops

**Modern APL Compilers:**

**1. Dyalog APL**
- Hybrid interpreter + compiler
- JIT compilation to machine code
- Vectorization optimization
- Array fusion

**2. BQN (dzaima/BQN)**
- Compiler to C++ (via CBQN)
- Array fusion
- Constant folding
- Dead code elimination

**3. K (KDB+ / q)**
- JIT compilation
- Query optimization
- Vectorization
- Memory pooling

#### Common Patterns in APL Compilers

**1. Expression Flattening**

Convert nested expressions to flat operations:

```
Source:  (a + b) × (c - d)
Tree:    ×
         / \
        +   -
       / \ / \
      a  b c  d

Flattened:  t1 = a + b
             t2 = c - d
             t3 = t1 × t2
```

**2. Array Fusion**

Combine consecutive array operations:

```
Source:  (a + b) * (c + d)
Naive:   t1 = a + b  (allocate array)
         t2 = c + d  (allocate array)
         t3 = t1 * t2 (allocate array)

Fused:   t3 = map(a, b, c, d): (x + y) * (z + w)
         (one allocation)
```

**3. Rank Inference**

Determine result shapes statically:

```
a: shape (5, 3)
b: shape (5,)
c: a + b  → shape (5, 3) (broadcast)
```

Compile-time rank checking eliminates runtime errors.

**4. Monadic/Dyadic Dispatch**

Generate specialized code for each operator variant:

```
+: monadic  → identity
+: dyadic   → addition
```

**5. Tacit Expansion**

Convert tacit (point-free) to explicit:

```
Source:  +/
Mean:    sum over
Tacit:   +
        /
Explicit: sum arr = / + arr
```

### Point-Free/Tacit Programming Compilation Challenges

#### Challenge 1: Name Resolution Without Arguments

**Problem:**
```
sum = +/
mean = sum % #
```

In `sum % #`, what is the argument?

**Solution:**
1. **Implicit argument insertion:**
   ```
   mean = sum % #
   mean x = (sum x) % (# x)
   ```

2. **Curried composition:**
   ```
   mean = (∑) ÷ (≢)
   ```

3. **Compiler pass:**
   - Identify tacit expressions
   - Insert implicit arguments
   - Generate composition nodes

**IR Representation:**
```
TACIT:
  sum = +/
  mean = sum % #

HIR:
  sum = lambda x: reduce(add, x)
  mean = lambda x: (sum x) / (length x)
```

#### Challenge 2: Partial Application Order

**Problem:**
```
add = { a: a + 1 }
increment = add
```

What does `increment` mean?
- `add` itself (the map)
- `add` applied to nothing (partial application)
- `add` applied to implicit argument?

**Solution:**
- Explicit partial application syntax
- Compiler flag for implicit application
- Type-directed disambiguation

**LR Approach:**
```
add = { x: x + 1 }
inc = add          # Reference to map
inc = add 1        # Partial application: { x: (add x) 1 }
```

#### Challenge 3: Operator Composition

**Problem:**
```
compose = { f g x: f (g x) }
incThenDouble = compose (add 1) (multiply 2)
```

**Optimization:**
1. **Inlining:**
   ```
   incThenDouble x = (add 1) ((multiply 2) x)
   incThenDouble x = ((add 1) ((multiply 2) x))  # Simplify
   incThenDouble x = (x + 1) * 2
   ```

2. **Fusion:**
   - Combine `add` and `multiply` into single operation
   - Avoid intermediate allocations

3. **Specialization:**
   - Generate specialized code for known argument types
   - Example: array specialization

**Implementation:**
```
Pass 1: Expand composition
Pass 2: Inline small functions
Pass 3: Constant fold
Pass 4: Array fusion
```

### Curried Evaluation Compilation Strategies

#### What is Currying?

Transforming a function that takes multiple arguments into a sequence of functions that each take a single argument:

```
add a b
↓ Curried
((add a) b)
```

#### Currying Challenges

**1. Intermediate Closure Allocation:**

```
add 1 2 3

Naive compilation:
  t1 = add 1           # Returns closure
  t2 = t1 2           # Returns closure
  t3 = t2 3           # Final result
```

Each partial application allocates a closure.

**2. Performance Impact:**
- Allocation overhead
- Indirect function calls
- Poor cache locality

#### Currying Optimization Strategies

**Strategy 1: Partial Evaluation (Specialization)**

```
add = { a b: a + b }

Application: add 1 2

Analyze: add is fully applied with constant 1
Specialize:
  add_1 = { b: 1 + b }
  add_1 2  → 3
```

**Strategy 2: Currying Flattening**

Detect fully applied chains and flatten:

```
add 1 2

Directly compile as:
  t1 = 1 + 2
```

**Strategy 3: Closure Elimination**

Inline partial applications:

```
add 1 2 3

Detect: 3 applications of add
Flatten:
  ((add 1) 2) 3
  ↓
  ((1 + _) 2) 3
  ↓
  (1 + 2) 3
  ↓
  3 3
```

**Strategy 4: Currying Suspension**

Suspend currying until runtime for dynamic values:

```
apply func args = fold (apply f) func args

Compiles to loop, not nested calls:
  result = func
  for arg in args:
    result = result arg
```

#### LR Currying Strategy

**Left-to-Right Evaluation:**

```
a b c  →  ((a b) c)
```

**Compilation Pipeline:**

**Pass 1: Currying Analysis**
- Identify curried chains
- Mark fully-applied expressions
- Detect partial applications

**Pass 2: Closure Allocation Optimization**
- Eliminate closures for fully-applied chains
- Reuse closures for repeated partial applications

**Pass 3: Flattening**
- Flatten fully-applied chains
- Generate direct operations

**Example:**
```
Source:  add 1 2 3

AST:
  Apply
  ├── Apply
  │   ├── Apply
  │   │   ├── add
  │   │   └── 1
  │   └── 2
  └── 3

After Flattening:
  AddChain
  ├── add
  ├── 1
  ├── 2
  └── 3

Optimized:
  ((1 + 2) + 3)
```

### Zero-Precedence Parsing Implications

#### What is Zero Precedence?

All operators have equal precedence. Evaluation order determined solely by left-to-right application.

```
1 + 2 * 3
↓ Zero precedence (left-to-right)
((1 + 2) * 3)  # NOT (1 + (2 * 3))
```

#### Parsing Challenges

**1. No Operator Precedence Table**

Traditional parsers use precedence:

```
*: 7
+: 6
=: 2
```

Zero precedence: all same level.

**2. Simpler Grammar, Different Semantics**

```
Expression → Term (Apply Term)*
```

But semantics different from traditional languages.

**3. Ambiguity in Grouping**

Without parentheses, left-to-right is the only rule.

#### Parsing Strategy

**Recursive Descent with Left-Associative Parsing:**

```
parseExpression():
  left = parseTerm()
  while (peek() is identifier):
    op = consume()
    right = parseTerm()
    left = Apply(left, op, right)
  return left
```

**IR Generation:**

```
Source:  a + b * c

Tokens:  a, +, b, *, c

AST:
      *
     / \
    +   c
   / \
  a   b

LIR:
  t1 = a + b
  t2 = t1 * c
```

#### Optimization Implications

**1. Reassociation Opportunity:**

Since user can't control grouping, compiler can reassociate for performance:

```
1 + 2 + 3 + 4
↓ Parse left-to-right
(((1 + 2) + 3) + 4)
↓ Reassociate (commutative)
(1 + 2 + 3 + 4)  # Could parallelize
```

**2. Constant Folding:**

```
1 + 2 * 3
↓ Evaluate left-to-right
((1 + 2) * 3) = 9
```

**3. Array Fusion:**

```
(a + b) * (c + d)
↓ Parse left-to-right
(((a + b) *) (c + d))
↓ Fusion opportunity
fused_map(a, b, c, d): (x + y) * (z + w)
```

#### LR Parsing Architecture

**Lexer:**
- Identify operators as identifiers
- Recognize literals
- Handle string interpolation

**Parser:**
- Left-associative application
- No precedence levels
- Simple grammar

**AST:**
```rust
enum Expr {
    Identifier(String),
    Literal(Literal),
    Apply(Box<Expr>, Box<Expr>),  // Left-associative application
}
```

**Example:**
```
a b c

Parse:
  term: a
  apply: a b
  apply: (a b) c

AST:
  Apply(
    Box::new(Apply(
      Box::new(Identifier("a")),
      Box::new(Identifier("b"))
    )),
    Box::new(Identifier("c"))
  )
```

---

## 3. Compiler Pipeline Design

### Source → Tokens → AST → IR → Optimized IR → Target Code

#### Complete Pipeline Flow

```
Source.lr → Lexer → Tokens → Parser → AST → Type Inference → Symbol Table
                                                                   ↓
                                                               HIR Gen
                                                                   ↓
                                                            High-Level IR
                                                                   ↓
                                                          Optimization Passes
                                                                   ↓
                                                              MIR Gen
                                                                   ↓
                                                           Mid-Level IR
                                                                   ↓
                                                              LIR Gen
                                                                   ↓
                                                           Low-Level IR
                                                                   ↓
                                                          Target Selection
                                                         /              \
                                                   JS CodeGen         Rust CodeGen
                                                         ↓                   ↓
                                                     target.js           target.rs
```

#### Stage 1: Lexical Analysis

**Input:** Raw source code

**Output:** Stream of tokens

**Token Types:**
- Identifier (includes ALL operators — `+`, `@`, `><`, `!!!`, etc.)
- NumberLiteral (decimal only)
- StringLiteral (with interpolation: `{expr}`, not `${expr}`)
- MapStart (`{`)
- MapEnd (`}`)
- ListStart (`[`)
- ListEnd (`]`)
- Comma (`,`)
- Colon (`:`)
- Dot (`.`) — reverse-args operator
- LeftArg (`_<`) — 2-char token
- RightArg (`_>`) — 2-char token
- Quote (`'`) — reserved, unused
- EndOfLine / EndOfFile

**Important:** All operators are `Identifier` tokens, not a separate `Operator` token kind. Maximal munch rule applies: `!!!?`, `///`, `\\\`, `><`, `$@` are each a single identifier.

**Dot operator (`.`):** This is the reverse-args operator, NOT property access like JavaScript. It takes an unexecuted operator on its LEFT and returns a new unexecuted operator with left/right slots SWAPPED. Example: `key`@.data means `key` string → `@` (curried get) → `.` reverses → data flows in from left as the map. Everything still evaluates left-to-right.

**Special tokens:**
- `_: expr` = execute silently, discard result, return undefined (two tokens: `_` + `:`)
- `+: expr` = spread/merge: add all key-values from right to left map (two tokens: `+` + `:`)
- These are NOT composite tokens — always two separate tokens

**String Interpolation Handling:**

```
Source:  `Hello {name}, score is {score}`

Tokens:
  StringStart("`")
  StringLiteral("Hello ")
  InterpolationStart("{")
  Identifier("name")
  InterpolationEnd("}")
  StringLiteral(", score is ")
  InterpolationStart("{")
  Identifier("score")
  InterpolationEnd("}")
  StringEnd("`")
```

Note: Left-Right uses curly-brace-only interpolation `{expr}` — no dollar sign prefix like `${expr}`.

**Error Handling:**
- Unterminated strings
- Invalid escape sequences
- Unexpected characters

**Source Location Tracking:**
```rust
struct Span {
    start: usize,
    end: usize,
    line: usize,
    column: usize,
    file: PathBuf,
}

struct Token {
    kind: TokenKind,
    span: Span,
}
```

#### Stage 2: Parsing

**Input:** Token stream

**Output:** Abstract Syntax Tree (AST)

**Grammar:**
```
Program       → Expression  // Every .lr file is one root expression
Module        → Expression*

Expression    → Term (Apply Term)*
Apply         → (implicit whitespace)
Term          → Literal | Identifier | Map | List | Grouped

Map           → `{` MapBody? `}` `}@&` ExportList?  // Export pattern at end
MapBody       → Pair (`,` Pair)*
Pair          → Expression `:` Expression
List          → `[` Expression? (`,` Expression)* `]`
Grouped       → `(` Expression `)`

Literal       → NumberLiteral | StringLiteral
ExportList    → `[` StringLiteral (`,` StringLiteral)* `]`
```

**Note:** Comments use triple-backtick ` ``` ` prefix at line start. Lexer must distinguish triple-backtick at line start (comment) from single backtick (string delimiter).

**AST Structure:**
```rust
enum Expr {
    Number(f64),
    String(String),  // backtick string, possibly with interpolation
    Boolean(bool),
    Undefined,
    Identifier(String),  // includes ALL operators
    List(Vec<Expr>),
    Map(Vec<(Expr, Expr)>),  // key-value pairs, handles functions/conditionals
    Apply { func: Expr, arg: Expr },  // left-to-right curried application
    Grouped(Expr),  // parenthesized expression
    LeftArg,  // _< token (2-char token)
    RightArg, // _> token (2-char token)
}
```

**Note on control flow:** Left-Right has no `If`, `Function`, `Loop`, `Break`, `Continue`, or `Let` AST nodes. Maps handle these:
- Maps with `_<` references = functions (unexecuted operators at runtime)
- Maps with expression keys = conditionals
- Iteration operators (`$`, `$@`, `$?`, `$_`) are identifiers, not AST nodes

**Parse Example:**

```
Source:  add: { x y: x + y }
         result: add 1 2

Tokens:
  Identifier("add"), Colon,
  MapStart, Identifier("x"), Identifier("y"), Colon,
  Identifier("x"), Identifier("+"), Identifier("y"), MapEnd,
  Identifier("result"), Colon,
  Identifier("add"), Identifier("1"), Identifier("2")

AST:
  Expression: Apply(
    Apply(Identifier("result"), Identifier(":")),
    Apply(
      Apply(Identifier("add"), Identifier(":")),
      Map([
        (Identifier("x"), Identifier("x")),
        (Identifier("y"), Identifier("y")),
        (Identifier(""), Apply(
          Apply(Identifier("x"), Identifier("+")),
          Identifier("y")
        ))
      ])
    )
  )
```

**Note:** `:` is assignment in map context (alpha-start keys), not `=`. `=` is equality operator.

**Error Recovery:**
- Skip to next statement on error
- Continue parsing
- Report multiple errors

#### Stage 3: Semantic Analysis

**Input:** AST

**Output:** Decorated AST + Symbol Table

**Phases:**

**Phase 1: Name Binding**
- Resolve identifiers to definitions
- Build scope hierarchy
- Detect undefined references

**Phase 2: Type Inference**
- Infer types for all expressions
- Check type consistency
- Handle loose typing (union types)

**Phase 3: Validation**
- Check map literal validity
- Validate curried applications
- Detect circular references

**Symbol Table Structure:**
```rust
struct SymbolTable {
    modules: HashMap<String, ModuleScope>,
    current_scope: Vec<ScopeId>,
}

struct ModuleScope {
    definitions: HashMap<String, DefinitionInfo>,
    imports: Vec<String>,
}

struct DefinitionInfo {
    name: String,
    type_: Type,
    span: Span,
    is_recursive: bool,
}
```

**Type System:**
```rust
enum Type {
    Operator,      // Function/map (inferred at runtime)
    Map(Vec<Type>), // Map with specific types
    List(Type),    // List of type
    String,
    Boolean,
    Number,        // Decimal only — no hex, binary, octal, scientific notation
    Undefined,
    Union(Vec<Type>),
}
```

**Note:** Left-Right has no type annotations, type signatures, or type declarations. All types inferred at runtime. No IEEE 754 details, NaN, Infinity, hex (0x), binary (0b), octal (0o), or scientific (1e10) notation. Floats must start with digit: `0.5` valid, `.5` invalid. No negative literals — `-` is binary operator: `0-5` for negative 5.

**Type Inference Example:**

```
add = { x y: x + y }
    → Type: Map([Number, Number] → Number)

result = add 1 2
    → add: Map([Number, Number] → Number)
    → 1: Number
    → 2: Number
    → result: Number
```

**Error Propagation:**

Each stage accumulates errors:

```rust
struct ErrorReporter {
    errors: Vec<Error>,
    warnings: Vec<Warning>,
}

impl ErrorReporter {
    fn report_error(&mut self, error: Error) {
        self.errors.push(error);
    }

    fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}
```

**Imports and Exports:**

Left-Right has no `import` or `export` keywords. Instead:

- **Imports:** `imports` is a runtime variable (a map). Import syntax:
  ```lr
  imports@[`lodash`, `fp`, `map`]  // @ get operator with list of strings
  files@[`path/to/file.lr`]         // Local file imports
  ```
  No static import analysis possible. Circular dependencies handled at runtime.

- **Exports:** End-of-file pattern `}@&[...]`:
  - `}` = closing brace of the root map
  - `@` = get operator
  - `&` = pick operator
  - `[...]` = list of string keys to export

  Parser recognizes this as a specific end-of-file pattern. No `export` keyword.

#### Stage 4: High-Level IR (HIR)

**Purpose:**
- Language-agnostic intermediate representation
- Preserve high-level structure
- Enable targeted optimizations

**HIR Design:**
```rust
enum HIR {
    Number(f64),
    String(String),  // with interpolation
    Boolean(bool),
    Undefined,
    Identifier(String, SymbolId),
    List(Vec<HIR>, Type),
    Map(Vec<(HIR, HIR)>, Type),
    Apply(Box<HIR>, Box<HIR>, Type),
    LeftArg,   // _< token
    RightArg,  // _> token

    // Optimized forms
    BinaryOp(BinOp, Box<HIR>, Box<HIR>),
    UnaryOp(UnOp, Box<HIR>),
    CurriedCall(String, Vec<HIR>),
}
```

**Note:** Left-Right has no `If`, `Let`, `Loop`, `Break`, `Continue`, `Function` HIR nodes. Maps with expression keys handle conditionals. Maps with `_<` references handle functions. Iteration operators are identifiers.

**AST → HIR Example:**

```
AST: Apply(Apply(Identifier("add"), Identifier("1")), Identifier("2"))

HIR: CurriedCall("add", [Literal(Number(1)), Literal(Number(2))])
```

**Optimization Opportunity:**
- Curried calls can be specialized
- Direct function calls can be inlined

#### Stage 5: Optimization Passes

**Pass 1: Constant Folding**
```rust
fn constant_fold(expr: &HIR) -> HIR {
    match expr {
        BinaryOp(Add, a, b) if is_const(a) && is_const(b) => {
            Literal(eval_add(a, b))
        }
        _ => expr.clone(),
    }
}
```

**Pass 2: Dead Code Elimination**
```rust
fn dead_code_elim(expr: &HIR) -> HIR {
    match expr {
        Map(pairs) => {
            // Remove unused map entries (alpha-start keys)
            let filtered = pairs.into_iter()
                .filter(|(key, _)| is_used(key) || !is_alpha_start(key))
                .collect();
            Map(filtered)
        }
        _ => expr.clone(),
    }
}
```

**Pass 3: Inlining**
```rust
fn inline(expr: &HIR, symbol_table: &SymbolTable) -> HIR {
    match expr {
        Apply(func, args) if is_small_map(func) => {
            let body = get_map_body(func);
            substitute(body, args)  // Inline map with _< references
        }
        _ => expr.clone(),
    }
}
```

**Pass 4: Array Fusion**
```rust
fn array_fusion(expr: &HIR) -> HIR {
    match expr {
        Apply(Apply(map1, arr1), Apply(map2, arr2))
            if is_same_array(arr1, arr2) => {
            FusedMap([map1, map2], arr1)
        }
        _ => expr.clone(),
    }
}
```

**Pass 5: Currying Optimization**
```rust
fn optimize_currying(expr: &HIR) -> HIR {
    match expr {
        CurriedCall(func, args) if is_fully_applied(func, args) => {
            DirectCall(func, args)
        }
        _ => expr.clone(),
    }
}
```

#### Stage 6: Mid-Level IR (MIR)

**Purpose:**
- Lower-level representation
- Control flow explicit
- Ready for target-specific optimization

**MIR Design:**
```rust
enum MIR {
    Number(f64),
    String(String),
    Boolean(bool),
    Undefined,
    Var(String),
    Let(String, Box<MIR>, Box<MIR>),  // Map assignment via :

    // Operations
    BinaryOp(BinOp, Box<MIR>, Box<MIR>),
    Call(String, Vec<MIR>),
    Closure(String, Vec<String>, Box<MIR>),

    // Control flow via maps and operators
    Jump(Label),
    Label(Label),
}
```

**Note:** Left-Right has no `If`, `Loop`, `Break`, `Continue` MIR nodes. Control flow via maps with expression keys and iteration operators (`$`, `$@`, `$?`, `$_`). Error handling via `!!!` (throw) and `!!!?` (catch) identifiers. Async via `///` and `\\\` identifiers.

**HIR → MIR Example:**

```
HIR: CurriedCall("add", [Literal(1), Literal(2)])

MIR: Call("add", [Literal(1), Literal(2)])
```

#### Stage 7: Low-Level IR (LIR)

**Purpose:**
- Target-oriented
- Virtual registers
- Ready for code generation

**LIR Design:**
```rust
enum LIR {
    Literal(Literal),
    VirtualReg(RegId),

    // Operations
    Add(VirtualReg, VirtualReg),
    Sub(VirtualReg, VirtualReg),
    Mul(VirtualReg, VirtualReg),
    Div(VirtualReg, VirtualReg),

    // Control flow
    If(VirtualReg, Label, Label),
    Jump(Label),
    Label(Label),

    // Calls
    Call(String, Vec<VirtualReg>, VirtualReg),
    Return(VirtualReg),
}
```

**MIR → LIR Example:**

```
MIR: Let("x", Box::new(Literal(1)), Box::new(Literal(2)))

LIR:
  x = AllocReg()
  t1 = Literal(1)
  Store(x, t1)
  t2 = Literal(2)
  Return(t2)
```

#### Stage 8: Code Generation

**Target Selection:**
```rust
enum Target {
    JavaScript,
    Rust,
}
```

**JavaScript CodeGen:**

```
LIR → JavaScript

Example:
  LIR: Add(v1, v2)
  JS:  v1 + v2

  LIR: Call("add", [v1, v2], v3)
  JS:  const v3 = add(v1, v2)
```

**Rust CodeGen:**

```
LIR → Rust

Example:
  LIR: Add(v1, v2)
  Rust: v1 + v2

  LIR: Call("add", [v1, v2], v3)
  Rust: let v3 = add(v1, v2)
```

### Error Propagation and Source Map Generation

#### Error Accumulation Strategy

**Per-Stage Error Collection:**

```rust
struct CompilationStage {
    name: String,
    errors: Vec<Error>,
    warnings: Vec<Warning>,
}
```

**Error Types:**

```rust
enum ErrorKind {
    Lexical,      // Lexer errors
    Syntax,       // Parser errors
    Semantic,     // Type errors, name errors
    Optimization, // Optimization warnings
    Codegen,      // Target-specific errors
}

struct Error {
    kind: ErrorKind,
    message: String,
    span: Span,
    stage: String,
}
```

#### Error Propagation Flow

```
Lexer → Parser → Semantic Analysis → HIR → MIR → LIR → CodeGen
  ↓         ↓              ↓              ↓    ↓    ↓     ↓
  Errors   Errors        Errors        Warnings  (warnings only at later stages)
```

**Early Termination:**
- Stop after stage with errors
- Report all errors in that stage
- Don't proceed to next stage

**Error Reporting:**

```
error[E001]: undefined identifier
  → file.lr:5:10
   |
 5 | result: undefined_func 1
   |          ^^^^^^^^^^^^^ undefined identifier
   |
help: Did you mean `add`?
```

#### Source Map Generation

**Purpose:** Map generated code back to original source for debugging.

**Source Map Structure:**

```rust
struct SourceMap {
    mappings: Vec<Mapping>,
}

struct Mapping {
    generated: Position,
    original: Position,
    name: Option<String>,
}

struct Position {
    line: usize,
    column: usize,
}
```

**Generation Process:**

```
Source:
  add = { x y: x + y }

Generated (JavaScript):
  function add(x, y) {
      return x + y;
  }

Mappings:
  Line 1, Col 0 → Line 1, Col 0 (function)
  Line 1, Col 0 → Line 1, Col 9 (add)
  Line 1, Col 6 → Line 2, Col 4 (x)
  Line 1, Col 8 → Line 2, Col 7 (y)
  Line 1, Col 12 → Line 3, Col 4 (return)
  Line 1, Col 14 → Line 3, Col 11 (x)
  Line 1, Col 16 → Line 3, Col 13 (+)
  Line 1, Col 18 → Line 3, Col 15 (y)
```

**Source Map Format:**
- Use standard source map format (JSON)
- Compatible with source-map library
- Enables debugging in browser/devtools

---

### Pipeline Summary

**Data Structures by Stage:**

| Stage | Input | Output | Data Structures |
|-------|-------|--------|-----------------|
| Lexer | Source text | Tokens | `Token`, `Span` |
| Parser | Tokens | AST | `Expr`, `Definition`, `Module` |
| Semantic | AST | Decorated AST | `SymbolTable`, `Type` |
| HIR Gen | AST | HIR | `HIR` nodes |
| Optimization | HIR | Optimized HIR | Inlined, fused nodes |
| MIR Gen | HIR | MIR | `MIR` with explicit control flow |
| LIR Gen | MIR | LIR | `VirtualReg`, `Label` |
| CodeGen | LIR | Target code | Source mappings |

**Error Handling:**
- Per-stage error accumulation
- Early termination on errors
- Detailed error messages with source locations

**Incremental Compilation:**
- AST cache per file
- HIR cache per module
- Dependency tracking
- Invalidation on change

--- END OF PART A ---


# Compiler Architecture — Part B

## 4. Performance-Oriented Architecture

Compilers face two distinct performance challenges: compilation speed (how fast the compiler runs) and generated code speed (how fast the resulting programs execute). These pull in opposite directions. More aggressive optimizations produce better runtime performance but require longer compile times.

For a language like Left-Right targeting both JavaScript and native code, we need balanced tradeoffs. JavaScript code can't be too aggressively optimized by the compiler, since the JIT compiler will do its own optimizations. Native code benefits more from compile-time analysis.

### Compilation Speed vs Generated Code Speed

A three-tier optimization strategy works well:

**Tier 1 (Fast compile, decent output):** Basic type checking, dead code elimination, constant folding. This is the default for development builds. The compiler finishes in milliseconds, ideal for rapid feedback loops.

**Tier 2 (Moderate compile, good output):** Inlining, loop unrolling, allocation sinking, escape analysis. Suitable for production builds of libraries and tools where startup time matters.

**Tier 3 (Slow compile, best output):** Whole-program analysis, profile-guided optimization, aggressive vectorization. For performance-critical applications where build time is acceptable.

Left-Right's point-free nature gives us some advantages. Every operator application is explicit, making the syntax tree clean and predictable. This reduces the complexity of some analysis passes. We also benefit from curried evaluation, which makes the data flow graph explicit and easier to analyze.

The key insight is that not all code needs Tier 3 optimization. Most business logic can ship with Tier 2. Reserve Tier 3 for hot paths in performance-sensitive domains like data processing or game engines.

### Parallel Compilation Opportunities

Compiler workloads are inherently parallelizable at multiple levels:

**Module-level parallelism:** Left-Right programs naturally decompose into modules. Each module can be parsed, type-checked, and optimized independently. Only code generation and linking require coordination.

**Pass-level parallelism:** Within a module, some optimization passes can run on disjoint subsets of the IR. For example, function inlining can happen on different functions concurrently if they don't reference each other.

**Phased parallelism:** The traditional compiler pipeline (parse → type check → optimize → generate) can overlap phases. While type checking one module, we can be optimizing another that was already type-checked.

A practical approach for Left-Right:

1. Parse all modules in parallel using Rayon or a similar thread pool
2. Perform type checking in parallel, with cross-module type references resolved lazily
3. Optimization passes run per-module, with a final interprocedural optimization pass if whole-program optimization is enabled
4. Code generation runs in parallel per module, with a final linking phase

The limiting factors are memory bandwidth (loading source code and IR) and synchronization costs. Rayon's work-stealing scheduler handles load balancing well for compiler workloads.

### Memory-Efficient Compiler Data Structures

Compilers allocate and discard vast amounts of temporary data. Poor allocation patterns can dominate compilation time.

**Arena allocation:** Instead of allocating AST nodes, type information, and IR values individually, allocate them in large arenas that are freed in bulk. This reduces allocator overhead and improves cache locality. In Rust, the `bumpalo` crate provides a simple arena allocator.

**Interned strings:** Symbol names, identifiers, and string literals should be interned. String interning deduplicates identical strings, reducing memory usage and enabling pointer equality checks for symbol comparison. This also speeds up hashing when building symbol tables.

**Persistent data structures:** For intermediate representations, consider using persistent (functional) data structures. These enable efficient versioning of the IR during optimization passes without copying entire structures. Immutable IR also simplifies reasoning about transformation correctness.

**Compact representation:** Represent enums and structs with minimal overhead. Use `#[repr(u8)]` for enums with few variants, and pack small structs to reduce padding. For Left-Right's operator sequences, consider using a flat representation instead of a tree.

A practical memory layout for Left-Right:

- Source code: Arena-allocated tokens, span information compacted
- AST: Arena-allocated nodes with indices instead of pointers
- IR: Static single assignment (SSA) form with value indices
- Type information: Interned type terms in a type arena
- Symbol tables: Interned strings with hash maps

The goal is to minimize pointer-chasing and maximize cache locality. Bump allocation naturally improves locality since temporaries are allocated sequentially.

### Caching Strategies

Compiler workloads have significant repetition. Developers rebuild frequently after small changes. Caching reduces redundant work.

**Content-addressable caching:** Compute a hash of each input source file and store compilation artifacts keyed by hash. When recompiling, check if any input files changed. Unchanged modules can reuse their artifacts. This is similar to how Bazel and other build systems operate.

For Left-Right, each module's compilation output can be cached separately. The cache key includes:
- Hash of the source file
- Hash of all imported modules
- Compiler version
- Optimization tier and flags

**Dependency graph caching:** Store the import dependency graph between compilations. When a file changes, only recompile modules that transitively depend on it. This is standard incremental compilation.

**Incremental type checking:** Persist type information across compilations. Only re-type-check modules affected by changes. More complex to implement but provides significant speedups for large codebases.

**IR snapshotting:** Cache intermediate representations at optimization boundaries. If source code changes but the IR is identical, skip earlier passes.

For a minimal viable compiler, start with dependency graph caching. Store a `.lr-deps` file in the project root tracking module imports and their hashes. On rebuild, only recompile changed modules.

## 5. Project Structure and Organization

A compiler is a substantial program. Organizing it well makes it maintainable and approachable for contributors. Production compilers follow established patterns.

### How Production Compilers Organize Code

**Rustc:** The Rust compiler uses a workspace with multiple crates:
- `rustc_*` crates contain compiler internals (parser, type checker, optimizer)
- `rustc_middle` contains shared data structures
- `rustc_*_target` crates handle platform-specific code generation
- `rustc_driver` provides the CLI interface
- `rustc_*_mir` crates handle the mid-level IR

This separation allows focused work on specific compiler phases without needing to understand the entire codebase.

**Go compiler (cmd/compile):** The Go compiler is monolithic but internally structured by phases:
- `cmd/compile/internal/syntax` for parsing
- `cmd/compile/internal/types` for type checking
- `cmd/compile/internal/gc` for optimization and code generation
- `cmd/compile/internal/ssa` for SSA transformations

The compiler operates on a unified internal representation but cleanly separates concerns by package.

**V8:** V8 is highly modular with clear boundaries:
- Parsing and AST representation in the `parser` directory
- Bytecode generation in the `interpreter` directory
- Optimization passes in the `compiler` directory
- Runtime and builtin functions in the `runtime` directory

Each component has well-defined interfaces, enabling independent development and testing.

### Module/Crate Organization Patterns for a Compiler

For Left-Right in Rust, a reasonable crate structure:

```
left-right/
├── Cargo.toml (workspace)
├── compiler/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs (CLI driver)
│   │   ├── lexer.rs
│   │   ├── parser.rs
│   │   ├── ast.rs
│   │   ├── typeck.rs
│   │   ├── ir.rs
│   │   ├── optimizer.rs
│   │   ├── codegen_js.rs
│   │   ├── codegen_native.rs
│   │   └── driver.rs (orchestrates pipeline)
│   └── tests/ (integration tests)
├── runtime/ (native runtime library)
│   ├── Cargo.toml
│   └── src/
├── std/ (Left-Right standard library)
│   └── .lr files
└── crates/ (shared components)
    ├── arena/ (bump allocator)
    ├── intern/ (string interning)
    └── diagnostics/ (error reporting)
```

Key principles:

- **Separate compilation phases:** Each phase (lex, parse, type check, optimize, codegen) lives in its own module. Clear interfaces between them.
- **Shared data structures in separate crates:** Common utilities like arenas, interning, and diagnostics can be reused across the compiler and tools.
- **Runtime separate from compiler:** The native runtime is a separate crate that the compiler links against.
- **Tests co-located with code:** Unit tests for each module live in the same file as the implementation.

As the compiler grows, split phases into separate crates:

```
compiler/
├── Cargo.toml
├── lr-lexer/
├── lr-parser/
├── lr-typeck/
├── lr-opt/
├── lr-codegen-js/
├── lr-codegen-native/
└── lr-driver/
```

This mirrors how Rustc evolved. Start monolithic, split when phases become unwieldy.

### Testing Strategy for Each Pipeline Stage

A compiler needs comprehensive testing at multiple levels:

**Unit tests per pass:** Each compiler pass should have tests covering common cases and edge cases. For example:
- Lexer tests: valid tokens, invalid characters, whitespace handling
- Parser tests: valid expressions, error recovery, precedence
- Type checker tests: correct type inference, type errors
- Optimizer tests: expected transformations, no-op preservation

Use property-based testing where appropriate. For example, optimizer passes should preserve program semantics.

**Integration tests:** Test entire programs through the full pipeline. Verify that the compiler produces correct output for valid inputs and appropriate errors for invalid inputs.

**Snapshot tests:** For the optimizer and code generator, snapshot tests capture the IR or generated code. When a pass changes, diff against the expected output. Regression testing is critical for compilers.

**Negative tests:** Ensure the compiler rejects invalid programs with clear error messages. This is as important as accepting valid programs.

**Cross-compilation tests:** If targeting multiple platforms, test that generated code works correctly on each target.

For Left-Right, structure tests like this:

```
compiler/tests/
├── fixtures/ (source files with expected outputs)
│   ├── valid/ (should compile successfully)
│   ├── invalid/ (should fail with specific errors)
│   └── snapshots/ (expected IR and generated code)
├── integration_tests.rs (end-to-end tests)
└── snapshot_tests.rs (golden file tests)
```

Use `insta` or similar crate for snapshot testing. Store test data in version control to catch regressions.

### Build System Integration

The compiler needs to be buildable and testable with standard tools.

**Cargo workspace:** If using multiple crates, organize them as a Cargo workspace. This ensures consistent versions and simplifies building the entire compiler.

**Make:** A Makefile provides convenient targets:
- `make build`: build the compiler
- `make test`: run all tests
- `make bench`: run benchmarks
- `make install`: install the compiler to `$PATH`

**Continuous integration:** Configure CI to run tests and benchmarks on every commit. Catch regressions early.

**Release builds:** Provide pre-built binaries for common platforms. Use GitHub Actions or similar to automate releases.

For Left-Right:

```makefile
.PHONY: build test bench install clean

build:
	cargo build --release

test:
	cargo test --all

bench:
	cargo test --all --release -- --ignored

install:
	cargo install --path compiler

clean:
	cargo clean
```

This provides a familiar interface for contributors and users.