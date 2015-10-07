use ast::Statement;
use ast::Expression;
use ast::BinaryOp;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn op_to_str(op: BinaryOp) -> &'static str {
    match op {
        BinaryOp::Plus => "+",
        BinaryOp::Minus => "-",
    }
}

fn print_expression(expr: Expression) -> String {
    match expr {
        Expression::Value(v) => format!("{}", v),
        Expression::BinaryOp(op, l, r) => format!("{} {} {}", l,
                                                  op_to_str(op), r),
    }
}

fn print_statement(tree: Statement) -> String {
    match tree {
        Statement::Return(v) => format!("return {}", print_expression(*v)),
    }
}

pub fn generate_code(tree: Statement) {
    let code = print_statement(tree);

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
