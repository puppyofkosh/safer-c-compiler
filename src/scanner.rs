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
        "if" => Lexeme::If,
        "return" => Lexeme::Return,
        "print" => Lexeme::Print,
        "let" => Lexeme::Let,
        "=" => Lexeme::Operator(OperatorType::Assign),
        "==" => Lexeme::Operator(OperatorType::CompareEqual),
        ">" => Lexeme::Operator(OperatorType::CompareGreater),
        "<" => Lexeme::Operator(OperatorType::CompareLess),
        ">=" => Lexeme::Operator(OperatorType::CompareGreaterOrEqual),
        "<=" => Lexeme::Operator(OperatorType::CompareLessOrEqual),
        "!=" => Lexeme::Operator(OperatorType::CompareNotEqual),
        "*" => Lexeme::Operator(OperatorType::Star),
        "/" => Lexeme::Operator(OperatorType::Divide),
        "+" => Lexeme::Operator(OperatorType::Plus),
        "-" => Lexeme::Operator(OperatorType::Minus),
        "(" => Lexeme::LParen,
        ")" => Lexeme::RParen,
        "{" => Lexeme::StartBlock,
        "}" => Lexeme::EndBlock,
        ";" => Lexeme::EndOfStatement,
        _ => {
            if token.chars().all(|ch| ch.is_alphanumeric()) {
                Lexeme::Identifier(token.to_string())
            }
            else {
                panic!("Unkown token! {}", token)            
            }
        }
    }
}

fn read_until(iter: &mut Iterator<Item=char>, stop_ch: char) {
    while let Some(ch) = iter.next() {
        if ch == stop_ch {
            break;
        }
    }
}

fn get_token_strings(source: &str) -> LinkedList<Lexeme> {
    let mut iter = source.chars().peekable();
    let mut tokens = LinkedList::new();

    let mut s = String::new();

    while let Some(ch) = iter.next() {
        // Check for comments. FIXME: This is super ugly. Why do we have 3 nested statements?
        if ch == '/' {
            if let Some(&'/') = iter.peek() {
                // We have a comment. Continue reading until we hit a newline
                read_until(&mut iter, '\n');
                continue;
            }
        }
        
        if ch.is_whitespace() {
            if s.len() > 0 {
                tokens.push_back(token_to_lexeme(&s));
                s = String::new();
            }
        } else {
            s.push(ch);
        }
    }

    if s.len() > 0 {
        tokens.push_back(token_to_lexeme(&s));
    }

    tokens
}

pub fn get_tokens(source: &str) -> TokenStream {
    let t = get_token_strings(source);
    TokenStream::new(t)
}
