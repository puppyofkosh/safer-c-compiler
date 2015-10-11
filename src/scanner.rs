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
        _ => panic!("Unkown token!"),
    }
}

pub fn get_tokens(source: &str) -> TokenStream {
    let split = source.split(" ");
    let tokens = split.map(|x| token_to_lexeme(x)).collect();
    TokenStream::new(tokens)
}
