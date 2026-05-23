use lr_bytecode::{Chunk, Constant, Instruction, Opcode};
use lr_vm::VM;

fn main() {
    let mut chunk = Chunk::new();
    
    let idx_one = chunk.add_constant(Constant::Number(1.0)).unwrap();
    let idx_five = chunk.add_constant(Constant::Number(5.0)).unwrap();

    println!("Instructions:");
    chunk.emit(Instruction::new(Opcode::MakeClosure, 1, 1, 0));
    println!("  0: MakeClosure a=1 b=1 c=0");
    
    chunk.emit(Instruction::new(Opcode::LoadArg, 2, 0, 0));
    println!("  1: LoadArg a=2 b=0 c=0");
    
    chunk.emit(Instruction::new(Opcode::LoadConstant, 3, 0, idx_one));
    println!("  2: LoadConstant a=3 b=0 c={}", idx_one);
    
    chunk.emit(Instruction::new(Opcode::Add, 0, 2, 3));
    println!("  3: Add a=0 b=2 c=3");
    
    chunk.emit(Instruction::new(Opcode::Return, 0, 0, 0));
    println!("  4: Return");

    chunk.emit(Instruction::new(Opcode::LoadConstant, 4, 0, idx_five));
    println!("  5: LoadConstant a=4 b=0 c={}", idx_five);
    
    chunk.emit(Instruction::new(Opcode::Call, 5, 1, 4));
    println!("  6: Call a=5 b=1 c=4");
    
    chunk.emit(Instruction::new(Opcode::LoadRegister, 0, 5, 0));
    println!("  7: LoadRegister a=0 b=5 c=0");
    
    chunk.emit(Instruction::new(Opcode::Return, 0, 0, 0));
    println!("  8: Return");

    let mut vm = VM::new();
    println!("\nExecuting chunk...");
    let result = vm.execute(&chunk);
    println!("Result: {:?}", result);
}