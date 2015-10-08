use ast::Statement;
use ast::Expression;
use ast::BinaryOp;

use assembly::Instruction;
use assembly::Instruction::Move;
use assembly::Instruction::Pop;
use assembly::Value;
use assembly::Value::EAX;
use assembly::Value::EBX;
use assembly::Value::EBP;
use assembly::Value::IntConstant;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn op_to_str(op: BinaryOp) -> &'static str {
    match op {
        BinaryOp::Plus => "plus",
        BinaryOp::Minus => "minus",
        BinaryOp::Multiply => "times",
        BinaryOp::Divide => "/",
    }
}

fn evaluate_expression(expr: Expression) -> String {
    match expr {
        Expression::Value(v) => format!("{}", v),
        Expression::BinaryOp(op, l, r) =>
            format!("({} {} {})",
                    evaluate_expression(*l),
                    op_to_str(op),
                    evaluate_expression(*r)),
    }
}

fn evaluate_statement(tree: Statement,
                      instructions: &mut Vec<Instruction>) {
    match tree {
        Statement::Return(v) => {
            instructions.push(Pop(EBP));
            instructions.push(Move(IntConstant(0), EBX));
            instructions.push(Move(IntConstant(1), EAX));
            instructions.push(Instruction::Other("int $0x80".to_string()));
        }
    }
}

pub fn generate_code(tree: Statement) {
    let asm_header = ".section .data\n\
                      .section .text\n\
                      .globl _start\n\
                      _start:\n";
    let function_start = "pushl %ebp\n\
                          movl %esp, %ebp\n";
    let mut code = asm_header.to_string();
    code.push_str(function_start);

    // Generate what the instructions are
    let mut instructions = Vec::new();
    evaluate_statement(tree, &mut instructions);
    
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
