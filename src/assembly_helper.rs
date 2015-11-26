use assembly::Instruction::*;
use assembly::Instruction;

use assembly::Operand;
use assembly::Operand::*;
use assembly::RegisterVal;
use assembly::RegisterVal::*;


use ast::VarType;

pub const WORD_SIZE: i32 = 4;

pub fn get_type_size(t: &VarType) -> i32 {
    match *t {
        VarType::Pointer(_) => WORD_SIZE,
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


pub fn get_low_byte(o: &RegisterVal) -> RegisterVal {
    match *o {
        EAX => AL,
        EBX => BL,
        ECX => CL,
        _ => panic!("Register doesn't have low byte"),
    }
}

pub fn move_type(from: Operand, to: Operand,
             typ: &VarType) -> Instruction {
    if to == from {
        return NOP;
    }


    let sz = get_type_size(typ);
    match sz {
        WORD_SIZE => Move(from, to),
        1 => {
            if let Register(_) = to {
                OtherTwoArg("movzbl", from, to)
            } else if let Dereference(_, _) = to {
                let mut src = from;
                if let Register(reg) = src {
                    src = Register(get_low_byte(&reg));
                }
                
                OtherTwoArg("movb", src, to)
            } else {
                panic!("You cannot move to {:?}", to);
            }
        }
        _ => panic!("Unsupported type of size {}", sz)
    }
}
