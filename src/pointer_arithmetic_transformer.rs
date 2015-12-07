use ast;
use ast::AstExpressionNode;
use ast::Block;
use ast::Expression;
use ast::Expression::*;
use ast::Function;
use ast::FunctionCall;
use ast::Program;
use ast::Statement;
use ast::VarType;

use ast_helper::is_pointer;
use type_checker_helper::type_contains;

use std::collections::VecDeque;

struct PointerArithmeticTransformer;

fn multiply_by(left: AstExpressionNode, right: AstExpressionNode,
               typ: &VarType) -> AstExpressionNode {
    assert!(left.typ.is_some() && right.typ.is_some());
    assert!(type_contains(typ, left.typ.as_ref().unwrap()));
    assert!(type_contains(typ, right.typ.as_ref().unwrap()));

    let expr = BinaryOp(ast::BinaryOp::Multiply,
                        Box::new(left), Box::new(right));
    let mut new_expr = AstExpressionNode::new(expr);
    new_expr.typ = Some(typ.clone());
    new_expr
}


fn replace_pointer_arithmetic(mut left: AstExpressionNode,
                              mut right: AstExpressionNode,
                              pointer_type: &VarType)
                              -> (AstExpressionNode, AstExpressionNode) {
    let mut type_size = AstExpressionNode::new(
        SizeOf((*pointer_type).clone()));
    type_size.typ = Some(VarType::Int);
    
    if is_pointer(left.typ.as_ref().unwrap()) {
        right = multiply_by(type_size, right, &VarType::Int);
    } else if is_pointer(right.typ.as_ref().unwrap()) {
        left = multiply_by(type_size, left, &VarType::Int);
    } else {
        panic!("how are neither types a pointer?");
    }

    (left, right)
}


impl PointerArithmeticTransformer {
    pub fn new() -> PointerArithmeticTransformer {
        PointerArithmeticTransformer
    }

    fn transform_call(&self, mut fn_call: FunctionCall) -> FunctionCall {
        let mut new_args = Vec::new();

        while !fn_call.args_exprs.is_empty() {
            let expr = fn_call.args_exprs.remove(0);
            new_args.push(self.transform_expr(expr));
        }

        FunctionCall {
            name: fn_call.name,
            args_exprs: new_args
        }
    }

    fn transform_expr(&self, expr_node: AstExpressionNode) -> AstExpressionNode {
        
        let new_expr = 
        match expr_node.expr {
            BinaryOp(op, l_node, r_node) => {
                assert!(l_node.typ.is_some() && r_node.typ.is_some());
                assert!(expr_node.typ.is_some());
                let binop_type = expr_node.typ.as_ref().unwrap();

                let mut left = self.transform_expr(*l_node);
                let mut right = self.transform_expr(*r_node);

                if let VarType::Pointer(_, ref typ) = *binop_type {
                    if op == ast::BinaryOp::Plus || op == ast::BinaryOp::Minus {
                        let (nl, nr) = replace_pointer_arithmetic(left,
                                                                  right, typ);
                        left = nl;
                        right = nr;
                    }
                }

                Expression::BinaryOp(op.clone(), Box::new(left),
                                     Box::new(right))
            }
            Call(fn_call) => {
                Call(self.transform_call(fn_call))
            }
            Reference(expr) => {
                Reference(Box::new(self.transform_expr(*expr)))
            }
            Dereference(expr) => {
                Dereference(Box::new(self.transform_expr(*expr)))
            }
            FieldAccess(expr, field_name) => {
                FieldAccess(Box::new(self.transform_expr(*expr)),
                            field_name)
            }
            _ => expr_node.expr
        };

        let mut node = AstExpressionNode::new(new_expr);
        node.typ = expr_node.typ.clone();
        node

    }

    // Given a statement, return a list of statements to replace it with
    fn transform_stmt(&self,
                      stmt: Statement) -> Statement {
        
        // FIXME: Do match *&mut stmt instead, to avoid having all the weird returns
        match stmt {
            Statement::Return(expr) => {
                Statement::Return(self.transform_expr(expr))
            }
            Statement::If(expr, mut then_block, else_block_opt) => {
                self.transform_block(&mut then_block);
                let transformed_else_block = 
                if let Some(mut else_block) = else_block_opt {
                    self.transform_block(&mut else_block);
                    Some(else_block)
                } else {
                    None
                };
                Statement::If(self.transform_expr(expr), then_block, transformed_else_block)
            }
            Statement::While(expr, mut block) => {
                self.transform_block(&mut block);
                Statement::While(self.transform_expr(expr), block)
            }
            Statement::Let(name, typ, value_expr) => {
                let new_val_expr = 
                if let Some(expr) = value_expr {
                    Some(self.transform_expr(expr))
                } else {
                    None
                };

                Statement::Let(name, typ, new_val_expr)
            }
            Statement::Assign(left_expr, right_expr) => {
                Statement::Assign(self.transform_expr(left_expr),
                                  self.transform_expr(right_expr))
            }
            Statement::Call(fn_call) => {
                Statement::Call(self.transform_call(fn_call))
            }
            Statement::Print(expr) => {
                Statement::Print(self.transform_expr(expr))
            }
        }
    }

    fn transform_block(&self, block: &mut ast::Block) {
        // Analyze each statement, and replace it with whatever the
        // transform function tells us to
        let mut new_statements = VecDeque::new();
        while let Some(stmt) = block.statements.pop_front() {
            let replacement = self.transform_stmt(stmt);
            new_statements.push_back(replacement);
        }
        block.statements = new_statements;
    }

    fn transform_function(&self, function: &mut Function) {
        self.transform_block(&mut function.statements);
    }

    pub fn transform_program(&self, program: &mut Program) {
        for fun in program.functions.iter_mut() {
            self.transform_function(fun);
        }
    }
}

pub fn transform_pointer_arithmetic(program: &mut Program) {
    let t = PointerArithmeticTransformer::new();
    t.transform_program(program);
}
