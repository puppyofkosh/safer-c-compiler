use ast::AstExpressionNode;
use ast::Block;
use ast::PointerType;
use ast::Expression;
use ast::FunctionType;
use ast::FunctionCall;
use ast::Program;
use ast::Statement;
use ast::VarType::*;
use ast::VarType;
use ast::StructDefinition;
use ast;

use code_block::CodeBlock;

use type_checker_helper;
use type_checker_helper::type_contains;
use type_checker_helper::is_pointer;
use type_checker_helper::is_pointer_arithmetic;
use type_checker_helper::expression_has_address;

use struct_analyzer::StructAnalyzer;

use std::collections::HashMap;

// FIXME:/ TODO:
// Rename some things "annotate_" rather than get_


pub struct TypeChecker {
    errors_found: Vec<String>,
    variable_to_type: HashMap<String, VarType>,
    struct_to_definition: HashMap<String, StructDefinition>,
    blocks: Vec<CodeBlock>,

    current_fn: String,
    function_to_type: HashMap<String, ast::FunctionType>,
}

impl TypeChecker {
    pub fn new() -> TypeChecker {
        let mut t = TypeChecker {
            errors_found: Vec::new(),
            variable_to_type: HashMap::new(),
            struct_to_definition: HashMap::new(),
            blocks: Vec::new(),
            function_to_type: HashMap::new(),
            current_fn: "".to_string(),
        };

        t.function_to_type.insert("printf".to_string(),
                                  FunctionType {
                                      return_type: Int,
                                      arg_types: vec![Pointer(PointerType::Raw,
                                                              Box::new(Char))],
                                      is_var_args: true,
                                  });
        t.function_to_type.insert("scanf".to_string(),
                                  FunctionType {
                                      return_type: Int,
                                      arg_types: vec![Pointer(PointerType::Raw,
                                                              Box::new(Char))],
                                      is_var_args: true,
                                  });
        t.function_to_type.insert("fflush".to_string(),
                                  FunctionType {
                                      return_type: Int,
                                      arg_types: vec![Int],
                                      is_var_args: false
                                  });
        t.function_to_type.insert("alloc_int".to_string(),
                                  FunctionType {
                                      return_type: Pointer(PointerType::Raw,
                                                           Box::new(Int)),
                                      arg_types: vec![Int],
                                      is_var_args: false,
                                  });
        t.function_to_type.insert("free_int".to_string(),
                                  FunctionType {
                                      return_type: Int,
                                      arg_types: vec![Pointer(PointerType::Raw,
                                                              Box::new(Int))],
                                      is_var_args: false,
                                  });
        t.function_to_type.insert("alloc_owned_int".to_string(),
                                  FunctionType {
                                      return_type: Pointer(PointerType::Owned,
                                                           Box::new(Int)),
                                      arg_types: vec![Int],
                                      is_var_args: false,
                                  });
        t.function_to_type.insert("allocate".to_string(),
                                  FunctionType {
                                      return_type: Pointer(PointerType::Raw,
                                                           Box::new(Char)),
                                      arg_types: vec![Int],
                                      is_var_args: false,
                                  });
        t.function_to_type.insert("free".to_string(),
                                  FunctionType {
                                      return_type: Int,
                                      arg_types: vec![Pointer(PointerType::Raw,
                                                              Box::new(Char))],
                                      is_var_args: false,
                                  });

        t
    }

    // Is this a legit type? Only non trivial case is when the
    // type is a struct
    fn type_exists(&self, typ: &VarType) -> bool {
        match *typ {
            Int | Char => true,
            Pointer(_, ref t) => self.type_exists(t),
            Struct(ref struct_name) => {
                self.struct_to_definition.get(struct_name).is_some()
            }
        }
    }

    /// Return None if there exists a type error
    /// Return function's return type otherwise
    fn check_function_call(&mut self,
                           call: &mut FunctionCall) -> Option<VarType> {

        // Make sure the function exists
        let fn_type_opt = self.function_to_type.get(&call.name).cloned();
        if fn_type_opt.is_none() {
           self.errors_found.push(format!("Unkown function {}",
                                          call.name));
           return None
        }
        let fn_type = fn_type_opt.unwrap();

        let call_len = call.args_exprs.len();
        let definition_len = fn_type.arg_types.len();
        if (call_len != definition_len && !fn_type.is_var_args)
            || call_len < definition_len {
                self.errors_found.push(format!("function call's parameter \
                                                num({}) doesn't match with \
                                                the definition ({})",
                                               call_len, definition_len));
                return None
        }

        // Make sure the type of the argument makes sense
        for i in 0..call.args_exprs.len() {
            let arg_type_opt = self.annotate_type(call.args_exprs.get_mut(i).unwrap());
            if arg_type_opt.is_none() { return None; }
            let arg_type = arg_type_opt.unwrap();

            // Var args functions may have more arguments than the definition
            if i < definition_len {
                let param_type = fn_type.arg_types.get(i).unwrap();
                // HACK: this is how we let free get called with any type
                if !type_contains(param_type, &arg_type) && call.name != "free" {
                    let err = format!("Expected type {:?} but got type {:?}",
                                      param_type, arg_type);
                    self.errors_found.push(err);
                    return None;
                }
            } else {
                assert_eq!(fn_type.is_var_args, true);
            }
        }


        Some(fn_type.return_type.clone())
    }

    fn get_var_type_or_report(&mut self, name: &str) -> Option<&VarType> {
        let res = self.variable_to_type.get(name);
        if res == None {
            self.errors_found.push(format!("Unkown variable {}", name));
        }
        res
    }

    // If name is a variable of type Pointer(Int), we return Int.
    // FIXME: Fix this function, it is terrible
    fn get_type_pointed_to_or_report(&mut self, name: &str) -> Option<VarType> {
        let mut is_ptr = false;
        let res =
        {
            let var_type_opt = self.get_var_type_or_report(name);
            if var_type_opt.is_none() {
                None
            } else if let Some(&Pointer(_, ref t)) = var_type_opt {
                is_ptr = true;
                Some((**t).clone())
            } else {
                None
            }
        };

        if !is_ptr {
            self.errors_found
                .push(format!("Cannot dereference non pointer, {}",
                              name));
        }
        res
    }

    fn get_binary_op_expr_type(&mut self,
                               op: &ast::BinaryOp, l: &mut AstExpressionNode,
                               r: &mut AstExpressionNode) -> Option<VarType> {
        let l_type_opt = self.annotate_type(l);
        let r_type_opt = self.annotate_type(r);

        if l_type_opt.is_none() || r_type_opt.is_none() {
            return None
        }

        let l_type = l_type_opt.unwrap();
        let r_type = r_type_opt.unwrap();

        if is_pointer_arithmetic(&l_type, &r_type, *op) {
            if is_pointer(&l_type) {
                return Some(l_type);
            } else {
                return Some(r_type);
            }
        }

        if type_contains(&Int, &l_type) &&
            type_contains(&Int, &r_type) {
                // Adding two ints or two chars
                if l_type == r_type && l_type == Char {
                    Some(Char)
                } else {
                    Some(Int)
                }
            } else {
                self.errors_found.push(format!(
                    "Cannot do operation {:?} on types {:?} and {:?}",
                    op, l_type, r_type));
                None
            }
    }

    /// Set the type of expression node
    fn annotate_type(&mut self,
                     expr_node: &mut AstExpressionNode) -> Option<VarType> {
        let expr = &mut expr_node.expr;
        let typ =
        match *expr {
            Expression::Value(v) if v >= 0 && v < 256 => Some(Char),
            Expression::Value(_) => Some(Int),
            Expression::Variable(ref name) => {
                self.get_var_type_or_report(name).cloned()
            }
            Expression::StringValue(_) => Some(Pointer(PointerType::Raw,
                                                       Box::new(Char))),
            Expression::BinaryOp(ref op, ref mut l, ref mut r) => {
                self.get_binary_op_expr_type(op, l, r)
            }
            Expression::Call(ref mut fn_call) => {
                self.check_function_call(fn_call)
            }
            Expression::Reference(ref mut expr) => {
                let expr_type = self.annotate_type(expr);
                if let Some(t) = expr_type {
                    if expression_has_address(expr) {
                        Some(Pointer(PointerType::Raw, Box::new(t)))
                    } else {
                        self.errors_found.push(format!(
                            "Cannot reference expression \
                             {:?} (with type {:?})",
                            expr, t));
                        None
                    }
                } else {
                    None
                }
            }
            Expression::Dereference(ref name) => {
                self.get_type_pointed_to_or_report(name)
            }
            Expression::FieldAccess(ref mut expr, ref field_name) => {
                let expr_t = self.annotate_type(expr);

                if expr_t.is_none() {
                    None
                } else if let Some(VarType::Struct(ref struct_name)) = expr_t {
                    // return type of the field
                    let struct_defn = self.struct_to_definition
                        .get(struct_name)
                        .expect(&format!("Struct {} unkown!", struct_name));

                    let field_type = struct_defn.fields.get(field_name);
                    if field_type.is_none() {
                        self.errors_found.push(
                            format!("Unkown field {} on struct {}",
                                    field_name, struct_name));
                    }
                    field_type.cloned()
                } else {
                    self.errors_found.push(format!(
                        "Cannot access field {:?}(type {:?}) of {:?}",
                        field_name, expr_t, expr));
                    None
                }
            }
        };
        expr_node.typ = typ;
        expr_node.typ.clone()
    }

    fn annotate_types_stmt(&mut self, stmt: &mut Statement) -> bool {
        match *stmt {
            Statement::Return(ref mut expr) => {
                let expr_type = self.annotate_type(expr);
                let ret_type = &self.function_to_type
                    .get(&self.current_fn)
                    .unwrap()
                    .return_type;

                let mut res = true;
                if let Some(expr_t) = expr_type {
                    if !type_contains(ret_type, &expr_t) {
                        let msg = format!("Cannot return expr {:?} \
                                           for function with ret \
                                           type {:?}", expr_t,
                                          ret_type);
                        self.errors_found.push(msg);
                        res = false;
                    }
                } else {
                    res = false;
                }
                res
            }
            Statement::Print(ref mut expr) => {
                let typ = self.annotate_type(expr);
                let res = typ.is_some() && type_contains(&Int, typ.as_ref().unwrap());
                if !res {
                    self.errors_found.push(
                        format!("Cannot print something of type {:?}", typ));
                }
                res
            }
            Statement::If(ref mut expr, ref mut stmts, ref mut else_opt) => {
                let expr_type = self.annotate_type(expr);
                if let &mut Some(ref mut else_stmts) = else_opt {
                    // TODO: add else checker
                    self.annotate_types_block(else_stmts) && self.annotate_types_block(stmts) && expr_type.is_some()
                } else {
                    self.annotate_types_block(stmts) && expr_type.is_some()
                }
            }
            Statement::While(ref mut expr, ref mut stmts) => {
                let expr_type = self.annotate_type(expr);
                self.annotate_types_block(stmts) && expr_type.is_some()
            }
            Statement::Let(ref name, ref var_type, ref mut expr_opt) => {
                let mut res = true;
                if !self.type_exists(var_type) {
                    self.errors_found.push(format!("Type {:?} doesn't exist.",
                                                   var_type));
                    res = false;
                }

                if let &mut Some(ref mut expr) = expr_opt {
                    self.annotate_type(expr);

                    if !type_checker_helper::can_assign_expr_to_type(var_type,
                                                                     expr) {
                        self.errors_found.push(
                            format!("Cant assign expr {:?} of type ({:?}) \
                                     to var of type {:?}",
                                    expr.expr, expr.typ, var_type));
                        res = false;
                    }
                } else {
                    // There was no initialization expression, which is fine.
                }

                if res == true {
                    self.blocks.last_mut()
                        .unwrap()
                        .declared_variables
                        .insert(name.clone());
                    self.variable_to_type.insert(name.clone(),
                                                 var_type.clone());
                }

                res
            }
            Statement::Assign(ref mut left, ref mut right) => {
                self.annotate_type(left);
                self.annotate_type(right);

                let res = type_checker_helper::is_assignment_valid(left, right);

                if !res {
                    self.errors_found.push(format!("Cannot assign {:?} to {:?}",
                                                   right, left));
                }
                res
            }
            Statement::Call(ref mut call) => {
                self.check_function_call(call).is_some()
            }
        }
    }

    fn annotate_types_block(&mut self, block: &mut Block) -> bool {
        self.blocks.push(CodeBlock::new());
        let mut res = true;
        for stmt in block.statements.iter_mut() {
            if !self.annotate_types_stmt(stmt) {
                res = false;
            }
        }

        let b = self.blocks.pop().unwrap();
        for variable in b.declared_variables {
            self.variable_to_type.remove(&variable);
        }

        res
    }

    fn add_structs(&mut self, structs: &Vec<StructDefinition>) {
        for struct_defn in structs {
            self.struct_to_definition.insert(struct_defn.name.clone(),
                                             struct_defn.clone());
        }
    }

    pub fn annotate_types(&mut self, program: &mut Program) -> bool {
        let mut struct_analyzer = StructAnalyzer::new();
        if !struct_analyzer.check_structs(&program.structs) {
            self.errors_found.extend(struct_analyzer.get_errors());
            return false;
        } else {
            self.add_structs(&program.structs);
        }

        let mut res = true;
        for fun in program.functions.iter_mut() {
            // For now we cannot pass or return structs from functions.
            for arg_type in &fun.fn_type.arg_types {
                if let &Struct(_) = arg_type {
                    self.errors_found.push("Cannot pass structs yet".to_string());
                    return false;
                }
            }
            if let Struct(_) = fun.fn_type.return_type {
                self.errors_found.push("Cannot pass structs yet".to_string());
                return false;
            }

            // Add the current function to our table of functions
            // (note we do this before checking the function body)
            self.function_to_type.insert(fun.name.clone(),
                                         fun.fn_type.clone());
            self.current_fn = fun.name.clone();
            for i in 0..fun.args.len() {
                self.variable_to_type.insert(fun.args.get(i).unwrap().clone(),
                                             fun.fn_type.arg_types.get(i).unwrap().clone());
            }

            if !self.annotate_types_block(&mut fun.statements) {
                // Don't return here, because we should type check the other
                // functions too.
                res = false;
            }

            for arg in &fun.args {
                self.variable_to_type.remove(arg);
            }
            assert!(self.variable_to_type.is_empty());
        }

        res
    }

    pub fn get_errors(&self) -> &Vec<String> {
        &self.errors_found
    }
}
