use ast::Statement;
use ast::Expression;
use lexeme::Lexeme;
use token_stream::TokenStream;

pub fn parse_term(tokens: &mut TokenStream) -> Expression {
    match tokens.consume() {
        Lexeme::IntConstant(v) => Expression::Value(v),
        _ => panic!("wth"),
    }
}

pub fn parse_return(tokens: &mut TokenStream) -> Statement {
    assert_eq!(tokens.consume(), Lexeme::Return);
    assert!(!tokens.is_empty());

    let expr = parse_term(tokens);
    return Statement::Return(Box::new(expr));
}
