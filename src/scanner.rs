use std::collections::LinkedList;

use lexeme::Lexeme;
use lexeme::OperatorType;
use lexeme::VarType;
use token_stream::TokenStream;

/// Identify the token and return the corresponding lexeme
/// ```
/// token_to_lexeme("if") = Lexeme::If
/// ```
fn token_to_lexeme(token: &str) -> Lexeme {
    assert!(token.len() > 0);


    let parsed_int = token.parse::<i32>().ok();
    if let Some(value) = parsed_int {
        return Lexeme::IntConstant(value);
    }

    match token {
        "if" => Lexeme::If,
        "else" => Lexeme::Else,
        "while" => Lexeme::While,
        "return" => Lexeme::Return,
        "print" => Lexeme::Print,
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
        "." => Lexeme::Dot,
        _ => {
            // Case 1: It's a string constant
            if token.starts_with("\"") && token.ends_with("\"") {
                // Keep the double quote marks
                return Lexeme::StringConstant(token.to_string());
            }

            // Case 2: It's a identifier
            if token.chars().all(|ch| ch.is_alphanumeric() || ch == '_') {
                Lexeme::Identifier(token.to_string())
            }

            else {
                panic!("Unkown token! {}", token)
            }
        }
    }
}

/// Move forward the head of linkedList until meeting the character
/// ```
/// l = abcdef;
/// pop_until(&mut l, 'd');
/// assert_eq!(l, def)
/// ```
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

/// Get called when we meet a quote mark (")
/// Get the string and move the linkedList
/// ```
/// get_string_constant("fdjdk"adf) = "fdjdk"
/// ```
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

/// Return a linkedList of lexemes given the source code
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

        // We need the next character as well
        let next_char = chars.front().cloned();

        // We meet comments
        if c == '/' && next_char == Some('/') {
            pop_until(&mut chars, '\n');
            continue;
        }

        // It's a token
        let mut s = String::new();
        s.push(c);

        match c {
            '>' | '<' | '=' | '!' => {
                if next_char == Some('=') {
                    // We should append the '=' since '>=' is a single token
                    s.push(chars.pop_front().unwrap());
                }
            }
            '"' => {
                // Push c back and get the string constant
                chars.push_front(c);
                s = get_string_constant(&mut chars);
            }
            'a'...'z' | 'A' ... 'Z' | '0'...'9' => {
                while let Some(next_ch) = chars.front().cloned() {
                    if !next_ch.is_alphanumeric() && next_ch != '_' {
                        // '_' character is also allowed in identifiers
                        // so we need to consider it as well
                        break;
                    }
                    s.push(chars.pop_front().unwrap());
                }
            }
            _ => {

            }
        };

        // Store the token in the linkedList
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
