#[derive(PartialEq, Debug)]
pub enum Operand {
    EAX,
    EBX,
    ECX,
    EBP,
    ESP,
    IntConstant(i32),
    Variable(&'static str),
}

pub enum Instruction {
    Add(Operand, Operand),
    Multiply(Operand, Operand),
    Subtract(Operand, Operand),
    Divide(Operand),
    Move(Operand, Operand),
    Push(Operand),
    Pop(Operand),
    Compare(Operand, Operand),
    JumpIfEqual(String),
    Label(String),
    Other(String),
    Comment(String),
}
