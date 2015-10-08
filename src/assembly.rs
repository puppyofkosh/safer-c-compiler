pub enum Value {
    EAX,
    EBX,
    EBP,
    IntConstant(i32),
}

pub enum Instruction {
    Move(Value, Value),
    Push(Value),
    Pop(Value),
    Other(String),
}
