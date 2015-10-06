use ast::Statement;

pub fn generate_code(tree: Statement) {
    match tree {
        Statement::Return{val: v} => println!("Val is {}", v),
    }
}

