// TODO:
// Move op_to_str stuff somewhere else
// Make Register a separate operand type and dereference won't need a box

use ast::Statement;
use ast::Expression;
use ast::BinaryOp;
use ast::Function;
use ast::VarType;

use assembly::Instruction;
use assembly::Instruction::*;
use assembly::Operand;
use assembly::Operand::*;
use assembly::is_register;
use assembly::get_low_byte;

use assembly_printer::instruction_list_to_asm;

use code_generator::GeneratesCode;

use std::collections::HashMap;
use std::collections::HashSet;

static WORD_SIZE: i32 = 4;

fn get_type_size(t: VarType) -> i32 {
    match t {
        VarType::Int => WORD_SIZE,
        VarType::Char => 1,
    }
}



#[derive(Clone, Copy)]
struct LocalVariable {
    stack_offset: i32,
    var_type: VarType,
}

impl LocalVariable {
    pub fn new(off: i32, var_type: VarType) -> LocalVariable {
        LocalVariable {
            stack_offset: off,
            var_type: var_type,
        }
    }
}

struct ActiveBlock {
    declared_variables: HashSet<String>,
}

impl ActiveBlock {
    pub fn new() -> ActiveBlock {
        ActiveBlock {
            declared_variables: HashSet::new(),
        }
    }
}

pub struct X86CodeGenerator {
    label_num: i32,

    // keep track of where in memory variables are stored
    identifier_to_var: HashMap<String, LocalVariable>,
    blocks: Vec<ActiveBlock>,
    current_stack_offset: i32,
    current_function: String,

    // string
    string_to_label: HashMap<String, String>,
    current_label_num: i32,

    instructions: Vec<Instruction>,
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
        }
    }
    
    fn move_var_to_register(&mut self,
                            var: &LocalVariable, reg: Operand) {
        let from_op = Dereference(Box::new(EBP), var.stack_offset);
        match var.var_type {
            VarType::Int => {
                self.instructions.push(Move(from_op, reg));
            },
            VarType::Char => {
                self.instructions.push(OtherTwoArg("movzbl", from_op, reg));
            }
        }
    }

    fn move_value_to_var(&mut self, reg: Operand,
                         var: &LocalVariable) {
        let to_operand = Dereference(Box::new(EBP), var.stack_offset);
        match var.var_type {
            VarType::Int => {
                self.instructions.push(Move(reg, to_operand));
            },
            VarType::Char => {
                let mut src = reg;
                if is_register(&src) {
                    src = get_low_byte(&src);
                }
                self.instructions.push(OtherTwoArg("movb", src, to_operand));
            }
        }
    }
                         

    fn evaluate_block(&mut self, statements: &Vec<Statement>) {
        self.blocks.push(ActiveBlock::new());
        for stmt in statements {
            self.evaluate_statement(stmt);
        }

        // Wipe out all of the variables we declared in this block, as
        // we shouldn't able to use them again
        match self.blocks.pop() {
            None => panic!("Invalid sate. Why is there no current block?"),
            Some(block) => {
                assert!(block.declared_variables.len() <
                        i32::max_value() as usize);
                let previous_offset = self.current_stack_offset;
                for variable in block.declared_variables {
                    let var = self.identifier_to_var
                        .remove(&variable)
                        .unwrap();
                    self.current_stack_offset += get_type_size(var.var_type);
                }
                
                // Give the stack space back
                let space_freed = previous_offset - self.current_stack_offset;
                if space_freed > 0 {
                    self.instructions.push(Add(IntConstant(space_freed),
                                               ESP));
                }
            }
        }
    }

    // Generate code to evaluate an expression and return the operand where
    // the result is stored
    fn evaluate_expression(&mut self,
                           expr: &Expression) -> Operand {
        match *expr {
            Expression::Call(ref fn_call) => {
                let reg = self.evaluate_expression(&fn_call.arg_expr);
                self.instructions.push(Push(reg));
                
                self.instructions.push(Call(fn_call.name.clone()));
                self.instructions.push(Add(IntConstant(WORD_SIZE), ESP));
                EAX
            }
            Expression::Value(ref v) => {
                // FIXME: We should probably use more than just the register
                // EAX...
                self.instructions.push(Move(IntConstant(*v), EAX));
                EAX
            }
            Expression::StringValue(ref v) => {
                let label = format!(".LC{}", self.current_label_num);
                self.current_label_num += 1;
                self.string_to_label.insert(v.clone(), label.clone());
                self.instructions.push(Move(Variable(label.clone()), EAX));
                EAX
            }
            Expression::BinaryOp(ref op, ref l, ref r) => {
                self.evaluate_binary_op(op, l, r)
            }
            Expression::Variable(ref name) => {
                let var = *self.identifier_to_var
                    .get(name)
                    .expect(&format!("Unkown variable {}", name));
                self.move_var_to_register(&var, EAX);
                EAX
            }
        }
    }

    fn evaluate_return_statement(&mut self, value: &Expression) {
        let out_reg = self.evaluate_expression(&value);
        let instr = &mut self.instructions;
        // For now everything goes into eax
        if out_reg != EAX {
            instr.push(Move(out_reg, EAX));
        }

        instr.push(Move(EBP, ESP));
        instr.push(Pop(EBP));
        if self.current_function == "_start" {
            instr.push(Move(EAX, EBX));
            instr.push(Move(IntConstant(1), EAX));
            instr.push(Instruction::OtherStatic("int $0x80"));
        }
        else {
            instr.push(Instruction::OtherStatic("ret"));
        }
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
                instr.push(Add(IntConstant(8), ESP));

                // Call fflush(0)

                instr.push(Push(IntConstant(0)));
                instr.push(Instruction::Other("call fflush".to_string()));
                instr.push(Add(IntConstant(4), ESP));
            }
            Statement::If(ref expr, ref statements) => {
                let reg = self.evaluate_expression(&expr);

                let label = format!("L{}", self.label_num);
                self.label_num += 1;
                self.instructions.push(Compare(IntConstant(0), reg));
                // Jump PAST the "then statement" if the expression is false
                self.instructions.push(JumpIfEqual(label.to_string()));

                self.evaluate_block(statements);

                // print the label to jump to if the expr is false
                self.instructions.push(Instruction::Label(label.to_string()));
            }
            Statement::While(ref expr, ref statement) => {
                let label1 = format!("L{}", self.label_num);
                let label2 = format!("L{}", self.label_num+1);
                self.label_num += 2;

                self.instructions.push(Jump(label2.to_string()));
                self.instructions.push(Label(label1.to_string()));
                self.evaluate_block(statement);

                self.instructions.push(Label(label2.to_string()));
                let reg = self.evaluate_expression(&expr);
                self.instructions.push(Compare(IntConstant(0), reg));
                self.instructions.push(JumpIfNotEqual(label1.to_string()));
            }
            Statement::Let(ref name, ref var_type, ref expr) => {
                self.instructions.push(Comment(
                    format!("variable declaration{}",
                            name)));

                if self.identifier_to_var.contains_key(name) {
                    panic!("Variable {} already declared", *name);
                }
                let reg = self.evaluate_expression(expr);

                let var_size = get_type_size(*var_type);
                self.current_stack_offset -= var_size;
                
                self.identifier_to_var.insert(name.clone(),
                                              LocalVariable::new(
                                                  self.current_stack_offset,
                                                  var_type.clone()));
                {
                    let mut current_block = self.blocks.last_mut().unwrap();
                    current_block.declared_variables.insert(name.clone());
                }

                // TODO: Allocate all stack space in advance
                self.instructions.push(Subtract(IntConstant(var_size), ESP));
                let local_var = *self.identifier_to_var.get(name).unwrap();
                self.move_value_to_var(reg, &local_var);
            }
            Statement::Assign(ref name, ref expr) => {
                let reg = self.evaluate_expression(expr);
                let var = *self.identifier_to_var.get(name).unwrap();
                self.move_value_to_var(reg, &var);
            }
            Statement::Call(ref fn_call) => {
                let reg = self.evaluate_expression(&fn_call.arg_expr);
                self.instructions.push(Push(reg));
                
                self.instructions.push(Call(fn_call.name.clone()));
                self.instructions.push(Add(IntConstant(WORD_SIZE), ESP));
            }
        }
    }

    fn evaluate_binary_op(&mut self,
                          op: &BinaryOp,
                          l: &Expression, r: &Expression) -> Operand {
        self.instructions.push(Comment("Evaluating binary operation"
                                       .to_string()));

        let left_register = self.evaluate_expression(&l);
        // Save the value that we computed in case evaluating
        // the right side overwrites this register
        self.instructions.push(Push(left_register));
        
        let right_register = self.evaluate_expression(&r);

        // For now we use EAX for everything
        if right_register != EAX {
            self.instructions.push(Move(right_register, EAX));
        }
 
        // put the value of the left expression into EBX
        self.instructions.push(Pop(EBX));

        let instr = &mut self.instructions;
        match *op {
            BinaryOp::Plus => instr.push(Add(EBX, EAX)),
            BinaryOp::Multiply => instr.push(Multiply(EBX, EAX)),
            BinaryOp::Minus => {
                instr.push(Subtract(EAX, EBX));
                instr.push(Move(EBX, EAX));
            }
            BinaryOp::Divide => {
                instr.push(Move(EAX, ECX));
                instr.push(Move(EBX, EAX));
                instr.push(Other("cltd".to_string()));
                instr.push(Divide(ECX));
            }
            BinaryOp::CompareEqual => {
                instr.push(Compare(EAX, EBX));
                // FIXME: weird
                instr.push(Other("sete %al".to_string()));
                instr.push(Other("movzbl %al, %eax".to_string()));
            }
            BinaryOp::CompareGreater => {
                instr.push(Compare(EAX, EBX));
                instr.push(Other("setg %al".to_string()));
                instr.push(Other("movzbl %al, %eax".to_string()));
            }
            BinaryOp::CompareLess => {
                instr.push(Compare(EAX, EBX));
                instr.push(Other("setl %al".to_string()));
                instr.push(Other("movzbl %al, %eax".to_string()));
            }
            BinaryOp::CompareGreaterOrEqual => {
                instr.push(Compare(EAX, EBX));
                instr.push(Other("setge %al".to_string()));
                instr.push(Other("movzbl %al, %eax".to_string()));
            }
            BinaryOp::CompareLessOrEqual => {
                instr.push(Compare(EAX, EBX));
                instr.push(Other("setle %al".to_string()));
                instr.push(Other("movzbl %al, %eax".to_string()));
                
            }
            BinaryOp::CompareNotEqual => {
                instr.push(Compare(EAX, EBX));
                instr.push(Other("setne %al".to_string()));
                instr.push(Other("movzbl %al, %eax".to_string()));
            }

        }
        EAX
    }

    fn generate_code_for_function(&mut self, fun: &Function) -> String {
        assert!(self.identifier_to_var.is_empty());
        assert!(self.blocks.is_empty());

        let name = if &fun.name == "main" {
            "_start".to_string()
        } else {
            fun.name.clone()
        };

        self.current_function = name.clone();
        let var = LocalVariable::new(WORD_SIZE * 2, VarType::Int);
        self.identifier_to_var.insert(fun.arg.clone(), var);

        let mut code = String::new();
        self.instructions = Vec::new();
        {
            let instr = &mut self.instructions;
            instr.push(Label(name.clone()));
            instr.push(Push(EBP));
            instr.push(Move(ESP, EBP));
        }
        self.evaluate_block(&fun.statements);
        if name == "_start" {
            let ret_stmt = Statement::Return(Box::new(Expression::Value(0)));
            self.evaluate_statement(&ret_stmt);
        }

        // Remove arguments from active identifiers
        self.identifier_to_var.remove(&fun.arg);

        code.push_str(&instruction_list_to_asm(&mut self.instructions));
        code
    }
}

impl GeneratesCode for X86CodeGenerator {

    fn generate_code(&mut self, functions: &Vec<Function>) -> String {
        let asm_header = ".section .data\n\
                          decimal_format_str: .asciz \"%d\\n\"\n\
                          .section .text\n\
                          .globl _start\n";
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
        // Bunch of file opening crap

        complete_code
    }
}
