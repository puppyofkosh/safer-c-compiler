#[derive(PartialEq, Debug)]
pub enum Operand {
    EAX,
    EBX,
    ECX,
    EBP,
    ESP,
    Dereference(Box<Operand>, i32),
    IntConstant(i32),
    Variable(&'static str),
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
    Comment(String),
}
