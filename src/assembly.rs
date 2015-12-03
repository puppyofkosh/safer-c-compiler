#[derive(PartialEq, Debug, Clone, Copy)]
pub enum RegisterVal {
    EAX, EBX, ECX,
    AL, BL, CL,
    ESP, EBP
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MachineType {
    Long,
    Byte,
    Chunk(i32), // When something is a chunk of memory of arbitrary size
}

#[derive(PartialEq, Debug, Clone)]
pub enum Operand {
    Register(RegisterVal),
    Dereference(RegisterVal, i32),
    IntConstant(i32),
    Variable(String),
    VariableStatic(&'static str),
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
