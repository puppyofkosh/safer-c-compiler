use assembly::Instruction::*;
use assembly::Instruction;

use assembly::Operand::*;
use assembly::RegisterVal::*;

use ast::VarType;

pub static WORD_SIZE: i32 = 4;

pub fn get_type_size(t: VarType) -> i32 {
    match t {
        VarType::Int => WORD_SIZE,
        VarType::Char => 1,
    }
}

pub fn alloc_stack(size: i32) -> Instruction {
    assert!(size >= 0);
    if size == 0 {
        return NOP;
    }
    Subtract(IntConstant(size),
             Register(ESP))
}

pub fn free_stack(size: i32) -> Instruction {
    assert!(size >= 0);
    if size == 0 {
        return NOP;
    }
    Add(IntConstant(size),
        Register(ESP))
}
