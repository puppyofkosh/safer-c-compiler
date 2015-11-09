use std::collections::LinkedList;

use lexeme::Lexeme;
use lexeme::OperatorType;
use token_stream::TokenStream;

fn token_to_lexeme(token: &str) -> Lexeme {
    assert!(token.len() > 0);

    // TODO: Actually write this...need to do some cases on the token
    let parsed_int = token.parse::<i32>().ok();
    if let Some(value) = parsed_int {
        return Lexeme::IntConstant(value);
    }

    match token {
        "if" => Lexeme::If,
        "while" => Lexeme::While,
        "return" => Lexeme::Return,
        "print" => Lexeme::Print,
        "let" => Lexeme::Let,
        "call" => Lexeme::Call,
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

fn get_token_strings(source: &str) -> LinkedList<Lexeme> {
    let chars: Vec<char> = source.chars().collect();
    let mut tokens = LinkedList::new();
    if source.len() == 0 {
        return tokens;
    }

    let mut s = String::new();
    let mut index = 0;
    let mut all_chars_alphanumeric = chars[0].is_alphanumeric();
    while index < chars.len() {
        let ch = chars[index];
        let next_ch = if index + 1 < chars.len() {
            Some(chars[index + 1])
        } else {
            None
        };

        if ch == '/' && next_ch == Some('/'){
            let next_ind = chars.iter().position(|x| *x == '\n');
            if let Some(p) = next_ind {
                index = p + 1;
                continue;
            }
            else {
                break;
            }
        }        

        let is_ws = ch.is_whitespace();
        let alphanum_changed = ch.is_alphanumeric() != all_chars_alphanumeric;
        if s.len() > 0 && (is_ws || alphanum_changed) {
            tokens.push_back(token_to_lexeme(&s));
            s = String::new();
        }

        if alphanum_changed && !is_ws {
            all_chars_alphanumeric = ch.is_alphanumeric();

        }

        if !is_ws {
            s.push(ch);
        }

        index += 1;
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
