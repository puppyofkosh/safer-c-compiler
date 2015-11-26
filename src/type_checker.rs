use ast::Expression;
use ast::Function;
use ast::Program;
use ast::Statement;
use ast::VarType::*;
use ast::VarType;
use ast;
use code_block::CodeBlock;

use std::collections::HashMap;

pub struct TypeChecker {
    errors_found: Vec<String>,
    variable_to_type: HashMap<String, VarType>,
    blocks: Vec<CodeBlock>,

    function_to_type: HashMap<String, ast::FunctionType>,
}

impl TypeChecker {
    pub fn new() -> TypeChecker {
        TypeChecker {
            errors_found: Vec::new(),
            variable_to_type: HashMap::new(),
            blocks: Vec::new(),
            function_to_type: HashMap::new(),
        }
    }

    fn get_var_type_or_report(&mut self, name: &str) -> Option<&VarType> {
        let res = self.variable_to_type.get(name);
        if res == None {
            self.errors_found.push(format!("Unkown variable {}", name));
        }
        res
    }

    fn get_type(&mut self, expr: &Expression) -> Option<VarType> {
        match *expr {
            Expression::Value(_) => Some(Int),
            Expression::Variable(ref name) => {
                self.get_var_type_or_report(name).cloned()
            }
            Expression::StringValue(_) => Some(Pointer(Box::new(Char))),
            Expression::BinaryOp(ref op, ref l, ref r) => {
                // TODO: Actually write this
                // If we add an int to a pointer, we get a pointer
                // we cannot multiply ints and pointers though
                let l_type = self.get_type(l);
                let r_type = self.get_type(r);

                let are_equal = l_type == r_type;
                let is_one_int = l_type == Some(Int) || r_type == Some(Int);
                let is_one_char = l_type == Some(Char) || r_type == Some(Char);

                if are_equal && (is_one_int || is_one_char) {
                    l_type
                } else if is_one_char && is_one_int {
                    Some(Int)
                } else {
                    self.errors_found.push(format!(
                        "Cannot do operation {:?} on types {:?} and {:?}",
                        op, l_type, r_type));
                    None
                }
            }
            Expression::Call(ref fn_call) => {
                None
            }
            Expression::Reference(_) => {
                None
            }
            Expression::Dereference(_) => {
                None
            }
        }
    }

    fn check_types_stmt(&mut self, stmt: &Statement) -> bool {
        match *stmt {
            Statement::Return(ref expr) => {
                // TODO: Check type of expression is same as return
                // type of current function
                true
            }
            Statement::Print(ref expr) => {
                self.get_type(expr) == Some(Int)
            }
            Statement::If(ref expr, ref stmts) => {
                let expr_type = self.get_type(expr);
                self.check_types_block(stmts) && expr_type.is_some()
            }
            Statement::While(ref expr, ref stmts) => {
                false
            }
            Statement::Let(ref name, ref expr_type, ref expr) => {
                let res = self.get_type(expr).as_ref() == Some(expr_type);
                if res {
                    self.variable_to_type.insert(name.clone(), expr_type.clone());
                }
                res
            }
            Statement::Assign(ref name, ref expr) => {
                let t = self.get_type(expr);
                let expected = self.get_var_type_or_report(name);

                t.as_ref() == expected
            }
            Statement::AssignToDereference(ref name, ref expr) => {
                false
            }
            Statement::Call(ref call) => {
                false
            }
        }
    }

    fn check_types_block(&mut self, stmts: &Vec<Statement>) -> bool {
        self.blocks.push(CodeBlock::new());
        let mut res = true;
        for stmt in stmts {
            if !self.check_types_stmt(&stmt) {
                res = false;
            }
        }

        let b = self.blocks.pop().unwrap();
        for variable in b.declared_variables {
            self.variable_to_type.remove(&variable);
        }

        res
    }

    pub fn check_types(&mut self, program: &Program) -> bool {
        let mut res = true;
        for fun in &program.functions {
            self.function_to_type.insert(fun.name.clone(),
                                         fun.fn_type.clone());

            if !self.check_types_block(&fun.statements) {
                println!("Did not pass type checker!");
                res = false;
            }
        }
        println!("Type checker done!");
        res
    }
}
