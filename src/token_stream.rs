use std::collections::LinkedList;
use lexeme::Lexeme;

pub struct TokenStream {
    token_list: LinkedList<Lexeme>,
}

impl TokenStream {
    // FIXME: Perhaps this should take an iterator and build a linked list from it?
    pub fn new(tokens: LinkedList<Lexeme>) -> TokenStream {
        TokenStream{token_list: tokens}
    }

    pub fn consume(&mut self) -> Lexeme {
        self.token_list.pop_front().expect("no more tokens!")
    }

    pub fn peek(&self) -> Lexeme {
        self.token_list.front().expect("no more tokens!").clone()
    }

    pub fn push(&mut self, tok: Lexeme) {
        self.token_list.push_front(tok);
    }

    pub fn is_empty(&self) -> bool {
        self.token_list.is_empty()
    }
}
