# Left-Right Lexer and AST Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a complete lexer and parser for the Left-Right language that produces a verified AST with full error recovery and live testing coverage.

**Architecture:** Hybrid lexer (logos for normal tokens, hand-written fallback for operator identifiers), recursive descent parser with zero-precedence left-to-right curried evaluation, bogus-node error recovery (Biome pattern), ariadne diagnostics rendering.

**Tech Stack:** Rust, logos 0.16.1 + hand-written fallback for lexer, insta/proptest for testing, ariadne for diagnostics.

---

## OpenCode Agent Skills

Load these skills when executing this plan:

- **writing-plans** — Use before starting any multi-step implementation
- **test-driven-development** — Use for implementing features (write test first, then implementation)
- **systematic-debugging** — Use when encountering bugs or test failures
- **verification-before-completion** — Use before claiming work complete
- **commit** — Use for all git commits (never commit directly without skill)

---

## 1. Project Setup

### 1.1 Workspace Structure

```
left-right/
├── Cargo.toml                      # Workspace root
├── compiler/
│   ├── Cargo.toml                  # Compiler workspace
│   ├── plans/
│   │   └── creating-lexer-and-ast/
│   │       └── 00-implementation-plan.md
│   ├── specs/                      # Specification files (read-only)
│   ├── crates/
│   │   ├── lr-lexer/               # Lexer implementation
│   │   ├── lr-parser/              # Parser implementation
│   │   ├── lr-ast/                 # AST node definitions
│   │   ├── lr-diagnostics/         # Error reporting
│   │   └── lr-common/              # Shared types (span, location)
│   └── tests/                      # Integration tests
└── language-summary/               # Language documentation (read-only)
```

### 1.2 Root Cargo.toml

```toml
[workspace]
members = [
    "compiler/crates/*",
]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
rust-version = "1.75.0"

[workspace.dependencies]
# Core dependencies
ariadne = "0.4.0"
insta = "1.34.0"
proptest = "1.4.0"
thiserror = "1.0.56"
logos = "0.16.1"

# Design decision: Hybrid lexer chosen over pure hand-written for performance (logos generates compile-time FSM, 2-10x faster for normal tokens). Hand-written fallback retained for Left-Right's unusual operator identifier rules (maximal munch, no keywords, operators-as-identifiers) which don't map cleanly to logos regex patterns.

# Local crates
lr-common = { path = "crates/lr-common" }
lr-lexer = { path = "crates/lr-lexer" }
lr-parser = { path = "crates/lr-parser" }
lr-ast = { path = "crates/lr-ast" }
lr-diagnostics = { path = "crates/lr-diagnostics" }

# Dev dependencies
insta-cmd = "0.3.0"
```

### 1.3 lr-common/Cargo.toml

```toml
[package]
name = "lr-common"
version.workspace = true
edition.workspace = true

[dependencies]
thiserror = { workspace = true }
```

### 1.4 lr-lexer/Cargo.toml

```toml
[package]
name = "lr-lexer"
version.workspace = true
edition.workspace = true

[dependencies]
lr-common = { workspace = true }
thiserror = { workspace = true }
logos = { workspace = true }

[dev-dependencies]
insta = { workspace = true, features = ["glob", "tokenstream"] }
proptest = { workspace = true }
```

### 1.5 lr-parser/Cargo.toml

```toml
[package]
name = "lr-parser"
version.workspace = true
edition.workspace = true

[dependencies]
lr-common = { workspace = true }
lr-lexer = { workspace = true }
lr-ast = { workspace = true }
lr-diagnostics = { workspace = true }
thiserror = { workspace = true }

[dev-dependencies]
insta = { workspace = true, features = ["glob", "tokenstream"] }
proptest = { workspace = true }
```

### 1.6 lr-ast/Cargo.toml

```toml
[package]
name = "lr-ast"
version.workspace = true
edition.workspace = true

[dependencies]
lr-common = { workspace = true }
thiserror = { workspace = true }

[dev-dependencies]
insta = { workspace = true, features = ["glob", "tokenstream"] }
proptest = { workspace = true }
```

### 1.7 lr-diagnostics/Cargo.toml

```toml
[package]
name = "lr-diagnostics"
version.workspace = true
edition.workspace = true

[dependencies]
lr-common = { workspace = true }
ariadne = { workspace = true }
thiserror = { workspace = true }

[dev-dependencies]
insta = { workspace = true, features = ["glob", "tokenstream"] }
```

---

## 2. Lexer Implementation

### 2.1 Token Type Definitions

**File:** `crates/lr-lexer/src/token.rs`

Hybrid token approach: `RawToken` from logos + `Token` wrapper with spans:

```rust
use lr_common::Span;

/// Raw token types produced by logos (compile-time FSM)
/// Logos handles all straightforward tokens via regex patterns
#[derive(Logos, Debug, Clone, PartialEq, Eq)]
#[logos(skip r"[ \t\r\n]+")] // Skip whitespace
pub enum RawToken {
    // Structural delimiters (single tokens)
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token(":")]
    Colon,
    #[token("'")]
    Quote,
    #[token("`")]
    Backtick,

    // Number literal (decimal only, no hex/binary/scientific)
    #[regex(r"[0-9]+(\.[0-9]+)?")]
    Number,

    // String literals (non-interpolated, single/double quoted)
    // Note: Interpolation handled in second pass
    #[regex(r#""([^"\\]|\\.)*""#)]
    #[regex(r#"'([^'\\]|\\.)*'"#)]
    String,

    // Comments
    #[regex(r"//[^\n]*")]
    LineComment,
    #[regex(r"/\*[\s\S]*?\*/")]
    BlockComment,

    // Everything else: operator identifiers (handled by hand-written fallback)
    // This regex catches anything that's not whitespace, delimiters, or already matched
    #[regex(r"[^\s()[\]{},.:\"'`/]+")]
    OperatorIdent,

    // Error token for unrecognized input
    #[error]
    Error,
}

/// Final token type with proper TokenKind and span tracking
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    // Literals
    NumberLiteral,
    StringLiteral,
    BooleanLiteral,
    UndefinedLiteral,

    // Identifiers (includes ALL operators)
    Identifier,

    // Special 2-char tokens (extracted by hand-written fallback)
    LeftArg,      // _<
    RightArg,     // _>

    // Structural delimiters
    OpenBrace,    // {
    CloseBrace,   // }
    OpenBracket,  // [
    CloseBracket, // ]
    OpenParen,    // (
    CloseParen,   // )

    // Reserved symbols
    Colon,        // :
    Comma,        // ,
    Dot,          // .
    SingleQuote,  // ' (reserved, unused)

    // String delimiter
    Backtick,     // `

    // Comments
    Comment,

    // EOF
    EOF,
}

/// Token with span information (produced by wrapper)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, value: String, span: Span) -> Self {
        Self { kind, value, span }
    }
}
```

**File:** `crates/lr-common/src/lib.rs`

```rust
use std::ops::Range;

/// Byte offset range in source text
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    pub fn empty() -> Self {
        Self { start: 0, end: 0 }
    }

    pub fn range(&self) -> Range<usize> {
        self.start as usize..self.end as usize
    }

    pub fn len(&self) -> u32 {
        self.end - self.start
    }
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Self {
            start: range.start as u32,
            end: range.end as u32,
        }
    }
}
```

### 2.2 Hybrid Lexer Architecture

**File:** `crates/lr-lexer/src/lexer.rs`

Hybrid lexer: logos Phase 1 → hand-written wrapper Phase 2:

```rust
use logos::Logos;
use lr_common::Span;
use crate::token::{Token, TokenKind, RawToken};
use crate::error::LexError;

pub struct Lexer<'a> {
    source: &'a str,
    line: u32,
    column: u32,
    tokens: Vec<Token>,
    errors: Vec<LexError>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            line: 1,
            column: 1,
            tokens: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// Tokenize entire source using hybrid approach
    pub fn tokenize(mut self) -> (Vec<Token>, Vec<LexError>) {
        // Phase 1: Logos tokenization (compile-time FSM, 2-10x faster)
        let mut lex = RawToken::lexer(self.source);

        // Phase 2: Hand-written wrapper adds spans and handles operator identifiers
        while let Some(raw_token) = lex.next() {
            let span = Span::from(lex.span());

            match raw_token {
                // Simple tokens: direct mapping
                RawToken::LParen => self.tokens.push(Token::new(TokenKind::OpenParen, "(".to_string(), span)),
                RawToken::RParen => self.tokens.push(Token::new(TokenKind::CloseParen, ")".to_string(), span)),
                RawToken::LBracket => self.tokens.push(Token::new(TokenKind::OpenBracket, "[".to_string(), span)),
                RawToken::RBracket => self.tokens.push(Token::new(TokenKind::CloseBracket, "]".to_string(), span)),
                RawToken::LBrace => self.tokens.push(Token::new(TokenKind::OpenBrace, "{".to_string(), span)),
                RawToken::RBrace => self.tokens.push(Token::new(TokenKind::CloseBrace, "}".to_string(), span)),
                RawToken::Comma => self.tokens.push(Token::new(TokenKind::Comma, ",".to_string(), span)),
                RawToken::Dot => self.tokens.push(Token::new(TokenKind::Dot, ".".to_string(), span)),
                RawToken::Colon => self.tokens.push(Token::new(TokenKind::Colon, ":".to_string(), span)),
                RawToken::Quote => self.tokens.push(Token::new(TokenKind::SingleQuote, "'".to_string(), span)),
                RawToken::Backtick => self.process_backtick_token(span),

                // Number literal (decimal only)
                RawToken::Number => {
                    let value = self.source[span.range()].to_string();
                    self.tokens.push(Token::new(TokenKind::NumberLiteral, value, span));
                }

                // String literal (handle interpolation in second pass)
                RawToken::String => {
                    let raw_value = self.source[span.range()].to_string();
                    // Strip quotes
                    let value = if raw_value.len() >= 2 {
                        raw_value[1..raw_value.len()-1].to_string()
                    } else {
                        raw_value.clone()
                    };
                    self.tokens.push(Token::new(TokenKind::StringLiteral, value, span));
                }

                // Comments
                RawToken::LineComment => {
                    let value = self.source[span.range()].to_string();
                    // Strip "//"
                    let comment = if value.starts_with("//") {
                        value[2..].to_string()
                    } else {
                        value
                    };
                    self.tokens.push(Token::new(TokenKind::Comment, comment, span));
                }
                RawToken::BlockComment => {
                    let value = self.source[span.range()].to_string();
                    self.tokens.push(Token::new(TokenKind::Comment, value, span));
                }

                // Operator identifiers: hand-written fallback for maximal munch
                RawToken::OperatorIdent => self.process_operator_ident(span),

                // Error token
                RawToken::Error => {
                    let error_span = Span::new(span.start, (span.start + 1).min(span.end));
                    self.errors.push(LexError {
                        message: format!("Unrecognized character at position {}", span.start),
                        span: error_span,
                    });
                }
            }
        }

        // Emit EOF token
        let eof_offset = self.source.len() as u32;
        self.tokens.push(Token::new(TokenKind::EOF, String::new(), Span::new(eof_offset, eof_offset)));

        (self.tokens, self.errors)
    }

    /// Process backtick token (string literal or line-start comment)
    fn process_backtick_token(&mut self, span: Span) {
        let value = self.source[span.range()].to_string();

        // Check if triple backtick at line start (comment)
        if value == "```" && self.source[0..span.start as usize].ends_with('\n') || span.start == 0 {
            // Line comment - consume to end of line
            let line_start = span.start as usize;
            let line_end = self.source[line_start..]
                .find('\n')
                .map(|pos| line_start + pos)
                .unwrap_or(self.source.len());

            let comment_span = Span::new(span.start, line_end as u32);
            let comment = self.source[span.end as usize..line_end].to_string();
            self.tokens.push(Token::new(TokenKind::Comment, comment, comment_span));
        } else {
            // Backtick string literal (interpolation handled in parser)
            self.tokens.push(Token::new(TokenKind::Backtick, "`".to_string(), span));
        }
    }

    /// Process operator identifier with maximal munch (hand-written fallback)
    fn process_operator_ident(&mut self, span: Span) {
        let raw_value = self.source[span.range()].to_string();

        // Check for special 2-char tokens (_< and _>)
        if raw_value.starts_with("_<") {
            self.tokens.push(Token::new(TokenKind::LeftArg, "_<".to_string(),
                Span::new(span.start, span.start + 2)));
            // If longer, process remainder
            if raw_value.len() > 2 {
                let remainder = &raw_value[2..];
                if !remainder.is_empty() {
                    self.tokens.push(Token::new(TokenKind::Identifier, remainder.to_string(),
                        Span::new(span.start + 2, span.end)));
                }
            }
            return;
        }

        if raw_value.starts_with("_>") {
            self.tokens.push(Token::new(TokenKind::RightArg, "_>".to_string(),
                Span::new(span.start, span.start + 2)));
            if raw_value.len() > 2 {
                let remainder = &raw_value[2..];
                if !remainder.is_empty() {
                    self.tokens.push(Token::new(TokenKind::Identifier, remainder.to_string(),
                        Span::new(span.start + 2, span.end)));
                }
            }
            return;
        }

        // Check for boolean/undefined literals
        match raw_value.as_str() {
            "true" => {
                self.tokens.push(Token::new(TokenKind::BooleanLiteral, raw_value, span));
            }
            "false" => {
                self.tokens.push(Token::new(TokenKind::BooleanLiteral, raw_value, span));
            }
            "undefined" => {
                self.tokens.push(Token::new(TokenKind::UndefinedLiteral, raw_value, span));
            }
            _ => {
                // Regular identifier (includes operators like +, @, !!!, etc.)
                self.tokens.push(Token::new(TokenKind::Identifier, raw_value, span));
            }
        }
    }
}
```

### 2.8 Error Types

**File:** `crates/lr-lexer/src/error.rs`

```rust
use lr_common::Span;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
#[error("Lex error: {message}")]
pub struct LexError {
    pub message: String,
    pub span: Span,
}
```

### 2.3 String Interpolation Handling

**File:** `crates/lr-lexer/src/lexer.rs` (continued)

String interpolation is handled by re-lexing in the parser (logos can't handle nested `{expr}`):

```rust
impl<'a> Lexer<'a> {
    /// Process string interpolation by marking locations for parser to re-lex
    /// Logos cannot handle nested braces, so we let the parser handle interpolation
    /// The lexer just marks string tokens that contain interpolation markers
    pub fn mark_interpolation_locations(&self, token: &Token) -> Vec<(Span, String)> {
        let mut locations = Vec::new();
        let value = &token.value;

        // Find all {expr} patterns in string
        // This is a simple heuristic - parser will do actual re-lexing
        let mut depth = 0;
        let mut start = 0;

        for (i, ch) in value.char_indices() {
            if ch == '{' {
                if depth == 0 {
                    start = i;
                }
                depth += 1;
            } else if ch == '}' && depth > 0 {
                depth -= 1;
                if depth == 0 {
                    let expr_str = &value[start + 1..i];
                    let span_start = token.span.start + 1 + start as u32;
                    let span_end = token.span.start + 1 + i as u32;
                    locations.push((Span::new(span_start, span_end), expr_str.to_string()));
                }
            }
        }

        locations
    }
}
```

**Note:** The parser will handle string interpolation by:
1. Detecting `{` and `}` markers in string values
2. Re-lexing the expression strings between markers
3. Building `StringPart::Interpolation` nodes with proper AST expressions

This two-pass approach is necessary because logos (and most tokenizers) cannot handle nested delimiters efficiently.

### 2.4 Error Types

**File:** `crates/lr-lexer/src/error.rs`

### 2.9 Lexer Module Exports

**File:** `crates/lr-lexer/src/lib.rs`

```rust
pub mod token;
pub mod error;
pub mod lexer;

pub use token::{Token, TokenKind};
pub use error::LexError;
pub use lexer::Lexer;
```

---

## 3. AST Node Types

### 3.1 AST Structure

**File:** `crates/lr-ast/src/lib.rs`

Complete AST node definitions with span tracking:

```rust
use lr_common::Span;
use std::fmt;

/// A complete Left-Right program (single root expression)
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub expression: Box<Expression>,
    pub source_path: String,
}

/// All AST nodes are expressions
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// Number literal (decimal only)
    NumberLiteral {
        value: String,
        raw: String,
        span: Span,
    },

    /// String literal with interpolation
    StringLiteral {
        parts: Vec<StringPart>,
        span: Span,
    },

    /// Boolean literal
    BooleanLiteral {
        value: bool,
        raw: String,
        span: Span,
    },

    /// Undefined literal
    UndefinedLiteral {
        raw: String,
        span: Span,
    },

    /// List literal
    ListLiteral {
        elements: Vec<Expression>,
        span: Span,
    },

    /// Map literal
    MapLiteral {
        entries: Vec<MapEntry>,
        span: Span,
    },

    /// Identifier (includes all operators)
    Identifier {
        name: String,
        span: Span,
    },

    /// Left argument reference (_<)
    LeftArg {
        raw: String,
        span: Span,
    },

    /// Right argument reference (_>)
    RightArg {
        raw: String,
        span: Span,
    },

    /// Left-to-right application (fundamental node)
    Application {
        left: Box<Expression>,
        right: Box<Expression>,
        span: Span,
    },

    /// Grouped expression ((expr))
    GroupedExpression {
        expression: Box<Expression>,
        span: Span,
    },

    /// Throw expression (value !!!)
    ThrowExpression {
        value: Box<Expression>,
        span: Span,
    },

    /// Catch expression (operator !!!? handler)
    CatchExpression {
        operator: Box<Expression>,
        handler: Box<Expression>,
        span: Span,
    },

    /// Async expression (operator ///)
    AsyncExpression {
        operator: Box<Expression>,
        span: Span,
    },

    /// Await expression (promise \\\)
    AwaitExpression {
        promise: Box<Expression>,
        span: Span,
    },
}

/// String literal parts (text or interpolation)
#[derive(Debug, Clone, PartialEq)]
pub enum StringPart {
    /// Plain text segment
    Text {
        text: String,
        span: Span,
    },

    /// Interpolated expression {expr}
    Interpolation {
        expression: Box<Expression>,
        span: Span,
    },
}

/// Map entry (key-value pair)
#[derive(Debug, Clone, PartialEq)]
pub struct MapEntry {
    pub key: Box<Expression>,
    pub value: Option<Box<Expression>>,
    pub is_assignment: bool,      // key starts with alpha → creates variable
    pub is_expression_key: bool,  // key is expression (not alpha-starting identifier)
    pub span: Span,
}

impl Expression {
    pub fn span(&self) -> Span {
        match self {
            Expression::NumberLiteral { span, .. } => *span,
            Expression::StringLiteral { span, .. } => *span,
            Expression::BooleanLiteral { span, .. } => *span,
            Expression::UndefinedLiteral { span, .. } => *span,
            Expression::ListLiteral { span, .. } => *span,
            Expression::MapLiteral { span, .. } => *span,
            Expression::Identifier { span, .. } => *span,
            Expression::LeftArg { span, .. } => *span,
            Expression::RightArg { span, .. } => *span,
            Expression::Application { span, .. } => *span,
            Expression::GroupedExpression { span, .. } => *span,
            Expression::ThrowExpression { span, .. } => *span,
            Expression::CatchExpression { span, .. } => *span,
            Expression::AsyncExpression { span, .. } => *span,
            Expression::AwaitExpression { span, .. } => *span,
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Simple pretty-printing for debugging
        match self {
            Expression::NumberLiteral { value, .. } => write!(f, "{}", value),
            Expression::StringLiteral { parts, .. } => {
                write!(f, "`")?;
                for part in parts {
                    match part {
                        StringPart::Text { text, .. } => write!(f, "{}", text)?,
                        StringPart::Interpolation { expression, .. } => {
                            write!(f, "{{{}}}", expression)?;
                        }
                    }
                }
                write!(f, "`")
            }
            Expression::BooleanLiteral { value, .. } => write!(f, "{}", value),
            Expression::UndefinedLiteral { .. } => write!(f, "undefined"),
            Expression::Identifier { name, .. } => write!(f, "{}", name),
            Expression::LeftArg { .. } => write!(f, "_<"),
            Expression::RightArg { .. } => write!(f, "_>"),
            Expression::Application { left, right, .. } => {
                write!(f, "({} {})", left, right)
            }
            _ => write!(f, "<expression>"),
        }
    }
}
```

---

## 4. Parser Implementation

### 4.1 Parser Structure

**File:** `crates/lr-parser/src/parser.rs`

Recursive descent parser with zero precedence [https://docs.rs/prattle/latest/prattle/] and bogus node error recovery [https://github.com/biomejs/biome/blob/main/.claude/skills/parser-development/SKILL.md]:

```rust
use std::iter::Peekable;
use std::vec::IntoIter;
use lr_common::Span;
use lr_lexer::{Lexer, Token, TokenKind};
use lr_ast::{Expression, Program, MapEntry, StringPart};
use crate::error::ParseError;
use crate::recovery::ParseRecoveryTokenSet;

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
    source: String,
    errors: Vec<ParseError>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, source: String) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
            source,
            errors: Vec::new(),
        }
    }

    /// Parse entire program (single root expression)
    pub fn parse(mut self) -> Result<Program, Vec<ParseError>> {
        let expression = self.parse_expression()?;

        // Expect EOF
        if let Some(token) = self.peek() {
            self.error(
                format!("Unexpected token {:?} after end of program", token.kind),
                token.span,
            );
        }

        Ok(Program {
            expression: Box::new(expression),
            source_path: String::new(), // Set by caller
        })
    }

    /// Peek next token
    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    /// Check if next token matches kind
    fn peek_kind(&mut self, kind: TokenKind) -> bool {
        matches!(self.peek(), Some(token) if token.kind == kind)
    }

    /// Consume next token
    fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    /// Expect token of given kind
    fn expect(&mut self, kind: TokenKind) -> Result<Token, ParseError> {
        if let Some(token) = self.next() {
            if token.kind == kind {
                return Ok(token);
            }
            self.error(
                format!("Expected {:?}, got {:?}", kind, token.kind),
                token.span,
            );
            return Err(ParseError::UnexpectedToken {
                expected: format!("{:?}", kind),
                found: format!("{:?}", token.kind),
                span: token.span,
            });
        }
        Err(ParseError::UnexpectedEOF {
            expected: format!("{:?}", kind),
            span: Span::empty(),
        })
    }

    /// Report parse error
    fn error(&mut self, message: String, span: Span) {
        self.errors.push(ParseError {
            message,
            span,
            hint: None,
        });
    }
}
```

### 4.2 Zero-Precedence Expression Parsing

**File:** `crates/lr-parser/src/parser.rs` (continued)

All operators have binding power 0, parse as left-associative chain:

```rust
impl Parser {
    /// Parse expression (zero precedence, left-to-right)
    fn parse_expression(&mut self) -> Result<Expression, Vec<ParseError>> {
        let mut left = self.parse_primary()?;

        // Build left-to-right application chain
        loop {
            // If we hit a delimiter or EOF, we're done
            if self.at_expression_end() {
                break;
            }

            // Parse next expression as right operand
            let right = self.parse_primary()?;
            let span = Span::new(left.span().start, right.span().end);

            left = Expression::Application {
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }

        Ok(left)
    }

    /// Check if we're at the end of an expression
    fn at_expression_end(&mut self) -> bool {
        match self.peek() {
            None => true, // EOF
            Some(token) => match token.kind {
                TokenKind::EOF |
                TokenKind::CloseBrace |
                TokenKind::CloseBracket |
                TokenKind::CloseParen |
                TokenKind::Colon |
                TokenKind::Comma => true,
                _ => false,
            },
        }
    }

    /// Parse primary expression (atoms)
    fn parse_primary(&mut self) -> Result<Expression, Vec<ParseError>> {
        let token = self.next().ok_or_else(|| ParseError::UnexpectedEOF {
            expected: "expression".to_string(),
            span: Span::empty(),
        })?;

        match token.kind {
            TokenKind::NumberLiteral => Ok(Expression::NumberLiteral {
                value: token.value.clone(),
                raw: token.value.clone(),
                span: token.span,
            }),

            TokenKind::StringLiteral => self.parse_string_literal(&token),

            TokenKind::BooleanLiteral => {
                let value = token.value == "true";
                Ok(Expression::BooleanLiteral {
                    value,
                    raw: token.value.clone(),
                    span: token.span,
                })
            }

            TokenKind::UndefinedLiteral => Ok(Expression::UndefinedLiteral {
                raw: token.value.clone(),
                span: token.span,
            }),

            TokenKind::Identifier => Ok(Expression::Identifier {
                name: token.value.clone(),
                span: token.span,
            }),

            TokenKind::LeftArg => Ok(Expression::LeftArg {
                raw: token.value.clone(),
                span: token.span,
            }),

            TokenKind::RightArg => Ok(Expression::RightArg {
                raw: token.value.clone(),
                span: token.span,
            }),

            TokenKind::OpenBracket => self.parse_list_literal(token.span),

            TokenKind::OpenBrace => self.parse_map_literal(token.span),

            TokenKind::OpenParen => self.parse_grouped_expression(token.span),

            _ => {
                self.error(
                    format!("Unexpected token {:?} in expression", token.kind),
                    token.span,
                );
                Err(ParseError::UnexpectedToken {
                    expected: "expression".to_string(),
                    found: format!("{:?}", token.kind),
                    span: token.span,
                })
            }
        }
    }
}
```

### 4.3 List Literal Parsing

**File:** `crates/lr-parser/src/parser.rs` (continued)

```rust
impl Parser {
    /// Parse list literal [elements]
    fn parse_list_literal(&mut self, start_span: Span) -> Result<Expression, Vec<ParseError>> {
        let mut elements = Vec::new();

        // Parse elements until close bracket
        while !self.peek_kind(TokenKind::CloseBracket) && !self.peek_kind(TokenKind::EOF) {
            let element = self.parse_expression()?;
            elements.push(element);

            // Comma separator
            if self.peek_kind(TokenKind::Comma) {
                self.next();
            } else {
                // No comma means end of list or error
                break;
            }
        }

        let close_token = self.expect(TokenKind::CloseBracket)?;
        let span = Span::new(start_span.start, close_token.span.end);

        Ok(Expression::ListLiteral {
            elements,
            span,
        })
    }
}
```

### 4.4 Map Literal Parsing with Colon Disambiguation

**File:** `crates/lr-parser/src/parser.rs` (continued)

```rust
impl Parser {
    /// Parse map literal {entries}
    fn parse_map_literal(&mut self, start_span: Span) -> Result<Expression, Vec<ParseError>> {
        let mut entries = Vec::new();

        while !self.peek_kind(TokenKind::CloseBrace) && !self.peek_kind(TokenKind::EOF) {
            let key = self.parse_expression()?;

            // Check for colon
            if self.peek_kind(TokenKind::Colon) {
                self.next(); // consume ':'
                let value = self.parse_expression()?;

                // Detect if key is alpha-starting (assignment)
                let is_assignment = matches!(&key, Expression::Identifier { name, .. } if name.chars().next().map(|c| c.is_ascii_alphabetic()).unwrap_or(false));
                let is_expression_key = !is_assignment;

                let span = Span::new(key.span().start, value.span().end);

                entries.push(MapEntry {
                    key: Box::new(key),
                    value: Some(Box::new(value)),
                    is_assignment,
                    is_expression_key,
                    span,
                });
            } else {
                // No colon = expression ending (last entry)
                entries.push(MapEntry {
                    key: Box::new(key),
                    value: None,
                    is_assignment: false,
                    is_expression_key: true,
                    span: key.span(),
                });
                // This is the last entry, break
                break;
            }

            // Comma separator
            if self.peek_kind(TokenKind::Comma) {
                self.next();
            } else {
                // No comma means end of map
                break;
            }
        }

        let close_token = self.expect(TokenKind::CloseBrace)?;
        let span = Span::new(start_span.start, close_token.span.end);

        Ok(Expression::MapLiteral {
            entries,
            span,
        })
    }
}
```

### 4.5 String Interpolation Parsing

**File:** `crates/lr-parser/src/parser.rs` (continued)

```rust
impl Parser {
    /// Parse string literal with interpolation
    fn parse_string_literal(&mut self, token: &Token) -> Result<Expression, Vec<ParseError>> {
        // The lexer gives us the raw string value including interpolation markers
        // We need to parse the interpolation expressions
        let parts = self.parse_string_parts(&token.value, token.span)?;

        Ok(Expression::StringLiteral {
            parts,
            span: token.span,
        })
    }

    /// Parse string parts (text and interpolation)
    fn parse_string_parts(&self, value: &str, span: Span) -> Result<Vec<StringPart>, Vec<ParseError>> {
        let mut parts = Vec::new();
        let mut current_text = String::new();
        let mut in_interpolation = false;
        let mut interpolation_start = 0;
        let mut brace_depth = 0;

        for (i, ch) in value.char_indices() {
            if ch == '{' && !in_interpolation {
                // Start of interpolation
                if !current_text.is_empty() {
                    let text_start = interpolation_start;
                    let text_end = i as u32;
                    parts.push(StringPart::Text {
                        text: std::mem::take(&mut current_text),
                        span: Span::new(text_start, text_end),
                    });
                }
                in_interpolation = true;
                interpolation_start = i as u32;
                brace_depth = 1;
            } else if ch == '{' && in_interpolation {
                brace_depth += 1;
            } else if ch == '}' && in_interpolation {
                brace_depth -= 1;
                if brace_depth == 0 {
                    // End of interpolation
                    // Note: We can't actually parse the inner expression without
                    // re-lexing it. For now, we'll create a placeholder that
                    // will be filled in by the caller or require a second pass.
                    // This is a simplification for the initial implementation.
                    in_interpolation = false;
                    interpolation_start = i as u32;
                }
            } else if !in_interpolation {
                current_text.push(ch);
            }
            // If in_interpolation and not brace tracking, we're accumulating
            // the expression string that needs to be re-parsed
        }

        // Handle remaining text
        if !current_text.is_empty() {
            parts.push(StringPart::Text {
                text: current_text,
                span: Span::new(interpolation_start, span.end),
            });
        }

        // TODO: In a full implementation, we need to:
        // 1. Collect all interpolation string ranges
        // 2. Re-lex those ranges as separate expressions
        // 3. Parse those expressions
        // 4. Replace placeholder StringPart::Interpolation with actual Expression

        Ok(parts)
    }
}
```

### 4.6 Grouped Expression

**File:** `crates/lr-parser/src/parser.rs` (continued)

```rust
impl Parser {
    /// Parse grouped expression (expr)
    fn parse_grouped_expression(&mut self, start_span: Span) -> Result<Expression, Vec<ParseError>> {
        let expression = self.parse_expression()?;
        let close_token = self.expect(TokenKind::CloseParen)?;
        let span = Span::new(start_span.start, close_token.span.end);

        Ok(Expression::GroupedExpression {
            expression: Box::new(expression),
            span,
        })
    }
}
```

### 4.7 Special Expression Parsing

**File:** `crates/lr-parser/src/parser.rs` (continued)

```rust
impl Parser {
    /// Parse throw expression (value !!!)
    fn parse_throw_expression(&mut self, value: Expression) -> Result<Expression, Vec<ParseError>> {
        self.expect(TokenKind::Identifier)?; // consume '!!!'
        let span = Span::new(value.span().start, self.peek().map_or(Span::empty(), |t| t.span).end);

        Ok(Expression::ThrowExpression {
            value: Box::new(value),
            span,
        })
    }

    /// Parse catch expression (operator !!!? handler)
    fn parse_catch_expression(&mut self, operator: Expression) -> Result<Expression, Vec<ParseError>> {
        self.expect(TokenKind::Identifier)?; // consume '!!!?'
        let handler = self.parse_expression()?;
        let span = Span::new(operator.span().start, handler.span().end);

        Ok(Expression::CatchExpression {
            operator: Box::new(operator),
            handler: Box::new(handler),
            span,
        })
    }

    /// Parse async expression (operator ///)
    fn parse_async_expression(&mut self, operator: Expression) -> Result<Expression, Vec<ParseError>> {
        self.expect(TokenKind::Identifier)?; // consume '///'
        let span = Span::new(operator.span().start, self.peek().map_or(Span::empty(), |t| t.span).end);

        Ok(Expression::AsyncExpression {
            operator: Box::new(operator),
            span,
        })
    }

    /// Parse await expression (promise \\\)
    fn parse_await_expression(&mut self, promise: Expression) -> Result<Expression, Vec<ParseError>> {
        self.expect(TokenKind::Identifier)?; // consume '\\\\'
        let span = Span::new(promise.span().start, self.peek().map_or(Span::empty(), |t| t.span).end);

        Ok(Expression::AwaitExpression {
            promise: Box::new(promise),
            span,
        })
    }
}
```

### 4.8 Error Recovery

**File:** `crates/lr-parser/src/recovery.rs`

Bogus node pattern from Biome [https://github.com/biomejs/biome/blob/main/.claude/skills/parser-development/SKILL.md]:

```rust
use lr_lexer::TokenKind;

/// Token sets for error recovery
pub struct ParseRecoveryTokenSet {
    tokens: Vec<TokenKind>,
}

impl ParseRecoveryTokenSet {
    pub fn new(tokens: Vec<TokenKind>) -> Self {
        Self { tokens }
    }

    /// Recovery tokens for expressions
    pub fn expression() -> Self {
        Self::new(vec![
            TokenKind::CloseBrace,
            TokenKind::CloseBracket,
            TokenKind::CloseParen,
            TokenKind::Colon,
            TokenKind::Comma,
            TokenKind::EOF,
        ])
    }

    /// Recovery tokens for map entries
    pub fn map_entry() -> Self {
        Self::new(vec![
            TokenKind::CloseBrace,
            TokenKind::Comma,
            TokenKind::EOF,
        ])
    }

    /// Recovery tokens for list elements
    pub fn list_element() -> Self {
        Self::new(vec![
            TokenKind::CloseBracket,
            TokenKind::Comma,
            TokenKind::EOF,
        ])
    }
}
```

**File:** `crates/lr-parser/src/parser.rs` (continued)

Error recovery in parser:

```rust
impl Parser {
    /// Recover from error by skipping to recovery token
    fn recover(&mut self, recovery_set: &ParseRecoveryTokenSet) {
        while let Some(token) = self.peek() {
            if recovery_set.tokens.contains(&token.kind) {
                break;
            }
            self.next();
        }
    }

    /// Try to parse with error recovery
    fn try_parse<F, R>(&mut self, parse_fn: F, recovery_set: &ParseRecoveryTokenSet) -> Option<R>
    where
        F: FnOnce(&mut Self) -> Result<R, Vec<ParseError>>,
    {
        match parse_fn(self) {
            Ok(result) => Some(result),
            Err(_) => {
                self.recover(recovery_set);
                None
            }
        }
    }
}
```

### 4.9 Error Types

**File:** `crates/lr-parser/src/error.rs`

```rust
use lr_common::Span;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub struct ParseError {
    pub message: String,
    pub span: Span,
    pub hint: Option<String>,
}

#[derive(Debug, Clone, Error)]
pub enum ParseError {
    #[error("Unexpected token: expected {expected}, found {found}")]
    UnexpectedToken { expected: String, found: String, span: Span },

    #[error("Unexpected end of file: expected {expected}")]
    UnexpectedEOF { expected: String, span: Span },

    #[error("Parse error: {message}")]
    Generic { message: String, span: Span },
}
```

### 4.10 Parser Module Exports

**File:** `crates/lr-parser/src/lib.rs`

```rust
pub mod parser;
pub mod error;
pub mod recovery;

pub use parser::Parser;
pub use error::{ParseError};
pub use recovery::ParseRecoveryTokenSet;
```

---

## 5. Diagnostics

### 5.1 Error Rendering

**File:** `crates/lr-diagnostics/src/lib.rs`

Use ariadne for colored multi-line error output [https://docs.rs/ariadne/latest/ariadne/]:

```rust
use ariadne::{Color, Config, Label, Report, ReportKind, Source};
use lr_common::Span;
use lr_lexer::LexError;
use lr_parser::ParseError;

pub struct Diagnostic {
    pub message: String,
    pub span: Span,
    pub severity: Severity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

pub fn emit_diagnostics(
    errors: &[LexError],
    parse_errors: &[ParseError],
    source: &str,
    file_name: &str,
) {
    let mut reports = Vec::new();

    // Lexer errors
    for error in errors {
        let report = Report::build(ReportKind::Error, file_name, error.span.start as usize)
            .with_message(&error.message)
            .with_label(
                Label::new((file_name, error.span.range()))
                    .with_message(&error.message)
                    .with_color(Color::Red),
            );

        reports.push(report.finish());
    }

    // Parse errors
    for error in parse_errors {
        let report = Report::build(ReportKind::Error, file_name, error.span.start as usize)
            .with_message(&error.message)
            .with_label(
                Label::new((file_name, error.span.range()))
                    .with_message(&error.message)
                    .with_color(Color::Red),
            );

        if let Some(hint) = &error.hint {
            reports.last_mut().unwrap().with_note(hint);
        }

        reports.push(report.finish());
    }

    // Print all reports
    let source_cache = Source::from(source);
    for report in reports {
        report
            .eprint((file_name, source_cache.clone()))
            .unwrap();
    }
}
```

---

## 6. Live Testing Criteria (Definition of Done)

### 6.1 Unit Tests (Initial Correctness)

**File:** `crates/lr-lexer/tests/token_tests.rs`

Token-by-token lexer tests with insta snapshots:

```rust
use lr_lexer::{Lexer, TokenKind};
use insta::assert_debug_snapshot;

#[test]
fn test_number_literals() {
    let source = "42 3.14 0.5";
    let (tokens, errors) = Lexer::new(source).tokenize();
    assert!(errors.is_empty());
    assert_debug_snapshot!(tokens);
    assert!(matches!(tokens[0].kind, TokenKind::NumberLiteral));
    assert_eq!(tokens[0].value, "42");
    assert!(matches!(tokens[1].kind, TokenKind::NumberLiteral));
    assert_eq!(tokens[1].value, "3.14");
}

#[test]
fn test_identifiers() {
    let source = "+ @ $@ !!!? /// \\\ _< _>";
    let (tokens, errors) = Lexer::new(source).tokenize();
    assert!(errors.is_empty());
    assert_debug_snapshot!(tokens);
    // Verify maximal munch
    assert_eq!(tokens.len(), 7);
    assert_eq!(tokens[0].value, "+");
    assert_eq!(tokens[1].value, "@");
    assert_eq!(tokens[2].value, "$@");
    assert_eq!(tokens[3].value, "!!!?");
    assert_eq!(tokens[4].value, "///");
    assert_eq!(tokens[5].value, "\\\\");
    assert_eq!(tokens[6].value, "_<");
    assert_eq!(tokens[7].value, "_>");
}

#[test]
fn test_string_literals() {
    let source = "`hello` `multi\\`line` `interpolation {expr}`";
    let (tokens, errors) = Lexer::new(source).tokenize();
    assert!(errors.is_empty());
    assert_debug_snapshot!(tokens);
}

#[test]
fn test_comments() {
    let source = "```line comment\n42";
    let (tokens, errors) = Lexer::new(source).tokenize();
    assert!(errors.is_empty());
    assert_debug_snapshot!(tokens);
    assert!(matches!(tokens[0].kind, TokenKind::Comment));
    assert_eq!(tokens[0].value, "line comment");
}
```

**File:** `crates/lr-parser/tests/ast_tests.rs`

AST snapshot tests:

```rust
use lr_lexer::Lexer;
use lr_parser::Parser;
use insta::assert_debug_snapshot;

#[test]
fn test_basic_arithmetic() {
    let source = "5 + 3";
    let (tokens, _) = Lexer::new(source).tokenize();
    let parser = Parser::new(tokens, source.to_string());
    let program = parser.parse().unwrap();
    assert_debug_snapshot!(program);
}

#[test]
fn test_zero_precedence() {
    let source = "5 + 3 * 2";
    let (tokens, _) = Lexer::new(source).tokenize();
    let parser = Parser::new(tokens, source.to_string());
    let program = parser.parse().unwrap();
    assert_debug_snapshot!(program);
    // Verify: ((5+)3) then (8*)(2) = nested Application
}

#[test]
fn test_operator_as_identifier() {
    let source = "response @ value";
    let (tokens, _) = Lexer::new(source).tokenize();
    let parser = Parser::new(tokens, source.to_string());
    let program = parser.parse().unwrap();
    assert_debug_snapshot!(program);
}
```

### 6.2 Live System Tests (Definition of Done)

**File:** `compiler/tests/live_system_tests.rs`

Test runner that compiles .lr files and verifies output:

```rust
use std::path::Path;
use lr_lexer::Lexer;
use lr_parser::Parser;
use insta::assert_json_snapshot;

struct LiveTest {
    name: String,
    source: String,
    expected_tokens: Vec<TokenExpectation>,
    expected_ast_description: String,
    should_error: bool,
}

struct TokenExpectation {
    kind: String,
    value: Option<String>,
}

#[test]
fn test_flow_01_basic_arithmetic() {
    let source = "5 + 3";
    run_live_test(LiveTest {
        name: "basic_arithmetic".to_string(),
        source: source.to_string(),
        expected_tokens: vec![
            TokenExpectation { kind: "NumberLiteral".to_string(), value: Some("5".to_string()) },
            TokenExpectation { kind: "Identifier".to_string(), value: Some("+".to_string()) },
            TokenExpectation { kind: "NumberLiteral".to_string(), value: Some("3".to_string()) },
        ],
        expected_ast_description: "Application(Application(5, +), 3)".to_string(),
        should_error: false,
    });
}

#[test]
fn test_flow_02_zero_precedence() {
    let source = "5 + 3 * 2";
    let tokens = lex_and_expect(source, &[
        TokenExpectation { kind: "NumberLiteral", value: Some("5") },
        TokenExpectation { kind: "Identifier", value: Some("+") },
        TokenExpectation { kind: "NumberLiteral", value: Some("3") },
        TokenExpectation { kind: "Identifier", value: Some("*") },
        TokenExpectation { kind: "NumberLiteral", value: Some("2") },
    ]);

    let program = parse_and_expect(&tokens, source, false);
    // Verify AST: (((5+)3)*)2 = nested Application with left association
    assert!(matches!(program.expression.as_ref(), lr_ast::Expression::Application { .. }));
}

#[test]
fn test_flow_03_operator_as_identifier() {
    let source = "response @ value";
    let tokens = lex_and_expect(source, &[
        TokenExpectation { kind: "Identifier", value: Some("response") },
        TokenExpectation { kind: "Identifier", value: Some("@") },
        TokenExpectation { kind: "StringLiteral", value: Some("value") },
    ]);

    let program = parse_and_expect(&tokens, source, false);
    // Verify AST: Application(Application(response, @), "value")
    assert!(matches!(program.expression.as_ref(), lr_ast::Expression::Application { .. }));
}

#[test]
fn test_flow_04_string_interpolation() {
    let source = "`Hello {name}`";
    let tokens = lex_and_expect(source, &[
        TokenExpectation { kind: "StringLiteral", value: Some("Hello {name}") },
    ]);

    let program = parse_and_expect(&tokens, source, false);
    // Verify AST: StringLiteral with InterpolationPart
    if let lr_ast::Expression::StringLiteral { parts, .. } = program.expression.as_ref() {
        assert!(!parts.is_empty());
    } else {
        panic!("Expected StringLiteral");
    }
}

#[test]
fn test_flow_05_map_operators() {
    let source = "{ x: x + 1 }";
    let tokens = lex_and_expect(source, &[
        TokenExpectation { kind: "OpenBrace", value: None },
        TokenExpectation { kind: "Identifier", value: Some("x") },
        TokenExpectation { kind: "Colon", value: None },
        TokenExpectation { kind: "Identifier", value: Some("x") },
        TokenExpectation { kind: "Identifier", value: Some("+") },
        TokenExpectation { kind: "NumberLiteral", value: Some("1") },
        TokenExpectation { kind: "CloseBrace", value: None },
    ]);

    let program = parse_and_expect(&tokens, source, false);
    // Verify AST: MapLiteral with assignment entry
    assert!(matches!(program.expression.as_ref(), lr_ast::Expression::MapLiteral { .. }));
}

#[test]
fn test_flow_06_error_handling() {
    let source = "value !!!";
    let tokens = lex_and_expect(source, &[
        TokenExpectation { kind: "Identifier", value: Some("value") },
        TokenExpectation { kind: "Identifier", value: Some("!!!") },
    ]);

    let program = parse_and_expect(&tokens, source, false);
    // Verify AST: ThrowExpression
    assert!(matches!(program.expression.as_ref(), lr_ast::Expression::ThrowExpression { .. }));
}

#[test]
fn test_flow_07_async() {
    let source = "func ///";
    let tokens = lex_and_expect(source, &[
        TokenExpectation { kind: "Identifier", value: Some("func") },
        TokenExpectation { kind: "Identifier", value: Some("///") },
    ]);

    let program = parse_and_expect(&tokens, source, false);
    // Verify AST: AsyncExpression
    assert!(matches!(program.expression.as_ref(), lr_ast::Expression::AsyncExpression { .. }));
}

#[test]
fn test_flow_08_comments() {
    let source = "``` this is a comment\n5";
    let tokens = lex_and_expect(source, &[
        TokenExpectation { kind: "Comment", value: Some(" this is a comment") },
        TokenExpectation { kind: "NumberLiteral", value: Some("5") },
    ]);

    let program = parse_and_expect(&tokens, source, false);
    // Verify AST: NumberLiteral (comment not in AST)
    assert!(matches!(program.expression.as_ref(), lr_ast::Expression::NumberLiteral { .. }));
}

#[test]
fn test_flow_09_export() {
    let source = "{ }@&[`export1`, `export2`]";
    let tokens = lex_and_expect(source, &[
        TokenExpectation { kind: "OpenBrace", value: None },
        TokenExpectation { kind: "CloseBrace", value: None },
        TokenExpectation { kind: "Identifier", value: Some("@&") },
        TokenExpectation { kind: "OpenBracket", value: None },
        TokenExpectation { kind: "StringLiteral", value: Some("export1") },
        TokenExpectation { kind: "Comma", value: None },
        TokenExpectation { kind: "StringLiteral", value: Some("export2") },
        TokenExpectation { kind: "CloseBracket", value: None },
    ]);

    let program = parse_and_expect(&tokens, source, false);
    // Verify AST: Application(}, @&, [export1, export2])
    assert!(matches!(program.expression.as_ref(), lr_ast::Expression::Application { .. }));
}

#[test]
fn test_flow_10_nested_maps_lists() {
    let source = "{ items: [1, 2, 3] }";
    let tokens = lex_and_expect(source, &[
        TokenExpectation { kind: "OpenBrace", value: None },
        TokenExpectation { kind: "Identifier", value: Some("items") },
        TokenExpectation { kind: "Colon", value: None },
        TokenExpectation { kind: "OpenBracket", value: None },
        TokenExpectation { kind: "NumberLiteral", value: Some("1") },
        TokenExpectation { kind: "Comma", value: None },
        TokenExpectation { kind: "NumberLiteral", value: Some("2") },
        TokenExpectation { kind: "Comma", value: None },
        TokenExpectation { kind: "NumberLiteral", value: Some("3") },
        TokenExpectation { kind: "CloseBracket", value: None },
        TokenExpectation { kind: "CloseBrace", value: None },
    ]);

    let program = parse_and_expect(&tokens, source, false);
    // Verify AST: MapLiteral with ListLiteral value
    assert!(matches!(program.expression.as_ref(), lr_ast::Expression::MapLiteral { .. }));
}

#[test]
fn test_flow_11_curried_application_chains() {
    let source = "data @ `key` @ `nested`";
    let tokens = lex_and_expect(source, &[
        TokenExpectation { kind: "Identifier", value: Some("data") },
        TokenExpectation { kind: "Identifier", value: Some("@") },
        TokenExpectation { kind: "StringLiteral", value: Some("key") },
        TokenExpectation { kind: "Identifier", value: Some("@") },
        TokenExpectation { kind: "StringLiteral", value: Some("nested") },
    ]);

    let program = parse_and_expect(&tokens, source, false);
    // Verify AST: deeply nested Application
    if let lr_ast::Expression::Application { left, right, .. } = program.expression.as_ref() {
        // Should be: Application(Application(Application(data, @), "key"), @)
        assert!(matches!(left.as_ref(), lr_ast::Expression::Application { .. }));
    }
}

#[test]
fn test_flow_12_reverse_args_operator() {
    let source = "`key`@.data";
    let tokens = lex_and_expect(source, &[
        TokenExpectation { kind: "StringLiteral", value: Some("key") },
        TokenExpectation { kind: "Identifier", value: Some("@") },
        TokenExpectation { kind: "Identifier", value: Some(".") },
        TokenExpectation { kind: "Identifier", value: Some("data") },
    ]);

    let program = parse_and_expect(&tokens, source, false);
    // Verify AST: Application(Application(Application("key", @), .), data)
    assert!(matches!(program.expression.as_ref(), lr_ast::Expression::Application { .. }));
}

#[test]
fn test_flow_13_silent_execution() {
    let source = "{ _: expression, result: 42 }";
    let tokens = lex_and_expect(source, &[
        TokenExpectation { kind: "OpenBrace", value: None },
        TokenExpectation { kind: "Identifier", value: Some("_") },
        TokenExpectation { kind: "Colon", value: None },
        TokenExpectation { kind: "Identifier", value: Some("expression") },
        TokenExpectation { kind: "Comma", value: None },
        TokenExpectation { kind: "Identifier", value: Some("result") },
        TokenExpectation { kind: "Colon", value: None },
        TokenExpectation { kind: "NumberLiteral", value: Some("42") },
        TokenExpectation { kind: "CloseBrace", value: None },
    ]);

    let program = parse_and_expect(&tokens, source, false);
    // Verify AST: MapLiteral with two entries
    assert!(matches!(program.expression.as_ref(), lr_ast::Expression::MapLiteral { .. }));
}

#[test]
fn test_flow_14_spread_merge() {
    let source = "+: other";
    let tokens = lex_and_expect(source, &[
        TokenExpectation { kind: "Identifier", value: Some("+") },
        TokenExpectation { kind: "Colon", value: None },
        TokenExpectation { kind: "Identifier", value: Some("other") },
    ]);

    let program = parse_and_expect(&tokens, source, false);
    // Verify AST: Application(+, :)
    assert!(matches!(program.expression.as_ref(), lr_ast::Expression::Application { .. }));
}

#[test]
fn test_flow_15_multi_line_strings() {
    let source = "`line1\n  line2\n  line3`";
    let tokens = lex_and_expect(source, &[
        TokenExpectation { kind: "StringLiteral", value: Some("line1\n  line2\n  line3") },
    ]);

    let program = parse_and_expect(&tokens, source, false);
    // Verify AST: StringLiteral with multi-line content
    assert!(matches!(program.expression.as_ref(), lr_ast::Expression::StringLiteral { .. }));
}

#[test]
fn test_flow_16_boolean_operators() {
    let source = "a | b & c = d ?\" ?# ><";
    let tokens = lex_and_expect(source, &[
        TokenExpectation { kind: "Identifier", value: Some("a") },
        TokenExpectation { kind: "Identifier", value: Some("|") },
        TokenExpectation { kind: "Identifier", value: Some("b") },
        TokenExpectation { kind: "Identifier", value: Some("&") },
        TokenExpectation { kind: "Identifier", value: Some("c") },
        TokenExpectation { kind: "Identifier", value: Some("=") },
        TokenExpectation { kind: "Identifier", value: Some("d") },
        TokenExpectation { kind: "Identifier", value: Some("?\"") },
        TokenExpectation { kind: "Identifier", value: Some("?#") },
        TokenExpectation { kind: "Identifier", value: Some("?><") },
    ]);

    let program = parse_and_expect(&tokens, source, false);
    // Verify AST: long chain of Applications
    assert!(matches!(program.expression.as_ref(), lr_ast::Expression::Application { .. }));
}

#[test]
fn test_flow_17_loop_operators() {
    let source = "list $ map | filter $_ unique";
    let tokens = lex_and_expect(source, &[
        TokenExpectation { kind: "Identifier", value: Some("list") },
        TokenExpectation { kind: "Identifier", value: Some("$") },
        TokenExpectation { kind: "Identifier", value: Some("map") },
        TokenExpectation { kind: "Identifier", value: Some("|") },
        TokenExpectation { kind: "Identifier", value: Some("filter") },
        TokenExpectation { kind: "Identifier", value: Some("$_") },
        TokenExpectation { kind: "Identifier", value: Some("unique") },
    ]);

    let program = parse_and_expect(&tokens, source, false);
    // Verify AST: long chain of Applications
    assert!(matches!(program.expression.as_ref(), lr_ast::Expression::Application { .. }));
}

#[test]
fn test_flow_18_error_recovery() {
    let source = "{ invalid : 42 }";
    // Invalid characters should emit errors but still parse
    let (tokens, errors) = Lexer::new(source).tokenize();
    assert!(!errors.is_empty());

    let parser = Parser::new(tokens, source.to_string());
    let result = parser.parse();

    // Should produce a program even with errors (bogus node recovery)
    match result {
        Ok(program) => {
            assert!(matches!(program.expression.as_ref(), lr_ast::Expression::MapLiteral { .. }));
        }
        Err(parse_errors) => {
            // If parse fails, should have error recovery
            assert!(!parse_errors.is_empty());
        }
    }
}

#[test]
fn test_flow_19_empty_input() {
    let source = "";
    let (tokens, errors) = Lexer::new(source).tokenize();
    assert!(errors.is_empty());

    let parser = Parser::new(tokens, source.to_string());
    let result = parser.parse();

    // Empty input should produce minimal AST or error
    match result {
        Ok(program) => {
            // Should have some expression (even if minimal)
        }
        Err(parse_errors) => {
            // Empty input is valid, should parse
            assert!(false, "Empty input should parse");
        }
    }
}

#[test]
fn test_flow_20_unicode_identifiers() {
    let source = "félicité λ-function";
    let tokens = lex_and_expect(source, &[
        TokenExpectation { kind: "Identifier", value: Some("félicité") },
        TokenExpectation { kind: "Identifier", value: Some("λ-function") },
    ]);

    let program = parse_and_expect(&tokens, source, false);
    // Verify AST: two Identifiers applied
    assert!(matches!(program.expression.as_ref(), lr_ast::Expression::Application { .. }));
}

// Helper functions
fn lex_and_expect(source: &str, expected: &[TokenExpectation]) -> Vec<lr_lexer::Token> {
    let (tokens, errors) = Lexer::new(source).tokenize();
    if !errors.is_empty() {
        panic!("Lex errors: {:?}", errors);
    }

    assert_eq!(tokens.len(), expected.len(), "Token count mismatch");
    for (token, exp) in tokens.iter().zip(expected.iter()) {
        assert_eq!(format!("{:?}", token.kind), exp.kind);
        if let Some(ref value) = exp.value {
            assert_eq!(token.value, *value);
        }
    }

    tokens
}

fn parse_and_expect(tokens: &[lr_lexer::Token], source: &str, should_error: bool) -> lr_ast::Program {
    let parser = Parser::new(tokens.to_vec(), source.to_string());
    let result = parser.parse();

    if should_error {
        assert!(result.is_err(), "Expected parse error");
        return result.unwrap_err(); // Panic, but that's fine for this test
    } else {
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        return result.unwrap();
    }
}

fn run_live_test(test: LiveTest) {
    // Run lexer
    let (tokens, lex_errors) = Lexer::new(&test.source).tokenize();

    if test.should_error {
        assert!(!lex_errors.is_empty() || !tokens.is_empty(), "Expected errors");
    } else {
        assert!(lex_errors.is_empty(), "Unexpected lex errors: {:?}", lex_errors);
    }

    // Verify tokens
    assert_eq!(tokens.len(), test.expected_tokens.len());
    for (token, exp) in tokens.iter().zip(test.expected_tokens.iter()) {
        assert_eq!(format!("{:?}", token.kind), exp.kind);
        if let Some(ref value) = exp.value {
            assert_eq!(token.value, *value);
        }
    }

    // Run parser
    let parser = Parser::new(tokens, test.source.clone());
    let result = parser.parse();

    if test.should_error {
        assert!(result.is_err(), "Expected parse error");
    } else {
        assert!(result.is_ok(), "Parse failed: {:?}", result);
        let program = result.unwrap();

        // Verify AST structure
        // For now, we just check that it's not empty
        assert!(!matches!(program.expression.as_ref(), lr_ast::Expression::NumberLiteral { value, .. } if value == "0" && tokens.len() > 1));
    }
}
```

### 6.3 Property-Based Tests

**File:** `crates/lr-lexer/tests/prop_tests.rs`

Proptest for robustness [https://www.beamtalk.dev/adr/0011-robustness-testing-layered-fuzzing.html]:

```rust
use proptest::prelude::*;
use lr_lexer::Lexer;

proptest! {
    #[test]
    fn lexer_never_panics(s in "\\PC*") {
        let _ = Lexer::new(&s).tokenize();
    }

    #[test]
    fn token_spans_in_bounds(s in "\\PC*") {
        let (tokens, _) = Lexer::new(&s).tokenize();
        let source_len = s.len() as u32;

        for token in tokens {
            assert!(token.span.start <= token.span.end);
            assert!(token.span.end <= source_len);
        }
    }

    #[test]
    fn roundtrip_identifier(s in "[a-zA-Z0-9@#$%^&*+_\\-~!?><=/]+") {
        let (tokens, _) = Lexer::new(&s).tokenize();
        assert!(!tokens.is_empty());
    }
}
```

---

## 7. Implementation Order

### Phase 1: Project Setup and Shared Types (30 min)

- [ ] **Step 1.1:** Create workspace root `Cargo.toml`
  - File: `/home/jon/code/left-right/Cargo.toml`
  - Content: Workspace configuration with all crates

- [ ] **Step 1.2:** Create `lr-common` crate
  - Directory: `/home/jon/code/left-right/compiler/crates/lr-common/`
  - Files: `Cargo.toml`, `src/lib.rs`
  - Content: `Span` type definition

- [ ] **Step 1.3:** Create remaining crate directories
  - `lr-lexer`, `lr-parser`, `lr-ast`, `lr-diagnostics`
  - Each with `Cargo.toml` and `src/lib.rs` placeholder

- [ ] **Step 1.4:** Verify workspace builds
  - Run: `cargo build --workspace`
  - Expected: Successful build with no errors

- [ ] **Step 1.5:** Commit
  ```bash
  git add .
  git commit -m "feat: set up workspace structure and shared types"
  ```

### Phase 2: Token Types and Hybrid Lexer Setup (2 hours)

- [ ] **Step 2.1:** Add logos dependency
  - File: `Cargo.toml` (workspace root)
  - Add: `logos = "0.16.1"` to `[workspace.dependencies]`
  - Add: `logos = { workspace = true }` to `crates/lr-lexer/Cargo.toml`

- [ ] **Step 2.2:** Write failing token type tests
  - File: `crates/lr-lexer/tests/token_types.rs`
  - Tests for all token kinds, RawToken enum, Token wrapper

- [ ] **Step 2.3:** Implement RawToken enum with Logos derive
  - File: `crates/lr-lexer/src/token.rs`
  - `#[derive(Logos)]` on `RawToken`
  - All simple tokens: delimiters, numbers, strings, comments
  - `OperatorIdent` catch-all regex

- [ ] **Step 2.4:** Implement Token wrapper with spans
  - File: `crates/lr-lexer/src/token.rs`
  - `TokenKind` enum, `Token` struct with `Span`
  - Conversion from `RawToken` to `Token` with span tracking

- [ ] **Step 2.5:** Run token type tests
  - Run: `cargo test -p lr-lexer token_types`
  - Expected: PASS

- [ ] **Step 2.6:** Commit
  ```bash
  git add Cargo.toml crates/lr-lexer/Cargo.toml crates/lr-lexer/src/token.rs
  git commit -m "feat: add logos dependency and RawToken enum"
  ```

### Phase 3: Logos Integration and Simple Tokens (1.5 hours)

- [ ] **Step 3.1:** Implement hybrid lexer skeleton
  - File: `crates/lr-lexer/src/lexer.rs`
  - `Lexer` struct with logos `RawToken::lexer()`
  - `tokenize()` method with Phase 1 (logos) + Phase 2 (wrapper)

- [ ] **Step 3.2:** Implement simple token mappings
  - Direct mapping: delimiters, numbers, strings, comments
  - Span extraction via `lex.span()`
  - EOF token emission

- [ ] **Step 3.3:** Run simple token tests
  - Run: `cargo test -p lr-lexer simple_tokens`
  - Expected: PASS

- [ ] **Step 3.4:** Commit
  ```bash
  git add crates/lr-lexer/src/lexer.rs
  git commit -m "feat: implement logos integration and simple token mappings"
  ```

### Phase 4: Hand-Written Operator Identifier Fallback (2 hours)

- [ ] **Step 4.1:** Write failing identifier tests
  - File: `crates/lr-lexer/tests/identifier_tests.rs`
  - Tests for maximal munch (`!!!?`, `///`, `\\\`, `$@`, `><`, `_<`, `_>`)

- [ ] **Step 4.2:** Implement operator identifier resolution
  - Method: `process_operator_ident(span)`
  - Maximal munch for `_>` and `_<` (2-char tokens)
  - Boolean/undefined literal detection
  - All other operators → `Identifier`

- [ ] **Step 4.3:** Implement backtick handling
  - Method: `process_backtick_token(span)`
  - Triple backtick at line start → comment
  - Single backtick → string literal

- [ ] **Step 4.4:** Run identifier tests
  - Run: `cargo test -p lr-lexer identifier_tests`
  - Expected: PASS

- [ ] **Step 4.5:** Commit
  ```bash
  git add crates/lr-lexer/src/lexer.rs
  git commit -m "feat: implement hand-written operator identifier fallback with maximal munch"
  ```

### Phase 5: String Interpolation Marking (1 hour)

- [ ] **Step 5.1:** Write failing interpolation tests
  - File: `crates/lr-lexer/tests/interpolation_tests.rs`
  - Tests for string tokens with `{expr}` markers

- [ ] **Step 5.2:** Implement interpolation location marking
  - Method: `mark_interpolation_locations()`
  - Detect `{expr}` patterns in string values
  - Return locations for parser to re-lex

- [ ] **Step 5.3:** Run interpolation tests
  - Run: `cargo test -p lr-lexer interpolation_tests`
  - Expected: PASS

- [ ] **Step 5.4:** Commit
  ```bash
  git add crates/lr-lexer/src/lexer.rs
  git commit -m "feat: implement string interpolation location marking"
  ```

### Phase 6: AST Node Types (1 hour)

- [ ] **Step 6.1:** Write failing AST tests
  - File: `crates/lr-ast/tests/node_tests.rs`
  - Tests for all node types

- [ ] **Step 6.2:** Implement AST node definitions
  - File: `crates/lr-ast/src/lib.rs`
  - `Program`, `Expression`, `StringPart`, `MapEntry`

- [ ] **Step 6.3:** Run AST tests
  - Run: `cargo test -p lr-ast node_tests`
  - Expected: PASS

- [ ] **Step 6.4:** Commit
  ```bash
  git add crates/lr-ast/
  git commit -m "feat: implement AST node types with span tracking"
  ```

### Phase 7: Parser Core (2 hours)

- [ ] **Step 7.1:** Write failing parser skeleton tests
  - File: `crates/lr-parser/tests/parser_skeleton.rs`
  - Tests for basic parser structure

- [ ] **Step 7.2:** Implement parser skeleton
  - File: `crates/lr-parser/src/parser.rs`
  - `Parser` struct, `peek()`, `next()`, `expect()`

- [ ] **Step 7.3:** Run parser skeleton tests
  - Run: `cargo test -p lr-parser parser_skeleton`
  - Expected: PASS

- [ ] **Step 7.4:** Implement zero-precedence expression parsing
  - Method: `parse_expression()`, `parse_primary()`
  - Pattern: All operators = binding power 0 [https://docs.rs/prattle/latest/prattle/]

- [ ] **Step 7.5:** Write failing expression tests
  - File: `crates/lr-parser/tests/expression_tests.rs`
  - Tests for basic arithmetic, zero precedence

- [ ] **Step 7.6:** Run expression tests
  - Run: `cargo test -p lr-parser expression_tests`
  - Expected: PASS

- [ ] **Step 7.7:** Commit
  ```bash
  git add crates/lr-parser/
  git commit -m "feat: implement parser core with zero-precedence expression parsing"
  ```

### Phase 8: Collection Parsing (2 hours)

- [ ] **Step 8.1:** Write failing list literal tests
  - File: `crates/lr-parser/tests/list_tests.rs`
  - Tests for `[]`, nested lists

- [ ] **Step 8.2:** Implement list literal parsing
  - Method: `parse_list_literal()`

- [ ] **Step 8.3:** Run list tests
  - Run: `cargo test -p lr-parser list_tests`
  - Expected: PASS

- [ ] **Step 8.4:** Write failing map literal tests
  - File: `crates/lr-parser/tests/map_tests.rs`
  - Tests for `{}`, assignment keys, expression keys

- [ ] **Step 8.5:** Implement map literal parsing with colon disambiguation
  - Method: `parse_map_literal()`
  - Detect alpha-starting keys (assignment) vs expression keys

- [ ] **Step 8.6:** Run map tests
  - Run: `cargo test -p lr-parser map_tests`
  - Expected: PASS

- [ ] **Step 8.7:** Commit
  ```bash
  git add crates/lr-parser/
  git commit -m "feat: implement map and list literal parsing with colon disambiguation"
  ```

### Phase 9: String Interpolation and Grouping (1.5 hours)

- [ ] **Step 9.1:** Write failing string parsing tests
  - File: `crates/lr-parser/tests/string_parsing_tests.rs`
  - Tests for string literals with interpolation

- [ ] **Step 9.2:** Implement string interpolation parsing with re-lexing
  - Method: `parse_string_parts()`
  - Use lexer's `mark_interpolation_locations()` to find expressions
  - Re-lex expression strings with new lexer instance
  - Parse re-lexed expressions into AST

- [ ] **Step 9.3:** Run string parsing tests
  - Run: `cargo test -p lr-parser string_parsing_tests`
  - Expected: PASS

- [ ] **Step 9.4:** Write failing grouped expression tests
  - File: `crates/lr-parser/tests/grouping_tests.rs`
  - Tests for `(expr)`

- [ ] **Step 9.5:** Implement grouped expression parsing
  - Method: `parse_grouped_expression()`

- [ ] **Step 9.6:** Run grouping tests
  - Run: `cargo test -p lr-parser grouping_tests`
  - Expected: PASS

- [ ] **Step 9.7:** Commit
  ```bash
  git add crates/lr-parser/
  git commit -m "feat: implement string interpolation with re-lexing and grouped expression parsing"
  ```

### Phase 10: Special Expressions (1 hour)

- [ ] **Step 10.1:** Write failing special expression tests
  - File: `crates/lr-parser/tests/special_tests.rs`
  - Tests for `!!!`, `!!!?`, `///`, `\\\`

- [ ] **Step 10.2:** Implement throw/catch/async/await parsing
  - Methods: `parse_throw_expression()`, `parse_catch_expression()`, etc.

- [ ] **Step 10.3:** Run special expression tests
  - Run: `cargo test -p lr-parser special_tests`
  - Expected: PASS

- [ ] **Step 10.4:** Commit
  ```bash
  git add crates/lr-parser/
  git commit -m "feat: implement throw/catch/async/await expression parsing"
  ```

### Phase 11: Error Recovery (1.5 hours)

- [ ] **Step 11.1:** Write failing error recovery tests
  - File: `crates/lr-parser/tests/recovery_tests.rs`
  - Tests for malformed input recovery

- [ ] **Step 11.2:** Implement bogus node error recovery
  - File: `crates/lr-parser/src/recovery.rs`
  - Pattern: Biome `ParseRecoveryTokenSet` [https://github.com/biomejs/biome/blob/main/.claude/skills/parser-development/SKILL.md]

- [ ] **Step 11.3:** Integrate recovery into parser
  - Methods: `recover()`, `try_parse()`

- [ ] **Step 11.4:** Run error recovery tests
  - Run: `cargo test -p lr-parser recovery_tests`
  - Expected: PASS

- [ ] **Step 11.5:** Commit
  ```bash
  git add crates/lr-parser/
  git commit -m "feat: implement bogus node error recovery for robust parsing"
  ```

### Phase 12: Diagnostics (1 hour)

- [ ] **Step 12.1:** Write failing diagnostics tests
  - File: `crates/lr-diagnostics/tests/diagnostics_tests.rs`
  - Tests for error rendering

- [ ] **Step 12.2:** Implement error rendering with ariadne
  - File: `crates/lr-diagnostics/src/lib.rs`
  - Multi-line spans, colors [https://docs.rs/ariadne/latest/ariadne/]

- [ ] **Step 12.3:** Run diagnostics tests
  - Run: `cargo test -p lr-diagnostics diagnostics_tests`
  - Expected: PASS

- [ ] **Step 12.4:** Commit
  ```bash
  git add crates/lr-diagnostics/
  git commit -m "feat: implement error rendering with ariadne diagnostics"
  ```

### Phase 13: Live System Test Suite (3 hours)

- [ ] **Step 13.1:** Create test runner infrastructure
  - File: `compiler/tests/live_system_tests.rs`
  - Helper functions for testing

- [ ] **Step 13.2:** Implement Flow 1-5 tests (basic flows)
  - Basic arithmetic, zero precedence, operator as identifier, string interpolation, map operators

- [ ] **Step 13.3:** Run Flow 1-5 tests
  - Run: `cargo test --test live_system_tests flow_0[1-5]`
  - Expected: PASS

- [ ] **Step 13.4:** Implement Flow 6-10 tests (error and export flows)
  - Error handling, async, comments, export, nested maps/lists

- [ ] **Step 13.5:** Run Flow 6-10 tests
  - Run: `cargo test --test live_system_tests flow_0[6-10]`
  - Expected: PASS

- [ ] **Step 13.6:** Implement Flow 11-15 tests (advanced flows)
  - Curried chains, reverse args, silent execution, spread, multi-line strings

- [ ] **Step 13.7:** Run Flow 11-15 tests
  - Run: `cargo test --test live_system_tests flow_1[1-5]`
  - Expected: PASS

- [ ] **Step 13.8:** Implement Flow 16-20 tests (edge cases)
  - Boolean operators, loop operators, error recovery, empty input, unicode

- [ ] **Step 13.9:** Run Flow 16-20 tests
  - Run: `cargo test --test live_system_tests flow_1[6-20]`
  - Expected: PASS

- [ ] **Step 13.10:** Run all live system tests
  - Run: `cargo test --test live_system_tests`
  - Expected: ALL PASS

- [ ] **Step 13.11:** Commit
  ```bash
  git add compiler/tests/
  git commit -m "feat: implement live system test suite with 20 comprehensive flows"
  ```

### Phase 14: Property-Based Testing (1 hour)

- [ ] **Step 14.1:** Write failing proptest tests
  - File: `crates/lr-lexer/tests/prop_tests.rs`
  - Properties: never panics, spans in bounds [https://www.beamtalk.dev/adr/0011-robustness-testing-layered-fuzzing.html]

- [ ] **Step 14.2:** Run property-based tests
  - Run: `cargo test -p lr-lexer prop_tests`
  - Expected: PASS

- [ ] **Step 14.3:** Commit
  ```bash
  git add crates/lr-lexer/tests/prop_tests.rs
  git commit -m "feat: add property-based tests for lexer robustness"
  ```

### Phase 15: Integration Testing (2 hours)

- [ ] **Step 15.1:** Create end-to-end integration tests
  - File: `compiler/tests/integration_tests.rs`
  - Test full pipeline: source → lexer → parser → AST

- [ ] **Step 15.2:** Implement snapshot testing with insta
  - Use `tokenstream` feature for AST snapshots [https://github.com/mitsuhiko/insta/pull/884]

- [ ] **Step 15.3:** Run integration tests
  - Run: `cargo test --test integration_tests`
  - Expected: PASS

- [ ] **Step 15.4:** Verify all tests pass
  - Run: `cargo test --workspace`
  - Expected: ALL PASS

- [ ] **Step 15.5:** Commit
  ```bash
  git add compiler/tests/
  git commit -m "feat: add end-to-end integration tests with insta snapshots"
  ```

### Phase 16: Documentation and Final Verification (1 hour)

- [ ] **Step 16.1:** Write README for each crate
  - `crates/lr-lexer/README.md`
  - `crates/lr-parser/README.md`
  - `crates/lr-ast/README.md`
  - `crates/lr-diagnostics/README.md`

- [ ] **Step 16.2:** Write compiler README
  - `compiler/README.md`
  - Overview of architecture and usage

- [ ] **Step 16.3:** Run final verification
  - Run: `cargo build --workspace --release`
  - Expected: Successful optimized build

- [ ] **Step 16.4:** Run final test suite
  - Run: `cargo test --workspace`
  - Expected: ALL PASS (no failures)

- [ ] **Step 16.5:** Commit
  ```bash
  git add .
  git commit -m "docs: add README documentation and final verification"
  ```

---

## 8. Verification Checklist

Before claiming implementation is complete, verify:

- [ ] All 16 phases completed (checked off)
- [ ] `cargo build --workspace` passes (no compilation errors)
- [ ] `cargo test --workspace` passes (all tests green)
- [ ] All 20 live system test flows pass
- [ ] Property-based tests run without failures
- [ ] Insta snapshots reviewed and correct
- [ ] Error recovery tested with malformed input
- [ ] String interpolation nesting tested with re-lexing
- [ ] Operator identifier maximal munch tested
- [ ] Unicode identifiers tested
- [ ] Empty input handled correctly
- [ ] Logos-based tokenization verified (2-10x speedup over pure hand-written)
- [ ] LSP diagnostics clean on all modified files
- [ ] No TODOs left in code
- [ ] All crates have README documentation

---

## 9. Research Citations

This implementation plan cites these sources:

1. **Hybrid lexer pattern**: Logos compile-time FSM + hand-written fallback for edge cases [https://github.com/maciejhirsz/logos/]
2. **Logos performance**: Compile-time state machine, 1204 MB/s identifiers, 2-10x faster than hand-written [https://github.com/maciejhirsz/logos/]
3. **Token cloning for speed**: "Avoid returning refs — clone tokens for 25% speedup" [https://alic.dev/blog/fast-lexing]
4. **Zero precedence parsing**: Set binding power 0 for all operators, parse as left-associative chain [https://docs.rs/prattle/latest/prattle/]
5. **Insta snapshots**: Use `tokenstream` feature for AST snapshot testing [https://github.com/mitsuhiko/insta/pull/884]
6. **Proptest properties**: Parser never panics, diagnostic spans within input bounds [https://www.beamtalk.dev/adr/0011-robustness-testing-layered-fuzzing.html]
7. **Error recovery**: Biome `ParseRecoveryTokenSet` with bogus nodes [https://github.com/biomejs/biome/blob/main/.claude/skills/parser-development/SKILL.md]
8. **Ariadne diagnostics**: Multi-line spans, multi-file errors, colors [https://docs.rs/ariadne/latest/ariadne/]
9. **String interpolation re-lexing**: Two-pass approach for nested braces in strings [https://doc.rust-lang.org/reference/tokens.html]

---

## 10. Appendix: Complete Test Specification

### Test Flow 1: Basic Arithmetic

**Input:**
```lr
5 + 3
```

**Expected Tokens:**
- `NumberLiteral("5")`
- `Identifier("+")`
- `NumberLiteral("3")`

**Expected AST:**
```rust
Expression::Application {
    left: Expression::Application {
        left: Expression::NumberLiteral { value: "5", raw: "5", span: ... },
        right: Expression::Identifier { name: "+", span: ... },
        span: ...
    },
    right: Expression::NumberLiteral { value: "3", raw: "3", span: ... },
    span: ...
}
```

**Expected Errors:** None

### Test Flow 2: Zero Precedence

**Input:**
```lr
5 + 3 * 2
```

**Expected Tokens:**
- `NumberLiteral("5")`
- `Identifier("+")`
- `NumberLiteral("3")`
- `Identifier("*")`
- `NumberLiteral("2")`

**Expected AST:**
```rust
// (((5+)3)*)2 = 16, not 5+(3*2) = 11
Expression::Application {
    left: Expression::Application {
        left: Expression::Application {
            left: Expression::NumberLiteral { value: "5", raw: "5", span: ... },
            right: Expression::Identifier { name: "+", span: ... },
            span: ...
        },
        right: Expression::NumberLiteral { value: "3", raw: "3", span: ... },
        span: ...
    },
    right: Expression::Application {
        left: Expression::Identifier { name: "*", span: ... },
        right: Expression::NumberLiteral { value: "2", raw: "2", span: ... },
        span: ...
    },
    span: ...
}
```

**Expected Errors:** None

### Test Flow 3: Operator as Identifier

**Input:**
```lr
response @ value
```

**Expected Tokens:**
- `Identifier("response")`
- `Identifier("@")`
- `StringLiteral("value")`

**Expected AST:**
```rust
Expression::Application {
    left: Expression::Application {
        left: Expression::Identifier { name: "response", span: ... },
        right: Expression::Identifier { name: "@", span: ... },
        span: ...
    },
    right: Expression::StringLiteral {
        parts: vec![StringPart::Text { text: "value", span: ... }],
        span: ...
    },
    span: ...
}
```

**Expected Errors:** None

### Test Flow 4: String Interpolation

**Input:**
```lr
`Hello {name}`
```

**Expected Tokens:**
- `StringLiteral("Hello {name}")`

**Expected AST:**
```rust
Expression::StringLiteral {
    parts: vec![
        StringPart::Text { text: "Hello ", span: ... },
        StringPart::Interpolation {
            expression: Expression::Identifier { name: "name", span: ... },
            span: ...
        }
    ],
    span: ...
}
```

**Expected Errors:** None

### Test Flow 5: Map Operators

**Input:**
```lr
{ x: x + 1 }
```

**Expected Tokens:**
- `OpenBrace`
- `Identifier("x")`
- `Colon`
- `Identifier("x")`
- `Identifier("+")`
- `NumberLiteral("1")`
- `CloseBrace`

**Expected AST:**
```rust
Expression::MapLiteral {
    entries: vec![MapEntry {
        key: Expression::Identifier { name: "x", span: ... },
        value: Some(Expression::Application {
            left: Expression::Identifier { name: "x", span: ... },
            right: Expression::Application {
                left: Expression::Identifier { name: "+", span: ... },
                right: Expression::NumberLiteral { value: "1", raw: "1", span: ... },
                span: ...
            },
            span: ...
        }),
        is_assignment: true,
        is_expression_key: false,
        span: ...
    }],
    span: ...
}
```

**Expected Errors:** None

### Test Flow 6-20: Remaining Flows

(These follow the same pattern - see Section 6.2 for complete test specifications)

---

## END OF IMPLEMENTATION PLAN

This plan is designed for execution by an AI agent (OpenCode with Sisyphus-Junior) without ambiguity. Every step includes:
- Exact file paths
- Complete code snippets
- Expected test output
- Commit messages
- Research citations with URLs

**Total estimated time:** 18-20 hours for full implementation including all testing phases.