mod scanner;
mod parser;
mod ast;
mod code_generator;
mod lexeme;
mod token_stream;
mod assembly;

fn main() {
    let mut tokens = scanner::get_tokens("print 8 * ( 3 * ( 1 + 2 ) )");
    let ast = parser::parse_statement(&mut tokens);
    code_generator::generate_code(ast);
}
