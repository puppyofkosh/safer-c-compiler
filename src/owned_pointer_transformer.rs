use ast;
use ast::AstExpressionNode;
use ast::Block;
use ast::PointerType;
use ast::Expression;
use ast::Function;
use ast::FunctionCall;
use ast::Program;
use ast::Statement;
use ast::VarType;

use code_block::CodeBlock;

use std::collections::VecDeque;
use std::collections::HashMap;

// This function checks that the "owned_pointers"
// are "moved."
// We also insert calls to free() here to destroy the owned_pointers
// who go out of scope/before "return"
// This must be done after type checking

// Problems with this so far:
// Need to disallow stuff like *&*&my_owned_pointer
// Need way of destroying structs who have owned_pointers inside them
// Right now we assume we cannot return owned_pointers
//
// TODO: Rerun the typechecker again after this runs,
// that way we won't have to add in the types which feels hackish

#[derive(Clone, Copy, PartialEq)]
enum OwnedPointerStatus {
    // It stores a pointer to something, and we want to free
    // it at the end
    Alive,

    // This pointer has been moved, so it can never be used again
    Moved,
}

struct OwnedPointerInfo {
    status: OwnedPointerStatus,
    target_type: VarType,
}

struct OwnedPointerTransformer {
    blocks: Vec<CodeBlock>,
    pointer_to_info: HashMap<String, OwnedPointerInfo>,
}

fn get_free_call(varname: &str, typ: &VarType) -> Statement {
    let mut arg = AstExpressionNode::new(
        Expression::Variable(varname.to_string()));
    // FIXME: This feels weird, setting the type manually
    // Instead we should try to re-run the type checker after
    // this transformation, and it'll do that for us.
    arg.typ = Some(typ.clone());

    let fn_call = FunctionCall {name: "free".to_string(),
                                args_exprs: vec![arg]
    };
    Statement::Call(fn_call)
}

impl OwnedPointerTransformer {
    pub fn new() -> OwnedPointerTransformer {
        OwnedPointerTransformer {
            blocks: Vec::new(),
            pointer_to_info: HashMap::new(),
        }
    }

    // Given a statement, return a list of statements to replace it with
    fn transform_stmt(&mut self,
                      stmt: Statement) -> Vec<Statement> {
        
        // FIXME: Do match *&mut stmt instead, to avoid having all the weird returns
        match stmt {
            Statement::Return(expr) => {
                // TODO: Right now we assume no owned_pointer is returned
                let mut res = Vec::new();

                // We're about to return, so destroy all alive owned pointers
                for (name, info) in self.pointer_to_info.iter() {
                    if info.status == OwnedPointerStatus::Moved {
                        continue;
                    }

                    res.push(get_free_call(name, &info.target_type));
                }

                res.push(Statement::Return(expr));
                res
            }
            Statement::If(expr, then_block, else_block_opt) => {
                vec![Statement::If(expr, then_block, else_block_opt)]
            }
            Statement::While(expr, block) => {
                vec![Statement::While(expr, block)]
            }
            Statement::Let(name, typ, value_expr) => {

                if let VarType::Pointer(PointerType::Owned,
                                        ref to_typ) = *&typ {
                    // Keep track of which block this owned_pointer was
                    // declared in
                    self.blocks.last_mut()
                        .expect("No current block!")
                        .declared_variables.insert(name.clone());

                    // TODO deal with case where we're moving another
                    // shared pointer
                    let info = OwnedPointerInfo {
                        status: OwnedPointerStatus::Alive,
                        target_type: *(to_typ.clone()),
                    };
                    self.pointer_to_info.insert(name.clone(), info);
                }
                vec![Statement::Let(name, typ, value_expr)]
            }
            Statement::Assign(left_expr, right_expr) => {
                vec![Statement::Assign(left_expr, right_expr)]
            }
            Statement::Call(fn_call) => {
                vec![Statement::Call(fn_call)]
            }
            Statement::Print(expr) => {
                vec![Statement::Print(expr)]
            }
        }
    }

    // Free all the pointers who were declared in the current block
    // Add the free instructions to the end of the block.
    fn free_pointers_in_cur_block(&mut self, block: &mut Block) {
        // If the last statement is a return statement, we don't
        // need to add free calls after it
        if let Some(&Statement::Return(_)) = block.statements.back() {
            return;
        }

        // Else free everything declared in that block
        let b = self.blocks.last().expect("No current block!");
        for varname in b.declared_variables.iter() {
            let var_info = self.pointer_to_info.get(varname)
                .expect("var not in pointer_to_info");

            let call = get_free_call(&varname, &var_info.target_type);
            block.statements.push_back(call);
                                               
        }
    }

    fn transform_block(&mut self, block: &mut ast::Block) {
        // Clear some data structures here probably
        self.blocks.push(CodeBlock::new());
 
        // Analyze each statement, and replace it with whatever the
        // transform function tells us to
        let mut new_statements = VecDeque::new();
        while let Some(stmt) = block.statements.pop_front() {
            let replacement = self.transform_stmt(stmt);
            new_statements.extend(replacement);
        }
        block.statements = new_statements;

        // If the last statement of the block is not a return, then we need
        // to free all of the owned_pointers in that block
        self.free_pointers_in_cur_block(block);

        let b = self.blocks.pop().expect("No current block!");
        for variable in b.declared_variables {
            self.pointer_to_info.remove(&variable);
        }
    }

    fn transform_function(&mut self, function: &mut Function) {
        self.transform_block(&mut function.statements);
    }

    pub fn transform_program(&mut self, program: &mut Program) {
        for fun in program.functions.iter_mut() {
            self.transform_function(fun);
        }
    }
}

pub fn transform_program(program: &mut Program) {
    let mut t = OwnedPointerTransformer::new();
    t.transform_program(program);
}
