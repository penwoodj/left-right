use crate::token::{Token, TokenKind, LexError};
use lr_common::Span;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LexerState {
    Normal,
    InStringLiteral,
    InStringInterpolation,
    InComment,
}

pub struct Lexer<'a> {
    source: &'a str,
    chars: Peekable<Chars<'a>>,
    position: u32,
    state: LexerState,
    string_start: u32,
    string_parts: Vec<StringPart>,
    interpolation_depth: usize,
    interpolation_start: u32,
    comment_start: u32,
    current_text_part: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StringPart {
    Text(String),
    Interpolation { tokens: Vec<Token> },
}

impl Lexer<'_> {
    pub fn new(source: &str) -> Lexer<'_> {
        Lexer {
            source,
            chars: source.chars().peekable(),
            position: 0,
            state: LexerState::Normal,
            string_start: 0,
            string_parts: Vec::new(),
            interpolation_depth: 0,
            interpolation_start: 0,
            comment_start: 0,
            current_text_part: String::new(),
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn next(&mut self) -> Option<char> {
        let c = self.chars.next();
        if c.is_some() {
            self.position += 1;
        }
        c
    }

    fn is_whitespace(c: char) -> bool {
        matches!(c, ' ' | '\t' | '\n' | '\r')
    }

    fn is_digit(c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_reserved_symbol(c: char) -> bool {
        matches!(c, ':' | ',' | '.' | '\'' | '(' | ')' | '[' | ']' | '{' | '}' | '`' | '@')
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>, Vec<LexError>> {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();

        loop {
            match self.state {
                LexerState::Normal => {
                    match self.lex_normal(&mut errors) {
                        Ok(Some(token)) => {
                            if token.kind != TokenKind::Comment {
                                tokens.push(token);
                            }
                        }
                        Ok(None) => {}
                        Err(e) => errors.push(e),
                    }
                }
                LexerState::InStringLiteral => {
                    match self.lex_string_literal(&mut errors) {
                        Ok(Some(token)) => tokens.push(token),
                        Ok(None) => {}
                        Err(e) => errors.push(e),
                    }
                }
                LexerState::InStringInterpolation => {
                    match self.lex_string_interpolation(&mut errors) {
                        Ok(Some(token)) => tokens.push(token),
                        Ok(None) => {}
                        Err(e) => errors.push(e),
                    }
                }
                LexerState::InComment => {
                    match self.lex_comment() {
                        Ok(Some(token)) => {
                            if token.kind != TokenKind::Comment {
                                tokens.push(token);
                            }
                        }
                        Ok(None) => {}
                        Err(e) => errors.push(e),
                    }
                }
            }

            if self.state == LexerState::Normal && self.peek().is_none() {
                break;
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(tokens)
    }

    fn lex_normal(&mut self, _errors: &mut Vec<LexError>) -> Result<Option<Token>, LexError> {
        while let Some(&c) = self.peek() {
            if Self::is_whitespace(c) {
                self.next();
                continue;
            }

            let start = self.position;

            match c {
                '`' => {
                    self.next();

                    if let Some(&'`') = self.peek() {
                        self.next();
                        if let Some(&'`') = self.peek() {
                            self.next();

                            if start == 0 || self.source[..start as usize].ends_with('\n') {
                                self.state = LexerState::InComment;
                                self.comment_start = start;
                                return Ok(None);
                            } else {
                                return Ok(Some(Token::new(
                                    TokenKind::Backtick,
                                    "`".repeat(2),
                                    Span::new(start, self.position),
                                )));
                            }
                        }
                    }

                    self.state = LexerState::InStringLiteral;
                    self.string_start = start;
                    self.string_parts.clear();
                    self.current_text_part.clear();
                    return Ok(None);
                }

                ':' => {
                    self.next();
                    return Ok(Some(Token::new(TokenKind::Colon, ":".to_string(), Span::new(start, self.position))));
                }

                ',' => {
                    self.next();
                    return Ok(Some(Token::new(TokenKind::Comma, ",".to_string(), Span::new(start, self.position))));
                }

                '\'' => {
                    self.next();
                    return Ok(Some(Token::new(TokenKind::SingleQuote, "'".to_string(), Span::new(start, self.position))));
                }

                '(' => {
                    self.next();
                    return Ok(Some(Token::new(TokenKind::OpenParen, "(".to_string(), Span::new(start, self.position))));
                }

                ')' => {
                    self.next();
                    return Ok(Some(Token::new(TokenKind::CloseParen, ")".to_string(), Span::new(start, self.position))));
                }

                '[' => {
                    self.next();
                    return Ok(Some(Token::new(TokenKind::OpenBracket, "[".to_string(), Span::new(start, self.position))));
                }

                ']' => {
                    self.next();
                    return Ok(Some(Token::new(TokenKind::CloseBracket, "]".to_string(), Span::new(start, self.position))));
                }

                '{' => {
                    self.next();
                    return Ok(Some(Token::new(TokenKind::OpenBrace, "{".to_string(), Span::new(start, self.position))));
                }

                '}' => {
                    self.next();
                    return Ok(Some(Token::new(TokenKind::CloseBrace, "}".to_string(), Span::new(start, self.position))));
                }

                _ if Self::is_digit(c) => {
                    let first = c;
                    self.next();
                    return self.lex_number(start, first);
                }

                _ if Self::is_digit(c) => {
                    let first = c;
                    self.next();
                    return self.lex_number(start, first);
                }

                '_' => {
                    let first = c;
                    self.next();
                    if let Some(&c) = self.peek() {
                        match c {
                            '<' => {
                                self.next();
                                return Ok(Some(Token::new(TokenKind::LeftArg, "_<".to_string(), Span::new(start, self.position))));
                            }
                            '>' => {
                                self.next();
                                return Ok(Some(Token::new(TokenKind::RightArg, "_>".to_string(), Span::new(start, self.position))));
                            }
                            _ => {}
                        }
                    }
                    return self.lex_identifier(start, first);
                }

                _ if Self::is_reserved_symbol(c) => {
                    self.next();
                    let kind = match c {
                        ':' => TokenKind::Colon,
                        ',' => TokenKind::Comma,
                        '.' => TokenKind::Dot,
                        '\'' => TokenKind::SingleQuote,
                        '(' => TokenKind::OpenParen,
                        ')' => TokenKind::CloseParen,
                        '[' => TokenKind::OpenBracket,
                        ']' => TokenKind::CloseBracket,
                        '{' => TokenKind::OpenBrace,
                        '}' => TokenKind::CloseBrace,
                        '`' => TokenKind::Backtick,
                        '@' => TokenKind::Identifier,
                        _ => unreachable!(),
                    };
                    return Ok(Some(Token::new(kind, c.to_string(), Span::new(start, self.position))));
                }

                _ => {
                    let first = c;
                    self.next();
                    return self.lex_identifier(start, first);
                }
            }
        }

        Ok(None)
    }

    fn lex_identifier(&mut self, start: u32, first: char) -> Result<Option<Token>, LexError> {
        let mut value = String::new();
        value.push(first);

        while let Some(&c) = self.peek() {
            if Self::is_whitespace(c) {
                break;
            }
            if Self::is_reserved_symbol(c) {
                break;
            }
            if Self::is_digit(c) {
                break;
            }
            if c == '_' && let Some(&next) = self.chars.peek()
                && matches!(next, '<' | '>') {
                    break;
            }
            self.next();
            value.push(c);
        }

        let kind = match value.as_str() {
            "true" | "false" => TokenKind::BooleanLiteral,
            "undefined" => TokenKind::UndefinedLiteral,
            _ => TokenKind::Identifier,
        };

        Ok(Some(Token::new(kind, value, Span::new(start, self.position))))
    }

    fn lex_number(&mut self, start: u32, first: char) -> Result<Option<Token>, LexError> {
        let mut value = String::new();
        value.push(first);
        let mut has_dot = first == '.';

        while let Some(&c) = self.peek() {
            if Self::is_digit(c) {
                self.next();
                value.push(c);
            } else if c == '.' && !has_dot {
                let remaining = &self.source[self.position as usize..];
                if let Some(next_char) = remaining.chars().nth(1) {
                    if Self::is_digit(next_char) {
                        self.next();
                        value.push('.');
                        has_dot = true;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Ok(Some(Token::new(TokenKind::NumberLiteral, value, Span::new(start, self.position))))
    }

    fn lex_string_literal(&mut self, _errors: &mut Vec<LexError>) -> Result<Option<Token>, LexError> {
        while let Some(&c) = self.peek() {
            match c {
                '`' => {
                    self.next();

                    if !self.current_text_part.is_empty() {
                        self.string_parts.push(StringPart::Text(std::mem::take(&mut self.current_text_part)));
                    }

                    let mut value = String::new();
                    for part in &self.string_parts {
                        match part {
                            StringPart::Text(text) => value.push_str(text),
                            StringPart::Interpolation { tokens } => {
                                value.push('{');
                                for token in tokens {
                                    value.push_str(&token.value);
                                }
                                value.push('}');
                            }
                        }
                    }

                    let token = Token::new(TokenKind::StringLiteral, value, Span::new(self.string_start, self.position));
                    self.state = LexerState::Normal;
                    return Ok(Some(token));
                }

                '\\' => {
                    self.next();
                    if let Some(&'`') = self.peek() {
                        self.next();
                        self.current_text_part.push('`');
                    } else {
                        self.current_text_part.push('\\');
                    }
                }

                '{' => {
                    if !self.current_text_part.is_empty() {
                        self.string_parts.push(StringPart::Text(std::mem::take(&mut self.current_text_part)));
                    }

                    self.next();
                    self.state = LexerState::InStringInterpolation;
                    self.interpolation_depth = 1;
                    self.interpolation_start = self.position;
                    return Ok(None);
                }

                _ => {
                    self.next();
                    self.current_text_part.push(c);
                }
            }
        }

        _errors.push(LexError::UnclosedString(format!("position {}", self.string_start)));
        self.state = LexerState::Normal;
        Ok(None)
    }

    fn lex_string_interpolation(&mut self, _errors: &mut Vec<LexError>) -> Result<Option<Token>, LexError> {
        let mut tokens = Vec::new();

        while self.interpolation_depth > 0 {
            match self.state {
                LexerState::InStringInterpolation => {
                    if let Some(&c) = self.peek() {
                        match c {
                            '{' => {
                                self.next();
                                self.interpolation_depth += 1;
                            }
                            '}' => {
                                self.next();
                                self.interpolation_depth -= 1;

                                if self.interpolation_depth == 0 {
                                    self.state = LexerState::InStringLiteral;
                                    self.string_parts.push(StringPart::Interpolation { tokens });
                                    return self.lex_string_literal(_errors);
                                }
                            }
                            '`' => {
                                return Ok(Some(Token::new(
                                    TokenKind::Backtick,
                                    "`".to_string(),
                                    Span::new(self.position, self.position + 1),
                                )));
                            }
                            _ if Self::is_whitespace(c) => {
                                self.next();
                            }
                            _ => {
                                if let Some(token) = self.lex_normal(_errors)? {
                                    tokens.push(token);
                                }
                            }
                        }
                    } else {
                        _errors.push(LexError::UnclosedString(format!("position {}", self.string_start)));
                        self.state = LexerState::Normal;
                        return Ok(None);
                    }
                }
                _ => return Ok(None),
            }
        }

        Ok(None)
    }

    fn lex_comment(&mut self) -> Result<Option<Token>, LexError> {
        let mut text = String::new();

        while let Some(&c) = self.peek() {
            if c == '\n' {
                self.next();
                let token = Token::new(TokenKind::Comment, text, Span::new(self.comment_start, self.position));
                self.state = LexerState::Normal;
                return Ok(Some(token));
            }
            self.next();
            text.push(c);
        }

        let token = Token::new(TokenKind::Comment, text, Span::new(self.comment_start, self.position));
        self.state = LexerState::Normal;
        Ok(Some(token))
    }
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, Vec<LexError>> {
    let lexer = Lexer::new(source);
    match lexer.tokenize() {
        Ok(mut tokens) => {
            tokens.push(Token::eof(source.len() as u32));
            Ok(tokens)
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_number() {
        let source = "42";
        let tokens = tokenize(source).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::NumberLiteral);
        assert_eq!(tokens[0].value, "42");
        assert_eq!(tokens[1].kind, TokenKind::EOF);
    }

    #[test]
    fn test_tokenize_float() {
        let source = "3.14";
        let tokens = tokenize(source).unwrap();
        println!("Tokens: {:?}", tokens.iter().map(|t| (t.kind.clone(), t.value.clone())).collect::<Vec<_>>());
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::NumberLiteral);
        assert_eq!(tokens[0].value, "3.14");
    }

    #[test]
    fn test_tokenize_response_at_value() {
        let source = "`response@value`";
        let tokens = tokenize(source).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::StringLiteral);
        assert_eq!(tokens[0].value, "response@value");
    }

    #[test]
    fn test_tokenize_identifier() {
        let source = "hello";
        let tokens = tokenize(source).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::Identifier);
        assert_eq!(tokens[0].value, "hello");
    }

    #[test]
    fn test_tokenize_operator() {
        let source = "+";
        let tokens = tokenize(source).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::Identifier);
        assert_eq!(tokens[0].value, "+");
    }

    #[test]
    fn test_tokenize_left_arg() {
        let source = "_<";
        let tokens = tokenize(source).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::LeftArg);
        assert_eq!(tokens[0].value, "_<");
    }

    #[test]
    fn test_tokenize_right_arg() {
        let source = "_>";
        let tokens = tokenize(source).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::RightArg);
        assert_eq!(tokens[0].value, "_>");
    }

    #[test]
    fn test_tokenize_boolean_true() {
        let source = "true";
        let tokens = tokenize(source).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::BooleanLiteral);
        assert_eq!(tokens[0].value, "true");
    }

    #[test]
    fn test_tokenize_boolean_false() {
        let source = "false";
        let tokens = tokenize(source).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::BooleanLiteral);
        assert_eq!(tokens[0].value, "false");
    }

    #[test]
    fn test_tokenize_undefined() {
        let source = "undefined";
        let tokens = tokenize(source).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::UndefinedLiteral);
        assert_eq!(tokens[0].value, "undefined");
    }

    #[test]
    fn test_tokenize_string_literal() {
        let source = "`hello`";
        let tokens = tokenize(source).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::StringLiteral);
        assert_eq!(tokens[0].value, "hello");
    }

    #[test]
    fn test_tokenize_string_with_interpolation() {
        let source = "`hello {name}`";
        let tokens = tokenize(source).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::StringLiteral);
    }

    #[test]
    fn test_tokenize_comment() {
        let source = "```comment\n";
        let tokens = tokenize(source).unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::EOF);
    }

    #[test]
    fn test_tokenize_delimiters() {
        let source = "()[]{}:,. '";
        let tokens = tokenize(source).unwrap();
        println!("Tokens generated: {:?}", tokens.iter().map(|t| (t.kind.clone(), t.value.clone())).collect::<Vec<_>>());
        assert_eq!(tokens[0].kind, TokenKind::OpenParen);
        assert_eq!(tokens[1].kind, TokenKind::CloseParen);
        assert_eq!(tokens[2].kind, TokenKind::OpenBracket);
        assert_eq!(tokens[3].kind, TokenKind::CloseBracket);
        assert_eq!(tokens[4].kind, TokenKind::OpenBrace);
        assert_eq!(tokens[5].kind, TokenKind::CloseBrace);
        assert_eq!(tokens[6].kind, TokenKind::Colon);
        assert_eq!(tokens[7].kind, TokenKind::Comma);
        assert_eq!(tokens[8].kind, TokenKind::Dot);
        assert_eq!(tokens[9].kind, TokenKind::SingleQuote);
    }

    #[test]
    fn test_tokenize_5_plus_3() {
        let source = "5+3";
        let tokens = tokenize(source).unwrap();
        println!("Tokens: {:?}", tokens.iter().map(|t| (t.kind.clone(), t.value.clone())).collect::<Vec<_>>());
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].kind, TokenKind::NumberLiteral);
        assert_eq!(tokens[0].value, "5");
        assert_eq!(tokens[1].kind, TokenKind::Identifier);
        assert_eq!(tokens[1].value, "+");
        assert_eq!(tokens[2].kind, TokenKind::NumberLiteral);
        assert_eq!(tokens[2].value, "3");
    }

    #[test]
    fn test_tokenize_triple_bang_question() {
        let source = "!!!?";
        let tokens = tokenize(source).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::Identifier);
        assert_eq!(tokens[0].value, "!!!?");
    }

    #[test]
    fn test_tokenize_escaped_backtick() {
        let source = "`hello\\`world`";
        let tokens = tokenize(source).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].kind, TokenKind::StringLiteral);
        assert_eq!(tokens[0].value, "hello`world");
    }
}