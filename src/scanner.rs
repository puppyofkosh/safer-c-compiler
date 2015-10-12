use std::collections::LinkedList;

use lexeme::Lexeme;
use lexeme::OperatorType;
use token_stream::TokenStream;

fn token_to_lexeme(token: &str) -> Lexeme {
    // TODO: Actually write this...need to do some cases on the token
    let parsed_int = token.parse::<i32>().ok();
    if let Some(value) = parsed_int {
        return Lexeme::IntConstant(value);
    }

    match token {
        "return" => Lexeme::Return,
        "print" => Lexeme::Print,
        "*" => Lexeme::Operator(OperatorType::Star),
        "/" => Lexeme::Operator(OperatorType::Divide),
        "+" => Lexeme::Operator(OperatorType::Plus),
        "-" => Lexeme::Operator(OperatorType::Minus),
        "(" => Lexeme::LParen,
        ")" => Lexeme::RParen,
        ";" => Lexeme::EndOfStatement,
        _ => panic!("Unkown token! {}", token),
    }
}

pub fn get_tokens(source: &str) -> TokenStream {
    // split by lines, get rid of comments
    let line_split = source.split("\n");
    let lines = line_split.filter(|l| !l.starts_with("//"));
    
    let mut tokens = LinkedList::new();
    for line in lines {
        let split = line.split(" ");
        let mut line_tokens: LinkedList<Lexeme> = split
            .map(token_to_lexeme)
            .collect();
        tokens.append(&mut line_tokens);
    }

    TokenStream::new(tokens)
}
