use ast::Statement;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn get_asm_code(tree: Statement) -> String{
    match tree {
        Statement::Return{val: v} => format!("Val is {}", v),
    }
}

pub fn generate_code(tree: Statement) {
    let code = get_asm_code(tree);

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

