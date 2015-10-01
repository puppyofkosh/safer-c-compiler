mod scanner;
mod parser;
mod ast;

fn main() {
    let tokens: Vec<&str> = scanner::get_tokens("return 1");
    assert_eq!(tokens[0], "return");
    assert_eq!(tokens[1], "1");
    let res = parser::parse_return(tokens);
    
}