use std::collections::VecDeque;
use std::collections::HashMap;

#[derive(Debug)]
pub struct FunctionCall {
    pub name: String,
    pub args_exprs: Vec<AstExpressionNode>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PointerType {
    // just a regular pointer, nothing special about it
    Raw,

    // Is freed when it goes out of scope, and can be "moved"
    Owned,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
    Int,
    Char,
    Pointer(PointerType, Box<VarType>),
    Struct(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionType {
    pub return_type: VarType,
    pub arg_types: Vec<VarType>,
    pub is_var_args: bool,
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
    Reference(Box<AstExpressionNode>),
    // FIXME: Should eventually be an expression, not string
    // For now we'll say this is ok though.
    Dereference(String),
    FieldAccess(Box<AstExpressionNode>, String),
}

// Part of AST. The "typ" field is set when we go to the type checker/annotator
#[derive(Debug)]
pub struct AstExpressionNode {
    pub expr: Expression,
    // Before type checking, it's None.
    // If it passes the type checker, it's guaranteed to be Some.
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
    If(AstExpressionNode, Block, Option<Block>),
    While(AstExpressionNode, Block),
    Let(String, VarType, Option<AstExpressionNode>),
    Assign(AstExpressionNode, AstExpressionNode),
    Call(FunctionCall),
}

#[derive(Debug)]
pub struct Block {
    pub statements: VecDeque<Statement>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub statements: Block,

    pub args: Vec<String>,

    pub fn_type: FunctionType,
}

#[derive(Debug, Clone)]
pub struct StructDefinition {
    pub name: String,
    pub fields: HashMap<String, VarType>,
}

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
    pub structs: Vec<StructDefinition>,
}
