#[derive(PartialEq, Debug)]
pub enum Operand {
    EAX,
    EBX,
    EBP,
    IntConstant(i32),
}

pub enum Instruction {
    Add(Operand, Operand),
    Multiply(Operand, Operand),
    Move(Operand, Operand),
    Push(Operand),
    Pop(Operand),
    Other(String),
}
