use assembly::MachineType;
use assembly::Instruction::*;
use assembly::Instruction;

use assembly::Operand;
use assembly::Operand::*;
use assembly::RegisterVal;
use assembly::RegisterVal::*;

pub const WORD_SIZE: i32 = 4;

/// Get the size of the machine type
pub fn get_mtype_size(t: MachineType) -> i32 {
    match t {
        MachineType::Long => WORD_SIZE,
        MachineType::Byte => 1,
        MachineType::Chunk(i) => i
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

/// Return a register beside the given register
pub fn register_other_than(r: &RegisterVal) -> RegisterVal {
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

/// Move things between register and memory
/// ```
/// Treat the thing in eax as char and move it onto the stack
/// move_type(EAX, some place on the stack,i.e. -10(EBP), char)
/// ```
pub fn move_type(from: Operand, to: Operand,
                 typ: MachineType) -> Instruction {
    if to == from {
        return NOP;
    }


    match typ {
        MachineType::Long => Move(from, to),
        MachineType::Byte => {
            if let Register(_) = to {
                // Move from stack to register
                // Put the char to register which treats it as a Long, zero
                // out the rest of the register as well
                OtherTwoArg("movzbl", from, to)
            } else if let Dereference(_, _) = to {
                // Move from register to stack
                let mut src = from;
                if let Register(reg) = src {
                    src = Register(get_low_byte(&reg));
                }

                OtherTwoArg("movb", src, to)
            } else {
                panic!("You cannot move to {:?}", to);
            }
        }
        MachineType::Chunk(_) => panic!("Use memcpy to move chunks!"),
    }
}
