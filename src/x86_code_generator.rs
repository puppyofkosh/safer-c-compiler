use ast;
use ast::Statement;
use ast::Expression;
use ast::AstExpressionNode;
use ast::BinaryOp;
use ast::Function;
use ast::FunctionCall;
use ast::VarType;
use ast::Program;

use assembly::Instruction;
use assembly::Instruction::*;
use assembly::Operand;
use assembly::Operand::*;
use assembly::RegisterVal::*;
use assembly::RegisterVal;
use assembly::MachineType;

use assembly_printer::instruction_list_to_asm;

use code_generator::GeneratesCode;

use code_block::CodeBlock;

use std::collections::HashMap;

use assembly_helper::alloc_stack;
use assembly_helper::free_stack;
use assembly_helper::get_mtype_size;
use assembly_helper::register_other_than;
use assembly_helper::move_type;
use assembly_helper::WORD_SIZE;

use representation_manager::RepresentationManager;


#[derive(Clone)]
struct LocalVariable {
    stack_offset: i32,
    // We need thisv for pointer dereferencing (need to know size of the thing
    // to dereference)
    var_type: VarType,

    // Used mainly for when we have to copy things and
    // knowing which asm instruction to use
    machine_type: MachineType,
}

impl LocalVariable {
    pub fn new(off: i32, var_type: VarType, machine_type: MachineType) -> LocalVariable {
        LocalVariable {
            stack_offset: off,
            var_type: var_type,
            machine_type: machine_type,
        }
    }
}

pub struct X86CodeGenerator {
    label_num: i32,

    // keep track of where in memory variables are stored
    identifier_to_var: HashMap<String, LocalVariable>,
    blocks: Vec<CodeBlock>,
    current_stack_offset: i32,
    current_function: String,

    // string
    string_to_label: HashMap<String, String>,
    current_label_num: i32,

    instructions: Vec<Instruction>,

    representation_mgr: RepresentationManager,
}



impl X86CodeGenerator {
    pub fn new() -> X86CodeGenerator {
        X86CodeGenerator {
            label_num: 0,
            identifier_to_var: HashMap::new(),
            blocks: Vec::new(),
            current_function: String::new(),

            current_stack_offset: 0,
            string_to_label: HashMap::new(),
            current_label_num: 0,

            instructions: Vec::new(),

            representation_mgr: RepresentationManager::new(),
        }
    }

    // Move to a register. Doesn't matter which one.
    fn move_op_to_register(&mut self, op: Operand) -> RegisterVal {
        if let Register(reg) = op {
            reg
        } else {
            // TODO: some clever register allocation someday
            self.instructions.push(Move(op, Register(EAX)));
            EAX
        }
    }

    fn move_var_to_register(&mut self,
                            varname: &str, reg: Operand) {
        let var = self.identifier_to_var
            .get(varname)
            .expect(&format!("Unkown variable {}", varname));

        let from_op = Dereference(EBP, var.stack_offset);
        let instr = move_type(from_op, reg, var.machine_type);
        self.instructions.push(instr);
    }

    fn move_value_to_var(&mut self, reg: Operand,
                         varname: &str) {
        let var = self.identifier_to_var
            .get(varname)
            .expect(&format!("Unkown variable {}", varname));

        let to_operand = Dereference(EBP, var.stack_offset);
        let instr = move_type(reg, to_operand, var.machine_type);
        self.instructions.push(instr);
    }

    // Return a (register, offset) to write to assign to this expression
    // Assumes the expression is "assignable," which the type checker
    // checks.
    // Example:
    // x = y --> return address of x (register=EBP, off=offset)
    // *x = y --> move x into a register, and return that
    fn load_address_of_expr(&mut self,
                            expr: &AstExpressionNode) -> (RegisterVal, i32) {
        match expr.expr {
            Expression::Variable(ref name) => {
                let var = self.identifier_to_var.get(name).unwrap();
                (EBP, var.stack_offset)
            }
            Expression::Dereference(ref expr) => {
                let expr_op = self.evaluate_expression(expr);

                let reg = self.move_op_to_register(expr_op);
                (reg, 0)
            }
            Expression::FieldAccess(ref expr, ref field_name) => {
                let (addr_reg,
                     struct_off) = self.load_address_of_expr(expr);

                // field_off is the offset of where the field we want is
                let field_info = self.representation_mgr
                    .get_field_info(expr.typ.as_ref().unwrap(),
                                    field_name);
                // Address of the field is at the offset of the struct,
                // plus the offset of the field within the struct
                let total_offset = struct_off + field_info.offset;
                (addr_reg, total_offset)
            }
            _ => panic!("Cannot assign to this type of expr"),
        }
    }

    fn evaluate_block(&mut self, block: &ast::Block) {
        self.blocks.push(CodeBlock::new());
        for stmt in block.statements.iter() {
            self.evaluate_statement(stmt);
        }

        // Wipe out all of the variables we declared in this block, as
        // we shouldn't able to use them again
        let block_opt = self.blocks.pop();
        if block_opt.is_none() {
            panic!("Invalid state. Why is there no current block?");
        }
        let block = block_opt.unwrap();

        // Pretty reasonable assumption
        assert!(block.declared_variables.len() < i32::max_value() as usize);
        let previous_offset = self.current_stack_offset;
        for variable in block.declared_variables {
            let var = self.identifier_to_var
                .remove(&variable)
                .unwrap();
            self.current_stack_offset += get_mtype_size(var.machine_type);
        }

        self.instructions.push(free_stack(self.current_stack_offset -
                                          previous_offset));
    }

    // Generate code to evaluate an expression and return the operand where
    // the result is stored
    fn evaluate_expression(&mut self,
                           expr_node: &AstExpressionNode) -> Operand {
        let expr = &expr_node.expr;
        match *expr {
            Expression::Call(ref fn_call) => {
                self.evaluate_function_call(fn_call);
                Register(EAX)
            }
            Expression::Value(ref v) => {
                // FIXME: We should probably use more than just the register
                // EAX...
                self.instructions.push(Move(IntConstant(*v), Register(EAX)));
                Register(EAX)
            }
            Expression::StringValue(ref v) => {
                if self.string_to_label.contains_key(v) {
                    let label = self.string_to_label.get(v).unwrap();
                    self.instructions.push(Move(Variable(label.clone()),
                                                Register(EAX)));
                } else {
                    let label = format!(".LC{}", self.current_label_num);
                    self.current_label_num += 1;
                    self.string_to_label.insert(v.clone(), label.clone());
                    self.instructions.push(Move(Variable(label.clone()),
                                                Register(EAX)));
                }
                Register(EAX)
            }
            Expression::BinaryOp(ref op, ref l, ref r) => {
                self.evaluate_binary_op(op, l, r)
            }
            Expression::Variable(ref name) => {
                self.move_var_to_register(name, Register(EAX));
                Register(EAX)
            }
            Expression::Reference(ref expr) => {
                let (reg, offset) = self.load_address_of_expr(expr);
                self.instructions.push(OtherTwoArg("leal",
                                                   Dereference(reg, offset),
                                                   Register(EAX)));

                Register(EAX)
            }
            Expression::Dereference(ref expr) => {
                let addr_op = self.evaluate_expression(expr);
                let addr_reg = self.move_op_to_register(addr_op);

                let typ = expr.typ
                    .as_ref()
                    .expect("Expressions should all have types now!");
                let instr = if let VarType::Pointer(_, ref t) = *typ {
                    move_type(Dereference(addr_reg, 0),
                              Register(EAX),
                              self.representation_mgr.get_machine_type(t))
                } else {
                    panic!("Cannot dereference non pointer")
                };

                // The address of the thing we want to dereference is in EAX

                self.instructions.push(instr);
                Register(EAX)
            }
            Expression::FieldAccess(ref struct_expr, ref field_name) => {
                // Load the address of this whole expression
                let (register, offset) = self.load_address_of_expr(expr_node);
                let result_reg = Register(EAX);
                let field_info = self.representation_mgr
                    .get_field_info(struct_expr.typ.as_ref().unwrap(),
                                    field_name);

                let instr = move_type(Dereference(register, offset),
                                      result_reg.clone(),
                                      field_info.machine_type);
                self.instructions.push(instr);
                result_reg
            }
        }
    }

    fn evaluate_return_statement(&mut self, value: &AstExpressionNode) {
        let out_reg = self.evaluate_expression(value);
        let instr = &mut self.instructions;
        // For now everything goes into eax
        if out_reg != Register(EAX) {
            instr.push(Move(out_reg, Register(EAX)));
        }

        instr.push(Move(Register(EBP), Register(ESP)));
        instr.push(Pop(Register(EBP)));
        // if self.current_function == "_start" {
        //     instr.push(Move(Register(EAX), Register(EBX)));
        //     instr.push(Move(IntConstant(1), Register(EAX)));
        //     instr.push(Instruction::OtherStatic("int $0x80"));
        // }
        // else {
        //instr.push(Instruction::OtherStatic("ret"));
        //}
        instr.push(Instruction::OtherStatic("ret"));
    }

    fn evaluate_statement(&mut self,
                          tree: &Statement) {
        match *tree {
            Statement::Return(ref v) => {
                self.evaluate_return_statement(v);
            }
            Statement::Print(ref expr) => {
                self.instructions.push(Comment("Evaluating print statement"
                                               .to_string()));
                let result_reg = self.evaluate_expression(&expr);

                let instr = &mut self.instructions;
                instr.push(Push(result_reg));
                instr.push(Push(VariableStatic("decimal_format_str")));
                instr.push(Instruction::Other("call printf".to_string()));
                // pop args off the stack
                instr.push(free_stack(WORD_SIZE*2));

                // Call fflush(0)
                instr.push(Push(IntConstant(0)));
                instr.push(Instruction::Other("call fflush".to_string()));
                instr.push(free_stack(WORD_SIZE));
            }
            Statement::If(ref expr, ref then_block, ref else_block_opt) => {
                let reg = self.evaluate_expression(&expr);

                let label = format!("L{}", self.label_num);
                self.label_num += 1;
                self.instructions.push(Compare(IntConstant(0), reg));
                // Jump PAST the "then statement" if the expression is false
                self.instructions.push(JumpIfEqual(label.to_string()));

                self.instructions.push(Comment("The start of if block".to_string()));
                self.evaluate_block(then_block);

                if else_block_opt.is_some() {
                    self.instructions.push(Comment("The end of if block, skipping the else block".to_string()));
                    let label = format!("L{}", self.label_num);
                    self.instructions.push(Jump(label.to_string()));
                }
                // print the label to jump to if the expr is false
                self.instructions.push(Instruction::Label(label.to_string()));

                if let &Some(ref else_statements) = else_block_opt {
                    self.instructions.push(Comment("The start of else block".to_string()));
                    self.evaluate_block(else_statements);
                    let label = format!("L{}", self.label_num);
                    self.label_num += 1;
                    self.instructions.push(Instruction::Label(label.to_string()));
                }
            }
            Statement::While(ref expr, ref block) => {
                let label1 = format!("L{}", self.label_num);
                let label2 = format!("L{}", self.label_num+1);
                self.label_num += 2;

                self.instructions.push(Jump(label2.to_string()));
                self.instructions.push(Label(label1.to_string()));
                self.evaluate_block(block);

                self.instructions.push(Label(label2.to_string()));
                let reg = self.evaluate_expression(&expr);
                self.instructions.push(Compare(IntConstant(0), reg));
                self.instructions.push(JumpIfNotEqual(label1.to_string()));
            }
            Statement::Let(ref name, ref var_type, ref expr_opt) => {
                self.instructions.push(Comment(
                    format!("variable declaration{}",
                            name)));

                if self.identifier_to_var.contains_key(name) {
                    panic!("Variable {} already declared", *name);
                }

                let machine_type = self.representation_mgr.
                    get_machine_type(var_type);

                let var_size = get_mtype_size(machine_type);
                self.current_stack_offset -= var_size;

                self.identifier_to_var.insert(name.clone(),
                                              LocalVariable::new(
                                                  self.current_stack_offset,
                                                  var_type.clone(),
                                                  machine_type));
                {
                    let mut current_block = self.blocks.last_mut().unwrap();
                    current_block.declared_variables.insert(name.clone());
                }

                // TODO: Allocate all stack space in advance
                self.instructions.push(alloc_stack(var_size));
                if let &Some(ref expr) = expr_opt {
                    let reg = self.evaluate_expression(expr);
                    self.move_value_to_var(reg, name);
                }
            }
            Statement::Assign(ref left_expr, ref right_expr) => {
                // Figure out where we're going to store this
                let (mut addr_reg, off) = self.load_address_of_expr(left_expr);

                if addr_reg != EBP {
                    self.instructions.push(Push(Register(addr_reg)));
                }

                // Evaluate the right hand expression. This means
                // addr_reg now contains junk if its not EBP
                let value_op = self.evaluate_expression(right_expr);

                // Put the address we may have saved back into a register
                // (We can put it in any register besides the register
                // storing the expression's value)
                if addr_reg != EBP {
                    if let Register(reg) = value_op {
                        addr_reg = register_other_than(&reg);
                    }
                    self.instructions.push(Pop(Register(addr_reg)));
                }

                let l_type = left_expr.typ.as_ref().unwrap();
                let machine_type = self.representation_mgr
                    .get_machine_type(l_type);
                let instr = move_type(value_op,
                                      Dereference(addr_reg, off),
                                      machine_type);
                self.instructions.push(instr);
            }
            Statement::Call(ref fn_call) => {
                self.evaluate_function_call(fn_call);
            }
        }
    }

    fn evaluate_binary_op(&mut self,
                          op: &BinaryOp,
                          l_node: &AstExpressionNode,
                          r_node: &AstExpressionNode) -> Operand {
        self.instructions.push(Comment("Evaluating binary operation"
                                       .to_string()));

        let left_register = self.evaluate_expression(l_node);
        // Save the value that we computed in case evaluating
        // the right side overwrites this register
        self.instructions.push(Push(left_register));

        let right_register = self.evaluate_expression(r_node);

        // For now we use Register(EAX) for everything
        if right_register != Register(EAX) {
            self.instructions.push(Move(right_register, Register(EAX)));
        }

        // put the value of the left expression into Register(EBX)
        self.instructions.push(Pop(Register(EBX)));

        let instr = &mut self.instructions;

        match *op {
            BinaryOp::Plus => instr.push(Add(Register(EBX), Register(EAX))),
            BinaryOp::Multiply => instr.push(Multiply(Register(EBX),
                                                      Register(EAX))),
            BinaryOp::Minus => {
                instr.push(Subtract(Register(EAX), Register(EBX)));
                instr.push(Move(Register(EBX), Register(EAX)));
            }
            BinaryOp::Divide => {
                instr.push(Move(Register(EAX), Register(ECX)));
                instr.push(Move(Register(EBX), Register(EAX)));
                instr.push(Other("cltd".to_string()));
                instr.push(Divide(Register(ECX)));
            }
            BinaryOp::CompareEqual => {
                instr.push(Compare(Register(EAX), Register(EBX)));
                // FIXME: weird
                instr.push(Other("sete %al".to_string()));
                instr.push(Other("movzbl %al, %eax".to_string()));
            }
            BinaryOp::CompareGreater => {
                instr.push(Compare(Register(EAX), Register(EBX)));
                instr.push(Other("setg %al".to_string()));
                instr.push(Other("movzbl %al, %eax".to_string()));
            }
            BinaryOp::CompareLess => {
                instr.push(Compare(Register(EAX), Register(EBX)));
                instr.push(Other("setl %al".to_string()));
                instr.push(Other("movzbl %al, %eax".to_string()));
            }
            BinaryOp::CompareGreaterOrEqual => {
                instr.push(Compare(Register(EAX), Register(EBX)));
                instr.push(Other("setge %al".to_string()));
                instr.push(Other("movzbl %al, %eax".to_string()));
            }
            BinaryOp::CompareLessOrEqual => {
                instr.push(Compare(Register(EAX), Register(EBX)));
                instr.push(Other("setle %al".to_string()));
                instr.push(Other("movzbl %al, %eax".to_string()));

            }
            BinaryOp::CompareNotEqual => {
                instr.push(Compare(Register(EAX), Register(EBX)));
                instr.push(Other("setne %al".to_string()));
                instr.push(Other("movzbl %al, %eax".to_string()));
            }

        }
        Register(EAX)
    }

    fn evaluate_function_call(&mut self, fn_call: &FunctionCall) {
        for arg_expr in fn_call.args_exprs.iter().rev() {
            let reg = self.evaluate_expression(arg_expr);
            self.instructions.push(Push(reg));
        }

        let fn_name = match &fn_call.name[..] {
            "alloc_int" | "alloc_owned_int" => "malloc".to_string(),
            "allocate" => "malloc".to_string(), // allocate exact number of bytes given
            "free" => "free".to_string(),
            "free_int" => "free".to_string(),
            _ => fn_call.name.clone(),
        };

        // FIXME: This does not belong here. It should be done at an earlier stage.
        if &fn_call.name == "alloc_int" || &fn_call.name == "alloc_owned_int" {
            // Multiply argument by four
            self.instructions.push(Pop(Register(EAX)));
            self.instructions.push(Multiply(IntConstant(WORD_SIZE), Register(EAX)));
            self.instructions.push(Push(Register(EAX)));
        }

        self.instructions.push(Call(fn_name));
        self.instructions.push(free_stack(WORD_SIZE * fn_call.args_exprs.len() as i32));
    }

    /// Generate the assembly for a function
    fn generate_code_for_function(&mut self, fun: &Function) -> String {
        assert!(self.identifier_to_var.is_empty());
        assert!(self.blocks.is_empty());

        // let name = if &fun.name == "main" {
        //     "_start".to_string()
        // } else {
        //     fun.name.clone()
        // };
        let name = fun.name.clone();

        self.current_function = name.clone();

        // Add the function's parameters as local variables
        for i in 0..fun.fn_type.arg_types.len() {
            let arg_type = fun.fn_type.arg_types.get(i).expect("function parameter with no type");
            let machine_type = self.representation_mgr.get_machine_type(arg_type);
            let var = LocalVariable::new(WORD_SIZE * (2 + i as i32), arg_type.clone(), machine_type);
            self.identifier_to_var.insert(fun.args.get(i).unwrap().clone(), var);
        }

        let mut code = String::new();
        self.instructions = Vec::new();
        {
            let instr = &mut self.instructions;
            instr.push(Label(name.clone()));
            instr.push(Push(Register(EBP)));
            instr.push(Move(Register(ESP), Register(EBP)));
        }

        self.evaluate_block(&fun.statements);

        if name == "main" {
            //If the function is main, then returns 0 at the end
            let expr = AstExpressionNode::new(Expression::Value(0));
            let ret_stmt = Statement::Return(expr);
            self.evaluate_statement(&ret_stmt);
        }

        // Remove arguments from active identifiers
        for arg in &fun.args {
            self.identifier_to_var.remove(arg);
        }

        code.push_str(&instruction_list_to_asm(&mut self.instructions));
        code
    }
}

impl GeneratesCode for X86CodeGenerator {

    fn generate_code(&mut self, prog: &Program) -> String {
        self.representation_mgr.init(&prog.structs);

        let functions = &prog.functions;
        let asm_header = ".section .data\n\
                          decimal_format_str: .asciz \"%d\\n\"\n\
                          .section .text\n\
                          .globl main\n";
        let mut code = asm_header.to_string();
        for function in functions {
            code.push_str(&self.generate_code_for_function(function));
        }

        let mut complete_code = String::new();
        complete_code.push_str(".section .rodata\n");
        for (st, label) in self.string_to_label.iter() {
            complete_code.push_str(&format!("{}:\n\
                                            .string {}\n", label, st));
        }
        complete_code.push_str(&code);

        complete_code
    }
}
