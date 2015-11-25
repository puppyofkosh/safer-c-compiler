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
}

#[derive(PartialEq, Debug, Clone)]
pub enum Lexeme {
    Call,
    Function,
    If,
    While,
    IntConstant(i32),
    Operator(OperatorType),
    Reference,
    Assign,
    Let,
    Identifier(String),
    StringConstant(String),
    Return,
    Print,
    LParen,
    RParen,
    EndOfStatement, // ;
    StartBlock, // {
    EndBlock,
    Comma,
    IntType,
    CharType,
}
