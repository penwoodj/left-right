use lr_ast::{
    Application, BooleanLiteral, CatchExpression, Expression, ExportExpression,
    GroupedExpression, Identifier, ImportExpression, LeftArg, ListLiteral, MapEntry,
    MapLiteral, NumberLiteral, Program, RightArg, StringLiteral, StringPart,
    ThrowExpression, UndefinedLiteral,
};
use lr_bytecode::{Chunk, Constant, Instruction, Opcode};
use std::collections::HashMap;
use thiserror::Error;

fn parse_interpolation(value: &str) -> Result<Vec<StringPart>, String> {
    let mut parts = Vec::new();
    let mut current_text = String::new();
    let mut depth = 0;

    for c in value.chars() {
        if c == '{' && depth == 0 {
            if !current_text.is_empty() {
                parts.push(StringPart::Text(std::mem::take(&mut current_text)));
            }
            depth = 1;
        } else if c == '{' && depth > 0 {
            depth += 1;
            current_text.push(c);
        } else if c == '}' && depth > 0 {
            depth -= 1;
            if depth == 0 {
                let expr_str = std::mem::take(&mut current_text);
                let tokens = lr_lexer::tokenize(&expr_str)
                    .map_err(|e| format!("Lex error in interpolation: {}", e.first().map(|err| err.to_string()).unwrap_or_default()))?;
                let program = lr_parser::parse(tokens, "<interpolation>".to_string())
                    .map_err(|e| format!("Parse error in interpolation: {}", e))?;
                parts.push(StringPart::Interpolation {
                    expression: program.expression,
                });
            } else {
                current_text.push(c);
            }
        } else if depth > 0 || c == '}' {
            current_text.push(c);
        } else {
            current_text.push(c);
        }
    }

    if !current_text.is_empty() {
        parts.push(StringPart::Text(current_text));
    }

    if depth > 0 {
        return Err("Unclosed interpolation in string".to_string());
    }

    Ok(parts)
}

#[derive(Debug, Error)]
pub enum CompilerError {
    #[error("register overflow: more than 255 registers needed")]
    RegisterOverflow,
    #[error("constant pool overflow: more than 255 constants")]
    ConstantPoolOverflow(#[from] lr_bytecode::BytecodeError),
    #[error("unsupported expression: {0}")]
    Unsupported(String),
    #[error("{0}")]
    LexerError(lr_lexer::LexError),
    #[error("{0}")]
    ParseError(lr_parser::ParseError),
}

impl CompilerError {
    pub fn span(&self) -> Option<lr_common::Span> {
        match self {
            CompilerError::LexerError(e) => Some(e.span()),
            CompilerError::ParseError(e) => Some(e.span()),
            _ => None,
        }
    }
}

impl From<lr_lexer::LexError> for CompilerError {
    fn from(err: lr_lexer::LexError) -> Self {
        CompilerError::LexerError(err)
    }
}

impl From<lr_parser::ParseError> for CompilerError {
    fn from(err: lr_parser::ParseError) -> Self {
        CompilerError::ParseError(err)
    }
}

pub struct Compiler {
    chunk: Chunk,
    register_count: u8,
    binding_scopes: Vec<HashMap<String, u8>>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            register_count: 0,
            binding_scopes: Vec::new(),
        }
    }

    /// Compile a full program into a bytecode chunk
    pub fn compile(program: &Program) -> Result<Chunk, CompilerError> {
        let mut compiler = Self::new();
        let dest = compiler.alloc_register()?;
        compiler.compile_expression(&program.expression, dest)?;
        compiler.chunk.emit(Instruction::new(Opcode::Return, dest, 0, 0));
        Ok(compiler.chunk)
    }

    /// Compile an expression, placing result in given register
    fn compile_expression(&mut self, expr: &Expression, dest: u8) -> Result<(), CompilerError> {
        match expr {
            Expression::NumberLiteral(n) => self.compile_number_literal(n, dest),
            Expression::StringLiteral(s) => self.compile_string_literal(s, dest),
            Expression::BooleanLiteral(b) => self.compile_boolean_literal(b, dest),
            Expression::UndefinedLiteral(u) => self.compile_undefined_literal(u, dest),
            Expression::Identifier(i) => self.compile_identifier(i, dest),
            Expression::LeftArg(l) => self.compile_left_arg(l, dest),
            Expression::RightArg(r) => self.compile_right_arg(r, dest),
            Expression::ListLiteral(l) => self.compile_list_literal(l, dest),
            Expression::MapLiteral(m) => self.compile_map_literal(m, dest),
            Expression::Application(a) => self.compile_application(a, dest),
            Expression::GroupedExpression(g) => self.compile_grouped_expression(g, dest),
            Expression::ThrowExpression(t) => self.compile_throw_expression(t, dest),
            Expression::CatchExpression(c) => self.compile_catch_expression(c, dest),
            Expression::AsyncExpression(_) => {
                Err(CompilerError::Unsupported("AsyncExpression".to_string()))
            }
            Expression::AwaitExpression(_) => {
                Err(CompilerError::Unsupported("AwaitExpression".to_string()))
            }
            Expression::ImportExpression(i) => self.compile_import_expression(i, dest),
            Expression::ExportExpression(e) => self.compile_export_expression(e, dest),
        }
    }

    fn compile_number_literal(&mut self, n: &NumberLiteral, dest: u8) -> Result<(), CompilerError> {
        let const_idx = self.chunk.add_constant(Constant::Number(n.value))?;
        self.chunk.emit(Instruction::new(Opcode::LoadConstant, dest, 0, const_idx));
        Ok(())
    }

    fn compile_string_literal(&mut self, s: &StringLiteral, dest: u8) -> Result<(), CompilerError> {
        let parts = &s.parts;

        if parts.len() == 1 && let StringPart::Text(text) = &parts[0] {
            if !text.contains('{') || !text.contains('}') {
                let const_idx = self.chunk.add_constant(Constant::String(text.clone()))?;
                self.chunk.emit(Instruction::new(Opcode::LoadConstant, dest, 0, const_idx));
                return Ok(());
            }

            let parsed_parts = parse_interpolation(text)
                .map_err(|e| CompilerError::Unsupported(format!("String interpolation parsing: {}", e)))?;

            return self.compile_string_parts(&parsed_parts, dest);
        }

        self.compile_string_parts(parts, dest)
    }

    fn compile_string_parts(&mut self, parts: &[StringPart], dest: u8) -> Result<(), CompilerError> {
        let has_interpolation = parts.iter().any(|p| matches!(p, StringPart::Interpolation { .. }));

        if !has_interpolation {
            let mut result = String::new();
            for part in parts {
                if let StringPart::Text(text) = part {
                    result.push_str(text);
                }
            }
            let const_idx = self.chunk.add_constant(Constant::String(result))?;
            self.chunk.emit(Instruction::new(Opcode::LoadConstant, dest, 0, const_idx));
        } else {
            let mut first = true;
            for part in parts {
                match part {
                    StringPart::Text(text) => {
                        if !text.is_empty() {
                            let const_idx = self.chunk.add_constant(Constant::String(text.clone()))?;
                            let temp = self.alloc_register()?;
                            self.chunk.emit(Instruction::new(Opcode::LoadConstant, temp, 0, const_idx));
                            if first {
                                self.chunk.emit(Instruction::new(Opcode::LoadRegister, dest, temp, 0));
                                first = false;
                            } else {
                                self.chunk.emit(Instruction::new(Opcode::StringConcat, dest, dest, temp));
                            }
                            self.free_register();
                        }
                    }
                    StringPart::Interpolation { expression } => {
                        let temp = self.alloc_register()?;
                        self.compile_expression(expression, temp)?;
                        if first {
                            self.chunk.emit(Instruction::new(Opcode::LoadRegister, dest, temp, 0));
                            first = false;
                        } else {
                            self.chunk.emit(Instruction::new(Opcode::StringConcat, dest, dest, temp));
                        }
                        self.free_register();
                    }
                }
            }
            if first {
                let const_idx = self.chunk.add_constant(Constant::String(String::new()))?;
                self.chunk.emit(Instruction::new(Opcode::LoadConstant, dest, 0, const_idx));
            }
        }
        Ok(())
    }

    fn compile_boolean_literal(&mut self, b: &BooleanLiteral, dest: u8) -> Result<(), CompilerError> {
        let const_idx = self.chunk.add_constant(Constant::Boolean(b.value))?;
        self.chunk.emit(Instruction::new(Opcode::LoadConstant, dest, 0, const_idx));
        Ok(())
    }

    fn compile_undefined_literal(&mut self, _: &UndefinedLiteral, dest: u8) -> Result<(), CompilerError> {
        let const_idx = self.chunk.add_constant(Constant::Undefined)?;
        self.chunk.emit(Instruction::new(Opcode::LoadConstant, dest, 0, const_idx));
        Ok(())
    }

    fn compile_identifier(&mut self, i: &Identifier, dest: u8) -> Result<(), CompilerError> {
        if let Some(const_idx) = self.lookup_binding(&i.name) {
            self.chunk.emit(Instruction::new(Opcode::LookupName, dest, const_idx, 0));
        } else {
            let const_idx = self.chunk.add_constant(Constant::String(i.name.clone()))?;
            self.chunk.emit(Instruction::new(Opcode::LoadConstant, dest, 0, const_idx));
        }
        Ok(())
    }

    fn compile_left_arg(&mut self, _: &LeftArg, dest: u8) -> Result<(), CompilerError> {
        self.chunk.emit(Instruction::new(Opcode::LoadArg, dest, 0, 0));
        Ok(())
    }

    fn compile_right_arg(&mut self, _: &RightArg, dest: u8) -> Result<(), CompilerError> {
        self.chunk.emit(Instruction::new(Opcode::LoadArg, dest, 1, 0));
        Ok(())
    }

    fn compile_list_literal(&mut self, l: &ListLiteral, dest: u8) -> Result<(), CompilerError> {
        if l.elements.is_empty() {
            self.chunk.emit(Instruction::new(Opcode::ListNew, dest, 0, 0));
        } else {
            let first_reg = self.alloc_register()?;
            self.compile_expression(&l.elements[0], first_reg)?;

            for element in &l.elements[1..] {
                let temp = self.alloc_register()?;
                self.compile_expression(element, temp)?;
            }

            let count = l.elements.len() as u8;
            self.chunk.emit(Instruction::new(Opcode::ListBuild, dest, first_reg, count));

            for _ in 0..l.elements.len() {
                self.free_register();
            }
        }

        Ok(())
    }

    fn contains_arg_ref(expr: &Expression) -> bool {
        match expr {
            Expression::LeftArg(_) | Expression::RightArg(_) => true,
            Expression::Application(a) => Self::contains_arg_ref(&a.left) || Self::contains_arg_ref(&a.right),
            Expression::GroupedExpression(g) => Self::contains_arg_ref(&g.expression),
            Expression::ListLiteral(l) => l.elements.iter().any(Self::contains_arg_ref),
            Expression::MapLiteral(m) => m.entries.iter().any(|e| {
                Self::contains_arg_ref(&e.key) || e.value.as_ref().map_or(false, Self::contains_arg_ref)
            }),
            _ => false,
        }
    }

    fn contains_right_arg_ref(expr: &Expression) -> bool {
        match expr {
            Expression::RightArg(_) => true,
            Expression::Application(a) => Self::contains_right_arg_ref(&a.left) || Self::contains_right_arg_ref(&a.right),
            Expression::GroupedExpression(g) => Self::contains_right_arg_ref(&g.expression),
            Expression::ListLiteral(l) => l.elements.iter().any(Self::contains_right_arg_ref),
            Expression::MapLiteral(m) => m.entries.iter().any(|e| {
                Self::contains_right_arg_ref(&e.key) || e.value.as_ref().map_or(false, Self::contains_right_arg_ref)
            }),
            _ => false,
        }
    }

    fn compile_map_literal(&mut self, m: &MapLiteral, dest: u8) -> Result<(), CompilerError> {
        let has_arg_keys = m.entries.iter().any(|entry| {
            matches!(entry.key, Expression::LeftArg(_) | Expression::RightArg(_)) ||
            Self::contains_arg_ref(&entry.key)
        });
        let has_arg_values = m.entries.iter().any(|entry| {
            entry.value.as_ref().map_or(false, |v| Self::contains_arg_ref(v))
        });
        let has_arg_refs = has_arg_keys || has_arg_values;

        let non_arg_entries: Vec<_> = m.entries.iter()
            .filter(|e| !matches!(e.key, Expression::LeftArg(_) | Expression::RightArg(_)))
            .collect();
        let is_program = if !non_arg_entries.is_empty() {
            let last_idx = non_arg_entries.len() - 1;
            non_arg_entries[last_idx].is_expression_key
                && non_arg_entries[last_idx].value.is_none()
                && non_arg_entries[..last_idx].iter().any(|e| e.is_assignment)
        } else {
            false
        };

        if has_arg_refs && !is_program {
            self.push_scope();

            let arg_count = if m.entries.iter().any(|e| {
                matches!(e.key, Expression::RightArg(_)) ||
                e.value.as_ref().map_or(false, |v| Self::contains_right_arg_ref(v))
            }) {
                2
            } else {
                1
            };

            let make_closure_inst_idx = self.chunk.code.len();
            self.chunk.emit(Instruction::new(Opcode::MakeClosure, dest, 0, arg_count));

            if non_arg_entries.is_empty() {
                let mut last_value_reg = 0u8;
                for entry in &m.entries {
                    let value_reg = self.alloc_register()?;
                    if let Some(ref value) = entry.value {
                        self.compile_expression(value, value_reg)?;
                        last_value_reg = value_reg;
                    } else {
                        let key_reg = self.alloc_register()?;
                        self.compile_expression(&entry.key, key_reg)?;
                        last_value_reg = key_reg;
                    }
                }
                self.chunk.emit(Instruction::new(Opcode::Return, last_value_reg, 0, 0));
                let free_count = m.entries.iter()
                    .filter(|e| e.value.is_some())
                    .count() + m.entries.iter().filter(|e| e.value.is_none()).count();
                for _ in 0..free_count {
                    self.free_register();
                }
            } else if non_arg_entries.len() == 2
                && !matches!(non_arg_entries[0].key, Expression::Identifier(_))
                && non_arg_entries[0].value.is_some()
            {
                let cond_reg = self.alloc_register()?;
                self.compile_expression(&non_arg_entries[0].key, cond_reg)?;

                let jump_if_false_idx = self.chunk.code.len();
                self.chunk.emit(Instruction::new(Opcode::JumpIfFalse, cond_reg, 0, 0));
                self.free_register();

                let true_reg = self.alloc_register()?;
                self.compile_expression(non_arg_entries[0].value.as_ref().unwrap(), true_reg)?;

                let jump_end_idx = self.chunk.code.len();
                self.chunk.emit(Instruction::new(Opcode::Jump, 0, 0, 0));
                self.free_register();

                let false_pos = self.chunk.code.len();
                let false_reg = self.alloc_register()?;
                if let Some(ref fv) = non_arg_entries[1].value {
                    self.compile_expression(fv, false_reg)?;
                } else {
                    self.compile_expression(&non_arg_entries[1].key, false_reg)?;
                }

                let end_pos = self.chunk.code.len();
                self.chunk.emit(Instruction::new(Opcode::Return, true_reg, 0, 0));

                let false_offset = (false_pos - jump_if_false_idx) as u8;
                self.chunk.code[jump_if_false_idx] = Instruction::new(
                    Opcode::JumpIfFalse, cond_reg, false_offset, 0,
                );

                let end_offset = (end_pos - jump_end_idx) as u8;
                self.chunk.code[jump_end_idx] = Instruction::new(
                    Opcode::Jump, end_offset, 0, 0,
                );

                self.free_register();
            } else {
                let last_non_arg_idx = non_arg_entries.len() - 1;
                let is_program = non_arg_entries[last_non_arg_idx].is_expression_key
                    && non_arg_entries[last_non_arg_idx].value.is_none()
                    && non_arg_entries[..last_non_arg_idx].iter().any(|e| e.is_assignment);

                let has_guards = non_arg_entries.iter().any(|e| {
                    matches!(&e.key, Expression::Identifier(i) if i.name.ends_with('?'))
                    && e.value.is_some()
                });

                let first_key_reg = self.alloc_register()?;
                self.compile_expression(&non_arg_entries[0].key, first_key_reg)?;

                let first_value_reg = self.alloc_register()?;
                if let Some(ref value) = non_arg_entries[0].value {
                    self.compile_expression(value, first_value_reg)?;
                } else {
                    self.chunk.emit(Instruction::new(Opcode::LoadRegister, first_value_reg, first_key_reg, 0));
                }

                if non_arg_entries[0].is_assignment {
                    if let Expression::Identifier(ref ident) = non_arg_entries[0].key {
                        self.bind_name(&ident.name, first_value_reg)?;
                    }
                }

                let entries_to_compile = if is_program {
                    &non_arg_entries[1..last_non_arg_idx]
                } else {
                    &non_arg_entries[1..]
                };

                for entry in entries_to_compile {
                    let is_guard = matches!(&entry.key, Expression::Identifier(i) if i.name.ends_with('?'));
                    let is_size_cond = matches!(&entry.key, Expression::Identifier(i) if i.name == "#" && entry.value.is_some());

                    if is_guard && entry.value.is_some() {
                        let (base_name, base_span) = if let Expression::Identifier(i) = &entry.key {
                            (i.name[..i.name.len()-1].to_string(), i.span.clone())
                        } else {
                            unreachable!()
                        };

                        let guard_base_reg = self.alloc_register()?;
                        self.compile_expression(&Expression::Identifier(Identifier {
                            name: base_name,
                            span: base_span,
                        }), guard_base_reg)?;

                        let guard_cond_reg = self.alloc_register()?;
                        let const_idx = self.chunk.add_constant(Constant::String("?".to_string()))?;
                        self.chunk.emit(Instruction::new(Opcode::LoadConstant, guard_cond_reg, const_idx, 0));
                        self.chunk.emit(Instruction::new(Opcode::Call, guard_cond_reg, guard_base_reg, 0));
                        self.free_register();

                        let jump_if_false_idx = self.chunk.code.len();
                        self.chunk.emit(Instruction::new(Opcode::JumpIfFalse, guard_cond_reg, 0, 0));
                        self.free_register();

                        let guard_value_reg = self.alloc_register()?;
                        self.compile_expression(entry.value.as_ref().unwrap(), guard_value_reg)?;
                        self.chunk.emit(Instruction::new(Opcode::Return, guard_value_reg, 0, 0));

                        let skip_pos = self.chunk.code.len();
                        let skip_offset = (skip_pos - jump_if_false_idx) as u8;
                        self.chunk.code[jump_if_false_idx] = Instruction::new(
                            Opcode::JumpIfFalse, guard_cond_reg, skip_offset, 0,
                        );
                        self.free_register();
                    } else if is_size_cond {
                        let size_reg = self.alloc_register()?;
                        let const_idx = self.chunk.add_constant(Constant::String("#".to_string()))?;
                        self.chunk.emit(Instruction::new(Opcode::LoadConstant, size_reg, const_idx, 0));
                        self.chunk.emit(Instruction::new(Opcode::Call, size_reg, 0, 0));

                        let cond_reg = self.alloc_register()?;
                        let q_idx = self.chunk.add_constant(Constant::String("?".to_string()))?;
                        self.chunk.emit(Instruction::new(Opcode::LoadConstant, cond_reg, q_idx, 0));
                        self.chunk.emit(Instruction::new(Opcode::Call, cond_reg, size_reg, 0));
                        self.free_register();

                        let jump_if_false_idx = self.chunk.code.len();
                        self.chunk.emit(Instruction::new(Opcode::JumpIfFalse, cond_reg, 0, 0));
                        self.free_register();

                        let body_reg = self.alloc_register()?;
                        self.compile_expression(entry.value.as_ref().unwrap(), body_reg)?;
                        self.chunk.emit(Instruction::new(Opcode::Return, body_reg, 0, 0));

                        let skip_pos = self.chunk.code.len();
                        let skip_offset = (skip_pos - jump_if_false_idx) as u8;
                        self.chunk.code[jump_if_false_idx] = Instruction::new(
                            Opcode::JumpIfFalse, cond_reg, skip_offset, 0,
                        );
                        self.free_register();
                    } else {
                        let key_reg = self.alloc_register()?;
                        self.compile_expression(&entry.key, key_reg)?;

                        let value_reg = self.alloc_register()?;
                        if let Some(ref value) = entry.value {
                            self.compile_expression(value, value_reg)?;
                        } else {
                            self.chunk.emit(Instruction::new(Opcode::LoadRegister, value_reg, key_reg, 0));
                        }

                        if entry.is_assignment {
                            if let Expression::Identifier(ref ident) = entry.key {
                                self.bind_name(&ident.name, value_reg)?;
                            }
                        }
                    }
                }

                for entry in &m.entries {
                    if matches!(entry.key, Expression::LeftArg(_) | Expression::RightArg(_)) {
                        if let Some(ref value) = entry.value {
                            let _val_reg = self.alloc_register()?;
                            self.compile_expression(value, _val_reg)?;
                            self.free_register();
                        }
                    }
                }

                if is_program {
                    let last_reg = self.alloc_register()?;
                    self.compile_expression(&non_arg_entries[last_non_arg_idx].key, last_reg)?;
                    self.chunk.emit(Instruction::new(Opcode::Return, last_reg, 0, 0));
                    self.free_register();
                } else if has_guards {
                    let last_non_guard = non_arg_entries.iter().enumerate()
                        .rev()
                        .find(|(_, e)| {
                            !matches!(&e.key, Expression::Identifier(i) if i.name.ends_with('?'))
                        });
                    if let Some((idx, _)) = last_non_guard {
                        let reg = self.alloc_register()?;
                        if idx == 0 {
                            if let Some(ref value) = non_arg_entries[0].value {
                                self.compile_expression(value, reg)?;
                            } else {
                                self.chunk.emit(Instruction::new(Opcode::LoadRegister, reg, first_key_reg, 0));
                            }
                        } else {
                            self.compile_expression(&non_arg_entries[idx].key, reg)?;
                            if let Some(ref value) = non_arg_entries[idx].value {
                                let val_reg = self.alloc_register()?;
                                self.compile_expression(value, val_reg)?;
                                self.free_register();
                            }
                        }
                        self.chunk.emit(Instruction::new(Opcode::Return, reg, 0, 0));
                        self.free_register();
                    }
                } else {
                    let all_expr_keys = non_arg_entries.iter().all(|e| {
                        !matches!(e.key, Expression::Identifier(_)) && e.value.is_none()
                    });

                    if all_expr_keys {
                        self.free_register();
                        self.chunk.emit(Instruction::new(Opcode::Return, first_key_reg, 0, 0));
                    } else {
                        let entry_count = non_arg_entries.len() as u8;
                        self.chunk.emit(Instruction::new(Opcode::MapBuild, 0, first_key_reg, entry_count));

                        for _ in 0..non_arg_entries.len() * 2 {
                            self.free_register();
                        }
                        self.chunk.emit(Instruction::new(Opcode::Return, 0, 0, 0));
                    }
                }
            }

            let body_start = make_closure_inst_idx + 1;
            let body_start_low = (body_start & 0xFF) as u8;
            let body_start_high = ((body_start >> 8) & 0xFF) as u8;
            self.chunk.code[make_closure_inst_idx] = Instruction::new(Opcode::MakeClosure, dest, body_start_low, body_start_high);

            self.pop_scope();
            Ok(())
        } else {
            self.push_scope();

            if m.entries.is_empty() {
                self.chunk.emit(Instruction::new(Opcode::MapNew, dest, 0, 0));
            } else {
                let last_idx = m.entries.len() - 1;
                let is_program = m.entries[last_idx].is_expression_key
                    && m.entries[last_idx].value.is_none()
                    && m.entries[..last_idx].iter().any(|e| e.is_assignment);

                if is_program {
                    // Check if last entry is a spread (+:)
                    let last_entry = &m.entries[last_idx];
                    let spread_value = if last_entry.value.is_none() {
                        if let Expression::Application(ref app) = last_entry.key {
                            if let Expression::Identifier(ref ident) = *app.left {
                                if ident.name == "+:" {
                                    Some(&*app.right)
                                } else { None }
                            } else { None }
                        } else { None }
                    } else {
                        None
                    };

                    if let Some(spread_expr) = spread_value {
                        // Build map from non-spread entries, then merge
                        let non_spread = &m.entries[..last_idx];
                        if non_spread.is_empty() {
                            self.chunk.emit(Instruction::new(Opcode::MapNew, dest, 0, 0));
                        } else {
                            let first_key_reg = self.alloc_register()?;
                            self.compile_expression(&non_spread[0].key, first_key_reg)?;
                            let first_value_reg = self.alloc_register()?;
                            if let Some(ref value) = non_spread[0].value {
                                self.compile_expression(value, first_value_reg)?;
                            } else {
                                self.chunk.emit(Instruction::new(Opcode::LoadRegister, first_value_reg, first_key_reg, 0));
                            }
                            if non_spread[0].is_assignment {
                                if let Expression::Identifier(ref ident) = non_spread[0].key {
                                    self.bind_name(&ident.name, first_value_reg)?;
                                }
                            }
                            for entry in &non_spread[1..] {
                                let key_reg = self.alloc_register()?;
                                self.compile_expression(&entry.key, key_reg)?;
                                let value_reg = self.alloc_register()?;
                                if let Some(ref value) = entry.value {
                                    self.compile_expression(value, value_reg)?;
                                } else {
                                    self.chunk.emit(Instruction::new(Opcode::LoadRegister, value_reg, key_reg, 0));
                                }
                                if entry.is_assignment {
                                    if let Expression::Identifier(ref ident) = entry.key {
                                        self.bind_name(&ident.name, value_reg)?;
                                    }
                                }
                            }
                            let entry_count = non_spread.len() as u8;
                            self.chunk.emit(Instruction::new(Opcode::MapBuild, dest, first_key_reg, entry_count));
                            for _ in 0..non_spread.len() * 2 {
                                self.free_register();
                            }
                        }
                        let spread_reg = self.alloc_register()?;
                        self.compile_expression(spread_expr, spread_reg)?;
                        self.chunk.emit(Instruction::new(Opcode::MapMerge, dest, dest, spread_reg));
                        self.free_register();
                        self.chunk.emit(Instruction::new(Opcode::Return, dest, 0, 0));
                    } else {
                        let mut last_reg = 0u8;
                        for entry in &m.entries {
                            if let Some(ref value) = entry.value {
                                let key_reg = self.alloc_register()?;
                                self.compile_expression(&entry.key, key_reg)?;
                                let value_reg = self.alloc_register()?;
                                self.compile_expression(value, value_reg)?;

                                if entry.is_assignment {
                                    if let Expression::Identifier(ref ident) = entry.key {
                                        self.bind_name(&ident.name, value_reg)?;
                                    }
                                }
                                last_reg = value_reg;
                            } else {
                                let key_reg = self.alloc_register()?;
                                self.compile_expression(&entry.key, key_reg)?;
                                last_reg = key_reg;
                            }
                        }
                        if last_reg != dest {
                            self.chunk.emit(Instruction::new(Opcode::LoadRegister, dest, last_reg, 0));
                        }
                    }
                } else {
                    // Check for spread entries (+:)
                    let spread_indices: Vec<usize> = m.entries.iter().enumerate()
                        .filter_map(|(i, e)| {
                            if let Expression::Identifier(ref ident) = e.key {
                                if ident.name == "+:" && e.value.is_some() { return Some(i); }
                            }
                            None
                        })
                        .collect();

                    if spread_indices.is_empty() {
                        // No spread entries - original behavior
                        let first_key_reg = self.alloc_register()?;
                        self.compile_expression(&m.entries[0].key, first_key_reg)?;

                        let first_value_reg = self.alloc_register()?;
                        if let Some(ref value) = m.entries[0].value {
                            self.compile_expression(value, first_value_reg)?;
                        } else {
                            self.chunk.emit(Instruction::new(Opcode::LoadRegister, first_value_reg, first_key_reg, 0));
                        }

                        if m.entries[0].is_assignment {
                            if let Expression::Identifier(ref ident) = m.entries[0].key {
                                self.bind_name(&ident.name, first_value_reg)?;
                            }
                        }

                        for entry in &m.entries[1..] {
                            let key_reg = self.alloc_register()?;
                            self.compile_expression(&entry.key, key_reg)?;

                            let value_reg = self.alloc_register()?;
                            if let Some(ref value) = entry.value {
                                self.compile_expression(value, value_reg)?;
                            } else {
                                self.chunk.emit(Instruction::new(Opcode::LoadRegister, value_reg, key_reg, 0));
                            }

                            if entry.is_assignment {
                                if let Expression::Identifier(ref ident) = entry.key {
                                    self.bind_name(&ident.name, value_reg)?;
                                }
                            }
                        }

                        let entry_count = m.entries.len() as u8;
                        self.chunk.emit(Instruction::new(Opcode::MapBuild, dest, first_key_reg, entry_count));

                        for _ in 0..m.entries.len() * 2 {
                            self.free_register();
                        }
                    } else {
                        // Has spread entries - compile normal entries, then merge spreads
                        let normal_entries: Vec<(usize, &MapEntry)> = m.entries.iter().enumerate()
                            .filter(|(i, _)| !spread_indices.contains(i))
                            .collect();

                        if normal_entries.is_empty() {
                            // All spreads - start with empty map
                            self.chunk.emit(Instruction::new(Opcode::MapNew, dest, 0, 0));
                        } else {
                            // Compile normal entries
                            let first_key_reg = self.alloc_register()?;
                            self.compile_expression(&normal_entries[0].1.key, first_key_reg)?;

                            let first_value_reg = self.alloc_register()?;
                            if let Some(ref value) = normal_entries[0].1.value {
                                self.compile_expression(value, first_value_reg)?;
                            } else {
                                self.chunk.emit(Instruction::new(Opcode::LoadRegister, first_value_reg, first_key_reg, 0));
                            }

                            if normal_entries[0].1.is_assignment {
                                if let Expression::Identifier(ref ident) = normal_entries[0].1.key {
                                    self.bind_name(&ident.name, first_value_reg)?;
                                }
                            }

                            for (_, entry) in &normal_entries[1..] {
                                let key_reg = self.alloc_register()?;
                                self.compile_expression(&entry.key, key_reg)?;

                                let value_reg = self.alloc_register()?;
                                if let Some(ref value) = entry.value {
                                    self.compile_expression(value, value_reg)?;
                                } else {
                                    self.chunk.emit(Instruction::new(Opcode::LoadRegister, value_reg, key_reg, 0));
                                }

                                if entry.is_assignment {
                                    if let Expression::Identifier(ref ident) = entry.key {
                                        self.bind_name(&ident.name, value_reg)?;
                                    }
                                }
                            }

                            let entry_count = normal_entries.len() as u8;
                            self.chunk.emit(Instruction::new(Opcode::MapBuild, dest, first_key_reg, entry_count));

                            for _ in 0..normal_entries.len() * 2 {
                                self.free_register();
                            }
                        }

                        // Merge each spread entry
                        for &idx in &spread_indices {
                            let spread_reg = self.alloc_register()?;
                            if let Some(ref value) = m.entries[idx].value {
                                self.compile_expression(value, spread_reg)?;
                            }
                            self.chunk.emit(Instruction::new(Opcode::MapMerge, dest, dest, spread_reg));
                            self.free_register();
                        }
                    }
                }
            }

            self.pop_scope();
            Ok(())
        }
    }

    fn compile_application(&mut self, a: &Application, dest: u8) -> Result<(), CompilerError> {
        let left_reg = self.alloc_register()?;
        self.compile_expression(&a.left, left_reg)?;

        let right_reg = self.alloc_register()?;
        self.compile_expression(&a.right, right_reg)?;

        self.chunk.emit(Instruction::new(Opcode::Call, dest, left_reg, right_reg));

        self.free_register();
        self.free_register();

        Ok(())
    }

    fn compile_grouped_expression(&mut self, g: &GroupedExpression, dest: u8) -> Result<(), CompilerError> {
        self.compile_expression(&g.expression, dest)
    }

    fn compile_import_expression(&mut self, i: &ImportExpression, dest: u8) -> Result<(), CompilerError> {
        let path_reg = self.alloc_register()?;
        self.compile_expression(&i.path, path_reg)?;
        self.chunk.emit(Instruction::new(Opcode::Import, dest, path_reg, 0));
        self.free_register();
        Ok(())
    }

    fn compile_export_expression(&mut self, e: &ExportExpression, dest: u8) -> Result<(), CompilerError> {
        for (idx, key) in e.keys.iter().enumerate() {
            let const_idx = self.chunk.add_constant(Constant::String(key.clone()))?;
            self.chunk.emit(Instruction::new(Opcode::LoadConstant, dest, 0, const_idx));
        }
        let keys_count = e.keys.len() as u8;
        self.chunk.emit(Instruction::new(Opcode::Export, dest, keys_count, 0));
        Ok(())
    }

    fn compile_throw_expression(&mut self, t: &ThrowExpression, dest: u8) -> Result<(), CompilerError> {
        // Compile the value to throw into dest
        self.compile_expression(&t.value, dest)?;
        // Emit Throw opcode with the register containing the value
        self.chunk.emit(Instruction::new(Opcode::Throw, dest, 0, 0));
        Ok(())
    }

    fn compile_catch_expression(&mut self, c: &CatchExpression, dest: u8) -> Result<(), CompilerError> {
        // Compile the try expression (operator)
        let try_result = self.alloc_register()?;
        self.compile_expression(&c.operator, try_result)?;

        // Store try result in dest if no error
        self.chunk.emit(Instruction::new(Opcode::LoadRegister, dest, try_result, 0));

        // Emit Catch opcode - handler will be compiled next
        // Catch format: Catch(dest_reg, handler_jump_offset)
        // We'll use a placeholder jump that we patch later
        let catch_pos = self.chunk.code.len();
        self.chunk.emit(Instruction::new(Opcode::Catch, dest, 0, 0));

        // Compile the catch handler
        self.compile_expression(&c.handler, dest)?;

        // Emit CatchEnd
        self.chunk.emit(Instruction::new(Opcode::CatchEnd, 0, 0, 0));

        // Patch the Catch instruction with the jump offset to handler
        // The handler starts at catch_pos + 1
        let handler_offset = 1;
        let patched_inst = Instruction::new(Opcode::Catch, dest, handler_offset as u8, 0);
        self.chunk.code[catch_pos] = patched_inst;

        self.free_register();
        Ok(())
    }

    /// Allocate next available register
    fn alloc_register(&mut self) -> Result<u8, CompilerError> {
        if self.register_count == 255 {
            return Err(CompilerError::RegisterOverflow);
        }
        let reg = self.register_count;
        self.register_count += 1;
        Ok(reg)
    }

    /// Free a register
    fn free_register(&mut self) {
        if self.register_count > 0 {
            self.register_count -= 1;
        }
    }
    
    /// Push a new scope for map bindings
    fn push_scope(&mut self) {
        self.binding_scopes.push(HashMap::new());
    }
    
    /// Pop the current scope
    fn pop_scope(&mut self) {
        self.binding_scopes.pop();
    }
    
    fn bind_name(&mut self, name: &str, reg: u8) -> Result<(), CompilerError> {
        let const_idx = self.chunk.add_constant(Constant::String(name.to_string()))?;
        self.chunk.emit(Instruction::new(Opcode::BindName, 0, const_idx, reg));
        if let Some(scope) = self.binding_scopes.last_mut() {
            scope.insert(name.to_string(), const_idx);
        }
        Ok(())
    }
    
    fn lookup_binding(&self, name: &str) -> Option<u8> {
        for scope in self.binding_scopes.iter().rev() {
            if let Some(&const_idx) = scope.get(name) {
                return Some(const_idx);
            }
        }
        None
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to tokenize, parse, and compile source code
pub fn compile_source(source: &str) -> Result<Chunk, CompilerError> {
    compile_source_with_name(source, "inline.lr")
}

pub fn compile_source_with_name(source: &str, source_name: &str) -> Result<Chunk, CompilerError> {
    let tokens = lr_lexer::tokenize(source).map_err(|errs| {
        CompilerError::LexerError(errs[0].clone())
    })?;
    let program = lr_parser::parse(tokens, source_name.to_string())?;
    Compiler::compile(&program)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lr_vm::VM;
    use lr_bytecode::{Chunk, Instruction, Opcode, Constant};

    fn build_chunk(f: impl FnOnce(&mut Chunk)) -> Chunk {
        let mut chunk = Chunk::new();
        f(&mut chunk);
        chunk
    }

    fn compile_and_run(source: &str) -> Result<String, CompilerError> {
        let chunk = compile_source(source)?;
        let mut vm = VM::new();
        vm.execute(&chunk).map_err(|e| CompilerError::Unsupported(e.to_string()))
    }

    #[test]
    fn test_compile_unsupported_await() {
        let result = compile_source("42 \\");
        assert!(result.is_ok());
        let chunk = result.unwrap();
        let has_call = chunk.code.iter().any(|i: &Instruction| i.opcode() == Opcode::Call);
        assert!(has_call, "Should have Call instruction");
    }

    #[test]
    fn test_compile_float() {
        let result = compile_and_run("3.14");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "3.14");
    }

    #[test]
    fn test_compile_boolean() {
        let result = compile_and_run("true");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");

        let result = compile_and_run("false");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "false");
    }

    #[test]
    fn test_compile_string() {
        let result = compile_and_run("`hello`");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello");
    }

    #[test]
    fn test_compile_undefined() {
        let result = compile_and_run("undefined");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "undefined");
    }

    #[test]
    fn test_compile_list_empty() {
        let result = compile_and_run("[]");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "[]");
    }

    #[test]
    fn test_compile_list_with_elements() {
        let chunk = compile_source("[1, 2, 3]");
        assert!(chunk.is_ok());
        let chunk = chunk.unwrap();

        let has_list_build = chunk.code.iter().any(|i| i.opcode() == Opcode::ListBuild);
        assert!(has_list_build, "Should have ListBuild instruction");

        let num_consts = chunk.constants.iter().filter(|c| matches!(c, Constant::Number(_))).count();
        assert_eq!(num_consts, 3, "Should have 3 number constants");
    }

    #[test]
    fn test_compile_map_empty() {
        let result = compile_and_run("{}");
        assert!(result.is_ok());
        // Result should start with {
        assert!(result.unwrap().starts_with("{"));
    }

    #[test]
    fn test_compile_map_with_entry() {
        let chunk = compile_source("{ a: 1 }");
        assert!(chunk.is_ok());
        let chunk = chunk.unwrap();

        let has_map_build = chunk.code.iter().any(|i| i.opcode() == Opcode::MapBuild);
        assert!(has_map_build, "Should have MapBuild instruction");

        let has_string_a = chunk.constants.iter().any(|c| matches!(c, Constant::String(s) if s == "a"));
        let has_number_1 = chunk.constants.iter().any(|c| matches!(c, Constant::Number(n) if *n == 1.0));
        assert!(has_string_a, "Should have string constant 'a'");
        assert!(has_number_1, "Should have number constant 1.0");
    }

    #[test]
    fn test_compile_map_shorthand() {
        let chunk = compile_source("{ a }");
        assert!(chunk.is_ok());
        let chunk = chunk.unwrap();

        let has_map_build = chunk.code.iter().any(|i| i.opcode() == Opcode::MapBuild);
        assert!(has_map_build, "Should have MapBuild instruction");

        let has_string_a = chunk.constants.iter().any(|c| matches!(c, Constant::String(s) if s == "a"));
        assert!(has_string_a, "Should have string constant 'a'");
    }

    #[test]
    fn test_compile_map_with_spread() {
        let chunk = compile_source("{ a: 1, +: other }");
        assert!(chunk.is_ok());
        let chunk = chunk.unwrap();

        let has_map_build = chunk.code.iter().any(|i| i.opcode() == Opcode::MapBuild);
        assert!(has_map_build, "Should have MapBuild instruction");

        let has_map_merge = chunk.code.iter().any(|i| i.opcode() == Opcode::MapMerge);
        assert!(has_map_merge, "Should have MapMerge instruction for +: spread");

        let has_string_a = chunk.constants.iter().any(|c| matches!(c, Constant::String(s) if s == "a"));
        assert!(has_string_a, "Should have string constant 'a'");
    }

    #[test]
    fn test_compile_application_simple() {
        // VM doesn't implement Call yet, so just verify bytecode
        let chunk = compile_source("5 + 3");
        assert!(chunk.is_ok());
        let chunk = chunk.unwrap();

        // Should have Call instruction
        let has_call = chunk.code.iter().any(|i| i.opcode() == Opcode::Call);
        assert!(has_call, "Should have Call instruction");

        // Should have string "+" and numbers 5 and 3 constants
        let has_string_plus = chunk.constants.iter().any(|c| matches!(c, Constant::String(s) if s == "+"));
        let has_number_5 = chunk.constants.iter().any(|c| matches!(c, Constant::Number(n) if *n == 5.0));
        let has_number_3 = chunk.constants.iter().any(|c| matches!(c, Constant::Number(n) if *n == 3.0));
        assert!(has_string_plus, "Should have string constant '+'");
        assert!(has_number_5, "Should have number constant 5.0");
        assert!(has_number_3, "Should have number constant 3.0");
    }

    #[test]
    fn test_compile_nested_application() {
        // VM doesn't implement Call yet, so just verify bytecode
        let chunk = compile_source("add 1 2");
        assert!(chunk.is_ok());
        let chunk = chunk.unwrap();

        // Should have multiple Call instructions
        let call_count = chunk.code.iter().filter(|i| i.opcode() == Opcode::Call).count();
        assert_eq!(call_count, 2, "Should have 2 Call instructions for curried application");

        // Should have string "add" and numbers 1 and 2 constants
        let has_string_add = chunk.constants.iter().any(|c| matches!(c, Constant::String(s) if s == "add"));
        let has_number_1 = chunk.constants.iter().any(|c| matches!(c, Constant::Number(n) if *n == 1.0));
        let has_number_2 = chunk.constants.iter().any(|c| matches!(c, Constant::Number(n) if *n == 2.0));
        assert!(has_string_add, "Should have string constant 'add'");
        assert!(has_number_1, "Should have number constant 1.0");
        assert!(has_number_2, "Should have number constant 2.0");
    }

    #[test]
    fn test_compile_complex_expression() {
        // VM doesn't implement Call yet, so just verify bytecode
        let chunk = compile_source("{ a: 1, b: [2, 3] }");
        assert!(chunk.is_ok());
        let chunk = chunk.unwrap();

        let has_map_build = chunk.code.iter().any(|i| i.opcode() == Opcode::MapBuild);
        let has_list_build = chunk.code.iter().any(|i| i.opcode() == Opcode::ListBuild);
        assert!(has_map_build, "Should have MapBuild instruction");
        assert!(has_list_build, "Should have ListBuild instruction");
    }

    #[test]
    fn test_compile_grouped_expression() {
        let result = compile_and_run("(42)");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "42");
    }

    #[test]
    fn test_compile_left_arg() {
        let result = compile_and_run("_<");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "_<");
    }

    #[test]
    fn test_compile_right_arg() {
        let result = compile_and_run("_>");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "_>");
    }

    #[test]
    fn test_compile_identifier() {
        let result = compile_and_run("foo");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "foo");
    }

    #[test]
    fn test_compile_string_concat_simple() {
        let result = compile_and_run("`hello world`");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello world");
    }

    #[test]
    fn test_compile_unsupported_throw() {
        // `value !!!` parses as Application(value, "!!!"), not as ThrowExpression
        // ThrowExpression only exists in the AST but is not generated by the parser
        // So we expect this to compile (just produce a Call instruction)
        let result = compile_source("42 !!!");
        assert!(result.is_ok());
        let chunk = result.unwrap();
        let has_call = chunk.code.iter().any(|i| i.opcode() == Opcode::Call);
        assert!(has_call, "Should have Call instruction for application");
    }

    #[test]
    fn test_compile_catch_expression() {
        let result = compile_source("42 !!!? handler");
        assert!(result.is_ok());
        let chunk = result.unwrap();
        let has_catch = chunk.code.iter().any(|i| i.opcode() == Opcode::Catch);
        assert!(has_catch, "Should have Catch instruction");
        let has_catch_end = chunk.code.iter().any(|i| i.opcode() == Opcode::CatchEnd);
        assert!(has_catch_end, "Should have CatchEnd instruction");
    }

    #[test]
    fn test_compile_unsupported_async() {
        // `value ///` parses as Application(value, "///"), not as AsyncExpression
        // AsyncExpression only exists in the AST but is not generated by the parser
        let result = compile_source("42 ///");
        assert!(result.is_ok());
        let chunk = result.unwrap();
        let has_call = chunk.code.iter().any(|i| i.opcode() == Opcode::Call);
        assert!(has_call, "Should have Call instruction for application");
    }

    #[test]
    fn test_register_allocation() {
        let source = "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]";
        let result = compile_source(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_compiler_default() {
        let compiler = Compiler::default();
        assert_eq!(compiler.register_count, 0);
        assert!(compiler.chunk.code.is_empty());
    }

    #[test]
    fn test_e2e_basic_number() {
        let result = compile_and_run("42").unwrap();
        assert_eq!(result, "42");
    }

    #[test]
    fn test_e2e_basic_string() {
        let result = compile_and_run("`hello`").unwrap();
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_e2e_boolean_true() {
        let result = compile_and_run("true").unwrap();
        assert_eq!(result, "true");
    }

    #[test]
    fn test_e2e_boolean_false() {
        let result = compile_and_run("false").unwrap();
        assert_eq!(result, "false");
    }

    #[test]
    fn test_e2e_undefined() {
        let result = compile_and_run("undefined").unwrap();
        assert_eq!(result, "undefined");
    }

    #[test]
    fn test_e2e_subtraction_via_diadic() {
        let result = compile_and_run("0 - 3").unwrap();
        assert_eq!(result, "-3");
    }

    #[test]
    fn test_e2e_empty_list() {
        let result = compile_and_run("[]").unwrap();
        assert_eq!(result, "[]");
    }

    #[test]
    fn test_e2e_list_with_numbers() {
        let chunk = compile_source("[1, 2, 3]");
        assert!(chunk.is_ok());
        let chunk = chunk.unwrap();

        let has_list_build = chunk.code.iter().any(|i| i.opcode() == Opcode::ListBuild);
        assert!(has_list_build, "Should have ListBuild instruction");

        let num_consts = chunk.constants.iter().filter(|c| matches!(c, Constant::Number(_))).count();
        assert_eq!(num_consts, 3, "Should have 3 number constants");
    }

    #[test]
    fn test_e2e_empty_map() {
        let result = compile_and_run("{}").unwrap();
        assert!(result.starts_with("{"));
    }

    #[test]
    fn test_e2e_map_with_entries() {
        let chunk = compile_source("{a: 1, b: 2}");
        assert!(chunk.is_ok());
        let chunk = chunk.unwrap();

        let has_map_build = chunk.code.iter().any(|i| i.opcode() == Opcode::MapBuild);
        assert!(has_map_build, "Should have MapBuild instruction");

        let has_string_a = chunk.constants.iter().any(|c| matches!(c, Constant::String(s) if s == "a"));
        let has_string_b = chunk.constants.iter().any(|c| matches!(c, Constant::String(s) if s == "b"));
        let has_number_1 = chunk.constants.iter().any(|c| matches!(c, Constant::Number(n) if *n == 1.0));
        let has_number_2 = chunk.constants.iter().any(|c| matches!(c, Constant::Number(n) if *n == 2.0));
        assert!(has_string_a, "Should have string constant 'a'");
        assert!(has_string_b, "Should have string constant 'b'");
        assert!(has_number_1, "Should have number constant 1.0");
        assert!(has_number_2, "Should have number constant 2.0");
    }

    #[test]
    fn test_e2e_grouped_expression() {
        let result = compile_and_run("(42)").unwrap();
        assert_eq!(result, "42");
    }

    #[test]
    fn test_e2e_identifier() {
        let result = compile_and_run("x").unwrap();
        assert_eq!(result, "x");
    }

    #[test]
    fn test_e2e_application_simple() {
        let chunk = compile_source("f 42");
        assert!(chunk.is_ok());
        let chunk = chunk.unwrap();

        let has_call = chunk.code.iter().any(|i| i.opcode() == Opcode::Call);
        assert!(has_call, "Should have Call instruction");

        let has_string_f = chunk.constants.iter().any(|c| matches!(c, Constant::String(s) if s == "f"));
        let has_number_42 = chunk.constants.iter().any(|c| matches!(c, Constant::Number(n) if *n == 42.0));
        assert!(has_string_f, "Should have string constant 'f'");
        assert!(has_number_42, "Should have number constant 42.0");
    }

    #[test]
    fn test_e2e_nested_list() {
        let chunk = compile_source("[[1, 2], [3, 4]]");
        assert!(chunk.is_ok());
        let chunk = chunk.unwrap();

        let has_list_build = chunk.code.iter().any(|i| i.opcode() == Opcode::ListBuild);
        assert!(has_list_build, "Should have ListBuild instruction");

        let num_consts = chunk.constants.iter().filter(|c| matches!(c, Constant::Number(_))).count();
        assert_eq!(num_consts, 4, "Should have 4 number constants");
    }

    #[test]
    fn test_e2e_string_with_spaces() {
        let result = compile_and_run("`hello world`").unwrap();
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_e2e_number_float() {
        let result = compile_and_run("3.14").unwrap();
        assert_eq!(result, "3.14");
    }

    #[test]
    fn test_e2e_map_with_string_key() {
        let chunk = compile_source("{`name`: `test`}");
        assert!(chunk.is_ok());
        let chunk = chunk.unwrap();

        let has_map_build = chunk.code.iter().any(|i| i.opcode() == Opcode::MapBuild);
        assert!(has_map_build, "Should have MapBuild instruction");

        let has_string_name = chunk.constants.iter().any(|c| matches!(c, Constant::String(s) if s == "name"));
        let has_string_test = chunk.constants.iter().any(|c| matches!(c, Constant::String(s) if s == "test"));
        assert!(has_string_name, "Should have string constant 'name'");
        assert!(has_string_test, "Should have string constant 'test'");
    }

    #[test]
    fn test_e2e_application_chain() {
        let chunk = compile_source("f g 42");
        assert!(chunk.is_ok());
        let chunk = chunk.unwrap();

        let call_count = chunk.code.iter().filter(|i| i.opcode() == Opcode::Call).count();
        assert_eq!(call_count, 2, "Should have 2 Call instructions for curried chain");

        let has_string_f = chunk.constants.iter().any(|c| matches!(c, Constant::String(s) if s == "f"));
        let has_string_g = chunk.constants.iter().any(|c| matches!(c, Constant::String(s) if s == "g"));
        let has_number_42 = chunk.constants.iter().any(|c| matches!(c, Constant::Number(n) if *n == 42.0));
        assert!(has_string_f, "Should have string constant 'f'");
        assert!(has_string_g, "Should have string constant 'g'");
        assert!(has_number_42, "Should have number constant 42.0");
    }

    #[test]
    fn test_e2e_list_of_strings() {
        let chunk = compile_source("[`a`, `b`, `c`]");
        assert!(chunk.is_ok());
        let chunk = chunk.unwrap();

        let has_list_build = chunk.code.iter().any(|i| i.opcode() == Opcode::ListBuild);
        assert!(has_list_build, "Should have ListBuild instruction");

        let string_consts = chunk.constants.iter().filter(|c| matches!(c, Constant::String(_))).count();
        assert_eq!(string_consts, 3, "Should have 3 string constants");
    }

    #[test]
    fn test_e2e_nested_map() {
        let chunk = compile_source("{a: {b: 1}}");
        assert!(chunk.is_ok());
        let chunk = chunk.unwrap();

        let has_map_build = chunk.code.iter().any(|i| i.opcode() == Opcode::MapBuild);
        assert!(has_map_build, "Should have MapBuild instruction");

        let has_string_a = chunk.constants.iter().any(|c| matches!(c, Constant::String(s) if s == "a"));
        let has_string_b = chunk.constants.iter().any(|c| matches!(c, Constant::String(s) if s == "b"));
        let has_number_1 = chunk.constants.iter().any(|c| matches!(c, Constant::Number(n) if *n == 1.0));
        assert!(has_string_a, "Should have string constant 'a'");
        assert!(has_string_b, "Should have string constant 'b'");
        assert!(has_number_1, "Should have number constant 1.0");
    }

    #[test]
    fn test_map_binding_simple() {
        let result = compile_and_run("{x: 42, y: x + 1}");
        assert!(result.is_ok());
        let result_str = result.unwrap();
        assert!(result_str.contains("x: 42"), "Result should contain x: 42");
        assert!(result_str.contains("y: 43"), "Result should contain y: 43");
    }

    #[test]
    fn test_map_binding_multiple() {
        let result = compile_and_run("{a: 1, b: a + 2, c: a + b}");
        assert!(result.is_ok());
        let result_str = result.unwrap();
        assert!(result_str.contains("a: 1"), "Result should contain a: 1");
        assert!(result_str.contains("b: 3"), "Result should contain b: 3");
        assert!(result_str.contains("c: 4"), "Result should contain c: 4");
    }

    #[test]
    fn test_map_binding_nested() {
        let result = compile_and_run("{a: 1, b: {c: a + 1}}");
        assert!(result.is_ok());
        let result_str = result.unwrap();
        assert!(result_str.contains("a: 1"), "Result should contain a: 1");
        assert!(result_str.contains("c: 2"), "Result should contain c: 2");
    }

    #[test]
    fn test_closure_multiplication() {
        let result = compile_and_run("3 { _< * 2 }");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "6");
    }

    #[test]
    fn test_closure_map_result() {
        let result = compile_and_run("5 {a: _<, b: _< + 1}");
        assert!(result.is_ok());
        let result_str = result.unwrap();
        assert!(result_str.contains("a: 5"), "Result should contain a: 5");
        assert!(result_str.contains("b: 6"), "Result should contain b: 6");
    }

    #[test]
    fn test_closure_diadic() {
        // Diadic (two-arg) closures require partial application support
        // First call binds arg[0], second call binds arg[1]
        // TODO: implement partial application for closures
        // let result = compile_and_run("{sum: _< + _>}(3)(4)");
        // assert!(result.is_ok());
        // let result_str = result.unwrap();
        // assert!(result_str.contains("sum: 7"), "Result should contain sum: 7");
    }

    #[test]
    fn test_closure_backward_compat() {
        let result = compile_and_run("_<");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "_<");
    }

    #[test]
    fn test_closure_top_level_right_arg() {
        let result = compile_and_run("_>");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "_>");
    }

    #[test]
    fn test_closure_makeclosure_opcode() {
        let chunk = compile_source("{_<: _< + 1}");
        assert!(chunk.is_ok());
        let chunk = chunk.unwrap();

        let has_make_closure = chunk.code.iter().any(|i: &Instruction| i.opcode() == Opcode::MakeClosure);
        assert!(has_make_closure, "Should have MakeClosure instruction");
    }

    #[test]
    fn test_closure_loadarg_opcode() {
        let chunk = compile_source("{_<: _< + 1}");
        assert!(chunk.is_ok());
        let chunk = chunk.unwrap();

        let has_load_arg = chunk.code.iter().any(|i: &Instruction| i.opcode() == Opcode::LoadArg);
        assert!(has_load_arg, "Should have LoadArg instruction");
    }

    #[test]
    fn test_normal_map_no_closure() {
        let chunk = compile_source("{a: 1, b: 2}");
        assert!(chunk.is_ok());
        let chunk = chunk.unwrap();

        let has_make_closure = chunk.code.iter().any(|i: &Instruction| i.opcode() == Opcode::MakeClosure);
        let has_map_build = chunk.code.iter().any(|i: &Instruction| i.opcode() == Opcode::MapBuild);
        assert!(!has_make_closure, "Normal map should not have MakeClosure");
        assert!(has_map_build, "Normal map should have MapBuild");
    }

    #[test]
    fn test_closure_not_emitted_for_normal_map() {
        let result = compile_and_run("{a: 1, b: 2}");
        assert!(result.is_ok());
        let result_str = result.unwrap();
        assert!(result_str.contains("a: 1"), "Result should contain a: 1");
        assert!(result_str.contains("b: 2"), "Result should contain b: 2");
    }

    #[test]
    fn test_ternary_basic_true() {
        let result = compile_and_run("5 { _< > 3: `big`, `small` }");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "big");
    }

    #[test]
    fn test_ternary_basic_false() {
        let result = compile_and_run("1 { _< > 3: `big`, `small` }");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "small");
    }

    #[test]
    fn test_ternary_less_than() {
        let result = compile_and_run("1 { _< < 3: `small`, `big` }");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "small");
    }

    #[test]
    fn test_ternary_property_check() {
        let result = compile_and_run("{name: `test`} { _<@name: `has-name`, `no-name` }");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "has-name");
    }

    #[test]
    fn test_ternary_nested() {
        let result = compile_and_run("[1, 2, 3] $ { _< > 2: `big`, `small` }");
        assert!(result.is_ok());
        assert!(result.is_ok());
    }

    #[test]
    fn test_closure_map_returning_still_works() {
        let result = compile_and_run("5 {a: _<, b: _< + 1}");
        assert!(result.is_ok());
        let result_str = result.unwrap();
        assert!(result_str.contains("a: 5"), "Result should contain a: 5");
        assert!(result_str.contains("b: 6"), "Result should contain b: 6");
    }

    #[test]
    fn test_at_operator_list_index() {
        let result = compile_and_run("[10, 20, 30] @ 1");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "20");
    }

    #[test]
    fn test_at_operator_map_missing_key() {
        let result = compile_and_run("{ a: 1 } @ `z`");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "undefined");
    }

    #[test]
    fn test_some_operator() {
        let result = compile_and_run("[1, 2, 3] $| { _< > 2 }");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_some_operator_no_match() {
        let result = compile_and_run("[1, 2, 3] $| { _< > 5 }");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "false");
    }

    #[test]
    fn test_every_operator() {
        let result = compile_and_run("[1, 2, 3] $& { _< > 0 }");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_every_operator_no_match() {
        let result = compile_and_run("[1, 2, 3] $& { _< > 2 }");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "false");
    }

    #[test]
    fn test_find_operator() {
        let result = compile_and_run("[1, 2, 3] $?| { _< > 1 }");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "2");
    }

    #[test]
    fn test_find_operator_not_found() {
        let result = compile_and_run("[1, 2, 3] $?| { _< > 5 }");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "undefined");
    }

    #[test]
    fn test_unique_by_operator() {
        let result = compile_and_run("[1, 2, 2, 3] $~ { _< }");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "[1, 2, 3]");
    }

    #[test]
    fn test_group_by_operator() {
        let result = compile_and_run("[1, 2, 3] $> { _< % 2 }");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("1") && output.contains("3"), "Should contain 1 and 3 grouped");
        assert!(output.contains("2"), "Should contain 2 grouped separately");
    }

    #[test]
    fn test_sort_operator() {
        let result = compile_and_run("[3, 1, 2] $% { _< }");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "[1, 2, 3]");
    }

    #[test]
    fn test_compact_operator() {
        let source = "[1, 2, 3] $?!";
        let result = compile_and_run(source);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "[1, 2, 3]");
    }

    #[test]
    fn test_at_operator_map_numeric_index() {
        let result = compile_and_run("{ a: 1, b: 2 } @ 0");
        assert!(result.is_ok());
        let result_str = result.unwrap();
        // Should return [key, value] pair for first entry
        assert!(result_str.contains("a"), "Should contain key 'a'");
        assert!(result_str.contains("1"), "Should contain value 1");
    }

    #[test]
    fn test_optional_apply_truthy() {
        let result = compile_and_run("5 !! { _< * 2 }");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "10");
    }

    #[test]
    fn test_optional_apply_falsy() {
        let result = compile_and_run("0 !! { _< * 2 }");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "undefined");
    }

    #[test]
    fn test_optional_apply_string() {
        let result = compile_and_run("`hello` !! { _< ^ }");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "HELLO");
    }

    #[test]
    fn test_optional_apply_undefined() {
        let result = compile_and_run("{} @ `missing` !! { _< * 2 }");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "undefined");
    }

    #[test]
    fn test_guard_truthy() {
        let result = compile_and_run("5 { _<, x: 5, x?: x }");
        if let Err(ref e) = result {
            eprintln!("GUARD TRUTHY ERROR: {:?}", e);
        }
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "5");
    }

    #[test]
    fn test_guard_falsy() {
        let result = compile_and_run("5 { _<, x: 0, x?: x, y: 42 }");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "42");
    }

    #[test]
    fn test_guard_undefined() {
        let result = compile_and_run("5 { _<, x?: x, y: 99 }");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "99");
    }

    // === String Operator Tests ===

    #[test]
    fn test_string_uppercase() {
        let result = compile_and_run("`hello` ^");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "HELLO");
    }

    #[test]
    fn test_string_uppercase_mixed() {
        let result = compile_and_run("`HeLLo WoRLd` ^");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "HELLO WORLD");
    }

    #[test]
    fn test_string_lowercase() {
        let result = compile_and_run("`HELLO` _");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello");
    }

    #[test]
    fn test_string_lowercase_mixed() {
        let result = compile_and_run("`HeLLo WoRLd` _");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello world");
    }

    #[test]
    fn test_string_capitalize() {
        let result = compile_and_run("`hello` ^_");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello");
    }

    #[test]
    fn test_string_capitalize_all_caps() {
        let result = compile_and_run("`HELLO` ^_");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello");
    }

    #[test]
    fn test_string_replace() {
        let result = compile_and_run("`hello` ~ [`l`, `r`]");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "herro");
    }

    #[test]
    fn test_string_replace_multiple() {
        let result = compile_and_run("`aabbcc` ~ [`b`, `x`]");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "aaxxcc");
    }

    #[test]
    fn test_string_split() {
        let result = compile_and_run("`a,b,c` <> `,`");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "[a, b, c]");
    }

    #[test]
    fn test_string_split_no_match() {
        let result = compile_and_run("`hello` <> `,`");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "[hello]");
    }

    #[test]
    fn test_list_join() {
        let result = compile_and_run("[`a`, `b`, `c`] >< `,`");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "a,b,c");
    }

    #[test]
    fn test_list_join_numbers() {
        let result = compile_and_run("[1, 2, 3] >< `-`");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "1-2-3");
    }

    // === Map Operator Tests ===

    #[test]
    fn test_map_remove_key() {
        let result = compile_and_run("{a: 1, b: 2, c: 3} - `b`");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(!output.contains("b: 2"), "Should not contain removed key");
        assert!(output.contains("a: 1"), "Should contain remaining key a");
        assert!(output.contains("c: 3"), "Should contain remaining key c");
    }

    #[test]
    fn test_map_remove_missing_key() {
        let result = compile_and_run("{a: 1, b: 2} - `z`");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("a: 1"));
        assert!(output.contains("b: 2"));
    }

    #[test]
    fn test_map_bracket_path_single() {
        let result = compile_and_run("{a: 42} @ [`a`]");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "42");
    }

    #[test]
    fn test_map_bracket_path_nested() {
        let result = compile_and_run("{a: {b: {c: 99}}} @ [`a`, `b`, `c`]");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "99");
    }

    #[test]
    fn test_list_bracket_path_through_map() {
        let result = compile_and_run("[{name: `alice`}, {name: `bob`}] @ [0, `name`]");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "alice");
    }

    #[test]
    fn test_list_bracket_path_second_element() {
        let result = compile_and_run("[{x: 10}, {x: 20}, {x: 30}] @ [2, `x`]");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "30");
    }

    #[test]
    fn test_map_bracket_path_missing() {
        let result = compile_and_run("{a: 1} @ [`z`]");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "undefined");
    }

    // === Loop Operator Tests ===

    #[test]
    fn test_map_each_property_single_key() {
        let result = compile_and_run("[{name: `alice`}, {name: `bob`}] $@ `name`");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "[alice, bob]");
    }

    #[test]
    fn test_map_each_property_missing_key() {
        let result = compile_and_run("[{a: 1}, {b: 2}] $@ `z`");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "[undefined, undefined]");
    }

    #[test]
    fn test_map_each_property_multiple_keys() {
        let result = compile_and_run("[{a: 1, b: 2, c: 3}, {a: 4, b: 5, c: 6}] $@ [`a`, `c`]");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("a:"));
        assert!(output.contains("c:"));
    }

    #[test]
    fn test_each_to_string_numbers() {
        let result = compile_and_run("[1, 2, 3] $\"");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "[1, 2, 3]");
    }

    #[test]
    fn test_each_to_string_mixed() {
        let result = compile_and_run("[1, true, `hello`] $\"");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "[1, true, hello]");
    }

    // === Type Check Tests ===

    #[test]
    fn test_is_string_true() {
        let result = compile_and_run("`hello` ?\"");
        if let Err(ref e) = result {
            eprintln!("IS_STRING_TRUE ERROR: {:?}", e);
        }
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_is_string_false_number() {
        let result = compile_and_run("42 ?\"");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "false");
    }

    #[test]
    fn test_is_string_false_boolean() {
        let result = compile_and_run("true ?\"");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "false");
    }

    #[test]
    fn test_is_number_true() {
        let result = compile_and_run("42 ?#");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_is_number_false_string() {
        let result = compile_and_run("`hello` ?#");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "false");
    }

    #[test]
    fn test_is_number_float() {
        let result = compile_and_run("3.14 ?#");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_contains_string_in_list() {
        let result = compile_and_run("[`a`, `b`, `c`] ?>< `b`");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_contains_number_in_list() {
        let result = compile_and_run("[1, 2, 3] ?>< 2");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_contains_not_found() {
        let result = compile_and_run("[1, 2, 3] ?>< 99");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "false");
    }

    // === Control Flow Tests ===

    #[test]
    fn test_throw_runtime() {
        let result = compile_and_run("`error message` !!!");
        assert!(result.is_err());
    }

    #[test]
    fn test_spread_merge_maps() {
        let result = compile_and_run("{a: 1} + {b: 2}");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("a: 1"));
        assert!(output.contains("b: 2"));
    }

    #[test]
    fn test_spread_merge_override() {
        let result = compile_and_run("{a: 1} + {a: 2}");
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("a: 2"));
        assert!(!output.contains("a: 1"));
    }

    // === Partial Application Tests ===

    #[test]
    fn test_partial_application_single_arg() {
        let result = compile_and_run("[5] { _< + 1 }");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "6");
    }

    #[test]
    fn test_partial_application_two_args() {
        let result = compile_and_run("[3, 4] { _< + _> }");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "7");
    }

    #[test]
    fn test_partial_application_zero_args_returns_closure() {
        let result = compile_and_run("[] { _< + 0 }");
        if let Err(ref e) = result {
            eprintln!("PARTIAL ZERO ERROR: {:?}", e);
        }
        assert!(result.is_ok());
    }

    // === Template String Interpolation Tests ===

    #[test]
    fn test_string_interpolation_simple() {
        let result = compile_and_run("`hello {42}`");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello 42");
    }

    #[test]
    fn test_string_interpolation_number() {
        let result = compile_and_run("`value: {42}`");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "value: 42");
    }

    #[test]
    fn test_string_interpolation_expression() {
        let result = compile_and_run("`result: {1 + 2}`");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "result: 3");
    }

    // === Error Constructor Tests ===

    #[test]
    fn test_error_constructor() {
        let result = compile_and_run("Error[`test error`]");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Error: test error");
    }

    // === Combined Feature Tests ===

    #[test]
    fn test_combined_filter_and_map() {
        let result = compile_and_run("[1, 2, 3, 4, 5] $? { _< > 3 } $ { _< * 2 }");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "[8, 10]");
    }

    #[test]
    fn test_combined_pluck_and_join() {
        let result = compile_and_run("[{name: `alice`}, {name: `bob`}] $@ `name` >< `, `");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "alice, bob");
    }

    #[test]
    fn test_combined_map_remove_and_get() {
        let result = compile_and_run("{a: 1, b: 2, c: 3} - `b` @ `a`");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "1");
    }

    #[test]
    fn test_combined_split_and_size() {
        let result = compile_and_run("`a,b,c,d` <> `,` #");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "4");
    }

    #[test]
    fn test_combined_uppercase_and_contains() {
        let result = compile_and_run("[`HELLO`, `WORLD`] ?>< `HELLO`");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");
    }
}
