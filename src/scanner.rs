use std::collections::LinkedList;

use lexeme::Lexeme;
use lexeme::OperatorType;
use lexeme::VarType;
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
        "fn" => Lexeme::Function,
        "struct" => Lexeme::Struct,
        "int" => Lexeme::Type(VarType::Int),
        "char" => Lexeme::Type(VarType::Char),
        "pointer" => Lexeme::Type(VarType::Pointer),
        "&" => Lexeme::Reference,
        "=" => Lexeme::Assign,
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
        "," => Lexeme::Comma,
        _ => {
            if token.starts_with("\"") && token.ends_with("\"") {
                // Keep the double quotes for now
                return Lexeme::StringConstant(token.to_string());
            }

            if token.chars().all(|ch| ch.is_alphanumeric() || ch == '_') {
                Lexeme::Identifier(token.to_string())
            }

            else {
                panic!("Unkown token! {}", token)
            }
        }
    }
}

fn pop_until(l: &mut LinkedList<char>, c: char) {
    while !l.is_empty() {
        {
            let first = l.front().unwrap();
            if *first == c {
                break;
            }
        }

        l.pop_front();
    }
}

fn get_string_constant(chars: &mut LinkedList<char>) -> String {
    assert_eq!(chars.front(), Some(&'"'));
    let mut s = String::new();
    s.push(chars.pop_front().unwrap());
    while let Some(c) = chars.pop_front() {
        s.push(c);

        if c == '"' {
            break;
        }
    }
    s
}

fn get_token_strings(source: &str) -> LinkedList<Lexeme> {
    let mut chars: LinkedList<char> = source.chars().collect();
    let mut tokens = LinkedList::new();
    if source.len() == 0 {
        return tokens;
    }

    while let Some(c) = chars.pop_front() {
        if c.is_whitespace() {
            continue;
        }
        let next_char = chars.front().cloned();

        // Comment
        if c == '/' && next_char == Some('/') {
            pop_until(&mut chars, '\n');
            continue;
        }

        let mut s = String::new();
        s.push(c);

        match c {
            '>' | '<' | '=' | '!' => {
                if next_char == Some('=') {
                    s.push(chars.pop_front().unwrap());
                }
            }
            '"' => {
                chars.push_front(c);
                s = get_string_constant(&mut chars);
            }
            'a'...'z' | 'A' ... 'Z' | '0'...'9' => {
                while let Some(next_ch) = chars.front().cloned() {
                    if !next_ch.is_alphanumeric() && next_ch != '_' {
                        break;
                    }
                    s.push(chars.pop_front().unwrap());
                }
            }
            _ => {

            }
        };

        tokens.push_back(token_to_lexeme(&s));
    }

    tokens
}

/// Our starter for scanner.rs
/// Convert the source code to a stream of tokens
pub fn get_tokens(source: &str) -> TokenStream {
    let t = get_token_strings(source);
    TokenStream::new(t)
}
