use lr_ast::{
    Application, BooleanLiteral, Expression, GroupedExpression, Identifier,
    LeftArg, ListLiteral, MapLiteral, NumberLiteral, Program,
    RightArg, StringLiteral, StringPart, UndefinedLiteral,
};
use lr_bytecode::{Chunk, Constant, Instruction, Opcode};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompilerError {
    #[error("Register overflow: more than 255 registers needed")]
    RegisterOverflow,
    #[error("Constant pool overflow: more than 255 constants")]
    ConstantPoolOverflow(#[from] lr_bytecode::BytecodeError),
    #[error("Unsupported expression: {0}")]
    Unsupported(String),
    #[error("Lexer error: {0}")]
    LexerError(String),
    #[error("Parse error: {0}")]
    ParseError(String),
}

impl From<lr_lexer::LexError> for CompilerError {
    fn from(err: lr_lexer::LexError) -> Self {
        CompilerError::LexerError(err.to_string())
    }
}

impl From<lr_parser::ParseError> for CompilerError {
    fn from(err: lr_parser::ParseError) -> Self {
        CompilerError::ParseError(err.to_string())
    }
}

pub struct Compiler {
    chunk: Chunk,
    register_count: u8,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            register_count: 0,
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
            Expression::ThrowExpression(_) => {
                Err(CompilerError::Unsupported("ThrowExpression".to_string()))
            }
            Expression::CatchExpression(_) => {
                Err(CompilerError::Unsupported("CatchExpression".to_string()))
            }
            Expression::AsyncExpression(_) => {
                Err(CompilerError::Unsupported("AsyncExpression".to_string()))
            }
            Expression::AwaitExpression(_) => {
                Err(CompilerError::Unsupported("AwaitExpression".to_string()))
            }
        }
    }

    fn compile_number_literal(&mut self, n: &NumberLiteral, dest: u8) -> Result<(), CompilerError> {
        let const_idx = self.chunk.add_constant(Constant::Number(n.value))?;
        self.chunk.emit(Instruction::new(Opcode::LoadConstant, dest, 0, const_idx));
        Ok(())
    }

    fn compile_string_literal(&mut self, s: &StringLiteral, dest: u8) -> Result<(), CompilerError> {
        // Check if there's any interpolation
        let has_interpolation = s.parts.iter().any(|p| matches!(p, StringPart::Interpolation { .. }));

        if !has_interpolation {
            // Simple case: concatenate all text parts
            let mut result = String::new();
            for part in &s.parts {
                if let StringPart::Text(text) = part {
                    result.push_str(text);
                }
            }
            let const_idx = self.chunk.add_constant(Constant::String(result))?;
            self.chunk.emit(Instruction::new(Opcode::LoadConstant, dest, 0, const_idx));
        } else {
            // Complex case: compile each part and use StringConcat
            let mut first = true;
            for part in &s.parts {
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
            // Handle empty string case
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
        let const_idx = self.chunk.add_constant(Constant::String(i.name.clone()))?;
        self.chunk.emit(Instruction::new(Opcode::LoadConstant, dest, 0, const_idx));
        Ok(())
    }

    fn compile_left_arg(&mut self, _: &LeftArg, dest: u8) -> Result<(), CompilerError> {
        let const_idx = self.chunk.add_constant(Constant::String("_<".to_string()))?;
        self.chunk.emit(Instruction::new(Opcode::LoadConstant, dest, 0, const_idx));
        Ok(())
    }

    fn compile_right_arg(&mut self, _: &RightArg, dest: u8) -> Result<(), CompilerError> {
        let const_idx = self.chunk.add_constant(Constant::String("_>".to_string()))?;
        self.chunk.emit(Instruction::new(Opcode::LoadConstant, dest, 0, const_idx));
        Ok(())
    }

    fn compile_list_literal(&mut self, l: &ListLiteral, dest: u8) -> Result<(), CompilerError> {
        // Create empty list
        self.chunk.emit(Instruction::new(Opcode::ListNew, dest, 0, 0));

        // Push each element (note: VM doesn't support ListPush on GC lists yet)
        for element in &l.elements {
            let temp = self.alloc_register()?;
            self.compile_expression(element, temp)?;
            // For now, just emit the push instruction. VM will error at runtime.
            // This is OK for initial implementation.
            self.chunk.emit(Instruction::new(Opcode::ListPush, temp, dest, 0));
            self.free_register();
        }

        Ok(())
    }

    fn compile_map_literal(&mut self, m: &MapLiteral, dest: u8) -> Result<(), CompilerError> {
        // Create empty map
        self.chunk.emit(Instruction::new(Opcode::MapNew, dest, 0, 0));

        // Set each entry
        for entry in &m.entries {
            // Compile key
            let key_reg = self.alloc_register()?;
            self.compile_expression(&entry.key, key_reg)?;

            // Compile value if present, otherwise use key as value (shorthand)
            let value_reg = self.alloc_register()?;
            if let Some(ref value) = entry.value {
                self.compile_expression(value, value_reg)?;
            } else {
                self.chunk.emit(Instruction::new(Opcode::LoadRegister, value_reg, key_reg, 0));
            }

            // Set entry (note: VM doesn't support MapSet on GC maps yet)
            self.chunk.emit(Instruction::new(Opcode::MapSet, value_reg, dest, key_reg));

            self.free_register();
            self.free_register();
        }

        Ok(())
    }

    fn compile_application(&mut self, a: &Application, dest: u8) -> Result<(), CompilerError> {
        // Compile left expression
        let left_reg = self.alloc_register()?;
        self.compile_expression(&a.left, left_reg)?;

        // Compile right expression
        let right_reg = self.alloc_register()?;
        self.compile_expression(&a.right, right_reg)?;

        // Emit Call instruction (VM doesn't implement it yet, but compiler should emit it)
        self.chunk.emit(Instruction::new(Opcode::Call, dest, left_reg, right_reg));

        self.free_register();
        self.free_register();

        Ok(())
    }

    fn compile_grouped_expression(&mut self, g: &GroupedExpression, dest: u8) -> Result<(), CompilerError> {
        self.compile_expression(&g.expression, dest)
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
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to tokenize, parse, and compile source code
pub fn compile_source(source: &str) -> Result<Chunk, CompilerError> {
    let tokens = lr_lexer::tokenize(source).map_err(|errs| {
        CompilerError::LexerError(format!("{} errors: {}", errs.len(), errs[0]))
    })?;
    let program = lr_parser::parse(tokens, "inline.lr".to_string())?;
    Compiler::compile(&program)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lr_vm::VM;

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
        // VM doesn't support ListPush on GC lists yet, so just verify bytecode
        let chunk = compile_source("[1, 2, 3]");
        assert!(chunk.is_ok());
        let chunk = chunk.unwrap();

        // Should have ListNew and ListPush instructions
        let has_list_new = chunk.code.iter().any(|i| i.opcode() == Opcode::ListNew);
        let has_list_push = chunk.code.iter().any(|i| i.opcode() == Opcode::ListPush);
        assert!(has_list_new, "Should have ListNew instruction");
        assert!(has_list_push, "Should have ListPush instructions");

        // Should have 3 number constants
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
        // VM doesn't support MapSet on GC maps yet, so just verify bytecode
        let chunk = compile_source("{ a: 1 }");
        assert!(chunk.is_ok());
        let chunk = chunk.unwrap();

        // Should have MapNew and MapSet instructions
        let has_map_new = chunk.code.iter().any(|i| i.opcode() == Opcode::MapNew);
        let has_map_set = chunk.code.iter().any(|i| i.opcode() == Opcode::MapSet);
        assert!(has_map_new, "Should have MapNew instruction");
        assert!(has_map_set, "Should have MapSet instruction");

        // Should have string "a" and number 1 constants
        let has_string_a = chunk.constants.iter().any(|c| matches!(c, Constant::String(s) if s == "a"));
        let has_number_1 = chunk.constants.iter().any(|c| matches!(c, Constant::Number(n) if *n == 1.0));
        assert!(has_string_a, "Should have string constant 'a'");
        assert!(has_number_1, "Should have number constant 1.0");
    }

    #[test]
    fn test_compile_map_shorthand() {
        // VM doesn't support MapSet on GC maps yet, so just verify bytecode
        let chunk = compile_source("{ a }");
        assert!(chunk.is_ok());
        let chunk = chunk.unwrap();

        // Should have MapNew and MapSet instructions
        let has_map_new = chunk.code.iter().any(|i| i.opcode() == Opcode::MapNew);
        let has_map_set = chunk.code.iter().any(|i| i.opcode() == Opcode::MapSet);
        assert!(has_map_new, "Should have MapNew instruction");
        assert!(has_map_set, "Should have MapSet instruction");

        // Should have string "a" constant
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

        // Should have MapNew, ListNew, MapSet, and ListPush instructions
        let has_map_new = chunk.code.iter().any(|i| i.opcode() == Opcode::MapNew);
        let has_list_new = chunk.code.iter().any(|i| i.opcode() == Opcode::ListNew);
        let has_map_set = chunk.code.iter().any(|i| i.opcode() == Opcode::MapSet);
        let has_list_push = chunk.code.iter().any(|i| i.opcode() == Opcode::ListPush);
        assert!(has_map_new, "Should have MapNew instruction");
        assert!(has_list_new, "Should have ListNew instruction");
        assert!(has_map_set, "Should have MapSet instructions");
        assert!(has_list_push, "Should have ListPush instructions");
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
    fn test_compile_unsupported_catch() {
        // `value !!!? handler` parses as Application chain, not as CatchExpression
        // CatchExpression only exists in the AST but is not generated by the parser
        let result = compile_source("42 !!!? handler");
        assert!(result.is_ok());
        let chunk = result.unwrap();
        let call_count = chunk.code.iter().filter(|i| i.opcode() == Opcode::Call).count();
        assert!(call_count >= 1, "Should have at least one Call instruction");
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
}
