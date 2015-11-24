#[derive(PartialEq, Debug)]
pub enum Operand {
    EAX,
    EBX,
    ECX,
    AL,
    BL,
    CL,
    EBP,
    ESP,
    Dereference(Box<Operand>, i32),
    IntConstant(i32),
    Variable(String),
    VariableStatic(&'static str),
}

pub fn is_register(o: &Operand) -> bool {
    match *o {
        Operand::EAX | Operand::EBX | Operand::ECX => true,
        Operand::AL | Operand::BL | Operand::CL => true,
        Operand::EBP | Operand::ESP => true,
        _ => false
    }
}

pub fn get_low_byte(o: &Operand) -> Operand {
    match *o {
        Operand::EAX => Operand::AL,
        Operand::EBX => Operand::BL,
        Operand::ECX => Operand::CL,
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
}
