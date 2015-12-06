#[derive(PartialEq, Debug, Clone, Copy)]
pub enum OperatorType {
    Plus,
    Minus,
    Star,
    Divide,
    CompareEqual,
    CompareGreater,
    CompareLess,
    CompareGreaterOrEqual,
    CompareLessOrEqual,
    CompareNotEqual,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum VarType {
    Int,
    Char,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Lexeme {
    If,
    Else,
    While,
    IntConstant(i32),
    CharConstant(i32),
    Type(VarType),
    Operator(OperatorType),
    Reference,
    Assign,
    Identifier(String),
    StringConstant(String),
    Return,
    Struct,
    Print,
    LParen,
    RParen,
    EndOfStatement, // ;
    StartBlock, // {
    EndBlock,
    Comma,
    Dot,
}
