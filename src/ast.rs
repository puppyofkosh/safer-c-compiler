pub enum BinaryOp {
    Plus,
    Minus,
}

pub enum Expression {
    Value(i32),
    BinaryOp(BinaryOp, i32, i32)
}

pub enum Statement {
    Return(Box<Expression>),
}
