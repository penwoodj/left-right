# Report 3: Parser Design and AST Construction

## 1. Parsing Theory

Parser design begins with choosing a parsing strategy. Each major approach has distinct characteristics that affect implementation complexity, error recovery, and performance.

### LL(k) and Recursive Descent

LL(k) parsers scan input left-to-right, producing a leftmost derivation with k tokens of lookahead. Recursive descent implements LL parsing as a set of mutually recursive functions, one for each grammar rule.

**Advantages:**
- Intuitive, readable code structure
- Excellent error messages with clear call stacks
- Easy to add custom actions during parsing
- Natural fit for hand-written parsers

**Disadvantages:**
- Cannot handle left recursion (requires grammar transformation)
- Limited lookahead can cause exponential backtracking with ambiguity
- Requires careful grammar design to avoid conflicts

**Performance:** O(n) for well-designed grammars with proper memoization, but can degrade to O(k^n) with backtracking.

### LR(k) and Bottom-Up Parsing

LR(k) parsers build rightmost derivations in reverse, using shift-reduce operations on a stack. Yacc, Bison, and many parser generators use LR(1) or LALR(1).

**Advantages:**
- Handles left recursion naturally
- More powerful than LL parsing
- Never backtracks (deterministic)
- Excellent for expression grammars

**Disadvantages:**
- Parser generator required (not hand-writable)
- Poor error messages (reduce-reduce conflicts obscure)
- Complex to debug
- Grammar must be LALR(1) or similar

**Performance:** O(n) guaranteed, constant-time operations per token.

### PEG (Parsing Expression Grammars)

PEG treats parsing as recognizing patterns, not generating derivations. PEG grammars are ordered and unambiguous by definition.

**Advantages:**
- Simple, declarative syntax
- No ambiguity conflicts
- Easy to extend with semantic predicates
- Natural fit for combinator libraries

**Disadvantages:**
- Ordering matters (unlike CFG)
- Can't express all CFG grammars
- No natural error recovery
- Backtracking can be expensive without memoization

**Performance:** O(n) with packrat memoization, O(2^n) worst-case without.

### Pratt Parsing (Top-Down Operator Precedence)

Pratt parsing specializes in expression parsing with operator precedence. It assigns each token a binding power and climbs up or down the precedence tree accordingly.

**Advantages:**
- Minimal code for expression grammars
- Handles arbitrary precedence elegantly
- Easy to add new operators
- No grammar transformations

**Disadvantages:**
- Specialized for expressions only
- Not a general-purpose parsing technique
- Requires careful binding power management

**Performance:** O(n), one pass through tokens.

### When Each Is Appropriate

- **Recursive descent:** Languages with simple, non-ambiguous grammars and hand-written parser preference
- **LR/LALR:** Languages requiring complex grammars or using parser generators
- **PEG:** Prototype languages, DSLs, or when declarative grammar specification is priority
- **Pratt:** Expression-heavy languages with precedence rules

For Left-Right with zero precedence, recursive descent or PEG are ideal choices.

---

## 2. Parsing Zero-Precedence Operator Languages

Zero precedence means all operators have equal precedence and associate left-to-right. This dramatically simplifies parsing by eliminating the need for precedence climbing or complex precedence tables.

### Simplified Grammar

A typical expression grammar with precedence has nested productions:

```ebnf
Expr   := Term (('+' | '-') Term)*
Term   := Factor (('*' | '/') Factor)*
Factor := '(' Expr ')' | Number
```

With zero precedence, the grammar flattens:

```ebnf
Expr := Primary (Operator Expr)?
Primary := Identifier | Number | '(' Expr ')' | '[' Expr* ']' | '{' Pair* '}'
Operator := Identifier  # `.` is NOT an operator here - handled separately as delimiter
```

This is a simple left-associative chain. Each operator consumes an operand and recurses naturally.

### Left-Associative Chain Parsing

Parsing becomes straightforward: read a primary, then if an operator follows, parse another expression as the operand.

```
1 + 2 * 3

Parse:
1. Read primary: 1
2. Read operator: +
3. Parse operand: 2 * 3 (as a single unit, left-to-right: ((1 + 2) * 3))
4. Return: Apply(Apply(Apply(Number(1), Identifier("+")), Number(2)), Identifier("*")), Number(3))
```

Left-Right evaluates left-to-right with zero precedence: `1 + 2 * 3` = `((1 + 2) * 3)` = `9`, not the traditional `7`.

### APL/J/K/BQN Approach

Array languages handle this elegantly. In APL, all functions are right-associative with equal precedence:

```
a + b × c  →  a + (b × c)  (right-associative)
```

BQN and similar languages use right-to-left evaluation:

```
F G H x  →  F (G (H x))  (nested right to left)
```

These languages often use a simple stack-based parser or Pratt parser with a single precedence level. The parser either accumulates arguments right-to-left or handles operators uniformly.

For Left-Right with left-to-right curried evaluation:

```
a b c  →  ((a b) c)  (left-associative)
```

This maps directly to recursive descent or simple combinator parsing.

### Benefits for Left-Right

1. **No precedence table:** All identifiers are operators with equal precedence
2. **Simple error messages:** Expected "expression or operator" is always valid
3. **Predictive parsing:** One token lookahead suffices
4. **No ambiguity:** Left-associative chains have one parse tree
5. **Fast parsing:** O(n) with minimal backtracking

The zero-precedence design is a deliberate simplification that makes the parser trivial while still enabling powerful composition through currying.

---

## 3. AST Design Patterns

Once parsing produces a tree, the Abstract Syntax Tree (AST) must be structured for efficient traversals, transformations, and code generation.

### Node Type Hierarchies

#### Enum-Based (Tagged Unions)

```rust
enum Expr {
    Number(f64),
    String(String),                    // backtick string, possibly with interpolation
    Boolean(bool),
    Undefined,
    Identifier(String),                // includes ALL operators
    List(Vec<Expr>),
    Map(Vec<(Expr, Expr)>),            // {key: value, key: value}
    Apply { func: Box<Expr>, arg: Box<Expr> },  // left-to-right curried application
    Grouped(Box<Expr>),                // parenthesized expression
    LeftArg,                           // _< token
    RightArg,                          // _> token
}
```

**Advantages:**
- Memory efficient (single enum variant)
- Exhaustive matching guarantees
- Zero-cost abstraction in Rust

**Disadvantages:**
- Adding fields requires changing all constructors
- Large enums can be unwieldy
- No shared base for metadata

#### Trait-Based (Visitor Pattern)

```rust
trait ExprNode {
    fn accept(&self, visitor: &mut dyn Visitor);
}

struct Number {
    value: f64,
    span: Span,
}

impl ExprNode for Number {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_number(self);
    }
}
```

**Advantages:**
- Extensible (add new node types without modifying base)
- Open/closed principle for visitors
- Metadata easy to attach via struct fields

**Disadvantages:**
- Dynamic dispatch overhead
- Boilerplate for each node type
- More memory (vtable pointers)

#### Struct-Based (Flat Hierarchy)

```rust
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

pub enum ExprKind {
    Number { value: f64 },
    String { content: String, interpolations: Vec<Expr> },
    Boolean { value: bool },
    Undefined,
    Identifier { name: String },
    List { elements: Vec<Expr> },
    Map { pairs: Vec<(Expr, Expr)> },
    Apply { func: Box<Expr>, arg: Box<Expr> },
    Grouped { expr: Box<Expr> },
    LeftArg,
    RightArg,
}
```

**Advantages:**
- Metadata separated from logic
- Each variant has its own struct
- Easy to add common fields

**Disadvantages:**
- More allocation (Box for nested nodes)
- Slightly verbose

### Arena Allocation

Standard AST allocation creates many small heap objects. Arena allocation groups all nodes in a contiguous region.

```rust
struct Arena {
    nodes: Vec<ExprNode>,
}

struct NodeId(usize);  // Index into arena

let mut arena = Arena::new();
let id = arena.alloc(ExprKind::Number { value: 42.0 });
```

**Advantages:**
- Single allocation for entire AST
- Better cache locality
- Simplifies memory management (drop entire arena)
- Enables efficient graph structures (DAGs)

**Disadvantages:**
- Cannot drop individual nodes
- Requires indices instead of references
- Borrowing complexity in Rust

For a compiler, arena allocation is ideal. The AST lives for the entire compilation phase and is dropped as a unit.

### Source Span Metadata

Every AST node should track its source location for error reporting and debugging.

```rust
#[derive(Debug, Clone)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}
```

Spans enable:
- Precise error messages pointing to source
- Source maps for debugging generated code
- IDE integration (hover, go-to-definition)
- Coverage tracking

### Immutable vs Mutable ASTs

**Immutable ASTs:**
- Safer (no unexpected modifications)
- Enables sharing (DAG for common subexpressions)
- Compatible with functional transformations
- Best for pure functional compilers

**Mutable ASTs:**
- In-place transformations without copying
- Easier optimization passes
- Simpler desugaring in-place
- More memory efficient for large transformations

For Left-Right, start immutable, mutate via transformations that produce new ASTs. This balances safety and efficiency.

---

## 4. PEG Parser Construction

Parsing Expression Grammars offer a declarative, combinator-based approach. For Left-Right, PEG is ideal because the grammar is simple and benefits from combinator composition.

### Packrat Parsing and Memoization

PEG backtracking can be expensive. Packrat parsing memoizes results of parsing at each position:

```rust
struct Parser {
    input: Vec<Token>,
    pos: usize,
    memo: HashMap<(usize, Rule), ParseResult>,
}

fn parse_expr(&mut self) -> Result<Expr> {
    if let Some(result) = self.memo.get(&(self.pos, Rule::Expr)) {
        return result.clone();
    }
    let result = self.parse_expr_impl();
    self.memo.insert((self.pos, Rule::Expr), result.clone());
    result
}
```

**Advantages:**
- Guarantees O(n) parsing time
- Eliminates exponential backtracking
- Simple to implement

**Disadvantages:**
- Higher memory usage (memo table)
- Doesn't handle left recursion directly

For Left-Right's simple grammar, memoization is unnecessary overhead. Backtracking is minimal.

### Parser Combinators

Combinators are small functions that compose into complex parsers:

```rust
// Basic combinators
fn literal(expected: Token) -> impl Parser<Output = Token>
fn one_of(set: HashSet<Token>) -> impl Parser<Output = Token>
fn many(p: impl Parser) -> impl Parser<Output = Vec>
fn optional(p: impl Parser) -> impl Parser<Output = Option>
fn seq(p1: impl Parser, p2: impl Parser) -> impl Parser<Output = (Output1, Output2)>
fn choice(p1: impl Parser, p2: impl Parser) -> impl Parser<Output = Output>

// Higher-level combinators
fn parens(p: impl Parser) -> impl Parser<Output = Output> {
    seq(literal(Token::LParen), p, literal(Token::RParen))
}
```

**Example: Left-Right primary parser**

```rust
fn parse_primary(&mut self) -> Result<Expr> {
    choice!(
        self.parse_number(),
        self.parse_boolean(),
        self.parse_string(),
        self.parse_identifier(),
        self.parse_list(),
        self.parse_map(),
        self.parse_grouped(),
    )
}

fn parse_identifier(&mut self) -> Result<Expr> {
    let token = self.expect(TokenKind::Identifier)?;
    Ok(Expr::new(ExprKind::Identifier(token.lexeme), token.span))
}
```

### Handling Left-Right Grammar in PEG

The Left-Right grammar maps cleanly to PEG:

```peg
# Left-Right PEG Grammar
File       <- Expr EOF

Expr       <- Primary (Operator Expr)?
           / ReverseApply

Primary    <- Identifier
            / Number
            / String
            / Boolean
            / Undefined
            / List
            / Map
            / '(' Expr ')'
            / LeftArg
            / RightArg

Operator   <- Identifier

ReverseApply <- Primary '.' Expr

List       <- '[' Expr (',' Expr)* ']'
Map        <- '{' Pair (',' Pair)* '}'
Pair       <- Identifier ':' Expr      # Assignment (alpha-start key)
            / Expr ':' Expr            # Early return (expression key)

String     <- '`' (Interpolation / EscapedChar / Char)* '`'
Interpolation <- '{' Expr '}'          # {expr}, NOT ${expr}
```

**Implementation notes:**

1. **Left recursion elimination:** The `Expr` rule is not left-recursive. It uses right recursion, which PEG handles naturally.

2. **Zero precedence:** No need for precedence levels. All identifiers bind the same.

3. **Currying:** `Expr (Operator Expr)?` builds curried applications naturally.

4. **Maps:** The `Pair` rule handles both assignment (alpha-start key) and early return (expression key). Maps stay as `Map` nodes in the AST — they are NOT desugared to `If` or `Function` nodes.

5. **String interpolation:** Embedded expressions use `{expr}`, not `${expr}`.

6. **`.` is NOT an operator:** The parser recognizes `.` as a delimiter token and creates a special `ReverseApply` node, treating the right operand as the function.

**Rust implementation sketch:**

```rust
impl Parser {
    pub fn parse(&mut self) -> Result<Expr> {
        let expr = self.parse_expr()?;
        self.expect(TokenKind::EOF)?;
        Ok(expr)
    }

    fn parse_expr(&mut self) -> Result<Expr> {
        let func = self.parse_primary()?;
        if let Some(TokenKind::Dot) = self.peek() {
            self.consume();
            let arg = self.parse_expr()?;
            Ok(Expr::new(
                ExprKind::Apply { func: Box::new(arg), arg: Box::new(func) },
                Span::merge(&func.span, &arg.span)
            ))
        } else if let Some(_) = self.peek_operator() {
            self.consume();
            let arg = self.parse_expr()?;
            Ok(Expr::new(
                ExprKind::Apply { func: Box::new(func), arg: Box::new(arg) },
                Span::merge(&func.span, &arg.span)
            ))
        } else {
            Ok(func)
        }
    }

    fn parse_map(&mut self) -> Result<Expr> {
        let start_span = self.current_span();
        self.expect(TokenKind::LBrace)?;
        let mut pairs = Vec::new();
        loop {
            if self.peek(TokenKind::RBrace) { break; }
            pairs.push(self.parse_pair()?);
            if !self.consume_if(TokenKind::Comma) { break; }
        }
        self.expect(TokenKind::RBrace)?;
        Ok(Expr::new(ExprKind::Map(pairs), start_span))
    }

    fn parse_pair(&mut self) -> Result<(Expr, Expr)> {
        let key = self.parse_expr()?;
        self.expect(TokenKind::Colon)?;
        let value = self.parse_expr()?;

        // Colon disambiguation is handled at runtime, not parsing
        // Parser creates Map node with both key and value
        Ok((key, value))
    }

    fn parse_string(&mut self) -> Result<Expr> {
        let token = self.expect(TokenKind::Backtick)?;
        let mut content = String::new();
        let mut interpolations = Vec::new();

        while !self.peek(TokenKind::Backtick) {
            if self.peek(TokenKind::LBrace) {
                self.consume();
                let expr = self.parse_expr()?;
                interpolations.push(expr);
                self.expect(TokenKind::RBrace)?;
            } else {
                // Handle escaped chars and regular chars
                content.push(self.advance().lexeme);
            }
        }
        self.consume(); // closing backtick

        Ok(Expr::new(
            ExprKind::String { content, interpolations },
            token.span
        ))
    }
}
```

PEG provides a clean, declarative way to express the Left-Right grammar. The simple, uniform structure benefits greatly from combinator composition.

---

## 5. Error Recovery

A robust parser must recover from errors and continue parsing to report multiple issues in a single run.

### Panic Mode and Resynchronization

Panic mode discards tokens until a synchronization point is found, then continues parsing.

**Synchronization points:**
- Statement/Expression boundaries (end of file, semicolon)
- Block delimiters (`}`, `]`, `)`)
- Top-level constructs

```rust
fn synchronize(&mut self) {
    while !self.at_end() && !self.is_sync_point() {
        self.advance();
    }
}

fn is_sync_point(&self) -> bool {
    matches!(self.peek().kind,
        TokenKind::RBrace | TokenKind::RBracket | TokenKind::RParen |
        TokenKind::EOF | TokenKind::Comma)
}
```

### Meaningful Error Messages for Operator Chains

In zero-precedence languages, error messages must be specific:

```
Error at line 5: Expected expression after operator '+'

   5 | a + + b
         ^

The operator '+' requires a following expression.
```

Implementation:

```rust
fn parse_expr(&mut self) -> Result<NodeId> {
    let func = self.parse_primary()?;
    if let Some(op) = self.peek_operator() {
        self.consume();
        if self.at_expression_start() {
            let arg = self.parse_expr()?;
            Ok(self.alloc(ExprKind::Apply(func, arg)))
        } else {
            Err(ParseError::ExpectedExpressionAfterOperator {
                operator: op,
                span: self.current_span(),
            })
        }
    } else {
        Ok(func)
    }
}
```

### Continuing Parse After Errors

Error recovery strategies:

1. **Skip until sync point:** Discard tokens until a known structure is found.

2. **Insert missing tokens:** Assume missing tokens and continue (e.g., missing `}`).

3. **Replace with error node:** Create a placeholder node for malformed syntax.

```rust
fn parse_expr(&mut self) -> Result<NodeId> {
    match self.parse_primary() {
        Ok(func) => { /* ... */ }
        Err(e) => {
            self.errors.push(e);
            self.synchronize();
            Ok(self.alloc(ExprKind::Undefined))  // Placeholder
        }
    }
}
```

For Left-Right, error recovery is simpler due to the uniform grammar structure. Most errors occur at expression boundaries.

---

## 6. AST Transformation (No Desugaring for Control Flow)

Left-Right's design intentionally avoids desugaring maps to traditional control flow nodes like `If`, `Function`, or `Loop`. Maps remain as `Map` nodes throughout the AST, and code generation decides how to emit them.

### Maps Stay as Maps

Three map patterns are recognized but NOT transformed:

1. **Map-as-function:** `{ arg: _<@0, body }`
   - Map with `_<` references = unexecuted operator at runtime
   - Parser creates `Map` node
   - Codegen emits closure/function

2. **Map-as-conditional:** `{ _<: trueCase, falseCase }`
   - Expression key `_<` evaluates truthiness
   - `:` returns if truthy, falls through if not
   - Parser creates `Map` node
   - Codegen emits if-else chain

3. **Map-as-loop:** Uses iteration operators like `$`, `$@`, `$?`
   - These are identifiers with runtime semantics
   - Parser creates `Map` node with `Identifier` keys
   - Codegen emits loop construct

### No Desugaring Visitor Needed

Unlike traditional compilers, Left-Right does NOT need a desugaring pass that converts maps to `If`/`Function` nodes. The AST remains in its parsed form:

```rust
// This is NOT needed for Left-Right:
// struct DesugarVisitor;
// impl Visitor for DesugarVisitor {
//     fn visit_expr(&mut self, expr: &mut Expr) {
//         match &mut expr.kind {
//             ExprKind::Map(map) => {
//                 // NO desugaring happens here
//                 // Maps stay as maps
//             }
//             _ => {}
//         }
//     }
// }
```

### Visitor Pattern for Other Transformations

Multi-pass transformations via visitors are still useful for other purposes:

```rust
trait Visitor {
    fn visit_expr(&mut self, expr: &mut Expr);
}

struct InlineVisitor;

impl Visitor for InlineVisitor {
    fn visit_expr(&mut self, expr: &mut Expr) {
        match &mut expr.kind {
            ExprKind::Apply { func, arg } => {
                // Inline small functions, constant fold, etc.
            }
            _ => {}
        }
        self.visit_children(expr);
    }
}

fn run_transformations(mut ast: Expr) -> Expr {
    let mut visitor = InlineVisitor;
    visitor.visit_expr(&mut ast);
    ast
}
```

Multiple visitors can be composed:

```rust
let mut ast = parse(input)?;
ast = InlineVisitor::transform(ast);
ast = ConstantFoldVisitor::transform(ast);
```

This modular approach makes transformations easy to test and maintain, but control flow transformations are handled entirely by code generation.

---

## 7. Recommendations

### AST Node Types for Left-Right

Based on the language design, these AST node types are correct for Left-Right:

```rust
pub enum ExprKind {
    // Literals
    Number(f64),
    String { content: String, interpolations: Vec<Expr> },
    Boolean(bool),
    Undefined,

    // Identifiers (includes ALL operators - +, @, ><, $@, !!!, etc.)
    Identifier(String),

    // Special argument tokens
    LeftArg,  // _< (2-char token)
    RightArg, // _> (2-char token)

    // Application (core of Left-Right)
    Apply { func: Box<Expr>, arg: Box<Expr> },  // f x - left-to-right curried

    // Collections
    List(Vec<Expr>),
    Map(Vec<(Expr, Expr)>),  // {key: value} - stays as map, NOT desugared

    // Grouped expressions
    Grouped(Box<Expr>),  // (expr)

    // Error node for recovery
    Error,
}

pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}
```

**Key points:**
- NO `If`, `Function`, `Loop`, `Break`, `Continue` nodes
- Maps handle all control flow, functions, conditionals
- Operators are identifiers, not special AST nodes
- Iteration operators (`$`, `$@`, etc.) are identifiers
- `.` is NOT an operator - handled via `Apply` node with swapped args in codegen

### Recommended Parsing Approach

1. **Use recursive descent with combinators:**
   - Simple to write and debug
   - Excellent error messages
   - Natural fit for zero-precedence grammar

2. **Arena allocation for AST nodes:**
   - Single allocation for entire AST
   - Better performance and cache locality

3. **Minimal transformations:**
   - No desugaring of maps to control flow nodes
   - Maps stay as `Map` nodes throughout
   - Codegen decides how to emit map constructs

4. **Source span metadata on every node:**
   - Essential for error reporting
   - Enables source maps

5. **Error recovery with sync points:**
   - Continue parsing after errors
   - Report multiple issues per file

6. **Optional packrat memoization:**
   - Not needed for this simple grammar
   - Only add if backtracking becomes problematic

### Special Parsing Requirements

**String interpolation:**
- Use `{expr}` syntax, NOT `${expr}`
- Parse `LBrace Expr RBrace` inside backtick strings

**Colon disambiguation:**
- Parser creates `Map` node with both key and value
- Runtime decides: alpha-start key = assignment, expression key = early return
- No parsing-time distinction needed

**Reverse-args operator `.`:**
- NOT an operator in the grammar
- Parser recognizes `.` as a delimiter token
- Creates `Apply` node with func/arg swapped at parse time

**Export pattern `}@&[...]`:**
- Recognize as special end-of-file pattern
- Parse as `Apply(RBrace, Apply(Identifier("&"), List(...)))`

**LeftArg/RightArg tokens:**
- `_<` and `_>` are single 2-char tokens, not two tokens
- Parse as `LeftArg` and `RightArg` nodes

The zero-precedence design makes Left-Right unusually simple to parse. A straightforward recursive descent parser with arena-allocated AST nodes will be fast, maintainable, and produce excellent error messages. Maps remain as maps throughout the AST, with code generation handling the semantic interpretation of map-as-function, map-as-conditional, and map-as-loop patterns.