mod scanner;
mod parser;
mod ast;
mod code_generator;
mod lexeme;
mod token_stream;
mod assembly;

use std::io::prelude::*;
use std::fs::File;

fn read_file(name: &str) -> std::io::Result<String> {
    let mut f = try!(File::open(name));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}

fn main() {
    let result = read_file("tests/arithmetic/addition.sc");
    if let Err(_) = result {
        panic!("Error reading file");
    }
    
    let program_text = result.unwrap();

    let mut tokens = scanner::get_tokens(&program_text);
    let ast = parser::parse_tokens(&mut tokens);
    code_generator::generate_code(&ast);
}
