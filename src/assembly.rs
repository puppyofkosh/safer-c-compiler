#[derive(PartialEq, Debug, Clone, Copy)]
pub enum RegisterVal {
    EAX, EBX, ECX,
    AL, BL, CL,
    ESP, EBP
}


#[derive(PartialEq, Debug, Clone)]
pub enum Operand {
    Register(RegisterVal),
    Dereference(RegisterVal, i32),
    IntConstant(i32),
    Variable(String),
    VariableStatic(&'static str),
}

pub fn get_low_byte(o: &RegisterVal) -> RegisterVal {
    match *o {
        RegisterVal::EAX => RegisterVal::AL,
        RegisterVal::EBX => RegisterVal::BL,
        RegisterVal::ECX => RegisterVal::CL,
        _ => panic!("Register doesn't have low byte"),
    }
}

pub enum Instruction {
    Add(Operand, Operand),
    Call(String),
    Multiply(Operand, Operand),
    Subtract(Operand, Operand),
    Divide(Operand),
    Move(Operand, Operand),
    Push(Operand),
    Pop(Operand),
    Compare(Operand, Operand),
    JumpIfEqual(String),
    JumpIfNotEqual(String),
    Jump(String),
    Label(String),
    Other(String),
    OtherStatic(&'static str),
    OtherTwoArg(&'static str, Operand, Operand),
    Comment(String),
    NOP, // No-op. Do nothing
}
