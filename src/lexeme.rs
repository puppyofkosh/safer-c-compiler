#[derive(PartialEq, Debug, Clone)]
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
    Assign,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Lexeme {
    If,
    IntConstant(i32),
    Operator(OperatorType),
    Let,
    Identifier(String),
    Return,
    Print,
    LParen,
    RParen,
    EndOfStatement, // ;
    StartBlock, // {
    EndBlock,
}
