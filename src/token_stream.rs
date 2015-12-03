use std::collections::LinkedList;
use lexeme::Lexeme;

/// TokenStream is simply a stack of Lexeme implemented with linkedList
pub struct TokenStream {
    token_list: LinkedList<Lexeme>,
}

impl TokenStream {
    // FIXME: Perhaps this should take an iterator and build a linked list from it?
    pub fn new(tokens: LinkedList<Lexeme>) -> TokenStream {
        TokenStream{token_list: tokens}
    }

    /// Pop the top of the stack
    pub fn consume(&mut self) -> Lexeme {
        self.token_list.pop_front().expect("no more tokens!")
    }

    /// Peek the top of the stack
    pub fn peek(&self) -> Lexeme {
        self.token_list.front().expect("no more tokens!").clone()
    }

    /// Push a lexeme onto the stack
    pub fn push(&mut self, tok: Lexeme) {
        self.token_list.push_front(tok);
    }

    /// Return true if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.token_list.is_empty()
    }
}
