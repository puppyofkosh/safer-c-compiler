#[derive(PartialEq, Debug)]
pub enum Lexeme {
    IntConstant(i32),
    Operator(String),
    Return,
}
