use assembly::MachineType;
use assembly::Instruction::*;
use assembly::Instruction;

use assembly::Operand;
use assembly::Operand::*;
use assembly::RegisterVal;
use assembly::RegisterVal::*;


use ast::VarType;

pub const WORD_SIZE: i32 = 4;

// TODO: Remove this function
pub fn get_type_size(t: &VarType) -> i32 {
    match *t {
        VarType::Pointer(_) => WORD_SIZE,
        VarType::Int => WORD_SIZE,
        VarType::Char => 1,
        VarType::Struct(_) => panic!("wth")
    }
}

// TODO: Remove this function
pub fn type_to_machine_type(t: &VarType) -> MachineType {
    match *t {
        VarType::Pointer(_) => MachineType::Long,
        VarType::Int => MachineType::Long,
        VarType::Char => MachineType::Byte,
        VarType::Struct(_) => panic!("wth")
    }
}

pub fn get_mtype_size(t: MachineType) -> i32 {
    match t {
        MachineType::Long => WORD_SIZE,
        MachineType::Byte => 1,
        MachineType::Chunk(i) => i
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

pub fn register_besides(r: &RegisterVal) -> RegisterVal {
    match *r {
        EAX | AL => EBX,
        EBX | BL => EAX,
        ECX | CL => EBX,
        ESP | EBP => panic!("What are you doing with this function?"),
    }
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
                 typ: MachineType) -> Instruction {
    if to == from {
        return NOP;
    }

    match typ {
        MachineType::Long => Move(from, to),
        MachineType::Byte => {
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
        _ => panic!("Unsupported type {:?}", typ)
    }
}
