use assembly::Instruction;
use assembly::Instruction::*;
use assembly::Operand;
use assembly::Operand::*;

use assembly::RegisterVal::*;
use assembly::RegisterVal;

fn reg_to_str(r: &RegisterVal) -> String {
    match *r {
        EAX => "%eax".to_string(),
        EBX => "%ebx".to_string(),
        ECX => "%ecx".to_string(),
        EBP => "%ebp".to_string(),
        ESP => "%esp".to_string(),
        AL => "%al".to_string(),
        BL => "%bl".to_string(),
        CL => "%cl".to_string(),
    }
}

fn op_to_str(o: &Operand) -> String {
    match *o {
        Register(r) => reg_to_str(&r),
        IntConstant(i) => "$".to_string() + &i.to_string(),
        VariableStatic(n) => "$".to_string() + &n.to_string(),
        Variable(ref s) => "$".to_string() + &s.clone(),
        Dereference(ref e, offset) => format!("{}({})", offset, reg_to_str(e)),
    }
}

fn instruction_to_asm(ins: &Instruction) -> String {
    let mut s = match *ins {
        Add(ref a, ref b) => format!("addl {}, {}", op_to_str(a),
                                     op_to_str(b)),
        Multiply(ref a, ref b) => format!("imull {}, {}", op_to_str(a),
                                          op_to_str(b)),
        Subtract(ref a, ref b) => format!("subl {}, {}", op_to_str(a), op_to_str(b)),
        Divide(ref a) => format!("idivl {}", op_to_str(a)),
        Move(ref a, ref b) => format!("movl {}, {}", op_to_str(a),
                                      op_to_str(b)),
        Push(ref a) => format!("pushl {}", op_to_str(a)),
        Pop(ref a) => format!("popl {}", op_to_str(a)),
        Instruction::Other(ref st) => st.clone(),
        Instruction::OtherStatic(ref st) => st.to_string(),
        Instruction::OtherTwoArg(ref st, ref a, ref b) => {
            format!("{} {}, {}", st, op_to_str(a), op_to_str(b))
        },
        Compare(ref a, ref b) => format!("cmp {}, {}", op_to_str(a),
                                         op_to_str(b)),
        JumpIfEqual(ref a) => format!("je {}", a),
        JumpIfNotEqual(ref a) => format!("jne {}", a),
        Jump(ref a) => format!("jmp {}", a),
        Label(ref l) => format!("{}:", l),
        Comment(ref s) => format!("# {}", s),
        Call(ref name) => format!("call {}", name),
        NOP => "".to_string()
    };

    if !s.is_empty() {
        s.push_str("\n");
    }
    s
}

pub fn instruction_list_to_asm(instructions: &Vec<Instruction>) -> String {
    instructions.iter().fold(String::new(),
                             |acc, ins| acc + &instruction_to_asm(ins))
}
