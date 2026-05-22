use lr_common::Span;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    NumberLiteral,
    StringLiteral,
    BooleanLiteral,
    UndefinedLiteral,
    Identifier,
    LeftArg,
    RightArg,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    OpenParen,
    CloseParen,
    Colon,
    Comma,
    Dot,
    SingleQuote,
    Backtick,
    Comment,
    EOF,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::NumberLiteral => write!(f, "NumberLiteral"),
            TokenKind::StringLiteral => write!(f, "StringLiteral"),
            TokenKind::BooleanLiteral => write!(f, "BooleanLiteral"),
            TokenKind::UndefinedLiteral => write!(f, "UndefinedLiteral"),
            TokenKind::Identifier => write!(f, "Identifier"),
            TokenKind::LeftArg => write!(f, "LeftArg"),
            TokenKind::RightArg => write!(f, "RightArg"),
            TokenKind::OpenBrace => write!(f, "OpenBrace"),
            TokenKind::CloseBrace => write!(f, "CloseBrace"),
            TokenKind::OpenBracket => write!(f, "OpenBracket"),
            TokenKind::CloseBracket => write!(f, "CloseBracket"),
            TokenKind::OpenParen => write!(f, "OpenParen"),
            TokenKind::CloseParen => write!(f, "CloseParen"),
            TokenKind::Colon => write!(f, "Colon"),
            TokenKind::Comma => write!(f, "Comma"),
            TokenKind::Dot => write!(f, "Dot"),
            TokenKind::SingleQuote => write!(f, "SingleQuote"),
            TokenKind::Backtick => write!(f, "Backtick"),
            TokenKind::Comment => write!(f, "Comment"),
            TokenKind::EOF => write!(f, "EOF"),
        }
    }
}

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

    pub fn eof(position: u32) -> Self {
        Self {
            kind: TokenKind::EOF,
            value: String::new(),
            span: Span::new(position, position),
        }
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum LexError {
    #[error("unexpected character '{0}'")]
    UnexpectedCharacter(char, Span),

    #[error("unclosed string literal")]
    UnclosedString(Span),

    #[error("invalid number literal: {0}")]
    InvalidNumber(String, Span),

    #[error("{0}")]
    General(String, Span),  // (message, span)
}

impl LexError {
    pub fn span(&self) -> Span {
        match self {
            LexError::UnexpectedCharacter(_, s) => *s,
            LexError::UnclosedString(s) => *s,
            LexError::InvalidNumber(_, s) => *s,
            LexError::General(_, s) => *s,
        }
    }
}