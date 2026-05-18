pub mod instruction;

pub use instruction::{Instruction, Opcode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ValueType {
    Undefined = 0,
    Boolean = 1,
    Number = 2,
    String = 3,
    List = 4,
    Map = 5,
    Operator = 6,
}

#[derive(Debug, thiserror::Error)]
pub enum BytecodeError {
    #[error("Constant pool overflow: cannot add more than 255 constants")]
    ConstantPoolOverflow,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    Undefined,
    Boolean(bool),
    Number(f64),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chunk {
    pub code: Vec<Instruction>,
    pub constants: Vec<Constant>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn emit(&mut self, instruction: Instruction) {
        self.code.push(instruction);
    }

    pub fn add_constant(&mut self, constant: Constant) -> Result<u8, BytecodeError> {
        if self.constants.len() >= 256 {
            return Err(BytecodeError::ConstantPoolOverflow);
        }
        self.constants.push(constant);
        Ok((self.constants.len() - 1) as u8)
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_operations() {
        let mut chunk = Chunk::new();

        let idx1 = chunk.add_constant(Constant::Number(42.0)).unwrap();
        assert_eq!(idx1, 0);

        let idx2 = chunk.add_constant(Constant::String("hello".to_string())).unwrap();
        assert_eq!(idx2, 1);

        let idx3 = chunk.add_constant(Constant::Boolean(true)).unwrap();
        assert_eq!(idx3, 2);

        assert_eq!(chunk.constants.len(), 3);
        assert_eq!(chunk.constants[0], Constant::Number(42.0));
        assert_eq!(chunk.constants[1], Constant::String("hello".to_string()));
        assert_eq!(chunk.constants[2], Constant::Boolean(true));

        chunk.emit(Instruction::new(Opcode::LoadConstant, 0, 0, idx1));
        chunk.emit(Instruction::new(Opcode::Add, 1, 0, idx2));
        chunk.emit(Instruction::new(Opcode::Return, 1, 0, 0));

        assert_eq!(chunk.code.len(), 3);
        assert_eq!(chunk.code[0].opcode(), Opcode::LoadConstant);
        assert_eq!(chunk.code[1].opcode(), Opcode::Add);
        assert_eq!(chunk.code[2].opcode(), Opcode::Return);
    }

    #[test]
    fn chunk_constant_overflow() {
        let mut chunk = Chunk::new();

        for i in 0..256 {
            chunk.add_constant(Constant::Number(i as f64)).unwrap();
        }

        let result = chunk.add_constant(Constant::Number(999.0));
        assert!(matches!(result, Err(BytecodeError::ConstantPoolOverflow)));
    }

    #[test]
    fn chunk_default() {
        let chunk = Chunk::default();
        assert!(chunk.code.is_empty());
        assert!(chunk.constants.is_empty());
    }

    #[test]
    fn constant_equality() {
        let c1 = Constant::Number(42.0);
        let c2 = Constant::Number(42.0);
        let c3 = Constant::Number(43.0);

        assert_eq!(c1, c2);
        assert_ne!(c1, c3);

        let s1 = Constant::String("hello".to_string());
        let s2 = Constant::String("hello".to_string());
        let s3 = Constant::String("world".to_string());

        assert_eq!(s1, s2);
        assert_ne!(s1, s3);
    }

    #[test]
    fn chunk_equality() {
        let mut chunk1 = Chunk::new();
        let mut chunk2 = Chunk::new();
        let mut chunk3 = Chunk::new();

        chunk1.add_constant(Constant::Number(42.0)).unwrap();
        chunk1.emit(Instruction::new(Opcode::LoadConstant, 0, 0, 0));

        chunk2.add_constant(Constant::Number(42.0)).unwrap();
        chunk2.emit(Instruction::new(Opcode::LoadConstant, 0, 0, 0));

        chunk3.add_constant(Constant::Number(43.0)).unwrap();
        chunk3.emit(Instruction::new(Opcode::LoadConstant, 0, 0, 0));

        assert_eq!(chunk1, chunk2);
        assert_ne!(chunk1, chunk3);
    }
}