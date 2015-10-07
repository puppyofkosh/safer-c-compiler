use lexeme::Lexeme;
use token_stream::TokenStream;

// FIXME: Maybe Lexeme::Operator should have an operator instead of a string...
fn is_operator(s: &str) -> Option<Lexeme> {
    match s {
        "+" => Some(Lexeme::Operator(s.to_string())),
        "*" => Some(Lexeme::Operator(s.to_string())),
        "/" => Some(Lexeme::Operator(s.to_string())),
        _ => None
    }
}

fn token_to_lexeme(token: &str) -> Lexeme {
    // TODO: Actually write this...need to do some cases on the token
    let parsed_int = token.parse::<i32>().ok();
    if let Some(value) = parsed_int {
        return Lexeme::IntConstant(value);
    }

    let parsed_operator = is_operator(token);
    if let Some(op) = parsed_operator {
        return op;
    }

    match token {
        "return" => Lexeme::Return,
        _ => panic!("Unkown token!"),
    }
}

pub fn get_tokens(source: &str) -> TokenStream {
    let split = source.split(" ");
    let tokens = split.map(|x| token_to_lexeme(x)).collect();
    TokenStream::new(tokens)
}
