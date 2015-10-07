mod scanner;
mod parser;
mod ast;
mod code_generator;
mod lexeme;

fn main() {
    let tokens = scanner::get_tokens("return 1");
    let res = parser::parse_return(&tokens);
    code_generator::generate_code(res);
}
