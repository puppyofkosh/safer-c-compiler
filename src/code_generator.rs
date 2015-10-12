use ast::Statement;
use ast::Expression;
use ast::BinaryOp;

use assembly::Instruction;
use assembly::Instruction::Move;
use assembly::Instruction::Pop;
use assembly::Instruction::Multiply;
use assembly::Instruction::Push;
use assembly::Instruction::Add;
use assembly::Operand;
use assembly::Operand::EAX;
use assembly::Operand::EBX;
use assembly::Operand::ESP;
use assembly::Operand::EBP;
use assembly::Operand::Variable;
use assembly::Operand::IntConstant;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;


fn evaluate_binary_op(op: &BinaryOp, l: &Expression, r: &Expression,
                      instructions: &mut Vec<Instruction>) -> Operand {
    let left_register = evaluate_expression(&l, instructions);
    // Save the value that we computed in case evaluating
    // the right side overwrites this register
    instructions.push(Push(left_register));
    
    let right_register = evaluate_expression(&r, instructions);
    // For now we use EAX for everything
    assert_eq!(right_register, EAX);

    // put the value of the left expression into EBX
    instructions.push(Pop(EBX));

    match *op {
        BinaryOp::Plus => instructions.push(Add(EBX, EAX)),
        BinaryOp::Multiply => instructions.push(Multiply(EBX, EAX)),
        _ => panic!("WTH"),
    }

    return EAX;
}

// Generate code to evaluate an expression and return the operand where
// the result is stored
fn evaluate_expression(expr: &Expression,
                       instructions: &mut Vec<Instruction>) -> Operand {
    match *expr {
        Expression::Value(ref v) => {
            // FIXME: We should probably use more than just the register
            // EAX...
            instructions.push(Move(IntConstant(*v), EAX));
            EAX
        }
        Expression::BinaryOp(ref op, ref l, ref r) => {
            evaluate_binary_op(op, l, r, instructions)
        }
    }
}

fn evaluate_statement(tree: &Statement,
                      instructions: &mut Vec<Instruction>) {
    match *tree {
        Statement::Return(ref v) => {
            let out_reg = evaluate_expression(&v, instructions);
            // For now everything goes into eax
            assert_eq!(out_reg, EAX);

            instructions.push(Pop(EBP));
            // FIXME: For now we assume retval is in EAX, then we put it
            // into ebx
            instructions.push(Move(EAX, EBX));
            //instructions.push(Move(IntConstant(0), EBX));
            instructions.push(Move(IntConstant(1), EAX));
            instructions.push(Instruction::Other("int $0x80".to_string()));
        }
        Statement::Print(ref expr) => {
            let result_reg = evaluate_expression(&expr, instructions);
            instructions.push(Push(result_reg));
            instructions.push(Push(Operand::Variable("decimal_format_str")));
            instructions.push(Instruction::Other("call printf".to_string()));
            // pop args off the stack
            instructions.push(Add(IntConstant(8), ESP));

            // Call fflush(0)

            instructions.push(Push(IntConstant(0)));
            instructions.push(Instruction::Other("call fflush".to_string()));
            instructions.push(Add(IntConstant(4), ESP));
        }
    }
}

fn op_to_str(o: &Operand) -> String {
    match *o {
        EAX => "%eax".to_string(),
        EBX => "%ebx".to_string(),
        EBP => "%ebp".to_string(),
        ESP => "%esp".to_string(),
        IntConstant(i) => "$".to_string() + &i.to_string(),
        Variable(n) => "$".to_string() + &n.to_string(),
    }
}

fn instruction_to_asm(ins: &Instruction) -> String {
    let mut s = match *ins {
        Add(ref a, ref b) => format!("addl {}, {}", op_to_str(a),
                             op_to_str(b)),
        Multiply(ref a, ref b) => format!("imull {}, {}", op_to_str(a),
                                  op_to_str(b)),
        Move(ref a, ref b) => format!("movl {}, {}", op_to_str(a),
                              op_to_str(b)),
        Push(ref a) => format!("pushl {}", op_to_str(a)),
        Pop(ref a) => format!("popl {}", op_to_str(a)),
        Instruction::Other(ref st) => st.clone(),
    };

    s.push_str("\n");
    s
}

fn instruction_list_to_asm(instructions: &Vec<Instruction>) -> String {
    instructions.iter().fold(String::new(),
                             |acc, ins| acc + &instruction_to_asm(ins))
}

pub fn generate_code(tree: &Vec<Statement>) {
    let asm_header = ".section .data\n\
                      decimal_format_str: .asciz \"%d\\n\"\n\
                      .section .text\n\
                      .globl _start\n\
                      _start:\n";
    let function_start = "pushl %ebp\n\
                          movl %esp, %ebp\n";
    let mut code = asm_header.to_string();
    code.push_str(function_start);

    let mut instructions = Vec::new();
    for stmt in tree {
        evaluate_statement(stmt, &mut instructions);
    }

    // For now we MANUALLY return 0 at the end of our program.
    evaluate_statement(&Statement::Return(Box::new(Expression::Value(0))),
                       &mut instructions);
    code.push_str(&instruction_list_to_asm(&instructions));
    
    // Bunch of file opening crap
    let path = Path::new("out/code.s");

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           path.display(),
                           Error::description(&why)),
        Ok(file) => file,
    };

    match file.write_all(code.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", path.display(),
                   Error::description(&why))
        },
        Ok(_) => println!("successfully wrote code"),
    }
}
