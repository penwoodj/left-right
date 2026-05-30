use lr_ast::*;
use lr_common::Span;
use lr_lexer::{Token, TokenKind};

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("unexpected token {0:?}")]
    UnexpectedToken(TokenKind, Span),

    #[error("unexpected end of input")]
    UnexpectedEOF(Span),

    #[error("expected {0}, found {1:?}")]
    ExpectedToken(&'static str, TokenKind, Span),
}

impl ParseError {
    pub fn span(&self) -> Span {
        match self {
            ParseError::UnexpectedToken(_, s) => *s,
            ParseError::UnexpectedEOF(s) => *s,
            ParseError::ExpectedToken(_, _, s) => *s,
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn peek_kind(&self) -> Option<TokenKind> {
        self.peek().map(|t| t.kind.clone())
    }

    fn next(&mut self) -> Option<&Token> {
        let current = self.current;
        self.current += 1;
        self.tokens.get(current)
    }

    fn consume(&mut self, expected: TokenKind) -> Result<Token, ParseError> {
        if let Some(token) = self.next() {
            if token.kind == expected {
                return Ok(token.clone());
            }
            return Err(ParseError::ExpectedToken("something else", token.kind.clone(), token.span));
        }
        Err(ParseError::UnexpectedEOF(Span::at(self.current as u32)))
    }

    fn contains_arg_tokens(&self, start_pos: usize) -> bool {
        let mut depth = 1;
        let mut pos = start_pos;
        while depth > 0 && pos < self.tokens.len() {
            match self.tokens[pos].kind {
                TokenKind::OpenBrace => depth += 1,
                TokenKind::CloseBrace => depth -= 1,
                TokenKind::LeftArg | TokenKind::RightArg => return true,
                _ => {}
            }
            pos += 1;
        }
        false
    }

    fn parse_primary(&mut self) -> Result<Expression, ParseError> {
        let position = self.current;
        let token = self.next().ok_or_else(|| ParseError::UnexpectedEOF(Span::at(position as u32)))?;

        match token.kind {
            TokenKind::NumberLiteral => {
                let value = token.value.parse::<f64>()
                    .map_err(|_| ParseError::UnexpectedToken(token.kind.clone(), token.span))?;
                Ok(Expression::NumberLiteral(NumberLiteral {
                    value,
                    raw: token.value.clone(),
                    span: token.span,
                }))
            }

            TokenKind::StringLiteral => {
                Ok(Expression::StringLiteral(StringLiteral {
                    parts: vec![StringPart::Text(token.value.clone())],
                    span: token.span,
                }))
            }

            TokenKind::BooleanLiteral => {
                let value = token.value == "true";
                Ok(Expression::BooleanLiteral(BooleanLiteral {
                    value,
                    raw: token.value.clone(),
                    span: token.span,
                }))
            }

            TokenKind::UndefinedLiteral => {
                Ok(Expression::UndefinedLiteral(UndefinedLiteral {
                    raw: token.value.clone(),
                    span: token.span,
                }))
            }

            TokenKind::Identifier => {
                Ok(Expression::Identifier(Identifier {
                    name: token.value.clone(),
                    span: token.span,
                }))
            }

            TokenKind::LeftArg => {
                Ok(Expression::LeftArg(LeftArg {
                    raw: token.value.clone(),
                    span: token.span,
                }))
            }

            TokenKind::RightArg => {
                Ok(Expression::RightArg(RightArg {
                    raw: token.value.clone(),
                    span: token.span,
                }))
            }

            TokenKind::OpenBracket => {
                let mut elements = Vec::new();
                let start = token.span.start;

                loop {
                    if let Some(TokenKind::CloseBracket) = self.peek_kind() {
                        self.next();
                        break;
                    }

                    if !elements.is_empty() {
                        self.consume(TokenKind::Comma)?;
                    }

                    elements.push(self.parse_expression()?);
                }

                let end = self.tokens.get(self.current - 1).map(|t| t.span.end).unwrap_or(start);
                Ok(Expression::ListLiteral(ListLiteral {
                    elements,
                    span: Span::new(start, end),
                }))
            }

            TokenKind::OpenBrace => {
                let mut entries = Vec::new();
                let start = token.span.start;

                loop {
                    if let Some(TokenKind::CloseBrace) = self.peek_kind() {
                        self.next();
                        break;
                    }

                    if !entries.is_empty() {
                        self.consume(TokenKind::Comma)?;
                    }

                    let key = self.parse_expression()?;

                    let mut is_assignment = false;
                    let mut is_expression_key = false;

                    if let Expression::Identifier(ref ident) = key {
                        is_assignment = ident.name.chars().next().map(|c| c.is_alphabetic()).unwrap_or(false);
                    } else {
                        is_expression_key = true;
                    }

                    let value = if let Some(TokenKind::Colon) = self.peek_kind() {
                        let position = self.current;
                        self.next().ok_or_else(|| ParseError::UnexpectedEOF(Span::at(position as u32)))?;
                        Some(self.parse_expression()?)
                    } else {
                        None
                    };

                    entries.push(MapEntry {
                        key,
                        value,
                        is_assignment,
                        is_expression_key,
                    });
                }

                let end = self.tokens.get(self.current - 1).map(|t| t.span.end).unwrap_or(start);
                Ok(Expression::MapLiteral(MapLiteral {
                    entries,
                    span: Span::new(start, end),
                }))
            }

            TokenKind::OpenParen => {
                let start = token.span.start;
                let expression = self.parse_expression()?;
                self.consume(TokenKind::CloseParen)?;
                let end = expression.span().end;
                Ok(Expression::GroupedExpression(GroupedExpression {
                    expression: Box::new(expression),
                    span: Span::new(start, end),
                }))
            }

            TokenKind::Dot => {
                Ok(Expression::Identifier(Identifier {
                    name: ".".to_string(),
                    span: token.span,
                }))
            }

            TokenKind::SingleQuote => {
                Ok(Expression::Identifier(Identifier {
                    name: "'".to_string(),
                    span: token.span,
                }))
            }

            TokenKind::Backtick => {
                Ok(Expression::Identifier(Identifier {
                    name: "``".to_string(),
                    span: token.span,
                }))
            }

            _ => Err(ParseError::UnexpectedToken(token.kind.clone(), token.span)),
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        let mut first = self.parse_primary()?;

        while let Some(token) = self.peek() {
            match token.kind {
                TokenKind::EOF
                | TokenKind::CloseBrace
                | TokenKind::CloseBracket
                | TokenKind::CloseParen
                | TokenKind::Comma
                | TokenKind::Colon => break,
                _ => {
                    let right = self.parse_primary()?;
                    let span = Span::new(first.span().start, right.span().end);
                    first = Expression::Application(Application {
                        left: Box::new(first),
                        right: Box::new(right),
                        span,
                    });
                }
            }
        }

        Ok(Self::try_parse_import_export(Self::try_parse_catch(first)))
    }

    fn try_parse_catch(expr: Expression) -> Expression {
        // Look for: Application(Application(left, Identifier("!!!?")), handler)
        // Rewrite as: CatchExpression { operator: left, handler }
        if let Expression::Application(outer) = &expr {
            if let Expression::Application(inner) = &*outer.left {
                if let Expression::Identifier(ident) = &*inner.right {
                    if ident.name == "!!!?" {
                        return Expression::CatchExpression(CatchExpression {
                            operator: inner.left.clone(),
                            handler: outer.right.clone(),
                            span: expr.span(),
                        });
                    }
                }
            }
        }
        expr
    }

    fn try_parse_import_export(expr: Expression) -> Expression {
        if let Expression::Application(app) = &expr {
            if let Expression::Application(inner_app) = &*app.left {
                if let Expression::Identifier(ident) = &*inner_app.left {
                    if ident.name == "+" {
                        if let Expression::Identifier(right_ident) = &*inner_app.right {
                            if right_ident.name == ":" {
                                if let Expression::Application(path_app) = &*app.right {
                                    if let Expression::Identifier(source_ident) = &*path_app.left {
                                        if source_ident.name == "@" {
                                            if let Expression::StringLiteral(path_str) = &*path_app.right {
                                                return Expression::ImportExpression(ImportExpression {
                                                    source: Box::new(Expression::Identifier(Identifier {
                                                        name: "imports".to_string(),
                                                        span: source_ident.span,
                                                    })),
                                                    path: Box::new(Expression::StringLiteral(path_str.clone())),
                                                    destructuring: None,
                                                    span: expr.span(),
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if let Expression::Application(app) = &expr {
            if let Expression::Identifier(left_ident) = &*app.left {
                if left_ident.name == "imports" || left_ident.name == "files" {
                    if let Expression::Application(path_app) = &*app.right {
                        if let Expression::Identifier(at_ident) = &*path_app.left {
                            if at_ident.name == "@" {
                                if let Expression::StringLiteral(path_str) = &*path_app.right {
                                    return Expression::ImportExpression(ImportExpression {
                                        source: Box::new(Expression::Identifier(left_ident.clone())),
                                        path: Box::new(Expression::StringLiteral(path_str.clone())),
                                        destructuring: None,
                                        span: expr.span(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        expr
    }

    fn parse_program(&mut self, source_path: String) -> Result<Program, ParseError> {
        let expression = self.parse_expression()?;
        self.consume(TokenKind::EOF)?;
        Ok(Program {
            expression: Box::new(expression),
            source_path,
        })
    }
}

pub fn parse(tokens: Vec<Token>, source_path: String) -> Result<Program, ParseError> {
    let mut parser = Parser::new(tokens);
    parser.parse_program(source_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lr_lexer::tokenize;

    #[test]
    fn test_parse_number() {
        let tokens = tokenize("42").unwrap();
        let program = parse(tokens, "test.lr".to_string()).unwrap();
        assert!(matches!(*program.expression, Expression::NumberLiteral(_)));
    }

    #[test]
    fn test_parse_application() {
        let tokens = tokenize("5+3").unwrap();
        let program = parse(tokens, "test.lr".to_string()).unwrap();
        assert!(matches!(*program.expression, Expression::Application(_)));
    }

    #[test]
    fn test_parse_list() {
        let tokens = tokenize("[1, 2, 3]").unwrap();
        let program = parse(tokens, "test.lr".to_string()).unwrap();
        assert!(matches!(*program.expression, Expression::ListLiteral(_)));
    }

    #[test]
    fn test_parse_map() {
        let tokens = tokenize("{ a: 1 }").unwrap();
        let program = parse(tokens, "test.lr".to_string()).unwrap();
        assert!(matches!(*program.expression, Expression::MapLiteral(_)));
    }

    #[test]
    fn test_parse_nested_application() {
        let tokens = tokenize("a b c").unwrap();
        let program = parse(tokens, "test.lr".to_string()).unwrap();
        assert!(matches!(*program.expression, Expression::Application(_)));
        if let Expression::Application(app) = &*program.expression {
            assert!(matches!(*app.left, Expression::Application(_)));
        }
    }

    #[test]
    fn test_parse_grouped() {
        let tokens = tokenize("(a b)").unwrap();
        let program = parse(tokens, "test.lr".to_string()).unwrap();
        assert!(matches!(*program.expression, Expression::GroupedExpression(_)));
    }

    #[test]
    fn test_parse_empty_list() {
        let tokens = tokenize("[]").unwrap();
        let program = parse(tokens, "test.lr".to_string()).unwrap();
        assert!(matches!(*program.expression, Expression::ListLiteral(_)));
        if let Expression::ListLiteral(list) = &*program.expression {
            assert!(list.elements.is_empty());
        }
    }

    #[test]
    fn test_parse_empty_map() {
        let tokens = tokenize("{}").unwrap();
        let program = parse(tokens, "test.lr".to_string()).unwrap();
        assert!(matches!(*program.expression, Expression::MapLiteral(_)));
        if let Expression::MapLiteral(map) = &*program.expression {
            assert!(map.entries.is_empty());
        }
    }

    #[test]
    fn test_parse_left_right_arg() {
        let tokens = tokenize("_< _>").unwrap();
        let program = parse(tokens, "test.lr".to_string()).unwrap();
        assert!(matches!(*program.expression, Expression::Application(_)));
    }

    #[test]
    fn test_parse_boolean() {
        let tokens = tokenize("true false").unwrap();
        let program = parse(tokens, "test.lr".to_string()).unwrap();
        assert!(matches!(*program.expression, Expression::Application(_)));
    }

    #[test]
    fn test_parse_string() {
        let tokens = tokenize("`hello`").unwrap();
        let program = parse(tokens, "test.lr".to_string()).unwrap();
        assert!(matches!(*program.expression, Expression::StringLiteral(_)));
    }

    // --- Integration tests with translation files ---

    #[test]
    #[ignore = "translation files contain inline ``` pseudo-comments not supported by spec; needs file cleanup"]
    fn test_parse_lookup_translation() {
        let source = include_str!("../../../../docs/translations/javascript/lookup-manual-translation.lr");
        let tokens = tokenize(source).expect("lookup translation should tokenize");
        let program = parse(tokens, "lookup-manual-translation.lr".to_string());
        assert!(program.is_ok(), "lookup translation should parse: {:?}", program.err());
        let expr = &*program.unwrap().expression;
        assert!(matches!(expr, Expression::MapLiteral(_)), "root should be a map");
    }

    #[test]
    #[ignore = "translation files contain inline ``` pseudo-comments not supported by spec; needs file cleanup"]
    fn test_parse_async_http_translation() {
        let source = include_str!("../../../../docs/translations/javascript/async-http-manual-translation.lr");
        let tokens = tokenize(source).expect("async-http translation should tokenize");
        let program = parse(tokens, "async-http-manual-translation.lr".to_string());
        assert!(program.is_ok(), "async-http translation should parse: {:?}", program.err());
        let expr = &*program.unwrap().expression;
        assert!(matches!(expr, Expression::MapLiteral(_)), "root should be a map");
    }

    #[test]
    fn test_parse_import_like_expression() {
        // +: is now a single compound token — use non-compound key for this map test
        let source = "{ config: imports@`lodash`@&[`map`] }";
        let tokens = tokenize(source).unwrap();
        let program = parse(tokens, "test.lr".to_string()).unwrap();
        assert!(matches!(*program.expression, Expression::MapLiteral(_)));
    }

    #[test]
    fn test_parse_map_with_operator_keys() {
        // +: is now a single compound token — use non-compound operator keys
        let source = "{ *: 1, /: 2 }";
        let tokens = tokenize(source).unwrap();
        let program = parse(tokens, "test.lr".to_string()).unwrap();
        if let Expression::MapLiteral(map) = &*program.expression {
            assert_eq!(map.entries.len(), 2);
        } else {
            panic!("expected MapLiteral");
        }
    }

    // --- Edge case tests ---

    #[test]
    fn test_parse_deeply_nested() {
        // (((1)))
        let tokens = tokenize("(((1)))").unwrap();
        let program = parse(tokens, "test.lr".to_string()).unwrap();
        // Should be GroupedExpression(GroupedExpression(GroupedExpression(NumberLiteral)))
        assert!(matches!(*program.expression, Expression::GroupedExpression(_)));
    }

    #[test]
    fn test_parse_complex_application_chain() {
        // a b c d e — should be Application(Application(Application(Application(a, b), c), d), e)
        let tokens = tokenize("a b c d e").unwrap();
        let program = parse(tokens, "test.lr".to_string()).unwrap();
        assert!(matches!(*program.expression, Expression::Application(_)));
    }

    #[test]
    fn test_parse_map_no_values() {
        // { a, b, c } — all assignment shorthand
        let tokens = tokenize("{ a, b, c }").unwrap();
        let program = parse(tokens, "test.lr".to_string()).unwrap();
        if let Expression::MapLiteral(map) = &*program.expression {
            assert_eq!(map.entries.len(), 3);
            // All should be assignment shorthand (no colon, no value)
            for entry in &map.entries {
                assert!(entry.is_assignment);
                assert!(entry.value.is_none());
            }
        } else {
            panic!("expected MapLiteral");
        }
    }

    #[test]
    fn test_parse_map_mixed_entries() {
        // { a, b: 1, c }
        let tokens = tokenize("{ a, b: 1, c }").unwrap();
        let program = parse(tokens, "test.lr".to_string()).unwrap();
        if let Expression::MapLiteral(map) = &*program.expression {
            assert_eq!(map.entries.len(), 3);
            assert!(map.entries[0].is_assignment);
            assert!(map.entries[0].value.is_none());
            assert!(map.entries[1].value.is_some());
            assert!(map.entries[2].is_assignment);
            assert!(map.entries[2].value.is_none());
        } else {
            panic!("expected MapLiteral");
        }
    }

    #[test]
    fn test_parse_nested_maps() {
        // { a: { b: 1 } }
        let tokens = tokenize("{ a: { b: 1 } }").unwrap();
        let program = parse(tokens, "test.lr".to_string()).unwrap();
        assert!(matches!(*program.expression, Expression::MapLiteral(_)));
    }

    #[test]
    fn test_parse_nested_lists() {
        // [[1, 2], [3, 4]]
        let tokens = tokenize("[[1, 2], [3, 4]]").unwrap();
        let program = parse(tokens, "test.lr".to_string()).unwrap();
        assert!(matches!(*program.expression, Expression::ListLiteral(_)));
        if let Expression::ListLiteral(list) = &*program.expression {
            assert_eq!(list.elements.len(), 2);
        }
    }

    #[test]
    fn test_parse_operators_as_identifiers() {
        // + - * / @ !!! ??? should all parse as identifiers
        let tokens = tokenize("+ - * /").unwrap();
        let program = parse(tokens, "test.lr".to_string()).unwrap();
        // Should be a chain of Applications of Identifiers
        assert!(matches!(*program.expression, Expression::Application(_)));
    }

    #[test]
    fn test_parse_catch_expression() {
        let tokens = tokenize("value !!!? handler").unwrap();
        let program = parse(tokens, "test.lr".to_string()).unwrap();
        assert!(matches!(*program.expression, Expression::CatchExpression(_)));
    }

    #[test]
    fn test_parse_standalone_catch_operator() {
        let tokens = tokenize("!!!?").unwrap();
        let program = parse(tokens, "test.lr".to_string()).unwrap();
        assert!(matches!(*program.expression, Expression::Identifier(_)));
    }

    #[test]
    fn test_parse_undefined() {
        let tokens = tokenize("undefined").unwrap();
        let program = parse(tokens, "test.lr".to_string()).unwrap();
        assert!(matches!(*program.expression, Expression::UndefinedLiteral(_)));
    }

    #[test]
    fn test_parse_negative_number() {
        // -5 is Application(Identifier("-"), NumberLiteral(5)) per spec
        let tokens = tokenize("-5").unwrap();
        let program = parse(tokens, "test.lr".to_string()).unwrap();
        assert!(matches!(*program.expression, Expression::Application(_)));
    }

    #[test]
    fn test_parse_string_with_interpolation() {
        let tokens = tokenize("`hello {name}`").unwrap();
        let program = parse(tokens, "test.lr".to_string()).unwrap();
        assert!(matches!(*program.expression, Expression::StringLiteral(_)));
    }

    #[test]
    fn test_parse_minimal_string() {
        let tokens = tokenize("` `").unwrap();
        let program = parse(tokens, "test.lr".to_string()).unwrap();
        assert!(matches!(*program.expression, Expression::StringLiteral(_)));
    }

    #[test]
    fn test_parse_at_operator() {
        // @ is an identifier operator for path access
        let tokens = tokenize("data@`key`").unwrap();
        let program = parse(tokens, "test.lr".to_string()).unwrap();
        // Should be Application(Identifier("data"), Application(Identifier("@"), StringLiteral("key")))
        assert!(matches!(*program.expression, Expression::Application(_)));
    }

    #[test]
    fn test_parse_dot_operator() {
        let tokens = tokenize("data.name").unwrap();
        let program = parse(tokens, "test.lr".to_string()).unwrap();
        // Should be Application(Application(Identifier("data"), Dot), Identifier("name"))
        assert!(matches!(*program.expression, Expression::Application(_)));
    }

    #[test]
    fn test_parse_single_quote() {
        let tokens = tokenize("'template").unwrap();
        let program = parse(tokens, "test.lr".to_string()).unwrap();
        assert!(matches!(*program.expression, Expression::Application(_)));
    }
}