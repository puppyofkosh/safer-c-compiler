#[derive(Debug)]
pub enum BinaryOp {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub enum Expression {
    Value(i32),
    BinaryOp(BinaryOp, Box<Expression>, Box<Expression>)
}

#[derive(Debug)]
pub enum Statement {
    Return(Box<Expression>),
    Print(Box<Expression>),
    If(Box<Expression>, Vec<Statement>),
}
