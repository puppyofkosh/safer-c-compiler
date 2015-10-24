#[derive(PartialEq, Debug, Clone)]
pub enum OperatorType {
    Plus,
    Minus,
    Star,
    Divide,
    CompareEqual,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Lexeme {
    If,
    IntConstant(i32),
    Operator(OperatorType),
    Return,
    Print,
    LParen,
    RParen,
    EndOfStatement, // ;
    StartBlock, // {
    EndBlock,
}
