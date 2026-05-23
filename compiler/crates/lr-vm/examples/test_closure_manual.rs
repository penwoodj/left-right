use lr_bytecode::{Chunk, Constant, Instruction, Opcode};
use lr_vm::VM;

fn main() {
    let mut chunk = Chunk::new();
    
    let idx_one = chunk.add_constant(Constant::Number(1.0)).unwrap();
    let idx_five = chunk.add_constant(Constant::Number(5.0)).unwrap();

    chunk.emit(Instruction::new(Opcode::MakeClosure, 1, 1, 0));
    chunk.emit(Instruction::new(Opcode::LoadArg, 2, 0, 0));
    chunk.emit(Instruction::new(Opcode::LoadConstant, 3, 0, idx_one));
    chunk.emit(Instruction::new(Opcode::Add, 0, 2, 3));
    chunk.emit(Instruction::new(Opcode::Return, 0, 0, 0));

    chunk.emit(Instruction::new(Opcode::LoadConstant, 4, 0, idx_five));
    chunk.emit(Instruction::new(Opcode::Call, 5, 1, 4));
    chunk.emit(Instruction::new(Opcode::LoadRegister, 0, 5, 0));
    chunk.emit(Instruction::new(Opcode::Return, 0, 0, 0));

    let mut vm = VM::new();
    println!("Executing chunk...");
    let result = vm.execute(&chunk);
    println!("Result: {:?}", result);
}