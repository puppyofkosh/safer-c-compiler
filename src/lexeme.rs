#[derive(PartialEq, Debug, Clone)]
pub enum Lexeme {
    IntConstant(i32),
    Operator(String),
//    AddOperator,
//    MinusOperator,
//    StarOperator,
//    DivideOperator,
    Return,
}
