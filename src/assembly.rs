#[derive(PartialEq, Debug)]
pub enum Operand {
    EAX,
    EBX,
    EBP,
    ESP,
    IntConstant(i32),
    Variable(&'static str),
}

pub enum Instruction {
    Add(Operand, Operand),
    Multiply(Operand, Operand),
    Move(Operand, Operand),
    Push(Operand),
    Pop(Operand),
    Other(String),
}
