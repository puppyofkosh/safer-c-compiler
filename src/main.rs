mod code_block;
mod assembly;
mod assembly_helper;
mod assembly_printer;
mod ast;
mod code_generator;
mod lexeme;
mod parser;
mod scanner;
mod token_stream;
mod type_checker;
mod x86_code_generator;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


use code_generator::GeneratesCode;


fn read_file(name: &str) -> std::io::Result<String> {
    let mut f = try!(File::open(name));
    let mut s = String::new();

    try!(f.read_to_string(&mut s));
    Ok(s)
}

fn write_code(complete_code: &String) {
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

fn main() {
    let filename_res = env::args().nth(1);
    if filename_res.is_none() {
        println!("You can run with cargo run <filename>.sc");
        return;
    }
    let filename = &filename_res.unwrap();

    let result = read_file(filename);
    if let Err(_) = result {
        panic!("Error reading file {}", filename);
    }
    let program_text = result.unwrap();

    let mut tokens = scanner::get_tokens(&program_text);
    let prog = parser::parse_program(&mut tokens);

    let mut type_checker = type_checker::TypeChecker::new();
    let passed = type_checker.check_types(&prog);

    if !passed {
        println!("did not pass type checker!");
        
        for err in type_checker.get_errors() {
            println!("{}", err);
        }
        return;
    }

    let mut code_generator = x86_code_generator::X86CodeGenerator::new();
    let codestr = code_generator.generate_code(&prog);

    write_code(&codestr);
}
