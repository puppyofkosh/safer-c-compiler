use assembly::Instruction::*;
use assembly::Instruction;

use assembly::Operand;
use assembly::Operand::*;
use assembly::RegisterVal;
use assembly::RegisterVal::*;


use ast::VarType;

pub const WORD_SIZE: i32 = 4;

/// Return the byte size of a type
/// ```
/// assert_eq!(4, get_type_size(Int))
/// ```
pub fn get_type_size(t: &VarType) -> i32 {
    match *t {
        VarType::Pointer(_) => WORD_SIZE,
        VarType::Int => WORD_SIZE,
        VarType::Char => 1,
    }
}

/// Generate assembly code for allocating stack
pub fn alloc_stack(size: i32) -> Instruction {
    assert!(size >= 0);
    if size == 0 {
        return NOP;
    }
    Subtract(IntConstant(size),
             Register(ESP))
}

/// Generate assembly code for freeing stack
pub fn free_stack(size: i32) -> Instruction {
    assert!(size >= 0);
    if size == 0 {
        return NOP;
    }
    Add(IntConstant(size),
        Register(ESP))
}

/// Return the register beside the given register
pub fn register_besides(r: &RegisterVal) -> RegisterVal {
    match *r {
        EAX | AL => EBX,
        EBX | BL => EAX,
        ECX | CL => EBX,
        ESP | EBP => panic!("What are you doing with this function?"),
    }
}

/// Return the low bit version of the given register
pub fn get_low_byte(o: &RegisterVal) -> RegisterVal {
    match *o {
        EAX => AL,
        EBX => BL,
        ECX => CL,
        _ => panic!("Register doesn't have low byte"),
    }
}

/// 
pub fn move_type(from: Operand, to: Operand,
             typ: &VarType) -> Instruction {
    if to == from {
        return NOP;
    }


    let sz = get_type_size(typ);
    match sz {
        WORD_SIZE => Move(from, to),
        1 => {
            // the type is char, currently
            if let Register(_) = to {
                // move from unsigned integer to wider unsigned integer
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
