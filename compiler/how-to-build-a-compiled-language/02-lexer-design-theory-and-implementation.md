# Lexer Design Theory and Implementation

## Report 2 of 8: How to Build a Compiled Programming Language

---

## 1. Lexer Theory

A lexer transforms raw source text into a stream of tokens, each representing a meaningful unit for parsing. The lexer operates at the lowest level of compilation, handling character classification, token boundary detection, and error recovery.

### Finite Automata for Token Recognition

Lexers are fundamentally built on regular languages, which can be recognized by finite automata. Two types exist:

- **Nondeterministic Finite Automata (NFA)**: At any state, multiple transitions may exist for a given input. NFAs can have epsilon transitions (moves without consuming input).
- **Deterministic Finite Automata (DFA)**: For each state and input, exactly one transition exists. No ambiguity.

NFAs are easier to construct from regex patterns, but DFAs are more efficient to execute because there is no backtracking or state set tracking. The classic conversion from NFA to DFA uses the subset construction algorithm, which may cause exponential state explosion in theory but rarely in practice for real programming languages.

For a simple identifier rule `[a-zA-Z_][a-zA-Z0-9_]*`, the DFA looks like:

```
State 0 (start): [a-zA-Z_] -> State 1
State 1: [a-zA-Z0-9_] -> State 1 (loop back)
State 1 is accepting (end of identifier)
```

When the DFA reaches a non-accepting state or no transition exists for the current character, the lexer retreats to the last accepting state and emits the token.

### Maximal Munch Algorithm

The maximal munch principle states that the lexer should always consume the longest possible prefix that matches any token rule. This resolves ambiguities between overlapping token patterns.

For example, with rules for both `<` and `<=`, the lexer must match `<=` when both `<` and `<=` could match. The implementation typically works as follows:

1. Run the DFA while consuming input
2. Track the most recent accepting state encountered
3. When stuck (no transition), backtrack to the last accepting state
4. Emit the token corresponding to that state
5. Reset to the start state and continue from the backtrack position

For our language with `_>` as a two-character token versus `_` as a one-character token, maximal munch ensures `_>` is always tokenized as a single unit when both characters are present. Similarly, `_<` is a two-character token tokenized as `LeftArg`, while `_>` is tokenized as `RightArg`.

### Hand-Written Recursive Descent vs Generated Lexers

Two implementation paradigms dominate production lexers:

**Generated lexers (lex/flex, ragel):**
- Specify token rules as regex patterns
- Tool generates the DFA tables and driver code
- Pros: Declarative, easy to modify rules, optimized state machines
- Cons: Generated code can be hard to debug, build-time dependency, less control over error handling

**Hand-written recursive descent lexers:**
- Direct imperative code for each token type
- Often use large `match` statements on the current character
- Pros: Complete control, easier to add context-sensitive behavior, no build-time tooling
- Cons: More verbose, easier to introduce bugs in state transitions

Modern compilers often use hybrid approaches. For example, V8 uses a hand-written lexer in C++ but heavily optimized with jump tables. Go's lexer is hand-written. Rustc's lexer is hand-written but uses procedural macros for repetitive pattern matching.

### Performance Characteristics

Generated lexers excel when:
- Token rules are complex and overlapping
- Performance is critical and hand-tuning is impractical
- Regex patterns change frequently during language evolution

Hand-written lexers excel when:
- Context-sensitive behavior is needed (e.g., template strings in JavaScript)
- Special error recovery or diagnostic handling is required
- Control over memory allocation is critical

Benchmarks show that well-optimized hand-written lexers can outperform generated ones by 10-30%, but generated lexers are often within 5-10% of hand-written performance for most real-world workloads. The biggest performance wins come from avoiding unnecessary character lookups, minimizing branching, and using efficient token data structures.

---

## 2. Token Classification Strategies

### Languages Without Keywords

Most programming languages have a fixed set of reserved keywords (`if`, `while`, `function`, etc.) that are lexed as keyword tokens. Our Left-Right language takes a different approach: no keywords at all. Everything that looks like an identifier is an identifier token.

This approach simplifies the lexer because there is no keyword table to maintain. All identifiers are tokenized identically, and the parser or later phase determines semantic meaning based on context. For example, in a typical language, `if` would be a keyword token, but in Left-Right, it would be an `Identifier` token like any other word.

The tradeoff is that the parser must be more sophisticated. It cannot distinguish syntactic constructs based on token type alone; it must use positional context or special operators.

### Identifier vs Operator Disambiguation

In languages with rich operator syntax, distinguishing identifiers from operators can be tricky. Consider these cases from our language:

- `response@value` should be `Identifier`, `@`, `Identifier`
- `_>` is a two-character operator token
- `!!!` is a single identifier (not three `!` tokens)
- `!!!?` is also a single identifier

The lexer needs classification rules:

1. **Single-character delimiters**: Characters like `:`, `,`, `.`, `'`, `(`, `)`, `[`, `]`, `{`, `}` are always delimiters. The backtick `` ` `` is a string delimiter (not emitted as a token).
2. **Two-character tokens**: `_>` and `_<` are fixed two-character tokens named `RightArg` and `LeftArg`. The lexer checks for both characters before emitting.
3. **Identifier rules**: All sequences of non-delimiter characters that are not reserved are identifiers. This includes ALL operators: `+`, `@`, `><`, `$@`, `!!!`, `!!!?`, `///`, `\\\`. Maximal munch applies: `!!!?` is ONE identifier (not `!!!` + `?`), `///` is ONE identifier, `\\\` is ONE identifier.
4. **Non-composite sequences**: `_:`, `+:`, `?:` are explicitly NOT composite. This means `_` and `:` are two separate tokens, not a single operator. Same for `+:` (identifier `+` + delimiter `:`).

One effective strategy is to use a prefix trie for operator recognition. As characters are consumed, the lexer walks down the trie. If it reaches a terminal node, that's a valid operator. If no further match exists, emit the longest valid operator found so far.

### Context-Sensitive Lexing

Most lexers are context-free, meaning the same character sequence always produces the same tokens regardless of surrounding code. However, some languages require context sensitivity:

- **Python**: Indentation is significant, requiring the lexer to track indentation levels.
- **JavaScript template strings**: Inside `` `{expr}` `` interpolation, the lexer must switch to expression mode.
- **C preprocessor**: `#` at line start is a directive, but `#` elsewhere is the tokenization operator.

Left-Right requires minimal context sensitivity. The only context-sensitive element is the interpolation syntax within backtick strings. When the lexer encounters `{` inside a backtick string, it must switch to "expression mode" and balance parentheses until the closing `}`. The lexer must distinguish `{` at the start of interpolation from `{` used as a map delimiter in regular code.

This requires a stack-based approach: push a new context when entering interpolation, pop when exiting, and return to string-lexing mode.

### Unicode Handling in Identifiers

Modern languages support Unicode in identifiers. The standard approach is to use the Unicode standard's identifier properties:

- `XID_Start`: Characters that can start an identifier
- `XID_Continue`: Characters that can continue an identifier

For Left-Right, the simplest rule is `[a-zA-Z_][a-zA-Z0-9_]*` for ASCII-only identifiers initially, with plans to expand to Unicode later using the XID properties. This matches Rust's approach, which supports full Unicode identifiers via the `XID_Start` and `XID_Continue` properties.

Implementation typically involves using the `unicode-ident` crate in Rust or the `ucd` tables in other languages to classify characters efficiently without loading the entire Unicode database.

---

## 3. String Interpolation Lexing

### State Machine Approach

String interpolation requires a lexer with multiple modes. The base state machine might have these states:

1. **Default**: Tokenizing regular source code
2. **Inside backtick string**: Collecting string content, processing escapes
3. **Inside interpolation**: Tokenizing expression content inside `{...}`
4. **Inside nested backtick**: Handling recursive backtick strings within interpolation

The state transitions:

```
Default -> InsideBacktick (on seeing `)
InsideBacktick -> InsideInterpolation (on seeing {)
InsideInterpolation -> InsideBacktick (on seeing nested `)
InsideInterpolation -> InsideBacktick (on seeing } and stack empty)
InsideBacktick -> Default (on seeing ` and stack empty)
```

A simple implementation uses a stack to track nesting depth. When entering interpolation, push the context. When exiting, pop. Only return to default when the stack is empty.

### Balancing Delimiters Inside Interpolation

When inside an interpolation expression, the lexer must balance parentheses, brackets, and braces to find the correct closing `}`. The interpolation syntax is `{expr}` where `expr` can contain nested delimiters.

For example:
```
`Result: {map.key + calculate(1, 2)}`
```

The lexer must recognize that `calculate(1, 2)` contains parentheses but does not end the interpolation. Only the `}` that matches the opening `{` should close the interpolation.

This requires a delimiter balance counter:
- Increment on `(`, `[`, `{`
- Decrement on `)`, `]`, `}`
- Interpolation ends when balance returns to zero (the initial `{`)

### Multi-Line String Handling

Left-Right backtick strings are multi-line by default. This means newlines within backticks are part of the string, not string terminators. The lexer must handle:

- Preserving newlines exactly as they appear
- Handling indentation (optional: some languages normalize leading whitespace)
- Allowing empty lines

No special state logic is needed beyond treating newlines as regular string content. The lexer simply consumes characters until the closing backtick, counting newlines for accurate line number tracking in diagnostics.

### Escape Sequence Processing

Backtick strings support `\` escaping. Common escape sequences include:

- `\"` -> literal quote (though quotes are not special in backticks)
- `\\` -> literal backslash
- `\n` -> newline
- `\t` -> tab
- `\{` -> literal `{` (prevents interpolation)
- `\`` -> literal backtick (prevents string termination)

The lexer processes escapes by:
1. When seeing `\` inside a string, enter escape mode
2. Read the next character
3. Emit the corresponding character or emit an error for invalid escapes
4. Return to normal string collection mode

For zero-copy efficiency, the lexer can emit string tokens with raw spans and defer escape processing to a later phase, or it can process escapes on-the-fly. The choice depends on whether the parser or later phases need access to the raw string source.

---

## 4. Performance Optimization

### Zero-Copy Lexing

Tokenization typically involves allocating new strings for each identifier or literal. This is expensive due to memory allocation and copying. Zero-copy lexing avoids this by storing only spans (start index, end index) into the original source text.

Instead of:

```rust
struct Token {
    kind: TokenKind,
    value: String,  // Allocated and copied
    span: Span,
}
```

Use:

```rust
struct Token {
    kind: TokenKind,
    span: Span,  // Start and end positions in source
}

impl Token {
    fn value<'a>(&self, source: &'a str) -> &'a str {
        &source[self.span.start..self.span.end]
    }
}
```

The source text is borrowed for the lifetime of the compilation session, and tokens reference it via spans. String values are materialized only when needed (e.g., when comparing to a symbol table). This reduces allocations from O(number of tokens) to O(1).

### SIMD-Accelerated Character Classification

Character classification (is this a letter? a digit? whitespace?) is called millions of times during lexing. Modern CPUs offer SIMD instructions that can classify 16, 32, or 64 characters at once.

For example, classifying ASCII letters:

- **Scalar approach**: Check each character with range comparisons
- **SIMD approach**: Load 16 bytes, use shuffle/mask instructions to classify in parallel

The GCC and LLVM optimizers can auto-vectorize some patterns, but manual SIMD using intrinsics or libraries like `memchr` can provide significant speedups.

A common optimization is using `memchr` (or equivalent) to skip whitespace in bulk. Instead of calling a loop for each space character, use `memchr` to find the next non-whitespace character in a single system call.

### Lookup Tables vs Branching

Character classification can use:

1. **Branching**: `if c.is_ascii_alphabetic() { ... }`
2. **Lookup tables**: Precomputed boolean arrays indexed by character code

Lookup tables are faster for hot loops because they replace unpredictable branches with predictable memory accesses. For ASCII characters (0-127), a 128-byte table can classify digits, letters, delimiters, etc.

For example:

```rust
// Precomputed table
static IS_ALPHA: [bool; 256] = [
    false, false, ..., true, true, ...,  // 'a'-'z'
    true, true, ...,  // 'A'-'Z'
    ...
];

// Fast check
if IS_ALPHA[c as usize] {
    // It's a letter
}
```

Modern CPUs can cache these tiny tables completely, making the access essentially free.

### Memory Allocation Strategies

Beyond zero-copy token storage, consider these allocation strategies:

**Token stream as a Vec**:
- Pre-allocate capacity based on source length estimate
- Resize with `reserve` if needed
- Worst case: O(number of tokens) allocation, but amortized O(1)

**Streaming token iterator**:
- Lexer implements `Iterator<Item = Token>`
- Parser consumes tokens lazily
- No need to store all tokens simultaneously

**Chunked allocation**:
- Allocate tokens in fixed-size chunks (e.g., 1024 tokens per chunk)
- Reduces reallocation frequency
- Good for very large files

For most compilers, a simple `Vec<Token>` with pre-allocated capacity is sufficient. Only massive source files (millions of tokens) benefit from chunking.

---

## 5. Error Recovery

### Invalid Character Recovery

When the lexer encounters a character that doesn't match any token rule, it must recover to continue lexing the rest of the file. Strategies include:

1. **Skip and report**: Skip the invalid character, emit an error, continue
2. **Skip to next delimiter**: Skip characters until a known delimiter (whitespace, `(`, `)`, etc.)
3. **Greedy skip**: Skip all consecutive invalid characters as a single error

The most robust approach is greedy skip with delimiter detection. For example:

```rust
while pos < source.len() && !is_delimiter(source[pos]) {
    pos += 1;
}
emit_error!("Invalid character in token");
```

This groups consecutive invalid characters into one error message, which is more user-friendly than separate errors for each character.

### Unterminated String/Comment Handling

Unterminated strings and comments occur when the closing delimiter is missing. For backtick strings, this means the file ends before the closing `` ` ``.

Handling strategies:

1. **Fail fast**: Stop lexing immediately, report error
2. **Treat as unterminated**: Emit the string token with an error, continue
3. **Synthetic terminator**: Inject a synthetic closing delimiter at end of file

Fail fast is simplest but prevents reporting multiple errors. Treating as unterminated is better for diagnostics. The lexer emits the string token with the unterminated flag set, so the parser can still parse the rest of the file (though it may encounter cascading errors).

For comments, unterminated comments are rarely fatal. The lexer can simply consume the rest of the file as comment content and report one error.

### Error Span Tracking

Good error messages require accurate source spans. The lexer must track:

- Byte offset for each token
- Line and column numbers for diagnostics
- Whether the token is on a new line

Line-column tracking can be done incrementally:

```rust
let mut line = 1;
let mut col = 1;

for ch in source.chars() {
    if ch == '\n' {
        line += 1;
        col = 1;
    } else {
        col += 1;
    }
}
```

Or computed on-demand from byte offsets using a line-start table. The line-start table records the byte offset of each line start, allowing binary search to find the line for any byte offset. This is more memory-efficient for large files but requires extra computation.

### Continuing Lexing After Errors

The lexer should never panic on invalid input. After encountering an error:

1. Emit an error token or diagnostic
2. Reset to a known state (usually the default state)
3. Resume from the position after the error
4. Continue tokenizing

The key is to advance the lexer position past the error. If the lexer doesn't advance, it will loop infinitely on the same invalid character.

For example, when seeing an invalid character:

```rust
emit_error!("Invalid character: {:?}", ch);
pos += 1;  // Always advance!
```

For unterminated strings at end of file, set the position to the end of the file so the lexer stops naturally.

---

## 6. Production Lexer Examples

### V8 (JavaScript Engine)

V8's lexer is hand-written in C++ and highly optimized. Key features:

- **Two-pass design**: First pass does quick tokenization, second pass does more detailed analysis if needed
- **Scan literals lazily**: String literals are not fully scanned until needed
- **Context-sensitive**: Switches between code, regex, and template string modes
- **Template literals**: Complex interpolation tracking with recursion
- **Unicode**: Full support for Unicode identifiers and strings

Performance: V8 can lex JavaScript at 10-50 MB/s on modern hardware, depending on complexity. The lexer is bottlenecked by memory bandwidth more than CPU cycles.

### Rustc (Rust Compiler)

Rustc's lexer is hand-written in Rust with extensive use of pattern matching. Key features:

- **Functional style**: Heavy use of iterators and combinators
- **Token enum**: Exhaustive `TokenKind` enum with 100+ variants
- **Span tracking**: Integrated with Rust's `Span` type
- **Unicode identifiers**: Uses the `unicode-ident` crate
- **Procedural macros**: Uses macros to reduce boilerplate in token matching

Performance: Rustc lexes at roughly 5-20 MB/s. The lexer is not the bottleneck for typical Rust code; type checking and borrow checking dominate compilation time.

### Go Compiler

Go's lexer is hand-written with a straightforward approach. Key features:

- **Simple state machine**: Switch on current character, classify token type
- **Precomputed tables**: Use lookup tables for character classification
- **Incremental API**: Supports incremental tokenization for editor features
- **Unicode support**: Full Unicode identifier support

Performance: Go's lexer is extremely fast, capable of lexing 50-100+ MB/s. The language's simple syntax contributes to this speed.

### Hand-Written vs Generated Comparison

| Aspect | Hand-Written | Generated |
|--------|--------------|-----------|
| Performance | 10-30% faster | Slightly slower |
| Maintainability | Verbose, risk of bugs | Concise, safer |
| Debugging | Easier (read source) | Harder (generated code) |
| Flexibility | Full control | Constrained by tool |
| Build time | No extra step | Extra generation step |
| Error handling | Customizable | Tool-dependent |

Real-world benchmarks (lexing a 100MB JavaScript file):
- **V8 (hand-written)**: ~8 seconds
- **Esprima (generated)**: ~11 seconds
- **Acorn (hand-written)**: ~7 seconds

The 20-30% performance difference matters for build times on large codebases, which is why most production compilers use hand-written lexers.

---

## 7. Implementation Recommendations

### Recommended Approach for Left-Right

Given our language's characteristics, recommend a **hand-written recursive descent lexer** for the following reasons:

1. **Context-sensitive interpolation**: The nested `{...}` syntax in strings requires careful state management that is easier to hand-code than express in regex patterns
2. **No keywords**: Simplifies the lexer considerably; no keyword table needed
3. **Operators are identifiers**: All operators are just identifiers, no special operator token types
4. **Two fixed multi-char tokens**: Only `_<` and `_>` are two-character tokens, making pattern matching straightforward
5. **Error recovery**: Custom error handling for unterminated strings and comment-line detection is easier with direct control
6. **Rust ecosystem**: Hand-written lexers are idiomatic in Rust and benefit from pattern matching

**Lexer architecture:**
```rust
struct Lexer<'a> {
    source: &'a str,
    pos: usize,
    line: usize,
    col: usize,
    state: LexerState,
    interpolation_depth: usize,
}

enum LexerState {
    Default,
    InBacktickString,
    InInterpolation,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, LexerError>;
    fn next(&mut self) -> Option<Self::Item>;
}
```

### Data Structures for Token Storage

Recommended token data structures:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line_start: usize,
    pub col_start: usize,
    pub line_end: usize,
    pub col_end: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier,      // All operators, function names, variables: +, @, ><, $@, !!!, !!!?, //, \\, map, etc.
    NumberLiteral,   // Decimal numbers only (integer or float): 42, 3.14, -5 is 0-5 (operator)
    StringLiteral,   // Backtick-delimited strings with optional interpolation

    // Delimiters (single-character)
    LBrace, RBrace,     // { }
    LBracket, RBracket, // [ ]
    LParen, RParen,     // ( )
    Colon,              // :
    Comma,              // ,
    Dot,                // .
    Quote,              // ' (reserved, unused)

    // Two-character tokens
    LeftArg,   // _<  (left input reference)
    RightArg,  // _>  (right input reference)

    // Special
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}
```

For zero-copy, store only spans. Materialize values on-demand:

```rust
impl Token {
    pub fn text<'a>(&self, source: &'a str) -> &'a str {
        &source[self.span.start..self.span.end]
    }
}
```

### API Design

**Lexer trait**:
```rust
pub trait Lexer {
    fn next_token(&mut self) -> Result<Token, LexerError>;
    fn peek(&self) -> Option<&Token>;
    fn source(&self) -> &str;
    fn position(&self) -> usize;
}
```

**Token stream iterator**:
```rust
pub struct TokenStream<'a> {
    lexer: Lexer<'a>,
    peeked: Option<Result<Token, LexerError>>,
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = Result<Token, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        // Implementation with peek support
    }
}
```

**Error handling**:
```rust
#[derive(Debug)]
pub enum LexerError {
    InvalidCharacter { ch: char, span: Span },
    UnterminatedString { span: Span },
    UnterminatedComment { line: usize },
    UnexpectedEOF { expected: String, span: Span },
}

impl LexerError {
    pub fn report(&self, source: &str) -> String {
        // Generate user-friendly error message with source snippet
    }
}
```

**Integration with parser**:
```rust
impl Parser {
    pub fn new(source: &str) -> Self {
        let lexer = Lexer::new(source);
        let tokens: Vec<Token> = lexer.collect::<Result<_, _>>().unwrap();
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn consume(&mut self) -> &Token {
        let token = &self.tokens[self.pos];
        self.pos += 1;
        token
    }
}
```

This design provides:
- Zero-copy tokenization
- Efficient iteration and peeking
- Clear error reporting
- Simple parser integration
- Room for future extensions (e.g., incremental lexing for IDE features)

---

## 8. Left-Right-Specific Notes

### Comments

Left-Right uses triple-backtick comments: `` ``` `` at the start of a line denotes a comment to end of line. The lexer must distinguish this from backtick-delimited strings:

- If `` ``` `` appears at line start (after optional whitespace), it's a comment
- Otherwise, `` ` `` is a string delimiter

### Numbers Are Decimal Only

Left-Right numbers are decimal only—no hex, binary, octal, or scientific notation. This simplifies number lexing considerably. Floats must start with a digit: `0.5` is valid, `.5` is invalid. Negative numbers are expressed using the `-` operator: `0-5` for negative 5.

### Operators Are Identifiers

All operators in Left-Right are identifiers: `+`, `@`, `><`, `$@`, `!!!`, `!!!?`, `///`, `\\\`. The lexer does NOT distinguish operators from identifiers. Operator semantics emerge at runtime based on value types, not at lex/parse time.

Maximal munch ensures that `!!!?` is ONE identifier, not `!!!` + `?`. Similarly, `///` is ONE identifier, not three `/` characters, and `\\\` is ONE identifier.

### Two-Character Tokens

Only two tokens are multi-character in Left-Right:
- `_<` → `LeftArg` token
- `_>` → `RightArg` token

These are always emitted as single tokens, never as two separate characters.

### Non-Composite Sequences

Some sequences look like they could be single tokens but are NOT:
- `_: ` is identifier `_` + colon `:`
- `+:` is identifier `+` + colon `:`
- `?:` is identifier `?` + colon `:`

These are always tokenized as two separate tokens.

### No Type Annotations

Left-Right has no type syntax. The lexer does NOT handle type annotations, type signatures, or type-related tokens. All types are inferred at runtime.

---

## Conclusion

Lexer design is foundational to compiler architecture. For Left-Right, a hand-written recursive descent lexer with state machine support for string interpolation, zero-copy token storage, and robust error recovery provides the best balance of performance, maintainability, and flexibility.

The lexer should be implemented first and thoroughly tested with edge cases before moving to parsing. A comprehensive test suite covering:
- All token types and combinations
- Maximal munch behavior
- String interpolation nesting
- Escape sequences
- Error conditions

With a solid lexer in place, the parser can focus on building the abstract syntax tree without worrying about tokenization details.

---

**Next report**: Parser theory and implementation, covering recursive descent parsing, operator precedence, AST design, and error recovery.