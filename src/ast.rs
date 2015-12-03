use std::collections::HashMap;

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
    Struct(String),
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
    // FIXME: Should eventually be an expression, not string
    // For now we'll say this is ok though.
    Dereference(String),
    FieldAccess(Box<AstExpressionNode>, String),
}

// Part of AST. The "typ" field is set when we go to the type checker/annotator
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
    Return(AstExpressionNode),
    Print(AstExpressionNode),
    If(AstExpressionNode, Vec<Statement>),
    While(AstExpressionNode, Vec<Statement>),
    Let(String, VarType, Option<AstExpressionNode>),
    Assign(AstExpressionNode, AstExpressionNode),
    Call(FunctionCall),
}

pub struct Function {
    pub name: String,
    pub statements: Vec<Statement>,

    pub arg: String,

    pub fn_type: FunctionType,
}

#[derive(Debug, Clone)]
pub struct StructDefinition {
    pub name: String,
    pub fields: HashMap<String, VarType>,
}

pub struct Program {
    pub functions: Vec<Function>,
    pub structs: Vec<StructDefinition>,
}
