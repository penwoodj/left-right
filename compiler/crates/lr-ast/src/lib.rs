use lr_common::Span;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    NumberLiteral(NumberLiteral),
    StringLiteral(StringLiteral),
    BooleanLiteral(BooleanLiteral),
    UndefinedLiteral(UndefinedLiteral),
    ListLiteral(ListLiteral),
    MapLiteral(MapLiteral),
    Identifier(Identifier),
    LeftArg(LeftArg),
    RightArg(RightArg),
    Application(Application),
    GroupedExpression(GroupedExpression),
    ThrowExpression(ThrowExpression),
    CatchExpression(CatchExpression),
    AsyncExpression(AsyncExpression),
    AwaitExpression(AwaitExpression),
    ImportExpression(ImportExpression),
    ExportExpression(ExportExpression),
}

    impl fmt::Display for Expression {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Expression::NumberLiteral(n) => write!(f, "{}", n),
                Expression::StringLiteral(s) => write!(f, "{}", s),
                Expression::BooleanLiteral(b) => write!(f, "{}", b),
                Expression::UndefinedLiteral(u) => write!(f, "{}", u),
                Expression::ListLiteral(l) => write!(f, "{}", l),
                Expression::MapLiteral(m) => write!(f, "{}", m),
                Expression::Identifier(i) => write!(f, "{}", i),
                Expression::LeftArg(l) => write!(f, "{}", l),
                Expression::RightArg(r) => write!(f, "{}", r),
                Expression::Application(a) => write!(f, "{}", a),
                Expression::GroupedExpression(g) => write!(f, "{}", g),
                Expression::ThrowExpression(t) => write!(f, "{}", t),
                Expression::CatchExpression(c) => write!(f, "{}", c),
                Expression::AsyncExpression(a) => write!(f, "{}", a),
                Expression::AwaitExpression(a) => write!(f, "{}", a),
                Expression::ImportExpression(i) => write!(f, "{}", i),
                Expression::ExportExpression(e) => write!(f, "{}", e),
            }
        }
    }

#[derive(Debug, Clone, PartialEq)]
pub struct NumberLiteral {
    pub value: f64,
    pub raw: String,
    pub span: Span,
}

impl fmt::Display for NumberLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.raw)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StringPart {
    Text(String),
    Interpolation { expression: Box<Expression> },
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringLiteral {
    pub parts: Vec<StringPart>,
    pub span: Span,
}

impl fmt::Display for StringLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for part in &self.parts {
            match part {
                StringPart::Text(text) => write!(f, "{}", text)?,
                StringPart::Interpolation { expression } => write!(f, "{{{}}}", expression)?,
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BooleanLiteral {
    pub value: bool,
    pub raw: String,
    pub span: Span,
}

impl fmt::Display for BooleanLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.raw)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UndefinedLiteral {
    pub raw: String,
    pub span: Span,
}

impl fmt::Display for UndefinedLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.raw)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListLiteral {
    pub elements: Vec<Expression>,
    pub span: Span,
}

impl fmt::Display for ListLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, elem) in self.elements.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", elem)?;
        }
        write!(f, "]")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MapEntry {
    pub key: Expression,
    pub value: Option<Expression>,
    pub is_assignment: bool,
    pub is_expression_key: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MapLiteral {
    pub entries: Vec<MapEntry>,
    pub span: Span,
}

impl fmt::Display for MapLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for (i, entry) in self.entries.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", entry.key)?;
            if let Some(ref value) = entry.value {
                write!(f, ": {}", value)?;
            }
        }
        write!(f, "}}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub name: String,
    pub span: Span,
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LeftArg {
    pub raw: String,
    pub span: Span,
}

impl fmt::Display for LeftArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "_<")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RightArg {
    pub raw: String,
    pub span: Span,
}

impl fmt::Display for RightArg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "_>")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Application {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub span: Span,
}

impl fmt::Display for Application {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.left, self.right)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupedExpression {
    pub expression: Box<Expression>,
    pub span: Span,
}

impl fmt::Display for GroupedExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.expression)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowExpression {
    pub value: Box<Expression>,
    pub span: Span,
}

impl fmt::Display for ThrowExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} !!!)", self.value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CatchExpression {
    pub operator: Box<Expression>,
    pub handler: Box<Expression>,
    pub span: Span,
}

impl fmt::Display for CatchExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} !!!? {})", self.operator, self.handler)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AsyncExpression {
    pub operator: Box<Expression>,
    pub span: Span,
}

impl fmt::Display for AsyncExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} ///)", self.operator)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AwaitExpression {
    pub promise: Box<Expression>,
    pub span: Span,
}

impl fmt::Display for AwaitExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} \\\\\\)", self.promise)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportExpression {
    pub source: Box<Expression>,
    pub path: Box<Expression>,
    pub destructuring: Option<Box<Expression>>,
    pub span: Span,
}

impl fmt::Display for ImportExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}", self.source, self.path)?;
        if let Some(ref destructure) = self.destructuring {
            write!(f, "{}", destructure)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExportExpression {
    pub keys: Vec<String>,
    pub span: Span,
}

impl fmt::Display for ExportExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "}}@&[")?;
        for (i, key) in self.keys.iter().enumerate() {
            if i > 0 {
                write!(f, ",")?;
            }
            write!(f, "`{}`", key)?;
        }
        write!(f, "]")
    }
}

#[derive(Debug, Clone, PartialEq)]
    pub struct Program {
    pub expression: Box<Expression>,
    pub source_path: String,
}

    impl Expression {
        pub fn span(&self) -> Span {
            match self {
                Expression::NumberLiteral(n) => n.span,
                Expression::StringLiteral(s) => s.span,
                Expression::BooleanLiteral(b) => b.span,
                Expression::UndefinedLiteral(u) => u.span,
                Expression::ListLiteral(l) => l.span,
                Expression::MapLiteral(m) => m.span,
                Expression::Identifier(i) => i.span,
                Expression::LeftArg(l) => l.span,
                Expression::RightArg(r) => r.span,
                Expression::Application(a) => a.span,
                Expression::GroupedExpression(g) => g.span,
                Expression::ThrowExpression(t) => t.span,
                Expression::CatchExpression(c) => c.span,
                Expression::AsyncExpression(a) => a.span,
                Expression::AwaitExpression(a) => a.span,
                Expression::ImportExpression(i) => i.span,
                Expression::ExportExpression(e) => e.span,
            }
        }
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_literal() {
        let num = NumberLiteral {
            value: 42.0,
            raw: "42".to_string(),
            span: Span::new(0, 2),
        };
        assert_eq!(num.value, 42.0);
        assert_eq!(num.raw, "42");
    }

    #[test]
    fn test_string_literal() {
        let string = StringLiteral {
            parts: vec![StringPart::Text("hello".to_string())],
            span: Span::new(0, 7),
        };
        assert_eq!(string.parts.len(), 1);
    }

    #[test]
    fn test_list_literal() {
        let list = ListLiteral {
            elements: vec![
                Expression::NumberLiteral(NumberLiteral {
                    value: 1.0,
                    raw: "1".to_string(),
                    span: Span::new(0, 1),
                }),
                Expression::NumberLiteral(NumberLiteral {
                    value: 2.0,
                    raw: "2".to_string(),
                    span: Span::new(2, 3),
                }),
            ],
            span: Span::new(0, 5),
        };
        assert_eq!(list.elements.len(), 2);
    }
}