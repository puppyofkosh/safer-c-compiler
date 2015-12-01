#[derive(Debug)]
pub struct FunctionCall {
    pub name: String,
    pub arg_expr: Box<AstExpressionNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
    Int,
    Char,
    Pointer(Box<VarType>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionType {
    pub return_type: VarType,
    pub arg_types: Vec<VarType>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
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
    BinaryOp(BinaryOp, Box<AstExpressionNode>, Box<AstExpressionNode>),
    Call(FunctionCall),
    Reference(String),
    Dereference(String),
}

#[derive(Debug)]
pub struct AstExpressionNode {
    pub expr: Expression,
    pub typ: Option<VarType>,
}

impl AstExpressionNode {
    pub fn new(ex: Expression) -> AstExpressionNode {
        AstExpressionNode {
            expr: ex,
            typ: None
        }
    }
}


#[derive(Debug)]
pub enum Statement {
    Return(Box<AstExpressionNode>),
    Print(Box<AstExpressionNode>),
    If(Box<AstExpressionNode>, Vec<Statement>),
    While(Box<AstExpressionNode>, Vec<Statement>),
    Let(String, VarType, Box<AstExpressionNode>),
    Assign(String, Box<AstExpressionNode>),
    AssignToDereference(String, Box<AstExpressionNode>),
    Call(FunctionCall),
}

pub struct Function {
    pub name: String,
    pub statements: Vec<Statement>,

    pub arg: String,

    pub fn_type: FunctionType,
}

pub struct Program {
    pub functions: Vec<Function>,
}
