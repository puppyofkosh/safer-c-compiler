mod scanner;
mod parser;
mod ast;
mod code_generator;

fn main() {
    let tokens = scanner::get_tokens("return 2");
    assert_eq!(tokens, ["return", "2"]);
    let res = parser::parse_return(&tokens);
    code_generator::generate_code(res);
}
