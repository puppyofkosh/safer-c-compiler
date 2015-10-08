use ast::Statement;
use ast::Expression;
use ast::BinaryOp;

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

fn print_expression(expr: Expression) -> String {
    match expr {
        Expression::Value(v) => format!("{}", v),
        Expression::BinaryOp(op, l, r) => format!("({} {} {})", print_expression(*l),
                                                  op_to_str(op), print_expression(*r)),
    }
}

fn print_statement(tree: Statement) -> String {
    match tree {
        Statement::Return(v) => format!("popl %ebp\n\
                                         movl $0, %ebx\n\
                                         movl $1, %eax\n\
                                         int $0x80\n"),
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
    code.push_str(&print_statement(tree));

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
