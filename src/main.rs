mod scanner;
mod parser;
mod ast;
mod code_generator;
mod lexeme;
mod token_stream;
mod assembly;

fn main() {
    let mut tokens = scanner::get_tokens("return 8 * ( 3 * ( 1 + 2 ) )");
    let res = parser::parse_return(&mut tokens);
    code_generator::generate_code(res);
}
