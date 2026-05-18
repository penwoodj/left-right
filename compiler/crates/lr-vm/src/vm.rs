use crate::value::Value;
use gc_arena::{Collect, Mutation, Rootable};
use lr_bytecode::{Chunk, Constant, Instruction, Opcode};
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, Collect)]
#[collect(no_drop)]
pub struct VMRoot<'gc> {
    _marker: PhantomData<&'gc ()>,
}

pub struct Frame<'gc> {
    registers: Vec<Value<'gc>>,
    pc: usize,
}

impl<'gc> Frame<'gc> {
    pub fn new() -> Self {
        Self {
            registers: vec![Value::undefined(); 256],
            pc: 0,
        }
    }

    pub fn get(&self, reg: u8) -> Value<'gc> {
        self.registers[reg as usize]
    }

    pub fn set(&mut self, reg: u8, value: Value<'gc>) {
        self.registers[reg as usize] = value;
    }

    pub fn pc(&self) -> usize {
        self.pc
    }

    pub fn advance(&mut self) {
        self.pc += 1;
    }

    pub fn jump(&mut self, offset: i16) {
        self.pc = (self.pc as i64 + offset as i64) as usize;
    }
}

impl<'gc> Default for Frame<'gc> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct VM {
    arena: gc_arena::Arena<Rootable![VMRoot<'_>]>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            arena: gc_arena::Arena::<Rootable![VMRoot<'_>]>::new(|_| VMRoot {
                _marker: PhantomData,
            }),
        }
    }

    pub fn execute(&mut self, chunk: &Chunk) -> Result<String, VMError> {
        self.arena.mutate(|mc, _root| {
            let mut frame = Frame::new();
            let code = &chunk.code;
            let constants = &chunk.constants;

            let result = self.run_dispatch(mc, &mut frame, code, constants)?;
            Ok(result.to_string())
        })
    }

    fn run_dispatch<'a>(
        &self,
        mc: &Mutation<'a>,
        frame: &mut Frame<'a>,
        code: &[Instruction],
        constants: &[Constant],
    ) -> Result<Value<'a>, VMError> {
        while frame.pc() < code.len() {
            let inst = code[frame.pc()];
            let opcode = inst.opcode();

            match opcode {
                // Control flow
                Opcode::Nop => {
                    frame.advance();
                }
                Opcode::Return => {
                    let result = frame.get(inst.a());
                    return Ok(result);
                }
                Opcode::Jump => {
                    let offset = inst.a() as i8 as i16;
                    frame.jump(offset);
                }
                Opcode::JumpIfTrue => {
                    let cond = frame.get(inst.a());
                    let offset = inst.b() as i8 as i16;
                    if cond.is_truthy() {
                        frame.jump(offset);
                    } else {
                        frame.advance();
                    }
                }
                Opcode::JumpIfFalse => {
                    let cond = frame.get(inst.a());
                    let offset = inst.b() as i8 as i16;
                    if !cond.is_truthy() {
                        frame.jump(offset);
                    } else {
                        frame.advance();
                    }
                }
                Opcode::Call => {
                    frame.advance();
                }
                Opcode::TailCall => {
                    frame.advance();
                }

                // Load/Store
                Opcode::LoadConstant => {
                    let const_idx = inst.c() as usize;
                    let constant = constants
                        .get(const_idx)
                        .ok_or(VMError::ConstantIndexOutOfBounds(const_idx))?;

                    let value = match constant {
                        Constant::Undefined => Value::undefined(),
                        Constant::Boolean(b) => Value::boolean(mc, *b),
                        Constant::Number(n) => Value::number(*n),
                        Constant::String(s) => Value::string(mc, s.clone()),
                    };

                    frame.set(inst.a(), value);
                    frame.advance();
                }
                Opcode::LoadRegister => {
                    let value = frame.get(inst.b());
                    frame.set(inst.a(), value);
                    frame.advance();
                }
                Opcode::StoreRegister => {
                    let value = frame.get(inst.a());
                    frame.set(inst.b(), value);
                    frame.advance();
                }
                Opcode::LoadLocal => {
                    let value = frame.get(inst.b());
                    frame.set(inst.a(), value);
                    frame.advance();
                }
                Opcode::StoreLocal => {
                    let value = frame.get(inst.a());
                    frame.set(inst.b(), value);
                    frame.advance();
                }

                // Arithmetic
                Opcode::Add => {
                    let b = frame.get(inst.b());
                    let c = frame.get(inst.c());

                    let result = match (b, c) {
                        (Value::Number(b), Value::Number(c)) => Value::number(b + c),
                        (Value::String(b), Value::String(c)) => {
                            let combined = format!("{}{}", b, c);
                            Value::string(mc, combined)
                        }
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "Add requires numbers or strings, got {} and {}",
                                b.type_name(),
                                c.type_name()
                            )))
                        }
                    };

                    frame.set(inst.a(), result);
                    frame.advance();
                }
                Opcode::Sub => {
                    let b = frame.get(inst.b());
                    let c = frame.get(inst.c());

                    let result = match (b, c) {
                        (Value::Number(b), Value::Number(c)) => Value::number(b - c),
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "Sub requires numbers, got {} and {}",
                                b.type_name(),
                                c.type_name()
                            )))
                        }
                    };

                    frame.set(inst.a(), result);
                    frame.advance();
                }
                Opcode::Mul => {
                    let b = frame.get(inst.b());
                    let c = frame.get(inst.c());

                    let result = match (b, c) {
                        (Value::Number(b), Value::Number(c)) => Value::number(b * c),
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "Mul requires numbers, got {} and {}",
                                b.type_name(),
                                c.type_name()
                            )))
                        }
                    };

                    frame.set(inst.a(), result);
                    frame.advance();
                }
                Opcode::Div => {
                    let b = frame.get(inst.b());
                    let c = frame.get(inst.c());

                    let result = match (b, c) {
                        (Value::Number(b), Value::Number(c)) => {
                            if c == 0.0 {
                                return Err(VMError::DivisionByZero);
                            }
                            Value::number(b / c)
                        }
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "Div requires numbers, got {} and {}",
                                b.type_name(),
                                c.type_name()
                            )))
                        }
                    };

                    frame.set(inst.a(), result);
                    frame.advance();
                }
                Opcode::Mod => {
                    let b = frame.get(inst.b());
                    let c = frame.get(inst.c());

                    let result = match (b, c) {
                        (Value::Number(b), Value::Number(c)) => {
                            if c == 0.0 {
                                return Err(VMError::DivisionByZero);
                            }
                            Value::number(b % c)
                        }
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "Mod requires numbers, got {} and {}",
                                b.type_name(),
                                c.type_name()
                            )))
                        }
                    };

                    frame.set(inst.a(), result);
                    frame.advance();
                }
                Opcode::Neg => {
                    let b = frame.get(inst.b());

                    let result = match b {
                        Value::Number(n) => Value::number(-n),
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "Neg requires a number, got {}",
                                b.type_name()
                            )))
                        }
                    };

                    frame.set(inst.a(), result);
                    frame.advance();
                }

                // Comparison
                Opcode::Eq => {
                    let b = frame.get(inst.b());
                    let c = frame.get(inst.c());
                    let result = Value::boolean(mc, b.deep_eq(&c));
                    frame.set(inst.a(), result);
                    frame.advance();
                }
                Opcode::Ne => {
                    let b = frame.get(inst.b());
                    let c = frame.get(inst.c());
                    let result = Value::boolean(mc, !b.deep_eq(&c));
                    frame.set(inst.a(), result);
                    frame.advance();
                }
                Opcode::Lt => {
                    let b = frame.get(inst.b());
                    let c = frame.get(inst.c());

                    let result = match (b, c) {
                        (Value::Number(b), Value::Number(c)) => Value::boolean(mc, b < c),
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "Lt requires numbers, got {} and {}",
                                b.type_name(),
                                c.type_name()
                            )))
                        }
                    };

                    frame.set(inst.a(), result);
                    frame.advance();
                }
                Opcode::Le => {
                    let b = frame.get(inst.b());
                    let c = frame.get(inst.c());

                    let result = match (b, c) {
                        (Value::Number(b), Value::Number(c)) => Value::boolean(mc, b <= c),
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "Le requires numbers, got {} and {}",
                                b.type_name(),
                                c.type_name()
                            )))
                        }
                    };

                    frame.set(inst.a(), result);
                    frame.advance();
                }
                Opcode::Gt => {
                    let b = frame.get(inst.b());
                    let c = frame.get(inst.c());

                    let result = match (b, c) {
                        (Value::Number(b), Value::Number(c)) => Value::boolean(mc, b > c),
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "Gt requires numbers, got {} and {}",
                                b.type_name(),
                                c.type_name()
                            )))
                        }
                    };

                    frame.set(inst.a(), result);
                    frame.advance();
                }
                Opcode::Ge => {
                    let b = frame.get(inst.b());
                    let c = frame.get(inst.c());

                    let result = match (b, c) {
                        (Value::Number(b), Value::Number(c)) => Value::boolean(mc, b >= c),
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "Ge requires numbers, got {} and {}",
                                b.type_name(),
                                c.type_name()
                            )))
                        }
                    };

                    frame.set(inst.a(), result);
                    frame.advance();
                }

                // Boolean
                Opcode::Not => {
                    let b = frame.get(inst.b());
                    let result = Value::boolean(mc, !b.is_truthy());
                    frame.set(inst.a(), result);
                    frame.advance();
                }
                Opcode::And => {
                    let b = frame.get(inst.b());
                    let c = frame.get(inst.c());
                    let result = Value::boolean(mc, b.is_truthy() && c.is_truthy());
                    frame.set(inst.a(), result);
                    frame.advance();
                }
                Opcode::Or => {
                    let b = frame.get(inst.b());
                    let c = frame.get(inst.c());
                    let result = Value::boolean(mc, b.is_truthy() || c.is_truthy());
                    frame.set(inst.a(), result);
                    frame.advance();
                }
                Opcode::ToBoolean => {
                    let b = frame.get(inst.b());
                    let result = Value::boolean(mc, b.is_truthy());
                    frame.set(inst.a(), result);
                    frame.advance();
                }

                // Map operations
                Opcode::MapNew => {
                    frame.set(inst.a(), Value::map(mc, vec![]));
                    frame.advance();
                }
                Opcode::MapGet => {
                    let map_val = frame.get(inst.b());
                    let key = frame.get(inst.c());

                    match map_val {
                        Value::Map(map) => {
                            let result = map
                                .iter()
                                .find(|(k, _)| k.deep_eq(&key))
                                .map(|(_, v)| *v)
                                .unwrap_or(Value::undefined());
                            frame.set(inst.a(), result);
                        }
                        Value::List(list) => {
                            match key {
                                Value::Number(idx) if idx.fract() == 0.0 => {
                                    let idx = idx as i64;
                                    if idx >= 0 && (idx as usize) < list.len() {
                                        frame.set(inst.a(), list[idx as usize]);
                                    } else {
                                        frame.set(inst.a(), Value::undefined());
                                    }
                                }
                                _ => {
                                    return Err(VMError::TypeError(format!(
                                        "List index must be integer number, got {}",
                                        key.type_name()
                                    )))
                                }
                            }
                        }
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "MapGet requires map or list, got {}",
                                map_val.type_name()
                            )))
                        }
                    }
                    frame.advance();
                }
                Opcode::MapSet => {
                    let map_val = frame.get(inst.b());
                    let _key = frame.get(inst.c());
                    let _value = frame.get(inst.a());

                    match map_val {
                        Value::Map(_) => {
                            return Err(VMError::Runtime(
                                "MapSet on Gc map not yet supported (immutable)".to_string(),
                            ))
                        }
                        Value::List(_) => {
                            return Err(VMError::Runtime(
                                "MapSet on Gc list not yet supported (immutable)".to_string(),
                            ))
                        }
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "MapSet requires map or list, got {}",
                                map_val.type_name()
                            )))
                        }
                    }
                }
                Opcode::MapMerge => {
                    return Err(VMError::UnimplementedOpcode(Opcode::MapMerge))
                }
                Opcode::MapPick => {
                    return Err(VMError::UnimplementedOpcode(Opcode::MapPick))
                }
                Opcode::MapOmit => {
                    return Err(VMError::UnimplementedOpcode(Opcode::MapOmit))
                }

                // List operations
                Opcode::ListNew => {
                    frame.set(inst.a(), Value::list(mc, vec![]));
                    frame.advance();
                }
                Opcode::ListGet => {
                    let list_val = frame.get(inst.b());
                    let idx_val = frame.get(inst.c());

                    match list_val {
                        Value::List(list) => {
                            match idx_val {
                                Value::Number(idx) if idx.fract() == 0.0 => {
                                    let idx = idx as i64;
                                    if idx >= 0 && (idx as usize) < list.len() {
                                        frame.set(inst.a(), list[idx as usize]);
                                    } else {
                                        frame.set(inst.a(), Value::undefined());
                                    }
                                }
                                _ => {
                                    return Err(VMError::TypeError(format!(
                                        "List index must be integer number, got {}",
                                        idx_val.type_name()
                                    )))
                                }
                            }
                        }
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "ListGet requires list, got {}",
                                list_val.type_name()
                            )))
                        }
                    }
                    frame.advance();
                }
                Opcode::ListSet => {
                    return Err(VMError::Runtime(
                        "ListSet on Gc list not yet supported (immutable)".to_string(),
                    ))
                }
                Opcode::ListPush => {
                    return Err(VMError::Runtime(
                        "ListPush on Gc list not yet supported (immutable)".to_string(),
                    ))
                }
                Opcode::ListPop => {
                    return Err(VMError::Runtime(
                        "ListPop on Gc list not yet supported (immutable)".to_string(),
                    ))
                }
                Opcode::ListLen => {
                    let list_val = frame.get(inst.b());

                    match list_val {
                        Value::List(list) => {
                            frame.set(inst.a(), Value::number(list.len() as f64));
                        }
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "ListLen requires list, got {}",
                                list_val.type_name()
                            )))
                        }
                    }
                    frame.advance();
                }

                // String operations
                Opcode::StringConcat => {
                    let b = frame.get(inst.b());
                    let c = frame.get(inst.c());

                    match (b, c) {
                        (Value::String(b), Value::String(c)) => {
                            let combined = format!("{}{}", b, c);
                            frame.set(inst.a(), Value::string(mc, combined));
                        }
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "StringConcat requires strings, got {} and {}",
                                b.type_name(),
                                c.type_name()
                            )))
                        }
                    }
                    frame.advance();
                }
                Opcode::StringLen => {
                    let str_val = frame.get(inst.b());

                    match str_val {
                        Value::String(s) => {
                            frame.set(inst.a(), Value::number(s.len() as f64));
                        }
                        _ => {
                            return Err(VMError::TypeError(format!(
                                "StringLen requires string, got {}",
                                str_val.type_name()
                            )))
                        }
                    }
                    frame.advance();
                }
                Opcode::StringSlice => {
                    return Err(VMError::UnimplementedOpcode(Opcode::StringSlice))
                }
                Opcode::StringToUpper => {
                    return Err(VMError::UnimplementedOpcode(Opcode::StringToUpper))
                }
                Opcode::StringToLower => {
                    return Err(VMError::UnimplementedOpcode(Opcode::StringToLower))
                }
                Opcode::StringCapitalize => {
                    return Err(VMError::UnimplementedOpcode(Opcode::StringCapitalize))
                }

                // Loop operators
                Opcode::LoopMap => return Err(VMError::UnimplementedOpcode(Opcode::LoopMap)),
                Opcode::LoopFilter => {
                    return Err(VMError::UnimplementedOpcode(Opcode::LoopFilter))
                }
                Opcode::LoopFlatMap => {
                    return Err(VMError::UnimplementedOpcode(Opcode::LoopFlatMap))
                }
                Opcode::LoopUniqueBy => {
                    return Err(VMError::UnimplementedOpcode(Opcode::LoopUniqueBy))
                }
                Opcode::LoopGroupBy => {
                    return Err(VMError::UnimplementedOpcode(Opcode::LoopGroupBy))
                }
                Opcode::LoopEachToString => {
                    return Err(VMError::UnimplementedOpcode(Opcode::LoopEachToString))
                }
                Opcode::LoopEvery => {
                    return Err(VMError::UnimplementedOpcode(Opcode::LoopEvery))
                }
                Opcode::LoopSome => {
                    return Err(VMError::UnimplementedOpcode(Opcode::LoopSome))
                }
                Opcode::LoopFind => {
                    return Err(VMError::UnimplementedOpcode(Opcode::LoopFind))
                }
                Opcode::LoopSort => {
                    return Err(VMError::UnimplementedOpcode(Opcode::LoopSort))
                }
                Opcode::LoopCompact => {
                    return Err(VMError::UnimplementedOpcode(Opcode::LoopCompact))
                }

                // Error handling
                Opcode::Throw => return Err(VMError::UnimplementedOpcode(Opcode::Throw)),
                Opcode::Catch => return Err(VMError::UnimplementedOpcode(Opcode::Catch)),
                Opcode::CatchEnd => {
                    return Err(VMError::UnimplementedOpcode(Opcode::CatchEnd))
                }

                // Async
                Opcode::MakeAsync => {
                    return Err(VMError::UnimplementedOpcode(Opcode::MakeAsync))
                }
                Opcode::Await => return Err(VMError::UnimplementedOpcode(Opcode::Await)),

                // Special
                Opcode::Push => return Err(VMError::UnimplementedOpcode(Opcode::Push)),
                Opcode::Pop => return Err(VMError::UnimplementedOpcode(Opcode::Pop)),
                Opcode::Dup => return Err(VMError::UnimplementedOpcode(Opcode::Dup)),
                Opcode::ReverseArgs => {
                    return Err(VMError::UnimplementedOpcode(Opcode::ReverseArgs))
                }
                Opcode::SilentExec => {
                    return Err(VMError::UnimplementedOpcode(Opcode::SilentExec))
                }
                Opcode::Import => return Err(VMError::UnimplementedOpcode(Opcode::Import)),
                Opcode::Export => return Err(VMError::UnimplementedOpcode(Opcode::Export)),
            }
        }

        Ok(frame.get(0))
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum VMError {
    #[error("Invalid opcode: {0}")]
    InvalidOpcode(u8),
    #[error("Unimplemented opcode: {0:?}")]
    UnimplementedOpcode(Opcode),
    #[error("Constant index out of bounds: {0}")]
    ConstantIndexOutOfBounds(usize),
    #[error("Type error: {0}")]
    TypeError(String),
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Runtime error: {0}")]
    Runtime(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use lr_bytecode::Instruction;

    fn build_chunk(f: impl FnOnce(&mut Chunk)) -> Chunk {
        let mut chunk = Chunk::new();
        f(&mut chunk);
        chunk
    }

    #[test]
    fn test_vm_nop() {
        let chunk = build_chunk(|c| {
            c.emit(Instruction::new(Opcode::Nop, 0, 0, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "undefined");
    }

    #[test]
    fn test_vm_load_constant_number() {
        let chunk = build_chunk(|c| {
            let idx = c.add_constant(Constant::Number(42.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 1, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "42");
    }

    #[test]
    fn test_vm_load_constant_string() {
        let chunk = build_chunk(|c| {
            let idx = c.add_constant(Constant::String("hello".to_string())).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 1, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello");
    }

    #[test]
    fn test_vm_load_constant_boolean() {
        let chunk = build_chunk(|c| {
            let idx = c.add_constant(Constant::Boolean(true)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 1, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_vm_arithmetic_add() {
        let chunk = build_chunk(|c| {
            let idx5 = c.add_constant(Constant::Number(5.0)).unwrap();
            let idx3 = c.add_constant(Constant::Number(3.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx5));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx3));
            c.emit(Instruction::new(Opcode::Add, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "8");
    }

    #[test]
    fn test_vm_arithmetic_sub() {
        let chunk = build_chunk(|c| {
            let idx10 = c.add_constant(Constant::Number(10.0)).unwrap();
            let idx3 = c.add_constant(Constant::Number(3.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx10));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx3));
            c.emit(Instruction::new(Opcode::Sub, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "7");
    }

    #[test]
    fn test_vm_arithmetic_mul() {
        let chunk = build_chunk(|c| {
            let idx4 = c.add_constant(Constant::Number(4.0)).unwrap();
            let idx3 = c.add_constant(Constant::Number(3.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx4));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx3));
            c.emit(Instruction::new(Opcode::Mul, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "12");
    }

    #[test]
    fn test_vm_arithmetic_div() {
        let chunk = build_chunk(|c| {
            let idx10 = c.add_constant(Constant::Number(10.0)).unwrap();
            let idx2 = c.add_constant(Constant::Number(2.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx10));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx2));
            c.emit(Instruction::new(Opcode::Div, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "5");
    }

    #[test]
    fn test_vm_arithmetic_neg() {
        let chunk = build_chunk(|c| {
            let idx3 = c.add_constant(Constant::Number(3.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx3));
            c.emit(Instruction::new(Opcode::Neg, 2, 1, 0));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 2, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "-3");
    }

    #[test]
    fn test_vm_comparison_eq() {
        let chunk = build_chunk(|c| {
            let idx5 = c.add_constant(Constant::Number(5.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx5));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx5));
            c.emit(Instruction::new(Opcode::Eq, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_vm_comparison_lt() {
        let chunk = build_chunk(|c| {
            let idx3 = c.add_constant(Constant::Number(3.0)).unwrap();
            let idx5 = c.add_constant(Constant::Number(5.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx3));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx5));
            c.emit(Instruction::new(Opcode::Lt, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_vm_boolean_not() {
        let chunk = build_chunk(|c| {
            let idx = c.add_constant(Constant::Boolean(true)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx));
            c.emit(Instruction::new(Opcode::Not, 2, 1, 0));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 2, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "false");
    }

    #[test]
    fn test_vm_string_concat() {
        let chunk = build_chunk(|c| {
            let idx1 = c.add_constant(Constant::String("hello".to_string())).unwrap();
            let idx2 = c
                .add_constant(Constant::String(" world".to_string()))
                .unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx1));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx2));
            c.emit(Instruction::new(Opcode::StringConcat, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello world");
    }

    #[test]
    fn test_vm_jump() {
        let chunk = build_chunk(|c| {
            let idx = c.add_constant(Constant::Number(42.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx));
            c.emit(Instruction::new(Opcode::Jump, 2, 0, 0)); // skip next instruction
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx)); // should be skipped
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 1, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "42");
    }

    #[test]
    fn test_vm_conditional_jump() {
        let chunk = build_chunk(|c| {
            let idx_true = c.add_constant(Constant::Boolean(true)).unwrap();
            let idx_skip = c.add_constant(Constant::Number(99.0)).unwrap();
            let idx_exec = c.add_constant(Constant::Number(42.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx_true));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx_skip));
            c.emit(Instruction::new(Opcode::JumpIfTrue, 1, 3, 0)); // skip 3 instructions
            c.emit(Instruction::new(Opcode::LoadConstant, 0, 0, idx_skip)); // should be skipped
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0)); // should be skipped
            c.emit(Instruction::new(Opcode::LoadConstant, 0, 0, idx_exec));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "42");
    }

    #[test]
    fn test_vm_list_operations() {
        let chunk = build_chunk(|c| {
            let _idx42 = c.add_constant(Constant::Number(42.0)).unwrap();
            c.emit(Instruction::new(Opcode::ListNew, 1, 0, 0));
            c.emit(Instruction::new(Opcode::ListLen, 2, 1, 0));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 2, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "0");
    }

    #[test]
    fn test_vm_map_operations() {
        let chunk = build_chunk(|c| {
            c.emit(Instruction::new(Opcode::MapNew, 1, 0, 0));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 1, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("{"));
    }

    #[test]
    fn test_vm_division_by_zero() {
        let chunk = build_chunk(|c| {
            let idx10 = c.add_constant(Constant::Number(10.0)).unwrap();
            let idx0 = c.add_constant(Constant::Number(0.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx10));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx0));
            c.emit(Instruction::new(Opcode::Div, 3, 1, 2));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(matches!(result, Err(VMError::DivisionByZero)));
    }

    #[test]
    fn test_vm_type_error() {
        let chunk = build_chunk(|c| {
            let idx_num = c.add_constant(Constant::Number(42.0)).unwrap();
            let idx_str = c.add_constant(Constant::String("hello".to_string())).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx_num));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx_str));
            c.emit(Instruction::new(Opcode::Add, 3, 1, 2));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(matches!(result, Err(VMError::TypeError(_))));
    }

    #[test]
    fn test_vm_add_string_strings() {
        let chunk = build_chunk(|c| {
            let idx1 = c.add_constant(Constant::String("foo".to_string())).unwrap();
            let idx2 = c.add_constant(Constant::String("bar".to_string())).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx1));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx2));
            c.emit(Instruction::new(Opcode::Add, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "foobar");
    }

    #[test]
    fn test_vm_comparison_ne() {
        let chunk = build_chunk(|c| {
            let idx5 = c.add_constant(Constant::Number(5.0)).unwrap();
            let idx3 = c.add_constant(Constant::Number(3.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx5));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx3));
            c.emit(Instruction::new(Opcode::Ne, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_vm_comparison_le() {
        let chunk = build_chunk(|c| {
            let idx3 = c.add_constant(Constant::Number(3.0)).unwrap();
            let idx5 = c.add_constant(Constant::Number(5.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx3));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx5));
            c.emit(Instruction::new(Opcode::Le, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_vm_comparison_gt() {
        let chunk = build_chunk(|c| {
            let idx5 = c.add_constant(Constant::Number(5.0)).unwrap();
            let idx3 = c.add_constant(Constant::Number(3.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx5));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx3));
            c.emit(Instruction::new(Opcode::Gt, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_vm_comparison_ge() {
        let chunk = build_chunk(|c| {
            let idx5 = c.add_constant(Constant::Number(5.0)).unwrap();
            let idx3 = c.add_constant(Constant::Number(3.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx5));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx3));
            c.emit(Instruction::new(Opcode::Ge, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_vm_boolean_and() {
        let chunk = build_chunk(|c| {
            let idx1 = c.add_constant(Constant::Boolean(true)).unwrap();
            let idx2 = c.add_constant(Constant::Boolean(false)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx1));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx2));
            c.emit(Instruction::new(Opcode::And, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "false");
    }

    #[test]
    fn test_vm_boolean_or() {
        let chunk = build_chunk(|c| {
            let idx1 = c.add_constant(Constant::Boolean(true)).unwrap();
            let idx2 = c.add_constant(Constant::Boolean(false)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx1));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx2));
            c.emit(Instruction::new(Opcode::Or, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_vm_boolean_to_boolean() {
        let chunk = build_chunk(|c| {
            let idx42 = c.add_constant(Constant::Number(42.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx42));
            c.emit(Instruction::new(Opcode::ToBoolean, 2, 1, 0));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 2, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "true");
    }

    #[test]
    fn test_vm_mod() {
        let chunk = build_chunk(|c| {
            let idx10 = c.add_constant(Constant::Number(10.0)).unwrap();
            let idx3 = c.add_constant(Constant::Number(3.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx10));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx3));
            c.emit(Instruction::new(Opcode::Mod, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "1");
    }

    #[test]
    fn test_vm_mod_by_zero() {
        let chunk = build_chunk(|c| {
            let idx10 = c.add_constant(Constant::Number(10.0)).unwrap();
            let idx0 = c.add_constant(Constant::Number(0.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx10));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx0));
            c.emit(Instruction::new(Opcode::Mod, 3, 1, 2));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(matches!(result, Err(VMError::DivisionByZero)));
    }

    #[test]
    fn test_vm_unimplemented_opcodes() {
        let unimplemented_opcodes = vec![
            Opcode::MapMerge,
            Opcode::MapPick,
            Opcode::MapOmit,
            Opcode::StringSlice,
            Opcode::StringToUpper,
            Opcode::StringToLower,
            Opcode::StringCapitalize,
            Opcode::LoopMap,
            Opcode::LoopFilter,
            Opcode::LoopFlatMap,
            Opcode::LoopUniqueBy,
            Opcode::LoopGroupBy,
            Opcode::LoopEachToString,
            Opcode::LoopEvery,
            Opcode::LoopSome,
            Opcode::LoopFind,
            Opcode::LoopSort,
            Opcode::LoopCompact,
            Opcode::Throw,
            Opcode::Catch,
            Opcode::CatchEnd,
            Opcode::MakeAsync,
            Opcode::Await,
            Opcode::Push,
            Opcode::Pop,
            Opcode::Dup,
            Opcode::ReverseArgs,
            Opcode::SilentExec,
            Opcode::Import,
            Opcode::Export,
        ];

        for opcode in unimplemented_opcodes {
            let chunk = build_chunk(|c| {
                c.emit(Instruction::new(opcode, 0, 0, 0));
                c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
            });

            let mut vm = VM::new();
            let result = vm.execute(&chunk);
            assert!(
                matches!(result, Err(VMError::UnimplementedOpcode(op)) if op == opcode),
                "Opcode {:?} should return UnimplementedOpcode",
                opcode
            );
        }
    }

    #[test]
    fn test_vm_arithmetic_float() {
        let chunk = build_chunk(|c| {
            let idx1 = c.add_constant(Constant::Number(3.0)).unwrap();
            let idx2 = c.add_constant(Constant::Number(2.14)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx1));
            c.emit(Instruction::new(Opcode::LoadConstant, 2, 0, idx2));
            c.emit(Instruction::new(Opcode::Add, 3, 1, 2));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 3, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        let result_str = result.unwrap();
        assert!(result_str.starts_with("5.14"), "Expected result to start with '5.14', got '{}'", result_str);
    }

    #[test]
    fn test_vm_string_len() {
        let chunk = build_chunk(|c| {
            let idx = c.add_constant(Constant::String("hello".to_string())).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx));
            c.emit(Instruction::new(Opcode::StringLen, 2, 1, 0));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 2, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "5");
    }

    #[test]
    fn test_vm_number_display() {
        let chunk = build_chunk(|c| {
            let idx_int = c.add_constant(Constant::Number(42.0)).unwrap();
            let _idx_float = c.add_constant(Constant::Number(3.14159)).unwrap();
            let _idx_neg_int = c.add_constant(Constant::Number(-7.0)).unwrap();
            let _idx_neg_float = c.add_constant(Constant::Number(-2.5)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 0, 0, idx_int));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert_eq!(result.unwrap(), "42");

        let chunk = build_chunk(|c| {
            let idx_float = c.add_constant(Constant::Number(3.14159)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 0, 0, idx_float));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let result = vm.execute(&chunk);
        assert_eq!(result.unwrap(), "3.14159");
    }

    #[test]
    fn test_vm_zero_is_falsy() {
        let chunk = build_chunk(|c| {
            let idx = c.add_constant(Constant::Number(0.0)).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx));
            c.emit(Instruction::new(Opcode::ToBoolean, 2, 1, 0));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 2, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "false");
    }

    #[test]
    fn test_vm_empty_string_is_falsy() {
        let chunk = build_chunk(|c| {
            let idx = c.add_constant(Constant::String("".to_string())).unwrap();
            c.emit(Instruction::new(Opcode::LoadConstant, 1, 0, idx));
            c.emit(Instruction::new(Opcode::ToBoolean, 2, 1, 0));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 2, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "false");
    }

    #[test]
    fn test_vm_undefined_is_falsy() {
        let chunk = build_chunk(|c| {
            c.emit(Instruction::new(Opcode::ToBoolean, 1, 0, 0));
            c.emit(Instruction::new(Opcode::LoadRegister, 0, 1, 0));
            c.emit(Instruction::new(Opcode::Return, 0, 0, 0));
        });

        let mut vm = VM::new();
        let result = vm.execute(&chunk);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "false");
    }
}