#[derive(Debug)]
pub struct FunctionCall {
    pub name: String,
    pub arg_expr: Box<Expression>,
}

#[derive(Debug, Clone, Copy)]
pub enum VarType {
    Int,
    Char,
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Plus,
    Minus,
    Multiply,
    Divide,
    CompareEqual,
    CompareGreater,
    CompareLess,
    CompareGreaterOrEqual,
    CompareLessOrEqual,
    CompareNotEqual,
}

#[derive(Debug)]
pub enum Expression {
    Value(i32),
    Variable(String),
    StringValue(String),
    BinaryOp(BinaryOp, Box<Expression>, Box<Expression>),
    Call(FunctionCall),
}

#[derive(Debug)]
pub enum Statement {
    Return(Box<Expression>),
    Print(Box<Expression>),
    If(Box<Expression>, Vec<Statement>),
    While(Box<Expression>, Vec<Statement>),
    Let(String, VarType, Box<Expression>),
    Assign(String, Box<Expression>),
    //Call(String, Box<Expression>),
    Call(FunctionCall),
}

pub struct Function {
    pub name: String,
    pub statements: Vec<Statement>,
    pub arg: String,
}
