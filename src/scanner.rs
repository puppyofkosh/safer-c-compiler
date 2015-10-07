use lexeme::Lexeme;
use token_stream::TokenStream;

fn token_to_lexeme(token: &str) -> Lexeme {
    // TODO: Actually write this...need to do some cases on the token
    match token {
        "return" => Lexeme::Return,
        "1" => Lexeme::IntConstant(1),
        "+" => Lexeme::Operator(token.to_string()),
        _ => panic!("Wth"),
    }
}

pub fn get_tokens(source: &str) -> TokenStream {
    let split = source.split(" ");
    let tokens = split.map(|x| token_to_lexeme(x)).collect();
    TokenStream::new(tokens)
}
