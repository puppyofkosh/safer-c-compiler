use ast::Statement;
use ast::Expression;
use lexeme::Lexeme;

pub fn parse_expr(tokens: &[Lexeme]) -> Expression {
    match tokens[0] {
        Lexeme::IntConstant(v) => Expression::Value(v),
        _ => panic!("Wth"),
    }
}

pub fn parse_return(tokens: &[Lexeme]) -> Statement {
    assert_eq!(tokens[0], Lexeme::Return);
    assert!(tokens.len() >= 2);

    let expr = parse_expr(&tokens[1..]);
    return Statement::Return(Box::new(expr));
}
