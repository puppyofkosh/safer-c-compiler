use ast::Statement;
use ast::Expression;
use ast::BinaryOp;
use ast::Function;

use assembly::Instruction;
use assembly::Instruction::*;
use assembly::Operand;
use assembly::Operand::*;

use code_generator::GeneratesCode;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use std::collections::HashMap;
use std::collections::HashSet;

static WORD_SIZE: i32 = 4;

fn op_to_str(o: &Operand) -> String {
    match *o {
        EAX => "%eax".to_string(),
        EBX => "%ebx".to_string(),
        ECX => "%ecx".to_string(),
        EBP => "%ebp".to_string(),
        ESP => "%esp".to_string(),
        IntConstant(i) => "$".to_string() + &i.to_string(),
        Variable(n) => "$".to_string() + &n.to_string(),
        Dereference(ref e, offset) => format!("{}({})", offset, op_to_str(e)),
    }
}

fn instruction_to_asm(ins: &Instruction) -> String {
    let mut s = match *ins {
        Add(ref a, ref b) => format!("addl {}, {}", op_to_str(a),
                                     op_to_str(b)),
        Multiply(ref a, ref b) => format!("imull {}, {}", op_to_str(a),
                                          op_to_str(b)),
        Subtract(ref a, ref b) => format!("subl {}, {}", op_to_str(a), op_to_str(b)),
        Divide(ref a) => format!("idivl {}", op_to_str(a)),
        Move(ref a, ref b) => format!("movl {}, {}", op_to_str(a),
                                      op_to_str(b)),
        Push(ref a) => format!("pushl {}", op_to_str(a)),
        Pop(ref a) => format!("popl {}", op_to_str(a)),
        Instruction::Other(ref st) => st.clone(),
        Instruction::OtherStatic(ref st) => st.to_string(),
        Compare(ref a, ref b) => format!("cmp {}, {}", op_to_str(a),
                                         op_to_str(b)),
        JumpIfEqual(ref a) => format!("je {}", a),
        JumpIfNotEqual(ref a) => format!("jne {}", a),
        Jump(ref a) => format!("jmp {}", a),
        Label(ref l) => format!("{}:", l),
        Comment(ref s) => format!("# {}", s),
        Call(ref name) => format!("call {}", name),
    };

    s.push_str("\n");
    s
}

fn instruction_list_to_asm(instructions: &Vec<Instruction>) -> String {
    instructions.iter().fold(String::new(),
                             |acc, ins| acc + &instruction_to_asm(ins))
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
    identifier_to_offset: HashMap<String, i32>,
    blocks: Vec<ActiveBlock>,
    current_stack_offset: i32,
    current_function: String,

    // string
    string_to_label: HashMap<String, String>,
    current_label_num: i32,
}

impl X86CodeGenerator {
    pub fn new() -> X86CodeGenerator {
        X86CodeGenerator {
            label_num: 0,
            identifier_to_offset: HashMap::new(),
            blocks: Vec::new(),
            current_function: String::new(),

            current_stack_offset: 0,
            string_to_label: HashMap::new(),
            current_label_num: 0,
        }
    }

    fn evaluate_block(&mut self, statements: &Vec<Statement>,
                      instructions: &mut Vec<Instruction>) {
        self.blocks.push(ActiveBlock::new());
        for stmt in statements {
            self.evaluate_statement(stmt, instructions);
        }

        // Wipe out all of the variables we declared in this block, as
        // we shouldn't able to use them again
        match self.blocks.pop() {
            None => panic!("Invalid sate. Why is there no current block?"),
            Some(block) => {
                assert!(block.declared_variables.len() < i32::max_value() as usize);
                let previous_offset = self.current_stack_offset;
                for variable in block.declared_variables {
                    self.current_stack_offset += WORD_SIZE;
                    self.identifier_to_offset.remove(&variable);
                }
                
                // Give the stack space back
                let space_freed = previous_offset - self.current_stack_offset;
                if space_freed > 0 {
                    instructions.push(Add(IntConstant(space_freed), ESP));
                }
            }
        }
    }

    // Generate code to evaluate an expression and return the operand where
    // the result is stored
    fn evaluate_expression(&mut self,
                           expr: &Expression,
                           instructions: &mut Vec<Instruction>) -> Operand {
        match *expr {
            Expression::Value(ref v) => {
                // FIXME: We should probably use more than just the register
                // EAX...
                instructions.push(Move(IntConstant(*v), EAX));
                EAX
            }
            Expression::StringValue(ref v) => {
                let label = format!(".LC{}", self.current_label_num);
                self.current_label_num += 1;
                self.string_to_label.insert(v.clone(), label.clone());
                EAX
            }
            Expression::BinaryOp(ref op, ref l, ref r) => {
                self.evaluate_binary_op(op, l, r, instructions)
            }
            Expression::Variable(ref name) => {
                match self.identifier_to_offset.get(name) {
                    Some(offset) => Dereference(Box::new(EBP), *offset),
                    None => panic!("Unkown variable {}", name),
                }
            }
        }
    }

    fn evaluate_return_statement(&mut self, value: &Expression,
                                 instructions: &mut Vec<Instruction>) {
        let out_reg = self.evaluate_expression(&value, instructions);
        // For now everything goes into eax
        if out_reg != EAX {
            instructions.push(Move(out_reg, EAX));
        }

        instructions.push(Move(EBP, ESP));
        instructions.push(Pop(EBP));
        if self.current_function == "_start" {
            instructions.push(Move(EAX, EBX));
            instructions.push(Move(IntConstant(1), EAX));
            instructions.push(Instruction::OtherStatic("int $0x80"));
        }
        else {
            instructions.push(Instruction::OtherStatic("ret"));
        }
    }

    fn evaluate_statement(&mut self,
                          tree: &Statement,
                          instructions: &mut Vec<Instruction>) {
        match *tree {
            Statement::Return(ref v) => {
                self.evaluate_return_statement(v, instructions);
            }
            Statement::Print(ref expr) => {
                instructions.push(Comment("Evaluating print statement".to_string()));
                let result_reg = self.evaluate_expression(&expr, instructions);
                instructions.push(Push(result_reg));
                instructions.push(Push(Variable("decimal_format_str")));
                instructions.push(Instruction::Other("call printf".to_string()));
                // pop args off the stack
                instructions.push(Add(IntConstant(8), ESP));

                // Call fflush(0)

                instructions.push(Push(IntConstant(0)));
                instructions.push(Instruction::Other("call fflush".to_string()));
                instructions.push(Add(IntConstant(4), ESP));
            }
            Statement::If(ref expr, ref statements) => {
                let reg = self.evaluate_expression(&expr, instructions);

                let label = format!("L{}", self.label_num);
                self.label_num += 1;
                instructions.push(Compare(IntConstant(0), reg));
                // Jump PAST the "then statement" if the expression is false
                instructions.push(JumpIfEqual(label.to_string()));

                self.evaluate_block(statements, instructions);

                // print the label to jump to if the expr is false
                instructions.push(Instruction::Label(label.to_string()));
            }
            Statement::While(ref expr, ref statement) => {
                let label1 = format!("L{}", self.label_num);
                let label2 = format!("L{}", self.label_num+1);
                self.label_num += 2;

                instructions.push(Jump(label2.to_string()));
                instructions.push(Label(label1.to_string()));
                self.evaluate_block(statement, instructions);

                instructions.push(Label(label2.to_string()));
                let reg = self.evaluate_expression(&expr, instructions);
                instructions.push(Compare(IntConstant(0), reg));
                instructions.push(JumpIfNotEqual(label1.to_string()));
            }
            Statement::Let(ref name, ref expr) => {
                instructions.push(Comment(format!("variable declaration{}", name)));

                if self.identifier_to_offset.contains_key(name) {
                    panic!("Variable {} already declared", *name);
                }

                // Evaluate the expression and put it on the stack
                let reg = self.evaluate_expression(expr, instructions);
                
                self.current_stack_offset -= WORD_SIZE;
                self.identifier_to_offset.insert(name.clone(),
                                                 self.current_stack_offset);
                let mut current_block = self.blocks.last_mut().unwrap();
                current_block.declared_variables.insert(name.clone());

                // TODO: Allocate all stack space in advance
                instructions.push(Subtract(IntConstant(WORD_SIZE), ESP));
                instructions.push(Move(reg, Dereference(Box::new(EBP),
                                                        self.current_stack_offset)));
            }
            Statement::Assign(ref name, ref expr) => {
                let offset = *self.identifier_to_offset
                    .get(name)
                    .expect(&format!("Unkown identifier {}", name));

                let reg = self.evaluate_expression(expr, instructions);
                instructions.push(Move(reg, Dereference(Box::new(EBP),
                                                        offset)));
            }
            Statement::Call(ref fn_name, ref arg_expr) => {
                let reg = self.evaluate_expression(arg_expr, instructions);
                instructions.push(Push(reg));
                
                instructions.push(Call(fn_name.clone()));
                instructions.push(Add(IntConstant(WORD_SIZE), ESP));
            }
        }
    }

    fn evaluate_binary_op(&mut self,
                          op: &BinaryOp, l: &Expression, r: &Expression,
                          instructions: &mut Vec<Instruction>) -> Operand {
        instructions.push(Comment("Evaluating binary operation".to_string()));

        let left_register = self.evaluate_expression(&l, instructions);
        // Save the value that we computed in case evaluating
        // the right side overwrites this register
        instructions.push(Push(left_register));
        
        let right_register = self.evaluate_expression(&r, instructions);

        // For now we use EAX for everything
        if right_register != EAX {
            instructions.push(Move(right_register, EAX));
        }
 
        // put the value of the left expression into EBX
        instructions.push(Pop(EBX));

        match *op {
            BinaryOp::Plus => instructions.push(Add(EBX, EAX)),
            BinaryOp::Multiply => instructions.push(Multiply(EBX, EAX)),
            BinaryOp::Minus => {
                instructions.push(Subtract(EAX, EBX));
                instructions.push(Move(EBX, EAX));
            }
            BinaryOp::Divide => {
                instructions.push(Move(EAX, ECX));
                instructions.push(Move(EBX, EAX));
                instructions.push(Other("cltd".to_string()));
                instructions.push(Divide(ECX));
            }
            BinaryOp::CompareEqual => {
                instructions.push(Compare(EAX, EBX));
                // FIXME: weird
                instructions.push(Other("sete %al".to_string()));
                instructions.push(Other("movzbl %al, %eax".to_string()));
            }
            BinaryOp::CompareGreater => {
                instructions.push(Compare(EAX, EBX));
                instructions.push(Other("setg %al".to_string()));
                instructions.push(Other("movzbl %al, %eax".to_string()));
            }
            BinaryOp::CompareLess => {
                instructions.push(Compare(EAX, EBX));
                instructions.push(Other("setl %al".to_string()));
                instructions.push(Other("movzbl %al, %eax".to_string()));
            }
            BinaryOp::CompareGreaterOrEqual => {
                instructions.push(Compare(EAX, EBX));
                instructions.push(Other("setge %al".to_string()));
                instructions.push(Other("movzbl %al, %eax".to_string()));
            }
            BinaryOp::CompareLessOrEqual => {
                instructions.push(Compare(EAX, EBX));
                instructions.push(Other("setle %al".to_string()));
                instructions.push(Other("movzbl %al, %eax".to_string()));
                
            }
            BinaryOp::CompareNotEqual => {
                instructions.push(Compare(EAX, EBX));
                instructions.push(Other("setne %al".to_string()));
                instructions.push(Other("movzbl %al, %eax".to_string()));
            }

        }
        EAX
    }

    fn generate_code_for_function(&mut self, fun: &Function) -> String {
        assert!(self.identifier_to_offset.is_empty());
        assert!(self.blocks.is_empty());

        let name = if &fun.name == "main" {
            "_start".to_string()
        } else {
            fun.name.clone()
        };

        self.current_function = name.clone();
        self.identifier_to_offset.insert(fun.arg.clone(), WORD_SIZE * 2);

        let mut code = String::new();
        // TODO: Insert argument to identifier_to_offset
        let mut instructions = Vec::new();
        instructions.push(Label(name.clone()));
        instructions.push(Push(EBP));
        instructions.push(Move(ESP, EBP));

        self.evaluate_block(&fun.statements, &mut instructions);
        if name == "_start" {
            let ret_stmt = Statement::Return(Box::new(Expression::Value(0)));
            self.evaluate_statement(&ret_stmt, &mut instructions);
        }

        // Remove arguments from active identifiers
        self.identifier_to_offset.remove(&fun.arg);

        code.push_str(&instruction_list_to_asm(&instructions));
        code
    }
}

impl GeneratesCode for X86CodeGenerator {

    fn generate_code(&mut self, functions: &Vec<Function>) {
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
        let path = Path::new("out/code.s");

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}",
                               path.display(),
                               Error::description(&why)),
            Ok(file) => file,
        };

        match file.write_all(complete_code.as_bytes()) {
            Err(why) => {
                panic!("couldn't write to {}: {}", path.display(),
                       Error::description(&why))
            },
            Ok(_) => println!("successfully wrote code"),
        }
    }
}
