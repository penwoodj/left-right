use lr_ast::{
    Application, AsyncExpression, AwaitExpression, BooleanLiteral, CatchExpression,
    Expression, GroupedExpression, Identifier, ImportExpression, LeftArg,
    ListLiteral, MapEntry, MapLiteral, NumberLiteral, Program, RightArg, StringLiteral,
    StringPart, ThrowExpression, UndefinedLiteral,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CodegenError {
    #[error("Lexer error: {0}")]
    LexerError(lr_lexer::LexError),
    #[error("Parse error: {0}")]
    ParseError(lr_parser::ParseError),
    #[error("Unsupported expression: {0}")]
    Unsupported(String),
}

impl From<lr_lexer::LexError> for CodegenError {
    fn from(err: lr_lexer::LexError) -> Self {
        CodegenError::LexerError(err)
    }
}

impl From<lr_parser::ParseError> for CodegenError {
    fn from(err: lr_parser::ParseError) -> Self {
        CodegenError::ParseError(err)
    }
}

pub struct CodeGenerator {
    output: String,
    depth: usize,
    var_counter: usize,
    in_catch_handler: bool,
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            depth: 0,
            var_counter: 0,
            in_catch_handler: false,
        }
    }

    fn indent(&mut self) {
        for _ in 0..self.depth {
            self.output.push_str("  ");
        }
    }

    fn gen_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::NumberLiteral(n) => self.gen_number_literal(n),
            Expression::StringLiteral(s) => self.gen_string_literal(s),
            Expression::BooleanLiteral(b) => self.gen_boolean_literal(b),
            Expression::UndefinedLiteral(u) => self.gen_undefined_literal(u),
            Expression::Identifier(i) => self.gen_identifier(i),
            Expression::LeftArg(l) => self.gen_left_arg(l),
            Expression::RightArg(r) => self.gen_right_arg(r),
            Expression::ListLiteral(l) => self.gen_list_literal(l),
            Expression::MapLiteral(m) => self.gen_map_literal(m),
            Expression::Application(a) => self.gen_application(a),
            Expression::GroupedExpression(g) => self.gen_grouped_expression(g),
            Expression::ThrowExpression(t) => self.gen_throw_expression(t),
            Expression::CatchExpression(c) => self.gen_catch_expression(c),
            Expression::AsyncExpression(a) => self.gen_async_expression(a),
            Expression::AwaitExpression(a) => self.gen_await_expression(a),
            Expression::ImportExpression(i) => self.gen_import_expression(i),
        }
    }

    fn gen_number_literal(&mut self, n: &NumberLiteral) {
        self.output.push_str(&n.raw);
    }

    fn gen_string_literal(&mut self, s: &StringLiteral) {
        let has_interpolation = s.parts.iter().any(|p| matches!(p, StringPart::Interpolation { .. }));

        if !has_interpolation {
            if let Some(StringPart::Text(text)) = s.parts.first() {
                self.output.push('"');
                for c in text.chars() {
                    if c == '"' {
                        self.output.push_str("\\\"");
                    } else if c == '\\' {
                        self.output.push_str("\\\\");
                    } else {
                        self.output.push(c);
                    }
                }
                self.output.push('"');
            }
        } else {
            self.output.push('`');
            for part in &s.parts {
                match part {
                    StringPart::Text(text) => {
                        for c in text.chars() {
                            if c == '`' {
                                self.output.push_str("\\`");
                            } else if c == '\\' {
                                self.output.push_str("\\\\");
                            } else {
                                self.output.push(c);
                            }
                        }
                    }
                    StringPart::Interpolation { expression } => {
                        self.output.push_str("${");
                        self.gen_expression(expression);
                        self.output.push('}');
                    }
                }
            }
            self.output.push('`');
        }
    }

    fn gen_boolean_literal(&mut self, b: &BooleanLiteral) {
        self.output.push_str(&b.raw);
    }

    fn gen_undefined_literal(&mut self, _u: &UndefinedLiteral) {
        self.output.push_str("undefined");
    }

    fn gen_identifier(&mut self, i: &Identifier) {
        self.output.push_str(&i.name);
    }

    fn gen_left_arg(&mut self, _l: &LeftArg) {
        if self.in_catch_handler {
            self.output.push_str("__e");
        } else {
            self.output.push('x');
        }
    }

    fn gen_right_arg(&mut self, _r: &RightArg) {
        self.output.push('y');
    }

    fn gen_list_literal(&mut self, l: &ListLiteral) {
        self.output.push('[');
        for (i, elem) in l.elements.iter().enumerate() {
            if i > 0 {
                self.output.push_str(", ");
            }
            self.gen_expression(elem);
        }
        self.output.push(']');
    }

    fn gen_grouped_expression(&mut self, g: &GroupedExpression) {
        self.output.push('(');
        self.gen_expression(&g.expression);
        self.output.push(')');
    }

    fn contains_arg_ref(expr: &Expression) -> bool {
        match expr {
            Expression::LeftArg(_) | Expression::RightArg(_) => true,
            Expression::Application(a) => Self::contains_arg_ref(&a.left) || Self::contains_arg_ref(&a.right),
            Expression::GroupedExpression(g) => Self::contains_arg_ref(&g.expression),
            Expression::ListLiteral(l) => l.elements.iter().any(Self::contains_arg_ref),
            Expression::MapLiteral(m) => m.entries.iter().any(|e| {
                Self::contains_arg_ref(&e.key)
                    || e.value.as_ref().map_or(false, Self::contains_arg_ref)
            }),
            Expression::ThrowExpression(t) => Self::contains_arg_ref(&t.value),
            Expression::CatchExpression(c) => {
                Self::contains_arg_ref(&c.operator) || Self::contains_arg_ref(&c.handler)
            }
            Expression::AsyncExpression(a) => Self::contains_arg_ref(&a.operator),
            Expression::AwaitExpression(a) => Self::contains_arg_ref(&a.promise),
            Expression::ImportExpression(i) => {
                Self::contains_arg_ref(&i.source)
                    || Self::contains_arg_ref(&i.path)
                    || i.destructuring.as_ref().map_or(false, |d| Self::contains_arg_ref(d))
            }
            _ => false,
        }
    }

    fn contains_right_arg_ref(expr: &Expression) -> bool {
        match expr {
            Expression::RightArg(_) => true,
            Expression::Application(a) => {
                Self::contains_right_arg_ref(&a.left) || Self::contains_right_arg_ref(&a.right)
            }
            Expression::GroupedExpression(g) => Self::contains_right_arg_ref(&g.expression),
            Expression::ListLiteral(l) => l.elements.iter().any(Self::contains_right_arg_ref),
            Expression::MapLiteral(m) => m.entries.iter().any(|e| {
                Self::contains_right_arg_ref(&e.key)
                    || e.value.as_ref().map_or(false, Self::contains_right_arg_ref)
            }),
            _ => false,
        }
    }

    fn gen_application(&mut self, a: &Application) {
        let result = self.detect_operator_pattern(a);

        match result {
            OperatorPattern::Infix(op, left, right) => {
                self.gen_expression(left);
                self.output.push(' ');
                self.gen_operator_symbol(op);
                self.output.push(' ');
                self.gen_expression(right);
            }
            OperatorPattern::Dollar(op, left, right) => {
                self.gen_dollar_operator(op, left, right);
            }
            OperatorPattern::PropertyAccess(obj, key) => {
                self.gen_property_access(obj, key);
            }
            OperatorPattern::Size(expr) => {
                self.output.push('(');
                self.gen_expression(expr);
                self.output.push_str(").length");
            }
            OperatorPattern::Throw(expr) => {
                self.output.push_str("throw ");
                self.gen_expression(expr);
            }
            OperatorPattern::Partial(left, op) => {
                self.gen_expression(left);
                self.output.push(' ');
                self.output.push_str(op);
            }
            OperatorPattern::FunctionCall(func, args) => {
                self.output.push_str(func);
                self.output.push('(');
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.output.push_str(", ");
                    }
                    self.gen_expression(arg);
                }
                self.output.push(')');
            }
            OperatorPattern::ClosureApply(closure, args) => {
                self.output.push('(');
                self.gen_expression(closure);
                self.output.push(')');
                self.output.push('(');
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.output.push_str(", ");
                    }
                    self.gen_expression(arg);
                }
                self.output.push(')');
            }
            OperatorPattern::Generic(left, right) => {
                self.output.push('(');
                self.gen_expression(left);
                self.output.push(' ');
                self.gen_expression(right);
                self.output.push(')');
            }
        }
    }

    fn detect_operator_pattern<'a>(&self, a: &'a Application) -> OperatorPattern<'a> {
        // Check for nested Application on left with operator on inner right
        if let Expression::Application(inner_app) = a.left.as_ref() {
            if let Expression::Identifier(op_ident) = inner_app.right.as_ref() {
                let op = &op_ident.name;

                if self.is_infix_operator(op) {
                    return OperatorPattern::Infix(op, &inner_app.left, &a.right);
                } else if op.starts_with('$') {
                    return OperatorPattern::Dollar(op, &inner_app.left, &a.right);
                } else if op == "@" {
                    return OperatorPattern::PropertyAccess(&inner_app.left, &a.right);
                } else if op == "#" {
                    return OperatorPattern::Size(&inner_app.left);
                }
            }

            // Check if left is a chain of closure applications
            // e.g., { _< + _> } 3 5 → ClosureApply(closure, [3, 5])
            if let Some((closure, args)) = self.collect_closure_args(&a.left) {
                let mut all_args = args;
                all_args.push(&a.right);
                return OperatorPattern::ClosureApply(closure, all_args);
            }
        }

        // Check for direct closure application: { _< + 1 } 5
        if Self::is_closure_expr(&a.left) {
            return OperatorPattern::ClosureApply(&a.left, vec![&a.right]);
        }

        if let Expression::Identifier(op_ident) = a.right.as_ref() {
            if op_ident.name == "#" {
                return OperatorPattern::Size(&a.left);
            }
            if op_ident.name == "!!!" {
                return OperatorPattern::Throw(&a.left);
            }
            if self.is_infix_operator(&op_ident.name) {
                return OperatorPattern::Partial(&a.left, &op_ident.name);
            }
        }

        if let Expression::Identifier(func_ident) = a.right.as_ref() {
            if let Expression::ListLiteral(list) = a.left.as_ref() {
                let args: Vec<_> = list.elements.iter().collect();
                return OperatorPattern::FunctionCall(&func_ident.name, args);
            } else {
                return OperatorPattern::FunctionCall(&func_ident.name, vec![&a.left]);
            }
        }

        // Check if left is a closure applied to right (non-identifier right)
        if Self::is_closure_expr(&a.left) {
            return OperatorPattern::ClosureApply(&a.left, vec![&a.right]);
        }

        OperatorPattern::Generic(&a.left, &a.right)
    }

    fn is_closure_expr(expr: &Expression) -> bool {
        if let Expression::MapLiteral(m) = expr {
            m.entries.iter().any(|e| {
                matches!(e.key, Expression::LeftArg(_) | Expression::RightArg(_))
                    || Self::contains_arg_ref(&e.key)
                    || e.value.as_ref().map_or(false, |v| Self::contains_arg_ref(v))
            })
        } else {
            false
        }
    }

    /// Walk a chain of applications to find a closure at the base and collect args.
    /// For `{ _< + _> } 3 5`:
    ///   Application { left: Application { left: MapLiteral, right: 3 }, right: 5 }
    ///   → returns (MapLiteral, [3])
    /// The caller adds the outer right (5) to complete the arg list.
    fn collect_closure_args<'a>(&self, expr: &'a Expression) -> Option<(&'a Expression, Vec<&'a Expression>)> {
        if let Expression::Application(inner) = expr {
            if Self::is_closure_expr(&inner.left) {
                return Some((&inner.left, vec![&inner.right]));
            }
            // Recurse deeper
            if let Some((closure, mut args)) = self.collect_closure_args(&inner.left) {
                args.push(&inner.right);
                return Some((closure, args));
            }
        }
        None
    }

    fn is_spread_entry(entry: &MapEntry) -> bool {        match &entry.key {
            Expression::Identifier(ident) if ident.name == "+:" && entry.value.is_some() => true,
            Expression::Application(app) => {
                if let Expression::Identifier(ident) = app.left.as_ref() {
                    ident.name == "+:"
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn is_infix_operator(&self, op: &str) -> bool {
        matches!(op,
            "+" | "-" | "*" | "/" | "%" | "^" |
            "==" | "!=" | "=" | "<" | ">" | "<=" | ">=" |
            "&" | "|" | "!"
        )
    }

    fn gen_operator_symbol(&mut self, op: &str) {
        match op {
            "^" => self.output.push_str("**"),
            "==" => self.output.push_str("==="),
            "!=" => self.output.push_str("!=="),
            "=" => self.output.push_str("==="),
            "&" => self.output.push_str("&&"),
            "|" => self.output.push_str("||"),
            "!" => self.output.push('!'),
            _ => self.output.push_str(op),
        }
    }

    fn gen_dollar_operator(&mut self, op: &str, left: &Expression, right: &Expression) {
        match op {
            "$" => {
                self.gen_expression(left);
                self.output.push_str(".map(");
                self.gen_closure(right);
                self.output.push(')');
            }
            "$?" => {
                self.gen_expression(left);
                self.output.push_str(".filter(");
                self.gen_closure(right);
                self.output.push(')');
            }
            "$_" => {
                self.gen_expression(left);
                self.output.push_str(".flatMap(");
                self.gen_closure(right);
                self.output.push(')');
            }
            "$|" => {
                self.gen_expression(left);
                self.output.push_str(".some(");
                self.gen_closure(right);
                self.output.push(')');
            }
            "$&" => {
                self.gen_expression(left);
                self.output.push_str(".every(");
                self.gen_closure(right);
                self.output.push(')');
            }
            "$?|" => {
                self.gen_expression(left);
                self.output.push_str(".find(");
                self.gen_closure(right);
                self.output.push(')');
            }
            "$%" => {
                self.output.push_str("[...");
                self.gen_expression(left);
                self.output.push_str("].sort(");
                self.gen_closure(right);
                self.output.push(')');
            }
            "$?!" => {
                self.gen_expression(left);
                self.output.push_str(".filter(Boolean)");
            }
            "$@" => {
                self.gen_expression(left);
                self.output.push_str(".map(x => x[");
                self.gen_expression(right);
                self.output.push_str("])");
            }
            "$\"" => {
                self.gen_expression(left);
                self.output.push_str(".map(String)");
            }
            "$|||" => {
                self.output.push_str("await Promise.all(");
                self.gen_expression(left);
                self.output.push_str(".map(");
                self.gen_closure(right);
                self.output.push_str("))");
            }
            "$~" => {
                self.output.push_str("/* uniqueBy: ");
                self.gen_expression(left);
                self.output.push_str(" $~ ");
                self.gen_expression(right);
                self.output.push_str(" */");
            }
            "$>" => {
                self.output.push_str("/* groupBy: ");
                self.gen_expression(left);
                self.output.push_str(" $> ");
                self.gen_expression(right);
                self.output.push_str(" */");
            }
            "$+" | "$-" | "$*" | "$/" => {
                self.gen_expression(left);
                self.output.push_str(".map((x, i) => x ");
                self.gen_operator_symbol(&op[1..]);
                self.output.push(' ');
                self.gen_expression(right);
                self.output.push_str("[i])");
            }
            "$?>" | "$?<" | "$?>=" | "$?<=" | "$?+" | "$?-" => {
                self.gen_expression(left);
                self.output.push_str(".filter(x => x ");
                self.gen_operator_symbol(&op[2..]);
                self.output.push(' ');
                self.gen_expression(right);
                self.output.push(')');
            }
            _ => {
                self.output.push_str("/* unknown operator: ");
                self.output.push_str(op);
                self.output.push_str(" */");
            }
        }
    }

    fn gen_property_access(&mut self, obj: &Expression, key: &Expression) {
        self.gen_expression(obj);
        self.output.push('[');
        self.gen_expression(key);
        self.output.push(']');
    }

    fn gen_closure(&mut self, expr: &Expression) {
        if let Expression::MapLiteral(map) = expr {
            let has_right_arg = map.entries.iter().any(|e| {
                matches!(e.key, Expression::RightArg(_))
                    || Self::contains_right_arg_ref(&e.key)
                    || e.value.as_ref().map_or(false, |v| Self::contains_right_arg_ref(v))
            });
            let has_left_arg = map.entries.iter().any(|e| {
                matches!(e.key, Expression::LeftArg(_))
                    || Self::contains_arg_ref(&e.key)
                    || e.value.as_ref().map_or(false, |v| Self::contains_arg_ref(v))
            });

            if has_right_arg {
                self.output.push_str("(x, y) => ");
            } else if has_left_arg {
                self.output.push_str("x => ");
            }

            self.gen_map_literal_body(map);
        } else {
            self.output.push_str("x => ");
            self.gen_expression(expr);
        }
    }

    fn gen_map_literal(&mut self, m: &MapLiteral) {
        let has_arg_keys = m.entries.iter().any(|e| {
            matches!(e.key, Expression::LeftArg(_) | Expression::RightArg(_))
                || Self::contains_arg_ref(&e.key)
                || e.value.as_ref().map_or(false, |v| Self::contains_arg_ref(v))
        });
        let non_arg_entries: Vec<_> = m.entries.iter()
            .filter(|e| !matches!(e.key, Expression::LeftArg(_) | Expression::RightArg(_)))
            .collect();

        let is_program = !non_arg_entries.is_empty() && {
            let last_idx = non_arg_entries.len() - 1;
            let last_entry = non_arg_entries[last_idx];

            if last_entry.value.is_some() || Self::is_spread_entry(last_entry) {
                false
            } else {
                let earlier_has_value = non_arg_entries[..last_idx].iter().any(|e| e.value.is_some());
                earlier_has_value
            }
        };

        if has_arg_keys && !is_program {
            let has_right_arg = m.entries.iter().any(|e| {
                matches!(e.key, Expression::RightArg(_))
                    || Self::contains_right_arg_ref(&e.key)
                    || e.value.as_ref().map_or(false, |v| Self::contains_right_arg_ref(v))
            });

            if has_right_arg {
                self.output.push_str("(x, y) => ");
            } else {
                self.output.push_str("x => ");
            }

            self.gen_map_literal_body(m);
        } else if is_program {
            self.gen_program_literal(&non_arg_entries);
        } else {
            self.gen_object_literal(m);
        }
    }

    fn gen_program_literal(&mut self, entries: &[&MapEntry]) {
        self.output.push_str("(() => {\n");
        self.depth += 1;

        let last_idx = entries.len() - 1;
        for entry in &entries[..last_idx] {
            if let Expression::Identifier(ident) = &entry.key {
                if ident.name == "+:" {
                    continue;
                }
                self.indent();
                self.output.push_str("const ");
                self.output.push_str(&ident.name);
                self.output.push_str(" = ");
                if let Some(ref value) = entry.value {
                    self.gen_expression(value);
                } else {
                    self.gen_expression(&entry.key);
                }
                self.output.push_str(";\n");
            }
        }

        let last_entry = &entries[last_idx];
        self.indent();
        self.output.push_str("return ");
        if let Some(ref value) = last_entry.value {
            self.gen_expression(value);
        } else {
            self.gen_expression(&last_entry.key);
        }
        self.output.push_str(";\n");

        self.depth -= 1;
        self.indent();
        self.output.push_str("})()");
    }

    fn gen_object_literal(&mut self, m: &MapLiteral) {
        self.output.push('{');
        for (i, entry) in m.entries.iter().enumerate() {
            if i > 0 {
                self.output.push_str(", ");
            }

            if let Expression::Identifier(ident) = &entry.key {
                if ident.name == "+:" {
                    self.output.push_str("...");
                    if let Some(ref value) = entry.value {
                        self.gen_expression(value);
                    }
                    continue;
                }
            }

            if let Expression::Application(app) = &entry.key {
                if let Expression::Identifier(ident) = app.left.as_ref() {
                    if ident.name == "+:" {
                        self.output.push_str("...");
                        self.gen_expression(&app.right);
                        continue;
                    }
                }
            }

            self.gen_expression(&entry.key);
            if let Some(ref value) = entry.value {
                self.output.push_str(": ");
                self.gen_expression(value);
            }
        }
        self.output.push('}');
    }

    fn gen_map_literal_body(&mut self, m: &MapLiteral) {
        let entries: Vec<_> = m.entries.iter()
            .filter(|e| !matches!(e.key, Expression::LeftArg(_) | Expression::RightArg(_)))
            .collect();

        if entries.is_empty() {
            self.output.push_str("undefined");
            return;
        }

        let last_idx = entries.len() - 1;
        let is_simple_return = entries.len() == 1
            || (entries[last_idx].is_expression_key
                && entries[last_idx].value.is_none()
                && !entries[..last_idx].iter().any(|e| e.is_assignment));

        if is_simple_return {
            let last_entry = &entries[last_idx];
            if let Some(ref value) = last_entry.value {
                self.gen_expression(value);
            } else {
                self.gen_expression(&last_entry.key);
            }
        } else {
            self.output.push_str("{\n");
            self.depth += 1;

            for entry in &entries {
                if let Expression::Identifier(ident) = &entry.key {
                    if ident.name.ends_with('?') && entry.value.is_some() {
                        let base_name = &ident.name[..ident.name.len() - 1];
                        self.indent();
                        self.output.push_str("if (");
                        self.output.push_str(base_name);
                        self.output.push_str(") return ");
                        if let Some(ref value) = entry.value {
                            self.gen_expression(value);
                        }
                        self.output.push_str(";\n");
                        continue;
                    }
                }

                if entry.is_assignment {
                    if let Expression::Identifier(ident) = &entry.key {
                        self.indent();
                        self.output.push_str("const ");
                        self.output.push_str(&ident.name);
                        self.output.push_str(" = ");
                        if let Some(ref value) = entry.value {
                            self.gen_expression(value);
                        } else {
                            self.gen_expression(&entry.key);
                        }
                        self.output.push_str(";\n");
                    }
                }
            }

            let last_entry = &entries[last_idx];
            self.indent();
            self.output.push_str("return ");
            if let Some(ref value) = last_entry.value {
                self.gen_expression(value);
            } else {
                self.gen_expression(&last_entry.key);
            }
            self.output.push_str(";\n");

            self.depth -= 1;
            self.indent();
            self.output.push('}');
        }
    }

    fn gen_throw_expression(&mut self, t: &ThrowExpression) {
        self.output.push_str("throw ");
        self.gen_expression(&t.value);
    }

    fn gen_catch_expression(&mut self, c: &CatchExpression) {
        self.output.push_str("try {");
        self.gen_expression(&c.operator);
        self.output.push_str("} catch (__e) {\n");
        self.depth += 1;

        let prev_in_catch = self.in_catch_handler;
        self.in_catch_handler = true;

        if let Expression::MapLiteral(map) = c.handler.as_ref() {
            self.gen_handler_body(map);
        } else {
            self.indent();
            self.gen_expression(&c.handler);
            self.output.push('\n');
        }

        self.in_catch_handler = prev_in_catch;

        self.depth -= 1;
        self.indent();
        self.output.push('}');
    }

    fn gen_handler_body(&mut self, m: &MapLiteral) {
        let entries: Vec<_> = m.entries.iter()
            .filter(|e| !matches!(e.key, Expression::LeftArg(_) | Expression::RightArg(_)))
            .collect();

        if entries.is_empty() {
            return;
        }

        let last_idx = entries.len() - 1;
        let is_simple_return = entries.len() == 1
            || (entries[last_idx].is_expression_key
                && entries[last_idx].value.is_none()
                && !entries[..last_idx].iter().any(|e| e.is_assignment));

        if is_simple_return {
            let last_entry = &entries[last_idx];
            if let Some(ref value) = last_entry.value {
                self.indent();
                self.gen_expression(value);
                self.output.push('\n');
            } else {
                self.indent();
                self.gen_expression(&last_entry.key);
                self.output.push('\n');
            }
        } else {
            for entry in &entries[..last_idx] {
                if entry.is_assignment {
                    if let Expression::Identifier(ident) = &entry.key {
                        self.indent();
                        self.output.push_str("const ");
                        self.output.push_str(&ident.name);
                        self.output.push_str(" = ");
                        if let Some(ref value) = entry.value {
                            self.gen_expression(value);
                        } else {
                            self.gen_expression(&entry.key);
                        }
                        self.output.push_str(";\n");
                    }
                }
            }

            let last_entry = &entries[last_idx];
            self.indent();
            self.output.push_str("return ");
            if let Some(ref value) = last_entry.value {
                self.gen_expression(value);
            } else {
                self.gen_expression(&last_entry.key);
            }
            self.output.push_str(";\n");
        }
    }

    fn gen_async_expression(&mut self, a: &AsyncExpression) {
        if let Expression::MapLiteral(_) = a.operator.as_ref() {
            self.output.push_str("async ");
            self.gen_closure(&a.operator);
        } else {
            self.output.push_str("(async () => ");
            self.gen_expression(&a.operator);
            self.output.push_str(")()");
        }
    }

    fn gen_await_expression(&mut self, a: &AwaitExpression) {
        self.output.push_str("await ");
        self.gen_expression(&a.promise);
    }

    fn gen_import_expression(&mut self, i: &ImportExpression) {
        if let Expression::Identifier(source_ident) = i.source.as_ref() {
            if source_ident.name == "files" {
                self.output.push_str("require(");
                self.gen_expression(&i.path);
                self.output.push(')');
            } else if source_ident.name == "imports" {
                self.output.push_str("/* npm import: ");
                self.gen_expression(&i.path);
                self.output.push_str(" */");
            }
        } else {
            self.output.push_str("/* import: ");
            self.gen_expression(&i.source);
            self.output.push('@');
            self.gen_expression(&i.path);
            self.output.push_str(" */");
        }
    }

    pub fn transpile(program: &Program) -> Result<String, CodegenError> {
        let mut codegen = Self::new();
        codegen.gen_expression(&program.expression);
        Ok(codegen.output)
    }
}

enum OperatorPattern<'a> {
    Infix(&'a str, &'a Expression, &'a Expression),
    Dollar(&'a str, &'a Expression, &'a Expression),
    PropertyAccess(&'a Expression, &'a Expression),
    Size(&'a Expression),
    Throw(&'a Expression),
    Partial(&'a Expression, &'a str),
    FunctionCall(&'a str, Vec<&'a Expression>),
    ClosureApply(&'a Expression, Vec<&'a Expression>),
    Generic(&'a Expression, &'a Expression),
}

impl Default for CodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

pub fn transpile_source(source: &str) -> Result<String, CodegenError> {
    transpile_source_with_name(source, "inline.lr")
}

pub fn transpile_source_with_name(source: &str, _name: &str) -> Result<String, CodegenError> {
    let tokens = lr_lexer::tokenize(source).map_err(|errs| {
        CodegenError::LexerError(errs[0].clone())
    })?;
    let program = lr_parser::parse(tokens, _name.to_string())?;
    CodeGenerator::transpile(&program)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t(source: &str) -> String {
        transpile_source(source).unwrap()
    }

    #[test]
    fn test_program_map() {
        let result = t("{ x: 1, x }");
        assert!(result.contains("const x"), "{}", result);
        assert!(result.contains("return x"), "{}", result);
    }

    #[test]
    fn test_program_map_chained() {
        let result = t("{ a: 1, b: a + 1, b }");
        assert!(result.contains("const a = 1"), "{}", result);
        assert!(result.contains("const b = a + 1"), "{}", result);
        assert!(result.contains("return b"), "{}", result);
    }

    #[test]
    fn test_literals() {
        assert_eq!(t("42"), "42");
        assert_eq!(t("3.14"), "3.14");
        assert_eq!(t("true"), "true");
        assert_eq!(t("false"), "false");
        assert_eq!(t("undefined"), "undefined");
    }

    #[test]
    fn test_string_literal() {
        assert_eq!(t("`hello`"), "\"hello\"");
    }

    #[test]
    fn test_list_literal() {
        assert_eq!(t("[1, 2, 3]"), "[1, 2, 3]");
        assert_eq!(t("[]"), "[]");
    }

    #[test]
    fn test_infix_ops() {
        assert_eq!(t("5 + 3"), "5 + 3");
        assert_eq!(t("5 - 3"), "5 - 3");
        assert_eq!(t("5 * 3"), "5 * 3");
        assert_eq!(t("10 / 2"), "10 / 2");
        assert_eq!(t("10 % 3"), "10 % 3");
        assert_eq!(t("2 ^ 3"), "2 ** 3");
    }

    #[test]
    fn test_comparison_ops() {
        assert_eq!(t("5 == 3"), "5 === 3");
        assert_eq!(t("5 != 3"), "5 !== 3");
        assert_eq!(t("5 < 3"), "5 < 3");
        assert_eq!(t("5 > 3"), "5 > 3");
        assert_eq!(t("5 <= 3"), "5 <= 3");
        assert_eq!(t("5 >= 3"), "5 >= 3");
    }

    #[test]
    fn test_boolean_ops() {
        assert_eq!(t("true & false"), "true && false");
        assert_eq!(t("true | false"), "true || false");
    }

    #[test]
    fn test_property_access() {
        assert_eq!(t("options@`key`"), "options[\"key\"]");
    }

    #[test]
    fn test_size_op() {
        assert_eq!(t("[1, 2, 3] #"), "([1, 2, 3]).length");
    }

    #[test]
    fn test_function_call() {
        assert_eq!(t("entities removePrivateIps"), "removePrivateIps(entities)");
    }

    #[test]
    fn test_spread_object() {
        let result = t("{ a: 1, +: other }");
        assert!(result.contains("...other"), "{}", result);
        assert!(result.contains("a: 1"), "{}", result);
    }

    #[test]
    fn test_closure_monadic() {
        let result = t("[1, 2, 3] $ { _< * 2 }");
        assert!(result.contains(".map("), "{}", result);
        assert!(result.contains("=>"), "{}", result);
    }

    #[test]
    fn test_closure_filter() {
        let result = t("[1, 2, 3] $? { _< > 2 }");
        assert!(result.contains(".filter("), "{}", result);
    }

    #[test]
    fn test_parallel_map() {
        let result = t("[1, 2, 3] $||| { _< * 2 }");
        assert!(result.contains("await Promise.all("), "{}", result);
        assert!(result.contains(".map("), "{}", result);
    }

    #[test]
    fn test_plain_object() {
        let result = t("{ a: 1, b: 2 }");
        assert!(result.starts_with("{"), "{}", result);
        assert!(result.contains("a: 1"), "{}", result);
        assert!(result.contains("b: 2"), "{}", result);
    }

    #[test]
    fn test_closure_apply() {
        let result = t("{ _< + 1 } 5");
        assert!(result.contains("=>"), "{}", result);
        assert!(result.contains("+ 1"), "{}", result);
    }

    #[test]
    fn test_strict_left_to_right() {
        let result = t("1 + 2 * 3");
        assert!(result.contains("+"), "{}", result);
        assert!(result.contains("*"), "{}", result);
    }

    #[test]
    fn test_throw() {
        assert_eq!(t("42 !!!"), "throw 42");
    }

    #[test]
    fn test_await() {
        assert_eq!(t("42 \\\\\\"), "await 42");
    }

    #[test]
    fn test_async() {
        let result = t("42 ///");
        assert!(result.contains("async"), "{}", result);
        assert!(result.contains("42"), "{}", result);
    }
}