#[derive(PartialEq, Debug, Clone)]
pub enum OperatorType {
    Plus,
    Minus,
    Star,
    Divide,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Lexeme {
    IntConstant(i32),
    Operator(OperatorType),
    Return,
}
